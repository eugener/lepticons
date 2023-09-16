
use base64::*;
use core::fmt;
use flate2::read::ZlibDecoder;
use fmt::Result;
use std::io::prelude::*;
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideIcon {
    Accessibility,
    ActivitySquare,
    Activity,
    AirVent,
    Airplay,
    AlarmCheck,
    AlarmClockOff,
    AlarmClock,
    AlarmMinus,
    AlarmPlus,
    Album,
    AlertCircle,
    AlertOctagon,
    AlertTriangle,
    AlignCenterHorizontal,
    AlignCenterVertical,
    AlignCenter,
    AlignEndHorizontal,
    AlignEndVertical,
    AlignHorizontalDistributeCenter,
    AlignHorizontalDistributeEnd,
    AlignHorizontalDistributeStart,
    AlignHorizontalJustifyCenter,
    AlignHorizontalJustifyEnd,
    AlignHorizontalJustifyStart,
    AlignHorizontalSpaceAround,
    AlignHorizontalSpaceBetween,
    AlignJustify,
    AlignLeft,
    AlignRight,
    AlignStartHorizontal,
    AlignStartVertical,
    AlignVerticalDistributeCenter,
    AlignVerticalDistributeEnd,
    AlignVerticalDistributeStart,
    AlignVerticalJustifyCenter,
    AlignVerticalJustifyEnd,
    AlignVerticalJustifyStart,
    AlignVerticalSpaceAround,
    AlignVerticalSpaceBetween,
    Ampersand,
    Ampersands,
    Anchor,
    Angry,
    Annoyed,
    Antenna,
    Aperture,
    AppWindow,
    Apple,
    ArchiveRestore,
    ArchiveX,
    Archive,
    AreaChart,
    Armchair,
    ArrowBigDownDash,
    ArrowBigDown,
    ArrowBigLeftDash,
    ArrowBigLeft,
    ArrowBigRightDash,
    ArrowBigRight,
    ArrowBigUpDash,
    ArrowBigUp,
    ArrowDown01,
    ArrowDown10,
    ArrowDownAZ,
    ArrowDownCircle,
    ArrowDownFromLine,
    ArrowDownLeftFromCircle,
    ArrowDownLeftSquare,
    ArrowDownLeft,
    ArrowDownNarrowWide,
    ArrowDownRightFromCircle,
    ArrowDownRightSquare,
    ArrowDownRight,
    ArrowDownSquare,
    ArrowDownToDot,
    ArrowDownToLine,
    ArrowDownUp,
    ArrowDownWideNarrow,
    ArrowDownZA,
    ArrowDown,
    ArrowLeftCircle,
    ArrowLeftFromLine,
    ArrowLeftRight,
    ArrowLeftSquare,
    ArrowLeftToLine,
    ArrowLeft,
    ArrowRightCircle,
    ArrowRightFromLine,
    ArrowRightLeft,
    ArrowRightSquare,
    ArrowRightToLine,
    ArrowRight,
    ArrowUp01,
    ArrowUp10,
    ArrowUpAZ,
    ArrowUpCircle,
    ArrowUpDown,
    ArrowUpFromDot,
    ArrowUpFromLine,
    ArrowUpLeftFromCircle,
    ArrowUpLeftSquare,
    ArrowUpLeft,
    ArrowUpNarrowWide,
    ArrowUpRightFromCircle,
    ArrowUpRightSquare,
    ArrowUpRight,
    ArrowUpSquare,
    ArrowUpToLine,
    ArrowUpWideNarrow,
    ArrowUpZA,
    ArrowUp,
    ArrowsUpFromLine,
    Asterisk,
    AtSign,
    Atom,
    Award,
    Axe,
    Axis3D,
    Baby,
    Backpack,
    BadgeAlert,
    BadgeCent,
    BadgeCheck,
    BadgeDollarSign,
    BadgeEuro,
    BadgeHelp,
    BadgeIndianRupee,
    BadgeInfo,
    BadgeJapaneseYen,
    BadgeMinus,
    BadgePercent,
    BadgePlus,
    BadgePoundSterling,
    BadgeRussianRuble,
    BadgeSwissFranc,
    BadgeX,
    Badge,
    BaggageClaim,
    Ban,
    Banana,
    Banknote,
    BarChart2,
    BarChart3,
    BarChart4,
    BarChartBig,
    BarChartHorizontalBig,
    BarChartHorizontal,
    BarChart,
    Baseline,
    Bath,
    BatteryCharging,
    BatteryFull,
    BatteryLow,
    BatteryMedium,
    BatteryWarning,
    Battery,
    Beaker,
    BeanOff,
    Bean,
    BedDouble,
    BedSingle,
    Bed,
    Beef,
    Beer,
    BellDot,
    BellMinus,
    BellOff,
    BellPlus,
    BellRing,
    Bell,
    Bike,
    Binary,
    Biohazard,
    Bird,
    Bitcoin,
    Blinds,
    Blocks,
    BluetoothConnected,
    BluetoothOff,
    BluetoothSearching,
    Bluetooth,
    Bold,
    Bomb,
    Bone,
    BookCopy,
    BookDown,
    BookKey,
    BookLock,
    BookMarked,
    BookMinus,
    BookOpenCheck,
    BookOpen,
    BookPlus,
    BookTemplate,
    BookUp2,
    BookUp,
    BookX,
    Book,
    BookmarkMinus,
    BookmarkPlus,
    Bookmark,
    BoomBox,
    Bot,
    BoxSelect,
    Box,
    Boxes,
    Braces,
    Brackets,
    BrainCircuit,
    BrainCog,
    Brain,
    Briefcase,
    BringToFront,
    Brush,
    BugOff,
    BugPlay,
    Bug,
    Building2,
    Building,
    BusFront,
    Bus,
    CableCar,
    Cable,
    CakeSlice,
    Cake,
    Calculator,
    CalendarCheck2,
    CalendarCheck,
    CalendarClock,
    CalendarDays,
    CalendarHeart,
    CalendarMinus,
    CalendarOff,
    CalendarPlus,
    CalendarRange,
    CalendarSearch,
    CalendarX2,
    CalendarX,
    Calendar,
    CameraOff,
    Camera,
    CandlestickChart,
    CandyCane,
    CandyOff,
    Candy,
    CarFront,
    CarTaxiFront,
    Car,
    Carrot,
    CaseLower,
    CaseSensitive,
    CaseUpper,
    CassetteTape,
    Cast,
    Castle,
    Cat,
    CheckCheck,
    CheckCircle2,
    CheckCircle,
    CheckSquare,
    Check,
    ChefHat,
    Cherry,
    ChevronDownCircle,
    ChevronDownSquare,
    ChevronDown,
    ChevronFirst,
    ChevronLast,
    ChevronLeftCircle,
    ChevronLeftSquare,
    ChevronLeft,
    ChevronRightCircle,
    ChevronRightSquare,
    ChevronRight,
    ChevronUpCircle,
    ChevronUpSquare,
    ChevronUp,
    ChevronsDownUp,
    ChevronsDown,
    ChevronsLeftRight,
    ChevronsLeft,
    ChevronsRightLeft,
    ChevronsRight,
    ChevronsUpDown,
    ChevronsUp,
    Chrome,
    Church,
    CigaretteOff,
    Cigarette,
    CircleDashed,
    CircleDollarSign,
    CircleDotDashed,
    CircleDot,
    CircleEllipsis,
    CircleEqual,
    CircleOff,
    CircleSlash2,
    CircleSlash,
    Circle,
    CircuitBoard,
    Citrus,
    Clapperboard,
    ClipboardCheck,
    ClipboardCopy,
    ClipboardEdit,
    ClipboardList,
    ClipboardPaste,
    ClipboardSignature,
    ClipboardType,
    ClipboardX,
    Clipboard,
    Clock1,
    Clock10,
    Clock11,
    Clock12,
    Clock2,
    Clock3,
    Clock4,
    Clock5,
    Clock6,
    Clock7,
    Clock8,
    Clock9,
    Clock,
    CloudCog,
    CloudDrizzle,
    CloudFog,
    CloudHail,
    CloudLightning,
    CloudMoonRain,
    CloudMoon,
    CloudOff,
    CloudRainWind,
    CloudRain,
    CloudSnow,
    CloudSunRain,
    CloudSun,
    Cloud,
    Cloudy,
    Clover,
    Club,
    Code2,
    Code,
    Codepen,
    Codesandbox,
    Coffee,
    Cog,
    Coins,
    Columns,
    Combine,
    Command,
    Compass,
    Component,
    Computer,
    ConciergeBell,
    Construction,
    Contact2,
    Contact,
    Container,
    Contrast,
    Cookie,
    CopyCheck,
    CopyMinus,
    CopyPlus,
    CopySlash,
    CopyX,
    Copy,
    Copyleft,
    Copyright,
    CornerDownLeft,
    CornerDownRight,
    CornerLeftDown,
    CornerLeftUp,
    CornerRightDown,
    CornerRightUp,
    CornerUpLeft,
    CornerUpRight,
    Cpu,
    CreativeCommons,
    CreditCard,
    Croissant,
    Crop,
    Cross,
    Crosshair,
    Crown,
    CupSoda,
    Currency,
    DatabaseBackup,
    DatabaseZap,
    Database,
    Delete,
    Dessert,
    Diamond,
    Dice1,
    Dice2,
    Dice3,
    Dice4,
    Dice5,
    Dice6,
    Dices,
    Diff,
    Disc2,
    Disc3,
    Disc,
    DivideCircle,
    DivideSquare,
    Divide,
    DnaOff,
    Dna,
    Dog,
    DollarSign,
    Donut,
    DoorClosed,
    DoorOpen,
    Dot,
    DownloadCloud,
    Download,
    Dribbble,
    Droplet,
    Droplets,
    Drumstick,
    Dumbbell,
    EarOff,
    Ear,
    EggFried,
    EggOff,
    Egg,
    EqualNot,
    Equal,
    Eraser,
    Euro,
    Expand,
    ExternalLink,
    EyeOff,
    Eye,
    Facebook,
    Factory,
    Fan,
    FastForward,
    Feather,
    FerrisWheel,
    Figma,
    FileArchive,
    FileAudio2,
    FileAudio,
    FileAxis3D,
    FileBadge2,
    FileBadge,
    FileBarChart2,
    FileBarChart,
    FileBox,
    FileCheck2,
    FileCheck,
    FileClock,
    FileCode2,
    FileCode,
    FileCog,
    FileDiff,
    FileDigit,
    FileDown,
    FileEdit,
    FileHeart,
    FileImage,
    FileInput,
    FileJson2,
    FileJson,
    FileKey2,
    FileKey,
    FileLineChart,
    FileLock2,
    FileLock,
    FileMinus2,
    FileMinus,
    FileOutput,
    FilePieChart,
    FilePlus2,
    FilePlus,
    FileQuestion,
    FileScan,
    FileSearch2,
    FileSearch,
    FileSignature,
    FileSpreadsheet,
    FileStack,
    FileSymlink,
    FileTerminal,
    FileText,
    FileType2,
    FileType,
    FileUp,
    FileVideo2,
    FileVideo,
    FileVolume2,
    FileVolume,
    FileWarning,
    FileX2,
    FileX,
    File,
    Files,
    Film,
    FilterX,
    Filter,
    Fingerprint,
    FishOff,
    FishSymbol,
    Fish,
    FlagOff,
    FlagTriangleLeft,
    FlagTriangleRight,
    Flag,
    Flame,
    FlashlightOff,
    Flashlight,
    FlaskConicalOff,
    FlaskConical,
    FlaskRound,
    FlipHorizontal2,
    FlipHorizontal,
    FlipVertical2,
    FlipVertical,
    Flower2,
    Flower,
    Focus,
    FoldHorizontal,
    FoldVertical,
    FolderArchive,
    FolderCheck,
    FolderClock,
    FolderClosed,
    FolderCog,
    FolderDot,
    FolderDown,
    FolderEdit,
    FolderGit2,
    FolderGit,
    FolderHeart,
    FolderInput,
    FolderKanban,
    FolderKey,
    FolderLock,
    FolderMinus,
    FolderOpenDot,
    FolderOpen,
    FolderOutput,
    FolderPlus,
    FolderRoot,
    FolderSearch2,
    FolderSearch,
    FolderSymlink,
    FolderSync,
    FolderTree,
    FolderUp,
    FolderX,
    Folder,
    Folders,
    Footprints,
    Forklift,
    FormInput,
    Forward,
    Frame,
    Framer,
    Frown,
    Fuel,
    FunctionSquare,
    GalleryHorizontalEnd,
    GalleryHorizontal,
    GalleryThumbnails,
    GalleryVerticalEnd,
    GalleryVertical,
    Gamepad2,
    Gamepad,
    GanttChartSquare,
    GanttChart,
    GaugeCircle,
    Gauge,
    Gavel,
    Gem,
    Ghost,
    Gift,
    GitBranchPlus,
    GitBranch,
    GitCommit,
    GitCompare,
    GitFork,
    GitMerge,
    GitPullRequestClosed,
    GitPullRequestDraft,
    GitPullRequest,
    Github,
    Gitlab,
    GlassWater,
    Glasses,
    Globe2,
    Globe,
    Goal,
    Grab,
    GraduationCap,
    Grape,
    Grid2X2,
    Grid3X3,
    GripHorizontal,
    GripVertical,
    Grip,
    Group,
    Hammer,
    HandMetal,
    Hand,
    HardDriveDownload,
    HardDriveUpload,
    HardDrive,
    HardHat,
    Hash,
    Haze,
    HdmiPort,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Heading,
    Headphones,
    HeartCrack,
    HeartHandshake,
    HeartOff,
    HeartPulse,
    Heart,
    HelpCircle,
    HelpingHand,
    Hexagon,
    Highlighter,
    History,
    Home,
    HopOff,
    Hop,
    Hotel,
    Hourglass,
    IceCream2,
    IceCream,
    ImageMinus,
    ImageOff,
    ImagePlus,
    Image,
    Import,
    Inbox,
    Indent,
    IndianRupee,
    Infinity,
    Info,
    Instagram,
    Italic,
    IterationCcw,
    IterationCw,
    JapaneseYen,
    Joystick,
    KanbanSquareDashed,
    KanbanSquare,
    Kanban,
    KeyRound,
    KeySquare,
    Key,
    Keyboard,
    LampCeiling,
    LampDesk,
    LampFloor,
    LampWallDown,
    LampWallUp,
    Lamp,
    Landmark,
    Languages,
    Laptop2,
    Laptop,
    LassoSelect,
    Lasso,
    Laugh,
    Layers,
    LayoutDashboard,
    LayoutGrid,
    LayoutList,
    LayoutPanelLeft,
    LayoutPanelTop,
    LayoutTemplate,
    Layout,
    Leaf,
    LeafyGreen,
    Library,
    LifeBuoy,
    Ligature,
    LightbulbOff,
    Lightbulb,
    LineChart,
    Link2Off,
    Link2,
    Link,
    Linkedin,
    ListChecks,
    ListEnd,
    ListFilter,
    ListMinus,
    ListMusic,
    ListOrdered,
    ListPlus,
    ListRestart,
    ListStart,
    ListTodo,
    ListTree,
    ListVideo,
    ListX,
    List,
    Loader2,
    Loader,
    LocateFixed,
    LocateOff,
    Locate,
    Lock,
    LogIn,
    LogOut,
    Lollipop,
    Luggage,
    MSquare,
    Magnet,
    MailCheck,
    MailMinus,
    MailOpen,
    MailPlus,
    MailQuestion,
    MailSearch,
    MailWarning,
    MailX,
    Mail,
    Mailbox,
    Mails,
    MapPinOff,
    MapPin,
    Map,
    Martini,
    Maximize2,
    Maximize,
    Medal,
    MegaphoneOff,
    Megaphone,
    Meh,
    MemoryStick,
    MenuSquare,
    Menu,
    Merge,
    MessageCircle,
    MessageSquareDashed,
    MessageSquarePlus,
    MessageSquare,
    MessagesSquare,
    Mic2,
    MicOff,
    Mic,
    Microscope,
    Microwave,
    Milestone,
    MilkOff,
    Milk,
    Minimize2,
    Minimize,
    MinusCircle,
    MinusSquare,
    Minus,
    MonitorCheck,
    MonitorDot,
    MonitorDown,
    MonitorOff,
    MonitorPause,
    MonitorPlay,
    MonitorSmartphone,
    MonitorSpeaker,
    MonitorStop,
    MonitorUp,
    MonitorX,
    Monitor,
    MoonStar,
    Moon,
    MoreHorizontal,
    MoreVertical,
    MountainSnow,
    Mountain,
    MousePointer2,
    MousePointerClick,
    MousePointerSquareDashed,
    MousePointerSquare,
    MousePointer,
    Mouse,
    Move3D,
    MoveDiagonal2,
    MoveDiagonal,
    MoveDownLeft,
    MoveDownRight,
    MoveDown,
    MoveHorizontal,
    MoveLeft,
    MoveRight,
    MoveUpLeft,
    MoveUpRight,
    MoveUp,
    MoveVertical,
    Move,
    Music2,
    Music3,
    Music4,
    Music,
    Navigation2Off,
    Navigation2,
    NavigationOff,
    Navigation,
    Network,
    Newspaper,
    Nfc,
    NutOff,
    Nut,
    Octagon,
    Option,
    Orbit,
    Outdent,
    Package2,
    PackageCheck,
    PackageMinus,
    PackageOpen,
    PackagePlus,
    PackageSearch,
    PackageX,
    Package,
    PaintBucket,
    Paintbrush2,
    Paintbrush,
    Palette,
    Palmtree,
    PanelBottomClose,
    PanelBottomInactive,
    PanelBottomOpen,
    PanelBottom,
    PanelLeftClose,
    PanelLeftInactive,
    PanelLeftOpen,
    PanelLeft,
    PanelRightClose,
    PanelRightInactive,
    PanelRightOpen,
    PanelRight,
    PanelTopClose,
    PanelTopInactive,
    PanelTopOpen,
    PanelTop,
    Paperclip,
    Parentheses,
    ParkingCircleOff,
    ParkingCircle,
    ParkingMeter,
    ParkingSquareOff,
    ParkingSquare,
    PartyPopper,
    PauseCircle,
    PauseOctagon,
    Pause,
    PawPrint,
    PcCase,
    PenLine,
    PenSquare,
    PenTool,
    Pen,
    PencilLine,
    PencilRuler,
    Pencil,
    PercentCircle,
    PercentDiamond,
    PercentSquare,
    Percent,
    PersonStanding,
    PhoneCall,
    PhoneForwarded,
    PhoneIncoming,
    PhoneMissed,
    PhoneOff,
    PhoneOutgoing,
    Phone,
    PiSquare,
    Pi,
    PictureInPicture2,
    PictureInPicture,
    PieChart,
    PiggyBank,
    PilcrowSquare,
    Pilcrow,
    Pill,
    PinOff,
    Pin,
    Pipette,
    Pizza,
    PlaneLanding,
    PlaneTakeoff,
    Plane,
    PlayCircle,
    PlaySquare,
    Play,
    Plug2,
    PlugZap2,
    PlugZap,
    Plug,
    PlusCircle,
    PlusSquare,
    Plus,
    PocketKnife,
    Pocket,
    Podcast,
    Pointer,
    Popcorn,
    Popsicle,
    PoundSterling,
    PowerOff,
    Power,
    Presentation,
    Printer,
    Projector,
    Puzzle,
    QrCode,
    Quote,
    Rabbit,
    Radar,
    Radiation,
    RadioReceiver,
    RadioTower,
    Radio,
    RailSymbol,
    Rainbow,
    Rat,
    Ratio,
    Receipt,
    RectangleHorizontal,
    RectangleVertical,
    Recycle,
    Redo2,
    RedoDot,
    Redo,
    RefreshCcwDot,
    RefreshCcw,
    RefreshCwOff,
    RefreshCw,
    Refrigerator,
    Regex,
    RemoveFormatting,
    Repeat1,
    Repeat2,
    Repeat,
    ReplaceAll,
    Replace,
    ReplyAll,
    Reply,
    Rewind,
    Rocket,
    RockingChair,
    RollerCoaster,
    Rotate3D,
    RotateCcw,
    RotateCw,
    Router,
    Rows,
    Rss,
    Ruler,
    RussianRuble,
    Sailboat,
    Salad,
    Sandwich,
    SatelliteDish,
    Satellite,
    SaveAll,
    Save,
    Scale3D,
    Scale,
    Scaling,
    ScanFace,
    ScanLine,
    Scan,
    ScatterChart,
    School2,
    School,
    ScissorsLineDashed,
    ScissorsSquareDashedBottom,
    ScissorsSquare,
    Scissors,
    ScreenShareOff,
    ScreenShare,
    ScrollText,
    Scroll,
    SearchCheck,
    SearchCode,
    SearchSlash,
    SearchX,
    Search,
    SendHorizontal,
    SendToBack,
    Send,
    SeparatorHorizontal,
    SeparatorVertical,
    ServerCog,
    ServerCrash,
    ServerOff,
    Server,
    Settings2,
    Settings,
    Shapes,
    Share2,
    Share,
    Sheet,
    Shell,
    ShieldAlert,
    ShieldBan,
    ShieldCheck,
    ShieldEllipsis,
    ShieldHalf,
    ShieldMinus,
    ShieldOff,
    ShieldPlus,
    ShieldQuestion,
    ShieldX,
    Shield,
    ShipWheel,
    Ship,
    Shirt,
    ShoppingBag,
    ShoppingBasket,
    ShoppingCart,
    Shovel,
    ShowerHead,
    Shrink,
    Shrub,
    Shuffle,
    SigmaSquare,
    Sigma,
    SignalHigh,
    SignalLow,
    SignalMedium,
    SignalZero,
    Signal,
    Siren,
    SkipBack,
    SkipForward,
    Skull,
    Slack,
    Slash,
    Slice,
    SlidersHorizontal,
    Sliders,
    SmartphoneCharging,
    SmartphoneNfc,
    Smartphone,
    SmilePlus,
    Smile,
    Snail,
    Snowflake,
    Sofa,
    Soup,
    Space,
    Spade,
    Sparkle,
    Sparkles,
    Speaker,
    SpellCheck2,
    SpellCheck,
    Spline,
    SplitSquareHorizontal,
    SplitSquareVertical,
    Split,
    SprayCan,
    Sprout,
    SquareAsterisk,
    SquareCode,
    SquareDashedBottomCode,
    SquareDashedBottom,
    SquareDot,
    SquareEqual,
    SquareSlash,
    SquareStack,
    Square,
    Squirrel,
    Stamp,
    StarHalf,
    StarOff,
    Star,
    StepBack,
    StepForward,
    Stethoscope,
    Sticker,
    StickyNote,
    StopCircle,
    Store,
    StretchHorizontal,
    StretchVertical,
    Strikethrough,
    Subscript,
    Subtitles,
    SunDim,
    SunMedium,
    SunMoon,
    SunSnow,
    Sun,
    Sunrise,
    Sunset,
    Superscript,
    SwissFranc,
    SwitchCamera,
    Sword,
    Swords,
    Syringe,
    Table2,
    TableProperties,
    Table,
    TabletSmartphone,
    Tablet,
    Tablets,
    Tag,
    Tags,
    Tally1,
    Tally2,
    Tally3,
    Tally4,
    Tally5,
    Target,
    Tent,
    TerminalSquare,
    Terminal,
    TestTube2,
    TestTube,
    TestTubes,
    TextCursorInput,
    TextCursor,
    TextQuote,
    TextSelect,
    Text,
    ThermometerSnowflake,
    ThermometerSun,
    Thermometer,
    ThumbsDown,
    ThumbsUp,
    Ticket,
    TimerOff,
    TimerReset,
    Timer,
    ToggleLeft,
    ToggleRight,
    Tornado,
    TouchpadOff,
    Touchpad,
    TowerControl,
    ToyBrick,
    Tractor,
    TrafficCone,
    TrainFrontTunnel,
    TrainFront,
    TrainTrack,
    TramFront,
    Trash2,
    Trash,
    TreeDeciduous,
    TreePine,
    Trees,
    Trello,
    TrendingDown,
    TrendingUp,
    TriangleRight,
    Triangle,
    Trophy,
    Truck,
    Turtle,
    Tv2,
    Tv,
    Twitch,
    Twitter,
    Type,
    Umbrella,
    Underline,
    Undo2,
    UndoDot,
    Undo,
    UnfoldHorizontal,
    UnfoldVertical,
    Ungroup,
    Unlink2,
    Unlink,
    Unlock,
    Unplug,
    UploadCloud,
    Upload,
    Usb,
    User2,
    UserCheck2,
    UserCheck,
    UserCircle2,
    UserCircle,
    UserCog2,
    UserCog,
    UserMinus2,
    UserMinus,
    UserPlus2,
    UserPlus,
    UserSquare2,
    UserSquare,
    UserX2,
    UserX,
    User,
    Users2,
    Users,
    UtensilsCrossed,
    Utensils,
    UtilityPole,
    Variable,
    Vegan,
    VenetianMask,
    VibrateOff,
    Vibrate,
    VideoOff,
    Video,
    Videotape,
    View,
    Voicemail,
    Volume1,
    Volume2,
    VolumeX,
    Volume,
    Vote,
    Wallet2,
    WalletCards,
    Wallet,
    Wallpaper,
    Wand2,
    Wand,
    Warehouse,
    Watch,
    Waves,
    Webcam,
    Webhook,
    WheatOff,
    Wheat,
    WholeWord,
    WifiOff,
    Wifi,
    Wind,
    WineOff,
    Wine,
    Workflow,
    WrapText,
    Wrench,
    XCircle,
    XOctagon,
    XSquare,
    X,
    Youtube,
    ZapOff,
    Zap,
    ZoomIn,
    ZoomOut,
}
const ACCESSIBILITY: &'static str = r#"eJxtzVEKwjAMxvGrfOS90axNWqHdDTzEqIKCggwf9PZ282GDjTyE8P9Bcr2P9XFF/RQSI9RvoUAY20V9Pvxrn1/D+4ZLoackyAniojPMYiqrrkjwzkNZ2+7YGzzrFp4DdwESWAeF4jiPcUqwHSyeY/sX19pN2i36B+E3Mq0="#;

const ACTIVITY_SQUARE: &'static str = r#"eJwdi7EKgDAMRH8lZC+aiOjQdnbxI8QWU3CQElD/XtPluOPe8zXvCpLLIRqQZoQn4IDwtqz/YIS7JJX2Rt+ZEP21qUAKuNIExOL4dAyjY0e9lWUy1KD4AUbJGkQ="#;

const ACTIVITY: &'static str = r#"eJyzKUgsyVBIsVXyNTJSMDTK0DXJ0TVWsPSxVDAGMzyMlOxs9EGK7AD2QAsc"#;

const AIR_VENT: &'static str = r#"eJxtT7sOwjAM/BWrewzn1E6GUImNhZU9EkNGBtTvx1EQZKj8kHxnn87lVd+NnpflbgS5rVVI6OyBIEEe+p/J5wabAZJd5wOSFmTZyqmLbmWSzg1HBDJHQuJUhZV6DaXIMIqcY8e+bTBgrO7rUM3Yf1C267CEXp5pD/rb/gCvNjbM"#;

const AIRPLAY: &'static str = r#"eJxNTb0KgCAYfJXDXer76Gcx55bWdiHIIFRIgt4+tSE5OLjjflQw0WKbxNKDxrkzDEabQJIlr/2vkbSloTbAN7V1A2wlCa2avKpV8Oeze4fgDxevSRCD8g2YUKgYJf8l9Qs3UyJk"#;

const ALARM_CHECK: &'static str = r#"eJxtjUEKwjAQRa/ymX2iMxOTCElv0ENIFBQsSHHR3r4JhXbRrj689+Cn8hnL94UxUySUKRNL3bmuUpcuq+7S7/F/45mpv0Eh8M01tptBKjZq9Kh6bzWCow1wED4JOFjvWuED5HraDHdwuxY44za9AKfuMts="#;

const ALARM_CLOCK_OFF: &'static str = r#"eJxtjjEOgCAMRa/y4061FSsk6A08BImDi4mD94+ARgfJb9/U1zYc8dywTs2i5EZkRAeHDpybSfRmM4c2z87hNdiTB1uS4VFSjCceCv7CLgI1vekru7QccqQjLIRrMlK6VBXbZgnfjxfqOjWs"#;

const ALARM_CLOCK: &'static str = r#"eJxti0EKgzAQRa/ymb1pZyZNUki8QQ9R0kILCiIienujbgRdfXjv/Zj/fW6+6BMFQp4TsZadygrV8bbrOnbv4YdPohcLnqNtBJte8UE+oBC4s2ml4EorvTg5owEcjIeF8EXA3ji7Fs5D7sdmAZNjMvs="#;

const ALARM_MINUS: &'static str = r#"eJxtjUEKhDAQBL/SzN3szoybZCHxBz5CoqCgIOJBf6/Bgwc9dlVBhzQsaeyQ9kishCWSJ6TtXEJV+Fy6CnOz9mgj1T8oBDa7zG4zyYkLLfSpamvUg71xKCH8ErAztsyFdZDve/MHa38fH4LGMgA="#;

const ALARM_PLUS: &'static str = r#"eJx1zUEKgzAQheGrPGZv6szYJIXEG+QQJS1YaKGIiN5egwsX6mrgfT9MyJ8+f9/IUyQWQp7Xq4Q+kqc23DZuw/85dHhFSncoBLZY2Xb5yTpXWumRkjXqwd44NBA+CdgZ25TCOkh90Qi4Hk8+pwdYux0W+YE5Xw=="#;

const ALBUM: &'static str = r#"eJw1i1EKgCAQRK8yeIEwg/xQL1OSgqiYkN4+1wiWfQMzTxV7VDTNBMPjz+o045LBWX+5+uU+yzKwDjSCUQt5RuUUevDRIicf6z0EDoHx6DZI8H3GHYKkf25e4mog9Q=="#;

const ALERT_CIRCLE: &'static str = r#"eJxNyzEOwCAIBdCrEA7QikPTAb0McTAxHZzw9rVFEify+e+z1C6tgIyEFBH6PAFB9I+ZT+szt/oU0GhqUMIbQWnF6PpTbvfywrU9Atl8vty/Hy8h/Q=="#;

const ALERT_OCTAGON: &'static str = r#"eJxFjFEKgCAQRK+y7AGsXcL6UG8TIYgK9ZG3z3TLr2HePMbkFMqRIuTk43VaXNWmgYG0oqUmM3TyIRkGHm4jHaMzkzw7E3zcobBFYoRbspDFrVZqteqvJa5AcdVMXSeN/Ub//gM2Ly3l"#;

const ALERT_TRIANGLE: &'static str = r#"eJxtjDEKgDAUQ68S3H9tfj9aoQoewAu4CQ4dFBzE80sHdZFseclLx3JmrH21K10bwChRaItC4UskOIvwm0TQxreGQZnZfEOUv4S5GlJdpEN61BMV3WX/gG12ni+7AUBZIpk="#;

const ALIGN_CENTER_HORIZONTAL: &'static str = r#"eJxtjysOgDAMQK/S4Atts/BJxjQGi19ATCLIzs+KYIMsNf29vtSe/gpwzM0qwBKEGmc77Tn7TpiA+2i8gAClYEzZ0pc1SkRTR8ctk6SbBUl6KdZAeZz8dQaUv5RrUpOkw07ILbeTgpg+yySoNIM3FO0+Iw=="#;

const ALIGN_CENTER_VERTICAL: &'static str = r#"eJx1j7sOgDAIRX+FuKNAGh9JdXZxdTc6MDoYv1/awaqpYYKbc0/w+3IobH0xsYCcQsXgq3Ab/J20wDS6xXIgG0ZBmeuVkEsuO7Q7iroMyLWRmkiCSKY9NCm6rFNobN7OE+Uj5R+pU04oxMeeVWDShF5EPj45"#;

const ALIGN_CENTER: &'static str = r#"eJxVy0EKwCAMRNGrSC5QktJqIfU2XRTEtd7e6Ijg6i/mjaY/f67wS8LkqvW2yEixnBT16CbqkuyxeUgWHK27HWOAvQjXZ9qwbANo0CC2"#;

const ALIGN_END_HORIZONTAL: &'static str = r#"eJxNjE0KgCAQha8yzAWiQQLBcd2mQ0RK4y5kILt9KoTt3t/3XI6HQmE0CA8jIdwpqDAuCBLTKco4V51LK72b2t67TtW9/ZofVv1sBm0Hde0qEBg3IiBa+1/L/AsDzSXW"#;

const ALIGN_END_VERTICAL: &'static str = r#"eJxtjE0KgCAQha8yvAuEQwSC4w3ato+UdBcyUN0+dVGbdu/3cyVuSinmPalgAp05aBKYKi8Bg27BCCrdeDe0vXf9VRvzVm1t8UOy3+tYNVEQzMzEvHRey/wDEzMl5A=="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: &'static str = r#"eJxtjMsOQDAURH/lZvaCUt30+gNbe0HUTqTx+HuahpR0N5kzZ/Q69pb2ebCGUYHMOE/GMvIStB4MAToZEnTnErVO3b7WP+vwwr1UwUX2XLza0llDA6PJFQmxJdIhV36RakUEeKWIEhkYF9ZAO7g="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_END: &'static str = r#"eJxdjMsNgCAQBVvZvAZE4ufC2oFFGCHCzRCi2L2gkRgue3gzs8qbNdDpdLCMAeQjQ4KscZsNjLYDpSXdi9FjUk32J1VV8TVLJR5//N6VbF+CJc2YW0HykCKTvP2IrMgNjOotgQ=="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_START: &'static str = r#"eJxdjMsNgCAUBFt52QZUgnrh0YFFGCHCzRDip3uBgxCvOzujgt0iXd5Ex5hAN0OCHsYIctbvLjKGtIQEBLTq8l+rYn28L169Nb1Umqt2rNGRYSySxCn6DPLUgOFHXlfeLVc="#;

const ALIGN_HORIZONTAL_JUSTIFY_CENTER: &'static str = r#"eJxFjMsNgCAQRFvZTAMC8XNh6cAijBDhZsjGT/cKJnKZw8y8Z3NYhW7GADqTl8gYQfliGNCXMaQtCkP3cLYrf2cr9Q+qXnUDm+k1Tw3bF4nkGbM2ZA6jylI69wA1/yYo"#;

const ALIGN_HORIZONTAL_JUSTIFY_END: &'static str = r#"eJxNy8sNgCAQBNBWNtOAsPFzYenAIowQ4WYI8dO9YoLhMoeZeSb5NdMtGEBndDkIRlDwcQtZoHtQugQM+tKarvytqWpq1XvR3GBV8c/2JQdygpmZ+GBVltLZBzcZJiU="#;

const ALIGN_HORIZONTAL_JUSTIFY_START: &'static str = r#"eJxty7ENgCAUBNBVfm4BhSg2fDZwCCNE6Az5UdxeodHC5oq7ezaHVSgXhgadyUtkGFBpeTFGUAxpi8JQA5zt6t/Zpp55wq9V5sP6l+2LRPKMWZM+dBtq5W4I2iX7"#;

const ALIGN_HORIZONTAL_SPACE_AROUND: &'static str = r#"eJxVyjEKgDAQRNGrLHMB4yKKkM0NbO3FBDedhAX19prOlDPv+5J2I035UBP0DvQIJlC5BQy6cjQVjKBvzwi+q33w52ZKUbAMxLxyhXr9gF0rLzYyHZ4="#;

const ALIGN_HORIZONTAL_SPACE_BETWEEN: &'static str = r#"eJxVjMkNgCAURFv5mQYUcLnw6cAijBDhZghx6V7QxOBx5s0bHd2S6GL0IO/C6hNDdKB4MiToCDZ5xgDKWcHopuyNfqxciVprUY7G2npvPm2bkyfLmBTJXbYFlKoCUvzJDUyjLVQ="#;

const ALIGN_JUSTIFY: &'static str = r#"eJxlzEEKgDAQA8CvLPmAbATxsO1vPAjiuf7epuKh7SlsmE1c531Y8YQVVphAhz313GpQkWORydGkOudHlP3nYGV2TOts9W9fZVcgpA=="#;

const ALIGN_LEFT: &'static str = r#"eJxdjEsKwCAMBa8ScoGSlH4W0dt0URDXent9fhBcDfOYxMIfP0riWIUpV95MSR2f1RTm7UDjrZXYRFciQ8Fra9v4jOjt/8B+O9sCZlUgrA=="#;

const ALIGN_RIGHT: &'static str = r#"eJxlzEEKACEIheGriBcYdGCmwLpNiyBa1+1TdNfqR/l4MvpssKggE8Ligi/C1vPTsKXKY6aKS/1lF8ROrDFw2z9sCpsuewBoJCCu"#;

const ALIGN_START_HORIZONTAL: &'static str = r#"eJxFzFEKgCAMBuCrjF0gGiIEzudeOkSkNN9CBtXtU0l82tj+73c5HgoPo0HIZRDCy2gR7hRU2iYxnaKMs0Xvppr3rqkRKXA2Xf41nS1DXbsKBMaNCGil+qgn/wHgQCWl"#;

const ALIGN_START_VERTICAL: &'static str = r#"eJxNjFEKgCAQRK+yzAVKiSBwvUGHiJT0L2Qpu31sFPn75r1xJa5CZw6SGBMoxbwlYYygi2EGUKkMC6rKvOvU9+6p6qf9VpO/n6ap9kUSBcZsyR6210GRvwH0OyXR"#;

const ALIGN_VERTICAL_DISTRIBUTE_CENTER: &'static str = r#"eJxtjEEKgCAQRa8yzD5Ky9xo6zYdIiqadiFCdftGqTBw+99737hl8nBZFA2COy1KBFq2lbzFFoEHhXBss6eodKYMQWdi9vgvrqKvfwf8nFT76Almi4OUoKlQgYQtIRp0LzI7F4KTOoMUk15+4Aa3Njt/"#;

const ALIGN_VERTICAL_DISTRIBUTE_END: &'static str = r#"eJxVjMENgCAQBFu5XAMCQf1wdGARRojHz5BL1O4VNEaeu7OzLsdFYE9BmFBbhIOwRzifkO9kEDimlYVwQO+6InjXaKpqY9Xsf/4efNY2C0MgnAwYxUYVUrqG6IZcbustZQ=="#;

const ALIGN_VERTICAL_DISTRIBUTE_START: &'static str = r#"eJxVzMENgCAMheFVmreAQlAvwAYOYYRYboY0UbcXvAiXJu2frzbHXSjfDhrEMR0sDjPoSkHYQRlQaRPo+RZvhwq8bVmZyy/G7k9hjTo3YQoOqyZlWI+11FtXuvACTlQtOw=="#;

const ALIGN_VERTICAL_JUSTIFY_CENTER: &'static str = r#"eJxtzEEKgCAQheGrDO8C6VC2UW/QISKlcRciVLdPhWjT9v18z+a4FTpTKOKgR1C+HBgkMe1SHAyoDhPortnA26EBbzurZf4T753qjD91rEUoOCxMmoVVK23zDzp2Jho="#;

const ALIGN_VERTICAL_JUSTIFY_END: &'static str = r#"eJxVzM0JgDAMhuFVwreAGvy5tN3AIUSL6U1KQLu9bVHEUyBvnpjoV6UzbCoWXQ8SH3ZRixF0WQygmAeDUs4MZ5oCnKksl+knUj19xPu0/dSxqNBmMTMxC9dSdu4GOhAmFw=="#;

const ALIGN_VERTICAL_JUSTIFY_START: &'static str = r#"eJxNjEEKgCAQRa8y/AtkUrZRb9AhIqVxFyJUt2+UqFYDb977Nse1UD4dNIhj2rg4GNDl0MsRPoKOFAoLGOBtVwNvW/bzRZxaZT5d4Rl+q30pTMFh1qRZq/qoyN8RkSXt"#;

const ALIGN_VERTICAL_SPACE_AROUND: &'static str = r#"eJxVyjEKgDAQRNGrLHMB4yKKkE1t4yHEBDedhAX19horrQbmfV/SaqQpb2qCHnQJRtCRo6mgdaBTMIDKM4zgm9oHvy+mFAUzM7GbXqnfX7oP3DUKHYI="#;

const ALIGN_VERTICAL_SPACE_BETWEEN: &'static str = r#"eJxVjEEKgCAQRa8y/AtUlrVRb9AhIqVxFyJUt08Fw1YD/703Krg9Ejt/cNSYQZe3kTWGCXRrSFBIR4CetEkY1eXAqJJVt0f7IflL8ccaf9W5RSarsQoSA4s+k7z9yNiCF2H9LTg="#;

const AMPERSAND: &'static str = r#"eJxFjT0OgCAMha/y4k4FCsiAJB7AQxAcGB28f2xZSNPlfe+nvO0beM7tdgdFON8tAgXDlJBNRr6CyPpWzkEskWK3RnFQC8XG4AlVtVLA8BLIsw+Ot1p2nalljSVBY5EfpI0eCQ=="#;

const AMPERSANDS: &'static str = r#"eJy1jbENgDAMBFex6B2cDyYghWzAEFEoKCnYXziA2ACd/osv/tJRzp22pVu9kI+VlQPHm7mAQGJ4GkiqEJyyvj223UVLoAe4yTZlOFhrl1PfznP6FMBPigsqKSkM"#;

const ANCHOR: &'static str = r#"eJwlirEKwCAMRH8lZC+tEaGDce7SjxArKEgp0kH/vrGScMfdPRtyDSVCZdQIoTEqEu+MBp1d5+psyXeEpubaiXEXk0gSG/210INy9vFvgovxNKDoIK82kJ9HImnRAx6Y+wCBliCu"#;

const ANGRY: &'static str = r#"eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vapbYpcDJ/PezPGzs4uI+zVMJIMzh/BYM/4tiZ/eGu2/pgwNKyjElTunDLFJS9CEBPkIL3UKlOoQQL6D1cqoCH9CiWa2rcmkQCkvuQGQqkzew=="#;

const ANNOYED: &'static str = r#"eJxtyUEKgCAQQNGrDHOBGikwGL1Bh4gpmKBFiAu9vYorwdWH/1jeIN8DkhySQZDcG2pW9Lx09/xfUeF2eFqgXW2jtgY41Ew+bQMUShEgiQ=="#;

const ANTENNA: &'static str = r#"eJx9zLsNwCAMhOFVTvR52Ipx47ABQ0RKQRMpRZT5kQuoEO19ut/e6yu4z5AZxFBwSLb5mKzRo06y0D4w4hlOnvlYBVpIBuTR+McuFZG1LR8="#;

const APERTURE: &'static str = r#"eJxtT0sOhSAMvArpAfqoKJ8EvQxxYWLewhXc/rXgi4S46m9mOhPTcaVzV6msQBOoi4sGlXIdt/hp9y2ex3dXmXg9oyFQhVsPKk8rTBr1whtuyWGYhSbwm/RHcglow80hJNc4fsTLGbVtQDIovkREqqg4NCNHkAbD0r8RHVt1Ktm+vKHn2GerBtGb8UlFou3slBaFMzezGv1j7Qe0GVYM"#;

const APP_WINDOW: &'static str = r#"eJxli0sKgCAURbfyuBtIRaSBuoMWESk9ZyGPPrtPGxVN7oVzOL7mRegKsCDOZWUJ0A50lCQcYBSonu1Bz0Y/9CD6bRamFDBpRXa3XXT0EoZGbvlfuE9wA1XsJM4="#;

const APPLE: &'static str = r#"eJxFjksKAkEMRK9SzD6xUybzgXFAXHsBd8O4cOnC+2O6G5SQBOoDb33vnxeel+FuBIsufpgGCqhTwLSM8P7OKY4y5xqVvLouhnZKG5sQh1QLRWrHXQIUE40x9WkWSgj37NRer2XE07oR5sg4MedJBnaIkM7Q3mPY1lMF3tY/dqYOQzKzTfwyX+slLWo="#;

const ARCHIVE_RESTORE: &'static str = r#"eJxtzEEKgzAQBdCrfP4+NDOpUCHxBj2EVOm4KBQJtr19ExeiIH8xw8zjx3l8ZHymIVuiesLG6Wk5sSF+iYH4ljsxlyHs4qX6Lr77bBgS71fcFpFeofBrymZaYSU7qP4oxVXpTuirhTQILqDkpEkUoku7ff4IcTOm"#;

const ARCHIVE_X: &'static str = r#"eJx1i70OgkAQhF9lMj3Krm7E5I43sLUnQlwKE0Mu/rw9dzTQkC12Mt98YRoeCb9IJab8hPBhfHqKNOI79skzq4l/5IltOJZ9G95dcvSRtzOaj0inUNTL5eSi26LSe1PM4qzm63owyAVW2R5UGFY4Aw0lLFM="#;

const ARCHIVE: &'static str = r#"eJxNizsKhTAURLcyTP94uVcFi8Qd2NqLitdOJPjZvYmFyDTzO36bhogzUIkrsCC2FIQ4ljFaqh1h0zJbDKzY+H/+N37to2EMbEvUu0ivULhHyZnot/hpV2cyMx9SHEStfJcb+dgkIA=="#;

const AREA_CHART: &'static str = r#"eJw9yiEOwCAMQNGrNPhmKdCsouMGswgcCaICgSCcn2Cw/z8ddRq0z/0BwiIxEpf0OTXpfS+QX2zks3RkYIwYy3UblaES0g=="#;

const ARMCHAIR: &'static str = r#"eJxtTrsKgDAM/JXgHjTns1A7u7g6uBUcOjpIvt920RZKhnBPzt7+CXStzS6GzDF5EKhLx2Bsc44J2jfOtini7BfsSUTH3xg/ggw5wVDOHDxQp7FcGSV5VupHkkVREeLgQnkB4E0xNA=="#;

const ARROW_BIG_DOWN_DASH: &'static str = r#"eJxNiiEKACEQAL+y2OU4Tk8WVrPFahcMGwwGMfh6tYhMmxmqqTFkK8KrQXsUjp6tHN0B+8eqSANGLlhF5H+cdQKREBMR"#;

const ARROW_BIG_DOWN: &'static str =
    r#"eJyzKUgsyVBIsVXyNTRVMCszyzDJ0TVXMNcFwgyTMLMMsyolOxt9kCI7AAXWDAM="#;

const ARROW_BIG_LEFT_DASH: &'static str = r#"eJxFiiEKACEQAL+y2JdD1BNhzx9ctQuGDQaDbPD1atEwZWao5c5QPvXrANqloCI920W6xa3CaMRW9OhhIZaNvOPME8ayE9Y="#;

const ARROW_BIG_LEFT: &'static str =
    r#"eJyzKUgsyVBIsVXyNbRQMDTN0DUrM8nRNdc1VwDiMpMMszKzKiU7G32QMjsAG7YMkQ=="#;

const ARROW_BIG_RIGHT_DASH: &'static str = r#"eJw9yisKQCEURdGpXOzywvPDgavZYrULBoPBIAZHrxbDLovNPY9KxYmoCdMIz98Vz89BqH/SzZKVpylVQMJ65wZ4SRKn"#;

const ARROW_BIG_RIGHT: &'static str =
    r#"eJyzKUgsyVBIsVXyNVOwzDALM80xVzDXBeIyXRMPszDLKiU7G32QGjsA+EYLmw=="#;

const ARROW_BIG_UP_DASH: &'static str = r#"eJxNiiEKACEQAL+y2Jfj8DxZWM0WHyEYNhgMYvD1ahGZNjNcUxPITkWCl+RXnp+tPN/BdNTBFIsWFoJf14HGeSelxRM0"#;

const ARROW_BIG_UP: &'static str =
    r#"eJyzKUgsyVBIsVXytVQwtCjTNfMwzTHXNVcAwgxdkzIzD8sqJTsbfZAqOwAMiAwN"#;

const ARROW_DOWN_01: &'static str = r#"eJxtjTsOgzAQRK8ymt5K7Bin8XKDtOmjgLIUkRCy+NwetkBQoOnmzSf3n6JohP8HfEI0ucg63wzUecevJ8L9fQJD+y3QtvtpESZi6pqiwkgMizAQs9BXxGLe1rL8ac7b3uiSunBx5quN6vG2Avn8LhQ="#;

const ARROW_DOWN_10: &'static str = r#"eJxtzDEKgDAMBdCrfLKLVqsubW/g6i4qpoMgpaDe3lQQHEqWkJf/zTFFxmJpb6A66DSFJmfKBM58PPSoqzEHqocS4aLOYSvIv1hY54jbktKE0y+RLcl2yaEl8Oo3jpY6QpCftzAF3APTfC30"#;

const ARROW_DOWN_AZ: &'static str = r#"eJx1jDEOgCAQBL+ysQfvkEML5Ae29CQWV2hi4f8jFMYGM9lsMdmNV7kV+zqcEzjAN4wfUhybSPHV2wxHuSccYVEjHcMCphysFGcFLVRh1M5MPwOvchhB0O/xAQRkKyE="#;

const ARROW_DOWN_CIRCLE: &'static str = r#"eJxFyUEKgDAMBMCvLLmLtvSQQ9of+AiJgoKCFBH9vY2CZQ/L7oguWdcJekdynpBLdQS93pmk/TzJPhwzxki98+CTjeyqsDEKBUsTfn4AHk4aeg=="#;

const ARROW_DOWN_FROM_LINE: &'static str = r#"eJxlyaEKACEQRdFfediXZVycRRjNFqtdMFgEg/+PWDTIbfdIz6OiOBXJ4gtGeXnX8nJAQ1P6b2kMMuDVw5sns+EWXA=="#;

const ARROW_DOWN_LEFT_FROM_CIRCLE: &'static str = r#"eJxtijEKACEQA7+y2MtpiuOKPWsbHyFY2AgW4vvdLbSShBCY4Z5HpfKbBPLI3pFURqPfBH5UCbzFBoK4zt5g+gRGTPsetgC6/RlO"#;

const ARROW_DOWN_LEFT_SQUARE: &'static str = r#"eJxFy7EKgDAMBNBfCdlFqlAytJ1dXN3FFtNBkBJQ/96GDl2O4x7nSjoEyutxQqg5Izw5Cns0hMApnyytf4rBjXoI7t6FIXq8jAUaCEhJx05rJWMX2rr9uKQeuQ=="#;

const ARROW_DOWN_LEFT: &'static str =
    r#"eJxNySEOACAMBMGvNP0AqTpzVGOweBIEEsH/Q6po1u3wzLtlVe0GiQzqLHGd2QwN49sDricPOQ=="#;

const ARROW_DOWN_NARROW_WIDE: &'static str = r#"eJx1yTEOgCAQBMCvbOiNLF7E4uQHtvYmFteYWPD/EAqoIFOO/k82vKf7NnCHVIu4pGuNpK2viODvUZAQm8RhcRwMRt+rAIoRJN8="#;

const ARROW_DOWN_RIGHT_FROM_CIRCLE: &'static str = r#"eJxtiqENACAMBFdp8IR+RVVhA4YgQSARhPmhBkX+8ubOZluDeg4VQiINTJd7Po7gUCx5U+yV4iXB+Vvo1hH1yQP8WBnp"#;

const ARROW_DOWN_RIGHT_SQUARE: &'static str = r#"eJxFy0sKgDAMBNCrDLmAqCBZtF278RBii+lCkBL83F6jixIIwzzGlbQoyuWpI7y/J0jKq6inlglnjip/vA2Da2wQ3D6rIHraGN+ZWFdlagfwwWOlB4DeHm4="#;

const ARROW_DOWN_RIGHT: &'static str =
    r#"eJw9ySEOACAIQNGrMC4gJAqSLR7CzWBxMzjPLwbZfvpPV9sDesYpIMDkoWl61/RbZcfDVCTsArN7D2Q="#;

const ARROW_DOWN_SQUARE: &'static str = r#"eJxFizsOgCAQRK8y2d4YkIICuIGHMEJcChNDNn5uL1BoppjPy7iSVsHtaSKUaprw9MIpbyyelCVcOQr3GNzYDsEdizCip1lp2LODNv1gt6jINA3mwy/TbB7C"#;

const ARROW_DOWN_TO_DOT: &'static str = r#"eJxFylEKgDAMA9CrhP4PaRHGYOsNPIRUQUFBhoje3g2Rka/kJR7juWBKNLBALu5JY1c3jb/sHBCch3cljW3Nts2wOxELwZ5EwoRcaj19rC8OMxqf"#;

const ARROW_DOWN_TO_LINE: &'static str = r#"eJxlyaEKwCAUheFXOdjHOI7dMbgzr1jtgsEiGHx/xKBF/vZ/WmPLSJ/xtOATLuP0HM/plCIgIaNDdvZ8YfnfSzq25haJ"#;

const ARROW_DOWN_UP: &'static str = r#"eJxtyzEKwCAQBdGrfOwlWbNoio03SJs+YGEjWIjnly20kikfI/VvGekx5QJ5sGbZRDkUokx+A9z5baA4wm1Zw+6jAO7klwzAeR6i"#;

const ARROW_DOWN_WIDE_NARROW: &'static str = r#"eJxtzDEKgDAQRNGrDOnFTFyMxZob2NoLFtsIFrk/IUVIs/zywdf/qYb3DN8G7pDeIqHo2qHo4CsjxdsDEmKMvhyWfWCyOWuEkCTf"#;

const ARROW_DOWN_ZA: &'static str = r#"eJxtzDEOgCAMBdCrNOwglBYdkBu4upM4dNDEwXB+y2BcyE/TJi/9+a6PwLGaK0JIQD2WTMlTh5I/3magFtIAAgMJn5YhCY8dfbPRcUWnt47XBNC9ox98oOoi9i97AUmrK5I="#;

const ARROW_DOWN: &'static str =
    r#"eJw9yUEKABAQQNGrTPbSTDSpMTdwCGVhoyzk/FjQ3/0no8wGNZmMBGGhNyruPpUnHSMgWQa2p+8b3MgPwg=="#;

const ARROW_LEFT_CIRCLE: &'static str = r#"eJxFybEKgDAMBNBfObKLJoh0SDu7+BESBQUFKQ7699oKlhuOu6e2Rttm2O2JhRDfagh25Rm0/jzoMZ4LJk8Dd2DpXbL0FdlZ4KoWOT8/Lgcaew=="#;

const ARROW_LEFT_FROM_LINE: &'static str = r#"eJxtySsKQCEURdGpHOyPx/UHwtUZWO2CwSIYnD+ioEl224t7HhXFi+ZgP4udCPyvH/hoVCBZST9EEsglc2UCmzUWfQ=="#;

const ARROW_LEFT_RIGHT: &'static str = r#"eJxtyzsOgCAQhOGrTOiNLm7AYqW28RAmFhSaUHD/8CiggEw3X34JT/R4T3Uf2MGwH4OVk7X8Tppm8WRG+MlAE3ipm4R6A9mrSwLUyx6o"#;

const ARROW_LEFT_SQUARE: &'static str = r#"eJxFy7EKgDAQA9BfCbeLXBXp0Dq7+BFii9dBkHKg/r22gxIIhEdcjqviTEHFE1vC5akj3LXzOwxBYtpEK4+uLYfRHYsKgqedDWzTo6ZwgZ9nHsBmsp884qweww=="#;

const ARROW_LEFT_TO_LINE: &'static str = r#"eJxlycEKABEURuFX+bOfpjsGqcsb2NorCxtl4f0TxUZndz5uqRdkJ4IE2aiE53cuzxsqSehHY3VzMKCv0H9kAKk7Fnw="#;

const ARROW_LEFT: &'static str =
    r#"eJw9ySEKACAMAMCvDLvIBjIGc9niIwSDRTD4f9Sg4dLprKtDS24gAYpnz3A403DH9H1BAaQc/2zIPw+Z"#;

const ARROW_RIGHT_CIRCLE: &'static str = r#"eJxFyUEKgDAMRNGrDNmLJoh0kfYGHkKiUEFBigu9vW0FZRYf5qmtybYFyRN3BLtyJfeuDdq+HvSYzojZ0+jAEl2hcv2ws4AH9E3d5w81hhq1"#;

const ARROW_RIGHT_FROM_LINE: &'static str = r#"eJxlyaEKwCAYReFXudjHuG5uC//MFh9CMFgEg/j8osEip51PSqgJ8Vf+gmm8lZVzLCsLNEHt3l0yDfjhOWbLO9QHFsQ="#;

const ARROW_RIGHT_LEFT: &'static str = r#"eJxtyzsKwCAQRdGtPOyHZMygKSbWabKIQAobIYX7xw9oZXGrw9X/zRHfZRI7HBAI1UzQrUHQwY/d4e8FpBOWqV+0+gTsI7spBbwbHoY="#;

const ARROW_RIGHT_SQUARE: &'static str = r#"eJxFy0EKgDAMBMCvLLmLpIr00PoDHyG2mB4EKQH199oKSi5hZ9fluChOTx3hSEHFE1uCxLSKvv9VMT8dQ6Nry2B0+6yC4GmyYCO2QIl+2NiAB/RNvc9v/1se/Q=="#;

const ARROW_RIGHT_TO_LINE: &'static str = r#"eJxlyTsKwCAURNGtDPYhjPkWL9ZpXIRgYSNYiOsXBW3kdvdIcjnAf8ryAfV/KCN7e0aGRBJ8cW+91a0mrsJzSgXNKRbx"#;

const ARROW_RIGHT: &'static str =
    r#"eJw9ybEJACAMBdFVPvYiESVNzAYOIVikESzcH7XQ4pp7Mtsy9OJqBkWj5FTCfSpPBkVkMNifPm/J3A9l"#;

const ARROW_UP_01: &'static str = r#"eJxtjksKgDAMRK8yZF+0tVYXrTfwEKJiXAgixc/tbQWpCwlkMS+ZGbt2njE4WgrU0EIjDDU2i3pjX9pW0Ls0CWxj73HMg2dHmnA9m8d5Yu/IELagKMLpSJbxK95/7GQFle/CsFA/YbIMlFONG+TXLgo="#;

const ARROW_UP_10: &'static str = r#"eJxtzDsKgEAMBNCrDOlFV+On2N0b2NqLirEQRBY/tzcKgoWkm5cZu7RB0DuaM1TgiKFH3sZ37u2rdQneTPEDpoRJGpYo/cNcUT5769AFnI4ME2SYRgmOCsI+9UEcaXio5YRVf57Bu+AvzPQt6g=="#;

const ARROW_UP_AZ: &'static str = r#"eJx1jDEOgCAUQ6/SuIN85CMDcgNXdhKHP2jiYDy/MBgXTNM06Usbz3IJtmU4JgQ45VA1pDi2PsWXrjPcTb4DrEEQxR1CDDLZay5WM5pNFaFmJvMzcMK7Ynj5Hh/2jSsX"#;

const ARROW_UP_CIRCLE: &'static str = r#"eJxFyjEKgDAQRNGrDNuLbgjBYpMb2NrLKigoSLDQ2+sSMEzxizeiW9Z9gd6R2BH0Kc1fOkrSFk9yTteKOdLBAewab4O3h0n1gR04jP0vLzwtGtI="#;

const ARROW_UP_DOWN: &'static str = r#"eJxtyyEOwCAQBdGr/OBJu3QDFVtuUFvfBIEhQRDODwhQZOTLSP5LRHhUMgSymsG6p7wcQ7xMf8nBnN9G0oV7XOhtPgeuZBc0vRUeog=="#;

const ARROW_UP_FROM_DOT: &'static str = r#"eJxFyrEKgDAMBNBfObKLJKBFaPIHru4SBQUFKQ7697aIyA0H9y4e47lgUtobdAhVQA5ZrMtu8dOeBdwO8ouvybcZfisJE/xSYiGkXOX0sj3cARoy"#;

const ARROW_UP_FROM_LINE: &'static str =
    r#"eJxtybsJACEQBcBWHubHsf5QWO3AIgQDE8FArF8MBAOZcLjnUVGCaOTgP7vBisj/jsinE0moSfoxBpLqNQu2hxbl"#;

const ARROW_UP_LEFT_FROM_CIRCLE: &'static str = r#"eJxlijEKwCAUQ68SvEBrhtLhV+gBXLsXOrgUHLw/RkEXSciQ9yy/JeG7XCTOh+lwwbb2BRvkJwi/qyuLXvDuEBpFKqdXAcBbGGM="#;

const ARROW_UP_LEFT_SQUARE: &'static str = r#"eJxNy0EKgCAQBdCrfOYCYYHMQr1B2/aR0rgLGahun7rJzTD/P74r6VA8nhbC22+pYSbcOap4MkyQlE/R/gc3tUFw166C6GllGLuxdGrlQMZWA+O3D7M9Hqw="#;

const ARROW_UP_LEFT: &'static str =
    r#"eJw9ySEOACAIRuGrMC6gJMovN7Da3QxEg/efUNhr78Pdz+kMnkqiS106G1peQ5kkUlT2AbzUD1I="#;

const ARROW_UP_NARROW_WIDE: &'static str = r#"eJx1yTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lQIqyO/+0/t4DOfmrgUrZBL8uaxz+Vmr7gnyMnaABIPJQKKlvgRv9I0+vWIlMg=="#;

const ARROW_UP_RIGHT_FROM_CIRCLE: &'static str = r#"eJxtirENwCAMBFd5sUDiL1w5SBkgQ0SioKRAzI/dIAr0p2/urP29ojzpIyF85YbjFyOYsl2RZNvDSJ2DFAWrDl1qApTBGP4="#;

const ARROW_UP_RIGHT_SQUARE: &'static str = r#"eJxFyzEKgDAQBMCvLNeLREGuSPIDHyEmeCkECYfG38thkW7Z2fU174o30EyoLdBEkFwO0UCOCU9JKn9stol+tEP016aCFGhlsPDNJtZ1ORluAQ+dPo7fHps="#;

const ARROW_UP_RIGHT: &'static str =
    r#"eJxNybENACAIBdFVCAsIFc2XDRzCxILSwji/0cKQXHUPs6+gUbkZWahsFXaUex3J9GXfDsEAD2s="#;

const ARROW_UP_SQUARE: &'static str = r#"eJxFyzEKgDAMheGrhOwiqVI6tL2Bq7vYYjoIUgLW20s6KG97H7+veRd4Ak4InMvBEpAcQm0BDULrcJck3P/oRw2ivzZhSAFPskBmmHUwq6v8vpABsqv75AUBwR8a"#;

const ARROW_UP_TO_LINE: &'static str = r#"eJxli7sJACEQBVt5mB/H+g9WO7AIwcBEMBDrFwNBkMlmGO55VJQgkoGqpEXkf6vIJzTyIPXZDezbE0m4eZ0LxYoW5A=="#;

const ARROW_UP_WIDE_NARROW: &'static str = r#"eJx1zTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lYLQQKabV3y9j8dwbu5asEImwT+XdS5/1qp7gryMHSDBYPQDipb6Ery1zge34CUy"#;

const ARROW_UP_ZA: &'static str = r#"eJxtjLEOgCAMRH+lYQehtMiA/IGrO4lDB00cDN9vHYwLuTS93MtdudotsC/mjJCBLIHK1DK9eS0fXWegHtIABAYSPixDEh5z9N1Gxw2dej2vCqB/Qz9ooNIs9h97ACo5K2M="#;

const ARROW_UP: &'static str =
    r#"eJw9ySEKACAMAMCvDLvIhDGEuR9Y7YLBIhj8P86gXDxZbQ/o2U0CjMCewTiVcEPldbHEVOnPAbejD1g="#;

const ARROWS_UP_FROM_LINE: &'static str = r#"eJx9yzEKwCAUg+GrBPdSUkWXV2/g2r3QwUXo4P1RB51UsuXjl//NEd+tkoGFPjTqlJez/V66Bge6ZwKJ+46rMBhcjLRDChiqJYo="#;

const ASTERISK: &'static str = r#"eJxljMEJgDAQBFtZUsCZjXp6cKYDixB8+BF8iPWrCOaR78wwfiznhnUKMxP0YgrZm5dlL2YQmsKgMsYO7Otm/5SBUVp7Tn9yA/iVGVE="#;

const AT_SIGN: &'static str = r#"eJwli0EKgCAQRa/ymb00kyYu1Bt0CLGgoEVISN2+NP7i8Xk8n/eSjxUlkCHkJ5CMH+/O6IdfR3+ma8MSaBYLV6ekocF9FlyVJGFIuwJWBq61rYkv9/IYhQ=="#;

const ATOM: &'static str = r#"eJxNj0sOwjAMRK9iZW/TuGlDpbQ34ALsUEACiQWqWMDtybhK6GbysV7mJeXHmp83yp/ZeXWUv9u6lsUt6bCNl/S6vO90nd1JO1FCZJUucIleOuUo/chBBvZeJmwCQnmSY+QRxzoAEggcF5AAUrkmzLEJCCWABLANzvCBx87GDxIJka3NcGszvPlYm/maJ4Trq6ZpvaZpvdx8AJIJA4Rv/cbf5gfnI0Wl"#;

const AWARD: &'static str = r#"eJwly0EKgCAQQNGrDLPXmLFJBccbdIiwoMBFSIu6fUmrv/i8VI5W6gZNcUIotyLx10cxYE7Df3M6l2uHVXEmsaP3QGxDBPLAXI0YZwQckBV2JlqiTjvJL8RUGAo="#;

const AXE: &'static str = r#"eJw9yzsKgDAURNGtDPZPnfwDMSvQRQQsUihYWLl6o6DFvd1JRzkr1qnbaUAlobdoFdVT4d0IgqJFzyRil9PwkJw+uNCCGhF+M2Lg4KouAaHBRsXDXz+6AZMwGeM="#;

const AXIS_3_D: &'static str =
    r#"eJw9ybkJACAMAMBVgr34EJImZgOHECzSCBbi/GKh7Z3Mtgx6cRUBdyJL5FTCVZV3AyFHYM+/DrXGD2Q="#;

const BABY: &'static str = r#"eJx9T0EKw0AI/MqQu1bdmnZhG+gD+oiwPeRYSP9P3aWkOZQgKOrMOJbX/F7wvA2PDLWFRYepnNpwKttK/WAn0LGyc4KyscPAvio7scGI/R8nY+Q0Z2RIhAbzisQ5uPadSPQjNgTpJVrZAygA952EIVXrvHDL2mv4Wanphh1ruVJcEur2zr38fvoA9VM6ug=="#;

const BACKPACK: &'static str = r#"eJxtjL0KgCAUhV/l4C7p5SoG5tzS6tAmNDg2hM+fEZSI3OWen+/4M10ZxyI2BqmoVWIwVD0Nlpxdq8GlFgj0GrJ+q221pF0EPz2bwX/LM2zkv4Zay9RqUKEB50C6SNOR3JFmSGqV3ThwTXAD6no85Q=="#;

const BADGE_ALERT: &'static str = r#"eJxtzs0KwyAMB/BXCbmbGWfVg/YN9gK7FTbYYIweemjfvgn2E4oQ9J+f0dx3wwdeBR93Sg0kCq7z4MHKYvAUk5ESYc8CRd2e0VK2zFa2BcuQEzJCmuMkc/WcVRae2OabfrXNv+//DZMryA5h4oIJYeR6HGssVtXRhrVJlnev16W1+hkeRTiD"#;

const BADGE_CENT: &'static str = r#"eJxtjz0KgDAMha8S3BsTTX+G6g28gFvBwdFBPL+tlmqhBEJ4+Xh58Uc4d9imbhnRaXBohiAgQLEYBK1TsVn4NIM2jTWUW9HoxYqQTSpIRUT/nVTrHCXMrN3s+xR19iUwD2AvptZGowBTfoQfJyncDdeeMrc="#;

const BADGE_CHECK: &'static str = r#"eJxtjksKgDAMRK8S3De2Nf0ItTfwAu4KLtwILrw/NlirBQkMYfKYTDjSucE6dfOA3oBHqxMBgcyjgNB5kcXB61l0vLZQkerJG6tGCWkgkRHzTRJ/7yRjduli6LlqDE/hfQSlgYcE1fMFaJYpxQ=="#;

const BADGE_DOLLAR_SIGN: &'static str = r#"eJxtjrEOgzAMRH/lxJ40SY3jIWXu0rUDWySGjAz8v0hEZEBCliz7/Hx2WvNWsHyG39vKCLEcMoHganiQjWJqijg1trGVd6gn1dyBqdBNbpCpyHh1Mk/nXMN4Hqb0aq9OSR/2DCmGc0DoN6tXIe2bQl952gzw8med7DbPNnI="#;

const BADGE_EURO: &'static str = r#"eJxtj8EKgCAMhl9leHepOTUw36AX6CZ08Nih9yctsQQZjPHv498/f8YrwbGybUZH4NCoqEGDyCVBo3U8NwufZtCWsYdqa5p4sSZUkw7iGaG/Ex+dEwUzOwt+KlGDb4EtSJVosJAEC+r6hnx8CFUDb9hNMrY="#;

const BADGE_HELP: &'static str = r#"eJxtjkkKAjEQRa/yyT5lhs4E6dzAC7gLKiiIuHDR3t6UhrQNElJUPV4N+VGfF5xmsbcUHSJ5UydMUO1pTBSibCFgZZ4Cp1uph8HUVxugD9lIsinud5L8t06x5g+i5B2fWvI4OJFKSNXCdtVRtNBHBSMb5L923a73MxYzC21IaYEXp0Fg0YxarT9189ksbw+eOd8="#;

const BADGE_INDIAN_RUPEE: &'static str = r#"eJxtjz0KgDAMha8S3FMb7U+G2ht4AbeCQxfBwftji6W1UAIhvHy8vLg7PBHObdpXwRpYmCUoUCBTEShhGVOz0DQjbB57qLSqyQ+rQjHpIEyI/jvh6JzMmDkm7+Yc1bsamIEjD3VaRouLViCLGilS/TEVNvYF3tE6rA=="#;

const BADGE_INFO: &'static str = r#"eJxtzsEKgzAMBuBXCbk3a7ra5lB9g73AbsIGG4zhwYO+vamWqiCFpPx8TZOGfvzAq8XHnaQBoeB6Dx6sHgZPUYyWCHsWKObrGZVSM7uxGpQhJ2SUNMdJ5uo7m1l4YpduedUu/b7/N8zcIgeEyWl3CHPpE69dcWYHLJuRSspTslz1AuX5OFQ="#;

const BADGE_JAPANESE_YEN: &'static str = r#"eJxtjk0KhTAMhK8yuG9f09S0Qp838AJvJ7yFG8GFeH5b/McSCGHmYzJx6ucB/2/VsQ41ghbbOziYNASnfVBpeVyaaJ/PJ7SvUzMbdgp7yANSCanvSar0zmRMflUbP7lqG4/CY4MABi++4JEFEVjx2+sakB2kbMjNWAHz+T9x"#;

const BADGE_MINUS: &'static str = r#"eJxtjk0KhDAMha/yyL6dtlPbLmpvMBeYnaCgIOLChd7eBos/IIG85PHxkjg3S4+2pt9XhgpBOtNYWKhcGlb6IHLzuDwnPY9PqLTTUwd2GiXkAYmMVPck8XZOMeb+lOKHX01xHKYOm6lJG8LK6gibLnvWwCxTaQeW0yzF"#;

const BADGE_PERCENT: &'static str = r#"eJxtjksKgDAMRK8yuG9stU1bqN7AC7gTXLgRXHh/bPFbLIEQZl6SCdu0L5i7amjJGTjiZtLQkLEUNFknYrN4NSabxhy62qPJE3uE60gGiYiY7yVReicTxmPVhzpF7cMdeFUGXjD4bw0efiGpCk5cUibzDhktODI="#;

const BADGE_PLUS: &'static str = r#"eJxtzsEKgCAMANBfGd41NVMP1h/0A92CgoKIDh3q79tITCGEzc3nNBzjucDUsr4WvgEvrB4NGJC4FBjhPMfg4OtZ4WhbohhST74sNeKQAnEkTT6J/z0nidmBdaGir3ZhW/cZLt0ypRlc6s031RYz1p4sqdzGs2Tj3c8+Qr43yQ=="#;

const BADGE_POUND_STERLING: &'static str = r#"eJxtjjEOgCAMRa/SuFMBoWCC3sDVwY3EgdHB+8cSCUpCCG3z+/rbcMU7wbkM24TegkfS0YAByU+BQecFBwefRuhy2UIlVE2+WBWKSQMJRuzfSfTWyYzRMaxhzKeuoR7sQelkOg3Fc7TPaKNGC/m/Vpz7NpRcbTxHfDr+"#;

const BADGE_RUSSIAN_RUBLE: &'static str = r#"eJxtjksKwCAMRK8S3Mf6/4D1Br1Ad0IXLrsoPX+Viq0ggSFMHpMJZ7oyHCvZJHUaHDUiKVDAynBQ1DosYuHzDLV1HaEm3WMv1o0WMkBYEP1Pwtk7VjGzkxiWWjWGXtgDN1nPDyLrJEC0LgxVRnn7zj59MTQT"#;

const BADGE_SWISS_FRANC: &'static str = r#"eJx1jj0KwCAMha8SumuN/wXrDbp26CZ0cOzQ+1OlYhUsgRBevrw8d4U7wrlOm6BWgaWaBwkSWCoESY0lqRn4NE1NHnuotKqxF6tCMekgkhDVOpHRO5YxfUzezTmqdzUwIqDZbZQ/Ox7FYLMA6ubkAbZuOB4="#;

const BADGE_X: &'static str = r#"eJxtjksKwzAMRK8yaG/XTv0LOLlBL9BdoIUWSumii+T2kRLnB0EgidHTSPnX/V94NHS76uSRdKg6BwfDYeF0TIpTxKYFHaU9QiWtmpmxVSgmB0gx4vdO6uycESzcqc0XebXNn/f3iaFqyHpCz7UmDHYqvZ1URgVa0G1WzwuyWAwWdATdLTd7"#;

const BADGE: &'static str = r#"eJxtjjEKgDAMRa8SujdWbZsMtTfwAm4FB0cH748JlmpBAp/wePwkneU6YF/MOiMHYIxT8eDByYzgkdhKELwsIunaSzUac4/WQC3pJCtK+DbZv3NOtbiZnAZ9Nd/PhiHB"#;

const BAGGAGE_CLAIM: &'static str = r#"eJxljE0LwjAMhv9KyH3YxLFNaHv24nX30RVb8CCl6Pz3JiibMnLIx/s8sfepJpgdXpiBhnM3MTAYKWq44bFfd6M7entQw9vVox6oHdt/LjX0ewB+kNncEkOFl8MOIcV8TdXhgFAWh4TwzHNNMh0RFr2Lpby3IZdwixAUEzzIAzaiSVPoE2/YN1b8tKPenblALg=="#;

const BAN: &'static str = r#"eJwlikEKwCAMBL8S8gBbxUsh5i8lLbSgIOJBf6/Rw+4cZkj+IvGFEtCeCNIm3WRfZDq2Z8p3/eAJmLy5QGe9ceu0UssDJmwUSg=="#;

const BANANA: &'static str = r#"eJw9jrEOwjAMRH/l1D3GduI2lUKXzvwAWxUGRgb+X1wAVZEdyz6/c3sd7yce1+lWYLlnieSoDFP4ERIYoXyGipi2dhkbWzv3Qixgi9S1U+rJmFBljjSzhyWZ78ZmgA78QXaG08qd2J9BEK6Y6V6Ec1JUyspiJ95Apf+Tdh0WMLGSvtjMA5JTdj+v+wBeFCzP"#;

const BANKNOTE: &'static str = r#"eJwti8EJwCAQBFs5roBEfUgeagcWEU7JCXkEEaLdR7m8dmBnXM3UgHO5uHnUBmF4tAjd4+Qq85bUeJLC4PYVBEel0p2BukQ0ZOvypyR3cM/ZGJLHaEEb3pSO+vhpaesOH4USJDI="#;

const BAR_CHART_2: &'static str = r#"eJxdzE0KABAQBeCraC6ASbIYbmOhZM3t/UZj9Xr1vUc5lSiq9qAdiIo72+ioRs6uIJCcLNDDeDBybD7Ljgysrd1Te2kHlMQhAg=="#;

const BAR_CHART_3: &'static str = r#"eJxtyyEKACAMQNGrDLvIGOKE6Q2sdsFgNIjnlxXT6n982eMsmMU1ArrIC9lVCVqrfEMGTD1bQirREF2upy8PSfIeAA=="#;

const BAR_CHART_4: &'static str =
    r#"eJxtySEKACAMAMCvDLvIGOKE6Q+sdsFgNIjvlxXT6p3scRbM4hoBXeSF7KoE1Sr/kABTz9awTjRG43r68wBJcB4A"#;

const BAR_CHART_BIG: &'static str = r#"eJxNjMsNgCAQRFvZbAO6IpED0IFFGCEuN0OIn+51jRJvk5l5z65TYQgORwVqI8Nk0NtGWm9znAucDqlFOBwOCBzTwuWJ+W4IYU+hsMNeKPm/lIz6t4pGV+jTUFexC3vBJs0="#;

const BAR_CHART_HORIZONTAL_BIG: &'static str = r#"eJxNzMsJgDAQhOFWlmlA1ih6yNqBRYgGNzcJwUf3JhjF6/B/Y7cpKi2C0ZDZuVfuMdgqr4MNbo50CVrQ4ZeoAq5Bp6ADqfOrRkEDCmnhrHJf1BMly+YN/qTcdZ+6AVelJqE="#;

const BAR_CHART_HORIZONTAL: &'static str =
    r#"eJx1ybEJACEQBMBWDht49g/U4LQDixAMNjQQ6xcTI01nrNdBackVFZ2IRHTZvq3ZzgWB5yNA/Nfx1OML/ZsdfQ=="#;

const BAR_CHART: &'static str = r#"eJxdjTEOwCAMA7+C8oGSqIo6BH7ToVLFDL+HYMTAZNk+2fZ/5Q2NE0mkUCURy1CGNveRsl2OZZvwhB6U92LdYuNgR6i7w57iRzfaAZOgIQQ="#;

const BASELINE: &'static str = r#"eJxlijEKACEMBL+y2B93CUewiP7ARwgWaQQL/49aaGM3zIy23A0luPSDPyNxUd/lou5SBSSQhxiT+B6Sn9r8CQPOWxbq"#;

const BATH: &'static str = r#"eJxdjU8LwjAMxb9K6L11Sf9t0O7i2av3gsIEEQ8i7tubLBvKKOUleb+8lGd7TXCp5jRAguQieBcbssrv5Fm0Lh6DS70HD4EBqQIEF8+YGwEpx0oT0v/A0ttGM5aDXBnL/fa4wkzVZAMfrAY7Vm57AzO3CynMSopFpB6Kkqqs0o5dmEG9rKlZNwj3sYKuDP6gLWKDv1xhQoQ="#;

const BATTERY_CHARGING: &'static str = r#"eJxljDsKwzAQRK8ybC+iWX/UyKrT5BCGBDbghBQhxLf3yoWNMSpGD97b/Bm/hvsgN3ZIxlGhiP7oq79+5+A/CyolX2pT8lb2SNd2E2M4hrEeMp67F4kUGnTWTnV2Y3q+H5h1EDaCv6+qYKYznbmyy1UrC4epLv0="#;

const BATTERY_FULL: &'static str = r#"eJx1jtEKgCAMRX9l7AdqFvZi/UxJCtGDCOnfN1OLgp4uuzuHTTk9e4gjDgiOQyAYbVfjR6QWIVyNy3HYxRvuJU6qSd6kNrtrCMRr3geRM/JMxMkzdQlOWIW5lA+TXPmHUnlC5CxQdd/w52iW+yL3N3wCqBA9jg=="#;

const BATTERY_LOW: &'static str = r#"eJxFy0kOgCAMBdCrNL2Atia4AS6jREiMC0Ii3N4Wp02n/2pzWApUh4wQQ9picUgjwpnWEmU0CLn1VOosS6feDvrn7Z6OAI0FTgiVJBNZ+e5NdiLFyh6syHzZ/8t6fukF7n8nOA=="#;

const BATTERY_MEDIUM: &'static str = r#"eJx1zlEKgCAMBuCrjF2gtsBezMuUpBA9iFDevk2rh6AXx36/H2aTnzOUCUeEJIMRgo9ryBNSL9FZo/YecclBcoPOdtpzdou7h8ISDqJImGpus8hOpFjZB9+ftWRax/xQJXrMW+G6P/gCypoyXw=="#;

const BATTERY_WARNING: &'static str = r#"eJxVjEEKwkAMRa8Ssh+dxNJspl278RBFhRFEXIi0t+9P2xlaskh+eO+n7/DL9Oj4Jg1Z1kFJKWIEW//tHVfAHZByUO7T2Y0+Va8luzZVi6FqawW0nfV+fZ40acdyYRqxVZkmQRZkWTJgxzbYnxJX2LfDxqXjyC5PO8Wtq+BiVS/8DOagPBA="#;

const BATTERY: &'static str = r#"eJwti1EKgCAQRK+y7AVqDepHvUxJCtGHCLm3b7b8eszMG1/T3kgDb0y1B3aAfnjK0XJgWZn+Pqdy5oZm5ugn+0V/lTtRd9ghKCgLdBkZFDHZtPgCKG8cZw=="#;

const BEAKER: &'static str = r#"eJxtizsOgCAQRK8yofezu0AsVm5ga09isaWF8fwKBaEg08zMy9M7P4Zrd4efA8QouKRLOZM2FCEvxcxgrDV/s63fE58yFMkbcSMfJSwcOg=="#;

const BEAN_OFF: &'static str = r#"eJxtjk0KwjAQha/yyL5jZjIZE6gFD+AhShUURFy4aG/v9Ie6kYQXJnl8X9p3/7njegqXijo0ZOq7YcrCVLM2QhoFTGz5bDDEZRWI9JyoGtZcryvVRolD1x5mbNfucI50zMgUa9ox7BCUIcIdzK5muLy43EqCkdkfUKakDMeZ9ArdxEZajv5LieWyvGHp9UqRM9bclBRTnVN/8OfjdcMopyASMLKfAdM2Tsvo1bnUfQGxSkJh"#;

const BEAN: &'static str = r#"eJw9TrsKAzEM+xVze9yzYzs2pAelc3+gW7gOHTv0/6nTIfiBBJJQ/4zvG17X7UE7kikYavg9MFSgoTSHQBNwrAYBcZZkuYVQmaaqMMrOQNN9MzDY/+PAfDZslZOQFEM2n4BkLFUhQq+aYRnx3I5+mXWOvkopViHIasZDQGZUniK3KPl9WX6EayrH"#;

const BED_DOUBLE: &'static str = r#"eJxtyzEKwCAUA9CrBHfp/0HEwXqDrt2FDo4dyj9/7SJFJEMg4eW7Pg3X7g6CYj7V3pAeBT2bxv8AWnIlbx8qedAAlTPOkpMMC6lEsLg4CE2NMp4XTUop6Q=="#;

const BED_SINGLE: &'static str = r#"eJxtzDEKgDAMheGrPLoXk7RKhtgbuLoXHDI6SM+vLipF3vbg+22vh2Obw5Ig1KJWgYCuMSSKc/4ekKah2HCjYg8dwbROvaRO5h+ZwOr8Nk+HfSK9"#;

const BED: &'static str = r#"eJxtijEKgDAQBL9y5ANmF4knnPmBjwhYXGkh935jIynCFgszY3d7XK4jnZQ1UFK15UPVBqEObRRK7kN/BvK0xOacmSIa+89fUJMgDQ=="#;

const BEEF: &'static str = r#"eJxVkM2KAjEQhF+lyD29050fe2BGkL3uvsDehqygoCDiQd/e7gkiElJUmkrqI1M7Xttpj3afAwuVgPaYg7q5zsEH2+mrZ7bTZbkd8D+HX09Clmrqe/AVK4kgU22RiZGIU6SNmhmjHdSSg+4SUo+jgLVlM0rZbiiYzWRKu49n0bv+nMP73xRnVg9CiEfrLYaTvcVkvQdKDPHB2GWdRqGKQvLNhTJkILFiGPnGZHnhrTE1rFp/xHJc+0+sBE9+MkHY"#;

const BEER: &'static str = r#"eJx1TjEKwzAM/IrILjWSo9iBND/oVOjQzaSDx0JK3l8pLqHQFCH5zJ1ONz7zq8Dj3Fw4AnPhHCBAa8XWfUFupvHkomncpQOwrP0BweEv00EkndFckanrSDEA6YKCDn0wRbHHmjTbBO9PEtSZYnJMGitFeh3MCMRigyzilEX36U5YVfjrBRZjM7OLzmOV3Q9SK6SVJYsdabcyVNL3H+WW9sU3RR5Lvg=="#;

const BELL_DOT: &'static str = r#"eJxFjbEKwzAMRH/l0G7Vsowdg+2lS5d+RHALLXQoaYf272MnQxA6EPd0l9/z94Fboask9hDP6ewsO0jopxNI3PWiHzUOalKzRlnhOJqA0JcjLISVO8eJlWo+jdiaj3A7PmTuvscmdhvtJfbg23Nprzvav9BEWAopof0KyTSY3a0r/BYp9A=="#;

const BELL_MINUS: &'static str = r#"eJxtjbEKgDAMRH8luDcmTa0VqrOLHyE6OAr6/3hV0EVCLpC8u+R9Pjda+2rSxIHUL5zI0J4jNY+OdpjzZK5bxBkbtq2LFNEKUsAHzMDgqyHXJXLIX7AUi87KHR4UkbvgIvnjG0pbfA8XfEgljg=="#;

const BELL_OFF: &'static str = r#"eJxtjcEKhDAMRH9l8N5s07hNha6wNy9+RMGDF0HQ/8cawYsyYQgkbyavZZ8x/ZoxkUL+ERG+isEJqQQmgZk3UcS36fPnpPp8s6xgHWQTFyCuKy1FhdmVReKY9A30ll+YuhZmV49Q3Z7/S0CVr3PfDrAyLRY="#;

const BELL_PLUS: &'static str = r#"eJxtjTsOg0AMRK8yol/Hn2U/0oYmTZocAiUFZSQQ58dAQYMsjyXP87j9x2XC79l9pJJBIpWXMgkkUYT6zKe+bbagsFC/HMxRpRwSkreAIVRJfVcod0N77KlDu7J5P5DRqYhD+CjzH3zH9yhTujMK+vUyNuQDLUU="#;

const BELL_RING: &'static str = r#"eJxtizsOwzAMQ69CZJeqj+vYgJulS5cewkiHjgWa+yOKh0wBIUoQH9uvb198HtM7o/SMDAkp1CCrYIajHvPyv5PBqU5Lux2lpZ1VFXaYduWaMEyGnOO64BPsaVwin2G4Dy8XnMV7FTJ24kIpllE+uR0G8yw/"#;

const BELL: &'static str = r#"eJw9izEOgCAQBL+yoQfvOIOSnNQ2PuKihaWJ/j8CBZnsZosdfey7cW3uSFgtIYEqDI6gk7BAkFt2ecVHiM+u6NSkokNlCoLIxiHP6EUdCXWN/w84fxis"#;

const BIKE: &'static str = r#"eJxtzTsKgDAMBuCrhOxV+kKHtjdwdZcqVOggxUFvb0IHKziE/JAviYt7iXmDeHmUY2cR4k1p4FQ8aurB9RUF12DbiHfnx9KoSon1y0cdy5lg9ThJBXxhliYLLTQYKgU6KdaswgPDOS4j"#;

const BINARY: &'static str = r#"eJxtjFEKgCAQRK+y7AVKEb+0G3SISGn9C1kqb18alIJfA/PmjYl+ZbgsCoWQ3iAfNmKLGiE+RCKcwTFZVDiZIQuTKdpX54N/XfmpkfaFCZzFWYMcqYBcVUAoEH2iQSiSh+5LLboBSQA8rw=="#;

const BIOHAZARD: &'static str = r#"eJx1kEtqAzEQRK/SaK/KtL4jmPEm6zlEUAIJJBBMCMntXT022GB5IanpT70uLf3j2D/fpP+tToOT/s9X0ZwcVxfcYXk6NxyW75efd3ld3VZQJSJ1jyYBWSbJCIyCsPJcmGmsV2koEoRixWRs/CryNXNYJ6goEub7hk0r4o65pfgLhZnklbHliCmmhjzgaDYGQf4haNebMHeqF0/hxMfChOozot/dWUqFEpY27kiKdiPa79DyxlLmID9OvVmxK8o09C7citarJ4s+uXwxg9xhYHLLFDZwBy/M7LST5bb7BAxadIo="#;

const BIRD: &'static str = r#"eJxtjrEOwjAMRH/l1D3GdtLYQ+jMAGv3SAxdkBgQ308KqB0aWbJkn33vyrO+FtzPw00ybCGWYSqndTmVTYqUIH4RrQ4Hf8uDz1YT0m8ORupBKV4VykePhzIMChrD2joQ4cZ4x57S6EY2ztoLZ+2tZuR/rkiegjDl/fYDj1o5KQ=="#;

const BITCOIN: &'static str = r#"eJxdj00KwjAQRq8Sus9nZjKZH6g9QXsI0YULCy68P6YRRNzNB4/3mPl5ed3T7TxtRDC1RIHicRUEC1w9KUiyonBLBCbtt4fseYw0xto6mchRxPaGKN7RIpxRpRsRthOa1uydk687/t1tuOvH3Ya75oqQDO3JBmoHwquDIwlYf2s95vmIbQbxxAVV5VFBrJkMxjYt8+n4d3kDLn40Kw=="#;

const BLINDS: &'static str = r#"eJx1ykEKgCAUhOGrPN4FatQoQV236RBhgUGLkIi6fUpQG1sNzPebbdwDTZYHSTKgY2eqfDnzgqip7X8AKAqS6IC6QB2hCRAFUSQPqA/8Ev06U7QsmPxpWaW5LEPn5lF3A20gNlE="#;

const BLOCKS: &'static str = r#"eJxdjb0KhEAQg18lTL+cUY+7Ynfra661FxXHTmTx5+11Kn8YmJCPkPipaxK2IIVAu6HXFOQjmNYgFNgvBcvQJjUe/cvy0Y91UrRB/syQs/rWBJHZOTr+yqsHZ+YnOJR6B46zez8qChuzmbgDO2Mmmw=="#;

const BLUETOOTH_CONNECTED: &'static str = r#"eJw9i7EKgDAMRH8lZBdNROvQ9guc3QUFhSoODvXvvRZaCHlw984+63vQ5vgyZEg6XDPQsGjAn5EY9rZNkrfhvHeK4lgmpqiOVZg+UBSUTMhJKzLKsXZ529dJUX91kyC/"#;

const BLUETOOTH_OFF: &'static str = r#"eJxlTDEKgDAQ+0roXvWKxyGc/YGru+DgoOAgvr/p0qUkIZCE+Ht8F841PGIQiwrdJd3VQ/axttnbJoGYyL7bZB4UC8Uj40HUn0kbFrcHG1k="#;

const BLUETOOTH_SEARCHING: &'static str = r#"eJxtijsOgCAQRK8yoQd3kV+BnEBbexILCk0svH9cY7QiM3nFzMtnvRq2SR0REUxS7eFXuwtnWaIqeXikkj91sWTSCHbC6uBAb7Q3IXR0TmDbDPH/3cd/HX8="#;

const BLUETOOTH: &'static str = r#"eJyzKUgsyVBIsVXKNVcwVzA0ACJdUwXTMKMcIOkDFDFXsrPRBymyAwDtywrA"#;

const BOLD: &'static str = r#"eJxlyiEKACEQBdCrfOzLqswuhtFs8QI2wWA0iOcXETTIq49ragXZiqAISicCQS6P8X83wvE7j+MzP2h5z7jrAIv2FT0="#;

const BOMB: &'static str = r#"eJxNjUsKgDAMRK8yZG819YNC612kCgoKIi709nYUioQXCI/MuLAcYZ0Qbi9aCg4vnSBc8VLpXf7p3u3DOWP0smlnahA1bRYZrKlAim+y0lRrFA2hVLwrSRTULf9ZwOBfvLWwUTO+TvoBN4Ynrw=="#;

const BONE: &'static str = r#"eJxdj70KhTAMhV/l4J7ctJDbCuob+AJuUgcHBwffH9NUHKQ05Pc7yXCu145t7OaQEKRwIjaH/z0EkRWyVusegn0hBauHliBWespvizVIEXAOTsoNdFBCKgY3hYanL749LRzrjInEWJU+CzidjE5Oyg20dNPwq7dMNyD0KbE="#;

const BOOK_COPY: &'static str = r#"eJxtzLEKgCAYBOBX+XGXusOkwZxdWt2FBseG6Pn7CzIIueU4Pi7s5aiyLWalwGdXKJRRA6FlBUwMw21iaHISuPRKPNrVHqR+zgn4U2T/rRqr7QTbwwWrsiO7"#;

const BOOK_DOWN: &'static str = r#"eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v81SEDkO7njw4l2eE47FbCNQcFItycpOQIstBJO+zFgZc9vlS9FKYybFQU0pdh8xkN/nP7kCEILXWN/xC535ISE="#;

const BOOK_KEY: &'static str = r#"eJxljVEKhDAMRK8S8l+3ybaLQivsX388hFRBQUFERG9viqCIhBDCzLxxU7100HisDFCR2VWR/XNmIa2WIfilL5DB0n2Su3RXhjXkK5kglvoZ0soG1nck9nMcWpg9MkLcPeZyNo8n9lRv8ChgVkZosu/ekQr4Sgld0gE5kzK5"#;

const BOOK_LOCK: &'static str = r#"eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCDkk+b4//pVKhingzQJdeqkdyZV7AW3TisDpFMng6E9Kj35zuN2lnmND0lEynUTeKev8KPBeppIDDgjfgA5h/QQkhDwvz1wCCoIuWC3ld49oAHe3iYH/6RZM5S3+BwsYMf0="#;

const BOOK_MARKED: &'static str = r#"eJxVTDkKgDAQ/MqQXt1dTUSIATsbHyEoKIgGFMHfmwQb2YMZ5rB+vBZMrRoqcJPrO2PdSa4Rj8IwTGS90C3UBzz+Vcp00JSzRWxy1h/bs637DH+s+3W2igmC8OKWqMEmQQNJoc/uXrRlIYY="#;

const BOOK_MINUS: &'static str = r#"eJxVjLEKwCAMRH8luGuT0AhCKnRz6UcIHRw7FL+/ySKU4+COB0+f/g64j3DtQCXJjCQnJwEvWgiyv8Y4GZvt/qcYxViourmp6vIVIBx5gQ/5rhkD"#;

const BOOK_OPEN_CHECK: &'static str = r#"eJxljaEOwzAMRH/lVO4strOkICseKS0YmzIQMmlgyvfPntSS6iQT371XP89vx+s2rTP0LoOvvTQOBREKDmpXt9IiSRDiMFPyPKalXny51H3/5gwWeKxw/q8iyJt2ys1ZiJTw5yENTiZwqQnJhKS9DGvpgfkBY3kofQ=="#;

const BOOK_OPEN: &'static str = r#"eJxdyzEKgDAMheGrhO7Bpgm6xM4uHiLgkNFBHDy9qUtA3vDgh09PuxyOtewN2GcTEKgxipebxBj4CxUZeWtP6ToN0zXloJi24s8ShPUl7QuPERw5"#;

const BOOK_PLUS: &'static str = r#"eJxtjLEKgDAMRH8ldG9NgqkUasGtix9RcOjoIP1+k0UR5Di448HLZ7s6HKvbZ6AUZHiSjYOAFTUE0V5lHIxVd/tS9KLMlTyZqeTHl4Cwxx9ADMt4wQ3I/CA4"#;

const BOOK_TEMPLATE: &'static str = r#"eJx1jrEKgDAMRH8luLc2p9EKVXBzcXUXHDo6SL9fu1VICRmSd5dcuM8n0jU3OxwB0aBZQpuXSygRS6rAnniycrDoRp86BbAnRKTKPT4mzYPsqYAq4VEl339sg5UTVii3+4rJGdm8GkqSsbL+1UOeCv0LdedZwg=="#;

const BOOK_UP_2: &'static str = r#"eJxtizsKgDAQRK8y2Ed3N64fiIKdja29YJFGsJCc36QRJGGYYnjz3H08HudUbS14rDUY1kVqRSrFMLq0qtk16Tq7T+AB4iUIrfFx/B0yugqVJAHbvc/JNYIJ1ljEFLFm9AVAAS9x"#;

const BOOK_UP: &'static str = r#"eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v+lSEDkO7njw4l2eE47FbCNQcFItycpOoBU1BFN7mbEyZt3lS9GKMpPi0Ewpdh8xkN/nP7kCEIK3HjQdv54tISE="#;

const BOOK_X: &'static str = r#"eJxtzDEKgDAMBdCrhO6tSWmUQiy4dfEQBQcXwUF6fpNBUJAQSHifL2e7dthmtyagHLh74iUGBlvUIRjtqxF7xKp3+yp6VnNFBmsq8vQdlDQzeQb+wWwGb7sBl4ghLQ=="#;

const BOOK: &'static str = r#"eJxVjLEKwCAMRH8luGuT0BQKVujm4kcIDo4O4vebjHIc3PHgxVFnh/a5cgO9QZYn+TkIWFFD8NjLjIsx664nRS/KXIqXmdIG3dMR3A=="#;

const BOOKMARK_MINUS: &'static str = r#"eJxNy0EKgCAUBNCrDO4l/ycTQT1Ge6HAoKJFhHX6/LSJWTwYZsKRz4Ipqo08mLTTvXboR5sZDNNCYM2FzL8AXzQ8KoVO7imsyz6jclRe4W6QadJnFa1sZZVe67MbBg=="#;

const BOOKMARK_PLUS: &'static str = r#"eJxNi0EKhDAQBL/SzD1splc3CNFn7F1QUFDxIKK+3oSoyByKaar8XC8dmlJGLUA1zmTGIfvnNUHYcAoadmrfA7jq75DKf2Je+aGfWuwsRb+CTQMZyMQ9/C660brd6NjkFJJSe6X5456J/CYG"#;

const BOOKMARK: &'static str = r#"eJxNy7EKgCAYReFXubj/5L1YEpiP0S40uAQN0dDTq5uc7cCXnvJWXIe7uUO0aMEiwrkWQfA9QqZKPw/o4/a7nJbBcwMwSBAG"#;

const BOOM_BOX: &'static str = r#"eJx1jEEKgzAQRa/ymX2pM6hESHKDbruXGJpAFyUE296+SQVRUAaG4f/3Rr/GHDAZurUY7t0oEDRlGHKRwLINIHNLVl+rYvUqKqiZD3KWs6LfF8m7jPQxJIRlv+OUQ7kawtfQQAg+PkI2xFK1KljtYnJPD1cI7giuqIqQ6oPCLO1K/eMdy/0G+wE3G0g5"#;

const BOT: &'static str = r#"eJx1jDsOgzAQRK8ymgsku3EiR7Jdp0mbPgooS4eQxef24AaEBN3MPM0L7Tcbqsi3KPzHvTxTuJQxha7+ZQxNlS1SHoTVzd/ykpUYIx0xRXqiW4qWWzmksBoV4kw33wauZ0TukFt/RJ57MAPY8DQa"#;

const BOX_SELECT: &'static str = r#"eJx1jzEOgCAMRa/yw070FxhIkBt4CBIHRgfj+YVFMSnp0vS936bpLFfFsZk9wBWBYO1lW2dyWjrN6XUYB4nQJSEYP2uyKkA4SlaT2rlKdS5UAf0k0cAk4hBv6m/owIF+lviRB3Y+XYo="#;

const BOX: &'static str = r#"eJxljrEKgDAMRH/lcE81qdoOVfADXB3cBAcHBQfx+00FS0ECCVzyLhfO5dqwdsUoDL8IBFUsYmLj7E6O6kzVqRLqIUmw8HcGgvGCepSJkVOnDNN33M5FH8qYoA9fjsMaCwdvHJrYqfnfjKw+MrGk1QOWvyyv"#;

const BOXES: &'static str = r#"eJx9kbtuwzAMRX+FyE5WJPWwATdA93jNkC1Ahw4q0KHQ95dS7TwVQ4AE6F6J95DTz/n3Cz7fd7PQmIBtlw8BAdeWAHuKWpTEn6/XzUqJs9ox3AhCLoI7sL0bCwYKGQMqenJqmpfTbj+91ZL7aS38bV9FCuZJHoWG8NIC9tWzOP+LJRCnjlqjKIUjj1lrbCF9DpwV7znMiRWw4D05rsKBraxrfKA9Kl6wuplXFTagZ94GS5XGW09vxpUgEmvxpClbrtqwY73pAGR8YMalFdhmeuq3coBEEsEi9cbUDJtIyzCGi/YHZ/qXkw=="#;

const BRACES: &'static str = r#"eJxNjr0OgCAMhF+lcS/aGvxJkNnF1Z3o0NHB8PxSTIB0ucvlvp57witwb92xwLjPgYFh0MOkoi2e1ENxkNMrKUNm/ROhzrteed4VKk3AJFS5wMgRUxVzVbHI7Rvk07YzkAUr+QNcHSae"#;

const BRACKETS: &'static str =
    r#"eJw9yqEOABAQANBfuelm58aEIyuqbhMuCub7US6/x2tsgZlNwwgkdDCJJVPYfSisnMBjDf0VxQsGrRD3"#;

const BRAIN_CIRCUIT: &'static str = r#"eJydj0EKAjEMRa8SZt/YpE3JwDgn0O0s3BVddOlCPL9pFWS0gkopLT8/+S/TOV8KnLbDnhgiSmYUqNfX4yKOyWFMsJYJR4XwJoY6gqNVwl0DDBEERddW+40JAvpX3eIIveyMhf1iOIdhnjYVcZ6eoAl0kWMNNA5nVsc9GwOFEj9UtKRsnS2V7OUr9Z1atFNgb8yaURo6ATmq28pjExPA99nNQeH3xhbI9EeimuPrwBsYdnYr"#;

const BRAIN_COG: &'static str = r#"eJx1kFGKwzAMRK8i/K9Z25KzMSSFPcAeomQXWmihlH60t6/lfhQ3MSFga4bR80zL8bqc/ml5zC5ER9fZiaPlXm+76esl76bL/nagv9n9hkiKtI9IZL+3jxV5YOhA7TggjySroVhE1KLIa0YQpYQ0ttZyygMJ/PjTzgtDyJ8QZBBQ3chgy2jX8XpdJeNKtn4Gy5qh9GAVWTXvgs4h4ZuChzIydMOQUfAFsacXbSBL4Zqx5fAYKVtCz1AZBKnI3IfwpnYMRUyVortDEAyi1Z9/pIV2"#;

const BRAIN: &'static str = r#"eJx1zksKAjEMgOGrhNknpnlIC3XAA3gBdwMuXLoQz2+qIHRaCcniX4SvPrbnHW6n5VLIQc7SbizHJEgCRv5KvnUdjcqRzKCvEhWVOIPGfBupoZNn2D0mlfgiNvSSUXeKj+y6rPXQsGv9kZMNZv5jZpiZGUYzw9TMODe3PphDYT36DbcxRg8="#;

const BRIEFCASE: &'static str = r#"eJxNi8sKgCAQRX9lmL2kg9lG/YO27SWlcRciPf4+rU1cuAcOHFvSWuF2OCGcOVZ2SBKhNEMN14vvOeWNq0Ol0duhd97uoTJEh7MyQGoZAwGB7BMkiIX+C6BDmd72yj/bGx88"#;

const BRING_TO_FRONT: &'static str = r#"eJxVjb0KgDAMhF/l6F5sQweH6hu4uouKcZNS/Hl7mw62Ejhy4e6LD+scweu+cexUq3DtS+S83VmfrCEZUr1vJN77Y4qMpVODgzUTgWDSWE2aRlc8kuefR4ZIvYJYByoUI6mqZYRy6uogbz7MC4/MLiU="#;

const BRUSH: &'static str = r#"eJw9jkEOAjEIRa9CZg9CoZ02GecGHqLRhQtNXLjy9P52oiF5tLQ82F79fafbeXk20UJm0qiKrgyUnqRmmlAyRIj6xGO+j5912bfTkOzbT3VZISALaXFlk1JI2cnEM5KLpmETd06ScciJE4agbNABhlu0oxRHSjKaArYKYoPoEBlN6Ah2HurPf50v+pwtgA=="#;

const BUG_OFF: &'static str = r#"eJxtkL0OwjAMhF/lxB4Tx85PpVKJjQFW9qgMXZAYUJ8fBxAdiLxEyfm7u4yP+lxwO+wuHJGJ5ZqqQODbuEisLhCfC8JuGvdNPI3bihIHCJUCTl1FCGBZnK4uVIV+sep0cUzSW/A0ZMTZw2xNkxreCUXov/oeYGNS3yFlylYoH3+2SGBepSY7fC6YrbAZcC8JN/TqSufJQHLq1RUEnn37MTBliz2842/RX2yzWlg="#;

const BUG_PLAY: &'static str = r#"eJxtkDEOwjAMRa/y1d0mjp00lUoXFhYuwFaVoUslBtTz4xSBGCLHHuyXn++Mz/m14nHutoII4VKO0k3jqU6m8Tu/ibFE6EFkxAYxoGfRnWRWDkHxqQHikRF2aam6mswGc86DjGwl+2/Adp0zu4Tn0XOjxqkhljkphotxRuECrX68NlGIXltLKKIsgaLfFO5Jeag7kzXQGHjokRY3xELir1ZSOaFBb/55fgoSed5/wBtv/12L"#;

const BUG: &'static str = r#"eJx1UbsKAjEQ/JXBftdsdi8POG1sbPyI4yyuESwk3+/mRLCIBAbCPDKbnZ/La8P9dHgURAiXssPhPB87c56//E2MJUJ3RUIcKCoyizaSRTkExQcDxE9CaDJKjYhhJWUXUqLI2TE10sVg7u1uI9vs9w5rugbP126AG5D+ZTeqAyrxpKgX44TCBdqbO04jKUSvo3EVUdbgDXw+r61c+++QDaQxcM2YvLOrSfzVrlSeMFT7InQbBklmJ/PqMb0w923tz/4EvQE+O3JX"#;

const BUILDING_2: &'static str = r#"eJx1jbEKgDAMBX8luBebRykt1M4urg5uBYeMDuL3a6YWacl0XC5JV7mFzmXaPAG7KyCQ/YYJBhJaJjwcjimnWaOcmpSx1tQaXfWVNRV0Qg4UBb8fsbIeEtMtLXlxfcF2aNzQhMa8YhVF7g=="#;

const BUILDING: &'static str = r#"eJx9zcsJgDAMgOFVQu5qW0qo0LqBQ4iK8SZSfGyvxYuI6SkkHz/x69hH4HGeOAY0CuEIaBHOe0FYj2c82z4PkQNqwsZXqWv80kWGIWBbgzFbYZk2mzTdX+qAuFT6RzTJZHKkVcas/E3uSO6cnLlvdQGA8V3z"#;

const BUS_FRONT: &'static str = r#"eJx1zl0OgjAQBOCrTPa9yq61YtJyAw9hhLg8mBjSoN7eAgkQfl7n28mOf9+jogx0s3AQXKjwxy4r/Cicwaldw0vSvRHDEzXVI+IbyBI+dRk1EDtCkxIh/AKdCFrVT409pFpXmL2yYNZBFiNy8FkPGW/tc/uW6NrKVimHcGsm+gOsHUpd"#;

const BUS: &'static str = r#"eJx1TkEKwkAM/ErYe2KTdrcpdHvx4sVHlFVYoQepUvT3Zimoh5ZASGYyk+nv4zPDJbqzQliCG/pDQYb+i7PfIQRYMne0KVJgzfWDPDK1pCikiRipIUFSa0wCle1YUMUVmaw16I9SEUMgM+nKYF4QTs0oYJpSaNPCVa5/r9NtTtMV0iu61kF6R8fqYI5Oys3K/gXsSj6/Keewq/8AykJNzg=="#;

const CABLE_CAR: &'static str = r#"eJx1jb0Kg0AQhF9l2D7mdj0Nwp1vkDZ9iJK1CAQ58vP27hWixdkN8/HNhPc9KYZIV3aotXJMfTjnsg8b8pAyegk6iDs1JUvAcmurHZvHR8I/EgvhOw1JLbaE+RepJug4PTVZ5QjW+OxlY7fZ2eSneNYcEQ++qL2sZAHnZkOX"#;

const CABLE: &'static str = r#"eJx1j7EOwyAQQ3/Fyk7KmR4IieYPujJ0Q+rA2KHi+wtLk0hEpxv8LFty+pRvxfuxPO+IhSBsPzE0zFp945GBr2VLtxHZ0j/ooNlNeLjgEiGa/arFrYrxo96aANtETlAGzHFWEkBphhOLcm0REqvx3d13oW+tJ33c+QNgjUiK"#;

const CAKE_SLICE: &'static str = r#"eJx1zEEKwkAMheGrPLJPbGYkcWCmazceQqJQoQspUvT2dnDTjWSX7/HXeCwx3xGfRk6Id6NCWBolGuvhh2N9Xl8Tbo0uLgkuBRmqa4kBYnKE9pvUQgwDlPuLdeVtwIkzGzufZs4bJ7Ee7sFdVg2az/mP+E6+jeMs7Q=="#;

const CAKE: &'static str = r#"eJx1jMEKhDAMRH9l8J5uM1Ttoet5L/sRwh56XFD6/aYIItISBpKZvEn/dc/4vYcvPahF4koQvo5Q+JnuN1jisKRXhZZ0oQE6bW4UBavcaEQw2WZMEN6986vRYplm+kYyIxY2fGUv6BEzQnZe213d6Ekd9QpP0Q=="#;

const CALCULATOR: &'static str = r#"eJx9i2EKwjAMRq8ScgBtShn90ewGHkJcMQURGQXb29tMwW3Q/Ur43nthjrcMldEiFEaH8E5TFkYaEOay7BLTXXJ7DY7hrMEYHukZoVrGphVi9O3Yb1VJ12aq8zNVWZg6O5fcX35ds8DEeKEByMjJkDJd18z2mT/OXD9zB5nvZ1v0ARNsWqA="#;

const CALENDAR_CHECK_2: &'static str = r#"eJxlzzELwjAQBeC/8rg92HvGkCHp7OLqXlCooOIgUv+9Sa+thZIh3OO74y69unePS5YTFerPoSOIpj5Hx+NhXYMf9f+g/OyjtGlXh7TpfnteMTCLBsFXs1Aw6FSWOFRa0UQtGzuiyTj3baE2JqlG90ZLvNj5lIcGcNwO3vllvx/E4DlC"#;

const CALENDAR_CHECK: &'static str = r#"eJxVzNEKwjAMBdBfCXkXTVdGhab/Iq7YgoqMguvfmyyyMfqQtvfcxDnfG3RGjzAvjA7hW6dWGCkglFwfpdldwkFMV5PiWXspPus7Q3eMowASKLPTumZx61Oooj9VEywLm7T+EeofXYzo1OJgRUe7/dxagYnxdQUawcnxJ6+xBukHeO45jQ=="#;

const CALENDAR_CLOCK: &'static str = r#"eJx1TjEOgzAM/MqJPSk+iNshZe7SlaFbpA4ZGRDvxwEJGIJs2bo7+3RxSnPG/918KXj6MGoiiLaUo+MnXDG4SH8Stpk7H5ohPorPEA83UbvtK8Lrhu8gba46WSrsQ609wyg1A9LUpFDLJRAnJeQON8Lg73hbAYTYPNw="#;

const CALENDAR_DAYS: &'static str = r#"eJx9jGEKwjAMha8ScgBdujH6o90NPIS4YgoiUgp2tzdZkapgfz2S73vPpXDJkDaPBuEZ18weySIUjyMCh3jlXD+iTAipqLm4o/YWd4v3ABvt7WJEnEWUlCi0n6Kq9FYb08lWtL+itoc6pqmOodocm/s4Z4bV48kCTXwYSJE+PxCZDpv/M5m0nUnbmfxmLxSrX1c="#;

const CALENDAR_HEART: &'static str = r#"eJx1ULsOwjAM/BWL3Udtx0kjFWYGWBnYqjJ0YGBAfD8OQsDQKi/nzndxPNzHx0zX3eakQtKd86ik1LXBynrw/zvpU9LUkUBQqRE6l81+2DaT/fC1khyZaYHoV3CLp2fpFxgVaCVJKKMiGb23TznIznCdGAbRCAuM0cPtFlCKGYe3cE1rP21u2lIDY1RHTSwhcIOSoaSjFDip3gyZDe5TFMaxpHUjhWEPqdxSL99vvAC6R0/J"#;

const CALENDAR_MINUS: &'static str = r#"eJxlzc0KwkAMBOBXCbkXO1MtK+z27MWr94JCBREPIvXtm+32v+QwJHxJ/Kf+NnIPeiUExa2sKZQ8VsaMl9OyF/5wnAeWbJxW/hCPVP71fD+kZVCUKn/LGAhKlRb91GhEC+pWxI17e0gkUiSCPC1abizm/zgPyLK/wQl3vb48Tg=="#;

const CALENDAR_OFF: &'static str = r#"eJx1jMEKwjAQRH9lyD0xO01ihTTgzYtX7wWFCCIepNS/NyloFVoWdpbdty8++mfGuVNHZ6RFbXuCsFM1CIO4fl6UKf8txLTUUr5UipvqSvFrpEC88acw85qah53xC7QEcHALhwZi83bFb7P2v7rb9X7ByE6RCq9PSkmFcYqCVii9AQyLPdU="#;

const CALENDAR_PLUS: &'static str = r#"eJx1jssKg0AMRX8lzF5qrq04MLruptvuhRYsFHEh4vy9CRnxAZLFIcm5IWFox44+tXuBiYt32YJAuVaGDM/HvidMfN8GQnSVa8JNjzTh/+u/FFG70tEsYCUboxCqqpRU3VWmClLwSixsxbkFwJaQ/uTq0Cd5fcCbjPPhuJf99u18lBdd50eM"#;

const CALENDAR_RANGE: &'static str = r#"eJxtjcEKwjAMQH8l5K4u7dh6aHf24keIK7YgIqVg/XuTduIGO6XNe4/Y5G8ZikONkD4OFULw8R6yQzII7zjn0J6pVMpOj5M9STfZR3x6KMTGwIiqUdTy5TmIKtJKNU0xP2PptqLsqGuKTAl1CxX93dc1B5gdXmgE6sOh3pPlGmkgcx53SG2OHe1FzMyGfQFF0k+A"#;

const CALENDAR_SEARCH: &'static str = r#"eJx1Tr0KwjAYfJWje2Lu0sQKsbOLq4Nb0CGL4CB9fhMLRWjL8Q333Q+X3vlT8Dx3VxHULWZBcA1GRpfwz6GJ/cOBlvaEJqgcbejGdGg1Y1rKGKu33xCGnb8HXeGwVVUjzB6+buBvR8TMZsTJ3dexl+o4GdrQbtG/o5I7uQ=="#;

const CALENDAR_X_2: &'static str = r#"eJxtzj0LwkAMBuC/ErIXm7fadrjr7OLqXlCoIOIg0v57E3NHPyg3vOTyJCS8+89At8gXCEl1rXsQqLRXoMD5tKwJXznOH5oYWu7CwZZ04fl43WlC5JppkshgGjVEyxH/VGoo02S01zrV8Pk1NAFxUvluKZ1qbqw1G98KJNw41nrngnxmQovhjH9adkd+"#;

const CALENDAR_X: &'static str = r#"eJyFzkEKhDAMBdCrhFxgJnWQLlovM1OmBXEhBdvbm5iKIoKr39D3Q9wcvhmW9MvRI1mEGNI/Zn1Xjx+E4rFDmHkwHEVicC/pDW5MU4BCzHuGRrPSRiuPvVBBJ2pV2kfYqaC3EkkpGrrY7bOtIzmYDszzPS77xlZq5R2vcD1H6w=="#;

const CALENDAR: &'static str = r#"eJxty0EKgCAQBdCrDHOBGotwoV2mJIVoIUJ6+2YUo0Wrz59530S3JbjDnrxF0gjF4owQORRCtjhxybV4Fw6fqlrNILvVnOFykImPC2vVslD1hesiVNCH6v7qUHb6B04N0tikpFBFr30A9P4xhQ=="#;

const CAMERA_OFF: &'static str = r#"eJxtjMEKwjAQRH9lyN3Y2SSmgbTgzYN+REEhgogHKfbvzRasPZSFfbswb/Lj/rxhks6IGHx+ZKXBNKPPew31+TW8C66duUTEkx8EgkZnV68x/f9KKTyoqMpKTDbAl3BmbShuMajGGG3YUOgthWBQHB3cnCeSbVuwqXuRvtqaMLs="#;

const CAMERA: &'static str = r#"eJxNTTsKgDAMvUrIHrVpqwhtZwdd3aUKFRxERNTT2+KgBJK8H8+swx5gtNgJlWlQgXRbQdWogYGhSEPxO+oPx8tBlH+CuK//AeJAciHONMkbnclTizN+3vwygb8sComwWYzbnxFxMr2yewCjdyMO"#;

const CANDLESTICK_CHART: &'static str = r#"eJxtjUEOgzAMBL9i7QcqN6QEKc4P+oiqoJpbhaIAvydBiHDgYsle74z/f6JSL3h3ZFOD4B/lEvw0fCPNYx9V0IB0GH8aBS/QKuhA0yJgUJ5tKZX34C8wtulZaWfALZlrsGsOVrWV3e4mW9XuxpNxbJK5EZnsYafszmwDOsZDTA=="#;

const CANDY_CANE: &'static str = r#"eJxtjrEOwjAMRH/l1D0mdp04kUL/gB9gi8TAABID/y9cqrZDK0++e75z+/TvE4/rcEtkEO4CQfThMFIK8iqUA2vPyIsMjqS+LBz/Oc0aZFPiqtyHqV3m/KltLWxkCQZOEOIzIFKFUgGPqEf/bW7X+VWH9ORe3Wd1wDsK7Qk/1Os2rA=="#;

const CANDY_OFF: &'static str = r#"eJxtkM2qwjAQhV9l6D65nfxNCrXgzsV1676goFDFhRR9e89ERdEQkoGcOd9Jpj+Plz1tF80x20jYhonHYLtI5WjLEpKJDTdD/6f9Q/9yrZltDp6S5SxLOILQ41QbUAlUsXGsSNazx02KUuOG4t1wqIvQ5hJdk4FvZ6T+akdO+AybqPFJbMp+6ck/v8mdlTaTXzkG3HVufGt4ragp8L9zGIciKvxMLBghxqh88xUQFEqOV342XwFGA8wzAJApfs57Opx2dONF4xq6ORTU66uWa7Rq03AHtUtsDQ=="#;

const CANDY: &'static str = r#"eJxtjrEKwkAMhl8ldE+8JHeXHtSCWwd9Abeig0MLDtLnN6dYkJaQDPnzf3+65/h6wP3YzIUSGCUUkDFSSfAZAdjbwCbBzR4N7dr03aFC+u6HunCETGnhsKeFmrKrzZw9iTEBkxrl9qSgnlSLCxnoILy4dgtAMRNTEVKp13oW8R+rdYfbAhsyJKxc/AdHUhAedKniuO7RwfgFu3ly68p9A63CRy0="#;

const CAR_FRONT: &'static str = r#"eJxtTssKwjAQ/JVh743ZPJoUkoI3D/oRxRbTgyAl+Ph7E0FRLHvYYWZndsJlyAljpLNi+EZBNSxso4XbFgz5GraiNS3szgszfOhy2EkNFsq6vQVLaHjqw6ZG9uEdfHBgk4TkFYn/tWU6ZtzmMadI7Alpmk8pRyrwURhJuEfShKUsVW3V8BVZivirWvvV/SpPlXhAPA=="#;

const CAR_TAXI_FRONT: &'static str = r#"eJxtjs0KwkAMhF9lyL3rZn/aLXQL3jzYhyi2uD0IUhZ/3t5UUJSWHDLky0zSXPucMETqWMMkR22zW0Zt8wEXwwiFgSlY+cKqai8a+l3sVelK+ENQrv+OZbHWFqyMr44eEmwR1sFdBXZJad5AvGbzeMp4RmJNuE9DTiIDIY3TOeVIIh+RLGGWZhbbYviJlEfCzWzdqv/JC9lER1o="#;

const CAR: &'static str = r#"eJyNT80KwjAMfpXQe2OTdusK7UC8ePEhJAoTdpApQ9/etAPxKOmX0Hxf/vL9/JzgUsyJElCcWLAHB2QxqKPVenEWk8VoqaFTpAMNGIGcakmf2/xDKa8IlpEVXqzK9UtIW4NBw7HTtM6o2cYFTHMNwJj2XvtWuGYMxGsQB9jXfapNbMa8q1uPWW6LzFeQdzEUDSzFsAF5FROrZmPH/L2wHdj/UU6/9R9MlESR"#;

const CARROT: &'static str = r#"eJxtjz0OwyAMha9iZbcbmx8TKc3CzJQTROrQoUMlOuX0xUQqS4V4Apv3+bG+j88THvepCImCMGldKCk6CsBC6jCSi4dvV9uzrV4y0RxIFZgpefgBxuksiaIH9i8UmoOJLxzItVqwmo9dpm29WZBtHXEElopMzqFYGJTMkVKENi7A0qVaGwQsq+zmAJPzD649l4oCF9Ac+8AIXh+wOZm1dRvWHF0G7guQjUIT"#;

const CASE_LOWER: &'static str = r#"eJx1y8EJgDAMRuFVwr+ARsUqJNnAISQKCh6kSNHtpQjeen2PT3yPfqzkj4IbUFS0IL8VASbVd03O+dpoUUxc05j6vHIyKXIu+I5CGn7/Ahw0JGA="#;

const CASE_SENSITIVE: &'static str = r#"eJxNy1sKgCAQRuGt/Mx7xKiFgbqDFhFTYFAQElG770bY4+HjuKVbI3pPswZXMIWFgaXgyhuC+7g1YB3rDDImmQbI4YkVIXnSBNmveu6Xf79iNFv+T8kHIXw="#;

const CASE_UPPER: &'static str = r#"eJxti7sKgDAQBH9lSe/jkjtNcaa2sbUPWFwjWPj/eBYKgizbzDB61NOwTWFPIAE3GYwcina3KProhUHJhh/hFZFxKzUioveRn2eSdTT+wje/AEDkHbQ="#;

const CASSETTE_TAPE: &'static str = r#"eJyFjWEKwyAMha/yyP8540p1oMIOsEMUK1PoYIiw7fbTFgb9VUJ4JO97iS0xVLzzXJMjJQmfJoSySYr5kaojHglfRwN5e+4Bb0MuYYkIjTMNX+nQEJad2VxvX1NNmB3dDVgl062+2sX77aP8c4SSEPqkxPXGYkBv2YphBIPHpIWZ9sY6LULj8n/7A4EAPfw="#;

const CAST: &'static str = r#"eJxtjMEKwjAQRH9l2Hs0O7QhhTR/4NV7QCGCiAeR+vduSqEtlD28YXg76V0+FbdRLkS8hkIQ3k5Bx6q7Avwq18JZqi5ITuc2ktNmyrwBw/IYEY+lUHr0i9ShW6Xn43XHpKNQMNFw8ir4teSNOtPs5uU/cd4uWw=="#;

const CASTLE: &'static str = r#"eJx1jrEKgDAMRH8luAfbJFYL1dnF1cGt4ODoIPl+q4IotGS53OOOC3s8Nlj7aiICMop+JPUxaTD3JbVZ9zWQlmoI9RUcwhu3HVg7y+i0zdEGiBQlMvDTg4ys5vsDq2SiaVcq9jlSAg5kpvzGAjAlID9wApOkT7o="#;

const CAT: &'static str = r#"eJx1T7FOREEI/BVyPePCwrKbPC8xtvoDduYsrrAw0fj9wjO5a+4KZoFZmGH7ev8508fj4VWU/IQR1EjQHW2REnRkFZOTROusmEYDppntlMEnZ03xjwSPbLcEyi+WT4fZswpJYJEMrKCqNPGbF/eMgI9TY4E6PFfnGNdYeqnuXLxrJlfL+8pskiFSoacdL6n+tNCMdmh1REn42+G4PdSNx+1y6SSxX/gNRsZ9qsyVffWzwF9yucQnI7ziKvMHnzZJmQ=="#;

const CHECK_CHECK: &'static str = r#"eJw9yrENgDAMRNFVTukNXJBjCpMJYAgkCgqQKNhfJEVS/Oo/f4/vwrmGnQsSDLRbVDRkH+vK3sATIziJDYrSxhlMXf2XeBIq"#;

const CHECK_CIRCLE_2: &'static str = r#"eJw9TDsKgDAMvUro3trE1CLE3sDJE4gOLoKgk6fXVCLJ8P5yzNcG6+BGJCBaUkjUQgSMngPnrADjhLnqBJp6r6ue0tNi39+uSKObRWx5763Fnn/7AV3LG5c="#;

const CHECK_CIRCLE: &'static str = r#"eJwtjEEKgCAURK/yca85ZpCg3qBte6EgQVTITbfPJBhm9d6zNbSLDsc2pQgQct2hAiT19SPwRZiZGwHNvJ0+2tta0pNiPqmWmNvtWHc1oQe0kCAzQhj8T/oXX2kbIQ=="#;

const CHECK_SQUARE: &'static str = r#"eJxNjMEKgCAUBH9l8R71HkYE6rlL1+5CgYKokAT9fRYEspcdGEblFO7g44GcfCynFjOIQAySYIYURvW/Y1S2xWHXYuXXuSbLYAx11NW3jC13vDWMyo7oy9WIeQDpTB6r"#;

const CHECK: &'static str = r#"eJyzKcjPqczJzEtVKMjPzCsptlUyMlAwU7BUMDRXMFEwNFKys9GHKbEDAEfUDao="#;

const CHEF_HAT: &'static str = r#"eJxljrEKAjEQRH9lSO+aickmQi5wnY2thd2BgoKIhcX59+7dIRbHwA6zPHanvob3DZfOHRXcScl9RIQ3EVkioUMSEvNY1hSfNpQUkUw/1Bf4foWqpD0U/5ssy5tT4EHPrtXtVKDVx/15xcjOqcPHjNlhDObFcpizsRPVvsQuJgU="#;

const CHERRY: &'static str = r#"eJytjkEOAjEIRa/yM3uwUFqmSZ0beAF3k7pw6cL7RzomunFpSD4Q4PH7Y3/ecTsvF4X4XlCQjpCQkUjZa0ihGfmolApYLTRfl62fJmHrH478C+QQG5lVY7kJjLXRyh5kmj+4VijnNQbN0OKf6A+KKtqgeYxEzmJBy5kkGhR2eTs0rj6zf428AJKEPnE="#;

const CHEVRON_DOWN_CIRCLE: &'static str = r#"eJwlykEKgDAMRNGrDNkXGymu0txFoqCgIMWF3r5pyyze4o/YWezaUTJxJNjvzu7XVZlGV3nW98CW6eYFHENCCr72aEUr2gITiQ=="#;

const CHEVRON_DOWN_SQUARE: &'static str = r#"eJwly8EKgDAMA9BfCb0PnQ7xsPVfxA27gyCjoP69K5JL4CWxlV3xJJoJd80qifxKkFIP0b+3rhPhtQ3HwQ4cr00FOdHpF/jRBQTXY27CHzopF9E="#;

const CHEVRON_DOWN: &'static str = r#"eJyzKUgsyVBIsVXKNVOwVDADQV0zJTsbfZC4HQB8NQfk"#;

const CHEVRON_FIRST: &'static str =
    r#"eJw9yTEOABAMAMCvNPZGamgN1R94hMRgkRjE+zEw3HQ6ymxQk+skQBEZGQ5n6u+Yvs8CvCj82LwRD5U="#;

const CHEVRON_LAST: &'static str =
    r#"eJw9ySEKACAMAMCvjHWRGTbD3A98hGCwCAbx/aKgXDwdZTaoCbsARWB3oak/Yfo6kwAvCn82vfwPlQ=="#;

const CHEVRON_LEFT_CIRCLE: &'static str = r#"eJwliksKgDAMRK8yZF80ElyluYtEQUFBigt7+/4Ww/B4T/1Kfh/wHIkXQqo3E/zvaDoNb/pu34k90sMCXoMEQV0rmrEC294TjQ=="#;

const CHEVRON_LEFT_SQUARE: &'static str = r#"eJwlizEKgDAQBL+yXB8kGsQiub+ICV4KQcKB+ntzpNgtZpjYyqGQUk/RRH4jtDfRTOi/EJ6aVQb/DHCcLOB47yrIiS4f4FcXXECfeTP8Az8DF9U="#;

const CHEVRON_LEFT: &'static str = r#"eJyzKUgsyVBIsVXKNTRVMLTQNdM1UwBiJTsbfZCMHQCMUQhe"#;

const CHEVRON_RIGHT_CIRCLE: &'static str = r#"eJwlykEKgCAQheGrPGYfabhoMc5dYgoKCkJc6O0ddfH4Fv9jfZK+F1Ik7whazM2sQ+F1duH/yDfOSJ932BEQFls/9CANxggTQA=="#;

const CHEVRON_RIGHT_SQUARE: &'static str = r#"eJwli8EKgCAQRH9l2HuU5cGD7r9ESushCFmo/j4XD8MMvHmxlUPxJtoIrddKkFJP0UQuEJ6aVcb87MNxNoHjvasgJ7rcggAPP/UYNsA/G9gXiA=="#;

const CHEVRON_RIGHT: &'static str = r#"eJyzKUgsyVBIsVXKtVQwtFAw0wVDJTsbfZCEHQCFAQgx"#;

const CHEVRON_UP_CIRCLE: &'static str = r#"eJwlyjEKgDAMheGrPLKLVjI4pLmLREFBQYpDe/um7fD4hv+J3cmeEylSWAiW3dUtXZV5dJVv/y8ckd4NgcGTD9wOLWgFxnATRA=="#;

const CHEVRON_UP_SQUARE: &'static str = r#"eJwly8EKgCAQBNBfGfYeYXnwoPsvkdJ6CEIWqr/PzcMwA4+JreyKN9FKkFIP0UQuEO6aVcZ8fmy9FuI424HjtakgJzoDnIefeuCNDfgDIcoXjA=="#;

const CHEVRON_UP: &'static str = r#"eJyzKUgsyVBIsVXKNbRQMDTVNQNBBTMlOxt9kIwdAIx0CF4="#;

const CHEVRONS_DOWN_UP: &'static str =
    r#"eJxNybsJACAMBcBVHumDIgabmF0ECxvBwv3x0yhXno4yG2qmnhA8hAUbmboTpl9H3OO3C+/GD+U="#;

const CHEVRONS_DOWN: &'static str =
    r#"eJxlyTEKACAMBMGvHOlFRKLNmb8IFjaChf9H0gmy3Q53PxOjyaooUC+oGKN/46Mpf3wB8o4P6Q=="#;

const CHEVRONS_LEFT_RIGHT: &'static str =
    r#"eJxNybEJACAMBdFVPumDWIQgxOwiWNgIFu6PxELkunu22h7olWaBsuBGbim+29Ms0CD++QDzuA/u"#;

const CHEVRONS_LEFT: &'static str =
    r#"eJxtySEKACAMBdCrjPUhC0PDd3cRDBbB4P1Rk2XhpYfV9qBeeaqSZjExutiR3jj+l+gPIp8QsA=="#;

const CHEVRONS_RIGHT_LEFT: &'static str =
    r#"eJxNySsKACAMANCrjPXhB4dl7i6CwSIYvD9uRQwvPdn9TBgNV46QKjExGFQJPirvi7UX/X0BElQQew=="#;

const CHEVRONS_RIGHT: &'static str =
    r#"eJxtySEKACAQBMCvLNdFDlHLen8RDBbB4P8RDSaZOJx1dbQiI0EzorvE6E8YX2v4/QYVRxB/"#;

const CHEVRONS_UP_DOWN: &'static str =
    r#"eJxNybEJACAMBMBVQnoRiyDCm10ECxvBwv2RtxC58rDaHtKrzizJ5AqmjshwfF04/LcH8N8P7g=="#;

const CHEVRONS_UP: &'static str =
    r#"eJxlySEKACAMBdCrjHWRH4aG7+4iGCyCwfsjiyIvPu5+poymC0WAZEFMnTnG+Xz9/gIi4hCw"#;

const CHROME: &'static str = r#"eJxdzcENgCAMBdBVSAeoFAExQZchHkyMB06wvUAhGk8N/PdbH84YrkOEvAEpECHxjGVI2P3E+e6H6/nwxekvu877EFlt4EBkaiMppqk8FSEt1VfXdc0dGs0Fi9KynXE10HaR/lX6Z2Mk0Y0rBrXlNeXQ+rYesrk7Hg=="#;

const CHURCH: &'static str = r#"eJxtjLEKgDAMRH/lcA82sVqF6uzi6i44OCg4SL7fOEg7lCw57t6L9/Yc2Mfq4h4BHqLMm0Dg7Jjsm32eSdbh9CTVFOuPneJvWNhoUUpz983V5dn8voT2hq7tSR011KFRDqWVIGhbKhyGI3lfcQ01rg=="#;

const CIGARETTE_OFF: &'static str = r#"eJx9zbEKgCAQBuBXEXer+0lyUOeWHiIqMIhoCMm3T8OhwVru5/j+4/S27gsLMBzg7MoZKGZcn7C6TiWrj/F0bDZ8IDBCD986ahMneDES+5KQiuJEJQvW/RypqRGopECesvz1u3YDA049QQ=="#;

const CIGARETTE: &'static str = r#"eJx1zCEOwCAQRNGrEDxtdwItYouu6SGaIpAIwvmBhKDAjHmTz/FLQfhbvmQF4UHWgU7peG/geDBQOeuJXCuoRfsfCptR6Gvm5fWtAOAqKss="#;

const CIRCLE_DASHED: &'static str = r#"eJx1kEsKwzAMRK8yZG/VslN/wM0NeohAF1120ftTSVmUJAqG2ViPp9H4rN83Xo/pyZEYibitnXqGRZTHyNQQp2XcdHYZf6JSkc/KStxhsRGJSpeoDpSYWoLKTp6oJg+JlDrUdhQFcZjM2y6T7KA2pWZYbNRVoUIzTOZ4Sg8XheRkUNm5T/D76MkgLudsNeza/AAXpmBD"#;

const CIRCLE_DOLLAR_SIGN: &'static str = r#"eJxNi7EKgCAYhF/lcI/8f0Qc1LmltV0sMGgIaai3TwvE4fi4Oz4b9xyPDdkJkgLxLuTC56O34/97e4YrYXViJg2TBh0YDAkqkVBJtV4XNZnqVqc3GWQW3Z4X6iEf7A=="#;

const CIRCLE_DOT_DASHED: &'static str = r#"eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgI3D8nl5GZ8LM8rzpM5sSOGJx6WRKlDESfF6GiAM/N40N15/DoiBRlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrXdSQ3KE1dPYpU116gQD0KrMEJye4EkpdBYds8tp1HXwZhNd4W7W+afFvz/YJ1MmyQX9K89HfpslTH8wegzGtN"#;

const CIRCLE_DOT: &'static str =
    r#"eJxlyrkNACAMQ9FVoizAUYcsY1EgUaWC7TlFQ2VZ/wmKoWZCTxwik83xTGj7qrjTVa5bnZ/+2AB9EhZF"#;

const CIRCLE_ELLIPSIS: &'static str = r#"eJx1yzEKgDAQBMCvLPcAzV1jc8kPfIScQgQLCRb6e41pAiHVsjus2p7s2GCPJxZC+sIR7P5r0LF40HO5IlZPM09giYPjrHmtTfrW3F5NeiIY"#;

const CIRCLE_EQUAL: &'static str = r#"eJxVisEJwCAMAFcJWaBaCv3EbNAhSlqw0IeID93eqB99HdwdhTt5eBxeJ1jjrUGmrTmmuRxLkS/K/0J0qBYkK3dl6dRrdK62Ihnf"#;

const CIRCLE_OFF: &'static str = r#"eJxti8EJgDAQBFtZUsCZu5iQwBmwAIsQfPgRfNg/XgLmJbPsY5fRe39OHIu7BIa3uKpTW6t+35YpRAilsrKHpcEQpgCOlOKPwoV8Ru8hsTFTkV7DeQE/Qx75"#;

const CIRCLE_SLASH_2: &'static str =
    r#"eJwlibsJQCEMAFcJWeA9U8ds4BASBQULEQvd3h9XHNyx5qYlgg6LhhB0PretH4W/94Wr7wmCRUcEFzr3VFmQnxKW"#;

const CIRCLE_SLASH: &'static str = r#"eJwtirEJwDAMBFcRv0AiQYqArGWEC4NJ4UrePnLi6vi/096eSsEFNyikgC/Q5E1Zt+mxIlNvw3slj7QCGokT5PObWf3eXoymFhg="#;

const CIRCLE: &'static str = r#"eJyzSc4sSs5JVUiusFUyNFJSKAJSBkoKyZVgrp2NPkTeDgDowQs7"#;

const CIRCUIT_BOARD: &'static str = r#"eJx9jDEKwzAMRa8itIdWSkobsH2Drt2DYypDh2JM29y+UjwkU9Dw4Ov/50qKFSTlp1SPdENYPPYI3zxXacFvDYqCMbiTDYJ7T1Vg9ngnglGGiYHhvB53/OitaJXgYi7xlaDYGqJKRsVi0Ep77mxXYPp0m45MJ8OBji7Np9yEf4LHOgM="#;

const CITRUS: &'static str = r#"eJxtTrsOwjAM/JVT95pcEztBCpW6sbCyV2JgQWJg4utxAoKByg/Jp3u43tfHFZfDcJooZmAWyyslFPQVvDhKSH7awgneHUOSHDFJLM4m2rxx53XBc5jrrrnP9ZvBvZiC6klLwccdRaK6XUz/ihs9OIwqCp8tx9ReLnpmOJrwR3kBSy0yAQ=="#;

const CLAPPERBOARD: &'static str = r#"eJxtjssKwkAMRX8ldJ/Ym5mUFsau3fQH3JW66KKCC/H7vVMRBSXkxbl5lNt8X+VybCZvzaWTJMCmNqhbXtSSwsDohK8cG5KF5oWAXEj2GJWfm7Ec6sqxvBdfO44GJclAH/4owFtEeZfkX8FUn1rRP/rZxaWlQVmd4rtX/1x/Ag40MrQ="#;

const CLIPBOARD_CHECK: &'static str = r#"eJxNjLEKwzAMRH/l0G5aCWFasDN36do9NKHKUCjGtOnfx8qQBA13uuNeKuOzosyZmFD+q9g4vaxmUsJvGqpluhBaJYTZfZdOvurSp6+GIdOdI9SkFwjO7bipfFn3IDR3i8c/yCMeB0FMnOzMnfy+gtVp0KBbvQB90yyi"#;

const CLIPBOARD_COPY: &'static str = r#"eJxtjD0KwzAMha8itJvmCTVksDNn6SFCE6oMhRKMm96+tocm0KDhie/9+HW+R7J5eVgMrExb4I5p/QRGlq3Ke5miVZ6xcO8vpdX71xiNpsC3jnRoRyGhppzLX4LuIKsY5AicJFenyshhCi2p7UmUatKToICgA/BvPXElNE6p3s//Asw+O5I="#;

const CLIPBOARD_EDIT: &'static str = r#"eJxtjLEOwjAMRH/F8p5Qu05LpKZfACsDW0QrUokBVRG0f0/cATow2Hc63b1uHm8Z3tOQU8AjQhqne8oBBWENyAjzEpCKrJssWuq7g6767hlzgiHgmSorDMS2ociWQK8CAnW+3d6ptd4BEwgwP6z3ptbAWalNeXJVrAL32AYkcWTgQlNecS+SX2CKS2Xu/owFqLbu0uznhhN/qx++NTyw"#;

const CLIPBOARD_LIST: &'static str = r#"eJx1jDEOgzAMRa9ieU+LrchiSJi7dO2OCqrZKhS19Pa1WQgSKEPs9/x/msdngSVjizD/MhKCjtNLS8aIYICNLyv/TkNRP+zS1VNdevdFYch4J4Go3DMwNPbIfv5Q3ECw6Sb1HvghdSCwsjd7Z93MQKTxxMihaT1yaehYyU79AeDXQkY="#;

const CLIPBOARD_PASTE: &'static str = r#"eJxljjELwkAMhf9K6J7zXoxRoXbu0tXB7TiHWwQHud9vAtIKJYQk3wsvGd/l0+h5GxacSOZrAYFyBHvXpWZKlpQCo1lN5ho4CON+rJmT+cSIeAzTeAjDaVxtL6SzFSH5uXrXoRvwKg3yD1gWGGnbIGKrO/YntCHv77xwJmRSUvZc9S/UAjKN"#;

const CLIPBOARD_SIGNATURE: &'static str = r#"eJxtjbEOgzAMRH/Fyh63Ni4kEmHu0P5AN1RQg9ShQhEtf18nCwwoiu90Oj+38/hM8AvGGfhOQ4rFrcGwgVljUlmLxHF6xRSMmK495a2u/fQpwhDM3YFc656B4ZyfVbeQbIEqR+J9YHmxeMmsTNmxqAaJW5WAsKmAjpoOhcFjTT0jQf6lr843ZdyoRn8BaoAU4d7ova1yIiiV1SGPA67TatwO/gEf1UaZ"#;

const CLIPBOARD_TYPE: &'static str = r#"eJxtjLEOwjAMRH/F8h7BWVEAKenMwspe0Qp3Q1UUyt/jZmgzVB7ufKd7cR5fmZbEV6bvNGStbv4lhslSRcfprTmxZ7JcuIunddXFT5+VhsQPBPIqvZDQ2Q6mUuD3wJm7h/Z38gztwIlW8spsyDeCFAcNBQctQLgc7iAElLA1fw0zO/M="#;

const CLIPBOARD_X: &'static str = r#"eJxtjLEKwzAMRH/l0G5aCde0YGfu0rV7aEKVoVCMaZO/j5wlHoIGSe+4F/P4KshLIib8p6FooishzxswLIR5QzpOby2JPHXxVFtd/PZFMSR6cIBX6QWCsw3blh/7HTi77qH9nTxDW3CiUs3VuZs/fAGzCwgH2c0itNEKBjozgg=="#;

const CLIPBOARD: &'static str = r#"eJxNjLEOgCAQQ3/lcjtRLoQ4ALOLq7tR47EZQhT/3sNF0qHtS1qX9jVD8Tgg3HHL/KXHIyEkMY3Aezw4ezRCSiXBdXUV3Llkhs3jpC0YpoWAoBdpcbq0+YGSNNq2K5ptO1DEVJ/rZ3gBgi0knA=="#;

const CLOCK_1: &'static str = r#"eJw1iksKgDAMBa8ScgB/qLiIuUzoIhDaUl3Y29tQCo83ixkSLWIB5Ltx3RCkdpaGBZnm7plysmoaA+Sk8X28ghPa+fbpgMvrUfEPcRkZcw=="#;

const CLOCK_10: &'static str = r#"eJw1i10KgDAMg68SegF/HsSH2suUPRTKNqYP7vZWZRASQr6wWlNP0H7QshJaxEzQ+6vC078L1+LdLSfUYvk63xkbwkI74hPsYOQBQIoZDA=="#;

const CLOCK_11: &'static str = r#"eJw1iksKgDAMRK8y9AD+QFGouUzoIhDaUl3Y25sqhWHe4j3PUlgDyunmyYEf42KsH8mPvyefk1aVGJCTxPtqGhvsbMewYm9xj+gFVvkZRw=="#;

const CLOCK_12: &'static str = r#"eJw1yjsKwDAMA9CrCF+gn6GT68uYDAaThLRDc/vmQxYJ8cRqRT1A603HSSitdoJ+Ywpv04Vz8uoWA3Ky+D6dcaHFvC2WH9BOGDM="#;

const CLOCK_2: &'static str = r#"eJw1iksKgDAMRK8y9AJWF65iLhO6CIS2VBf29iaKMMyHeSQ6xArkPtK6JQyPnCDznUzL9zP1ZtO0FvSm9Trjxg63kJcc8A/xA1epGTs="#;

const CLOCK_3: &'static str = r#"eJw1iksKwCAMRK8yeID+oK5sLhNcBIKK7aLevrEiDPMW7wWWyhpRL7dvDtyMh/H9SWEdnkLJ2lRSRMmSnrtreNj1+eXEyGdGH4WfGaA="#;

const CLOCK_4: &'static str = r#"eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGObDPBLtYhlyH2lZE7rHnCDjnUzT9zO1asO0ZLSq5Trjxg63kJct4B/iB1fhGT8="#;

const CLOCK_5: &'static str = r#"eJw1ilEKgCAQRK8yeIDKKL9sL7P4sbCoWB95+9wiGGYY3ossjTWB78P51aGNWRy4v5fi/HGKtWhXyQm1SL5OwwgYZdmmHT6Y/mv0AIciGaI="#;

const CLOCK_6: &'static str = r#"eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbWtMMwwvEeiRSxA7sOtm0NpszhIfS/T/HGmnKyaxoCcNF5nx/Bo9cdPe9eHxg+G4Rmg"#;

const CLOCK_7: &'static str = r#"eJw1ikEKgDAMBL8S8gC1ggUh5jOhh0BoS/Vgf29KEZadwwyJNrEE0i8MO4K8k82xIdM6PVMt1k1zglo0P/eoIIKf71wOCHHUf8UfcVcZdg=="#;

const CLOCK_8: &'static str = r#"eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzdVhGWXZYZEm1iC9CPEJaD5zAFyv5dp+jhTLdZNc0Itmq9zYGzw8uyI63B/hx9AwhkQ"#;

const CLOCK_9: &'static str = r#"eJw1iksKgDAMRK8y9AD+QN3EXCZ0EQhtqS7s7Y0WYZhheI9Eq1iE3EeYl4DqMwVI+y7T2DlTydZMU0TJmq7zxdjg5dmHFd3+LX4Ab3EZcA=="#;

const CLOCK: &'static str = r#"eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGObDPBLtYhlyH2lZE7rHnCDjnUzT9zO1asO0ZLSq5Trjxg63kJct4B/iB1fhGT8="#;

const CLOUD_COG: &'static str = r#"eJx9j00KwjAQha8yZN9nJj8mgbbgATxEiUIFBSku9PbO2IUuUgkhgfnmmzd9vSz1eqb6HAw7Q/UlbzK0DMabsd+t5bG/T4+ZToM5BjjiCD4kSmSJ9UQkpjwzUpkCIum1n6KTX4ZTkxq+nps2EWeEDqWDbxBlnVQ2AbbYk7NI8MK0AA9JJ4n/AKuhkxzcHpHV4LeADK9b7ImFaAmCrhnV8gu8AW08Xzs="#;

const CLOUD_DRIZZLE: &'static str = r#"eJx1zMEJgDAQBMBWFgs43eM0CUTBAiwi4CNPH2L9Gh/iI+E4WBh245HOjH3uNgNNfAirg8MAlhvFET5TXEgmI8oPL+qTvKhpt8S+bCzxW/JguFgHqwKnVqVIo6NQtoTTT26Mzjww"#;

const CLOUD_FOG: &'static str = r#"eJxtjDEKgDAQBL+y5AFnLly8HMSAXRofEbBIaeH/0VikkmVhYGDy1e6Oc3OHgIWS2a5QePBYJGWkzqTWhCLG/SfDS4mCBFfyMholzxKvYK36ZxSBq03zADdVHrg="#;

const CLOUD_HAIL: &'static str = r#"eJx1jMsJgDAQBVt5pIA1u+QLMWABFhHwkKMHsX4TD4KQsCw8GGbSWa6KY1W7ARsKMW4eHhrcz5JnhMrkYzFk0V+/UNoKJEZUTktv5PSV2LXUPSJhBpoiupLmoTRFLGA3DgpEftYDwxw9FA=="#;

const CLOUD_LIGHTNING: &'static str = r#"eJw9jEsKgDAQQ68S3Dt2+psWasEDeIiCiy4UXHh/dAQlhAQSXjnb1bHNwxrBkZyNi0BgwKpAwkidSXLzFKA27/iURFncUMukiFo+0MEObEeH0P2u8V9uMssZVg=="#;

const CLOUD_MOON_RAIN: &'static str = r#"eJxtjTsOwjAQRK8ySu9lf97EkolER8MhLCgoKRDnx2CEUkQjTfPmUx/tecftOF2EiRdDOQUxK4Zzl0AC3pzUDcP5q0BcGUqqSUi5h0giGTJJmae1Hj7ba/0/GJRbRh6bWKgkP4s1g/2OtHfVd6rSGb90h8yQsgFvgzYwWg=="#;

const CLOUD_MOON: &'static str = r#"eJxFjDEKgDAQBL+y2CfmTHIhEAN2Nn7A7sDC0sL/YzyCMrCwMLvlkvvEMQ8beRCLh4cDNRx4TRIRew82G96HWsZ3Uss/dJaQFwY3U11GkGCnAA2ncEcV4xEt5fS9PQTGHV0="#;

const CLOUD_OFF: &'static str = r#"eJxtTkEKwzAM+4rovZqdzHMCWSEP2CMKO+Qy2GH/Z2m79VCKbGEkLFTe86fheR9eAR3SZ5jKZVGn8vceRk8BK1eHQ1ZkaG6JNl9pWHaTlVF8pOZ4khSUFgP0RquHN++nShuVnqtTJGHjny+9gPie+QXScymt"#;

const CLOUD_RAIN_WIND: &'static str = r#"eJxtzEEKgCAUBNCrDO39+b/KVzChA3QIoYWboEX3pwzahAwDAw8mn/Vq2Jdp82BPMaVVobDgnkDKiI1JU/UU0GtflGdFEi9TyXP/KPl7OhIJROCMjhDsjMOIWP92A4dsJvg="#;

const CLOUD_RAIN: &'static str = r#"eJx1zMEJgDAMheFVQgeITUibFGrBARyi4KFHD+L8th7EgxICDz74816PBtvsVgEStJQWBQUPNC6gElgj1FQFA4z3N3JfhizsSp5Go+SnRLGnzvgh9gfEQPElF6hNJkQ="#;

const CLOUD_SNOW: &'static str = r#"eJx1zcEJgDAMBdBVggPEJrSmgVpwAIcoeOjRg/tj40EQDCEQeOT/crarw7FOewSKmFU3AYEAZJNQCHInFG0RE9iGB3lcGTnyVMtsGbW8SXk8dgz0T+oQMZD4xuTZ4teZfftu9RA+Ew=="#;

const CLOUD_SUN_RAIN: &'static str = r#"eJx9TzsKwzAMvcoju1VLlpoY3MxdfAhDhyyFDqXnr2IoFGKC4A3vJ6m82nvD4zZVFshHprVcdmotP+GplBM6MCl3OLqqRLBsozxninMvCGcFbJR19hK6WlMo4j7BWbGgxLIMQgkSm8Hc6sVYKAe9c2oJqecZAiPR0T7X4ujh6kfkP+ELYhNJyA=="#;

const CLOUD_SUN: &'static str = r#"eJx9jb0KwzAMhF9FZLdq/dk1uJmz5AW6GTpkKXQoff7KhnZJCEI3nO4+1Vd7b/C4TSsx8IenuV66Ndff4alYBIYQKg3Zp1aOQLwd9algzAMQzgBkWDQ7BJM1BYXYJ7jLFhSJr0clAeYlNwPztLP7n5AWkiYgA0G+6f7vfgFeCzuq"#;

const CLOUD: &'static str = r#"eJwlizEKgDAQBL+ypPfMgbosxNQ2fsDuwCKlhf/HBBmmGqY88TbcezqdtsJ1KAgiwzub0Sc1NyqWnod/ydCVapnHXj8tUw+0"#;

const CLOUDY: &'static str = r#"eJw9jLEKgDAQQ38luPe8u1qPQnV28QfcDhw6Ovj/WCmUkCwvSXn8rbi36RSjBJUju8HAkKaVTEKuQpZ9afh3J4x8TXuZ//lexokqhD0itkJTiCHWoKRsniixomeHwmSslMbPB+OFH6M="#;

const CLOVER: &'static str = r#"eJxtkrGOgzAMhl/F6m5f7IQkJ3FdmO8hKm64gUodOvXp+9ugSi3IECGw489fGG+X+z/9/Zx+tYpRln4xaeR38mC8UUoLS5HcseLaZxilqUrLVKQPVKU2Xwp16kshBJdZJWfGUknFGqp0EOMi9jidxy/HOI8vGBTOkcxR5skcZchnsXcChFMuAAy+T8L1u00Bpw3brHR4zBmMWg8IFJx13niR5ghBjhltTxCeYCkcHVjaPE367fuEK29fV1lo1g8YmnSytGu2HYl38Yl3rdYDAXtyfckHUGPD9FCnrhCkxYNKWM60qcZ0oRp/wvHBXMHaeKDh9ekJMXV6CA=="#;

const CLUB: &'static str = r#"eJxVjbEKgDAQQ38ldO95PbizQi246w+4FRwcHfx/bEGUDoEQ8pJ0lfvEMbstjCQRE7EWJUUTI4B9YFIDL12KIKiEWV+uTqKPZLvLaWjbOf0PL7JWI/IVHkOeHTU="#;

const CODE_2: &'static str = r#"eJxtzCsKACAQhOGrDHaVhVUMq3cRDBbB4P3xETbJHz9mZNbV0bIZlEARbF+miL9SRD0iWcbro8QunHE4H6obZTgYJQ=="#;

const CODE: &'static str = r#"eJxly0EKABAQRuGr/LmAxmKaxXAbCyUUG7cnZSH1lt/TVvPMqUS0msro3hCDBM6Bdgw2Qe1FQT8uYBwre3vsAgb7G4s="#;

const CODEPEN: &'static str = r#"eJx1jk0KgCAQha8yeIDCgaCFeZsIQVSoRd6+mfGvFuHi6fjevM+k6PMRA6TownVuSiPQQVinhUUvpFom9VG+eKSsmWvcGu/CDhlpAZkU3HxDUl00k6Ik2GgN5yTSerHvHS2t4C/S6V6w30yh4u6B0tAYtpaI+QGhH0aL"#;

const CODESANDBOX: &'static str = r#"eJyFkMEKwjAQRH9lyT2xu41JCrXgB3j1XlCwUNqCRVq/3s2mSDxIyWGXYd7OkHpq5wfcTupCCOiuoSUgKOLTqNH4stde20zlyRLY81eCEsIrA4EvRZBNmRg5vpRhkvhWTX2IJZp6Gvu174Y7TGM3zM+T8uYI1kQbgTMh2jdFoM3+B8TK+Apks8ZxR6QdKgVJhiBpi2d2wNKQ54KVE55MgUAF/4Bov6xwK3IY24KChVcuBiuluaTJjPg/l9to2w=="#;

const COFFEE: &'static str = r#"eJxlzE0KhDAMBeCrhOzLNJ0wjtC6duMF3AkKCiIuRPT2Jlr8Qbp4ffC9+LGaWqgDFpTAv6WKgcECybPSDWHmP2oyf8qvQp7TaEUb+eXJvRsur2XfDQ2sFNAhLBI/hNUFZGlOm0Alb0j2IJrH4kl3E89oxumTbgCLOFM="#;

const COG: &'static str = r#"eJx1j7EKwzAMRH/lyG7XJxs7hTR/0B/oZuiQJdCh5PtrR9B2kBEaxOnpdMurvjc8b9OdAgl1xowAtg6OGTpqMT+mdbl0YF3+Maba2C+WoJNWGkByyECQwxnSztL+8zk7OvoSLZhg8FJQEH1Mxo3zAFjOC47GRgcb33XQ8kigbLOhSBdkaKqeGHpSTc2nqIEcYSffe/LY6YTsr7+ND6UAefk="#;

const COINS: &'static str = r#"eJxli1EKgzAQRK8y5L/brMpmC4nQA/QQJS0oKIiI6O1NFESQhZ0ZZp6P7Ri7P8ZgxCCuwWiSJUvtn0dZ++E7NfgF82El+wJbKt1bILDgfClX4B3J0wvgIA3P1b3pWcgltCRVcsk+CtIC+Z3jDdDKKIk="#;

const COLUMNS: &'static str = r#"eJwly8EJwDAIheFVxAWK6aWHJMu0oQmUHkSobl9NTuLP9zK3U6C3cXcpSAeCFtwRvnFJX4G9JASbnS2emrfY1fyMt4Emhy6U1jWa1DwnChuq/iVvHDw="#;

const COMBINE: &'static str = r#"eJydkMEKgzAQRH9lyD02Gy1aSDz30o8oUZqAqNhg6993E1oQj73sMsO8w4xZehfh+/Dw0YpG4G2FFniFLvqst6yXbLfmlOKtme/Ro7PiRhW0o4KgoFFc+Oi1cgpsySQlGwlLwA7T6i+MONFcz05SUUPJkn/Jt1wl7dLTsA1h7DFPYYxPK2poQkZRg845+c20JtfnjlSJwwrJWQ5r/Ab4AKaLUW0="#;

const COMMAND: &'static str = r#"eJxNjCEOwCAQBL+ywV/a66Yoiq6prSdBIBGE93O4E5vsiJnUy2ioT/j0Rpx6FYI4oTYK3+gZ/B2LveYFMSHkdOxkXnlLE2Y="#;

const COMPASS: &'static str = r#"eJxFjFEKgCAQRK+yeIAtRdTAvIxECKJifeTtM1foZx7MG8b6UH08oO6Mrwz80yk626CzC3lnS47tzAlKDum+ulUoJGjUCrhELmZSMdyGxlD82+9xPrkXEk0fzQ=="#;

const COMPONENT: &'static str = r#"eJyFTTsKgDAMvUroXiWRQAu1J7AXcBMcHBQcvD8m2kIFocOD95L3CedybbCOJnHH4AQekHY7CBNMpEqIHmYTQ6/+GErqQAKCYhbhnw7t0s9PIqHLQ0SfJeT2FHK9RW/aZolV6gajODNM"#;

const COMPUTER: &'static str = r#"eJxdi00KgCAQRq8yzAVKKXGh3qBDRErjLkQob19jv7SZge+9Z1KYMlCIM2WLGqFYlAhpq++4PcIafSaLokNnGvad+VenXqp0x1cm2zdbxkzgLQ4KhCbJgKcPEJKJesgOUxMtSw=="#;

const CONCIERGE_BELL: &'static str = r#"eJxtizEOgCAQBL+yoSfebQihQGobP2BHYkFpYe79og0WZLudmXzVu+Fc3U5oqgQhfQp6No3/AzRuNM/Dlby8Xcmj7kqsCelzxWuETDQlgoUZEIQ2wAPrtiZ6"#;

const CONSTRUCTION: &'static str = r#"eJx1zDEOwjAMBdCrfP0DhCZBSYa4N+AQiFakAxKqogK3x1mqDulgD//5O6/zo+KzTLUI3UCUeXmWKkzE+hVaQrcjfsLAMV/a/Zjf91owCW82wl632KRlBzkFrfjNdxv93A76Cs54BNPxl2JANLFNh5NqQtrlD8joQ5A="#;

const CONTACT_2: &'static str = r#"eJw1i1EKwyAQRK8y7H9p1oaQD/UGPUQwUoVQShCqt++uNuzH7DDv2c9WEnZHT17A6zZjxqR3WzGRt3fdvQ35DEdEqI7YEE5HD0Jo0lihMXt7xlCQYn6lItNK+Oa9pPHW7ogyiy/FqKiCt0d+R1TjSDHu0VgJNKO8gIpcoEy8UBc0B/M3LvQHfyc5JA=="#;

const CONTACT: &'static str = r#"eJxNjcEKwjAMhl8l5D5couiEtmcvPsToii0MkVKwfXuTzsHo4W+S70vMZy4RFotPugFNMwPDqG/ggR/3Yw2MzpyUdyYHX6BZvCBUi2eELMEIMaRXLBZpQvimpcT+FU0FZ3zKfg3gxaQRwYtEYmV1BdrGzqzpHaBR31glZFnlHo31pqCK/EEl6Iq7sCFdkO6O/gAiPTwv"#;

const CONTAINER: &'static str = r#"eJxtj70OwjAQg1/lxH4ml+ankUqfgK4d2KowMBSJAfX5cUDqQhXposT2F2d4Le+H3C+nyXvJyNUpkiKowSt6bnHVhE47lMWQvXyHa4tiFreqOXSSqiK2TGEKhbawJaTqhNeBR49eGo0wOYDJD9ZYmiqigiY1aQPxdhqHc6s6Dnthc+INZbZw9TApsH/Tkybj6/TxF+WIQpWVN+2P8pOxdEacC8KufgD6tEfM"#;

const CONTRAST: &'static str = r#"eJwlyjsKgDAQhOGrDOnF7BbBYrM38BBhFRQsJIiopzcPpviK+cX2bMeKHB15B3uLXHyaKmP/Vc50bViim4lBUwoI8H0D8U381bhG+gO1exYG"#;

const COOKIE: &'static str = r#"eJx1jLEOgCAMRH/lwg5yhIoD8gd+BImDo4Px+60MJiSSXpu2L3f5rNeBfTUbA0Klh0oH2qKKWu1jxUp/mZKn117yF7I4gfbtPH8oZ1DGNIBhxAimAUtg7NADGX01yw=="#;

const COPY_CHECK: &'static str = r#"eJw9TtEKwjAM/JWQ99YmRHHQ9g989V26YfYgyCjo/t5kshHI5XK5cPn96ApjwRcx0BnYSoJgzSdXal6m1mEteEX4bv0zj10LkiDoND+1/+fFVDZYHcztvpr37zcBurRAkSAFDnGwxndpyVfOwLhSatuFZYiDRzli/ABagSn5"#;

const COPY_MINUS: &'static str = r#"eJwlTEsKAyEMvUrIXmvElg6oN+i2++JIFUopIlRvP4mzeY/39Z/6zTAoIFmEaZmvCEP4zpqWjv4itehbTh3+de+FfYfQZkCejYBcLrm+Sz+DuZw2JOa17KL/vXqBPeDDAd2SIk1glFV6Y7BPl4xYooB1IZNWAyzojWEdyUU8AHosLP8="#;

const COPY_PLUS: &'static str = r#"eJxdjMEKgiEQhF9l8a7pYtEP6ht07R7+kkJEiJC+fbvm6b/sMLPfjHuVd4JhvDAoYCDpVUBnfybFqcGdGAtuwn3BfcFjweMA1xQb1OEFsd+yt0xvS5gXVMqpPHP7J30mtTNJbe4F93m0DLsXNwvmEqVRBrREqTY6eLdRc8QOyGej4yQAQW105hBPhB9e8Dg2"#;

const COPY_SLASH: &'static str = r#"eJwljkEKAyEMRa8Ssh9rgpQOqDfotvviSBVKKSJUb9/E2SR5+f+H+Hf9ZJgUkG4Ik6UzwuCTBy2O/qK26FtOHdoMKJ6S66t00R3Crx69nKOIEmxjeYaCpDUX/ffZCxwB7w7omjYyBHbjzexS+OGS1ZUSCBeyaTmAwexS1ht6Iv4Bftws/w=="#;

const COPY_X: &'static str = r#"eJxNjsEKAjEMRH8l5L61KUVcaPsHXr1Lt9iCiJSC2783qa7uZYZk3oS4e3kk6OSRDMK6uWE/IfSPB3cQLLg/vIU7+Fv+wTXFBnX1yMyrLC1zbLnmUeChtY80p3LLbcTcll5wz2vLsHg8W6BjnEgR6MlMamYxFxu1rGQCnjPpOAgwoGaW8YacCG9WFzg2"#;

const COPY: &'static str = r#"eJwljMEKxCAQQ38lzF3XEVm2oP7BXve+2NLprYjQ9u/r2MsjCUliXUpDvRJ5QueHcA7Ksq3SEnEgHNvc5JH11GaOL93luP+bYE70DeB3MWwZznhjpw7/C8VppA7dC7syGvCwU8c40ot8A3t9Icg="#;

const COPYLEFT: &'static str = r#"eJwlikEKgCAQRa/ymQOYpi6EcW7QIWQKClqEtKjbq8hbvMV7rFfV+4B+mdxK0H+6dlkSXmYXfsp7Ys+0JSTjYwkIsHAdi2j8WMciDYnYFRM="#;

const COPYRIGHT: &'static str = r#"eJwlykEKgCAQheGrPDyAqelCGOcGHUKmoKBFSIu6fZMuHt/i/SRHk3NDK8Y7A3nUoL5dpmn8TFe9d6zFLD4h2znFGhHh4HUOyYbc8z/jD9mNFd0="#;

const CORNER_DOWN_LEFT: &'static str = r#"eJw1ykEKgCAQRuGr/LiPVIwQdNZtOoRQkCA6kATdviGQt3mLL3Arb8n1BLdc+x2Vh9FwMAs8rFYU5kEocOoXjqh2K+RZkxOoJTPJbe7HQugDUxcX5g=="#;

const CORNER_DOWN_RIGHT: &'static str = r#"eJw1ylEKgCAQhOGrDF4gFaUX3Rt0CKFAQXQhCbp9myADw//wBe71raVd4F7auKMyHkbDavzhJRSFbSkKnEbGGdXh4J49yUPPSWVjJxZCH42kGIE="#;

const CORNER_LEFT_DOWN: &'static str = r#"eJw1ylEKgCAQRdGtDP5LKkYEOjtoEUKBguhAErT7ZgJ5PxfeCdTrW0u7gHpp447KerAr7OAMSCkMyzQYKI0MZ1SHvFlvyTMyMs31WPdzRvgBlccYkg=="#;

const CORNER_LEFT_UP: &'static str = r#"eJw1ysEKgCAMANBfGd4llUEIc3/QtbtQoCA6yEt/3wriXR/JaHer/QQZtc8rGY8QFapomJY/MEmeBY5ktuAguGLX/CanvEWLO35dEz9bDxf3"#;

const CORNER_RIGHT_DOWN: &'static str = r#"eJw1itEJwCAMBVcJLlAjSn9iNugQQgsKooFKods3FuQd736OpNe3lnaB9NLGHQ1awDBxdoLBMG2rYpI0MpzRHB583pM+WB2q/YPujzXhD4zGGII="#;

const CORNER_RIGHT_UP: &'static str = r#"eJw1ykEKgCAQheGrDO4jjYkIxrlB2/ZCgYLoQG66fZMgb/Pg/0hqfnMqN0hNpT3eOAs7uBUQFn2GaR6ESUKLcHlz/DFuAVXZPpzwxI6V8AdSBRfl"#;

const CORNER_UP_LEFT: &'static str = r#"eJw1ysEKgCAMANBfGd6lJYMQ5s5d+gihIEF0kAT9fUrEuz7Wmp+cygFaU2lXMB5mAgLfkRGe/iCssZ2wB7M5BIe3XeKIOFiytH69J3kBW+4X9g=="#;

const CORNER_UP_RIGHT: &'static str = r#"eJw1ykEKgCAQRuGr/LiXHJmIQL1BhxAKFESFJOj2TUG8zVt8rrdyl1wP9JbrOL2iGcSwBitkWQU3/Sa4HkfC7tX2iksvkcEwEoE1J7IfFxQelrAYkw=="#;

const CPU: &'static str = r#"eJx1jm0KgCAMhq8ydoFUKhhoN+gQUdH8FyJ93L4Uwgr9M7Y97wOvdvPo4TBYI5xxuvtQCDzbhb1B2SLsdvIc105XQej0o1HUKOVT/JVeB88wGexlA2pTAYTXD4gsUSAbzgPK/0XREAWFCq3oW+oCvjhWOA=="#;

const CREATIVE_COMMONS: &'static str = r#"eJytjEEKgCAQRa8yuG/SLCxQb9AhxIKCFiEt6vbNaEEHiM/nwczn2bimuM2QnFBSQDyJDfHK9LYuf2/3cCwwOTEqCQPq0GAPXMmpNHagQGOpfKKxpY3JfS+0Yy3rvlLzm/QGGz0t/A=="#;

const CREDIT_CARD: &'static str = r#"eJwli0EKgDAMBL8S9gO2RW9tP6PFBsRDCRh/b2JPyywzebRdSAsS6C3YQGNCb3x2KYgr6OFDur0BNS8e1Hzx3Ujj7GxiAGkydE4/m+tW/QBOkhpw"#;

const CROISSANT: &'static str = r#"eJxtkLFOBDEMRH/Fut4mduwkKy3XbEPDD9CdQkEBEgXi+xlHaAs4reJdbSYzb7J/3r7e6PXx8uHSSKuoUkjfuIrpVBkbq5Qgxz9S6YOqdF3jPTUG9dBjyCDD4bZkIVbp9Hu5XPeHjLnuZ5gWCdokWNnEtmMToyY+KI0aYT25xGFp1yj3AnNI3HrGr1HWYwga9X/EMzxmAXyAxtnZclFBHqxE+/rwe2xITvKUGdpM6VUqyhv6rxdGfEMwS16J8pLGr/NfROUEZLuDqIO0zUSEzhmc5CiEUpyEy9Rznmd/ANAiXZE="#;

const CROP: &'static str = r#"eJxFyisOwCAQRdGtvOBJOy+kQUzRNbX1JBVIBGH9fATkmiuO5lgS/tu8F1jFRYI4Z/2SOBP0GCbokuJBfn5TS8uHSzZ+uRTQ"#;

const CROSS: &'static str = r#"eJxdzqEOgDAMBNBfafAd62UVJGMag8UvICoRZIKvhzloapon7i6f9TI65mEVIVQQKPbj92u6JCfYI0mQMFF3mDZ18skgMBqrqadfD8NYt+QJ91Dy2NeVB2qWI5I="#;

const CROSSHAIR: &'static str = r#"eJx1jDEKwCAMRa8ScoHWDMXBepngIEgHJ3P7RlOk0Do9kv/+D5wrlwTcTnSEwGKsih1j2CyPoeQrgThLm5I6SW+PIDT+qnftRz6mMzofdfxsTinWaG6xSi/3kWzeT/kG4hA3OQ=="#;

const CROWN: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Im0yIDQgMyAxMmgxNGwzLTEyLTYgNy00LTctNCA3LTYtN3ptMyAxNmgxNCI+PC9wYXRoPr6hDrQ="#;

const CUP_SODA: &'static str = r#"eJxtjkEKgDAMBL+yeLea0LQ5VF+gjyh46EXw4P/RFBRFCexhlgybtrwXLEOzBijIRQGxY80MRl+PDXPxTvyTtkYnUmgzps40Y7pks0AL+Z8igiQH5yNqmIkgZ75Zb+z7vhLbyjYUvssDEdktxA=="#;

const CURRENCY: &'static str = r#"eJxdzDsKwCAQRdGtDG8DQQPBwnEzg4UgKax09zHxk5jqgZ65VkKS6EkyQ2mQlLaJYeDs1r6djeH0lBVjB5U+mnGA8jOV3uQDtVqIMuPwRzt5m0PWwEr746zPbNuBLwvbNmI="#;

const DATABASE_BACKUP: &'static str = r#"eJxtjs0OgjAMgF+l2X2V/iFLgCfQq3eCJJhwIGqIvr0dGsOBLF3X9cvX1sM03ebHAP2rCcQB7p5TgP7dBPPKk4S2Pvywtp675wjXJpwFiLsEAsV6DBjLlNFMbDgmSCgX22kJ2EK6kZSoR/dU1Q5M7AMXHXWvJ8BFZ77E15OigKKtsf5Ef+XoI6H4tMhoXmoUVALy65T95d/9AViJRYg="#;

const DATABASE_ZAP: &'static str = r#"eJxtzbEKg0AMBuBX+ckD2Cbn4QXuhG4d2rV7sUILDlId9O3NiYiDBPKH5IPEtut+/dCimRN5QjMlYiH8LdXCto7qeNlYHfv3+MUn0dPBv1hvCofrWuwhXIQy86wOVthwOD+wPDiAq7vYoBA5Yc7U8VNZeM09VDteAFAJM/0="#;

const DATABASE: &'static str = r#"eJxdykEKgCAQheGrPOYCNUqLARU6QFv3UUKBC7EWdftMIije4ofHZ0KMa9oC8mlJE/JhSQhTCavS8nbkTPMwZ9K4L5gtDRqdZ+kFGm2dYrD4qm/0oax+UL3sAhvvIcg="#;

const DELETE: &'static str = r#"eJxFi7EKgCAURX/l8XYpH4gK6tzS2tAmFBhINDTY36cmyR0u53KuufwdYLM40whi0pFJqAmcewKCsYYYLbIzy7yiM0N5OxOPc4fELXKFkCg3ITyZda6CorjFam4bv09361X97gthyCW7"#;

const DESSERT: &'static str = r#"eJxdjkEKAjEMRa8Ssk9sMqm20M7GtYcoVVBwIYOI3t52lEGHQD78vMBL9TLV6wmmjIpQnxml5yuj4Zg2n+uYbuV+hmPGgzhWGFj3nj0YKASWtmUo2sLNY+AeJEshc2FrwFaA/APVkXGkgT1FCqwUOXSjbvLj01RAjEOJEL/PsuMtuIV9A0S5Mwo="#;

const DIAMOND: &'static str = r#"eJx1jrEKgDAMRH8ldE9MG6UItX/gD7gVHBwcHPx/tGm3thwcd/A4LjzpveDczO7Ig2WS5Gi2oMZV8pfb07JCtgYQjQpgF2DUBRwSWCYOE8OUD8UPV9AfXg=="#;

const DICE_1: &'static str = r#"eJwli7ENgDAMBFexPADg0FDE3oAhEIlwOhRZgmxPTJq/178u1nwaPCWZMtKGoLlcaqM3xhXh/bN2hI7mkDi7J/E+TCEx7hSAgk4L+eerfIrGGO0="#;

const DICE_2: &'static str = r#"eJxVy9EJgDAMBNBVQgZQoxQstN3AIcQW0z8pAev2NvqjP3dwj3MlbQJnjsIeaUYol8exVX2KU95ZXmkwIVTN4Hr9BXeswhA9LmTAcjeQko4fskDmRzd4tSBL"#;

const DICE_3: &'static str = r#"eJx1y8EJgDAMheFVQgZQU0F6aLuBQ4gtpjcpAe32NnrRg6cH/8dzJa0CnPLG4pEswulxRDhyFH5CvUNp3bSpOsH1+gtuX4QhepxpAsvdQEoa32SAzI9ZoOlDF2uNJ/c="#;

const DICE_4: &'static str = r#"eJx9zFEKgCAMBuCrjB2gWoH4oN6gQ0RK8y1kUN4+9SmCfNnP9m0zKewCHOLBYpE0wm1xQcitptLMJXKLK3rhtuTMWO+cOTdh8BZXUqB5mKhSHb5Id4TUD5V/H3sA4F8vVQ=="#;

const DICE_5: &'static str = r#"eJx9zDEOgCAMheGrND2AWkwMA3ADD2GEWDZDSMTbS3XRAZcO/5c+k8KagUPcOFskjZBOiwqh3hHhiD7z08sdUhF1ppc/Z/YlM3iLM02guRtISOKL9I/Q1KC61zYFpD52AYh3NwE="#;

const DICE_6: &'static str = r#"eJx9jMENgCAMRVdpOoCKJqQHYAOHMEIsN0OaKNsL3DTBS3/a1/9MCrvAFb2wRUUIt8UFIbeZyjKXyC04xIOlfTkz1p4z5yYM3uKqNBAPk6qoHt9IzT9Mdxh1jdQX0tf3AKEwPoY="#;

const DICES: &'static str = r#"eJx1jF0KwjAQhK8y5D0xmz9bSHoCPUSxxRQUJATU25s0D6K0LPvDfjPj03zJSO/AFEN6rSvOyzXmwKjc7fNcphzboyhJssEfqnHwjzFHTIHd6Sh6BTLQwvLSoxLKYB2yFdc3brkpsn/GNeSJJFzNrYnf3LMDdVFI2kDFQWaPWbg91KH/QR+ajkKw"#;

const DIFF: &'static str =
    r#"eJx1ybENABAQBdBVLhbgH7pzGxhCorhSIeYXjWi078lo06gXV8EUF5JT8cdU7mRCsM8w3tlo4hYH"#;

const DISC_2: &'static str = r#"eJxNikEKgDAMBL+y5AHaBI9pfuAjJAoVPEjxoL/XWgo9LTsz6nv2Y4M/kVgI+ZtA8Pu/pmP1pq2rou+nPjuXK2GNNLOAJQ2Biy3UXl9iHfQ="#;

const DISC_3: &'static str = r#"eJxtzTEKgDAQBMCvHOlzelFMiiQ/8BFyCgoWEiz0914MooXVFrvMel4SrxPwERQZBXyWTBK1ir4qffTbsM8wBtV3QIZrTWjR6gYNEDrdosnjPIr+IV8qw+XgTyR3kwJZjRbEFN3B17wAo5orgg=="#;

const DISC: &'static str =
    r#"eJxFyrsNACAIRdFVCAv4qZFliIWJFZVuL9GYV93iHrHhNjt545KZbEdrdN2qpPdV4LA/Bzt7phZG"#;

const DIVIDE_CIRCLE: &'static str = r#"eJxdzDEKwCAMBdCrSC7Q6iAO1ssEB0E6OJnbNzFpKU6fkPd/7u2ujsIFPoAjrznljpx8Jyj5EFXystMMmVmdaJ2w499w0k6Cd+Kj2Ab26nAqRtIcHKco/ZcHrlQsRw=="#;

const DIVIDE_SQUARE: &'static str = r#"eJx1zUEKgCAQBdCrDHOBmAmihXqZkhSihQjp7ZtRjDat5H/fZ0zyW4bg4xGyRVoRUrHICHfcc+iNFLP0tfVVgzOT7pw54+WhkEVlLHwRIZFUcnvFqhqWP5+Cy8DU8g/uB3SzvpNBH7mIMk0="#;

const DIVIDE: &'static str = r#"eJxVjEEKgDAMBL8S9gOSgqKQ9jOlh0Lx0FPyexsiiKdh2WGk9llHo5nBoKoLadEyDhTZ4i0y+t1IOWMHWQrJOKi+L7fdKvIWvcEn/uUv+QDipyDy"#;

const DNA_OFF: &'static str = r#"eJx1kEEKwjAQRa/y6T4xM9OkFtreoIeQKihYceHC3t6ZWMRFQmk+4f28DBmep9cV57GZKYIXR14iyEfHPvQM0YxofZx7MC9GtaJNo040o1PaTMPBRNPw0zEoLuJFRGvJp9RpUnCy2rJkS9bJLmuztqBaqUOyOVwFtziazv4C7kBHE1fkYnPo3fYVMAf0qLBkR81MoNJcmYc6D6CE7zv86e+3xwUbjw1zgzdpauzbLW+1aqXpA6znbd0="#;

const DNA: &'static str = r#"eJx1kDEKwzAMRa/yyS7XsmLZhjQ36CFKOnQpdOj9qaShdHAwhg/vP1l4e98/Tzyuy62A66FJtZGCJYkIMkomXfbt4rV9+5UHSjk4tdGJ0xgdJVXuJJar5Z4bVcsyUdkKB4WLcClchEvh4sR9cYN63e8Mr7B9iCeogWNJzE0xwmuqfia4ZAycMHXVJzNmD3PwfM4z2P7b+d/4LwfzZpk="#;

const DOG: &'static str = r#"eJx1kk1OAzEMha9idR8TO/5JpFKp6hYuwA6VRRcgIYE4Py8zhXbRajQeZxLbn5+z/Xz9PtHb4+ZZKjlL6gFO4+xKnU0bKUcOCnZqx6LctbFlMRZp+FtrFKMkrp2zNhJO9cUqsoQHCVYaUhClcEcUYXOkbWbI55vd9mFC7LYXFENErjjHivNtINIzcd5GQ2KHh03646ErHgJPOfOUK55y5ikXHrjdfQFCQm3jDlAnsZ/bqHF/C3VQOmBPoH8StJ/vhdPn+3IjwiA4zTDLvTQWx2r91OUx1GL3OMBBe9op2TEohcbTfvXZBWTpJdjMFu3q7FVC51aZ8kGHVsdHGajTcLB23XfuVWi1s9KS0I+4BnPBmETtEFsCs6jxj/4Lg1t75w=="#;

const DOLLAR_SIGN: &'static str = r#"eJxVTEEKgDAM+0rpA6atzCGsPe/iIwYKE0Q8eHC/dxUvEkJISBL37VihsiAzwk2C1LQ2Ncuv1dhZS+OZrwKL4EwBfJqcz4PzYOw/hPLPyLI02oWN9QHjwRoQ"#;

const DONUT: &'static str = r#"eJxFTUEOwjAM+0rVe0yTlLFJbc9ceMQUkEDigCYO8Pul22FKHEt2LJfP/H2Ge403STgHTrM4dSQfJkEmvfLo8tT16TAGysie6OvHMUDDBWwEhRBGKDEEGls59Z5W7LXY+xHsVyNLDPbfealxe9rttgJfCyHo"#;

const DOOR_CLOSED: &'static str = r#"eJxty6EOgCAUheFXOaOj954xR7iSLVY7m4FocDy/UhiBnXT27bcnvwX37k6NoFxbJghp8/Q84vjBqsElW1uUrKc/SKFMRAOUdRHt9gElqxzE"#;

const DOOR_OPEN: &'static str = r#"eJxtjrEOwjAMRH8l6l6Tc+xWkUIldlgZ2CIxZGBgQP1+riBVHSIPHt75ncu7flp4nocbUrCWqgYNkQNuXWHDUk5bZil7kiC21AFUkOQeoVBXiehfmfjEskngcwW7fx+MEDWVPF+d3rtvocN/EIczk+1lIy4H8BfisZd9AaQ/OCY="#;

const DOT: &'static str = r#"eJyzSc4sSs5JVUiusFUyNNIzVFJIroSxioAMJTsbfYgSOwAHXgvJ"#;

const DOWNLOAD_CLOUD: &'static str = r#"eJxljDEKgDAQBL+ypPfMHRcugRjwAT4iYJFGsBDfr7FII8PCwsDks14N++I2BSvFlFaDwYM7gYwRG5OlqhTQ5z8p74skKq7kuTdKHiUWsNzpb44INmhn0qEf09kfsA=="#;

const DOWNLOAD: &'static str = r#"eJxNzDEKgDAMBdCrfLIXTbS4tJ1dPISgoCBaUMTe3rQ4SIb8Dy9xcbwWTJ4GYbC921EgqHXYaOrtvxu5TUvBVfkouHhsaVv3GfFY9+v01IHVif4B51zoh4IrNLEntoQknhrCk6volrLVF/sCTvUm0w=="#;

const DRIBBBLE: &'static str = r#"eJxtjL0KAkEMhF8lXL/jJpv9g71rrH0IWQUFCzks9O3NaqHFkZBJMnzT+nXttzOt88R+ov4yFdPnR5e2+/pLux8fFzrN04ErOFCEr3uOECG7ldhbQ5UEEsdaddCD+mOFkc0VFO0uIYljKDt7jAjHCaGQDdlgC2Ky9By7ImRKVhUqVIgz8o94A1O6Mj8="#;

const DROPLET: &'static str = r#"eJxNTTEKgEAM+0pwb7W9q0VQF2cfIefgKOjk6+05SSAkISTjud0H9qlZRaG6ORzdBycvHSkJJR4okbFdIY1yoGcrxAYNr8hRyIhs6SHCAoOkSvbfgz/NPLb1cH4B+JcZPA=="#;

const DROPLETS: &'static str = r#"eJw9TjsOwjAMvYrFnkecxE4ilUocgKknqMrAiAQTp+clQxXFlu33W9779yXP2+VRRR35SEgSpQRFy2wF0SRyUg+wGnh2TlVDhvatInVxVJMqRnYYs6LYYBRJaGVQumRU37Io18LGnR2R95QIbzJtymy/y7pcR6p1ObMRb04fv2tEZ9JZ43y0yYjpgFGOnyKdUg77ZF6MdgbbHb3JLJMUmKTrwNbT7w85Azev"#;

const DRUMSTICK: &'static str = r#"eJxFTztuQzEMu4qQXerTx/J7wGtu0EME7dChBTJkyulDJkgymLBNkSL38+nyKz+fhy8fVkOI3xrmaT20LC1Dh011c3zHRuBrtrZh3nzqZhUStqUSZJoPTasWEEnlirnyJ8yStDGpFVg7JkptODTRaktgbxfobFBJca64rRO6oGG2YCSvh+P+wQLH/Vnj393iXqPZooVwCiTlWQT74T6aMRJ/tT1gIccgpKPkraAmzZEw449ud9/X5hvxnkEj"#;

const DUMBBELL: &'static str = r#"eJx1zjEKgDAMBdCrfLpHTRpbh9i7CA4ugoP3x3QRhEiS6ZHPt2u7D+xrOsswox+zb2o2dmn2ujCEySewjAx/C4QXiEBJo0SUH/G0CZVqFKhe42MPCnM0mw=="#;

const EAR_OFF: &'static str = r#"eJxtjk0Kg0AMha8S3E86yfwKKngADyFtoYVSuuhCb98XXdiFi0wm4b3vpfvM3wfd+mbKJJXTHDiRlSdBFfJX74RT4VadclJS9tEFjrkZuou5h+6PAQQMXBJLgC8WDhk+iSfqyhVRksaMvLxlWqq0O4WEc3AcIyKrAOZbyMtZrqgRRnv14CTjTIJ/mHH3tjZqVcCkHpzX832nVfpGG1oVDX3Zx2UfITXR8AO3CUTW"#;

const EAR: &'static str = r#"eJw9jDEOgDAMA79isbekKQ0MpT/gEREMjAyI95NWUFmWB9uXL71PHOuwCRafVHxCNSFURdBOECfNgTRaF//ezaCh5LEySu6kkBqKbcZtanKWT1AGf2jC1K8vtAodcg=="#;

const EGG_FRIED: &'static str = r#"eJwtjrEOAiEQRH9lQs8JhwuYAI21P2Bn0EQTC3NnoX/vLGexL8PuMLulP5b+vGGpJkxi0L/V+HmoD5WnamW3mVp5Xd53XKs5BeTuLH+AXhsRBwUO+ykHBCSOZOWYzdi1LZbWYUyD+nKDWwS52mRnlkyi6eD2P+Wou7xj8oGVz3qVXtN+E/gqvA=="#;

const EGG_OFF: &'static str = r#"eJxFjk1KBEEMha8Sel/P/FRSHehpENceQlRQEHHhYub2pqZlhoLkpfKSL9vPy+8HvZ2W54Bl0jU+OSyUVogP6ggnYciapUvqa4PN/24k0EGJdBqzxWRQjwbWIC+ZTeB9zLXDmyHUl317mNB9u6El4aYkhnX4o3TwoCPy9c3+Pzdah/XalN6KG624TbgwXAOiTcGrwlnLwz3mhTnuzK/P73c6y2nRhc5aqfLlKC9HWdZp2v8A5rc84w=="#;

const EGG: &'static str = r#"eJwtjTsOhDAQQ68yoh9vMp/MRgJuwCEQFJQUVJyeBNHZenr2eK7XQfs0LFlIZCsQZSSnwD/Y4dGSc04MLWxQY0V1rqiFP6JwR7JWs5GjvIBeJchgShkS1JQ+28k9zOOvP88P0ZEa4Q=="#;

const EQUAL_NOT: &'static str = r#"eJxdjMsJACEMRFuRNLBMYA9CtBsPgnjW7k38IHh6TPJmpOSaXOdAnlxDoF+hCRo77BrlMyfKNLdiLxh5cVcel9cc7typXHUAUPwgmA=="#;

const EQUAL: &'static str =
    r#"eJxdjEEKACAIBL8ifiAUOgTqbzoE0dl+XwpdOg2zDCtzrA5OihXBWZEawr4a4IBJicYky9joS9Pz4bUHuLQVuw=="#;

const ERASER: &'static str = r#"eJxljbEKgDAMRH/lcG80rVUK1dlBP0Lq4NCCg/+PSUEXOS7Du3AXr/0+cUxNGWHZ9OTUyXCVJY/OOOpzoMGIk1AIlSsUXfY0wCuHqv5rsrKTumaOrfbP8V3ZrBW+jP+keDAjIHzRA7T1JMo="#;

const EURO: &'static str = r#"eJxtjTsKgDAQRK8ypM+abP4QAx7AQwQtUlp4f0xSiIUsszDzGCZf9W44V7FbaNU0i5KXkZX8JbalH6ATfA0UMKTGSUcseQuUMDQzeGg+FCxZGHKIHcX+uSNDUXYzW+/AA+FjIqY="#;

const EXPAND: &'static str = r#"eJxtzTEOgCAMQNGrNOwgLaYyIDdwZTdxYIDEwXh+i8bowNI0zctv2NcjwzarSgiEmjVXBj71aHy1IDO3VcUwNBnD6xcHyIYSoTCbxciFsEiggyU+GZ/cbVuwXVzR8qqb/vATduUvL5SdLq8="#;

const EXTERNAL_LINK: &'static str = r#"eJxNjMEKhDAMRH9lyF3WpKu40Pa8l73uXVBQEC0oon9vWgUlkMmDl7GhXjo0jn5cgc1a1gJBrsOZXt/iyZn8q5uh3JXk7St2eBumYR/6sUWY+nGZHXEBA+FzfZJ4Kd4mcRNHwoRd0xA21p9cMeY7+sk9AOKPKVM="#;

const EYE_OFF: &'static str = r#"eJxtjV0KwkAMhK8y7Ltxk/0ttIUewEMUFRREBH2ot3eTUn2RZb8JZGbSP+bXBafBHTqqFYo5IMCD248k0eDGfq/Osf/62VMJSOTr1MYYsNJblAXpWHT0KIaZA3HGSjPtmHKBUK5/2jNlhmJqiSQWVPH2pB14hq25oxJhWLeJQqfl/Ou9Xe9nLDI4EYf3ptzUYTFpVjWNHy6DPJQ="#;

const EYE: &'static str = r#"eJwliz0OgDAIha9C2BttO7gAN/ACbgZNNHEw1UFvr8DAx8v7oXO+N1gYxwK5XDUNkHsHBFL9z4X5jgmFOtsJ6d70WEFfxlwQ9InfGKuVIpYPvrEZCg=="#;

const FACEBOOK: &'static str = r#"eJwdjKEOgDAMRH+lmW+gtMsmxjQGi1+COIFAkAq+no2cuCdeXrnbAzrXsEumBawtUqR5jDu5bskN6hnmnKGXsIHtSE1Ifq8/C/QNtUwjVj9wLRT+"#;

const FACTORY: &'static str = r#"eJx1jbsKgDAMRX8ldA82QW2H2NnF1cGt4JDBwcH/xyj4RAnkdbj3ypwXhbFxHQP7bA38XrYp1fcHch8nDFAdo7woGm0fN/DgkhSbfZIzhAJQVPoi/EdekhVwBCwG"#;

const FAN: &'static str = r#"eJxtzzEOgDAIBdCrEHcQUNuaVG/gBdxMHBw6OBjPry2JDhoSFl6AH/fl2GAdqkmYgnoQR43vF0ccFKzzXYKBnAT0xKypo1YUhNruC6EQ4wmzgcJ/ZDHmExaE/zvtrPmUiem5GmOdA4zxjXH/pSexPLMLEfkw9A=="#;

const FAST_FORWARD: &'static str = r#"eJxdy0sKgDAUQ9GthG5AkuJAeHY7IkhfQSfuvh86KmQU7rHiz395RvE7f+8ZGMEDEtgWsWMcIdk2y2QrURdkF2pAS18BljccAw=="#;

const FEATHER: &'static str = r#"eJw1i0EKhDAMRa8Ssp9ME0ynhdYTjFv3goKCiAsX6ultFAn88OC9tHbbCH3GRhxJBSxlOw8enN0nUBXv+SuwI205joH0xDp9La3TPC0D7JJREHbOyB7hMCx8FA6mmvSq5vxI8Y7iI7E+UfmvfQE4/iWf"#;

const FERRIS_WHEEL: &'static str = r#"eJx1j1EKwjAQRK8y5D8xu7FmA0lv0EOUKFRQkCKitzdpof1Jvwb27czuxHyf8+OGOSlWyN+kqOpv0T6eVtzH1/iecE1qIAZ/zhXV0Q6eFyOgTjvTgRuYrfHwR3io7gBnHPyRmRa3boUHMMNpgYM0wqXgqQWovCzGj6HctiBYXerZbfEPQnhL1Q=="#;

const FIGMA: &'static str = r#"eJx9kLsKgDAMRX8luPtIJFahCm5dXN0LDh0dxMGvN3WwSFsJpSXNOVyid3s42MZiYeCK57Zi8KeRQujlRQbpVKaPPh/gKiZde8ekXxMSkJM5G+ZRqgElqpUyCFJEKLk/nVI6KZwBh2R47Hz6KMuPh5KeIb8ETwTXDVYDTAY="#;

const FILE_ARCHIVE: &'static str = r#"eJx1jkELwjAMhf9K6D2xCW2t0O3iVa+7SxU2GNuYY7h/b+YUdvGSEN6X914ablMN98JcHYhULlskT4IMFJDJnT0ICXjSDQGkjuQvYuFIvhKbLZBH1RkpgOJIbh2n7ZlCjWLKdFhDyjT07dI23QOGvummZ2FYM0FHBHWMH/CLlCk3Y24fMBZGDOSXwlb3oqddwU1Wz1991k5V2IXtBJYZ+Y8U513FNxN4Rhk="#;

const FILE_AUDIO_2: &'static str = r#"eJxFzcEKgzAMBuBX+eldZ2OdDtTzDtt199IJFooWJ2W+/WK3WQptSb78ab1eRzw7cVcgGqXSBEIRD2X0qPPqJlVega7n1Mr4F0j07Wkf71s/u83ZaYCf7bS+OiE5DXw1oAJNhD/C+L+RRR2yUiuoGCuZF6FMscYuxg0w705cBMzGubXAws9uvt1DxXK05WGT+gAviT3S"#;

const FILE_AUDIO: &'static str = r#"eJx1j8EKwjAQRH9l6X3XJF1jC20vXjzo1YO3UoUWSluwBPx7J0WlChJYspk3zKSY6rmla5mc7E625Fwr2wYXQ5bFkRVl8XGoeJYcA09noEerkT/4hiPOlsRFTfxeYUxJKYWgpCFNqmITc6piGvtH3w03msZumO9lYpWQopSRM5Qt4AsB/KlmoAa2tQOMZjhKJqx2xn5Zxbyd/sdo+Nto6I8RpfLAae3JA1oy4Fh95QljR0sW"#;

const FILE_AXIS_3_D: &'static str = r#"eJxNTk0LwjAM/SuP3jub0GkO3c4e9Op9oNDB3AqOgf56s09KIAnvixdSM0Y8K3MnX5Tg67lhMNw8Vr+JMkAvR+IcsPy4FOVtNf9MHU5zYB3S0H27tn8hDW0/fipDXk26BOwgi3CTqHjvICA3SZQsZ6PeSgm89Qf1B6d4MEc="#;

const FILE_BADGE_2: &'static str = r#"eJxljLEOgzAQQ3/FYr+0viPpkjJ3aNcO3SJ1YKnUATHw9RxEAiR0w9mW/fK/DD2+9+bFNkToIxWF4rqcuBp5CPxrTz0Gou9biM86npouXxZgl3esglYM5m2u1ITq6qXPefRjC6oTiRiimFDMNcXd1p4BOzQqBA=="#;

const FILE_BADGE: &'static str = r#"eJxNjb0KwzAMhF9FeJdry7+Dkydo1wzdDC04YBJDs/TtqyRNGw6EhL67Sy0vBR6duFkIg80EBIqlgZBKlO5KCoJ0A6n/D3kr6EWfLqu9T22u7zpOT2jzOC2vTmjLLI8IbI8b+EUYPhod6JANmK2Og9HDfu3y91PB4QmgvXRrLlU0qNGwV6OT7gd/ALbOM/I="#;

const FILE_BAR_CHART_2: &'static str = r#"eJxtjr0KhTAMhV8ldK/X5GrtUJ0ddHUXFBREC4qgT2+sP3SQQBLynXOIsfXSQZOKEqMgBspVTUAQniV5W9E78KQOyT9IqpIgLi7zLjLzOwMzY6dhG/qxBTv14zKnAiM2cdNAIWgnvCUsfn9giV6l8nIepB3BD4LKof+LDus9N5k="#;

const FILE_BAR_CHART: &'static str = r#"eJxtjr0KhTAMhV8ldK/XhNrboTo76OouKCiIFhRBn95Yf+gggSTkO+cQ6+qlgyYVJaooAcp1TUAQnyV5WzE48KQOKTxIqv5RUlzmXWT2dwZm1k3DNvRjC27qx2VOBSo2cTNAMRgvvCUsfn9giVmlCnIeZDyhD4LaI/2iA+sCN5s="#;

const FILE_BOX: &'static str = r#"eJx9j8GKAjEMhl8lzD3ZNum0FUbPHtbrHvY2jIJC1QFlwLc3nXbQixJIQv4vf0g39vcj7NfNzjpqgXlrY8/AYOZg5L9A7W8Rt/4loXaTazbdT3bYdOM1PdLpcoDxerrcb+vGOkU1RWADcQYrovBylGkVwApZHpA8icd5QIZrE9wkxHEwQIFJ8kjiIiXREmUgLyQr7d1cmYwHkwSriOqrK5htS1VXLLaotqrnoURc1IRlubfllKb6dLHGcvj/7f360VnxgE49lI3tBwC+ALsMTC/hCXGWYyA="#;

const FILE_CHECK_2: &'static str = r#"eJxFjbsKgDAMRX/l0r1qY3wM1dlBV3dBQcFHQRH8e6MUyoUQyMm51g3XjLFSHYNoNjwQCMkf0tQXUdYajjJQk4eTlu1mVdv4e6+tO9ZnXfYJ7lj266yUERtklKAE5Q96RGDfuKUw4pWwDq4XV/slhw=="#;

const FILE_CHECK: &'static str = r#"eJxNjs0KhDAMhF9l6L2uCe2uC9XzHtard0FBwZ+CIujTm6qIBJIwfDOJ8+XcoEpVTiay4N+7ZDDiUFq2hR6CTG6In4Lm4hPZ/2neVOZeITBzfuzWrh1q+LEd5ilVZMQkLQHHSA7wQgS+fui/IBuOwGhzZ+2HsSkT"#;

const FILE_CLOCK: &'static str = r#"eJw9jr0OgkAQhF9lcv2t7rGckHA0Nhba0puTBBICBImRt3fPv2aKnW9nppqva4dbMBf2cK5zkXLswZYcmMSSTyLkLZUqemoOlJ9ZFHMnH23CLYNc8sgfRR8zCDI1BPLITF3tUkldzdOwDf3YYp76cb0HwwJtERRwexRv8IvUVeyXOLSIm2LeID6DKQyWYHziPq5G/taXWse6TLPYk8sbln/xC3JoOUw="#;

const FILE_CODE_2: &'static str = r#"eJxNjc0KgzAQhF9lyD3WHZM2hei5h/bau9CCgtVApdC3Nwb/GFgWvm9nfajHBq9SPQzIRkxNEHkKNZ+XzN7FZBa8nXek4/YzqvKn+bzyYej+Xdu/EYa2H7+lktiGOByYwyVxUaK8fPxcIQ6FTjl0rdhCqAukbHgCbGwtqg=="#;

const FILE_CODE: &'static str = r#"eJxNTssKg0AM/JVh79pNulYPq+ce2mvvQgsKVhcUQb/eREWWgSQk84gP9dTgW5o3uTQDPx81g2EViUwzRQvp3BDHi4Q/eZq9DvFqKn9Tw8qHoVu6tv8hDG0/jaUhJyIpBdii2IknRcjnD3+yoLvGKiKz6+5AuYYqrvsGYW0xgA=="#;

const FILE_COG: &'static str = r#"eJx1z80KwjAQBOBXGXLP2t38tdD07EGv3ksUKihIEdG3N1HRHiK5JHyTyaZPxzmdDkj3qLxCekTFRmGOyqihX7116C/jdcI+qnNHAWzJauo0vSKFFgFDAuaifwKWPNhTIJMjFQ8f1/kRrt5vwU25XXchUybMJTlR8bb8gEtJ3QNZdKXe1MbbsoXc/OQrZOHI7ewoEDR5MUTL1JLbSIOQSZqf6bxb++VZM7lv6xNedWi3"#;

const FILE_DIFF: &'static str = r#"eJxtjDEOgCAUQ6/yww7yq0BMkNlBV3YSB0YH4+DphTjIQDq06Uvrz3RlOhax86QMYbUJBNJVsqSbm6I4MqMtJKJTZvvGjwh+qIfB/7cgHqPrkJlYZ9sHrgEvUZUo/Q=="#;

const FILE_DIGIT: &'static str = r#"eJx1jTEPgkAMhf9K0x2kL+V0uDI76OpOhFg2Qy6o/947HXCQNGmbfn3vxXm8JvJxunkyDkzz0xhM3/4yljwe05DcWLmLu/LfxXufnAbjsyhhCR4KKscflAlctAeBmk+hwmVftyfRuiUcw4qqvC36x0QaEniO2GAHX1VvAzs2Rg=="#;

const FILE_DOWN: &'static str = r#"eJxNTk0LgzAM/SuP3utMtK6D6nmH7bq7sIGC04Ii6K83VZHyIAnvizhfTw2+pXpTnhjws6gZjDRAyzVTRMjmhjgmNH/uiXkd4VVV7hYKK+eHbuna/gc/tP00lopyCcmw4BR2N54WMV8/iMXOuoh6Tun/ABlkATq75A1ZmTCn"#;

const FILE_EDIT: &'static str = r#"eJxFjt0KwyAMhV8l9N5MY5wKtk/Q3e5id8IGFqQV1pu9/WLZDyEhJB/nnNTyXuA+DhcGY9FdORMQaCkDpKgEdDNp8PIi/f8p2Ypy6IYpnbrGlNpWX3VZH9C2Zd2f42BYaBkBRCAc4AcR+GtrNLJQhGeTCcVTunv3LfpjzB6jAzIgelQxRmX7wSFbScB8+2V4A1iIMYk="#;

const FILE_HEART: &'static str = r#"eJxtj7tuwzAMRX+F8M5b8SFZApzMHdI1QzcjLZAARmKgWfr3oYO8hkDgFSWeS4nDPJ739LPqvpzK1kclpRRLSFn3FXmjiXrkraZnjSP79G49fCzu9TCfpv/pcPyl+XQ4nv9WnXigIZXCXa/gDQn4/qAkaKPQflS40VWW/okVpTCy7jgQiEZeYIw+PmQTwzLMly3C37mjkC3c9mpuccdoGc055osTlAy9bwoySZ0MhQ057wTKEUIC8ehXIY0X8vsx9AVCNUbF"#;

const FILE_IMAGE: &'static str = r#"eJxNTssKgzAQ/JUld9PsVquFmHMP9tq7pIKBVIOVUvv13ahIWNjH7MwwOrRzD89a3DGXBdDt0hIQqFgZbx9MAJ7UI6VARo9SFs0m/gmjT9HQ6DD6xbuhgzC6YX7XAnMWcauAFFQrcacYbd1kfQf2yzQlYKoFCbALX+dI3N7suUd9sQOWGUp1XVuaWFa8NsjJ6AjzBz/NOSY="#;

const FILE_INPUT: &'static str = r#"eJxNjcsKwjAQRX/lkn1qZkw0Qtq1C926LyikUNuARejfdxr64sIwcM7cCakeIt6lelowR7I1g2FyWPPrWrgH2cKB75cdadn+VlXhNJ9XIfXt2DbdB6lvuuFXKpI2yPBgA5/FRRF5/SiGi2QONQv53kAeZ52z4Qmf1yz6"#;

const FILE_JSON_2: &'static str = r#"eJxVjUELgzAMhf/Kw3tdE+rWQ/W8w3bdvbCBgmhhUti/X1tqrQRCXt7LF+PsNuLdN08F5pGUZTBkKhb8urXdg1Tbge/XwxJh8qoZzCWeD8at82+elg/cOi3bt28o0BCaBkvoFMyRED4+ElsCZWaYPBVNUaMonN20qf7vSA3SdUiQFxWzULN5tmVcFOgfdmdBJA=="#;

const FILE_JSON: &'static str = r#"eJxVT8sKgzAQ/JUh99jsom0O0XMP7bV3oQUF0UAl0H59NzbESCDZeewwcb5fBzxbdae6asDXc89gmHi0TIEKQl4eiEtC8+NSNbf/8ld17hQDO+eX6TON8wt+Gef13SqqZUkuCzawmzFZxJw7GEg8gVIBmQJlTBEjIxzVjSkK7P8C2dKlKegiNMcm8SibSOTUHwvTRQQ="#;

const FILE_KEY_2: &'static str = r#"eJxNTs0OgjAMfpUvvYNrMxSTjbMHvXInkwSSCQtykLe3Iv6kadPk+3WpmTtcPV0s2NS2EQiMDkMy6cq8OIvBIS9qMT8s0+9kqXK7l7xyaYxL7IcWaeyH+e6JrVL1lFB1uRI3SuVCP4XYIixK2xMmT0IID0+r4RtVy63XjTXPZDYvoPsX+cGPYNa2/IWeiDg32w=="#;

const FILE_KEY: &'static str = r#"eJxNTcsKwzAM+xXhe7rYLNklyXmH7bp7yQYdbFBKKW2/vg6lD4SRkWwptHXf4B3pydfKQe6+FghsgdFt4JOgLA3LWTDyulXusT7PlMKlBKaQv13+fdBFEkIeI7FVnpR9OVrtFLb2P3uwNSVF54g5fAdmKHZvASy2Lj0="#;

const FILE_LINE_CHART: &'static str = r#"eJxNjrkKhkAMhF9l2H79Tby2WK3/Qlt7QUHBY0ER9OmNByIhB8PMR6yrlhZ1qgoKvQj8jysGwz9Ly7XSR5DNLfFX0FwmXpTf4V1l9ncCM+umfuu7sYGbunGZU0WhhGQYsA9zGR+LmJ8fBopBgQ6EJa0FnhtQ8lIPJsUrKQ=="#;

const FILE_LOCK_2: &'static str = r#"eJxFjsEKwjAQRH9l2Hs12SaYQ9KzB732XmwxgdKWGqr9e7dSlYVhYd4w46cmR7SBrga2Ng2DoeQ0uODoDvbCCqeDrVn9vUK+s6HKH7d05aexX/s0dJjGNORHIG0EFXGQtPuAO1L5ubtlrMKUhGdqcwzkCLFL95gDWcL8EpMgylty46XiO9NBl0vB+xgNVRiohX9j3jHNNxI="#;

const FILE_LOCK: &'static str = r#"eJxNjr0OwjAMhF/l5D2ldklgSDIzwMoe0Yp0Q1UUfp4eBySoPNj6dOc7f0slYwx04m1nIQeXBIK+jdGr8grolsyyBkbOu84ev+YXRb9pD6NfpkvBI9Ce8AzEQrjPY8kfkKf5mksgR1hUws3V9NH/y1iwVCNpwKAxrGUc+iq/gDfTqC13"#;

const FILE_MINUS_2: &'static str = r#"eJxFjbsKgDAUQ38ldPfRa6sO1dlBV3dBoYK0BYvg31uLDy5cAjlJlJu8xtywQYBIczERCHk8SmisUtlzkUpQV/5WEtQhWKuyO94qZ7dzW80CZ1fj94bx0IbwalCOOoIPEuB3sQCXuvxaLoHNJKw="#;

const FILE_MINUS: &'static str = r#"eJxNjEELgzAMhf/KI3edCXZz0Pa8w3bdXdhAQbQwGdZfb2w9SCBfeHwvNrRzh4+jF9elgTyurUBQ7VPo9edToJSO5RwU8r6V5pnLK3l72R96G6YhDv34RZj6cf454lpLuhpIhSaJh+JtEhd2dCdEBRvCIpkxU/3kbq26LBY="#;

const FILE_OUTPUT: &'static str = r#"eJxNjcEKwjAQRH9lyD01OybaQ9qzB716LyikUNuARfDvXUO1ZWBZeG9nY+7mhFtjLh5kEt8RhCuh5fVYhbP4KoCnw4qsbi9v2rj7nrcxT8N76Mc78tSP87Mxom3QUYMOdREXReXfRzVCErepWcgjQGj3KPnjD535LNY="#;

const FILE_PIE_CHART: &'static str = r#"eJxNTsEKwjAM/ZXH7otN1nYVtoE3D3rdwdtAoYOxFRyCf29bdUrIS0je470mDKvHtS3ObCHiZRAIVC4ppa/JnFiTgRzt71XG7VEVXbNL8q4Jy/ScxvmGsIzzem8L1pEawUEUXCZ+KJH8ddSkNJip5sGQ08igwLEdSer9n8WWsyJXge3BpJnhHdeBFXHds/Xpetm0L1GHN+c="#;

const FILE_PLUS_2: &'static str = r#"eJxNjbEKhDAQRH9lSK/nziU5i2h9xV1rLyhEEA0ogn9vFEVZWBbe2xkX6tmjKdRfg/Sia4LIjmHC6pOan+jUgF97oyRei1ale+3vpQtjv/bd0CKM3TBPhZKYhrhyMEN+iKcS5avxDTHePlIuYCFcbrABdJsr4A=="#;

const FILE_PLUS: &'static str = r#"eJxNjM0Kg0AMhF8l5K41wbRbWPfcQ3vtXWhBQXShUtSnN+6qLIFMfr4Z6+uxgU+FLypzAX5cawaGYqtMpz8lB1VuiNNDxu9bLs9oXtDZyxborB+6uWv7L/ih7cdfhVSqSZsBLsAEcEecDeDECjHCfCipGoSJwq6GBA5P2WGJ0B1jhpzsCm0EN0c="#;

const FILE_QUESTION: &'static str = r#"eJxtTjEOwkAM+0rUPeaS67UgHZ0ZYGU/leFGBsTA6zmrUtuhiqxYtmMlv8unyuvaPaxHEr8NxcUlcLSxr+2Etr2a7wX154h0X45/3ZRPLJzyVhvEAuIMV7SQ4oyLWnGYECwhG9DPiEy0QBJDpIHIH4ijZhcbK4Kt3h9K1C5J"#;

const FILE_SCAN: &'static str = r#"eJx1z8EKwjAQBNBfWXJPzC5prJD27EGv3osKLZQ2YBH8eydF2xxaAiEhj5lNiM3U0qNSV7HE9nY0xYWdKUjOvhESsmlpnN7s7yCGzYnSg7Rgqg6HlFCHOPafvhueFMdumF6VYgeErSQklzP8EeB/KXsSWXpYi5YsMhstU7jsKXZrVhpxQ6ExU/PPFvUFnStDKg=="#;

const FILE_SEARCH_2: &'static str = r#"eJxNjsEKgzAMhl8l9G5nMut6aHveYbvuLp1goWhxMuae3lRFJCEJ4fv/xKRm6uBtxRMrqYDudUNAUOYoePriacGdOqTzoqDXTarHJv4LZy7Z0Jk0xDmGvoU0hH76WIEVi7hooBL0Cu6IMz6MPrbgf4yhVAJGKyh3P2chT4xvEDsfD18lKcB6rZz6uL4A9N02NA=="#;

const FILE_SEARCH: &'static str = r#"eJxNTckKwjAQ/ZVH7omdydIIac8e9Oo9oNBCbQMWQb/eNK0LjxlmeFtIce5wacTJgLkjExmMqoAln2tlj2SUBR/cj5L5emjRht1ib0OahufQj1ekqR/neyMopyEvD67gi3CTZPGn0YLqqKFzIpVUh/Vb4V5/BZvntgd5Scou86XfA0AxhQ=="#;

const FILE_SIGNATURE: &'static str = r#"eJxtjLEOAjEIhl+F3F48KPQgqTc7nKuDWxOHDg4OxueXOpwOhvDn/yD56qM9O9yO05lnIEd9oTYGhjmGUrRT+eXEF/kyBHdD3chAUae1HoZvrbvVgKzTnwcZCoNjocYYotghHM2XT2xU0BVoAcphuaN7yuMiKDlFyHX3vgHedC2Q"#;

const FILE_SPREADSHEET: &'static str = r#"eJx1jr0KhEAMhF8lbK9ncnpusVpfoa29oLCC6IIi6NOb9QdSKIEkZOYbYlw9W2gyVWIcJkD/X01AEPkKeFtQHHiSRZKHgKo0TIoT3lRuPj4wN27s174bWnBjN8xTpjBmiJsGikAfxsvC5vsHDfi1JFKEkD4KnPiCeEUyOwoFPgI="#;

const FILE_STACK: &'static str = r#"eJyFjj0OwzAIRq+CskMNMTaV3MxZeojKHTx06FB56OmL+5MpUoUQ6Ht6gnK/PBpcT9OZE0jXptNSDiNcyoaEIfVEWgOQIWVgUvQes2GuSAZhBM7wyzoea3DwSX7GyvkWIT53jmSwbmTjxkxCiaLL4qXvLa6899oMLH803rQX7z85Gw=="#;

const FILE_SYMLINK: &'static str = r#"eJxNjr0KhDAQhF9lSB8vuxc1RbS+4mztBYUI/gQUwbc3BlEZWBb225mxvlkd2kJUGsyOdMNgqCiWXOdJ+iedpOBf9pxk2LZclPZzvpfWz8M+9FMHP/fTuhSCghvCMGAFE8ELCfCVOJICGXxl1MvsaURmk3TH0tnIZTd5AJnKMFo="#;

const FILE_TERMINAL: &'static str = r#"eJxNTrsKgDAM/JXQ3UdC1Q7V2UFXd0Ghgo+CIujXm6qUcpCEy91x2vaHgaEULco4A6rznoAgdYj4OjEgeJNBComIuiLOms98i0onLrDSdpuveVpHsNu0HnspULKJhwJKQb3CX8Liv8OiAHOX6RBk+YqcoIz0nwcasDCA"#;

const FILE_TEXT: &'static str = r#"eJxVjM0KgzAQhF9l2bs22foTIcm5h/bau9CCgmigUrRP393USiSQyUy+GRvauYOHw5su8hLoUrUEBEpOxq+3TgJW6jSlQUb3Oi+vv/IHvT3JoLdhGtahH58Qpn6cXw51wSW+DJACE8EN8TaCCzk0CKtm9oywiFbsKXrmU/b/KVrj3qVoj6yEDabzzbaudvILL5xByg=="#;

const FILE_TYPE_2: &'static str = r#"eJxtjsEKwjAQRH9lyD21OyYxh7RnD3r1XlBIobQBS8C/Nxa1PZSFZeG9nd2Qujni3qirARnFdARRL0XN26myFzGVBc9uRbpM2ag2HD7rbUjT8Br68YE09eP8bJSUNJTmwRp+Eb9KkX8Xi3HMWqLLsola/xEfuQMshNn9wRsecTR9"#;

const FILE_TYPE: &'static str = r#"eJxtTssKwjAQ/JUl99Tu2MYIac8e9Oq9oJBCaQOWgH692yc5lIXdZV6MC83o6VWpBxdZSbiZBgTKp9HyRU4AufCMFNB4XrLyvph/qnanKbB2Yei+Xdu/KQxtP34qxYWYZFlCTnYWrhIRbx2uxOeo2ZvISdTekImtxxEj0YhmZ/5ndDhm"#;

const FILE_UP: &'static str = r#"eJxNTssKhDAM/JWh97omWtdD9byH3eveBQUFHwVF0K83VZEykIR5EeuqpUVdqB+lkQF/sorBiD20XCsFhGxuiUNC8/8dme8V3lVpX76wtG7qt74bG7ipG5e5UJRKSEYOjpGfxtsi5ucHsfCaBTW3MpABGZ14IHn0A1zwMLs="#;

const FILE_VIDEO_2: &'static str = r#"eJxFjcEKgzAQRH9l2HtSXRKbg/HcQ3v1LlWagFWxoa1/7yqCLOwO7JuZcmpSQOvpYeBq0zAYmUwOVhyctnfOcNW25uz8KVE3Q1V52dxVOY390sehwzTGIX085UZQWQ7idjt4IAIfhe9ckqy2EFbbryp6taszdu6eCb/YpuDJEUIXXyF5KgiLVDBh/sslyObNtfHVCvtwNyM="#;

const FILE_VIDEO: &'static str = r#"eJxNjrkKhEAQRH+lmHzc6V7HNRiNN9hNDcwEBQWPAUXQr7c9EGn6oHhVtPPFVKNM1J/CwIK/UcFgmL20XDM9BNlcEz8FzdknsL/TvKrUvfbA1PmhXdqmr+CHpp/GRFEoJhkx2CA+wAsR+PqhIwMiWLy19Kyj/A7cAFeUKm4="#;

const FILE_VOLUME_2: &'static str = r#"eJxtjkELgzAMhf9K8N7MpFY7UM87bNfdxQ0URAVlsP36PecoPUihSV6+99pybtaOHlVyk4wd6SVvlJTS7Rh0L4kEVO1EY8HovWB33c2fpC5PW2BdztPwHvrxSfPUj+tSJZLBhMuTpuR/4B8BHP4giBHLrmWrjEw+b8FQF8Pi0YjZ5+ilYMZCW84zBgjGFygWRpuTshoMBy6wruNUwuoLppFDOQ=="#;

const FILE_VOLUME: &'static str = r#"eJxNjsEKg0AMRH9l8L6pya66B/XcQ3vtoTdpCwpbXVAW+vdNxbYSCBMy85I6dkuPe5OdHUR6dp1AkK8lRi4VFSd2VECO5X9lVCWbtfXhE2/rOIVXGMYH4jSMy9xkrDRo85AcfjVuFjVvF58VODdWyZJcL0FVMv66o35fYwbzjUpHHirJaxJ2NmRLCInR4Zd6A985NVM="#;

const FILE_WARNING: &'static str = r#"eJxtjLEKgCAYhF/lx13zPzQJzLmh1nahwbEhHHr6lKAc5IY77rjPn/FKdMxiY6MsYRkjCKSrZEmZm6I4EqMtJHan7PqebxH8UIHB/1jQlE1/YJeU5m97AO/7Iks="#;

const FILE_X_2: &'static str = r#"eJxtzLEKgDAMRdFfCe6t9plWh9rZQVf3gkMXwUH6/cYOKihvCRxy/R6PROtQzUxAMhxBoKYMCkun7WRYW8LoHlJyZa6Cr6/34O+IkUp2yX1pa8lAOrIf7AuqN55zTScl"#;

const FILE_X: &'static str = r#"eJxdjUELwjAMhf9KyH11Catu0PbsQa/eBwobjK3gkM1fb1IVuhFIHsn7Xlxs5w7uHq9UGQt8PrYMDKVWIepF2UImd8T5ouDbydjLF35jcAcNDC5Owzr04wPi1I/z0yNVAkmrgUuok/FnCS4ZF1aTsQgLeWxUrCKIk9KjPFJuxzSbe8ZoTAr8Mx+hdTo0"#;

const FILE: &'static str = r#"eJxNTM0KgCAYe5UP7/340Y8H9dyhrt2FAgVRIQnq6dPqIINtjG08qKhhE2ShXd0DToNCQGgzquROWgRJUVMsgwrXse7nb3wTyZt8KHnw9rLG7RC8cfEQhHZplIgBtsDe4l+RDwRlIQw="#;

const FILES: &'static str = r#"eJyNjqEOwzAMRH/FKvetTuokk7LigY2OTx0IHJgK9vW11aqooMSyzud3V7/vX6PPrXuKQincC9LEGKhnFAQWCJQRERmKbINMmiWgTD1hgC2r5h7kbdd2NYNjCIURXHRAdJTxzOfoV4I+1uB/N9aLlxnrXilSRjqXdfAtRp216X5aALdbNbI="#;

const FILM: &'static str = r#"eJx1jU0KgCAQha8yzAEKtbKFeoMOESmNuxDp5/apbSJyMwPv43tPBbdEIOdXihrZiHBpFAhnuSE9jnB4G6lQo9osGLXNkcBqnCSI/QE5egEBsump+yWMU8VhQ0Vi1aFEZF36FN4AqkOk"#;

const FILTER_X: &'static str = r#"eJxty6EKgDAQBuBX+bHv3N3tdIO5bLHaBYNhgkF8fjEIBvPHl4/l3LAOzcRKnhU6So1IFLqZUw2Qy0WyUCk5Jm/WlNw+p+R37iJQZ/gj7qH40g1QqR0m"#;

const FILTER: &'static str = r#"eJwti0EKwCAQA78SfEDb3UqhoH5HBHEFvfh7WfWQEMKMq5JHlIIqqfTmDTNeaOgB8WW/NX6QBZP2PhUzwd1HDxOwCBH0"#;

const FINGERPRINT: &'static str = r#"eJxtkDtuAzEMRK9CuOdEpERKC2zcuM4hFpvCZYrcHx7ZgN0I+9FvRvPI/e/4v8vv9+XHxfzmkojnN5fihxXhOx+TIe1y3b+m47q/fSG2IW5Bjw1JsZg/P4uiw1wNtaM29YXXOnwTNxQ/pxaJVtVREVq5ubIQrBxEe0IV5ewkHMUKE0eYwnNJOpCsi0lOTTIqJhwtfQ3XxCpmJby2DqYlB2UfMMZCT7C8o9jqyDB4ymTywqppoEYjfi7UG4PGkWzjq++bBCYFWp9rWFeWKx/mBwdGaeE="#;

const FISH_OFF: &'static str = r#"eJxtUUFqAzEM/IrI3aplWZYMaWDJpZc8YkkPOWShh5L3d7y7LKWUhVlbYkYz8vlr/n7Q5/vpJkFSuPqLsy45sb1wWRLAyDhbmxp7rbRhxickRhL3pGyNcnL8UmHTFGypEWuNJFwgzCIVrapKhaUIKBqLcq2jnCOm4MiFNjzE232TxtDcwLRKTi2x5j40a2KPURd3DKqipHAap8v5beS6nI90TpK5+dUpkMYGdAfTdVzvCePgWciIi2KcDeN1K46Dpb1+XdkS2BK8SBuAMLo4I21l9TJ1dqcV9iRtBPAFZ+Fo858+JngJrKTJP9YXARcazl1XEwg6FTq2pBxU5KOzzcjUaIW8dbl2yPa44UGwm4dx8fkXFy1A8+eue0PvWTKVfPj4AWW/cBc="#;

const FISH_SYMBOL: &'static str =
    r#"eJyzKUgsyVBIsVXyNVIwNCu21DU0VTAy0DVxNjRUMDJWMFKwAGElOxt9kEI7ABKqCyk="#;

const FISH: &'static str = r#"eJxtUUFqAzEM/IrI3VPLsiUb0kDJpZd+oLclPeTQQqGl7+94s4RClwUhS6OZkfb4uXxf5e3x8OJoouWCUZOhutSZuXQ0RkNzyeLILgWtSognAtioMfGVb9a+UhCZCLG0Tr4eTseHqXE63pW0U+gHba/looFhy0CErCHzU8lJFd13RkI0w+Mc0qWhzTCCJsPm85JUlEZUmqAYN2COUm/FmbS01c/rtHZk43p0wmAw27OZ54UCxc9MC4V6p1sNXo2UXezK0lKkbPYVg9S0+T7FFPU/6YfyunpbP22opz8Mhi5Fnwfawg1d1pBvXdTBnzD6nfUXOYFnOA=="#;

const FLAG_OFF: &'static str = r#"eJxtjLEKgDAQQ38ldBe9s4JD2z9wdRcVFESEirR/75WiLg5HLuQl5hjOBZNVXQsea1RowJDf64IgdxEpZ8qEOfPCGsy9/g2o8ZSrz9SHbes+I5JVrBBZRDRkG7IVNEHuBpSmJoo="#;

const FLAG_TRIANGLE_LEFT: &'static str =
    r#"eJwBIwDc/zxwYXRoIGQ9Ik0xNyAyMlYyTDcgN2wxMCA1Ij48L3BhdGg+p+oJQA=="#;

const FLAG_TRIANGLE_RIGHT: &'static str = r#"eJyzKUgsyVBIsVXyNVcwMgozyjE0UDDVBRFKdjb6IEk7AKZ7CRg="#;

const FLAG: &'static str = r#"eJwljDEKgDAMRa8SsgdJTKBD2xu4ugsKCiJCHaynN6XDI3nw+PFenh3WhJMCW2FiUMdAIDjafR6LP0zekJFQcLT7hzkObSTH87g2qJKQDeHlhIpQ/Yi4SlNPW5R/On0bhA=="#;

const FLAME: &'static str = r#"eJxVjjsKw0AMRK8yuJey+u3a4BhygBxiSYqUKVLl9NEuboIQvEHoMfu7f154Xpf7ygFxjpsmjC1zRCD6KCRsK3GQkpBlKk1JWdyIVZ2cSziUKs5vhfMGR50sXGGwZENw9IY23BASR5l6CWM3S6lungfr/0XO9F2O/TJKHz+k7yUw"#;

const FLASHLIGHT_OFF: &'static str = r#"eJxVjMEKg0AMRH8l5L50Z1i2Paz+Qa+9iy1YKKWHsujfm6goEkJmwpspv+4/yLPRO7Ig19RRKNEGwdQQTo/AB2IfXSyTtC0Xb2jL3nO1FFBTH4VeYZsqDvDz/r5kZKO4qUx2s8oIszALt4Y6tKGOkLpE/DrDNcIdnQG8yzDQ"#;

const FLASHLIGHT: &'static str = r#"eJxNjD0KgDAMha8SsheTUMSh9Qau7qJCBREHkerpbaNCeYT3w0fcPhwBJo8dN1CPBGJEz55MQ4pASZzHYGw5GOmZRspBZXsJLDe2rso/W7cu2wxRPHKDcLHHOpmoRW2JzExJykuqfz2+/YcfXwAqXA=="#;

const FLASK_CONICAL_OFF: &'static str = r#"eJx1jOEKwjAMhF8l9P9ic2sbB13fwIcYKFQY4g8R9/am7ocKGwkJd99x+T49Kp1HdxJPtoEVBM8xTkJC/jM8kHCIVcCa/vyu+bNdKDpwDHAlH1pnyb/NeIL70G/BYDDtwCNHQtUNoiSpDl8wX28XWmR0cPSCvfZXuazSoi1U3oJ+PKk="#;

const FLASK_CONICAL: &'static str = r#"eJxtjrEKwzAMRH9FZLeqE5YVgxvo3q7dDR08dOhQ+v21MxgCQYt4D+6ufOq30eu6PCCkP2dTr0pK0g+BFeA1p3tk71TYrIKwWyHOBI7WoOzpwMPg72Asqf/CUL/NVEKkPIqeumzlMhZsZe5Y2UibnxgnpAaZ5g8VAStI"#;

const FLASK_ROUND: &'static str = r#"eJx1iysOgDAQRK8ywbN0tz82Kb0BFt8EUYkgnB9qwBQxZt576Shnxb4MKxvIFcnykNPU3pw+5qBkNybVDp3JQ2r87Up4hDYDBo8OpqN68gIOlYU0vPwGYNMp/g=="#;

const FLIP_HORIZONTAL_2: &'static str = r#"eJx1yiEOgDAQRNGrTOoJ7ECziKU3wOJJEBgSBOH8UNNUdMVX79u9PyeOJVwjFBGx+9s0JOuzJCtOgWaEM6xCcHjZFpk8mT1gBR/RPC4Y"#;

const FLIP_HORIZONTAL: &'static str = r#"eJx1zDEOgCAMQNGrNO4gLWgwQWYXD0F06OhgOL80JugA6dL05Tdc6WY412H3YLcpERAYGVW2jO4wgBr1AgLEdohhlCSGGuIMlm0tEd7yO8grVs20iMnUFnQ98T2gHzz3SDhV"#;

const FLIP_VERTICAL_2: &'static str = r#"eJxtyzEKgDAMheGrhO7B5mnVIXZ28RCCQxbBwftj61RKeOP3P33O1+jawi0LjZwocZlJDFmHalnbAsJ/QH5yTCTY4YDEKqsncxFj7wR09AHzYC89"#;

const FLIP_VERTICAL: &'static str = r#"eJxtjLEOgCAQQ3/lwk7kihIGZGZxdSdxYHQwfL/eYtBcOjWvr+msV6NjNRuY4r5UEMhJLCzKpxO6NzlNouQ0ihy6f5csy8FkeepWU2diFCiAnZCokfCQZjUJ+KEbbso4VQ=="#;

const FLOWER_2: &'static str = r#"eJx1jtEOgjAMRX/lpu/FtXOAycYf8AO+kWmCiSSGGKJ/74YgvJg9dDk9va1/dM8el0CtKFxnYWEg6aXfwJbtDxnOKJFJ2hPqjWc1k14GB7NFrH7PMrDCTizU+ENe2Ph4G+P9ivgOVBPiK5AoYQyk2fl2G78/TswkugXsW6rxWGjaWrEUZVml6ngmXGEmqbrz3+HV5cVlhyUPS95++gOXlUjI"#;

const FLOWER: &'static str = r#"eJx9T8EKwjAM/ZXQ++ay2tpCW/C2i9fdSxUULMjwoH9vYjvnEKSkJHkv7yXuFu9nOHpxwB52rYrbVgFHB0ivVhXcf4MddwD7io02NwzJuCbNCoU82MxJt7ZpapUpOB9Q5UZyZ0QlgtvwksGly5SuJ0hPL7AXMHkhBaTHuyJSgYObL8oGUAOSJMWishzMrpbX0mB+cRo3PA1/pstHJvrDeAGy0U+X"#;

const FOCUS: &'static str = r#"eJxtyr0KhDAQBOBXGbYPdztB0iTW11x7veSECBYiEvTt/Wu2kGlmmC/mYc5jj7wlUQrmJF6Q12u18XXfbZy6peCf5OsRfk1HEO8jCjqWS57COA3whRaC9QlSoaEa6Y5W3BMNoH4aKx2rkTud/TR3"#;

const FOLD_HORIZONTAL: &'static str = r#"eJx1yTEKgDAMRuGr/HQPmtSKhdgbeAjBoYvgIJ5fzGCXlLe9T6/9rjjWsAlY6hyKDt8q2sCEPGKBPOLD0gOeeiKjJydnZIqwHE7ghEjWzy/WfDvl"#;

const FOLD_VERTICAL: &'static str = r#"eJx1y7EKgDAMBNBfCd2L5mpLhdjZxdVdcOgiOIjfr1nEIeW2e3dyblelfXILg4DbJ1ek07LIn/IKAwZizBZwr5ItSa9Ub52AFh0ciUcfNBRsj0o6+PgBbzg8QQ=="#;

const FOLDER_ARCHIVE: &'static str = r#"eJxVjsEKAjEMRH8l9J64mZXuFtqCNy9evS9VqLAHWaTo39siVCWHIXmTSfx9eWS6BHMCCMN5XqrQ0IrByDyJG/tMWcVaFreyzKgNDt1OzUnjcf+bQCg6prooKo4aQLYm+l07G326bWm90hYMDKVnMGqrvqq6Zvrg6PuPakm1sH4T/tBUGB29ARr9NNY="#;

const FOLDER_CHECK: &'static str = r#"eJxNjr0KhTAMhV8luCfXnJZeC73C3VxcHdxEhw4KDuLz2wr+kOnLlxNOWIct0vQrWksoo7oBBCrPAaOrHubEkb/izb1TVnGOxc8sFRLg/8TzJZnGvj8QdjVjCoqKpyzQF3X45BZ1uLosntSc1rK99QFHySWH"#;

const FOLDER_CLOCK: &'static str = r#"eJxNjkELwjAMhf9K6L0xyWa10Pa8i1fvIwoTepAhov/eqKMbgfCS95GXdB8fE1yyOx1AaOhHAQGyYi9eznslz8gYve29TB3GbkWAMQSMFY9iUppBizEIbWBTrqTdN7Akvc1ar6Dv7Dg4mLOzrq/fZNDfLql9xwG4f0q1zHbkA4tMLLA="#;

const FOLDER_CLOSED: &'static str = r#"eJxNjq0OhTAMhV+lwbd3PSO7LBkkOAwWgSMgJhAIwvOzIQap+np+csKxnJG2thprgonqFhDIPAfG1LzMiSP/xdvyU1ZxjsXvLA0SoH/j2Ul2qL8NhEvtmoKi4ikLmKsu/PKKLpQtIDURpig3lK4kzg=="#;

const FOLDER_COG: &'static str = r#"eJx9j8EKwjAQRH9lyT1jdtPGBtqee/HqvUShQgUpIvr3Ji3YHlLJJZN5mdmtw20K45XCu1FcKZoaZRWFz6za+rDYbf3onwNdGnVig5LEdEUvJGTiYS1azuWqKerBwm9eGM7Dj6jSVX6GScYRvhOz/U7ysihTfapdy+/COBJ7FBpew2YILiHELtl7hIOjFAQbmRzgEaeMOX+AJUHHQThfUaUEuwsUsDTHcERyi5q06ByzBb7aMGpl"#;

const FOLDER_DOT: &'static str = r#"eJxNjsEKAjEMRH8l9J64mS51C92CNy9ePXhbqlBhD7KI6N+bKriSw/AyMyHpNt0rnUd36Ald1TCBQN1nwDgOK7Nx5a1E/9spq4TAEmeWAQbYrfWWJL/v/y8QHuqLFUUlUjNwcjlt2hc5letS5gsto1NH5WkC05epb6Gvnd+PjCiN"#;

const FOLDER_DOWN: &'static str = r#"eJxljrEOgzAMRH/FYrcbX2hKpBSpW5euHbqhdshQJAbE9+MwBCR00/n5TpemYc70uzevluCyhgEEcpvAeHe7Z/OZbxJ9vSmrhMAS/ywdzOCxx8sn+Wd7bCAs6r8WFJVIBeDT9OlSVvSpblGQuiWcyahXUs/Wy6bKV7zfLS0="#;

const FOLDER_EDIT: &'static str = r#"eJxNTbsOwjAM/BWre47YeVBLoTMDrAxsEQwZOjAgvh+nqqIOtk73LJ/6bfS+TPcZUYg9MlcBUz9PTB3peXu3BE3ESkLiV6i60ImEGJy9+JyWcup9SxmtVslIj/TyjsFQZ1knzYKhGrSJbQQ5Q1fMZocMwe/CVfzBbOjHB6JXNqdIY/4Pc4suCg=="#;

const FOLDER_GIT_2: &'static str = r#"eJxtTsEKwjAM/ZXQe2P6ZrWBruddvHofVZiwgwwZ+ve2DObAEcjLy0teEp/9a6Bbay5KkO7Yg0BSwllYXP2PU+FDw7rpOD4p68ihllgFqcKZtYNs1wmzNyke6sUU82PK453ypzUOhvK7YGNoag3q0CKnuL7nAjnNFhxIrC+Ikv0c9h11cYT8OX4BAuY6bw=="#;

const FOLDER_GIT: &'static str = r#"eJxVjjELAjEMhf9K6N54SUrvCm3BzcXVwe2oQoUb5BDRf2+r0HpkCC9f3uP5+/zIcAnqaICHTHZmYBi+w5pPU9e66KxHdNJupAmt1egWjRMXwftur58gB/OfAPwkScWIhA4q4LOKfldbRJ9ua1qukF5BEStI77JFwRoU16cfjr5VJgMkWXpAI+MWfAAeITcG"#;

const FOLDER_HEART: &'static str = r#"eJx1T7tuwzAM/BUiO6/iUZQtwM2coV07dDPcIYOHDkW/v1RQOFkCCdLxcTze8r3+XOXr9fRuJiyXulIoJY8plR+xFTUYumZeeXV0v7eIoTX0HTMT8iiU/8KF5aE50a8hTuflZaiel0ObSe5ijmklqsvtuQ1SooUiuCkcxoQTXDEjfM9UzZtfDPiM63duG9ypZ07RA71qrpURKCle32xCCG13NHVEbLlY+udwajUHzrCuo/XzsPEHPFZDyw=="#;

const FOLDER_INPUT: &'static str = r#"eJxljjELg0AMhf9KcE+al2uPBq7OLl27Sx1uUHAQf7+niBzIW77kC+GluV8yDZ/ma+S/118ZAnE2MrYcxENfkLQEBIlRfJS3FbRL6Ck60+q40Ipqsb/snvXMtjKaNj32Dm2qmiBk6N1MTogU+MilN2B/LLw="#;

const FOLDER_KANBAN: &'static str = r#"eJxtjrEOwjAMRH/F6m4TXyKTSKESGwsrA1sFQwYGBtTvJ2ZIGSpPz89nXX0vn0bP03RNhNDUFhAo/AaMW96YOzc+Soljp6xixlJeLBkdcN7ifknxkv4/EFaNjx4UlUIucJ/mevAWcx1dMmlY045QuMGeMTc2zBdtjTNy"#;

const FOLDER_KEY: &'static str = r#"eJxNjk0LwjAMhv9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEPLxvCRvuk33GU7ZHJhAaHSTgABpsBUrR1/JMjJGq3sr84Bx6BJgDAHjgnvRVjZAXzAK/Ym1e4gpadd+llQva13OsGYjBuozGw5aXzpSE31wST+DV9Efzjr0oNnPdK52fPO0sTfHBzT1"#;

const FOLDER_LOCK: &'static str = r#"eJxNTrsOwjAM/BXLu43tvqWkcxdW9qqtSCUGVEUF/p5EgoK8nO/hs7uPMcDs8awCJkM5GhhIGiUju1STkLJyR4knCwV3xc8CynXN3Y1bS9AOQT7CYPJnTmg3rrB3p9zau22ZImxPj4rwWOcYPLYIeS8RwrJeQ/RYIbwS0+RYDvTueNnS1Wanb6+CUAmy29HwBvRlNL0="#;

const FOLDER_MINUS: &'static str = r#"eJxNjjELwkAMhf9KuD2xeVfPHlwP3FxcHdyKCicUcRCx/95ch7ZkeHwv74Wk9/ApdO/duSU0RcMAAjXzgHHpVmbjwgeJfvGUVUJgiSNLBwMc13pNkj+12wuEr/qbFUUlUl3g6nLa1S9yGp+vB03aO/WOfjDdmxpHRxNm27I1lf8/HCiI"#;

const FOLDER_OPEN_DOT: &'static str = r#"eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA83nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171pV/wn1FoX5T1u3OJqfd8n5O9TzXdoJ5MGSgPjS85lNTltJH5xeKQTNW"#;

const FOLDER_OPEN: &'static str = r#"eJxVjkEKwlAMRK8Suk9Mpj+fBmrB3T+B+6KLLiq4EM9vFLGVbIb3wjDjfX4sdD12t0pWyKQ4Q+IEAmmeUQiSa4POGzSJQhBf2cSd6qb4o5K2soNgnP2iKVNzcsbSS/R/lbVKrDIgI35Cv6LZsHvO9EQ3jYf3/OkFiKsoRw=="#;

const FOLDER_OUTPUT: &'static str = r#"eJxljbEKgDAMRH8luDcmKVUL1bmLq3vRoYOCg/j9tiKlILc88sKdO8MVYRubWaBHs5iVFCOjVQKiJGq0OiQESmFg7Dq0Ow6SUIqgT3ih6jnRzdUhV3ppJtfm0clV06wj098cBpiUhjdFPxuHKY8="#;

const FOLDER_PLUS: &'static str = r#"eJxNjrEKwkAMhl8lZE+85OrZg2vBzcXVwa2ocEIRB5H69l7q0UqG8CXfH5KewyvDtcNjA+qyhEFBwc2lpKd2ZSqcacfRLzMh4RCI40jcagHdr3EzwR+a/wugb/GXEmThCLbQM/ZpY1/0abw/bjBph6IIH+sBYZLK1p3JplV5lvxPitUxtPF2cb9fADO1"#;

const FOLDER_ROOT: &'static str = r#"eJxNTj0LwjAQ/StH9jt7lzY2kATcXFwd3EoUInSQIkX/vTmFVG54vHsfvPCYngWu0Zx6kK6wmwQEuu8JynncOFZecE/eth8jk3NIfkYapRI5bHF1gj32/w0gK9tcg8TkQQW5mBR2uiKFfF/yfIMlGjGQX9Gw4ruiVdNPTqFNZgEe1qEVfAASLC/x"#;

const FOLDER_SEARCH_2: &'static str = r#"eJxNTsEKwjAM/ZXQe+OSbt0KbcGbF68evI0qVNhBhoj+vWmFbgSSvOS9x/PP+ZXhFtS5B+4y2ZmBoavFmi/ThrXgrEd0pt1IE1qr0S0aJxbAx01emGBO/d4B+E0miRAJHZQHX1X0h5Ii+vRY03KH9A2KGAcFa1B1po9cSDah/knRt+BkkEegvvYByDbDHwP3Mqg="#;

const FOLDER_SEARCH: &'static str = r#"eJxNTssKwkAM/JWQ+8ZNtq0u7O65F6/eSxQqVJAion9vqvZBCEwyw8yke/fo4ZzxyAzi26oTEPA27MTJqVbvmJiis7+TPlAMqwSYmobiQAcxKAvh/0QrfiM29KywpN2UWZJeRx0uoO+MvEcYMwYEfX0vE/3okuaCNzEHtjL1tIvNB2MmLYo="#;

const FOLDER_SYMLINK: &'static str = r#"eJxlTjEKgEAM+0pxb732sHhwOru4uosODgoO4vutoseBZAlJSBL38VhgbopeIAzV5JCJKaCAoCyegh+NgjMwMKlSWKkWo5IM9xqduCxs7ORMuCs7KdpY3pNt/Ia3GljB44O/bb9YT+S82Z5pSl5pryyE"#;

const FOLDER_SYNC: &'static str = r#"eJxtTjEOgzAM/IrFbjcxISUSZWbp2h21Q4ZWqkTF+3tOEUUC2ZFyPvvuuvf4yfS4VNdE6oYwKik5lGdlvTV/TMC5lrSZeIlJ0lNa++pKOCPOkgZ123PS2Vd9dzLHvlt9vZJ3c8hhz72MC1BrGO8uiQUpRQ1R/ZtOKlGiISo7AdVaNoMHdoocOnPIfGQI1rdF3sRgB0+VosWLxYRBZPQSAFuhNO6sNq5f4sFPkw=="#;

const FOLDER_TREE: &'static str = r#"eJyVjrEOwjAMRH/F6m7TixNIpFCJjYWVga2CIQMDA8r3k4IEFlWHyovtu7NffozPQrd9d4IS+rIbQaD+XWCct7+Z21zYSfiuwBJZ/J0lMcQdTDSQNq8NE2qwxwmXbsib6f+QLYXDP0VlnWPEaDkSSwgNxPvWG/NHOELXoyhpddcmCySRm6rogg+6aHwBcFlOJQ=="#;

const FOLDER_UP: &'static str = r#"eJxljrEOwjAMRH/l1N0mvlShkUIlNhZWBrYKhgwgMaB+Pw5DioQsD+fnO115Le+K+2E4j2ColhaCCN+h8DJtWlxX2WuO/WZimpJofohOdMHjZm+fiKfxNwFcLd7cqKYZDfA6zGXXWsyldzHCwpr+yTPDPFR8ETv+AI+RLOY="#;

const FOLDER_X: &'static str = r#"eJxtjr0OwjAMhF/F6m4TX0JopFCJjYWVga2CIQNIDIjnr1P1b6hOsnT+fNblb/8r9Do3t0BwRWMPArlRYNzb1bP5widJftkpq8TIkt4sLczgssbrJflr2H4g/NU/LSgqiSrAo+nyobbo8tzlk+RI6myYdrCGifOWD4i/Le0="#;

const FOLDER: &'static str = r#"eJxNjq0OgDAMhF+lwbfQGxksARIcBovALSAmEAjC87MhGKn6ej+57vRXoL0v5ppQBbUeBKreA2NpM3PkwI048/2UVaxlcQdLiwgYczw5yUz1v4Fwq9liUFQcJQFrMXRlWjE8Mlcdgg=="#;

const FOLDERS: &'static str = r#"eJxVjrEORkAQhF9lo9/9/x3CbXLUGq1CJxRXKBTi+R2JQ7b6Zmcm49dxCzTXWedIq6AYQaD/dWD09jBHDpyL5UlTVilLFltYHCK84verde8Kwm5TDIqK0aljyBr/O1c0Pm0BuV316wtaJOcBC7woZA=="#;

const FOOTPRINTS: &'static str = r#"eJx1kLEOwjAMRH/lxG5jO6mTSoWlMysDW1WGjgyI78ehUDHAdOdc5HvycJvuC66H3SlD/UHGqY5hlTsY9wUq4RLqzJIiLQbl3JMjc0c+9pwK4k2QuDbpuJvboEoWg3tIZa9n9cnip0ChlCGX3XHYt/bjsDGYwGSFmCVEW1v0ppejWGf04aCVg1YOzewJDs0oDSSj30CwguANYrKBCP6AaGwqS/6RxG3SV/AEmOBGyw=="#;

const FORKLIFT: &'static str = r#"eJxVjrsKwzAMRX9FaHcaySiuwc7cJWv34BZcSKGEEpK/j0UgDzRcwT1wT/j1/wyviB0xED+kZ2Co9Uz5JsE23JRpQ/qMaXjDGJER0hKRfMm5pFVoq3dMC8ErfaL22TuQz/YrhtxELjddUzSeLtWGKqq8ShjOdhA4VFYsxzAl"#;

const FORM_INPUT: &'static str = r#"eJx1y8EJgDAQRNFWlilAkz3oJZsOLEJMcHOTsKB2r/EkiKcP85hQ82KkuaxqAs+gUzCA9pJMBexAxx1QfRJD3w4xbLMpJcHkmTxr53yztr5t/LcPXYYKJlU="#;

const FORWARD: &'static str = r#"eJw1ykEKgCAQRuGr/LiXnGHCFuoNOoRQoCAqJEG3T4J4m7f4XG/lKbme6C3XcXlFK8iCDYgx36rglh8F1+NIOLzaBbTdmqNAYGYE0ZKIPz5ReAGrIBjF"#;

const FRAME: &'static str = r#"eJxtjEEKwCAMBL8i+UDJHkoPqb/poVB61t8bYxQRT0M2w8j3/k/IfNNJIcOQFFDoCFCUozpRzPTRVL6GWwN6Lq7nPM7NxKbqv9Qq3bHAVC2xsStR"#;

const FRAMER: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Ik01IDE2VjloMTRWMkg1bDE0IDE0aC03bS03IDAgNyA3di03bS03IDBoNyI+PC9wYXRoPtteD3g="#;

const FROWN: &'static str = r#"eJxNi0EKgCAQRa8yzN5qJINAvUGHCAsKIsJa6O1zNKHF5zF/3tdu9+5YwRukDsGFRJkYM61uy9/qa342WAxONAANt6BGCSl6DuSwzJLVx36uEMngiBAKoiwXo+mIXbY+l2tS3P9npOqu2i9dXywv"#;

const FUEL: &'static str = r#"eJxVjbEKAjEQRH9lSZ+YnSSSQHK1hbbXHyhEELEQ8f7erCd3kS2GGd7M5tv1fqGZiwIUvZu6JiiKg6IZ33jIO6GGvLAtTD/GL5UmspA28jE9K52LOrEnYPQTCGTlNDQO+94TXhylKqX/KruKFWUSdPNWfLW91xiTia5bNyFpNh5HjhTWJx+p/zkw"#;

const FUNCTION_SQUARE: &'static str = r#"eJxVi7EKwzAQQ39F3O6r746SFOz8QdfuxQm9bCUYmvx9Yw+FDpJA0kvbUio+61w9k4yEI5MRtj2TnnH02Hvny/ry2l9TujRuSu9ndcyZ7jfIUBQRymOQ7qceEksMCgnGBmMN1thG/bPC6lcefuMX3FEnAw=="#;

const GALLERY_HORIZONTAL_END: &'static str = r#"eJxFi8sNgCAQBVvZvAYE/B6ADizCCHG5GbJB7V646GmSmYw9N2EKDquhuWgFb7umvP3CRGPRwx9y3IWuFIQdtAHdFQr0OPQgjulgqWYB5VpM+9rhX13KHg8="#;

const GALLERY_HORIZONTAL: &'static str = r#"eJxdjMsJgDAQRFtZpgFJAuJhNx1YhJjg5iZh8dO9iRfByzA8eI/3xZSSYPYUDjch8tBR5JpXo1sQQPUSeFDbEXSWZCpwDWgum1r7r9aFyF/vF3wAShMeGA=="#;

const GALLERY_THUMBNAILS: &'static str = r#"eJx1i0EKgCAURK/ymQvENxcF6g06RKT03YV8qG6ftorAzTAzj+dK2pTOHFU8eAJJyrto7RZ0eYyg+81Sh0FwQxOCO1YVih6LJcPCDbTrA+Ye4K7CP+cBykgsZw=="#;

const GALLERY_VERTICAL_END: &'static str = r#"eJxFi8sNgCAUBFvZbAMCfg9ABxZhhPi4GUKidi9c9LLZzGTsuRVBcFxnGNGK3nYNefuJEZPo4Rc57gVXCkUc9ULk29EQdXviqUwREtMhpX7Tulb4F1VsHfM="#;

const GALLERY_VERTICAL: &'static str = r#"eJxVjEEKgDAMBL8S9gNiC+Ih6Q98hNhiepMSUH9verOXPcwyw9duSlmwRQo6r0g8dZS4lcPortlU4JzaIwgg3wjSUk81P5y8gqVrXUj87w3BD1dWHfw="#;

const GAMEPAD_2: &'static str = r#"eJxtkMFKBDEMhl8l9N7YpEnbgZmBxYsHfYhhFFZYxIMH16c36ZRVRApN2n750z/z5fXtBa68BKIAn7SEYsGPKcCV+vU63zm1zp11ph1MO5ApHAL5D9nLeTzykFVMoxHpf3wafBrQaEXN6278+/ZxhuclPFHFzKAPBUvbBASSr5hxqg0y6rRHTKlgUraEkBJ5rEjK94wlCUwoVICBBEV7UrYM+VCymHeySKgRFTjShaxAYt9PbLxjZDKNm9WeBbPI9vPQQdRWHqkCtR3VdOzaO/1uFPOeTFXFGtm/YrEasbRprMja3Ei1Td2AuTiMWFQ63ZzDGMiXz8qntH4DhBNo7Q=="#;

const GAMEPAD: &'static str = r#"eJxtj0EKwyAQRa8ic4DWb5riQr1MK1UoWQQh5vYZdSAQsvqL9x7MuH9eoqrGEzSpCk9vUjsPDK/pG9yzWcENl6GlnlhRtaivGxWzwGk0mB8a0k1Xv3F7ckiKcRns6a/xU1T1xFeuY1LMv1Tk8P7Glr8lMdMta0E4AN0rPM8="#;

const GANTT_CHART_SQUARE: &'static str = r#"eJxty0EKgCAQRuGr/MwFYorMQL1Bh4iUxl2IUN2+hiBatP0ez5W0VJTDU0vYc6ziiS3hho4gKa9SHzlVgmt0CG6bqyB6mkZYGdRVPm7BrZifwAw20r/lAoCmJP8="#;

const GANTT_CHART: &'static str =
    r#"eJxtyTEOABAMBdCrNC7AN1BJ9QYOITF0NLh/xNLJ+p7seYxWD4OpGFJQiY9UPAohW/sEQGCrPhdFVRWp"#;

const GAUGE_CIRCLE: &'static str = r#"eJxFjEsKgDAMRK8ScoBoqlWEtDfwECUKCi6kuNDbm1JQmM9ihidnujZYAs7saQBHY+IWTBZmT2MxRmnKMYruWY8V9A7IDkGf2jmgK6c6R/mpHfWGMzRP4D/OC/7hHl0="#;

const GAUGE: &'static str = r#"eJw9ijEKACEQA78S7M9z3YXjYPUHPkKwsBEs/D9qoSSkyIz2PCpKMI08SCCPmKjvfqMeltiygP5MDqtrdj7LHu7aE8xdEk4="#;

const GAVEL: &'static str = r#"eJx1jTEOgCAMRa/y416koFAT9ARegujgYuLg/WNxwQHT3zbtS/5PV74P7HN38gD2FM0I7Y2MeBU5w7FsD/utrH+Hd5SbYcmvzGDbLakvnkuqzgGqQKHBBPJDJkRl0iBOc5iEKnsA1mwwrA=="#;

const GEM: &'static str = r#"eJxtyyEOgDAQRNGrTOo3dLpNQ5OlJ4AL4EgQKxAI7h+KqUL/9+0+Hse5hK1AnenKKMII6ppQ99Bs+kSz4UgoZtQuqchCFZXyA/vvKY7wAq57G4I="#;

const GHOST: &'static str = r#"eJx9TL0KgCAQfpXDXbsfJAXzCWptFxocHBqioafPW6QpjoPvP53lqnAsZotAWB2SyWlSMadhkf/xGLgECIB6tqObuIkVYOf1156g2AablQnITvjt2fCM9RczACSN"#;

const GIFT: &'static str = r#"eJxtjsEKgzAMhl8l5D7XZJZeWi+77LKHkClrQVS2wnRPv6Syi0jpX0q+L/x+noZ1SGMP85TG/A7IBohBkhnqLYix8ec/2fhX/8gQ+/SMOaBF+KQuRzURFnkQ1oBOFQUbX9avJBMZLRyQFGFlYKHyFXZbPbc5QhfwLh3czVW25cqCXiOHwJzslQhYO7oS39JNtJ0c651sNvlyLP8ABA5DOw=="#;

const GIT_BRANCH_PLUS: &'static str = r#"eJyVjsEKgCAQRH9l6C45iouC+Qd9hNDBY4fw0NdnCXXxEsvC7rwZmLjno2BbplVgK82U4nxLKb6AHiFbWGiwrVbN+Xx95BxkBIZ/M3SQHBC6R7VrXIauyoAYgr6oD10qxzbq"#;

const GIT_BRANCH: &'static str = r#"eJxtykEKgCAQBdCrDLOPsigS1Bt0CDEhQSKkxXj7RhPaxCz+8P9TMZweSGhcEGiskTnEzMnthEb1xRjlQnLRgyNeVwSXK07NvOunclP0qy57H7Br3MQK0kqQMPCJjr/CymwewFspow=="#;

const GIT_COMMIT: &'static str = r#"eJw9yzEOwCAIQNGrEC7QQNOhCXoZ4mBiOjjJ7StqmMiHh2jt2gqoJSRG6AlvBB2rslz7nKXVr4DRRoOWMj455+vY0aG+Y8J4Cev9BP4BhjAhDA=="#;

const GIT_COMPARE: &'static str = r#"eJxlyrEKgCAUheFXudw96l7BDLS5pbVdLDBoCImot08RSogz/MP5tFuD2xZwt0FSCO7KDQYF9rrOd68LJrOSf7Tbw8NscCQB0gvLwNDEUSyfbaKJlJCA1KA+WXHFU/fKB8foKnM="#;

const GIT_FORK: &'static str = r#"eJx1i7EKgDAMBX8lZBdNBKnQdnZxdS9VUHCQIkX/3paKZJEsedyd9lvw+wL+MkiMEAy2CP5OS6HVdcFWC62TVvcjpfrlRRbW4c4VZoMjKegjOQaGJh1V6RuU3BVPfU5zIkMG4th+5AE8IzVq"#;

const GIT_MERGE: &'static str = r#"eJxlyjsKgEAMBNCrhFxAV2ExkOQGtvZLFBQsZLHQ27ufRlimmIF5bEe0c4MoOCLYK+im1E9p5a7eyg3zVfk/usK9wyo4exjcQoGAoC9JK8MM9ANd8h93"#;

const GIT_PULL_REQUEST_CLOSED: &'static str = r#"eJxtjTEKgDAQBL9yXB/lIgaFJD+wtZcoKKiIWMTf6yWKKayW3RlY7abdzQPsBgsEdxqk6k4f0uo8YqsfjQWFr+25JNLWHSP0BhuqgCgrWyoZ8/zBRRIUQoH6R7VQIkHztA7gKbye0qAkBC9jvdeaTXbsBUICNxo="#;

const GIT_PULL_REQUEST_DRAFT: &'static str = r#"eJxVjE0KgCAQRq8yzD5iiqRAvUHb9mFCgkRIhN4+/6Bafbx5j+HKOGU1qCCQRgQnsEdQPpPkbdGS16zqGLNSsW90rtcOm8CZRmDLkFQ6/QXR3dCrrDk0+C4/9JQnROoobsQplamRDx2GLxk="#;

const GIT_PULL_REQUEST: &'static str = r#"eJxVzFEKgCAMxvGrjF2gNsEK1Bt0iLCgICIkQm/f1B6KPfwf9uMzfgt+XyBYVAg+WaReGkudaerbmT+Tt65af9E5XSvMFkdSoFc1MTC0ciTlu8s0E2f27VggUllJkkHCFpkQIr+j2bgHyVUqvQ=="#;

const GITHUB: &'static str = r#"eJw9UMuOAkEI/BXindqBhrYnmTXZ7NmPMOPBowfj91tMJ6bDq4CCZnveXg+5/56uluL+1rgFhpQs9dS0IfdGv6tTEollqMFT4Wd1xJhF8ILtsI6WbCY6ScoXQzLRQ2kTrZflmP8uLjllV1SdZdlJIiT5S8TSZOpjLwlZ9yMnhaf00uxaEauiD7IsjEfS6XTsDA50Z+iNYeEj33G6bD91g8v2vcQqNnYNpIkrV1b+8lv2AUS9PiU="#;

const GITLAB: &'static str = r#"eJyNkDEOwjAMRa9idc8ntiMnkUpnhpyALRIDQ5EYuL9wBGWoGKq8LD92nuz52V93up2nhwixQmpQqAaOHUkciuMEcPJboIMtE/GModX5ZYqY963euQaBGBksX/wPaQYm99ne80djAbEc07TvFB05OZ83gmQUbSwkvFZYDYZSvIadrUZdXfQ6LfNpLGV5AxTnODA="#;

const GLASS_WATER: &'static str = r#"eJw9jLEKgDAQQ3/l6N7aO722Qi24OegPuBUcOjg4+P94hVoCIQ+SxCe/Ba5FHciGgGgLJmQJYEWoSaPx884wFpxugYCA3hCuvQNteaoUh3qXYj91gJQZuDWd+E+2Up98HKUeEA=="#;

const GLASSES: &'static str = r#"eJxtzU0KAjEMBeCrPLJvNWnjKLRzAw8xREHBhQwu9PamFKWClEJ+vkeKXVe7nWGvSqyEtVIm2LPSjuay6du5fJTPeU+/emD35XHBqdKRM1gXgWDbXpDQ6m8Haammh4xEBScoJotT4JjAMbtN4a/mzvngPvSAf/VTY+ANwHk3+A=="#;

const GLOBE_2: &'static str = r#"eJxtTk0LwjAM/Suh99YmbalC1/MuXncvVZjgQYYM/fcmHWwTRg4vj/eRpFd5j3Dr1JXQBA8YeoyFgMDKaN5mz4LK6STWnNZABGecH0Jx4BY3o5vtmkaQtK28GTQXYcK3diDddN10uaVpdAbjwTXkNjaFAc/79/S+EBvX+G/oydiwNdbHVJ93mDqFVkH9MBLjtyG7Fj3/AFqYQXc="#;

const GLOBE: &'static str = r#"eJxtjUEKgDAMBL+y9K42UW+1P/ARUgWFIlI8tL7ext5UAhmYXVjjtuD8AhcHRazgUmHI0MqapuTW+G1fkKikiQtjJguzZ2lLy5pjOlfMgxqJwRP1dYvn6XyEDqTxktW/rL4SIi/ZkhV7A9heK/k="#;

const GOAL: &'static str = r#"eJxtyjEKgDAMheGrPLob09RAA7XgAVzdCw4ODg7eH6NDJ3m86f/K1e4D+xzWKIhpkzNjGvyhlvFttXQhTKqITJIWg4ERfW7J/nh22hT6OUYmgxBLlw9pCB3e"#;

const GRAB: &'static str = r#"eJyFj70KgDAMhF/lcG9NYpQWqm/g6l5wcHSQPr+tgz9QkFuS474cCXs8NqxjM7MDs+0XHwUCKjJiJNF7hyS22kyhLdwUHlrBtLgfVmokwVv/W9tX0AGs32AFpFpn+fVOcpaCUhcd3EWyydNm9G1k6xO/z57yqkpv"#;

const GRADUATION_CAP: &'static str = r#"eJw9yrENgDAMRNFVTukjbEeOhGSyQYZAUFBQUCAXTE/cpLj7zbNnfy+cW+oiYPLaIzdT1lFojvtSsyVgs8krWFyPgoJ1jAXkWSf8Ad17FjY="#;

const GRAPE: &'static str = r#"eJx1z8EKwjAMBuBXCbkbl3apLbR7A6/epQoKO8jwoG9vulWmsF56aL78/ImP8/MGl4RHY0BOZtwJ+QDlwSHuy3SI+T7l8Qr5nZDnCUwJLUJ+6YcjV+RiVqsjT8w4bx2orzvbkg1ZWSgzOWljFdpA/hpYCrxhqyiZHXWhYi0lbazCkbi6Zsk0OrBe1NfEjnyz7xr7PS/8qA+Ht2h/"#;

const GRID_2_X_2: &'static str = r#"eJxNy8sJwCAQBNBWhm0gqJcc1A5SRIiS9RZkyaf77OLFyzDMY2Kvh6C/iTxBMxCeVoQTuZXAtZ0so3+GOS52yPHahVESbQHOs7qKbZM4j3BP8gNmJB49"#;

const GRID_3_X_3: &'static str = r#"eJxty0EKgCAQQNGrDHOBMBESHG/QISKlcRcylN0+1JXg9j++y/EU+Ag1Qi6EKwLHdLEQqg2hNHhTEG7Bu6UO3t2HMATCXYPlDjUNoMxcLOhnCsqM8gPBUSyp"#;

const GRIP_HORIZONTAL: &'static str = r#"eJx9jTEKwCAQBL9y7AfCBa4QTj9zpAikskp+H9ETBNFqixl21O5sz0X2RQRQjmCQvWVOJD0aTeqW4+ZWK6ytgqXLs+SN4ZJlW2TZJD22OPsBgONBeg=="#;

const GRIP_VERTICAL: &'static str = r#"eJx9zTsKwCAQBNCrLHOBoGARWL3MkiKQyiq5fUSFVfxUW8ybHZY7ynORvB4nKHoYkHzpWAQ+Shq4Ko0damVEua2/dsq2yrgZK0G3vJ7stLIfif5Beg=="#;

const GRIP: &'static str = r#"eJydjkEKgDAMBL8S8gGx4CGQ9jPBg+CpJ/29xUQMaKD0GHaysyxblX0FOTLOCUHOjAtCbRcWnjQt7CnqoZ7U0C/0prdW9T9dHqBQaW0qtp9gfvLLWmXgpAGnL7sAryBicQ=="#;

const GROUP: &'static str = r#"eJxtzrsKwzAMQNFfEdqVWgrBBOLMXbp2L06ospVg0vTva7kPPGQRBp8rNDxuSWEKeGnBX7voiBtuehIQEhUch5OJcfg79tCqxMzAgUDT5yHbkRQG9ptEBxmTQVurdGQ9CJ+7SGUtCdkJJFtt1zkmWPeAjPAK6BH2Mp/LlLS8dF7umgJ2Fhn/Rta4+tt6ljr97P1lbwYgSW0="#;

const HAMMER: &'static str = r#"eJxlj8EKwjAMhl8leG9s0jZrYQ68eZgPMfSwwwYeZM/vv4giSNM/TWh+vvSP6TnT/XRYpZBoqFwI9xa4JkRQlm7PieLvmdBXctlroRjSiKIdhv64Ww79x/gKA8sEe1WSiPf/n1UjNyER7oKwFhcwmEdLqLNnRX8Dm41iHIUy21S4GLk4WkjcMgYsX9rCTbnq2VgqubxpQVo5b4KhBWvMyrlb4G0k4PjyvQD1SD5Z"#;

const HAND_METAL: &'static str = r#"eJxtj70OwjAMhF/l1D0mdpvGkUIXZtbskRg6gMSA8vy4gPgpleWTLH+ns/O13mac9t2RFSwUCvsqEPilnDhpPzOkMQ3dlHeLccof+wDmkl4sGzvAN9kibe0tKKzY9M9eIjg4pjg+5PsQ0h4mcu5phPUhUoAw2R2JjBP7xnSWqtCnCeq0xFVqeKfeAdW9PSQ="#;

const HAND: &'static str = r#"eJyFjbEKwzAMRH9FZJdqybKrgJsvaNfsph0ydOhQ8v2VS0gyBMIh0B33uPKp3wlet+7BBsxjrgICoQkFZQ57D+67oVwaM5SNVOAw6gkpR2RwktLpqh2xBrbU2KUQ5lw9+2OM/k0oTxTyBJUSkmVM1PceRX1jpNxuXWLwamx9uV+B0zr5A510Qiw="#;

const HARD_DRIVE_DOWNLOAD: &'static str = r#"eJxljUsKgDAQQ68SZq92ipQuWm/QQ4iK40IQKX5u7w8pKNmEPJK4qY6C1lNgDb1YqlxxRZV7wcgGJitRZqcSnrsmYt48acI6tFFOpwjSDb1ET5bwsN0T37WrkFaDAVvJFf8PA6svOwAbbC4G"#;

const HARD_DRIVE_UPLOAD: &'static str = r#"eJxtjU0KgzAUhK8yzL5tXgghi8Qb9BClSp+Lgkjw5/YaQRSRWc18zEzsPllRJ/7Fwz9cERyr+Cqgijt+i4UdwgH65psxtnXWRGsIbdqf5sRATGtCzIniiH5za60UTnseEvRp5O7KXNkCC98uBg=="#;

const HARD_DRIVE: &'static str = r#"eJxVjsEKwjAMhl8l9N7YxDV10BW8efEhBgoThniQ4Xx6m9ZZpIck/f8/+eJ8u19hpcEQG1i51leeWWue2aS4U1eKj/E5wWUwZ4+dB49EwEC8yMi5ceXlbqK/D8uLldnuc8YKHvpj00gwCHSngNy1iCUMPVBe/9bbejXFximVTyqeoKMvuDTS4laZXNE1QO7n2xZt/g/e8TyZ"#;

const HARD_HAT: &'static str = r#"eJxtjaEOgDAMRH/lgm9YN9YgBhqDxS9BTCLIBF9PIWSMhNT0ru+uYYt7wjo0swX3kcEw9+iWvgZxJvsapMbkag3O9mjG0F6dYyjNrGGz+ILy1ZVsrTXqf5Id2GdyUSAPKSTJ/P3ooIcahGRXwBPOnDYz"#;

const HASH: &'static str = r#"eJxtzEEOgCAMBMCvNP2AbkWjSeE3HkyMZ/m9SJVo4Ljd2eq+HStF8bwwnfDsmCIspaP0HLS7TdAsPwRjMflByrXFUwoMz7YdWnR6u98CrtgL2IwreA=="#;

const HAZE: &'static str = r#"eJx1zKsOgDAMheFXaeY3tu6GGNMYLH4JYmYJgvcPrQDVpan6Tv5yt6fDtakRDUKidybwq1oWtlq+xUHmO0pgJzJcplo2iYt6UqVs3lEWdKI4CvoWIIDl0ytYaYUQTzTxpxd0gj8V"#;

const HDMI_PORT: &'static str = r#"eJxNjTEKgDAUQ6/ycf9qoiKF2tnFC7gVHP7QwUE8v+1SS4bweJD4Oz4m19YdpLgIgYwlCsU+tSx4559zw5AoNDBRc7VScXbBD2U8+Hqx9ouA5qr6AP+hHYs="#;

const HEADING_1: &'static str = r#"eJx1zDEOABEQQNGrTPSbzcxuUAw30OolCo1EIc6PioL2v+RzCTVBNML9gJS0sPzOZHkH7eUBkC6SUY0ZfA+1NexHth3A"#;

const HEADING_2: &'static str = r#"eJx1y6EOgDAMBNBfaeYP1lKWibI/wOIJiEoE/x+KIQgQd+Jezo71dNqnNCuxeE3N+ntq9oa6lA9g+RPhEIduGUqKIVIog7sRAolW8HO7AFDBIlo="#;

const HEADING_3: &'static str = r#"eJx1jDESgCAMBL9yY59IIqgF8gNbewYLSgv/P4KNFNhkctns+SveGec27BaieR2CH+sp+Basx9wBor9kYQcx7JLwQoKpRPNOYRcVWpKBUNm6NmpB0voNC55gqRUNKX3iA4+XMDo="#;

const HEADING_4: &'static str = r#"eJx1zaENACEMQNFVGha4a9MAorABFk+CqEQQ5icoEGD/E19a6Qo1mMSApN5E+VaKcoLP9gJIT3GA/2DlixEu26MJc4wk5Q=="#;

const HEADING_5: &'static str = r#"eJx1ja0OgDAQg1/lgqdwd4MbyUBjUCR4AmISQXh+fgRBMNMm/Zo2bPMeaW2zwRFL9FkXijvqwhf4qf4BLElixHrkGl0CGmyBg8BDiaGPVlSSwHIGPy6oRm7gr63r6K5oz/YunpvIL4w="#;

const HEADING_6: &'static str = r#"eJxtjMEKgCAQRH9l2Xula4mB+gddu8cWGHQI6VB/n9IhiS4zMG94dp+OALPDoQVJwaC3TZ68LYEZ9Q+Q9CG8Rt4W4NOh7BH4Sq0RokPKpwcXAhIgBVcEVClQdZfytd0lMCou"#;

const HEADING: &'static str =
    r#"eJxtySEKACAMAMCvjH1AN0QMcz+w2gXDosH/IzaF1TtZYxvMii0DsRGjSrim8gzHnpyg8s8BT+sVqQ=="#;

const HEADPHONES: &'static str = r#"eJxdjrEKgDAMRH8ldA82qVILtbOLHxFwyOgg+X4jOLRyw90Nj7t6ya1wbuFIQLMmYWCILnJn6zp62pe+IxtmKVA+gFaIlkdCkf7IsIGsKbQ6vTfaA11OHfs="#;

const HEART_CRACK: &'static str = r#"eJxdTksKAlEMu0qYfeu0fR8HngMewAu4k+fChYIL7495A7oYSpoS0jbtffs8cD9NF1tgqZumRdgKQkLdSFnzmcDAvJUVjtHpqwWzBDRLouRU8gbXmsSHKL5bdhw1d7IGaEXSmceG4VlRr9PaDiPT2n7JXuawEBPjAxfmGvz3fQE/mSkh"#;

const HEART_HANDSHAKE: &'static str = r#"eJxtT0uqAkEMvErhvvOS9B/mCR7AC7gbxoULBRfi+a0eBPFDOpWmKt/pOt9OOP5v9tZhaTFJPRAKYojixpAl7+gYrqtZ4TcuzKsFGiIkh0TKyeTVXWoKPsjgH8WOJnlhlAimIomy2Ug4V9TDZjv9jZ2202szR0YXTajSy+xiFSvo06Jou+sizfmGFKWNnqL17ASu1OenRlhPYFHtUOq9kC7le/DFGowHBP+lZVh70x7IP0hr"#;

const HEART_OFF: &'static str = r#"eJxdTUEKwkAM/EroPbGZ3XQrtAUf4CPKKigU8eCh/t6EBREJkyHJZGba7o8r7Tp36OjdaIdTjI2X6RCiZXqurxtd5u6sgxi1BoJuXLhUVjFHNk6cBN5NbHVQoPdSgmjmLMnCNOx+TEcpAyXRqqImQIiTlNF3yKRypBZgfiiZQdlHnP4CQKNY7eMZLk4UFgwZiqu0fHM/aF42Mw=="#;

const HEART_PULSE: &'static str = r#"eJxdTjEOAjEM+0p0e0ObNA2VyklsLHyADZXhBgYG/i/cIt1wipxEluO4fZ7fjV6X5Z4qpdwT5xrQCmlQloRhbFeABuKsVLBqh84LxaDEFjIoAWMTwp6DDDLI4VjozNYxWQlSyhxhNgRvJ38sazuNTGvbkyGHUJJbhWT4w+L/zaeBsm3G4vvlDzz7LPI="#;

const HEART: &'static str = r#"eJxdjjsOAjEMRK8y2t4m8SdhpWUlDsAF6FAoKCgouL+YbIms8VijZ8vb5/F94XlZbnVFjVE1VmFrcHG1SkvNK4WpclRtHH2Q6w1FHJoSjIxJHjLtITZDsb9lw1lz0NVBFKGFxybw7uj3Zd9O86f9B4uwH20="#;

const HELP_CIRCLE: &'static str = r#"eJxNi0EKgCAURK8yuM/+V6IE8wYdQn6BQYuQFnX7FCFiGAZm5nnZsxwb5J4VGwV5WuYSpILv2x78Ga+EdVaL0+TgooUFFTEGPVmwEExXyurK1f+PYgMekyb+thenCB9G"#;

const HELPING_HAND: &'static str = r#"eJxNjrEKw0AMQ39FZD819jmXM6SBbhn6EwcdMjTQof9PnYaGItBgoWdNr/Ze8bh2W4YMGCiadrtlZPQhgfRUgy+Sm0KPU7itSTlsllgqjDImY/UmrBVfO9pKjxZzeUau8cDtn230AhkXZ/3hQ0loAff7CPFuni77ynk6t2oUUVDO6AMQlyqM"#;

const HEXAGON: &'static str = r#"eJxNjjEKgDAMRa/y6R41bbEdasEDuLoXHBwcHMTB05s4SMgQeOT9/HK2a8c2ucUzeFxz8/AYdIiJuxQOShQNlS0Icf4RAvJtREiSinJkoHqSZLTv4+Nq6bVEfQEGuB0S"#;

const HIGHLIGHTER: &'static str = r#"eJxFjDEOgCAQBL+yoT/kDiSSIC/wEyQWFJhYGN8v12A2U81m8l2fhnM3VwIzRcTXt9Q9eVPyorLkeREBCwUbMagCgRtjErvBdVqtKL+AU3VwQJi1D07EGss="#;

const HISTORY: &'static str = r#"eJxtjD0KgDAUg68Surf2x2ctPHsCPUTBoYODg/T8viKIg4SEQPjCZ7kq9kVtAc6XhAQLJ05auon0hO3Sk4kjvMQaMKvMQ4czfy5Co0o/i/OIjQ6h3/EG4sUegA=="#;

const HOME: &'static str = r#"eJxNjFEKgCAQRK+y+C+1GyKC+t01hIIEUyEJ6vRt4UcMDDPwZmwNbYPFiX0CA0Zqdn0iBgKCkYWS06z+XdItvB3epbe1pCvFvEItMbfDCQNEfIIEqLoTfXwn/QOGnh24"#;

const HOP_OFF: &'static str = r#"eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUXndv3yRlmNoZYsvGnpxd5/Xn9/aC3y3LVKpmy5F07VbKAUydTUj1YJVoUK5lNSioSOydRbWJWlm19cZFtfUhBhVxurwSxfoqpQu0Ql8A9Z1ZukifVw4iHw1EKnLAK6wS1AFmAcAnSKI6Y5rIDCYcnfZKIcKOjUWRs59VmZAZDTgHyHzjLJmyS2iJ5tF0l1EhNUjd0VW+I9u1Qxji8lzvncmYQCSv7gK4+XLyD5Lq79dpAxAgmS8oRF6F1KBHGJK3hgzHMW2pReKtWsC/anv6+Pr/f6WaXxWyhP0VGuh9v4wjUoe0fpoh3eQ=="#;

const HOP: &'static str = r#"eJxtkb1uBCEMhF/Fut4EA+ZH2lyzdV4gXUSKlCny/srY7N01FJi11t/MAMfv198Pfb/fPqQFJQ16yqBGKaIblIREJqeAH6yE0rHkdj/eDLwfTxwDZBJnIwiMJSAChQniEmAxfkNL9emgEwQJZWupGkeRC1bduqb4oFBBZq/FMrpjNhr73tSZOMXNnijq4sFmkCawtTasPiKj8dDrEjkZ7MasS20jUULrmayms4cyMvDYIFY8kXTSydjZU3G99OWqhS6zdVZr3Ww9gqWpeBLCaJ9cPE4OIzUeQWPnkKQxxpL0z2049dvIlCcH6cWKXl9dKg74epF/cDF/lg=="#;

const HOTEL: &'static str = r#"eJx9kDELwkAMhf9K6J54yeXOE87ODnbt4FZ0uKXgIP39pqByYK9kSMh774MkP6dXgce5GziBXOIkIODWQpsWrhbWpbDUC5RR6wTKrevzYWX2+UueT8ARyGtCEr0zaQzI5NiDp6SfMZB3Cu7Kwdz/kCHBsZhxQzF4U5I9iblNbGppJ2YfkQUjhdkuGUV+njcAW1oJ"#;

const HOURGLASS: &'static str = r#"eJxtzrsKgDAMBdBfubi3mku1Hargrqu74ODg4CB+v6mDipRAIK9D4j4fK5a2GGuQq7iii2XqdfE7yQ/E681pnBXPmSCqFMbWoTFinbhBCOGmG1rgzv2zBw/xNjBMZAZX+/zJSDK+8k2aP6x/Nelyet0L4p82Fg=="#;

const ICE_CREAM_2: &'static str = r#"eJyVjTEOhDAMBL+yoo/PxrGTk3LU1/ABOgQFDRIF4v2EDkq0Wk0z2i3buC+Yf00vLSRNBkYOLfm3wv9xYiipQOHI8GENEXHJFUEP7Y0kQmRUMlxlSI1TEnDTlc+13pXbx0vfquNkD59Dusknob8sJw=="#;

const ICE_CREAM: &'static str = r#"eJx1jDsKgDAQRK8ypDfuxg1JEQN2FnqIgIWNYOH9MfEHFmHZYob3JuzpWLH0anNghmjyYNKdTQwGXcfaC2jiQqgY2qLE8Ipz7t1gYR86pwqUDMw9mF/GX27kk07c3SHz"#;

const IMAGE_MINUS: &'static str = r#"eJxNjsEKgzAQRH9l2Htsdos2QpJzD+21d0kFBVuKlBL/vhtEkUDeDjssz3+674BnoLsw2h/bTiCw+tjodK2P2cjjkKF5uFD0p3Ij+ml898gciBtClkAihEVZK7hAq6UUfRrnNPWYtUNIOVCrWAq0si6j38xeasa1OVfWNeu/O1gjlRMHe2sgvKv8AWhzM8A="#;

const IMAGE_OFF: &'static str = r#"eJxtT8EKwjAM/ZWwe2OTNbJBN/Dmxav3ocIEEQ8i29+btHW9SOAlDe+9vMbH/XmDlYaGG1hYm/a19CWtx7gz0hhf03uG69CcyGMgSDgxMHjQl2Ps2gSmMO4YN3NqUYovZd+8sYv7eqHyf2koq0iKqvsTR516MDgQ9j0k8KlakA+FktFKp5nCBUUsNHphh8w6BXJqUJNv5qxfk7NUD8eOj5X5BWtgS9E="#;

const IMAGE_PLUS: &'static str = r#"eJxNj0ELwjAMhf9K6L2zjbSr0PbsQa/eRx1sMEWGyPrvTVrdRqEv7+UjJP7VvQe4B3FFDRo/bYeAoOhpSdXZ7L3E284D+aEV0R94RvTT+Owh6yCMgIxFFhJEUkq1ZZShH1rCU2VdZYulGDc0jXOaepg5hLQEQUjKLITUZvT/Ix58hJHHRjlb/3VdJbFx6EBdLKBet/4CC/c+nQ=="#;

const IMAGE: &'static str = r#"eJwli2EKwyAMha/yyP92mrLiQD3BLlGsTGGDIULr7WssgXwh7322xFDRHC2EcjrijjZwjl+K+ZOqI20IR95rGqe3D/G8DbmEb0To5RchtIEifq/cobf/rSbsjn6soZ/TMiuz3ntjMJTMxLNhA/VewVpkkfwFdjApgQ=="#;

const IMPORT: &'static str = r#"eJxljLEKgDAQQ3/l6H7Yi1U71M4uXd0LDl0EB+n32w7igWRIeCEJV74LHatJAhqrwMQwdBbD25yeRMh1sfvXydO0uQwC2S5uqYr9QHMUmTVg7IteMIq6fgDgbyNR"#;

const INBOX: &'static str = r#"eJxVTdsKgzAM/ZXQ92ama1OFWtjbXvYRwgYKRQsTYft604KIJBzCueSEvKRfmuYP5GWa12+vjAGS5YoWyAE1BdtCFE3FcDtSMeRhHeHdq5dD68AhUTVtPBR3U0eukS6ENpvmpO+S0Yxt9zg1YvQM9unR2DOiCX0HJO//tV9a4w70CCtk"#;

const INDENT: &'static str = r#"eJxVzMEJwCAQBMBWlmsgnILxoXaThyAqJI/YfVQ06Ou4ZXZNTqEEHy/k5ONzW5LQOMECEqzImWMKZ7orbIkF4a1XMKGI8bfLrbDgEXaklg63d6d9R/87m635xB/wsi6p"#;

const INDIAN_RUPEE: &'static str = r#"eJxtyzEKgDAMBdCrfLpbG4OxQuwNPITokEVw8P5YHaRDlgT++1+v7TYcS1gFbDSEov0bFW0gu3AKiJHjiOyuiI0dmCvsEkUmJHy/o4RUz19+ACWGKAU="#;

const INFINITY: &'static str = r#"eJw1jbEKgDAMRH/lcI+2aagdav/AH+hW6uAiOPj/eBXkSAJ3eUm+23Pi2KbdK7x2UdE5rmJUFGsGg4NnOaSu7CZ+DgEM60WX3gAwFiN+4JMknuMwDEKY1qnkZXwsLxFqGW4="#;

const INFO: &'static str = r#"eJxVyUEKgCAQQNGrDLOvHIloMXqDDhFTYNAiJERvryIIrj78x/J4eW+QZJA0gsRWX6LQ8tLc8nf+Di6DB2mgLUxrxTpH2t2sqFMG6SQaTA=="#;

const INSTAGRAM: &'static str = r#"eJwtjVEKwyAQRK8y7AFs1qSmHyr0AD1EaaQKpZQgVHv6uhr2Yx6zM7t2D4+M6kgTvmnLsdFE2Iujc5PaJYb0jHlsikS9PUnP2889R2yObmzArOb1umDBBJbRysy4YDjdO0I/OSBVb1/pHVDZkVHtUdUHlObw2kkPYulI2v8BDgEsAA=="#;

const ITALIC: &'static str = r#"eJxVzUEKACEIBdCriBeYUWoRWLeZxcAw67p9qRW0+qjvo3zv/0DjjAGhUkZKI8dIN0IjXRe5FBUxaia4iU5YKVue1vvrZpU0v8RNO2FJIKQ="#;

const ITERATION_CCW: &'static str = r#"eJwtjFEKgCAYg68yfLfURP4H9QYdIipQEBX0pduXEnvYBt9m69EDLsd2JSDFKbheNN8Ww2moccIsmI7hFIh5u46ht7WkJ8V8o5aYe3NMGkiNcUb4slKT/Sn/AgR/HOI="#;

const ITERATION_CW: &'static str = r#"eJwtjEEKgCAURK8yuLfUJP5CXbfpEFFBgqigm26fVqt5DG/G5K1eOCxbNaTYBdeDxjTMnECcCnVG5zd7u2jmzNhnzuQU7uDjiZx8rMUyglJoT82D/MRfcQ99UxvK"#;

const JAPANESE_YEN: &'static str = r#"eJxtizEKgDAQBL+ypDe6F+5QOPMCbe0FizQBC/+P2ohFuoGZ8XO/Co45rBRMUTdhHToy6mJI1WAPcUQK2fu3zf4dBmqhtA3/5gZNNhsY"#;

const JOYSTICK: &'static str = r#"eJxtzLEKg0AQBNBfGbY/kp1EQ+Du6jRpLezkFBQsRET0771DUAvZYpl9y9ihmlrUTv5U6KciiGcaQ8Nfds3gzDPHzVbf14PhbFiKt49U6u1RnUOzSDeijFR8TwndGPoGYXWSC8LiRCkYnbzSz65+AyHhLXo="#;

const KANBAN_SQUARE_DASHED: &'static str = r#"eJx1jzEKgDAMRa8S3EWTVmug9gYeouDQ0UF6ftNFKyRkCf+Fx0+88l3g3Idjg1DDkOLUkhTfHEmA18AqgBWwgMsEBHObUTblhsEV1KTeAtxZEXQrIXBFHaA3CX9io67UIlR7sQUWAb131LzOquV+jzyUMHME"#;

const KANBAN_SQUARE: &'static str = r#"eJxti10KgCAQhK+y7AVCizRQb9AhIqX1LWTp5/a5BNGDTzPzfYwraWU4c2TyqCwCpbwRv/3y2COUGhrhlhFcJ4fg9oUJosfZgjmMcCE/rnQVQ0uMVUyfeABtQyT9"#;

const KANBAN: &'static str =
    r#"eJxty7ENACAIAMFVCAsYjBILZAOHMLGwtDDOrzRUtn95WX1PGBUbQz5EqBIsqThQfMI/KLYklwtKwhXO"#;

const KEY_ROUND: &'static str = r#"eJwljEsKgDAMRK8Ssm9tbfws2t7AC7iTKFRwISJFb2+CDPNZPCaey11gTTi14Mca2IHtLYFXFaomlKDRHt6SES+97UDthHCGDM2YY6M3OfJ+8bEBPwm9MAj8Jhx0XAmlBPyR/AEN5R52"#;

const KEY_SQUARE: &'static str = r#"eJxljT0OwjAMha9idbdJ4rRJpNAT0AuwoTBkCBID6vnxq4AF+Vf+5Pfq8/bqdD9Pmw8SKUhqUliKbTOG2tGNWWayMoQAIwc0WCXZkhoDsAHrx88lGymSQT55vIFep7WeYLzWr/3DRzIl0n+0FdPzThYK5POuzZEsOCF63Fm7ooXhJbLVT+INd94zSQ=="#;

const KEY: &'static str = r#"eJxNjEEKgCAURK8y/L2GikmgnqBLxC8oMAhpUbfPTxAthhmYNxN5q1wW8JUoaE/gO5Hxkmoi8Ry7l8nxmM4Vc6LdGlg16B5NAkjxq2WP9gYHN1qLUJRT7gMfJm4ehQ=="#;

const KEYBOARD: &'static str = r#"eJx9y8EJgDAMBdBVQgbQpEjtoXUDhxAV05uUgrq9rYJ4aS8J/z++DescIZwOFUK4npduh3D4JUrKhPCqrH6T6JA1DrbNu8HuUxRYHI4ajDREnCmXP2KqWFcxUzYDrIo7VUNdwR5YC9MnNwQJTpg="#;

const LAMP_CEILING: &'static str = r#"eJxtijEOgCAQBL+yoRc5kCOXnNQ2fsCOxILCwsL4frGhotgtZkbv8lScq9nJw7/RZJ1/lLULRqrkrwWytZ/kGDRiKYG4BAQ4UFu0zHA9/QDhERtm"#;

const LAMP_DESK: &'static str = r#"eJxtjTsOwkAQQ69ipfeS8Wx+0pKahgvQRVCkAIkC5fxMlCZSotHYzbNdvtNvxutafSyjocMhdOjZs6Me1VguKzGWA0ffNBJ+wt2H1KCNzzC9He0JEltaqGdNS5YGCqLmPIWjjrNwLbrt+v8SQytU"#;

const LAMP_FLOOR: &'static str = r#"eJxty6EOABAUheFXudPNsDHbJSteQLMJgiCY53eVm5RTvvPjantAj6IEMMNNCz7TSl9FQvUwIV+0gXC0/QjFVDNcHpwYWQ=="#;

const LAMP_WALL_DOWN: &'static str = r#"eJxtzaEOgCAYBOBXudGZ8oOAG5IpVoKNzUAgGJzPL6R/c+7ifbcLV7krzk3sSkHpapuGS75p6Q4RwzTqGBiZjrIvBMI8IklS8j/SYK3EDt3l5bMzj+WTF6xiIpw="#;

const LAMP_WALL_UP: &'static str = r#"eJxtjb0KgDAMhF8ldC+a9Heonbv4Am4Fhw4dHCTPb5wCIjfcwX3HlavfA87N7IjgR5wOUsvT2XSYWpa3rUUZD4gcOgHBKkIrqeUfUsAwSEFxYvoMPduoNw/Z/SLa"#;

const LAMP: &'static str = r#"eJxtjLEOgCAQQ3/lwn7INcRggswM+gNuRAcGBwfD93suupAOTfPaxqvclY7ZrIFQw+lJXPaLhs2kOLw0xa8jIEEbO0QHaIzdsVixE4PAqL6ok1OJOhpy+G8f+w0f5w=="#;

const LANDMARK: &'static str = r#"eJyFjkEKgCAQRa8yzAXKSaqFeZuIIFSohd6+GS3BNuHi8533hjHH7laIasEBIXEQIUTiVNwpd2s6oazJrEBqLkPFkLhjccYv+iKSPVZVFO4tnD81tpKu0v8ReTO1cPBH2ryD4Hd3nTwj4NfDBBomwR7A3m3bRBo="#;

const LANGUAGES: &'static str = r#"eJxtjrENgDAQA1ex0r/Im+RJEbIBQyBRfINEwf4CGihI65Ptq8d6OrY57BkFBgutDk/W6ksSNMHEQBn/eCGyKztgAl07eyRIyaJRMjR2mvefFv9cLstPLVM="#;

const LAPTOP_2: &'static str = r#"eJwtzEEOgDAIBMCvkP2Atnrw0PYz2tgmxgMhUX4vNJ4Iy7CJ6y7Eb0YEPf2QlhE2kGasoFb72cQSOxpZQKwuS5r8r6Sr35U0WDabCKNF47/6HNZV+QAdWhw1"#;

const LAPTOP: &'static str = r#"eJxVjbEKgCAYhF/laNf8L7WCcnaotV1oaAkaoudPnYobjvs4+KYr3Qf2uVlpIH7rE0GYEkXF6L8bfMZTPEy0tSCaA6idSwKpL1F6zNi62GnPH1YFLzZbmjC1xRtexQka6A=="#;

const LASSO_SELECT: &'static str = r#"eJxtT8tqAzEM/BWRu6fWw2sbtoF+QD/CbA97yKHQkkO+PtIuISEssmVkjWZG8+/4X+nn8/RdSWQUKpQ9OEmy03n+iPZ5fgHxhK4L+gRT4nirkaB3dD4YUKgR29eERnE3chLivORkMCFPLTX/SO2PMylKFNRGBXfa0u4IqiQHEoW4DXHOYHZocj9btYfdDmbYNjZZE3If0OLnISPmUnJNnL1h4X837RPesAU51gAXZIWwl5caqMnhGrQPHkbpayzZL0lQqu/WyrtYA9s1Pz3eAUYLWOM="#;

const LASSO: &'static str = r#"eJxtTTEOwkAM+4rVPeGSS0qGoxIP4BEnGBiRYOL15FrUqbIS2bITt1f/PPG4TLczVLvDURJCSjYt7TTspe2hyhVi15kDY9YoFFLuhYwNxk6RkuItBZXn5Ii0PPegXTKt/w4nOehwSHSFrr8zSIZNbbDvfvMDzDwp6A=="#;

const LAUGH: &'static str = r#"eJxNy0EKwjAQBdCrfGZfzY8kKCS5gRdwV2KhhSJSXCS3N2MslCx+Zub9kJctrxNyiUIryLXn1sJICud+T+E9fmY8o9x5BS+jh4dpj4OHw2Ea3Ez70KY2UliX14Rio9xOhoLK9hOUHlX3alUdLN0Pq6Lb2b+76y9Jfi6S"#;

const LAYERS: &'static str = r#"eJxtyzEKgDAMheGrhF5AzJIl9jYihZIUdPH29mkUhJIM//A9bV7PzY2aFzv2Jc1MOKEe6EhOWaewWRG12PqtupXgjH410Jg/LzfnH78AQ4cqjA=="#;

const LAYOUT_DASHBOARD: &'static str = r#"eJyFjksOgCAMRK/SzAUMfmMCXEaJsCUk4u0F8QcbVk3TeX3DrVocaWU27QRm0CHQgXazOi0wgfy12zAYJG9iXPIMGh4ohXKW9QX1XQPEWvzdKV+T3RpfFI3vxpc6AUArPT0="#;

const LAYOUT_GRID: &'static str = r#"eJxtjUEOgCAMBL/S9AOGYOIF+IwSy5U0UX6vRYhAOG53Zmui3xnibVEhkA8nscUN4T1ohCscTDknyc4sgjuTpYYu/oDLiFoH62e+dvI7TbSm7hZ0xctOtR5D6z09"#;

const LAYOUT_LIST: &'static str = r#"eJx1jEEKgCAQRa8yzAXKMiJQb9AhIqVxFzJQ3T4VwVq4Gua//58Kbmd4NI4Il7dMGmeEO/8hHoFAzh/EKTeqS3Wj8qjguqrFIohaIevq3JjAalyFBElZl6I/WFpATC0y9B/yAnvFO24="#;

const LAYOUT_PANEL_LEFT: &'static str = r#"eJxdjEEKgCAQRa8y/AtEFNTC8TIljVsZSG9falm4/Pz3ngluUxLnD1HGuIISYwKFeC9QLOP0uwpjgTVD5q0pVkbmTmhoa/ZWqtZ3/5ynWFuvdgHWZC4g"#;

const LAYOUT_PANEL_TOP: &'static str = r#"eJx1jTEKgDAMRa8S/gWkKNSh6WW0mK4loN5ejS3q4JIhee8llDQpScqLKMODNkYP2m2ueVZhuBFUzr1DDN3Fx2BWPXvD3dCgmniaf9b36ytw55p2APbqLiA="#;

const LAYOUT_TEMPLATE: &'static str = r#"eJxtjFsKgCAQRbcy3A2E9AbHzZQ0/spAtvsUMiL8vI9zbPSb0hl2FYZZQBejB8WUE0h8OEQZMyiV3tmu/J39UuszNqAsM8OPaqrNVM/VOr7YDe1KLiI="#;

const LAYOUT: &'static str = r#"eJxdjEEKgDAMBL8S8gFJvCi0/YwWWxAPpWD6e5PgRS/ZDMxuaHnrIBFnhOG3KbDG8Ljr3ktEWhBKrkfp/qcwWS+Fs14ZBkVctc4eQj4jSkxmmvOawn/TiD7mA074JpU="#;

const LEAF: &'static str = r#"eJw9jbEOAjEMQ3/Fur0hbps2lcpJ6GZ+gA0dAyMD/y/CDSdbyhA/e36e3zde1+VOIuuto0NDxBBHE240MRjYUaU6OJD3iIaq0ON4xCOTqvR4a2LQ+ljWefl3r/NcCIa7pgKKWzIpLTj11LYRE6xiGQyXw/ls+AEmyyMJ"#;

const LEAFY_GREEN: &'static str = r#"eJxdUEEOAjEI/ArxvmOhpZRk3cR49hEbPXj04P8j7SbqGkJTOnQYZn6urwfdT4erkMiNITrBm5FAbGK4KWX4FOWq0ErjSBEc740YSiUijajgZtGbNQeqIzcko/qkYK7nPSAJrtR+2rlnjSLpuuuN/y1zjMz2R78xjyFfNV1HaV2T6iWjuBMrilBBSXGvMO6FEDf0bVwqscNKZ09cujbL4Us4c1jmY7dqmXeGERvZB3sD0LZBkw=="#;

const LIBRARY: &'static str = r#"eJxtybEJACEQBMBWFht4Vo7D4N4OvogHAxPBQK5+MdDIdMb6PyrKGxoVCgElZHuWZtv3MUL9OgnJGS8hEKeemPErHVU="#;

const LIFE_BUOY: &'static str = r#"eJx9j8sKgCAQRX9lcK/lA0pQ/yUsKCgIaVF/n45SLcLNvYs5XOYYvwS/ThAs4S0Bf8YWsS9sZ5p8d2YfjhlGSzbFtIQSQmEkLt0/FFesl6AZ75CgVSxnbQ6HEKM/WHEoTycX9ai8Cjcz/D0Q"#;

const LIGATURE: &'static str = r#"eJx1yzEKgDAMheGrBPfG5BklQ+0NXN1Fh44O4vltF0Gow4MHP188tyvTMXeLE2T1XQIYpOzByMqURxICOzsN5aNLsa8oxZdOpMjWCGq14PYmgvyhT3kAImAq+Q=="#;

const LIGHTBULB_OFF: &'static str = r#"eJxtTkEKwzAM+4roPVrjNE4KWX+QR4TtkMtgh/2fOYHt0mLL2AhJLu/26Xjel+qVGd5THsyOO2xzMmegNIViHeV2BheXo9yG8ig//UtgtVqfuaoMMLSNmjDH9BruiCMxMdkRDHFCeBFR7anc9YLwlit9+zNfFTMz5w=="#;

const LIGHTBULB: &'static str = r#"eJxtTTsKgDAMvcrDvbGJtlWoggfwEFKHjg7eH9NSOkkI4eX94nO9Gfc2nOzAcyIxDAqGKYDJGSF9G1obkHoncoeHh63jsSQLdQmaACqgUBOmistq0rDHsfTtsbdq8JL9D8EWInnuzAfmLiZs"#;

const LINE_CHART: &'static str =
    r#"eJw9irEJACAMwF4p7iJFBYXaDzxCcOgiOIj3WwclWxKabQn0YqoHvzEJJsPkrmV6bWCGbCNEGxQd/3IAMF0Q5A=="#;

const LINK_2_OFF: &'static str = r#"eJxFyzEKgDAMBdCrfHoBTbBEoQpuLh5CUFAQcXCwtze11pLh85MXd07Xirk1YwOSQXoLi1KHIBDTuSKAzv2MLGTlKbMKdWb7dizw1BpiA88xb+21Rqxqg0pWl8zxyJ8Nr28k+gA/BCjz"#;

const LINK_2: &'static str = r#"eJxNyrEKgDAMBNBfObKLplCj0BbcXPyIgkIFEQcH+/c2KCIHgdw9d8QzYfY09WAZZbCwaEoYAkmGgquVBPdBtjrEB3JJuU2qfnRb9wXZeGJDuNhTR8j8vlq3alWFG/x0H7M="#;

const LINK: &'static str = r#"eJxVjbENwCAMBFd50ZvYAcsNYYMMgZQiBUWK7K8ATUC2rnjp79NT3hvX4U5hSCgKBY8zr7F9DTSlZJ5toJJ429EgLqeta3L6ZREiS00jdRsFrCNsA7WLaLF9iJQicg=="#;

const LINKEDIN: &'static str = r#"eJxNjGEKwyAMha8S8l9WpdgNjDfYISSVRdiPIdK1O/2MgzICeUne9xJeqQmshHfr4Zo8eJh62a5+W8TMm1mSAzeuk3FG53MDdzJ/SeM/GMNFP8dQMzd4l7UJ4YwguTykEVqHsBP2fhDeFFcwBi6VnxnqsHgfGT5UOvIz4xdLbi3e"#;

const LIST_CHECKS: &'static str =
    r#"eJx9ybENABAQAMBVPnqRR1A8GxhCotBIFPYPui+QK49GmQ1qFN0AetCblVYkUicSsf5sRgOuhXugfk5gswAJAiWx"#;

const LIST_END: &'static str = r#"eJxtzbEKgDAMBNBfOboXexGDQ+zs4kcUHLoIDuL32ywiWG4JeQdnZ7kq9iVsVFDWMWQb/JftK9qHBM5dEUJvpiIQpBbGdtU4/auH76qz5/UHXPgoUg=="#;

const LIST_FILTER: &'static str =
    r#"eJxtybENABAQBdBVLhbgIyjObWAIieJKhf0jmqu07/GeR2l1NxIVRXPC/pGwRSVERfgMAqFptrlXJRXR"#;

const LIST_MINUS: &'static str =
    r#"eJx1zCEKACAMQNGrDLvIFIZhLls8hGBYNHh/ZEWL1v/g8+xLYRTXEAFjTU44WBO+QkA/wPyUaDP1dGgD8Zsc8Q=="#;

const LIST_MUSIC: &'static str = r#"eJxti7EKgDAMRH8ldFebSEOG2NnF1cGt4ODo4P9jSkAcwnEcd4/Tuz0XnEvaCAHLzqnq1LeqH0EZC6A0sujOgOY8ePPFVY7oToC0zhFh4BjYRX7kBYkpKn0="#;

const LIST_ORDERED: &'static str = r#"eJxtjcsKwkAMRX8lZB+dG8dhFjNdu/EjRIURRARF7N93UkoftIsQcs9Jkp6P151azRyY/sgMV3sdFUwtLG7S3qQm9eqcQacdO1HnpdxLcYBx9WCU35dvoVvms6dQ8PNGLFsQuKIbIBDiyV+dgFS01uEjEOyOooLR7wD4HzuV"#;

const LIST_PLUS: &'static str = r#"eJx1zSEKACAMQNGrDLvIFIbCNFs8hGAwGsTzi2ll1v/C59X3hJFNQwT0NZjC7rXCIgT0A4y6REiHFPDvMq3QBSuRJC4="#;

const LIST_RESTART: &'static str = r#"eJx1jL0OgCAMhF+lYQcpLf4kyOygD0F0YHQwPr+FQRdMc23vvuTCma4Mx6w2h9AvpGLoShTDCwZA9wfGJkAnJHnwYOtMmoCNr6qJlq9o12iIxDvjxbImwwgoa0W53OqW3N6cP/YAIHQwew=="#;

const LIST_START: &'static str = r#"eJxtzLEKgDAMBNBfCd2LzYklQ+zs4upecOgiOPj/mCyCtIQsebnTuz6NzjXsnImxzaHo5LeiP5GxJMpDAFvkkAoCJZ+IiBaX/vWydnEl249fMZwoMw=="#;

const LIST_TODO: &'static str = r#"eJx1ykEKgCAUBNCrDH8fYZa5UG/QISKl7yIIEarbp6taFAPDwBuTwpKRTkuCUFoSLksD4Yg+syVF4BBXznU609a7M/ucGd7SJiFGdCV901eu8PAkJBTrbxDdr+iX3DbWLPw="#;

const LIST_TREE: &'static str = r#"eJx1izEOgCAQBL9yoRfZw1wwOaltfITR4koLw/sJHQVku50Z/e7f6D3cxSCwLcllXduZtUdyTgDSuIkkZXsCwcPvxG0Whx5CkalYAccUKro="#;

const LIST_VIDEO: &'static str = r#"eJxtyzEOQBEQBNCrbPTyg/yNYqk1LqCTKDQShTi/1dAoppk3Qz2PCsWJqDQoHYzw9O3O0xUEfANf7EsaXxh/MJIzJaazWfatHtc="#;

const LIST_X: &'static str = r#"eJx1ybEKgCAUBdBfubhH3TIpeDm39BGBg4vg4P+jTjroeo7EP3m4R30kuL+HsrJWs9LGwMyC12gCb3BbNPToznLoLwOnqSR+"#;

const LIST: &'static str = r#"eJxljkEOgDAIBL/S8ACVkpgesL/xYGI819+LIBLraVM6C8P7dqyp5QUyQjolZwnUaBIFKo83U9lInUVBnpitKPlldVh8j7HR/e8l+6NhwtfBjDo2IDvr7cems8CwIAgpX+L0Bd2KQnM="#;

const LOADER_2: &'static str =
    r#"eJyzKUgsyVBIsVXyNTJUMDRKtFSwVDBQADJ1zfSMDC11LfRMzZTsbPRByuwA+mYKxg=="#;

const LOADER: &'static str = r#"eJxtj0sOgCAMRK9iOABKNSAJcBsXJsY13F6gBRRZ9femmTHXeR+TB8sEsCnEKmMRlsXJi7x1Zk6QM4jisknSvKMURvDG9YoYdRFUXEl8kbtOlD5rvih6LjlsZAbbUIG/UNK5OKMYwxR7Y4rGj1JkCA1Vy6ho1nK2TgYl8st7F+4T/wGiFGDj"#;

const LOCATE_FIXED: &'static str = r#"eJxtzMEJwCAMBdBVJAsUU0oR1GXEgyA9eEq2rzZVS/H0SfLybU5XVKQdICiuoVuiJNU8wNutIW+F1h2ubJvND7P0Ek57vBQXvbr3moGeH/zgkErIUQWSc3FwggrcG+U8GQ+2Q3+a7AYa9kIb"#;

const LOCATE_OFF: &'static str = r#"eJx9kM0KwjAQhF9lyT1rJ7H5gdaLZx9CqqAg4sFD+/buxpRWUFnYJcPst0y62/V+psn1Bs7QiN7okGdraEJRd91GTbvu0yrT1RXkP+YKmq119atVeVPl/TI/js8LnXpziAyQtn3LyVNin6klNAwdbmjIc4rS4CmWAmdSNcDqxpZTtk4AClfsCo7EEQTPOQyMbDl4dtlC2nvmQI3VA1YP2FiKQxQV7EUrHvEu8BJzXMVcPryGn0O+AMRgXpY="#;

const LOCATE: &'static str = r#"eJxljUEKwCAMBL8i+UAxUEpB/UzwIEgPnpLf1zS2tfS0yIyTUMuRHfsICI4xwgpO+vj+FH9tCotKKZjaIaqrcH+k+9NXFgvyCLJ/w7/uYDwf362Lk0ylUc2O2DQS2xZhU8lwOgHZQzcP"#;

const LOCK: &'static str = r#"eJwli0EKgCAURK8y/AvUr8QW6g3atpeUdBciZbfPTwzMDMMbU+JRUV5LE6E7M+HJoaZeV0KK+Uz1n5ulmVCaoM4McnTm8jUhWNo0mHftFRTGLgb3uBchhXEf8+Mb1Q=="#;

const LOG_IN: &'static str = r#"eJxNjEEKgDAMBL+y5C6aqHip/YGPEBQqiBYUsb93LR4kh9ksk7g4ngFTL4O2qEMzGgwVR0m79FcUTKFoxLvyPfIu7mtal21G3JftPHpRWh34SA3MXVY/ybusJqNmgpusCeXaCpLmmn52H0wsJss="#;

const LOG_OUT: &'static str = r#"eJxNjLEKg0AQRH9l2F6SGZKIcHd1mrTphQQURA+00L93PSxki7cDbybkdunwi/ZpIL6frSDc/Vip0veS4bl7WAq3o5JCnoZt6Mc/8tSPyxyNL7D2EVDwvy7qKaVQ1JXRRMOqaI1hc1BOFrpf3B0DcSZY"#;

const LOLLIPOP: &'static str = r#"eJxFjF0KgCAQhK+y+N7PbBI+qDfoEGJBQUFID3X7VoVimB2WbxgbtxT3hZJTRlG8nQIkn5LedhV7e4Zrpdmpg0GMRrdDdm5k8vMJICAwMfVFuljXrzFyR1FlkNI38QLEyCFR"#;

const LUGGAGE: &'static str = r#"eJxtjr0KwzAQg19F3B56J0zwYOcNumYPbsGFDiWU0r59bQL55yadpA+F1/DOuEW5tqBmHQhCy1nDhr1fNIrOxvUD/NimAWaVLlwqtAsz2sN873Yotye5k6ZpXbVy0mNMzzvSLwpVkL5RrBWMRdbQZJ/G/CH1Bza/PtE="#;

const M_SQUARE: &'static str = r#"eJwlizEKgDAMRa8SsotUi2RoegNXd7HFFBykBK231yB/e/+9UPOmILnsooyOEGpjHBAexhHhLknl581ADL0FMZyrCiTGmcBNCx0evnX+IlPsjC+MkBj2"#;

const MAGNET: &'static str = r#"eJxtjUEKgDAMBL+yeE81TdoqVF/QTxQ8eFDw4P+xLSIIkjCXnWTjma8N69wdHuxISeFNcFQQcjBhQsNQhsF1Ewus3R9VpqKKz9awoqGqAwlJqh+7Jfa1YolvkcMIhf4kbMvFJ7sBFZIlMA=="#;

const MAIL_CHECK: &'static str = r#"eJxtzLEKwzAMBNBfObJLta6unYCbuUO7djftkKXQIeT7YyckZAgCgbjTS/88DvjemxcJu75DJghXRyh8+OMNTsaPg6lphxpwaJs+XSrSp436FSpKq13ETWMuXY9lVcaE6gLcs3ROXi3AVtqL3/MZZ4cmtA=="#;

const MAIL_MINUS: &'static str = r#"eJxljLEKhDAYg18luPe//kFbCz3nG8719nIOXQQH8fltBUWQQCAk+eKS1ozp3YwktPu5RBC2ytDw094zuCn/FioqAbVg7pshvipkiCdqLihvegkenfhUti0Oqxg1FOtgv2XzvI7qoCG7q9kBi08l2w=="#;

const MAIL_OPEN: &'static str = r#"eJxNjbEKAjEQRH9luD7j7prL7UG82kJbC7ughcUJFuL3m4hKWFhmhhlefpTnDdfdcDSlwRkvHLl1OueJDmV6qRSDQeppqGofex/s1BdAD3W0ekhdaIyQ1ZHOw5I3jbnkH/luBpXQgBg5FeUc8XlfAiVBDq30H78BVKQoIA=="#;

const MAIL_PLUS: &'static str = r#"eJxtjMEKwjAQRH9l6D1rdqybBmLPHuzVe9BDLoIH6febCKWFloWF4c289Mnfgte1m0jo+WGZIHw7R8dbv83grHx6qKhENMAydGM6NcmYFtW7qoIbJAZcJOTa7fF/TaOO4g3+Xjv76aQRarMdEYPGspIfyCktRQ=="#;

const MAIL_QUESTION: &'static str = r#"eJxtjs0KwkAMhF8leN8xme72B2rPHuzV+1IPvQgepM9vUlAEJUxImMlHxkd9rnI7HWZSTFGubaVQNCox8Zy/d+FmXFQMhkHC4GpEOUzjMUjT+ObdndelHkMnBV31eJa9BckSoa3oxTO/p7P1YgXsFzAhoyQ4J1klTEI7wqcWeUETCQ8UxzdhoIk/Q3/QVCE3qH28F1Y8Opo="#;

const MAIL_SEARCH: &'static str = r#"eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb++64j9GSELIe7+kuYX7gGOrehGwkN27IBCYFFq0bMvfGTKxRAMmphppIYMnq7pmk0Bd88ZdZ5zXFdUelnyY1SWWkkCshYyD2c2atbXnCsKhQJGky12HPOVwkzl8bfE8xssJ8dEqrhTiM/exVUUS5fX/YyKayab8YF6EkD8y"#;

const MAIL_WARNING: &'static str = r#"eJxtjLEKwzAMBX/lkd2q9LDjBtzMHZq1u2mHLIUOxd9fO5AQSBAIxJ0uffNvxvvWTSRMJTz7TBDaxtHx7vc3WIwvhYnJgAY4GyV0Y7q00pjW3qf2orvKEBEk5qp7LKuVzFG0hz6qc3ydWA1f/Dkhi6ht7A/ZAy6D"#;

const MAIL_X: &'static str = r#"eJxtjLEKwzAMRH/lyC7VurpxDW7mDu3a3bRDlkKGkO+PHAhkCBIC8e5emeo84vfo3iTs+ukrQYQ2QuEzHn9wMX4DTE0zGuCYu6FcmmQou+rvqiR3zQk3TdWzEdtpGhNq6BFenjmpWoJvRDzTmjM5shVquS2S"#;

const MAIL: &'static str = r#"eJwdi0EKgCAURK8y/H2mUlmgnqBLSEq6CEKE6vapDAzM8J7O4SiIIZ2xGBIL4TUkCZ+hifAkX2LdnJD7b/XYBKtvVyK8oUtKqGFlm8LMlBNsm9CL14hBMr6A75VpapPsD+2yHQM="#;

const MAILBOX: &'static str = r#"eJxdjsEKgzAQRH9lyLmm7pJoBPXixUuvHnqTtlBBVKhI8/fdxJaWEpjNwGPflku/3nGt1IkZlPcMRiqPEvm15rcn3BXaNowcBhaZtrAtuQvrgBiQdpJmc2dVl8ewty6XefTjMN2wzMO0PipF9lCA3B5EkXwzQn9uibubQixEopMQc/d/3pZ+RVHiSQSpgud9PqVnMqTmAY2WF1pEOuA="#;

const MAILS: &'static str = r#"eJxFy0sKwzAMBNCrDNpbjeTUdsHOCdpDFCfUWRRKMP3cvnEWCVpoYObFZcoVv0Q9YfkmUsJnHmtJJI5QpvlR6potYS0dDfHUwBBf91owJnqqwhvPAss+ZMNnz9YIq91eQHd18M01cbibIrxFcgdh4Qu0XZF+X/4BM/QnRQ=="#;

const MAP_PIN_OFF: &'static str = r#"eJxtjt0KwjAMhV8l9L6xTX+WwjbYA/gQQwUFES+8mG9v0k4HMgpfe8LJR/vn/LrCeTDHhDGAYmJ0GSpcPRG8OzmQGXiqmKlgidDYSskmM/YH1Y39T+oLelkImEi0WbX5qyUn3plFWLNly9Ahr2gzXUSmHbOIAygmwo6honmLagOENerrv4LcWVnd+3DEIlpKm8ES+qTIW/9+e1xg8YMhA2+SS+8WlxalqqXxAx3wTS8="#;

const MAP_PIN: &'static str = r#"eJwlizEKgDAQBL+yXB+8iyBXJPmBH7ALp6BgIdFCf2+iLCwMuxOOfK2YI42eIWyMwSnEf3U6dRWd+KxQcI1ABvBEKXTNTMG2YvsCeyIJE0qknmB3Jd9O/5xe/vgZuQ=="#;

const MAP: &'static str = r#"eJw9jEEKwCAMBL8S/ECJoihYf1OKICq0h/r7bqz0ktmEycbeyjhbpd5yva9dGXIUyBBbBM1IGOxlRwgS5aRS3NZriiXXgwbjWdEDBEBPDIC9yOIsUxS2nyMUCY2zwf3uC+jCJtc="#;

const MARTINI: &'static str = r#"eJxlyTEKwCAQBMCvLPZH2EvhBS7+IB9IF0hhI1iI7xcbG9sZr1/L+O/wGFSzheTHpOQrqCA7uVfhhVMiTKLYu34A7k8XQw=="#;

const MAXIMIZE_2: &'static str = r#"eJxlzEsOgCAMBNCrNL2AFmRBAtzGBQkBEl3Q28tHjMZNJ2nf1OQUOPi4Q04+nodFUiBB0BganVkmceaH9SMlkPriDlnUjytCabnVJIuCELimbP5lS9/1UyPlrvKoTnsBLFoxPw=="#;

const MAXIMIZE: &'static str = r#"eJxtzKEOgDAMBNBfaeYb6DUsE2Mag8UvQVQiyL6fzQBLlhN34uXilW+jc3V7IN2WDALNLVxXUZfi1ESKr4NQODrIMB5JJfFFP1kbNoLiCWKdZJTf5wORMylt"#;

const MEDAL: &'static str = r#"eJxtjlEKwkAMRK8y7P9GE9vdFNqC//UQZRVWqCBFRG9vasEWlUAgmZfJ1Nf+lnFs3CGSMLiEUAiIxEUvEGytGMQ7LyRdQYXpul+UAMksK5QpkA7WDfzymG6l40CxskeurTfT87b+RGA7F5Rkzchf4MK7GVD1FekfB0XMq306j2k4IT0bx9FhbFzpkB42vd1neR1AwHr3kj0t+V4uoEZh"#;

const MEGAPHONE_OFF: &'static str = r#"eJxtzMEKwkAMBNBfGfa+sZNs1wrb/QOv3gsKCot4EGn/3rSIXrxMmPCS8pieV5zHcDyIZmxhIF/WmIQJ5hFq2a2ulq9Wgr1YOuUW92KESmd/HCkZzDJM5n87+FnsZYi+/ul2u18w6xhUA2b6DFg+ddmq0xXVN+EpKyQ="#;

const MEGAPHONE: &'static str = r#"eJw9yzEOgCAQRNGrTOgXHVcIBXICPQSJhY2JhaHw9K6FlDN5P1/1PrAv7lSQYJLQOK025ib6uJKHT5T8u430EYw+VYVihEUSfBK7u34By7AVog=="#;

const MEH: &'static str = r#"eJxdjUEKwCAMBL8S8oC2ESwVrJ8JHgTpwZP+volBKD0Nyc4mkUvjmoHHjeQQuBub4MAUd8tTrOXJ0J2sT4RBQo/QhZeMbo5iq7VcyYKpwRSBHgjbQT/3k5LXeLb1g9WX/QKwFizD"#;

const MEMORY_STICK: &'static str = r#"eJx1zjEKgDAMBdCrBPdok2pVqM4urg5uBYeODtLz2zqUCg1Zkv/4EHu7x8O1NLsBmgPqZrVdylabhZRMvUyTSFHomGsdIwpLEmHwrKoyOgYGFYeAkT2ZMgAO1FJO0uh20uNBRQ/jtvXljRxw+Pfw6535hxcV3lbG"#;

const MENU_SQUARE: &'static str = r#"eJx1y8EJgDAQBMBWjm1Ao6B55NKBRYgJXn4SDozdy+FHH/52d9hQ86YkueyiDOdBZ0kqT7wYI6g2xgBqVmLo7BDDsapQYiwzeXG9gU0fcMOvTG+5AcraJVI="#;

const MENU: &'static str = r#"eJxljDEOgDAMA78S+QMQC6EOaX/DgISYy+9pMlStOlm2z7bnfi+pmnFAKjO4Q75mlU0ZWmxzqliwXp5YJvR0JsdOU5/Ee+rsD18qIKQ="#;

const MERGE: &'static str = r#"eJxlizEOgCAMRa/SsBdpLcJQOYEegsTBhcTBeH7LoAt523v/61XvE47VtQwLCAoYrujUfdGv7sTADwU/174IBiF5SqZ9TrwJMI+3xsE8Rox/ewFPZhxV"#;

const MESSAGE_CIRCLE: &'static str =
    r#"eJwBNgDJ/zxwYXRoIGQ9Im0zIDIxIDEuOS01LjdhOC41IDguNSAwIDEgMSAzLjggMy44eiI+PC9wYXRoPmBODLo="#;

const MESSAGE_SQUARE_DASHED: &'static str = r#"eJx1jjsOwjAQRK8ySr8ms3YIlkxuQEuPQrENEgXy+Vk30Gyaad782vvxMTyv0y3jfF/2WZiYqihU1HTa2mk4tvbzkciWI3BxwN3zmKFI1SUqUKL2A0B/AG+QkR4nTBhNFXA1CV68VidSULosQTCD2uU//gWc+UPs"#;

const MESSAGE_SQUARE_PLUS: &'static str = r#"eJxNjLEKgDAMBX/l0b1oYkWE6uzi6i4oKBRxcLB+vQkWKRkuHJf4c742LJ0ZmUD1zGCUMmRlG5pgHdyUabDljVwuwI/pfaGPeh/2Y0WkzlBpEPnjrayF4ltttUqtOuLUCPW2SafV376ztSez"#;

const MESSAGE_SQUARE: &'static str = r#"eJxNjCEOwCAQBL+ywV/a21xTQ9E1tfUkCAQCgeL1gCNjJiPG19gy0uM+KvSKBHFOVKa9dxGD/VsGhVltD2B3wR9rFAbHDxGz"#;

const MESSAGES_SQUARE: &'static str = r#"eJxdjbEOgCAQQ3/lwn7IXU4CCTK7uDq4ER0YHBwM3y8sYkyHNk3zGq50ZzgmtZCATwwMpoqwptmeKCCr7AZJk/atRM6uz6pzGTcVw9BAMXScA5/5tySqRJSM9vuEXJBexAOz9SIC"#;

const MIC_2: &'static str = r#"eJwli00KgCAQha8yzD5zrPwBxxN0CbGgoCCkRd0+tcX73uK9z1/x3mBhPEmB7ZyQIxToqIRV0CCBSgbhbMNMGkhh8H1Vg097TscKmXFCSA8jmdIvo6mffw0f8DsZlw=="#;

const MIC_OFF: &'static str = r#"eJxtj80KwkAMhF8l9L6xSYy7C9uCD+DV+4KCgogHWerbuz9qhZbADCHfQCbcrvczTDR03MGr2cTZytp8DJsCjeERnxc4Dd2BHDoPJMiyt0gMVfo6lA+cTI2VwF9MgfrE0YL9sgy6wpGCR9keNQpIQ43izhlCkRXeg08ys6DlG84ys7+W9KlJvvWkZc839aFCsw=="#;

const MIC: &'static str = r#"eJxNjLEKhDAQRH9l2D5cdnN3QUjyB7YWdgEFBRELCfr3ujaRLWaHeTNhy/uEIVLLAskODlbP3F/x1eMP2/3euXE9pfDRfgp1pQHbItnDPyAb/sIWI5Vd5nXEyZG4IZwSSYRwqFeVR29YsXQBlAcl4Q=="#;

const MICROSCOPE: &'static str = r#"eJxtjbEKwzAQQ39FZD8aycZJwM3cpWuGboYON3Yo+f7YBEIGDzecnsTLv/J3fJ/DO4Gzz8OaHy1a8wUCJGePMFZUJkwYwXqjMbqx01xQifpARVBbgybTljzt8Z5Bn55cSFsoPM3NbXwt9x/cwzU8ABDWOw0="#;

const MICROWAVE: &'static str = r#"eJxtzE0KgCAQhuGrDN8FyujHQLtBh4iUdBciVrdPXWRBy5l3nhFOr57cKdGALokWdFjlTZxrkNF2M16CdaB8MokqgUlkFlc9MmYP40UN+SMvaF+8ISUxM048DCmk1Sv0xMbQ/IQoPuUG7lA0fQ=="#;

const MILESTONE: &'static str = r#"eJxtTKsOgDAM/JVmfkBbRibKNAJ+ALcEMYFAkH4/q2FmOXF3uYc8+S1wre7ACMsWMgHBZPBVKTdfmQryPXsewm7t0yUZbZ+kvRAga+wnrPwHH/jPHto="#;

const MILK_OFF: &'static str = r#"eJxtTssKg0AM/JWwd1OTfYMK/QCvvS+0YEFKD0X075vVZe2hLJlJJslsunf6THDv1RiAp6CG7pKVoat6BF4ItdEjWUkZfYjJgIF2f+g8AyNTnNFZhzGY2qXazRakafQYQGJukDggRb6eTh6oFR++cZtkqaiSTe63bnhp9J87Zd0mh8bDDsf/VjCXXPDw0Gi0zXHazM/XA1bqFSvYWEh4LbztsozmoeELlFtEwg=="#;

const MILK: &'static str = r#"eJxtjsEKwzAIhl9Fek8WxSQKWWEPsGvvgR1y2GGH0eefXdrSQhF+9Fc/LZ/6bfC6D08BajKM5bY4Y9l9BZrJZ9HKwBAs0PmUCcgT6tvymLwKP7Z2gAwYrEkThWpzq2tZS8fa0ez0RA5/sjuQ3QmNgBHYVmSii1ftbqzJsz3Xte9E06Xu0o+btwN+z7Y4xQ=="#;

const MINIMIZE_2: &'static str = r#"eJxlzMEJwCAMBdBVQhaosR7VbXoQRIX2oNv3K0ppCyEh4eXbkmOLIR1UckjX6diQoNTsWrG320Le/rhWD0Y3bz5o1WDCVMWxGKaGfcfo64j/WByH6bb/TIuIZW/B+THt"#;

const MINIMIZE: &'static str = r#"eJxtyqEOgDAMhOFXaeYb6DVZJsY0BotfgqhEkD0/xQBLlhP3iy+f9TI6lrAl0qYVBJp9wl6rhpKnR5T8Oggl404y9pFUkmg/6I82ghIJ0rijDPvoDZPFKW0="#;

const MINUS_CIRCLE: &'static str = r#"eJwlybsJQCEMQNFVQhZ4TyuLmA0cQqKgYCFiodv7qw7cS5KblAjNovoRZGz1dl6ZvveZqu8JgkVnQOlkzjqJF12HEmU="#;

const MINUS_SQUARE: &'static str = r#"eJwdi8ENgDAMA1eJvABq+fSRZAOGQLQi/aEqErA9NB/b8ul4tMPpFaygJ3L8lUF3r26CVEDW+mkeW3mZgvK1u1EVbIVStgDz0g+Nfxat"#;

const MINUS: &'static str = r#"eJyzKUgsyVBIsVXyNVUwNMowNFGys9EHidkBAGN3B1U="#;

const MONITOR_CHECK: &'static str = r#"eJxNjEkKgDAQBL/SzF10oqBCkh/4CFFxPAgSgsvvTRSD9KEPRZXeei8YDa0tuIAKq7KKrM4jsNpNg8exjF4MqYJwhiO49y5DJUGmZRZviB8tClZ/1Y4VuN5/wUQaKJYmgRsa9yYH"#;

const MONITOR_DOT: &'static str = r#"eJxtjLsKgDAQBH9luV70zjck1ja29hKFCBYiIvr3RgVNIdvsMDDKjIuZBphDU0ZYNMUEs2vikioVPrZSc7da9JoaEbBscScQRG4cuFcnPgfSph/Dsb1bV8MrsSvlW/JjCgjb4hUnbgMqHg=="#;

const MONITOR_DOWN: &'static str = r#"eJxljT0KgDAUg68S3i76WkWH1hu4uouKz0EQKf7cXq1oB8kS8pHEzI0TdJYqVmBd51Sa+M5K85KJM3ASaejoUuBL3zosuyVF2MbOyeUSwhNIPw7iLHFKOCz52l0Is/4wX9P/YVVAsRQfOAGRzS2R"#;

const MONITOR_OFF: &'static str = r#"eJxtjDEOgCAMRa/SuKO0oQETZGZxZSc6sJg4GM9vu7BI2g7N++/Huz4Nzm3a0QP67CoBgZVBQ4YKH9bgzPrpTSkuaqTYPRLAhbtn1cvrIBmAsIUBQKnwr/uTS0vJynb2ATplLWw="#;

const MONITOR_PAUSE: &'static str = r#"eJxtjT0OQEAUhK8ymQvw1iYUu26g1Qvi6UQ2fm7PKhDRzjffjJuaoOg8K0khWZ2zdEnMSvcQ+yFz3wZsnobYPTNC+3HQ4CmWWMcu6MlSYr46pxaF96CB5Iv9uSpgRIsbHMcKLIs="#;

const MONITOR_PLAY: &'static str = r#"eJxFjDsKgDAQRK8ybC+6UdEiyQ28gJ2ouBaChODn9iaIsRgY5jFP74MXTIY2LtCgRpmF9GR1HonVbh493GVIEc518hJaQXgHmddFvCGuCLehMt7iwepP27ECN0f1CxNpoVjaBB5N+CZj"#;

const MONITOR_SMARTPHONE: &'static str = r#"eJxtTL0OgjAQfpUvtxfbQ0tJ2s4uru5EiMdmSIP69lwnGMgN9/3Hz1AEY6KHCwhPPzAYtp5hw/frkYPXbuf6WQLleKkTOe5DFq5fTdv0Hm3jbieRThNyMJbpVbD8EjFBpvktJZGzBFWcJ3znsUgiRX8VuPZqI28fJjBk"#;

const MONITOR_SPEAKER: &'static str = r#"eJxNTcsKAyEM/JWQe7cq6m5BPe+lH7G40gg9FJE+/r4JhboEMpNMJhMeWyfYI17d5MCodcEUzrJM4S/pGS40KT2kVnKHT0SL0N4RDQJ3zfCqeyemCoFKvVFn7sUnjsPLBfxqNwMGlNSJ2dOPmdGQHXm5tnwvkCVlRsgcrR1nM8jRT05fUDI2zA=="#;

const MONITOR_STOP: &'static str = r#"eJxNi1kKgCAURbfyuBsoLRpA20GLiJSefyHSsPvSyPo9g/J2DnRqtKDdmcAaDejQ6EFs3cIhgkEVsRtUqm8rcy1LxL0C+Ye/l6i/bZ0Ck9EYhSTRbslE9jMdScFdFhcL7ytu"#;

const MONITOR_UP: &'static str = r#"eJxtjbsKg0AURH9lmD7EuytoYNc/SJs+qHgtBJHFx9+7CiqITDHF4cy4/h8UlWf3gSSwL4sYFu69gcId+CsGYn/ZRYa6DNC6bTR4SkosnpaY2iqop0mIORYx7BW1TbgNZmP6cJXDiOYnWAFZXC1K"#;

const MONITOR_X: &'static str = r#"eJxlzUsKg0AQBNCrFLWPSU9GYmDGG3iIEMV2IYgMfm7vKKKI9KIXj6py3S8oSs9WbJJCTJI+4jF3z1Vyd/h3Z1y4r/4BY1MG9TQvYoqPmD3fhFZNrcFTLNFvEGNr4GwtxEA+g73vFRmMaHbAAjgwLo4="#;

const MONITOR: &'static str = r#"eJxNi0EKgCAURK/ymQvUt6gW6mVKUogWImS376sIrYaZeU9HtyfKBgoUW7wGE+gJR/LSR5B34fTJgGdYPRTB6ivcjjIbbCIo4RiUJXmRzrULW6jOllO1k9efxHXv8Af7biWr"#;

const MOON_STAR: &'static str = r#"eJxtizEKwCAQBL+y2B/xNBEOLv4gH0gnpLBMIb5fFMFGppwZ/VPJ+G7zsINPAQF2IBMLBpOQvCbq0euo6xH4em6EY1yZlmkXJhpl"#;

const MOON: &'static str =
    r#"eJwBNADL/zxwYXRoIGQ9Ik0xMiAzYTYgNiAwIDAgMCA5IDkgOSA5IDAgMSAxLTktOVoiPjwvcGF0aD481Avf"#;

const MORE_HORIZONTAL: &'static str =
    r#"eJx1jLEJACAMBFcJWUAULISYZYKFYJVKtzcQBAutrrj7J+kqo4HMijEhyHKqAZmCa6aT3dpH5ZWZyN+zDY6gIPg="#;

const MORE_VERTICAL: &'static str =
    r#"eJx1jLEJACAMBFcJWUAULISYZYKFYJVKt1cMiIVWz3PHkVSVVkBGRh8QdA2C9P2YnGGmS4uHm/22fPrGJpD1IPg="#;

const MOUNTAIN_SNOW: &'static str = r#"eJw9zLEOgzAQA9Bfsdhxm8sduUgpc4f2I6oysCAxMPH1HANslv3ktv62GdOrWxwZCof1BkOyt3yi2ruxPU4ztkt+lUkD8Ol/4SB9ohUYRSNpRqEPVIGwBGPV2LRC4JTMVO/HAy6pHFg="#;

const MOUNTAIN: &'static str =
    r#"eJyzKUgsyVBIsVXKtVAwVjBRsFAw1TVVMFUwNPUw8gEKVSnZ2eiD1NgBAOC1CoQ="#;

const MOUSE_POINTER_2: &'static str =
    r#"eJwBMgDN/zxwYXRoIGQ9Im00IDQgNy4wNyAxNyAyLjUxLTcuMzlMMjEgMTEuMDd6Ij48L3BhdGg+MmsMEA=="#;

const MOUSE_POINTER_CLICK: &'static str = r#"eJxtTjEOwzAI/ArKbhqwASO5eUHziEodOjRSh059fbEtZepyoLvjuPa+f57wuC6Hg4MAMRCalSTIrDcmoBKCf5etXbp3a+cFKa4WhjkKcskT/3gNqVZg5OwRb7FVt12QsoKhq7xSp2rq6k4ZXSJrHTQxw8AjWqkYdNQhTP789wOqoS9m"#;

const MOUSE_POINTER_SQUARE_DASHED: &'static str = r#"eJx1TjkOg0AM/MqIfklmdkmEtOEF4QN0SBTbIFEg3o9pgMLILqw5nZdxLZh+Vd8gjoLwPibYVXX5dbBdPjVsbyLCFc0UbBNokvobUh3/MugzOIENxCswKHitVlro4mJx30wPjoh283DRCLkGpptjB6mwUE4="#;

const MOUSE_POINTER_SQUARE: &'static str = r#"eJxVjDEKgDAUQ68Surf2x1qX2tlBVwe3gkMXwUE8v+2gIgmEhPDCkc6MbVAzBSJLlwjCVmlqjr8OXuK+oSSzVzE0FRLDg9qFKHYQCzG9dqadWCa/vt8bV2sbew=="#;

const MOUSE_POINTER: &'static str = r#"eJw9i0EKgDAMBL+y9N5oDCYUYl/gJwQPXgQPnvr6NpfCsrDsjH/X/+A+0isQGK0GViqGjXbORlIQlWOeA2mp+hJO9WmyYESh8+t7GRUe"#;

const MOUSE: &'static str = r#"eJwVi8kNgCAQRVv5+Q2oBPXC0IFFGCEON0MmLt0L17eEmg/DK5wJzeVUE7qReEoyFU6e+BohamtWxjD0IYZrN0USbpPDcvsuOoo/nhQWsA=="#;

const MOVE_3_D: &'static str = r#"eJxtyTEKgDAQRNGrDOkXGZcMCmtu4CEEizSChXh+TSFYhN/9F+d2VexLWjP8piqVSgztlvjsyOAMWY9GCG6Ot45yAtXM/v4Acckfcw=="#;

const MOVE_DIAGONAL_2: &'static str = r#"eJxlzEEKgDAMBMCvLPmAROlBiP2Nh0JpC3pofm8b60F6SXZhWCk5agzpRMkh3ddBDsxweB95WT7hZbK8gzf0O8Lfm61rd4TKbZugo6rVxo0+yA0mmA=="#;

const MOVE_DIAGONAL: &'static str = r#"eJxlysEJwCAQRNFWlm0gjOJBULvJQRAVkoN2n2UlgeDlDwwv9FZmyfWk3nK9r8iw5Ah+BeAUjteksGt8VmL/WuWAKM80TGTHNLHG6Ctc6QO/uSaY"#;

const MOVE_DOWN_LEFT: &'static str =
    r#"eJxFyiEKACAMRuGrDC8gP7IwmMsGrXbBYDR4f9Si9X1PZ1uDenQFIEjiiuBM/c2mH4U48xmebcWDD4w="#;

const MOVE_DOWN_RIGHT: &'static str =
    r#"eJw9yiEOACAIQNGrMC7gGCOwIdmg1e5mMBq8/xyF+t+3u96BXXGQAvEkbcToVqK7pQpIj0PTPtMbD8M="#;

const MOVE_DOWN: &'static str =
    r#"eJw9yqENACAMBdFVGhYg/YIgSicoFk+CQCLYP1BTd8k7OfNuWi31SlyNQYBx+Z1UsqNKLK4DCHnwMxA+"#;

const MOVE_HORIZONTAL: &'static str = r#"eJxlzDEKwCAMBdCrhFygJIM4RG/TQRAV2qG5fZOAkxD45PP4smbX3sYNa7bxPgUpQwZmILsMlLDKtVGVgyfXjtNhwynZJCN8XJA97bdQjtp82B+89iaG"#;

const MOVE_LEFT: &'static str =
    r#"eJw9ySEOACAIBdCrMC/g+IEZkGzAQ7gZjAbvP0chP73rHdq9TKHmIIYLsRTTGmKaHjaAlA/Icg+Y"#;

const MOVE_RIGHT: &'static str =
    r#"eJw9yqENACAMBdFVGhYg/YIgSjWiDEGCQCLYP1BTd8k7OfNuWi0NrlQNIIb95JJUsqNKLI4dCHnvjhAw"#;

const MOVE_UP_LEFT: &'static str =
    r#"eJxNySEKACAMBdCrjF1AflgYzGWDVrtgMBq8P2KR1fdsj7NoZm5CQJcCsFt66hZOKpSg/y64nA9V"#;

const MOVE_UP_RIGHT: &'static str =
    r#"eJxFySEKACAMBdCrjF1APrIwmMsGrXbBYDR4f8Si9T1bfU8aiSsiSYY2gN3CZbefSlKEoO8OxaYPjA=="#;

const MOVE_UP: &'static str =
    r#"eJw9ySEOACAIBdCrMC/g+IEZkBNgtbsZjAbvP0chP73rHdq9jEbiDIKzkBTTGmKaHjaBlA/JSg+m"#;

const MOVE_VERTICAL: &'static str = r#"eJxtykEKwCAMBMCvhHygZA/Sg/E3PQiiQnvQ3zeG9lAoBLK7TOytzJLrQb3lep3KO4kdCCAJljnF7UUp/vDgeuHwte4GlAVMU5TtDXmqzcDybm+9GSaG"#;

const MOVE: &'static str = r#"eJyFzjEKwCAMBdCrBC9QEnAIqLfpIIgK7aC3byIWpB2cPoGX5LtaUk8xn1BLzPfljQUGAiSwgNYEd7wiuJ9lRaTcws4KQR6a5APyjrP2GEV07XN90IbekIFGEpJdRtSkkeIX26ddDM3dxT7KCk0L"#;

const MUSIC_2: &'static str = r#"eJwBRAC7/zxjaXJjbGUgY3g9IjgiIHI9IjQiIGN5PSIxOCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+nsgTIw=="#;

const MUSIC_3: &'static str = r#"eJwBQQC+/zxjaXJjbGUgcj0iNCIgY3g9IjEyIiBjeT0iMTgiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+YagSWw=="#;

const MUSIC_4: &'static str = r#"eJxFy0EKgCAQQNGrDLOPmCJRcLxB2/YxBQUGIRF1+9QK1+9/u4/HAhNjb4D00HlqquakFp2tEzn7B5uBmEQtJGsQP4NcjApBbkbSCIEx76+WKrH6OD8xLtkDVismgw=="#;

const MUSIC: &'static str = r#"eJxVyjEKgDAMBdCrhL+LpEWp0PQGru4SBYUOUkT09lqKiPN7fhv3hSZB3xG7oYlsKnOwRfB1puB1TRpn0lPQgpLAgvQSsMun6G89UMLbv3YDvWAfIA=="#;

const NAVIGATION_2_OFF: &'static str = r#"eJxVy0EKgCAUhOGrDO413tPQQL1BhwgKCiRatMjbpxZBm/k33/hjOlfMQYyD0oQ2PZiSlQYWRpIiK3UZEX1XcfTfhYzqNZxyDsTg1DD+OG37gkxBsMD1hktK89NCK4o3FwchoA=="#;

const NAVIGATION_2: &'static str =
    r#"eJwtyrEJACAMBMBVHheQBEQEdR0RJAlo4/ZapLviqum6QwWmU85ugRgMKmDCJ2UkJ4deo+/+ALgSDuw="#;

const NAVIGATION_OFF: &'static str = r#"eJxFi1EKgCAQRK+y+N/GrpkK6g06RFBQINFHH3b7VoliYWZneBPO+dpgiWpyOGhoooEoO2A5UTS2M1KrFPoKp/BNyKL2AqPVwILnztd3wJF+Ou/HCoWjYlZwk7jYG0uLglYoPSzmIcE="#;

const NAVIGATION: &'static str =
    r#"eJwtisEJACAMxFY5XEBav9p1RJC2oB+314K/JKS6zdNN4TZ0r5YKiMAMBhUwhT2ImqTmf8sFtm4O3w=="#;

const NETWORK: &'static str = r#"eJx1jc0Kg0AMhF8l5C7tRCot7PoGvfYuVRpvRRZ/3t7NCv6AkkOGmfkS1zXfQJNnFExDWwf1HJU27U9DkuOSdba5dDcDSpewaMmhux2YrqnFM/iVenL+eIX+VVCqPb8fhKLP8goEuseJO4NC9gahzw01aIdCCPJ5rskMWA9DAw=="#;

const NEWSPAPER: &'static str = r#"eJxtja0OhTAMhV+lwZe7noylJLtoDBaBIyBqSBBkzw9DwASp6Xd+cuI+H0brvxo8ASZhBoHcfWCM/mW+uNeSCakoSBam7XpLiZG4XRxLLXWbEwxD1cVfHu7iMy9K4o31y2pI1Lj5shwF05SLY5iewAllezQR"#;

const NFC: &'static str = r#"eJxtjUsKgDAMRK8SPMDYpDVVqL2Bhyi4cOnC+2M/tKsS8sjAPBLe9D10n8ultMNK8nCWKkwezuthdYlhLc0YRv+AU1IIJ2Z4pcYu5bTtE4sFB5NDtrZyNTYLpuapprCeJIkBKzX2V2KG8ANXli9x"#;

const NUT_OFF: &'static str = r#"eJxtUUFOAzEM/Eq095jYseNYavsCuHKvChJIBXFAiP6e8S6qeqiijR17djyZ7L6O32/lZb88sRR9luWwe8jSYXdtWOH2o0en1rRse1uXkbiXQT79RMpC3JRmE5KQwjgJWcgjiEXONIRrbifqlTqNqKRzbriawJzAZd02fiUWq0LR444sjtTVqWu/14UCOVVGG4PMO9Johr215MSHsg4UJYuaSrr1bJW1whNyVZHqyPPEH2tDJjkjGUEBUC9css1XGjcHTevg6YU4tJL7oJlcZon2AED4g8FiCiMjYOGcDk/MARAMn5FQs5wOi3NIhxk8kI3Z0zlgEdGX8BzfV6zNdTzUwum8qGLNmjLwdOuFgB2JYoO+m0c/v3++lgvvF1nK738QBMTLFgFN0OEPsoF5kw=="#;

const NUT: &'static str = r#"eJxtUD9PRDEI/yrkdrBQoCV5XmJcdXVwuzwHBwcHc59f6EvedGlCaeH3B7bf2983fD1f3llAP+Ry3Z7q67qdBQNud70Nak3hiG0dIxkDnMYcOykLcVOaTUhCgPMlZCFvSSzyQy6MFXbqSJ08kHTOow+r8aW4OxzxUOAA1jvqA1dl93WCgpKB5z13JNFOMZCigBQmKNAz6yndZpmKkWIS0JGBTCmEYhW6OaTRzDRh5DoWgyJTiwk5SnKlTaMIK9fG1SujSgvXj/kymy4FsxPmEqlcjOxLQvoJk5abMK1N+EzrnoxYhdbTG/v670tx1DDJLDlj1T3jsImSOQrmXlMNNc9E/TyX9g9MgGML"#;

const OCTAGON: &'static str = r#"eJxFzE0KABAUBOCrvByAvIQFriMlT7Fxe/KT1dQ307hKeUQqUCmV3jwz3GpAkJpLtRIRjjy6xee/3XKYBSfuc5i5PBcj"#;

const OPTION: &'static str =
    r#"eJw9ySEOACAIAMCvMD7gGA4NyA98hJuBYDD4/6mFfLrHcZgNOwO7LAGqLmiaPpgGU35eAi7D/Q/j"#;

const ORBIT: &'static str = r#"eJxtjEEKg0AMRa8Ssjf1jzOlgRlv0EPItNBCF0Vc6O11jItBJNmE915i/o7596Y8J4ZjGhN3THnZrz7eDPfx0DbsDAe2Ri+sAsLxRdmiyvoP04deiZ9oxZOD6ICWtrVRUY8GQTzuJSt6HXUSyAnqplF5eNCpWQHy3zj/"#;

const OUTDENT: &'static str = r#"eJxVzUEKgCAQBdCrfOYCMQbmQr1NC0FUqIXevjStXA1/ePNHp+iLd2FHii6ch6ENCitYYANLsnoZwurmMhsSTCjCEAtCrrNmbvk++OG6lI+VH+0VMx29U5/qf9SLL/NyLq0="#;

const PACKAGE_2: &'static str = r#"eJxljLsKhTAQRH9lsM9ed5NcXYiCnYW2FnaCRRrBQvx+Y+EDZFmYYeZMWKctYq6y3kIjlzvnk0CQp2OTVOvf3sigY1aH34nV4YKXBEPIeeNIm7uPgsTBRiUvzwqYyvTcCUO/Wz0L7P6/gwMIIyZR"#;

const PACKAGE_CHECK: &'static str = r#"eJxtjc0KwkAMhF9l2PuuTbo1FrYFH8Cr94KChf6BRdq3N9uq7EESGJh8MwlTMz9wq0xPR+iyjrfe1OEQL3X43i9MoOx6aiKRxbFkyUneWbE+cVXVgj//LOQ4vZIgtCkGFUrMmGNtpD/Pe3EFvGNBicJRkRBjt3btcMc0tsP8rEzuuISAOC5nTgiy4R+wDhu+UmWYDRZVUl1512VXDWzwG7ulRUc="#;

const PACKAGE_MINUS: &'static str = r#"eJxljdEKgzAMRX/l0vd2JlYzoQr7gL3uXdhAwakwGfPvl+qQwmjhwsm5SZjbpcO9NlcqQWVXmiacImvCMWECZbdzy2Bk8Vmy5CQfrFifUE1F8JcDIcf5nRShm2JRpQTGHutG8v/Hn+IKeMeCCoWjIjGmYR368YF56sflVZvccQUBcfycOSHIpv/EJmz6SrVhNvhoUkzec91TC5v8BbsKRGw="#;

const PACKAGE_OPEN: &'static str = r#"eJxtkMtqw0AMRX9FzH7UkeR5GJxA9+m2i+4MLbTgli5CSP4+V3Ye4IQBDTNXVzrS8D/uv+lzE940cS/UuHUIuZCy2ijcG80h+YnCTSjtjIU6FhuVRWkOi84pk3HpJ4FdqcA5ImFRvVAHtwpBzvLE3Ue460fYDi8Oth2ueL8GcaGD13IsXGSFh7Iz3oQa2b9XDYSA7XS7xhW+jmu+0Um80NlTOHgf4aafvy866iaIBjr5bYGOcnnjVvVkT7tPgkWTGOeDsVV0Sb5qhCuEzGPYFAtYU1vPiIzaMGMs0eXXVQHCfgq3+u4tbqRnSRZmZw=="#;

const PACKAGE_PLUS: &'static str = r#"eJxtjdEKgyAUhl/lx3td52i5QIM9wG53H2xQ0CpYRL39VGITNhR+/M73H93cLh3uXlypAlVdJRp3iqxx30kN0uu/CROouJ1bBqOIR5IkZfUgrTQZDRkQzOWDoHFesyLCplgMUgZjj8NGMr+fP60qYRRb1CgVlZkxDfvQjw/MUz8uLy+04hoWxPFyoSzBJv0QG5f0jb0gFtjJC455vDdKGQpJfgNAuEvV"#;

const PACKAGE_SEARCH: &'static str = r#"eJxNjlEKgzAQRK8y5N/UXQ1RUKEH6G//xQoVUhUrRW/fjVYbErLL5M3uFGM9P/Eo1Y0JFN+zmsGI/YkoIm0TF9koDVSpIiG9nhISZJ/ACJnkjQIFovexTKRUVcXFr62KY/nLaoNUs0UOo8kExOBW1/UtxqHr53epEs05LIj95Vhbgt3wH1gVG75QqYgVFt7relTRmb1hh5tualyLZpVvo41Cs0iX+W4SUm9Rdugf9yZ7JStZ/7Ikyc/AX9WSUkM="#;

const PACKAGE_X: &'static str = r#"eJxNjl0KwjAQhK8y5D2xu02IhaTgAXz1vaBgoX9gkfb2bmqtIYFlh29mJ0zN/MQ9qisTqLidGwajSE+TJuPLTnttM1WmSLCXQ0KJ8zszQpKSUaBMTD6WRLKqDqd0tg6/4703DtawRwVnyGXE2K1dOzwwje0wv6IqDVfwIE6fC+MJfsN3sA4bvnBUxAorRcVp7vtC2xTDFz4akGSWcHC9dtLW6X+JD4wYRmw="#;

const PACKAGE: &'static str = r#"eJxtTjsKwzAMvcoju9RKtisX3EAP0LVDt0CHDg10KDl/5ECCIUEgwfvpld/w/+B960bjhMhquCKxpK4vp8r1ZVU8VJAHheJch4SELXzJKDaoX4cQ7xuEgDw1RggWo4sasPo8qbH5O7m89j3GwAGGzIZUNx11Fc/Rp+hGzffYNWU="#;

const PAINT_BUCKET: &'static str = r#"eJxtTjEOwkAM+4rV/UKS3sEhHX0BfICtKkMHkBj4v3BQRYdWkTPYjp32Hj8zHpfuZWeYpRojRxCjw6HLuNRnEQcxSZVKKRah19/lvRvaIcKG9o8sdBWUrXJzWD/brsJWXaoNljJ00mR8yeSUXDKcnPQIirYgkNf2Lx1cMMo="#;

const PAINTBRUSH_2: &'static str = r#"eJxtjTEKgDAUQ6/ycbc2aRUK1dnF1cFNdOjoIP/8VpFOJWR6IS9e+53kHJsFXhBMWDEkt1Mo9gtbasu5Vx5WYGCCvJDJqTPhHyLXi92aKXbv4RTL7SDgygQqbAVnK9XXQFYrC3gAjIcrKQ=="#;

const PAINTBRUSH: &'static str = r#"eJxtjjEOwzAIRa/ylR0asB1syc0J0gt0s9Khg4cOvb+KoypThPj6Eo8P9dO+b7zu00MyB4PyEiAR1kk4lUOaQjGPIuXsdss+Lyi4QP7QJgaZe/RMGtKUxZkhM8ShQOE5rfU27q/1/KIg76QIFBE4kSH27EG7kmCh5G0XWxI5QczlMOlEfiwvMfM="#;

const PALETTE: &'static str = r#"eJx1kMFOwzAMhl/Fyr0mdmI3kdpeduYhpoA0JA5ocBg8PXbSCSZtUpt8rb/8tbu0t3N7f4XzGlACtO81aIfLGigZbcvTULZlV92h6NL1kLvzQ3e+OuXvzB3zsn/5XyfEt+rH8esEL2t4JgY+mA22w9iJP7MBxXE1rKxghJrLhHPWHW0tBeKEOc0TktVKkvFESGzI1e5S/Z1VVXhQrx49pCdZtqU7ag/VciKsVVvCKGQ1QRGZbIKYdvY1H9g0Fes62hT227JSbx/4x2f1GbdfHGFe7g=="#;

const PALMTREE: &'static str = r#"eJxtUDEOwjAM/IrFblM7dtNKpRIPYOIFKAwZOjAw8XrOXWCorFwcy3c5e3k93p2el9NNC01tYJM6Anzk4BCcu1GIORlN3TZlJUT307qck7su/wpV1K8hU3IAAwLdowSNrcjgeEOULAUzic5lY+VdFvmBKoRmmqVqgy1NrgayAr5XlhL41MvmkGRACIoV7UDNZkuaNVaZIwGDiVVkE4MIofgczQI/MNh2s8GilRy3Embp3mzfTRYsvf+28QWkH0sY"#;

const PANEL_BOTTOM_CLOSE: &'static str = r#"eJwljNEKgCAMRX9l7D1iiuCD+i+RkkJFiJD+fXMx2Ni5Z3M17Q26R40wpFdeFEJO5cjNI1lGQ9BbYstCglvnXXBnuRMMYmgQumKNeNL/Tglmd1rBPVvLED1eZMAuGvTCNeMZhA/7iCRi"#;

const PANEL_BOTTOM_INACTIVE: &'static str = r#"eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KQN1e05OCXj4hj/99zYtAPQIahDstwtlyL0k4II0InMvK0u7oOy1Ev83CkAJO1AMNTCr6e4pTMR9i/8C9ty6tCix4"#;

const PANEL_BOTTOM_OPEN: &'static str = r#"eJwli90KgCAMhV9l7D5qSlCgvkukpFARIqRv32YMdjg/n8lhL/AmX6JFWhCqRY3Q+s9sFEvrEkM6YukrZ0bhnDnTHaARhzND6tdKna5sFclWVs48W4ngLV4r0AR60MAntRTuAwVKJEQ="#;

const PANEL_BOTTOM: &'static str = r#"eJwti0EKgDAMBL8S8gFJRfDQ9jNabEA8lIDp701sT8vOzsZWDoFa+KqSkHaE1hMGhJdPqYNowtW4/rx7yXHxX443PwU02EI2kfmbZRipNF238gcqUhxC"#;

const PANEL_LEFT_CLOSE: &'static str = r#"eJxFi0EKgCAQRa8yzD7CpChwvEGHiJTGRRAylN4+LajF/4v3/jfRrwJXcMKEakSImbBDYB82lhclQo2Qn46pamva+rPmWITBEc4T6LNsi6joF7saQPWNbjSUfP4G334gyg=="#;

const PANEL_LEFT_INACTIVE: &'static str = r#"eJx1jEEKgDAMBL8S8gFp68FC0x/4CLHF9CYlVP29Bi8KelmWGXZDzbMA57KwEJoBYStJ+K51J7QIVzqEQzOGTgcxrJMwJMLRg+mbUaHoLXyzn8L9cP84OgE4gSv8"#;

const PANEL_LEFT_OPEN: &'static str = r#"eJxFyzEKgDAMheGrPLKL1DootL2BhxBbbAdBSlB7e1MFHZLh/xKTw8LIl6WOEENaI1tSA0GKJpRnn8lzfHMu9dKZtv45s88c4S1NI/QhLlDTD5vqIQbdyHx8A7JYIIE="#;

const PANEL_LEFT: &'static str = r#"eJwdi0EKwCAMBL8S8oESe2lB/UwrVSg9iFDzezc5DezOxF6uQZp4Z6qlPXUkloOpYwrAdEz//3aP6neOm3U5vu0rpAGSwAJPQBwqFsE0Jy/h3Bvo"#;

const PANEL_RIGHT_CLOSE: &'static str = r#"eJwti9EKgCAMRX/lsvcIlaBA/ZcoSaEiQkj/vq18GGfbPdfeYckojgzhZmhG/fCkNUdHaiTEkLaY/72K6m0vPW/3dAYUzdHAEVMrQlHtVs0Vy9trzhGro2PEBAPT8Ugqf/8C31kkGw=="#;

const PANEL_RIGHT_INACTIVE: &'static str = r#"eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KqLq9Vi8iegt5/3+b4ySwOdQIawrCDqlH4JhmlvveL8xnRqG3TS14u4zCEBwO1AG1harU30tMUd+i/8A8tg7F9Syw"#;

const PANEL_RIGHT_OPEN: &'static str = r#"eJwli9EKgCAMRX9l7D1qitCD+i+RkkJFiJD+fXM9jMvhnNkS9wpvDjU5pBWhOdQIhUch9B+6QIr5SFUqb+fx5+2Z7whdsSeuSfLGSIaXZLkdlbfPVhMEhxctQGbSkwa+4YfxHys6JIs="#;

const PANEL_RIGHT: &'static str = r#"eJwlzEEKgDAMBMCvhHxAUhE8tP2MFlsQDyVg8nuTeJqw7CbPdjBowRVhSsFkaCCRvePkXpB2hN7G1Tnumhff1XyPp4GQhZst0q+aiUzyF9b1Vv0ACaQcQg=="#;

const PANEL_TOP_CLOSE: &'static str = r#"eJwti0EKhTAMRK8yZP/5pAVRaHsX0WILKlIK2tubqGQRZt4bV+JUUS5PhnDmuSZP3BOksIQU85Lq25T2OE1BcH/dBbfmPaIZT4NM5BkWg9/In6lOcMdYE2ZP2wDuYH8WcooVhBvPsSPw"#;

const PANEL_TOP_INACTIVE: &'static str = r#"eJx1jEEKgDAMBL8S8gFp68FC0x/4CLHF9CYlYP29Bi8K9rY7w26oeRXgXDYWQjMhHCUJP/EkdAi1EVqEpiWGQQcx7IswJMLZjODZqFD0Fv4W9ke4Dvefowss2ivE"#;

const PANEL_TOP_OPEN: &'static str = r#"eJwljNEKgCAMRX9l7D1iWlCg/kukpFARIqR/32bsYWzn3Gty2Au8yZdokRaEZlEjZF4KofYjhnTE8uNcBTgzSs6ZM90BGllcWaZuV8UGcZGSL5viOPNsJYK3eNEMNA0a9MAjXIj7AO7FJDU="#;

const PANEL_TOP: &'static str = r#"eJwly0EKgDAMBMCvhP2ApF4Umn5Giy2Ih1Kw+b1JPW2yzMaWj05NBQFUcr1KF/AGeuvZy38OwQpqYxr1J8XFdyne9cmkQbAbswhsydMre2vSTfoA8ZEb6A=="#;

const PAPERCLIP: &'static str = r#"eJw1jbEKgDAQQ38luPf06rVeoRb8lIKDQwUH/x+9ogTekJAkX/U+sK/D6ZlEwExTcIk4wVAjIqZX7JQkdTSlsDjDJhDLTAollWZ+goXVw39VTzp3tH9Eh5JHuy4PCMYboQ=="#;

const PARENTHESES: &'static str =
    r#"eJw9ykEKQBEQBuCr/NlPL28mUcMNHEJZWCruH0oW3+7TXmZDjSZ7/HaQEG8Bl0n6nZH0PevAQ8AQBLpeWxmZE54="#;

const PARKING_CIRCLE_OFF: &'static str = r#"eJxtjEsKgDAQQ68Sulc7HT8I1bUbD1GqoKAgRYre3lYX3QgzhOSFaLs6u82wVydICbggUsDer+118fFeH+ZcMHVir1CBynCRxjSxkRjEhsGQoPAyq4fWq59mC2p8pnJOMw9LGSUj"#;

const PARKING_CIRCLE: &'static str = r#"eJwlijEKgDAQBL+y3AdMjBgCydU2tvbhFCJYSLDQ33tRlmVZZqLsVY4N8iSyPaHqGILc3+XY/Zzjma+CNdEcYP3iy5AdHIzGascpNLlJ/AK9ChXz"#;

const PARKING_METER: &'static str = r#"eJx1jrEOwjAQQ3/F6n5HLpdeEil0YeYjKhgyIoH4fi4dUIdWlrz42XJ7rZ+O53W6V9RVoQgQlyFMS7uMdGl/RiIkfvUo8dLc41EyhjPyNhxIlA2R9WacIMmtwJFhvbyFjdIGGGXOD1biwomEZ3evnZyqu1M/5nA0HQ=="#;

const PARKING_SQUARE_OFF: &'static str = r#"eJxtjrsKwzAMRX/lkt2qJeVRgxvo1qVrd0MHL4EMwd8fOUMSSBACIY6ObpzTkvF/NV+lHtZvgcBbMTpo5jYdC5vKeeGoC2BquRnjo4rGeOjwpOHHYad9Pc/sSa/0VCExwt+YWMGaFLplMJHrP6HIDWlhhuLk9GEFj5w16w=="#;

const PARKING_SQUARE: &'static str = r#"eJwdi0EKgCAURK8y/AuUGZmgrtu0bS8pfXchQnX7VIZh4PHG5HgWfJYkgWO6uFgSK+HtJNeZCE8KhTt3ZmgHZ25fGMHSriHUoXj2EhJjjahdNt3UJrkfQn0aOw=="#;

const PARTY_POPPER: &'static str = r#"eJx1kb1uwzAMhF+FyK6rSOrPgJulS4d27dAtcAcPLdChyPP3qACZHNiSaR8/Hk2tv5e/Xb6eT+8VQ1ThYmL2rRk9OfpyOq9PkXNe75lFfEfWA8VMxgNJq9hjyvKx9hNaMlhBrxfDIrFyXEmxNHGoXfMGxWgJtYuiOaVSZ7Qn+NgSxQk0NO69USvlTYtoPvZUJ2OEfcIMCsHBRsKWg4IqbRMfnBM6CVhoxWf0qv2gsCqHy5ostLHiLBzkCJJilL7mlwXVpPA/F6kRLtJg/tGPpkrIN3Jhy80wnKh2GlV2TivnMWb2yFXTbN9vW+SmyOXXKvEScCTzFh7X593wH2lof2M="#;

const PAUSE_CIRCLE: &'static str = r#"eJw9zDEKwDAIheGriBdoE9qhYHIZyRAIHTLp7Ru17fQj70PiPnk0mAXTjsC6mlfFW2mLvdLodwPNBS8EyaElRdV6mjb1WkfHPwa223989gGcMiFD"#;

const PAUSE_OCTAGON: &'static str = r#"eJx1jCsOgDAQRK+y6QEKndAPyVKNKLaeBFGJIAhOD3QJKOR7LzO8zluhZVCTacnY3KvIze0if6X7K177K6IEbT0SQFXsgsZpBEfAWG0CicmCz/Z4b0/5JSL5"#;

const PAUSE: &'static str = r#"eJxdzEsKACEMA9CrlFxgEAZXrZeZEetWCurt/YAbd4G8hEv8jLrgBdX8m+6kMSc1gfOgJvAI/CwYePOrdnNxPRw/ADIvG1Q="#;

const PAW_PRINT: &'static str = r#"eJxtjUEKQjEMRK8yZG9talIrtAUP4MKtu1IFBRfyEdHb29qN4icMEzIvTKyXqV5PmBI5Qn0mYm7+SiSU43KkOf5TYVBhhuq5syNnT+PrC7uV+xnHRLsN2BaFwrbh5vpYGS1N6PpcF94EARsruvdGHXhtJECMeHDPtj84tG1sD72v9+Q3qA83+g=="#;

const PC_CASE: &'static str = r#"eJxti0EKgCAQRa8yzAHKCRUC9QYdIlIadyED1e1zdi7a/P95jx9aOQTumoUjkkXgUk+WiItBaE9vhJ4O4dWdwqyHFK5dGHLEjRyQ5cmQOqWDW8Gz/+VkBvEBvRUlTQ=="#;

const PEN_LINE: &'static str = r#"eJxFizsKgDAQRK8ypE/M7iZKYM0J9AJ2AYsUFhbeH7ONMjAwn6d3ezrO1e3E4NiLqzpZV/Vf5pAhITcOdjKLQwSBbAuoXD6NRD4dH/0C6QYVeg=="#;

const PEN_SQUARE: &'static str = r#"eJxVTKsOgDAM/JVmvoN2XUCMaQT8AG4JYgKBIPt+WsMjTXqP3F06y1Vhn9xKBDJLYWDo7VBZo4+hyPVvIDccXE6dreT0bo0+AvtY2JPG7VmeIEBYVFA8UFQSyvbUb5EQH1Y="#;

const PEN_TOOL: &'static str = r#"eJxty8EKAjEMBNBfGXJPJY2xFdp+gT8hVVCoIIsH9euNu4e97GVgmDfleX7dcKn0kAg5InGCQj0TK+uXWtn9SSsrzBBlCcYp2CkiDg0G2XsRheRhbFs/l/BHPiy5in6f+riivyuJEKZKkdA/c3O0zO0HQxMr0w=="#;

const PEN: &'static str = r#"eJwBQgC9/zxwYXRoIGQ9Ik0xNyAzYTIuODUgMi44MyAwIDEgMSA0IDRMNy41IDIwLjUgMiAyMmwxLjUtNS41WiI+PC9wYXRoPvZCDv4="#;

const PENCIL_LINE: &'static str = r#"eJxlizEKgDAUQ68Surf2t/1K4dsT6AXcCg4dFBy8P/YvOkggkORFrno37LNZKSD4lk2RQbsi3zI6RnRcg1NIzXcRIuIygfJhU09k0/Z/n8RgJd/pAZUVHMo="#;

const PENCIL_RULER: &'static str = r#"eJx1jz0OwjAMRq/yiT2mdpwfpNIThAuwVWVgoBID9xc2EQxQ5MSR8uRne7zPjysux93KCQkK3U3j3j+n8Y1OHFFQqUCozELKeKXBI0RSDM0IEsVvamG8FXD89a4VGRJkgwhEzJcCU2rCxAWZquurIU+9uQZtkRI4UzpveLga+tPDnMxQin4XOmg/Pn3uc9+CkFVTXkJnwZm/fWkr5/IxPwFZsEzm"#;

const PENCIL: &'static str = r#"eJw9jDEOgDAMA79idSc0baMyhL4APsBWiYEBJAb+L5IBZOkGW2e9+3Ngn8PKFbknmgSGjAi2FJSlklXRgZROJhmEZAtNR3ebfg8XC8SNf3oB80QWUA=="#;

const PERCENT_CIRCLE: &'static str = r#"eJxty0EKgDAMRNGrhO7VRmghkPYGHkKiUEFBigu9vakuROjqwzyGZcmyzpCDQWtATm2vvZ5G7l6PvI9HgimYDR1Q48EXLONHAwGl1mJF9ITuZzcQICGs"#;

const PERCENT_DIAMOND: &'static str = r#"eJxtjzsKgDAMhq8S3BubtkGF6gn0Am6CQwcFB++PaRVBW0JCwv/l5Y/lDLD21WSwAdJoF4OOIAX9mJVia5A7iCEDbEoToIqAVhEZycoKWUN/QN0T5mrwdbxn8O9VHRoQD6gpV3dyyCKzYuBCs8jyk8P2034BAHk3rA=="#;

const PERCENT_SQUARE: &'static str = r#"eJxty00KgCAQQOGrDLPvx0JJUG/QISKlcRGEDFS3T2sRQdv38UwKM8MePZNFMSAcFnsECnEhfsp5l5ShQ2eaMjizTUzgLa5Cgq4UqEIlvjRq0FS34kfyJOTHLkV8JfQ="#;

const PERCENT: &'static str = r#"eJxtyjEKgDAMheGrhBxASUFFSHqZ0KFQHDolt7emiIvb/3gft3oVcBLcEDwJ0olgKaZRzMzrgzJr7doKdMG0jFtNcI/wGQNO8kvpeG3Uh2/kbSL2"#;

const PERSON_STANDING: &'static str = r#"eJxlykEKgCAQRuGr/Mw+0rGGAvUGHSIsKCgIiajbpwQRtHqL79kwx7CMiI40IZwpnHo5qsnb8lFvt36fMDhaW7CCKQQGkocMHxY0EDCk4L92mqHVUb1yA3ZlIh0="#;

const PHONE_CALL: &'static str = r#"eJx1UMsKwkAM/JXQe8dNut0H1ELv+hELHnrw4EE8+PUm7bYUVAIDSWYyYYZHec50OzdXEeKALK+uCAk5LW4FnLThjJgrrouE0LUdXLRpv8K6CW34wTeqieK0HycPZpL54GdWiAqC5Cu6pRBJOyNsr8H3OmK+JLhMGZkLq3OogkDhzpDYGhwdVLJovz3MAMcHt0jezTicLKhx2ONiD6cPFLWu7EQR2f+lhqmnmhFpqOx25getik6Q"#;

const PHONE_FORWARDED: &'static str = r#"eJxtUNEKgzAM/JXQd7MmutSCCnvfPkLYQEFUmAzd1y+tOoSNwKVN7nIhxTh0S9f2DxiHtp+epaEcGJhBQF9kTVWcdk5VROZMysoMzFwaZgOLZtFEISl9pY711MC9NDedRYKeX2mtg8FqUMIYfcij8xuujRwlTVK0LlTPK6wdSeQPP1CDyF2+wyFDIuDm4Bes0Ckw5tmGNgY60F8g7KthdtYS0TVH68Gjp5rUWTaBgHSE7JIARweVRO2vRzDA44L7Sd7xunqo6gOnM1MS"#;

const PHONE_INCOMING: &'static str = r#"eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaaEFUqBTt6ZvsahFassySZJIJU41Dt3SP/gbj8OinZ23IAoOAB2bwpqkOG6WpInGm2jAbmFnJBhbN5ZM00hN1bKc7XGtzYd2GgV9Fy7I5l6CMkbzKBHRhxdTwaIuswNxp9ZggdWxm//CVqkPu9F0OJRIB33d6KoVOgNGXK+Yx0IFkSthOw/IoJaKzxzxAwECtuCEvDViwHSG7TGGvICNx9ldDBXB/4GbJO7orRjUfYXVS5w=="#;

const PHONE_MISSED: &'static str = r#"eJxtUNEKwjAM/JXQ98Um29IWtoHv+hEDhQlDfBBxfr3J1o2BErhyyV0vpBlv9ytM3LroYKLWsYO3MhJ9jbLrmoOJumaWWtOG2WJa1WTrJn30zwEurTszAwkmfpU9A4PXooKRohJKGFLGZRBRyqJEH6xbL7BMpJA/epOaKRy3z6FCIuBhl2dRGBQYY5XRz4UBlJlgXQ2rWltEp4g+QcJEPWmyZIOAjIQcCoN9glpm72+GBeB+wfUkHzuXHar7AjGQUA4="#;

const PHONE_OFF: &'static str = r#"eJxlUO0KwjAMfJWw/4tNWvsBc+B/fYiCwoQp/hDZ3t6kHVQYLddekwt3Hd75M8Ht1F3JoI9AFi1l8iDblGXRETD6mZBDr5AZuJT0nahHdwRijG7DqpMrYYDWK0zI1zZ5L/IohBKGtGEtRPS2t2iCvjresNbEoRWlD8+Cyl3ej1B1mXNuFpzYBZ7sf4Lqa+9fvUuEFqDk1MCXiCZBwkTdOBz0A8dhfrzusPCp4w4WkkPOlQpduVBp1abxByuCSc4="#;

const PHONE_OUTGOING: &'static str = r#"eJxtUFsKg0AMvErYf1MTdR+gQv/bQwgtKIgKlVJ7+iauFqEly4RNJpkw5TT2S98Nd5jGbpgflWEGDwIMZIFNXZ52Sl2uxIWFZOClSTNVhqyBRbJXeqROzdzCrTJX1kUY+Jk1ujSVoISRvCoEdGHD2PBosyTD1Gm1iBA7NrF/+ErVIXf+LocciYDbg55KoRNg9PmG6RroQH5K2E/DvJAS0cVjGiBgoEaMkBcHLNiekF2icFSQkXX2V0MF8Hjgbsl7dVeMqj9VCFLe"#;

const PHONE: &'static str = r#"eJxtkMsKwkAMRX8ldD/XSdpmZqAW3OtHFFx04cKFuPDrTfqQAUvgQh43J2R4Tq+Z7ufmJkKsKPJuJyGhaMFBwNkSLkhl07WRoW1oEZNX+1XWjgY9mPdRN6XLbzl1YCaZK56jkEwEuds0LoFElvnAfhq63krM14xYqKDwxEbWzaCkD4ak4FITzLJ4/xkOQH3g/pJPMw4nf9T4BdqtOmY="#;

const PI_SQUARE: &'static str = r#"eJxtizsKgDAQRK8ybC9mV3AtEm9gay8qrp1I8HN7DYJYyHTvvfHr2EecgQqCjfNkMRBXhPUIJIR9HqI95EhN7fN0qP3SRcMQqFGosUsioY9gB93+TQnWTiBw9ziTTFp9uwvMEyfm"#;

const PI: &'static str = r#"eJxNykEKgCAQheGrDLOfchzBAvUGbduHBQYRLVrk7VMX0eLx+OFzx35ukLVHrRAe9jgi5HKmlK4VXF9NcNdyJ1g9TgZsVMSdBe6EBMoSS4WV/CAPoFVsUpFQ0ySz+egL4vIfgg=="#;

const PICTURE_IN_PICTURE_2: &'static str = r#"eJxNjM0Kg0AQg18lzF27k0qLsLvnXnrtXVQ63kQWf95e5yaBQBK+xLkrhiHJl4r29+oIIrgqVvw09wyuGvoArbVu4QOtkRwf/pHjMvYFRxJ9Cmyc/laSvAXbNBS72iBY9iQUXK50zol8Au5aIGk="#;

const PICTURE_IN_PICTURE: &'static str = r#"eJwVjEsKAyEQRK/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR3mLoihehfezKuwRHytM7Ju/u0JCM3SKODCNXDYkLDPxOpEdKC2FNtiazQaEjefNgx2owxRu4zOF68gV9DhfWiMuCL+I4tgjfM69ai8G4dvDIlx9s8McTvoDoPwlcQ=="#;

const PIE_CHART: &'static str = r#"eJxFy7EJgEAUA9BVwg1wXiLCF74HDuAQgoWlhVg4vV+Lk4RUL36s545tSouYRXDINs4siMZEDMrWp+rdS6v/B4Fq9OOCLpa74QcRMBYw"#;

const PIGGY_BANK: &'static str = r#"eJxtTjEKAzEM+4q53W7sxO0F0vtB19tLOnjsUDL09VUo3HQYhJGEpPZ+foJe9+WhlbyzilNik5VUCmcyzuKTZVWWDCQYIK5AyFTEd0tRBlvkYfMpHVaH58aKAAsDF2w98RT+aaDT7t9la5c5YWvHEKM6tM8OlYoGXOiJT6+kGulQflP1MIs="#;

const PILCROW_SQUARE: &'static str = r#"eJxti80KgCAQhF9l2XvlGvYD2rlLDxEp6S1E+nn7XDoJMcwwMPPp6LYEV7DJG6QBwbuw+/T1eBuUCDlbhIdz0g0Dkz7W5MEaXEgCyXms1SprBWyRRSAqNVPPAF9LoD9J/C1dubzyKyne"#;

const PILCROW: &'static str = r#"eJx1ijEKACEMBL8SfIB3wXhHIFrb+AjBwtJCfL8IopUs28yM1NQKZKciGqCOn/LyTOblmP9qGCiwtom0hfl3jQOa3Q9sSBoy"#;

const PILL: &'static str = r#"eJxljDEKgEAMBL+yXJ8zEY8oxHuI3YGFhYKF/8fE4hqLnWIHxu72HNjXdAnngjEgTMJtykvBB4aASUlP//FXUOiWqg3RqtaLs7dirrt9AZOcGgo="#;

const PIN_OFF: &'static str = r#"eJxNjEELwjAMhf9K6H2xr7brAt3AmxevuxcUKgzxIOL+vRnddOTwheR7L033x41m9MYZ+qxwCuVcOaTDIg2pqttTXcQqY81iJz/zq9C1NxcheYNjmx05sjpowADpTSZdY8dy2n6WAiGw8yNiqXVL0a4OgYSPfmwL8j9lG3+O3Mkv8AVA4zO6"#;

const PIN: &'static str = r#"eJxNjrEKwzAMRH9FeI/rE0qcgGPo1qVrh26BFlwIpUMJyd9XCpQYgY6TdA+l+fV+0obRITpaWZUdbaqsumL3OZ3sLKfP9C30GN21JcQCWRr42E1MTMFKLWCzYbbeN344/5cgaCro/a0rODKakktfe5KCRWowdjAd4IobSLGtZ7nbn/Zh/gE0bS6N"#;

const PIPETTE: &'static str = r#"eJx1jTEKgDAMRa/ycU81iYqF6gn0Am6Cg0MLDuL5TQcdChLe9PL44dyuA/tYJYEImPjQ6MlXU6izmsL7sCiEb/qxiTv0UNeSsYljZBqwnUJnHuCja0tH5qIlQ6bMbCsX67f2AMSFKQ4="#;

const PIZZA: &'static str = r#"eJx1jbEKwCAMRH8luGs90bhYoR/QjxA6uBQ69P9pgtAuloQj3ONy5Wp3p2M1OxIB3XmYWhZ1a/kYCOmPMYHn7AyCKHhiyzb4TS7ZMYom35LL0pUd0ER9pKGaEB8uWpU3+ACqBy9e"#;

const PLANE_LANDING: &'static str = r#"eJxNUEsKAjEMvUpwn2eTJraF0RPoBdwN42IWXQh6f0wHGaU05H3IC5me83ulx/lwU1JdNR0u03Fwl2lXMkohSaMqta5scBII3Jf40IqGamhBmr+QN7V88bVShffMp4DJ4XmOKErxZDDD5dpRlBy2a+lPMxgpdIFFVZSYNyKSoJ06trk5xFiiMtLArbKMTiE9OqkQXxhWgmQ0D5sGnwtD7WrQSAqb/1bjCGbket8P8gF1sD+l"#;

const PLANE_TAKEOFF: &'static str = r#"eJxNUEkOwjAM/IrVu4fY2aXSF5QPcIvg0EMOHPi/cFhKFSmWZ9HYnh/tudH9PF2UVDd10zKfBrbMO5PgE0lGoGCls7JVCCPGZi5y9sSQQq7DdLKj7oOuhUQpUuqojHB0KVyF9gCn5A8+hbxhqWyS1BRhiO37pWXPFraqUG6CMVH4cijZulo7wxfkdGOoR0iMhBJY4AbvypoRy1hM/wMZq5ZdrvsdXoLvPKc="#;

const PLANE: &'static str = r#"eJwdj7EOwjAMRH/l1N0G22naSqULC0P5CBQGhgwMiIGv51xFSeR79p28vh+fF56X4W6TzrBFHVZh1kNH4b26ocJNRxR+iCYmiXCWQpFgt8CMwvGq3hIqWxY+psRdlD7U/SAwaHAs9gXmXRxxK11SDzgPodhXokcyRowajRYMOPJCo6eTNy1pXUUn1qb+G7b1lAttf7mWLN0="#;

const PLAY_CIRCLE: &'static str = r#"eJwti0EKgDAMBL+y9APaHsRDzGeCSKE0pXqwvzehnnbY2SXJXcoJeY8QU4CMmd1iDUzL9ExNy7i0ommuz+0WO+KGmGDoYIUf/iF/lUMZUA=="#;

const PLAY_SQUARE: &'static str = r#"eJwli0sKgDAMRK8yZC/iB6nQ5iDuxBbThSAloN7eRhczzPB4vqRNceWoEqhzBEl5F/33HWggPF+Xenpi35rA/lxVEAMdMxwmjE3NYtgAvy0zF74="#;

const PLAY: &'static str = r#"eJyzKcjPqUzPz1MoyM/MKym2VTJVMFYwtFQwNFIwVTAyBBLGSnY2+lBVdgBVDA2H"#;

const PLUG_2: &'static str = r#"eJx1zDEOgCAQRNGrTOiNDMqqCXIDWws7EgtKC7PnFxsqaP+bTHjSm3Hv5tjgVEwM419iqJ2+Bw5c1DfEY82cGyAgz2JOpyQQWBAcypFVe9X9B0vKKRA="#;

const PLUG_ZAP_2: &'static str = r#"eJx1jjEKgDAMRa/ycU81iVKF6gn0Am6iQxfBQXp+00FwqEsC/30eP1zbHXGM1ckKIYG4LurMAl9Noc50Cm9n4QbcJtISav8Rg4ed2Hk0pPbVriaS2CfZTel8DmEhdC0JbJZ83Q8lsy+W"#;

const PLUG_ZAP: &'static str = r#"eJxtjDsKgDAQRK8y2BuzuzFaRE+gF7ATLCwiWHh/zAa0SRj2A/N44d6fE8fUrN4I2BrZ2Tjo2BzRbyEGjbH1KaxcgWVwa+bQqXAOn/ZiMENaKat1MD1IdFkQVQCyWvqMgVzFTmNyO7jTRz0/8QIM0jQN"#;

const PLUG: &'static str = r#"eJx1izEKgDAQBL+ypD80xx2ccOYHtinsAhYpLSTvNzbBQtlmmWH8LFfFsYYtMpgbaUg+PTD5UAss8weP+icM1rQIBHNfpP4qyRuQZNtHegPetSOj"#;

const PLUS_CIRCLE: &'static str = r#"eJxNyUEKgDAMRNGrDLmAJqss0t7AQ0gUKriQIqK319pFuxrmP/Mt+77Cn0AshPzNSPD7v9GG6tGO+UxYAk0KlqSFSuqABXo1eAGJABmd"#;

const PLUS_SQUARE: &'static str = r#"eJxNy7ENgDAMRNFVrFsAJTQu4mzAEIhEOB2KLAjbI4sC2v/uUq+b0S2YQVrbriYIDOpDEEFXK6ZvGb7JafJDTsdqSkWwMIWo7ODpByESnx88Mn0d5Q=="#;

const PLUS: &'static str =
    r#"eJxNybENACAIBMBVCAsYiHQvGziEiQWlhXF+Y2No77DGDpqNu5FoSGVHeeb4I0p20lyGNA63"#;

const POCKET_KNIFE: &'static str = r#"eJx1jr0OwjAMhF/l1D0h5wSTIfQNmJA6sFUwdGBAAvH8OFFpl1bRXWR//iuv8TPhce4uEfLlPYCQJrlGKCJO7zl29pvkz7u+HGp3X5YZzNDJB24gBfMOkuBzRDYbExJCfe7oVZs9HW2hzIimmm122z6BNDaIrNOQXBp0qf4B/eU6Pg=="#;

const POCKET: &'static str = r#"eJxNjEEKgCAQRa8yuI8aK2lhQgdo214oUBAVkqBO32gtZBbz+bz3ZdTJwD6zdYDeoNAcOHR0SJ9fQiPF7muakpeqIQZxG2up4Q9Tss2zSsbgbmf9ATFYn86ZTVlFsgZAQbmgP6RexLYiBQ=="#;

const PODCAST: &'static str = r#"eJxtjUEKwkAMRa/ymX3i/HZiW2h7Ay/gbhgFBRdSXOjtzYybLiTwfyCPl7nct/K4onyWQAZsXgHl7dWFdT78zuv8zK8bLks4keCQPRB9iA6xRKiJ9gm9qCGpZTVfGiAOFlEehRUySS3UzlVftTv5CKZssGYmRsQ/EAdw9B8TpsZFYdyBX+/TMew="#;

const POINTER: &'static str = r#"eJyFj88KwjAMxl8l7J6YpOlsYe4J9Lp70cMOHjxIn99UpChMRgjkz+/jS6ZHea5wOw0XVRArCRKwh6BXwzwd2n6eOiUJRCpKUdA3yKiolb978H5LaiC85B2lbCkZMsXF9qT87+APKB4GXMPvmyvqFZV8gkYRKY0YKWcfBbtjoLFl9xJwNDRez0eQ2D1fm+1L9Q=="#;

const POPCORN: &'static str = r#"eJyNTjsOwjAMvYrV3Y88N20dKXRmgAuwVWVgAImB+wtHSGWAAXnx+9r1sTyvctl3J7r4YmKS3qNZNhR7+g/F5G6uu1Y71095WEyK+Ld2Z24alb+ClsRXDFGLggwX3tQwCm1VEINialFEwWEKaoyHCCpyI5XHPqTSvKGhVwpced4uvQDH6jhT"#;

const POPSICLE: &'static str = r#"eJw9jTEOgzAQBL+yor8Nd7LDWXL4QT6QDpGCAiQK/i/OSFDMNFNM3adjwf/TfdX5hiammS70kKEXo6/i1MaUWDIu9dCIAxXBGg3BzMIBRr1chPrrxvpqh7Hen80MZpKZG08/AR+vHzg="#;

const POUND_STERLING: &'static str = r#"eJxtjCEOgDAQBL+yqS/c9oBWHP0BjyAgTiII7ycIUlM1YiZj1347zjVsLMiHxHlQ1Vh+QkK18YuqtVSQH04dsyDRmbqG6my3F42TIJY="#;

const POWER_OFF: &'static str = r#"eJxtjUsKgDAMRK8SeoDYxJq2UAsewEMUXLgRXIjnN6XQjSWfxcwbJt3lOeFYzU4BZwFBcVuECFaHgC16D7SYnKZK5tR5Qao4SWk41WOU0P4gQQz8ur9xqa5Nut37AL33JXI="#;

const POWER: &'static str = r#"eJwtykEKgCAQRuGr/Mw+c8awBPUGHUIoKIho0UJvX0OtHg++eJV7w5Jo5sk4D2/8UAICLBjcsZjRwVKOvcIcj/1cUSURC6Hy1/Z/e1/UqsoP8V8WFQ=="#;

const PRESENTATION: &'static str = r#"eJxli7EKgDAMRH/l6B5sEoJL7Ozi6l5w6CI4iN9vXFQot9zd4/lRz4ZtSotAm+RUfHiu4h9g6MVcBYIcYYo223+TrNqb+4hwjQyRF9+Q0Ryn"#;

const PRINTER: &'static str = r#"eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv794sZQAiGcnFzi8zyuY5oG5DlN5RnIoYUDwza/1lL0p78Vfe6KoA90dbq8VB2rddayhg0vpt4BFIh1RwBe6uMFWAx/8zU1+sdwK1gD2YrwSn0RHZkgQ7pLCdQQ3vrf5m9m/AAFujAw"#;

const PROJECTOR: &'static str = r#"eJxtTkEKwjAQ/Mqw99Tupk0tJD334tV7iUIEBSlS9PeuCqalZQ+zM8MM4+/DI+EU6FCjgUVNnd99tM7/nRbuaNf6ja1GxEi24mWM1zPGQJYQn4FahVcg/sZ/7qyYudhbsPRSDgJBqceKMlWZG/36BTcymWoeMJKk4GZjOjuwS3nhG38KPMA="#;

const PUZZLE: &'static str = r#"eJxtk8FOJDEMRH8l4h5v4iS2I7FIe9/9gb2NhgMHDkjw/6LKAaERaLprunviLj9X5v7l8vZUHn/f/etb5tjFJda1Sptbhqq0tcVmiMaW8HjusixK6lWm8/BmeNDi88rbfK2iY+Fax6hfj59rF+u9pF5kB47S8OlVYrioG6ynwx5nNMVNVNlG0XVRWa2XoygqrQ7RPksq2pkm3QwAS+Z2lrAWRr55flrBhnanm2wGb55tlqNnFRteOD2f6od+/TaTi+/AKI5eMAXNWUC5sgHCQaKbWHsAC2UxATArl2V74CFTEnUSDT8KIsP6HhIb30uBZJ6FP1khovS76enPt+5RtDH2rlfUWMfUh1UmNQl02P5O3JcQRw8Ka0XLgcmNNWR3EI02ZPUFIKcleuOmaD6o/QapMZ61ayrnEIi1b0Ok3BxrsdPGoaIcyS+8fZg0wzJDUrCqNOXW01VTf6LSA3YlE58kEdjO1suwbjavcsawh8OIhNDNkLj5JkKqTCknXL/HVDKgo2SyiZgqc2JMLPAs/X/3cP+Lf7GHd/XQrjQ="#;

const QR_CODE: &'static str = r#"eJx1j8EKwyAMhl8leHdrEmYv2vMue4iylqW3UcStbz8jbKVMESSa7/+Mfp3vEV7LFCWYiwGZl4fEUr6DYQNb2dd8QDP4s+KDLyG9ckegKvpLcZ37Orbi/aWeYxSYgrkRAjqxPBIQdLpsrhIrqtARJUynDis9JOjTLkGVXPsKyIAkTQe3W+haTztVYn1gpPbA+TN2b30AxGZ3oQ=="#;

const QUOTE: &'static str = r#"eJyNjrEOwjAMRH/F6u7gcxpSpNKZhbU7CkNHJFAHvp5LkMqCEIocW3f26Y23y2OR67E7R3GUKCZZwRrmVEwRPGnIaa8eDFld/dSXJoupCy3+CIfsM1BMqkNRvD0w7l1Ym8toRojfOQSzgQ3BIma3sq0Knt007irZNG58SL8A8wdw0f8JF07UvGo86JlCq1/jd5oX7HxAYQ=="#;

const RABBIT: &'static str = r#"eJxtj7EKwzAMRH/lyC7VkhXbgTRfkK7dTTp47FDy/ZVDyBSMD5t7HHfzt/4aPs/hpQGFR3tbVSgCBIEMYY/DMj86tcwXKwUqjUo1ONJhMoezn+OHTLmxrhMnJLYqPEUc0k2BciG/q4xc0MktcveSO9k17XK26NH+aiQbiXuBelB0veuVOXm88hirA+cI8RLGtzM8K51kOGqpYbzfK9o4yOX9AakVRzA="#;

const RADAR: &'static str = r#"eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBUqKEjxQW9vYrWCRPbvYXY+pr3ubxMdOrPjBBvJI7mBLeVeSpESOThv+nZbnvt2tXjSCZYrikASJaisLM4jDMfUwIWKhRXiKSLqoKRvRwNxlBWNNYcQN38CcCy5maEfXAFyQIy/vPE0j+cjzZ0RQ+OjM1zu/XXz0yJ/0Rd28BllERKFzN+UteKez9tSxQ=="#;

const RADIATION: &'static str = r#"eJx1kLFuwzAMRH+FyM6rSYm2DLhZOvcjCnfw2KFTvr53DpouDQTZoEg+3nH7+vg+7PP18h5pkcd0uW4vertuj8yCso7cHc1RjtWxeCjobw2rFfOJZgNhhAQLA4VuoXPUPrni4Ylh6Q2dcfCurqj4Xt7Rb//MThJyx2yThQvpIRnJ3iCxzs7wBcoqIzJS8iRwPuskT1Nm2mgWSCwUqsPpR92eeI4VQ6blmTJoivMJEIgQ+r7/B+9kqiLYMU5/v6tik+fDo/SpiJvgt5s2IzR3Y+1eKY1/in4ABr5YTg=="#;

const RADIO_RECEIVER: &'static str = r#"eJxdjF0KgCAQhK+y7AVSoTBQb9AhIqX1LWTp5/a1BRK+zMB8M+O2mQmix6kHPewGg+skCq4CPTakpIXh9GgQymeU8krs0SJcrx45Mj1IyUr6/z8L2pCqfzelvCU9"#;

const RADIO_TOWER: &'static str = r#"eJxtjsEKwjAQRH9lyL2rE5ImgaYXz35EiUIFBSke9O/dRLCCZckGdnZm33CfHjNO2RydJLAXHghasSC8RLSpJDMOu7o5Dt/90NQwqcWhtX2tTsdB/Gool6Vcz1iysQblmQ3r/8qmhX7Un1hFsJobi4WW2F4xyBbqwgYGk7ASTklSj9YqB+tzwg1HEg/G2f9LtwirxzsSDly9bxI+SG4="#;

const RADIO: &'static str = r#"eJxVj7EOwjAMRH/l5L0Gu7SkUtKlMx+BDBJIDKhigL/HCSJtBzuKffd0js/z64ZLotOBB8jAMgmkY4UgcECeetEYd1k5xqo/+lZ6VmuU21o9C/ZN4G5x2H22xxX2SSRKmBN5t3f5uei3XoEzFE435+FfhYsNdzF46hxy0rZk9scvkHKNt+r4ApCjORM="#;

const RAIL_SYMBOL: &'static str =
    r#"eJxtirEJACAMwF4p7kUrVhCqH3iE4OAiOPg/0kUcnAJJZLU9oGdTGYgHBVPEqivylPQNkwJ4h4wMEaPyPgcuEReZ"#;

const RAINBOW: &'static str = r#"eJxtzLEKgDAQA9BfCd1FL8h1OfsHfsSBg6OD/4+9IqVDScj0iD3+3riOdJKQ7LKhtmVhnVRsDVKsQw2n0KYEwqmKo+wEf7YP6gOKCxw+"#;

const RAT: &'static str = r#"eJxtT7sOwjAM/BWL3Sa22ySVSucufEQVhg4MSCC+n3MGulRRHvZd7nzza/vs9Lhd7lpobIlVCrZzX2926gV5SyRVohylkq2qjV0mSlzIRXGWbwLJxMCpNMS6LPM1DJb5sMkA8bFBVlwyy0jKUkKYJQOCWhDQyKh7OxqOsQKennGb1CYGrkkJR8ajcv9lkQG3UqLIAwtn0KC+amkdTUewsxk9JOsGNDTARioeT5lktg6bkXUmPHnY1U6D07RL0j/0Az8IU9o="#;

const RATIO: &'static str = r#"eJw9jMsNgDAMQ1eJvACfQ09Jl4GK9FpFomxPKFEvli37mVs5jO56mgq2HfQIXLsggVofQUu91NyuyLx8QOaB/XWs4sNHE4i/NLEXwN8e7w=="#;

const RECEIPT: &'static str = r#"eJx1jKEOwCAMRH+lmW8GTQMIhp6ZRcyRTCAQE8u+f4dBsTTNy7W5F+/yVLq25VCSV0wTtiSEnTNLY4AR/3guKa5dm+KQW0ehsisCh4HNYLTqyP2ie5g1YfXZj88Hf6koeQ=="#;

const RECTANGLE_HORIZONTAL: &'static str =
    r#"eJwBNwDI/zxyZWN0IHJ4PSIyIiB5PSI2IiB3aWR0aD0iMjAiIGhlaWdodD0iMTIiIHg9IjIiPjwvcmVjdD64Zw94"#;

const RECTANGLE_VERTICAL: &'static str =
    r#"eJwBNwDI/zxyZWN0IHk9IjIiIGhlaWdodD0iMjAiIHJ4PSIyIiB4PSI2IiB3aWR0aD0iMTIiPjwvcmVjdD62Xg94"#;

const RECYCLE: &'static str = r#"eJxtkEFqQzEMRK8isrdqWZItQRrorov2EIEuskihi96fjp3woclHIBsJvRnp+HP+vdDX6+FzkOS7cYifhUNppYqQIuyjcISgOMLv+dbjWq3Mgn0MluyU7IfT8WVyT8eNLpjNS3Cr+oAHhN07+LnAds+3Xp3s4Vc8rfXSWNoz/Vsw1IvSih116CYklR0GN5ukDFcVxdhhJqsZOQ4y166psBDZ3h6OQyKcuMZc67/5OebWcbe4Kqcp4W+6Z19ZR8BSVyWDVl+KsfIoq7KN/QFn4mBy"#;

const REDO_2: &'static str = r#"eJxdjLEKgDAQQ3/l6O55V5uhcBbcuvgRgoOL4CB+v21BhxKy5CWxa7sP2md3KkgDYWhyycZKkn189UIxR8YCBlVLUygrxiNdXIrkJev0P71daBiS"#;

const REDO_DOT: &'static str = r#"eJxNi1sKgCAQRbdymf8eKiiCuoMWIRYY+BESUbtvJIoYmPs4XJfWmsqCdHkShlBZCOlkkRTc8ODgtrhnzJ4mKWAOnTvdaGt/TEGYaGEx8gnYrvk3acheFcXffNMbff0gPw=="#;

const REDO: &'static str = r#"eJw9izsKwCAQRK8y2JtkXdhlYeMNcgghhUWKFCHnVwtlYJgPz9/yVdxnuBJBf6lRQvZ9rNnXxyAtBsPRRbA48myCtPHD3XWhDdu4FTA="#;

const REFRESH_CCW_DOT: &'static str = r#"eJxtjMEKgzAQRH9l2HvSZhfXCjHQe/sRkhYieBAR0b83KuhF9rDMzOP5vhkTfjV9BTxpUgr+sXXBnws7OH5XqPDcT1FY+Qhe9yzzZDSZO5NkUXOJXAG1ZSeGbXnRsR1i90dcanJMGPIjxHlPGTrmsAKQIDBq"#;

const REFRESH_CCW: &'static str = r#"eJxtjlEKgCAQRK8y+K+lpiaYJ6hDCH342Ud4/tyEipCFYZhl30440pmxL2xTElIlD4+RhntevXCmScuscBNUlVVjZjEMdB3Dw9DQxWTT3XzhIPeDg+D8hlMV24FIW/NsyvvgArClLag="#;

const REFRESH_CW_OFF: &'static str = r#"eJxtTzEOwjAM/IrVPaa2EyeVSiXUhQEegcrQBYkB8X4uGboQJTrZuvPZN78fn52e5+GuQuUmhXOkBLhMnBM1GNsTJVtFyEhGHo2MxcFLrWIelvlUrZb5MCwkfrVv6lDw0NUwnYQiZVbHTo8N/uUvyJ20qir0LlPZlOOEJuL2IORsHrRrV5OKbpgLNYLwlAMSwLvkvhoh9tDLodiMT3pwP/YfT40="#;

const REFRESH_CW: &'static str = r#"eJxtjs0KgCAQhF9l8b6Wmn9gnjvUQwgdPHaInr81QUJkYVhm2Pk2XOnOcK7sUCBk8uBhphHgkXZudZXqGW4XkCS7FOBYDFO5jqF1kK0enVGPsz+g1PcALAD8APSNGZQ4sjdCtOgFyMktuA=="#;

const REFRIGERATOR: &'static str = r#"eJxtjLEKgCAYhF/lcJf85dTFnFtaG9qEBseG8PkzgjCQW+7ugy+e+So4ZrU6+EwQpkVAzfLbYBVmC/seurUl9Fvbze8qxekxpth5xRThgIhDqP4DN7Z8Ie8="#;

const REGEX: &'static str = r#"eJx1jLsKhTAQRH9lsN/c7FySKERrGz9CsLARLCSFX6+CT1C2WM7MYeLYTj26Mms04J/UZlX8bVkVj2ZQGh/gjENuvIf7VNTujrw4TQENLUHY7YTC2t0ZTLx4/ewfLEzC+RxeAEsvLSk="#;

const REMOVE_FORMATTING: &'static str = r#"eJxtyTEKgDAQRNGrDOnFbLKJFmtuYGsvWGwjWIjnd20kkDBT/SfXfiuOxa2MaWOl/ERXZPxykR8TgtfcAYpgzKatnZRgt3UseLOhthd8SCX1"#;

const REPEAT_1: &'static str = r#"eJxtjDEKwCAQBL+y2B9xVUhjrNPkEUIKGyFF8P05AyEWsmw1w8Qr3wXnZipXOAQE0ZsUlw5S/PDhQTZhVgqrYzcLJ2rVkJO3I7OSI+jbUOrm7icm1bSF7a88Zf4tOA=="#;

const REPEAT_2: &'static str = r#"eJxtjDEKgDAUQ68S3Iv9KVaF2tnF1b3g0KXgIJ7f30EdlDcEkvDCno6MbWoKMcIZB6WJoa19DPe6iIMMc58IwipiaLj677OQkK56jPJnEvjsX5EmT7HP8wKymSTx"#;

const REPEAT: &'static str = r#"eJxtjDEKgDAQBL+ypD90k4BNTG3jIwIWaQIWkvd7JygWYdlqhklnuSqO1TUu8IiIonc5TQZyevEeQHZhUYpZRzMrB2rTkJenI6OSJxj6r2TmFj7zBt9bJUM="#;

const REPLACE_ALL: &'static str = r#"eJyVkN0KwjAMhV/l0PvMJatug3Vv4K330g27C0FG8eftTXuhIgWRQhrIl3OSDJdjDJic2bOF9TVxxVVPAiEx47BJ5XF4QVJDvCLQH1WvoUgJOl9DOUoMlSnegWtPWY2EkmvJ89ygRZMeNQWVJHLY5sFbdVQogYHf6Dr7iNsyxeBMZxDm5RRiTh/OsDVY786IQY7alfjPMS3Yfu18tb/X01P90fcExIFhrw=="#;

const REPLACE: &'static str = r#"eJxtzkEKwjAQBdCrfGY/NZlW20LTG7h1L2kxXQhSAurtncnCbkIgE8jL/5le95ywBLr6Dl107BvfjCwQFpqnk13P0x+Jg0Ql0Ilm1K2qBEN0UMdmuK78Bd5FLmksbK21zmeLHq0tbispFnI7l4/32qjIYPIH3deY8Q3kO0Jat0fKgQbCe1tyKqdPICHsZegr8/MPjwpHuA=="#;

const REPLY_ALL: &'static str = r#"eJxlyrEKgCAURuFX+XGXUoTboM4tPYRQkCAqJEFv35VoKc54PltLulLMG2qJuR1OEBRBQ2kQSHg7vMLbn2XEmDru/qND27E6sWi+0yl1MDAYe9JIMz+ckb8Bghsl/w=="#;

const REPLY: &'static str = r#"eJw1ykEKgCAQBdCrfNxLKgMmqOs2HUIoSBAdSIJunwXxts9zK3fJdQe3XPsZhIO2IGgDByuin/4RPad+YAtiNQp6vqRJNKZ6SZK00NdHig9v0Rgo"#;

const REWIND: &'static str = r#"eJxdy0sKwCAMBNCrDF6gTIqLQup1SqEYoW68vR9ciJBFZpinyb7yWESyN+b/diR4QcB2hMfILugxh0F3IdIFz07a7zGKlVTOiRxj"#;

const ROCKET: &'static str = r#"eJxtUDsOwjAMvYrF7kfsxE0qFSQ2Fg5RlYEFCamIgdNjt8BUJbb8e362h8f4vNH1sLsUGEkHm1jCgnasZCFzRi3sQfcmVGG0gsoKyYzUu9HL6F6jRaV4SzCy791x2AfJcfhR3UVJjDPnUZX8B0JIOaO3kyhao1V/E140JW9elVEbVRh3JM6pyEarXkq5kG4QXnrvdy4zzFmRslOVSdCpr5oaBdhlA7cM+ooDJF/VjIpfIDAcaN/SQv7AD1TxScQ="#;

const ROCKING_CHAIR: &'static str = r#"eJxNjmsKgDAIgK8iHqDS6AXbbtAhgoKCqEH9aLfP2RNFBb9PNH6dwzwtA/h1WvbNYp4UwFBKJY6l1o7OpA/qjAqBLXKGEMiiInDI1Oggq+KS/sIHyp7qRyHp97FX8N0+Qm+x5aSKT3SUg2SmEVVQOGLuBP7YMb0="#;

const ROLLER_COASTER: &'static str = r#"eJxtz7sOhTAIBuBX+ePenl6Qo0n1DVzdjQ6OJhqfXzBGF9KhhA8ClG06VixdNTBiO9ZVX36a6ssLMaiwbywjsdP9bWxQn2SDdrFBKemw1pIbJgIhyIsgR3OSSFbwOYNBu6L8T1FEcNmx5++qC7M8Pi8="#;

const ROTATE_3_D: &'static str = r#"eJxlTzEOwzAI/ArqDjXGGCylWTrnEVU6dKlUKf2/ComqDl04g++4Y3rd3g+4X04Ld2q9g5FeWak3gUZVDFhoaIUKnGXEbAQaBN0skOtW9yEX0CgrSatQgLoZUunxgVRP83ROq3n6Gj7DhUfL/VYMhJyjIe+Y5ej/VQvHPiW1NWmlkbtiBLXUNkE7IBhVoCAX3MPlQ7eWkTFDoq4hFY+cTm4SKmOP6xx+ph8MLT08"#;

const ROTATE_CCW: &'static str = r#"eJxNjEsKgDAQQ68Sum/tx7EOjD2BHqLgoksX4vmdIkgJeQRCIle9G87NHAkhVgbDI6jZanaZPvguu7g8Iyr2hNUUmfq4yHCRHmr0Ny/fYhZa"#;

const ROTATE_CW: &'static str = r#"eJxNTDsOgCAMvUrD3iqFqk0qJ9BDEB0YHYznty6G5eX97ap3g3MNO0eIXBUURnCKinowCbvMpMm9ieYM7LB5dwnFhm9crL9IjzSUP3sBG9YW+w=="#;

const ROUTER: &'static str = r#"eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PER06RF4sf88vbMut4B2pI7zWueTfJoRPJDXC9oe8rPdcInmawrk+TOGRSsYc6epYFOovrlq9HkxlR5WWDlB5WktG9oaRdUwGg9Q5DewcWj2dVDHuLXn4Pa3KvR7iX5QaPFE="#;

const ROWS: &'static str = r#"eJwli0EKgDAMBL8S8gFJvXho+xkttiAeSsDk9ybpaWF2Js92MkwpmBB6G3fngnQgTA1kx47wjYv74uqg5s27mp/xNhAKSZIVZIYtWaoUa65b9QcgIhw8"#;

const RSS: &'static str = r#"eJxNikEKgDAMBL+y5AMaqEKg7Q98RIiCggcpHvT3pvag7MIuzMRDzxVzoimAWQWC3sO+Qjl2Fef4k4LyCG+z3v95thXbF9iVaCDYnYiFUHyq02h+AIcgHaQ="#;

const RULER: &'static str = r#"eJx1jk0KgEAIRq8i7cfS+SuYOkFdoF3QokVBi+5PGrUoGkQRH0++tE/HAnNbDExogTzaidGBdiVF0hbdahiDHMMbGqtbzxihxqiM4Bq3qnwVy9zmi17uWHSp1AxdepJs5NADsQw2/MdJUJPFtZCQl6M+9x9+AsHHPTA="#;

const RUSSIAN_RUBLE: &'static str =
    r#"eJxNyrEKABAQgOFXueziCp06ZouHUIYbDfL8koH+8f94tCnQk6oBEIWaAwf2pqnEhaQym6My/9bLGxttFxJB"#;

const SAILBOAT: &'static str = r#"eJxti7EKgDAQQ38ldBeb85AOZ2cXf8Ct4HCjg/j9tg6KIFlCXp7t5XBsU1hEwDRLUSjindqcn6HTNWTrm5PtNQkqGCEYanOmv1fjJ8eHXAAuHlc="#;

const SALAD: &'static str = r#"eJxtj7EKwzAMRH/l6G7VkpyqgjTQrUt/oFtohyyFDv1/KscQGhKMD3xPPk79Z/xOeF0Od4PwxPkw9MfqDf1CWAKNDkeejye/6f8b/tj7xqRnsIxCBfXWWU5UUiEzrFwoSZI9u1iik25stSbrbKbKqttxkzZPWRFsW/LNAQRRaW+BTA4j6a5K7pilbVzA+ZlBFrUjuDB1UWOJ+AFQYUoE"#;

const SANDWICH: &'static str = r#"eJxtjr0KwzAMhF9FZLdqyT9KwM2coX2BboYOHlroUPz8lUuIPQQNEsfdp0uf/C3wvE53B0TVZQIC+x+9CsVRMFSNm9Z0aaE1HVFioGXzh1WNzcpdaNIIa5sqjwn9ZhzOJ/x3qwaCIiai9ZmBdwijj2BvrGHa3OO0Gi6i7TQMukNRIbz0kQTwGDrLMMqMLB3yA6WWQD4="#;

const SATELLITE_DISH: &'static str = r#"eJxtjTEKgDAQBL+ypI9mjV4QYsAH5AN2AQsbwcL/o4kEC+WObXaY9Uc6N6yTij1okmssUcKUY/5FBd9mLvhK7yM4wGr7rSIdaJNAHocWLT9Ux5uai79uEa/uAj2lJUE="#;

const SATELLITE: &'static str = r#"eJxtzEEKgCAQheGrPNxLjdqoYN7AC7QTWrQoaNH9yQlqkwyz+j9eOuu1YZ1VIQuPCIsJfndwKqdBYk4vOciDCK3p53XPBJARAtYsZOntMAKstv9UIgxVBmOUaxP8oRubdSuF"#;

const SAVE_ALL: &'static str = r#"eJxtjS0LgEAQRP/KYF+9HRbdcJotVoNNMFwwGOR+v1r8AJk2D96L27wnLG0x1LCZIMI5BYVJw2qwrKHkQ4TQ0nt/P8Kp6GJ1qbp4CzWA2VL9hxzqWZoknpt/Tvb2bYyP6QC/jS1m"#;

const SAVE: &'static str = r#"eJxtzEELQEAUBOC/Mu1dzNOGWs4uru6Koja2SPHrLTms0qupqe+Ncd02oi9VwwLCWncCQeKPkUTSBh2+j6TV0DsZOsipKhPfU5Vxiz3sNA9wyzRva6mY+WH4ZIo3hI9/5c9PhpvloEb+oRcebS6u"#;

const SCALE_3_D: &'static str = r#"eJxNzEsKgDAMBNCrhOxFUkil0PYGHkKiUEFBiojeXm39dBXImxkrY5RpANkdkkGQI9/oUKG3dWZv39jF/GjqcBlaujVA77BlaDZSgdLE/f1tZiADutIfna2bJS8="#;

const SCALE: &'static str = r#"eJyljbEKg0AQRH9lsN+Ns8t5F7j4B/mBdMEU1wQExe93bazshGGaYd6r83dt+L26PwdEXAocZRItWYck1KeB4uAipnRRT+LCTzfWx/Ed60mwm4B3hrGxv1ho8I3lYnHkZpOhR7iQxRA6pOigNTsfO3vQOMw="#;

const SCALING: &'static str = r#"eJxtjDEOgCAQBL+yoSfmODFcclLT+AgTiystzL1fbaCh3dkZvc/HcO3hSASGgHKouvxr1c4ogRs7FaPiUWaPDWzZp/L6RZt4HPAFjH8g1Q=="#;

const SCAN_FACE: &'static str = r#"eJxtz7EKg0AQBNBfGezXuKPHRbhYp0mbXrDYUlDu+z0t5JBlGViY10xa592wfJpfj/gPM0F05RQUGpspvU4xpdtpRG+sIZg9SIXGXEkpn4lHI6jfUEthduUbOmzahiKHM3LFgSNGazv1BoRHdQBC+EI5"#;

const SCAN_LINE: &'static str = r#"eJxtyjEKgDAUA9CrhO5Ff0T+Uju7uLoLChVEHKTo7W0RpINkSALPHdMZMHdmaKBjOxFEnSKgZaDxrsrCu8+JogksIRj/IAWisZA2rWD/qILSt6W0jKXc1n3BLZ0RGtx8+0pfU+Wr2WblH6zMNG8="#;

const SCAN: &'static str = r#"eJxtyiEOgDAQRNGrTOob2GmaNaUag8U3QaxEkJ6fYsiK5psvXrnbY7i2cCTomRtBrCMBI42hluUTtfxOFMnoIdhnkALR7mQcZ3FGFZQ9exnZnXwBlDYpaw=="#;

const SCATTER_CHART: &'static str = r#"eJyFzV0KgCAMB/CryA5QDLN8UG/QIWIFBj2ERNTt8yORiOhJt/3+m6LZ0TIxOjV0lQDmNISHjlQbVSdhVJZ+gjIanxEl86Z5l3eIeWv8fdv7bgq1fxSLbZ52HTbLRg09Z3xHaVGGaeiaC0AbQ20="#;

const SCHOOL_2: &'static str = r#"eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRB9v3dKeL6bak+QrpOTgWB4stB+lli9wYt188xvv0yHAZ3FEEhE59Rj9jQI8B/EGUZTId6DN2ZQ61gHIuYSWkigrAe90Qr6NdA3HfthXW9HkQUez+vdB6KdU/iHY/6xvzp0kX"#;

const SCHOOL: &'static str = r#"eJxtjsEKwkAMRH9lyH1xE2KNsLtnL35EWQWFClJk0b83VtCiJZdM3syQdO1vJxwyXRQdLCgMSiWtXveSPpQNHKGQZr1AEH04+LbTuQ7Sgg0a5L9iz552/PXHyR/n2vt1KWpYN94skO4H1PNYhyPqI9OWMGYSQr1n4umjNy1PN58++Q=="#;

const SCISSORS_LINE_DASHED: &'static str = r#"eJyFjkEKgCAQRa8yzD5rBhMD9QYdIiwoKIhoUbdPiyhCaPP/4n0e38zN2kNrsS6FZKhiaCBGZ/KInPHD4scOFouM4DeLMtRuUcfJBZ25LRNJUJkWpYYY/xZSKU1NhQgv5BlAL8+zUOFln3ECMX/QAdMCPhs="#;

const SCISSORS_SQUARE_DASHED_BOTTOM: &'static str = r#"eJxtTdEKAiEQ/JXBd6/bxcRA77mXXns/LDAoiCOi/r5V6RISl3F3Z2fG3+dHwimogwHzzGCM8kiz5qP5zZA5kW0X4Ge70NKpyW+y4eRXWxrFd+96jBUm6UYUL0u8nhHfQTmF+CrfElQ5qWSj3w2GDCoSS/UyzOBQIddfVDH/JuVcsr2sm6itdsPWWRRcjT6LelHB"#;

const SCISSORS_SQUARE: &'static str = r#"eJx1jWEKhCAQha8y+H/dNVxxwbzBHiJMUigIEarbN6NQ/SgY3huY780zybsMW8saBmktVjX4OISM64fBEvscymrNmwLWuJjc6CEV1mFEo21kiNSjNXOXA/Qt+/+4FBKqigaHKLpeGCG5hio0J/FUJdRd14Rp9dL8qxUUPR7tvHc/pQ=="#;

const SCISSORS: &'static str = r#"eJx9jUsKgDAMRK8ScoBqa5Us2t7AQ0gUFFxIcaG316b+VkKYLOYxz/EUeR6Ad48NAm/yoscKgytyGdzSrSP0HltS2oCEXGJS9yFMCfYiakX0ErcoG5JP049JW0UgcS6a8tk5AKqzLzo="#;

const SCREEN_SHARE_OFF: &'static str = r#"eJxtjDEOgCAQBL+ysSdydyAUSG3jI0gsaEwsDO8XGiWRbLGbnWTCle6MY512EshmEoOhW1RdhfR31OZMS38oLkqmGOZmieF1eTBlPwDEIFfMn5zMEGVhB4gcBD16ALRLLkM="#;

const SCREEN_SHARE: &'static str = r#"eJxtjDEOgCAUQ6/SuBP5HxAGZHbxECQOLCYO5p9fWMRE0qFNX9p45bvgWKedDMxmM4Ohm1RNQroX1bnQ8i0UizJTinN7SfH9CmAqYQCIQV7sn5zkEeCUG408THHS0QO6wy7K"#;

const SCROLL_TEXT: &'static str = r#"eJxtjLEOgCAMRH+lYW9sK0RMkJnF1Z3EoaOD4fsFE4kDuenuXl648q1wbmb3IKwsWUCA3ghKQUlM5VsZGC3Q4Xqn1sus1sQwNVcM3cgr8NLZGqzGNCQdeEU3flj+1wOvSiwV"#;

const SCROLL: &'static str = r#"eJxFjDEKgDAQBL9ypD+8WyMqxNRpbO0DFiktJO83ETzZapZhwpXvQufm9oWgRZFBIHkHRmUklfq9Ssqe5JiMpXMdi3cxDL0VgxV1JZ3NbeNWTL/5AA4dHTw="#;

const SEARCH_CHECK: &'static str = r#"eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf29qoSx7WGYZvuZ7g0XwzEAE0ZNCQuWxAmXbix0r2CNIhFAEM4K9//JTw8rdEt1BIQ1Tbdd8iLkbaw=="#;

const SEARCH_CODE: &'static str = r#"eJxNjFEKgDAMQ68S+j+l1Q8H2+4iVVBQkOGH3l63wZS0BPJC3DGeCyZPu4U1giwKrk15cJVyh/fEZH1c16jbDL08MRP0Lh49DalU8G9GGMKmb7r0deYBSCcjhQ=="#;

const SEARCH_SLASH: &'static str = r#"eJw9jEEKgDAMBL8Scm8lrYEe0vxFoqCgIMWD/l5CoYdlDzOM3Muzw1rxohwZSuTAwKgyOVCxo9m5gb0ViRDs698qFpc6VhmVRJAozDH7RuYHlqkbkQ=="#;

const SEARCH_X: &'static str = r#"eJxVzMEKgCAQBNBfGfausdqCB/VfYgsKCkI61N+HCFKHYQ5vmHhO14o50cHeCoIVIxDKcaiQY+fQFD/Vrei+QO9EzAR9WpdEoY4af04cw7EZra/pNy88PCOA"#;

const SEARCH: &'static str = r#"eJwlijEKgDAMAL8SsqukOjgk+YtEQUFBSof2923a4bjhju2J9l4QBXcEy4JEzaVbeRlZ+T/SDafgFwgCTdu8On540QrMxBNp"#;

const SEND_HORIZONTAL: &'static str =
    r#"eJw9irEJACAMwF4p7iJVEAq1H/iAm+DQRXDwf7SDEjIlvPpWGMXNBAb5KyB5ak44WBV+T82AUTH/cgD7URB8"#;

const SEND_TO_BACK: &'static str = r#"eJxNjMENgCAMRVdpuoCWkOgB3MAhjBDLzRCCur0UDpAe2vS//0z0ZwL24eJkcUV4LZJG+Np6gktc/7EECjczSWEztdbTQdBAEahqG0r3kRicxX0B0pkOBQrmOuViElCQASQNC3eQBMwd/AHPWTMs"#;

const SEND: &'static str = r#"eJw9yrEJACEQBdFWPubL3e4Jh7DagQ2YCQYmgoH9owbKhPO051FRvGkiEPohL1lyK5tM0GfvoAfFjcAM/u6cOUoQ/w=="#;

const SEPARATOR_HORIZONTAL: &'static str = r#"eJxty0EKgCAUBNCrDP8C9X8hLtTbtBBEhVro7fMXLQJhYBh441LMB5p4EiZ09sQyWt5uY28U3KIquFpSf3wtMV+nJwsLFuxgA6vuExM7iEYgq77Mj9/3zyaG"#;

const SEPARATOR_VERTICAL: &'static str = r#"eJxtyjEKgDAMheGrhFxAE6V0SHsbh0JpCzq0tzdRHIRO/+PxSU7lgM4BiRE6vR3aTaM3E0ZZTEVpNY/Ht5rKdQb04GEHYg05c5+YWHJqgFfjOvyP3/fZJoY="#;

const SERVER_COG: &'static str = r#"eJx1kMEKwjAQRH9lyX3XbJLWFFrPXrx6L1GIoCBFiv69u9RDKik5hGRm3mTTp9uU7ldIn8GwM5Deyz4NxptDv1vkQ/8cXxkugzkFaoDtMYwOHFhZjA7duTiDnDO35QW4eRUAl5Ea5Sv3nx4KusV12CqspFutm3GVwC18C20my1WFY116cEN7YE8BqUPyFUdHTv5E1Q2DiFIgHPLiqRn8z4BSw3VChE7jdT2ShwUijlo+6BAK2TLIgBC1wBcv/AKAfocs"#;

const SERVER_CRASH: &'static str = r#"eJx1jbsKgDAMRX8luEeTWIJDdXZxdS84dFBwkH6/7SIpVDKEPM65/g5PhGPuNgWm1QUBAcrFKCi7mSHPkdUuQFIFgESUbvFDkS7eqp1RE9YkFZNVU8lKWBH469bYE7dTp/bp4hEUXSb1LO37eAFErUAW"#;

const SERVER_OFF: &'static str = r#"eJxtT70KAjEMfpVwe2oTm16HeouLiw9x4JBBwUE6+PQmHlwVSijNz/eT1Of6UridpusMrHRcGRiiBdnPLfUaLVOUaakH5yx1Z5JNI3AQf2cn8FaBtC4Yvav5PRBgBpobUscio5nRAJuA0i/ub8nNg3KQO2EQDyxYLmnkmoGKhjgweXwvMLG4zz5i60de"#;

const SERVER: &'static str = r#"eJxljFsKgCAQRbcyzAIqJcIPdTMlKUQfImS7b3QEi77uPM65Oro1QcwGJUK8a3gXdp8MKoQrbMnTcUJgpBJWj0WzmmW2WsfPeLURKOZuH+F0kKXBZZgEfQVNFLJGrhuxhWpsIYRipGR3v/QDeJA4sg=="#;

const SETTINGS_2: &'static str = r#"eJxFizsKgDAQBa+ybC8aPwRhN7WNh5BViGAhwUJv7yZKUg28mUfncnlYGee2AeurER3VcXOUjenB2GkoRvYgxwaBsUOQh9FY5Z2o0adzFoPfK9KpRC8fhyRa"#;

const SETTINGS: &'static str = r#"eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGdmiXpP+X/G13PdzPcOzdjoVEQM5IIRx0A038UHcP4s0vwsjAtPaT1nmStkjoqlFukZpNqaDV+YAI+SIDGoWYnfQQ8aIECaHcli03JAI12wKwyFjAjLZNcVwnO0DG+r00pYRaUjkCgvLAypLYXMIEj7ViwsR+bVf47RfJm1GSX/8+KiJCgqJbSw7fAbe1t7NSotgbwZlSQ/Id2jky2++rh4HydkO3ig9o6MbLbZxOML56x+Lg1jvvYHymPy3K6eEDOjV/Ow=="#;

const SHAPES: &'static str = r#"eJxVjUEKwyAQRa8yzD5TJyYxgnqC9ALdBSNV6KKI0Ob21bSLhtl83rzPN8+1RNgsXmeSwGIlRQpEPe5o6qeOSSi9MNMA8u8JTKznjsQgF55qdSZ96o6qb84Nnbm0DWdy8AV2izwgvNJWokWFkN+VIMSQ7rEcpALZWs13xqfsHwFyhTQi+KarI+2/VNWv5D4CKjI9"#;

const SHARE_2: &'static str = r#"eJx1jE0KgCAQRq8yzAGssewHtMtIC0FauNLb56iREO2G+d572rpg/Qk2GqQNwSaDCiEYnPDQQ10P3VFLWwtK8od6WrR/Y95dJySZx1XMeY9kcBOKL34qMUuElJ80CUVsstF7Y/EYWZgoXitQLbzaDZhDO3c="#;

const SHARE: &'static str = r#"eJxNjEEKgCAQRa8yuI9yKHFh3qBDCAUKokIievtGaxGzeMPn/a+SyRbOnR0rcCzSICAs4+izHP/BhGWSTKu5l7RK0TfvwgUpupDvnXEBglaoIEEM7zO0Gl5FcpBB5S8bsaPHW/eH+wDuTSZQ"#;

const SHEET: &'static str = r#"eJxtzFEKgCAQBNCryF4g1ggKtMuUpBB9iJDePnddoaCvkfXNmOi2pLKFEdQd9uQt4AzKu3D41N6FP2M1ukahWM1AvdWc4XKqaAtLdciRkX2uR40kyYiUI1OcgJuUrfO1fU7GpZn5+ivfhEfbeLcPe008nQ=="#;

const SHELL: &'static str = r#"eJwtTkkOgCAM/ErDHXRKo5ggP/ARJB48evD/sS2kzXSbybS+/XvoPsMFIaAzMa0EQhStQoY2F8VNw6cIIxVfOtn28Bxa9gbpyBPHgZNw3BObybShnFStBwmtLvZM+wELIxuz"#;

const SHIELD_ALERT: &'static str = r#"eJxtyyEOgDAMRuGr/JkvtNsYFWU3wOIJiAkECWTnJyAIAvPMy2f7fBasgxvFw/tDKUJJeOo2UgqkCLVfGAkK4ScuW3urbF+rNf4PSaVhed8FUzEdSQ=="#;

const SHIELD_BAN: &'static str = r#"eJw9yzEKgDAMRuGr/HQPNmmqGWJv4OouOjgoCIrnFwW7vOXx+TFdK5Y+DCwQOY0URhzHvJFRIkO6uzmihYHjl1C8eVXx3+6KDFaw1PcAweEV5Q=="#;

const SHIELD_CHECK: &'static str = r#"eJw9yzEKgDAQRNGrDOkXk82qK6y5ga29aGGhICieXyMYBqb5PDuma8XSuyEwmE8lgVLwY72RUiRFvNvZo4Ei+O9csiqrZL/dO2T9TkhKfgABKBZp"#;

const SHIELD_ELLIPSIS: &'static str = r#"eJx1jCEOgDAQBL+yqT+4a6GcKP0BFk9AVCBIILyfgkAQalbsZCZs05Gw9GYQC2t3pQZKwmO7kpIjhTu7meGhEH7GxFDfVgyvm29JFcsPytky8192ATAQJT8="#;

const SHIELD_HALF: &'static str = r#"eJxVy6sOgDAMRuFX+TPf0HZcKsreADtPQCAQJBCenzCxZOaYk8+v9Tmwz2ERhept1MNIOA8nGUUyxHfaGCMMwiUhefer5K3NWs8HmPIVow=="#;

const SHIELD_MINUS: &'static str = r#"eJw9iysOgDAQBa/yUr+hu+XzxNIbYPEERAWCBML5+YiaETMZP5arYBvDpAazk9KConHudqEkIdI9rBE9CI0/Qvbmu7LX99VaWMMDhacVjg=="#;

const SHIELD_OFF: &'static str = r#"eJxlTbsOgzAQ+xWL/a6XuyQEKeUPWLtH7dChlRgQ308IggX5sVi281yWLz7PbnID93C+RB6wSxrYSF/hR4mMjBWOtRvzY2+N+ez+FRVSec8mX2ebENb+LYhIcNKsqLCh2fEWOCp5tnTtbO3nJUE="#;

const SHIELD_PLUS: &'static str = r#"eJxtjDEOgCAQBL+yob/IgcgVJz+wpTdaUFiYaHy/aEFFs8VOZvRc74J9Ngs7OHcJjRBim8NBQp4E/ombxQQB239M0uGzkja33lykA2qUQ46NvBjBHNM="#;

const SHIELD_QUESTION: &'static str = r#"eJxtjb0KgDAQg18luF+9u/pzQvUNXN1Fhw4OguLz2zp0kpAMCR8J53pH7GM1i0L1MmpgJLy0Bxl5Mvin3xgdDMJfVFOoMzWFwg5OMKweHpwkaJ0pJGFKqcz+gdKh9NGxlO0FX0MiNw=="#;

const SHIELD_X: &'static str = r#"eJxtyzEKgDAQBdGrfNKvZpOsbiDmBrb2ooWFgqB4fokgWNhM85i0j+eCuTM9Ozh3KAUosR1kJSVPCn+1k0UDBdsnJqe6XDm978ahEkQSyA/GYvjaDbTxHbo="#;

const SHIELD: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Ik0xMiAyMnM4LTQgOC0xMFY1bC04LTMtOCAzdjdjMCA2IDggMTAgOCAxMCI+PC9wYXRoPsBaDmU="#;

const SHIP_WHEEL: &'static str = r#"eJx9kMsKwjAQRX9lyD7RuXWIgaRrN926lyhUUJAion9vkhaUvjYDM2fuPK6P1y7eLhTfQTEUdUHtFcVPyWq/6XHtH6dnS+egGgbhZY1kmIs/dGdHosWgohymDQ1AjFYvqtkVuV6Qp9W8M3IE5uA2L+bKWEuSJs20OJMADivqEoX+zhvZMxiTbUL/xWDQF/jnV9A="#;

const SHIP: &'static str = r#"eJyVT7sKwzAM/JUju1VLVuII3EC3Ll27h3To0KFD8ffXdiBQ8FKEDj3vpPReP088zsNNILzRRCOYBAypUUNf0QmqM4WSM1mp7xP/rwxLOlXZJR3ibBRmiL8w04QGvpkwWF/OnDqDbpXXyBQjBS3xzIgUpy4jONzjWt7aqZw4uf7kkNzdLCf7rP2O5HA0vqsaRnA="#;

const SHIRT: &'static str = r#"eJxVjrEOwjAMRH/l1N3GdtyQSKUzA6zsEQwZGBgQA19PEoaq8ubTe3fLq7wrHqfpasIhIbBHaIQVh0PaKSXIJXC0ERaDjb+QcnAYW3jyPMhjUeg/BOfMyc/xo3JvElbO6KjVtClgZDeVaqzznqUGdy0N7b6Teud3WpdDH7/+AGDDKHk="#;

const SHOPPING_BAG: &'static str = r#"eJxtjDsKgDAQRK8ypF/MJsuSIuYGthZ2AYsUFhbi+V1TBASZZpjPy2e9GvbZLYqACL1ZajDru8y1b0Bh1YMiyeZKnt53yYNh/8bpp2AF+yqQDmFK8GP1ACINICo="#;

const SHOPPING_BASKET: &'static str = r#"eJxtjrEOwzAIRH8FZecaqF0Hyc0f9COidvDQSh3y/wr2EGVATNzjjqv/bW/0eU6/TCKUuExrvXVxrScSc8Yhe6mjpnPgumNECh5UkDYlpXmMdq0ZljfMV8HCKKTs6ldQ2K+DOBthZEGH1D9l5CY5at9r8NV5AFOAPuI="#;

const SHOPPING_CART: &'static str = r#"eJxNjcsKwkAMRX8lZN/YXDsvmJm1Gz+ijMIIs5DiQv/ejpVSAgnknOTG8lhKu9OSWJnKO7FfxycxlHM8bTTHv9W5hl3Yrg7ac35VuiW+QkZDvVU0iLWkkAkzCDT+CqRifA3i/GGrEsywAtdUrBmcTOeLEUXP6L/zF/5DK5s="#;

const SHOVEL: &'static str = r#"eJxljDsKgDAQRK+y2GfdbFw/EHMCPUTAwkJBUCw8vdkUKZSBx8B8/BGvFZaxmhmYbyObGIEk5VMFX2sh+FIbUMA2ihb6f77bDjjvUVCiQ3GQQUk2ndJJ2dM3AzIy6bqcvtw6JLk="#;

const SHOWER_HEAD: &'static str = r#"eJx1zjEKgDAMBdCrfNyt/W3aIlRv4CEEBxfBQTy/reAgNHySIS+E5HO9dmxTdwgEzoRa3ZyHOp/zpwt9oWjCKmYMeJut6RNSaz2ghs1TAqbbWLbMglEzD3rNImg1I5xqCRTFXPmFP3sAkGFQiA=="#;

const SHRINK: &'static str = r#"eJxtzT0LgCAQgOG/crRrmh94YM4tre5Bg4NCQ/T7u4NyEvSG4+G9eB13gXOdmnZAz4Nvwgv/WBmaEjQL/SnFmWGKP98RNMqQtWsK1GblwhtXBQUGmspEMjLmIC+w0p1hudOvi2DAdPkCPSQt8A=="#;

const SHRUB: &'static str = r#"eJxljbEKgDAMRH/lcG9totYOVejm4g+4FRw6KDhIv98UUQcJSTjeceePeCasQzUTgzmrflOsuBp9XcjoP97DZe2ChYWRIVCjHdhkM5GcYHWHsjcUdzKxw6NJ3vKP3akFtUq6X3YBbJUjoQ=="#;

const SHUFFLE: &'static str = r#"eJxtjrEOwzAIRH8FZedasE1syc3cpR9RpUOGVOrQ/1ewo0zJgAQc3L36e/8X+jyGl5LkRRBnQaA7KRLDKCCwYFwNwhk2Y/RRSNvyEJ+qw1RvzWmqh99XMilFiux1lj3OPK14WuppBcX9zDu9uu50nPyBOx8rjB1mR8grN9x8jSHxxLEBS9c7Pw=="#;

const SIGMA_SQUARE: &'static str = r#"eJwlizsOgCAQBa+y2R4MfiEBahtbeyPEJbEwhPi5vaDFe8VkRke/JiAfNkoGhUR4DDYI9/dXcIl+HDOp0eqqBFYfSyJwBifRg+RqHka5t9CxPJInE1wVt1j2BW0yGws="#;

const SIGMA: &'static str = r#"eJyzKUgsyVBIsVXyNbRQMA8z8TDLMVOw0AXiDEOjMl1jJTsbfZASOwDmBwru"#;

const SIGNAL_HIGH: &'static str = r#"eJxtyzkOABAQQNGriN42kVAMN9DqJQqlQpzfUtBM+18+9jIaq4EnYKCb1IZHVCdGfOQ2TWEJMXDJU3Su/GUBNKMdsA=="#;

const SIGNAL_LOW: &'static str =
    r#"eJw9yTEOABAMAMCvNN1RjcRS/YFHSAwdDeL9WLreyRrbYDbsDEwWKaNK+qjiVV+dUHwuj+oO2g=="#;

const SIGNAL_MEDIUM: &'static str =
    r#"eJxtyakNACAMAMBVCJ6vIQFRugFDkCAqEYT5eUQV9g5Hm6x60RUUeLY+aEJ3kVAqnVomfibAqyy1AYslFmk="#;

const SIGNAL_ZERO: &'static str = r#"eJyzKUgsyVBIsVXyNVIwMsjQMzBUsrPRBwnaAQBpcAd7"#;

const SIGNAL: &'static str = r#"eJxtyysOwCAQRdGtTPBtmRfSVkzZQS2eBIFEENbPR6DG3pMrJdZM6TM/CDaflo2Xa0Yvm55B7XCKMBa9Gs0raAKQa3xv6ZzTJRQ="#;

const SIREN: &'static str = r#"eJx1jbEKwzAMRH/lyG7XOiI7ATdzlv5AN0MHL4UORd9fu5TQgos4gXT3uPwoz4rbebokCItCEdoI1KmF7xtqcU/m4nXa8qljWz5gBUMh+AnTscrPAzTuao4jmtK6qwwcWbxibpIFOiL/gr1xYNxnv3LFe/sUUtcYF1o8nBebI0wl"#;

const SKIP_BACK: &'static str = r#"eJwti0EKwCAMBL+y5ANtpD0Imt+UIkgitAf9fU3xtDswk5rVcZuiWdH3ycQRYUcEB8x74GeStC1RUi16oYdMJ2GsYQ8JnR2n7I58tDIYzA=="#;

const SKIP_FORWARD: &'static str = r#"eJwtykEKwCAMRNGrDLlAGzGLgnqbUgQxQrswt6+im/kwvNC02KMVTXP93kgCDxawg8CdYzylcGyVQsn1RudIfBHMrfZdG79MPlX6AaLAGJY="#;

const SKULL: &'static str = r#"eJxljV0KwjAQhK+y7PvGzEpqhKTgAXqIEoUIClKkqKc3qT8RZB7255udDek4pdOB0j0ylGkqhSndIm+5D6sX7cPb1Sgcf25+bJfxmmkfefCkdtbsZ1l43Td6hhpH2IhxAqltxuPfNqArKaOSkl0E4zpZG3Xki8pMVgCDwnfNVV9/w56j4Di/"#;

const SLACK: &'static str = r#"eJyFkEEPgjAMhf9K07tIhSWYbCTeuHjlToQ4boYsOv697QxoNgmHHrbX9ut7ehpuDrxBKhBmgyeE19g7a5Dfdhjv1hmsECZpyRTW+igTtX50zkJv8EpnqDLVUm5Zv3CBVA4kFTQZkvZaL7BqhyKXUPkHxsuVwMqmSGAfLYIty4uVGGNY8Rs02cceGiqf3N5F3jJ1oIT3hfyQ5/DhQ7rbQXJSoBrK29RakFbSGzPZcC0="#;

const SLASH: &'static str = r#"eJyzKUgsyVBIsVXyNTJSAEMjJTsbfZCoHQBr6Adc"#;

const SLICE: &'static str = r#"eJw9jDEKgDAQBL+y2CfmsnJGiHmBfsAuYJFCwUJ8v0bQZouZZeKRz4J1bPYA6YxCy3AZNim21aT4+VmCZQ9aJep1Izh5qUwflr0Vj3ccBM7QcPkrNzZ1GKw="#;

const SLIDERS_HORIZONTAL: &'static str = r#"eJxtkEEOgCAMBL9C+gFtNYYD+hsPJsYz/l5aVgnCaVOYbrcN53HtLvJKwuSirMQzuTvVKqKyhUGhLRia34ycyBp5fBtq0swk/5mixrAWhp8vKFo7vsvnY7AgQ9JfXMYnoIgweV672lKOYCiOIR1b3VtZX8eee6inL7YlwRlS/cIP3u1ibA=="#;

const SLIDERS: &'static str = r#"eJx9kEEKwCAMBL8i+UBNFPGg/qaHQulZf18lUWyxPS3GyToYzuPaVaYIFlTBCISgMvKxTtFCCluDUpjRjtRAzah5k+2OpKZmlpRXnjAXDGbs1vQLFB0zpHmn5bK3Q2WSMWNnwQpqmXEfv9CGXnRt12Rr/aOLXuqdKNCAb99wYmw="#;

const SMARTPHONE_CHARGING: &'static str = r#"eJwli00KgCAQRq8yzL5SMWvheIMOESmN0CJkILt9iavvh/d8SYfAk6MwobYIlXBGeAkNQqk9+uKUT5a/Kgx+al7w9y4MkXDTZnRugRW0Am3YXkM/bGMbFT4QyRwi"#;

const SMARTPHONE_NFC: &'static str = r#"eJxti2EKwjAMha8ScoC4tF26QbsbeIjiht0/GQX19qYtoj9GyEte8r5wbLcCr4gG4dDBCM99LTmiR8jbfs9Fj/p8RxRcwqXml/BIJcMa8coWJrImeXIWmgxarO3JNqBG/wEhJyBkODGTF+j6pdSN0xk208zgSKmxbl07RUPzP+wDtFEzzQ=="#;

const SMARTPHONE: &'static str = r#"eJwdy1EKgCAQBNCrLHOAcqWgD9cbdIhIaf0LWahun/ozD2aYUPNppLlcagLvQE9JpgJeQK9gBdWGb3yDkTHM/RfDfZhSEuzsiTedHPett/EHi6sY6Q=="#;

const SMILE_PLUS: &'static str = r#"eJxtzt0KwjAMBeBXOeS+syesw0K3N/AhBgoTRARluLe3qay72CA/kHyBpNf4mXDt5aIKcuZIjxy5gC46ehnSydCQKj2D7ZtNgKK1dCU3+Lg/b1i0lyj4Wms8BQv/g9KyNbW3DAWbYliPqq4vsEOYuoPfGKHztvgBfcc5VQ=="#;

const SMILE: &'static str = r#"eJw1i0sKgDAMRK8SsveTYkGh7Q08hFRBoYhUF+3tTapdDC9k3hh/RB82iBapR/CJqZi50Jnu6525lmeH1eI8Ag03tRoUDJKmRFRRnAnHuUEmixNCVgVJ0PbEV/mzK1Z1f4kr0linsiEto2q/BEsrqg=="#;

const SNAIL: &'static str = r#"eJxtjd0KwjAMhV/lkPvVJp21g65v4K33pQoTvJAhom9vsoE/ICEJyZeTk6/1NuE40l7AoUZEeLClaOk1lrFL2gW288veU8kb05bcznO7nNAeI7EnzCMlQnvqFOxoxSV/GQlPLK139jx1wUWt6bCrq4HZKbnHj8VbywkBPDjG1skfLqJcvBt++AsqpTdL"#;

const SNOWFLAKE: &'static str = r#"eJxtjEEKgCAQRa/ycS81IuJCvUtQUFDRooXdvhmxcOFieIv3/oR9OxdkisooZMNgPkwSUmEKg1Qp/C3VyNTRFzftNd0r5qgOM4KcttqCT7yYxlt4FIeeJYc67Y49+H0JmvULZ1c2RQ=="#;

const SOFA: &'static str = r#"eJx1TbsOQEAQ/JWJfuN2HCE5ao1WoZMolArZ70fDXXKyxWbe4VjPHVtfTHTo5mYlCPecUDgmGLSqGEL5RIbwBaFq9We8P3dtYkJoEjnEw9ldbsKUXDL1Htoac7vuT1HCW/cKFwOLOFc="#;

const SOUP: &'static str = r#"eJylzj0OwjAMBeCrWN3ziO38WSqdWbgAWwVDRwbuL5KKoiLChDLZL++Tx/v8WOh2HM4sJDwbGfn1mbOT7meyyzCNh1aYxnct19bCvpOwIVJThVIvTpBIeoVkMAqiItf/0OTgE4o6mNZZHJPAS91G5AINbRmQW5ikJ/O3HDY57mVY2ey02vqy5Zf979HhE34C3W9WWg=="#;

const SPACE: &'static str =
    r#"eJwVizEOABEUBa/yov/WsxENao1DCIVSIc6PZmaaCbOugR5VsRb0m81AO9EOFIL5b6+N8OqCW6hS+N6VDrfeDkU="#;

const SPADE: &'static str = r#"eJxVTssKwkAM/JWh98TdZJetsBb8AH/AW1kPHjx4EL/fiZRCCZP3JNPf6+eJx2W6VZyHZK0gxOFq9FXrlYiI9LfGzNLIOrNwYVW4b9iYMG2w6MHWI5OZECOJqccnKeLReUkLQ7tPSz+FoKXvsjJPz9+yT36hoiZB"#;

const SPARKLE: &'static str = r#"eJxVjjEKgDAMRa8S3FNNirSF2hP0BG4FB4cKDt4fEzNICQSS93lJvttzwrFNFzF4JJdgdbExMCxSJBuOAbTH6oG4C5Yx/RGDXw9VLExdOA4ao2gaJvVoAAeRYbu4TyXP+lt5AdHHIfo="#;

const SPARKLES: &'static str = r#"eJxtjksKwzAMRK8yZO+0I9W4BjcnSC/QXaCLLFLoIuT88YcQE4xAAunpMeE/rTO+r+5HgRr2Pk7bP6mTQHCPxbgVZ5H7qKAsGUCGTww1Fi3CJRPmoitEkY7C5CuIuQhNDaZ4n24ItxR4CEfst4Vuj8aeHnTNi8LOzQ8H+uqyAzJIQrg="#;

const SPEAKER: &'static str = r#"eJwljFEOgCAMQ6+y7ADqjPELuMwkQkL8ICTC7e3wZ13a17oatVGK+U7N874xvflqybOcTHXAgvQpuAfTtIJbrRec5qolksIVhApGrGEooD8OruQnUt8tXDbBCF7sD5nSZbbAGxc+3ocnpw=="#;

const SPELL_CHECK_2: &'static str = r#"eJydjjEKgDAQBL+ypE90TwkpYn7gI0SLNIKg/8eLkFSpbI5lZoqL1/ZkHIs5PejhLQW6xKQ4FJViDdagOIeOmCHc6YgRei0hbrK8C/k2OhaN1Aa/mvbOC4BbK4A="#;

const SPELL_CHECK: &'static str = r#"eJxlizEKACEQA78S7OXORRaL1R/4CMHCRrDw/7gW2kiakMnIKLOhRtMZjsHWEbSRSfJtlOQcctC5hRd09egHabz1ly8+OBfH"#;

const SPLINE: &'static str = r#"eJxNzNEJgDAMBNBVwi2gKRQRmoIDOIREQcEPKX7Y7U2rgiQQwj0u6JZ0XygJHEgvAfd2s8AjhuZJY3hVyT0+nCv+qWM6V5oFoyfuBnZk29qwvVT7iog3eAEfSg=="#;

const SPLIT_SQUARE_HORIZONTAL: &'static str = r#"eJxNi70KwzAMhF/l0G4qKU1/wPbcpWv34hZcKKVDCMnb55whZDikO74v/p9DxSvJ/QK73voSDBo8GOOPc1F2/mCrneR4aHyOm2Un9LUrlOAwxkfTonSsOfAadtr383tjsiTmgtmTuPKyHwWTrzPZRuUFLtclRQ=="#;

const SPLIT_SQUARE_VERTICAL: &'static str = r#"eJxNjL0KAjEQhF9lSL+4M+f5A7nUNrb2EoUTRCwkeG/vBkEsltmB75v8PL9mXKZ0HLE7jdWNoAkyzfRKOATGqQ2p5FXHS/5J3IObNlQPiV2CDtsar5ssdkzN/rz77XHFoilRCQu/+Y4uj4y+7mynygc3niVh"#;

const SPLIT: &'static str = r#"eJxtyzsOgCAQhOGrTOhBd5eXCVJb6CFILGhMLAznNzQ0munmy5/u8lScqzrIQ6prTuU09TOnQRGyya8Qg7npaKRYWMx9mgwF1mxi4F0g3+wihwVe+0EvvWIjqg=="#;

const SPRAY_CAN: &'static str = r#"eJx1jksKwzAQQ68iZm/Xcn41xLlBt92XJNSBBkox/dy+9qYlBG/EoKcR6u+XGDB5OVWogjaUoT9kb+h/pENTICS6AqqKpIMr/pBb9JjHiLcXNoKPl6RhXq4heqkFr2WKIV8pnoP/ppUODhb2STMa6FbpGlQEg2pHpVsYxeQl4Zm8WWX3c1amPTWOReY27AviBFky"#;

const SPROUT: &'static str = r#"eJxtjjEOwzAMA78iZBdr2bIdA2l+0EcE6ZCxQ6e+vlSHZgkMwYRIHLW8tvchz/v06JLTYWlal1vs1uXvWKK1V1TNqJi1waXoZXSgyoDvBsMsMRmZU6Sgaxa4FjJcnRwQgqxo5IYcWriPOL0qzpokNSo/V1c5TNrWpTPFR4BJNA+lKCSikRFgF1PjNb8i/p01oTtMw3MMyWfHF64iPXg="#;

const SQUARE_ASTERISK: &'static str = r#"eJx1yzsOgCAQBNCrTLb3g2KgAG7gIYwQoTAxhPi5vaIFlc3uZF5GRTcneBcWnzQxSTiCTf6Ll6aecL43Pq8jo5o8MGqbkofVNLIOcpcZclVglfUAxiEq/octBArenngmOA=="#;

const SQUARE_CODE: &'static str = r#"eJxNy0EKgCAQheGrDLOX1Fq0cLxLpDQugpCB7PY1BhH/2328UPMqcJYkTOhmhItwRKiN0CNwLhvLC00hhkEPMRyLMCTC3Vlw1njoqav8fYJn3vQ+vwG5QB/x"#;

const SQUARE_DASHED_BOTTOM_CODE: &'static str = r#"eJxtjjEOwCAIRa9C3E2F6NDEegPX7iYdWJp0aHr+goskms/Cf3wgP+1luA53YwAMnqDLlbwpKXnwCFLku2ZeExA2DQcR6tCZRq9BxmgNoM8afnm27rKWcQH0GUt+dzwxuQ=="#;

const SQUARE_DASHED_BOTTOM: &'static str = r#"eJxtjLEKgDAQQ38ldD/0QjsIZ//A1b3gcKOD+P2ei1aQLEkePNvb4djmtBRQG0GMERUK1/JuxHbN/QGe/SHRUrXhFlZ7tFNoXX+A5i+5AIJlIUs="#;

const SQUARE_DOT: &'static str = r#"eJwli0EOgDAIBL9C+ICpXjy0fAaJkHgiJNbfS9vTZmd3qgsHeG+4I6jYrdGwnAhJDoTXrtAFvgGobkOgyub8CHDSkqZnIHCfLU9rph/7hxqN"#;

const SQUARE_EQUAL: &'static str = r#"eJxVy8EJgDAMQNFVQhawUUEPSTdwCLHF9CYloG6v6Umv//G55s3gEhwQzpJMBWlGuFuob+8RNJddrUHkzofIx2oKSXCZgIJScPH2l/ErD1+XHic="#;

const SQUARE_SLASH: &'static str = r#"eJwdi0EKgDAMBL8S9gPSiqDQ9jNabEA8lIDp701z3J2Z1OsppBkrqFW+m2SEHTT86QYi6ONLmv8lLTMo6eG3koaMA6TR2GZN9DmCT1OnVH5CnRpg"#;

const SQUARE_STACK: &'static str = r#"eJyNjM0KAjEMhF9lyL21CUVcaPsGXr1LdzF7k6X48/YmCh48eZkwk5mvXM9DMVc6ZnDqgSMjBQlxMpFT7skjdzCvub8LEMTJRKiVnRNa+XI4gfe/oFv4n7QtfeBRiTPh+Tn3dR5a6UDY7CEEXdaLDk9s5oP2AmIvMKk="#;

const SQUARE: &'static str =
    r#"eJwli0EKABAQRa8y/QsIGwtcBhlbTeH2TDavV68XZytCcyc4ELfRWRJsAK1Rhb++6EFHmaPRIV/D5Q+D"#;

const SQUIRREL: &'static str = r#"eJxtTjEOwjAM/Mqpu4PtJG2RQhdmVvYIhowMiPdzKRUwVKeL7dg+X3nUZ8P9NFxsxlgTErRDejYRn4rZTSUjrTQNuabAlFQYB46MzsrXnw7G8wRTRJj1hwpsz6SzSTTTYSmH7mApPx8jXHnMArdDlAiyWXX4puziL/F/r5KuaU8q85R7pcQ2yduS9yZnWGxB7dt7A0AdPds="#;

const STAMP: &'static str = r#"eJxtTkEKwkAM/EroPbGzaXYVakE8e+3BW6mHHgXF9zu1WETKsruTTDIz7X14TnI7VpeQlCY0Vdfu5l7XrgwOlorArfgpGed4689BIYRPCvwxjSAsepQBgu+0gAa/DcVLYTHWajmrpczKlT78S7luZaGw93uLMxFDxfLE4OKLqrr6yP2cZ0wPMg9IoqvPiXxVfQPcMDdG"#;

const STAR_HALF: &'static str =
    r#"eJwBNwDI/zxwYXRoIGQ9Ik0xMiAxNy44IDUuOCAyMSA3IDE0LjEgMiA5LjNsNy0xTDEyIDIiPjwvcGF0aD5iWAyo"#;

const STAR_OFF: &'static str = r#"eJxFjVEKwzAMQ69i8h8NO0mdQJoTdIcYbLBBGPvYR3v7uWtpfywkpOf6uX2fdB/dNSNE+h+hAtGeKCLrlJCFhImFWKFKnMHZku6Rig+IwbV6WTmtHjQrRRsIdCDZgX5AYc+TkaR7RrRXlpzr/no/aObRiaNZTEyXzS6btepaaj//lCqn"#;

const STAR: &'static str = r#"eJw1jUEKgDAQA78SfEB0F2u3UPsdEaQt6MXfW7FCTmEyibUc91Yyatnzda6DKBTiOAUYdYEqAtVDWmbKDDGKQYVT41o8vYejae9+ru+MQT7Tax5SHPtjegDvlRuE"#;

const STEP_BACK: &'static str = r#"eJwtij0KwCAMRq8SMgvVkKGD5jalCJII7aC3byxO3897uVW9YKSC6USYVJA9fFJEGPTfko9lSe7W5m0K3aq+jzMOFIFDIvDKS9yKfBYeGDQ="#;

const STEP_FORWARD: &'static str = r#"eJwtijEKwCAMAL8SMgvVIJ1iflOKIInQDvr7pq3T3cFxq3rASAV3hEEfpld2eFFE4e19hLu1eZpCt6r3VTDFkIFiSASu/7keeQAGhRf0"#;

const STETHOSCOPE: &'static str = r#"eJx9TcsKAjEM/JWQe2OTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjxr2nCYTclhw5sMC6AWTnq4Ao1zHKeqgR4qoVY7ff2sTDj9/IbLiSkLwSRziWtBmtJb27J+DQ+X9yN/7z2c5Lu5yg3TOKRVgUENojI9thWs/lCdImMtk="#;

const STICKER: &'static str = r#"eJxtjDELgzAQRv/Kh7vXfBcTLKTOHdq1QzfRIaOg+PtNdFAhHAcH790LU79EjK/qSycO9u16hcLkqdO1shkMKJQnMtDI5jSgtf5acZ/j+V914ZGDXbhkYVcffQmlsJ+lBVOPA8XuReYt6Mmz0ZQ6/k42COY04A=="#;

const STICKY_NOTE: &'static str = r#"eJxNjLEOgCAQQ3+lcQe5gyOaILODrg5uRAdGB8P3KzFR06XJa1840pmxD81MogV2lMRgmBp1t0JuMyBNukcFnMl9C7DipdMyPee1iaGtwhh+Wtjis3/RBdRlG0I="#;

const STOP_CIRCLE: &'static str = r#"eJwlyksKwDAIRdGtiBvoZ1AoJG7GShU6EqHp7muS0ePyTmFzfgT4q7jtCJ6zInAbSWWZPxUXDlCxW6PigZD+RHjtCh3deqfvjn41YBim"#;

const STORE: &'static str = r#"eJydkb0KwzAMhF9FZJdqXZzYhTSQvV27Bzp4KXQofv7K6Y9TyBSMbTjpk3RoeMzPRLdTcwcF8uKVyzOBQM6OUpDYElKU1s9VVfGQ7niGYc04HEqZcfgWu3hS5PjLd/YjKdYCI3PcQLUjWKg2c2y5ibEWCNlvsDZMgtsKWCS3df6lgpshgcp9a2qGWPpWwkdyLNGA6S+NtKfiZBeK3WjcTS67cGvvjGtd2gsI8GyR"#;

const STRETCH_HORIZONTAL: &'static str = r#"eJxNjEsKgDAQQ68ScgE/FFczvYwWp9syoN7eWkW6CSG8FylpdVzKQFjKu7lyIcqpnIkjb261jUQbogwPH6VZHd+Bn/pmPZ7Cr93CsR7C"#;

const STRETCH_VERTICAL: &'static str = r#"eJxdzEsKwCAMBNCrhLlAP5SuEi/TSuNWArW3rwqKuAoZ5g1HfxmpD4+aYF9BSXCAvvyAYqrnDbep4ITjpfQdV9XjYrYZDZuN/azaHsI="#;

const STRIKETHROUGH: &'static str = r#"eJxFyjsKgDAQBNCrDNv72c0iCknqNB4ioKAgYmERb2+SJgzDMPDsE98Dm6OVJ2hYooHBWNJJPxsoeTsU422TCpao0Ao5dw5Tc9d570jiSEZCYkdK+PJlyct1sy3K/6WlHuE="#;

const SUBSCRIPT: &'static str = r#"eJxtjjEOhEAIRa9C7GEHBnZmkllvsJUnMFrYmFh4/wgWVoZAfv7jE/oxnxusv2FXMKhQh7F/whv7Q1jA8BX9JQG3DXVJyGSkigIuUMimYEY5Q4jYIC1I7N2y6+oJabMQM9wjRXnyK0518SEkilT0tpofTuX54QJsriqC"#;

const SUBTITLES: &'static str = r#"eJxtjDELgCAYRP/K0S55n4oE5tzS2tAmNDg0NES/P13EQW453oMXnvRmXOu0e9BkO8UwVxRDE3TVyMB4LENOU8QoJQRdEgh0GVV5m7+VhT06DFGSaXsA+ajPVvwByMYvBA=="#;

const SUN_DIM: &'static str = r#"eJyNjUEKgCAQRa8yzAFMR9ONeoMOERYYtAhpUbcvcxOE0urDvPcYG5YU1hnC4VAQQjjLJocKve0K9nYb9wiTw0EQqMi4yDAfX4g4CKqwOyNeYaqRGaZ7A5pJJdtKmYrz9H+U76ML4TVNzg=="#;

const SUN_MEDIUM: &'static str = r#"eJx9jEEKwjAQRa8yZO+YmUknLpLewENIFCooSBGxt2/TdFOadjF8mPd4IT379HpAH40zkP7REE87zNuGc8Ft+Ny+HdyjuRKD/Cij/FoDtlUiQNzVANsd8qYLijpoUEVP6K3PV/EUxU15j9r4I28OldyBVTpLbuONN3ZP1g=="#;

const SUN_MOON: &'static str = r#"eJx9TkEOgCAM+8rincnmopAgP+ARJB64kHgwvt+AHkwE07SXtuvcHo8E2zoEYjCR0UxQRVfIAw0EpETJ4N1YKt69i3xyx9AtJwtaKCSUwkaCFlzgll4mMBCn1jDrjpNnnOpV9bNskcpzn8gFd6xNEA=="#;

const SUN_SNOW: &'static str = r#"eJx1j7EOwjAMRH/l1N0hF5ukSKF/wMpeiaELEgPi+3GWdnEHe3n3rHP/rN8Nr/v0YMZtVSgy6JNRp6VfBl76Hipg2RgAGgqfGpEMOwOFPwmuvTVVA+ekNTVJLUhYUkP1JSeJ0ci7zoHLBhP/NEIuNcSsELyKDlMO/gcsEFYT"#;

const SUN: &'static str = r#"eJx9jtEKQEAQRX9l2ndjZ2wrtesPfISWoihJ4u8xPGl5ubfm3KbjQj+HoYXZK6MgbF4Rn71Lly69cemmeumg8aoiBl4FXacX0DEyGiwykCA0JBFZUY7WwpPfu4qBuIsJsP4go8XM3J+TX4MCdS6ikdkB4dBOsg=="#;

const SUNRISE: &'static str = r#"eJx9jrEOgCAMRH+lcQfbShQSZHbhI0gcWEgcjN8vYHQRzSU39F2vtVvYI6xz54mBD90525eRszdISpoBCKtLRdXeMc9AOnIL4AdJZCROV7X4rc6v8dJq0DCCEgqyGns05suhUCwSGvBJnYlYQYE="#;

const SUNSET: &'static str = r#"eJx9jLEKgDAQQ3/lcG+9O4tWqM4uXd0LDl0KDv4/Xiu6WCWQIS+J28MRYZsaTwyEKzeza3M2u5sko8dOWHFtqNi75mVvI9cAfpBEo8bhula/1wzMS/Whh14ZMEpUGQomG4QDZikL+LRO2T9B0w=="#;

const SUPERSCRIPT: &'static str = r#"eJxtjj0KwzAMha9isutVlmVbAjc36NQThHTIUujQ+1OZQqYgENL7nn7GZ/se6XVf3pqyJyNb1nGb4jpOlCUYXbKHcMpykO5MGRWqQuFGJUF9BjSUoimKDmYJF7QLIXeCl2hMY058E2Su6Z95Rixo06Wl7ZEFUpzQi4WsBHOPM9zOl35fri4U"#;

const SWISS_FRANC: &'static str =
    r#"eJxtyzsOABAQANGrbBwAS/yS5QZavUSxpcL9g0alnZeh2RfDyKKiBoPNchSF1K2FnnlAz+kDZ0rScXi0AZPsFpU="#;

const SWITCH_CAMERA: &'static str = r#"eJxljs0KwkAMhF9lyD3YzFp/YLdnL169l1VYwYIUEX17Yz1spYQchpkvk3jvHwXnJEcz2P6w7gmi8TGl8rStGq5LK11cfZkuVjKgLX9B8GnN/BJYdIbm65hvF+R3EqNgTBIE+TUpD/3s2jDYDqQGDfBdfjBsvGnyUN0PoEk2cQ=="#;

const SWORD: &'static str = r#"eJxdjFEKgCAQRK+y7AGqtTIC6zZ9CKJCfejtW1eF6GN3YGbemBhcdtZfEIP1z30gLcMKtPGbQfPVL0aJ8DRjZ04jZCKmZoSkWHeETE2V+Ax8y2yqqYaipayxjuhfObfF1BYFpgqx9vIL8XUyXQ=="#;

const SWORDS: &'static str = r#"eJx9UNEKwyAM/BXxA7ZFbazg+jd7KJS2sD20f7+Yc0OG7MGEJHfJnXnflnOZ14fZt3l9Pe+WwmUwFCV4w/IQtVFGdsrXD2fKyjycsJI1Z8leMqE+SGshtODSZKskd6tgBlnqDjhVMAGsGRe/4L4JLpJHEe8IgWEjdV3ozhG3UtUVYGL4kaWziFkEI/xx4OuMmq9x7dY3wfNjzA=="#;

const SYRINGE: &'static str = r#"eJx1jjsOg0AMRK8yoreD8X6wtOEEySGipEgBEgX3F14KKADZcjFPnpkyf5Y/fs9mkh4dAkIzlEcVh3KgjAwlPaO3GAw9Z4ixfkkg1HH0qxzQjsTJ1/VtKmkreUlEvAgyiNx0cE8jhV7V8w8kpB2tcwA2Eg=="#;

const TABLE_2: &'static str = r#"eJxNjbEKgDAMRH8ldA82UUsLtbOLq3vBoUvBQfr9JhWk3HK8O+7inZ8C12aOAPO+ZgYGq0JxbakOXSH7Y4KOtdzIV0FjaoGRzyAp07BF2PHX9ibFSU/TC8dLG+Y="#;

const TABLE_PROPERTIES: &'static str = r#"eJxNjEsKgDAMRK8ScgGJpaCQdu3GQ4gtpjspwc/tbRG0qxnm83hfVCA4nMmCOWhAz13NPOe4KlwODUIu0iOcKag4LCOQmDbR1991U2714PkD9gTjZH5eW5Btmge4cyUj"#;

const TABLE: &'static str = r#"eJxNjFEKgCAQRK+y7AVCJSjY9QYdIlJa/0KWstunBOLPMMxjHl27CgTGzVhwt1nQ09Q2TzkeChLTKcpYATwpqPw1F0aL8DI6hNKy3trBUxc6WGX0DcDMI/kA0YIlbA=="#;

const TABLET_SMARTPHONE: &'static str = r#"eJxNjEEOgzAQA79i+R6a3aZVDgk/6CNQQV1uFYpa+D3JBZAv1lietEzvgv88FssUT6yZd8Km+WOlkkBsmZFY6qDs060d+vQdimHMfD0QBoXC1wjUqYleAfQnzxO42sxpF5qqSS6qCInWeTmmHZb5J7Q="#;

const TABLET: &'static str = r#"eJwljF0KgCAQhK+yzAFKl4ge1MuUpBA9iNB6+1Z9mh++GVfiWUk8NlDzYFCRKTN9+arJw+6gFPOdqrYGwa19F9yT30jCCvBirF5YtQdIuuq88cjKdzL8cgAc/g=="#;

const TABLETS: &'static str = r#"eJxNikEKgCAQRa8yzD5zSrOFeoMOERYYFIS0yNuHU1Sb/+G9Z8OSwjpDyA4NQjj5kkON3ta39PaJGHNDpc38v2wfjwiTw4EaIBNJFlngp7ZWqA5ICq3ACNlXZd7sAkPpJws="#;

const TAG: &'static str = r#"eJw9jbEKgDAMRH8ldE9MY7Ut1M4O+gNuokOHDg7i92sQCsfdcI+7dO13gXMyqxWQWR7LNZJEUDsouk8g5IJmT06A60hDQDXt8WdQGWBUZNGpzeTU6XhO7cKDL8S2NS8qIx3l"#;

const TAGS: &'static str = r#"eJxljTsOwkAMRK8ySr9mvZ8klpbUFKGloIugoAgSBeL8eIREisi/YsZv2mt5P3A/dmdDPaXPsPaSDFw3seKNJGXkzVIS4pqljoGLevh5Aj2IgZbZQdduageSp/bn9zCJerG99NSK6pGZsziKkYhe6uPMWQfo9vgFD8kqfg=="#;

const TALLY_1: &'static str = r#"eJyzKUgsyVBIsVXyNVEwKTM0U7Kz0QcJ2QEAXiEHNQ=="#;

const TALLY_2: &'static str =
    r#"eJxVybEJACAMBMBVQhYQIQjCmw0cQrCwtBDnD2kCae9w1zu0B08h+bWxojgpInoOA3CwDm4="#;

const TALLY_3: &'static str =
    r#"eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnvcVh9TxqVm5IeyWxILxk+lAjELRdMKxXT"#;

const TALLY_4: &'static str =
    r#"eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnveFh9TxqVm5IeyWxILxn+KNGQkIgzF+gzHT0="#;

const TALLY_5: &'static str =
    r#"eJx1y7EJACAMBdFVPi4gCSIKMRs4hGBhaSHOLzbp0t7jZI+zMFvoCelSDirxJxWD6gG5C7kPMzIYVMweh1QkoQ=="#;

const TARGET: &'static str = r#"eJxtyz0OABAMhuGriAv4GUzlMo1BYurE7RVNiJjepH0+wEJYs6KondUKG9dz+2oCs/8JxD1g7sKH8flCMjpsAMGuIVU="#;

const TENT: &'static str = r#"eJx1y0EKgCAURdGtPJxLPu1Hws8duIiggZOgQfunHASBOr2Hq9d+FxybyYzwDnSYTdKp5qQfnlItWi6t5fBa4dq56EGxATIy/O0BP0Alhg=="#;

const TERMINAL_SQUARE: &'static str = r#"eJxFy0EKgCAQBdCrDLOPGA1qod6gQ0RK4yIIGShvnxohA/PhP765NmHwFs8ZiEAN7dCZsYIzP68FSfPUJYVdgEM8WCzSgpAtaoSn/VRCIdzRC3+acm3Kuu7cC6BSIKk="#;

const TERMINAL: &'static str = r#"eJw1y0EKwCAMRNGrDLlATbCUQuJtuhBEhXaht68NdPUZeKO9lVlyvdBbrs9tFMEHOIAZETsl3X6S1OFgIxbCECMJhPntc1W86+D4BbC6GNo="#;

const TEST_TUBE_2: &'static str = r#"eJxlzDsKgDAQhOGrDPZZMxtZI0RrCz1EwMJGsBDP7wO0UAam+eBPa95mTG0xKlHDJCqUwphVYsB9/hxdkKZx4rn7L8G7amCNUHSpvHpdeqoLDQqD/WmkgtZXrxyfJSFF"#;

const TEST_TUBE: &'static str = r#"eJxtzaEOgDAMBNBfaeZb1mZliLE/wOLJEJUIwvdDCZnC3svdlWM7DfY5LJxIQS7OpC0CU0ImBiFFcSC12J4oQfTAET9cJdQy+FAtfW7ykuUfeY94NNSON2E+ITM="#;

const TEST_TUBES: &'static str = r#"eJx1js0KgCAQhF9l8a65mz8IJnTr0rW70MFjh/D5SxChMpY5zAz7Mf6IZ4J9YqsDymiFnkloKJL3IZjiKMtXrACd0Bux4IeCCL6BSFZSfLzw6rL85kVd1giUbCdH9VM4QLOo/io0ietWXcWDPgo="#;

const TEXT_CURSOR_INPUT: &'static str = r#"eJx1jbEKhTAMRX/l4h5ek9hXhers4kcUHDI6SL/fCkUqKJnuzTlJ3NNh2KZu9eiNk0LhyjC0Tk2kxt0cfxc/x9tihTijRqSCoknQxb+IHvxf+iSQCgpJpqZAKb5eDhZaEJIflyBG4cUcETK7e3ECuCc7lg=="#;

const TEXT_CURSOR: &'static str = r#"eJx1jCEOgDAMRa/yM9+wLg2Yshtg8UsQlQiy89MhKIZ89ZL3vp7tMhxr2nhBKUbcBILsYxKSfQ6Gs3GqOo2o6ps+ZYR5iJ1+zI/oj5Ae3g1sVSJx"#;

const TEXT_QUOTE: &'static str =
    r#"eJx1yzEOABAMQNGrNC4gJcFQnbs4hMRgNIjzY+nE+l8+jTo7tGwKRgjiDZO9iUnBIaCT9JH0FH+WFRQ2v+Acqg=="#;

const TEXT_SELECT: &'static str = r#"eJx1z1sKwjAUBNCtDPkvOjdNayDpDlxEQaGCiB8idfcmtvYBN+QjjzmTkPDsXwMu0ZwdbC8QHPOo0sp04ZDTLiyGfoMIHQlBv6rCVQ7CLao0lJ4bqJ4L1YB1oZGCQsXCv6l/Qw8sWJca++R+e1wxSjR0Bh9Gc0qT/KYx7doss5lljij/bCq2U5Gi2WZv7WybxX4BZVt+Qg=="#;

const TEXT: &'static str =
    r#"eJx1yTsOABAQBcCrbByAPOJTLLXGISQKpcL9g0Y02hkedXZqURR4chLZiMTqYOJbGgT9OVi5Nzy3ALf9Fo8="#;

const THERMOMETER_SNOWFLAKE: &'static str = r#"eJxtjbEOgCAMRH/l4q7SUokkyB+4OriROLCYOBi/3+LAIkPb3F1fG650ZxxLtzKIM5kuhrF4MdTEQx5y/+C08LCwvVYDI4bTBa+9BTvQ/LE0TKUaF9iUz2aYJAkEBgTqdW6SGKy6OKr3yr52xjTn"#;

const THERMOMETER_SUN: &'static str = r#"eJxtTrEKQjEM/JXj7Y1Neq0+qG92eauDW8HBRXCQ9/2mIiJYQi6Q3OWuPtrzhutxWtUwN4KIvYJhL3la6q4TlvpLS5v9H+5FCvQgDCqE90BrEdw0SubHSKHB55nNYG9j7fvLQOs/02nguxZJ9KwOlDkhO3xZL3hzN9M="#;

const THERMOMETER: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Ik0xNCA0djEwLjU0YTQgNCAwIDEgMS00IDBWNGEyIDIgMCAwIDEgNCAwWiI+PC9wYXRoPrArDeE="#;

const THUMBS_DOWN: &'static str = r#"eJxNTrsOwjAM/BWre0zu3DapFCqxdYCVgS0SQwYGBsT3Y0AKyJJt3cPncq+PJtf9cEISjGcOa9m9sbV0ZhFkBQXRFduoSJVCiV4I0IWBOs03qlnIh07JrJNwY/ypffKZ/9y+NXenuWPRLyZPVODomWSL1RQmn/Z1WTDN+dJffQEUJiqI"#;

const THUMBS_UP: &'static str = r#"eJxNjrEKwzAMRH9FZLfqk+JYATfQrUP7A90MHTx06FDy/VVacIJA6N6dhMq7fho9z8M9E+IKGZZy2thSuoNEic0Io0eaj1qFhKIXCDy74DS9grAq2eXgZU4kch33fJAga7DDAQdNOE8dOeQ8BzBwg8MWqzKUfu2/swmzR//1C10SKtg="#;

const TICKET: &'static str = r#"eJx1jbEKgDAMRH8luBebq7YItbOLq4NbwaGjg/T7jQ6aQQkhXN4dF/d8FNrGZgYN2ZEjK8OyviKDcGsrF4W9fhhUA50wfgmvwYhh6rQmrE2K7VWY4lPLjvqKb8Dhl7AiJ97sLxs="#;

const TIMER_OFF: &'static str = r#"eJxtjjEKgDAMRa/ycTc2sdgKtTfwEAWHLoKDeH5Th3awhE/ICy8kXOnOOLZhZwPJdohhKiiGurC0gDl5eJivmBx8SSVlUtqRHVloqs1gQ7O2BmgtTP7yKdDSt0znMAtYnrFpLxSyMK8="#;

const TIMER_RESET: &'static str = r#"eJxtizEOgCAQBL+ysQc5OAQTpLbQR5BYUFoY3u8ZE23IVruzk85yVRzLsJOBrTzkND5TTj+wIG6qhxjkSkSEkRCiCngbgZTXTsRNPkFPHXkWsHLzH7oBER0j3w=="#;

const TIMER: &'static str = r#"eJw9jEEKgDAMBL8S8gFNUfCQ5jOhh0Lx0FPze2OjPS3Lziy3ehewlDEhGM0Y3ujw9Eo7Cm8vJLxQom/84TNclxastWsr0DNeCGrxqGNKDsUsDxFdIQw="#;

const TOGGLE_LEFT: &'static str = r#"eJwti7ENgDAMBFexvACQAlEkWcZE2BKVZQmyPTZJ88XfXdZGBtzkYiu4JQR9C+4I/d9HTuOCafV/PE4T1rxEVzOJ0t2A+mwDArlzhDNo/QCZbhwe"#;

const TOGGLE_RIGHT: &'static str = r#"eJwljEsOwCAIRK9CuEBrF67Uy1BTSLoiJNXbF2U3vzdFOxlwl4etYroQZsWMMCq61rGNRvbJbez5ia0ci2uFROntoHtNMx7IqZTXKOr2A6egHE0="#;

const TORNADO: &'static str = r#"eJxtyzEKgDAQRNGrDOlFZ5UlgTX1Nh5CsEhp4f2RNCHFtv/x7b2/hudMlxCH76na2lO1AczIrhEUULxEoqC2JZwI2abpBy/qJAk="#;

const TOUCHPAD_OFF: &'static str = r#"eJx1jqEOgDAMRH+lmW9YL8vUmMZg8UsQMyQIsu+nQzBESUWbXu9d01muSvvs1kChgEC+F+vUBGOhHVWiy2nqjpxeH0hCFVjKI7ElieJ8Y4N39Ehonv8hxi1+/2QsMk5veig1VQ=="#;

const TOUCHPAD: &'static str = r#"eJxNy00KgCAUBOCrPGYfpYgrnzfoEJHScxfy6Of2JUK0GZj5mFDzqiS5bKIM40H1YlhQz5vhQGdJKm+fEMPYDjHsiwolxmzJOOnStp8YS3Y6Bv/RA4TQHlc="#;

const TOWER_CONTROL: &'static str = r#"eJxtzLEKAjEQhOFXGa7PmtmYTQLxaptrLewOLFIoWMg9v9FCOQhb7Xzw1+f6aridpoVZFFTRBPWw8/FOybD+rwThvyclSgqNXuJuNdfn6zTXw6c31181g2ErA6BBdXMDehRYT6eBMMLckBYq7KIjCNDm/vIGRyE+qw=="#;

const TOY_BRICK: &'static str = r#"eJx1yzsKgEAMBNCrhPTRjT8Udq1tbO1FxdiJLH5ub9ZKEBmYhIFnt2nwcCyjF4dcIlwOtWVaZvG6JAjbqRdBO8XaxgHUdu29wOiwZQNllw+GooKijDikKXoGBhNC+u0PDOQNqy8USn/lDfiCKuY="#;

const TRACTOR: &'static str = r#"eJxtjtEKAiEQRX9l8N3JKcd1QfcPeu09LDAwiCWi/r6xjd2FFcQLnjN3DI/zM8MlquMBbO4LQaeGsKuvQ5iZBaKTbQAPZJqAPHDS6MBoArRy0YvRbc07GSmvB/tUfUlklGKkoiU5a1rG0m1M5QpjVMgK0juqTuITFXGVJjxrfy42t6zltw7IZ82bPb8ZPy2qKU37VcMXN+FWIA=="#;

const TRAFFIC_CONE: &'static str = r#"eJxtUEuOwjAMvYrF3m/iOCat1OEGc4HZVWXBAiSkcn/xHCQ2oEQvifU+dpb7+rjI+ffwN8PliLo2RMiAMlagSTmclp9knpY3v2MWK+gbZkxSyTI6NFieu6Mq0seUzy9yc+odsRpmlwEjTx2Twq7KYtl4SyNDSziSYOgMq+IS4nuku4T6VpRFQhKZ/P8ZeetsyKjVNjw63QOumNAJdLYdTH9dr51x/IYcL8g2IlvMso4yNWxRaZUj2q7DKneqM0MrpncbT9lfT0Q="#;

const TRAIN_FRONT_TUNNEL: &'static str = r#"eJx9TzsKwzAMvcoju1xLVkwMbm7QNUM3QYcshQ7F56/6oQRigoQQeh+e6sOeK27n4SIQWViMI7x9eIkvjeMw19ObN9c/m0fkMDUOagkSpq+AMuLi957ACeMaIvcwPcBcV0yh+IQiJW2ULCP/MrJ4xrRlQK97o3txHxKkDuTPcMEWewHVkkWw"#;

const TRAIN_FRONT: &'static str = r#"eJxtjrEKhEAMRH9lsI+3WTfeCnv+ga3FdYtXXCNYiN/vrCBYLAkJZF4ySVve//h9mimia3V+54AAd0WEmzlrxvQq1Jhudh2gJio1SQ0la9rEtWER3/KwGLtntUNCjsWLodAe7giLA6lCgATsWzGKPCYeXe2Hnhqe2glE3Dol"#;

const TRAIN_TRACK: &'static str = r#"eJx1zrEJwDAMBMBVHi8QJGysQvEGHiKQIk0gRfbHUmFXFnz1x8Prd/0P7jN1BlUPp6aHt02nvWYZAtlQAVFAAgnEJiWiDN5Tt2/sqcsGLq4zUA=="#;

const TRAM_FRONT: &'static str = r#"eJxtzU0Kg0AMBeCrPLIf2kQrU5jxBh6iVGlcFEQGf25vxI3i7B758nhh7L4J2vU/TZG4Isx9m/SI4xJJCGukgmC5pDo89kIdhk9StJGaEsxqzyb77SQsKCZ/h78Hv51hhthDxInLWGO1lz5zQ9VVNuewO34="#;

const TRASH_2: &'static str = r#"eJxtjLsKAjEQRX/lMn0wcw3ZFbJb29huL1GIIGIhi/v3ToKPwi2GO49zJt2Pj4LTIIctYtFexrSpqzF9D7pDnDVkD3UKOoL7LlvrHZ1acYorWo84mWScVsWxhGwOCHsCzvw518vtjEUHURUstOwEzzp7S7Y0uGJvuB2D/Ets+w/8AnxqN3Y="#;

const TRASH: &'static str = r#"eJxtzKEOgDAMBNBfueAb6EHGSMY0BjtPhphEkH0/nZlCNGnaexee6y249+Gc4Yr6IYaxnWLoD93gqi55goqCQvBYs62TUNSGyf0wD5cMWU4bEZYlmwFhJWBlNx/bXyEY"#;

const TREE_DECIDUOUS: &'static str = r#"eJxVT0EKAjEM/Mqw90SzaW0X6oIP8APeynrYowfx/U5EKlJKkplMJmmP/txxP0/XClv22hMSjp/nWmXWioGI6UmS5s2YGYxs0SRJtBCYWfKLs8m0dId/ZS6Rjwq+sYGqUCyaQwEqOIbBRTMCp23/swatL7/1Yt/btLZDXLC2cYfNJF4+mDcVry7n"#;

const TREE_PINE: &'static str = r#"eJx1jj0KhUAMhK8y2G/ey2YlCqtgZ6EXsBMsLBQsxPMbBQUFSTdf5icu/TpiKJKZFRwgEJKewfjbsSNTSetA+hCdic1hGN3r/SQ5cgO+ugEyKEnDHjIFhKcnOys4nT7KWbukjL9jaRmvva1leb85udEO0W4r7w=="#;

const TREES: &'static str = r#"eJxtzr0KwzAMBOBXObJH1cWJf8ANdOvQrh26GTp46NCh+PnrNGAymFuEPjgpftI343Ue7lRQi0wXAwOtIbwE0Ba9LkWzpgYjx0X8g4cVLPQ5rPG09a2xtbqtwHaABgzF9GSqkr2YxNqr/4gbKe5GD865KxYhy3QEv0N98V2nGZSlXfsBkWg53A=="#;

const TRELLO: &'static str = r#"eJxdjFEKgCAQRK8ie4EwCwtcL1PS+isL6e3LTQv6Gph5b1wKGysK8SBG0AuojGBAnXFneopUEMY7skSps3dD9bwTu7FGXAvv2yq0/dF9nQXXU4O+l45fhN8sOQ=="#;

const TRENDING_DOWN: &'static str = r#"eJxlizsKwCAQRK8yeICE3ZCYYuNtUgiiQtJ4e/FXiMUMPHhPYnDJWf8iBuv/71HMIA06thN3XwWGVkb24RtZSrpK2fryNPkZYRMeTQ=="#;

const TRENDING_UP: &'static str = r#"eJxlyzEKgDAQRNGrDDmAOhtiLNbcxiIQkoA23t5VsBCbzxTztLdyllw39Jbrsa9OBBH0QwCDZbnHZBEwuqTjC5L+KGejj7fQf94XPbYeGA=="#;

const TRIANGLE_RIGHT: &'static str = r#"eJwdjDEKgDAUQ68S3H/s/y1WoXZ20Au4lTo4Onh/bCWQPAi89JT3xrUOhxl0LgaDa1FptPkqSoVr7YWTMHTazTEg0Fcuwgj9nz6M55DT2J35A37lE6Y="#;

const TRIANGLE: &'static str = r#"eJxFjDsKgDAQRK8ypN+Y/aBbxIDXsAtYpFCw8P7INsp0w3uv3v0ZONZ0CedFwU5ObF0gKDHSbI5ykoNt+24YhAfPP4jwSffU6hTR9gKUFxOz"#;

const TROPHY: &'static str = r#"eJx1zzEPgjAQhuG/8oWds9+lvWsTZHFhcWVwIzowOhh/v6DCQCBNc8ub53LNc3iNeJyrq6F0UdKgkjD/MD0i1Kmzqm1Oc9c2a82MMnKTh2/OvNNHqI7claY1Ucx6+j1AUqolupRcS3FQlBeXnMAsnuDQIBrnqXtW3Fg/apVowoXiYvEIy9DO3j4Y7H8eFaHX2xp/AGz+Szs="#;

const TRUCK: &'static str = r#"eJxljsEKwjAQRH9l2Htitq5NC0nPXrx6l1WI0IMUKfr3Zi1qwcvA7Mw8Nt1O94JzpsMO3O236nyL4Nh5qcLHqKFezKDawkEtB8NLFZ6ZaUgbgwzpi2JBX2QUyCwa4FtnbUMU1/z6ep10vEAfmSJhytQQ9JmJO+ss6Zr6/rD/3y+TD6DSOK4AL9jdOrE="#;

const TURTLE: &'static str = r#"eJyNj7EOwjAMRH/l1D3B55iSSqFzF36ALRJDBpAYUL+fENSqQ4fKi6179p3TO38KHtfuRQUFCptDJghpVbui29lxdiFHxCaKYw85sPFUZ8Xu3ZhOP8cxLb4381Fx8UO9SNnRefaMDWgJ94jeD6FKk0rW+sE/hcAmXekv8GQ2BA=="#;

const TV_2: &'static str = r#"eJwdi8ENgCAQBFvZbAMCmvjh6MAijBCPnyGXqN0LfuYxk4nXboos3FYEr94xxWm4FFs5DHfNpsLgCC31VBP6hXiFM9GeXoiffRtD+gC9PxbJ"#;

const TV: &'static str = r#"eJw1i1EKgCAQRK8y7AUqIfxRL1OSC6JiC+Xt0yLm48Hwnql+EzRLmnDxLsGSmgl3ByF4PoJYWlZC/a7aBpyZRudMybFFTh4lc5KzqxoKi4Lue73fcA9ZPx6w"#;

const TWITCH: &'static str = r#"eJwBOwDE/zxwYXRoIGQ9Ik0yMSAySDN2MTZoNXY0bDQtNGg1bDQtNFYyem0tMTAgOVY3bTUgNFY3Ij48L3BhdGg+5+oQKQ=="#;

const TWITTER: &'static str = r#"eJwdjsEKwkAMRH9l6D1jk83uulB78exHSD14FPTk13daQgJ5eQxZPs/fG6/b9IhAfo0dQbdAYW7OBp9tMOGdxfwKP1hQDpJpWprFvcArVRgCBXWTIauhqlPugNxhKZzWBLsVKk2nGQoWP+d/WpfL8dK6A3M+Hxk="#;

const TYPE: &'static str = r#"eJxli9EJwCAQQ1c5skD1sEhB3aYfgqjQfuj2PS2FggQSQl5cLamnmE+qJeb78jBkyYhYvWYR3PZRwU22aY8D1CVYgRp76F06zy78nx0jY35GjpNZ2AcbjSQZ"#;

const UMBRELLA: &'static str = r#"eJxtyjEKgDAQRNGrDOnV3UHEYs0NvIDdgoWlheT8ZiGkCgO/mWevfw/uI50klK4yy4ZWCGRizZWyLSGzda/hy+4Ew9WtkLFj0X78GnkcUw=="#;

const UNDERLINE: &'static str = r#"eJwtykEKgDAMRNGrDLmAbQlZJbmBW/cFBQURFyL19ralzGL48PTOz47VaBbwK1kgCH0xISxMrlMjrudxbSjJKAXCN75EI64Ze1bblP+X9xU5"#;

const UNDO_2: &'static str = r#"eJxVjDEKwCAUQ68S3LX/gxmEX+cuPYTQwaFDh9LzV0EECRleHsSe8lZcuzsTNCIi3fR02bYusk3dTFUJLAxEr7QoBn2y7n7QoTq/fpJgGUc="#;

const UNDO_DOT: &'static str = r#"eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0uVJGEKpRkhnNUnBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtGHR/X"#;

const UNDO: &'static str = r#"eJw9izEKwCAQBL+y2JvkPDg5uPiC5BFCCssU4vvlCmWb3R3G/tobvju8jDykSSh2+ldsk0SgXBWKyxM1el9LkA5+GMRbnbn2FMg="#;

const UNFOLD_HORIZONTAL: &'static str = r#"eJx1yTEOgCAQBdGr/NAT3UUIJEht4yFMLGhMLIznVyiodjPdvHwfT8W5mp0CiGswJU/tlTwk/rCxAMTgV4GoAS2a8CzJRQnk4WxPcI9kHXpDP5hnO50="#;

const UNFOLD_VERTICAL: &'static str = r#"eJx1yzEKgDAQRNGrLOkX3YkJEWJqG1t7wSKNYCGe38RGiw3TzePHc7sy7ZNZBATc7E2KXT1T/FNYocBAglkD6asETXyRzFoEtOgQRzKyJctlujt+kT5+AG84PEE="#;

const UNGROUP: &'static str = r#"eJxNzEEKgDAMRNGrhLmAFKq4SHoZLabbElBvbyqldDv8N1zzYVQfQQBpLpeaYAP5sIJeQQTd5TQV7Ei8tDzxj5px1O2I5hf3IQ72AZTWHp8="#;

const UNLINK_2: &'static str = r#"eJxFizEKACEMBL+y2MtFIZfGE66z8RGCRRrBwv+jqWTZambSbEvRP1cDQzQ2BoPOgp3Ux+FfUJH/AjHR5fRYmzcFyA8+"#;

const UNLINK: &'static str = r#"eJxtzVEKwyAMBuCrBN/NTKjaQu1dCht00I097EFvP2Ok0G0IkcTP/PNrfW9wTeZBI44DECN7IIxsa6HNouPVo3MDaHVyLBLbiC62YehVnwJOHtwu31kWkVnmi6Qs85HlkSJQffQtpbHfFKgp8D8Feop8tOeQ/f68QeZkRgOFkuF61c4byCTDCoV0WBTmDkm75s9QBAXdxaym9TKfvnFHpaOsSHE48Aei81d1"#;

const UNLOCK: &'static str = r#"eJwli8sKgCAURH9luPsetxAT1D9o215S0l2IkP19SsxmHmd0DmfBa4iZ8CRfYrMbIYZ0xfLXuc0LoRpaW6g9WD31o9W3KxHe0C7BfEgnIDA3MdSoBu5gR+wH3EkbeA=="#;

const UNPLUG: &'static str = r#"eJxtjsEKwkAMRH9l6H3XTdJGhbVfoD/greDBQwUP/j8mOSilYdglZB6Z6e/l88TjMrzojAlSZJj7wZdz/1kM5ty6aRVwq7JwHeGvhcSnKzHotBY1sXM7LMB7cvZYJ5D410CUANTc1MBAY1LawjVksSWJLha9muHddGuWqP/v9QWv+US4"#;

const UPLOAD_CLOUD: &'static str = r#"eJxljDsKgDAQRK8ypHfNLpsfxIAH8BABizSChXh+jUUaeQwMPHj5rFfDvphNwUoxpTUgwII7jgIjNqaQqpJDn/2kvC+SqJiS594oeZRYwHKnvznYg/2kHejwD/RqH/g="#;

const UPLOAD: &'static str = r#"eJxNy7EKgDAMBNBfObIXTbTo0HZ28SMEBQXRgiL2721LB8lyF94ZP90rZkujMFg/7SQQ1PFYxTTof1fyqJacqdLIGX/uYd+OBf7cjvuyxB16sKBBDNkV4Ux2r0QjhMCWGsLLpaa3Tj7bD+ubJk4="#;

const USB: &'static str = r#"eJxtjUEKwjAQRa/ymX1iZpI0LSS9QS/gTqKgUEGKC72906pUsIsZGN7783O9THU8oT4KsSPUZ6FEmPSiPu/etM9fS6m4D14y4de6He5nHAsNwSZwZ70uxNmYycqvwvBGKQSy/+dDZ6VBsk2LCJZRNp+wAwcVBN5Go7PltJoHG9auZa9tL0QZQcM="#;

const USER_2: &'static str = r#"eJwlyjEKgDAMRuGr/GQXk4LSIckNPESJgoKDFAe9vZZOb3ifxlHj3FCNJkI8RpL+vkaZXMd+Xa9y71iNlsRIUjIyGAIeZAY32IB/PxwUXg=="#;

const USER_CHECK_2: &'static str = r#"eJwtzEEKgCAQheGrPGYfNRaS4HiDDhEWJEhJucjbhxVv+T5+m+a8YRGaeACbWUOjq2tYoSNn2wqc9eH0cYUvQoZwCg0EfwuNlXyns+mIJYZ9RTrCni8h1mAGj+AeSsG8vR+5BwQVIkA="#;

const USER_CHECK: &'static str = r#"eJxNjcEKwjAQRH9l2HvQXZdqIZuzFz+ixIKB0IZaiv17je2hzGmYNzO+dPMLT6MHNxBenHQKxbnKqdN7c/TQRSj4Uy0FH9MUc4/4MWoJcTW6EiYjrcgWBl/GvOY09ChjGua30e+HGXwDXyCC9r+3Q+ELWK8moQ=="#;

const USER_CIRCLE_2: &'static str = r#"eJxdyjEKgDAQRNGrDNuLSZBgkc0NPERYBQULWSz09m4UBGWKX8xLW9lnjEyD7xFciYhwdY0PcJRTW0FOsqisE5SpI8jB5IP1tN7ouV/2AVb9uQsvuB+9"#;

const USER_CIRCLE: &'static str = r#"eJxVjEEKgCAQRa8yuK/0B0ag3qBte7HAoEVIRN2+sVoYs3gM896YsKSwzpSsUFJQOJlgXg+dad67M5/3Ezjgri21ze+RJiuGjiBrrTGq3oNAkkcRKkRd7oRDZS3/yK27ATv8JWs="#;

const USER_COG_2: &'static str = r#"eJx9z8EKwjAMBuBXCb3vd2nWboV14AP4EFIFBQUZHvTtbezAHTrpIYd8/ZOM6Tqn25nmaMRQekXDQ67vXJ2Zxl1pT+PCFCz9YPRTt0aP4/NCp2gO3MIRC9zek6f2+yxxUKzoR++W0RN7dA1CA6kIdrAaFraFhyce0EOyqYEAJrawf0BJaPIiXB8xaIJsgg5CZZFMaoe2eqhozBp8APMuZLc="#;

const USER_COG: &'static str = r#"eJx9z8EKwjAMBuBXCbk3Lk3XbdDu7MWHGFVQUJAhom9vYwV36EYPLc3Xv0lIlzldT5DeEblFSK+89whzRMEx7Ep5DD+Wr11BA37fdEt0nx5nOEY8cAPc7v3kwEGjy+TT06pV85c3y9QBe3KGBkNSEdySBRYtrwlPHrinjiSbGhiIgS3ZDVASTG6E61/0miCrwJFAaSST2qCNDioaswQfNeplDQ=="#;

const USER_MINUS_2: &'static str = r#"eJwlisEJgDAQBFtZ9i96QYJCLh1YhERBQUTER9K9OcI+hmEnPOt3YFMuMkLm1cNjsHXiMDCG3oIY0vmma0cqypl4lSORsnKypJ0xXOe9o4hShCiuMVd3rtLcW25Z/AF+7x9i"#;

const USER_MINUS: &'static str = r#"eJxNTMsKwjAQ/JVh7kF3WSJCNmcvfkSJBQtFpEhp/t6m7aHMYZhn+na/N17Op0SozEE7g+HaECzYI541bFbmdGmjnMowlbFHqc4bMTmNKIvz3ip7mNM4fHpUcYoQi64ciaqHXn3dHlst/wHJsyPD"#;

const USER_PLUS_2: &'static str = r#"eJw9y1EKhDAMBNCrDPO/rCmlrND0BnsI6QoriIj40d5eY0XyEZJ5E9dh/+On/IqH9ENAQGfzEoeOKb4NpJinLc8jclF+iFyVPbEpvZEWpjhPy4jqlOKJYvtEVa5Kkes8tanb2tO5ZkTYunJ3w4MP5ioqcg=="#;

const USER_PLUS: &'static str = r#"eJxNi8EKwjAQRH9l2HvQXZZoIZuzFz+ixIKFIlKkpH9vl5RS5jAM81769r83XkZPjhBegvQKxdUTNOgjnjd0Ecrp4lJOZZzLNKCsRjfCbKSEUo06R9qZ0zR+BlQx4o6wem9U5X1vfXfaqTMb28e8O9wckQP+A/CELtM="#;

const USER_SQUARE_2: &'static str = r#"eJwly0EKgDAMBMCvLLmLpop4aPsDHyGx2IIHKQX196aVwJKQWXttJWJ3tPICw9uMGUOdjg0G8ravwFtJWc4AeRyxIWRHE0Fevbii/+1tDlKQFanRHAlvyxjSEYvqhXCnvcS2arEW/AcqaCQH"#;

const USER_SQUARE: &'static str = r#"eJxNjEEKwzAMBL+y6B4aqdD2YPsHfURQTGXooRiTJr+PlVyCLlp2ZkPN2rBFuhP+ZW4WiV8Ey+Vj7fzrGkkIqzMp3FxIQUvVb0Y9RO0dd0T7Do8OnXUKv6kZ5kjvJ4SXQSaBYOzHkEHscc2QRdx1J+2WMygy"#;

const USER_X_2: &'static str = r#"eJxVy0EKgzAQheGrPN6+1JmKrZDJDXoISYUKUop0kdy+TtSFzG7e94fv8HvjZXxKC+mHDh0av4soGsZwdRBDmpY0j0jZ+CBSMfbEYmydbGMM8/QZkdWoSmQxyp0oUpOyvuXm2tVuz1tNaqo1PewfvAoqQg=="#;

const USER_X: &'static str = r#"eJxNzMEKwjAMBuBXCf+9aGOYCk3PXnyIUQcOhsiQ0b29iXYwcghJvj/p3X+e9FDcY0ccl8C9kNDRK0iQW7efSRZGTgcP5VTGuUwDlaq4gsqqOINmhTj5H3OaxtdAlRXMoBoV0dBq/WLN1vHk2lWz7ebUI838Xlh0s1/C8y6j"#;

const USER: &'static str = r#"eJxNizEKgDAQBL+yXB80x4EEcqltfEQ4BQULCRL09xJswlbLzMQr3ztWpcUHsK+Os0AwtjlxMof+QypTikOLUrSj2LnBHiXPBHuVJkJRkub8NH2NBRjF"#;

const USERS_2: &'static str = r#"eJxVjEEKgCAQRa/ymb2kIqLgeIMOIVNQ0CKkRd2+LBDirT7/8dJejgUT02gcTCweHrqhjIWmnIYm5CRrlW2GnEyBUJkcQS6m2JTvzKm3rP23vPJwDxrm3aGHb0WGH+I="#;

const USERS: &'static str = r#"eJxljjEKgDAMRa8SsrfaNFqFtrOLh5AqKDiIiOjttQhVkJAh5P3Ht0u3jdA7bFUJpHZBHQNDHkew4Kb83sA7obdZDHkbpjXMA4TDYY2wOmSEcDo0EXme3iY/0c+vhZaVeYXfLloqnVB1r5GmSOgFeFMtjA=="#;

const UTENSILS_CROSSED: &'static str = r#"eJxtTjEKwzAM/MqR3aotJVYKbl6QfqCbaYcOMXTo/6kUUggkSKfhdHdS+dTvG69b11IGByaBoQoEcaueeEk0wrDjjUWcmTF2U7l4ylT+Wfc0wFoszFBdusq3yovaxvAkJQXDhx2I8+p7NBcp9BjcmBLYn8nUh0xyIklXDGHv/gFfZjQ5"#;

const UTENSILS: &'static str = r#"eJxtjbEKgDAQQ3/lcG+9u1qLUPsHrh3cig5dBAe577dV6CCSIYHwEn+mK8M+d4sBFrchkCY9AVflIVXHR6w4chd8X4ngG+cKx/hTMAHZyILJgn1HVEkyfk7MepRKXJu4AbKYJM8="#;

const UTILITY_POLE: &'static str = r#"eJx1yTEKgDAQRNGrDOmD7oSwCGtu4CEEizSCheT8mibVht/9Z8/5Vlx7OIRg4xqKLf0VG0Lk6kJCanS+Tr7MgOLDLRtyVGj8G/wBPT8zCw=="#;

const VARIABLE: &'static str = r#"eJxVy0EKgCAUhOGrDG8v8dQiQbtBhwgKCiKCWujtU7Oixb8Y+MbuwzljdNS3kHwILVTMoESdrZLo7Ou4gTo0FDSMKH1sXbYJgR0ZQpCOuCZ4mafnPCNNqFB/U1/o//nQC2clKVg="#;

const VEGAN: &'static str = r#"eJxtTTkOgDAM+4rFntCkJYBU+AGPqMrAyMD/RQCpE7JsefCRz3Id2JduU2hRY8MrwSGQAA2VZzKeFMIDzTwgkaRuzf1TXXMbEMNUk/eM1GnknnzOaX/xkZMgshV/ke8vICK27A3wxCPm"#;

const VENETIAN_MASK: &'static str = r#"eJxtjaEOgDAMRH+lwR+sXQaIgcZgEbgFxCSC/w/tEsgEuXS7Jveu8Up3pnNqViGWFCiQKzI3qsyzeqk3CFVJhG3ICOkNONRx3SCL7M0cOzs2x+9kT8wHt1bkST9Pos0O3ga/CI/GoEAwSB9RWYNODT2Cby7u"#;

const VIBRATE_OFF: &'static str = r#"eJxti7EOwjAQQ3/Fyn4ldyRRhjR/wMqOAlKQADGgiv49d1SoDJUHy/ZzeZ5eHefR3QUZAiGB/NzVsrO9lpVS7B/BJnbIyBP75jHEOIQINvXUNMKD6dsRT7R15gT2wz4cU/OkD6OJTZ2C9uvldn1cMPPoxOEtauZLnJeoqEH1A3aNOD0="#;

const VIBRATE: &'static str = r#"eJxVyzEKgDAUA9CrhOwifhQc2t5FbLEdBCkfrLe3HUSdQsiLORaN8Ja7YIZAOoE8SWf6tjvzqsq+BD+Ww6o4k9doORMxpC2q5TASl+VE5FIbUdpcX827G3q9Icw="#;

const VIDEO_OFF: &'static str = r#"eJxNjLEKwzAQQ39FeHfqU83hwfGcoV07dAu0kIIpHUpI/j4XMjhokAR6yr/xP+HVu7uEThU6SBwJIpjEnDO7a6wCuZFIc3IlX3ao5IYqRBvlLQ3x3D0f6fzqObGKxfBsd/XzfWOR3tFhoZn5etT1qDbdR2UDCnEqnw=="#;

const VIDEO: &'static str = r#"eJwli7EKgDAMRH8lZBexlNKh6Wc4uIktpoMgJaD9exMcDu7e8dK9C0MhvJyDOAXwoFnjhjnN9uXU6yHwtCJMuHiEl9AhcG0nixLt/UeDMOgYNtQ2L3/6cxot"#;

const VIDEOTAPE: &'static str = r#"eJx9jUEKgDAMBL8S8gFtEemh7Q98hMRiCh6kFNTfu8WLePASQnZn4kuSSlfggamcgS3TM4+8VMXWM2nKq9bAZuTouwZEv89VaQk8WXKKEoJ2il5ykS2RQGngFNgc1M2JzpO+cEfGqvvF8fbL33Y9M7o="#;

const VIEW: &'static str = r#"eJxtjLEOwkAMQ3/F6p5yMRduKZ27sLKfysCIVHQDX09SJApSFVmKk2cPj/q843buLgblwt6yiaGIzbnPlpFQYKFF4nmCiZtZ4mtI4mTo1Y3DIbrG4duohB6rQr0klMQPn20d7oXoRGmsBFdIPcTJfr2wCfej5bqRKcjpz4NtC74BR305uQ=="#;

const VOICEMAIL: &'static str = r#"eJxNyzEKwDAIheGriBcohlI6qJeRDIHQIZO5fSuxkOkN//fY2rBewaYgFYQheCKYC16ofKyqnCpr2lB076y3p4JTnGGWr8bSWi+/DqUvHmAhAA=="#;

const VOLUME_1: &'static str = r#"eJwtTFEKgCAUu8rDA5gvfJqg3qBDCEEFoUL+dPueImODjW2+luc7S4Za7tzeIBCBwICDdRC7YeEYXVcS0S9zFH1N7YIjiB1JkoZNapOIDxQDmVYqOwbciz/+Exru"#;

const VOLUME_2: &'static str = r#"eJxNjFEKhDAMRK8SeoBsZzfRLVRv4CEKggpiC/rj7Y0iImEGMryZWPK8D3mhkqdlWxsHkFJFgb6XcD5mFiOcrq6Nn7vUxpK2kfrGdVBWoT9LldQGvB1MNfv6Khj3poPlJBx+CQb6h4cw5CkcR2kmRg=="#;

const VOLUME_X: &'static str = r#"eJxVi0EKwCAMBL8S8oGSgIKg/qYUQVRoD/X3zRZ76CEbNpmJo9d59Eajl3adiUXIkadA+o6gWNhZAtJxjtuScqyl7TTVNMd0S2JV2+ieaVoPwEEtFgx+YMB+7p99AHkNJiw="#;

const VOLUME: &'static str =
    r#"eJwti1EKABAQRK8yuYBWUQrXkZJV/Li9XfmYV/Pqpcn9VB6Y3MZe2RDBIyDCvZEegWiKSm9Ksj8qFxX0EBo="#;

const VOTE: &'static str = r#"eJxtiyEPgCAQRv/KzX7Id4MxNiRTrAYb00BxMzh/v1zQ5F5876WzXo32aTgiQUhx7IacRhU5vXr2FDbLMDCRe8XSYKvmtgMdb0jxS1h/ZhFCLPKZB93vHd4="#;

const WALLET_2: &'static str = r#"eJxNTKsOgDAM/JVmfrBrttSUaQwWvwRRiSB8P8Vsy4l75U7v9hhdWzgghGxLQqi6/mnV3gmJgRsTU3LAmV+kEURXe5l95LPMg8iG3K8/zgUbpA=="#;

const WALLET_CARDS: &'static str = r#"eJxVTc0KwjAMfpWP3heXdNMN2p29+BCjG3bgQUZBfXuTHsQRyJfvj4R9TQXv6LzDriAOn0pe21JydDw45HW751LvKZysMIXnXDKW6G4e4ywQtDoMaSRz9y9ArGTxQ4k5+0SDZehMHkJM44NtJ1VMRUdcsacLWvMaNalvqFfZK7GWwlX49+MLamExsA=="#;

const WALLET: &'static str = r#"eJxtzKEOgDAMhOFXafALbbkBYkxjsAjcEkQlgvT5GWZBkBNnvvzpKrfRuXSbConu0xqLkhLXCXGACRxdTv0Lc2p8oOiCZrm+mowe4g+WubY/lgkGDziafQAcGSIR"#;

const WALLPAPER: &'static str = r#"eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2pghtD2VZhpnv8IV0H9PjhvSJ4gVjFArSN0onfTj8YR9ewzvjGuXZQVs0Vk35gSBcOQWtV+v0RELrS70QZ2h4rLYdnNStQ0lmbbaD4ayelav47EHNfgdoUbZTtZAfsKAytQ=="#;

const WAND_2: &'static str = r#"eJxtj7EKwzAMRH/lyB7Xkh3bATdzhvYHugU6ZGihQ8n3V3JJCIkRHEZPvpPyZ/rOeF6bN5MJHk6kJcOpyCRCKGK1pBkZ9sbGBVCS2eOElM689DdWC33soBqUsE5sDvwf8WiGfNHNhrztRx4RDu6M7h3C4it96kG+Tix44QqISGMtQo8Mc1v1Irix38APsIlS9A=="#;

const WAND: &'static str = r#"eJx1zTEOgCAMheGrNOwgLYI0QW7g6m7iwGLiYDy/4kRiWTq8P1+azu0qsM9qQQ/jSiqnoU45tQHDraUUgYu0k+0EnEwExHoY0MnfuNgeDYaq9P9+OCAE1ixRetlHsaEPPOZELw=="#;

const WAREHOUSE: &'static str = r#"eJxtjjELwjAQhf/KI3tick1DhiTg5uLawa3YYgodRALqvzeHUlsot7x37zvehXtfMoYozkTwqmk70j2BoOsYWdXJrr2kjqnjskKjyMGpdvayyj8Lo6yHnj0TK/7XcxEpHLg8heUFB+Ozof3EbpLHeC3I43TLJYoa4DkNJX/lKwon8K5G8wGj6QObrTfT"#;

const WATCH: &'static str = r#"eJxNjtEOgyAMRX+l4Z1O6lZ4UP/FMBNImBrnw/z7FYkboaHJ7b2n7XzcfJrAf3plSIE/St96xWrobmU8dOuSjhTnCdYlzvs7m8A0kH+pViq7L5f4xz3As1cvwyhji8wandF3bB4jAUGTnyZtkE3QhOxqGbKcNFoHOXGyhVhxLToHAm8ZxfPPQskGQku1ei5K1wk/4BdvIT54"#;

const WAVES: &'static str = r#"eJy1jT0KgDAMRq/y0d1ooq0KtYuzh5A6ODp4f0w6CV2VEMjL34vXfp84FrcJQqZAHkwChli1jrDwmJV8FgrodDJAtCdlx9g3yppMvTKTbZd7l2JrghRfGpbKU336yDT9aXoAe60/UA=="#;

const WEBCAM: &'static str = r#"eJxNy0EKgDAMRNGrhNmLNgq6aHsDDyFRqOBCioje3ojUdhXI+2NljbItJJeDYZDcehtQdBjgbf2xtynL3CKNimyfjkCzw9gTc9BS7f0VYljprLqfHpZbJPo="#;

const WEBHOOK: &'static str = r#"eJxtz0sKwlAMBdCthM5zfXn/QC2IYxdR6qATwYH7x9uKOimBQD4ckvE5v1a5n4ebdbEK76sWuC9qMAnMzjJrROYcfsmSJTBMolhbEEzRELmYUZrGYRpPmzmNX/lRuSgJlii3vqAkhTcYUeuKopzNH9cYFd01I9QDyqLUnRJS6WoFlXpE228X9rc30vy/Mkj/OW9lxjQU"#;

const WHEAT_OFF: &'static str = r#"eJx1kstuwyAQRX9l5D1TZnjYSEm+wNl2kZ3lVnIlt+qiqpq/7x1bcR/BQswguFyGA4f34WOip2PzqqRK4p345nR4sOnTYVuUTJ0TlpYs3AvOgSPWlFOgRCKz2MjCEDiRdY8m6JFL7KEps7N1VxM5E10qdXQoA/r/W/zNt5CkebMdOSXIE3dWvCYulkutfvFchBJrHlmwSTOHiL2wyY7b0EugsHsr1g7nZ60Yqyed9FOHSHFVO4wmp4/5Z4qiq133LLJgbZcYNmaVEhZi5Fewe1xXzaX+vpJrZDfjv2ShiYWWsFobVge+tUt03EaUz76M4OrAF1wd+IIrzsy9in2Zne9gnyErXuSX9/zy9kxXPTaqDX3dsiA3dF0SpCY6fQPD6qX0"#;

const WHEAT: &'static str = r#"eJydkDEOgzAMRa/yxZ4UOw4QiXKCdu3QDakDA0OHqudvnFR0SQGhyJZlf/k5v3+OrwmPc3VlMIMadNXQn7Q79MvMWWlBbL2DB9FMWmkanfXQqOOjGGKDXKImzEbnpiQyKroXMK1iOt0d0G5BAsgfgRApRdJyB7eFiZr44QMcrsETv3kUSBabWE2Gb82vBTErR1IyRC/42lm4M/FRZ8//XZk1RZDPiIxbLF0BJd8PgIIiUmJaLF3hZOP3gj5MWZvt"#;

const WHOLE_WORD: &'static str = r#"eJxtjMEKwjAQRH9l2HtqJrVGIcnZix8hq6DgQYoE+/dNKG0p9LI7zONN0Hevnyd0iEIn6KO0Av1H8ZLCYaIpfO+/Fx5RbrS45FNFtUph1otAL9uZXf8In8+rvwDnQJ+pFk1nmg40BK+t1mwNyyuH2XBxR1wlMqQ="#;

const WIFI_OFF: &'static str = r#"eJxtkEEKwjAQRa8ydO83M8lMI7S9gYcIKiiIuHBRb+/EtlKhZPEJvM8fXne/PS40ct9IQ+8pRvGo3ymHbl+hoXuW15XOfXPMUGKDFiWl4I+ppVDBiqxAoYwshR1fwARudwLTDZwDzEhPCYF3iOZtTjgQM2KiiNa2SoasFREt7AthHhKIEMPyRsf5+AcrJPlV64FJjCtg8XOaryNeJIXZTvjZ+QDf9E5Y"#;

const WIFI: &'static str = r#"eJxtykEKwjAUBNCrDP8A8c/XaBZJbuAhAgoKIi5ctLfvT2lLF2VmM8PLv/Z/4VHkHsFzo8LbQ/AClZpPXdS8uRRcXkNsEXGRt0NoSCFZo/MVmu7l5/19YmARmmC0IqaCwfoOSr84X+67rBPV4Ski"#;

const WIND: &'static str = r#"eJxtyz0KgDAMhuGrfPQA0cTaH6gFNxcPUXBwdPD+mAwWBwnvlOcrV7tPHIvbOVKE1oRmWCPYjhI8TZu4WgbDtfRJpqC/sArk1Yz0S1nUcibfsebB4aMfD3kgyQ=="#;

const WINE_OFF: &'static str = r#"eJxtT8sKwkAM/JWh98ZNso8s1IJ49iOKCiuoeBCpf++2PvBQQiaZZBKS7jbcCw7rZmcQKdb03Woq9d2vkcCu6CXBlZZJvS5oWMDhkZamSV3CjBsWUsUb3WzT7iEgfClpZvgJd0ZR602UjbdGKdQ0RoORJam5emRIiXumAA9BrG51B3mXWnKBzOWW2Acwcf577Hy6HjHKuhFp8OQaa/jQcaZVOon6F9E9QqE="#;

const WINE: &'static str = r#"eJx1jLEOgCAQQ3+lYT/kiCeYILOLP+BGcGB0MH6/6EAcNJdL076mYU9HwTapxcPa4lUM3R3F0IADm8Lmg7AFy+n+SBIIzHNCkg1Z0kJ9FT+PmVgLqsFQ37+7kLVNXnRpKBY="#;

const WORKFLOW: &'static str = r#"eJx1jlsKgCAQRbcyzAbKB9SHuoMWISmNfyFSufscpc8YGC7cw+GaHPcC+bEoESimg4rFFeFOoVBPrVIIlb8zE+POnL4QBIvbAkJc2kuQMPdriTSDjDjT7c0g1I+9jm4M+PwvIpko6g=="#;

const WRAP_TEXT: &'static str = r#"eJw1jkEKhDAQBL/SzH3ZdOIGD0l+sI8QFCKICnrQ35uJygzUHIrpDtM4DzgYxQkOG8VScBb6AipS+KqTwtrtGX2UvwNt5q9zcDBgGQOfP42qKhV1mc76eF3Ged+i0EO3AVs9rKnuY6VQXY1je6crtQ2NPOXeGhcYhi6A"#;

const WRENCH: &'static str = r#"eJxNjrEKgDAMRH/lcE80tqQItX/gRxQcHDo4OPn1Nq2gBF7gchcunvk6sK/DJp4DlF0WCKZ3hH0R1rr1p1cVU3EcAhmyQrtOgRcPQyHlRWDIM8uMhm5y5IodmuULtxz1cH2rMNxDiqN1TA/GsCOZ"#;

const X_CIRCLE: &'static str = r#"eJxNyTsKgDAQhOGrDNuLRjCwsMldZBUUFCRYJLfPq0k1zP+J3kGfE5ocmZUQyiwEje16mbt7+fb/wuHoNRt4srAVaxyIwRglA8gkGc8="#;

const X_OCTAGON: &'static str = r#"eJxNzEEKgCAQheGrPNxXKDUlqHcJggpKhdx0+0QNXQ18//CUd9e7OwvvThsezeZ+IQhw6vkYrxDI8lMJletvkszMqKEsG+XXcGDT7OYTZEegVCM2SUKiLR8PxSW3"#;

const X_SQUARE: &'static str = r#"eJxNi0EKgCAURK8y/H2ERZKg3iVS0kUQ8iG7ff7auHrDvBlb4s54HM2EOwdOjtRKSDEfif9cP1naZmqoAm9H+Xl7bZwQHJ1qgRk0tCgpO2Vg0JsXTCQf1Q=="#;

const X: &'static str =
    r#"eJw9ybEJACAMRNFVQhaQWEiKMxs4hGBhI1i4P0ZBuav+w6yrU8tcRCn5RNkQTjU8Gxei/9sGq2sO7w=="#;

const YOUTUBE: &'static str = r#"eJxtjkEKgDAMBL8SvDc2tVGEKvgAH1Hw4EXw4MnXG2uqghKykOwybFjjNsPUFaNDBmqi80gOLrUyBNaQBXnpSeiNLPgWuVZVp0YJDU/UkTB/kfBGJlxGsqo6H2RqWfShPIv3IddfBEgMbCoju9+BAwNZLQw="#;

const ZAP_OFF: &'static str = r#"eJyFy8EKgzAQBNBfGfIB2+6axAjRv+lBEBXag/n7jkEP4sHLzrDMy+sylWmcP1iXcf59e6cmXhGlDdAGBn1LaOGlMzfk1zkf8h2mfUjeKUzpoEFiZHmACQkN1NPWS85CGK+wok17Zw7FGMztyFLfnNfpH0wFOoI="#;

const ZAP: &'static str =
    r#"eJwti7EJACEQBFtZbOB/1w992xFBPEETu1fEZIJhJjQrM1lFs1xH/x09BA9+oA4JCSL4HrO5CxfDc8+4AF5nEOw="#;

const ZOOM_IN: &'static str = r#"eJxtjDEKwCAMRa8ScoCWiBUH9TLBQZAOTnr7Rkxxcfoh/70fuDSuGXhEJELgvrJF9JjCveoUankzDIpopOxGIHe5B2Hss69WpAmrMp/0K5JzwqtmD6xX1CqjCu3dD0KhLXU="#;

const ZOOM_OUT: &'static str = r#"eJw9jUsOwCAIRK9COEAbTGu6UC9DXJiYLlzh7UvxsyIw7zGBS+OagXtEIoQW8UFgsS2Fc8Qp1PJmEKdnf/gbQSiiU76vuSOVfngpZA/NvCY2tVFg7Adb0CJ1"#;

impl LucideIcon {

    fn decompress(&self, input: &str) -> String {
        use base64::decode;
        use flate2::read::ZlibDecoder;
        use std::io::prelude::*;

        let input = base64::decode(input).unwrap();
        let mut decoder = ZlibDecoder::new(input.as_slice());
        let mut decompressed = String::new();
        decoder
            .read_to_string(&mut decompressed)
            .expect("decompress");
        decompressed
    }

    pub fn svg(&self) -> String {
        match self {
            &Self::Accessibility => self.decompress(ACCESSIBILITY),
            &Self::ActivitySquare => self.decompress(ACTIVITY_SQUARE),
            &Self::Activity => self.decompress(ACTIVITY),
            &Self::AirVent => self.decompress(AIR_VENT),
            &Self::Airplay => self.decompress(AIRPLAY),
            &Self::AlarmCheck => self.decompress(ALARM_CHECK),
            &Self::AlarmClockOff => self.decompress(ALARM_CLOCK_OFF),
            &Self::AlarmClock => self.decompress(ALARM_CLOCK),
            &Self::AlarmMinus => self.decompress(ALARM_MINUS),
            &Self::AlarmPlus => self.decompress(ALARM_PLUS),
            &Self::Album => self.decompress(ALBUM),
            &Self::AlertCircle => self.decompress(ALERT_CIRCLE),
            &Self::AlertOctagon => self.decompress(ALERT_OCTAGON),
            &Self::AlertTriangle => self.decompress(ALERT_TRIANGLE),
            &Self::AlignCenterHorizontal => self.decompress(ALIGN_CENTER_HORIZONTAL),
            &Self::AlignCenterVertical => self.decompress(ALIGN_CENTER_VERTICAL),
            &Self::AlignCenter => self.decompress(ALIGN_CENTER),
            &Self::AlignEndHorizontal => self.decompress(ALIGN_END_HORIZONTAL),
            &Self::AlignEndVertical => self.decompress(ALIGN_END_VERTICAL),
            &Self::AlignHorizontalDistributeCenter => {
                self.decompress(ALIGN_HORIZONTAL_DISTRIBUTE_CENTER)
            }
            &Self::AlignHorizontalDistributeEnd => self.decompress(ALIGN_HORIZONTAL_DISTRIBUTE_END),
            &Self::AlignHorizontalDistributeStart => {
                self.decompress(ALIGN_HORIZONTAL_DISTRIBUTE_START)
            }
            &Self::AlignHorizontalJustifyCenter => self.decompress(ALIGN_HORIZONTAL_JUSTIFY_CENTER),
            &Self::AlignHorizontalJustifyEnd => self.decompress(ALIGN_HORIZONTAL_JUSTIFY_END),
            &Self::AlignHorizontalJustifyStart => self.decompress(ALIGN_HORIZONTAL_JUSTIFY_START),
            &Self::AlignHorizontalSpaceAround => self.decompress(ALIGN_HORIZONTAL_SPACE_AROUND),
            &Self::AlignHorizontalSpaceBetween => self.decompress(ALIGN_HORIZONTAL_SPACE_BETWEEN),
            &Self::AlignJustify => self.decompress(ALIGN_JUSTIFY),
            &Self::AlignLeft => self.decompress(ALIGN_LEFT),
            &Self::AlignRight => self.decompress(ALIGN_RIGHT),
            &Self::AlignStartHorizontal => self.decompress(ALIGN_START_HORIZONTAL),
            &Self::AlignStartVertical => self.decompress(ALIGN_START_VERTICAL),
            &Self::AlignVerticalDistributeCenter => {
                self.decompress(ALIGN_VERTICAL_DISTRIBUTE_CENTER)
            }
            &Self::AlignVerticalDistributeEnd => self.decompress(ALIGN_VERTICAL_DISTRIBUTE_END),
            &Self::AlignVerticalDistributeStart => self.decompress(ALIGN_VERTICAL_DISTRIBUTE_START),
            &Self::AlignVerticalJustifyCenter => self.decompress(ALIGN_VERTICAL_JUSTIFY_CENTER),
            &Self::AlignVerticalJustifyEnd => self.decompress(ALIGN_VERTICAL_JUSTIFY_END),
            &Self::AlignVerticalJustifyStart => self.decompress(ALIGN_VERTICAL_JUSTIFY_START),
            &Self::AlignVerticalSpaceAround => self.decompress(ALIGN_VERTICAL_SPACE_AROUND),
            &Self::AlignVerticalSpaceBetween => self.decompress(ALIGN_VERTICAL_SPACE_BETWEEN),
            &Self::Ampersand => self.decompress(AMPERSAND),
            &Self::Ampersands => self.decompress(AMPERSANDS),
            &Self::Anchor => self.decompress(ANCHOR),
            &Self::Angry => self.decompress(ANGRY),
            &Self::Annoyed => self.decompress(ANNOYED),
            &Self::Antenna => self.decompress(ANTENNA),
            &Self::Aperture => self.decompress(APERTURE),
            &Self::AppWindow => self.decompress(APP_WINDOW),
            &Self::Apple => self.decompress(APPLE),
            &Self::ArchiveRestore => self.decompress(ARCHIVE_RESTORE),
            &Self::ArchiveX => self.decompress(ARCHIVE_X),
            &Self::Archive => self.decompress(ARCHIVE),
            &Self::AreaChart => self.decompress(AREA_CHART),
            &Self::Armchair => self.decompress(ARMCHAIR),
            &Self::ArrowBigDownDash => self.decompress(ARROW_BIG_DOWN_DASH),
            &Self::ArrowBigDown => self.decompress(ARROW_BIG_DOWN),
            &Self::ArrowBigLeftDash => self.decompress(ARROW_BIG_LEFT_DASH),
            &Self::ArrowBigLeft => self.decompress(ARROW_BIG_LEFT),
            &Self::ArrowBigRightDash => self.decompress(ARROW_BIG_RIGHT_DASH),
            &Self::ArrowBigRight => self.decompress(ARROW_BIG_RIGHT),
            &Self::ArrowBigUpDash => self.decompress(ARROW_BIG_UP_DASH),
            &Self::ArrowBigUp => self.decompress(ARROW_BIG_UP),
            &Self::ArrowDown01 => self.decompress(ARROW_DOWN_01),
            &Self::ArrowDown10 => self.decompress(ARROW_DOWN_10),
            &Self::ArrowDownAZ => self.decompress(ARROW_DOWN_AZ),
            &Self::ArrowDownCircle => self.decompress(ARROW_DOWN_CIRCLE),
            &Self::ArrowDownFromLine => self.decompress(ARROW_DOWN_FROM_LINE),
            &Self::ArrowDownLeftFromCircle => self.decompress(ARROW_DOWN_LEFT_FROM_CIRCLE),
            &Self::ArrowDownLeftSquare => self.decompress(ARROW_DOWN_LEFT_SQUARE),
            &Self::ArrowDownLeft => self.decompress(ARROW_DOWN_LEFT),
            &Self::ArrowDownNarrowWide => self.decompress(ARROW_DOWN_NARROW_WIDE),
            &Self::ArrowDownRightFromCircle => self.decompress(ARROW_DOWN_RIGHT_FROM_CIRCLE),
            &Self::ArrowDownRightSquare => self.decompress(ARROW_DOWN_RIGHT_SQUARE),
            &Self::ArrowDownRight => self.decompress(ARROW_DOWN_RIGHT),
            &Self::ArrowDownSquare => self.decompress(ARROW_DOWN_SQUARE),
            &Self::ArrowDownToDot => self.decompress(ARROW_DOWN_TO_DOT),
            &Self::ArrowDownToLine => self.decompress(ARROW_DOWN_TO_LINE),
            &Self::ArrowDownUp => self.decompress(ARROW_DOWN_UP),
            &Self::ArrowDownWideNarrow => self.decompress(ARROW_DOWN_WIDE_NARROW),
            &Self::ArrowDownZA => self.decompress(ARROW_DOWN_ZA),
            &Self::ArrowDown => self.decompress(ARROW_DOWN),
            &Self::ArrowLeftCircle => self.decompress(ARROW_LEFT_CIRCLE),
            &Self::ArrowLeftFromLine => self.decompress(ARROW_LEFT_FROM_LINE),
            &Self::ArrowLeftRight => self.decompress(ARROW_LEFT_RIGHT),
            &Self::ArrowLeftSquare => self.decompress(ARROW_LEFT_SQUARE),
            &Self::ArrowLeftToLine => self.decompress(ARROW_LEFT_TO_LINE),
            &Self::ArrowLeft => self.decompress(ARROW_LEFT),
            &Self::ArrowRightCircle => self.decompress(ARROW_RIGHT_CIRCLE),
            &Self::ArrowRightFromLine => self.decompress(ARROW_RIGHT_FROM_LINE),
            &Self::ArrowRightLeft => self.decompress(ARROW_RIGHT_LEFT),
            &Self::ArrowRightSquare => self.decompress(ARROW_RIGHT_SQUARE),
            &Self::ArrowRightToLine => self.decompress(ARROW_RIGHT_TO_LINE),
            &Self::ArrowRight => self.decompress(ARROW_RIGHT),
            &Self::ArrowUp01 => self.decompress(ARROW_UP_01),
            &Self::ArrowUp10 => self.decompress(ARROW_UP_10),
            &Self::ArrowUpAZ => self.decompress(ARROW_UP_AZ),
            &Self::ArrowUpCircle => self.decompress(ARROW_UP_CIRCLE),
            &Self::ArrowUpDown => self.decompress(ARROW_UP_DOWN),
            &Self::ArrowUpFromDot => self.decompress(ARROW_UP_FROM_DOT),
            &Self::ArrowUpFromLine => self.decompress(ARROW_UP_FROM_LINE),
            &Self::ArrowUpLeftFromCircle => self.decompress(ARROW_UP_LEFT_FROM_CIRCLE),
            &Self::ArrowUpLeftSquare => self.decompress(ARROW_UP_LEFT_SQUARE),
            &Self::ArrowUpLeft => self.decompress(ARROW_UP_LEFT),
            &Self::ArrowUpNarrowWide => self.decompress(ARROW_UP_NARROW_WIDE),
            &Self::ArrowUpRightFromCircle => self.decompress(ARROW_UP_RIGHT_FROM_CIRCLE),
            &Self::ArrowUpRightSquare => self.decompress(ARROW_UP_RIGHT_SQUARE),
            &Self::ArrowUpRight => self.decompress(ARROW_UP_RIGHT),
            &Self::ArrowUpSquare => self.decompress(ARROW_UP_SQUARE),
            &Self::ArrowUpToLine => self.decompress(ARROW_UP_TO_LINE),
            &Self::ArrowUpWideNarrow => self.decompress(ARROW_UP_WIDE_NARROW),
            &Self::ArrowUpZA => self.decompress(ARROW_UP_ZA),
            &Self::ArrowUp => self.decompress(ARROW_UP),
            &Self::ArrowsUpFromLine => self.decompress(ARROWS_UP_FROM_LINE),
            &Self::Asterisk => self.decompress(ASTERISK),
            &Self::AtSign => self.decompress(AT_SIGN),
            &Self::Atom => self.decompress(ATOM),
            &Self::Award => self.decompress(AWARD),
            &Self::Axe => self.decompress(AXE),
            &Self::Axis3D => self.decompress(AXIS_3_D),
            &Self::Baby => self.decompress(BABY),
            &Self::Backpack => self.decompress(BACKPACK),
            &Self::BadgeAlert => self.decompress(BADGE_ALERT),
            &Self::BadgeCent => self.decompress(BADGE_CENT),
            &Self::BadgeCheck => self.decompress(BADGE_CHECK),
            &Self::BadgeDollarSign => self.decompress(BADGE_DOLLAR_SIGN),
            &Self::BadgeEuro => self.decompress(BADGE_EURO),
            &Self::BadgeHelp => self.decompress(BADGE_HELP),
            &Self::BadgeIndianRupee => self.decompress(BADGE_INDIAN_RUPEE),
            &Self::BadgeInfo => self.decompress(BADGE_INFO),
            &Self::BadgeJapaneseYen => self.decompress(BADGE_JAPANESE_YEN),
            &Self::BadgeMinus => self.decompress(BADGE_MINUS),
            &Self::BadgePercent => self.decompress(BADGE_PERCENT),
            &Self::BadgePlus => self.decompress(BADGE_PLUS),
            &Self::BadgePoundSterling => self.decompress(BADGE_POUND_STERLING),
            &Self::BadgeRussianRuble => self.decompress(BADGE_RUSSIAN_RUBLE),
            &Self::BadgeSwissFranc => self.decompress(BADGE_SWISS_FRANC),
            &Self::BadgeX => self.decompress(BADGE_X),
            &Self::Badge => self.decompress(BADGE),
            &Self::BaggageClaim => self.decompress(BAGGAGE_CLAIM),
            &Self::Ban => self.decompress(BAN),
            &Self::Banana => self.decompress(BANANA),
            &Self::Banknote => self.decompress(BANKNOTE),
            &Self::BarChart2 => self.decompress(BAR_CHART_2),
            &Self::BarChart3 => self.decompress(BAR_CHART_3),
            &Self::BarChart4 => self.decompress(BAR_CHART_4),
            &Self::BarChartBig => self.decompress(BAR_CHART_BIG),
            &Self::BarChartHorizontalBig => self.decompress(BAR_CHART_HORIZONTAL_BIG),
            &Self::BarChartHorizontal => self.decompress(BAR_CHART_HORIZONTAL),
            &Self::BarChart => self.decompress(BAR_CHART),
            &Self::Baseline => self.decompress(BASELINE),
            &Self::Bath => self.decompress(BATH),
            &Self::BatteryCharging => self.decompress(BATTERY_CHARGING),
            &Self::BatteryFull => self.decompress(BATTERY_FULL),
            &Self::BatteryLow => self.decompress(BATTERY_LOW),
            &Self::BatteryMedium => self.decompress(BATTERY_MEDIUM),
            &Self::BatteryWarning => self.decompress(BATTERY_WARNING),
            &Self::Battery => self.decompress(BATTERY),
            &Self::Beaker => self.decompress(BEAKER),
            &Self::BeanOff => self.decompress(BEAN_OFF),
            &Self::Bean => self.decompress(BEAN),
            &Self::BedDouble => self.decompress(BED_DOUBLE),
            &Self::BedSingle => self.decompress(BED_SINGLE),
            &Self::Bed => self.decompress(BED),
            &Self::Beef => self.decompress(BEEF),
            &Self::Beer => self.decompress(BEER),
            &Self::BellDot => self.decompress(BELL_DOT),
            &Self::BellMinus => self.decompress(BELL_MINUS),
            &Self::BellOff => self.decompress(BELL_OFF),
            &Self::BellPlus => self.decompress(BELL_PLUS),
            &Self::BellRing => self.decompress(BELL_RING),
            &Self::Bell => self.decompress(BELL),
            &Self::Bike => self.decompress(BIKE),
            &Self::Binary => self.decompress(BINARY),
            &Self::Biohazard => self.decompress(BIOHAZARD),
            &Self::Bird => self.decompress(BIRD),
            &Self::Bitcoin => self.decompress(BITCOIN),
            &Self::Blinds => self.decompress(BLINDS),
            &Self::Blocks => self.decompress(BLOCKS),
            &Self::BluetoothConnected => self.decompress(BLUETOOTH_CONNECTED),
            &Self::BluetoothOff => self.decompress(BLUETOOTH_OFF),
            &Self::BluetoothSearching => self.decompress(BLUETOOTH_SEARCHING),
            &Self::Bluetooth => self.decompress(BLUETOOTH),
            &Self::Bold => self.decompress(BOLD),
            &Self::Bomb => self.decompress(BOMB),
            &Self::Bone => self.decompress(BONE),
            &Self::BookCopy => self.decompress(BOOK_COPY),
            &Self::BookDown => self.decompress(BOOK_DOWN),
            &Self::BookKey => self.decompress(BOOK_KEY),
            &Self::BookLock => self.decompress(BOOK_LOCK),
            &Self::BookMarked => self.decompress(BOOK_MARKED),
            &Self::BookMinus => self.decompress(BOOK_MINUS),
            &Self::BookOpenCheck => self.decompress(BOOK_OPEN_CHECK),
            &Self::BookOpen => self.decompress(BOOK_OPEN),
            &Self::BookPlus => self.decompress(BOOK_PLUS),
            &Self::BookTemplate => self.decompress(BOOK_TEMPLATE),
            &Self::BookUp2 => self.decompress(BOOK_UP_2),
            &Self::BookUp => self.decompress(BOOK_UP),
            &Self::BookX => self.decompress(BOOK_X),
            &Self::Book => self.decompress(BOOK),
            &Self::BookmarkMinus => self.decompress(BOOKMARK_MINUS),
            &Self::BookmarkPlus => self.decompress(BOOKMARK_PLUS),
            &Self::Bookmark => self.decompress(BOOKMARK),
            &Self::BoomBox => self.decompress(BOOM_BOX),
            &Self::Bot => self.decompress(BOT),
            &Self::BoxSelect => self.decompress(BOX_SELECT),
            &Self::Box => self.decompress(BOX),
            &Self::Boxes => self.decompress(BOXES),
            &Self::Braces => self.decompress(BRACES),
            &Self::Brackets => self.decompress(BRACKETS),
            &Self::BrainCircuit => self.decompress(BRAIN_CIRCUIT),
            &Self::BrainCog => self.decompress(BRAIN_COG),
            &Self::Brain => self.decompress(BRAIN),
            &Self::Briefcase => self.decompress(BRIEFCASE),
            &Self::BringToFront => self.decompress(BRING_TO_FRONT),
            &Self::Brush => self.decompress(BRUSH),
            &Self::BugOff => self.decompress(BUG_OFF),
            &Self::BugPlay => self.decompress(BUG_PLAY),
            &Self::Bug => self.decompress(BUG),
            &Self::Building2 => self.decompress(BUILDING_2),
            &Self::Building => self.decompress(BUILDING),
            &Self::BusFront => self.decompress(BUS_FRONT),
            &Self::Bus => self.decompress(BUS),
            &Self::CableCar => self.decompress(CABLE_CAR),
            &Self::Cable => self.decompress(CABLE),
            &Self::CakeSlice => self.decompress(CAKE_SLICE),
            &Self::Cake => self.decompress(CAKE),
            &Self::Calculator => self.decompress(CALCULATOR),
            &Self::CalendarCheck2 => self.decompress(CALENDAR_CHECK_2),
            &Self::CalendarCheck => self.decompress(CALENDAR_CHECK),
            &Self::CalendarClock => self.decompress(CALENDAR_CLOCK),
            &Self::CalendarDays => self.decompress(CALENDAR_DAYS),
            &Self::CalendarHeart => self.decompress(CALENDAR_HEART),
            &Self::CalendarMinus => self.decompress(CALENDAR_MINUS),
            &Self::CalendarOff => self.decompress(CALENDAR_OFF),
            &Self::CalendarPlus => self.decompress(CALENDAR_PLUS),
            &Self::CalendarRange => self.decompress(CALENDAR_RANGE),
            &Self::CalendarSearch => self.decompress(CALENDAR_SEARCH),
            &Self::CalendarX2 => self.decompress(CALENDAR_X_2),
            &Self::CalendarX => self.decompress(CALENDAR_X),
            &Self::Calendar => self.decompress(CALENDAR),
            &Self::CameraOff => self.decompress(CAMERA_OFF),
            &Self::Camera => self.decompress(CAMERA),
            &Self::CandlestickChart => self.decompress(CANDLESTICK_CHART),
            &Self::CandyCane => self.decompress(CANDY_CANE),
            &Self::CandyOff => self.decompress(CANDY_OFF),
            &Self::Candy => self.decompress(CANDY),
            &Self::CarFront => self.decompress(CAR_FRONT),
            &Self::CarTaxiFront => self.decompress(CAR_TAXI_FRONT),
            &Self::Car => self.decompress(CAR),
            &Self::Carrot => self.decompress(CARROT),
            &Self::CaseLower => self.decompress(CASE_LOWER),
            &Self::CaseSensitive => self.decompress(CASE_SENSITIVE),
            &Self::CaseUpper => self.decompress(CASE_UPPER),
            &Self::CassetteTape => self.decompress(CASSETTE_TAPE),
            &Self::Cast => self.decompress(CAST),
            &Self::Castle => self.decompress(CASTLE),
            &Self::Cat => self.decompress(CAT),
            &Self::CheckCheck => self.decompress(CHECK_CHECK),
            &Self::CheckCircle2 => self.decompress(CHECK_CIRCLE_2),
            &Self::CheckCircle => self.decompress(CHECK_CIRCLE),
            &Self::CheckSquare => self.decompress(CHECK_SQUARE),
            &Self::Check => self.decompress(CHECK),
            &Self::ChefHat => self.decompress(CHEF_HAT),
            &Self::Cherry => self.decompress(CHERRY),
            &Self::ChevronDownCircle => self.decompress(CHEVRON_DOWN_CIRCLE),
            &Self::ChevronDownSquare => self.decompress(CHEVRON_DOWN_SQUARE),
            &Self::ChevronDown => self.decompress(CHEVRON_DOWN),
            &Self::ChevronFirst => self.decompress(CHEVRON_FIRST),
            &Self::ChevronLast => self.decompress(CHEVRON_LAST),
            &Self::ChevronLeftCircle => self.decompress(CHEVRON_LEFT_CIRCLE),
            &Self::ChevronLeftSquare => self.decompress(CHEVRON_LEFT_SQUARE),
            &Self::ChevronLeft => self.decompress(CHEVRON_LEFT),
            &Self::ChevronRightCircle => self.decompress(CHEVRON_RIGHT_CIRCLE),
            &Self::ChevronRightSquare => self.decompress(CHEVRON_RIGHT_SQUARE),
            &Self::ChevronRight => self.decompress(CHEVRON_RIGHT),
            &Self::ChevronUpCircle => self.decompress(CHEVRON_UP_CIRCLE),
            &Self::ChevronUpSquare => self.decompress(CHEVRON_UP_SQUARE),
            &Self::ChevronUp => self.decompress(CHEVRON_UP),
            &Self::ChevronsDownUp => self.decompress(CHEVRONS_DOWN_UP),
            &Self::ChevronsDown => self.decompress(CHEVRONS_DOWN),
            &Self::ChevronsLeftRight => self.decompress(CHEVRONS_LEFT_RIGHT),
            &Self::ChevronsLeft => self.decompress(CHEVRONS_LEFT),
            &Self::ChevronsRightLeft => self.decompress(CHEVRONS_RIGHT_LEFT),
            &Self::ChevronsRight => self.decompress(CHEVRONS_RIGHT),
            &Self::ChevronsUpDown => self.decompress(CHEVRONS_UP_DOWN),
            &Self::ChevronsUp => self.decompress(CHEVRONS_UP),
            &Self::Chrome => self.decompress(CHROME),
            &Self::Church => self.decompress(CHURCH),
            &Self::CigaretteOff => self.decompress(CIGARETTE_OFF),
            &Self::Cigarette => self.decompress(CIGARETTE),
            &Self::CircleDashed => self.decompress(CIRCLE_DASHED),
            &Self::CircleDollarSign => self.decompress(CIRCLE_DOLLAR_SIGN),
            &Self::CircleDotDashed => self.decompress(CIRCLE_DOT_DASHED),
            &Self::CircleDot => self.decompress(CIRCLE_DOT),
            &Self::CircleEllipsis => self.decompress(CIRCLE_ELLIPSIS),
            &Self::CircleEqual => self.decompress(CIRCLE_EQUAL),
            &Self::CircleOff => self.decompress(CIRCLE_OFF),
            &Self::CircleSlash2 => self.decompress(CIRCLE_SLASH_2),
            &Self::CircleSlash => self.decompress(CIRCLE_SLASH),
            &Self::Circle => self.decompress(CIRCLE),
            &Self::CircuitBoard => self.decompress(CIRCUIT_BOARD),
            &Self::Citrus => self.decompress(CITRUS),
            &Self::Clapperboard => self.decompress(CLAPPERBOARD),
            &Self::ClipboardCheck => self.decompress(CLIPBOARD_CHECK),
            &Self::ClipboardCopy => self.decompress(CLIPBOARD_COPY),
            &Self::ClipboardEdit => self.decompress(CLIPBOARD_EDIT),
            &Self::ClipboardList => self.decompress(CLIPBOARD_LIST),
            &Self::ClipboardPaste => self.decompress(CLIPBOARD_PASTE),
            &Self::ClipboardSignature => self.decompress(CLIPBOARD_SIGNATURE),
            &Self::ClipboardType => self.decompress(CLIPBOARD_TYPE),
            &Self::ClipboardX => self.decompress(CLIPBOARD_X),
            &Self::Clipboard => self.decompress(CLIPBOARD),
            &Self::Clock1 => self.decompress(CLOCK_1),
            &Self::Clock10 => self.decompress(CLOCK_10),
            &Self::Clock11 => self.decompress(CLOCK_11),
            &Self::Clock12 => self.decompress(CLOCK_12),
            &Self::Clock2 => self.decompress(CLOCK_2),
            &Self::Clock3 => self.decompress(CLOCK_3),
            &Self::Clock4 => self.decompress(CLOCK_4),
            &Self::Clock5 => self.decompress(CLOCK_5),
            &Self::Clock6 => self.decompress(CLOCK_6),
            &Self::Clock7 => self.decompress(CLOCK_7),
            &Self::Clock8 => self.decompress(CLOCK_8),
            &Self::Clock9 => self.decompress(CLOCK_9),
            &Self::Clock => self.decompress(CLOCK),
            &Self::CloudCog => self.decompress(CLOUD_COG),
            &Self::CloudDrizzle => self.decompress(CLOUD_DRIZZLE),
            &Self::CloudFog => self.decompress(CLOUD_FOG),
            &Self::CloudHail => self.decompress(CLOUD_HAIL),
            &Self::CloudLightning => self.decompress(CLOUD_LIGHTNING),
            &Self::CloudMoonRain => self.decompress(CLOUD_MOON_RAIN),
            &Self::CloudMoon => self.decompress(CLOUD_MOON),
            &Self::CloudOff => self.decompress(CLOUD_OFF),
            &Self::CloudRainWind => self.decompress(CLOUD_RAIN_WIND),
            &Self::CloudRain => self.decompress(CLOUD_RAIN),
            &Self::CloudSnow => self.decompress(CLOUD_SNOW),
            &Self::CloudSunRain => self.decompress(CLOUD_SUN_RAIN),
            &Self::CloudSun => self.decompress(CLOUD_SUN),
            &Self::Cloud => self.decompress(CLOUD),
            &Self::Cloudy => self.decompress(CLOUDY),
            &Self::Clover => self.decompress(CLOVER),
            &Self::Club => self.decompress(CLUB),
            &Self::Code2 => self.decompress(CODE_2),
            &Self::Code => self.decompress(CODE),
            &Self::Codepen => self.decompress(CODEPEN),
            &Self::Codesandbox => self.decompress(CODESANDBOX),
            &Self::Coffee => self.decompress(COFFEE),
            &Self::Cog => self.decompress(COG),
            &Self::Coins => self.decompress(COINS),
            &Self::Columns => self.decompress(COLUMNS),
            &Self::Combine => self.decompress(COMBINE),
            &Self::Command => self.decompress(COMMAND),
            &Self::Compass => self.decompress(COMPASS),
            &Self::Component => self.decompress(COMPONENT),
            &Self::Computer => self.decompress(COMPUTER),
            &Self::ConciergeBell => self.decompress(CONCIERGE_BELL),
            &Self::Construction => self.decompress(CONSTRUCTION),
            &Self::Contact2 => self.decompress(CONTACT_2),
            &Self::Contact => self.decompress(CONTACT),
            &Self::Container => self.decompress(CONTAINER),
            &Self::Contrast => self.decompress(CONTRAST),
            &Self::Cookie => self.decompress(COOKIE),
            &Self::CopyCheck => self.decompress(COPY_CHECK),
            &Self::CopyMinus => self.decompress(COPY_MINUS),
            &Self::CopyPlus => self.decompress(COPY_PLUS),
            &Self::CopySlash => self.decompress(COPY_SLASH),
            &Self::CopyX => self.decompress(COPY_X),
            &Self::Copy => self.decompress(COPY),
            &Self::Copyleft => self.decompress(COPYLEFT),
            &Self::Copyright => self.decompress(COPYRIGHT),
            &Self::CornerDownLeft => self.decompress(CORNER_DOWN_LEFT),
            &Self::CornerDownRight => self.decompress(CORNER_DOWN_RIGHT),
            &Self::CornerLeftDown => self.decompress(CORNER_LEFT_DOWN),
            &Self::CornerLeftUp => self.decompress(CORNER_LEFT_UP),
            &Self::CornerRightDown => self.decompress(CORNER_RIGHT_DOWN),
            &Self::CornerRightUp => self.decompress(CORNER_RIGHT_UP),
            &Self::CornerUpLeft => self.decompress(CORNER_UP_LEFT),
            &Self::CornerUpRight => self.decompress(CORNER_UP_RIGHT),
            &Self::Cpu => self.decompress(CPU),
            &Self::CreativeCommons => self.decompress(CREATIVE_COMMONS),
            &Self::CreditCard => self.decompress(CREDIT_CARD),
            &Self::Croissant => self.decompress(CROISSANT),
            &Self::Crop => self.decompress(CROP),
            &Self::Cross => self.decompress(CROSS),
            &Self::Crosshair => self.decompress(CROSSHAIR),
            &Self::Crown => self.decompress(CROWN),
            &Self::CupSoda => self.decompress(CUP_SODA),
            &Self::Currency => self.decompress(CURRENCY),
            &Self::DatabaseBackup => self.decompress(DATABASE_BACKUP),
            &Self::DatabaseZap => self.decompress(DATABASE_ZAP),
            &Self::Database => self.decompress(DATABASE),
            &Self::Delete => self.decompress(DELETE),
            &Self::Dessert => self.decompress(DESSERT),
            &Self::Diamond => self.decompress(DIAMOND),
            &Self::Dice1 => self.decompress(DICE_1),
            &Self::Dice2 => self.decompress(DICE_2),
            &Self::Dice3 => self.decompress(DICE_3),
            &Self::Dice4 => self.decompress(DICE_4),
            &Self::Dice5 => self.decompress(DICE_5),
            &Self::Dice6 => self.decompress(DICE_6),
            &Self::Dices => self.decompress(DICES),
            &Self::Diff => self.decompress(DIFF),
            &Self::Disc2 => self.decompress(DISC_2),
            &Self::Disc3 => self.decompress(DISC_3),
            &Self::Disc => self.decompress(DISC),
            &Self::DivideCircle => self.decompress(DIVIDE_CIRCLE),
            &Self::DivideSquare => self.decompress(DIVIDE_SQUARE),
            &Self::Divide => self.decompress(DIVIDE),
            &Self::DnaOff => self.decompress(DNA_OFF),
            &Self::Dna => self.decompress(DNA),
            &Self::Dog => self.decompress(DOG),
            &Self::DollarSign => self.decompress(DOLLAR_SIGN),
            &Self::Donut => self.decompress(DONUT),
            &Self::DoorClosed => self.decompress(DOOR_CLOSED),
            &Self::DoorOpen => self.decompress(DOOR_OPEN),
            &Self::Dot => self.decompress(DOT),
            &Self::DownloadCloud => self.decompress(DOWNLOAD_CLOUD),
            &Self::Download => self.decompress(DOWNLOAD),
            &Self::Dribbble => self.decompress(DRIBBBLE),
            &Self::Droplet => self.decompress(DROPLET),
            &Self::Droplets => self.decompress(DROPLETS),
            &Self::Drumstick => self.decompress(DRUMSTICK),
            &Self::Dumbbell => self.decompress(DUMBBELL),
            &Self::EarOff => self.decompress(EAR_OFF),
            &Self::Ear => self.decompress(EAR),
            &Self::EggFried => self.decompress(EGG_FRIED),
            &Self::EggOff => self.decompress(EGG_OFF),
            &Self::Egg => self.decompress(EGG),
            &Self::EqualNot => self.decompress(EQUAL_NOT),
            &Self::Equal => self.decompress(EQUAL),
            &Self::Eraser => self.decompress(ERASER),
            &Self::Euro => self.decompress(EURO),
            &Self::Expand => self.decompress(EXPAND),
            &Self::ExternalLink => self.decompress(EXTERNAL_LINK),
            &Self::EyeOff => self.decompress(EYE_OFF),
            &Self::Eye => self.decompress(EYE),
            &Self::Facebook => self.decompress(FACEBOOK),
            &Self::Factory => self.decompress(FACTORY),
            &Self::Fan => self.decompress(FAN),
            &Self::FastForward => self.decompress(FAST_FORWARD),
            &Self::Feather => self.decompress(FEATHER),
            &Self::FerrisWheel => self.decompress(FERRIS_WHEEL),
            &Self::Figma => self.decompress(FIGMA),
            &Self::FileArchive => self.decompress(FILE_ARCHIVE),
            &Self::FileAudio2 => self.decompress(FILE_AUDIO_2),
            &Self::FileAudio => self.decompress(FILE_AUDIO),
            &Self::FileAxis3D => self.decompress(FILE_AXIS_3_D),
            &Self::FileBadge2 => self.decompress(FILE_BADGE_2),
            &Self::FileBadge => self.decompress(FILE_BADGE),
            &Self::FileBarChart2 => self.decompress(FILE_BAR_CHART_2),
            &Self::FileBarChart => self.decompress(FILE_BAR_CHART),
            &Self::FileBox => self.decompress(FILE_BOX),
            &Self::FileCheck2 => self.decompress(FILE_CHECK_2),
            &Self::FileCheck => self.decompress(FILE_CHECK),
            &Self::FileClock => self.decompress(FILE_CLOCK),
            &Self::FileCode2 => self.decompress(FILE_CODE_2),
            &Self::FileCode => self.decompress(FILE_CODE),
            &Self::FileCog => self.decompress(FILE_COG),
            &Self::FileDiff => self.decompress(FILE_DIFF),
            &Self::FileDigit => self.decompress(FILE_DIGIT),
            &Self::FileDown => self.decompress(FILE_DOWN),
            &Self::FileEdit => self.decompress(FILE_EDIT),
            &Self::FileHeart => self.decompress(FILE_HEART),
            &Self::FileImage => self.decompress(FILE_IMAGE),
            &Self::FileInput => self.decompress(FILE_INPUT),
            &Self::FileJson2 => self.decompress(FILE_JSON_2),
            &Self::FileJson => self.decompress(FILE_JSON),
            &Self::FileKey2 => self.decompress(FILE_KEY_2),
            &Self::FileKey => self.decompress(FILE_KEY),
            &Self::FileLineChart => self.decompress(FILE_LINE_CHART),
            &Self::FileLock2 => self.decompress(FILE_LOCK_2),
            &Self::FileLock => self.decompress(FILE_LOCK),
            &Self::FileMinus2 => self.decompress(FILE_MINUS_2),
            &Self::FileMinus => self.decompress(FILE_MINUS),
            &Self::FileOutput => self.decompress(FILE_OUTPUT),
            &Self::FilePieChart => self.decompress(FILE_PIE_CHART),
            &Self::FilePlus2 => self.decompress(FILE_PLUS_2),
            &Self::FilePlus => self.decompress(FILE_PLUS),
            &Self::FileQuestion => self.decompress(FILE_QUESTION),
            &Self::FileScan => self.decompress(FILE_SCAN),
            &Self::FileSearch2 => self.decompress(FILE_SEARCH_2),
            &Self::FileSearch => self.decompress(FILE_SEARCH),
            &Self::FileSignature => self.decompress(FILE_SIGNATURE),
            &Self::FileSpreadsheet => self.decompress(FILE_SPREADSHEET),
            &Self::FileStack => self.decompress(FILE_STACK),
            &Self::FileSymlink => self.decompress(FILE_SYMLINK),
            &Self::FileTerminal => self.decompress(FILE_TERMINAL),
            &Self::FileText => self.decompress(FILE_TEXT),
            &Self::FileType2 => self.decompress(FILE_TYPE_2),
            &Self::FileType => self.decompress(FILE_TYPE),
            &Self::FileUp => self.decompress(FILE_UP),
            &Self::FileVideo2 => self.decompress(FILE_VIDEO_2),
            &Self::FileVideo => self.decompress(FILE_VIDEO),
            &Self::FileVolume2 => self.decompress(FILE_VOLUME_2),
            &Self::FileVolume => self.decompress(FILE_VOLUME),
            &Self::FileWarning => self.decompress(FILE_WARNING),
            &Self::FileX2 => self.decompress(FILE_X_2),
            &Self::FileX => self.decompress(FILE_X),
            &Self::File => self.decompress(FILE),
            &Self::Files => self.decompress(FILES),
            &Self::Film => self.decompress(FILM),
            &Self::FilterX => self.decompress(FILTER_X),
            &Self::Filter => self.decompress(FILTER),
            &Self::Fingerprint => self.decompress(FINGERPRINT),
            &Self::FishOff => self.decompress(FISH_OFF),
            &Self::FishSymbol => self.decompress(FISH_SYMBOL),
            &Self::Fish => self.decompress(FISH),
            &Self::FlagOff => self.decompress(FLAG_OFF),
            &Self::FlagTriangleLeft => self.decompress(FLAG_TRIANGLE_LEFT),
            &Self::FlagTriangleRight => self.decompress(FLAG_TRIANGLE_RIGHT),
            &Self::Flag => self.decompress(FLAG),
            &Self::Flame => self.decompress(FLAME),
            &Self::FlashlightOff => self.decompress(FLASHLIGHT_OFF),
            &Self::Flashlight => self.decompress(FLASHLIGHT),
            &Self::FlaskConicalOff => self.decompress(FLASK_CONICAL_OFF),
            &Self::FlaskConical => self.decompress(FLASK_CONICAL),
            &Self::FlaskRound => self.decompress(FLASK_ROUND),
            &Self::FlipHorizontal2 => self.decompress(FLIP_HORIZONTAL_2),
            &Self::FlipHorizontal => self.decompress(FLIP_HORIZONTAL),
            &Self::FlipVertical2 => self.decompress(FLIP_VERTICAL_2),
            &Self::FlipVertical => self.decompress(FLIP_VERTICAL),
            &Self::Flower2 => self.decompress(FLOWER_2),
            &Self::Flower => self.decompress(FLOWER),
            &Self::Focus => self.decompress(FOCUS),
            &Self::FoldHorizontal => self.decompress(FOLD_HORIZONTAL),
            &Self::FoldVertical => self.decompress(FOLD_VERTICAL),
            &Self::FolderArchive => self.decompress(FOLDER_ARCHIVE),
            &Self::FolderCheck => self.decompress(FOLDER_CHECK),
            &Self::FolderClock => self.decompress(FOLDER_CLOCK),
            &Self::FolderClosed => self.decompress(FOLDER_CLOSED),
            &Self::FolderCog => self.decompress(FOLDER_COG),
            &Self::FolderDot => self.decompress(FOLDER_DOT),
            &Self::FolderDown => self.decompress(FOLDER_DOWN),
            &Self::FolderEdit => self.decompress(FOLDER_EDIT),
            &Self::FolderGit2 => self.decompress(FOLDER_GIT_2),
            &Self::FolderGit => self.decompress(FOLDER_GIT),
            &Self::FolderHeart => self.decompress(FOLDER_HEART),
            &Self::FolderInput => self.decompress(FOLDER_INPUT),
            &Self::FolderKanban => self.decompress(FOLDER_KANBAN),
            &Self::FolderKey => self.decompress(FOLDER_KEY),
            &Self::FolderLock => self.decompress(FOLDER_LOCK),
            &Self::FolderMinus => self.decompress(FOLDER_MINUS),
            &Self::FolderOpenDot => self.decompress(FOLDER_OPEN_DOT),
            &Self::FolderOpen => self.decompress(FOLDER_OPEN),
            &Self::FolderOutput => self.decompress(FOLDER_OUTPUT),
            &Self::FolderPlus => self.decompress(FOLDER_PLUS),
            &Self::FolderRoot => self.decompress(FOLDER_ROOT),
            &Self::FolderSearch2 => self.decompress(FOLDER_SEARCH_2),
            &Self::FolderSearch => self.decompress(FOLDER_SEARCH),
            &Self::FolderSymlink => self.decompress(FOLDER_SYMLINK),
            &Self::FolderSync => self.decompress(FOLDER_SYNC),
            &Self::FolderTree => self.decompress(FOLDER_TREE),
            &Self::FolderUp => self.decompress(FOLDER_UP),
            &Self::FolderX => self.decompress(FOLDER_X),
            &Self::Folder => self.decompress(FOLDER),
            &Self::Folders => self.decompress(FOLDERS),
            &Self::Footprints => self.decompress(FOOTPRINTS),
            &Self::Forklift => self.decompress(FORKLIFT),
            &Self::FormInput => self.decompress(FORM_INPUT),
            &Self::Forward => self.decompress(FORWARD),
            &Self::Frame => self.decompress(FRAME),
            &Self::Framer => self.decompress(FRAMER),
            &Self::Frown => self.decompress(FROWN),
            &Self::Fuel => self.decompress(FUEL),
            &Self::FunctionSquare => self.decompress(FUNCTION_SQUARE),
            &Self::GalleryHorizontalEnd => self.decompress(GALLERY_HORIZONTAL_END),
            &Self::GalleryHorizontal => self.decompress(GALLERY_HORIZONTAL),
            &Self::GalleryThumbnails => self.decompress(GALLERY_THUMBNAILS),
            &Self::GalleryVerticalEnd => self.decompress(GALLERY_VERTICAL_END),
            &Self::GalleryVertical => self.decompress(GALLERY_VERTICAL),
            &Self::Gamepad2 => self.decompress(GAMEPAD_2),
            &Self::Gamepad => self.decompress(GAMEPAD),
            &Self::GanttChartSquare => self.decompress(GANTT_CHART_SQUARE),
            &Self::GanttChart => self.decompress(GANTT_CHART),
            &Self::GaugeCircle => self.decompress(GAUGE_CIRCLE),
            &Self::Gauge => self.decompress(GAUGE),
            &Self::Gavel => self.decompress(GAVEL),
            &Self::Gem => self.decompress(GEM),
            &Self::Ghost => self.decompress(GHOST),
            &Self::Gift => self.decompress(GIFT),
            &Self::GitBranchPlus => self.decompress(GIT_BRANCH_PLUS),
            &Self::GitBranch => self.decompress(GIT_BRANCH),
            &Self::GitCommit => self.decompress(GIT_COMMIT),
            &Self::GitCompare => self.decompress(GIT_COMPARE),
            &Self::GitFork => self.decompress(GIT_FORK),
            &Self::GitMerge => self.decompress(GIT_MERGE),
            &Self::GitPullRequestClosed => self.decompress(GIT_PULL_REQUEST_CLOSED),
            &Self::GitPullRequestDraft => self.decompress(GIT_PULL_REQUEST_DRAFT),
            &Self::GitPullRequest => self.decompress(GIT_PULL_REQUEST),
            &Self::Github => self.decompress(GITHUB),
            &Self::Gitlab => self.decompress(GITLAB),
            &Self::GlassWater => self.decompress(GLASS_WATER),
            &Self::Glasses => self.decompress(GLASSES),
            &Self::Globe2 => self.decompress(GLOBE_2),
            &Self::Globe => self.decompress(GLOBE),
            &Self::Goal => self.decompress(GOAL),
            &Self::Grab => self.decompress(GRAB),
            &Self::GraduationCap => self.decompress(GRADUATION_CAP),
            &Self::Grape => self.decompress(GRAPE),
            &Self::Grid2X2 => self.decompress(GRID_2_X_2),
            &Self::Grid3X3 => self.decompress(GRID_3_X_3),
            &Self::GripHorizontal => self.decompress(GRIP_HORIZONTAL),
            &Self::GripVertical => self.decompress(GRIP_VERTICAL),
            &Self::Grip => self.decompress(GRIP),
            &Self::Group => self.decompress(GROUP),
            &Self::Hammer => self.decompress(HAMMER),
            &Self::HandMetal => self.decompress(HAND_METAL),
            &Self::Hand => self.decompress(HAND),
            &Self::HardDriveDownload => self.decompress(HARD_DRIVE_DOWNLOAD),
            &Self::HardDriveUpload => self.decompress(HARD_DRIVE_UPLOAD),
            &Self::HardDrive => self.decompress(HARD_DRIVE),
            &Self::HardHat => self.decompress(HARD_HAT),
            &Self::Hash => self.decompress(HASH),
            &Self::Haze => self.decompress(HAZE),
            &Self::HdmiPort => self.decompress(HDMI_PORT),
            &Self::Heading1 => self.decompress(HEADING_1),
            &Self::Heading2 => self.decompress(HEADING_2),
            &Self::Heading3 => self.decompress(HEADING_3),
            &Self::Heading4 => self.decompress(HEADING_4),
            &Self::Heading5 => self.decompress(HEADING_5),
            &Self::Heading6 => self.decompress(HEADING_6),
            &Self::Heading => self.decompress(HEADING),
            &Self::Headphones => self.decompress(HEADPHONES),
            &Self::HeartCrack => self.decompress(HEART_CRACK),
            &Self::HeartHandshake => self.decompress(HEART_HANDSHAKE),
            &Self::HeartOff => self.decompress(HEART_OFF),
            &Self::HeartPulse => self.decompress(HEART_PULSE),
            &Self::Heart => self.decompress(HEART),
            &Self::HelpCircle => self.decompress(HELP_CIRCLE),
            &Self::HelpingHand => self.decompress(HELPING_HAND),
            &Self::Hexagon => self.decompress(HEXAGON),
            &Self::Highlighter => self.decompress(HIGHLIGHTER),
            &Self::History => self.decompress(HISTORY),
            &Self::Home => self.decompress(HOME),
            &Self::HopOff => self.decompress(HOP_OFF),
            &Self::Hop => self.decompress(HOP),
            &Self::Hotel => self.decompress(HOTEL),
            &Self::Hourglass => self.decompress(HOURGLASS),
            &Self::IceCream2 => self.decompress(ICE_CREAM_2),
            &Self::IceCream => self.decompress(ICE_CREAM),
            &Self::ImageMinus => self.decompress(IMAGE_MINUS),
            &Self::ImageOff => self.decompress(IMAGE_OFF),
            &Self::ImagePlus => self.decompress(IMAGE_PLUS),
            &Self::Image => self.decompress(IMAGE),
            &Self::Import => self.decompress(IMPORT),
            &Self::Inbox => self.decompress(INBOX),
            &Self::Indent => self.decompress(INDENT),
            &Self::IndianRupee => self.decompress(INDIAN_RUPEE),
            &Self::Infinity => self.decompress(INFINITY),
            &Self::Info => self.decompress(INFO),
            &Self::Instagram => self.decompress(INSTAGRAM),
            &Self::Italic => self.decompress(ITALIC),
            &Self::IterationCcw => self.decompress(ITERATION_CCW),
            &Self::IterationCw => self.decompress(ITERATION_CW),
            &Self::JapaneseYen => self.decompress(JAPANESE_YEN),
            &Self::Joystick => self.decompress(JOYSTICK),
            &Self::KanbanSquareDashed => self.decompress(KANBAN_SQUARE_DASHED),
            &Self::KanbanSquare => self.decompress(KANBAN_SQUARE),
            &Self::Kanban => self.decompress(KANBAN),
            &Self::KeyRound => self.decompress(KEY_ROUND),
            &Self::KeySquare => self.decompress(KEY_SQUARE),
            &Self::Key => self.decompress(KEY),
            &Self::Keyboard => self.decompress(KEYBOARD),
            &Self::LampCeiling => self.decompress(LAMP_CEILING),
            &Self::LampDesk => self.decompress(LAMP_DESK),
            &Self::LampFloor => self.decompress(LAMP_FLOOR),
            &Self::LampWallDown => self.decompress(LAMP_WALL_DOWN),
            &Self::LampWallUp => self.decompress(LAMP_WALL_UP),
            &Self::Lamp => self.decompress(LAMP),
            &Self::Landmark => self.decompress(LANDMARK),
            &Self::Languages => self.decompress(LANGUAGES),
            &Self::Laptop2 => self.decompress(LAPTOP_2),
            &Self::Laptop => self.decompress(LAPTOP),
            &Self::LassoSelect => self.decompress(LASSO_SELECT),
            &Self::Lasso => self.decompress(LASSO),
            &Self::Laugh => self.decompress(LAUGH),
            &Self::Layers => self.decompress(LAYERS),
            &Self::LayoutDashboard => self.decompress(LAYOUT_DASHBOARD),
            &Self::LayoutGrid => self.decompress(LAYOUT_GRID),
            &Self::LayoutList => self.decompress(LAYOUT_LIST),
            &Self::LayoutPanelLeft => self.decompress(LAYOUT_PANEL_LEFT),
            &Self::LayoutPanelTop => self.decompress(LAYOUT_PANEL_TOP),
            &Self::LayoutTemplate => self.decompress(LAYOUT_TEMPLATE),
            &Self::Layout => self.decompress(LAYOUT),
            &Self::Leaf => self.decompress(LEAF),
            &Self::LeafyGreen => self.decompress(LEAFY_GREEN),
            &Self::Library => self.decompress(LIBRARY),
            &Self::LifeBuoy => self.decompress(LIFE_BUOY),
            &Self::Ligature => self.decompress(LIGATURE),
            &Self::LightbulbOff => self.decompress(LIGHTBULB_OFF),
            &Self::Lightbulb => self.decompress(LIGHTBULB),
            &Self::LineChart => self.decompress(LINE_CHART),
            &Self::Link2Off => self.decompress(LINK_2_OFF),
            &Self::Link2 => self.decompress(LINK_2),
            &Self::Link => self.decompress(LINK),
            &Self::Linkedin => self.decompress(LINKEDIN),
            &Self::ListChecks => self.decompress(LIST_CHECKS),
            &Self::ListEnd => self.decompress(LIST_END),
            &Self::ListFilter => self.decompress(LIST_FILTER),
            &Self::ListMinus => self.decompress(LIST_MINUS),
            &Self::ListMusic => self.decompress(LIST_MUSIC),
            &Self::ListOrdered => self.decompress(LIST_ORDERED),
            &Self::ListPlus => self.decompress(LIST_PLUS),
            &Self::ListRestart => self.decompress(LIST_RESTART),
            &Self::ListStart => self.decompress(LIST_START),
            &Self::ListTodo => self.decompress(LIST_TODO),
            &Self::ListTree => self.decompress(LIST_TREE),
            &Self::ListVideo => self.decompress(LIST_VIDEO),
            &Self::ListX => self.decompress(LIST_X),
            &Self::List => self.decompress(LIST),
            &Self::Loader2 => self.decompress(LOADER_2),
            &Self::Loader => self.decompress(LOADER),
            &Self::LocateFixed => self.decompress(LOCATE_FIXED),
            &Self::LocateOff => self.decompress(LOCATE_OFF),
            &Self::Locate => self.decompress(LOCATE),
            &Self::Lock => self.decompress(LOCK),
            &Self::LogIn => self.decompress(LOG_IN),
            &Self::LogOut => self.decompress(LOG_OUT),
            &Self::Lollipop => self.decompress(LOLLIPOP),
            &Self::Luggage => self.decompress(LUGGAGE),
            &Self::MSquare => self.decompress(M_SQUARE),
            &Self::Magnet => self.decompress(MAGNET),
            &Self::MailCheck => self.decompress(MAIL_CHECK),
            &Self::MailMinus => self.decompress(MAIL_MINUS),
            &Self::MailOpen => self.decompress(MAIL_OPEN),
            &Self::MailPlus => self.decompress(MAIL_PLUS),
            &Self::MailQuestion => self.decompress(MAIL_QUESTION),
            &Self::MailSearch => self.decompress(MAIL_SEARCH),
            &Self::MailWarning => self.decompress(MAIL_WARNING),
            &Self::MailX => self.decompress(MAIL_X),
            &Self::Mail => self.decompress(MAIL),
            &Self::Mailbox => self.decompress(MAILBOX),
            &Self::Mails => self.decompress(MAILS),
            &Self::MapPinOff => self.decompress(MAP_PIN_OFF),
            &Self::MapPin => self.decompress(MAP_PIN),
            &Self::Map => self.decompress(MAP),
            &Self::Martini => self.decompress(MARTINI),
            &Self::Maximize2 => self.decompress(MAXIMIZE_2),
            &Self::Maximize => self.decompress(MAXIMIZE),
            &Self::Medal => self.decompress(MEDAL),
            &Self::MegaphoneOff => self.decompress(MEGAPHONE_OFF),
            &Self::Megaphone => self.decompress(MEGAPHONE),
            &Self::Meh => self.decompress(MEH),
            &Self::MemoryStick => self.decompress(MEMORY_STICK),
            &Self::MenuSquare => self.decompress(MENU_SQUARE),
            &Self::Menu => self.decompress(MENU),
            &Self::Merge => self.decompress(MERGE),
            &Self::MessageCircle => self.decompress(MESSAGE_CIRCLE),
            &Self::MessageSquareDashed => self.decompress(MESSAGE_SQUARE_DASHED),
            &Self::MessageSquarePlus => self.decompress(MESSAGE_SQUARE_PLUS),
            &Self::MessageSquare => self.decompress(MESSAGE_SQUARE),
            &Self::MessagesSquare => self.decompress(MESSAGES_SQUARE),
            &Self::Mic2 => self.decompress(MIC_2),
            &Self::MicOff => self.decompress(MIC_OFF),
            &Self::Mic => self.decompress(MIC),
            &Self::Microscope => self.decompress(MICROSCOPE),
            &Self::Microwave => self.decompress(MICROWAVE),
            &Self::Milestone => self.decompress(MILESTONE),
            &Self::MilkOff => self.decompress(MILK_OFF),
            &Self::Milk => self.decompress(MILK),
            &Self::Minimize2 => self.decompress(MINIMIZE_2),
            &Self::Minimize => self.decompress(MINIMIZE),
            &Self::MinusCircle => self.decompress(MINUS_CIRCLE),
            &Self::MinusSquare => self.decompress(MINUS_SQUARE),
            &Self::Minus => self.decompress(MINUS),
            &Self::MonitorCheck => self.decompress(MONITOR_CHECK),
            &Self::MonitorDot => self.decompress(MONITOR_DOT),
            &Self::MonitorDown => self.decompress(MONITOR_DOWN),
            &Self::MonitorOff => self.decompress(MONITOR_OFF),
            &Self::MonitorPause => self.decompress(MONITOR_PAUSE),
            &Self::MonitorPlay => self.decompress(MONITOR_PLAY),
            &Self::MonitorSmartphone => self.decompress(MONITOR_SMARTPHONE),
            &Self::MonitorSpeaker => self.decompress(MONITOR_SPEAKER),
            &Self::MonitorStop => self.decompress(MONITOR_STOP),
            &Self::MonitorUp => self.decompress(MONITOR_UP),
            &Self::MonitorX => self.decompress(MONITOR_X),
            &Self::Monitor => self.decompress(MONITOR),
            &Self::MoonStar => self.decompress(MOON_STAR),
            &Self::Moon => self.decompress(MOON),
            &Self::MoreHorizontal => self.decompress(MORE_HORIZONTAL),
            &Self::MoreVertical => self.decompress(MORE_VERTICAL),
            &Self::MountainSnow => self.decompress(MOUNTAIN_SNOW),
            &Self::Mountain => self.decompress(MOUNTAIN),
            &Self::MousePointer2 => self.decompress(MOUSE_POINTER_2),
            &Self::MousePointerClick => self.decompress(MOUSE_POINTER_CLICK),
            &Self::MousePointerSquareDashed => self.decompress(MOUSE_POINTER_SQUARE_DASHED),
            &Self::MousePointerSquare => self.decompress(MOUSE_POINTER_SQUARE),
            &Self::MousePointer => self.decompress(MOUSE_POINTER),
            &Self::Mouse => self.decompress(MOUSE),
            &Self::Move3D => self.decompress(MOVE_3_D),
            &Self::MoveDiagonal2 => self.decompress(MOVE_DIAGONAL_2),
            &Self::MoveDiagonal => self.decompress(MOVE_DIAGONAL),
            &Self::MoveDownLeft => self.decompress(MOVE_DOWN_LEFT),
            &Self::MoveDownRight => self.decompress(MOVE_DOWN_RIGHT),
            &Self::MoveDown => self.decompress(MOVE_DOWN),
            &Self::MoveHorizontal => self.decompress(MOVE_HORIZONTAL),
            &Self::MoveLeft => self.decompress(MOVE_LEFT),
            &Self::MoveRight => self.decompress(MOVE_RIGHT),
            &Self::MoveUpLeft => self.decompress(MOVE_UP_LEFT),
            &Self::MoveUpRight => self.decompress(MOVE_UP_RIGHT),
            &Self::MoveUp => self.decompress(MOVE_UP),
            &Self::MoveVertical => self.decompress(MOVE_VERTICAL),
            &Self::Move => self.decompress(MOVE),
            &Self::Music2 => self.decompress(MUSIC_2),
            &Self::Music3 => self.decompress(MUSIC_3),
            &Self::Music4 => self.decompress(MUSIC_4),
            &Self::Music => self.decompress(MUSIC),
            &Self::Navigation2Off => self.decompress(NAVIGATION_2_OFF),
            &Self::Navigation2 => self.decompress(NAVIGATION_2),
            &Self::NavigationOff => self.decompress(NAVIGATION_OFF),
            &Self::Navigation => self.decompress(NAVIGATION),
            &Self::Network => self.decompress(NETWORK),
            &Self::Newspaper => self.decompress(NEWSPAPER),
            &Self::Nfc => self.decompress(NFC),
            &Self::NutOff => self.decompress(NUT_OFF),
            &Self::Nut => self.decompress(NUT),
            &Self::Octagon => self.decompress(OCTAGON),
            &Self::Option => self.decompress(OPTION),
            &Self::Orbit => self.decompress(ORBIT),
            &Self::Outdent => self.decompress(OUTDENT),
            &Self::Package2 => self.decompress(PACKAGE_2),
            &Self::PackageCheck => self.decompress(PACKAGE_CHECK),
            &Self::PackageMinus => self.decompress(PACKAGE_MINUS),
            &Self::PackageOpen => self.decompress(PACKAGE_OPEN),
            &Self::PackagePlus => self.decompress(PACKAGE_PLUS),
            &Self::PackageSearch => self.decompress(PACKAGE_SEARCH),
            &Self::PackageX => self.decompress(PACKAGE_X),
            &Self::Package => self.decompress(PACKAGE),
            &Self::PaintBucket => self.decompress(PAINT_BUCKET),
            &Self::Paintbrush2 => self.decompress(PAINTBRUSH_2),
            &Self::Paintbrush => self.decompress(PAINTBRUSH),
            &Self::Palette => self.decompress(PALETTE),
            &Self::Palmtree => self.decompress(PALMTREE),
            &Self::PanelBottomClose => self.decompress(PANEL_BOTTOM_CLOSE),
            &Self::PanelBottomInactive => self.decompress(PANEL_BOTTOM_INACTIVE),
            &Self::PanelBottomOpen => self.decompress(PANEL_BOTTOM_OPEN),
            &Self::PanelBottom => self.decompress(PANEL_BOTTOM),
            &Self::PanelLeftClose => self.decompress(PANEL_LEFT_CLOSE),
            &Self::PanelLeftInactive => self.decompress(PANEL_LEFT_INACTIVE),
            &Self::PanelLeftOpen => self.decompress(PANEL_LEFT_OPEN),
            &Self::PanelLeft => self.decompress(PANEL_LEFT),
            &Self::PanelRightClose => self.decompress(PANEL_RIGHT_CLOSE),
            &Self::PanelRightInactive => self.decompress(PANEL_RIGHT_INACTIVE),
            &Self::PanelRightOpen => self.decompress(PANEL_RIGHT_OPEN),
            &Self::PanelRight => self.decompress(PANEL_RIGHT),
            &Self::PanelTopClose => self.decompress(PANEL_TOP_CLOSE),
            &Self::PanelTopInactive => self.decompress(PANEL_TOP_INACTIVE),
            &Self::PanelTopOpen => self.decompress(PANEL_TOP_OPEN),
            &Self::PanelTop => self.decompress(PANEL_TOP),
            &Self::Paperclip => self.decompress(PAPERCLIP),
            &Self::Parentheses => self.decompress(PARENTHESES),
            &Self::ParkingCircleOff => self.decompress(PARKING_CIRCLE_OFF),
            &Self::ParkingCircle => self.decompress(PARKING_CIRCLE),
            &Self::ParkingMeter => self.decompress(PARKING_METER),
            &Self::ParkingSquareOff => self.decompress(PARKING_SQUARE_OFF),
            &Self::ParkingSquare => self.decompress(PARKING_SQUARE),
            &Self::PartyPopper => self.decompress(PARTY_POPPER),
            &Self::PauseCircle => self.decompress(PAUSE_CIRCLE),
            &Self::PauseOctagon => self.decompress(PAUSE_OCTAGON),
            &Self::Pause => self.decompress(PAUSE),
            &Self::PawPrint => self.decompress(PAW_PRINT),
            &Self::PcCase => self.decompress(PC_CASE),
            &Self::PenLine => self.decompress(PEN_LINE),
            &Self::PenSquare => self.decompress(PEN_SQUARE),
            &Self::PenTool => self.decompress(PEN_TOOL),
            &Self::Pen => self.decompress(PEN),
            &Self::PencilLine => self.decompress(PENCIL_LINE),
            &Self::PencilRuler => self.decompress(PENCIL_RULER),
            &Self::Pencil => self.decompress(PENCIL),
            &Self::PercentCircle => self.decompress(PERCENT_CIRCLE),
            &Self::PercentDiamond => self.decompress(PERCENT_DIAMOND),
            &Self::PercentSquare => self.decompress(PERCENT_SQUARE),
            &Self::Percent => self.decompress(PERCENT),
            &Self::PersonStanding => self.decompress(PERSON_STANDING),
            &Self::PhoneCall => self.decompress(PHONE_CALL),
            &Self::PhoneForwarded => self.decompress(PHONE_FORWARDED),
            &Self::PhoneIncoming => self.decompress(PHONE_INCOMING),
            &Self::PhoneMissed => self.decompress(PHONE_MISSED),
            &Self::PhoneOff => self.decompress(PHONE_OFF),
            &Self::PhoneOutgoing => self.decompress(PHONE_OUTGOING),
            &Self::Phone => self.decompress(PHONE),
            &Self::PiSquare => self.decompress(PI_SQUARE),
            &Self::Pi => self.decompress(PI),
            &Self::PictureInPicture2 => self.decompress(PICTURE_IN_PICTURE_2),
            &Self::PictureInPicture => self.decompress(PICTURE_IN_PICTURE),
            &Self::PieChart => self.decompress(PIE_CHART),
            &Self::PiggyBank => self.decompress(PIGGY_BANK),
            &Self::PilcrowSquare => self.decompress(PILCROW_SQUARE),
            &Self::Pilcrow => self.decompress(PILCROW),
            &Self::Pill => self.decompress(PILL),
            &Self::PinOff => self.decompress(PIN_OFF),
            &Self::Pin => self.decompress(PIN),
            &Self::Pipette => self.decompress(PIPETTE),
            &Self::Pizza => self.decompress(PIZZA),
            &Self::PlaneLanding => self.decompress(PLANE_LANDING),
            &Self::PlaneTakeoff => self.decompress(PLANE_TAKEOFF),
            &Self::Plane => self.decompress(PLANE),
            &Self::PlayCircle => self.decompress(PLAY_CIRCLE),
            &Self::PlaySquare => self.decompress(PLAY_SQUARE),
            &Self::Play => self.decompress(PLAY),
            &Self::Plug2 => self.decompress(PLUG_2),
            &Self::PlugZap2 => self.decompress(PLUG_ZAP_2),
            &Self::PlugZap => self.decompress(PLUG_ZAP),
            &Self::Plug => self.decompress(PLUG),
            &Self::PlusCircle => self.decompress(PLUS_CIRCLE),
            &Self::PlusSquare => self.decompress(PLUS_SQUARE),
            &Self::Plus => self.decompress(PLUS),
            &Self::PocketKnife => self.decompress(POCKET_KNIFE),
            &Self::Pocket => self.decompress(POCKET),
            &Self::Podcast => self.decompress(PODCAST),
            &Self::Pointer => self.decompress(POINTER),
            &Self::Popcorn => self.decompress(POPCORN),
            &Self::Popsicle => self.decompress(POPSICLE),
            &Self::PoundSterling => self.decompress(POUND_STERLING),
            &Self::PowerOff => self.decompress(POWER_OFF),
            &Self::Power => self.decompress(POWER),
            &Self::Presentation => self.decompress(PRESENTATION),
            &Self::Printer => self.decompress(PRINTER),
            &Self::Projector => self.decompress(PROJECTOR),
            &Self::Puzzle => self.decompress(PUZZLE),
            &Self::QrCode => self.decompress(QR_CODE),
            &Self::Quote => self.decompress(QUOTE),
            &Self::Rabbit => self.decompress(RABBIT),
            &Self::Radar => self.decompress(RADAR),
            &Self::Radiation => self.decompress(RADIATION),
            &Self::RadioReceiver => self.decompress(RADIO_RECEIVER),
            &Self::RadioTower => self.decompress(RADIO_TOWER),
            &Self::Radio => self.decompress(RADIO),
            &Self::RailSymbol => self.decompress(RAIL_SYMBOL),
            &Self::Rainbow => self.decompress(RAINBOW),
            &Self::Rat => self.decompress(RAT),
            &Self::Ratio => self.decompress(RATIO),
            &Self::Receipt => self.decompress(RECEIPT),
            &Self::RectangleHorizontal => self.decompress(RECTANGLE_HORIZONTAL),
            &Self::RectangleVertical => self.decompress(RECTANGLE_VERTICAL),
            &Self::Recycle => self.decompress(RECYCLE),
            &Self::Redo2 => self.decompress(REDO_2),
            &Self::RedoDot => self.decompress(REDO_DOT),
            &Self::Redo => self.decompress(REDO),
            &Self::RefreshCcwDot => self.decompress(REFRESH_CCW_DOT),
            &Self::RefreshCcw => self.decompress(REFRESH_CCW),
            &Self::RefreshCwOff => self.decompress(REFRESH_CW_OFF),
            &Self::RefreshCw => self.decompress(REFRESH_CW),
            &Self::Refrigerator => self.decompress(REFRIGERATOR),
            &Self::Regex => self.decompress(REGEX),
            &Self::RemoveFormatting => self.decompress(REMOVE_FORMATTING),
            &Self::Repeat1 => self.decompress(REPEAT_1),
            &Self::Repeat2 => self.decompress(REPEAT_2),
            &Self::Repeat => self.decompress(REPEAT),
            &Self::ReplaceAll => self.decompress(REPLACE_ALL),
            &Self::Replace => self.decompress(REPLACE),
            &Self::ReplyAll => self.decompress(REPLY_ALL),
            &Self::Reply => self.decompress(REPLY),
            &Self::Rewind => self.decompress(REWIND),
            &Self::Rocket => self.decompress(ROCKET),
            &Self::RockingChair => self.decompress(ROCKING_CHAIR),
            &Self::RollerCoaster => self.decompress(ROLLER_COASTER),
            &Self::Rotate3D => self.decompress(ROTATE_3_D),
            &Self::RotateCcw => self.decompress(ROTATE_CCW),
            &Self::RotateCw => self.decompress(ROTATE_CW),
            &Self::Router => self.decompress(ROUTER),
            &Self::Rows => self.decompress(ROWS),
            &Self::Rss => self.decompress(RSS),
            &Self::Ruler => self.decompress(RULER),
            &Self::RussianRuble => self.decompress(RUSSIAN_RUBLE),
            &Self::Sailboat => self.decompress(SAILBOAT),
            &Self::Salad => self.decompress(SALAD),
            &Self::Sandwich => self.decompress(SANDWICH),
            &Self::SatelliteDish => self.decompress(SATELLITE_DISH),
            &Self::Satellite => self.decompress(SATELLITE),
            &Self::SaveAll => self.decompress(SAVE_ALL),
            &Self::Save => self.decompress(SAVE),
            &Self::Scale3D => self.decompress(SCALE_3_D),
            &Self::Scale => self.decompress(SCALE),
            &Self::Scaling => self.decompress(SCALING),
            &Self::ScanFace => self.decompress(SCAN_FACE),
            &Self::ScanLine => self.decompress(SCAN_LINE),
            &Self::Scan => self.decompress(SCAN),
            &Self::ScatterChart => self.decompress(SCATTER_CHART),
            &Self::School2 => self.decompress(SCHOOL_2),
            &Self::School => self.decompress(SCHOOL),
            &Self::ScissorsLineDashed => self.decompress(SCISSORS_LINE_DASHED),
            &Self::ScissorsSquareDashedBottom => self.decompress(SCISSORS_SQUARE_DASHED_BOTTOM),
            &Self::ScissorsSquare => self.decompress(SCISSORS_SQUARE),
            &Self::Scissors => self.decompress(SCISSORS),
            &Self::ScreenShareOff => self.decompress(SCREEN_SHARE_OFF),
            &Self::ScreenShare => self.decompress(SCREEN_SHARE),
            &Self::ScrollText => self.decompress(SCROLL_TEXT),
            &Self::Scroll => self.decompress(SCROLL),
            &Self::SearchCheck => self.decompress(SEARCH_CHECK),
            &Self::SearchCode => self.decompress(SEARCH_CODE),
            &Self::SearchSlash => self.decompress(SEARCH_SLASH),
            &Self::SearchX => self.decompress(SEARCH_X),
            &Self::Search => self.decompress(SEARCH),
            &Self::SendHorizontal => self.decompress(SEND_HORIZONTAL),
            &Self::SendToBack => self.decompress(SEND_TO_BACK),
            &Self::Send => self.decompress(SEND),
            &Self::SeparatorHorizontal => self.decompress(SEPARATOR_HORIZONTAL),
            &Self::SeparatorVertical => self.decompress(SEPARATOR_VERTICAL),
            &Self::ServerCog => self.decompress(SERVER_COG),
            &Self::ServerCrash => self.decompress(SERVER_CRASH),
            &Self::ServerOff => self.decompress(SERVER_OFF),
            &Self::Server => self.decompress(SERVER),
            &Self::Settings2 => self.decompress(SETTINGS_2),
            &Self::Settings => self.decompress(SETTINGS),
            &Self::Shapes => self.decompress(SHAPES),
            &Self::Share2 => self.decompress(SHARE_2),
            &Self::Share => self.decompress(SHARE),
            &Self::Sheet => self.decompress(SHEET),
            &Self::Shell => self.decompress(SHELL),
            &Self::ShieldAlert => self.decompress(SHIELD_ALERT),
            &Self::ShieldBan => self.decompress(SHIELD_BAN),
            &Self::ShieldCheck => self.decompress(SHIELD_CHECK),
            &Self::ShieldEllipsis => self.decompress(SHIELD_ELLIPSIS),
            &Self::ShieldHalf => self.decompress(SHIELD_HALF),
            &Self::ShieldMinus => self.decompress(SHIELD_MINUS),
            &Self::ShieldOff => self.decompress(SHIELD_OFF),
            &Self::ShieldPlus => self.decompress(SHIELD_PLUS),
            &Self::ShieldQuestion => self.decompress(SHIELD_QUESTION),
            &Self::ShieldX => self.decompress(SHIELD_X),
            &Self::Shield => self.decompress(SHIELD),
            &Self::ShipWheel => self.decompress(SHIP_WHEEL),
            &Self::Ship => self.decompress(SHIP),
            &Self::Shirt => self.decompress(SHIRT),
            &Self::ShoppingBag => self.decompress(SHOPPING_BAG),
            &Self::ShoppingBasket => self.decompress(SHOPPING_BASKET),
            &Self::ShoppingCart => self.decompress(SHOPPING_CART),
            &Self::Shovel => self.decompress(SHOVEL),
            &Self::ShowerHead => self.decompress(SHOWER_HEAD),
            &Self::Shrink => self.decompress(SHRINK),
            &Self::Shrub => self.decompress(SHRUB),
            &Self::Shuffle => self.decompress(SHUFFLE),
            &Self::SigmaSquare => self.decompress(SIGMA_SQUARE),
            &Self::Sigma => self.decompress(SIGMA),
            &Self::SignalHigh => self.decompress(SIGNAL_HIGH),
            &Self::SignalLow => self.decompress(SIGNAL_LOW),
            &Self::SignalMedium => self.decompress(SIGNAL_MEDIUM),
            &Self::SignalZero => self.decompress(SIGNAL_ZERO),
            &Self::Signal => self.decompress(SIGNAL),
            &Self::Siren => self.decompress(SIREN),
            &Self::SkipBack => self.decompress(SKIP_BACK),
            &Self::SkipForward => self.decompress(SKIP_FORWARD),
            &Self::Skull => self.decompress(SKULL),
            &Self::Slack => self.decompress(SLACK),
            &Self::Slash => self.decompress(SLASH),
            &Self::Slice => self.decompress(SLICE),
            &Self::SlidersHorizontal => self.decompress(SLIDERS_HORIZONTAL),
            &Self::Sliders => self.decompress(SLIDERS),
            &Self::SmartphoneCharging => self.decompress(SMARTPHONE_CHARGING),
            &Self::SmartphoneNfc => self.decompress(SMARTPHONE_NFC),
            &Self::Smartphone => self.decompress(SMARTPHONE),
            &Self::SmilePlus => self.decompress(SMILE_PLUS),
            &Self::Smile => self.decompress(SMILE),
            &Self::Snail => self.decompress(SNAIL),
            &Self::Snowflake => self.decompress(SNOWFLAKE),
            &Self::Sofa => self.decompress(SOFA),
            &Self::Soup => self.decompress(SOUP),
            &Self::Space => self.decompress(SPACE),
            &Self::Spade => self.decompress(SPADE),
            &Self::Sparkle => self.decompress(SPARKLE),
            &Self::Sparkles => self.decompress(SPARKLES),
            &Self::Speaker => self.decompress(SPEAKER),
            &Self::SpellCheck2 => self.decompress(SPELL_CHECK_2),
            &Self::SpellCheck => self.decompress(SPELL_CHECK),
            &Self::Spline => self.decompress(SPLINE),
            &Self::SplitSquareHorizontal => self.decompress(SPLIT_SQUARE_HORIZONTAL),
            &Self::SplitSquareVertical => self.decompress(SPLIT_SQUARE_VERTICAL),
            &Self::Split => self.decompress(SPLIT),
            &Self::SprayCan => self.decompress(SPRAY_CAN),
            &Self::Sprout => self.decompress(SPROUT),
            &Self::SquareAsterisk => self.decompress(SQUARE_ASTERISK),
            &Self::SquareCode => self.decompress(SQUARE_CODE),
            &Self::SquareDashedBottomCode => self.decompress(SQUARE_DASHED_BOTTOM_CODE),
            &Self::SquareDashedBottom => self.decompress(SQUARE_DASHED_BOTTOM),
            &Self::SquareDot => self.decompress(SQUARE_DOT),
            &Self::SquareEqual => self.decompress(SQUARE_EQUAL),
            &Self::SquareSlash => self.decompress(SQUARE_SLASH),
            &Self::SquareStack => self.decompress(SQUARE_STACK),
            &Self::Square => self.decompress(SQUARE),
            &Self::Squirrel => self.decompress(SQUIRREL),
            &Self::Stamp => self.decompress(STAMP),
            &Self::StarHalf => self.decompress(STAR_HALF),
            &Self::StarOff => self.decompress(STAR_OFF),
            &Self::Star => self.decompress(STAR),
            &Self::StepBack => self.decompress(STEP_BACK),
            &Self::StepForward => self.decompress(STEP_FORWARD),
            &Self::Stethoscope => self.decompress(STETHOSCOPE),
            &Self::Sticker => self.decompress(STICKER),
            &Self::StickyNote => self.decompress(STICKY_NOTE),
            &Self::StopCircle => self.decompress(STOP_CIRCLE),
            &Self::Store => self.decompress(STORE),
            &Self::StretchHorizontal => self.decompress(STRETCH_HORIZONTAL),
            &Self::StretchVertical => self.decompress(STRETCH_VERTICAL),
            &Self::Strikethrough => self.decompress(STRIKETHROUGH),
            &Self::Subscript => self.decompress(SUBSCRIPT),
            &Self::Subtitles => self.decompress(SUBTITLES),
            &Self::SunDim => self.decompress(SUN_DIM),
            &Self::SunMedium => self.decompress(SUN_MEDIUM),
            &Self::SunMoon => self.decompress(SUN_MOON),
            &Self::SunSnow => self.decompress(SUN_SNOW),
            &Self::Sun => self.decompress(SUN),
            &Self::Sunrise => self.decompress(SUNRISE),
            &Self::Sunset => self.decompress(SUNSET),
            &Self::Superscript => self.decompress(SUPERSCRIPT),
            &Self::SwissFranc => self.decompress(SWISS_FRANC),
            &Self::SwitchCamera => self.decompress(SWITCH_CAMERA),
            &Self::Sword => self.decompress(SWORD),
            &Self::Swords => self.decompress(SWORDS),
            &Self::Syringe => self.decompress(SYRINGE),
            &Self::Table2 => self.decompress(TABLE_2),
            &Self::TableProperties => self.decompress(TABLE_PROPERTIES),
            &Self::Table => self.decompress(TABLE),
            &Self::TabletSmartphone => self.decompress(TABLET_SMARTPHONE),
            &Self::Tablet => self.decompress(TABLET),
            &Self::Tablets => self.decompress(TABLETS),
            &Self::Tag => self.decompress(TAG),
            &Self::Tags => self.decompress(TAGS),
            &Self::Tally1 => self.decompress(TALLY_1),
            &Self::Tally2 => self.decompress(TALLY_2),
            &Self::Tally3 => self.decompress(TALLY_3),
            &Self::Tally4 => self.decompress(TALLY_4),
            &Self::Tally5 => self.decompress(TALLY_5),
            &Self::Target => self.decompress(TARGET),
            &Self::Tent => self.decompress(TENT),
            &Self::TerminalSquare => self.decompress(TERMINAL_SQUARE),
            &Self::Terminal => self.decompress(TERMINAL),
            &Self::TestTube2 => self.decompress(TEST_TUBE_2),
            &Self::TestTube => self.decompress(TEST_TUBE),
            &Self::TestTubes => self.decompress(TEST_TUBES),
            &Self::TextCursorInput => self.decompress(TEXT_CURSOR_INPUT),
            &Self::TextCursor => self.decompress(TEXT_CURSOR),
            &Self::TextQuote => self.decompress(TEXT_QUOTE),
            &Self::TextSelect => self.decompress(TEXT_SELECT),
            &Self::Text => self.decompress(TEXT),
            &Self::ThermometerSnowflake => self.decompress(THERMOMETER_SNOWFLAKE),
            &Self::ThermometerSun => self.decompress(THERMOMETER_SUN),
            &Self::Thermometer => self.decompress(THERMOMETER),
            &Self::ThumbsDown => self.decompress(THUMBS_DOWN),
            &Self::ThumbsUp => self.decompress(THUMBS_UP),
            &Self::Ticket => self.decompress(TICKET),
            &Self::TimerOff => self.decompress(TIMER_OFF),
            &Self::TimerReset => self.decompress(TIMER_RESET),
            &Self::Timer => self.decompress(TIMER),
            &Self::ToggleLeft => self.decompress(TOGGLE_LEFT),
            &Self::ToggleRight => self.decompress(TOGGLE_RIGHT),
            &Self::Tornado => self.decompress(TORNADO),
            &Self::TouchpadOff => self.decompress(TOUCHPAD_OFF),
            &Self::Touchpad => self.decompress(TOUCHPAD),
            &Self::TowerControl => self.decompress(TOWER_CONTROL),
            &Self::ToyBrick => self.decompress(TOY_BRICK),
            &Self::Tractor => self.decompress(TRACTOR),
            &Self::TrafficCone => self.decompress(TRAFFIC_CONE),
            &Self::TrainFrontTunnel => self.decompress(TRAIN_FRONT_TUNNEL),
            &Self::TrainFront => self.decompress(TRAIN_FRONT),
            &Self::TrainTrack => self.decompress(TRAIN_TRACK),
            &Self::TramFront => self.decompress(TRAM_FRONT),
            &Self::Trash2 => self.decompress(TRASH_2),
            &Self::Trash => self.decompress(TRASH),
            &Self::TreeDeciduous => self.decompress(TREE_DECIDUOUS),
            &Self::TreePine => self.decompress(TREE_PINE),
            &Self::Trees => self.decompress(TREES),
            &Self::Trello => self.decompress(TRELLO),
            &Self::TrendingDown => self.decompress(TRENDING_DOWN),
            &Self::TrendingUp => self.decompress(TRENDING_UP),
            &Self::TriangleRight => self.decompress(TRIANGLE_RIGHT),
            &Self::Triangle => self.decompress(TRIANGLE),
            &Self::Trophy => self.decompress(TROPHY),
            &Self::Truck => self.decompress(TRUCK),
            &Self::Turtle => self.decompress(TURTLE),
            &Self::Tv2 => self.decompress(TV_2),
            &Self::Tv => self.decompress(TV),
            &Self::Twitch => self.decompress(TWITCH),
            &Self::Twitter => self.decompress(TWITTER),
            &Self::Type => self.decompress(TYPE),
            &Self::Umbrella => self.decompress(UMBRELLA),
            &Self::Underline => self.decompress(UNDERLINE),
            &Self::Undo2 => self.decompress(UNDO_2),
            &Self::UndoDot => self.decompress(UNDO_DOT),
            &Self::Undo => self.decompress(UNDO),
            &Self::UnfoldHorizontal => self.decompress(UNFOLD_HORIZONTAL),
            &Self::UnfoldVertical => self.decompress(UNFOLD_VERTICAL),
            &Self::Ungroup => self.decompress(UNGROUP),
            &Self::Unlink2 => self.decompress(UNLINK_2),
            &Self::Unlink => self.decompress(UNLINK),
            &Self::Unlock => self.decompress(UNLOCK),
            &Self::Unplug => self.decompress(UNPLUG),
            &Self::UploadCloud => self.decompress(UPLOAD_CLOUD),
            &Self::Upload => self.decompress(UPLOAD),
            &Self::Usb => self.decompress(USB),
            &Self::User2 => self.decompress(USER_2),
            &Self::UserCheck2 => self.decompress(USER_CHECK_2),
            &Self::UserCheck => self.decompress(USER_CHECK),
            &Self::UserCircle2 => self.decompress(USER_CIRCLE_2),
            &Self::UserCircle => self.decompress(USER_CIRCLE),
            &Self::UserCog2 => self.decompress(USER_COG_2),
            &Self::UserCog => self.decompress(USER_COG),
            &Self::UserMinus2 => self.decompress(USER_MINUS_2),
            &Self::UserMinus => self.decompress(USER_MINUS),
            &Self::UserPlus2 => self.decompress(USER_PLUS_2),
            &Self::UserPlus => self.decompress(USER_PLUS),
            &Self::UserSquare2 => self.decompress(USER_SQUARE_2),
            &Self::UserSquare => self.decompress(USER_SQUARE),
            &Self::UserX2 => self.decompress(USER_X_2),
            &Self::UserX => self.decompress(USER_X),
            &Self::User => self.decompress(USER),
            &Self::Users2 => self.decompress(USERS_2),
            &Self::Users => self.decompress(USERS),
            &Self::UtensilsCrossed => self.decompress(UTENSILS_CROSSED),
            &Self::Utensils => self.decompress(UTENSILS),
            &Self::UtilityPole => self.decompress(UTILITY_POLE),
            &Self::Variable => self.decompress(VARIABLE),
            &Self::Vegan => self.decompress(VEGAN),
            &Self::VenetianMask => self.decompress(VENETIAN_MASK),
            &Self::VibrateOff => self.decompress(VIBRATE_OFF),
            &Self::Vibrate => self.decompress(VIBRATE),
            &Self::VideoOff => self.decompress(VIDEO_OFF),
            &Self::Video => self.decompress(VIDEO),
            &Self::Videotape => self.decompress(VIDEOTAPE),
            &Self::View => self.decompress(VIEW),
            &Self::Voicemail => self.decompress(VOICEMAIL),
            &Self::Volume1 => self.decompress(VOLUME_1),
            &Self::Volume2 => self.decompress(VOLUME_2),
            &Self::VolumeX => self.decompress(VOLUME_X),
            &Self::Volume => self.decompress(VOLUME),
            &Self::Vote => self.decompress(VOTE),
            &Self::Wallet2 => self.decompress(WALLET_2),
            &Self::WalletCards => self.decompress(WALLET_CARDS),
            &Self::Wallet => self.decompress(WALLET),
            &Self::Wallpaper => self.decompress(WALLPAPER),
            &Self::Wand2 => self.decompress(WAND_2),
            &Self::Wand => self.decompress(WAND),
            &Self::Warehouse => self.decompress(WAREHOUSE),
            &Self::Watch => self.decompress(WATCH),
            &Self::Waves => self.decompress(WAVES),
            &Self::Webcam => self.decompress(WEBCAM),
            &Self::Webhook => self.decompress(WEBHOOK),
            &Self::WheatOff => self.decompress(WHEAT_OFF),
            &Self::Wheat => self.decompress(WHEAT),
            &Self::WholeWord => self.decompress(WHOLE_WORD),
            &Self::WifiOff => self.decompress(WIFI_OFF),
            &Self::Wifi => self.decompress(WIFI),
            &Self::Wind => self.decompress(WIND),
            &Self::WineOff => self.decompress(WINE_OFF),
            &Self::Wine => self.decompress(WINE),
            &Self::Workflow => self.decompress(WORKFLOW),
            &Self::WrapText => self.decompress(WRAP_TEXT),
            &Self::Wrench => self.decompress(WRENCH),
            &Self::XCircle => self.decompress(X_CIRCLE),
            &Self::XOctagon => self.decompress(X_OCTAGON),
            &Self::XSquare => self.decompress(X_SQUARE),
            &Self::X => self.decompress(X),
            &Self::Youtube => self.decompress(YOUTUBE),
            &Self::ZapOff => self.decompress(ZAP_OFF),
            &Self::Zap => self.decompress(ZAP),
            &Self::ZoomIn => self.decompress(ZOOM_IN),
            &Self::ZoomOut => self.decompress(ZOOM_OUT),
        }
    }
}
