mod basic_object{
    use crate::basic_object::quaternion::Quaternion;

    //Main vector implementation based off quaternions
    pub struct Vector3(Quaternion);

    impl Vector3{

        pub fn new(x: f64, y: f64, z: f64) -> Vector3{
            Vector3(Quaternion::new_vector(x, y, z))
        }

        //Getters
        pub fn x(&self) -> f64{
            self.0.i
        }

        pub fn y(&self) -> f64{
            self.0.j
        }

        pub fn z(&self) -> f64{
            self.0.k
        }

        //adds two vectors component by component
        pub fn add(&self, other: &Vector3) -> Vector3{
            Vector3::new(self.x() + other.x(),
                         self.y() + other.y(),
                         self.z() + other.z())
        }

        
    }



    //Quaternion Stuff I want to abstract away from myself
    mod quaternion{
        

        //A quaternion is the basic type used to represent all 3 dimensional vectors and rotations of those vectors in the simulator.
        //Because quaternions are confusing as hell, their functionality should be abstracted away as best as possible.
        //For my purposes, i is x, j is y, k is z
        #[derive(PartialEq, Debug)]
        pub struct Quaternion{
            pub r: f64,
            pub i: f64,
            pub j: f64,
            pub k: f64
        }

        impl Quaternion{

            //A quaternion of the sort that represents a 3d vector
            pub fn new_vector(x: f64, y: f64, z: f64) -> Quaternion{
                Quaternion { r: 0.0, i: x, j: y, k: z }
            }

            //Adds two quaternions
            pub fn add(&self, other: &Quaternion) -> Quaternion{
                Quaternion { r: self. r + other.r, 
                    i: self.i + other.i,
                    j: self.j + other.j, 
                    k: self.k + other.k }
            }

            //This does cool rotation stuff. Particularly if using unit quaternions
            //see this force the source for the equation: https://en.wikipedia.org/wiki/Quaternion#Conjugation,_the_norm,_and_reciprocal
            //see here for quaternion multiplication explination: https://eater.net/quaternions
            fn mult(&self, other: &Quaternion) -> Quaternion{
                Quaternion { 
                    r: (self.r * other.r) - (self.i * other.i) - (self.j * other.j) - (self.k * other.k), 
                    i: (self.r * other.i) + (self.i * other.r) + (self.j * other.k) - (self.k * other.j), 
                    j: (self.r * other.j) - (self.i * other.k) + (self.j * other.r) + (self.k * other.i), 
                    k: (self.r * other.k) + (self.i * other.j) - (self.j * other.i) + (self.k * other.r) }
            }

            fn mult_unit(&self, other: &UnitQuaternion) -> Quaternion{
                self.mult(&other.0)
            }

            //Gives you the inverse quaternion
            fn invert(&self) -> Quaternion{
                Quaternion { r: self.r, i: -self.i, j: -self.j, k: -self.k }
            }

            pub fn rotate(&self, rotation: &UnitQuaternion) -> Quaternion{
                rotation.mult(&self.mult_unit(&rotation.invert()))
            }   

            //Gets the absolut magnitude of the quarternion
            pub fn length(&self) -> f64{
                (self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
            }

            
        }

        //Unit garruntteed to have a length of 1.
        #[derive(PartialEq, Debug)]
        pub struct UnitQuaternion(Quaternion);

        impl UnitQuaternion {

            //Creates a unit quaternion that will be used for rotations
            //x, y, z define the vector to rotate around
            //degrees is the number of degrees to rotate. Point right thumb in in direction of rotation vector, fingers curl in dirrection of positive degrees
            pub fn new_rotation(x: f64, y: f64, z: f64, degrees: f64) -> UnitQuaternion{
                let mut axis: UnitQuaternion = UnitQuaternion::new(0.0, x, y, z);
                let real: f64 = degrees.cos();
                let img: f64 = degrees.sin();
                axis.0.r = real;
                axis.0.i = axis.0.i * img;
                axis.0.j = axis.0.j * img;
                axis.0.k = axis.0.k * img;
                axis
            }



            //New will take an arbitrary quaternion and scale it down to be a unit quaternion
            fn new(r: f64, i: f64, j: f64, k: f64) -> UnitQuaternion{
                let quat: Quaternion = Quaternion { r: r, i: i, j: j, k: k };
                UnitQuaternion::new_from_existing(&quat)
            }

            fn new_from_existing(exist: &Quaternion) -> UnitQuaternion{
                let length: f64 = exist.length();
                UnitQuaternion(Quaternion {
                    r: exist.r / length,
                    i: exist.i / length,
                    j: exist.j / length,
                    k: exist.k / length
                })
            }

            fn mult(&self, other: &Quaternion) -> Quaternion{
                self.0.mult(other)
            }

            fn invert(&self) -> UnitQuaternion{
                UnitQuaternion::new_from_existing(&(self.0.invert()))
            }

            fn rotate(&self, rotation: &UnitQuaternion) -> UnitQuaternion{
                UnitQuaternion::new_from_existing(&(self.0.rotate(rotation)))
            }
        }

        #[cfg(test)]
        mod tests{
            use super::*;

            #[test]
            fn test_add(){
                let numerand: Quaternion = Quaternion{ r: 1.0, i: 1.0, j: 1.0, k: 1.0};
                let result: Quaternion = Quaternion { r: 2.0, i: 2.0, j: 2.0, k: 2.0 };
                assert_eq!(numerand.add(&numerand), result)
            }
        }
    }
}