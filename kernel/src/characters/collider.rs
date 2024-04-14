pub static PLAYER_COLLIDER_SIZE: (i16, i16) = (60, 35);
pub static WALL_COLLIDER_SIZE: (i16, i16) = (70, 40);
pub static ENEMY_COLLIDER_SIZE: (i16, i16) = (55, 40);
pub static BULLET_COLLIDER_SIZE: (i16, i16) = (5, 25);

pub struct Collider {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16,
}

impl Collider {
    pub fn collides_with(&self, other: &Collider) -> bool {
        self.top < other.bottom
            && self.bottom > other.top
            && self.left < other.right
            && self.right > other.left
    }
}

pub fn player_collider(position: &(i16, i16)) -> Collider {
    if position.1 < PLAYER_COLLIDER_SIZE.1 {
        return Collider {
            top: 0,
            bottom: PLAYER_COLLIDER_SIZE.1,
            left: position.0,
            right: position.0 + PLAYER_COLLIDER_SIZE.0,
        };
    }
    Collider {
        top: position.1 - PLAYER_COLLIDER_SIZE.1,
        bottom: position.1,
        left: position.0,
        right: position.0 + PLAYER_COLLIDER_SIZE.0,
    }
}

pub fn wall_collider(position: &(i16, i16)) -> Collider {
    if position.1 < WALL_COLLIDER_SIZE.1 {
        return Collider {
            top: 0,
            bottom: WALL_COLLIDER_SIZE.1,
            left: position.0,
            right: position.0 + WALL_COLLIDER_SIZE.0,
        };
    }
    Collider {
        top: position.1 - WALL_COLLIDER_SIZE.1,
        bottom: position.1,
        left: position.0,
        right: position.0 + WALL_COLLIDER_SIZE.0,
    }
}

pub fn enemy_collider(position: &(i16, i16)) -> Collider {
    if position.1 < ENEMY_COLLIDER_SIZE.1 {
        return Collider {
            top: 0,
            bottom: ENEMY_COLLIDER_SIZE.1,
            left: position.0,
            right: position.0 + ENEMY_COLLIDER_SIZE.0,
        };
    }
    Collider {
        top: position.1 - ENEMY_COLLIDER_SIZE.1,
        bottom: position.1,
        left: position.0,
        right: position.0 + ENEMY_COLLIDER_SIZE.0,
    }
}

pub fn bullet_collider(position: &(i16, i16)) -> Collider {
    if position.1 < BULLET_COLLIDER_SIZE.1 {
        return Collider {
            top: 0,
            bottom: BULLET_COLLIDER_SIZE.1,
            left: position.0,
            right: position.0 + BULLET_COLLIDER_SIZE.0,
        };
    }
    Collider {
        top: position.1 - BULLET_COLLIDER_SIZE.1,
        bottom: position.1,
        left: position.0,
        right: position.0 + BULLET_COLLIDER_SIZE.0,
    }
}
