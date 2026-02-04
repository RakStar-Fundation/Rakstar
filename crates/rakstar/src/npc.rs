use crate::call_api;
use crate::player::Player;
use bindings::types::NPCPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NPC {
    ptr: NPCPtr,
}

impl NPC {
    pub fn connect(name: &str, script: &str) -> bool {
        let c_name = std::ffi::CString::new(name).unwrap();
        let c_script = std::ffi::CString::new(script).unwrap();
        call_api!(npc.connect(c_name.as_ptr(), c_script.as_ptr())).unwrap_or(false)
    }

    pub fn create(name: &str) -> Option<Self> {
        let c_name = std::ffi::CString::new(name).ok()?;
        let mut id: i32 = -1;

        call_api!(npc.create(c_name.as_ptr(), &mut id)).and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn from_ptr(ptr: NPCPtr) -> Option<Self> {
        (!ptr.is_null()).then(|| Self { ptr })
    }

    pub fn as_ptr(&self) -> NPCPtr {
        self.ptr
    }

    pub fn from_id(id: i32) -> Option<Self> {
        call_api!(npc.from_id(id)).and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn get_id(&self) -> i32 {
        call_api!(npc.get_id(self.ptr)).unwrap_or(-1)
    }

    pub fn is_valid(&self) -> bool {
        call_api!(npc.is_valid(self.ptr)).unwrap_or(false)
    }

    pub fn get_player(&self) -> Option<Player> {
        let ptr = call_api!(npc.get_player(self.ptr))?;
        Player::from_ptr(ptr)
    }

    pub fn destroy(&self) -> bool {
        call_api!(npc.destroy(self.ptr)).unwrap_or(false)
    }

    pub fn spawn(&self) -> bool {
        call_api!(npc.spawn(self.ptr)).unwrap_or(false)
    }

    pub fn respawn(&self) -> bool {
        call_api!(npc.respawn(self.ptr)).unwrap_or(false)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(npc.set_pos(self.ptr, x, y, z)).unwrap_or(false)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);

        call_api!(npc.get_pos(self.ptr, &mut x, &mut y, &mut z));

        (x, y, z)
    }

    pub fn set_rot(&self, rx: f32, ry: f32, rz: f32) -> bool {
        call_api!(npc.set_rot(self.ptr, rx, ry, rz)).unwrap_or(false)
    }

    pub fn get_rot(&self) -> (f32, f32, f32) {
        let (mut rx, mut ry, mut rz) = (0.0f32, 0.0f32, 0.0f32);

        call_api!(npc.get_rot(self.ptr, &mut rx, &mut ry, &mut rz));

        (rx, ry, rz)
    }

    pub fn set_facing_angle(&self, angle: f32) -> bool {
        call_api!(npc.set_facing_angle(self.ptr, angle)).unwrap_or(false)
    }

    pub fn get_facing_angle(&self) -> f32 {
        let mut angle = 0.0f32;

        call_api!(npc.get_facing_angle(self.ptr, &mut angle));

        angle
    }

    pub fn set_virtual_world(&self, virtual_world: i32) -> bool {
        call_api!(npc.set_virtual_world(self.ptr, virtual_world)).unwrap_or(false)
    }

    pub fn get_virtual_world(&self) -> i32 {
        call_api!(npc.get_virtual_world(self.ptr)).unwrap_or(0)
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        call_api!(npc.set_interior(self.ptr, interior)).unwrap_or(false)
    }

    pub fn get_interior(&self) -> i32 {
        call_api!(npc.get_interior(self.ptr)).unwrap_or(0)
    }

    pub fn move_to(
        &self,
        x: f32,
        y: f32,
        z: f32,
        move_type: i32,
        move_speed: f32,
        stop_range: f32,
    ) -> bool {
        call_api!(npc.move_(self.ptr, x, y, z, move_type, move_speed, stop_range)).unwrap_or(false)
    }

    pub fn move_to_player(
        &self,
        player: &Player,
        move_type: i32,
        move_speed: f32,
        stop_range: f32,
        pos_check_update_delay: i32,
        auto_restart: bool,
    ) -> bool {
        call_api!(npc.move_to_player(
            self.ptr,
            player.as_ptr(),
            move_type,
            move_speed,
            stop_range,
            pos_check_update_delay,
            auto_restart
        ))
        .unwrap_or(false)
    }

    pub fn stop_move(&self) -> bool {
        call_api!(npc.stop_move(self.ptr)).unwrap_or(false)
    }

    pub fn is_moving(&self) -> bool {
        call_api!(npc.is_moving(self.ptr)).unwrap_or(false)
    }

    pub fn set_skin(&self, model: i32) -> bool {
        call_api!(npc.set_skin(self.ptr, model)).unwrap_or(false)
    }

    pub fn is_streamed_in(&self, player: &Player) -> bool {
        call_api!(npc.is_streamed_in(self.ptr, player.as_ptr())).unwrap_or(false)
    }

    pub fn is_any_streamed_in(&self) -> bool {
        call_api!(npc.is_any_streamed_in(self.ptr)).unwrap_or(false)
    }

    pub fn get_health(&self) -> f32 {
        call_api!(npc.get_health(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(npc.set_health(self.ptr, health)).unwrap_or(false)
    }

    pub fn get_armour(&self) -> f32 {
        call_api!(npc.get_armour(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_armour(&self, armour: f32) -> bool {
        call_api!(npc.set_armour(self.ptr, armour)).unwrap_or(false)
    }

    pub fn is_dead(&self) -> bool {
        call_api!(npc.is_dead(self.ptr)).unwrap_or(false)
    }

    pub fn set_invulnerable(&self, toggle: bool) -> bool {
        call_api!(npc.set_invulnerable(self.ptr, toggle)).unwrap_or(false)
    }

    pub fn is_invulnerable(&self) -> bool {
        call_api!(npc.is_invulnerable(self.ptr)).unwrap_or(false)
    }

    pub fn set_weapon(&self, weapon: u8) -> bool {
        call_api!(npc.set_weapon(self.ptr, weapon)).unwrap_or(false)
    }

    pub fn get_weapon(&self) -> u8 {
        call_api!(npc.get_weapon(self.ptr)).unwrap_or(0)
    }

    pub fn set_ammo(&self, ammo: i32) -> bool {
        call_api!(npc.set_ammo(self.ptr, ammo)).unwrap_or(false)
    }

    pub fn get_ammo(&self) -> i32 {
        call_api!(npc.get_ammo(self.ptr)).unwrap_or(0)
    }

    pub fn set_ammo_in_clip(&self, ammo: i32) -> bool {
        call_api!(npc.set_ammo_in_clip(self.ptr, ammo)).unwrap_or(false)
    }

    pub fn get_ammo_in_clip(&self) -> i32 {
        call_api!(npc.get_ammo_in_clip(self.ptr)).unwrap_or(0)
    }

    pub fn enable_reloading(&self, enable: bool) -> bool {
        call_api!(npc.enable_reloading(self.ptr, enable)).unwrap_or(false)
    }

    pub fn is_reload_enabled(&self) -> bool {
        call_api!(npc.is_reload_enabled(self.ptr)).unwrap_or(false)
    }

    pub fn is_reloading(&self) -> bool {
        call_api!(npc.is_reloading(self.ptr)).unwrap_or(false)
    }

    pub fn enable_infinite_ammo(&self, enable: bool) -> bool {
        call_api!(npc.enable_infinite_ammo(self.ptr, enable)).unwrap_or(false)
    }

    pub fn is_infinite_ammo_enabled(&self) -> bool {
        call_api!(npc.is_infinite_ammo_enabled(self.ptr)).unwrap_or(false)
    }

    pub fn get_weapon_state(&self) -> i32 {
        call_api!(npc.get_weapon_state(self.ptr)).unwrap_or(0)
    }

    pub fn set_keys(&self, up_and_down: u16, left_and_right: u16, keys: u16) -> bool {
        call_api!(npc.set_keys(self.ptr, up_and_down, left_and_right, keys)).unwrap_or(false)
    }

    pub fn get_keys(&self) -> (u16, u16, u16) {
        let (mut up_and_down, mut left_and_right, mut keys) = (0u16, 0u16, 0u16);

        call_api!(npc.get_keys(self.ptr, &mut up_and_down, &mut left_and_right, &mut keys));

        (up_and_down, left_and_right, keys)
    }

    pub fn set_weapon_skill_level(&self, skill: u8, level: i32) -> bool {
        call_api!(npc.set_weapon_skill_level(self.ptr, skill, level)).unwrap_or(false)
    }

    pub fn get_weapon_skill_level(&self, skill: i32) -> i32 {
        call_api!(npc.get_weapon_skill_level(self.ptr, skill)).unwrap_or(0)
    }
}
