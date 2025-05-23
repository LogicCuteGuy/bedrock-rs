use crate::v748::packets::{
    AddActorPacket, AddItemActorPacket, AddPlayerPacket, AwardAchievementPacket, BossEventPacket,
    CameraInstructionPacket, CameraPresetsPacket, ChangeDimensionPacket,
    ClientBoundCloseFormPacket, ClientBoundDebugRendererPacket, ClientBoundMapItemDataPacket,
    CodeBuilderSourcePacket, ContainerClosePacket, ContainerRegistryCleanupPacket,
    CorrectPlayerMovePredictionPacket, CurrentStructureFeaturePacket, DisconnectPacket,
    EditorNetworkPacket, EmotePacket, InventoryContentPacket, InventorySlotPacket,
    ItemStackRequestPacket, ItemStackResponsePacket, JigsawStructureDataPacket,
    LegacyTelemetryEventPacket, MobArmorEquipmentPacket, PlayerActionPacket,
    PlayerArmorDamagePacket, ResourcePackStackPacket, ServerBoundDiagnosticsPacket,
    ServerBoundLoadingScreenPacket, SetActorDataPacket, SetActorLinkPacket, SetTitlePacket,
    StopSoundPacket, TextPacket, TransferPlayerPacket, UpdateAttributesPacket,
    UpdatePlayerGameTypePacket, UpdateSoftEnumPacket,
};
use crate::version::v662::packets::{
    ActorEventPacket, ActorPickRequestPacket, AddBehaviourTreePacket, AddPaintingPacket,
    AddVolumeEntityPacket, AgentActionEventPacket, AgentAnimationPacket, AnimateEntityPacket,
    AnimatePacket, AnvilDamagePacket, AutomationClientConnectPacket,
    AvailableActorIdentifiersPacket, AvailableCommandsPacket, BiomeDefinitionListPacket,
    BlockActorDataPacket, BlockEventPacket, BlockPickRequestPacket, BookEditPacket, CameraPacket,
    CameraShakePacket, ChangeMobPropertyPacket, ChunkRadiusUpdatedPacket,
    ClientCacheBlobStatusPacket, ClientCacheMissResponsePacket, ClientCacheStatusPacket,
    ClientToServerHandshakePacket, CodeBuilderPacket,
    CommandOutputPacket, CommandRequestPacket, CompletedUsingItemPacket,
    CompressedBiomeDefinitionListPacket, ContainerOpenPacket, ContainerSetDataPacket,
    CreatePhotoPacket, DeathInfoPacket, DebugInfoPacket,
    DimensionDataPacket, EduUriResourcePacket, EducationSettingsPacket, EmoteListPacket,
    FeatureRegistryPacket, GameRulesChangedPacket, GameTestRequestPacket, GameTestResultsPacket,
    GuiDataPickItemPacket, HurtArmorPacket, InteractPacket, InventoryTransactionPacket,
    ItemComponentPacket, LabTablePacket, LecternUpdatePacket, LessonProgressPacket,
    LevelChunkPacket, LevelEventGenericPacket, LevelEventPacket, LevelSoundEventPacket,
    LevelSoundEventPacketV1, LevelSoundEventPacketV2, MapCreateLockedCopyPacket,
    MapInfoRequestPacket, MobEffectPacket, MobEquipmentPacket, ModalFormRequestPacket,
    ModalFormResponsePacket, MotionPredictionHintsPacket, MoveActorAbsolutePacket,
    MoveActorDeltaPacket, MovePlayerPacket, MultiplayerSettingsPacket,
    NetworkChunkPublisherUpdatePacket, NetworkSettingsPacket, NetworkStackLatencyPacket,
    NpcDialoguePacket, NpcRequestPacket, OnScreenTextureAnimationPacket, OpenSignPacket,
    PacketViolationWarningPacket, PassengerJumpPacket, PhotoTransferPacket, PlaySoundPacket,
    PlayerEnchantOptionsPacket, PlayerFogPacket, PlayerHotbarPacket,
    PlayerInputPacket, PlayerSkinPacket, PlayerStartItemCooldownPacket,
    PlayerToggleCrafterSlotRequestPacket, PositionTrackingDBClientRequestPacket,
    PositionTrackingDBServerBroadcastPacket, PurchaseReceiptPacket, RefreshEntitlementsPacket,
    RemoveActorPacket, RemoveObjectivePacket, RemoveVolumeEntityPacket, RequestAbilityPacket,
    RequestChunkRadiusPacket, RequestNetworkSettingsPacket, RequestPermissionsPacket,
    ResourcePackChunkDataPacket, ResourcePackChunkRequestPacket, ResourcePackClientResponsePacket,
    ResourcePackDataInfoPacket, RespawnPacket, ScriptMessagePacket,
    ServerPlayerPostMovePositionPacket, ServerSettingsRequestPacket, ServerSettingsResponsePacket,
    ServerStatsPacket, ServerToClientHandshakePacket, SetActorMotionPacket,
    SetCommandsEnabledPacket, SetDefaultGameTypePacket, SetDifficultyPacket,
    SetDisplayObjectivePacket, SetHealthPacket, SetHudPacket, SetLastHurtByPacket,
    SetLocalPlayerAsInitializedPacket, SetPlayerGameTypePacket, SetPlayerInventoryOptionsPacket,
    SetScorePacket, SetScoreboardIdentityPacket, SetSpawnPositionPacket, SetTimePacket,
    SettingsCommandPacket, ShowCreditsPacket, ShowProfilePacket, ShowStoreOfferPacket,
    SimpleEventPacket, SimulationTypePacket, SpawnExperienceOrbPacket, SpawnParticleEffectPacket,
    StructureBlockUpdatePacket, StructureDataRequestPacket, StructureDataResponsePacket,
    SubChunkPacket, SubChunkRequestPacket, SubClientLoginPacket, SyncActorPropertyPacket,
    TakeItemActorPacket, TickSyncPacket, TickingAreaLoadStatusPacket, ToastRequestPacket,
    TrimDataPacket, UnlockedRecipesPacket, UpdateAbilitiesPacket, UpdateAdventureSettingsPacket,
    UpdateBlockPacket, UpdateBlockSyncedPacket, UpdateClientInputLocksPacket, UpdateEquipPacket,
    UpdateSubChunkBlocksPacket, UpdateTradePacket,
};
use crate::version::v662::{
    get_gamepacket_header_size_prediction, read_gamepacket_header, write_gamepacket_header,
};
use crate::version::v766::packets::{
    CraftingDataPacket, MovementEffectPacket,
    PlayerAuthInputPacket, PlayerListPacket, ResourcePacksInfoPacket, SetMovementAuthorityPacket,
};
use bedrockrs_macros::gamepackets;
use bedrockrs_proto_core::sub_client::SubClientID;
use std::io::{Cursor, Write};
use crate::v729::packets::play_status::PlayStatusPacket;
use crate::v766::packets::CameraAimAssistPacket;
use crate::v776::packets::{CameraAimAssistPresetsPacket, ClientCameraAimAssistPacket, ClientMovementPredictionSyncPacket, CommandBlockUpdatePacket, CreativeContentPacket, ItemRegistryPacket, StartGamePacket};
use crate::v785::packets::{PlayerUpdateEntityOverridesPacket, PlayerVideoCapturePacket, UpdateClientOptionsPacket};
use crate::v729::packets::login::LoginPacket;
gamepackets! {
    CurrentStructureFeature: CurrentStructureFeaturePacket,
    MovementEffect: MovementEffectPacket,
    CameraAimAssist: CameraAimAssistPacket,
    CameraAimAssistPresets: CameraAimAssistPresetsPacket,
    AwardAchievent: AwardAchievementPacket,
    ClientBoundCloseForm: ClientBoundCloseFormPacket,
    ContainerRegistryCleanup: ContainerRegistryCleanupPacket,
    JigsawStructureData: JigsawStructureDataPacket,
    ServerboundDiagnostics: ServerBoundDiagnosticsPacket,
    ServerBoundLoadingScreen: ServerBoundLoadingScreenPacket,
    SetMovementAuthority: SetMovementAuthorityPacket,
    Login: LoginPacket,
    PlayStatus: PlayStatusPacket,
    ServerToClientHandshake: ServerToClientHandshakePacket,
    ClientToServerHandshake: ClientToServerHandshakePacket,
    Disconnect: DisconnectPacket,
    ResourcePacksInfo: ResourcePacksInfoPacket,
    ResourcePackStack: ResourcePackStackPacket,
    ResourcePackClientResponse: ResourcePackClientResponsePacket,
    Text: TextPacket,
    SetTime: SetTimePacket,
    StartGame: StartGamePacket,
    AddPlayer: AddPlayerPacket,
    AddActor: AddActorPacket,
    RemoveActor: RemoveActorPacket,
    AddItemActor: AddItemActorPacket,
    ServerPlayerPostMovePosition: ServerPlayerPostMovePositionPacket,
    TakeItemActor: TakeItemActorPacket,
    MoveActorAbsolute: MoveActorAbsolutePacket,
    MovePlayer: MovePlayerPacket,
    PassengerJump: PassengerJumpPacket,
    UpdateBlock: UpdateBlockPacket,
    AddPainting: AddPaintingPacket,
    TickSync: TickSyncPacket,
    LevelSoundEventV1: LevelSoundEventPacketV1,
    LevelEvent: LevelEventPacket,
    BlockEvent: BlockEventPacket,
    ActorEvent: ActorEventPacket,
    MobEffect: MobEffectPacket,
    UpdateAttributes: UpdateAttributesPacket,
    InventoryTransaction: InventoryTransactionPacket,
    MobEquipment: MobEquipmentPacket,
    MobArmorEquipment: MobArmorEquipmentPacket,
    Interact: InteractPacket,
    BlockPickRequest: BlockPickRequestPacket,
    ActorPickRequest: ActorPickRequestPacket,
    PlayerAction: PlayerActionPacket,
    HurtArmor: HurtArmorPacket,
    SetActorData: SetActorDataPacket,
    SetActorMotion: SetActorMotionPacket,
    SetActorLink: SetActorLinkPacket,
    SetHealth: SetHealthPacket,
    SetSpawnPosition: SetSpawnPositionPacket,
    Animate: AnimatePacket,
    Respawn: RespawnPacket,
    ContainerOpen: ContainerOpenPacket,
    ContainerClose: ContainerClosePacket,
    PlayerHotbar: PlayerHotbarPacket,
    InventoryContent: InventoryContentPacket,
    InventorySlot: InventorySlotPacket,
    ContainerSetData: ContainerSetDataPacket,
    CraftingData: CraftingDataPacket,
    GuiDataPickItem: GuiDataPickItemPacket,
    BlockActorData: BlockActorDataPacket,
    PlayerInput: PlayerInputPacket,
    LevelChunk: LevelChunkPacket,
    SetCommandsEnabled: SetCommandsEnabledPacket,
    SetDifficulty: SetDifficultyPacket,
    ChangeDimension: ChangeDimensionPacket,
    SetPlayerGameType: SetPlayerGameTypePacket,
    PlayerList: PlayerListPacket,
    SimpleEvent: SimpleEventPacket,
    LegacyTelemetryEvent: LegacyTelemetryEventPacket,
    SpawnExperienceOrb: SpawnExperienceOrbPacket,
    ClientBoundMapItemData: ClientBoundMapItemDataPacket,
    MapInfoRequest: MapInfoRequestPacket,
    RequestChunkRadius: RequestChunkRadiusPacket,
    ChunkRadiusUpdated: ChunkRadiusUpdatedPacket,
    GameRulesChanged: GameRulesChangedPacket,
    Camera: CameraPacket,
    BossEvent: BossEventPacket,
    ShowCredits: ShowCreditsPacket,
    AvailableCommands: AvailableCommandsPacket,
    CommandRequest: CommandRequestPacket,
    CommandBlockUpdate: CommandBlockUpdatePacket,
    CommandOutput: CommandOutputPacket,
    UpdateTrade: UpdateTradePacket,
    UpdateEquip: UpdateEquipPacket,
    ResourcePackDataInfo: ResourcePackDataInfoPacket,
    ResourcePackChunkData: ResourcePackChunkDataPacket,
    ResourcePackChunkRequest: ResourcePackChunkRequestPacket,
    TransferPlayer: TransferPlayerPacket,
    PlaySound: PlaySoundPacket,
    StopSound: StopSoundPacket,
    SetTitle: SetTitlePacket,
    AddBehaviourTree: AddBehaviourTreePacket,
    StructureBlockUpdate: StructureBlockUpdatePacket,
    ShowStoreOffer: ShowStoreOfferPacket,
    PurchaseReceipt: PurchaseReceiptPacket,
    PlayerSkin: PlayerSkinPacket,
    SubClientLogin: SubClientLoginPacket,
    AutomationClientConnect: AutomationClientConnectPacket,
    SetLastHurtBy: SetLastHurtByPacket,
    BookEdit: BookEditPacket,
    NpcRequest: NpcRequestPacket,
    PhotoTransfer: PhotoTransferPacket,
    ModalFormRequest: ModalFormRequestPacket,
    ModalFormResponse: ModalFormResponsePacket,
    ServerSettingsRequest: ServerSettingsRequestPacket,
    ServerSettingsResponse: ServerSettingsResponsePacket,
    ShowProfile: ShowProfilePacket,
    SetDefaultGameType: SetDefaultGameTypePacket,
    RemoveObjective: RemoveObjectivePacket,
    SetDisplayObjective: SetDisplayObjectivePacket,
    SetScore: SetScorePacket,
    LabTable: LabTablePacket,
    UpdateBlockSynced: UpdateBlockSyncedPacket,
    MoveActorDelta: MoveActorDeltaPacket,
    SetScoreboardIdentity: SetScoreboardIdentityPacket,
    SetLocalPlayerAsInitialized: SetLocalPlayerAsInitializedPacket,
    UpdateSoftEnum: UpdateSoftEnumPacket,
    NetworkStackLatency: NetworkStackLatencyPacket,
    SpawnParticleEffect: SpawnParticleEffectPacket,
    AvailableActorIdentifiers: AvailableActorIdentifiersPacket,
    LevelSoundEventV2: LevelSoundEventPacketV2,
    NetworkChunkPublisherUpdate: NetworkChunkPublisherUpdatePacket,
    BiomeDefinitionList: BiomeDefinitionListPacket,
    LevelSoundEvent: LevelSoundEventPacket,
    LevelEventGeneric: LevelEventGenericPacket,
    LecternUpdate: LecternUpdatePacket,
    ClientCacheStatus: ClientCacheStatusPacket,
    OnScreenTextureAnimation: OnScreenTextureAnimationPacket,
    MapCreateLockedCopy: MapCreateLockedCopyPacket,
    StructureDataRequest: StructureDataRequestPacket,
    StructureDataResponse: StructureDataResponsePacket,
    ClientCacheBlobStatus: ClientCacheBlobStatusPacket,
    ClientCacheMissResponse: ClientCacheMissResponsePacket,
    EducationSettings: EducationSettingsPacket,
    Emote: EmotePacket,
    MultiplayerSettings: MultiplayerSettingsPacket,
    SettingsCommand: SettingsCommandPacket,
    AnvilDamage: AnvilDamagePacket,
    CompletedUsingItem: CompletedUsingItemPacket,
    NetworkSettings: NetworkSettingsPacket,
    PlayerAuthInput: PlayerAuthInputPacket,
    CreativeContent: CreativeContentPacket,
    PlayerEnchantOptions: PlayerEnchantOptionsPacket,
    ItemStackRequest: ItemStackRequestPacket,
    ItemStackResponse: ItemStackResponsePacket,
    PlayerArmorDamage: PlayerArmorDamagePacket,
    CodeBuilder: CodeBuilderPacket,
    UpdatePlayerGameType: UpdatePlayerGameTypePacket,
    EmoteList: EmoteListPacket,
    PositionTrackingDbServerBroadcast: PositionTrackingDBServerBroadcastPacket,
    PositionTrackingDbClientRequest: PositionTrackingDBClientRequestPacket,
    DebugInfo: DebugInfoPacket,
    PacketViolationWarning: PacketViolationWarningPacket,
    MotionPredictionHints: MotionPredictionHintsPacket,
    AnimateEntity: AnimateEntityPacket,
    CameraShake: CameraShakePacket,
    PlayerFog: PlayerFogPacket,
    CorrectPlayerMovePrediction: CorrectPlayerMovePredictionPacket,
    ItemComponent: ItemComponentPacket,
    ClientBoundDebugRenderer: ClientBoundDebugRendererPacket,
    SyncActorProperty: SyncActorPropertyPacket,
    AddVolumeEntity: AddVolumeEntityPacket,
    RemoveVolumeEntity: RemoveVolumeEntityPacket,
    SimulationType: SimulationTypePacket,
    NpcDialogue: NpcDialoguePacket,
    EduUriResource: EduUriResourcePacket,
    CreatePhoto: CreatePhotoPacket,
    UpdateSubChunkBlocks: UpdateSubChunkBlocksPacket,
    SubChunk: SubChunkPacket,
    SubChunkRequest: SubChunkRequestPacket,
    PlayerStartItemCooldown: PlayerStartItemCooldownPacket,
    ScriptMessage: ScriptMessagePacket,
    CodeBuilderSource: CodeBuilderSourcePacket,
    TickingAreaLoadStatus: TickingAreaLoadStatusPacket,
    DimensionData: DimensionDataPacket,
    AgentActionEvent: AgentActionEventPacket,
    ChangeMobProperty: ChangeMobPropertyPacket,
    LessonProgress: LessonProgressPacket,
    RequestAbility: RequestAbilityPacket,
    RequestPermissions: RequestPermissionsPacket,
    ToastRequest: ToastRequestPacket,
    UpdateAbilities: UpdateAbilitiesPacket,
    UpdateAdventureSettings: UpdateAdventureSettingsPacket,
    DeathInfo: DeathInfoPacket,
    EditorNetwork: EditorNetworkPacket,
    FeatureRegistry: FeatureRegistryPacket,
    ServerStats: ServerStatsPacket,
    RequestNetworkSettings: RequestNetworkSettingsPacket,
    GameTestRequest: GameTestRequestPacket,
    GameTestResults: GameTestResultsPacket,
    UpdateClientInputLocks: UpdateClientInputLocksPacket,
    CameraPresets: CameraPresetsPacket,
    UnlockedRecipes: UnlockedRecipesPacket,
    CameraInstruction: CameraInstructionPacket,
    CompressedBiomeDefinitionList: CompressedBiomeDefinitionListPacket,
    TrimData: TrimDataPacket,
    OpenSign: OpenSignPacket,
    AgentAnimation: AgentAnimationPacket,
    RefreshEntitlements: RefreshEntitlementsPacket,
    PlayerToggleCrafterSlotRequest: PlayerToggleCrafterSlotRequestPacket,
    SetPlayerInventoryOptions: SetPlayerInventoryOptionsPacket,
    SetHud: SetHudPacket,
    ItemRegistry: ItemRegistryPacket,
    ClientCameraAimAssist: ClientCameraAimAssistPacket,
    ClientMovementPredictionSync: ClientMovementPredictionSyncPacket,
    PlayerUpdateEntityOverrides: PlayerUpdateEntityOverridesPacket,
    PlayerVideoCapture: PlayerVideoCapturePacket,
    UpdateClientOptions: UpdateClientOptionsPacket,
}