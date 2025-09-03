mod quaternion{
    
    //A quaternion is the basic type used to represent all 3 dimensional vectors and rotations of those vectors in the simulator.
    //Because quaternions are confusing as hell, their functionality should be abstracted away as best as possible.
    //For my purposes, i is x, j is y, k is z
    #[derive(PartialEq, Debug)]
    struct Quaternion{
        r: f64,
        i: f64,
        j: f64,
        k: f64
    }

    impl Quaternion{
        //Adds two quaternions
        fn add(&self, other: &Quaternion) -> Quaternion{
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

        //Gives you the inverse quaternion
        fn invert(&self) -> Quaternion{
            Quaternion { r: self.r, i: -self.i, j: -self.j, k: -self.k }
        }

        fn rotate(&self, rotation: &Quaternion) -> Quaternion{
            rotation.mult(&self.mult(&rotation.invert()))
        }   

        //Gets the absolut magnitude of the quarternion
        fn length(&self) -> f64{
            (self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)).sqrt()
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