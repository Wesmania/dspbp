use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{collections::HashMap, fmt::Debug};
use strum::{AsRefStr, EnumIter, EnumString};

#[cfg(feature = "python")]
use pyo3::prelude::*;

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

    fn recipes(&self) -> Option<Vec<Recipe>> {
        RECIPE_OUTPUT_MAP.get(self).map(|v| (*v).clone())
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
    ReformingRefine = 121,
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

impl Into<u32> for DSPIcon {
    fn into(self) -> u32 {
        match self {
            Self::Signal(v) => v,
            Self::Item(v) => v.into(),
            Self::Recipe(v) => {
                let v: u16 = v.into();
                v as u32 + 20000
            }
            Self::Tech(v) => v + 40000,
            Self::Unknown(v) => v,
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
            _ => anyhow::bail!("Building {:?} has no BP model", i),
        };
        Ok(o)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingClass {
    Assembler,
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
            _ => Self::Other,
        }
    }
}

#[cfg(feature = "python")]
#[derive(Clone)]
pub struct RecipeItem {
    #[pyo3(get)]
    pub item: DSPItem,
    #[pyo3(get)]
    pub count: usize,
}

#[cfg(not(feature = "python"))]
#[derive(Clone)]
pub struct RecipeItem {
    pub item: DSPItem,
    pub count: usize,
}

impl RecipeItem {
    pub const fn new(item: DSPItem, count: usize) -> Self {
        Self { item, count }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub enum DSPRecipeType {
    Assemble,
    Smelt,
    Chemical,
    Fractionate,
    Particle,
    Refine,
    Research,
}

#[cfg(feature = "python")]
#[derive(Clone)]
#[pyo3::pyclass]
pub struct Recipe {
    #[pyo3(get)]
    // FIXME these don't work with cfg_attr, hence duplication. See https://github.com/PyO3/pyo3/issues/1003
    pub name: DSPRecipe,
    #[pyo3(get)]
    pub type_: DSPRecipeType,
    #[pyo3(get)]
    pub inputs: Vec<RecipeItem>,
    #[pyo3(get)]
    pub outputs: Vec<RecipeItem>,
    #[pyo3(get)]
    pub build_time: f32,
}

#[cfg(not(feature = "python"))]
#[derive(Clone)]
pub struct Recipe {
    pub name: DSPRecipe,
    pub type_: DSPRecipeType,
    pub inputs: Vec<RecipeItem>,
    pub outputs: Vec<RecipeItem>,
    pub build_time: f32,
}

lazy_static! {
    static ref RECIPES: Vec<Recipe> = vec![
        Recipe {
            name: DSPRecipe::Gear,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::IronIngot, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Gear, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::MagneticCoil,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Magnet, 2),
                RecipeItem::new(DSPItem::CopperIngot, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MagneticCoil, 2)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::Prism,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::Glass, 3)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Prism, 2)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::PlasmaExciter,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::MagneticCoil, 4),
                RecipeItem::new(DSPItem::Prism, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PlasmaExciter, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::HydrogenFuelRod,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumIngot, 1),
                RecipeItem::new(DSPItem::Hydrogen, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::HydrogenFuelRod, 2)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::Thruster,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 2),
                RecipeItem::new(DSPItem::CopperIngot, 3),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Thruster, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::ReinforcedThruster,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 5),
                RecipeItem::new(DSPItem::ElectromagneticTurbine, 5),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ReinforcedThruster, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::TitaniumCrystal,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::OrganicCrystal, 1),
                RecipeItem::new(DSPItem::TitaniumIngot, 3),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::TitaniumCrystal, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::CasimirCrystal,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumCrystal, 1),
                RecipeItem::new(DSPItem::Graphene, 2),
                RecipeItem::new(DSPItem::Hydrogen, 12),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::CasimirCrystal, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::CasimirCrystalAdvanced,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::OpticalGratingCrystal, 8),
                RecipeItem::new(DSPItem::Graphene, 2),
                RecipeItem::new(DSPItem::Hydrogen, 12),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::CasimirCrystal, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::TitaniumGlass,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Glass, 2),
                RecipeItem::new(DSPItem::TitaniumIngot, 2),
                RecipeItem::new(DSPItem::Water, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::TitaniumGlass, 2)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::ParticleBroadband,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::CarbonNanotube, 2),
                RecipeItem::new(DSPItem::CrystalSilicon, 2),
                RecipeItem::new(DSPItem::Plastic, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ParticleBroadband, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::PlaneFilter,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::CasimirCrystal, 1),
                RecipeItem::new(DSPItem::TitaniumGlass, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PlaneFilter, 1)].to_vec(),
            build_time: 12.0,
        },
        Recipe {
            name: DSPRecipe::DeuteronFuelRod,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 1),
                RecipeItem::new(DSPItem::Deuterium, 20),
                RecipeItem::new(DSPItem::SuperMagneticRing, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::DeuteronFuelRod, 2)].to_vec(),
            build_time: 12.0,
        },
        Recipe {
            name: DSPRecipe::AnnihilationConstraintSphere,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ParticleContainer, 1),
                RecipeItem::new(DSPItem::Processor, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AnnihilationConstraintSphere, 1)].to_vec(),
            build_time: 20.0,
        },
        Recipe {
            name: DSPRecipe::AntimatterFuelRod,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Antimatter, 12),
                RecipeItem::new(DSPItem::Hydrogen, 12),
                RecipeItem::new(DSPItem::AnnihilationConstraintSphere, 1),
                RecipeItem::new(DSPItem::TitaniumAlloy, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AntimatterFuelRod, 2)].to_vec(),
            build_time: 24.0,
        },
        Recipe {
            name: DSPRecipe::CircuitBoard,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 2),
                RecipeItem::new(DSPItem::CopperIngot, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::CircuitBoard, 2)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::Processor,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::CircuitBoard, 2),
                RecipeItem::new(DSPItem::MicrocrystallineComponent, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Processor, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::QuantumChip,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Processor, 2),
                RecipeItem::new(DSPItem::PlaneFilter, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::QuantumChip, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::MicrocrystallineComponent,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::HighPuritySilicon, 2),
                RecipeItem::new(DSPItem::CopperIngot, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MicrocrystallineComponent, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::OrganicCrystalOriginal,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Log, 20),
                RecipeItem::new(DSPItem::PlantFuel, 30),
                RecipeItem::new(DSPItem::Water, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::OrganicCrystal, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::CrystalSiliconAdvanced,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::FractalSilicon, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::CrystalSilicon, 2)].to_vec(),
            build_time: 1.5,
        },
        Recipe {
            name: DSPRecipe::PhotonCombiner,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Prism, 2),
                RecipeItem::new(DSPItem::CircuitBoard, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PhotonCombiner, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::PhotonCombinerAdvanced,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::OpticalGratingCrystal, 1),
                RecipeItem::new(DSPItem::CircuitBoard, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PhotonCombiner, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::SolarSail,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Graphene, 1),
                RecipeItem::new(DSPItem::PhotonCombiner, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SolarSail, 2)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::SpaceWarper,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::GravitonLens, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::SpaceWarper, 1)].to_vec(),
            build_time: 10.0,
        },
        Recipe {
            name: DSPRecipe::SpaceWarperAdvanced,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::GravityMatrix, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::SpaceWarper, 8)].to_vec(),
            build_time: 10.0,
        },
        Recipe {
            name: DSPRecipe::FrameMaterial,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::CarbonNanotube, 4),
                RecipeItem::new(DSPItem::TitaniumAlloy, 1),
                RecipeItem::new(DSPItem::HighPuritySilicon, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::FrameMaterial, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::DysonSphereComponent,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::FrameMaterial, 3),
                RecipeItem::new(DSPItem::SolarSail, 3),
                RecipeItem::new(DSPItem::Processor, 3),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::DysonSphereComponent, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::SmallCarrierRocket,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::DysonSphereComponent, 2),
                RecipeItem::new(DSPItem::DeuteronFuelRod, 4),
                RecipeItem::new(DSPItem::QuantumChip, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SmallCarrierRocket, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::LogisticsDrone,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 5),
                RecipeItem::new(DSPItem::Processor, 2),
                RecipeItem::new(DSPItem::Thruster, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::LogisticsDrone, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::LogisticsVessel,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 10),
                RecipeItem::new(DSPItem::Processor, 10),
                RecipeItem::new(DSPItem::ReinforcedThruster, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::LogisticsVessel, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::ElectricMotor,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 2),
                RecipeItem::new(DSPItem::Gear, 1),
                RecipeItem::new(DSPItem::MagneticCoil, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ElectricMotor, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::ElectromagneticTurbine,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ElectricMotor, 2),
                RecipeItem::new(DSPItem::MagneticCoil, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ElectromagneticTurbine, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::ParticleContainer,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ElectromagneticTurbine, 2),
                RecipeItem::new(DSPItem::CopperIngot, 2),
                RecipeItem::new(DSPItem::Graphene, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ParticleContainer, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::ParticleContainerAdvanced,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::UnipolarMagnet, 10),
                RecipeItem::new(DSPItem::CopperIngot, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ParticleContainer, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::GravitonLens,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Diamond, 4),
                RecipeItem::new(DSPItem::StrangeMatter, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::GravitonLens, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::SuperMagneticRing,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ElectromagneticTurbine, 2),
                RecipeItem::new(DSPItem::Magnet, 3),
                RecipeItem::new(DSPItem::EnergeticGraphite, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SuperMagneticRing, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::ProliferatorMkI,
            type_: DSPRecipeType::Assemble,
            inputs: [RecipeItem::new(DSPItem::Coal, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::ProliferatorMkI, 1)].to_vec(),
            build_time: 0.5,
        },
        Recipe {
            name: DSPRecipe::ProliferatorMkII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ProliferatorMkI, 2),
                RecipeItem::new(DSPItem::Diamond, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ProliferatorMkII, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::ProliferatorMkIII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ProliferatorMkII, 2),
                RecipeItem::new(DSPItem::CarbonNanotube, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ProliferatorMkIII, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::Foundation,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::StoneBrick, 3),
                RecipeItem::new(DSPItem::Steel, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Foundation, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::Plastic,
            type_: DSPRecipeType::Chemical,
            inputs: [
                RecipeItem::new(DSPItem::RefinedOil, 2),
                RecipeItem::new(DSPItem::EnergeticGraphite, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Plastic, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::SulfuricAcid,
            type_: DSPRecipeType::Chemical,
            inputs: [
                RecipeItem::new(DSPItem::RefinedOil, 6),
                RecipeItem::new(DSPItem::Stone, 8),
                RecipeItem::new(DSPItem::Water, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SulfuricAcid, 4)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::OrganicCrystal,
            type_: DSPRecipeType::Chemical,
            inputs: [
                RecipeItem::new(DSPItem::Plastic, 2),
                RecipeItem::new(DSPItem::RefinedOil, 1),
                RecipeItem::new(DSPItem::Water, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::OrganicCrystal, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::Graphene,
            type_: DSPRecipeType::Chemical,
            inputs: [
                RecipeItem::new(DSPItem::EnergeticGraphite, 3),
                RecipeItem::new(DSPItem::SulfuricAcid, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Graphene, 2)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::GrapheneAdvanced,
            type_: DSPRecipeType::Chemical,
            inputs: [RecipeItem::new(DSPItem::FireIce, 2)].to_vec(),
            outputs: [
                RecipeItem::new(DSPItem::Graphene, 2),
                RecipeItem::new(DSPItem::Hydrogen, 1),
            ]
            .to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::CarbonNanotube,
            type_: DSPRecipeType::Chemical,
            inputs: [
                RecipeItem::new(DSPItem::Graphene, 3),
                RecipeItem::new(DSPItem::TitaniumIngot, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::CarbonNanotube, 2)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::CarbonNanotubeAdvanced,
            type_: DSPRecipeType::Chemical,
            inputs: [RecipeItem::new(DSPItem::SpiniformStalagmiteCrystal, 6)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::CarbonNanotube, 2)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::DeuteriumFractionation,
            type_: DSPRecipeType::Fractionate,
            inputs: [RecipeItem::new(DSPItem::Hydrogen, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Deuterium, 1),].to_vec(),
            build_time: 0.017,
        },
        Recipe {
            name: DSPRecipe::Deuterium,
            type_: DSPRecipeType::Particle,
            inputs: [RecipeItem::new(DSPItem::Hydrogen, 10)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Deuterium, 5)].to_vec(),
            build_time: 2.5,
        },
        Recipe {
            name: DSPRecipe::MassEnergyStorage,
            type_: DSPRecipeType::Particle,
            inputs: [RecipeItem::new(DSPItem::CriticalPhoton, 2)].to_vec(),
            outputs: [
                RecipeItem::new(DSPItem::Antimatter, 2),
                RecipeItem::new(DSPItem::Hydrogen, 2),
            ]
            .to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::StrangeMatter,
            type_: DSPRecipeType::Particle,
            inputs: [
                RecipeItem::new(DSPItem::ParticleContainer, 2),
                RecipeItem::new(DSPItem::IronIngot, 2),
                RecipeItem::new(DSPItem::Deuterium, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::StrangeMatter, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::PlasmaRefining,
            type_: DSPRecipeType::Refine,
            inputs: [RecipeItem::new(DSPItem::CrudeOil, 2)].to_vec(),
            outputs: [
                RecipeItem::new(DSPItem::Hydrogen, 1),
                RecipeItem::new(DSPItem::RefinedOil, 2),
            ]
            .to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::XRayCracking,
            type_: DSPRecipeType::Refine,
            inputs: [
                RecipeItem::new(DSPItem::RefinedOil, 1),
                RecipeItem::new(DSPItem::Hydrogen, 2),
            ]
            .to_vec(),
            outputs: [
                RecipeItem::new(DSPItem::Hydrogen, 3),
                RecipeItem::new(DSPItem::EnergeticGraphite, 1),
            ]
            .to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::ReformingRefine,
            type_: DSPRecipeType::Refine,
            inputs: [
                RecipeItem::new(DSPItem::RefinedOil, 2),
                RecipeItem::new(DSPItem::Hydrogen, 1),
                RecipeItem::new(DSPItem::Coal, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::RefinedOil, 3)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::ElectromagneticMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::MagneticCoil, 1),
                RecipeItem::new(DSPItem::CircuitBoard, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ElectromagneticMatrix, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::EnergyMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::EnergeticGraphite, 2),
                RecipeItem::new(DSPItem::Hydrogen, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::EnergyMatrix, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::StructureMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::Diamond, 1),
                RecipeItem::new(DSPItem::TitaniumCrystal, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::StructureMatrix, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::InformationMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::Processor, 2),
                RecipeItem::new(DSPItem::ParticleBroadband, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::InformationMatrix, 1)].to_vec(),
            build_time: 10.0,
        },
        Recipe {
            name: DSPRecipe::GravityMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::GravitonLens, 1),
                RecipeItem::new(DSPItem::QuantumChip, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::GravityMatrix, 2)].to_vec(),
            build_time: 24.0,
        },
        Recipe {
            name: DSPRecipe::UniverseMatrix,
            type_: DSPRecipeType::Research,
            inputs: [
                RecipeItem::new(DSPItem::ElectromagneticMatrix, 1),
                RecipeItem::new(DSPItem::EnergyMatrix, 1),
                RecipeItem::new(DSPItem::StructureMatrix, 1),
                RecipeItem::new(DSPItem::InformationMatrix, 1),
                RecipeItem::new(DSPItem::GravityMatrix, 1),
                RecipeItem::new(DSPItem::Antimatter, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::UniverseMatrix, 1)].to_vec(),
            build_time: 15.0,
        },
        Recipe {
            name: DSPRecipe::IronIngot,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::IronOre, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::IronIngot, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::Magnet,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::IronOre, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Magnet, 1)].to_vec(),
            build_time: 1.5,
        },
        Recipe {
            name: DSPRecipe::CopperIngot,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::CopperOre, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::CopperIngot, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::StoneBrick,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::Stone, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::StoneBrick, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::EnergeticGraphite,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::Coal, 2)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::EnergeticGraphite, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::SiliconOre,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::Stone, 10)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::SiliconOre, 1)].to_vec(),
            build_time: 10.0,
        },
        Recipe {
            name: DSPRecipe::CrystalSilicon,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::HighPuritySilicon, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::CrystalSilicon, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::Glass,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::Stone, 2)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Glass, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::HighPuritySilicon,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::SiliconOre, 2)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::HighPuritySilicon, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::Diamond,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::EnergeticGraphite, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Diamond, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::DiamondAdvanced,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::KimberliteOre, 1)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Diamond, 2)].to_vec(),
            build_time: 1.5,
        },
        Recipe {
            name: DSPRecipe::Steel,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::IronIngot, 3)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::Steel, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::TitaniumIngot,
            type_: DSPRecipeType::Smelt,
            inputs: [RecipeItem::new(DSPItem::TitaniumOre, 2)].to_vec(),
            outputs: [RecipeItem::new(DSPItem::TitaniumIngot, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::TitaniumAlloy,
            type_: DSPRecipeType::Smelt,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumIngot, 4),
                RecipeItem::new(DSPItem::Steel, 4),
                RecipeItem::new(DSPItem::SulfuricAcid, 8),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::TitaniumAlloy, 4)].to_vec(),
            build_time: 12.0,
        },
        Recipe {
            name: DSPRecipe::WindTurbine,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 6),
                RecipeItem::new(DSPItem::Gear, 1),
                RecipeItem::new(DSPItem::MagneticCoil, 3),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::WindTurbine, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::TeslaTower,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 2),
                RecipeItem::new(DSPItem::MagneticCoil, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::TeslaTower, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::MatrixLab,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 8),
                RecipeItem::new(DSPItem::Glass, 4),
                RecipeItem::new(DSPItem::CircuitBoard, 4),
                RecipeItem::new(DSPItem::MagneticCoil, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MatrixLab, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::WirelessPowerTower,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TeslaTower, 1),
                RecipeItem::new(DSPItem::PlasmaExciter, 3),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::WirelessPowerTower, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::OilExtractor,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 12),
                RecipeItem::new(DSPItem::StoneBrick, 12),
                RecipeItem::new(DSPItem::CircuitBoard, 6),
                RecipeItem::new(DSPItem::PlasmaExciter, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::OilExtractor, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::OilRefinery,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 10),
                RecipeItem::new(DSPItem::StoneBrick, 10),
                RecipeItem::new(DSPItem::CircuitBoard, 6),
                RecipeItem::new(DSPItem::PlasmaExciter, 6),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::OilRefinery, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::ChemicalPlant,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 8),
                RecipeItem::new(DSPItem::StoneBrick, 8),
                RecipeItem::new(DSPItem::Glass, 8),
                RecipeItem::new(DSPItem::CircuitBoard, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ChemicalPlant, 1)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::MiniatureParticleCollider,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 20),
                RecipeItem::new(DSPItem::FrameMaterial, 20),
                RecipeItem::new(DSPItem::SuperMagneticRing, 25),
                RecipeItem::new(DSPItem::Graphene, 10),
                RecipeItem::new(DSPItem::Processor, 8),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MiniatureParticleCollider, 1)].to_vec(),
            build_time: 15.0,
        },
        Recipe {
            name: DSPRecipe::ArtificialStar,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 20),
                RecipeItem::new(DSPItem::FrameMaterial, 20),
                RecipeItem::new(DSPItem::AnnihilationConstraintSphere, 10),
                RecipeItem::new(DSPItem::QuantumChip, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ArtificialStar, 1)].to_vec(),
            build_time: 30.0,
        },
        Recipe {
            name: DSPRecipe::AssemblingMachineMkI,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 4),
                RecipeItem::new(DSPItem::Gear, 8),
                RecipeItem::new(DSPItem::CircuitBoard, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AssemblingMachineMkI, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::AssemblingMachineMkII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::AssemblingMachineMkI, 1),
                RecipeItem::new(DSPItem::Graphene, 8),
                RecipeItem::new(DSPItem::Processor, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AssemblingMachineMkII, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::AssemblingMachineMkIII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::AssemblingMachineMkII, 1),
                RecipeItem::new(DSPItem::ParticleBroadband, 8),
                RecipeItem::new(DSPItem::QuantumChip, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AssemblingMachineMkIII, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::MiningMachine,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 4),
                RecipeItem::new(DSPItem::CircuitBoard, 2),
                RecipeItem::new(DSPItem::MagneticCoil, 2),
                RecipeItem::new(DSPItem::Gear, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MiningMachine, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::WaterPump,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 8),
                RecipeItem::new(DSPItem::StoneBrick, 4),
                RecipeItem::new(DSPItem::ElectricMotor, 4),
                RecipeItem::new(DSPItem::CircuitBoard, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::WaterPump, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::ArcSmelter,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 4),
                RecipeItem::new(DSPItem::StoneBrick, 2),
                RecipeItem::new(DSPItem::CircuitBoard, 4),
                RecipeItem::new(DSPItem::MagneticCoil, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ArcSmelter, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::ThermalPowerPlant,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 10),
                RecipeItem::new(DSPItem::StoneBrick, 4),
                RecipeItem::new(DSPItem::Gear, 4),
                RecipeItem::new(DSPItem::MagneticCoil, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ThermalPowerStation, 1)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::SolarPanel,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::CopperIngot, 10),
                RecipeItem::new(DSPItem::HighPuritySilicon, 10),
                RecipeItem::new(DSPItem::CircuitBoard, 5),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SolarPanel, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::EMRailEjector,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 20),
                RecipeItem::new(DSPItem::Gear, 20),
                RecipeItem::new(DSPItem::Processor, 5),
                RecipeItem::new(DSPItem::SuperMagneticRing, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::EMRailEjector, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::RayReceiver,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 20),
                RecipeItem::new(DSPItem::HighPuritySilicon, 20),
                RecipeItem::new(DSPItem::PhotonCombiner, 10),
                RecipeItem::new(DSPItem::Processor, 5),
                RecipeItem::new(DSPItem::SuperMagneticRing, 20),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::RayReceiver, 1)].to_vec(),
            build_time: 8.0,
        },
        Recipe {
            name: DSPRecipe::SatelliteSubstation,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::WirelessPowerTower, 1),
                RecipeItem::new(DSPItem::SuperMagneticRing, 10),
                RecipeItem::new(DSPItem::FrameMaterial, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SatelliteSubstation, 1)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::Accumulator,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 6),
                RecipeItem::new(DSPItem::SuperMagneticRing, 1),
                RecipeItem::new(DSPItem::CrystalSilicon, 6),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Accumulator, 1)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::EnergyExchanger,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 40),
                RecipeItem::new(DSPItem::Steel, 40),
                RecipeItem::new(DSPItem::Processor, 40),
                RecipeItem::new(DSPItem::ParticleContainer, 8),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::EnergyExchanger, 1)].to_vec(),
            build_time: 15.0,
        },
        Recipe {
            name: DSPRecipe::VerticalLaunchingSilo,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 80),
                RecipeItem::new(DSPItem::FrameMaterial, 30),
                RecipeItem::new(DSPItem::GravitonLens, 20),
                RecipeItem::new(DSPItem::QuantumChip, 10),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::VerticalLaunchingSilo, 1)].to_vec(),
            build_time: 30.0,
        },
        Recipe {
            name: DSPRecipe::ConveyorBeltMKI,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 2),
                RecipeItem::new(DSPItem::Gear, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ConveyorBeltMKI, 3)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::ConveyorBeltMKII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ConveyorBeltMKI, 3),
                RecipeItem::new(DSPItem::ElectromagneticTurbine, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ConveyorBeltMKII, 3)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::ConveyorBeltMKIII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ConveyorBeltMKII, 3),
                RecipeItem::new(DSPItem::SuperMagneticRing, 1),
                RecipeItem::new(DSPItem::Graphene, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::ConveyorBeltMKIII, 3)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::SorterMKI,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 1),
                RecipeItem::new(DSPItem::CircuitBoard, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SorterMKI, 1)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::SorterMKII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::SorterMKI, 2),
                RecipeItem::new(DSPItem::ElectricMotor, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SorterMKII, 2)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::SorterMKIII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::SorterMKII, 2),
                RecipeItem::new(DSPItem::ElectromagneticTurbine, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SorterMKIII, 2)].to_vec(),
            build_time: 1.0,
        },
        Recipe {
            name: DSPRecipe::StorageMKI,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 4),
                RecipeItem::new(DSPItem::StoneBrick, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::StorageMKI, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::StorageMKII,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 8),
                RecipeItem::new(DSPItem::StoneBrick, 8),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::StorageMKII, 1)].to_vec(),
            build_time: 4.0,
        },
        Recipe {
            name: DSPRecipe::Splitter,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 3),
                RecipeItem::new(DSPItem::Gear, 2),
                RecipeItem::new(DSPItem::CircuitBoard, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Splitter, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::PlanetaryLogisticsStation,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 40),
                RecipeItem::new(DSPItem::TitaniumIngot, 40),
                RecipeItem::new(DSPItem::Processor, 40),
                RecipeItem::new(DSPItem::ParticleContainer, 20),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PlanetaryLogisticsStation, 1)].to_vec(),
            build_time: 20.0,
        },
        Recipe {
            name: DSPRecipe::InterstellarLogisticsStation,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::PlanetaryLogisticsStation, 1),
                RecipeItem::new(DSPItem::TitaniumAlloy, 40),
                RecipeItem::new(DSPItem::ParticleContainer, 20),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::InterstellarLogisticsStation, 1)].to_vec(),
            build_time: 30.0,
        },
        Recipe {
            name: DSPRecipe::SprayCoater,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 4),
                RecipeItem::new(DSPItem::PlasmaExciter, 2),
                RecipeItem::new(DSPItem::CircuitBoard, 2),
                RecipeItem::new(DSPItem::MicrocrystallineComponent, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::SprayCoater, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::Fractionator,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 8),
                RecipeItem::new(DSPItem::StoneBrick, 4),
                RecipeItem::new(DSPItem::Glass, 4),
                RecipeItem::new(DSPItem::Processor, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::Fractionator, 1)].to_vec(),
            build_time: 3.0,
        },
        Recipe {
            name: DSPRecipe::OrbitalCollector,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::InterstellarLogisticsStation, 1),
                RecipeItem::new(DSPItem::SuperMagneticRing, 50),
                RecipeItem::new(DSPItem::ReinforcedThruster, 20),
                RecipeItem::new(DSPItem::AccumulatorFull, 20),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::OrbitalCollector, 1)].to_vec(),
            build_time: 30.0,
        },
        Recipe {
            name: DSPRecipe::MiniFusionPowerPlant,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 12),
                RecipeItem::new(DSPItem::SuperMagneticRing, 10),
                RecipeItem::new(DSPItem::CarbonNanotube, 8),
                RecipeItem::new(DSPItem::Processor, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::MiniFusionPowerStation, 1)].to_vec(),
            build_time: 10.0,
        },
        Recipe {
            name: DSPRecipe::StorageTank,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 8),
                RecipeItem::new(DSPItem::StoneBrick, 4),
                RecipeItem::new(DSPItem::Glass, 4),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::StorageTank, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::PlaneSmelter,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::ArcSmelter, 1),
                RecipeItem::new(DSPItem::FrameMaterial, 5),
                RecipeItem::new(DSPItem::PlaneFilter, 4),
                RecipeItem::new(DSPItem::UnipolarMagnet, 15),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::PlaneSmelter, 1)].to_vec(),
            build_time: 5.0,
        },
        Recipe {
            name: DSPRecipe::TrafficMonitor,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::IronIngot, 3),
                RecipeItem::new(DSPItem::Gear, 2),
                RecipeItem::new(DSPItem::Glass, 1),
                RecipeItem::new(DSPItem::CircuitBoard, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::TrafficMonitor, 1)].to_vec(),
            build_time: 2.0,
        },
        Recipe {
            name: DSPRecipe::GeothermalPowerStation,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 15),
                RecipeItem::new(DSPItem::CopperIngot, 20),
                RecipeItem::new(DSPItem::PhotonCombiner, 4),
                RecipeItem::new(DSPItem::SuperMagneticRing, 1),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::GeothermalPowerStation, 1)].to_vec(),
            build_time: 6.0,
        },
        Recipe {
            name: DSPRecipe::AdvancedMiningMachine,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::TitaniumAlloy, 20),
                RecipeItem::new(DSPItem::FrameMaterial, 10),
                RecipeItem::new(DSPItem::SuperMagneticRing, 10),
                RecipeItem::new(DSPItem::QuantumChip, 4),
                RecipeItem::new(DSPItem::OpticalGratingCrystal, 40),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AdvancedMiningMachine, 1)].to_vec(),
            build_time: 20.0,
        },
        Recipe {
            name: DSPRecipe::AutomaticPiler,
            type_: DSPRecipeType::Assemble,
            inputs: [
                RecipeItem::new(DSPItem::Steel, 3),
                RecipeItem::new(DSPItem::Gear, 4),
                RecipeItem::new(DSPItem::SuperMagneticRing, 1),
                RecipeItem::new(DSPItem::Processor, 2),
            ]
            .to_vec(),
            outputs: [RecipeItem::new(DSPItem::AutomaticPiler, 1)].to_vec(),
            build_time: 4.0,
        },
    ];
    static ref RECIPE_OUTPUT_MAP: HashMap<DSPItem, Vec<Recipe>> = {
        let mut m = HashMap::new();
        for item in &(*RECIPES)[..] {
            for output in &item.outputs[..] {
                let entry = match m.get_mut(&output.item) {
                    Some(e) => e,
                    None => {
                        m.insert(output.item, Vec::new());
                        m.get_mut(&output.item).unwrap()
                    }
                };
                entry.push((*item).clone());
            }
        }
        m
    };
}
