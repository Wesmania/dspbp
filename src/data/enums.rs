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
    //This comment is used to automatically regenerate enums, do not remove!
    //DSPItem enum start
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
    GratingCrystal = 1014,
    StalagmiteCrystal = 1015,
    UnipolarMagnet = 1016,
    IronIngot = 1101,
    CopperIngot = 1104,
    HighpuritySilicon = 1105,
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
    SupermagneticRing = 1205,
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
    StrangeAnnihilationFuelRod = 1804,
    Plastic = 1115,
    Graphene = 1123,
    CarbonNanotube = 1124,
    OrganicCrystal = 1117,
    TitaniumCrystal = 1118,
    CasimirCrystal = 1126,
    CombustibleUnit = 1128,
    ExplosiveUnit = 1129,
    CrystalExplosiveUnit = 1130,
    GravitonLens = 1209,
    SpaceWarper = 1210,
    AnnihilationConstraintSphere = 1403,
    Engine = 1407,
    Thruster = 1405,
    ReinforcedThruster = 1406,
    LogisticsBot = 5003,
    LogisticsDrone = 5001,
    InterstellarLogisticsVessel = 5002,
    FrameMaterial = 1125,
    DysonSphereComponent = 1502,
    SmallCarrierRocket = 1503,
    Foundation = 1131,
    ProliferatorMkI = 1141,
    ProliferatorMkII = 1142,
    ProliferatorMkIII = 1143,
    MagnumAmmoBox = 1601,
    TitaniumAmmoBox = 1602,
    SuperalloyAmmoBox = 1603,
    ShellSet = 1604,
    HighExplosiveShellSet = 1605,
    CrystalShellSet = 1606,
    PlasmaCapsule = 1607,
    AntimatterCapsule = 1608,
    MissileSet = 1609,
    SupersonicMissileSet = 1610,
    GravityMissileSet = 1611,
    Prototype = 5101,
    PrecisionDrone = 5102,
    AttackDrone = 5103,
    Corvette = 5111,
    Destroyer = 5112,
    DarkFogMatrix = 5201,
    SiliconbasedNeuron = 5202,
    MatterRecombinator = 5203,
    NegentropySingularity = 5204,
    CoreElement = 5205,
    EnergyShard = 5206,
    ConveyorBeltMKI = 2001,
    ConveyorBeltMKII = 2002,
    ConveyorBeltMKIII = 2003,
    SorterMKI = 2011,
    SorterMKII = 2012,
    SorterMKIII = 2013,
    Splitter = 2020,
    AutomaticPiler = 2040,
    TrafficMonitor = 2030,
    SprayCoater = 2313,
    LogisticsDistributor = 2107,
    DepotMKI = 2101,
    DepotMKII = 2102,
    StorageTank = 2106,
    AssemblingMachineMkI = 2303,
    AssemblingMachineMkII = 2304,
    AssemblingMachineMkIII = 2305,
    RecomposingAssembler = 2318,
    TeslaTower = 2201,
    WirelessPowerTower = 2202,
    SatelliteSubstation = 2212,
    WindTurbine = 2203,
    ThermalPowerPlant = 2204,
    MiniFusionPowerPlant = 2211,
    GeothermalPowerStation = 2213,
    MiningMachine = 2301,
    AdvancedMiningMachine = 2316,
    WaterPump = 2306,
    ArcSmelter = 2302,
    PlaneSmelter = 2315,
    NegentropySmelter = 2319,
    OilExtractor = 2307,
    OilRefinery = 2308,
    ChemicalPlant = 2309,
    QuantumChemicalPlant = 2317,
    Fractionator = 2314,
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
    SelfevolutionLab = 2902,
    GaussTurret = 3001,
    LaserTurret = 3002,
    ImplosionCannon = 3003,
    PlasmaTurret = 3004,
    MissileTurret = 3005,
    JammerTower = 3006,
    SignalTower = 3007,
    PlanetaryShieldGenerator = 3008,
    BattlefieldAnalysisBase = 3009,
    ElectromagneticMatrix = 6001,
    EnergyMatrix = 6002,
    StructureMatrix = 6003,
    InformationMatrix = 6004,
    GravityMatrix = 6005,
    UniverseMatrix = 6006,
    SoilPile = 1099,
    //DSPItem enum end
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
    //DSPRecipe enum start
    IronIngot = 1,
    Magnet = 2,
    CopperIngot = 3,
    StoneBrick = 4,
    Gear = 5,
    MagneticCoil = 6,
    WindTurbine = 7,
    TeslaTower = 8,
    ElectromagneticMatrix = 9,
    MatrixLab = 10,
    Prism = 11,
    PlasmaExciter = 12,
    WirelessPowerTower = 13,
    OilExtractor = 14,
    OilRefinery = 15,
    PlasmaRefining = 16,
    EnergeticGraphite = 17,
    EnergyMatrix = 18,
    HydrogenFuelRod = 19,
    Thruster = 20,
    ReinforcedThruster = 21,
    ChemicalPlant = 22,
    Plastic = 23,
    SulfuricAcid = 24,
    OrganicCrystal = 25,
    TitaniumCrystal = 26,
    StructureMatrix = 27,
    CasimirCrystal = 28,
    CasimirCrystalAdvanced = 29,
    TitaniumGlass = 30,
    Graphene = 31,
    GrapheneAdvanced = 32,
    CarbonNanotube = 33,
    SiliconOre = 34,
    CarbonNanotubeAdvanced = 35,
    ParticleBroadband = 36,
    CrystalSilicon = 37,
    PlaneFilter = 38,
    MiniatureParticleCollider = 39,
    Deuterium = 40,
    DeuteronFuelRod = 41,
    AnnihilationConstraintSphere = 42,
    ArtificialStar = 43,
    AntimatterFuelRod = 44,
    AssemblingMachineMkI = 45,
    AssemblingMachineMkII = 46,
    AssemblingMachineMkIII = 47,
    MiningMachine = 48,
    WaterPump = 49,
    CircuitBoard = 50,
    Processor = 51,
    QuantumChip = 52,
    MicrocrystallineComponent = 53,
    OrganicCrystalOriginal = 54,
    InformationMatrix = 55,
    ArcSmelter = 56,
    Glass = 57,
    XrayCracking = 58,
    HighpuritySilicon = 59,
    Diamond = 60,
    DiamondAdvanced = 61,
    CrystalSiliconAdvanced = 62,
    Steel = 63,
    ThermalPowerPlant = 64,
    TitaniumIngot = 65,
    TitaniumAlloy = 66,
    SolarPanel = 67,
    PhotonCombiner = 68,
    PhotonCombinerAdvanced = 69,
    SolarSail = 70,
    EMRailEjector = 71,
    RayReceiver = 72,
    SatelliteSubstation = 73,
    MassenergyStorage = 74,
    UniverseMatrix = 75,
    Accumulator = 76,
    EnergyExchanger = 77,
    SpaceWarper = 78,
    SpaceWarperAdvanced = 79,
    FrameMaterial = 80,
    DysonSphereComponent = 81,
    VerticalLaunchingSilo = 82,
    SmallCarrierRocket = 83,
    ConveyorBeltMKI = 84,
    SorterMKI = 85,
    DepotMKI = 86,
    Splitter = 87,
    SorterMKII = 88,
    ConveyorBeltMKII = 89,
    SorterMKIII = 90,
    DepotMKII = 91,
    ConveyorBeltMKIII = 92,
    PlanetaryLogisticsStation = 93,
    LogisticsDrone = 94,
    InterstellarLogisticsStation = 95,
    InterstellarLogisticsVessel = 96,
    ElectricMotor = 97,
    ElectromagneticTurbine = 98,
    ParticleContainer = 99,
    ParticleContainerAdvanced = 100,
    GravitonLens = 101,
    GravityMatrix = 102,
    SupermagneticRing = 103,
    StrangeMatter = 104,
    ProliferatorMkI = 106,
    ProliferatorMkII = 107,
    ProliferatorMkIII = 108,
    SprayCoater = 109,
    Fractionator = 110,
    OrbitalCollector = 111,
    Foundation = 112,
    MiniFusionPowerPlant = 113,
    StorageTank = 114,
    DeuteriumFractionation = 115,
    PlaneSmelter = 116,
    TrafficMonitor = 117,
    GeothermalPowerStation = 118,
    AdvancedMiningMachine = 119,
    AutomaticPiler = 120,
    ReformedRefinement = 121,
    LogisticsDistributor = 122,
    LogisticsBot = 123,
    QuantumChemicalPlant = 124,
    GaussTurret = 125,
    LaserTurret = 126,
    ImplosionCannon = 127,
    PlasmaTurret = 128,
    MissileTurret = 129,
    JammerTower = 130,
    SignalTower = 131,
    PlanetaryShieldGenerator = 132,
    CombustibleUnit = 133,
    ExplosiveUnit = 134,
    CrystalExplosiveUnit = 135,
    MagnumAmmoBox = 136,
    TitaniumAmmoBox = 137,
    SuperalloyAmmoBox = 138,
    ShellSet = 139,
    HighExplosiveShellSet = 140,
    CrystalShellSet = 141,
    PlasmaCapsule = 142,
    AntimatterCapsule = 143,
    MissileSet = 144,
    SupersonicMissileSet = 145,
    GravityMissileSet = 146,
    Prototype = 147,
    PrecisionDrone = 148,
    AttackDrone = 149,
    Corvette = 150,
    Destroyer = 151,
    Engine = 105,
    BattlefieldAnalysisBase = 152,
    SelfevolutionLab = 153,
    RecomposingAssembler = 154,
    NegentropySmelter = 155,
    StrangeAnnihilationFuelRod = 156,
    //DSPRecipe enum end
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
