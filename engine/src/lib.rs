use tuples::{add, is_point, is_vector, tuple_cmp, Tuple, is_point_at_or_below_ground};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Projectile {
    position:Tuple,
    velocity:Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        if is_vector(&position) {panic!("Position cannot be a vector");}
        if is_point(&velocity) {panic!("Velocity cannot be a point");}
        Projectile {position, velocity}
    }
}

pub fn projectile_cmp(a: &Projectile, b: &Projectile) -> bool {
        return tuple_cmp(&a.position, &b.position) && tuple_cmp(&a.velocity, &b.velocity);
}

#[allow(dead_code)]
pub struct Environment {
    gravity: Tuple,
    wind: Tuple
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        if is_point(&gravity) {panic!("gravity should be a vector not a point");} 
        if is_point(&wind) {panic!("wind should be a vector not a point");}
        Environment {gravity, wind}
    }
}

pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = add(&proj.position, &proj.velocity);
    println!("POSITION: {position:?}");
    let velocity = add(&add(&env.gravity, &env.wind), &proj.velocity);
    return Projectile::new(position, velocity);
}

#[cfg(test)]
mod tests {
    use tuples::{create_point, create_vector, normalization};
    use super::*;

    #[test]
    fn create_environment() {
        let gravity = create_vector(0.0, 9.8, 0.0);
        let wind = create_vector(1.0, 0.8, 0.0);
        Environment {gravity, wind};
    }

    #[test]
    #[should_panic]
    fn create_environment_with_points() {
        let gravity = create_point(0.0, 9.8, 0.0);
        let wind = create_point(1.0, 0.8, 0.0);
        Environment::new(gravity, wind);
    }

    #[test]
    fn create_projectile() {
        let position = create_point(1.0, 1.0, 1.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        Projectile::new(position, velocity);
    }

    #[test]
    fn add_position_of_projectiles() {

        let position = create_point(1.0, 1.0, 1.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        let proj_1 = Projectile::new(position, velocity);

        let position = create_point(1.0, 1.0, 1.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        let proj_2 = Projectile::new(position, velocity);

        let position_prime = add(&proj_1.position, &proj_2.position);
        let velocity = create_vector(1.0, 1.0, 1.0);
        let projectile_prime = Projectile::new(position_prime, velocity);

        let position = create_point(2.0, 2.0, 2.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        let proj_expected = Projectile::new(position, velocity);

        println!("EXPECTED: {proj_expected:?}");
        println!("ACTUAL: {projectile_prime:?}");

        assert!(projectile_cmp(&projectile_prime, &proj_expected));


    }


    #[test]
    #[should_panic]
    fn create_projectile_with_vector_for_position() {
        let position = create_vector(1.0, 1.0, 1.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        Projectile::new(position, velocity);
    }

    #[test]
    #[should_panic]
    fn create_projectile_with_point_for_velocity() {
        let position = create_point(1.0, 1.0, 1.0);
        let velocity = create_point(1.0, 1.0, 1.0);
        Projectile::new(position, velocity);
    }


    #[test]
    fn test_tick() {
        let position = create_point(0.0, 10.0, 0.0);
        let velocity = create_vector(0.0, 0.0, 0.0);
        let proj = Projectile::new(position, velocity);

        let gravity = create_vector(0.0, -10.0, 0.0);
        let wind = create_vector(0.0, 0.0, 0.0);
        let env = Environment::new(gravity, wind);


        let position_prime = create_point(0.0, 0.0, 0.0);
        let velocity_prime = create_vector(0.0, -20.0, 0.0);

        let proj_prime = tick(&env, &proj);
        let proj_prime = tick(&env, &proj_prime);
        println!("Initial: {proj:?}");
        println!("Prime: {proj_prime:?}");

        assert!(projectile_cmp(&proj_prime,&Projectile::new(position_prime, velocity_prime)))
    }

    #[test]
    fn test_cannon() {
        let position = create_point(1.0, 1.0, 0.0);
        let velocity = create_vector(1.0, 1.0, 1.0);
        let velocity = normalization(&velocity);
        let proj = Projectile::new(position,velocity);

        let gravity = create_vector(0.0, -0.1, 0.0);
        let wind = create_vector(-0.01, 0.0, 0.0);
        let env = Environment::new(gravity, wind);

        let position_prime = create_point(0.0, 0.0, 0.0);
        let velocity_prime = create_vector(0.0, -20.0, 0.0);

        println!("LOOPINT");

        let mut proj_prime = tick(&env, &proj);
        loop {
            proj_prime = tick(&env, &proj_prime); // variable shadowing doesn't work here. Must
            // make proj_prime mutable. Why? its because of lexical scope instead of dynamic scope
            // of "loop"

            println!("Prime: {proj_prime:?}");
            if is_point_at_or_below_ground(&proj_prime.position) {
                break;
            }
        } 
        // the above loop breaking means that the test works
        
        println!("Initial: {proj:?}");
        println!("Prime: {proj_prime:?}");
    }
}
