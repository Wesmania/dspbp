use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Debug;
use strum::{AsRefStr, EnumIter, EnumString};

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(
    TryFromPrimitive,
    IntoPrimitive,
    EnumString,
    EnumIter,
    AsRefStr,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Debug,
)]
#[repr(u16)]
pub enum DSPItem {
    IronOre = 1001,
    CopperOre = 1002,
    Stone = 1005,
    Coal = 1006,
    SiliconOre = 1003,
    TitaniumOre = 1004,
    Water = 1000,
    CrudeOil = 1007,
    Hydrogen = 1120,
    Deuterium = 1121,
    Antimatter = 1122,
    KimberliteOre = 1012,

    IronIngot = 1101,
    CopperIngot = 1104,
    StoneBrick = 1108,
    EnergeticGraphite = 1109,
    HighPuritySilicon = 1105,
    TitaniumIngot = 1106,
    SulfuricAcid = 1116,
    RefinedOil = 1114,
    HydrogenFuelRod = 1801,
    DeuteronFuelRod = 1802,
    AntimatterFuelRod = 1803,
    FractalSilicon = 1013,

    Magnet = 1102,
    MagneticCoil = 1202,
    Glass = 1110,
    Diamond = 1112,
    CrystalSilicon = 1113,
    TitaniumAlloy = 1107,
    FireIce = 1011,
    Plastic = 1115,
    OrganicCrystal = 1117,
    Graphene = 1123,
    Thruster = 1405,
    OpticalGratingCrystal = 1014,

    Steel = 1103,
    CircuitBoard = 1301,
    Prism = 1111,
    ElectricMotor = 1203,
    MicrocrystallineComponent = 1302,
    ProliferatorMkI = 1141,
    CasimirCrystal = 1126,
    StrangeMatter = 1127,
    TitaniumCrystal = 1118,
    CarbonNanotube = 1124,
    ReinforcedThruster = 1406,
    SpiniformStalagmiteCrystal = 1015,

    Gear = 1201,
    PlasmaExciter = 1401,
    PhotonCombiner = 1404,
    ElectromagneticTurbine = 1204,
    Processor = 1303,
    ProliferatorMkII = 1142,
    AnnihilationConstraintSphere = 1403,
    TitaniumGlass = 1119,
    ParticleBroadband = 1402,
    LogisticsDrone = 5001,
    UnipolarMagnet = 1016,

    Foundation = 1131,
    CriticalPhoton = 1208,
    ParticleContainer = 1206,
    SuperMagneticRing = 1205,
    GravitonLens = 1209,
    ProliferatorMkIII = 1143,
    SpaceWarper = 1210,
    PlaneFilter = 1304,
    QuantumChip = 1305,
    LogisticsVessel = 5002,
    Log = 1030,

    ElectromagneticMatrix = 6001,
    EnergyMatrix = 6002,
    StructureMatrix = 6003,
    InformationMatrix = 6004,
    GravityMatrix = 6005,
    UniverseMatrix = 6006,
    SolarSail = 1501,
    FrameMaterial = 1125,
    DysonSphereComponent = 1502,
    SmallCarrierRocket = 1503,
    PlantFuel = 1031,

    TeslaTower = 2201,
    WirelessPowerTower = 2202,
    SatelliteSubstation = 2212,
    WindTurbine = 2203,
    ThermalPowerStation = 2204,
    SolarPanel = 2205,
    GeothermalPowerStation = 2213,
    MiniFusionPowerStation = 2211,
    EnergyExchanger = 2209,
    RayReceiver = 2208,
    ArtificialStar = 2210,

    ConveyorBeltMKI = 2001,
    ConveyorBeltMKII = 2002,
    ConveyorBeltMKIII = 2003,
    Splitter = 2020,
    AutomaticPiler = 2040,
    TrafficMonitor = 2030,
    StorageMKI = 2101,
    StorageMKII = 2102,
    StorageTank = 2106,
    PlanetaryLogisticsStation = 2103,
    InterstellarLogisticsStation = 2104,
    OrbitalCollector = 2105,

    SorterMKI = 2011,
    SorterMKII = 2012,
    SorterMKIII = 2013,
    MiningMachine = 2301,
    AdvancedMiningMachine = 2316,
    WaterPump = 2306,
    OilExtractor = 2307,
    OilRefinery = 2308,
    MiniatureParticleCollider = 2310,
    EMRailEjector = 2311,
    VerticalLaunchingSilo = 2312,

    AssemblingMachineMkI = 2303,
    AssemblingMachineMkII = 2304,
    AssemblingMachineMkIII = 2305,
    ArcSmelter = 2302,
    PlaneSmelter = 2315,
    SprayCoater = 2313,
    Fractionator = 2314,
    ChemicalPlant = 2309,
    MatrixLab = 2901,
    Accumulator = 2206,
    AccumulatorFull = 2207,
}

impl DSPItem {
    pub fn is_belt(&self) -> bool {
        [
            Self::ConveyorBeltMKI,
            Self::ConveyorBeltMKII,
            Self::ConveyorBeltMKIII,
        ]
        .contains(self)
    }

    pub fn is_station(&self) -> bool {
        [
            Self::PlanetaryLogisticsStation,
            Self::InterstellarLogisticsStation,
        ]
        .contains(self)
    }

    pub fn is_interstellar_station(&self) -> bool {
        [Self::InterstellarLogisticsStation].contains(self)
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DSPItem {
    fn __hash__(&self) -> isize {
        *self as u16 as isize
    }
}

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(
    TryFromPrimitive,
    IntoPrimitive,
    EnumString,
    EnumIter,
    AsRefStr,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Debug,
)]
#[repr(u16)]
pub enum DSPRecipe {
    Gear = 5,
    MagneticCoil = 6,
    Prism = 11,
    PlasmaExciter = 12,
    HydrogenFuelRod = 19,
    Thruster = 20,
    ReinforcedThruster = 21,
    TitaniumCrystal = 26,
    CasimirCrystal = 28,
    CasimirCrystalAdvanced = 29,
    TitaniumGlass = 30,
    ParticleBroadband = 36,
    PlaneFilter = 38,
    DeuteronFuelRod = 41,
    AnnihilationConstraintSphere = 42,
    AntimatterFuelRod = 44,
    CircuitBoard = 50,
    Processor = 51,
    QuantumChip = 52,
    MicrocrystallineComponent = 53,
    OrganicCrystalOriginal = 54,
    CrystalSiliconAdvanced = 62,
    PhotonCombiner = 68,
    PhotonCombinerAdvanced = 69,
    SolarSail = 70,
    SpaceWarper = 78,
    SpaceWarperAdvanced = 79,
    FrameMaterial = 80,
    DysonSphereComponent = 81,
    SmallCarrierRocket = 83,
    LogisticsDrone = 94,
    LogisticsVessel = 96,
    ElectricMotor = 97,
    ElectromagneticTurbine = 98,
    ParticleContainer = 99,
    ParticleContainerAdvanced = 100,
    GravitonLens = 101,
    SuperMagneticRing = 103,
    ProliferatorMkI = 106,
    ProliferatorMkII = 107,
    ProliferatorMkIII = 108,
    Foundation = 112,
    Plastic = 23,
    SulfuricAcid = 24,
    OrganicCrystal = 25,
    Graphene = 31,
    GrapheneAdvanced = 32,
    CarbonNanotube = 33,
    CarbonNanotubeAdvanced = 35,
    DeuteriumFractionation = 115,
    Deuterium = 40,
    MassEnergyStorage = 74,
    StrangeMatter = 104,
    PlasmaRefining = 16,
    XRayCracking = 58,
    ElectromagneticMatrix = 9,
    EnergyMatrix = 18,
    StructureMatrix = 27,
    InformationMatrix = 55,
    GravityMatrix = 102,
    UniverseMatrix = 75,
    IronIngot = 1,
    Magnet = 2,
    CopperIngot = 3,
    StoneBrick = 4,
    EnergeticGraphite = 17,
    SiliconOre = 34,
    CrystalSilicon = 37,
    Glass = 57,
    HighPuritySilicon = 59,
    Diamond = 60,
    DiamondAdvanced = 61,
    Steel = 63,
    TitaniumIngot = 65,
    TitaniumAlloy = 66,
    WindTurbine = 7,
    TeslaTower = 8,
    MatrixLab = 10,
    WirelessPowerTower = 13,
    OilExtractor = 14,
    OilRefinery = 15,
    ChemicalPlant = 22,
    MiniatureParticleCollider = 39,
    ArtificialStar = 43,
    AssemblingMachineMkI = 45,
    AssemblingMachineMkII = 46,
    AssemblingMachineMkIII = 47,
    MiningMachine = 48,
    WaterPump = 49,
    ArcSmelter = 56,
    ThermalPowerPlant = 64,
    SolarPanel = 67,
    EMRailEjector = 71,
    RayReceiver = 72,
    SatelliteSubstation = 73,
    Accumulator = 76,
    EnergyExchanger = 77,
    VerticalLaunchingSilo = 82,
    ConveyorBeltMKI = 84,
    ConveyorBeltMKII = 89,
    ConveyorBeltMKIII = 92,
    SorterMKI = 85,
    SorterMKII = 88,
    SorterMKIII = 90,
    StorageMKI = 86,
    StorageMKII = 91,
    Splitter = 87,
    PlanetaryLogisticsStation = 93,
    InterstellarLogisticsStation = 95,
    SprayCoater = 109,
    Fractionator = 110,
    OrbitalCollector = 111,
    MiniFusionPowerPlant = 113,
    StorageTank = 114,
    PlaneSmelter = 116,
    TrafficMonitor = 117,
    GeothermalPowerStation = 118,
    AdvancedMiningMachine = 119,
    AutomaticPiler = 120,
}

impl DSPRecipe {
    pub fn for_item(item: &DSPItem) -> Option<Self> {
        let foo: &str = item.as_ref();
        Self::try_from(foo).ok()
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl DSPRecipe {
    fn __hash__(&self) -> isize {
        *self as u16 as isize
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum DSPIcon {
    Signal(u32),
    Item(DSPItem),
    Recipe(DSPRecipe),
    Tech(u32),
    Unknown(u32),
}

impl TryFrom<u32> for DSPIcon {
    type Error = anyhow::Error;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        let me = if n < 1000 {
            Self::Signal(n)
        } else if n < 20000 {
            Self::Item(DSPItem::try_from_primitive(n as u16)?)
        } else if n < 40000 {
            Self::Recipe(DSPRecipe::try_from_primitive((n - 20000) as u16)?)
        } else if n < 60000 {
            Self::Tech(n - 40000)
        } else {
            Self::Unknown(n)
        };
        Ok(me)
    }
}

impl From<DSPIcon> for u32 {
    fn from(value: DSPIcon) -> Self {
        match value {
            DSPIcon::Signal(v) => v,
            DSPIcon::Item(v) => v.into(),
            DSPIcon::Recipe(v) => {
                let v: u16 = v.into();
                v as u32 + 20000
            }
            DSPIcon::Tech(v) => v + 40000,
            DSPIcon::Unknown(v) => v,
        }
    }
}

#[derive(TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[repr(u16)]
pub enum BPModel {
    ConveyorBeltMKI = 35,
    ConveyorBeltMKII = 36,
    ConveyorBeltMKIII = 37,
    SorterMKI = 41,
    SorterMKII = 42,
    SorterMKIII = 43,
    AssemblingMachineMkI = 65,
    AssemblingMachineMkII = 66,
    AssemblingMachineMkIII = 67,
    ArcSmelter = 62,
    PlaneSmelter = 194,
    // TODO Some other buildings that we can't upgrade/downgrade. We can grab them from
    // dsp_blueprint_editor at some point.
}

impl BPModel {
    pub fn from_building(i: DSPItem) -> anyhow::Result<Self> {
        let o = match i {
            DSPItem::ConveyorBeltMKI => Self::ConveyorBeltMKI,
            DSPItem::ConveyorBeltMKII => Self::ConveyorBeltMKII,
            DSPItem::ConveyorBeltMKIII => Self::ConveyorBeltMKIII,
            DSPItem::SorterMKI => Self::SorterMKI,
            DSPItem::SorterMKII => Self::SorterMKII,
            DSPItem::SorterMKIII => Self::SorterMKIII,
            DSPItem::AssemblingMachineMkI => Self::AssemblingMachineMkI,
            DSPItem::AssemblingMachineMkII => Self::AssemblingMachineMkII,
            DSPItem::AssemblingMachineMkIII => Self::AssemblingMachineMkIII,
            DSPItem::ArcSmelter => Self::ArcSmelter,
            DSPItem::PlaneSmelter => Self::PlaneSmelter,
            _ => anyhow::bail!("Building {:?} has no BP model", i),
        };
        Ok(o)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingClass {
    Assembler,
    Smelter,
    Belt,
    Sorter,
    Other,
}

impl BuildingClass {
    pub fn replacement_is_valid(i: DSPItem, o: DSPItem) -> bool {
        return Self::from(i) == Self::from(o) && Self::from(i) != Self::Other;
    }
}

impl From<DSPItem> for BuildingClass {
    fn from(i: DSPItem) -> Self {
        match i {
            DSPItem::AssemblingMachineMkI => Self::Assembler,
            DSPItem::AssemblingMachineMkII => Self::Assembler,
            DSPItem::AssemblingMachineMkIII => Self::Assembler,
            DSPItem::SorterMKI => Self::Sorter,
            DSPItem::SorterMKII => Self::Sorter,
            DSPItem::SorterMKIII => Self::Sorter,
            DSPItem::ConveyorBeltMKI => Self::Belt,
            DSPItem::ConveyorBeltMKII => Self::Belt,
            DSPItem::ConveyorBeltMKIII => Self::Belt,
            DSPItem::ArcSmelter => Self::Smelter,
            DSPItem::PlaneSmelter => Self::Smelter,
            _ => Self::Other,
        }
    }
}
