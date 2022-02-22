use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use struct_deser::{IntoBytesOrdered, SerializedByteLen, FromBytesOrdered};
use struct_deser_derive::StructDeser;

use crate::{serialize::{Deser, Ser}, error::Error};

#[derive(TryFromPrimitive)]
#[repr(u16)]
pub enum DSPItem {
    Lava = u16::MAX,
    IronOre = 1001,
    CopperOre = 1002,
    SiliconOre = 1003,
    TitaniumOre = 1004,
    Stone = 1005,
    Coal = 1006,
    Log = 1030,
    PlantFuel = 1031,
    FireIce = 1011,
    KimberliteOre = 1012,
    FractalSilicon = 1013,
    OpticalGratingCrystal = 1014,
    SpiniformStalagmiteCrystal = 1015,
    UnipolarMagnet = 1016,
    IronIngot = 1101,
    CopperIngot = 1104,
    HighPuritySilicon = 1105,
    TitaniumIngot = 1106,
    StoneBrick = 1108,
    EnergeticGraphite = 1109,
    Steel = 1103,
    TitaniumAlloy = 1107,
    Glass = 1110,
    TitaniumGlass = 1119,
    Prism = 1111,
    Diamond = 1112,
    CrystalSilicon = 1113,
    Gear = 1201,
    Magnet = 1102,
    MagneticCoil = 1202,
    ElectricMotor = 1203,
    ElectromagneticTurbine = 1204,
    SuperMagneticRing = 1205,
    ParticleContainer = 1206,
    StrangeMatter = 1127,
    CircuitBoard = 1301,
    Processor = 1303,
    QuantumChip = 1305,
    MicrocrystallineComponent = 1302,
    PlaneFilter = 1304,
    ParticleBroadband = 1402,
    PlasmaExciter = 1401,
    PhotonCombiner = 1404,
    SolarSail = 1501,
    Water = 1000,
    CrudeOil = 1007,
    RefinedOil = 1114,
    SulfuricAcid = 1116,
    Hydrogen = 1120,
    Deuterium = 1121,
    Antimatter = 1122,
    CriticalPhoton = 1208,
    HydrogenFuelRod = 1801,
    DeuteronFuelRod = 1802,
    AntimatterFuelRod = 1803,
    Plastic = 1115,
    Graphene = 1123,
    CarbonNanotube = 1124,
    OrganicCrystal = 1117,
    TitaniumCrystal = 1118,
    CasimirCrystal = 1126,
    GravitonLens = 1209,
    SpaceWarper = 1210,
    AnnihilationConstraintSphere = 1403,
    Thruster = 1405,
    ReinforcedThruster = 1406,
    LogisticsDrone = 5001,
    LogisticsVessel = 5002,
    FrameMaterial = 1125,
    DysonSphereComponent = 1502,
    SmallCarrierRocket = 1503,
    Foundation = 1131,
    AccelerantMkI = 1141,
    AccelerantMkII = 1142,
    AccelerantMkIII = 1143,
    ConveyorBeltMKI = 2001,
    ConveyorBeltMKII = 2002,
    ConveyorBeltMKIII = 2003,
    SorterMKI = 2011,
    SorterMKII = 2012,
    SorterMKIII = 2013,
    Splitter = 2020,
    StorageMKI = 2101,
    StorageMKII = 2102,
    StorageTank = 2106,
    AssemblingMachineMkI = 2303,
    AssemblingMachineMkII = 2304,
    AssemblingMachineMkIII = 2305,
    PlaneSmelter = 2315,
    TeslaTower = 2201,
    WirelessPowerTower = 2202,
    SatelliteSubstation = 2212,
    WindTurbine = 2203,
    ThermalPowerStation = 2204,
    MiniFusionPowerStation = 2211,
    MiningMachine = 2301,
    Smelter = 2302,
    OilExtractor = 2307,
    OilRefinery = 2308,
    WaterPump = 2306,
    ChemicalPlant = 2309,
    Fractionator = 2314,
    SprayCoater = 2313,
    SolarPanel = 2205,
    Accumulator = 2206,
    AccumulatorFull = 2207,
    EMRailEjector = 2311,
    RayReceiver = 2208,
    VerticalLaunchingSilo = 2312,
    EnergyExchanger = 2209,
    MiniatureParticleCollider = 2310,
    ArtificialStar = 2210,
    PlanetaryLogisticsStation = 2103,
    InterstellarLogisticsStation = 2104,
    OrbitalCollector = 2105,
    MatrixLab = 2901,
    ElectromagneticMatrix = 6001,
    EnergyMatrix = 6002,
    StructureMatrix = 6003,
    InformationMatrix = 6004,
    GravityMatrix = 6005,
    UniverseMatrix = 6006,
}

// Because StructDeser isn't perfect
#[derive(Serialize, Deserialize)]
struct F32(f32);

impl SerializedByteLen for F32 {
    const BYTE_LEN: usize = 4;
}

impl FromBytesOrdered for F32 {
    fn from_bytes<BO: struct_deser::byteorder::ByteOrder>(bytes: &[u8]) -> Self {
        Self(BO::read_f32(bytes))
    }
}

impl IntoBytesOrdered for F32 {
    fn into_bytes<BO: struct_deser::byteorder::ByteOrder>(&self, bytes: &mut [u8]) {
        BO::write_f32(bytes, self.0);
    }
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Header {
    #[le] version: u32,
    #[le] cursor_offset_x: u32,
    #[le] cursor_offset_y: u32,
    #[le] cursor_target_area: u32,
    #[le] dragbox_size_x: u32,
    #[le] dragbox_size_y: u32,
    #[le] primary_area_index: u32,
    area_count: u8,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Area {
    index: i8,
    parent_index: i8,
    #[le] tropic_anchor: u16,
    #[le] area_segments: u16,
    #[le] anchor_local_offset_x: u16,
    #[le] anchor_local_offset_y: u16,
    #[le] width: u16,
    #[le] height: u16,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct BuildingHeader {
    #[le] index: u32,
    area_index: i8,
    #[le] local_offset_x: F32,
    #[le] local_offset_y: F32,
    #[le] local_offset_z: F32,
    #[le] local_offset_x2: F32,
    #[le] local_offset_y2: F32,
    #[le] local_offset_z2: F32,
    #[le] yaw: F32,
    #[le] yaw2: F32,
    #[le] item_id: u16,
    #[le] model_index: u16,
    #[le] output_object_index: u32,
    #[le] input_object_index: u32,
    output_to_slot: i8,
    input_from_slot: i8,
    output_from_slot: i8,
    input_to_slot: i8,
    output_offset: i8,
    input_offset: i8,
    #[le] recipe_id: u16,
    #[le] filter_id: u16,
    #[le] parameter_count: u16,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationHeader {
    #[le] work_energy: u32,
    #[le] drone_range: u32,
    #[le] vessel_range: u32,
    #[le] orbital_collector: u32,
    #[le] warp_distance: u32,
    #[le] equip_warper: u32,
    #[le] drone_count: u32,
    #[le] vessel_count: u32,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationSlots {
    #[le] direction: u32,
    #[le] storage_index: u32
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationStorage {
    #[le] item_id: u32,
    #[le] local_logic: u32,
    #[le] remote_logic: u32,
    #[le] max_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Station {
    header: StationHeader,
    is_interstellar: bool,
    storage: Vec<StationStorage>,
    slots: Vec<StationSlots>,
}

impl Station {
    const HEADER_OFFSET: usize = 128;
    const SLOTS_OFFSET: usize = 192;

    fn from_bp(d: &mut Deser, is_interstellar: bool, struct_len: usize) -> anyhow::Result<Self> {

        let slots_len = 12;
        let storage_len = if is_interstellar { 3 } else { 5 };

        let mut storage = vec![];
        let mut slots = vec![];

        let start_len = d.len();
        let header_off = start_len - Self::HEADER_OFFSET;
        let slot_off = start_len - Self::SLOTS_OFFSET;
        let end_off = start_len - struct_len;

        for _ in 0..storage_len {
            storage.push(d.read_type()?);
        }
        d.skip(d.len() - header_off)?;

        let header = d.read_type()?;

        d.skip(d.len() - slot_off)?;
        for _ in 0..slots_len {
            slots.push(d.read_type()?);
        }

        let end_len = d.len();
        if end_len < end_off {
            return Err(Error::E(format!("Unexpected station data length {} at read", struct_len)).into());
        }
        d.skip(end_len - end_off)?;

        Ok(Self {
            header,
            is_interstellar,
            storage,
            slots
        })
    }

    fn to_bp(&self, s: &mut Ser, struct_len: usize) -> anyhow::Result<()> {
        let len = s.len();
        let header_off = len + Self::HEADER_OFFSET;
        let slot_off = len + Self::SLOTS_OFFSET;
        let struct_off = len + struct_len;

        for sto in &self.storage {
            s.write_type(sto);
        }
        s.pad(header_off - s.len());

        s.write_type(&self.header);
        s.pad(slot_off - s.len());

        for sl in &self.slots {
            s.write_type(sl);
        }

        let end_len = s.len();
        if end_len > struct_off {
            return Err(Error::E(format!("Unexpected station data length {} at write", struct_len)).into());
        }
        s.pad(struct_off - end_len);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    header: BuildingHeader,
    station: Option<Station>,
    params: Vec<u32>,
}

impl BuildingHeader {
    fn has_station(&self) -> bool {
        match DSPItem::try_from(self.item_id) {
            Ok(DSPItem::PlanetaryLogisticsStation) => true,
            Ok(DSPItem::InterstellarLogisticsStation) => true,
            _ => false,
        }
    }
    fn has_interstellar(&self) -> bool {
        match DSPItem::try_from(self.item_id) {
            Ok(DSPItem::InterstellarLogisticsStation) => true,
            _ => false,
        }
    }
}

impl Building {
    fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
        let header: BuildingHeader = d.read_type()?;
        let mut station = None;
        let mut params: Vec<u32> = vec![];
        if header.has_station() {
            station = Some(Station::from_bp(d, header.has_interstellar(), header.parameter_count as usize * 4)?);
        } else {
            params.append(&mut d.skip(header.parameter_count as usize * 4)?
                                     .chunks_exact(4)
                                     .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                                     .collect());
        }
        Ok(Self {
            header,
            station,
            params,
        })
    }

    fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        d.write_type(&self.header);
        if self.station.is_some() {
            self.station.as_ref().unwrap().to_bp(d, self.header.parameter_count as usize * 4)?;
        } else {
            let le32_vec: Vec<u8> = self.params
                .iter()
                .flat_map(|b| b.to_le_bytes().into_iter())
                .collect();
            d.append(&le32_vec);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize,StructDeser)]
pub struct BuildingCount(#[le] u32);

#[derive(Serialize, Deserialize)]
pub struct BlueprintData {
    header: Header,
    areas: Vec<Area>,
    building_count: BuildingCount,
    buildings: Vec<Building>,
}

impl BlueprintData {
    fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
        let header: Header = d.read_type()?;
        if header.version != 1 {
            return Err(Error::E(format!("Expected blueprint version 1, got {}", header.version)).into());
        }
        let mut areas = vec![];
        let mut buildings = vec![];
        for _ in 0..header.area_count {
            areas.push(d.read_type()?);
        }
        let building_count: BuildingCount = d.read_type()?;
        for _ in 0..building_count.0 {
            buildings.push(Building::from_bp(d)?);
        }
        Ok(Self {
            header,
            areas,
            building_count,
            buildings,
        })
    }

    fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        d.write_type(&self.header);
        for a in &self.areas {
            d.write_type(a);
        }
        d.write_type(&self.building_count);
        for b in &self.buildings {
            b.to_bp(d)?;
        }
        Ok(())
    }

    pub fn new_from_buf(b: &[u8]) -> anyhow::Result<Self> {
        Self::from_bp(&mut Deser::new(b))
    }

    pub fn write(&self) -> anyhow::Result<Vec<u8>> {
        let mut w = Ser::new();
        self.to_bp(&mut w)?;
        Ok(w.data())
    }
}
