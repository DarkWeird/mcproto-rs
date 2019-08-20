//! MC Protocol packets

use serde_mcproto::types::var::VarInt;
use serde_mcproto::types::slot::Slot;
use serde_mcproto::types::uuid::{Uuid, Uuidi128};
use serde_mcproto::types::chat::Chat;
use serde_mcproto::types::entity_metadata::EntityMetadata;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_mcproto::types::array::{UBytePrefix, BytePrefix, ShortPrefix, IntPrefix, VarIntPrefix};
use serde_mcproto::types::chunk::ChunkDataBulk;
use serde_mcproto::types::nbt::{NBT, GZIPNBT};

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Clientbound,
    Serverbound,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NextState {
    None,
    //TODO variant index 0 :(
    Status,
    Login,
}


// F0 00 00 00 - x
// 0F 00 00 00 - z
// 00 FF 00 00 - y
// 00 00 FF F0 - block_id
// 00 00 00 0F - block_metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChangeRecord {
    xz: u8,
    y: u8,
    block_id: VarInt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    name: String,
    value: VarInt,
}


type Dimension = i8;


/// Response sent to clients as JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    // FIXME(toqueteos): This is ChatJson
    pub description: String,
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<Sample>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub name: String,
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ModifierData {
    uuid: Uuidi128,
    amount: f64,
    operation: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    key: String,
    value: f64,
    modifiers: ShortPrefix<ModifierData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    name: String,
    value: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData {
    object_id: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Handshake {
    Handshake { proto_version: VarInt, server_address: String, server_port: u16, next_state: NextState }
}

type MapData = Vec<u8>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayToClient {
    KeepAlive { keep_alive_id: i32 },
    JoinGame { entity_id: i32, gamemode: u8, dimension: Dimension, difficulty: u8, max_players: u8, level_type: String },
    ChatMessage { data: Chat },
    TimeUpdate { world_age: i64, time_of_day: i64 },
    EntityEquipment { entity_id: i32, slot: i16, item: Option<Slot> },
    SpawnPos { x: i32, y: i32, z: i32 },
    UpdateHealth { health: f32, food: i16, saturation: f32 },
    Respawn { dimension: Dimension, difficulty: u8, gamemode: u8, level_type: String },
    PlayerPositionAndLook { position: [f64; 3], yaw: f32, pitch: f32, on_ground: bool },
    HeldItemChange { slot: i8 },
    UseBed { entity_id: i32, x: i32, y: i8, z: i32 },
    Animation { entity_id: VarInt, animation: u8 },
    SpawnPlayer { entity_id: VarInt, player_uuid: Uuid/*String*/, player_name: String, data: VarIntPrefix<Data>, position: [i32; 3], yaw: u8, pitch: u8, current_item: i16, metadata: EntityMetadata },
    CollectItem { collected_eid: i32, collector_eid: i32 },
    SpawnObject { entity_id: VarInt, type_: i8, position: [i32; 3], pitch: i8, yaw: i8, data: ObjectData },
    SpawnMob { entity_id: VarInt, type_: u8, position: [i32; 3], yaw: i8, pitch: i8, head_pitch: i8, velocity: [i16; 3], metadata: EntityMetadata },
    SpawnPainting { entity_id: VarInt, title: String, x: i32, y: i32, z: i32, direction: i32 },
    SpawnExperienceOrb { entity_id: VarInt, position: [i32; 3], count: i16 },
    EntityVelocity { entity_id: i32, velocity: [i16; 3] },
    DestroyEntities { entity_ids: BytePrefix<i32> },
    EntityIdle { entity_id: i32 },
    EntityRelativeMove { entity_id: i32, delta: [i8; 3] },
    EntityLook { entity_id: i32, yaw: i8, pitch: i8 },
    EntityLookAndRelativeMove { entity_id: i32, delta: [i8; 3], yaw: i8, pitch: i8 },
    EntityTeleport { entity_id: i32, position: [i32; 3], yaw: i8, pitch: i8 },
    EntityHeadLook { entity_id: i32, head_yaw: i8 },
    EntityStatus { entity_id: i32, entity_status: i8 },
    AttachEntity { riding_eid: i32, vehicle_eid: i32, leash: bool },
    EntityMetadata { entity_id: i32, metadata: EntityMetadata },
    EntityEffect { entity_id: i32, effect_id: i8, amplifier: i8, duration: i16 },
    RemoveEntityEffect { entity_id: i32, effect_id: i8 },
    SetExperience { xp_bar: f32, level: i16, xp_total: i16 },
    EntityProperties { entity_id: i32, properties: IntPrefix<Property> },
    ChunkData { x: i32, z: i32, ground_up: bool, bit_map: u16, add_bit_map: u16, chunk_data: IntPrefix<u8> },
    MultiBlockChange { chunk_x: i32, chunk_z: i32, records: i16, data: IntPrefix<u8> },
    //TODO parse data
    BlockChange { x: i32, y: i8, z: i32, block_type: VarInt, metadata: u8 },
    BlockAction { x: i32, y: i16, z: i32, byte1: u8, byte2: u8, block_id: VarInt },
    BlockBreakAnimation { entity_id: VarInt, x: i32, y: i32, z: i32, destroy_stage: i8 },
    ChunkDataBulk(ChunkDataBulk),
    Explosion { position: [f32; 3], radius: f32, records: IntPrefix<[i8; 3]>, player_motion: [f32; 3] },
    Effect { effect_id: i32, x: i32, y: i8, z: i32, data: i32, global: bool },
    SoundEffect { name: String, position: [i32; 3], volume: f32, pitch: u8 },
    Particle {
        particle_name: String,
        position: [f32; 3],
        offset: [f32; 3],
        particle_data: f32,
        particle_count: i32,
    },
    ChangeGameState { reason: u8, value: f32 },
    SpawnGlobalEntity { entity_id: VarInt, type_: i8, position: [i32; 3] },
    OpenWindow {
        window_id: u8,
        inventory_type: u8,
        window_title: String,
        slots: u8,
        use_provided_title: bool,
        entity_id: Option<i32>,
        /*inventory_type == horse*/
    },
    CloseWindow { window_id: u8 },
    SetSlot { window_id: i8, slot: i16, item: Option<Slot> },
    WindowItems { window_id: u8, slots: ShortPrefix<Option<Slot>> },
    WindowProperty { window_id: i8, property: i16, value: i16 },
    ConfirmTransaction { window_id: u8, action_number: i16, accepted: bool },
    UpdateSign { x: i32, y: i16, z: i32, line0: String, line1: String, line2: String, line3: String },
    UpdateMap { item_damage: VarInt, data: MapData },
    UpdateBlockEntity { x: i32, y: i16, z: i32, action: u8, nbt_data: GZIPNBT },
    SignEditorOpen { x: i32, y: i32, z: i32 },
    Statistics { stats: VarIntPrefix<Stat> },
    UpdatePlayerList {
        player_name: String,
        online: bool,
        ping: i16,
    },
    PlayerAbilities { flags: i8, flying_speed: f32, walking_speed: f32 },
    TabComplete { matches: VarIntPrefix<String> },
    ScoreboardObjective { name: String, display_text: String, action: i8 },
    UpdateScore { item_name: String, action: i8, score_name: String, value: i32 },
    DisplayScoreboard { position: i8, name: String },
    UpdateTeam { team_name: String, action: Vec<u8> },
    //TODO implement TeamAction
    PluginMessage {
        channel: String,
        data: ShortPrefix<u8>,
    },
    Disconnect { reason: String },
//    ServerDifficulty { difficulty: u8 },
//    PlayCombatEvent { event: CombatEvent },
//    Camera { camera_id: VarInt },
//    WorldBorder { action: WorldBorderAction },
//    Title { action: TitleAction },
//    SetCompression { threshold: VarInt },
//    PlayerListHeaderFooter { header: Chat, footer: Chat },
//    ResourcePackSend { url: String, hash: String },
//    UpdateEntityNbt { entity_id: VarInt, tag: nbt::Blob },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayToServer {
    KeepAlive { keep_alive_id: i32 },
    ChatMessage { message: String },
    UseEntity { target_eid: i32, mouse: i8, position: Option<[f32; 3]> },
    // position, if mouse = 2
    PlayerIdle { on_ground: bool },
    PlayerPosition { x: f64, stance: f64, y: f64, z: f64, on_ground: bool },
    PlayerLook { yaw: f32, pitch: f32, on_ground: bool },
    PlayerPositionAndLook { x: f64, stance: f64, y: f64, z: f64, yaw: f32, pitch: f32, on_ground: bool },
    PlayerDigging { status: i8, x: i32, y: i8, z: i32, face: i8 },
    PlayerBlockPlacement { x: i32, y: i8, z: i32, direction: i8, held_item: Option<Slot>, cursor: [i8; 3] },
    HeldItemChange { slot: i16 },
    Animation { entity_id: i32, animation: i8 },
    EntityAction { entity_id: i32, action_id: i8, jump_boost: i32 },
    SteerVehicle { sideways: f32, forward: f32, jump: bool, unmount: bool },
    CloseWindow { window_id: u8 },
    ClickWindow { window_id: u8, slot: i16, button: i8, action_number: i16, mode: i8, clicked_item: Option<Slot> },
    ConfirmTransaction { window_id: i8, action_number: i16, accepted: bool },
    CreativeInventoryAction { slot: i16, clicked_item: Option<Slot> },
    EnchantItem { window_id: u8, enchantment: i8 },
    UpdateSign { x: i32, y: i16, z: i32, line0: String, line1: String, line2: String, line3: String },
    PlayerAbilities { flags: i8, flying_speed: f32, walking_speed: f32 },
    TabComplete { text: String },
    ClientSettings { locale: String, view_distance: i8, chat_mode: i8, chat_colors: bool, difficulty: u8, show_cape: bool },
    ClientStatus { action_id: i8 },
    CustomPayload {
        channel: String,
        data: Vec<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusToClient {
    StatusResponse { response: String },
    Pong { time: i64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusToServer {
    StatusRequest {},
    Ping { time: i64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginToClient {
    Disconnect { reason: String },
    EncryptionRequest { server_id: String, pubkey: ShortPrefix<u8>, verify_token: ShortPrefix<u8> },
    LoginSuccess { uuid: Uuid, username: String },
    //SetCompression { threshold: VarInt },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginToServer {
    LoginStart { name: String },
    EncryptionResponse { shared_secret: ShortPrefix<u8>, verify_token: ShortPrefix<u8> },
}
