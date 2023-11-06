use super::Trivector;

impl Trivector {
    pub fn empty() -> Trivector {
        Trivector {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        } 
    }

    pub fn vec_distance(vector_1: &Trivector, vector_2: &Trivector, box_length: f64) -> Trivector {
        let mut distance = *vector_2 - *vector_1;

        if distance.x > box_length/2. {
            distance.x -= box_length;
        } else if distance.x < -box_length/2. {
            distance.x += box_length;
        }

        if distance.y > box_length/2. {
            distance.y -= box_length;
        } else if distance.y < -box_length/2. {
            distance.y += box_length;
        }

        if distance.z > box_length/2. {
            distance.z -= box_length;
        } else if distance.z < -box_length/2. {
            distance.z += box_length;
        }
        return distance;
    }

    pub fn dot_product(vector_1: &Trivector, vector_2: &Trivector) -> f64 {
        vector_1.x * vector_2.x + vector_1.y * vector_2.y + vector_1.z * vector_2.z
    }

    #[allow(dead_code)]
    pub fn distance(vector_1: &Trivector, vector_2: &Trivector, box_length: f64) -> f64 {
    
        let distance_vector = Trivector::vec_distance(vector_1, vector_2,box_length);
        
        Trivector::dot_product(&distance_vector,&distance_vector).sqrt()
    }

    pub fn distance_from_vec_distance(distance_vector: &Trivector) -> f64 {
        Trivector::dot_product(distance_vector,distance_vector).sqrt()
    }

}