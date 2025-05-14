use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Surface {
    pub a: glm::Vec2,
    pub b: glm::Vec2,
}

impl Surface {
    pub fn find_closest_point(&self, p: &glm::Vec2) -> glm::Vec2 {
        let AB = self.b - self.a;
        let AB2 = glm::dot(&AB, &AB);
        if AB2 == 0.0f32 {
            return self.a;
        }
        let w = p - self.a;
        let mut t = AB.dot(&w) / AB2;
        t = t.clamp(-1.0, 1.0);

        // glm::Vec2::new(
        //     (self.a.x + t*AB.x),
        //     (self.a.y + t*AB.y),
        // )
        return self.a + t * AB;
    }
}

