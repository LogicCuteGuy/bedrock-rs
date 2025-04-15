use std::collections::HashMap;
use bedrockrs::proto::connection::Connection;
use bedrockrs::proto::listener::Listener;
use bedrockrs_proto::compression::Compression;
use tokio::time::Instant;
use uuid::Uuid;
use vek::{Vec2, Vec3};
use bedrockrs_proto::v662::enums::{ConnectionFailReason, Difficulty, Dimension, EditorWorldType, EducationEditionOffer, GamePublishSetting, GameType, Gamemode, GeneratorType, PacketCompressionAlgorithm, PlayerPermissionLevel, ServerAuthMovementMode};
use bedrockrs_proto::v662::packets::{AvailableActorIdentifiersPacket, NetworkSettingsPacket};
use bedrockrs_proto::v662::types::{ActorRuntimeID, ActorUniqueID, GameRulesChangedPacketData, NetworkBlockPosition, SyncedPlayerMovementSettings};
use bedrockrs_proto::v729::helper::ProtoHelperV729;
use bedrockrs_proto::v729::packets::play_status::PlayStatusPacket;
use bedrockrs_proto::v729::packets::player_disconnect::{DisconnectPlayerPacket, DisconnectReason};
use bedrockrs_proto::v729::types::base_game_version::BaseGameVersion;
use bedrockrs_proto::v729::types::chat_restriction_level::ChatRestrictionLevel;
use bedrockrs_proto::v729::types::edu_shared_uri_resource::EduSharedResourceUri;
use bedrockrs_proto::v729::types::experiments::Experiments;
use bedrockrs_proto::v729::types::network_permissions::NetworkPermissions;
use bedrockrs_proto::v729::types::play_status::PlayStatusType;
use bedrockrs_proto::v729::types::player_movement_mode::PlayerMovementMode;
use bedrockrs_proto::v729::types::player_movement_settings::PlayerMovementSettings;
use bedrockrs_proto::v729::types::spawn_biome_type::SpawnBiomeType;
use bedrockrs_proto::v729::types::spawn_settings::SpawnSettings;
use bedrockrs_proto::v748::packets::{DisconnectPacket, DisconnectPacketMessage, ResourcePackStackPacket};
use bedrockrs_proto::v748::types::LevelSettings;
use bedrockrs_proto::v766::packets::ResourcePacksInfoPacket;
use bedrockrs_proto::v776::gamepackets::GamePackets;
use bedrockrs_proto::v776::helper::ProtoHelperV776;
use bedrockrs_proto::v776::packets::{ItemRegistryPacket, StartGamePacket};

#[tokio::main]
async fn main() {
    let mut listener = Listener::new_raknet(
        "§5Hot Chickens in Your Area!!!".to_string(),
        "bedrockrs".to_string(),
        "1.0".to_string(),
        100,
        10,
        "127.0.0.1:19132".parse().unwrap(),
        false,
    )
    .await
    .unwrap();

    listener.start().await.unwrap();

    loop {
        let conn = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_login(conn).await;
        });
    }
}

async fn handle_login(mut conn: Connection<ProtoHelperV776>) {
    let time_start = Instant::now();

    // NetworkSettingsRequest
    conn.recv().await.unwrap();
    println!("NetworkSettingsRequest");

    let compression = Compression::None;

    // NetworkSettings
    conn.send(&[GamePackets::NetworkSettings(NetworkSettingsPacket {
        compression_threshold: 1,
        compression_algorithm: PacketCompressionAlgorithm::None,
        client_throttle_enabled: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    })])
    .await
    .unwrap();
    println!("NetworkSettings");

    conn.compression = Some(compression);

    // Login
    println!("Login data {:?}", conn.recv().await.unwrap());

    conn.send(&[
        GamePackets::PlaySatus(PlayStatusPacket {
            status: PlayStatusType::LoginSuccess,
        }),
        GamePackets::ResourcePacksInfo(ResourcePacksInfoPacket {
            resource_pack_required: false,
            has_addon_packs: false,
            has_scripts: false,
            world_template_uuid: Default::default(),
            resource_packs: vec![],
            world_template_version: "".to_string(),
        }),
        GamePackets::ResourcePackStack(ResourcePackStackPacket {
            texture_pack_required: false,
            addon_list: vec![],
            base_game_version: BaseGameVersion(String::from("1.0")),
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            texture_pack_list: vec![],
            include_editor_packs: false,
        }),
    ])
    .await
    .unwrap();
    println!("PlayStatus (LoginSuccess)");
    println!("ResourcePacksInfo");
    println!("ResourcePackStack");

    // println!("{:#?}", conn.recv::<ProtoHelperV662>().await);
    // println!("ClientCacheStatus");
    // println!("{:#?}", conn.recv::<ProtoHelperV662>().await);
    // println!("ResourcePackClientResponse");

    // conn.send(&[
    //     GamePackets::Disconnect(DisconnectPacket {
    //         reason: ConnectionFailReason::Unknown,
    //         messages: Some(DisconnectPacketMessage{
    //             message: String::from("Test Disconnected"),
    //             filtered_message: String::from("")
    //         }),
    //     })
    // ])
    // .await
    // .unwrap();

    let packet1 = StartGamePacket {
        target_actor_id: ActorUniqueID(0),
        target_runtime_id: ActorRuntimeID(0),
        position: Vec3 {
            x: 4.0,
            y: 6.0,
            z: 7.0,
        },
        rotation: Vec2 { x: 270.0, y: 90.0 },
        settings: LevelSettings {
            seed: 777777777777,
            spawn_settings: SpawnSettings {
                biome_type: SpawnBiomeType::Default,
                user_defined_biome_name: String::from("RandomBiome"),
                dimension: Dimension::Overworld,
            },
            generator_type: GeneratorType::Overworld,
            game_type: GameType::Undefined,
            is_hardcore_mode_enabled: false,
            game_difficulty: Difficulty::Peaceful,
            achievements_disabled: true,
            editor_world_type: EditorWorldType::NotEditor,
            is_created_in_editor: false,
            day_cycle_stop_time: 2000,
            education_edition_offer: EducationEditionOffer::None,
            education_product_id: String::from(""),
            rain_level: 300.0,
            lightning_level: 400.0,
            has_confirmed_platform_locked_content: false,
            multiplayer_enabled: false,
            lan_broadcasting_enabled: false,
            xbox_live_broadcast_setting: GamePublishSetting::NoMultiPlay,
            platform_broadcast_setting: GamePublishSetting::NoMultiPlay,
            commands_enabled: false,
            texture_packs_required: false,
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            bonus_chest_enabled: false,
            starting_map_enabled: false,
            player_permissions: PlayerPermissionLevel::Visitor,
            server_chunk_tick_range: 0,
            locked_behaviour_pack: false,
            from_locked_template: false,
            from_template: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: BaseGameVersion(String::from("1.21.0")),
            limited_world_width: 16,
            limited_world_depth: 16,
            edu_shared_uri_resource: EduSharedResourceUri {
                button_name: String::from(""),
                link_uri: String::from(""),
            },
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
            server_identifier: "".to_string(),
            server_world_identifier: "".to_string(),
            default_spawn_block_position: NetworkBlockPosition {
                x: 0,
                y: 0,
                z: 0,
            },
            is_exported_from_editor: false,
            education_features_enabled: false,
            rule_data: GameRulesChangedPacketData { rules_list: vec![] },
            locked_resource_pack: false,
            use_msa_gamer_tags: false,
            has_locked_template_settings: false,
            nether_type: false,
            override_force_experimental_gameplay: false,
            server_scenario_identifier: "".to_string(),
        },
        level_id: String::from("UmFuZG9tIFdvcmxk"),
        level_name: String::from("Random World"),
        template_content_identity: String::from(""),
        movement_settings: SyncedPlayerMovementSettings {
            authority_mode: ServerAuthMovementMode::ServerAuthoritative,
            rewind_history_size: 3200,
            server_authoritative_block_breaking: false,
        },
        current_level_time: 9000,
        enchantment_seed: 99000,
        block_properties: vec![],
        multiplayer_correlation_id: String::from("c5d3d2cc-27fd-4221-9de6-d22c4d423d53"),
        server_version: String::from("1.19.2"),
        player_property_data: nbtx::Value::Compound(HashMap::new()),
        world_template_id: Uuid::nil(),
        server_enabled_client_side_generation: false,
        block_network_ids_are_hashes: false,
        is_trial: false,
        enable_item_stack_net_manager: false,
        server_block_type_registry_checksum: 0,
        network_permissions: NetworkPermissions { server_auth_sound: false },
        player_gamemode: Gamemode::Survival,
    };
    //
    // let buf = vec![0xc2, 0x9, 0x92, 0x3, 0x2, 0x0, 0x0, 0x80, 0x40, 0x0, 0x0, 0xc0, 0x40, 0x0, 0x0, 0xe0, 0x40, 0x0, 0x0, 0x87, 0x43, 0x0, 0x0, 0xb4, 0x42, 0x71, 0xc, 0x2b, 0x17, 0xb5, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x52, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x42, 0x69, 0x6f, 0x6d, 0x65, 0x0, 0x2, 0x2, 0x0, 0x0, 0xc8, 0x1, 0xc8, 0x1, 0xd8, 0x4, 0x1, 0x0, 0x0, 0x0, 0xa0, 0x1f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x96, 0x43, 0x0, 0x0, 0xc8, 0x43, 0x0, 0x1, 0x1, 0x4, 0x4, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x31, 0x2e, 0x32, 0x31, 0x2e, 0x30, 0x10, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0x55, 0x6d, 0x46, 0x75, 0x5a, 0x47, 0x39, 0x74, 0x49, 0x46, 0x64, 0x76, 0x63, 0x6d, 0x78, 0x6b, 0xc, 0x52, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x0, 0x0, 0x2, 0x80, 0x32, 0x0, 0x28, 0x23, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xf0, 0x8a, 0xc, 0x0, 0x0, 0x24, 0x63, 0x35, 0x64, 0x33, 0x64, 0x32, 0x63, 0x63, 0x2d, 0x32, 0x37, 0x66, 0x64, 0x2d, 0x34, 0x32, 0x32, 0x31, 0x2d, 0x39, 0x64, 0x65, 0x36, 0x2d, 0x64, 0x32, 0x32, 0x63, 0x34, 0x64, 0x34, 0x32, 0x33, 0x64, 0x35, 0x33, 0x0, 0x6, 0x31, 0x2e, 0x31, 0x39, 0x2e, 0x32, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0];
    // println!("{buf:?}");
    // let mut buf = vec![];
    // packet1.proto_serialize(&mut buf).unwrap();
    // println!("{buf:?}");
    //

    conn.send(&[
        GamePackets::StartGame(packet1),
        GamePackets::ItemRegistryPacket(ItemRegistryPacket { items: vec![] })
    ]).await.unwrap();
    println!("StartGame");

    conn.send(&[GamePackets::PlaySatus(PlayStatusPacket {
        status: PlayStatusType::PlayerSpawn,
    })])
        .await.unwrap();
    println!("PlayStatusPacket (PlayerSpawn)");

    let time_end = Instant::now();

    println!("{:?}", time_end.duration_since(time_start));

    loop {
        let res = conn.recv().await;

        if let Ok(packet) = res {
            println!("{:?}", packet);
        }
    }
}
