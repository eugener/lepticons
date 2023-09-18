
use base64::*;
use flate2::read::ZlibDecoder;
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
    BookmarkCheck,
    BookmarkMinus,
    BookmarkPlus,
    BookmarkX,
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
    Drama,
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
    Speech,
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
    Theater,
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
const ACCESSIBILITY: &'static str = r#"eJxtzUEKwjAQBdCrfLLP6HQykwhJb+AhShQUFKS40Nub6MJCyyw+w3/wc73O9XbGXBw71HdxocWrfebGvPu1Y35MzwtOxd05gQ9gH72Bu+jNolckiBcoacuBxCCka3gMNARwIJ0Uiv33jFKCbWAWim0vLrXv2v/1B98OMq0="#;

const ACTIVITY_SQUARE: &'static str = r#"eJwlizEKgDAMAL8SshdtROrQdHbxEWKLKThICVh/L8Ht4O5iK4dC64yE8DJOCFLqKcroF4SnZpUfu8kUBxtSvHcVyIybD+BJHF2OYHbk/GiwBkstSh9FBRpE"#;

const ACTIVITY: &'static str = r#"eJyzKUgsyVBIsVXyNTJSMDTK0DXJ0TVWsPSxVDAGMzyMlOxs9EGK7AD2QAsc"#;

const AIR_VENT: &'static str = r#"eJxtT7sOwjAM/BWrewzn1E6GUImNhZU9EkNGBtTvx1EQZKj8kHxnn87lVd+NnpflbgS5rVVI6OyBIEEe+p/J5wabAZJd5wOSFmTZyqmLbmWSzg1HBDJHQuJUhZV6DaXIMIqcY8e+bTBgrO7rUM3Yf1C267CEXp5pD/rb/gCvNjbM"#;

const AIRPLAY: &'static str = r#"eJxNTb0KgCAYfJXDXer76Gcx55bWdiHIIFRIgt4+tSE5OLjjflQw0WKbxNKDxrkzDEabQJIlr/2vkbSloTbAN7V1A2wlCa2avKpV8Oeze4fgDxevSRCD8g2YUKgYJf8l9Qs3UyJk"#;

const ALARM_CHECK: &'static str = r#"eJxtzUEKgzAQheGrPGaftDOTJikk3sBDlLTQQgURF3p7DYIudDXwvh8mld9Q/h+UORMroUzrFcKQKVKTbhs3qX+NX7wztQ8oBL5a3Q7pZJ2NGj1T661GcLQBDsIXAQfrXS18gNwvm+4Jrq8FzridF6nOMts="#;

const ALARM_CLOCK_OFF: &'static str = r#"eJxtjjEOgCAMRa/y4061FSsk6A08BImDi4mD94+ARgfJb9/U1zYc8dywTs2i5EZkRAeHDpybSfRmM4c2z87hNdiTB1uS4VFSjCceCv7CLgI1vekru7QccqQjLIRrMlK6VBXbZgnfjxfqOjWs"#;

const ALARM_CLOCK: &'static str = r#"eJxti0EKgzAQRa/ymb1pZyZNUki8QQ9R0oKCgoiI3l6DG0FXH957P+Z2zN0feUnEQsjrvkoYEwWq4+PQdRy+U4Nfog8L3rPtBFJ0wSf5gkLgrqaXHVda6c3JGQ3gYDwshG8C9sbZUjgPeZ6bDZUjMvs="#;

const ALARM_MINUS: &'static str = r#"eJxtjUEKhDAQBL/SzN3szoybZCHxBz5CoqCgIOJBf6/Bgwc9NVQVdEjDksYOaYvEQkj7uUpYInmqwufSVZibtUcbqf5BIbDZZXabSU5caKFPVVujHuyNQwnhl4CdsWUurIN835s/WPv7+ACDmDIA"#;

const ALARM_PLUS: &'static str = r#"eJx1jUEKgzAQRa/ymb2pM2OTFBJvkEOUtGChhSKl6O1NcOFCXX1478EP+TXm9xNjJE/IcyTWslNZoT5cVt2H7/034BEpXaEQ2Ooq28xHCm600b1K1qgHe+PQQfggYGdsVwvrIO1JI+D2f/CcbmAdNrEA98E5Xw=="#;

const ALBUM: &'static str = r#"eJw1S10KgCAYu8r4LhBmoA/qZUpSEBUTytunRjD2wzZV7F5RHk0robQpPXDC7Y/qNDFJcNafrn6+jdKoZfyMyim04KNFTj7Wq08YODoNbJBgYlqBefrn5gXaYCD1"#;

const ALERT_CIRCLE: &'static str = r#"eJw9y7ENwCAMBMBVLA+QYAqUAljGokBCKajs7ZNgh+r1+vvMffJowFqQIsJ8IyCwrFrzaXvNo98NlApeCBINq6fQrz/l1tARaO/rTsl/afsHHj0h/Q=="#;

const ALERT_OCTAGON: &'static str = r#"eJxdjFEKgCAMQK8ydgBrI6wP8zYRgqhQH3r7rClCX4+9vc2k6MsZA6Town3tuKpNAwNpRUslM4jpqi2GHu1nRKM1U/tsjXfhgEw7EiOUyg0hcxuFNX+rXyuRmql1Ws4re/8ANl8t5Q=="#;

const ALERT_TRIANGLE: &'static str = r#"eJxtjDEKgDAUQ68S3H9tfj9aoQoewAu4CQ4dFBzE80sHdZFseclLx3JmrH21K10bwChRaItC4UskOIvwm0TQxreGQZnZfEOUv4S5GlJdpEN61BMV3WX/gG12ni+7AUBZIpk="#;

const ALIGN_CENTER_HORIZONTAL: &'static str = r#"eJxtjysOgDAMQK/S4Atts/BJxjQGi19ATCLIzs+KYIMsNf29vtSe/gpwzM0qwBKEGmc77Tn7TpiA+2i8gAClYEzZ0pc1SkRTR8ctk6SbBUl6KdZAeZz8dQaUv5RrUpOkw07ILbeTgpg+yySoNIM3FO0+Iw=="#;

const ALIGN_CENTER_VERTICAL: &'static str = r#"eJx1j7sOgDAIRX+FuKNAGh9JdXZxdTc6MDoYv1/awaqpYYKbc0/w+3IobH0xsYCcQsXgq3Ab/J20wDS6xXIgG0ZBmeuVkEsuO7Q7iroMyLWRmkiCSKY9NCm6rFNobN7OE+Uj5R+pU04oxMeeVWDShF5EPj45"#;

const ALIGN_CENTER: &'static str = r#"eJxtyzEKwCAMQNGrSC5QktJqIfU2DoI46+1NjDg5/eV9Lrkm1/AHQnBd+kpopkluiHypiTylCiQjWj3Rm/UnG5b5zDxrDdsOZ9Agtg=="#;

const ALIGN_END_HORIZONTAL: &'static str = r#"eJxNjEEKgCAURK/ymQtEHxEEv+s2HSJS0l2IkN0+FKxWA/Pmjc1hL1QFChRDOmIRzBp0CxiUa48r+RIFGs5Obe/ssGb1o00y340Z/mudW4nkBSszMS+dtM49CYwl1g=="#;

const ALIGN_END_VERTICAL: &'static str = r#"eJxFjMEJgDAMAFcJWUAaRBCabuDXv9hi+pMSULe3DVK/x935knYFSfkQZZwQbkZCuHJUYXQVPIwjQjEe/ND84K36pNkc16W2qOx/9urcVCAyLkRAtNqvsfACHBYl5A=="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: &'static str = r#"eJxtjMsOQDAURH/lZvaCUt30+gNbe0HUTqTx+HtKUEk3s5gzZ/Tct5bWsbOGUYB2hgTNG0OAzsxBph8HYxlpjlLHbl/qy9qu7hmfpoL39GrJp02NNdQxqlSREEskHXLlH6laBMCtZEEiPeMAtpw7uA=="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_END: &'static str = r#"eJxdjMsNgCAQRFvZTAMi8XNh6cAijBDhZghR7F7ggMbTbubNGxXsFslZv7vI6AfQzRhBiZH/kI8EXd5Ex5igVVf6WlWrxa8vqj83M9XNph1rdGQYSy9InlIUUrIPkT/yAIjzLYE="#;

const ALIGN_HORIZONTAL_DISTRIBUTE_START: &'static str = r#"eJxdjMsNgCAQBVvZvAZU4ufC0oFFGCHCzRDip3tZTZR42+zMPB3dnOhgtCDvwuITo8n3HmzyjB50MjpQzIqC0ZX4Rt/V69el/5gyKTu5Hr5snZInyxhbUpuqBcirAM2PXGKrLVc="#;

const ALIGN_HORIZONTAL_JUSTIFY_CENTER: &'static str = r#"eJxNjMsNgCAQRFvZTAMCUbywdGARRohwM4T46V5Bo54mmZn3TPJTpi26HBgatDMUKN0RfJxDZsgWdDA6WNOUvzWVemeBn+Fhr39fdVJ/2DLmQI4xSEVqVaIspbMnSscmKA=="#;

const ALIGN_HORIZONTAL_JUSTIFY_END: &'static str = r#"eJxFjMsNgCAQRFvZTAPCxs+FpQOLMEKEmyHET/cKBzjOzHtjkt8z3dHlIJhBwccjZIEeQY+AQa9gAqUarBkKb021Gqsqq3+4H6VmL107txzICVZm4otVWUpnP0oWJiU="#;

const ALIGN_HORIZONTAL_JUSTIFY_START: &'static str = r#"eJxVjFsKgCAURLdymQ2U0uPH6w5aRKSkfyFStvu6QkI/MzCPY5LfMgUf95AZagDdjBGUCkODruhyYEygImpNJ3tr6utXqtcapq+Y+cO027HmQI6xaNKn7qWQyD4fICX7"#;

const ALIGN_HORIZONTAL_SPACE_AROUND: &'static str = r#"eJxVyjEKgDAQRNGrLHMB4yKKkM0NbO3FBDedhAX19prOlDPv+5J2o1swgx7BBCrfYNCVo6lgBGnKh5qgdwi+q33w52ZKUbAMxLxyhXr9gF0rLyOXHZ4="#;

const ALIGN_HORIZONTAL_SPACE_BETWEEN: &'static str = r#"eJxVjMsNgCAQBVvZvAYU/F1YO7AII0S4GUL8dC+QqHjZw87MU94sgfzJkKB4G9DhdLCMHmSNW21giBZ0MTqMqkr+qHIVddGVfnQGPGNvXH/ZNgdLmjE1JHeZQXoVQIo/uQFHZC1U"#;

const ALIGN_JUSTIFY: &'static str = r#"eJxVjLENACAIBFcxLGDAxFig21iYGGvcXoSK6gJ//7zXmUmoAyGkq6wKNIiiwOD8ncHB9MwKSN5QRteeDcK63WH3AWVzIKQ="#;

const ALIGN_LEFT: &'static str = r#"eJxti7sNwCAMBVdBXiCyoyQUDtukiISoYXts82uoTk/vjuOfPlfwhRtcIUOWRSiUeULgQ53AZuqH1/gsRGqlcOM+y1XH98ZPtwJnMyCs"#;

const ALIGN_RIGHT: &'static str = r#"eJxdzEsKwCAMBNCrhLlASQr9QPQ2XRRK13p7E0VBV8MML9Hv/R9KHCAMShKwg7LVw0I8om5uolbpGwvGhdPabb8XO5mrmbO9tdptAWYZIK4="#;

const ALIGN_START_HORIZONTAL: &'static str = r#"eJxtjEEKgCAQRa8yzAXCQYTAcd2mQ0RK4y5koLx96qYW7T7/v/d9SbvCzWgRKqNDuHJUGam0mhAk5UOU0TgMfup88MOqf9T8fWiTsa91bioQGVcioIX60KvwAOPJJaU="#;

const ALIGN_START_VERTICAL: &'static str = r#"eJxNjEsKgDAMRK8ScgE1iCA0vYGHEFtMd1KCn9vbiL/lzLw3LsdJYUtBhbFH2Bk7hIOxaRFySYQgMc2iNnhXGe/dZX39Q752ke/P5mctowoExoGAVqptsMqf9IEl0Q=="#;

const ALIGN_VERTICAL_DISTRIBUTE_CENTER: &'static str = r#"eJxtzDEOgCAMQNGrNN2NgiILOLt4CKPGshlCot7ewoAMLB36+mv8sQV4LCqE16IYEOhwJwWLI4JnkAi32wMlnEwbg8mkjIMhH/HU/2lX/snVtQaC3eIiJWhqVJS4K0SDnkVlz4XgpK+QYpllhg+k8Tt/"#;

const ALIGN_VERTICAL_DISTRIBUTE_END: &'static str = r#"eJxVjFEKgCAQRK+y7AUysfpxvUGHiJTWv5CF6vZloeTfMG/e2BRWgZNwQOAQNxbCEeEi7A3CEb3wF9Oz0ehslwVnizYVUrfqtc3/rlr7IgyecNagFWuVSe4a0jfkBmd2LWU="#;

const ALIGN_VERTICAL_DISTRIBUTE_START: &'static str = r#"eJxVjEEKgCAQRa8yzAVKsdo43qBDRErjLmSgun0qZLj68B/v2RR2gZtwQkh5NMJDqAwCh3iwEM4IV/TC9XV2KIKzVfv+sUrZyYGldf5As85NGDzhqkEZ1mMh5etIB141ty07"#;

const ALIGN_VERTICAL_JUSTIFY_CENTER: &'static str = r#"eJxNjFEKgCAQRK+yzAVSKftRb9AhIqX1L0Sobp8bGH3tMjPvuZK2SpfHBCrtGNDtoS3ozLFye0cQp7xz9bAIbhAguBfrE/WfCN8szTV35Ucda2WKHoshbdgoaSQLDy1LJho="#;

const ALIGN_VERTICAL_JUSTIFY_END: &'static str = r#"eJxNjMsJgDAQRFtZpgF18XNJ0oFFiAlubhICmu7NCgYvMzCPNyaFPZOEeEi2mEFX9FkshhFUajEo3Ra1ak5wplPBmVer09L4J/b43xVlzTq3LOQtViZm4V6Jbu4BOw0mFw=="#;

const ALIGN_VERTICAL_JUSTIFY_START: &'static str = r#"eJxNjLsNgDAMBVexvABg8WnibMAQCCKcDkWWINvjhCKU757uXAq7goR4ijLOCHc8VBiHEeFhnBCyDePJFqF3XRG8q1quxvf8GwaWVuqbdW0qcDCuBCRUj4L8CxSwJe0="#;

const ALIGN_VERTICAL_SPACE_AROUND: &'static str = r#"eJxVyjEKgDAMRuGrhP8C1iCK0HR28RBii+kmJaDeXuuk6/ueL2k1OgUD6MjRVNA60CUYQZrypiboQeVZGME39Q9+X0wpCmZmYje9Uttfug/cKB4dgg=="#;

const ALIGN_VERTICAL_SPACE_BETWEEN: &'static str = r#"eJxVjEEKgCAURK/ymQtUlrVRb9AhIqXvLkSobp8KUq4G5s0bFdweKdwaAsTOHxw1ZlAqJOjyNrLGMIGeFBJGdVkwqmjtfimjEfWtyv1nnVtkshqrIDGwKCR3DRn/4AVOFS04"#;

const AMPERSAND: &'static str = r#"eJxFjT0OgCAMha/y4k4FCsiAJB7AQxAcGB28f2xZSNPlfe+nvO0beM7tdgdFON8tAgXDlJBNRr6CyPpWzkEskWK3RnFQC8XG4AlVtVLA8BLIsw+Ot1p2nalljSVBY5EfpI0eCQ=="#;

const AMPERSANDS: &'static str = r#"eJy1jbENgDAMBFex6B2cDyYghWzAEFEoKCnYXziA2ACd/osv/tJRzp22pVu9kI+VlQPHm7mAQGJ4GkiqEJyyvj223UVLoAe4yTZlOFhrl1PfznP6FMBPigsqKSkM"#;

const ANCHOR: &'static str = r#"eJwtjMEKgCAQRH9lmXuUG0EH13OXPiJMUJAI6aB/n2bsMjAzj9E2JBsd2SJYQDYLFIOSYIbRY2+NjuFylFUvCwtWUObf1pi50Y0y+j4eT6dgX0jxxoeaqH4/ruKHb7ph5gWEUSCu"#;

const ANGRY: &'static str = r#"eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vZpbopcDLN4b8bY2dllhL0aRpLBnrGdL8Fak0femq0/JgwN66gElTunTHHJixA8CXKQXmqVKdQgAf2HKxXQkP6FEkvtV5NIAFJfcgNEIjN7"#;

const ANNOYED: &'static str = r#"eJxtycEJgDAQRNFWhm1ANyhE2KQDi5BVWMGDBA/avcacAjkN85/onvTYoE8gdoT0TU/Q+79RuuJRzuUyrIFmDx7NZ8qpgslco/NQwQtIoCCJ"#;

const ANTENNA: &'static str = r#"eJx9zLsNwCAMhOFVTvR52Ipx47ABQ0RKQRMpRZT5kQuoEO19ut/e6yu4z5AZxFBwSLb5mKzRo06y0D4w4hlOnvlYBVpIBuTR+McuFZG1LR8="#;

const APERTURE: &'static str = r#"eJxtj0EKxCAMRa8iOUDGaBsVbC8jXRTKLLqyt58EhelMXX34efnJz2U/y7GZci1ADkypTU8RC2t+tfmaj/29mUpiT+gJTHULOIt2BnOJG0XEoYBp0jXF+9JtLKIRCTn1BEIKA15fUDKgj40kj2pqikPLfzu/qR7T3FBWtCXy6AzDoxNG37uMjhAjO/j+qJzF2PtJnVv9D7ZkVgw="#;

const APP_WINDOW: &'static str = r#"eJxdi00KgCAUBq/y+C6QikgL9QYdIlJ67kIe/dy+xE21mcUM42tehK4AC6pngAF1ci4rS4B2oKMk4ccqRD+0IfptFqYUMGlFdrctNPUKhkbuxy+4z3ADRVAkzg=="#;

const APPLE: &'static str = r#"eJxFjksKAkEMRK9SzD6xUybzgXFAXHsBd8O4cOnC+2O6G5SQBOoDb33vnxeel+FuBIsufpgGCqhTwLSM8P7OKY4y5xqVvLouhnZKG5sQh1QLRWrHXQIUE40x9WkWSgj37NRer2XE07oR5sg4MedJBnaIkM7Q3mPY1lMF3tY/dqYOQzKzTfwyX+slLWo="#;

const ARCHIVE_RESTORE: &'static str = r#"eJxtzE0KgzAQBeCrPN4+NDOpUCHxBj2EVOm4KBQJtr19ExeiIG8zPx8vzuMjw8bpaTmxIb6JSvwSA/GZhmxl98Rc7sIuXqrv4rvPhiHxfsVtEekVCr+mTKYVVrKD6o9SXJXuhL5aSIPgAkpOmkQhurTb5w8BLTOm"#;

const ARCHIVE_X: &'static str = r#"eJx1i7sOglAQRH9lMj3Krm7E5C5/YGtPhLgUJobc+Ph7uTbQkGkmM+ekabhlTB+nEDGM98hOI77OA/Ee+xxOrYmZULZpX/g2Pbsc6J2XI5qXSKdQ1P/MLUTXQ6XXppjFWczHeWeQE6yyrVNhWM4fFEMsUw=="#;

const ARCHIVE: &'static str = r#"eJxNizsKhEAQRK9SVL7sdK8LBtPewNRcVGwzkcHP7XUMRCop6tWLy9AlHMYfsexGIbapT27UQPgwjZ6Mf+Jiyip+87+Kc5scvbEuUK4irUIR7lzNRd/DR5sym9l5mRIg6sVDTgWIJCA="#;

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

const ARROW_DOWN_01: &'static str = r#"eJxtjTsKgDAQRK+ybB80McYm8Qa29qLiphAkBD+3NyuIKWSagTcfuw2RYHK4ViANaJbQ2NqCQWtf3DWgyj4DYR4jXA41wuGnSI+j2S8UHRqE06GsEUKKKG5xPpuTvLcLQ0L9nMk6UfrebuxILhQ="#;

const ARROW_DOWN_10: &'static str = r#"eJxtyzEKg0AUhOGrDNNL8jarNvv2BmnThyh5FoLIgnp7dwXBQqb85g/TNxk65fiCNPBllWcMjwIxnPxu4Z6fO5AWksUqd4d1Rrtkc/9LmDelI1al1IT1w9+SsiGWoUum9ER+yJGVIO7W9S30"#;

const ARROW_DOWN_AZ: &'static str = r#"eJx1jDEOgCAQBL+ysQfvkEML5Ae29CQWV2hi4f8jFMYGM9lsMdmNV7kV+zqcEzjAN4wfUhybSPHV2wxHuSccYVEjHcMCphysFGcFLVRh1M5MPwOvchhB0O/xAQRkKyE="#;

const ARROW_DOWN_CIRCLE: &'static str = r#"eJxFyUEKgDAMRNGrDNmLtnSRRdobeAiJgoKCFBG9vQ0Fyyw+zBPdsu4L9I3kPEGf2lwyUJK+epJzulbMkUbnwTcb2dXgYBQKti78/AEfxxp6"#;

const ARROW_DOWN_FROM_LINE: &'static str = r#"eJxlyaEKACEQRdFfediXZVycRRjNFqtdMFgEg/+PWDTIbfdIz6OiOBXJ4gtGeXnX8nJAQ1P6b2kMMuDVw5sns+EWXA=="#;

const ARROW_DOWN_LEFT_FROM_CIRCLE: &'static str = r#"eJxtijEKACEQA7+y2MtpiuOKPWsbHyFY2AgW4vvdLbSShBCY4Z5HpfKbBPLI3pFURqPfBH5UCbzFBoK4zt5g+gRGTPsetgC6/RlO"#;

const ARROW_DOWN_LEFT_SQUARE: &'static str = r#"eJxFy7EKgDAMBNBfCdlFqlAytJ1dXN3FFtNBkBJQ/95Ghy4hd49zJW0CV47CHg0hcMo7y//fHkeE57ulhgGD63UQ3LkKQ/R4GAvUEZCSlo3mSsZOtDR7AcbxHrk="#;

const ARROW_DOWN_LEFT: &'static str =
    r#"eJxNySEOACAMBMGvNP0AqTpzVGOweBIEEsH/Q6po1u3wzLtlVe0GiQzqLHGd2QwN49sDricPOQ=="#;

const ARROW_DOWN_NARROW_WIDE: &'static str = r#"eJx1yTEOgCAQBMCvbOiNLF7E4uQHtvYmFteYWPD/EAqoIFOO/k82vKf7NnCHVIu4pGuNpK2viODvUZAQm8RhcRwMRt+rAIoRJN8="#;

const ARROW_DOWN_RIGHT_FROM_CIRCLE: &'static str = r#"eJxtiqENACAMBFdp8IR+RVVhA4YgQSARhPmhBkX+8ubOZluDeg4VQiINTJd7Po7gUCx5U+yV4iXB+Vvo1hH1yQP8WBnp"#;

const ARROW_DOWN_RIGHT_SQUARE: &'static str = r#"eJxFyzsKgDAQhOGrDHsBiYJskaS28RBigptCkLD4uL1JmjDd/zE2x13xOpoIEtMh6sgw4WslFxgJTwoqrXs71IO316aC4OhktFWprctqZvDNS6cffT0ebg=="#;

const ARROW_DOWN_RIGHT: &'static str =
    r#"eJw9ySEOACAIQNGrMC4gJAqSLR7CzWBxMzjPLwbZfvpPV9sDesYpIMDkoWl61/RbZcfDVCTsArN7D2Q="#;

const ARROW_DOWN_SQUARE: &'static str = r#"eJxFy00KgCAQBeCrPGYfoblwod6gQ0RK4yIIGfq5feqiGBh47+O5klbBlaOwJ2UJt6eJ8PRfatAETnlj6Rzc2AbBHYswoqdZadizQ6t+2C0qmXaD+fgF3dwewg=="#;

const ARROW_DOWN_TO_DOT: &'static str = r#"eJxFikEKgDAMBL8Sci+SIJRCkx/4CImCgoIUEf29DSJlD8vuTD7Gc4FJcCAGvqhHzZ1/mn+yU4IUIsRQ07CtxbYZiiAh2CPI3ned7NKH9QUMARqf"#;

const ARROW_DOWN_TO_LINE: &'static str = r#"eJxlyaEKwCAUheFXOdjHOI7dMbgzr1jtgsEiGHx/xKBF/vZ/WmPLSJ/xtOATLuP0HM/plCIgIaNDdvZ8YfnfSzq25haJ"#;

const ARROW_DOWN_UP: &'static str = r#"eJxtyzEKwCAQBdGrfOwlWbNoio03SJs+YGEjWIjnly20kikfI/VvGekx5QJ5sGbZRDkUokx+A9z5baA4wm1Zw+6jAO7klwzAeR6i"#;

const ARROW_DOWN_WIDE_NARROW: &'static str = r#"eJxtzDEKgDAQRNGrDOnFTFyMxZob2NoLFtsIFrk/IUVIs/zywdf/qYb3DN8G7pDeIqHo2qHo4CsjxdsDEmKMvhyWfWCyOWuEkCTf"#;

const ARROW_DOWN_ZA: &'static str = r#"eJxtzDEOgCAMBdCrNOwglBYdkBu4upM4dNDEwXB+y2BcyE/TJi/9+a6PwLGaK0JIQD2WTMlTh5I/3magFtIAAgMJn5YhCY8dfbPRcUWnt47XBNC9ox98oOoi9i97AUmrK5I="#;

const ARROW_DOWN: &'static str =
    r#"eJw9yUEKABAQQNGrTPbSTDSpMTdwCGVhoyzk/FjQ3/0no8wGNZmMBGGhNyruPpUnHSMgWQa2p+8b3MgPwg=="#;

const ARROW_LEFT_CIRCLE: &'static str = r#"eJxFyTEKgDAQRNGrDNuL7iKSYpPaxkPIKigoSLDQ25tEUKb4ME9tjbbNiJ64IdidKqlXadD69aDHeC6YPA3cgaV32fL3y84CV7Uo+/gBLIcaew=="#;

const ARROW_LEFT_FROM_LINE: &'static str = r#"eJxtySsKQCEURdGpHOyPx/UHwtUZWO2CwSIYnD+ioEl224t7HhXFi+ZgP4udCPyvH/hoVCBZST9EEsglc2UCmzUWfQ=="#;

const ARROW_LEFT_RIGHT: &'static str = r#"eJxtyzsOgCAQhOGrTOiNLm7AYqW28RAmFhSaUHD/8CiggEw3X34JT/R4T3Uf2MGwH4OVk7X8Tppm8WRG+MlAE3ipm4R6A9mrSwLUyx6o"#;

const ARROW_LEFT_SQUARE: &'static str = r#"eJxFyzEKgDAQRNGrDNuLJIqkSKxtPISY4KYQJCyot9dVUKaY4vF9SbNgz1E4kHGEI1BDKPdZAqe8sLxwKvS+1qD32ySMGGg1Fq5q8UxZ4efRdDB2cJ9c6XUeww=="#;

const ARROW_LEFT_TO_LINE: &'static str = r#"eJxlycEKABEURuFX+bOfpjsGqcsb2NorCxtl4f0TxUZndz5uqRdkJ4IE2aiE53cuzxsqSehHY3VzMKCv0H9kAKk7Fnw="#;

const ARROW_LEFT: &'static str =
    r#"eJw9ySEKACAMAMCvDLvIBjIGc9niIwSDRTD4f9Sg4dLprKtDS24gAYpnz3A403DH9H1BAaQc/2zIPw+Z"#;

const ARROW_RIGHT_CIRCLE: &'static str = r#"eJxFyUEKgDAMBMCvLLmLJoj0kPYHPkKiUEFBigf9vbaCZQ/L7qitybYFdntiIaS3OoJdZQZtPw96TGfE7Gl0YIkuU74q7CzgAX1T8vsDNw4atQ=="#;

const ARROW_RIGHT_FROM_LINE: &'static str = r#"eJxlyaEKwCAYReFXudjHuG5uC//MFh9CMFgEg/j8osEip51PSqgJ8Vf+gmm8lZVzLCsLNEHt3l0yDfjhOWbLO9QHFsQ="#;

const ARROW_RIGHT_LEFT: &'static str = r#"eJxtyzsKwCAQRdGtPOyHZMygKSbWabKIQAobIYX7xw9oZXGrw9X/zRHfZRI7HBAI1UzQrUHQwY/d4e8FpBOWqV+0+gTsI7spBbwbHoY="#;

const ARROW_RIGHT_SQUARE: &'static str = r#"eJxFy0EKgDAMRNGrhOxFUkW6aHoDDyG2mC4EKQHr7SUKyuz+Y0LNq8JZkgojeYSLcUCQXDbRt9TG6BCaQQy9HWI4FhVIjLMHcuINLP2wkwOaYOyefX4DBY4e/Q=="#;

const ARROW_RIGHT_TO_LINE: &'static str = r#"eJxlyTsKwCAURNGtDPYhjPkWL9ZpXIRgYSNYiOsXBW3kdvdIcjnAf8ryAfV/KCN7e0aGRBJ8cW+91a0mrsJzSgXNKRbx"#;

const ARROW_RIGHT: &'static str =
    r#"eJw9ybEJACAMBdFVPvYiESVNzAYOIVikESzcH7XQ4pp7Mtsy9OJqBkWj5FTCfSpPBkVkMNifPm/J3A9l"#;

const ARROW_UP_01: &'static str = r#"eJxtjk0KgzAQha/yePvQJo2xi4w36CFKlY4LQST4c3uThehCBmYx37yfOH6TohUOL7zhjUceNvFR7k086KeGn204wdT9ErTr/5qEgViFtiKmTeiIpW+TCj2xlZ1V5f9iZ2u452yCGncTZqtM9ayxA+FjLgo="#;

const ARROW_UP_10: &'static str = r#"eJxtjLEKg0AQRH9lmF6SNaumuL0/SJs+RMlaBIIcUf/eUxAsZLp58yb8XsnRGr833KGFIocxXNY+hp0+Guhf6hMgDeT6VC/KM1hl6Ie/oXsnDLOxJMa+TW5Uwrv+48lYE5NRKiIvZNNWIS7U+C3q"#;

const ARROW_UP_AZ: &'static str = r#"eJx1jDEOgCAUQ6/SuIN85CMDcgNXdhKHP2jiYDy/MBgXTNM06Usbz3IJtmU4JgQ45VA1pDi2PsWXrjPcTb4DrEEQxR1CDDLZay5WM5pNFaFmJvMzcMK7Ynj5Hh/2jSsX"#;

const ARROW_UP_CIRCLE: &'static str = r#"eJxFyjEKgDAQRNGrDOlFdwnBYpMb2NrLKigoSLDQ25tVUKb4xRvRJes6IUdHjYOepVx6PU1Sv55kH44ZY3QbBRBX3gZvD5PfO2JQ6NtPbjk0GtI="#;

const ARROW_UP_DOWN: &'static str = r#"eJxtyyEOwCAQBdGr/OBJu3QDFVtuUFvfBIEhQRDODwhQZOTLSP5LRHhUMgSymsG6p7wcQ7xMf8nBnN9G0oV7XOhtPgeuZBc0vRUeog=="#;

const ARROW_UP_FROM_DOT: &'static str = r#"eJxFyrEKgDAMBNBfObKLJKBFaPIHru4SBQUFKQ7697aIyE137+Ixngsmpb1Bh1AF5JDFuuwWP+1ZwO0gv/iafJvhlxILwW8lYULKtZxetgfb+Roy"#;

const ARROW_UP_FROM_LINE: &'static str =
    r#"eJxtybsJACEQBcBWHubHsf5QWO3AIgQDE8FArF8MBAOZcLjnUVGCaOTgP7vBisj/jsinE0moSfoxBpLqNQu2hxbl"#;

const ARROW_UP_LEFT_FROM_CIRCLE: &'static str = r#"eJxlijEKwCAUQ68SvEBrhtLhV+gBXLsXOrgUHLw/RkEXSciQ9yy/JeG7XCTOh+lwwbb2BRvkJwi/qyuLXvDuEBpFKqdXAcBbGGM="#;

const ARROW_UP_LEFT_SQUARE: &'static str = r#"eJxNyzEKgDAQRNGrDHsBiULYIskNbO3FBDedhAXj7XWx0GaK/5jQyqaQUnfRSI4JrUcaCWfNKm+5Ik2EbpvCYIcUjlUFOdLMcH5hYSOLP3L+MTA+uwHOrx6s"#;

const ARROW_UP_LEFT: &'static str =
    r#"eJw9ySEOACAIRuGrMC6gJMovN7Da3QxEg/efUNhr78Pdz+kMnkqiS106G1peQ5kkUlT2AbzUD1I="#;

const ARROW_UP_NARROW_WIDE: &'static str = r#"eJx1yTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lQIqyO/+0/t4DOfmrgUrZBL8uaxz+Vmr7gnyMnaABIPJQKKlvgRv9I0+vWIlMg=="#;

const ARROW_UP_RIGHT_FROM_CIRCLE: &'static str = r#"eJxtirENwCAMBFd5sUDiL1w5SBkgQ0SioKRAzI/dIAr0p2/urP29ojzpIyF85YbjFyOYsl2RZNvDSJ2DFAWrDl1qApTBGP4="#;

const ARROW_UP_RIGHT_SQUARE: &'static str = r#"eJxFy00KgCAQhuGrDLOPsCC+hXqDDhEpjYsgZOjn9mUu3D4vr81xVXocj0xXCiqODZjuHySmTbRK/mhgb/syeHssKhQczyAITpRSrJUdZCZC19ILjhoemw=="#;

const ARROW_UP_RIGHT: &'static str =
    r#"eJxNybENACAIBdFVCAsIFc2XDRzCxILSwji/0cKQXHUPs6+gUbkZWahsFXaUex3J9GXfDsEAD2s="#;

const ARROW_UP_SQUARE: &'static str = r#"eJxFy7EKgDAMBNBfCdlFUkvp0PYPXN3FFtNBkBJQ/16DoNxy8O5CK4vAGXFA4FJXlojkEY6ahd/aHjUIl25S6PWQwj4LQ464kQMyndWAVVf5fSQD5Cb/yQ0JXB8a"#;

const ARROW_UP_TO_LINE: &'static str = r#"eJxli7sJACEQBVt5mB/H+g9WO7AIwcBEMBDrFwNBkMlmGO55VJQgkoGqpEXkf6vIJzTyIPXZDezbE0m4eZ0LxYoW5A=="#;

const ARROW_UP_WIDE_NARROW: &'static str = r#"eJx1zTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lYLQQKabV3y9j8dwbu5asEImwT+XdS5/1qp7gryMHSDBYPQDipb6Ery1zge34CUy"#;

const ARROW_UP_ZA: &'static str = r#"eJxtjLEOgCAMRH+lYQehtMiA/IGrO4lDB00cDN9vHYwLuTS93MtdudotsC/mjJCBLIHK1DK9eS0fXWegHtIABAYSPixDEh5z9N1Gxw2dej2vCqB/Qz9ooNIs9h97ACo5K2M="#;

const ARROW_UP: &'static str =
    r#"eJw9ySEKACAMAMCvDLvIhDGEuR9Y7YLBIhj8P86gXDxZbQ/o2U0CjMCewTiVcEPldbHEVOnPAbejD1g="#;

const ARROWS_UP_FROM_LINE: &'static str = r#"eJx9yzEKwCAUg+GrBPdSUkWXV2/g2r3QwUXo4P1RB51UsuXjl//NEd+tkoGFPjTqlJez/V66Bge6ZwKJ+46rMBhcjLRDChiqJYo="#;

const ASTERISK: &'static str = r#"eJxljMEJgDAQBFtZUsCZjXp6cKYDixB8+BF8iPWrCOaR78wwfiznhnUKMxP0YgrZm5dlL2YQmsKgMsYO7Otm/5SBUVp7Tn9yA/iVGVE="#;

const AT_SIGN: &'static str = r#"eJwli0EKgCAQRa/ymb00kyYu1Bt0CLGgoEVISN2+NP7i8Xk8n/eSjxUlkCHkO5CMH5/O6IdfR3+ma8MSaBYLV6ekocF9FlyVJGFIuwJWBq61rYkv9+oYhQ=="#;

const ATOM: &'static str = r#"eJxNj0sOwjAMRK9iZW/TuGlDpbQ34ALsUEACiQWqWMDtybhK6GbysV7mJeXHmp83yt/ZeXWUP9u6lsUt6bCNl/S6vO90nd1JO1FCZJUucIleOuUo/chBBvZeJmwCQnmSY+QRxzoAEggcF5AAUrkmzLEJCCWABLANzvCBx87GDxIJka3NcGszvPlYm/maJ4Trq6ZpvaZpvdx8AJIJA4Rv/cbf5gfnK0Wl"#;

const AWARD: &'static str = r#"eJwly0EKgCAQBdCrfGavMWNmgnqDDhFTUOAipEXdPqT14yU9m9Yd+mRiIbRME0HfTDOVNPxa0rXeB7ZMC3s7hgAWO0dwgEg13jjj4cDWizPRMvfaS/kAxUYYCg=="#;

const AXE: &'static str = r#"eJw9yzsKgDAURNGtDPZPnfwDMSvQRQQsUihYWLl6o6DFvd1JRzkr1qnbaUAlobdoFdVT4d0IgqJFzyRil9PwkJw+uNCCGhF+M2Lg4KouAaHBRsXDXz+6AZMwGeM="#;

const AXIS_3_D: &'static str =
    r#"eJw9ybkJACAMAMBVgr34EJImZgOHECzSCBbi/GKh7Z3Mtgx6cRUBdyJL5FTCVZV3AyFHYM+/DrXGD2Q="#;

const BABY: &'static str = r#"eJx9T0EKw0AI/MqQu1bdmnZhG+gD+oiwPeRYSP9P3aWkOZQgKOrMOJbX/F7wvA2PDLWFRYepnNpwKttK/WAn0LGyc4KyscPAvio7scGI/R8nY+Q0Z2RIhAbzisQ5uPadSPQjNgTpJVrZAygA952EIVXrvHDL2mv4Wanphh1ruVJcEur2zr38fvoA9VM6ug=="#;

const BACKPACK: &'static str = r#"eJxtjL0KgCAUhV/l4C7p5SoG5tzS6tAmNDg2hM+fEZSI3OWen+/4M10ZxyI2BqmoVWIwVD0Nlpxdq8GlFgj0GrJ+q221pF0EPz2bwX/LM2zkv4Zay9RqUKEB50C6SNOR3JFmSGqV3ThwTXAD6no85Q=="#;

const BADGE_ALERT: &'static str = r#"eJxtjs8KwyAMh18l5G6mzqoH2zfYC+xW2GCDMXbYoX37JjX9B0XIJ798iZZf/3/Bo8XblXIDmaLvAwSwfBwEStlwSbBlkZJcj5KWNbNVWwNdcpAMK81+kzl7zooW79iVi3y1K5/39wmjazEjDAznmb5yrGRXLHVrk6zTfsR5XKjziz8BHds4gw=="#;

const BADGE_CENT: &'static str = r#"eJxtjz0KgDAMha8S3BsTTX+G6g28gFvBwdFBPL+tlmqhBEJ4+Xh58Uc4d9imbhnRaXBohiAgQLEYBK1TsVn4NIM2jTWUW9HoxYqQTSpIRUT/nVTrHCXMrN3s+xR19iUwD2AvptZGowBTfoQfJyncDdeeMrc="#;

const BADGE_CHECK: &'static str = r#"eJxtjksKgDAMRK8S3De2Nf0ItTfwAu4KLtwILrw/NlirBQkMYfKYTDjSucE6dfOA3oBHqxMBgcyjgNB5kcXB61l0vLZQkerJG6tGCWkgkRHzTRJ/7yRjduli6LlqDE/hfQSlgYcE1fMFaJYpxQ=="#;

const BADGE_DOLLAR_SIGN: &'static str = r#"eJxtjrEOgzAMRH/lxJ40SY3jIWXu0rUDWySGjAz8v0hEZEBCliz7/Hx2WvNWsHyG39vKCLEcMoHganiQjWJqijg1trGVd6gn1dyBqdBNbpCpyHh1Mk/nXMN4Hqb0aq9OSR/2DCmGc0DoN6tXIe2bQl952gzw8med7DbPNnI="#;

const BADGE_EURO: &'static str = r#"eJxtj8EKgCAMhl9leHepOTUw36AX6CZ08Nih9yctsQQZjPHv498/f8YrwbGybUZH4NCoqEGDyCVBo3U8NwufZtCWsYdqa5p4sSZUkw7iGaG/Ex+dEwUzOwt+KlGDb4EtSJVosJAEC+r6hnx8CFUDb9hNMrY="#;

const BADGE_HELP: &'static str = r#"eJxtjkkKAjEQRa/yyT5lhs4E6dzAC7gLKiiIuHBh394UHdI2SFHT59WQX/V9w2UWR0vRIZI3dcIE1UxjohBlCwGb5ilwuYd6GJpasSH0JTtINsT9bpL/zinG/EmUfOBXSx4PJ1IJqVrYjjqKFvqsYGQT2bepx/15xaJnoYPAYtb84d60zL0hpZlnsnwBEeo53w=="#;

const BADGE_INDIAN_RUPEE: &'static str = r#"eJxtjz0KgDAMha8S3FMb7U+G2ht4AbeCQxfBwftji6W1UAIhvHy8vLg7PBHObdpXwRpYmCUoUCBTEShhGVOz0DQjbB57qLSqyQ+rQjHpIEyI/jvh6JzMmDkm7+Yc1bsamIEjD3VaRouLViCLGilS/TEVNvYF3tE6rA=="#;

const BADGE_INFO: &'static str = r#"eJxtzsEKhCAQBuBXGbw7q67pHKw32BfYW7ALBREdOtTb5+BgBSHM6M/HOGnp1wF+rfq8kRogDK734MHkY8FjJJ1LhDMLGPl6R1JqZgqrgQy5IZ1Jc52kn74zzMJXdenFq3ZpGuc/bK5V1inYpW9W3twDY2aCOaRi6aRlBBpb9QHlmThU"#;

const BADGE_JAPANESE_YEN: &'static str = r#"eJxtjk0KhTAMhK8yuG9f09S0Qp838AJvJ7yFG8GFeH5b/McSCGHmYzJx6ucB/2/VsQ41ghbbOziYNASnfVBpeVyaaJ/PJ7SvUzMbdgp7yANSCanvSar0zmRMflUbP7lqG4/CY4MABi++4JEFEVjx2+sakB2kbMjNWAHz+T9x"#;

const BADGE_MINUS: &'static str = r#"eJxtjk0KhDAMha/yyL6dtlPbLmpvMBeYnaCgIOLChd7eVMUfkECS9/h4SRyrqUVd0u8rQ4EgnaksLBSXhpU+CG4el+ekz+sTOtrpqR07jSPkAQlGinuSeDunMub+lOInv5pi3w0NZl1SICw8tCHMhqdjbTbNbKbSCpeKLMU="#;

const BADGE_PERCENT: &'static str = r#"eJxtjksKgDAMRK8yuG9stU1bqN7AC7gTXLgRXHh/bPFbLIEQZl6SCdu0L5i7amjJGTjiZtLQkLEUNFknYrN4NSabxhy62qPJE3uE60gGiYiY7yVReicTxmPVhzpF7cMdeFUGXjD4bw0efiGpCk5cUibzDhktODI="#;

const BADGE_PLUS: &'static str = r#"eJxtzkEOwiAQBdCr/MweZJACC+gNvIC7JppoYowLF+X2DoptSRqSGfh5DKTX9L7hkul01HFA1N5ODg5GFsPpEJWUgDXzOtRtj1pZMvNjS9CGdEgJGbaT1N5zpjJ/pjEd6lfH9Lg/ryicKRKKzcSeMMuRrXT77WKranbe2L+RO4U7+wFEqzfJ"#;

const BADGE_POUND_STERLING: &'static str = r#"eJxtjjEOgCAMRa/SuFMBoWCC3sDVwY3EgdHB+8cSCUpCCG3z+/rbcMU7wbkM24TegkfS0YAByU+BQecFBwefRuhy2UIlVE2+WBWKSQMJRuzfSfTWyYzRMaxhzKeuoR7sQelkOg3Fc7TPaKNGC/m/Vpz7NpRcbTxHfDr+"#;

const BADGE_RUSSIAN_RUBLE: &'static str = r#"eJxtjksKwCAMRK8S3Mf6/4D1Br1Ad0IXLrsoPX+Viq0ggSFMHpMJZ7oyHCvZJHUaHDUiKVDAynBQ1DosYuHzDLV1HaEm3WMv1o0WMkBYEP1Pwtk7VjGzkxiWWjWGXtgDN1nPDyLrJEC0LgxVRnn7zj59MTQT"#;

const BADGE_SWISS_FRANC: &'static str = r#"eJx1jj0KwCAMha8SumuN/wXrDbp26CZ0cOzQ+1OlYhUsgRBevrw8d4U7wrlOm6BWgaWaBwkSWCoESY0lqRn4NE1NHnuotKqxF6tCMekgkhDVOpHRO5YxfUzezTmqdzUwIqDZbZQ/Ox7FYLMA6ubkAbZuOB4="#;

const BADGE_X: &'static str = r#"eJxtzksKwyAQBuCr/Mxeq9ZXweQGvUB3gRZaKKWLLMztM0MkDwjCjPx8jlP+w/jGs6P7VeeArKMbPDwMHwuvU1ZcErYs6iTXI2plzczC1qANOSDFJOwnqbPvjLD4oL5cZNW+fD+/F6rr6EaYuNlAqHbpk5WYqaBG94afVHdGZ9xbN3s="#;

const BADGE: &'static str = r#"eJxtjjEKgDAMRa8SujdWbZsMtTfwAm4FB0cH748JlmpBAp/wePwkneU6YF/MOiMHYIxT8eDByYzgkdhKELwsIunaSzUac4/WQC3pJCtK+DbZv3NOtbiZnAZ9Nd/PhiHB"#;

const BAGGAGE_CLAIM: &'static str = r#"eJxVzMsOwiAQBdBfuZl9IzM2bU2gazduu28oERIXhhCtfy/4wBoWzOPc0dc5eSyGTiLg4djNAoHKjxtpZOprr0pPo96VxKhrjntwO7X/zje8HUBurH7Z6GxCXA0x4R6W5HO1J3gXzj4ZGggPQx1hLXVOFT9qG6K9OERDQrBZiMp/ufJC73VlX/DhmR026gme70Au"#;

const BAN: &'static str = r#"eJwlikEKwCAMBL8S8gBbxUsh5i8lLbSgIOJBf6/Rw+4cZkj+IvEFaQGtQ5C+WSZOZDq2Z8p3/eAJmLy5QGe9ceu0UssDKWUUSg=="#;

const BANANA: &'static str = r#"eJw9jrEOwjAMRH/l1D3GduI2lUKXzvwAWxUGRgb+X1wAVZEdyz6/c3sd7yce1+lWYLlnieSoDFP4ERIYoXyGipi2dhkbWzv3Qixgi9S1U+rJmFBljjSzhyWZ78ZmgA78QXaG08qd2J9BEK6Y6V6Ec1JUyspiJ95Apf+Tdh0WMLGSvtjMA5JTdj+v+wBeFCzP"#;

const BANKNOTE: &'static str = r#"eJwti8sJwCAQRFtZtoBEPUgOagcWEVbJCjkEEaLdx09OM8y8Z3KkAtWiQnhTKNybQOCYLi4WZZ+bRY2QJ+PMPgRnKGW6I+QpUlsk1ZkdWrczz1kYgkWvQSrehPTy+NvAxu0+g+UkMg=="#;

const BAR_CHART_2: &'static str = r#"eJxdzDsOACEIBNCrEC6wOjFmC9bbbGFirPX2fiAWVgR4M1Jy/an5j/3L1DGnY2qwfd7hOMmzWJKNFwpmcIx24Lb23MVBUdRsPHQAlVwhAg=="#;

const BAR_CHART_3: &'static str = r#"eJxtyyEKACAMQNGrDLvIGOKE6Q2sdsFgNIjnlxXT6n982eMsmMU1ArrIC9lVCVqrfEMGTD1bQirREF2upy8PSfIeAA=="#;

const BAR_CHART_4: &'static str =
    r#"eJxtySEKACAMAMCvDLvIGOKE6Q+sdsFgNIjvlxXT6p3scRbM4hoBXeSF7KoE1Sr/kABTz9awTjRG43r68wBJcB4A"#;

const BAR_CHART_BIG: &'static str = r#"eJxNzM0NgCAMhuFVmi6gFYkcgA0cwgix3Awh/mwvEDFeevjyPtX7khicwVmAOEgxKbS6K6vV0a8J2IeNk8EJ4ar3Nkg9whlcYoMjQsw7FVX6V+VIVkCyBT/RftLwsQeD+ibN"#;

const BAR_CHART_HORIZONTAL_BIG: &'static str = r#"eJxNjEsKgCAURbfyuBsIs6iBuoMWESk9ZyGPPrtP0aDhvZxzzLEKk7dYNOlTzaxmONOV15kUNqHbYgI9FiMo5aFAHOLOYjGAruiF89kXq/DN+iFNamRtKY0a/qwXa38moQ=="#;

const BAR_CHART_HORIZONTAL: &'static str =
    r#"eJx1ybEJACEQBMBWDht49g/U4LQDixAMNjQQ6xcTI01nrNdBackVFZ2IRHTZvq3ZzgWB5yNA/Nfx1OML/ZsdfQ=="#;

const BAR_CHART: &'static str = r#"eJxdzMsJACEMBNBWJA2shiV4iHbjQRDP2r0Gf+hpGPImnGIOqhgHBkEVHFl7R91TugbPnzDPB9uJ7Y3/x66bbGj+ozGlTRuS8yEE"#;

const BASELINE: &'static str = r#"eJxlijEKACEMBL+y2B93CUewiP7ARwgWaQQL/49aaGM3zIy23A0luPSDPyNxUd/lou5SBSSQhxiT+B6Sn9r8CQPOWxbq"#;

const BATH: &'static str = r#"eJxtzcEKwjAMBuBXCb23LmnTbtDu4tmr94LCBBEPIvPtTaxjO4xS/pJ+SfKzvia4FHMaIEJ0DN5xRUm9nR6L1vExuNh78BAE6CtAcHzGVAmoOUmakLYFS2/LZswH3TLm++1xhZmK6Q18JJIEFsMGZgnsVKpZpBSpSaRGNXUA0Y5NzRI285+Oww7F9fNnce1d8BddZkKE"#;

const BATTERY_CHARGING: &'static str = r#"eJxljL0KwzAMhF/l0G7qU3682J679CECLaiQlg6lpG9fuUNCCBpOJ75P+TW9DdciFw5IxkmhiD701M+49eCbBZWaT82peTVHpHO/gjHsxdgeGY/eg0QKHQbr5xYbMd+fNyxaRFXwZRHS0zs7wcL/3eGG1R+HiS79"#;

const BATTERY_FULL: &'static str = r#"eJx1jUEOgCAMBL9C+gEFjV6AzygREuOBkCi/twvqRb10091pV0c3JbGHOXlDciARsyFFwruw+MRWy9ZRrDo5HsnqBndWr2FzIivmOgZYFUBZNbNKCRjYBSMcnqzcYK0v3ij6wUDvnu+/CPtaDv2BT7/qPY4="#;

const BATTERY_LOW: &'static str = r#"eJxtyzEOgCAQRNGrbOYCuphgA15GiZAYC0Iie3sXKLX6U7xxOeyFqocBPeko0YMtKIZ0xqJ7BonHCsrSTe50c1P7be5KdyAxChdQ1Ro1lUdFy9xwYz/YDms/9AXtHSc4"#;

const BATTERY_MEDIUM: &'static str = r#"eJxdjUsOgCAMRK/S9AJaTHCDXEaJkBgXhES4veXjdzWZzpup8mYOkCYcETyLQDjcEuyEJBFiOfgq1rjVBg561KrLPa02txuIgnMGEisN3KLmWYkynLEXLCsjb+Sq/lAqz55d0fx39wTIWjJf"#;

const BATTERY_WARNING: &'static str = r#"eJxljMEKwjAMhl8l5F5t4lgu7c5efIihQgURDyLz7f2zlbIxevjzN9+X9B4/hW6ZL9KRFR2VlCKeIPXbXzEFzAGtBOUhHd0YUvN6snPXtBiatpyAtrKej9edJs2syvSTzCJIdDkxTTL/A3aswg7ZspPIs+tZnS27g7xbhe0QpfF/6C08EA=="#;

const BATTERY: &'static str = r#"eJwli0EKgDAMBL8S8gFNBb20/YwWGxAPJaD9vZt62WWW2djKblSLntUSy8zUeuLAhNwA74A/Hz2sQlo5x8l/OV56F+qCUWAFaH5FywKWwZBdyx82nxxn"#;

const BEAKER: &'static str = r#"eJxtizsOgCAQRK8yofezu0AsVm5ga09isaWF8fwKBaEg08zMy9M7P4Zrd4efA8QouKRLOZM2FCEvxcxgrDV/s63fE58yFMkbcSMfJSwcOg=="#;

const BEAN_OFF: &'static str = r#"eJxtjk0KwjAQha/yyL5jZjIZE6gFD+AhShUURFy4aG/v9Ie6kYQXJnl8X9p3/7njegqXijo0ZOq7YcrCVLM2QhoFTGz5bDDEZRWI9JyoGtZcryvVRolD1x5mbNfucI50zMgUa9ox7BCUIcIdzK5muLy43EqCkdkfUKakDMeZ9ArdxEZajv5LieWyvGHp9UqRM9bclBRTnVN/8OfjdcMopyASMLKfAdM2Tsvo1bnUfQGxSkJh"#;

const BEAN: &'static str = r#"eJw9TrsKAzEM+xVze9yzYzs2pAelc3+gW7gOHTv0/6nTIfiBBJJQ/4zvG17X7UE7kikYavg9MFSgoTSHQBNwrAYBcZZkuYVQmaaqMMrOQNN9MzDY/+PAfDZslZOQFEM2n4BkLFUhQq+aYRnx3I5+mXWOvkopViHIasZDQGZUniK3KPl9WX6EayrH"#;

const BED_DOUBLE: &'static str = r#"eJxtyzEKwCAUA9CrBHfp/0HEwXqDrt2FDo4dyj9/7SJFJEMg4eW7Pg3X7g6CYj7V3pAeBT2bxv8AWnIlbx8qedAAlTPOkpMMC6lEsLg4CE2NMp4XTUop6Q=="#;

const BED_SINGLE: &'static str = r#"eJxtzDEKgDAMheGrPLoXk7RKhtgbuLoXHDI6SM+vLipF3vbg+22vh2Obw5Ig1KJWgYCuMSSKc/4ekKah2HCjYg8dwbROvaRO5h+ZwOr8Nk+HfSK9"#;

const BED: &'static str = r#"eJxtijEKgDAQBL9y5ANmF4knnPmBjwhYXGkh935jIynCFgszY3d7XK4jnZQ1UFK15UPVBqEObRRK7kN/BvK0xOacmSIa+89fUJMgDQ=="#;

const BEEF: &'static str = r#"eJxVkM2KAjEQhF+lyD29050fe2BGkL3uvsDehqygoCDiQd/e7gkiElJUmkrqI1M7Xttpj3afAwuVgPaYg7q5zsEH2+mrZ7bTZbkd8D+HX09Clmrqe/AVK4kgU22RiZGIU6SNmhmjHdSSg+4SUo+jgLVlM0rZbiiYzWRKu49n0bv+nMP73xRnVg9CiEfrLYaTvcVkvQdKDPHB2GWdRqGKQvLNhTJkILFiGPnGZHnhrTE1rFp/xHJc+0+sBE9+MkHY"#;

const BEER: &'static str = r#"eJx1TjEKwzAM/IrILjWSo9iBND/oVOjQzaSDx0JK3l8pLqHQFCH5zJ1ONz7zq8Dj3Fw4AnPhHCBAa8XWfUFupvHkomncpQOwrP0BweEv00EkndFckanrSDEA6YKCDn0wRbHHmjTbBO9PEtSZYnJMGitFeh3MCMRigyzilEX36U5YVfjrBRZjM7OLzmOV3Q9SK6SVJYsdabcyVNL3H+WW9sU3RR5Lvg=="#;

const BELL_DOT: &'static str = r#"eJxFjcEKAjEMRH9l6L0xaZbWQtuLFy9+xFIFBQ+yetC/N9s9LCETwkxeymv+3HGt7iKZJshE+RSYAiTaGgSSNj3rW32A+tzZKykCJR8RrSmBIaRkOcqkrpXDim1lh/N6IbP5E4bwKLUnvOf7Y+nPG/q3Ojk69F91NpbqBnNz2x/83yn0"#;

const BELL_MINUS: &'static str = r#"eJxtjbEKgDAMRH8luDcmTa0VqrOLHyE6OAr6/3hV0EVCLpC8u+R9Pjda+2rSxIHUL5zI0J4jNY+OdpjzZK5bxBkbtq2LFNEKUsAHzMDgqyHXJXLIX7AUi87KHR4UkbvgIvnjG0pbfA8XfEgljg=="#;

const BELL_OFF: &'static str = r#"eJxtjcEKhDAMRH9l8N5s07hNha6wNy9+RMGDF0HQ/8cawYsyYQgkbyavZZ8x/ZoxkUL+ERG+isEJqQQmgZk3UcS36fPnpPp8s6xgHWQTFyCuKy1FhdmVReKY9A30ll+YuhZmV49Q3Z7/S0CVr3PfDrAyLRY="#;

const BELL_PLUS: &'static str = r#"eJxtjTsOg0AMRK8yol/Hn2U/0oYmTZocAiUFZSQQ58dAQYMsjyXP87j9x2XC79l9pJJBIpWXMgkkUYT6zKe+bbagsFC/HMxRpRwSkreAIVRJfVcod0N77KlDu7J5P5DRqYhD+CjzH3zH9yhTujMK+vUyNuQDLUU="#;

const BELL_RING: &'static str = r#"eJxtizsOwzAMQ69CZJeqj+vYgJulS5cewkiHjgWa+yOKh0wBIUoQH9uvb198HtM7o/SMDAkp1CCrYIajHvPyv5PBqU5Lux2lpZ1VFXaYduWaMEyGnOO64BPsaVwin2G4Dy8XnMV7FTJ24kIpllE+uR0G8yw/"#;

const BELL: &'static str = r#"eJw9izEOgCAQBL+yoQfvOIOSnNQ2PuKihaWJ/j8CBZnsZosdfey7cW3uSFgtIYEqDI6gk7BAkFt2ecVHiM+u6NSkokNlCoLIxiHP6EUdCXWN/w84fxis"#;

const BIKE: &'static str = r#"eJx1zDEOgCAMBdCrNOxqoBAdgBu4uptqggmDIQ56e6mwGOPQ9A/vf0tborgCXU7IvjUC6Mxp4JScwPy97Qry9oMreUrm1xYo6/ZL7fMRYHFilAp4cpI6Ntgg6HwKMCjWrPwNwvAuIw=="#;

const BINARY: &'static str = r#"eJxti8sNgCAQRFvZbAMKIXsCOrAII8TlZsjGT/dCuKDhNJN582yOmwDHtLM4JITHoTIIVwrCDku725BLavR2qoK3A619qXd/0rEKQ3C4EOiZTQV16oAyoMaEQBnWJ42lL3oBXxk8rw=="#;

const BIOHAZARD: &'static str = r#"eJx1kEtqAzEQRK/SaK/KtL4jmPEm6zlEUAIJJBBMCMntXT022GB5IanpT70uLf3j2D/fpP+tToOT/s9X0ZwcVxfcYXk6NxyW75efd3ld3VZQJSJ1jyYBWSbJCIyCsPJcmGmsV2koEoRixWRs/CryNXNYJ6goEub7hk0r4o65pfgLhZnklbHliCmmhjzgaDYGQf4haNebMHeqF0/hxMfChOozot/dWUqFEpY27kiKdiPa79DyxlLmID9OvVmxK8o09C7citarJ4s+uXwxg9xhYHLLFDZwBy/M7LST5bb7BAxadIo="#;

const BIRD: &'static str = r#"eJxtjrEOwjAMRH/l1D3GdtLYQ+jMAGv3SAxdkBgQ308KqB0aWbJkn33vyrO+FtzPw00ybCGWYSqndTmVTYqUIH4RrQ4Hf8uDz1YT0m8ORupBKV4VykePhzIMChrD2joQ4cZ4x57S6EY2ztoLZ+2tZuR/rkiegjDl/fYDj1o5KQ=="#;

const BITCOIN: &'static str = r#"eJxdj00KwjAQRq8Sus9nZjKZH6g9QXsI0YULCy68P6YRRNzNB4/3mPl5ed3T7TxtRDC1RIHicRUEC1w9KUiyonBLBCbtt4fseYw0xto6mchRxPaGKN7RIpxRpRsRthOa1uydk687/t1tuOvH3Ya75oqQDO3JBmoHwquDIwlYf2s95vmIbQbxxAVV5VFBrJkMxjYt8+n4d3kDLn40Kw=="#;

const BLINDS: &'static str = r#"eJx1yksKgCAURuGtXO4G6lcjBXXcpEWEBQYNQiJq972gJjY937Fzt0TqHbeSZIRmb4srefuCKKlufgDICk4xEWWGNKGKEBlRJFeoD8KYwjRQ2B3DMCXHgilsju/nUX8AbjM2UQ=="#;

const BLOCKS: &'static str = r#"eJxdjb0KhEAQg18lTL+cUY+7Ynfra661FxXHTmTx5+11Kn8YmJCPkPipaxKmNQgF9kvBMrRJg3wEW5BCoN3QazIQ/cvy0Y91UrRB/syQs/rWBJHZOTr+yqsHZ+YnOJR6B46zez8qChuzmbgDNeUmmw=="#;

const BLUETOOTH_CONNECTED: &'static str = r#"eJxlyjEKgDAMBdCrfLKLJlLr0PYEzu6CgkIVB4d6e1MVFyEk/Pzn9uGYMXpaLSy40ikMTC9Rd6cfS8GVGQUXl21CEk/ChJM9sRBSvq1mubPizF78IS2bx9Y/egF09SC/"#;

const BLUETOOTH_OFF: &'static str = r#"eJxlTDEKgDAQ+0roXvWKxyGc/YGru+DgoOAgvr/p0qUkIZCE+Ht8F841PGIQiwrdJd3VQ/axttnbJoGYyL7bZB4UC8Uj40HUn0kbFrcHG1k="#;

const BLUETOOTH_SEARCHING: &'static str = r#"eJxtijsOgCAQRK8yoQd3kV+BnEBbexILCk0svH9cY7QiM3nFzMtnvRq2SR0REUxS7eFXuwtnWaIqeXikkj91sWTSCHbC6uBAb7Q3IXR0TmDbDPH/3cd/HX8="#;

const BLUETOOTH: &'static str = r#"eJyzKUgsyVBIsVXKNVcwVzA0ACJdUwXTMKMcIOkDFDFXsrPRBymyAwDtywrA"#;

const BOLD: &'static str = r#"eJxlyiEKACEQBdCrfOzLqswuhtFs8QI2wWA0iOcXETTIq49ragXZiqAISicCQS6P8X83wvE7j+MzP2h5z7jrAIv2FT0="#;

const BOMB: &'static str = r#"eJxNjUsKgDAMRK8yZG819YNC612kCgoKIi709nYUioQXCI/MuLAcYZ0QLi+qgsNLJwh3vErpXf7p3u3DOWP0smlnahA1bRYZrKlAim+y0lRrFA2hVLwrSRTULf9ZwOBfvLWwUTO+TvoBN1wnrw=="#;

const BONE: &'static str = r#"eJxdj70KhTAMhV/l4J7ctJDbCuob+AJuUgcHBwffH9NUHKQ05Pc7yXCu145t7OaQEKRwIjaH/z0EkRWyVusegn0hBauHliBWespvizVIEXAOTsoNdFBCKgY3hYanL749LRzrjInEWJU+CzidjE5Oyg20dNPwq7dMNyD0KbE="#;

const BOOK_COPY: &'static str = r#"eJxtzLEKgCAYBOBX+XGXusOkwZxdWt2FBseG6Pn7CzIIueU4Pi7s5aiyLWalwGdXKJRRA6FlBUwMw21iaHISuPRKPNrVHqR+zgn4U2T/rRqr7QTbwwWrsiO7"#;

const BOOK_DOWN: &'static str = r#"eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v81SEDkO7njw4l2eE47FbCNQcFItycpOQIstBJO+zFgZc9vlS9FKYybFQU0pdh8xkN/nP7kCEILXWN/xC535ISE="#;

const BOOK_KEY: &'static str = r#"eJxljcEKgzAQRH9l2HtsdpsUhUToLZd+hKSFFioUKaJ/7wZBEVnmsMzMm/Dr/m88Iz0cuKn8aNjfpfIosnqMW/kSO2rDpaTbsHXEoh7ZJY10x5I1PondK/kz5O8LeY5UE4ZIQshTpBW7uju4V7AYpzTVebfnBlcd4c1aADruMrk="#;

const BOOK_LOCK: &'static str = r#"eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCCGEfN8f/0olwxTwZoEuvdSO5Mq9gLZpReB0i2Rw9CelR7853O5Sz7Eh6SiZTiLvlHV+FPgGdAjvZSo54ICQ5+WZS0BB+AQkRlh1qqX87hEN4O42MfA/3YKpvMX/AAi3Mf0="#;

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

const BOOKMARK_CHECK: &'static str = r#"eJxNjDEKgDAQBL+ypD+8W05C4MwzLOwCFmkEC/+Ppgmy3Q4zcben49zSZQU0yeKS4fvaCEK/GSjspv8DPFKNZbg1ZqHAdDC4+MQvVPgXCw=="#;

const BOOKMARK_MINUS: &'static str = r#"eJxNizsKgDAUBK+yvD6YffhBSHIMe0EhgoqFSPT0JlayxcAy447xjJi8bOyhNJ2pTYd6aEaFwuYRajTS/g/oxfaR4KqSB7cu+4xEL2wESb30gjuDNpMfs1us8ALrmxsG"#;

const BOOKMARK_PLUS: &'static str = r#"eJxNy80KgCAQBOBXGbxL7vYjgfkY3YUCg4oOEdbTpxghexhm+cYc7vSYBrFRDyapZSM1mrF1DIaKR2DJnlT5AF/UPcKaKs2tWZd9RqBBEAsEznnHrmOkWiebVGnbbEh9SOVt/9sXiZcmBg=="#;

const BOOKMARK_X: &'static str = r#"eJxtzLEKgDAMBNBfObqnNqEhFGI/w8Gt4NBFcPD/UXGwg9x2jzs/2tmxzWHnAmEyymTIizaBIN1hCEnnNBaQNVSfnm317yFHhUUlhf5weRWjXoysHyA="#;

const BOOKMARK: &'static str = r#"eJxNy7EKgCAYReFXubj/5L1YEpiP0S40uAQN0dDTq5uc7cCXnvJWXIe7uUO0aMEiwrkWQfA9QqZKPw/o4/a7nJbBcwMwSBAG"#;

const BOOM_BOX: &'static str = r#"eJx1i8EKgzAQRH9l2Hupu6goJP5Br71LDE2ghxKCbf++GwVtobKwMDPvmceYAyZLlxr9tRkFgkqPIScJLN8FZK5pMOeiDGYTO3Qz/+lZjob2d0jeZQQfbyFbYiE845SDJakIb0s9Ib00EZavWhEG42Jydw+nCDfKLIhTpivMuu6U9txu1Ors2AdMJEg5"#;

const BOT: &'static str = r#"eJx1jDsKwzAQRK8yzAWS3ShBAUl1mrTpQ2yy7owR/tzeVmNjsLthHu+F9psNVeRbFP7jXp4pXMqZQlf/MroxUokp0hPLdsTQVNki5UFY3fwtL1uLVoQU1qJCnOnW28D1jMgdcuuPyHMPZsGWNBo="#;

const BOX_SELECT: &'static str = r#"eJx1jzEOgCAMRa/yw070FxhIkBt4CBIHRgfj+YVFMSnp0vS936bpLFfFsZk9wBWBYO1lW2dyWjrN6XUYB4nQJSEYP2uyKkA4SlaT2rlKdS5UAf0k0cAk4hBv6m/owIF+lviRB3Y+XYo="#;

const BOX: &'static str = r#"eJxljrEKgDAMRH/lcE81qdoOVfADXB3cBAcHBQfx+00FS0ECCVzyLhfO5dqwdsUoDL8IBFUsYmLj7E6O6kzVqRLqIUmw8HcGgvGCepSJkVOnDNN33M5FH8qYoA9fjsMaCwdvHJrYqfnfjKw+MrGk1QOWvyyv"#;

const BOXES: &'static str = r#"eJx9kbtuwzAMRX+FyE5WJPWwATdA93jNkC1Ahw4q0KHQ95dS7TwVQ4AE6F6J95DTz/n3Cz7fd7PQmIBtlw8BAdeWAHuKWpTEn6/XzUqJs9ox3AhCLoI7sL0bCwYKGQMqenJqmpfTbj+91ZL7aS38bV9FCuZJHoWG8NIC9tWzOP+LJRCnjlqjKIUjj1lrbCF9DpwV7znMiRWw4D05rsKBraxrfKA9Kl6wuplXFTagZ94GS5XGW09vxpUgEmvxpClbrtqwY73pAGR8YMalFdhmeuq3coBEEsEi9cbUDJtIyzCGi/YHZ/qXkw=="#;

const BRACES: &'static str = r#"eJxNjr0OgCAMhF+lcS/aGvxJkNnF1Z3o0NHB8PxSTIB0ucvlvp57witwb92xwLjPgYFh0MOkoi2e1ENxkNMrKUNm/ROhzrteed4VKk3AJFS5wMgRUxVzVbHI7Rvk07YzkAUr+QNcHSae"#;

const BRACKETS: &'static str =
    r#"eJw9yqEOABAQANBfuelm58aEIyuqbhMuCub7US6/x2tsgZlNwwgkdDCJJVPYfSisnMBjDf0VxQsGrRD3"#;

const BRAIN_CIRCUIT: &'static str = r#"eJydj0EKAjEMRa8SZt/YpE3JwDgn0O0s3BVddOlCPL9pFWS0gkopLT8/+S/TOV8KnLbDnhgiSmYUqNfX4yKOyWFMsJYJR4XwJoY6gqNVwl0DDBEERddW+40JAvpX3eIIveyMhf1iOIdhnjYVcZ6eoAl0kWMNNA5nVsc9GwOFEj9UtKRsnS2V7OUr9Z1atFNgb8yaURo6ATmq28pjExPA99nNQeH3xhbI9EeimuPrwBsYdnYr"#;

const BRAIN_COG: &'static str = r#"eJx1kFGKwzAMRK8i/K9Z25KzMSSFPcAeomQXWmihlH60t6/lfhQ3MSFga4bR80zL8bqc/mm5zy5ER9fZiaPlUW+76esl76bL/nagv9n9hkiKtI9IZL+3jxV5YOhA7TggjySroVhE1KLIa0YQpYQ0ttZyygMJ/PjTzgtDyJ8QZBBQ3chgy2jX8XpdJeNKtn4Gy5qh9GAVWTXvgs4h4ZuChzIydMOQUfAFsacXbSBL4Zqx5fAYKVtCz1AZBKnI3IfwpnYMRUyVortDEAyi1Z9/loV2"#;

const BRAIN: &'static str = r#"eJx1zksKAjEMgOGrhNknpnlIC3XAA3gBdwMuXLoQz2+qIHRaCcniX4SvPrbnHW6n5VLIQc7SbizHJEgCRv5KvnUdjcqRzKCvEhWVOIPGfBupoZNn2D0mlfgiNvSSUXeKj+y6rPXQsGv9kZMNZv5jZpiZGUYzw9TMODe3PphDYT36DbcxRg8="#;

const BRIEFCASE: &'static str = r#"eJxNi80KgCAQhF9l2buki9lFfYOu3SWl9RYi/bx9WpcYGIZv+GxJawVOeePqUGmEcjskhOvtM8fKbcnGP9LeCb0duuftHipDdDgrA6SWMRAQyB5BgljoPwA6lOlut/wD66AfPA=="#;

const BRING_TO_FRONT: &'static str = r#"eJxVjb0KgDAMhF/l6F5sQweH6hu4uouKcZNS/Hl7mw62Egh34e6LD+scEe5OkULarcK1L5GzevLmdd84iux9I/HeH1NkLJ0aHKyZCASTxmrSNLrikTz/PEggUq8g1oEKxUiqahmhnLo6yJsP8wJ6oy4l"#;

const BRUSH: &'static str = r#"eJw9jkEOAjEIRa9CZg9CoZ02GecGHqLRhQtNXLjy9P52oiF5tLQ82F79fafbeXk20UJm0qiKrgyUnqRmmlAyRIj6xGO+j5912bfTkOzbT3VZISALaXFlk1JI2cnEM5KLpmETd06ScciJE4agbNABhlu0oxRHSjKaArYKYoPoEBlN6Ah2HurPf50v+pwtgA=="#;

const BUG_OFF: &'static str = r#"eJxtkL0OwjAMhF/lxB4Tx85PpVKJjQFW9qgMXZAYUJ8fBxAdiLxEyfm7u4yP+lxwO+wuHJGJ5ZqqQODbuEisLhCfC8JuGvdNPI3bihIHCJUCTl1FCGBZnK4uVIV+sep0cUzSW/A0ZMTZw2xNkxreCUXov/oeYGNS3yFlylYoH3+2SGBepSY7fC6YrbAZcC8JN/TqSufJQHLq1RUEnn37MTBliz2842/RX2yzWlg="#;

const BUG_PLAY: &'static str = r#"eJxtkDEOwjAMRa/y1d0mjp00lUoXFhYuwFaVoUslBtTz4xSBGCLHHuyXn++Mz/m14nHutoII4VKO0k3jqU6m8Tu/ibFE6EFkxAYxoGfRnWRWDkHxqQHikRF2aam6mswGc86DjGwl+2/Adp0zu4Tn0XOjxqkhljkphotxRuECrX68NlGIXltLKKIsgaLfFO5Jeag7kzXQGHjokRY3xELir1ZSOaFBb/55fgoSed5/wBtv/12L"#;

const BUG: &'static str = r#"eJx1UbsKAjEQ/JXBftdsdi8POG1sbPyI4yyuESwk3+/mRLCIBAbCPDKbnZ/La8P9dHgURAiXssPhPB87c56//E2MJUJ3RUIcKCoyizaSRTkExQcDxE9CaDJKjYhhJWUXUqLI2TE10sVg7u1uI9vs9w5rugbP126AG5D+ZTeqAyrxpKgX44TCBdqbO04jKUSvo3EVUdbgDXw+r61c+++QDaQxcM2YvLOrSfzVrlSeMFT7InQbBklmJ/PqMb0w923tz/4EvQE+O3JX"#;

const BUILDING_2: &'static str = r#"eJx1jbEKgDAMBX8luBebRykt1M4urg5uBYeMDuL3a6YWacl0XC5JV7mFzmXaPAG7KyCQ/YYJBhJaJjwcjimnWaOcmpSx1tQaXfWVNRV0Qg4UBb8fsbIeEtMtLXlxfcF2aNzQhMa8YhVF7g=="#;

const BUILDING: &'static str = r#"eJx9jUsKhDAQRK/S9H5mkiY0GUi8gYcQFdudSPBze41uRGw3VVCPR4WxrRMsER2CtH0nKSIZhHGfaK/1qDPnvkkS0TIW4Ze9IgxVEmgiln8gmj5OeHKZ5v1CPbB8jX0glnVEb8iaF+b0N91j3fO65u/WBn5fXfM="#;

const BUS_FRONT: &'static str = r#"eJx1jlEKg0AMRK8y5H9bk263Fna9QQ9RqjR+FIostt7eVUEF9SeEvJnM+O8zKspADwsHwY0Kfx5uhZ8JZ3Bqt+AjSW/E8IKa6hXRBboQtKrfGgOxI/zqMuq0Nv9AQkhz/DgYVlEWzJpk2xI5+KqnjPf6uWOW0L2VPVMO4dYsqAe13Upd"#;

const BUS: &'static str = r#"eJx1jsEKwjAMhl8l9J64ZGuXQbuLFy8+xKhChR1kytC3t3UwByqBkPzJ9yf+OtwTnII5KrjZmd7vitL7VWf7ZyDAkrijn5ACa6pvZJGpJUUhjcRIDQmS5sQkUOUei6q4KGNODdq9VMTgKJt0pche4A7NIJCZEpirmatUf07HyxTHM8RHMK2B+AyG1cAUjJSdZbp5sCv/2S/8vb/SxYzdhn8Bx/RNzg=="#;

const CABLE_CAR: &'static str = r#"eJx1jbsOgzAMRX/lyntpbAIVUsIfdO1eFVQzVKpQ1Mff4wwIhjDZ8vG5N7zvSTFEurJDrZVj6sM5H/uwIQ8po5egg7hTU7IELLe22rF5fCT8InnCdxqSRuKWoOP01GS7I/xtCGG2pzp72dhldhb5KZY1R8SDL2o1K1kA7AJDlw=="#;

const CABLE: &'static str = r#"eJx1j7EOwyAQQ3/Fyk7KmR4IieYPujJ0Q+rA2KHi+wtLk0hEpxv8LFty+pRvxfuxPO+IhSBsPzE0zFp945GBr2VLtxHZ0j/ooNlNeLjgEiGa/arFrYrxo96aANtETlAGzHFWEkBphhOLcm0REqvx3d13oW+tJ33c+QNgjUiK"#;

const CAKE_SLICE: &'static str = r#"eJx1zEEKwkAMheGrPLJPbGYkcWCmazceQqJQoQspUvT2dnDTjWSX7/HXeCwx3xHvRoWwNEqE+DRyGuvhh2N9Xl8Tbo0uLgkuBRmqa4kBYnKE9pvUQgwDlPuLdeVtwIkzGzufZs4bJ7Ee7sFdVg2az/mP+E6+jJUs7Q=="#;

const CAKE: &'static str = r#"eJx1jMEKhDAMRH9l8J5uM1Ttoet5L/sRwh56XFD6/aYIItISBpKZvEn/dc/4vYcvPahF4koQvo5Q+JnuN1jisKRXhZZ0oQE6bW4UBavcaEQw2WZMEN6986vRYplm+kYyIxY2fGUv6BEzQnZe213d6Ekd9QpP0Q=="#;

const CALCULATOR: &'static str = r#"eJx9jFEKhCAQQK8yzAF2HZHwQ7vBHmLZZEeIiBCy2+cYQX3Yl86898Yt4ZdgyR41Aof451S+CmGNQ2KP1CFsFRbFYO/eEvRujFOATXsUTvXJ+tBzGa2Y4pxm2ZG5O/W0zBd5/iaGweOHOiDFL0XCZHtlus3sc2bamXnIbDu7ox0nJlqg"#;

const CALENDAR_CHECK_2: &'static str = r#"eJxtj7EKAjEQRH9l2D54O8aQIrnaxtb+QOEEFQuR8+9N3DNecWyxzPBmmU2P4TnilOVAhfpjGAiiq+PouN8tNfhS/zfK5hilT5t6pE/Xy/2MiVk0CN6ahYJJZ1nsUNEKzah5jSwqWiCugNoZQjVma8FiN/b3yk0D+G0H73zr9wHEkzlC"#;

const CALENDAR_CHECK: &'static str = r#"eJxljOEKgzAMhF8l5P/YUots0PRdxixrYRORgu3bm1jFHxLI5bjv4ubwybCkIUdGeiLEkL4xt7swdghzZTQIsq2Yosa7u/a8+6UxQDWMvdAiJFpp4wttVlCFDnTPBJX/R5HUXUF6NERVma4VDZ3s9M4RBsb/C6gHI2NvVmMN/AqJbjmN"#;

const CALENDAR_CLOCK: &'static str = r#"eJx1TjEOgzAM/MqJPSk+iNshZe7SlaFbpA4ZGRDvxwEJGIJs2bo7+3RxSnPG/918KXj6MGoiiLaUo+MnXDG4SH8Stpk7H5ohPorPEA83UbvtK8Lrhu8gba46WSrsQ609wyg1A9LUpFDLJRAnJeQON8Lg73hbAYTYPNw="#;

const CALENDAR_DAYS: &'static str = r#"eJx9jFEKwjAQRK+y7AG0m5ayH0lv4CHEBhMQkRAwvb2bDaJSzNdk896MTf6SIRWHBiFtGs+45uCQGCH4eA25vcUZEUSZcLHH2lvsLd49bKS1IkGzGMahRDF6ilqlvcpN4XdhL9LQzLEpekoa+riPcw6wOjwx0BQOg6L6+YXIdNj8n8kkdya5M/nLXhYRX1c="#;

const CALENDAR_HEART: &'static str = r#"eJx1ULsOwjAM/BWL3Udtx0kjFWYGWBnYqjJ0YGBAfD8OQsDQKi/nzndxPNzHx0zX3eakQtKd86ik1LXBynrw/zvpU9LUkUBQqRE6l81+2DaT/fC1khyZaYHoV3CLp2fpFxgVaCVJKKMiGb23TznIznCdGAbRCAuM0cPtFlCKGYe3cE1rP21u2lIDY1RHTSwhcIOSoaSjFDip3gyZDe5TFMaxpHUjhWEPqdxSL99vvAC6R0/J"#;

const CALENDAR_MINUS: &'static str = r#"eJxtjc0Kg0AMhF8l5C41Y7ussHrupdfehRYslNKDiL69ibv+IJLDkMk3k/BvupZeFT8gJMXTNSBQbpMhw/223wm9XDdDFa3nOlyspA7fz+9Ng1QsjmlUBdOAtKo6Qw1aUPU8zwm/Bs5AO0keySJ2QiKq9qE0/Z8hxFopE1yu8AS84DxO"#;

const CALENDAR_OFF: &'static str = r#"eJx1jMEKwjAQRH9lyD0xO01ihTTgzYtX7wWFCCIepNS/N1GxCi0Lu8vs2xdv/T3j2Km9M9Kiti1B2Fc1CIO4fgrKlv8CMS21lC+V4qq6UvwaKRBv/CFMvKbmbmP8DC0BHNzMoYHYvF7w26z9r+5yvp4wslOkwihlKjw+450WtELpCQysPdU="#;

const CALENDAR_PLUS: &'static str = r#"eJxtjrsKg0AQRX9lmF7iXBNR2LVOkza9kICBEFKIuH/vjLviA9niMHfPPNy/7Tt6eX5ASIpn2YJAub0MGe63bU0Y5LoGSnQVN+5iQxr3/fzeFMQzmAI8l0yjVmLETFVNSqp9VnzsgIUHUTNInFZEU/LYqNy7YdlorKMMpAvqMzlJxnGRZXfuBF0VR4w="#;

const CALENDAR_RANGE: &'static str = r#"eJxtzcEOwiAMBuBXaXpXV1gcB9jZiw9hHBESY8xCIry9LRycC6fS/t8f7OrvCYKPj5AckkH4xCWF9swONUJxOCKsvCgeRcZsT9Kb7TO+PGRifmZIlRTlkLes6pWpoA01PWl2UAQNjciUom5U0c++bynA4vBKE9AYDvU/OW4jDWQuUyepneNAvRJn5i/7AlPuT4A="#;

const CALENDAR_SEARCH: &'static str = r#"eJx1Tr0KwjAYfJWje2Lu0sQKsbOLq4Nb0CGL4CB9fhMLRWjL8Q333Q+X3vlT8Dx3VxHULWZBcA1GRpfwz6GJ/cOBlvaEJqgcbejGdGg1Y1rKGKu33xCGnb8HXeGwVVUjzB6+buBvR8TMZsTJ3dexl+o4GdrQbtG/o5I7uQ=="#;

const CALENDAR_X_2: &'static str = r#"eJx9zj8Lg0AMBfCvErJLzbPVG+6cu3TtLrRgoZQOpei3NzGCfxC54ZHwe3cXv82vpUfiG4SkuJcNCJTbyZDhelnOhL+c54Um2sB1PNkldXy/Pk/qJTGYeiQumToNsZQxlRrapzoFb4QdKLmTwgnEi7pe2/Glir1TOQKmEg4wpq/YvMEDWm9Hfg=="#;

const CALENDAR_X: &'static str = r#"eJxtjkEKhDAMRa8ScoExtQxdtF5mptiCuCgF9fYmNm60qw8/733iS/xVKEdAg5BinlMNSA5hDzgibPlfUyvKfjFMWpz8R7zJL3mNcFC7mIBfFjlIkq5kVCBFpeStt+GeIHeGGkJDMyRlYOywZJWx6jj9YOjBOtSRbvgEaKVH6w=="#;

const CALENDAR: &'static str = r#"eJxtjFEKgCAQRK+y7AVqNcIP9TIlKUQfImS3b10JIvqa2Zk3a3NYCuTqUCHkS4QPjcB+QjjTWqJDMggxpC0W8d4Obeftno4AlTiceaEcNqH+RUnKaINeqPkjzQdsFY29U4Qy1H3I8cPe6YsxhQ=="#;

const CAMERA_OFF: &'static str = r#"eJxtjEEKwjAQRa/yyd7YP0lMA2nBnQs9REEhgogLKfX2ZlRqF2Vg3szw3+Tb9X7BJJ0RMZhYafD64Xvt81ZDfX4Mz4JzZ04R8eAHgaDR2tRpTP+9Ugp3KqqyEJMN8CUcWT8UNxtUY4w2rCj0lkIwKPYO7pMnkm1bsKl9lt7aujC7"#;

const CAMERA: &'static str = r#"eJxNTbsKgDAM/JWQPWrTVhHazg66uksVKjiIiKhfb4uDkuFyL86swx5gtNgJlWlQgXRbQdWogYGhSEfxO+qPR+Qgyr9A3Nf/AnEguRBnmuSNzuRpxRk/b36ZwF8WhUTwZ0RG2CzKFHpt9wCkgSMO"#;

const CANDLESTICK_CHART: &'static str = r#"eJxtjcENgzAMRVex/gKVG1KCFGeDDlEVVHOrUJSW7Yk5ABJcfPCz34vfV1bqBc+OfGmQ4s02KU7DO9Ms6EA6jB/NggfoN/ZZBQ1o+gsYVGdrT3ae4kHGvtx32wa4JXcEa8ZUHlbzey2ca+dO1bEr7iLkaoeDctjYAixzQ0w="#;

const CANDY_CANE: &'static str = r#"eJxtjrEOwjAMRH/l1D0mdp04kUL/gB9gi8TAABID/y9cqrZDK0++e75z+/TvE4/rcEtkEO4CQfThMFIK8iqUA2vPyIsMjqS+LBz/Oc0aZFPiqtyHqV3m/KltLWxkCQZOEOIzIFKFUgGPqEf/bW7X+VWH9ORe3Wd1wDsK7Qk/1Os2rA=="#;

const CANDY_OFF: &'static str = r#"eJxtkM1qwzAQhF9l0V2qd/WzMjiG3HJor70bWmjBKT0Uk7x9Z+2EhEQISaDZ+Ua7w+/090UfO3esIRO2Z+IphT7TenTrUtKZPbtxeLH6cbi63phDTZFK4Kp7OJLSdpoNqAKqhjw1pBA54qVkbXHT6n3n1BahLWt0Swa+W5D6rB25oBn22eKLhlLjPlK8tMl90K5SPAgDLr1MNw2/VTMlfhXBOAzR4FdixQgxRuP7h4BkUBI+xMU/BHgL8JcAQOZ8P+/5++eTTrJzIo7OjNvRabvO2ytKrWj8B7U0bA0="#;

const CANDY: &'static str = r#"eJxtjrEKwkAMhl8ldE+8JHeXHtSCWwd9Abeig0MLDtLnN6dYkJaQDPnzf3+65/h6wP3YzIUSGCUUkDFSSfAZAdjbwCbBzR4N7dr03aFC+u6HunCETGnhsKeFmrKrzZw9iTEBkxrl9qSgnlSLCxnoILy4dgtAMRNTEVKp13oW8R+rdYfbAhsyJKxc/AdHUhAedKniuO7RwfgFu3ly68p9A63CRy0="#;

const CAR_FRONT: &'static str = r#"eJxtjs0KwjAQhF9l2HtjNj9NCknBmwd9iGKL6UGQEvx5exNBUSx72GFn59sNlyEnjJHOiuEbBdWwsI0Wbls05KvYita0sDsvzPAZl8VOarBQ1u0tWELDUx82FdmHN/jgwCYJySsW/3vLdMx4RGJJuM1jTkV6QprmU8qRirxH0oSlNFVjNfCFLI/4q1q71f06T4imQDw="#;

const CAR_TAXI_FRONT: &'static str = r#"eJxtjt0KwjAMhV/lkPvVJm1dB+vAOy/cQww37C4EGcWft7cbKMpGIITzJeekvnUpog/UsoZES029m6Wm/oCrMHwhkIKVK4wqD3mGXoqd2ts93NEr233lvFhpA1biypNDNjbwa+O2BNuoNG8gXrNpOCc8xj7FQOwJ0zOQEHI3hFfWNCEO4yWmQEvafPBjmR/xd9nKqv7JG9i0R1o="#;

const CAR: &'static str = r#"eJx1T80KwjAMfpXQe2OTdusK7UC8ePEhJAoTdpApQ9/etANPSvolNN+Xv3w/Pye4FHOiBBQnFuzBAVkM6mi1XpzFZDFaaugU6UADRiCnWtLnNv9QyiuCZWSFF6ty/RLS1mDQcOw0rTNqtnEB01wDMKa9174VrhkD8RrEAfZ1n2oTmzHv6tZjltsi8xXkVUw0IO9iSONSTNNs7Ji/F7YD+5/l9L/+A09gRJE="#;

const CARROT: &'static str = r#"eJxtjz0OwyAMha9iZbcbmx8TKc3CzJQTROrQoUMlOuX0xUQqS4V4Apv3+bG+j88THvepCImCMGldKCk6CsBC6jCSi4dvV9uzrV4y0RxIFZgpefgBxuksiaIH9i8UmoOJLxzItVqwmo9dpm29WZBtHXEElopMzqFYGJTMkVKENi7A0qVaGwQsq+zmAJPzD649l4oCF9Ac+8AIXh+wOZm1dRvWHF0G7guQjUIT"#;

const CASE_LOWER: &'static str = r#"eJxtzMEJgDAQRNFWlmlAo2IUdtOBRcgqKHiQIEG7lxjiKdcZ3mfdvR4r6S2wIC9oQfoITAPHVXodn/O10SKYTE1j6OMVJ8eZJ/FlTO4UfUc2DL9/AR4MJGA="#;

const CASE_SENSITIVE: &'static str = r#"eJxNy0EKgCAURdGtPP484quFgbqDFhEWGBSERNTusyJ0dAeHa7ZhDxgtrRLcQFUaCpqcqR9w5udegWVoM/g5+mVCtCQJ/rLEIvVMfe+Pi18wuiP/N8f3IXw="#;

const CASE_UPPER: &'static str = r#"eJxti7sKgDAQBH9lSe/jkjtNcaa2sbUPWFwjWPj/eBYKgizbzDB61NOwTWFPIAE3GYwcina3KProhUHJhh/hFZFxKzUioveRn2eSdTT+wje/AEDkHbQ="#;

const CASSETTE_TAPE: &'static str = r#"eJxtjmEKwjAMha/yyH/rUsdWoR14AA8xumILE6QU1NubVhHEEUJIvveS2Bx8QQzpEosjHgj3tJToSHeEhxTC01FPyK2Z7L4aJutT9muAF8ii9EKNiD6aN53sbS4Ri6OzAetoKqqjP3vztSXywYb/OkB3UONOq+OJVY+anQTDKAYPcVRm/gWtW9WIw/fsC4oAPfw="#;

const CAST: &'static str = r#"eJxtjMEKgzAQRH9lyD1tdtAQIeYPeu090EIKpfQgon9vFgUVZA+zPN5M/Oeh4NWbBxGePhOEqyegZZETAEfhDmz9ivUmxbuOpHiYql6HbisGhGvJ5xbtJjVodun7+b0xszd0BrOsOWnWUHxzorZ6aQFyaS5b"#;

const CASTLE: &'static str = r#"eJx1jrEKgDAMRH8luAfbJFYL1dnF1cGt4ODoIPl+q4IotGS53OOOC3s8Nlj7aiICMop+JPUxaTD3JbVZ9zWQlmoI9RUcwhu3HVg7y+i0zdEGiBQlMvDTg4ys5vsDq2SiaVcq9jlSAg5kpvzGAjAlID9wApOkT7o="#;

const CAT: &'static str = r#"eJx1T7FOREEI/BVyPePCwrKbPC8xtvoDduYsrrAw0fj9wjO5a+4KZoFZmGH7ev8508fj4VWU/IQR1EjQHW2REnRkFZOTROusmEYDppntlMEnZ03xjwSPbLcEyi+WT4fZswpJYJEMrKCqNPGbF/eMgI9TY4E6PFfnGNdYeqnuXLxrJlfL+8pskiFSoacdL6n+tNCMdmh1REn42+G4PdSNx+1y6SSxX/gNRsZ9qsyVffWzwF9yucQnI7ziKvMHnzZJmQ=="#;

const CHECK_CHECK: &'static str = r#"eJw9yrENgDAMRNFVTukNXJBjCpMJYAgkCgqQKNhfJEVS/Oo/f4/vwrmGnQsSDLRbVDRkH+vK3sATIziJDYrSxhlMXf2XeBIq"#;

const CHECK_CIRCLE_2: &'static str = r#"eJw9TDsKgDAMvUro3trE1CLE3sDJE4gOLoKgk6fXVCLJ8P5yzNcG6+BGJCBaUkjUQgSMngPnrADjhLnqBJp6r6ue0tNi39+uSKObRWx5763Fnn/7AV3LG5c="#;

const CHECK_CIRCLE: &'static str = r#"eJwtjEEKgCAURK/yca85ZpCg3qBte6EgQVTITbfPJBhm9d6zNbSLDsc2pQgQct2hAiT19SPwRZiZGwHNvJ0+2tta0pNiPqmWmNvtWHc1oQe0kCAzQhj8T/oXX2kbIQ=="#;

const CHECK_SQUARE: &'static str = r#"eJxNjMEKgCAUBH9l8R71HkYE6rlL1+5CgYKokAT9fRYEspcdGEblFO7g44GcfCynFjOIQAySYIYURvW/Y1S2xWHXYuXXuSbLYAx11NW3jC13vDWMyo7oy9WIeQDpTB6r"#;

const CHECK: &'static str = r#"eJyzKcjPqczJzEtVKMjPzCsptlUyMlAwU7BUMDRXMFEwNFKys9GHKbEDAEfUDao="#;

const CHEF_HAT: &'static str = r#"eJxljjELAjEMhf/Ko7uxr7ZphV7hNhdXB7cDBQURB4fz39ta5IbjQT4SPpLk1/S+4TKYo4I7SXH08LA1RBRP6BSExK/0McWGDSV4hJq/ahPsuFJVwh6KZSdTP3NyPOjZlLxtD5T8uD+vmN1gmAw+rIyVrnOuvTa3WeULw/MmBQ=="#;

const CHERRY: &'static str = r#"eJytjkEOAjEIRa/yM3uwUFqmSZ0beAF3k7pw6cL7RzomunFpSD4Q4PH7Y3/ecTsvF4X4XlCQjpCQkUjZa0ihGfmolApYLTRfl62fJmHrH478C+QQG5lVY7kJjLXRyh5kmj+4VijnNQbN0OKf6A+KKtqgeYxEzmJBy5kkGhR2eTs0rj6zf428AJKEPnE="#;

const CHEVRON_DOWN_CIRCLE: &'static str = r#"eJwlirEKwCAMBX/lkV1qinSK+ZeSFlpoQcRB/15FbrjhTuzN9t2wFol3gtXlPORJZVtdJZ3lwRXp5wPsXUBwg3nMoh3c+xOJ"#;

const CHEVRON_DOWN_SQUARE: &'static str = r#"eJwli0EKgDAMBL8Sci9aLeKhyV/EFtODICWg/t7GsrAszE6seVd4CGeE9++7JBVCvyJILodo37V9JuQ4mMDx2lQgEZ5+AT+6AMG1GDfCHy6zF9E="#;

const CHEVRON_DOWN: &'static str = r#"eJyzKUgsyVBIsVXKNVOwVDADQV0zJTsbfZC4HQB8NQfk"#;

const CHEVRON_FIRST: &'static str =
    r#"eJw9yTEOABAMAMCvNPZGamgN1R94hMRgkRjE+zEw3HQ6ymxQk+skQBEZGQ5n6u+Yvs8CvCj82LwRD5U="#;

const CHEVRON_LAST: &'static str =
    r#"eJw9ySEKACAMAMCvjHWRGTbD3A98hGCwCAbx/aKgXDwdZTaoCbsARWB3oak/Yfo6kwAvCn82vfwPlQ=="#;

const CHEVRON_LEFT_CIRCLE: &'static str = r#"eJwlykEKgDAMRNGrDNkXjQRXae4iUVBQkOJCb2/aLoa3+KN+FD83lEw8EvwNp/Brmg69m97Ls2PNdLGA5yRJEKuPWuwH2lYTjQ=="#;

const CHEVRON_LEFT_SQUARE: &'static str = r#"eJwly8EKgCAQBNBfGfYuYUl00P2XSGk9BCEL1d/n4mGGgcfEVg5FexPNhN4LQUo9RRP5jfDUrDLmZ8hxsgPHe1dBTnT5AL+64AJ6zE34BzXIF9U="#;

const CHEVRON_LEFT: &'static str = r#"eJyzKUgsyVBIsVXKNTRVMLTQNdM1UwBiJTsbfZCMHQCMUQhe"#;

const CHEVRON_RIGHT_CIRCLE: &'static str = r#"eJwlijEKwCAQBL+yXB+iwSLFeX8Jl0AEBREL/b2KxTDFDGsoGj9o92QvgrbtMmVI+NxdOD/1x+spWYMbDu6YrGEFGckJE0A="#;

const CHEVRON_RIGHT_SQUARE: &'static str = r#"eJwdy8EKgCAQBNBfGfYeZXnwoPsvkdJ6CEIWqr/P9TAMzGNiK4fiS7QRnppVErlAeMfQeq0EKfUUHcBxtgPHe1dBTnS5BQEefuoxNuAfF34XiA=="#;

const CHEVRON_RIGHT: &'static str = r#"eJyzKUgsyVBIsVXKtVQwtFAw0wVDJTsbfZCEHQCFAQgx"#;

const CHEVRON_UP_CIRCLE: &'static str = r#"eJwlyjEKgDAMheGrPLKLVjI4pLmLREFBQYpDe/um7fD4hv+J3cmeEylSWAiW3dUtXZV5dJVv/y8ckd4NgcGTD9wOLWgFxnATRA=="#;

const CHEVRON_UP_SQUARE: &'static str = r#"eJwly7EKgDAMBNBfObKLVDt0aPIvYovpIEgJqH9vg8MdB4/Lve6G/jAthLsVU6aQCFrbofbvgSvh9ZY8+0HytZmiMJ0JISJOI4jODvIBJysXjA=="#;

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

const CHROME: &'static str = r#"eJxdjUEOhCAMRa/S9AAdygBigl6GuDAxLlzB7S3UqHH12/S9/pTXI28LHBOyQchV0kqWnnP66X1OF/cBmufe2LbuCxSe0DLxgFBljAjFKl5tW4Vv3EU3JJAJSkXyDvuHP41eDXYfpb/z5MJ97wYbilE7pX58rBOs2Tse"#;

const CHURCH: &'static str = r#"eJxtjLEKgDAMRH/lcA82sVqF6uzi6i44OCg4SL7fOEg7lCw57t6L9/Yc2Mfq4h4BHqLMm0Dg7Jjsm32eSdbh9CTVFOuPneJvWNhoUUpz983V5dn8voT2hq7tSR011KFRDqWVIGhbKhyGI3lfcQ01rg=="#;

const CIGARETTE_OFF: &'static str = r#"eJx9zbEKgCAQBuBXEXetO5Qc1Lmlh4gKDCIaQuzt87ChwVru5/j+4+y27gtL4DhydpVImIPWkt42VPL2GM/AZscHQAbYY1QBFDHBi5E41gRMliCkrlj3c2SmVqDUAp+p61+/azcDnT1B"#;

const CIGARETTE: &'static str = r#"eJx1zCEOwCAQRNGrEDxtdwItYouu6SGaIpAIwvmBhKDAjHmTz/FLQfhbvmQF4UHWgU7peG/geDBQOeuJXCuoRfsfCptR6Gvm5fWtAOAqKss="#;

const CIRCLE_DASHED: &'static str = r#"eJx1kEsKwzAMRK8yZG/VslN/wM0NeohAF1120ftTSVmUJAqG2ViPp9H4rN83Xo/pyZEYibitnXqGRZTHyNQQp2XcdHYZf6JSkc/KStxhsRGJSpeoDpSYWoLKTp6oJg+JlDrUdhQFcZjM2y6T7KA2pWZYbNRVoUIzTOZ4Sg8XheRkUNm5T/D76MkgLudsNeza/AAXpmBD"#;

const CIRCLE_DOLLAR_SIGN: &'static str = r#"eJxNizELgCAUhP/K4R75HiIO5tzS2i6vwKAhpKH+fYogDcdx9/F5ObKcO+SZFLGCvK1zKa2CHxsP/op3wjaphSxcGmxkMDSoRMMk03d9zOyqW52/ySC32k4+7Rof7A=="#;

const CIRCLE_DOT_DASHED: &'static str = r#"eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgI3D8nl5GZ8LM8rzpM5sSOGJx6WRKlDESfF6GiAM/N40N15/DoiBRlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrXdSQ3KE1dPYpU116gQD0KrMEJye4EkpdBYds8tp1HXwZhNd4W7W+afFvz/YL8ngx7g/yqfZWmS3U8fwCjDmtN"#;

const CIRCLE_DOT: &'static str =
    r#"eJxNi7kNACAIAFchLOBTI8sQCxMrKt1eVGKsrrg7kqbSK2jBFBFkGrNxHDKF65m8c/HC/X3ZAn3EFkU="#;

const CIRCLE_ELLIPSIS: &'static str = r#"eJx1yzEKgDAQBMCvLPcAzV1jc8kPfIScQgQLCRb6e41pAiHVsjus2p7s2GCPJxZC+sIR7P5r0LF40HO5IlZPM09giYPjrHmtTfrW3F5NeiIY"#;

const CIRCLE_EQUAL: &'static str = r#"eJxVyksKwCAMRdGthGygphQ6idlBF1HSgoUORBzo7v1NdHThncf+jg4ei9cJZBwZFN7aJjzLsYh+Qf8XNFmkHUHzaKjpr+FSALkbGd8="#;

const CIRCLE_OFF: &'static str = r#"eJxti8EJgDAQBFtZUsCZu5iQwBmwAIsQfPgRfNg/XgLmJbPsY5fRe39OHIu7BIa3uKpTW6t+35YpRAilsrKHpcEQpgCOlOKPwoV8Ru8hsTFTkV7DeQE/Qx75"#;

const CIRCLE_SLASH_2: &'static str = r#"eJwlycEJACEMRNFWQhrY3ZxjOtgiJAoKHkQ8aPdqZA4P5rPmpiVCc/i9CDq2tJ2m8HO7cPU9QXD4E4HN6nllAY2mEpY="#;

const CIRCLE_SLASH: &'static str = r#"eJwlyj0KwCAMBeCrhFygjdChkOYywUGQDk7m9j7j9PF+tLe/UpSPX6YJ5IGSMSSj6bVPpt6G90oD7c3kExYYKV5ntwWLnBYY"#;

const CIRCLE: &'static str = r#"eJyzSc4sSs5JVSiyVTI0UFJIrgTSRkC6Akzb2ehD5O0A51ALOw=="#;

const CIRCUIT_BOARD: &'static str = r#"eJxtzDEOwjAMBdCrWN4rsFsElZLcgJW9SiMciQFFEZTbY6dDO1Se7P/9XEmxwuKxR5CUn1I90g2h6IkRfi345rlKuwd3sofg3lMVmD3eiWCUYWJgOLfhjh+9Fa0SXMwlvhJElUZlmxoXW7SyhjvtCkyfbuPIOBkOObrsPd028A95bToD"#;

const CITRUS: &'static str = r#"eJxtTrsOwjAM/JVT95pcEztBCpW6sbCyV2JgQWJg4utxAoKByg/Jp3u43tfHFZfDcJooZmAWyyslFPQVvDhKSH7awgneHUOSHDFJLM4m2rxx53XBc5jrrrnP9ZvBvZiC6klLwccdRaK6XUz/ihs9OIwqCp8tx9ReLnpmOJrwR3kBSy0yAQ=="#;

const CLAPPERBOARD: &'static str = r#"eJxtjssKwkAMRX8ldJ/Ym5mUFsau3fQH3JW66KKCC/H7vVMRBSXkxbl5lNt8X+VybCZvzaWTJMCmNqhbXtSSwsDohK8cG5KF5oWAXEj2GJWfm7Ec6sqxvBdfO44GJclAH/4owFtEeZfkX8FUn1rRP/rZxaWlQVmd4rtX/1x/Ag40MrQ="#;

const CLIPBOARD_CHECK: &'static str = r#"eJxNjLEKwkAMhl/lJ/uhCeFQuHR2cXUvtpgOgpRD27f34mBLhiRf8n9lHu8Vn2mobnQirEZCmBcjJvg4PbwaaSPrjyzx1JVDpLry6qtjMLpyhrr0AsGxFbcub9YNpDZd8n5Pcsv7QBKXMIdzMz/PYA0bNOn//AWAIyyi"#;

const CLIPBOARD_COPY: &'static str = r#"eJxtjLEKwzAMRH/l0G4aCTVksDNn6UeEJtQZCsUYN/37WhmaQMMNEnfvzqf5npE+gZjwXqYcA3WEOC+PmAMpYd2MCgghrcb1/mKt3r/GHDEFunXQoR0Fgsbk6ldYd6NeiSxHw0lxYlM2cpjiFhp3kq1a9AQUBuvA/B89+QpunGLTL/8Cz+g7kg=="#;

const CLIPBOARD_EDIT: &'static str = r#"eJxtjLEOwjAMRH/llD0Bu05LpKZfACsDW0UrUokBVRGUvyfuAB0YfDo9+V47j9eMdzRssERzMHhNQ05rmwsmgzROt5SjkUIWJV2701XXPvqcMERzor0TBrGrqWdH0NuDoC00axwbFzyYIGC+uxBspcA7qWwJuahWhVttDUncM7jY1Ffak+QHbGmpzP2fsYAq58/1dm458ff1A6NePLA="#;

const CLIPBOARD_LIST: &'static str = r#"eJx1jLEOgzAMRH/F8p4WW5HFkDB36dodFVSzVShq6d/XZiFIIA+2791dmsdngSVjizD/MhKCjtNLS8aI8J2GoiszxOZY3NGlq6e69O6LwpDxTgJRuWdgaGzINn8obkKw6yb1H/ghdSCwsjd7Z93MQKTxhMghaT1yaegYyQ79AeksQkY="#;

const CLIPBOARD_PASTE: &'static str = r#"eJxljjELwkAMhf9K6J7zXoxRoXbu0tXB7TiHWwQHud9vAtIKJYQk3wsvGd/l0+h5GxacSOZrAYFyBHvXpWZKlpQCo1lN5ho4CON+rJmT+cSIeAzTeAjDaVxtL6SzFSH5uXrXoRvwKg3yD1gWGGnbIGKrO/YntCHv77xwJmRSUvZc9S/UAjKN"#;

const CLIPBOARD_SIGNATURE: &'static str = r#"eJxtjT0OgzAMha9ieY9bGzckEmHu0F6gGyqoQepQoYjS2zdhgQFZ/tHz8+dmGp4JpiUgI0y/tX3HPsWADiEO4yumgIqQV4KwFLltTuWqbT5ditAHvDvQq+0EBM4lTJ5m1k3IXSLLXjAyG7oUVqHsWGxB42ZlYKor4COnIxXwZLkTYii5+vPk67Xc2JK/ANfAGeHe5L2piqKklclFHwdcl61xe/gHLX5GmQ=="#;

const CLIPBOARD_TYPE: &'static str = r#"eJxtjLEOwjAMRH/F8h7BWZEBKenMwspe0Qp3Q1EUyt+TdIAMlYfz3eleSPMj0xr5zPSJLEypCpjey5Rti9O6BTYvT8uRPQ/h0FZDeI3ZaIp8g5I3GYWEjvVQVQr8P3D1u2rvndy1HzgxaeTG7MgXghQH04KdFiCcdncQAor+mi/8mzvz"#;

const CLIPBOARD_X: &'static str = r#"eJxtjLEKAkEMRH9lSL9owhoUdq+2sbU/vMNcIciy6Pn3Zm1uCwlMknnMpDLfKmxe7lYzRUL5ZGKCqxDey1Qt05Gw/rSsDQ5p11JDeo7VMGW6sCKajALB3od9y4vjZgS/ztr/Qa7aB4KYtObWuTU/+ADmoNA/7OQIPfoCEiozgg=="#;

const CLIPBOARD: &'static str = r#"eJxNjLEOgCAMRH+l6U6UhhAHYHZxdTdqLJshROXvLS6SDq93uTuX9jXD43FAKB4JIYnQgvKB93hw9mgQ7rhlrsHgutoK7lwyw+Zx0hYM00JA0MtpIV3a/IaSb7StVjTbtqCIqS7XzfACcq8knA=="#;

const CLOCK_1: &'static str = r#"eJw1iksKgDAMRK8y9AD+UHFRc5nQRSC0pbqwt7dRCsMMw3uepbAGcD3dvDiUNpMDP98lP/6cfE5aVWJAThLvyzB2tLKsw4bD7G7RC2+oGXM="#;

const CLOCK_10: &'static str = r#"eJw1i10KgDAMg68SegF/HsSH2suUPRTKNqYP7vZWZRASQr6wWlNP0PugZSW0iJmg/avC078L1+LdLSfUYvk63xkbwkI74hPsYOQBQHsZDA=="#;

const CLOCK_11: &'static str = r#"eJw1iksKgDAMRK8y9AD+QFGouUzoIhDaUl3Y25sqhWGG4T3PUlgD+DndvDgUm8mB63fJjz8nn5NWlRiQk8T7ahgbrCzHsGJvcpfoBVhyGUc="#;

const CLOCK_12: &'static str = r#"eJw1irsNwCAMBVd58gL5FKkcL2NRWLIAkRRh+xghmrvijtWaeoJ+Nx0nQft0C+0kvM0uXIt3t5xQi+X3GRcuBOKObWX5AdG/GDM="#;

const CLOCK_2: &'static str = r#"eJw1ikEKgDAMBL+y9ANWD55iPhN6CIS2VA/29yaKsOws7JDoECsYR1pzgtzOzTlfMi3fz9SbTdNa0JvW64wbO7wiPnLIv8QPVjAZOw=="#;

const CLOCK_3: &'static str = r#"eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbUiDDMM75FoEQuQ93L74VDabA5S/8u0Ds6Uk1XTGJCTxufuGB6tevxyYuhT4w+HEBmg"#;

const CLOCK_4: &'static str = r#"eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGOYNzCPRLpbRj7TMCXI7V+d4yTR9P1OrNkxLRqtarjNu7PCK+NhC/iV+AFZoGT8="#;

const CLOCK_5: &'static str = r#"eJw1ikEKgDAMBL8S8gC1oj3FfCb0EAhtqR7s720owrJzmCHRJpZA3gvDjiB9sg1syLROz1SLddOcoBbNz+0VRBjnO5YTQvT8z/gDiKIZog=="#;

const CLOCK_6: &'static str = r#"eJw1iksKwCAMBa8ScoD+oK7SXCa4CAQV20W9vYoKjzeLGRLNYh7kf/C8EKQM5oYDmfbhmVK0Yho8pKjhe3sFDtrNue3u+cq4AohhGaA="#;

const CLOCK_7: &'static str = r#"eJw1ikEKgDAMBL+y9AFqBQtCzWdCD4HQlurB/t5UEZadw0xkaawJ7XB+ceBuXI33S4rz5ynWol0lJ9Qi+TqHRoCdbZ82+DDqv6IHbl4Zdg=="#;

const CLOCK_8: &'static str = r#"eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzdVhGXnMEOiTSyhHSHOAXI7F2d/yTR9nqkW66Y5oRbN1zk0Nvj5dsR1tH/DDz86GRA="#;

const CLOCK_9: &'static str = r#"eJw1iksKgDAMRK8y9AD+QN3EXCZ0EQhtqS7s7Y0WYZi3eI9Eq1hEPcI8BcjtXJztI9PYPVPJ1kxTRMmarvPV2ODn24cVvf4rfgBt+Blw"#;

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

const CODEPEN: &'static str = r#"eJx1jlEKgCAQRK+yeIBiF4I+zNtEBKFBfejtc0y3+gg/lh33zYzdw5aW4GkPqz+PybBQfkJjN2DwkCcXpS73FyTjbF9xZ7fVzxQFBoYST0byjFx36BkGgUNnwRWk5Yr6Pikt4A/Rdq+yX6bcIx1uWk9roaZ2ugCheUaL"#;

const CODESANDBOX: &'static str = r#"eJyFkMEKwjAQRH9lyT2xu41JCrXgB3j1XlCwUNqCRVq/3s2mSDxIyWGXYd7OkHpq5wfcTupCCOiuoSUgKOLTqNH4stde20zlyRLY81eCEsIrA4EvRZBNmRg5vpRhkvhWTX2IJZp6Gvu174Y7TGM3zM+T8uYI1kQbgTMh2jdFoM3+B8TK+Apks8ZxR6QdKgVJhiBpi2d2wNKQ54KVE55MgUAF/4Bov6xwK3IY24KChVcuBiuluaTJjPg/l9to2w=="#;

const COFFEE: &'static str = r#"eJxdzEsKhTAMBdCthMzLa3zBD1THTtyAM0FBQcSBiO7exNYP0sFt0nPr5mbpoc2xogTSnhoGBgskx8psCAv3U1O4W/4V8poFK9rIrUzes+H6aY7D1MFOOUYIm0QsEZ2xS7BCJQHqE1m8Cp6cPdl+qC7Z/6YZrG9e9AAAzjhT"#;

const COG: &'static str = r#"eJx1j7EKwzAMRH/lyG7XJxs7hTR/0B/oZuiQJdCh5PtrR9B2kBEaxOnpdMurvjc8b9OdAgl1xowAtg6OGTpqMT+mdbl0YF3+Maba2C+WoJNWGkByyECQwxnSztL+8zk7OvoSLZhg8FJQEH1Mxo3zAFjOC47GRgcb33XQ8kigbLOhSBdkaKqeGHpSTc2nqIEcYSffe/LY6YTsr7+ND6UAefk="#;

const COINS: &'static str = r#"eJxli1EKgzAQRK8y5L/brMpmC4nQA/QQJS0oKIiI6O1NFESQhZ0ZZp6P7Ri7P8ZgxCCuwWiSJUvtn0dZ++E7NfgF82El+wJbKt1bILDgfClX4B3J0wvgIA3P1b3pWcgltCRVcsk+CtIC+Z3jDdDKKIk="#;

const COLUMNS: &'static str = r#"eJwtyzEOgDAIheGrEC5gqItD28toIyTGoSGx3F6wTv/wvpd72xW4yclakDaEUXBF6J7ksS+PHMpztphrXuJX8yV3A0uOyJ/kxPVIs0a/DVVfIK4cPA=="#;

const COMBINE: &'static str = r#"eJydkEEOgjAURK8y6b7YXyBg0rJ24yFMIbYJAYINyu1tq5Ku3fyfmcxbzKh1MB4vzSSDHdzdes1ahj0Z68d/ut7baHfqFOOdWm7eotfsShWkoYIgIFGcw5FbZQSCxaPkwYhYBDJMir8wCon2UhtORQPBy/DLcMuNU5aex31004BldpN/aNZAEhKKBlSn5DfTqVQ/K350jZtQdWywJ/Ub4A2wKlFt"#;

const COMMAND: &'static str = r#"eJxNjCEOwCAQBL+ywV/a66Yoiq6prSdBIBGE93O4E5vsiJnUy2ioT/j0Rpx6FYI4oTYK3+gZ/B2LveYFMSHkdOxkXnlLE2Y="#;

const COMPASS: &'static str = r#"eJxFjFEKgCAQRK+yeIAtJdTAvMwSIYiK9ZG3z1qhnxmYNzxHoVLcge5NSCWAGnftNQvvJubelRzbkROUHNJ1dqpRLWDQaJALSjWSh4+taC3H/32Nw+QfFUYfzQ=="#;

const COMPONENT: &'static str = r#"eJyFTTsKgDAMvUroXiWRQAu1J7AXcBMcHBQcvD8m2kIFocOD95L3CedybbCOJnHH4AQekHY7CBNMpEqIHmYTQ6/+GErqQAKCYhbhnw7t0s9PIqHLQ0SfJeT2FHK9RW/aZolV6gajODNM"#;

const COMPUTER: &'static str = r#"eJxljEEKgCAQRa8y/AuUUtFCvUGHiJTGXYhQ3b7GQoK27/33TQpLptNCg/boM1uoDpSOQjjElbPFCLpBD2ca2TtTqnevW8iBZL/q+anZNmcmbzENpEYuQtBHKC1mqOYCTTgtSw=="#;

const CONCIERGE_BELL: &'static str = r#"eJxtizEOgCAQBL+yoSfebQihQGobP2BHYkFpYe79og0WZLudmXzVu+Fc3U5oqgQhfQp6No3/AzRuNM/Dlby8Xcmj7kqsCelzxWuETDQlgoUZEIQ2wAPrtiZ6"#;

const CONSTRUCTION: &'static str = r#"eJx1zE0OhCAMBeCrvLwDMCITYAHeYA4xGc3gwsQY4s/tLRvjAhdt0n59Dcvwy0jD+E850hPLHqkJ6S1xRFpiG/ucZG7YhVe578L8zQl95Ec76PfqipTdTR5BImY11UR9rxt5hVYZWFXxSdDCKVeqwl7Uw19yAr8RQ5A="#;

const CONTACT_2: &'static str = r#"eJxVzEEKwyAQBdCrfGZfmrEhuFBv0EMEI1UIpYhQe/vOmE2CC/n+93WftWVsnp68gO06Y8ak52YxUXB37YOLpcY9If48MROqpwchdklG0VEHV1Ns6KOschnCt2wtC7ME2c6EnMort/EiQx0Et5d3QjeeVJnBOh+J9ReBSk6Ql4sc8UL/aZ45JA=="#;

const CONTACT: &'static str = r#"eJxNjNEKwjAMRX8l5H24RNEJbZ998SNGV2xhiJSC7d+b1IkjTzf3nGtec4mwWLzTBWiaGRhGvYEHvl33GRidOSjvTA6+wDstJVqkCaFaPCI0iyeELIERYkiPWHotmgrO+JT9GsALQYJ4EWgUQwWBvrUza3oGqGxRlhv1tcZ9u5I+BVXkB8qPzn+Et0jbakc/Lh88Lw=="#;

const CONTAINER: &'static str = r#"eJxtj70OwjAQg1/lxH4ml+ankUqfgK4d2KowMBSJAfX5cUDqQhXposT2F2d4Le+H3C+nyXvJyNUpkiKowSt6bnHVhE47lMWQvXyHa4tiFreqOXSSqiK2TGEKhbawJaTqhNeBR49eGo0wOYDJD9ZYmiqigiY1aQPxdhqHc6s6Dnthc+INZbZw9TApsH/Tkybj6/TxF+WIQpWVN+2P8pOxdEacC8KufgD6tEfM"#;

const CONTRAST: &'static str = r#"eJwlikEKgDAMBL+y5C42ORQPaX7gI0oUFDxIEVFfr7XsYRlm1Nfi2wy/E7EQyneB4NePpn3zpns+FkyJRhbwkCMiQlvHcrI8Na6Rvbb7FgY="#;

const COOKIE: &'static str = r#"eJx1jLEOgCAMRH/lwg5yhIoD8gd+BImDo4Px+60MJiSSXpu2L3f5rNeBfTUbA0Klh0oH2qKKWu1jxUp/mZKn117yF7I4gfbtPH8oZ1DGNIBhxAimAUtg7NADGX01yw=="#;

const COPY_CHECK: &'static str = r#"eJw9jsEKwkAMRH8l5L7rJkSxsLt/4NW7bIvpQZCyYPv3JhZLIDAvmWHy+9EVxoIvYqAzsI0EwZpPfql5mVoHnean9oIkCGvBK8KyFWSEzzx23fm289W5ud1X8z/9JkCXFigSpMAhDrb4Li05cgWmlVL7fViHOHiVo8YXaDMp+Q=="#;

const COPY_MINUS: &'static str = r#"eJwtjsEKAjEMRH8l5N7ahCoutP0Dr96lW2xBRJaC2783qXuZ5A0zIeHV3gUGRaQzws4yrwiDD1afMYWTxlLYSu4wIkpk2yOyJP4wJtTSnrVLxSN829rrXKWtvRQ+j15hjXjzQJdsyBI4w8YuInz32amlBMKVXJ4JYLCLyHxDT6QfYrss/w=="#;

const COPY_PLUS: &'static str = r#"eJxVjMEKAyEMRH8leNeaYEsX1D/otffiShVKKSJ0/fsmbi97yTCZN+Nf9Z1hYFBICjbRs4JBrFf2NH30J8Gin/D2D8ce7iUuDzzCLacOIyhGv3XthVPH9Hw0Fq60MaXk+ix95tyWXvSfRy+wBnVzgJek0SBYTdosfOjukpWXOGBf0KZJAIFZ+JAMyUT8AVJtODY="#;

const COPY_SLASH: &'static str = r#"eJwlTksKAyEMvUpwrzVBSgfUG3TbfXGkCqUUEercvklm8x7vFxLf/VNhUTJ4M3DgyUuYWJNyjhep5ThqmTBWMpwxykKx1f5qk7vBwDg0/vV9NnV4Lbscv8/ZYE/mHgCvxaJD8Jas2xjoEYoXSxSwbuiLNoDAbQz6hpzIf2eWLP8="#;

const COPY_X: &'static str = r#"eJxVjMEKAyEMRH8l5K5VkdIF9Q967b24UoVSigh1/76JdqF7mSGTN+Oe5ZWgG4/6grDtrskNQp8e3Imx4A7w7znhvfwH1xQbfMraMuUWoW4eCc+pPHKbUffIxaG185va3AvufW8ZVo9XC/ochZYalDBCLiTmZqPiiC+gO2sVBwEG5EIyhngifAFnvDg2"#;

const COPY: &'static str = r#"eJwljLEKwzAQQ39F3G7Xd5jSgO0/6Nq9OKGXrQRDkr+Pz1kknpCUtqU26LL+tGXiSDgyvQhbNyGcN5wD9nVuOkolPWxX0v/bFHOmdwQ/q2PPCE6cn7rIJ9ZgkRE6K4c6GhD4qYvYkV2UC4UjIcg="#;

const COPYLEFT: &'static str = r#"eJwlykEKgCAQheGrPDyAaepCGOcGHUKmoKBFSIu6fWPyFt/i/SRHk3NDK8Y7A3nVWX1+mabxM1313rEWs2RkG1KNiHDwOodkQ097wh+G5xUT"#;

const COPYRIGHT: &'static str = r#"eJwlikEKgCAQRa/ymQOYmi6EcW7QIWQKClqEtKjbq7h4vMV7rFfV+4B+mZwn6D9duywJL7MLP+U9sWfaXEQyawwlIMDCdSyi8cmPfWzSANyGFd0="#;

const CORNER_DOWN_LEFT: &'static str = r#"eJw1ykEKgCAQRuGr/LiPVIwQdNZtOoRQkCA6kATdviGQt3mLL3Arb8n1BLdc+x2Vh9FwMAs8rFYU5kEocOoXjqh2K+RZkxOoJTPJbe7HQugDUxcX5g=="#;

const CORNER_DOWN_RIGHT: &'static str = r#"eJw1ylEKgCAQhOGrDF4gFaUX3Rt0CKFAQXQhCbp9myADw//wBe71raVd4F7auKMyHkbDavzhJRSFbSkKnEbGGdXh4J49yUPPSWVjJxZCH42kGIE="#;

const CORNER_LEFT_DOWN: &'static str = r#"eJw1ylEKgCAQRdGtDP5LKkYEOjtoEUKBguhAErT7ZgJ5PxfeCdTrW0u7gHpp447KerAr7OAMSCkMyzQYKI0MZ1SHvFlvyTMyMs31WPdzRvgBlccYkg=="#;

const CORNER_LEFT_UP: &'static str = r#"eJw1ysEKgCAMANBfGd4llUEIc3/QtbtQoCA6yEt/3wriXR/JaHer/QQZtc8rGY8QFapomJY/MEmeBY5ktuAguGLX/CanvEWLO35dEz9bDxf3"#;

const CORNER_RIGHT_DOWN: &'static str = r#"eJw1itEJwCAMBVcJLlAjSn9iNugQQgsKooFKods3FuQd736OpNe3lnaB9NLGHQ1awDBxdoLBMG2rYpI0MpzRHB583pM+WB2q/YPujzXhD4zGGII="#;

const CORNER_RIGHT_UP: &'static str = r#"eJw1ykEKgCAQheGrDO4jjYkIxrlB2/ZCgYLoQG66fZMgb/Pg/0hqfnMqN0hNpT3eOAs7uBUQFn2GaR6ESUKLcHlz/DFuAVXZPpzwxI6V8AdSBRfl"#;

const CORNER_UP_LEFT: &'static str = r#"eJw1ysEKgCAMANBfGd6lJYMQ5s5d+gihIEF0kAT9fUrEuz7Wmp+cygFaU2lXMB5mAgLfkRGe/iCssZ2wB7M5BIe3XeKIOFiytH69J3kBW+4X9g=="#;

const CORNER_UP_RIGHT: &'static str = r#"eJw1ykEKgCAQRuGr/LiXHJmIQL1BhxAKFESFJOj2TUG8zVt8rrdyl1wP9JbrOL2iGcSwBitkWQU3/Sa4HkfC7tX2iksvkcEwEoE1J7IfFxQelrAYkw=="#;

const CPU: &'static str = r#"eJx1jt0JgDAMhFcJWcC2qBBo3cAhRMX4JqX4s722pYjSPiW5uy+ctvPo4FgnxwZli8DzurCL+2mwRrDPUAiXPzpdeaDTAUvZGCVMf9qQpje9DY5hMtjLBtSuvOGlnyGyjgLZcN6gvC6KhCggVGhF31I36LFWOA=="#;

const CREATIVE_COMMONS: &'static str = r#"eJytjFEKgCAMhq8yfG9pFhaoN+gQsoKCHkJ6qNvntKADxM/H2PbzWVojbTPQ5YRqBMQ0pAA68+ptXf7e7uFYYHJiVBIG1KHBHhjJqTR2oEBjQT7R2KaOybyX1GMt675S85v0BhzFLfw="#;

const CREDIT_CARD: &'static str = r#"eJwlzEEOgCAMBMCvNPsBLdEb8BklQmI8EBLb39OGS7eH2Y29XIM04QR1SQigdf92j2rfDqqlPXUk8IEcNy/k+LavkAQDZtWSDSqvFPYJs67yBE7yGnA="#;

const CROISSANT: &'static str = r#"eJxtkLFOBDEMRH/Fut4mduwkKy3XbEPDD9CdQkEBEgXi+xlHaAs4reJdbSYzb7J/3r7e6PXx8uHSSKuoUkjfuIrpVBkbq5Qgxz9S6YOqdF3jPTUG9dBjyCDD4bZkIVbp9Hu5XPeHjLnuZ5gWCdokWNnEtmMToyY+KI0aYT25xGFp1yj3AnNI3HrGr1HWYwga9X/EMzxmAXyAxtnZclFBHqxE+/rwe2xITvKUGdpM6VUqyhv6rxdGfEMwS16J8pLGr/NfROUEZLuDqIO0zUSEzhmc5CiEUpyEy9Rznmd/ANAiXZE="#;

const CROP: &'static str = r#"eJxFyisOwCAQRdGtvOBJOy+kQUzRNbX1JBVIBGH9fATkmiuO5lgS/tu8F1jFRYI4Z/2SOBP0GCbokuJBfn5TS8uHSzZ+uRTQ"#;

const CROSS: &'static str = r#"eJxdzqEOgDAMBNBfafAd62UVJGMag8UvICoRZIKvhzloapon7i6f9TI65mEVIVQQKPbj92u6JCfYI0mQMFF3mDZ18skgMBqrqadfD8NYt+QJ91Dy2NeVB2qWI5I="#;

const CROSSHAIR: &'static str = r#"eJxtzjEOgCAMBdCrkF5A28E4AJchDCTEganc3kJBjHFqwn/81oZUQo6mOMAdTGCZJLP26e2mubc5XdEwOiCJmSQ+wVRSXnHyxgYejx3Tsq3j+FIljKvuGFv+Wp8T3n/0pIlv3xM3OQ=="#;

const CROWN: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Im0yIDQgMyAxMmgxNGwzLTEyLTYgNy00LTctNCA3LTYtN3ptMyAxNmgxNCI+PC9wYXRoPr6hDrQ="#;

const CUP_SODA: &'static str = r#"eJxtjkEKgDAMBL+yeLea0LQ5VF+gjyh46EXw4P/RFBRFCexhlgybtrwXLEOzBijIRQGxY80MRl+PDXPxTvyTtkYnUmgzps40Y7pks0AL+Z8igiQH5yNqmIkgZ75Zb+z7vhLbyjYUvssDEdktxA=="#;

const CURRENCY: &'static str = r#"eJx1jEsKwCAMRK8ScoESC8WF8TLiQpAuXCW3b/x0UbCrYZI3L6TSUs3QGD1CEkZyljoyhmO+Y6jlziCO8UJQYjwtZpPRDO3IAvUDdsIRjjn5DWrH1y3LPSc/6PLp3vsAC2E2Yg=="#;

const DATABASE_BACKUP: &'static str = r#"eJxtjs0OgjAMgF+l2X2V/iFLgCfQq3eCJJhwIGqIvr0dGsOBLF3X9cvX1sM03ebHAP2rCcQB7p5TgP7dBPPKk4S2Pvywtp675wjXJpwFiLsEAsV6DBjLlNFMbDgmSCgX22kJ2EK6kZSoR/dU1Q5M7AMXHXWvJ8BFZ77E15OigKKtsf5Ef+XoI6H4tMhoXmoUVALy65T95d/9AViJRYg="#;

const DATABASE_ZAP: &'static str = r#"eJxtzb0Kg0AMB/BX+ZMHsE3OwwvcCd06tGv3YoUWHKQ66NubExEHCeTzB4lt1/36oUUzJWIh/OdEjtBY8TbZVqmOl43VsX+PX3wSPR38i/WmcLiuwR7CRSgzz+pghQ2H8wPLgwO4uos1CpET5kwdP5WF15xDteMFT8oz/Q=="#;

const DATABASE: &'static str = r#"eJxdykEKgCAQheGrPOYCNYqLAQ06QFv3YUKBC6kWdfs0Iije4ufBZ2NKS94iwuGIFWEtFUI4HZnySjR1tnlYZ/O4z5gcDRrGs/QCjfaeYrB4U3VFH8rqB9XLLhnRIcg="#;

const DELETE: &'static str = r#"eJxFjDELgCAQhf/KcbuUB5GCOre0NrQJBQYSDQ357/M0iRseH+97Zy5/B9gsztTDMOkoRigXpPQEBH05ErSMP4vMKzrT8dqZeJw7JLIoB4RH5lQ5mQkhZdbssvW5pVOtqxNW64vmvmIxJbs="#;

const DESSERT: &'static str = r#"eJxdjkEKAjEMRa8Ssk9s0lRbaGfj2kOUKii4kEFEb29nlEGHQAI/78PL7TK26wnaq6AhjAUVoT0LiuKQN5/vkG/1foZjwYM4VvCs+8ABDBQiS9/iq/bj5jFwD5IlkDmwNWArQP6B5sg4kedAiSIrJY6T0WTy49NVQIxjTZC+ZdnxFtzCvgFGazMK"#;

const DIAMOND: &'static str = r#"eJx1jrEKgDAMRH8ldE9MG6UItX/gD7gVHBwcHPx/tGm3thwcd/A4LjzpveDczO7Ig2WS5Gi2oMZV8pfb07JCtgYQjQpgF2DUBRwSWCYOE8OUD8UPV9AfXg=="#;

const DICE_1: &'static str = r#"eJwli7ENgDAMBFexPADg0FDE3oAhEIlwOhRZItkeTKqT/v5izadBZ1wRnpJMGWlDqI0xfOg/NJdLbZjmV4mzdxLvwxQS404BKOi0kDtf5QV79xjt"#;

const DICE_2: &'static str = r#"eJxVy8EJgDAMheFVQgZQoxQstN3AIcQW05uUgHZ7jXiwl3f4P54raRM4cxT2SDNCuTyOCJzyzvKl+qYHJoSqG1yvv+COVRiix4UMWO4GUtL4IwtkGroBfFUgSw=="#;

const DICE_3: &'static str = r#"eJx1y0EKgCAQBdCrDHOAagzEhXqDDhEpjbuQgfT2MbipRbvPf//7mg+BHnBFqC2gQbhLEg5IDoFzOVlGbmPTdRP9rL/or10YUsCNLDieFlLS8k0GyPyYA7IfegBgGSf3"#;

const DICE_4: &'static str = r#"eJx9jEEKgDAMBL8S8gA1CtJD2x/4CLHF9CYloP29jV5E0NPADLs2x0WAY1pZHJJBKA4HhD0F4Vscl8gVfUVReNvqztttFobgcKIRDDcdaVL5SOan0PiR6t+rnesgL1U="#;

const DICE_5: &'static str = r#"eJx9zMENgCAMBdBVmg6gFhPDAdjAIYwQy80QEmV7qV70gJf+pi/9JoU1Q7E4Ipz3PKLPbJE0Aoe4cX72VFXVKBLO9PLnzL5kBm9xpgk0dwMJyfFF+kdoalDta5sCUh+7AHhaNwE="#;

const DICE_6: &'static str = r#"eJx9jEEOgCAMBL/S9AEqmJAegB/4CCPEcjOERPm9Vk+a4K3d2R2b41IgHw41Ase0cnGoCGFPofBzXnBEyPXuVHm87WXn7TYXhuBwUgaIu0EJkvCNlP5hpsGoaaS2kL6+E7F9PoY="#;

const DICES: &'static str = r#"eJx1jF0KwjAQhK8y5D0xmz9bSHoCPUSxxRQUJAS0tzdJH0SxLDsL+82MT/Ml47lMOQZGiuEVWNE4L9eYt89ajmRIayOpGQZ/qMHBP8YcMQV2p6PoFchAC8vLjkoogyZyG65v3HJTbL+Ma8gTSbjaWxs/vWcH6qKQ9AeVBJk9ZuH2UIf+C70Bn2pCsA=="#;

const DIFF: &'static str =
    r#"eJx1ybENABAQBdBVLhbgH7pzGxhCorhSIeYXjWi078lo06gXV8EUF5JT8cdU7mRCsM8w3tlo4hYH"#;

const DISC_2: &'static str = r#"eJxNisEJgDAMAFcJGUCb4DPNBg4hUajgQ4oP3d5GS+nr4O7E9mzHBjkiBQS7C7nw+agy/l2lfjX4P7W9287lSrBGnImBOA2BvLrVF1zgHfQ="#;

const DISC_3: &'static str = r#"eJxtzDEKgDAMheGrhO6NJortUHsDDyFRUHAQcdDbG60FB6c3vJ8vyLzJMoIcrSE2IGfaTac0MRTpj2Ht9wmG1nQNEEtpCR06WyEDobc18h3fUQyZfKlMK8m/IvmHVMhZdKCm6h6+5gWkrCuC"#;

const DISC: &'static str =
    r#"eJx9yqENACAMBdFVmi4AVJcu84MgQVXB9jQgcKgn7hTdMRp55ZKZMEMJ19E03W76PvlsG3ueFkY="#;

const DIVIDE_CIRCLE: &'static str = r#"eJxdzLsNwCAMBNBVLC+QQIEoCMtYFEgoBRXePjbOvzrJ986p1b0Auw2dR2BvOTSDpNwj5rSoymna8bZi5jacG//DiuLdPdvvX6qdWoEu5YpAwxDx9dD6fACrWyxH"#;

const DIVIDE_SQUARE: &'static str = r#"eJxdzEEKgCAQBdCrDHOB0EBcqJcpSSFaiJDevhk1iVaD/7+vSX7LUCyuCHfcc7AoNEJtQaJcIgQfj5B7kSpHziy8c+aMl4cqqCNX6PJWjidfxZbV16qBVN9MLH/4LWv/eJi21ZM+o90yTQ=="#;

const DIVIDE: &'static str = r#"eJxNjEEKgDAMBL8S9gOSgqKQ9jOhh0Lx0FPze40B9TSHGUa0De2V1DI20MhgkM4bCUWWsEV6OytNzlhBxo8lS8HpPLz2qsjvyPs7i/W3vADljiDy"#;

const DNA_OFF: &'static str = r#"eJx1kEEKgzAQRa/yyT5pZsZEBfUGHqLYQgu1dNGF3r4zqZQulJAMw/t5GdK9zu8bLr0bKYEnT0ESKCTPIbYM0ZpQhTS2YJ6MakSTRr1oTV6pG7qTiYbup2NQmiSIiMZyyLnWStHLbMdULEUnm6wq2h3VTDWyzeEPcIXGdLZ3cA1qTHwgF5tD37a1gzmixQHLdtXMBNqbq/B4zCMo4/sPf/rH/XnFyr1jdlhJq8OytUtpNWqh4QOs9m3d"#;

const DNA: &'static str = r#"eJx1kDEKwzAMRa/yyS7XsmLZhjQ36CFKOnQpdOj9qaShdHAwhg/vP1l4e98/Tzyuy62A66FJtZGCJYkIMkomXfbt4rV9+5UHSjk4tdGJ0xgdJVXuJJar5Z4bVcsyUdkKB4WLcClchEvh4sR9cYN63e8Mr7B9iCeogWNJzE0xwmuqfia4ZAycMHXVJzNmD3PwfM4z2P7b+d/4LwfzZpk="#;

const DOG: &'static str = r#"eJx1kk1OAzEMha9idR8TO/5JpFKp6hYuwA6VRRcgIYE4Py8zhXbRajQeZxLbn5+z/Xz9PtHb4+ZZKjlL6gFO4+xKnU0bKUcOCnZqx6LctbFlMRZp+FtrFKMkrp2zNhJO9cUqsoQHCVYaUhClcEcUYXOkbWbI55vd9mFC7LYXFENErjjHivNtINIzcd5GQ2KHh03646ErHgJPOfOUK55y5ikXHrjdfQFCQm3jDlAnsZ/bqHF/C3VQOmBPoH8StJ/vhdPn+3IjwiA4zTDLvTQWx2r91OUx1GL3OMBBe9op2TEohcbTfvXZBWTpJdjMFu3q7FVC51aZ8kGHVsdHGajTcLB23XfuVWi1s9KS0I+4BnPBmETtEFsCs6jxj/4Lg1t75w=="#;

const DOLLAR_SIGN: &'static str = r#"eJxVjEEKgDAMBL8S8oBqIrUIbc69+IiCQgURDx7q702gFwlh2GV343lcOzRKSIzwKhWNu1Qyo8TBUhLv8lTYEq4UwOfF+TI5D/Zjv1D/HpmXZ5uwsnzjsRoQ"#;

const DONUT: &'static str = r#"eJxFTUEOwjAM+0rVe0yTlLFJbc9ceMQUkEDigCYO8Pul22FKHEt2LJfP/H2Ge403STgHTrM4dSQfJkEmvfLo8tT16TAGysie6OvHMUDDBWwEhRBGKDEEGls59Z5W7LXY+xHsVyNLDPbfealxe9rttgJfCyHo"#;

const DOOR_CLOSED: &'static str = r#"eJxty6EOgCAUheFXOaOj954xR7iSLVY7m4FocDy/UhiBnXT27bcnvwX37k6NoFxbJghp8/Q84vjBqsElW1uUrKc/SKFMRAOUdRHt9gElqxzE"#;

const DOOR_OPEN: &'static str = r#"eJxtjrEOwjAMRH8l6l6Tc+xWkUIldlgZ2CIxZGBgQP1+riBVHSIPHt75ncu7flp4nocbUrCWqgYNkQNuXWHDUk5bZil7kiC21AFUkOQeoVBXiehfmfjEskngcwW7fx+MEDWVPF+d3rtvocN/EIczk+1lIy4H8BfisZd9AaQ/OCY="#;

const DOT: &'static str = r#"eJyzSc4sSs5JVUiusFUyNNIzVFJIroSxioAMJTsbfYgSOwAHXgvJ"#;

const DOWNLOAD_CLOUD: &'static str = r#"eJxljDEKgDAQBL+ypPfMHRcugRjwAT4iYJFGsBDfr7FII8PCwsDks14N++I2BSvFlFaDwYM7gYwRG5OlqhTQ5z8p74skKq7kuTdKHiUWsNzpb44INmhn0qEf09kfsA=="#;

const DOWNLOAD: &'static str = r#"eJxNzDEKgDAMBdCrfLIXTbS4tJ1dPISgoCBaUMTe3rQ4SIb/Ay9xcbwWTJ4GYbC921EgqHXYaOvtfzdym5aCq/JRcPHY0rbuM+Kx7tfpqQOrE/0Dzr3QDwVXaBJPDSGxJ7aEJ6doSkn1xb5PFibT"#;

const DRAMA: &'static str = r#"eJx1ULsOwjAM/JUTu03spHlIpQszH4HC0BGp/L+4dEAM7WCfcuc7J5nfz8+K1+3ygAWYrRrssszXQS/zn5iQT7V6rmWdYFHPct0x9YAmCeaS2TbJElnmPYjDJYLERgkZ8SDCiiY0bV20aqVlNK0IR7NBDUXt3tidVfbKqJq7RN7VJZGI2ngqmDQjkS3Ma8TKVWa7M3FkvI3gDB1I58HONlR+Q9cotk8mYoHvmH6OL0h8XSk="#;

const DRIBBBLE: &'static str = r#"eJxtjL0KAkEMhF8lXL/jJpv9g71rrH0IWQUFCzks9O3NaqHFkZBJMnzT+nXttzOt88R+ov40FdPXR5e2+/pLux8fFzrN04ErOFCEr3uOECG7ldhbQ5UEEsdaddCD+mOFkc0VFO0uIYljKDt7jAjHCaGQDdlgC2Ky9By7ImRKVhUqVIgz8o94A1OyMj8="#;

const DROPLET: &'static str = r#"eJxNTTEKgEAM+0pwb7W9q0VQF2cfIefgKOjk6+05SSAkISTjud0H9qlZRaG6ORzdBycvHSkJJR4okbFdIY1yoGcrxAYNr8hRyIhs6SHCAoOkSvbfgz/NPLb1cH4B+JcZPA=="#;

const DROPLETS: &'static str = r#"eJw9TjsOwjAMvYrFnkecxE4ilUocgKknqMrAiAQTp+clQxXFlu33W9779yXP2+VRRR35SEgSpQRFy2wF0SRyUg+wGnh2TlVDhvatInVxVJMqRnYYs6LYYBRJaGVQumRU37Io18LGnR2R95QIbzJtymy/y7pcR6p1ObMRb04fv2tEZ9JZ43y0yYjpgFGOnyKdUg77ZF6MdgbbHb3JLJMUmKTrwNbT7w85Azev"#;

const DRUMSTICK: &'static str = r#"eJxFTztuQzEMu4qQXerTx/J7wGtu0EME7dChBTJkyulDJkgymLBNkSL38+nyKz+fhy8fVkOI3xrmaT20LC1Dh011c3zHRuBrtrZh3nzqZhUStqUSZJoPTasWEEnlirnyJ8yStDGpFVg7JkptODTRaktgbxfobFBJca64rRO6oGG2YCSvh+P+wQLH/Vnj393iXqPZooVwCiTlWQT74T6aMRJ/tT1gIccgpKPkraAmzZEw449ud9/X5hvxnkEj"#;

const DUMBBELL: &'static str = r#"eJx1zjEKgDAMBdCrfLpHTRpbh9i7CA4ugoP3x3QRhEiS6ZHPt2u7D+xrOsswox+zb2o2dmn2ujCEySewjAx/C4QXiEBJo0SUH/G0CZVqFKhe42MPCnM0mw=="#;

const EAR_OFF: &'static str = r#"eJxtjs0KwkAMhF8l9L5xk/0ttIU+QB+iqKAg4sGDfXsnrVAPPWRnCTPfpHvN7xtd+mbKJJXTHDiRjSfBFPJn74RT4VadclJS9tEFjrkZupOlh+6PAQQCXBJLQC4WDhk5iQfuyhVVksaMvrx2Wqu0G4WEc3AcIyqrAOZb2MtRr6gRRnt15yTjTIJ/mHH3ujZqVcCk7pzH/XmlRfpGG/r8RCHQZVNYzTR8AbcRRNY="#;

const EAR: &'static str = r#"eJw9jDEOgDAMA79isbekKQ0MpT/gEREMjAyI95NWUFmWB9uXL71PHOuwCRafVHxCNSFURdBOECfNgTRaF//ezaCh5LEySu6kkBqKbcZtanKWT1AGf2jC1K8vtAodcg=="#;

const EGG_FRIED: &'static str = r#"eJwtTjEOAjEM+4rVvUd7JW2R2i7MfIANFSSQGNAdA/wep8cQy0kcx6U/lv68YakmTGLQP9V4P9iXbCZrZbeJWnld3ndcqzkF5O4sL0CFjYgDBQ77KQcEJK5k5ZrD2HUsltIhTAO1cwM3C+Jqk51ZMom6gzn+KEf95R2dD6x81lSapv0AE+QqvA=="#;

const EGG_OFF: &'static str = r#"eJxFjs1KBEEMhF8lzL3L/HTSE5gdEM8+hKigIOLBg/v2pkd3l4ZUkVTny/b19P1GL6flMWCZdNQHh4XSCvFBHeEkDFmzfFl9brDZ70YCHZRIpzFHTAb1aGAN8rLZBN7HXDu8GUJ92be7Cd23K1oSbkpiWIffSwcP+qt8vDn/50brsF6b0ltxoxW3CReG64NoU/CqcNbKcI95YY4b8+P985XOclp0obOWlP5c9GhXdIb2X+a+POM="#;

const EGG: &'static str = r#"eJwtjTsOhDAQQ68yoh9vMp/MRgJuwCEQFJQUVJyeBNHZenr2eK7XQfs0LFlIZCsQZSSnwD/Y4dGSc04MLWxQY0V1rqiFP6JwR7JWs5GjvIBeJchgShkS1JQ+28k9zOOvP88P0ZEa4Q=="#;

const EQUAL_NOT: &'static str = r#"eJxdzU0KQCEIBOCriBd4GLR4YN2mRRCt6/Y1/SC0EuWbUUuuiboE/pmaCyyYc/VM3eEa9YOJuiWIN4rk2lfksVZzW+V8MToAUNMgmA=="#;

const EQUAL: &'static str =
    r#"eJw1i7sJACAMBVeRLCARLISYbSwEsTbbm49Wx3vc0Zp7JMEODZIUx9FVFbqwAVM2h8nNd3qBNRJjNN+9uXMVuw=="#;

const ERASER: &'static str = r#"eJxljbEKgDAMRH/lcG80rVUK1dlBP0Lq4NCCg/+PSUEXOS7Du3AXr/0+cUxNGWHZ9OTUyXCVJY/OOOpzoMGIk1AIlSsUXfY0wCuHqv5rsrKTumaOrfbP8V3ZrBW+jP+keDAjIHzRA7T1JMo="#;

const EURO: &'static str = r#"eJxtjTsKgDAQRK8ypM+abP4QAx7AQwQtUlp4f0xSiIUsszDzGCZf9W44V7FbaNU0i5KXkZX8JbalH6ATfA0UMKTGSUcseQuUMDQzeGg+FCxZGHKIHcX+uSNDUXYzW+/AA+FjIqY="#;

const EXPAND: &'static str = r#"eJxtzTEOgCAMQNGrNOwgLaYyIDdwZTdxYIDEwXh+i8bowNI0zctv2NcjwzarSgiEmjVXBj71aHy1IDO3VcUwNBnD6xcHyIYSoTCbxciFsEiggyU+GZ/cbVuwXVzR8qqb/vATduUvL5SdLq8="#;

const EXTERNAL_LINK: &'static str = r#"eJxNjEELgzAMhf/KI3eZSac4aHveZdfdhQ0URAuK2H9vWgUlkOR7fDwb2qXDz9GHG7BZ61YgKHW40O9d3bmQb3MxlLuavH2kDm/DNMShH/8IUz8usyOuYCB8rFcWT8XbLEZW6UnYxJEwIeo1iikuk5/dHeKfKVM="#;

const EYE_OFF: &'static str = r#"eJxtjV0KwjAQhK8y5N01u/kttIUewEMEFRREBH1ob2+yQX2RkG+WZGZ2fJTXBafJHAbKGQ3FwcGC6/UkXmHmcd+c8/j1s6XkEMjmpY7eodNqlAXhmNpokRSFHXFEp5p2TDFBKOY/7ZEio2GpiSAabGL1SF3wdJ/mgZKHov8GckMr51/v7Xo/Y5XJiBhsXNVg7bL112ptpvkNLqs8lA=="#;

const EYE: &'static str = r#"eJwliz0OgDAIha9C2BttO7gAN/ACbgZNNHEw1UFvX0oHvkfeD93re8DGOCeI6clhgjg6oCNkO3+a71hQaGg7IT2LXjsUxoygP2NMpp+rlXosFbyXGQo="#;

const FACEBOOK: &'static str = r#"eJwdjKEOgDAMRH+lmW+gtMsmxjQGi1+COIFAkAq+no2cuCdeXrnbAzrXsEumBawtUqR5jDu5bskN6hnmnKGXsIHtSE1Ifq8/C/QNtUwjVj9wLRT+"#;

const FACTORY: &'static str = r#"eJx1jbsKgDAMRX8ldA82QW2H2NnF1cGt4JDBwcH/xyj4RAnkdbj3ypwXhbFxHQP7bA38XrYp1fcHch8nDFAdo7woGm0fN/DgkhSbfZIzhAJQVPoi/EdekhVwBCwG"#;

const FAN: &'static str = r#"eJxtzzEOgDAIBdCrEHcQUNuaVG/gBdxMHBw6OBjPry2JDhoSFl6AH/fl2GAdqkmYgnoQR43vF0ccFKzzXYKBnAT0xKypo1YUhNruC6EQ4wmzgcJ/ZDHmExaE/zvtrPmUiem5GmOdA4zxjXH/pSexPLMLEfkw9A=="#;

const FAST_FORWARD: &'static str = r#"eJxdy0sKgDAUQ9GthG5AkuJAeHY7IkhfQSfuvh86KmQU7rHiz395RvE7f+8ZGMEDEtgWsWMcIdk2y2QrURdkF2pAS18BljccAw=="#;

const FEATHER: &'static str = r#"eJw1i8EKwzAMQ3/F+D4vNnWWQNIv2K67FzboYJQeemj79bUbikBC4qnMwzLCp+JLAkkHLOZDhAjBdUvU5dOeChxI35zHRLpjX+5+7cv/N31h5YocEVapKAib1WThTRx16EJtzNgeD9IGszba8qIPOcolnw=="#;

const FERRIS_WHEEL: &'static str = r#"eJx1j9EKwjAMRX/l0vfWJnU2hXZ/sI8YVZigIENE/952g+2lewrk5NwkMd/n/Lgh/5IiVsjftc5JserjacV9fI3vCdekBmLw51xRbe3geTEC6rQzHbiB2RoPf4SHagc44+CPZFps3QoPYIbTAgdphEvBUwtQOVmMH0PZbUGwurxnt8E/RKpL1Q=="#;

const FIGMA: &'static str = r#"eJx9kLsKgDAMRX8luPtIJFahCm5dXN0LDh0dxMGvN3WwSFsJpSXNOVyid3s42MZiYeCK57Zi8KeRQujlRQbpVKaPPh/gKiZde8ekXxMSkJM5G+ZRqgElqpUyCFJEKLk/nVI6KZwBh2R47Hz6KMuPh5KeIb8ETwTXDVYDTAY="#;

const FILE_ARCHIVE: &'static str = r#"eJx1Tj0PgjAU/CuX7n22L22tSWFx1ZXdVBNICBAkRP69D9GExeVuuM803KYa90JdHZgrl40mT6wtKGhL7uzBxPAkjACuI/kLGxzJV2yyAXktutUUIHZNboXTFqZQa1ZlOqwjZRr6dmmb7oGhb7rpWSgrmxCIkMb4MX4tZcrNmNsH8lIoNgqjkEJ+Scisxk2Wzt99K5+qsBvbCZZnbf9Icd5dfAMUmkYZ"#;

const FILE_AUDIO_2: &'static str = r#"eJxNzcEKgzAQBNBfGXLXmjVWC+q5h/bau6SCQtBgJdS/7yZNqwSSwLydrW23Dng24q5ANEjVEQhZOJTQo0yLm1RpAbqe9yjhnyPR1ic/3tZ2NpsZpx52Hqf11QjJbeCrAmWoAoyE8W8ji9IleaegQq1knrl8r9Xjok0PvXFhKbDwI6Dfjbh4803/KqbRepUf1ActjT3S"#;

const FILE_AUDIO: &'static str = r#"eJx1j8EKwjAQRH9l6X3XJF1jC20vXjzo1YO3UoUWSluwBPx7J0WlChJYspk3zKSY6rmla5mc7E625Fwr2wYXQ5bFkRVl8XGoeJYcA09noEerkT/4hiPOlsRFTfxeYUxJKYWgpCFNqmITc6piGvtH3w03msZumO9lYpWQopSRM5Qt4AsB/KlmoAa2tQOMZjhKJqx2xn5Zxbyd/sdo+Nto6I8RpfLAae3JA1oy4Fh95QljR0sW"#;

const FILE_AXIS_3_D: &'static str = r#"eJxNTk0LwjAM/SuP3jub0GkO3c4e9Op9oNDB3AqOgf56s09KIAnvixdSM0Y8K3MnX5Tg67lhMNw8Vr+JMkAvR+IcsPy4FOVtNf9MHU5zYB3S0H27tn8hDW0/fipDXk26BOwgi3CTqHjvICA3SZQsZ6PeSgm89Qf1B6d4MEc="#;

const FILE_BADGE_2: &'static str = r#"eJxljLEOgzAQQ3/FYr+0viPpkjJ3aNcO3SJ1YKnUATHw9RxEAiR0w9mW/fK/DD2+9+bFNkToIxWF4rqcuBp5CPxrTz0Gou9biM86npouXxZgl3esglYM5m2u1ITq6qXPefRjC6oTiRiimFDMNcXd1p4BOzQqBA=="#;

const FILE_BADGE: &'static str = r#"eJxNjb0KwzAMhF9FeJdry7+Dkydo1wzdDC04YBJDs/TtqyRNGw6EhL67Sy0vBR6duFkIg80EBIqlgZBKlO5KCoJ0A6n/D3kr6EWfLqu9T22u7zpOT2jzOC2vTmjLLI8IbI8b+EUYPhod6JANmK2Og9HDfu3y91PB4QmgvXRrLlU0qNGwV6OT7gd/ALbOM/I="#;

const FILE_BAR_CHART_2: &'static str = r#"eJxtjr0KhTAMhV8ldK/X5GrtUJ0ddHUXFBREC4qgT2+sP3SQQBLynXOIsfXSQZOKEqMgBspVTUAQniV5W9E78KQOyT9IqpIgLi7zLjLzOwMzY6dhG/qxBTv14zKnAiM2cdNAIWgnvCUsfn9giV6l8nIepB3BD4LKof+LDus9N5k="#;

const FILE_BAR_CHART: &'static str = r#"eJxtjr0KhTAMhV8ldK/XhNrboTo76OouKCiIFhRBn95Yf+gggSTkO+cQ6+qlgyYVJaooAcp1TUAQnyV5WzE48KQOKTxIqv5RUlzmXWT2dwZm1k3DNvRjC27qx2VOBSo2cTNAMRgvvCUsfn9giVmlCnIeZDyhD4LaI/2iA+sCN5s="#;

const FILE_BOX: &'static str = r#"eJx9j8GKAjEMhl8lzD3ZNum0FUbPHtbrHvY2jIJC1QFlwLc3nXbQixJIQv4vf0g39vcj7NfNzjpqgXlrY8/AYOZg5L9A7W8Rt/4loXaTazbdT3bYdOM1PdLpcoDxerrcb+vGOkU1RWADcQYrovBylGkVwApZHpA8icd5QIZrE9wkxHEwQIFJ8kjiIiXREmUgLyQr7d1cmYwHkwSriOqrK5htS1VXLLaotqrnoURc1IRlubfllKb6dLHGcvj/7f360VnxgE49lI3tBwC+ALsMTC/hCXGWYyA="#;

const FILE_CHECK_2: &'static str = r#"eJxFjbsKgDAMRX/l0r1qY3wM1dlBV3dBQcFHQRH8e6MUyoUQyMm51g3XjLFSHYNoNjwQCMkf0tQXUdYajjJQk4eTlu1mVdv4e6+tO9ZnXfYJ7lj266yUERtklKAE5Q96RGDfuKUw4pWwDq4XV/slhw=="#;

const FILE_CHECK: &'static str = r#"eJxNjs0KhDAMhF9l6L2uCe2uC9XzHtard0FBwZ+CIujTm6qIBJIwfDOJ8+XcoEpVTiay4N+7ZDDiUFq2hR6CTG6In4Lm4hPZ/2neVOZeITBzfuzWrh1q+LEd5ilVZMQkLQHHSA7wQgS+fui/IBuOwGhzZ+2HsSkT"#;

const FILE_CLOCK: &'static str = r#"eJw9jr0OgzAMhF/Fyh4XG5OClLB06dCu7FWKBBICRFFV3r4O/Vlu8H2+Oz/f1g7uwVzJAXPHEQvIgCwyEIpFl0TQWaxU9NQcsbiQKMZnF23CLQFy8tCdRB9zEMjVEJBnbmp/SCW1n6dhG/qxhXnqx/URDAloi0AJnEG5g1+k9rFf4tBC3BRzBpZgVOMrmJ37uBr5W19pHekyzSKHXDQk/+I3cS85TA=="#;

const FILE_CODE_2: &'static str = r#"eJxNjc0KgzAQhF9lyD3WHZM2hei5h/bau9CCgtVApdC3Nwb/GFgWvm9nfajHBq9SPQzIRkxNEHkKNZ+XzN7FZBa8nXek4/YzqvKn+bzyYej+Xdu/EYa2H7+lktiGOByYwyVxUaK8fPxcIQ6FTjl0rdhCqAukbHgCbGwtqg=="#;

const FILE_CODE: &'static str = r#"eJxNTssKg0AM/JVh79pNulYPq+ce2mvvQgsKVhcUQb/eREWWgSQk84gP9dTgW5o3uTQDPx81g2EViUwzRQvp3BDHi4Q/eZq9DvFqKn9Tw8qHoVu6tv8hDG0/jaUhJyIpBdii2IknRcjnD3+yoLvGKiKz6+5AuYYqrvsGYW0xgA=="#;

const FILE_COG: &'static str = r#"eJx1z80KwjAQBOBXGXLP2t38tdD07EGv3ksUKihIEdG3N1HRHiK5JHyTyaZPxzmdDkj3qLxCekTFRmGOyqihX7116C/jdcI+qnNHAWzJauo0vSKFFgFDAuaifwKWPNhTIJMjFQ8f1/kRrt5vwU25XXchUybMJTlR8bb8gEtJ3QNZdKXe1MbbsoXc/OQrZOHI7ewoEDR5MUTL1JLbSIOQSZqf6bxb++VZM7lv6xNedWi3"#;

const FILE_DIFF: &'static str = r#"eJxtjDEOgCAUQ6/yww7yq0BMkNlBV3YSB0YH4+DphTjIQDq06Uvrz3RlOhax86QMYbUJBNJVsqSbm6I4MqMtJKJTZvvGjwh+qIfB/7cgHqPrkJlYZ9sHrgEvUZUo/Q=="#;

const FILE_DIGIT: &'static str = r#"eJx1jbEOwjAMRH/F8t5Sn9zAEHdmgJW9ohXuhqqowN+TwFAGKg+27vnu4jxeE81PYzC9jCUvH6ebJ+PA9NUf05DcWLmLu/LfxXufnAbjsyhhCR4KKuIPygQu2oNAzWdQ4bKv25No3RKOYUVVvhb9EyINCTxXbLCDr643+Cs2Rg=="#;

const FILE_DOWN: &'static str = r#"eJxNTk0LgzAM/SuP3utMtK6D6nmH7bq7sIGC04Ii6K83VZHyIAnvizhfTw2+pXpTnhjws6gZjDRAyzVTRMjmhjgmNH/uiXkd4VVV7hYKK+eHbuna/gc/tP00lopyCcmw4BR2N54WMV8/iMXOuoh6Tun/ABlkATq75A1ZmTCn"#;

const FILE_EDIT: &'static str = r#"eJxFjt0KwyAMhV8l9N5MY5wKtk/Q3e5id8IGFqQV1pu9/WLZDyEhJB/nnNTyXuA+DhcGY9FdORMQaCkDpKgEdDNp8PIi/f8p2Ypy6IYpnbrGlNpWX3VZH9C2Zd2f42BYaBkBRCAc4AcR+GtrNLJQhGeTCcVTunv3LfpjzB6jAzIgelQxRmX7wSFbScB8+2V4A1iIMYk="#;

const FILE_HEART: &'static str = r#"eJxtj7tuwzAMRX+F8M5b8SFZApzMHdI1QzcjLZAARmKgWfr3oYO8hkDgFSWeS4nDPJ739LPqvpzK1kclpRRLSFn3FXmjiXrkraZnjSP79G49fCzu9TCfpv/pcPyl+XQ4nv9WnXigIZXCXa/gDQn4/qAkaKPQflS40VWW/okVpTCy7jgQiEZeYIw+PmQTwzLMly3C37mjkC3c9mpuccdoGc055osTlAy9bwoySZ0MhQ057wTKEUIC8ehXIY0X8vsx9AVCNUbF"#;

const FILE_IMAGE: &'static str = r#"eJxNTtEKgzAM/JXQd7sm0+mg9nkP7nXv0gkWOi1OxtzXL1WREkiOy91xOrRzD89a3DGXBdDt0hIQqDgZow8mBF/qkVIio0cpi2Yz/4TRpxhodBj94t3QQRjdML9rgTmbeFVACqpVuEuMtm6yvgP7ZZkSYBe+ZwFTLSgKtzdn7lVfnIBlhlJd15U2lhXDBrkZHWX+QOs5Jg=="#;

const FILE_INPUT: &'static str = r#"eJxNjcsKwjAQRX/lkn1qZkw0Qtq1C926LyikUNuARejfdxr64sIwcM7cCakeIt6lelowR7I1g2FyWPPrWrgH2cKB75cdadn+VlXhNJ9XIfXt2DbdB6lvuuFXKpI2yPBgA5/FRRF5/SiGi2QONQv53kAeZ52z4Qmf1yz6"#;

const FILE_JSON_2: &'static str = r#"eJxVjUELgzAMhf/Kw3tdE+rWQ/W8w3bdvbCBgmhhUti/X1tqrQRCXt7LF+PsNuLdN08F5pGUZTBkKhb8urXdg1Tbge/XwxJh8qoZzCWeD8at82+elg/cOi3bt28o0BCaBkvoFMyRED4+ElsCZWaYPBVNUaMonN20qf7vSA3SdUiQFxWzULN5tmVcFOgfdmdBJA=="#;

const FILE_JSON: &'static str = r#"eJxVT8sKgzAQ/JUh99jsom0O0XMP7bV3oQUF0UAl0H59NzbESCDZeewwcb5fBzxbdae6asDXc89gmHi0TIEKQl4eiEtC8+NSNbf/8ld17hQDO+eX6TON8wt+Gef13SqqZUkuCzawmzFZxJw7GEg8gVIBmQJlTBEjIxzVjSkK7P8C2dKlKegiNMcm8SibSOTUHwvTRQQ="#;

const FILE_KEY_2: &'static str = r#"eJxNTssOwiAQ/JUNd5DdgGICnD3otfcGm5QEW1J7sH/vVusjm30kM7MzvrZzD9cgLgZQN6YlINBcCCSpd8qeScNB2Yb0D5N8nYyIfrfKo69jWUoeOqhjHuZ7EGiYysMBq92LuFGiT3lKpYMpCBKQHkEYXgtr9ivvjfLLLdcN2U9Loyxw/1l+8CMgclr8Qk+HTDfb"#;

const FILE_KEY: &'static str = r#"eJxNTbsKwzAM/JVDu1NL1O5ie+7Qrt2DW0ihhRBCSPL1kQl5cAhJd9JdaOu+wTvSk6+Vg9x9LRDYAqPTwCdCuzQsZ8LI61a5x/o8UwqXYphC/nb590GeIrEndJGEkEfdbDla5RS29D97sDXFReuwOXQHZih2bQEuHi49"#;

const FILE_LINE_CHART: &'static str = r#"eJxNjrkKhkAMhF9l2H79Tby2WK3/Qlt7QUHBY0ER9OmNByIhB8PMR6yrlhZ1qgoKvQj8jysGwz9Ly7XSR5DNLfFX0FwmXpTf4V1l9ncCM+umfuu7sYGbunGZU0WhhGQYsA9zGR+LmJ8fBopBgQ6EJa0FnhtQ8lIPJsUrKQ=="#;

const FILE_LOCK_2: &'static str = r#"eJxFjsEOgjAMhl/lT+/oVra4w8bZg165EyFuCQGCC8rbWwxqmjRt+n3p76cmR7SBrga2Ng2DoaQ0uODoDvbCCqeDrVn9b4VMZ0OVP2525aexX/s0dJjGNORHIG0EleYgtvuAO1L5ubtlPFObYyBHiF26xxzIEuaXmIRVekmQhTdz4+XFN6aDLpeC9zAaqjBQC//CvAFAITcS"#;

const FILE_LOCK: &'static str = r#"eJxNjr0OwjAMhF/l5D2ldklgSDIzwMoe0Yp0Q1UUfp4eBySoPPj0yec7f0slYwx04m1nIQeXBIK+jVFVeQV0S2ZZAyPnXWePX/OLot+0h9Ev06UgT/M1l0CO8Ai0J9znseSPegZiISzKubnaffT/MhYs1UgaMGgMaxmHvsov4A3mOC13"#;

const FILE_MINUS_2: &'static str = r#"eJxFjbsKgDAUQ38ldPfRa6sO1dlBV3dBoYK0BYvg31uLDy5cAjlJlJu8xtywQYBIczERCHk8SmisUtlzkUpQV/5WEtQhWKuyO94qZ7dzW80CZ1fj94bx0IbwalCOOoIPEuB3sQCXuvxaLoHNJKw="#;

const FILE_MINUS: &'static str = r#"eJxNjMsKgzAQRX/lMnutM5jWQpJ1F+22e6EFBdFApRi/3jFxIQPzOJw7NrRzh4+jF9elgTyurUBQ7VXo9ucT0CkdyxkU8r6V5pnDK3l72R96G6YhDv34RZj6cf454lpD2hpIhSaJh+JtEhdRyRAWdnQnRM5nzFj95G6teSwW"#;

const FILE_OUTPUT: &'static str = r#"eJxNjcEKwjAQRH9lyD01OybaQ9qzB716LyikUNuARfDvXUO1ZWBZeG9nY+7mhFtjLh5kEt8RhCuh5fVYhbP4KoCnw4qsbi9v2rj7nrcxT8N76Mc78tSP87Mxom3QUYMOdREXReXfRzVCErepWcgjQGj3KPnjD535LNY="#;

const FILE_PIE_CHART: &'static str = r#"eJxNTsEKwjAM/ZXH7otN1nYVtoE3D3rdwdtAoYOxFRyCf29bdUrIS0je470mDKvHtS3ObCHiZRAIVC4ppa/JnFiTgRzt71XG7VEVXbNL8q4Jy/ScxvmGsIzzem8L1pEawUEUXCZ+KJH8ddSkNJip5sGQ08igwLEdSer9n8WWsyJXge3BpJnhHdeBFXHds/Xpetm0L1GHN+c="#;

const FILE_PLUS_2: &'static str = r#"eJxNjbEKhDAQRH9lSK/nziU5i2h9xV1rLyhEEA0ogn9vFEVZWBbe2xkX6tmjKdRfg/Sia4LIjmHC6pOan+jUgF97oyRei1ale+3vpQtjv/bd0CKM3TBPhZKYhrhyMEN+iKcS5avxDTHePlIuYCFcbrABdJsr4A=="#;

const FILE_PLUS: &'static str = r#"eJxNjMEKwjAQRH9lyb21uzQaIc3Zg169FxRaKG3AIqlf7ySpEgJ5y+ybtb5fB3p06sZtrUkux15IqImvwvTmIgBlYCmDSu6nWl9z+aOcPcSDzvpl2qZxfpJfxnl9dYpblPAZkoZMEnfF2SQGgSSKAmdukQbMOQqFnJZ6X4LhR+Tnv/sFbKM3Rw=="#;

const FILE_QUESTION: &'static str = r#"eJxtTjEOwkAM+0rUPeaS67UgHZ0ZYGU/leFGBsTA6zmrUtuhiqxYtmMlv8unyuvaPaxHEr8NxcUlcLSxr+2Etr2a7wX154h0X45/3ZRPLJzyVhvEAuIMV7SQ4oyLWnGYECwhG9DPiEy0QBJDpIHIH4ijZhcbK4Kt3h9K1C5J"#;

const FILE_SCAN: &'static str = r#"eJx1z8EKwjAQBNBfWXJPzC5prJD27EGv3osKLZQ2YBH8eydF2xxaAiEhj5lNiM3U0qNSV7HE9nY0xYWdKUjOvhESsmlpnN7s7yCGzYnSg7Rgqg6HlFCHOPafvhueFMdumF6VYgeErSQklzP8EeB/KXsSWXpYi5YsMhstU7jsKXZrVhpxQ6ExU/PPFvUFnStDKg=="#;

const FILE_SEARCH_2: &'static str = r#"eJxNjsEKgzAMhl8l9G5nMut6aHveYbvuLp1goWhxMuae3lRFJCH5CV/+xKRm6uBtxRMrqYDudUNAUOYoWH3xNOBOHdJ5UNDrJtVjW/4LZy7Z0Jk0xDmGvoU0hH76WIEVL3HRQCXoFdwRZ3wYfWzBzxmTSoD/scKsRiuIO+MbxM7Hw1dJCrBeK6c+ri/2nTY0"#;

const FILE_SEARCH: &'static str = r#"eJxNTckKwjAQ/ZVH7omdydIIac8e9Oo9oNBCbQMWQb/eNK0LjxlmeFtIce5wacTJgLkjExmMqoAln2tlj2SUBR/cj5L5emjRht1ib0OahufQj1ekqR/neyMopyEvD67gi3CTZPGn0YLqqKFzIpVUh/Vb4V5/BZvntgd5Scou86XfA0AxhQ=="#;

const FILE_SIGNATURE: &'static str = r#"eJxtjLEOAjEIhl+F3F48KPQgqTc7nKuDWxOHDg4OxueXOpwOhvDn/yD56qM9O9yO05lnIEd9oTYGhjmGUrRT+eXEF/kyBHdD3chAUae1HoZvrbvVgKzTnwcZCoNjocYYotghHM2XT2xU0BVoAcphuaN7yuMiKDlFyHX3vgHedC2Q"#;

const FILE_SPREADSHEET: &'static str = r#"eJx1jr0KhEAMhF8lbK9ncnpusVpfoa29oLCC6IIi6NOb9QdSKIEkZOYbYlw9W2gyVWIcJkD/X01AEPkKeFtQHHiSRZKHgKo0TIoT3lRuPj4wN27s174bWnBjN8xTpjBmiJsGikAfxsvC5vsHDfi1JFKEkD4KnPiCeEUyOwoFPgI="#;

const FILE_STACK: &'static str = r#"eJyFjj0OwzAIRq+CskMNMTaV3MxZeojKHTx06FB56OmL+5MpUoUQ6Ht6gnK/PBpcT9OZE0jXptNSDiNcyoaEIfVEWgOQIWVgUvQes2GuSAZhBM7wyzoea3DwSX7GyvkWIT53jmSwbmTjxkxCiaLL4qXvLa6899oMLH803rQX7z85Gw=="#;

const FILE_SYMLINK: &'static str = r#"eJxNjr0KhDAQhF9lSB8vuxc1RbS+4mztBYUI/gQUwbc3BlEZWBb225mxvlkd2kJUGsyOdMNgqCiWXOdJ+iedpOBf9pxk2LZclPZzvpfWz8M+9FMHP/fTuhSCghvCMGAFE8ELCfCVOJICGXxl1MvsaURmk3TH0tnIZTd5AJnKMFo="#;

const FILE_TERMINAL: &'static str = r#"eJxNTrsKgDAM/JXQ3UdC1Q7V2UFXd0Ghgo+CIujXm6qUcpCEy91x2vaHgaEULco4A6rznoAgdYj4OjEgeJNBComIuiLOms98i0onLrDSdpuveVpHsNu0HnspULKJhwJKQb3CX8Liv8OiAHOX6RBk+YqcoIz0nwcasDCA"#;

const FILE_TEXT: &'static str = r#"eJyFzMEKwjAMBuBXCblvNnFbK7Q9e9Cr94HCBmMrOGTz6U3bCcOLFPqH5EtsaOcO7g6vVJU18LlpGRhUfIVUL9o1JLkj3jcKvumyvuTlN3p7iAe9DdOwDv34gDD14/x0SJUsyWeAFZgEN+Jtggs7NAgLiW0Q1phHSU4p/o/Vm9W/Ng5VRqdsJPKFr/wALsdByg=="#;

const FILE_TYPE_2: &'static str = r#"eJxtjsEKwjAQRH9lyD21OyYxh7RnD3r1XlBIobQBS8C/Nxa1PZSFZeG9nd2Qujni3qirARnFdARRL0XN26myFzGVBc9uRbpM2ag2HD7rbUjT8Br68YE09eP8bJSUNJTmwRp+Eb9KkX8Xi3HMWqLLsola/xEfuQMshNn9wRsecTR9"#;

const FILE_TYPE: &'static str = r#"eJxtTssKwjAQ/JUl99Tu2MYIac8e9Oq9oJBCaQOWgH692yc5lIXdZV6MC83o6VWpBxdZSbiZBgTKp9HyRU4AufCMFNB4XrLyvph/qnanKbB2Yei+Xdu/KQxtP34qxYWYZFlCTnYWrhIRbx2uxOeo2ZvISdTekImtxxEj0YhmZ/5ndDhm"#;

const FILE_UP: &'static str = r#"eJxNTssKhDAM/JWh97omWtdD9byH3eveBQUFHwVF0K83VZEykIR5EeuqpUVdqB+lkQF/sorBiD20XCsFhGxuiUNC8/8dme8V3lVpX76wtG7qt74bG7ipG5e5UJRKSEYOjpGfxtsi5ucHsfCaBTW3MpABGZ14IHn0A1zwMLs="#;

const FILE_VIDEO_2: &'static str = r#"eJxFjcEKgzAQRH9l2HtSsyRtDsZzD+3Vu1RpAlbFhrb+fVcRZGF3YN7MllOTI9pAdwtf24bBKGQMWHH02t24wEW7movDU6KulqrytKarchr7pU9Dh2lMQ34HMlZQWR6S9hu4IwLvD19Gmpx2EFa7jzr3alNH7dw9MhZpY0Ls0jPmQGfCN7U5BvKEXyCxZjlmTa189QftxDcj"#;

const FILE_VIDEO: &'static str = r#"eJxNjrkKhEAQRH+lmHzc6V7HNRiNN9hNDcwEBQWPAUXQr7c9EGn6oHhVtPPFVKNM1J/CwIK/UcFgmL20XDM9BNlcEz8FzdknsL/TvKrUvfbA1PmhXdqmr+CHpp/GRFEoJhkx2CA+wAsR+PqhIwMiWLy19Kyj/A7cAFeUKm4="#;

const FILE_VOLUME_2: &'static str = r#"eJxtjkELgzAMhf9K8N7MpFY7UM87bNfdxQ0URAVlsP36PecoPUihSV6+99pybtaOHlVyk4wd6SVvlJTS7Rh0L4kEVO1EY8HovWB33c2fpC5PW2BdztPwHvrxSfPUj+tSJZLBhMuTpuR/4B8BHP4giBHLrmWrjEw+b8FQF8Pi0YjZ5+ilYMZCW84zBgjGFygWRpuTshoMBy6wruNUwuoLppFDOQ=="#;

const FILE_VOLUME: &'static str = r#"eJxNjsEKg0AMRH9l8L6pya66B/XcQ3vtoTdpCwpbXVAW+vdNxbYSCBMy85I6dkuPe5OdHUR6dp1AkK8lRi4VFSd2VECO5X9lVCWbtfXhE2/rOIVXGMYH4jSMy9xkrDRo85AcfjVuFjVvF58VODdWyZJcL0FVMv66o35fYwbzjUpHHirJaxJ2NmRLCInR4Zd6A985NVM="#;

const FILE_WARNING: &'static str = r#"eJxtjLEKgCAYhF/lx13zPzQJzLmh1nahwbEhHHr6lKAc5IY77rjPn/FKdMxiY6MsYRkjCKSrZEmZm6I4EqMtJHan7PqebxH8UIHB/1jQlE1/YJeU5m97AO/7Iks="#;

const FILE_X_2: &'static str = r#"eJxtzLEKgDAMRdFfCe6t9plWh9rZQVf3gkMXwUH6/cYOKihvCRxy/R6PROtQzUxAMhxBoKYMCkun7WRYW8LoHlJyZa6Cr6/34O+IkUp2yX1pa8lAOrIf7AuqN55zTScl"#;

const FILE_X: &'static str = r#"eJxNjMEKgzAMhl8l5G5ngp0O2p532K67CxsoiBYmQ/f0SzqVUmg+kv/7XWznDp4e71QZC3w9twwMpb5C6EPZQiZ3xPmi4Edt7O0vfzG4kxYGF6dhHfrxBXHqx/ntkSqR5GuAS2hScIsEl4ILebwYi7CwppVWpTqRHImFxMsd3pzjnjlamHp25wehxjo0"#;

const FILE: &'static str = r#"eJxNTM0KgCAYe5UP7/340Y8H9dyhrt2FAgVRIQnq6dPqIINtjG08qKhhE2ShXd0DToNCQGgzquROWgRJUVMsgwrXse7nb3wTyZt8KHnw9rLG7RC8cfEQhHZplIgBtsDe4l+RDwRlIQw="#;

const FILES: &'static str = r#"eJyNjqEOwzAMRH/FKvetTuokk7LigY2OTx0IHJgK9vW11aqooMSyzud3V7/vX6PPrXuKQincC9LEGKhnFAQWCJQRERmKbINMmiWgTD1hgC2r5h7kbdd2NYNjCIURXHRAdJTxzOfoV4I+1uB/N9aLlxnrXilSRjqXdfAtRp216X5aALdbNbI="#;

const FILM: &'static str = r#"eJx1jUsOgCAMRK/S9AAaQMUFcAMPYYRYdoYQP7cX4sYYu2mavr4Zk8KSIZ0WJcIRfSaLYkSgEFfKz16gQrjqdKatgjPbnAm8xUmD2stTAfX0Agp001P3S4QkxhEDIwm2qBDNS5/AGwhUQ6Q="#;

const FILTER_X: &'static str = r#"eJxty6EKgDAQBuBX+bHv3N3tdIO5bLHaBYNhgkF8fjEIBvPHl4/l3LAOzcRKnhU6So1IFLqZUw2Qy0WyUCk5Jm/WlNw+p+R37iJQZ/gj7qH40g1QqR0m"#;

const FILTER: &'static str = r#"eJwti0EKwCAQA78SfEDb3UqhoH5HBHEFvfh7WfWQEMKMq5JHlIIqqfTmDTNeaOgB8WW/NX6QBZP2PhUzwd1HDxOwCBH0"#;

const FINGERPRINT: &'static str = r#"eJxtkDtuAzEMRK9CuOdEpERKC2zcuM4hFpvCZYrcHx7ZgN0I+9FvRvPI/e/4v8vv9+XHxfzmkojnN5fihxXhOx+TIe1y3b+m47q/fSG2IW5Bjw1JsZg/P4uiw1wNtaM29YXXOnwTNxQ/pxaJVtVREVq5ubIQrBxEe0IV5ewkHMUKE0eYwnNJOpCsi0lOTTIqJhwtfQ3XxCpmJby2DqYlB2UfMMZCT7C8o9jqyDB4ymTywqppoEYjfi7UG4PGkWzjq++bBCYFWp9rWFeWKx/mBwdGaeE="#;

const FISH_OFF: &'static str = r#"eJxtUUFqAzEM/IrI3aplWZYMaWDJpZc8YkkPOWShh5L3d7y7LKWUhVlbYkYz8vlr/n7Q5/vpJkFSuPqLsy45sb1wWRLAyDhbmxp7rbRhxickRhL3pGyNcnL8UmHTFGypEWuNJFwgzCIVrapKhaUIKBqLcq2jnCOm4MiFNjzE232TxtDcwLRKTi2x5j40a2KPURd3DKqipHAap8v5beS6nI90TpK5+dUpkMYGdAfTdVzvCePgWciIi2KcDeN1K46Dpb1+XdkS2BK8SBuAMLo4I21l9TJ1dqcV9iRtBPAFZ+Fo858+JngJrKTJP9YXARcazl1XEwg6FTq2pBxU5KOzzcjUaIW8dbl2yPa44UGwm4dx8fkXFy1A8+eue0PvWTKVfPj4AWW/cBc="#;

const FISH_SYMBOL: &'static str =
    r#"eJyzKUgsyVBIsVXyNVIwNCu21DU0VTAy0DVxNjRUMDJWMFKwAGElOxt9kEI7ABKqCyk="#;

const FISH: &'static str = r#"eJxtUUFqAzEM/IrI3VPLsiUb0kDJpZd+oLclPeTQQqGl7+94s4RClwUhS6OZkfb4uXxf5e3x8OJoouWCUZOhutSZuXQ0RkNzyeLILgWtSognAtioMfGVb9a+UhCZCLG0Tr4eTseHqXE63pW0U+gHba/looFhy0CErCHzU8lJFd13RkI0w+Mc0qWhzTCCJsPm85JUlEZUmqAYN2COUm/FmbS01c/rtHZk43p0wmAw27OZ54UCxc9MC4V6p1sNXo2UXezK0lKkbPYVg9S0+T7FFPU/6YfyunpbP22opz8Mhi5Fnwfawg1d1pBvXdTBnzD6nfUXOYFnOA=="#;

const FLAG_OFF: &'static str = r#"eJxtjDEKgDAMRa/y6S6aWMGh7Q1c3UUFBRGhIvX2pkp1cQgvJO9/s3X7hMGqpgb3JQpUYMjudUaQOYiUM3nUnHllDeZW/z6o8vREU9WnLfM6IpBVrBBYIDwT77OoUXIXlHAmig=="#;

const FLAG_TRIANGLE_LEFT: &'static str =
    r#"eJwBIwDc/zxwYXRoIGQ9Ik0xNyAyMlYyTDcgN2wxMCA1Ij48L3BhdGg+p+oJQA=="#;

const FLAG_TRIANGLE_RIGHT: &'static str = r#"eJyzKUgsyVBIsVXyNVcwMgozyjE0UDDVBRFKdjb6IEk7AKZ7CRg="#;

const FLAG: &'static str = r#"eJwljDEKgDAMRa8SsgdJTKBD2xu4ugsKCiJCHaqnN6XD4+eHx4/38uywJpwU2AoTgzoGAsHR3uex+MHkDhkJBUd7/zDHoY3keB7XBlUSKsLrwebJCUUQKre3q03KPzqmG4Q="#;

const FLAME: &'static str = r#"eJxVjjsKw0AMRK8yuJey+u3a4BhygBxiSYqUKVLl9NEuboIQvEHoMfu7f154Xpf7ygFxjpsmjC1zRCD6KCRsK3GQkpBlKk1JWdyIVZ2cSziUKs5vhfMGR50sXGGwZENw9IY23BASR5l6CWM3S6lungfr/0XO9F2O/TJKHz+k7yUw"#;

const FLASHLIGHT_OFF: &'static str = r#"eJxVjEsKwzAMRK8itDf1CON2YecG3XYf0kIKpXRRjHP7SHE+BCFGIz1N+vX/kZ6Z74iEWEIvJOS14HQa3Wnh5AE/eBuWCtyliyV0ac+56hdQwuBJLEI7FBzg5/19UUVmgGmSzFEFi1R1uBlq0IraTRopqrXZ2uyGzry9MNA="#;

const FLASHLIGHT: &'static str = r#"eJxNjD0KgDAMha8SsheTUMSh9Qau7qJCBREHkerpbVoFeYT3w0fcPhwBJo8dN1CPBGIknz2ZhhSBkljHYOx/MNIzjaQhy/YSWG5sXaU/W7cu2wxRPHKDcLHHGiEWu0Qtkcr8SSlk9rfH0j/4AV7yKlw="#;

const FLASK_CONICAL_OFF: &'static str = r#"eJx1jOEKwjAMhF8l9P9ic2sbB13fwIcYKFQY4g+R7e1N3Q8VJgkJd99x+T49Kp1HdxJPtoEVBM8xTkJC/j08kHCIVcCafvyu+bNdKDpwDHAlH1pnyd/NeIL70O/BYDD9gUeOhKo7RElSHT5gvt4utMro4GiFPfvLJpdNWrSFyguCjTyp"#;

const FLASK_CONICAL: &'static str = r#"eJxtjrEKwzAMRH9FZLeqE5YVgxvo3q7dDR08dOhQ+v21MxgCQYt4D+6ufOq30eu6PCCkP2dTr0pK0g+BFeA1p3tk71TYrIKwWyHOBI7WoOzpwMPg72Asqf/CUL/NVEKkPIqeumzlMhZsZe5Y2UibnxgnpAaZ5g8VAStI"#;

const FLASK_ROUND: &'static str = r#"eJx1iysOgDAQRK8ywbN0tz82Kb0BFt8EUYkgnB9qwBQxZt576Shnxb4MKxvIFcnykNPU3pw+5qBkNybVDp3JQ2r87Up4hDYDBo8OpqN68gIOlYU0vPwGYNMp/g=="#;

const FLIP_HORIZONTAL_2: &'static str = r#"eJx1yiEOgDAQRNGrTOoJ7ECziKU3wOJJEBgSBOH8UNNUdMVX79u9PyeOJVwjFBGx+9s0JOuzJCtOgWaEM6xCcHjZFpk8mT1gBR/RPC4Y"#;

const FLIP_HORIZONTAL: &'static str = r#"eJx1zDEOgCAMQNGrNO4gLWgwQWYXD0F06OhgOL80JugA6dL05Tdc6WY412H3YLcpERAYGVW2jO4wgBr1AgLEdohhlCSGGuIMlm0tEd7yO8grVs20iMnUFnQ98T2gHzz3SDhV"#;

const FLIP_VERTICAL_2: &'static str = r#"eJxtyzEKgDAMheGrhO7B5mnVIXZ28RCCQxbBwftj61RKeOP3P33O1+jawi0LjZwocZlJDFmHalnbAsJ/QH5yTCTY4YDEKqsncxFj7wR09AHzYC89"#;

const FLIP_VERTICAL: &'static str = r#"eJxtjLEOgCAQQ3/lwk7kihIGZGZxdSdxYHQwfL/eYtBcOjWvr+msV6NjNRuY4r5UEMhJLCzKpxO6NzlNouQ0ihy6f5csy8FkeepWU2diFCiAnZCokfCQZjUJ+KEbbso4VQ=="#;

const FLOWER_2: &'static str = r#"eJx1jtEOgjAMRX/lpu/FtXOAycYf8AO+kWmCiSSGGKJ/74YgvJg9dDk9va1/dM8el0CtKFxnYWEg6aXfwJbtDxnOKJFJ2hPqjWc1k14GB7NFrH7PMrDCTizU+ENe2Ph4G+P9ivgKJEqI70A1YQyk2fl2G78/TswkugXsW6rxWGjaWrEUZVml6ngmXGEmqbrz3+HV5cVlhyUPS95++gOXBkjI"#;

const FLOWER: &'static str = r#"eJx9T8EKwjAM/ZXQ++ay2tpCW/C2i9fdSxUULMjwoH9vYjvnEKSkJHkv7yXuFu9nOHpxwB52rYrbVgFHB0ivVhXcf4MddwD7io02NwzJuCbNCoU82MxJt7ZpapUpOB9Q5UZyZ0QlgtvwksGly5SuJ0gPL7AXMHkhBaTnuyJSgYObL8oGUAOSJMWishzMrpbX0mB+cRo3PA1/pstHJvrDeAGyw0+X"#;

const FOCUS: &'static str = r#"eJxtyrEKhDAQBNBfGbYPdztB0iTW11xrL1GIYCEiQf9exWYLmWIG5sU8rXkekfckSkE+nl6TeGnj57nbuPRbwZDk7xG6pieI7xUFHQtveQvjNMAXWgjWN0iFhmqku1ZxbzSA+musdKxGnp7/NHc="#;

const FOLD_HORIZONTAL: &'static str = r#"eJx1yTEKgDAMRuGr/HQPmtSKhdgbeAjBoYvgIJ5fzGCXlLe9T6/9rjjWsAlY6hyKDt8q2sCEPGKBPOLD0gOeeiKjJydnZIqwHE7ghEjWzy/WfDvl"#;

const FOLD_VERTICAL: &'static str = r#"eJx1y7EKgDAMBNBfCd2L5mpLhdjZxdVdcOgiOIjfr1nEIeW2e3dyblelfXILg4DbJ1ek07LIn/IKAwZizBZwr5ItSa9Ub52AFh0ciUcfNBRsj0o6+PgBbzg8QQ=="#;

const FOLDER_ARCHIVE: &'static str = r#"eJxVjkELwjAMhf9KyD1xSaVboS148+LV+6hChR1kSNF/b4tQN3IIL9/LS/xzfmW4Bbyogg7Xaa4NhlakpJlGdqbPhIStJXYL8aRV6KnboTnBnI/bBNAiJtVFFnbQgGaL0R/a2ejTY03LHdI7oFiENaAipE9Vrpl+OPr+o1gQKST/hB0aC2lHXxwnNNY="#;

const FOLDER_CHECK: &'static str = r#"eJxNjr0KhTAMhV8luCfXnJZeC73C3VxcHdxEhw4KDuLz2wr+kOnLlxNOWIct0vQrWksoo7oBBCrPAaOrHubEkb/izb1TVnGOxc8sFRLg/8TzJZnGvj8QdjVjCoqKpyzQF3X45BZ1uLosntSc1rK99QFHySWH"#;

const FOLDER_CLOCK: &'static str = r#"eJxNjkELwjAMhf9K6L0xyWa10Pa8i1fvIwoTepAhov/eqKMbgfCS95GXdB8fE1yyOx1AaOhHAQGyYi9eznslz8gYve29TB3GbkWAMQSMFY9iUppBizEIbWBTrqTdN7Akvc1ar6Dv7Dg4mLOzrq/fZNDfLql9xwG4f0q1zHbkA4tMLLA="#;

const FOLDER_CLOSED: &'static str = r#"eJxNjq0OhTAMhV+lwbd3PSO7LBkkOAwWgSMgJhAIwvOzIQap+np+csKxnJG2thprgonqFhDIPAfG1LzMiSP/xdvyU1ZxjsXvLA0SoH/j2Ul2qL8NhEvtmoKi4ikLmKsu/PKKLpQtIDURpig3lK4kzg=="#;

const FOLDER_COG: &'static str = r#"eJx9j8EKwjAQRH9lyT1jdtPGBtqee/HqvUShQgUpIvr3Ji3YHlLJJZN5mdmtw20K45XCp1FcKZoaZRWF96za+rDYbf3onwNdGnVig5LEdEUvJGTiYS1azuWqKerBwm9eGM7Dj6jSVX6GScYRvhOz/U7ysihTfapdy+/COBJ7FBpew2YILiHELtl7hIOjFAQbmRzgEaeMOX+AJUHHQThfUaUEuwsUsDTHcERyi5q06ByzBb7aPmpl"#;

const FOLDER_DOT: &'static str = r#"eJxNjjELwkAMhf9KyJ7Y5MrZg+uBm4urg1uJwgkdpIjovzenYCXD48t7LyTfpnuF84iHHrSrEicFhe4zSnocVibnSltO4bcTEo6ROM3Egzrobq23JIR9/38B9CHBvMjCCZqhJyx5074o2a6LzRew54iiCPZyDQiLSwt97fIGkcwojQ=="#;

const FOLDER_DOWN: &'static str = r#"eJxljrEOgzAMRH/FYrcbX2hKpBSpW5euHbqhdshQJAbE9+MwBCR00/n5TpemYc70uzevluCyhgEEcpvAeHe7Z/OZbxJ9vSmrhMAS/ywdzOCxx8sn+Wd7bCAs6r8WFJVIBeDT9OlSVvSpblGQuiWcyahXUs/Wy6bKV7zfLS0="#;

const FOLDER_EDIT: &'static str = r#"eJxNTbsOwjAM/BWre47YeVBLoTMDrAxsEQwZOjAgvh+nqqIOtk73LJ/6bfS+TPcZUYg9MlcBUz9PTB3peXu3BE3ESkLiV6i60ImEGJy9+JyWcup9SxmtVslIj/TyjsFQZ1knzYKhGrSJbQQ5Q1fMZocMwe/CVfzBbOjHB6JXNqdIY/4Pc4suCg=="#;

const FOLDER_GIT_2: &'static str = r#"eJxtTsEKwjAM/ZXQe2v6ZrWBtuddvHofVZiwgwwZ+ve2TGYPI5CXl5e8JDyH10i3qC5C4P44gEBcwmpoXN2fU+FjZ6TpWHMSI5PxtcQmcBXORnpwu05YnErhUC+mkB9znu6U31HZTlH+FISiOSrUoVVOYXvPerKSNYwn1q4gSnaL33UE/5xWX2kcvwFcOm8="#;

const FOLDER_GIT: &'static str = r#"eJxVTrsKAjEQ/JUlfdbb3ZC7QBKws7G1sDuiEOEKOUT0700UEo8thtl5MP4+PzJcgjoa4CGTnRkYhu+x5tPUuS486xGdtB9pQms1ukXjxIXwvserE+Rg/huAnySpBJHQQRX4rKLf1RXRp9ualiusQbGC9AqKKr4LSjX95OjbZDJAkqUXNGXcCh8b8TcG"#;

const FOLDER_HEART: &'static str = r#"eJx1T7tuwzAM/BUiO6/iUZQtwM2coV07dDPcIYOHDkW/v1RQOFkCCdLxcTze8r3+XOXr9fRuJiyXulIoJY8plR+xFTUYumZeeXV0v7eIoTX0HTMT8iiU/8KF5aE50a8hTuflZaiel0ObSe5ijmklqsvtuQ1SooUiuCkcxoQTXDEjfM9UzZtfDPiM63duG9ypZ07RA71qrpURKCle32xCCG13NHVEbLlY+udwajUHzrCuo/XzsPEHPFZDyw=="#;

const FOLDER_INPUT: &'static str = r#"eJxljjELg0AMhf9KcE+al2uPBq7OLl27Sx1uUHAQf7+niBzIW77kC+GluV8yDZ/ma+S/118ZAnE2MrYcxENfkLQEBIlRfJS3FbRL6Ck60+q40Ipqsb/snvXMtjKaNj32Dm2qmiBk6N1MTogU+MilN2B/LLw="#;

const FOLDER_KANBAN: &'static str = r#"eJxtjrEOwjAMRH/F6m4TXyKTSKESGwsrA1sFQwYGBtTvJ2ZIGSpPz89nXX0vn0bP03RNhNDUFhAo/AaMW96YOzc+Soljp6xixlJeLBkdcN7ifknxkv4/EFaNjx4UlUIucJ/mevAWcx1dMmlY045QuMGeMTc2zBdtjTNy"#;

const FOLDER_KEY: &'static str = r#"eJxNjkELwjAMhf9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEBLyPfJeuk33GU7ZHJhAaHSTgABpsRUrR1/JMjJGq3cr84Bx6BJgDAHjgnvRVTZAXzAK/Yl1e4gpadc8S6qXtS5nqK9shAysOgzUZzYcmuiDS/oFvIp6OOvQg3Z/07nG8S3Txt7H+TT1"#;

const FOLDER_LOCK: &'static str = r#"eJxNTssKwkAM/JWQe2KSvmG35168ei9tcQsepCxV/95d0CqBMGRmMuPuYwwwezyrgMlQjgYGkkbJyC7VJKSs3FG6k4WCu+InAeW65u7GrSVoByEfYjD5Eye0G1fYu1NO7d22TBGeHrVEeKxzDB5bhLCs1xA9VghbJhFeaTfZlg29Oypb+trs9M1VECpBdjsS3vRcNL0="#;

const FOLDER_MINUS: &'static str = r#"eJxNjsEKwkAMRH8l7D2xma1rF7YL3rx49eCtqLBCEQ8i9u/diLQlh+ElM2HSc3gVuvbu2BKaomEAgZrfgHHqFubKhXcS/bxTVgmBJY4sHSpgv8TNSf7Qrj8Q3uovNSgqkeyAs8tpYy1yGu+PG03onXpHH9Oto0n/XDWa11z5Cz7kKIg="#;

const FOLDER_OPEN_DOT: &'static str = r#"eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA+5nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171sg/4b6iUL8pa7qzyWm3vJ9TPc+1naA+BkPeQH3qFgOzrqX00fkFjJkzVg=="#;

const FOLDER_OPEN: &'static str = r#"eJxVjkEKwlAMRK8Suk9Mpj+fBmrB3T+B+6KLLiq4EM9vFLGVbIb3wjDjfX4sdD12t0pWyKQ4Q+IEAmmeUQiSa4POGzSJQhBf2cSd6qb4o5K2soNgnP2iKVNzcsbSS/R/lbVKrDIgI35Cv6LZsHvO9EQ3jYf3/OkFiKsoRw=="#;

const FOLDER_OUTPUT: &'static str = r#"eJxljbEKgDAMRH8luDcmKVUL1bmLq3vRoYOCg/j9tiKlILc88sKdO8MVYRubWaBHs5iVFCOjVQKiJGq0OiQESmFg7Dq0Ow6SUIqgT3ih6jnRzdUhV3ppJtfm0clV06wj098cBpiUhjdFPxuHKY8="#;

const FOLDER_PLUS: &'static str = r#"eJxNjsEKwjAMhl8l5J64pLOu0A28efHqwdtQocIQDyLz7W22so0eyv/n+0Liu/8kuLd4rkGrJL5XUKimp6SXZs2Uc6IDB7d0QsLeE4eBuNEc9LjqRoI71dsNoF9xtyyycAAb6BW7uLMrujg8Xw8YtUVRhJ/kv0IYpWTrvcGGFXgqHc7SfoZDcd3C/gFeXzO1"#;

const FOLDER_ROOT: &'static str = r#"eJxNTr0KwjAQfpXj9jt7lzY2kATcXFwd3EoUInSQIkXf3kQhlRuO75fPP6ZnhmvAUw/aZbGTgkL3PSU9jxumgjPt2ZnGCQlbS+xm4lEL0MMWr04wx/6/AXQVk0qQhR1UQS8Y/a6uiD7dlzTfIL0CiiKkd/kGYQmo1fSTo2+TRUGGdWgFHxRcL/E="#;

const FOLDER_SEARCH_2: &'static str = r#"eJxNTsEKwjAM/ZXQe+KSdt0KbcGbF68evI0qVNhBhoj+ve2EbgRC3st7j+ef0yvDLaizAeky20lAoFtHUC7jhrHgjAM53ThGJmuR3Iw0SgFy3OxVCfpk9gkgb9apGInJQX3IVUV/qC2iT48lzXdI36BYqFeQPuXiei1BVaZI/6LoW3HWJAOwWXcPbFvgDwVvMqg="#;

const FOLDER_SEARCH: &'static str = r#"eJxNTtEKwjAM/JXQ98Yk3aaFts978dX3EYUJE2SI6N+bMp0jhBy54+7SfXiMcM7uyAxCfTMICJANe/FyapU8I2P09vcyBozhLwHGrsM44UEMykrQl+iFNmJDz8aVtKuZJel11ukC+sqO9w70vdw5u1BFC13Sr+BNzIGtTFt3tfkAZEYtig=="#;

const FOLDER_SYMLINK: &'static str = r#"eJxlTjEKgEAM+0pxb732sHhwOru4uosODgoO4vutoseBZAlJSBL38VhgbopeIAzV5JCJKaCAoCyegh+NgjMwMKlSWKkWo5IM9xqduCxs7ORMuCs7KdpY3pNt/Ia3GljB44O/bb9YT+S82Z5pSl5pryyE"#;

const FOLDER_SYNC: &'static str = r#"eJxtTjEOgzAM/IrFbjcxISUSZWbp2h21Q4ZWqkTF+3tOEUUC2ZFyPvvuuvf4yfS4VNdE6oYwKik5lGdlvTV/TMC5lrSZeIlJ0lNa++pKOCPOkgZ123PS2Vd9dzLHvlt9vZJ3c8hhz72MC1BrGO8uiQUpRQ1R/ZtOKlGiISo7AdVaNoMHdoocOnPIfGQI1rdF3sRgB0+VosWLxYRBZPQSAFuhNO6sNq5f4sFPkw=="#;

const FOLDER_TREE: &'static str = r#"eJyVjrEOwjAMRH/F6m7TixNIpFCJjYWVga2CIQMDA8r3k4IEFlWHyovtu7NffozPQrd9d4IS+rIbQaD+XWCct7+Z21zYSfiuwBJZ/J0lMcQdTDSQNq8NE2qwxwmXbsib6f+QLYXDP0VlnWPEaDkSSwgNxPvWG/NHOELXoyhpddcmCySRm6rogg+6aHwBcFlOJQ=="#;

const FOLDER_UP: &'static str = r#"eJxljrEOwjAMRH/l1N0mvlShkUIlNhZWBrYKhgwgMaB+Pw5DioQsD+fnO115Le+K+2E4j2ColhaCCN+h8DJtWlxX2WuO/WZimpJofohOdMHjZm+fiKfxNwFcLd7cqKYZDfA6zGXXWsyldzHCwpr+yTPDPFR8ETv+AI+RLOY="#;

const FOLDER_X: &'static str = r#"eJxtjr0OwjAMhF/F6m4TX0JopFCJjYWVga2CIQNIDIjnr1P1b6hOsnT+fNblb/8r9Do3t0BwRWMPArlRYNzb1bP5widJftkpq8TIkt4sLczgssbrJflr2H4g/NU/LSgqiSrAo+nyobbo8tzlk+RI6myYdrCGifOWD4i/Le0="#;

const FOLDER: &'static str = r#"eJxNjq0OgDAMhF+lwbfQGxksARIcBovALSAmEAjC87MhGKn6ej+57vRXoL0v5ppQBbUeBKreA2NpM3PkwI048/2UVaxlcQdLiwgYczw5yUz1v4Fwq9liUFQcJQFrMXRlWjE8Mlcdgg=="#;

const FOLDERS: &'static str = r#"eJxVjrEORkAQhF9lo9/9/x3CbXLUGq1CJxRXKBTi+R2JQ7b6Zmcm49dxCzTXWedIq6AYQaD/dWD09jBHDpyL5UlTVilLFltYHCK84verde8Kwm5TDIqK0aljyBr/O1c0Pm0BuV316wtaJOcBC7woZA=="#;

const FOOTPRINTS: &'static str = r#"eJx1kLEOwjAMRH/lxG5jO6mTSoWlMysDW1WGjgyI78ehUDHAdOdc5HvycJvuC66H3SlD/UHGqY5hlTsY9wUq4RLqzJIiLQbl3JMjc0c+9pwK4k2QuDbpuJvboEoWg3tIZa9n9cnip0ChlCGX3XHYt/bjsDGYwGSFmCVEW1v0ppejWGf04aCVg1YOzewJDs0oDSSj30CwguANYrKBCP6AaGwqS/6RxG3SV/AEmOBGyw=="#;

const FORKLIFT: &'static str = r#"eJxNjsEKhDAMRH8l5F7XpMRuofW8F697l+5CBQUREf17WxSVHCYwD+a5sZ0j/Dw2xED8kZaBocyn0rcI1u6VmdqFbgr9HyaPjBBWj6RTbiltho76ws7igcuTumbfQDbqQRSZhUysmippfE0oFRVU2CyhOOpe4FbZASxXMCU="#;

const FORM_INPUT: &'static str = r#"eJx1y8EJgDAQRNFWlilA3Rz0kk0HFiEmuLlJWFC7N8GLIN4+8xhf0mp0ChxIU97UBFz7Eoyg8sCRo2mtAcH37RD8vphSFMzsiJ12Azdr69umf/vQDXkTJlU="#;

const FORWARD: &'static str = r#"eJw1ykEKgCAQRuGr/LiXnGHCFuoNOoRQoCAqJEG3T4J4m7f4XG/lKbme6C3XcXlFK8iCDYgx36rglh8F1+NIOLzaBbTdmqNAYGYE0ZKIPz5ReAGrIBjF"#;

const FRAME: &'static str = r#"eJxdjEEKwCAMBL8S8oGSPZQeUn/TQ0E86+9NQMV4GkJmR/NfPqryMsBUYWRqhtsgjqSXO0mD6T95wsLOwx0ZjLisKg5zV2bVGd0OsDkrUQ=="#;

const FRAMER: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Ik01IDE2VjloMTRWMkg1bDE0IDE0aC03bS03IDAgNyA3di03bS03IDBoNyI+PC9wYXRoPtteD3g="#;

const FROWN: &'static str = r#"eJxdikEKgDAMBL8Scq+aooLQ9gc+QqqgICLVQ/t7k1YvHpZZdsf4Lfh9gWCRGgSfmJoZM52py+/MOd0rzBZH6oH6S1HVKa1aCeSILJIz+3YsEMnigJAKomZUDfEgTVyxPpc36sr9+iTL334AXLMsLw=="#;

const FUEL: &'static str = r#"eJxVjcEKwjAQRH9lyT0xO0kkgaRnD3rtvaAQQcSDSP37dltoU/awPHgzk1/P94NGFMVB0Z+LAhSN83czYsEun8Tq8uKKk1bFr4qQFPjd/AzfSveibuwJ6P0AAlk5DY3LuWXCj6NEJXSMsqvYVCZRd7bC1bas0ScTXdNuQtJsPK4cKWwjE6pgOTA="#;

const FUNCTION_SQUARE: &'static str = r#"eJxVi8EKhDAMRH8l5N5sk7DoQusf7HXvSxXjTaSg/r1tD4KHmYF5vLBNKcMRURFsWmbLEblH2MolZc42Z+P7MmZreAiv6g1h/WeDMeL3A9wlAQ9CvePWJT/2yTsBdkoKSuK0utV6ukxib+pueAHaWicD"#;

const GALLERY_HORIZONTAL_END: &'static str = r#"eJxFjMsNgCAQRFvZTAMKfg9ABxZhhLjcDNmgdi/ERG8v8zLPHKsweYtF05RVC2eaOjnziZGGrPpfpLAJ3RYdKF0WGnRGL2yhCnKIO0vhGVTkG6wP9wBbnB4P"#;

const GALLERY_HORIZONTAL: &'static str = r#"eJxdjEsKgDAMBa8S3gXEFsRF0ht4CLHFdCcl+Lm9zUpwO8MMH6spZcESKJ7jjMSDo8StbEZa6q4m6IIeQQTdggl01WzacQC1ToJnHiT+fr/hC1spHhg="#;

const GALLERY_THUMBNAILS: &'static str = r#"eJx1y0EKgCAQRuGrDP8FYsxFgXqDDhEpjbuQger2aasI3L6P50ralC6PESQp76IebEH3W0oFAzpzVKl9QnBDG4I7VhWKHoslw8INWvrA3APuLvx7HrjvLGc="#;

const GALLERY_VERTICAL_END: &'static str = r#"eJxFi8sNgCAQRFvZTAMCfg9ABxZhhLjcDNlE7d7loqfJvJfnz02YUsA6k2NrEH3XUPSfGGliO/yi5l3oDuhBnMvBEmAd6NExoKpG31WSsJKlda2IL09FHfM="#;

const GALLERY_VERTICAL: &'static str = r#"eJxVjMEJgDAQBFtZtgExgvi4SwcWISZ4+Uk4ULs3+elnH7PMyLm5ISnXCcHGhVGGjqLUvDuuktyUjaPeykC0nYhHOROWy2He7tC1LkT59n7BF1IoHfw="#;

const GAMEPAD_2: &'static str = r#"eJxdkMFKBDEMhl8l9N7YtEmbgZmBxYsHfYhhFFZYxIOH1ac36RRXlkKTtl/+9M98ef94g2teAqUA3x4pwJWWUO1I/bjOD06tc2f9cjoQHRUldAW9Ix0hOd5IMNHA81DO98rjF53XzncJHXy68Z/b1xlel/BCDUsGeapYdWNgSL5iwakpFJRpj5hSxSTZEkJK5LEhSX7MWBPDhEwVMhAjS0/qVqAcShbLThYJJaJAjnQhK+DY91M23jEyGc1qtWfGwrzdHjqIovWZGpDuKKZj197pf6NY9mSqwtbI/hWr1bClKrFhFnUjzTZxA+biMGJR6PTnHMZAfnxWPqX1F4KQaO0="#;

const GAMEPAD: &'static str = r#"eJxlj0EOhCAMRa9CeoAZquOEBXAZJUJiXBgS8fbSFhOjqxb63k9rl7QGVdDBH9TROcCuVpRa6K3B2y9R3jLL0A/YMcKYprxQ+uxbbi8KDi13+Gh88Dy/wrBxhjiJuPFbGLPaigNal5eXfk9TjrXToGJIc8x8SdVI8CfaqDzP"#;

const GANTT_CHART_SQUARE: &'static str = r#"eJxty0EKgCAQheGrDHOBGCMzcLxBh4iUxl3IQHb70EW0aPMW/8fzJe0KkvIhykgOoVRGg3Azjgi175WjStfgh3YI/txUIDKuCziZW2/l0x2QEfsDREBWplcegSsk/w=="#;

const GANTT_CHART: &'static str =
    r#"eJxtyTEOABAMBdCrNC7AN1BJ9QYOITF0NLh/xNLJ+p7seYxWD4OpGFJQiY9UPAohW/sEQGCrPhdFVRWp"#;

const GAUGE_CIRCLE: &'static str = r#"eJxFjEsKgDAMRK8ScoBoqlWEtDfwECUKCi6kuNDbm1JQmM9ihidnujZYAs7saQBHY+IWTBZmT2MxRmnKMYruWY8V9AnIDkHv2jmgK6c6R/mpHfWGMzRP4D/OC/7pHl0="#;

const GAUGE: &'static str = r#"eJw9ijEKACEQA78S7M9z3YXjYPUHPkKwsBEs/D9qoSSkyIz2PCpKMI08SCCPmKjvfqMeltiygP5MDqtrdj7LHu7aE8xdEk4="#;

const GAVEL: &'static str = r#"eJx1jTEOgCAMRa/y416koFAT9ARegujgYuLg/WNxwQHT3zbtS/5PV74P7HN38gD2FM0I7Y2MeBU5w7FsD/utrH+Hd5SbYcmvzGDbLakvnkuqzgGqQKHBBPJDJkRl0iBOc5iEKnsA1mwwrA=="#;

const GEM: &'static str = r#"eJxtyyEOgDAQRNGrTOo3dLpNQ5OlJ4AL4EgQKxAI7h+KqUL/9+0+Hse5hK1AnenKKMII6ppQ99Bs+kSz4UgoZtQuqchCFZXyA/vvKY7wAq57G4I="#;

const GHOST: &'static str = r#"eJx9TL0KgCAQfpXDXbsfJAXzCWptFxocHBqioafPW6QpjoPvP53lqnAsZotAWB2SyWlSMadhkf/xGLgECIB6tqObuIkVYOf1156g2AablQnITvjt2fCM9RczACSN"#;

const GIFT: &'static str = r#"eJxtjsEKwjAQRH9l2LuarA25JL148eJHFFtMoLRFAzZ+vZsUL0VCZtll3jBumcc8xmnAMscpvTyxgmaIMqPZRDO17vRztu453BOyJ0t4xz6FAhHCEB8heTKEVS4FKcbW1fis5caEzJVbZZVYrFyneLfopUsBvaebdLBXezQdHw3KV/I01MFctAaXjrbKp3YTbAeHZgerDT7/h7/410M7"#;

const GIT_BRANCH_PLUS: &'static str = r#"eJyVjsEKgCAQRH9l6C45iouC+Qd9hNDBY4fw0NdnCXXxEsvC7rwZmLjno2BbplVgK82U4nxLKb6AHiFbWGiwrVbN+Xx95BxkBIZ/M3SQHBC6R7VrXIauyoAYgr6oD10qxzbq"#;

const GIT_BRANCH: &'static str = r#"eJxVi0EKgCAQRa8yzD7KolBQb9AhxIQEiZAWevtmjKCYxZ/Pe1+neASowuCEUEeDYkYoVBeKkcPqnh2rfcw+BfC1wdwWvtBCsvPQn0WgCa/9sU537bAZXIUE5RQoGOhERx9rjO0NvyUpow=="#;

const GIT_COMMIT: &'static str = r#"eJx1yzEKwDAIheGriBcoWjoUNJeRDIHQIZPevo0hY6c3/N8Ta8N6haF4IpgrEn8buUWOlYv09lRwVrwRnBIHLRy88USbznhhXph+8QuGTCEM"#;

const GIT_COMPARE: &'static str = r#"eJxNy7sKgDAMheFXCdlFk0Kt0Dq7uLpLFRQcpIjo29taLyXDGf4v2s7OLiPY0yApBHvEdQYF1jqPudYv81k+9f6RKVr7bYLBYEsC5CR6BobCH/nlvQw0kBQSkGrULzPOuKs+eQHGmSpz"#;

const GIT_FORK: &'static str = r#"eJxtjLEKgDAMRH8lZBdNBKnQdnZxdS9VUHCQIqX+vS0FySBZctx7p/0R/LmBfwySQggGewSfcmK0uq211QIbJDX8QaluSVhQl7t3WA3OpGCM5BgYunzU5G9SMje8jEUtihQZiGP/NS88kjVq"#;

const GIT_MERGE: &'static str = r#"eJxNi0EKgDAMBL8S8gGtQrHQ5gdevZcoKHiQ4kF/b5J6KHuYhZ2NfBQ+NygJRwR+ErpJ+BopdnWm+Gs6+OoJ7NRIV753WBPOHga3hBwgQG+RpqIK9AFgdB93"#;

const GIT_PULL_REQUEST_CLOSED: &'static str = r#"eJxtjbEKhDAQRH9l2D53bMRFIckfXHu9REFBRcQi+XtNFLSwGnbfG8b4YfVjBx8scUVYLRUEH/PlzPfEzlzajYVyR57S0mw9Wks/rsD8Kf9cJpzeN5w0o1ACeUe1EvVA4zB3iNqSZkLQeTdwjnhEnczkuB1BsDca"#;

const GIT_PULL_REQUEST_DRAFT: &'static str = r#"eJxVzM0KgCAMwPFXGbtHzEgK1Dfo2j0sSJAIidC3zy+oTmP7/5jQxmm7gfYSaUBwEjsEHfKmRFuyEpXVHDEvin/RuVw7rBInGoDPfUrp9A9Ed0NvsubYIDCJjBA8y4895RHiGJNMRj0c+y8Z"#;

const GIT_PULL_REQUEST: &'static str = r#"eJxlzG0KgCAMgOGrjF2gNsEK1Bt0iLBAQSIkQm+fH3+C2I+XsYcp66MNB9ikkWYEm3ujRoFGDf1s1IfJruQfXdvtYNe4kgDpxMbAMJahUn6mSisxKvjzgMwamRAStWeJW3LZliqrMS/Nliq9"#;

const GITHUB: &'static str = r#"eJw9UMuOAkEI/BXindqBhrYnmTXZ7NmPMOPBowfj91tMJ6bDq4CCZnveXg+5/56uluL+1rgFhpQs9dS0IfdGv6tTEollqMFT4Wd1xJhF8ILtsI6WbCY6ScoXQzLRQ2kTrZflmP8uLjllV1SdZdlJIiT5S8TSZOpjLwlZ9yMnhaf00uxaEauiD7IsjEfS6XTsDA50Z+iNYeEj33G6bD91g8v2vcQqNnYNpIkrV1b+8lv2AUS9PiU="#;

const GITLAB: &'static str = r#"eJyNkDEOwjAMRa9idc8ntiMnkUpnhpyALRIDQ5EYuL9wBGWoGKq8LD92nuz52V93up2nhwixQmpQqAaOHUkciuMEcPJboIMtE/GModX5ZYqY963euQaBGBksX/wPaQYm99ne80djAbEc07TvFB05OZ83gmQUbSwkvFZYDYZSvIadrUZdXfQ6LfNpLGV5AxTnODA="#;

const GLASS_WATER: &'static str = r#"eJw9jLEKgDAQQ3/l6N7aO722Qi24OegPuBUcOjg4+P94hVoCIQ+SxCe/Ba5FHciGgGgLJmQJYEWoSaPx884wFpxugYCA3hCuvQNteaoUh3qXYj91gJQZuDWd+E+2Up98HKUeEA=="#;

const GLASSES: &'static str = r#"eJxtzVEKwjAMBuCr/OS91aSNU2h3Aw8xoqDggwwf9PamFMeQEQIJ+X5S7D7b4wr7VGIl2LvSgTBXyjSWXb+O5af8ykda9D97Tq8bLpXOnME6CQT7VkFCm5cN0lJNrzISFZygGCwOgWMCx+w2hU3NnfPJfegBb/VX68AXwbQ3+A=="#;

const GLOBE_2: &'static str = r#"eJxtTk0LwjAM/Suh99YmXalC1/MuXncvVZjgQYYM/fcmLWwTRg4vj/eRxFd+T3Dr1ZXQ+A7QDxgyAYGV0bwtHQsqxZNYU1wDAZxx3eizA9fcjG6xaxpB0rbwZtBchAnf2oF01XXV5ZamyRkMB9eQ29jkRzzv39P7Qqxc479hIGP91lgec3neoXx6haSgfBvODFZcTU8/XZFBdw=="#;

const GLOBE: &'static str = r#"eJxtjUEKgDAQA78Sele7q95qf+AjpAoKIlI8tL7ebntTWdiQSSDGbd7tC1wYFLGCT6IVXMzWmqbk1uzbsSBSaUUuGpIXSZZzW1rWnNO1Yh7USAyeqK9b5KfTETqQxgtW/7D6Qgi8ZUtW7APW9Sv5"#;

const GOAL: &'static str = r#"eJxtyjEKgDAMheGrPLob09RAA7XgAVzdCw4ODg7eH6NDJ3m86f/K1e4D+xzWKIhpkzNjGvyhlvFttXQhTKqITJIWg4ERfW7J/nh22hT6OUYmgxBLlw9pCB3e"#;

const GRAB: &'static str = r#"eJyFj70KgDAMhF/lcG9NYpQWqm/g6l5wcHSQPr+tgz9QkFuS474cCXs8NqxjM7MDs+0XHwUCKjJiJNF7hyS22kyhLdwUHlrBtLgfVmokwVv/W9tX0AGs32AFpFpn+fVOcpaCUhcd3EWyydNm9G1k6xO/z57yqkpv"#;

const GRADUATION_CAP: &'static str = r#"eJw9yrENgDAMRNFVTukjbEeOhGSyQYZAUFBQUCAXTE/cpLj7zbNnfy+cW+oiYPLaIzdT1lFojvtSsyVgs8krWFyPgoJ1jAXkWSf8Ad17FjY="#;

const GRAPE: &'static str = r#"eJyFj8sKAjEMRX8ldG/sw8QW2vkDt+6lCgqzkMGF8/e2TQUFZ9yUkpx7uIn30+MK56QO1gId7bgh9AHqo4a4rdsh5tuUxwvkZ1KGkRXkufwaA1NSrpLCfLEejRF2j7tV0lh01LUGmRbhNpaIw9DtpQj9ElebRh16oNT5K56b2EqCkXjJWy56nyR2jX5d22t8Nn0Bhwxofw=="#;

const GRID_2_X_2: &'static str = r#"eJxNi8ENgCAQBFu5bAMG+Pjg6MAijBCPnyEXxe7l4ofPZrOzE1s5lJ6aVRhuBUmpp+jfW2d40MsIoG6Z4mJCiteuQpmxBXJexnkQ2ybiPIV7Ih93JB49"#;

const GRID_3_X_3: &'static str = r#"eJxty0sKwCAMBNCrhFygWBEqqDfoIUqVxl2R0M/tG+tKcJUwb8aVtDO8HjXCnSOTR7UgUMoHcfufH4ucGYOb6iC4c2OC6HHVYElaAjXqQJmxWNDXEJTp5QPKxiyp"#;

const GRIP_HORIZONTAL: &'static str = r#"eJyNzTEKwCAMheGrhHeBYiGDEL1M6FDo5NTevhEdFAWdMvxfEtE76XORfgEelAIcSF8bJ6IcpUapqsmGy86ocmd0JyfIquPlxwnzW4wb9QOELkF6"#;

const GRIP_VERTICAL: &'static str = r#"eJyNjTEKwCAMRa8S/gVKChmE6GVCh0Inp3p7RRdBRacM7+U/tTfa95D9Hg5kyYNvUCwHQa9Gg46W7KRK26CbWX2p/rAsxgo4SLIsmxmOMkF6"#;

const GRIP: &'static str = r#"eJyNjjEKgDAQBL9y7AfEwBUHl3zmsBCsUunvNTmbIDGptthhZ9X2bMdGdkUwyM6INYDyE0i6eJu0pWrrrPSp0nN/6jUVeGCsmMv+1nhuTJr/YQ6TgfP77Aa2TGJx"#;

const GROUP: &'static str = r#"eJxtzjELg0AMBeC/8sgee0mRQ1DnLl27l1MatyKHbf99L1KLg0sI4XuPtM97NgwdXc+ItzoFlkqqhhXKakp9e3LRt38nEWfTVBgCFFVThi5HUgUSF00BBbNDrzU+shEqlzrxWsvK/gLrsrfzmDLeHUWCjdPDckc1YS4XIXzW+2sasvlWQs5/oR13HXbQg6JbzRb7AhuVSW0="#;

const HAMMER: &'static str = r#"eJxlj8EKwjAMhl8leG9s0jZrYQ68eZgPMfSwwwYeZM/vv4giSNM/TWh+vvSP6TnT/XRYpZBoqFwI9xa4JkRQlm7PieLvmdBXctlroRjSiKIdhv64Ww79x/gKA8sEe1WSiPf/n1UjNyER7oKwFhcwmEdLqLNnRX8Dm41iHIUy21S4GLk4WkjcMgYsX9rCTbnq2VgqubxpQVo5b4KhBWvMyrlb4G0k4PjyvQD1SD5Z"#;

const HAND_METAL: &'static str = r#"eJxtj70OwjAMhF/l1D0mdpvGkUIXZtbskRg6gMSA8vy4gPgpleWTLH+ns/O13mac9t2RFSwUCvsqEPilnDhpPzOkMQ3dlHeLccof+wDmkl4sGzvAN9kibe0tKKzY9M9eIjg4pjg+5PsQ0h4mcu5phPUhUoAw2R2JjBP7xnSWqtCnCeq0xFVqeKfeAdW9PSQ="#;

const HAND: &'static str = r#"eJyFjbEKwzAMRH9FZJdqybKrgJsvaNfsph0ydOhQ8v2VS0gyBMIh0B33uPKp3wlet+7BBsxjrgICoQkFZQ57D+67oVwaM5SNVOAw6gkpR2RwktLpqh2xBrbU2KUQ5lw9+2OM/k0oTxTyBJUSkmVM1PceRX1jpNxuXWLwamx9uV+B0zr5A510Qiw="#;

const HARD_DRIVE_DOWNLOAD: &'static str = r#"eJxljVEKgzAYg68S8u7Wv5TSh9Yb9BBjyn4fBkPK1NtbBRGUPATykST+XkXRJWaxsP/ANj63qI0H+IqHbxxcU3XisX8XLIniiGnoiiZaQ2g/fLQkBmKuCTHuVmtb4VzNHhL0YeR+mMVc2QoS8i4G"#;

const HARD_DRIVE_UPLOAD: &'static str = r#"eJxtzUsKgDAMBNCrDNmrTZHSResNPISoGBeCSPFze60iikg2gcfMuLEKgsbTwAYmyeMhp8JlEQp3c8kaerYPTG0dMK2eNGHpmyDHpwjS9p0ET5Zw2eaJz74YePUZsJVU8d+U+toOB/0uBg=="#;

const HARD_DRIVE: &'static str = r#"eJxdjsEKwjAQRH9lyT1rdk02FtKANy9+REGhQhEPUqpf76atDUoOs8nMZF8abvcrTNQaZgMTqxp46ZWK8qw57Uoqp0f37OHSmnNAHyAgETAQj9KxDm4+OvX082B5tDLYvXas4KE5Vo8Eo4A/RWRfK5YwNkD6/bvsLltz2jhl5ZOVTxZsQUeVdEuTW2xyxf9vfvMf4Ec8mQ=="#;

const HARD_HAT: &'static str = r#"eJxtjaEOgDAMRH/lgm9YN9YgBhqDxS9BTCLIBF9PIWSMhNT0ru+uYYt7wjo0swX3kcEw9+iWvgZxJvsapMbkag3O9mjG0F6dYyjNrGGz+ILy1ZVsrTXqf5Id2GdyUSAPKSTJ/P3ooIcahGRXwBPOnDYz"#;

const HASH: &'static str = r#"eJxly0EOgCAMBMCvNP2AbEWjSeE3HkyMZ/y9NCUc4LTZdlaf+72oIHFk+iTxyVRqSKgVVrMuZrKOEpsTyzaZLYI/D7erTwUDbUef7N3aErHbH9hlK3g="#;

const HAZE: &'static str = r#"eJx1zKsOgDAMheFXaeY3tu6GGNMYLH4JYmYJgvcPrQDVpan6Tv5yt6fDtakRDUKidybwq1oWtlq+xUHmO0pgJzJcplo2iYt6UqVs3lEWdKI4CvoWIIDl0ytYaYUQTzTxpxd0gj8V"#;

const HDMI_PORT: &'static str = r#"eJxNjTEKgDAUQ6/ycf9qoiKF2tnFC7gVHP7QwUE8v+1SS4bweJD4Oz4m19YdpLgIgYwlCsU+tSx4559zw5AoNDBRc7VScXbBD2U8+Hqx9ouA5qr6AP+hHYs="#;

const HEADING_1: &'static str = r#"eJx1zDEOABEQQNGrTPSbzcxuUAw30OolCo1EIc6PioL2v+RzCTVBNML9gJS0sPzOZHkH7eUBkC6SUY0ZfA+1NexHth3A"#;

const HEADING_2: &'static str = r#"eJx1y6EOgDAMBNBfaeYP1lKWibI/wOIJiEoE/x+KIQgQd+Jezo71dNqnNCuxeE3N+ntq9oa6lA9g+RPhEIduGUqKIVIog7sRAolW8HO7AFDBIlo="#;

const HEADING_3: &'static str = r#"eJx1jDESgCAMBL9yY59IIqgF8gNbewYLSgv/P4KNFNhkctns+SveGec27BaieR2CH+sp+Basx9wBor9kYQcx7JLwQoKpRPNOYRcVWpKBUNm6NmpB0voNC55gqRUNKX3iA4+XMDo="#;

const HEADING_4: &'static str = r#"eJx1zaENACEMQNFVGha4a9MAorABFk+CqEQQ5icoEGD/E19a6Qo1mMSApN5E+VaKcoLP9gJIT3GA/2DlixEu26MJc4wk5Q=="#;

const HEADING_5: &'static str = r#"eJx1ja0OgDAQg1/lgqdwd4MbyUBjUCR4AmISQXh+fgRBMNMm/Zo2bPMeaW2zwRFL9FkXijvqwhf4qf4BLElixHrkGl0CGmyBg8BDiaGPVlSSwHIGPy6oRm7gr63r6K5oz/YunpvIL4w="#;

const HEADING_6: &'static str = r#"eJxtjDEKgDAQBL9yXB9NLioRkvzA1l5OIYKFBAv9vVELg9gs7M4ydh22AKPDrgJFwaC35TV5mwPTNz9A0YfwHHmZgHeHqkWIDgmBj9Tu04MzAUlQkgUBCQ26qFO+thMkACou"#;

const HEADING: &'static str =
    r#"eJxtySEKACAMAMCvjH1AN0QMcz+w2gXDosH/IzaF1TtZYxvMii0DsRGjSrim8gzHnpyg8s8BT+sVqQ=="#;

const HEADPHONES: &'static str = r#"eJxdjrEKgDAMRH8ldA82qVILtbOLHxFwyOgg+X4jOLRyw90Nj7t6ya1wbuFIQLMmYWCILnJn6zp62pe+IxtmKVA+gFaIlkdCkf7IsIGsKbQ6vTfaA11OHfs="#;

const HEART_CRACK: &'static str = r#"eJxdTksKAlEMu0qYfeu0fR8HngMewAu4k+fChYIL7495A7oYSpoS0jbtffs8cD9NF1tgqZumRdgKQkLdSFnzmcDAvJUVjtHpqwWzBDRLouRU8gbXmsSHKL5bdhw1d7IGaEXSmceG4VlRr9PaDiPT2n7JXuawEBPjAxfmGvz3fQE/mSkh"#;

const HEART_HANDSHAKE: &'static str = r#"eJxtT0uqAkEMvErhvvOS9B/mCR7AC7gbxoULBRfi+a0eBPFDOpWmKt/pOt9OOP5v9tZhaTFJPRAKYojixpAl7+gYrqtZ4TcuzKsFGiIkh0TKyeTVXWoKPsjgH8WOJnlhlAimIomy2Ug4V9TDZjv9jZ2202szR0YXTajSy+xiFSvo06Jou+sizfmGFKWNnqL17ASu1OenRlhPYFHtUOq9kC7le/DFGowHBP+lZVh70x7IP0hr"#;

const HEART_OFF: &'static str = r#"eJxdTUEKwkAM/ErYe6KZ3XQrtAUf4CNKFRSKePDQ/t7EioiESZhkMtPNt/uFVvQJSLSqz0TLhy5vOnS7EA3dY3xe6dynkzZitDUQdObKdWIVcxTjzFng3cRGBwX2XkoQLVwkW5iG3Y9pK7WhLDqpqAkQ4iy19R0KqRxoCzA/1MKg4hTHvwBQKzbt4xkuzhQWDGmqq7R+c19oLjYz"#;

const HEART_PULSE: &'static str = r#"eJxdTjEOAjEM+0p0e0ObNA2VyklsLHyADZXhBgYG/i/cIt1wipxEluO4fZ7fjV6X5Z4qpdwT5xrQCmlQloRhbFeABuKsVLBqh84LxaDEFjIoAWMTwp6DDDLI4VjozNYxWQlSyhxhNgRvJ38sazuNTGvbkyGHUJJbhWT4w+L/zaeBsm3G4vvlDzz7LPI="#;

const HEART: &'static str = r#"eJxdjjsOAjEMRK8y2t4m8SdhpWUlDsAF6FAoKCgouL+YbIms8VijZ8vb5/F94XlZbnVFjVE1VmFrcHG1SkvNK4WpclRtHH2Q6w1FHJoSjIxJHjLtITZDsb9lw1lz0NVBFKGFxybw7uj3Zd9O86f9B4uwH20="#;

const HELP_CIRCLE: &'static str = r#"eJxNS0sKgCAUvMrgPntPiRLMG3QIeQUGLUJa1O3TgohhGObnZc2yLZBzVGwUchFSkOuxwbdvH/wej4R5VJPT5OCihQUVMDo9WLAQTFPCyvqr+9+LDbhPmvjrbqWIH0Y="#;

const HELPING_HAND: &'static str = r#"eJxNjrEKw0AMQ39FZD819jmXM6SBbhn6EwcdMjTQof9PnYaGItBgoWdNr/Ze8bh2W4YMGCiadrtlZPQhgfRUgy+Sm0KPU7itSTlsllgqjDImY/UmrBVfO9pKjxZzeUau8cDtn230AhkXZ/3hQ0loAff7CPFuni77ynk6t2oUUVDO6AMQlyqM"#;

const HEXAGON: &'static str = r#"eJxNjjEKgDAMRa/y6R41bbEdasEDuLoXHBwcHMTB05s4SMgQeOT9/HK2a8c2ucUzeFxz8/AYdIiJuxQOShQNlS0Icf4RAvJtREiSinJkoHqSZLTv4+Nq6bVEfQEGuB0S"#;

const HIGHLIGHTER: &'static str = r#"eJxFjDEOgCAQBL+yoT/kDiSSIC/wEyQWFJhYGN8v12A2U81m8l2fhnM3VwIzRcTXt9Q9eVPyorLkeREBCwUbMagCgRtjErvBdVqtKL+AU3VwQJi1D07EGss="#;

const HISTORY: &'static str = r#"eJxtjD0KgDAUg68Surf2x2ctPHsCPUTBoYODg/T8viKIg4SEQPjCZ7kq9kVtAc6XhAQLJ05auon0hO3Sk4kjvMQaMKvMQ4czfy5Co0o/i/OIjQ6h3/EG4sUegA=="#;

const HOME: &'static str = r#"eJxNjFEKgCAQRK+y+C+1GyKC+t01hIIEUyEJ6vRt4UcMDDPwZmwNbYPFiX0CA0Zqdn0iBgKCkYWS06z+XdItvB3epbe1pCvFvEItMbfDCQNEfIIEqLoTfXwn/QOGnh24"#;

const HOP_OFF: &'static str = r#"eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUXM7efpxhmNoZYcuJPTy/y+vf+/0Ufl+WqVTJlybt2qmQBb51MSfVglWhRrGQ2KalI7JxEtYlZWbb1zUW29SkFFXK5vRLE+hBThdohLoFzzqzcJE+qTyMeDkcpcMIqrBPUAmQBwiVIo3jGNJc9kXB40heJCDd6NoqM7bzajMxgyClA/gOjbMImqS2SR9tVQo3UJHVDV/WGaN8OZYzDe7lzLiODSFjZB3T14eIeJNfdrdcGIkYwWVKOOAitQ4kwJmkND4xh3lKLwlu1gn3R9vL38/37STe7LGYL3RV5odtI9/EVqEPbA6aYd3k="#;

const HOP: &'static str = r#"eJxtkb1uBCEMhF/Fut4EA+ZH2lyzdV4gXUSKlCny/srY7N01FJi11t/MAMfv198Pfb/fPqQFJQ16yqBGKaIblIREJqeAH6yE0rHkdj/eDLwfTxwDZBJnIwiMJSAChQniEmAxfkNL9emgEwQJZWupGkeRC1bduqb4oFBBZq/FMrpjNhr73tSZOMXNnijq4sFmkCawtTasPiKj8dDrEjkZ7MasS20jUULrmayms4cyMvDYIFY8kXTSydjZU3G99OWqhS6zdVZr3Ww9gqWpeBLCaJ9cPE4OIzUeQWPnkKQxxpL0z2049dvIlCcH6cWKXl9dKg74epF/cDF/lg=="#;

const HOTEL: &'static str = r#"eJx9kDELwkAMhf9K6J54yeXOE87ODnbt4FZ0uKXgIP39pqByYK9kSMh774MkP6dXgce5GziBXOIkIODWQpsWrhbWpbDUC5RR6wTKrevzYWX2+UueT8ARyGtCEr0zaQzI5NiDp6SfMZB3Cu7Kwdz/kCHBsZhxQzF4U5I9iblNbGppJ2YfkQUjhdkuGUV+njcAW1oJ"#;

const HOURGLASS: &'static str = r#"eJxtzrsKgDAMBdBfubi3mku1Hargrqu74ODg4CB+v6mDipRAIK9D4j4fK5a2GGuQq7iii2XqdfE7yQ/E681pnBXPmSCqFMbWoTFinbhBCOGmG1rgzv2zBw/xNjBMZAZX+/zJSDK+8k2aP6x/Nelyet0L4p82Fg=="#;

const ICE_CREAM_2: &'static str = r#"eJyVjTEOhDAMBL+yoo/PxrGTk3LU1/ABOgQFDRIF4v2EDkq0Wk0z2i3buC+Yf00vLSRNBkYOLfm3wv9xYiipQOHI8GENEXHJFUEP7Y0kQmRUMlxlSI1TEnDTlc+13pXbx0vfquNkD59Dusknob8sJw=="#;

const ICE_CREAM: &'static str = r#"eJx1jDsKgDAQRK8ypDfuxg1JEQN2FnqIgIWNYOH9MfEHFmHZYob3JuzpWLH0anNghmjyYNKdTQwGXcfaC2jiQqgY2qLE8Ipz7t1gYR86pwqUDMw9mF/GX27kk07c3SHz"#;

const IMAGE_MINUS: &'static str = r#"eJxNjkELgzAMhf9KyL2uyWhXoe15h+26u3SCghtDhth/b6ooEsjLSx7h87/m38E74JMJ6ol0w8CgpUjJdDdnr/h18iC+u2H0l/Ij+qH/tpA5oEHItMosQlZUtswlWkLRp35MQwtpDlgjpLzKKJkS2Y7R72QfISOjrpV2dusHg1ZcOXagHxaYDpQFa/8zwA=="#;

const IMAGE_OFF: &'static str = r#"eJxtT8EKwjAM/ZXQe2OTNbJBN/Dmxav3ocIEEQ8i8+9N2rJeJPCS0Jf3XtPj/rzBSqNjB9/SVtZma+lT2hlpSq/5vcB1dCcKGAkyzgwMAXTzjH2XwS6MO6UirjJ7lz2oQyk2dTIHag5bGOrLI0mNQ/XsXxyVGsDgQDgMkCHk6kA+FGtGK50WihcUsdAYhD0y6xTJq0BLvomzfk3O0jQ8ez425g9tHkvR"#;

const IMAGE_PLUS: &'static str = r#"eJxNj8EKwjAMhl8l5N7ZRLZVaHv2oFfvow42mCJDpHt7E8vmKPTvR76GxL+69wD3gFcmIP60HQODlUNGXud6z4ZvOwbhocXoD9oj+ml89pA5IDNCpoDUICzCtQRpiKrSqqpywt8XTVVdUfmvpnFOUw9pCSjOrDVIWUGUUox+XeKhS9TmWFnXlHsb1xquHDuwlwaYtqm/DGk+nQ=="#;

const IMAGE: &'static str = r#"eJwli2EKwyAUg68S3v92+sqKA/UEu0SxMoUNhgitt6+vEkggyWdLDBWlOWJCivmTqiNtCKejhdBuP/Je06jLKU9vH8J5G3IJ34hy46Fvrx5Nol/G6O1/qwm7ox9r6Oe0zMqswzcGQ4kmng0bqPcK1gIL5C+FLCmB"#;

const IMPORT: &'static str = r#"eJxljLEKgDAQQ3/l6H7Yi1U71M4uXd0LDl0EB+n32w7igWRIeCEJV74LHatJAhqrwMQwdBbD25yeRMh1sfvXydO0uQwC2S5uqYr9QHMUmTVg7IteMIq6fgDgbyNR"#;

const INBOX: &'static str = r#"eJxVTdsKgzAM/ZXQ92ama1OFWtjbXvYRwgYKRQsTYft604KIJBzCueSEvKRfmuYP5GWa12+vjAGS5YoWyAE1BdtCFE3FcDtSMeRhHeHdq5dD68AhUTVtPBR3U0eukS6ENpvmpO+S0Yxt9zg1YvQM9unR2DOiCX0HJO//tV9a4w70CCtk"#;

const INDENT: &'static str = r#"eJxtzDEOgCAMBdCrNL2A+TVBBuA2DiQESHSQ24sExMGp6e/rNzmFEnzcKScfz8PySpo2gtBKUOzMMoQzzRVYhjBdUieYivS95oLn4YvrUU3bTetQPxR6mrcfLR/4BvEHLqk="#;

const INDIAN_RUPEE: &'static str = r#"eJxtyzEKgDAMBdCrfLpbG4OxQuwNPITokEVw8P5YHaRDlgT++1+v7TYcS1gFbDSEov0bFW0gu3AKiJHjiOyuiI0dmCvsEkUmJHy/o4RUz19+ACWGKAU="#;

const INFINITY: &'static str = r#"eJw1jbEKgDAMRH/lcI+2aagdav/AH+hW6uAiOPj/eBXkSAJ3eUm+23Pi2KbdK7x2UdE5rmJUFGsGg4NnOaSu7CZ+DgEM60WX3gAwFiN+4JMknuMwDEKY1qnkZXwsLxFqGW4="#;

const INFO: &'static str = r#"eJxVyUEKgCAQQNGrDLOvHIloMXqDDhFTYNAiJERvryIIrj78x/J4eW+QaJA0gqRWX6LQ8tLc8nf+Di6DB2mgLUxrxTpH2t2sqFMG6RwaTA=="#;

const INSTAGRAM: &'static str = r#"eJwtjmEKwyAMha/yyAFcY1u7HyrsADvEWGUKY4wiTHf6GlsCIfnyXhK7hWdGcaQJteetNTPhl9YcGxgaqZ3EkF4xd+TtRXzefh85YnV0ZwNmNS63CRMGsIRWZsQVB+nsFP1lgVi9fadPQNGOeFEztx/YkVHtWtVnUfgYikfUfgf9iCwA"#;

const ITALIC: &'static str = r#"eJxdzEEKwCAMBMCvhHygjehBSP1ND4XSs/19d6sgegobZtfv6zmlhkNtV3lxo0o1xIxojMU3ouI/JQmgrKROY6N4z5Ym66jQpnX2A2DZIKQ="#;

const ITERATION_CCW: &'static str = r#"eJwtjFEKgCAYg68yfLfURP4H9QYdIipQEBX0pduXEnvYBt9m69EDLsd2JSDFKbheNN8Ww2moccIsmI7hFIh5u46ht7WkJ8V8o5aYe3NMGkiNcUb4slKT/Sn/AgR/HOI="#;

const ITERATION_CW: &'static str = r#"eJwtjEEKgCAURK8yuLfUJP5CXbfpEFFBgqigm26fVqt5DG/G5K1eOCxbNaTYBdeDxjTMnECcCnVG5zd7u2jmzNhnzuQU7uDjiZx8rMUyglJoT82D/MRfcQ99UxvK"#;

const JAPANESE_YEN: &'static str = r#"eJxtizEKgDAQBL+ypDe6F+5QOPMCbe0FizQBC/+P2ohFuoGZ8XO/Co45rBRMUTdhHToy6mJI1WAPcUQK2fu3zf4dBmqhtA3/5gZNNhsY"#;

const JOYSTICK: &'static str = r#"eJxtzLEKg0AQBNBfGbY/kp1EQ+Du6jRpLezkFBQsRET0771DUAvZYpl9y9ihmlrUTv5U6KciiGcaQ8Nfds3gzDPHzVbf14PhbFiKt49U6u1RnUOzSDeijFR8TwndGPoGYXGiFIxOXoKwOsnTz65+AyAaLXo="#;

const KANBAN_SQUARE_DASHED: &'static str = r#"eJx1jzEKgDAMRa8S3EWTVmug9gYeouDQ0UF6ftNFKyRkCf+Fx0+88l3g3Idjg1DDkOLUkhTfHEmA18AqgBWwgMsEBHObUTblhsEV1KTeAtxZEXQrIXBFHaA3CX9io67UIlR7sQUWAb131LzOquV+jzyUMHME"#;

const KANBAN_SQUARE: &'static str = r#"eJxty00KgCAQhuGrDHOB0CIN1Bt0iEhp3IUM/dw+hyBauPkWz8vnSloZKOWN2KOyCOXyqBHq9ghnjkyv3wLBdXIIbl+YIHqcLZjDiIv8XOkahlYYa5i+8ABnhyT9"#;

const KANBAN: &'static str =
    r#"eJxty7ENACAIAMFVCAsYjBILZAOHMLGwtDDOrzRUtn95WX1PGBUbQz5EqBIsqThQfMI/KLYklwtKwhXO"#;

const KEY_ROUND: &'static str = r#"eJwljEsKgDAMRK8Ssm9sbawu2t7AC7iTKFRwISKit7dBhvksHhOP+SqwJBxbcMPtxQIFYnCqwrfxxWu0uyM21XOgDtS2Etaw4QlzbPQmR9lO2VeQN2FPHYI8CV3QdSasVcEfyR8OVR52"#;

const KEY_SQUARE: &'static str = r#"eJxljT0OwjAMha9idbdJ4rRJpNAT0AuwoTBkCBID6vnxq4AF+Vf+5Pfq8/bqdD9Pmw8SKUhqUliKbTOG2tGNWWayMoQAIwc0WCXZkhoDsAHrx88lGymSQT55vIFep7WeYLzWr/3DRzIl0n+0FdPzThYK5POuzZEsOCF63Fm7ooXhJbLVT+INd94zSQ=="#;

const KEY: &'static str = r#"eJxNjEEKgCAURK8y/L2Fikng9wRdIn5BgUFIi7p9WhAthhmYNxNkzZJmyMWkXeMImelxOZl8CTG0LxPDPh4LJqbNaBjVNx2KKlCLX12PULawsIMx8ElZZT/wBiRbHoU="#;

const KEYBOARD: &'static str = r#"eJx9y0EKgCAQheGrDHOAmpEwF9oNOkRUNO5ChOr2qYtoo5th+D+eDfsaQXZ/SHTIGiE8DhXCXe7ltyjpI4SUh6SlT7bPu8meSxTYHM4ajHREnCnHHzE1bGiYqZsBVtWdaqFu4AishemTFxcXTpg="#;

const LAMP_CEILING: &'static str = r#"eJxtijEOgCAQBL+yoRc5kCOXnNQ2fsCOxILCwsL4frGhotgtZkbv8lScq9nJw7/RZJ1/lLULRqrkrwWytZ/kGDRiKYG4BAQ4UFu0zHA9/QDhERtm"#;

const LAMP_DESK: &'static str = r#"eJxtjTsOwkAQQ69ipfeS8Wx+0pKahgvQRVCkAIkC5fxMlCZSotHYzbNdvtNvxutafSyjocMhdOjZs6Me1VguKzGWA0ffNBJ+wt2H1KCNzzC9He0JEltaqGdNS5YGCqLmPIWjjrNwLbrt+v8SQytU"#;

const LAMP_FLOOR: &'static str = r#"eJxty6EOABAUheFXudPNsDHbJSteQLMJgiCY53eVm5RTvvPjantAj6IEMMNNCz7TSl9FQvUwIV+0gXC0/QjFVDNcHpwYWQ=="#;

const LAMP_WALL_DOWN: &'static str = r#"eJxtzaEOgCAYBOBXudGZ8oOAG5IpVoKNzUAgGJzPL6R/c+7ifbcLV7krzk3sSkHpapuGS75p6Q4RwzTqGBiZjrIvBMI8IklS8j/SYK3EDt3l5bMzj+WTF6xiIpw="#;

const LAMP_WALL_UP: &'static str = r#"eJxtjb0KgDAMhF8ldC+a9Heonbv4Am4Fhw4dHCTPb5wCIjfcwX3HlavfA87N7IjgR5wOUsvT2XSYWpa3rUUZD4gcOgHBKkIrqeUfUsAwSEFxYvoMPduoNw/Z/SLa"#;

const LAMP: &'static str = r#"eJxtjLEOgCAQQ3/lwn7INcRggswM+gNuRAcGBwfD93suupAOTfPaxqvclY7ZrIFQw+lJXPaLhs2kOLw0xa8jIEEbO0QHaIzdsVixE4PAqL6ok1OJOhpy+G8f+w0f5w=="#;

const LANDMARK: &'static str = r#"eJxtzl0KgCAMB/CrDC9QW1I9mLeJEESFetDbNz/yQcKHP9Pf5pQ17oSIh1gEJA4iAZE4kWsqtVZTVlp1u1aLe7VrpYgDbZfF4CxKa87W+4/74JyyNctxCarow/2nYXLwNl3eQfDGPTe/EfCZYQMJW2YN6Bduo0Qa"#;

const LANGUAGES: &'static str = r#"eJxtjrENgDAQA1ex0r/Im+RJEbIBQyBRfINEwf4CGihI65Ptq8d6OrY57BkFBgutDk/W6ksSNMHEQBn/eCGyKztgAl07eyRIyaJRMjR2mvefFv9cLstPLVM="#;

const LAPTOP_2: &'static str = r#"eJwti0sKgDAMRK8S5gLa6sJF28tosQVxUQImtzdBV/PhvTTqzjQkI4I0YwW12s/GGcGepx/crG4gQxbQUCdLmtwr6ep3JY32zaaHL8W3yRJ+1qnyAhjyHDU="#;

const LAPTOP: &'static str = r#"eJxVjbEKgCAYhF/laNf8L7WCcnaotV1oaAkaoudPnYobjvs4+KYr3Qf2uVlpIH7rE0GYEkXF6L8bfMZTPEy0tSCaA6idSwKpL1F6zNi62GnPH1YFLzZbmjC1xRtexQka6A=="#;

const LASSO_SELECT: &'static str = r#"eJxtT8tqAzEM/BWRu6fWw2sbtoF+QD/CbA97yKHQkkO+PtIuISEssmVkjWZG8+/4X+nn8/RdSWQUKpQ9OEmy03n+iPZ5fgHxhK4L+gRT4nirkaB3dD4YUKgR29eERnE3chLivORkMCFPLTX/SO2PMylKFNRGBXfa0u4IqiQHEoW4DXHOYHZocj9btYfdDmbYNjZZE3If0OLnISPmUnJNnL1h4X837RPesAU51gAXZIWwl5caqMnhGrQPHkbpayzZL0lQqu/WyrtYA9s1Pz3eAUYLWOM="#;

const LASSO: &'static str = r#"eJxtTTEOwkAM+4rVPeGSS0qGoxIP4BEnGBiRYOL15FrUqbIS2bITt1f/PPG4TLczVLvDURJCSjYt7TTspe2hyhVi15kDY9YoFFLuhYwNxk6RkuItBZXn5Ii0PPegXTKt/w4nOehwSHSFrr8zSIZNbbDvfvMDzDwp6A=="#;

const LAUGH: &'static str = r#"eJxli0EKwzAMBL+y6J7W62LTgu0f9AO9BTeQQCgl9OD8vrJDIBB0WK00E/K05HlALlFoBXndctEwksJ1+6fw7X8j3lGevIO33sPD6LDzcDi0zo20r2pWI4V5+gwoNsrjYihYqZuGbVFaU7ZSO6s3OmkO3Vna6T9KFS6S"#;

const LAYERS: &'static str = r#"eJxtyzEKgDAMheGrhF5AzJIl9jYihZIUdPH29mkUhJIM//A9bV7PzY2aFzv2Jc1MOKEe6EhOWaewWRG12PqtupXgjH410Jg/LzfnH78AQ4cqjA=="#;

const LAYOUT_DASHBOARD: &'static str = r#"eJxtzlEKgCAMBuCrjP8CYWUhqJcpSV9FqG5fGoalLxuMfdsvvVkC7W4NVmEGnQoD6EjV342BrHGbDQoCWnZxXcuE8pwXio0ojj0H2kokxfr3zV9/Wdydqkg8R63UBRijPT0="#;

const LAYOUT_GRID: &'static str = r#"eJxtjtEKgCAMRX9l7AciCnqZ/kxJ81UG5d+nFrjUlzHuzhmXgtsFosEF4S6TnT9ZDG4Ilz+EyxbSbUZLU8YtKalltJ+DtbdSNoDro5H2dvusiio//qwHEmo9PQ=="#;

const LAYOUT_LIST: &'static str = r#"eJx1y0EKgCAQheGrDHOBsowI1Bt0iEhp3IUMVLcvDcoCN7N43/wquJlh85ZJY4+wa2wRjnTJ+YU4zeHaBRpVxXejftGtbytkHj/VOjGB1TgKCZISxOkLQwlEV5KmzuQEe7w7bg=="#;

const LAYOUT_PANEL_LEFT: &'static str = r#"eJxtjUEKgDAMBL8S9gNSFOqh6We0mF5LQP29pgdpwEtCwuxsamVTklIPUUZYQWfdVRgRdDFm0N1ne4+AnCbjc+opey0e+ExxEPnUHzGobPmyB+8vLiA="#;

const LAYOUT_PANEL_TOP: &'static str = r#"eJxljEkOgCAQBL8y6Q8YogkegM8ocbiSSRx/L7gvp+5DVbkcByH1aEG5jAHNaRQurwdxTBOLhwUtFQmuqXxwm3WA9suZ7mrp39IX8DDv3N44tRXrLS4g"#;

const LAYOUT_TEMPLATE: &'static str = r#"eJxtjUsKwCAQQ68y5AJF+gf1Mq1UtzJQe/s6MJQKLhPeS2wOB9OdTo4OZgM9DiMol5pAMaQrssMKKtJ7Owjv7d/aG676ZlJcZ1pLqqX3oKauzp/2AuzvLiI="#;

const LAYOUT: &'static str = r#"eJw1y10KgCAQBOCrLHuB0F4S1MuUpBA9iJB7+/anngZmvom97AP6TOgRKOGKUEs760joNoROOjztGNWaKSbHRX45Xu0uMD0jx5PTP3EEDi/BUswnrdNDMB/M8/2XL2BPJpU="#;

const LEAF: &'static str = r#"eJw9jbEOAjEMQ3/Fur0hbps2lcpJ6GZ+gA0dAyMD/y/CDSdbyhA/e36e3zde1+VOIuuto0NDxBBHE240MRjYUaU6OJD3iIaq0ON4xCOTqvR4a2LQ+ljWefl3r/NcCIa7pgKKWzIpLTj11LYRE6xiGQyXw/ls+AEmyyMJ"#;

const LEAFY_GREEN: &'static str = r#"eJxdUEEOAjEI/ArxvmOhpZRk3cR49hEbPXj04P8j7SbqGkJTOnQYZn6urwfdT4erkMiNITrBm5FAbGK4KWX4FOWq0ErjSBEc740YSiUijajgZtGbNQeqIzcko/qkYK7nPSAJrtR+2rlnjSLpuuuN/y1zjMz2R78xjyFfNV1HaV2T6iWjuBMrilBBSXGvMO6FEDf0bVwqscNKZ09cujbL4Us4c1jmY7dqmXeGERvZB3sD0LZBkw=="#;

const LIBRARY: &'static str = r#"eJxtybEJACEQBMBWFht4Vo7D4N4OvogHAxPBQK5+MdDIdMb6PyrKGxoVCgElZHuWZtv3MUL9OgnJGS8hEKeemPErHVU="#;

const LIFE_BUOY: &'static str = r#"eJx9j8sKgCAQRX9lcK/lA0pQ/yUsKCgIaVF/n46CLcLNncU9XOYYvwW/L+AfS7gg4O98Qzw9cabLvTPndK0wW3IopiWUEAojcan/UFyxUYJmfECCNrGcrTkcQoz+YMUh/qyqQTGqCi817T0Q"#;

const LIGATURE: &'static str = r#"eJx1yzEKgDAMheGrBPfG5BklQ+0NXN1Fh44O4vltF0Gow4MHP188tyvTMXeLE2T1XQIYpOzByMqURxICOzsN5aNLsa8oxZdOpMjWCGq14PYmgvyhT3kAImAq+Q=="#;

const LIGHTBULB_OFF: &'static str = r#"eJxtTkEKwzAM+4roPVrjNE4KWX+QR4TtkMtgh/2fOYHt0mLL2AhJLu/26Xjel+qVGd5THsyOO2xzMmegNIViHeV2BheXo9yG8ig//UtgtVqfuaoMMLSNmjDH9BruiCMxMdkRDHFCeBFR7anc9YLwlit9+zNfFTMz5w=="#;

const LIGHTBULB: &'static str = r#"eJxtTTsKgDAMvcrDvbGJtlWoggfwEFKHjg7eH9NSOkkI4eX94nO9Gfc2nOzAcyIxDAqGKYDJGSF9G1obkHoncoeHh63jsSQLdQmaACqgUBOmistq0rDHsfTtsbdq8JL9D8EWInnuzAfmLiZs"#;

const LINE_CHART: &'static str =
    r#"eJw9irEJACAMwF4p7iJFBYXaDzxCcOgiOIj3WwclWxKabQn0YqoHvzEJJsPkrmV6bWCGbCNEGxQd/3IAMF0Q5A=="#;

const LINK_2_OFF: &'static str = r#"eJxFyjEKgDAMheGrPHIBTbFEoQpuLh5CUFAQcXCwt7ex1ZLh54XPndO1Ym5pbMAySG9hUYZjCIQ6Vyjo3M/YQlYzZVahzmzfjgU3t1QTfAgbwm1ifWywqpLVp1EUcLI6/Ts/+gA/qCjz"#;

const LINK_2: &'static str = r#"eJxNyrEKgDAMBNBfObKLplCj0BbcXPyIgkIFEQeH+vemCiIHOY48d8QzYfY09WAZZbCwaDQMgSRDwdWFBPdBtuURX8gavU2qfnRb9wXZeOKWcLG2IWTtTqd5ptqiwg38ax+z"#;

const LINK: &'static str = r#"eJxVjbENwCAMBFd50ZvYAcsNYYMMgZQiBUWK7K8ATUC2rnjp79NT3hvX4U5hSCgKBY8zr7F9DTSlZJ5toJJ429EgLqeta3L6ZREiS00jdRsFrCNsA7WLaLF9iJQicg=="#;

const LINKEDIN: &'static str = r#"eJxNjN0KwyAMhV8l5F5WpdgNjG+wh5BUFmEXQ6Rr9/QzDsoI5OTnOye8UhNYCe/WwzV58DD1sl39toiZN7MkB25cJ+OMzucG7mT+nMZ/MIaLJsdQMzd4l7UJ4YwguTykEVqHsBP2fhDeFFcwBi6Vnxl4HzQfQ6qCHfk94xdN7i3e"#;

const LIST_CHECKS: &'static str =
    r#"eJx9ybENABAQAMBVPnqRR1A8GxhCotBIFPYPui+QK49GmQ1qFN0AetCblVYkUicSsf5sRgOuhXugfk5gswAJAiWx"#;

const LIST_END: &'static str = r#"eJxtzbEKgDAMBNBfOboXexGDQ+zs4kcUHLoIDuL32ywiWG4JeQdnZ7kq9iVsVFDWMWQb/JftK9qHBM5dEUJvpiIQpBbGdtU4/auH76qz5/UHXPgoUg=="#;

const LIST_FILTER: &'static str =
    r#"eJxtybENABAQBdBVLhbgIyjObWAIieJKhf0jmqu07/GeR2l1NxIVRXPC/pGwRSVERfgMAqFptrlXJRXR"#;

const LIST_MINUS: &'static str =
    r#"eJx1zCEKACAMQNGrDLvIFIZhLls8hGBYNHh/ZEWL1v/g8+xLYRTXEAFjTU44WBO+QkA/wPyUaDP1dGgD8Zsc8Q=="#;

const LIST_MUSIC: &'static str = r#"eJxti7EKgDAMRH8ldFebSEOG2NnF1cGt4ODo4P9jSkAcwnEcd4/Tuz0XnEvaCAHLzqnq1LeqH0EZC6A0sujOgOY8ePPFVY7oToC0zhFh4BjYRX7kBYkpKn0="#;

const LIST_ORDERED: &'static str = r#"eJxti80KAjEMhF8l5B7txFp6aPfsxYcQFSqICIqsb2/jsr/sIQyT75t0vz2u1GpmBdMXmQNTWwOuVrXapK1JTerUnlkqj1P994U8hZHn4zjKz9O70CXz0VMo+Hgj9psRuKIrIBDiwZ+dgFS03u4lEGz2ooLB/wH3GjuV"#;

const LIST_PLUS: &'static str = r#"eJx1zSEKACAMQNGrDLvIFIbCNFs8hGAwGsTzi2ll1v/C59X3hJFNQwT0NZjC7rXCIgT0A4y6REiHFPDvMq3QBSuRJC4="#;

const LIST_RESTART: &'static str = r#"eJx1jL0OgCAMhF+lYQcpLf4kyOygD0F0YHQwPr+FQRdMc23vvuTCma4Mx6w2h9AvpGLoShTDCwZA9wfGJkAnJHnwYOtMmoCNr6qJlq9o12iIxDvjxbImwwgoa0W53OqW3N6cP/YAIHQwew=="#;

const LIST_START: &'static str = r#"eJxtzLEKgDAMBNBfCd2LzYklQ+zs4upecOgiOPj/mCyCtIQsebnTuz6NzjXsnImxzaHo5LeiP5GxJMpDAFvkkAoCJZ+IiBaX/vWydnEl249fMZwoMw=="#;

const LIST_TODO: &'static str = r#"eJx1ykEKgCAUBNCrDH8fYZa5UG/QISKl7yIIEarbp6taFAMDwxuTwpKRTkuCUFoSOMSVsyVFuCwNhCP6zHU709a7M/ucGd7SJiFGdCV901eu8PAkJBTrbxDdr+iX3DzULPw="#;

const LIST_TREE: &'static str = r#"eJx1izEOgCAQBL9yoRfZw1wwOaltfITR4koLw/sJHQVku50Z/e7f6D3cxSCwLcllXduZtUdyTgDSuIkkZXsCwcPvxG0Whx5CkalYAccUKro="#;

const LIST_VIDEO: &'static str = r#"eJxtyzEOQBEQBNCrbPTyg/yNYqk1LqCTKDQShTi/1dAoppk3Qz2PCsWJqDQoHYzw9O3O0xUEfANf7EsaXxh/MJIzJaazWfatHtc="#;

const LIST_X: &'static str = r#"eJx1ybEKgCAUBdBfubhH3TIpeDm39BGBg4vg4P+jTjroeo7EP3m4R30kuL+HsrJWs9LGwMyC12gCb3BbNPToznLoLwOnqSR+"#;

const LIST: &'static str = r#"eJyFjksOgDAIRK/ScAAVmhgXbW/jwsS4treXTzCVjSsC82aYch7XnjpWWCHdPDYeVIEQUie5tjIL04qSo4b0WiSA1y+rjItDrnkCaw3spRLZDHlaMHYIqn/3Uv8tzGY5Tj/e6UJz"#;

const LOADER_2: &'static str =
    r#"eJyzKUgsyVBIsVXyNTJUMDRKtFSwVDBQADJ1zfSMDC11LfRMzZTsbPRByuwA+mYKxg=="#;

const LOADER: &'static str = r#"eJx1j0EOgCAMBL9ieADaakAT9DceTIxn+b3QVkTQ0zZltuy6fTvWxuOsjGrOIIBBgdUHRbW4NkKLI/RmogWFgTF5CjgsBz31jPEUD1htDZ+gqTaB0ThIoEl3Vv7hrU/b2ijJ7vTUhsrVLTBjfN58LGB6lEBfNeA/Tpb4sWWNXvUvoJVg4w=="#;

const LOCATE_FIXED: &'static str = r#"eJxtjkEKwCAMBL8S8oGipZSC+pngQZAePCW/bzUKrXhayM4OcTndEcR4NBaB36xhPR4IYts1uK1Cwf3QXraJuXRjZ5g7JOoV9bJZeHm88JlUr0xeSoVyBBLFiscTgXgYtV5h+wJ7ABrPQhs="#;

const LOCATE_OFF: &'static str = r#"eJx1kMEKAjEMRH8l9N64k7ptA7tePPsRooKCiAcP69/bdNVdFQkkdHgzIe3Op8uBBvROHA3Su9bRvQyU5x11rrqFQauuok/xDZkVOnrlGx5kgtsx8GX5hw6YLdfROs+9bm9H2vdukxgga+uWc6DMQaklNAwbsmsocE6lIVCqBVYyNcKbY8lZvZQAC7fYWTgyJxACa9wx1HMMLOpR2jg1UuNtgbcFPtXimIoKDkWrTGGn8PeZIh9fIT9HPgDDwl6W"#;

const LOCATE: &'static str = r#"eJyNjUEKwCAMBL8S8oESQUpB/UzwIEgPnvT3NUbbU8HTkuzsrsvpjtCMRzIItavtQh771Wh8gzsECm6gghgxV0Sg6w+mr5dm48zaDXT06t4LcyqcIxSPJwJXhbmtcbXDA9ZgNw8="#;

const LOCK: &'static str = r#"eJwli0EKgCAURK8y/AvUr8QW6g3atpeUdBcilbfPT8ziwcwbU+JR8eRQkyVeCa2DCa+lmVA6JkKK+Uz1H0qTyplBjs5cviYES5sG8669gsLYw+COexFTHPcB9Bgb1Q=="#;

const LOG_IN: &'static str = r#"eJxNzEEKgDAMBMCvLLmLJipe2v7ARwgKFUQLiujvXYsHyWGXMIlLwxExeum1RR2bwWCoOMq0U3+Lgi0WjQRXvkfBpW25l3mdkLZ5PXYvStWBj9TA3mX6oeAyvcxLLbiUuhXcbxrTctJn+wBMUybL"#;

const LOG_OUT: &'static str = r#"eJxNjLEKg0AQRH9l2F6SGZKIcHd1mrTphQQURA+00L93PSxki7cDbybkdunwi/ZpIL6frSDc/Vip0veS4bl7WAq3o5JCnoZt6Mc/8tSPyxyNL7D2EVDwvy7qKaVQ1JXRRMOqaI1hc1BOFrpf3B0DcSZY"#;

const LOLLIPOP: &'static str = r#"eJxFjF0KgCAQhK+y7Hs/YxI+qDfoEGJBQUFID3X71IVimF2GbxgbtxT3heLtGGCKj/zk2LC3nWBvz3CtNDs+FEih0e1QXBqF/HwCCAiKFPVVulpLaky+Y5Yw5NI38QLGhiFR"#;

const LUGGAGE: &'static str = r#"eJxtjk0KwjAQha/ymH1x5hFKFklv4Lb7EoUILqSI6O1NFNq0lFm9v48Jj+mZcYly7kHNOhGElrOOHUe/ahSdja0BvmyzALPKEE4VOoQF7WF+dDuU25PcwdK0ftUk6Tan+xXpE4UqSO8o1gvmImvpHy+1n70p+6b1BTRYPtE="#;

const M_SQUARE: &'static str = r#"eJwli80KgDAMg1+l9C4yHdLDtjfw6l3csAMPMoo/b+/qCISE5HMlbQKc8s7i0RDCnaNwi4/HEeH9vdQyYHC9AsGdqzBEjzOBmRY6LFR19iK96Bg+kZEY9g=="#;

const MAGNET: &'static str = r#"eJxtjUEKgDAMBL+yeE81TdoqVF/QTxQ8eFDw4P+xLSIIkjCXnWTjma8N69wdHuxISeFNcFQQcjBhQsNQhsF1Ewus3R9VpqKKz9awoqGqAwlJqh+7Jfa1YolvkcMIhf4kbMvFJ7sBFZIlMA=="#;

const MAIL_CHECK: &'static str = r#"eJxtzLEKwzAMBNBfObJLta6unYCbuUO7djftkKXQIeT7YyckZAgCgbjTS/88DvjemxcJu75DJghXRyh8+OMNTsaPg6lphxpwaJs+XSrSp436FSpKq13ETWMuXY9lVcaE6gLcs3ROXi3AVtqL3/MZZ4cmtA=="#;

const MAIL_MINUS: &'static str = r#"eJxljLEKhDAYg18luPe//kFbCz3nG8719nIOXQQH8fltBUWQQCAk+eKS1ozp3YwktPu5RBC2ytDw094zuCn/FioqAbVg7pshvipkiCdqLihvegkenfhUti0Oqxg1FOtgv2XzvI7qoCG7q9kBi08l2w=="#;

const MAIL_OPEN: &'static str = r#"eJxNjbEKAjEQRH9luD7j7prL7UG82kJbC7ughcUJFuL3m4hKWFhmhhlefpTnDdfdcDSlwRkvHLl1OueJDmV6qRSDQeppqGofex/s1BdAD3W0ekhdaIyQ1ZHOw5I3jbnkH/luBpXQgBg5FeUc8XlfAiVBDq30H78BVKQoIA=="#;

const MAIL_PLUS: &'static str = r#"eJxtjMEKwjAQRH9l6D1rdqybBmLPHuzVe9BDLoIH6febCKWFloWF4c289Mnfgte1m0jo+WGZIHw7R8dbv83grHx6qKhENMAydGM6NcmYFtW7qoIbJAZcJOTa7fF/TaOO4g3+Xjv76aQRarMdEYPGspIfyCktRQ=="#;

const MAIL_QUESTION: &'static str = r#"eJxtjs0KwkAMhF8leN8xme72B2rPHuzV+1IPvQgepM9vUlAEJUxImMlHxkd9rnI7HWZSTFGubaVQNCox8Zy/d+FmXFQMhkHC4GpEOUzjMUjT+ObdndelHkMnBV31eJa9BckSoa3oxTO/p7P1YgXsFzAhoyQ4J1klTEI7wqcWeUETCQ8UxzdhoIk/Q3/QVCE3qH28F1Y8Opo="#;

const MAIL_SEARCH: &'static str = r#"eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb2+74T9GSOCR935Jcwv3AcdW9SJgIbt3QSAwubRo2Za/GjKxRAMmphp5IYMnq7pmk0Fd88ZdE87rimoPSz4kd4l5ZBBrIeNgdsmzjvZcQTgUKLJ1vuuwqKXcZA7fWDyP8XJCfLaKK4WxVYVCfMwqmZb1/2Mimsnm/mBeg2o/Mg=="#;

const MAIL_WARNING: &'static str = r#"eJxtjLEKwzAMBX/lkd2q9LDjBtzMHZq1u2mHLIUOxd9fO5AQSBAIxJ0uffNvxvvWTSRMJTz7TBDaxtHx7vc3WIwvhYnJgAY4GyV0Y7q00pjW3qf2orvKEBEk5qp7LKuVzFG0hz6qc3ydWA1f/Dkhi6ht7A/ZAy6D"#;

const MAIL_X: &'static str = r#"eJxtjLEKwzAMRH/lyC7VurpxDW7mDu3a3bRDlkKGkO+PHAhkCBIC8e5emeo84vfo3iTs+ukrQYQ2QuEzHn9wMX4DTE0zGuCYu6FcmmQou+rvqiR3zQk3TdWzEdtpGhNq6BFenjmpWoJvRDzTmjM5shVquS2S"#;

const MAIL: &'static str = r#"eJwli0EKgCAURK8y/H2mUlmgnqBLSEq6CEKE6vZpMTAzi/d0DlvBbUgS8j9X8iXWxwkxpD0WQ2IiPIYGsrpvgtWnKxHe0CElVDezRWFkygm2DPiK14hOMj6Br5VpapPsC+aMHQM="#;

const MAILBOX: &'static str = r#"eJxdjsEKgzAMhl/lp+fZmdBqBevFi5ddPewm22CCaGEi+vaL3cbGCPxJ4CNfytDNd1y9OjGD8o7BSKUokakxv3vCbaFtzchhYJFpC9uQu7DeEQPSTtIs7qyq8rjfrcowDdvQjzeEqR/nh1dkDwXIvYIokm9G6M8v8XZdiIVIdBJibv/fW9KvKEpW9ipXWMmrTGGTjVLpFLug0fIEW5w64A=="#;

const MAILS: &'static str = r#"eJxFi0sKwzAQQ68iZu9pZpzGLtg5QXuI4oQ6i0IJpp/b184iQQuJh15Y51SQ5+WRSySxhG+kgfBZppIrqHOtRAm/SD2N4dSEMbzuJWOK9FSFM44Flp1Phs+OrRFWu5VHdx3gmteMw7sp/FskdRAWvkBbsvT78w9AcSdF"#;

const MAP_PIN_OFF: &'static str = r#"eJxtjt0KwjAMhV8l9L6xTX+WwhzsAXyIoYKCiBdebG9v0k4HMgpfe8LJR/vX9L7B5WhOCWMAxcjoMlS4eiJ4d3YgM/BUMVHBEqGxlZJNZugPqhv6n9QX9LIQMJFos2rzV0tOvBOLsGbLlqFDXtFmuohMO2YRB1CMhB1DRfMW1QYIa9TXfwW5s7K69+GIRbSUNoMl9EmRt/7j/rzC4o+GDCwkl9xzi3OLUtXS8AEd/00v"#;

const MAP_PIN: &'static str = r#"eJwlizEKgDAQBL+yXB+8iyBXJPmBH7ALp6BgIdFCf2+iLCwMuxOOfK2YI42eIWyMwSnEf3U6dRWd+KxQcI1ABvBEKXTNTMG2YvsCeyIJE0qknmB3Jd9O/5xe/vgZuQ=="#;

const MAP: &'static str = r#"eJwtjNEJwCAMRFcJWaBEURTUbUoRRIX2Q7dvov4kl8u7C72V+bQKveX6vRE1WPCggQwLRax4kJObhRcpFqZwnWgKJdcbhoroESZxBy++yCEMEpdhYQ4piN0vMriCsiWyixf7A+l2Jtc="#;

const MARTINI: &'static str = r#"eJxlyTEKwCAQBMCvLPZH2EvhBS7+IB9IF0hhI1iI7xcbG9sZr1/L+O/wGFSzheTHpOQrqCA7uVfhhVMiTKLYu34A7k8XQw=="#;

const MAXIMIZE_2: &'static str = r#"eJxlzEEKgCAQheGrDHOBHM2FoN6mhSAq1EJvnzYlRZv3bz6eLTm2GNIGJYd07A5JgwJJPAa9XR7i7Q+bKRWQ/uILVtkfV4Q2KnrJoUKoPZKGf9l228qmseELMe0JLNYxPw=="#;

const MAXIMIZE: &'static str = r#"eJxtzKEOgDAMBNBfaeYb6DUsE2Mag8UvQVQiyL6fzQBLlhN34uXilW+jc3V7IN2WDALNLVxXUZfi1ESKr4NQODrIMB5JJfFFP1kbNoLiCWKdZJTf5wORMylt"#;

const MEDAL: &'static str = r#"eJxtjlEKwkAMRK8y7P9GE9vdFNqC//UQZRVWqCBFRG9vasEWlUACmZfM1Nf+lnFs3CGSMLiEUAiIxEUvEGytGMQ7LyRdQYXpul+UAMksK5QpkA7WDfz6Md1Kx4FiZUaurTeTeVt/IrCdC0qyZuQvcOHdDKj6ivTPB0XMq306j2k4IT0bx9EhPWyKw9i4t/0srwMIWO9esqcl3wsvmkZh"#;

const MEGAPHONE_OFF: &'static str = r#"eJxtzMEKwkAMBNBfGfa+sZNs1wrb/QOv3gsKCot4EGn/3rSIXrxMmPCS8pieV5zHcDyIZmxhIF/WmIQJ5hFq2a2ulq9Wgr1YOuUW92KESmd/HCkZzDJM5n87+FnsZYi+/ul2u18w6xhUA2b6DFg+ddmq0xXVN+EpKyQ="#;

const MEGAPHONE: &'static str = r#"eJw9yzEOgCAQRNGrTOgXHVcIBXICPQSJhY2JhaHw9K6FlDN5P1/1PrAv7lSQYJLQOK025ib6uJKHT5T8u430EYw+VYVihEUSfBK7u34By7AVog=="#;

const MEH: &'static str = r#"eJxdjEEKwCAMBL8S8oC2CpYGrJ8JHgTpwZP+volpofQ0uM4kcmlcM3A/0XkEHsYm2DDF1f5TrOXKMJzMAaELD4GX544w/JzFVutx1SGcCZlKy+ZMpp9r20z0/KdxQaPXvgGxByzD"#;

const MEMORY_STICK: &'static str = r#"eJx1zjEKgDAMBdCrBPdok2pVqM4urg5uBYeODtLz2zqUCg1Zkv/4EHu7x8O1NLsBmgPqZrVdylabhZRMvUyTSFHomGsdIwpLEmHwrKoyOgYGFYeAkT2ZMgAO1FJO0uh20uNBRQ/jtvXljRxw+Pfw6535hxcV3lbG"#;

const MENU_SQUARE: &'static str = r#"eJx1y7ENgDAMRNFVLC8ACRKksLMBQyAS4XQosgRsD4YGCtr/7qjmWWFn7BC2klQYXUA47iC5LKJPqdfGY6TGDpHWSQUS4zhAENcaWPqA87/Sv+UEuSElUg=="#;

const MENU: &'static str = r#"eJxdjDsKACEMBa8ScoFdg4hF9DYWgljr7U38FVYDj5nHJdcE3QQ0hNCEVkAB6UfoNOfIn1qRp7sdTdxS3C0ec9+cQt/8KoXHHV9BIKQ="#;

const MERGE: &'static str = r#"eJxlizEOgCAMRa/SsBdpLcJQOYEegsTBhcTBeH7LoAt523v/61XvE47VtQwLCAoYrujUfdGv7sTADwU/174IBiF5SqZ9TrwJMI+3xsE8Rox/ewFPZhxV"#;

const MESSAGE_CIRCLE: &'static str =
    r#"eJwBNgDJ/zxwYXRoIGQ9Im0zIDIxIDEuOS01LjdhOC41IDguNSAwIDEgMSAzLjggMy44eiI+PC9wYXRoPmBODLo="#;

const MESSAGE_SQUARE_DASHED: &'static str = r#"eJx1jjsOwjAQRK8ySr8ms3YIlkxuQEuPQrENEgXy+Vk30Gyaad782vvxMTyv0y3jfF/2WZiYqihU1HTa2mk4tvbzkciWI3BxwN3zmKFI1SUqUKL2A0B/AG+QkR4nTBhNFXA1CV68VidSULosQTCD2uU//gWc+UPs"#;

const MESSAGE_SQUARE_PLUS: &'static str = r#"eJxNzLEKgDAMBNBfObIXTayIUJ1dXN0FhQpFHBysX29LK5QMyR2PmGu9LbaBZmFwuwoEdRhW4Zo6pzT0UtQQJZZ1WUBeGk0VH43GHecOLwNxTXjibgmecw67jzaq0jbZSrJdoiH+9gO0Kiez"#;

const MESSAGE_SQUARE: &'static str = r#"eJxNjCEOwCAQBL+ywV/a21xTQ9E1tfUkCAQCgeL1gCNjJiPG19gy0uM+KvSKBHFOVKa9dxGD/VsGhVltD2B3wR9rFAbHDxGz"#;

const MESSAGES_SQUARE: &'static str = r#"eJxdjbEOgCAQQ3/lwn7IXU4CCTK7uDq4ER0YHBwM3y8sYkyHNk3zGq50ZzgmtZCATwwMpoqwptmeKCCr7AZJk/atRM6uz6pzGTcVw9BAMXScA5/5tySqRJSM9vuEXJBexAOz9SIC"#;

const MIC_2: &'static str = r#"eJwly00KgCAUBOCrDG+f9exHBfUEXUIsKCgIaVG3T20x32Zm7BXuDYujkyV0Y0Q3IDMFKbREpQPn9MLoyswTWJK3bbl6G/cUjxXxccSKkByNhPg6UmXzt/4D8VMZlw=="#;

const MIC_OFF: &'static str = r#"eJxtj90KwjAMhV/l0PvVJbGuhW7gA3jrfUFBQcQLKdvb278xL0YgIeQ7kM+/nu87Fh4Vs8JCaSrMbZ3LOvlDhib/Cd8HbqO6kNXWgUSznAdNjNL6UpQOHLsSy4G/mAH1kcOAYWUZZocjA6fleDVBIBXtjD7ZjrTIDu/gomwsTP6GU9vYapl0yFU9anp5NvvV8wf1uUKz"#;

const MIC: &'static str = r#"eJxNjMsKgCAQRX/l4l5yxkoC8w/atmgnFBREtAipv09tYczizOPMtae/Vsy9GIjBXkNDpZKxC6bMaKHG5n+XehLOVunf2ZLSgVRgb2CySJJqqCC5uPt2LLi5F8QCN318ErvIuOcsJ829k28l4Q=="#;

const MICROSCOPE: &'static str = r#"eJxtjbEKwzAQQ39FZD8aycZJwM3cpWuGboYON3Yo+f7YBEIGDzecnsTLv/J3fJ/DO4Gzz8OaHy1a8wUCJGePMFZUJkwYwXqjMbqx01xQifpARVBbgybTljzt8Z5Bn55cSFsoPM3NbXwt9x/cwzU8ABDWOw0="#;

const MICROWAVE: &'static str = r#"eJxtjFsKgCAQRbcy3A2U0cNA20GLiIr0L0Ssdp8PwoK+hplzzgizzpYOvVglUZWg0w/QJVGDTFrUqjdlJViDQRQhGETMvMXxxDz5LPtd/NbmaJ+sokViZJy46wIIpxdoifWu+gG++JAb8UA0fQ=="#;

const MILESTONE: &'static str = r#"eJxtTKsOgDAM/JVmfkBbRibKNAJ+ALcEMYFAkH4/q2FmOXF3uYc8+S1wre7ACMsWMgHBZPBVKTdfmQryPXsewm7t0yUZbZ+kvRAga+wnrPwHH/jPHto="#;

const MILK_OFF: &'static str = r#"eJxtTssKg0AM/JWwd1OTfYMK/QCvvS+0YEFKD0X075vVZe2hLJlJJslsunf6THDv1RiAp6CG7pKVoat6BF4ItdEjWUkZfYjJgIF2f+g8AyNTnNFZhzGY2qXazRakafQYQGJukDggRb6eTh6oFR++cZtkqaiSTe63bnhp9J87Zd0mh8bDDsf/VjCXXPDw0Gi0zXHazM/XA1bqFSvYWEh4LbztsozmoeELlFtEwg=="#;

const MILK: &'static str = r#"eJxtjsEKwzAIhl9Fek8WxSQKWWEPsGvvgR1y2GGH0eefXdrSQhF+9Fc/LZ/6bfC6D08BajKM5bY4Y9l9BZrJZ9HKwBAs0PmUCcgT6tvymLwKP7Z2gAwYrEkThWpzq2tZS8fa0ez0RA5/sjuQ3QmNgBHYVmSii1ftbqzJsz3Xte9E06Xu0o+btwN+z7Y4xQ=="#;

const MINIMIZE_2: &'static str = r#"eJxlzF0KwCAIB/CriBdYWo/VbfYQRAXbw7r9tG3sC0QRf399q7nnVGZoNZV1CeiApMzZ2WD004Wi/3E2N5bu3nzQzgEtwkYBycmUlQmh6z7ef6wiPSrSkD0yD7sDwqwx7Q=="#;

const MINIMIZE: &'static str = r#"eJxtyqEOgDAMhOFXaeYb6DVZJsY0BotfgqhEkD0/xQBLlhP3iy+f9TI6lrAl0qYVBJp9wl6rhpKnR5T8Oggl404y9pFUkmg/6I82ghIJ0rijDPvoDZPFKW0="#;

const MINUS_CIRCLE: &'static str = r#"eJwlybsJACAMANFVQhbwU1nEbOAQEgUFCxEL3V7F6uAeSR3SMsjyaCyC7N9xo5FJfWfqcRZIHoMDY4t79BYfYIASZQ=="#;

const MINUS_SQUARE: &'static str = r#"eJwdi8ENgDAMA1eJvABq+fSRZAOGQLQi/aEqErA9NB/L9ul4tMPpEaygN3L8I4Os9dNckAro7tUtqvIyBeVrd6Mq2AqlbAHmpR+OIhat"#;

const MINUS: &'static str = r#"eJyzKUgsyVBIsVXyNVUwNMowNFGys9EHidkBAGN3B1U="#;

const MONITOR_CHECK: &'static str = r#"eJxNjEsKgDAQQ68SZi86VVCh7Q08hKg4LgQpxc/tbRWLZBHCI09vvReMhtYWXECFVFlFVucRWO2mweM0pAjurctQSTiW0UvYBUGmZRZviJ9bPFj9WTtW4Hr/CRNpoFiaBG4PBSYH"#;

const MONITOR_DOT: &'static str = r#"eJxtjLsKgDAQBH9luV70zjck1ja29hKFCBYiIvr3JgiaQrbZYWCUmTezTDCnJq4J5tJUEDZNKTUqfmyj1mG3GDV1ImA50kEgSNw4cq/NQo6kzz+GY1v7lm8EJXal8sh+TAVhW73iBm7pKh4="#;

const MONITOR_DOWN: &'static str = r#"eJxljT0KgDAUg68S3i76WkWH1hu4uouKz0EQKf7cXqtoB8mUfCQxc+MEnaWKFVjXOZUm9llpXjJxBk4iDR1dCnzpW4dlt6QI0o+DOEucEp7ksKQJ29g5uXzia74QZu/DfE3/h1UBxVJ84ASO+C2R"#;

const MONITOR_OFF: &'static str = r#"eJxtjDEOgCAMRa/SuKO0oQETZGZxZSc6sJg4GM9vu7BI2g7N++/Huz4Nzm3a0QP67CoBgZVBQ4YKH9bgzPrpTSkuaqTYPRLAhbtn1cvrIBmAsIUBQKnwr/uTS0vJynb2ATplLWw="#;

const MONITOR_PAUSE: &'static str = r#"eJxtjU0KgCAQha8yzAVqVNCFeoO27SOlcRci/dy+JKiIdo/38b1n56EwBIcdtUCy1+htUztvH6I+JMexwOZQIHBMExeHpBDyVa0pFD5Ti7A7lFWrwntQAOlF/VwZEMTmBgfPaCyL"#;

const MONITOR_PLAY: &'static str = r#"eJxFjDsKgDAQRK8ybC+6UdEiyQ28gJ2ouBaChODn9iaIsZhh4DFP74MXTIY2LtCgRpmF9GR1HonVbh49ZF4X8Ya4ItyGSoK7DCnC2+c6eQmriLd4sPrTdqzAzVH9wkRaKJY2gQdNeSZj"#;

const MONITOR_SMARTPHONE: &'static str = r#"eJxtjL0OgzAMhF/l5D00MW0IUpK5S9fuqKCarUIRbd8eZ4IBefh8up/4GYpgTPRwAeHpBwbD1jNs+H49avDa7VrJEijHS53IcR+ycP1q2qb3aBt3O4l0mpCDsUyvgu88FknkCb9ETvFXMEGm+S1Ff0tY1OLaq428ARxEMGQ="#;

const MONITOR_SPEAKER: &'static str = r#"eJxNTcsKwyAQ/JVh701V1KSgnnPpRwQjVeihiPTx910p1LCHmdnZmXWPrWXsnq5mMlBiXSi4c18G97fkjEuehBxWTbEhp3LLzZO0hFfZW2YqCG8GRfh40oTKSvVcTxwqF9hVbwoKos+J2dMOzaiyHv9iqfGeEHv3TIhcLg23M/Sjnx2+a782zA=="#;

const MONITOR_STOP: &'static str = r#"eJxNi1sKgCAQRbcy3A2UFj1A3UGLiJTGvxDpsfsYIenvcs49JoUt02Mxgm6LGXRFn9liAHGIO2eZzjTyc6a8P6760mhI39VSt6BUeM2ONTN5i0VpUuPZixH2MxNpxVMVLxIYK24="#;

const MONITOR_UP: &'static str = r#"eJxtzb0Kg0AQBOBXGaYPce8EDdz5BmnTBxXXQhA5/Hl7T0EFkS22+JgZ1/+DovLsPpAE9mURj4V7b1C4g79iIPaXXTLUZcDUVkE9TULM8RGLpyW0bhsNnpISww4xtgVuhdmYPkzlMKL5CStSLi1K"#;

const MONITOR_X: &'static str = r#"eJxlzV0KgzAQBOCrDPNe200jtZB4Aw9Rqrg+CCLBn9sbRRSRfVn2Y3Zc9wuK0rMVm6QQk6SPOMzdc5XcHf7dGRfuq3/A2JRBPc2L0KqpNXiKJfop3ojZ801se4ytgfNrIQbyGey9r8hgRLMDFkXELo4="#;

const MONITOR: &'static str = r#"eJxVi0EOgCAMBL/S7Ae01agH8DNKhMR4ICTi7y1w0Uu3m50x0W2JvAuHTxY8gh6LARSzhYDavcOevH49VtMVYTVnuBxlUWVSii0WNbUKa3JNZQv1ZaWNPDendvnBLwqvJas="#;

const MOON_STAR: &'static str = r#"eJxtizEKwCAQBL+y2B/xNBEOLv4gH0gnpLBMIb5fFMFGppwZ/VPJ+G7zsINPAQF2IBMLBpOQvCbq0euo6xH4em6EY1yZlmkXJhpl"#;

const MOON: &'static str =
    r#"eJwBNADL/zxwYXRoIGQ9Ik0xMiAzYTYgNiAwIDAgMCA5IDkgOSA5IDAgMSAxLTktOVoiPjwvcGF0aD481Avf"#;

const MORE_HORIZONTAL: &'static str = r#"eJxlzLEJACAMBMBVQhYQBQshukywEKysdHslKgGtQvj7Jy6NawYeEa1DaOsgcJcvkdlxoocJCIf/TFc83o6qCYxoIPg="#;

const MORE_VERTICAL: &'static str =
    r#"eJx9zLEJACAMRNFVJAuIgoUQs0ywEKxS6faGiI2C1S/ucchNuFcnBQI4npqoHVZCv2fCh6WPspsMR99sAY1rIPg="#;

const MOUNTAIN_SNOW: &'static str = r#"eJw9zLEOgzAQA9Bfsdhxm8sduUgpc4f2I6oysCAxMPH1HANslv3ktv62GdOrWxwZCof1BkOyt3yi2ruxPU4ztkt+lUkD8Ol/4SB9ohUYRSNpRqEPVIGwBGPV2LRC4JTMVO/HAy6pHFg="#;

const MOUNTAIN: &'static str =
    r#"eJyzKUgsyVBIsVXKtVAwVjBRsFAw1TVVMFUwNPUw8gEKVSnZ2eiD1NgBAOC1CoQ="#;

const MOUSE_POINTER_2: &'static str =
    r#"eJwBMgDN/zxwYXRoIGQ9Im00IDQgNy4wNyAxNyAyLjUxLTcuMzlMMjEgMTEuMDd6Ij48L3BhdGg+MmsMEA=="#;

const MOUSE_POINTER_CLICK: &'static str = r#"eJxtTjEOwzAI/ArKbhqwASO5eUHziEodOjRSh059fbEtZepyoLvjuPa+f57wuC6Hg4MAMRCalSTIrDcmoBKCf5etXbp3a+cFKa4WhjkKcskT/3gNqVZg5OwRb7FVt12QsoKhq7xSp2rq6k4ZXSJrHTQxw8AjWqkYdNQhTP789wOqoS9m"#;

const MOUSE_POINTER_SQUARE_DASHED: &'static str = r#"eJx1TjkOg0AM/MqIfklmdkmEtOEF4QN0SBTbIFEg3o9pgMLILqw5nZdxLZh+Vd8gjoLwPibYVXX5dbBdPjVsbyLCFc0UbBNokvobUh3/MugzOIENxCswKHitVlro4mJx30wPjoh283DRCLkGpptjB6mwUE4="#;

const MOUSE_POINTER_SQUARE: &'static str = r#"eJxVjDEKgDAUQ68Surf2x1qX2tlBVwe3gkMXwUE8v+2gIgmEhPDCkc6MbVAzBSJLlwjCVmlqjr8OXuK+oSSzVzE0FRLDg9qFKHYQCzG9dqadWCa/vt8bV2sbew=="#;

const MOUSE_POINTER: &'static str = r#"eJw9i0EKgDAMBL+y9N5oDCYUYl/gJwQPXgQPnvr6NpfCsrDsjH/X/+A+0isQGK0GViqGjXbORlIQlWOeA2mp+hJO9WmyYESh8+t7GRUe"#;

const MOUSE: &'static str = r#"eJwVi8sNgCAQBVt5eQ2oBPXC0oFFGCEuN0M2froXTpPMZELNh+EpyVQ4eeIVzoTmcqoJ3Uh8DURtYWUMQx9iuHZTJOE2OSy376Gr+AOiEhaw"#;

const MOVE_3_D: &'static str = r#"eJxtyTEKgDAQRNGrDOkXGZcMCmtu4CEEizSChXh+TSFYhN/9F+d2VexLWjP8piqVSgztlvjsyOAMWY9GCG6Ot45yAtXM/v4Acckfcw=="#;

const MOVE_DIAGONAL_2: &'static str = r#"eJxlzTEKgDAMheGrPHIBidJBiL2NQ6G0BR3M7W2jHUqX/Bk+eFJy1BjSiZJDuq+DHJjh8IW8LF14mSzv4A3t/s/ozeraHOHp5bpBUEvlRl/IFyaY"#;

const MOVE_DIAGONAL: &'static str = r#"eJxlykEKgCAQheGrDHOBeIoLQb1NC0FUqIVz+yalINq8Hx5f6K1IyXWn3nI9j8iw5Ah+DcApbI9J4a/xWh371VMKIjumoYFnErM6zH0rn/QCv8cmmA=="#;

const MOVE_DOWN_LEFT: &'static str =
    r#"eJxFyiEKACAMRuGrDC8gP7IwmMsGrXbBYDR4f9Si9X1PZ1uDenQFIEjiiuBM/c2mH4U48xmebcWDD4w="#;

const MOVE_DOWN_RIGHT: &'static str =
    r#"eJw9yiEOACAIQNGrMC7gGCOwIdmg1e5mMBq8/xyF+t+3u96BXXGQAvEkbcToVqK7pQpIj0PTPtMbD8M="#;

const MOVE_DOWN: &'static str =
    r#"eJw9yqENACAMBdFVGhYg/YIgSicoFk+CQCLYP1BTd8k7OfNuWi31SlyNQYBx+Z1UsqNKLK4DCHnwMxA+"#;

const MOVE_HORIZONTAL: &'static str = r#"eJxlyjEKwCAMheGrhFygJINkiN6mgyAqtIPevkmgk/Dg58Gnc7Tdar9hjtrfJyMJCDAD2QQoYdHrR0UPnlw7TocNtygjIyy2WLdd8nLUfNgPvTMmhg=="#;

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

const MOVE_VERTICAL: &'static str = r#"eJxtykEKgDAMBMCvhHxAkkPwkPQ3HgqlLejB/N404EEQFnYXRudo3mo/YI7ar9NwB4owMANJbCy6vajoD5fUC8vXpnMyZISbDSnao3l9yh8+7QO9VyaG"#;

const MOVE: &'static str = r#"eJyFzjEKwCAMBdCrBC9Q8sEhYHubDoKo0A56+xpLqbSD0yfwknyXU6jBx51y8vE8VmNJCMQgS2zN5pZHbO5nRRGUW5rZRli6RvvAMuOiPXoRXftc77RgNYChyo1r4s7SZqgfbB2skoHyay/Jxk0L"#;

const MUSIC_2: &'static str = r#"eJwBRAC7/zxjaXJjbGUgY3g9IjgiIHI9IjQiIGN5PSIxOCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+nsgTIw=="#;

const MUSIC_3: &'static str = r#"eJwBQQC+/zxjaXJjbGUgY3g9IjEyIiByPSI0IiBjeT0iMTgiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+YqoSWw=="#;

const MUSIC_4: &'static str = r#"eJxVy00KgCAQQOGrDLOPGKVQcLxB2/YxBQUGIRF1+5Tob/2+55ZuHaFnbCyQaatAqlAbafSuzMm7G8wWEkn1TTJFCQPIzlgjREaNIAcjmWyu+qhPTjg/P3YCU88mgw=="#;

const MUSIC: &'static str = r#"eJxNyzEKgDAQBdGrLL8X2QQlQjY3sLWXVVBIIUFEby9R0NTzxm/jvtAk6DtiNzSRTWUOtgi+zil4XZPGmfQSsAMlgQXpKWizeeun/prt85TsBrq2HyA="#;

const NAVIGATION_2_OFF: &'static str = r#"eJxVy0EKgCAUhOGrDO413tPQQL1BhwgKCiRatLDbpxZBm/k33/hjOlfMQYyD0oQ2PZiSlQYWRpIiK3UZEX1XcfTfhYzqNZxyDsTg1DD+OG37gkxBsMD1hktK89NCK4o3FwghoA=="#;

const NAVIGATION_2: &'static str =
    r#"eJwtyrEJACAMBMBVHheQBEQEdR0RJAlo4/ZapLviqum6QwWmU85ugRgMKmDCJ2UkJ4deo+/+ALgSDuw="#;

const NAVIGATION_OFF: &'static str = r#"eJxFi1sKgCAQRbcy+O/E+EgFdQctIigokOijD9t9o1AxcM9wOTee87XBksTk0WjooYGoeFB8nGidtFyLHIcm5/hNyKEOLKPToFgvMrTX4Ei/XfZjhUpJKAFVMZj3y16z2qT8ACzuIcE="#;

const NAVIGATION: &'static str =
    r#"eJwtisEJACAMxFY5XEBav9p1RJC2oB+314K/JKS6zdNN4TZ0r5YKiMAMBhUwhT2ImqTmf8sFtm4O3w=="#;

const NETWORK: &'static str = r#"eJx1jMEKg0AMRH8l5C7tRCot7PoHvfYuVRpvRRZr/97sIroiEpiQybxxQ/cOpF3/0eC5Yhomz2D6m9r169ugyZ+SUbtLBGp3gpnKEd5TlnmkjOT9S8FWuULfJii1np83QjUWZQMCXW1sF1BIbhDGMqIRylAIQV739TMDX3lDAw=="#;

const NEWSPAPER: &'static str = r#"eJxtja0OhTAMhV+lwZe7noylJLtoDBaBIyBqSBBkzw9DwASp6Xd+cuI+H0brvxo8ASZhBoHcfWCM/mW+uNeSCakoSBam7XpLiZG4XRxLLXWbEwxD1cVfHu7iMy9K4o31y2pI1Lj5shwF05SLY5iewAllezQR"#;

const NFC: &'static str = r#"eJxtjUsKgDAMRK8SPMDYpDVVqL2Bhyi4cOnC+2M/tKsS8sjAPBLe9D10n8ultMNK8nCWKkwezuthdYlhLc0YRv+AU1IIJ2Z4pcYu5bTtE4sFB5NDtrZyNTYLpuapprCeJIkBKzX2V2KG8ANXli9x"#;

const NUT_OFF: &'static str = r#"eJxtUctOAzEM/JVo7zGxY8ex1PYL4Mq9WpBAKogDQuXvGe9KFYcqWj9nxxPn8HX+fisvx+WJpeizLKfDQ5ZOh1vDCrcfPTu1pmW3bTtG4l4G+fSVlIW4Kc0mJCGFkQlZyCOIRS40hGualXqlTiMq6Zw7riYwJ3DZzM6vxGJVKHrckcWRujp17fe6UCBrZbQxyLwjjGawrSUnPpR1oChZ1FTSrWerbBWekKuKUEfmE39sDZnkjGAEBUC9cMk232jcHDStg6cX4tBK7oNmcpkl2gMA4Q8GiykWGYEVzunYiTkAguEzEmqW07HiHNKxDB6Ixuy5OWDh0ZfwHN83rM1tPNRi03lRxZk1ZeDptgsBOxLFBn3/Hv3y/vlarnxcZCm/u7sKXKa7BzRBpz+yenmT"#;

const NUT: &'static str = r#"eJxtUD9PRDEI/yrkdrBQoCV5XmJcdXVwuzwHBwcHc59f6EvedGlCaeH3B7bf2983fD1f3llAP+Ry3Z7q67qdBQNud70Nak3hiG0dIxkDnMYcOykLcVOaTUhCgPMlZCFvSSzyQy6MFXbqSJ08kHTOow+r8aW4OxzxUOAA1jvqA1dl93WCgpKB5z13JNFOMZCigBQmKNAz6yndZpmKkWIS0JGBTCmEYhW6OaTRzDRh5DoWgyJTiwk5SnKlTaMIK9fG1SujSgvXj/kymy4FsxPmEqlcjOxLQvoJk5abMK1N+EzrnoxYhdbTG/v670tx1DDJLDlj1T3jsImSOQrmXlMNNc9E/TyX9g9MgGML"#;

const OCTAGON: &'static str = r#"eJxFzE0KABAUBOCrvByAvIQFriMlT7Fxe/KT1dQ307hKeUQqUCmV3jwz3GpAkJpLtRIRjjy6xee/3XKYBSfuc5i5PBcj"#;

const OPTION: &'static str =
    r#"eJw9ySEOACAIAMCvMD7gGA4NyA98hJuBYDD4/6mFfLrHcZgNOwO7LAGqLmiaPpgGU35eAi7D/Q/j"#;

const ORBIT: &'static str = r#"eJxty00KhDAMhuGrhOzN+NV2mEDrDeYQUgUFFyIu9Pb+dFNFkk3I8/o4zHHsKK6BYZjmwBVT3K6r9p/0rn3O9GImMfemtptaH2pqlp7awH+UYslAtEFJx6ZRUYsCTiy+Z3byPKrEkRHkTaHys6BHswPyWzj/"#;

const OUTDENT: &'static str = r#"eJx1zDEKwCAUA9CrhH+B8n9BHdTbdBBEhXaot69W7CB0CoGX2JJjjSEdKDmk63SkYbCDBRqsyNttCm9fV8URC6HyyLt3btm6cB8sWA2rPjInC+1/hsa/+f19APSJLq0="#;

const PACKAGE_2: &'static str = r#"eJxljLsKhTAQRH9lsM9ed5NcXYiCnYW2FnaCRRrBQvx+Y+EDZFmYYeZMWKctYq6y3kIjlzvnk0CQp2OTVOvf3sigY1aH34nV4YKXBEPIeeNIm7uPgsTBRiUvzwqYyvTcCUO/Wz0L7P6/gwMIIyZR"#;

const PACKAGE_CHECK: &'static str = r#"eJxtjd0KgzAMhV/l0Pt2JtZVoRX2ALvdvbDBBP9gMvTtl+o2eiEJhH75TuqnZn7iHlRPZ0izlNVW1f4UN7X/7a9MoOxWNtHIYmnSZFzeaadtQmUKgr38EXKU7yQIuRSDIiUw5lgu0sHnvTMFrGGHCoWhIjHGbu3a4YFpbIf5FVRuuIIDcWzOjCO4Tf+Ktd/0lYMiVlgpKJa50P5edi6BTf4Au7VFRw=="#;

const PACKAGE_MINUS: &'static str = r#"eJxljUEKgzAQRa/yyT6pM0ZHIQo9QLfdCy0oWBUqRW/fJG0lUBL48Ob9Gbd0a49boy5Ugsq+VK07Bda6Y8IEyq5Vx2Bk4WnSZCQftWibUJ8ewZ4PhBzVKynCbwpFLyUw9NhvJPt//CGmgDUsqFEYKhJjHvdxmO5Y5mFan43KDdcQEIfPmRGCRP0rti7qGzWKWGHjT+6/9Jw5FKL8BrryRGw="#;

const PACKAGE_OPEN: &'static str = r#"eJxtkMtqw0AMRX9FeD/qSPI8DE6g+3TbRXeGFhpwQhYhJH+fKzsPcMKAhpmrKx2pPwzHf/pdNV8auROqXFuElElZbRDujKYQ/QThKhQ3xkItiw3KojSFWeeYyDh3o8CulOEckDCrXqiFW4UgJ3nj7gLc5adZ9x8Otu7veDuDONPBaylkzrLAQ9kJb0SN5N+LBkLAdrpN5QJfyyU96CTc6OwtHLyvcON2/0dnWTWiDZ11vi94q9/+Nk/2tOckWDSJcToZW0GX6KtGuEPINIaNIYM11uWMyCgVM4YcXP5cFCDsJ3Mt397iQXoFSO5mZw=="#;

const PACKAGE_PLUS: &'static str = r#"eJxtjUEKgzAQRa/yyT6pM4mmQhR6gG67F1pQsCpUpN6+Ey1toCWBT968PwlTM7e4VupMBahoC1WHQ2R1+E5KkF3+TZhA2eXYMBhZPJo0GW977bVLqKQguNMHweK4JEXIplgUKYGxx7KR3O/nd29yOMMeJXJDeWKM/dp3ww3T2A3zo1LWcAkP4ng5M57gN/0t1mHTV64UscKT9lwlOb53LoVNfgFAsEvV"#;

const PACKAGE_SEARCH: &'static str = r#"eJxNjlEKgzAQRK8y5N/UXQ1RUKEH8Lf/YoUKVsVK0dt3Y1obEjJkeLM7xdysD9xLVTOB4lvWMBixOxFFpG0yRDZKA1dULKTX00KC7B0EIZNcUKDAdDmWiZSqqri4tVXxW/602iDVbJHDaDIBMQ370I8d5qkf11epEs05LIjd5Vhbgj3wL1gVB75RqYgVNva6y5+d+r8EPNz2Szt0aHexjTYKi3BO202cTB9VPPSvW8te6UrWvSxN8rPwB9PiUkM="#;

const PACKAGE_X: &'static str = r#"eJxNjl0KwjAQhK8y5D2xu0mIhbTgAXz1vaBgoX9gkfb2blqtIYGB4ZvZiVMzP3Gv1JUJVNzODYNRpKdJkwm200G7zBUVC+5yWLA4v7MgpCkFBcrMlGNpJKfqeEpn6/g73gfj4QwHlPCGfEaM3dq1wwPT2A7zq1LWcIkA4vS5MIEQNvwL1nHDF64UscJCu66inHT3JbDDxwKSTgsP32sva73+j/gAi/hGbA=="#;

const PACKAGE: &'static str = r#"eJxtTjsKwzAMvcoju9RKtisX3EAP0LVDt0CHDg10KDl/5ECCIUEgwfvpld/w/+B960bjhMhquCKxpK4vp8r1ZVU8VJAHheJch4SELXzJKDaoX4cQ7xuEgDw1RggWo4sasPo8qbH5O7m89j3GwAGGzIZUNx11Fc/Rp+hGzffYNWU="#;

const PAINT_BUCKET: &'static str = r#"eJxtTjEOwkAM+4rV/UKS3sEhHX0BfICtKkMHkBj4v3BQRYdWkTPYjp32Hj8zHpfuZWeYpRojRxCjw6HLuNRnEQcxSZVKKRah19/lvRvaIcKG9o8sdBWUrXJzWD/brsJWXaoNljJ00mR8yeSUXDKcnPQIirYgkNf2Lx1cMMo="#;

const PAINTBRUSH_2: &'static str = r#"eJxtjTEKgDAUQ6/ycbc2aRUK1dnF1cFNdOjoIP/8VpFOJWR6IS9e+53kHJsFXhBMWDEkt1Mo9gtbasu5Vx5WYGCCvJDJqTPhHyLXi92aKXbv4RTL7SDgygQqbAVnK9XXQFYrC3gAjIcrKQ=="#;

const PAINTBRUSH: &'static str = r#"eJxtjjEOwzAIRa/ylR0asB1syc0J0gt0s9Khg4cOvb+KoypThPj6Eo8P9dO+b7zu00MyB4PyEiAR1kk4lUOaQjGPIuXsdss+Lyi4QP7QJgaZe/RMGtKUxZkhM8ShQOE5rfU27q/1/KIg76QIFBE4kSH27EG7kmCh5G0XWxI5QczlMOlEfiwvMfM="#;

const PALETTE: &'static str = r#"eJxtkE1uAyEMha9isR8XG+wBaWY2WfcQEa2USl1UaRdtTx8bJtJEisTPg/fxwCzt49o+3+G6BpQA7XcNlLr6W4Oa2JaXgWzLjrpD0ZnjofkpeyDKPfU56YgeQ/0WfkS/zj8XeFvDKzHwyWiwGcZM/J1NUBytYWUFU6i5TDhn3aWNpUCcMKd5QjKvJBkrQmKTXK2X6nvmqvBQ3T17SE+ybEt3qT1Uy4WwVm0Jo5B5giIyWQUx7drHfGLDVOzV0aqwb8tK/fnA/16r17jdABjqXu4="#;

const PALMTREE: &'static str = r#"eJxtUDEOwjAM/IrFblM7dtNKpRIPYOIFKAwZOjAw8XrOXWCorFwcy3c5e3k93p2el9NNC01tYJM6Anzk4BCcu1GIORlN3TZlJUT307qck7su/wpV1K8hU3IAAwLdowSNrcjgeEOULAUzic5lY+VdFvmBKoRmmqVqgy1NrgayAr5XlhL41MvmkGRACIoV7UDNZkuaNVaZIwGDiVVkE4MIofgczQI/MNh2s8GilRy3Embp3mzfTRYsvf+28QWkH0sY"#;

const PANEL_BOTTOM_CLOSE: &'static str = r#"eJwdi9EKgCAMRX9l7F1iiuCD+i+RkkJFiJD+fXMMth3uPb7lo0ObATUCb4PQhsAQ+GrqJSA5hJLrWbr80W/Li/6qT4ZBUp18yLKo2SdmLczd1Yr+3XuBFPAmC04ZMIpnxSuIP/uJJGI="#;

const PANEL_BOTTOM_INACTIVE: &'static str = r#"eJx1zNEJgDAMBNBVQhaQWAULTTdwCLHF9E9KwLq9BH8U9O+4x12oeVWojbFHOBkdwlGSCiNNCJLLJnrnZhhDZ4MY9kUFEuNMA9AoZGLdU7xJ/yHuD/z76wK3Six4"#;

const PANEL_BOTTOM_OPEN: &'static str = r#"eJwdjFEKgDAMQ68S+i/aDUFh211EhxuoiAyct7cr/QjNS+KeuBZUT5bw5q0kTzwRPjUeESNSVVLMeyrKg+tbL7gjXxGfEXMkVNZWldewbLDakm2p4O6lJGyezhk8wHYWcg03EH4AmiRE"#;

const PANEL_BOTTOM: &'static str = r#"eJwdy0EKwDAIBMCviB8ophR6SPKZNjRC6UGEmt/HeNlFmc3SLoWfb+0F6UQQK5gQPHeEESkjXr3x0zVUzdva1fzy18AomCVn5CM/6fBO0W6XqhMZIhxC"#;

const PANEL_LEFT_CLOSE: &'static str = r#"eJxFi1EKgCAQRK+y7H+ESVHgeoMOESmtH0HIUnr7tKA+BmbmzZjoV4GYCDuEKzhhQjUisA8by+szoUaI+dmkGqxp68+aYxEGRzhPoM+yLaBWP9jVAKpvdKOh6OM33b4gyg=="#;

const PANEL_LEFT_INACTIVE: &'static str = r#"eJx1zEEKgCAQheGrDHOBUFskqDfoEJHSuAsZrG5fU5uCXA18P/NcSTMDpbwQe1QDwuHRIGw5Mj2w31CuozG4Th6CWycmiB5HC6qvSoLQN9iqf4NpuH0NnTEtK/w="#;

const PANEL_LEFT_OPEN: &'static str = r#"eJxFy0EKgCAQheGrPGYfYbYoUG/QISIlXQQhQ+XtGyNqMbP4P57JYWFcljShPP9MnqMlNRCy9I4QQ1ojv6nU5Exbd87sM0d4S9MIfYgL1PTDpnqIQTdyH9+nESCB"#;

const PANEL_LEFT: &'static str = r#"eJwli8EJgDAMAFcJWUBSPwptl9FiA+KjBEy2N6mvg+Muj3YIaMEVoTe+uhSkDcGmefmU/ovhTXJYoOYlvppvfhooFdx9oflY8oIQNIX1Mpr6AeVRG+g="#;

const PANEL_RIGHT_CLOSE: &'static str = r#"eJwljNEKgCAMRX9l7D1iSWCg/kukpFARIpR/37YetsO2c+dq2hrkVPbcPJJFeD0ahK698jAxuuIpsWWVghslF9xRrgSd/sjEFvEDJs1MUrIrVnD32jJEj6eFBQyYgUuusg8f5eEkGw=="#;

const PANEL_RIGHT_INACTIVE: &'static str = r#"eJx1jEsKgDAMRK8ScgGJVbDQ9gYeQmwx3UkJVW+vwY2IrubzmHElzQJbjsIeaUDglBeW2+8eDUK5pEU4NATX6CC4dRKG6HGkHqirpES7F7G1/SbmD9jH1wnRgiyw"#;

const PANEL_RIGHT_OPEN: &'static str = r#"eJwlzNEKgCAMBdBfGXuPmhL0oP5LpKRQESKkf9+mD+Myzt1MDkeBGNIZi0XaEJpFjZCrRcXRenzJlzi4Cjszy50zV3oCVGJamdTIRv1F41WRdKXlzLuXCN7iTQvQOulJA4+4iPsBNPwkiw=="#;

const PANEL_RIGHT: &'static str = r#"eJwti0EKgDAMBL8S8gFJRfDQ9jNabEE8hIDp701qT8vuzkQuh0At7aqSkHYE1oQBoSdcEd52Sv13HQN3f3Nc3Mvxbk8BJSM2U2ggGma1DOSsU/kDJJIcQg=="#;

const PANEL_TOP_CLOSE: &'static str = r#"eJwli1EKwyAQRK8yzH8pq1AacL1LaaQKbQkiRG8fN2E+Bt68CTW9G7rSE3UoHbGXtWWlPIlx8pzKJ7eL1G5ODHf7xfAt/4ThlAvRZzmZLedriNFpmhPD9moZq/K3QB7wN48Zm22IB8LDI/A="#;

const PANEL_TOP_INACTIVE: &'static str = r#"eJx1zEEKgCAQheGrDHOBUFskON6gQ0RK4y5kILt9jauC2v4f74WaV4GT0CHURmgROJeNhdBMCK3DUZJwDzEMOohhX4QhEc5mBM9GQdMT/A32A9xP96+jCxc/K8Q="#;

const PANEL_TOP_OPEN: &'static str = r#"eJwli1EKwyAQRK8yzH8oqy204HqX0kgV2hCCEL193IT5eAPzJmzpU9GVntia0hF7mWtWypPIqXxzvXq7nG5ODDf7xfArS0IX5Yvo7kQbcDIodhimOTGs75oxK//ygNwnDz+N2G5LPADs8iQ1"#;

const PANEL_TOP: &'static str = r#"eJwdy0sKwCAMBNCrhFygxG5aUC/TShVKFyI03t5JVg/mE3u5BmninamDAKZTS3vqSCwH0/T+b/eoHuS42S/Ht32FVLzWgJ9gDE8gBpa2yQvaCxvo"#;

const PAPERCLIP: &'static str = r#"eJw1jbEKgDAQQ38luPf06rVeoRb8lIKDQwUH/x+9ogTekJAkX/U+sK/D6ZlEwExTcIk4wVAjIqZX7JQkdTSlsDjDJhDLTAollWZ+goXVw39VTzp3tH9Eh5JHuy4PCMYboQ=="#;

const PARENTHESES: &'static str =
    r#"eJw9ykEKQBEQBuCr/NlPL28mUcMNHEJZWCruH0oW3+7TXmZDjSZ7/HaQEG8Bl0n6nZH0PevAQ8AQBLpeWxmZE54="#;

const PARKING_CIRCLE_OFF: &'static str = r#"eJxtjD0KgDAUg68Suqt9ff4gVGcXD1GqoKAgRYre3pYOXYSEQL4QbXdnjxX2GQQpAfumdCGkGHWV+Kgvc29YBnE2aEB1UKSxzWwmBrFhMCQoWBbt1Hv1s+xBnS9UyfnmA0yZJSM="#;

const PARKING_CIRCLE: &'static str = r#"eJwlijEKgDAMRa8ScgEbK5ZCmtnF1b1EoYKDFAe9vany+bzhPda96rFBTUgOQW9jb3w+Cne/Fz7zVWBNOEegsIQyZA8enI3s4xRb3CJ5AbuCFfM="#;

const PARKING_METER: &'static str = r#"eJx1jrEOwjAQQ3/F6n5HLpdeEil0YeYjKhgyIoH4fi4dUIdWlrz42XJ7rZ+O53W6V9RVoQgQlyFMS7uMdGl/RiIkfvUo8dLc41EyhjPyNhxIlA2R9WacIMmtwJFhvbyFjdIGGGXOD1biwomEZ3evnZyqu1M/5nA0HQ=="#;

const PARKING_SQUARE_OFF: &'static str = r#"eJxtjrsKwzAMRX/lkt2qJeVRgxvo1qVrd0MHL4EMwd8fOUMSSBACIY6ObpzTkvF/NV+lHtZvgcBbMTpo5jYdC5vKeeGoC2BquRnjo4rGeOjwpOHHYad9Pc/sSa/0VCExwt+YWMGaFLplMJHrP6HIDWlhhuLk9GEFj5w16w=="#;

const PARKING_SQUARE: &'static str = r#"eJwdi8EKgCAYg19l/C9QZmSCeu7Stbuk9HsLEaq3T2WMMfbN5HgWcEwXF0tiJbyWJCHXmAhfL08KhfvqzNAOzty+MIKlXUOoQ/HsJSTGKlG9bLqhDXI/SUAaOw=="#;

const PARTY_POPPER: &'static str = r#"eJx1kb1uwzAMhF+FyK6rSOrPgJulS4d27dAtcAcPLdChyPP3qACZHNiSaR8/Hk2tv5e/Xb6eT+8VQ1ThYmL2rRk9OfpyOq9PkXNe75lFfEfWA8VMxgNJq9hjyvKx9hNaMlhBrxfDIrFyXEmxNHGoXfMGxWgJtYuiOaVSZ7Qn+NgSxQk0NO69USvlTYtoPvZUJ2OEfcIMCsHBRsKWg4IqbRMfnBM6CVhoxWf0qv2gsCqHy5ostLHiLBzkCJJilL7mlwXVpPA/F6kRLtJg/tGPpkrIN3Jhy80wnKh2GlV2TivnMWb2yFXTbN9vW+SmyOXXKvEScCTzFh7X593wH2lof2M="#;

const PAUSE_CIRCLE: &'static str = r#"eJw9zDEKwCAMheGrhFygVexQUC8THATp4JTcvo1JnX7C+0imPmk0IC4YIgKJdX45sebD9ppHfxpILHgjcFgrcLSK3pdqVW4XSo7SRv7jty+fAyFD"#;

const PAUSE_OCTAGON: &'static str = r#"eJx1jCsOgDAQRK+y6QEKndAPyVKNKLaeBFGJIAhOD3QJKOR7LzO8zluhZVCTacnY3KvIze0if6X7K177K6IEbT0SQFXsgsZpBEfAWG0CicmCz/Z4b0/5JSL5"#;

const PAUSE: &'static str = r#"eJxVi0sKwCAMRK8S5gJFKF0lXqaVxm0JVG/vDxQ3w/B4j79wG2mIr5rAXaAkqJsFJ+iPj2l7no8meu76xFs4kpq75Rc9khtU"#;

const PAW_PRINT: &'static str = r#"eJxtjcEKAjEMRH9lyN3arEmt0Bb8AA9evZUqKHiQRUT/3rbrYYUlDAkzL0wot7HcLyjvSMyE8okkhDHSQCmspzSFH9XtzniaXvwC1YLB0oxmN8ce+XnFOdJhB7ZZobB1uG59bYzmKjR1d+WMF7CxokdndABvjXiIEQdu2f4Ph9aL7an1tZ70BamTN/o="#;

const PC_CASE: &'static str = r#"eJxti0EKgCAQRa8yzAHKCRUC9QYdIlIadyED1e1zdi7a/P95jx9aOQTumoUjkkXgUk+WiItBaE9vhJ4O4dWdwqyHFK5dGHLEjRyQ5cmQOqWDW8Gz/+VkBvEBvRUlTQ=="#;

const PEN_LINE: &'static str = r#"eJxFizsKgDAQRK8ypE/M7iZKYM0J9AJ2AYsUFhbeH7ONMjAwn6d3ezrO1e3E4NiLqzpZV/Vf5pAhITcOdjKLQwSBbAuoXD6NRD4dH/0C6QYVeg=="#;

const PEN_SQUARE: &'static str = r#"eJxVTKsOgDAM/JVmvoN2XUCMaQT8AG4JYgKBIPt+WsMjTXqP3F06y1Vhn9xKBDJLYWDo7VBZo4+hyPVvIDccXE6dreT0bo0+AvtY2JPG7VmeIEBYVFA8UFQSyvbUb5EQH1Y="#;

const PEN_TOOL: &'static str = r#"eJxty8EKAjEMBNBfGXJPJY2xFdp+gT8hVVCoIIsH9euNu4e97GVgmDfleX7dcKn0kAg5InGCQj0TK+uXWtn9SSsrzBBlCcYp2CkiDg0G2XsRheRhbFs/l/BHPiy5in6f+riifyqJEKZKkdDfc3O0zO0HQyEr0w=="#;

const PEN: &'static str = r#"eJwBQgC9/zxwYXRoIGQ9Ik0xNyAzYTIuODUgMi44MyAwIDEgMSA0IDRMNy41IDIwLjUgMiAyMmwxLjUtNS41WiI+PC9wYXRoPvZCDv4="#;

const PENCIL_LINE: &'static str = r#"eJxlizEKgDAUQ68Surf2t/1K4dsT6AXcCg4dFBy8P/YvOkggkORFrno37LNZKSD4lk2RQbsi3zI6RnRcg1NIzXcRIuIygfJhU09k0/Z/n8RgJd/pAZUVHMo="#;

const PENCIL_RULER: &'static str = r#"eJx1jz0OwjAMRq/yiT2mdpwfpNIThAuwVWVgoBID9xc2EQxQ5MSR8uRne7zPjysux93KCQkK3U3j3j+n8Y1OHFFQqUCozELKeKXBI0RSDM0IEsVvamG8FXD89a4VGRJkgwhEzJcCU2rCxAWZquurIU+9uQZtkRI4UzpveLga+tPDnMxQin4XOmg/Pn3uc9+CkFVTXkJnwZm/fWkr5/IxPwFZsEzm"#;

const PENCIL: &'static str = r#"eJw9jDEOgDAMA79idSc0baMyhL4APsBWiYEBJAb+L5IBZOkGW2e9+3Ngn8PKFbknmgSGjAi2FJSlklXRgZROJhmEZAtNR3ebfg8XC8SNf3oB80QWUA=="#;

const PERCENT_CIRCLE: &'static str = r#"eJxty0EKgDAMRNGrhO7VRmghkPYGHkKiUEFBigu9vakuROjqwzyGZcmyzpCDQWtALm2vPZ9G7l6PvI9HgimYDR1Q48EXLONHAwGl1mJF9ITuZzcQKCGs"#;

const PERCENT_DIAMOND: &'static str = r#"eJxtjzsKgDAMhq8S3BubtkGF6gn0Am6CQwcFB++PaRVBW0JCwv/l5Y/lDLD21WSwAdJoF4OOIAX9mJVia5A7iCEDbEoToIqAVhEZycoKWUN/QN0T5mrwdbxn8O9VHRoQD6gpV3dyyCKzYuBCs8jyk8P2034BAHk3rA=="#;

const PERCENT_SQUARE: &'static str = r#"eJxty0sKgDAMBNCrhOz9VLFYaHsDDyG2mC4EKQHr7TW4EMHdMG/G5rgwnA57hCMFJodqRKCYVuIn5+KwQyiy8baRg7f7zATB4aYGMJUGLSTlS5MBQ3WrfuQ+qeFjF0I7JfQ="#;

const PERCENT: &'static str = r#"eJxFy0EKwCAMBMCvSB7QotCWQuJnggdBevAUf9+koXrabJjFVp8SJBLEG8LQPDSSV0lWM+6GMnLt3ErgQXBuyjpBsmTxh0Ink06hk3j99rsWfgHmECL2"#;

const PERSON_STANDING: &'static str = r#"eJxlyuEKQDAUhuFb+Tr/l+2ME7XtDlyERlGUlsTds5SUn2/P6+KU4jwgHp4ME+LpqSKkuyi44tHg1m4b0XtaGrCGVQILyUOGDwtqCBii+K+tYRi9l69ceM8iHQ=="#;

const PHONE_CALL: &'static str = r#"eJx1UMsKwkAM/JXQe8dNut0H1ELv+hELHnrw4EE8+PUm7bYUVAIDSWYyYYZHec50OzdXEeKALK+uCAk5LW4FnLThjJgrrouE0LUdXLRpv8K6CW34wTeqieK0HycPZpL54GdWiAqC5Cu6pRBJOyNsr8H3OmK+JLhMGZkLq3OogkDhzpDYGhwdVLJovz3MAMcHt0jezTicLKhx2ONiD6cPFLWu7EQR2f+lhqmnmhFpqOx25getik6Q"#;

const PHONE_FORWARDED: &'static str = r#"eJxtUNEKgzAM/JXQd7MmutSCCnvfPkLYQEFUmAzd1y+tOoSNwKVN7nIhxTh0S9f2DxiHtp+epaEcGJhBQF9kTVWcdk5VRObMpWE2sGgWTRTTrImyQF+pYz01cC/NTWeRoOdXWutgsBqUMEYf8uj8hmsjR0mTFK0L1fMKa0cS+cMP1CByl+9wyJAIuDn4BSt0Cox5tqGNgQ70Fwj7apidtUR0zdF68OipJnWWTSAgHSG7JMDRQSVR++sRDPC44H6Sd7yuHqr6AKe5UxI="#;

const PHONE_INCOMING: &'static str = r#"eJxtUFsKg0AMvErYf1MTdR+gQv/bQwgtKIgKlVJ7+iauFqEly4RNJpkw5TT2S98Nd5jGbpgflSELDAIemMGbujztlLpciQtXxht4sXIlU2WYDSyalR6pUzO3cKvMlXUbBn5mDcvmVIISRvIqE9CFDWPDo82SDFOn1SJC7NjE/uErVYfc+bscciQCbg96KoVOgNHnG6ZroAP5KWE/DfNCSkQXj2mAgIEacUNeHLBge0J2icJRQUbW2V8NFcDjgbsl79VdMar+AGJmUuc="#;

const PHONE_MISSED: &'static str = r#"eJxtUO0KwjAMfJXQ/41NtvUDtoH/9SEGChOG+EPE+fQm6zYKSuDKJXe9kHa63a8wU2fYwJs7Q15epcJn4dH07UFFfVtK82hxqFId4tylj+E5wqUzZ2Ygj4lf1cDA4KTIMlIUQglDWjEPIvrKVuiCdpsMeeKt/6NXqZrCcf8caiQCHos8jcIgwBjrFd1SGECYCrbVsG6kRXSK6BIkTDSQJPvV4MFPhBysQpkglsX7m6EBWC64neSj59JD9V8w91AO"#;

const PHONE_OFF: &'static str = r#"eJxlUO0KwjAMfJXQ/4tLWvsB28D/+hADhQlT/CHi3t6kLVQYLddekwt3HV7ze4HraC7Uo49AFi3N5EF2n5dFR8DoV0IOncLMwLmk70QduiMQY3QVi06uhAFarzAhH9vkncijEEoYUsVSiOhtZ7EP+uq4YqmJQytKHx4Zlbt5P0LVec6pWXBiF3ix/wmKr71/9S4RWoCcUwOfI/YJEiYy03DQD5yG9f68wcajYTbwpXoqN7Ap1VZtmn4rPEnO"#;

const PHONE_OUTGOING: &'static str = r#"eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaUBAVKkV7+iauFqFllwlJJpkwxTh0S9f2DxiHtp+epWEGDwIMZIFNVZx2SlWsxJmVZGDRaGCm0pCVVKJXeqSO9dTAvTQ31kUY+JXVujSVRwkjeVUI6MKGseHRZkmGqdPqOULs2MT+4StVh9zluxxyJAJuDnoqhU6A0ecbputDB5IpYT8N87OUiK4e0wABA9VihPw4YMF2hOwShaOCjKyzvxoqgMcDd0veq7tiVPUBVOFS3g=="#;

const PHONE: &'static str = r#"eJxtkMsKwkAMRX8ldD/XSdpmZqAW3OtHFFx04cKFuPDrTfqQAUvgQh43J2R4Tq+Z7ufmJkKsKPJuJyGhaMFBwNkSLkhl07WRoW1oEZNX+1XWjgY9mPdRN6XLbzl1YCaZK56jkEwEuds0LoFElvnAfhq63krM14xYqKDwxEbWzaCkD4ak4FITzLJ4/xkOQH3g/pJPMw4nf9T4BdqtOmY="#;

const PI_SQUARE: &'static str = r#"eJxtizsKgDAQRK8ybC9mV3AtEm9gay8qrp1I8HN7jYVYyBQD7834dewjzkAFwcZ5shiIK8LxkPUuIezzEO3htc/TofZLFw1DoEahxi6JhD6CHXT7NyVYO4HA3eFMMmn13V3GASfm"#;

const PI: &'static str = r#"eJxNykEKgCAQheGrDLOfchzBAvUGbduHBQYRLVrk7VMX0eLx8+Bzx35ukLVHrRAyezQIT7ljCdcE11cT3LXcCVaPkwEbFXFngTshgbLEUmElP8gDaBWbVCTUNMlsPvoC4t0fgg=="#;

const PICTURE_IN_PICTURE_2: &'static str = r#"eJxNjLsKhUAQQ38lTK93Z64owu7WNrb2ouLYiSw+/t6dTlKEk5D4fUyKOVAvjHaoR4HAmQoppKu+DDnZTQ5cctnCCtGKov/ZR/THMiXosq2aAjWEOxAL4cn2JxwZM13bnDQnzna2iC/q6SBp"#;

const PICTURE_IN_PICTURE: &'static str = r#"eJwVjEsKAyEQRK/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR3mLKgpehfezKuwRHytM7Ju/u0JCM3SKODCNXDYkLDPxOpEdKC2FNtiazQaEjefNgx2owxRu4zOF68gVPudeNaIYBD3Ol9aIC8K3Lxbh18OxR7h6s8McTvoDprMlcQ=="#;

const PIE_CHART: &'static str = r#"eJxFy7EJgEAUA9BVwg1wXiLCF74HDuAQgoWlhVg4vV+Lk4RUL36s545tSouYRXDINs4siMZEDMrWp+rdS6v/B4Fq9OOCLpa74QcRMBYw"#;

const PIGGY_BANK: &'static str = r#"eJxtTjEKAzEM+4q53W7sxO0F0vtB19tLOnjsUDL09VUo3HQYhJGEpPZ+foJe9+WhlbyzilNik5VUCmcyzuKTZVWWDCQYIK5AyFTEd0tRBlvkYfMpHVaH58aKAAsDF2w98RT+aaDT7t9la5c5YWvHEKM6tM8OlYoGXOiJT6+kGulQflP1MIs="#;

const PILCROW_SQUARE: &'static str = r#"eJxti80KgCAQhF9l2XvlGvYD2rlLDxEp6S1E+nn7XDoJsSwzw8yno9sSXMEmb5AGBO/C7tPnb4MtQswiER4Ok24YmPSxJg/W4EISSM5jrVZZK+AX+QhEpWbqGeBpCfQnib+mK5sX8NEp3g=="#;

const PILCROW: &'static str = r#"eJx1ijEKACEMBL8SfIB3wXhHIFrb+AjBwtJCfL8IopUs28yM1NQKZKciGqCOn/LyTOblmP9qGCiwtom0hfl3jQOa3Q9sSBoy"#;

const PILL: &'static str = r#"eJxljDEKgEAMBL+yXJ8zEY8oxHuI3YGFhYKF/8fE4hqLnWIHxu72HNjXdAnngjEgTMJtykvBB4aASUlP//FXUOiWqg3RqtaLs7dirrt9AZOcGgo="#;

const PIN_OFF: &'static str = r#"eJxNjMEKwjAQRH9lyb1rJiZNF9KCNy9eew8oRCjiQaT9e5NWY9nD7M7OmzDdHzda0CujaDFZss7bOW/nEA4lNIQ1Wkx8M0ULCl/ZGn7GV6Jrry5C8gb7NhoypPOgAQOUPZny6juW0++nyREcGzvCJ6x1pWhXB0fCRzu2CfFP6caePXdSgQ9AczO6"#;

const PIN: &'static str = r#"eJxNjkELwjAMhf9K6H21L2TrBl3BmxevHrwNFCoM8SBj/nsTcawEEl6S9/HS/HjeacXowI4+NqOjlf9aJ7PL6WBvOb2md6Hb6M4tIRbI0sDHbmJiClYqAdsNs/W+8cNxO4KgrqD/l65g96hLTn2tSQoWqcH4gWkHV9xAim09y9VyWsL8BTQdLo0="#;

const PIPETTE: &'static str = r#"eJx1jTEKgDAMRa/ycU81iYqF6gn0Am6Cg0MLDuL5TQcdChLe9PL44dyuA/tYJYEImPjQ6MlXU6izmsL7sCiEb/qxiTv0UNeSsYljZBqwnUJnHuCja0tH5qIlQ6bMbCsX67f2AMSFKQ4="#;

const PIZZA: &'static str = r#"eJx1jbEKwCAMRH8luGs90bhYoR/QjxA6uBQ69P9pgtAuloQj3ONy5Wp3p2M1OxIB3XmYWhZ1a/kYCOmPMYHn7AyCKHhiyzb4TS7ZMYom35LL0pUd0ER9pKGaEB8uWpU3+ACqBy9e"#;

const PLANE_LANDING: &'static str = r#"eJxNUEsKAjEMvUpwn2eTJraF0RPoBdwN42IWXQh6f0wHGaU05H3IC5me83ulx/lwU1JdNR0u03Fwl2lXMkohSaMqta5scBII3Jf40IqGamhBmr+QN7V88bVShffMp4DJ4XmOKErxZDDD5dpRlBy2a+lPMxgpdIFFVZSYNyKSoJ06trk5xFiiMtLArbKMTiE9OqkQXxhWgmQ0D5sGnwtD7WrQSAqb/1bjCGbket8P8gF1sD+l"#;

const PLANE_TAKEOFF: &'static str = r#"eJxNUEkOwjAM/IrVu4fY2aXSF5QPcIvg0EMOHPi/cFhKFSmWZ9HYnh/tudH9PF2UVDd10zKfBrbMO5PgE0lGoGCls7JVCCPGZi5y9sSQQq7DdLKj7oOuhUQpUuqojHB0KVyF9gCn5A8+hbxhqWyS1BRhiO37pWXPFraqUG6CMVH4cijZulo7wxfkdGOoR0iMhBJY4AbvypoRy1hM/wMZq5ZdrvsdXoLvPKc="#;

const PLANE: &'static str = r#"eJwdj7EOwjAMRH/l1N0G22naSqULC0P5CBQGhgwMiIGv51xFSeR79p28vh+fF56X4W6TzrBFHVZh1kNH4b26ocJNRxR+iCYmiXCWQpFgt8CMwvGq3hIqWxY+psRdlD7U/SAwaHAs9gXmXRxxK11SDzgPodhXokcyRowajRYMOPJCo6eTNy1pXUUn1qb+G7b1lAttf7mWLN0="#;

const PLAY_CIRCLE: &'static str = r#"eJwti0EKgDAMBL+y9APaHsRDzGeCSKE0pXqwvzehnnbY2SXJXcoJGUeIKUDemd1iDUzL9ExNy7i0ommuz+0WO+KGmGDoYIUf/iF/lUsZUA=="#;

const PLAY_SQUARE: &'static str = r#"eJwlyzsKgDAQhOGrDNuL+EAiJHsQOzHBTSFIWFBvb5YU03w/40s6FOUNNBIk5VM00OAIVSbCk6NKg8+AfW8H9veughjoWuGwYO7qNssW+AclERe+"#;

const PLAY: &'static str = r#"eJyzKcjPqUzPz1MoyM/MKym2VTJVMFYwtFQwNFIwVTAyBBLGSnY2+lBVdgBVDA2H"#;

const PLUG_2: &'static str = r#"eJx1zDEOgCAQRNGrTOiNDMqqCXIDWws7EgtKC7PnFxsqaP+bTHjSm3Hv5tjgVEwM419iqJ2+Bw5c1DfEY82cGyAgz2JOpyQQWBAcypFVe9X9B0vKKRA="#;

const PLUG_ZAP_2: &'static str = r#"eJx1jjEKgDAMRa/ycU81iVKF6gn0Am6iQxfBQXp+00FwqEsC/30eP1zbHXGM1ckKIYG4LurMAl9Noc50Cm9n4QbcJtISav8Rg4ed2Hk0pPbVriaS2CfZTel8DmEhdC0JbJZ83Q8lsy+W"#;

const PLUG_ZAP: &'static str = r#"eJxtjDsKgDAQRK8y2BuzuzFaRE+gF7ATLCwiWHh/zAa0SRj2A/N44d6fE8fUrN4I2BrZ2Tjo2BzRbyEGjbH1KaxcgWVwa+bQqXAOn/ZiMENaKat1MD1IdFkQVQCyWvqMgVzFTmNyO7jTRz0/8QIM0jQN"#;

const PLUG: &'static str = r#"eJx1izEKgDAQBL+ypD80xx2ccOYHtinsAhYpLSTvNzbBQtlmmWH8LFfFsYYtMpgbaUg+PTD5UAss8weP+icM1rQIBHNfpP4qyRuQZNtHegPetSOj"#;

const PLUS_CIRCLE: &'static str = r#"eJxNycEJgDAMRuFVfrKAJqcc0m7gEBKFCh6kiOj2tnpoTw/eZ75l31fkQDwS/C6V0udrtOH3aMd8JiyBJgVL0kp1dcACvRq8h3gZnQ=="#;

const PLUS_SQUARE: &'static str = r#"eJxNy8EJgDAMRuFVwr+AtF5yaLqBQ4gtpjcpQev2Ejzo9X281OtmpLXtaoLAoD4EEXQLZtDViunbh4ecJh9yOlZTKoKFKURlB08/CJH4/OABOUYd5Q=="#;

const PLUS: &'static str =
    r#"eJxNybENACAIBMBVCAsYiHQvGziEiQWlhXF+Y2No77DGDpqNu5FoSGVHeeb4I0p20lyGNA63"#;

const POCKET_KNIFE: &'static str = r#"eJx1jr0OwjAMhF/l1D0h5wSTIfQNmJA6sFUwdGBAAvH8OFFpl1bRXWR//iuv8TPhce4uEfLlPYCQJrlGKCJO7zl29pvkz7u+HGp3X5YZzNDJB24gBfMOkuBzRDYbExJCfe7oVZs9HW2hzIimmm122z6BNDaIrNOQXBp0qf4B/eU6Pg=="#;

const POCKET: &'static str = r#"eJxNjEEKgCAQRa8yuI8aK2lhQgdo214oUBAVkqBO32gtZBbz+bz3ZdTJwD6zdYDeoNAcOHR0SJ9fQiPF7muakpeqIQZxG2up4Q9Tss2zSsbgbmf9ATFYn86ZTVlFsgZAQbmgP6RexLYiBQ=="#;

const PODCAST: &'static str = r#"eJxtjUEKwkAMRa/ymX3i/HZiW2h7Ay/gbhgFBRdSXOjtzYybLiTwfyCPl7nct/K4oryXwC6gfLwZsHmFdT78zuv8zK8bLks4keCQPRB9iA6xRKiJ9gm9qCGpZTVfGiAOFlEehRUySS3UzlVftTv5CKZssGYmRsQ/EAdw9B8TpsZFYdyBX/DtMew="#;

const POINTER: &'static str = r#"eJyFj88KwjAMxl8l7J6YpOlsYe4J9Lp70cMOHjxIn99UpChMRgjkz+/jS6ZHea5wOw0XVRArCRKwh6BXwzwd2n6eOiUJRCpKUdA3yKiolb978H5LaiC85B2lbCkZMsXF9qT87+APKB4GXMPvmyvqFZV8gkYRKY0YKWcfBbtjoLFl9xJwNDRez0eQ2D1fm+1L9Q=="#;

const POPCORN: &'static str = r#"eJyNTjsOwjAMvYrV3Y88N20dKXRmgAuwVWVgAImB+wtHSGWAAXnx+9r1sTyvctl3J7r4YmKS3qNZNhR7+g/F5G6uu1Y71095WEyK+Ld2Z24alb+ClsRXDFGLggwX3tQwCm1VEINialFEwWEKaoyHCCpyI5XHPqTSvKGhVwpced4uvQDH6jhT"#;

const POPSICLE: &'static str = r#"eJw9jTEOgzAQBL+yor8Nd7LDWXL4QT6QDpGCAiQK/i/OSFDMNFNM3adjwf/TfdX5hiammS70kKEXo6/i1MaUWDIu9dCIAxXBGg3BzMIBRr1chPrrxvpqh7Hen80MZpKZG08/AR+vHzg="#;

const POUND_STERLING: &'static str = r#"eJxtjCEOgDAQBL+yqS/c9oBWHP0BjyAgTiII7ycIUlM1YiZj1347zjVsLMiHxHlQ1Vh+QkK18YuqtVSQH04dsyDRmbqG6my3F42TIJY="#;

const POWER_OFF: &'static str = r#"eJxtjUsKgDAMRK8SeoDYxJq2UAsewEMUXLgRXIjnN6XQjSWfxcwbJt3lOeFYzU4BZwFBcVuECFaHgC16D7SYnKZK5tR5Qao4SWk41WOU0P4gQQz8ur9xqa5Nut37AL33JXI="#;

const POWER: &'static str = r#"eJwlykEKgCAQBdCrfGafORqWoHODDiEUFES0aJG3z8HV27z0lPfAlmnlxfiAYMJUIiIsGDywM7OHJUmjRknXee+onMkRvgarrlu77eqSH/HJFhU="#;

const PRESENTATION: &'static str = r#"eJxli7EKgDAMRH/l6B5sEoJL7Ozi6l5w6CI4iN9vXFQot9zd4/lRz4ZtSotAm+RUfHiu4h9g6MVcBYIcYYo223+TrNqb+4hwjQyRF9+Q0Ryn"#;

const PRINTER: &'static str = r#"eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv794sZQAnk4Oblcn+dxHdM0IM9pKs9ADi0cGLb5rZaiP/2t6HNXBH2gq9PjpepYrbOONWx4MfUOoECsOwLwUh8/wGL4m6+p0T+GW8FbKxBeqS8SyDJBhnSXEqghrEqqzd/M+AEIPzAw"#;

const PROJECTOR: &'static str = r#"eJxtjsEKwjAQRH9l2Htqd7dNLSQ99+LVe4lCBAUpUvTvjQqmpWUPuzPDW8bdh0fEydOhRgNFTZ3bfbzO/ZMW9qhr/8aaEDGSo3AZw/WM8PTUEsLLEyth9PTFf+nsMXOxV7D0Ug4CQZmG05apytqkq19oI5Op5oCRKAU3G9XZgm3MDd+BdDzA"#;

const PUZZLE: &'static str = r#"eJxtk8FOJDEMRH8l4h5v4iS2I7FIe9/9gb2NhgMHDkjw/6LKAaERaLprunviLj9X5v7l8vZUHn/f/etb5tjFJda1Sptbhqq0tcVmiMaW8HjusixK6lWm8/BmeNDi88rbfK2iY+Fax6hfj59rF+u9pF5kB47S8OlVYrioG6ynwx5nNMVNVNlG0XVRWa2XoygqrQ7RPksq2pkm3QwAS+Z2lrAWRr55flrBhnanm2wGb55tlqNnFRteOD2f6od+/TaTi+/AKI5eMAXNWUC5sgHCQaKbWHsAC2UxATArl2V74CFTEnUSDT8KIsP6HhIb30uBZJ6FP1khovS76enPt+5RtDH2rlfUWMfUh1UmNQl02P5O3JcQRw8Ka0XLgcmNNWR3EI02ZPUFIKcleuOmaD6o/QapMZ61ayrnEIi1b0Ok3BxrsdPGoaIcyS+8fZg0wzJDUrCqNOXW01VTf6LSA3YlE58kEdjO1suwbjavcsawh8OIhNDNkLj5JkKqTCknXL/HVDKgo2SyiZgqc2JMLPAs/X/3cP+Lf7GHd/XQrjQ="#;

const QR_CODE: &'static str = r#"eJx1j8EOwiAMhl+l4T5dS2QX2NmLD7G4xe5mFoLu7aXMyFQIByj/9/9t7TJdPfA039g7dVKwOqUVPObRc6qfqV7ihaq3R8F7m0zb3w79SxHA/Lh2UHa+o9bEby0/rvvgGUanLoSAhhs9EBC0cpr4CgkV6BslDIcWCxoSdCGHoIScuwKoAYmrGbouoam1NhKJ5YGR6gPHZZosvQDmFneh"#;

const QUOTE: &'static str = r#"eJyNjrEOwjAMRH/F6u7gcxpSpNKZhbU7CkNHJFAHvp5LkMqCEIocW3f26Y23y2OR67E7R3GUKCZZwRrmVEwRPGnIaa8eDFld/dSXJoupCy3+CIfsM1BMqkNRvD0w7l1Ym8toRojfOQSzgQ3BIma3sq0Knt007irZNG58SL8A8wdw0f8JF07UvGo86JlCq1/jd5oX7HxAYQ=="#;

const RABBIT: &'static str = r#"eJxtj7EKwzAMRH/lyC7VkhXbgTRfkK7dTTp47FDy/ZVDyBSMD5t7HHfzt/4aPs/hpQGFR3tbVSgCBIEMYY/DMj86tcwXKwUqjUo1ONJhMoezn+OHTLmxrhMnJLYqPEUc0k2BciG/q4xc0MktcveSO9k17XK26NH+aiQbiXuBelB0veuVOXm88hirA+cI8RLGtzM8K51kOGqpYbzfK9o4yOX9AakVRzA="#;

const RADAR: &'static str = r#"eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBQqKEjxQW9vQrWCtOzfw+x8THs/PkY6debACTaSR3IDWyo9lyIlcnDe9O2+PvftYvGkIyyvKAJJlKCysLiMMBxTAxdWLKwQTxFRByX9OBqIo6JoXHMIcbMRgGPNzQz94iqQA2L85+XLlK9nys/OsBjKr/lOnZH6NMs/9I0dfEFZhESh8Hd1Lbg30f1SxQ=="#;

const RADIATION: &'static str = r#"eJx1kLFuwzAMRH+FyM6rSYm2DLhZOvcjCnfw2KFTvr53DpouDQTZoEg+3nH7+vg+7PP18h5pkcd0uW4vertuj8yCso7cHc1RjtWxeCjobw2rFfOJZgNhhAQLA4VuoXPUPrni4Ylh6Q2dcfCurqj4Xt7Rb//MThJyx2yThQvpIRnJ3iCxzs7wBcoqIzJS8iRwPuskT1Nm2mgWSCwUqsPpR92eeI4VQ6blmTJoivMJEIgQ+r7/B+9kqiLYMU5/v6tik+fDo/SpiJvgt5s2IzR3Y+1eKY1/in4ABr5YTg=="#;

const RADIO_RECEIVER: &'static str = r#"eJxdjFsKgCAURLdymQ2UQmGg7qBFRErXv5BLj92nBBJ9zpyZY/dFmILDPJAaDw1vu1p524CafiTHVehy0KDbwYDOFIRL7kH57TmmjaXC8qr7r8+Q0tw33wOf9yU9"#;

const RADIO_TOWER: &'static str = r#"eJxtjk0KwjAQha/yyL6jb0iaBJpuXHuIEoUKClJc6O1NIljBMmQW837yDffpMeOUzNFKBHvhgaCKgnAS0K4SzTjsqnMcvn7fVD+ViEVb+zpdOXtxayBflnw9Iz+ToRrkVzLRYElGq+ej/tQWBC29ISvKiPYFg2yl1m9gMAor4RQl9mircrA+K9xIRHFgmN2/dAvQ8nlHwoJr9g0UsUhu"#;

const RADIO: &'static str = r#"eJxVj7EOwjAMRH/llL0Gu7SkUtKlMx+BDBJIDKhigL/HDiJtB+ek+O7pnJ7n1w2XHE4HGsAD8cTgjgSMSBH+axPGtHPnmKr/aFvuSbQRauv0xNg3kbolofdZH1fMOUiAfnJg13dRM/3WK7BDYXQ1Hv5TuNhwl4C19pKTtKWziV3A5Rp7auILj4s5Ew=="#;

const RAIL_SYMBOL: &'static str =
    r#"eJxtirEJACAMwF4p7kUrVhCqH3iE4OAiOPg/0kUcnAJJZLU9oGdTGYgHBVPEqivylPQNkwJ4h4wMEaPyPgcuEReZ"#;

const RAINBOW: &'static str = r#"eJxtzLEKgDAQA9BfCd1FL8h1OfsHfsSBg6OD/4+9IqVDScj0iD3+3riOdJKQ7LKhtmVhnVRsDVKsQw2n0KYEwqmKo+wEf7YP6gOKCxw+"#;

const RAT: &'static str = r#"eJxtT7sOwjAM/BWL3Sa22ySVSucufEQVhg4MSCC+n3MGulRRHvZd7nzza/vs9Lhd7lpobIlVCrZzX2926gV5SyRVohylkq2qjV0mSlzIRXGWbwLJxMCpNMS6LPM1DJb5sMkA8bFBVlwyy0jKUkKYJQOCWhDQyKh7OxqOsQKennGb1CYGrkkJR8ajcv9lkQG3UqLIAwtn0KC+amkdTUewsxk9JOsGNDTARioeT5lktg6bkXUmPHnY1U6D07RL0j/0Az8IU9o="#;

const RATIO: &'static str = r#"eJxFjEEOgCAQA7+y6QcUDp4WPqPE5Uo2UX4vixAuTdN2yiWdSpLyLRrgd9CTL5UA50HlbRGo6QGq5iNvBkTu2F/XXo/toO1onrqFfcd6Hu8="#;

const RECEIPT: &'static str = r#"eJx1jKEOwCAMRH+lmW8GTQMIhp6ZRcyRTCAQE8u+f4dBsTTNy7W5F+/yVLq25VCSV0wTtiSEnTNLY4AR/3guKa5dm+KQW0ehsisCh4HNYLTqyP2ie5g1YfXZj88Hf6koeQ=="#;

const RECTANGLE_HORIZONTAL: &'static str =
    r#"eJwBNwDI/zxyZWN0IHg9IjIiIHJ4PSIyIiB3aWR0aD0iMjAiIGhlaWdodD0iMTIiIHk9IjYiPjwvcmVjdD62dQ94"#;

const RECTANGLE_VERTICAL: &'static str =
    r#"eJwBNwDI/zxyZWN0IGhlaWdodD0iMjAiIHJ4PSIyIiB5PSIyIiB3aWR0aD0iMTIiIHg9IjYiPjwvcmVjdD7Bgw94"#;

const RECYCLE: &'static str = r#"eJxtkEFqQzEMRK8isrdqWZItQRrorov2EIEuskihi96fjp3woclHIBsJvRnp+HP+vdDX6+FzkOS7cYifhUNppYqQIuyjcISgOMLv+dbjWq3Mgn0MluyU7IfT8WVyT8eNLpjNS3Cr+oAHhN07+LnAds+3Xp3s4Vc8rfXSWNoz/Vsw1IvSih116CYklR0GN5ukDFcVxdhhJqsZOQ4y166psBDZ3h6OQyKcuMZc67/5OebWcbe4Kqcp4W+6Z19ZR8BSVyWDVl+KsfIoq7KN/QFn4mBy"#;

const REDO_2: &'static str = r#"eJxdjLEKgDAQQ3/l6O55V5uhcBbcuvgRgoOL4CB+v21BhxKy5CWxa7sP2md3KkgDYWhyycZKkn189UIxR8YCBlVLUygrxiNdXIrkJev0P71daBiS"#;

const REDO_DOT: &'static str = r#"eJxNi1sKgCAQRbdymf8eKiiCuoMWIRYY+BESUbtvJIoYmPs4XJfWmsqCdHoSklBZCOliMRTc8ODgtrhnzJ4mKWAOnTvdaGt/TEGYaGEx8gnYrvk3acheFcXffNMbfakgPw=="#;

const REDO: &'static str = r#"eJw9izsKwCAQRK8y2JtkXdhlYeMNcgghhUWKFCHnVwtlYJgPz9/yVdxnuBJBf6lRQvZ9rNnXxyAtBsPRRbA48myCtPHD3XWhDdu4FTA="#;

const REFRESH_CCW_DOT: &'static str = r#"eJxtjEsKhDAQRK9SZJ/MpBtbhRhwr4eQKERwISKit/cHupFaFFQ9nhubOaItVM2gRaIo737n5t3zkIWlMkeO/xVBYrhiZN8s0aIl6i8TH6LmFdkEYtKBNZn0pUM/haFD2AplSSGsd09HndB9+x2ROjBq"#;

const REFRESH_CCW: &'static str = r#"eJxtjlEKgCAQRK8y+K+lpiaYJ6hDCH342Ud4/tyEipCFYZhl30440pmxL2xTElIlD4+RhntevXCmScuscBNUlVVjZjEMdB3Dw9DQxWTT3XzhIPeDg+D8hlMV24FIW/NsyvvgArClLag="#;

const REFRESH_CW_OFF: &'static str = r#"eJxtTzEOwjAM/IrVPaa2EyeVSiXUhQEegcrQBYkB8X4uGboQJTrZuvPZN78fn52e5+GuQuUmhXOkBLhMnBM1GNsTJVtFyEhGHo2MxcFLrWIelvlUrZb5MCwkfrVv6lDw0NUwnYQiZVbHTo8N/uUvyJ20qir0LlPZlOOEJuL2IORsHrRrV5OKbpgLNYLwlAMSwLvkvhoh9tDLodiMT3pwP/YfT40="#;

const REFRESH_CW: &'static str = r#"eJxtjs0KgCAQhF9l8b6Wmn9gnjvUQwgdPHaInr81QUJkYVhm2Pk2XOnOcK7sUCBk8uBhphHgkXZudZXqGW4XkCS7FOBYDFO5jqF1kK0enVGPsz+g1PcALAD8APSNGZQ4sjdCtOgFyMktuA=="#;

const REFRIGERATOR: &'static str = r#"eJxtjLEKgCAYhF/lcJf85dTFnFtaG9qEBseG8PkzgjCQW+7ugy+e+So4ZrU6+EwQpkVAzfLbYBVmC/seurUl9Fvbze8qxekxpth5xRThgIhDqP4DN7Z8Ie8="#;

const REGEX: &'static str = r#"eJx1jLsKhTAQRH9lsN/c7FySKERrGz9CsLARLCSFX6+CT1C2WM7MYeLYTj26Mms04J/UZlX8bVkVj2ZQGh/gjENuvIf7VNTujrw4TQENLUHY7YTC2t0ZTLx4/ewfLEzC+RxeAEsvLSk="#;

const REMOVE_FORMATTING: &'static str = r#"eJxtyTEKgDAQRNGrDOnFbLKJFmtuYGsvWGwjWIjnd20kkDBT/SfXfiuOxa2MaWOl/ERXZPxykR8TgtfcAYpgzKatnZRgt3UseLOhthd8SCX1"#;

const REPEAT_1: &'static str = r#"eJxtjDEKwCAQBL+y2B9xVUhjrNPkEUIKGyFF8P05AyEWsmw1w8Qr3wXnZipXOAQE0ZsUlw5S/PDhQTZhVgqrYzcLJ2rVkJO3I7OSI+jbUOrm7icm1bSF7a88Zf4tOA=="#;

const REPEAT_2: &'static str = r#"eJxtjDEKgDAUQ68S3Iv9KVaF2tnF1b3g0KXgIJ7f30EdlDcEkvDCno6MbWoKMcIZB6WJoa19DPe6iIMMc58IwipiaLj677OQkK56jPJnEvjsX5EmT7HP8wKymSTx"#;

const REPEAT: &'static str = r#"eJxtjDEKgDAQBL+ypD90k4BNTG3jIwIWaQIWkvd7JygWYdlqhklnuSqO1TUu8IiIonc5TQZyevEeQHZhUYpZRzMrB2rTkJenI6OSJxj6r2TmFj7zBt9bJUM="#;

const REPLACE_ALL: &'static str = r#"eJyVkN0KwjAMhV8l5D6zyapusO4NvPVeumF3Icgo/ry9yQQVKYgUmkC+nJOkOx9ygiHgjj346IgrrloSEBLsu5WV++4FiQOJioBGqFr9ipRAEx0oR8ZQmeINsIu0qJGQuZY8TzVsobZHdUHFRPbrZfCtOipkYOI3Oo8xwz0ge4RbQEGYn+E6DTkFbBDSOB1TtlS7jP8c0wP7r50v/vd6eqo/+h6kV2Gv"#;

const REPLACE: &'static str = r#"eJxtzkEKwjAQBdCrfGY/NZlW20KTG7h1L2kxXQhSAurtnamgmxBIYPLyf6bHtWTMgc6+Q5cc+8Y3IwuEheJ0sOs4/ZA4SFICPdGMulWVYEgO6tgM15U/wbvEexoLW2ut896iR2uL20qKhVyO+8d7bVRkMPs/3ZZUsL0CCSEv6y2XQAPhO3gH8h3huc4l21hfmY8fjIVHuA=="#;

const REPLY_ALL: &'static str = r#"eJxlyrEKgCAURuFX+XGXUoTboM4tPYRQkCAqJEFv35VoKc54PltLulLMG2qJuR1OEBRBQ2kQSHg7vMLbn2XEmDru/qND27E6sWi+0yl1MDAYe9JIMz+ckb8Bghsl/w=="#;

const REPLY: &'static str = r#"eJw1ykEKgCAQBdCrfNxLKgMmqOs2HUIoSBAdSIJunwXxts9zK3fJdQe3XPsZhIO2IGgDByuin/4RPad+YAtiNQp6vqRJNKZ6SZK00NdHig9v0Rgo"#;

const REWIND: &'static str = r#"eJxdy0sKwCAMBNCrDF6gTIqLQup1SqEYoW68vR9ciJBFZpinyb7yWESyN+b/diR4QcB2hMfILugxh0F3IdIFz07a7zGKlVTOiRxj"#;

const ROCKET: &'static str = r#"eJxtUDsOwjAMvYrF7kfsxE0qFSQ2Fg5RlYEFCamIgdNjt8BUJbb8e362h8f4vNH1sLsUGEkHm1jCgnasZCFzRi3sQfcmVGG0gsoKyYzUu9HL6F6jRaV4SzCy791x2AfJcfhR3UVJjDPnUZX8B0JIOaO3kyhao1V/E140JW9elVEbVRh3JM6pyEarXkq5kG4QXnrvdy4zzFmRslOVSdCpr5oaBdhlA7cM+ooDJF/VjIpfIDAcaN/SQv7AD1TxScQ="#;

const ROCKING_CHAIR: &'static str = r#"eJxNjm0KgDAIhq8iHqCmsT5g2w06RFBQEDWoH3X7nLEIRQWfB14X9/Vel22CuC/beXisCgsMtUziNFrdGFyZ0eBUuNijlQ/ccrCRTR6VhUuu7pV+QuaSSG3myP7MT4jDOcPoseeiSSEGqkDaaCUZjEYSLDz/2DG9"#;

const ROLLER_COASTER: &'static str = r#"eJxtz7sOhTAIBuBX+ePenl6Qo0n1DVzdjQ6OJhqfXzBGF9KhhA8ClG06VixdNTBiO9ZVX36a6ssLMaiwbywjsdP9bWxQn2SDdrFBKemw1pIbJgIhyIsgR3OSSFbwOYNBu6L8T1FEcNmx5++qC7M8Pi8="#;

const ROTATE_3_D: &'static str = r#"eJxlTzEOwzAI/ArqDjXGGCylWTrnEVU6dKlUKf2/ComqDl04g++4Y3rd3g+4X04Ld2q9g5FeWak3gUZVDFhoaIUKnGXEbAQaBN0skOtW9yEX0CgrSatQgLoZUunxgVRP83ROq3n6Gj7DhUfL/VYMhJyjIe+Y5ej/VQvHPiW1NWmlkbtiBLXUNkE7IBhVoCAX3MPlQ7eWkTFDoq4hFY+cTm4SKmOP6xx+ph8MLT08"#;

const ROTATE_CCW: &'static str = r#"eJxNjEsKgDAQQ68Sum/tx7EOjD2BHqLgoksX4vmdIkgJeQRCIle9G87NHAkhVgbDI6jZanaZPvguu7g8Iyr2hNUUmfq4yHCRHmr0Ny/fYhZa"#;

const ROTATE_CW: &'static str = r#"eJxNTDsOgCAMvUrD3iqFqk0qJ9BDEB0YHYznty6G5eX97ap3g3MNO0eIXBUURnCKinowCbvMpMm9ieYM7LB5dwnFhm9crL9IjzSUP3sBG9YW+w=="#;

const ROUTER: &'static str = r#"eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PQR06RF4sf88vbMutYHtH6givdS75twkhL+s9l0ie8ImkRvhHpnCuD1N4pJIxR7o6FoX6i6tWrwdT2VGlpQNUntaSkb1hZB2TwSB1TgM7h1ZPJ1WMe0sefk+rcq+H+BelWTxR"#;

const ROWS: &'static str = r#"eJwlTMsNgDAIXYWwgKFePLRdRhvbxHggJML2Qnvh8b6Z2ymgBXcEm5edJIRvXNIL0oHQ27i7rJ8t3Jq36NX8jLeBJtcIQWnNOJAvWJro2UjVHw08HDw="#;

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

const SCALE_3_D: &'static str = r#"eJxNilsKgCAQRbcyzH/ECBqCuoMWEVNgUBASYbuv6enX4d5zHI+JpwGSR4XAu0eyJ/PF4OpbB/dkEmh86yyjiJZujdB7bDU0G6lISqy8v5s1kAVTmU8dqyUlLw=="#;

const SCALE: &'static str = r#"eJyljbEKg0AQRH9lsN+Ns8t5F7j4B/mBdMEU1wQExe93bazshGGaYd6r83dt+L26PwdEXAocZRItWYck1KeB4uAipnRRT+LCTzfWx/Ed60mwm4B3hrGxv1ho8I3lYnHkZpOhR7iQxRA6pOigNTsfO3vQOMw="#;

const SCALING: &'static str = r#"eJxtjDEOgCAQBL+yoSfmODFcclLT+AgTiystzL1fbaCh3dkZvc/HcO3hSASGgHKouvxr1c4ogRs7FaPiUWaPDWzZp/L6RZt4HPAFjH8g1Q=="#;

const SCAN_FACE: &'static str = r#"eJxtz7EKg0AQBNBfGezXuKPHRbhYp0mbXrDYUlDu+z0t5JBlGViY10xa592wfJpfj/gPM0F05RQUGpspvU4xpdtpRG+sIZg9SIXGXEkpn4lHI6jfUEthduUbOmzahiKHM3LFgSNGazv1BoRHdQBC+EI5"#;

const SCAN_LINE: &'static str = r#"eJxtyrEKhDAQBNBfGbYPdzsi28TU11xrLyhEELGQoH9vAiIpZIqZgee3YY8YO/k3sL4dCOKbo6BjpAT/KSL4x6mhiawhmN4gFWqpki6v6N6ogfpra+mYarnM64STnSgFR2kTnHr/3FZsUeECrNk0bw=="#;

const SCAN: &'static str = r#"eJxtyiEOgDAQRNGrTOob2GmaNaUag8U3QaxEkJ6fYsiK5psvXrnbY7i2cCTomRtBrCMBI42hluUTtfxOFMnoIdhnkALR7mQcZ3FGFZQ9exnZnXwBlDYpaw=="#;

const SCATTER_CHART: &'static str = r#"eJx9zlEKgCAMBuCriAcoRpk+qDfoELECgx5CIur2qTNCqJ782T63aZw9LhPDw3BZCc7wzMEbHh6raxJWZ0n15EQK4SeoV5pacGtKX2OLA6D7pVG0j6A9srTrsDk2Gt43rNlBOVCxG6v2AkVWQ20="#;

const SCHOOL_2: &'static str = r#"eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRh7P9dKeL6bak+QrL4NhBepmQ6dNU3Bi3XzzG+/TIcBncUQSETn1GP2NAjwH8QZRlsj/QZ2zLHOoHyrmElZAqKgDvdUO8jnYNxH3bVljT50FEsfv3QuulVN8g2v2sb/KFSRc="#;

const SCHOOL: &'static str = r#"eJxtjMEKAjEMRH9lyL3YhLhGaHv24kcsVVBYQRYp+ve2CrrsLjkkkzcz4d4/LjhFuik6mFMYlFLYtH8KP8oG9lBIsV4g8HXY1eugU+2kOBvUybLiyDVd8d/vP34/1bVf16KGbeHdCulmIF/HPJwxRhJCfkbitl+R9s3zpekNNac++Q=="#;

const SCISSORS_LINE_DASHED: &'static str = r#"eJx1jkEKgzAURK8y/L1p/yeGFJLcwEOUtKCgIOJCb6+JiCK6mc2beYzrv2ONn6eqVFrwSWHBQsG9EgouNkNs/4iTJ02IsydLGDzlygaD2y0da5jCqtIixZMlz7OLzZ2m4rdaX+gc4JPnaJj1ZV3IDRK5oAXW1z4b"#;

const SCISSORS_SQUARE_DASHED_BOTTOM: &'static str = r#"eJx1TcEKwjAM/ZWQe+sSaonQ7uzFq/dRhQoKMkT0781anAU3Gl6TvLz3wn14ZDhFPDhgHhgYOn1k2PDR/WbQOZNvF8DPdmG0wz5sJsM+zLbUqe9elhivTDaNKF3GdD1DekUUhPQu3xixnFSy0e+sIwcVibWWMpwVqDDVf5RmkMdv5FrWTdXeiN2Kh4Kz0QeNlFHB"#;

const SCISSORS_SQUARE: &'static str = r#"eJyFjWEKwyAMha8S/D83ixMH1hvsECOVKbRQRGh7+yYKbf8VwgvhfS/P5YAFYkj/WHrRvQRstATkta6mSxpKrK53Tw54hynjGACJtkRXDFc+CGmmd/OvRBh68f1IrTQ0VR0NU+xeGKWlhSY8J3GpUua2a6K0eVj5tgaqHo92zOk/pQ=="#;

const SCISSORS: &'static str = r#"eJxtjUEKgCAQRa8yzAFMzWIW6g06RFhQ0CKkRd0+taFaCMP8xX/8Z8MawzZDdNgihNNhn+LK4W3zlN7u47HA5HAgoTSUVy4zufsRWoJhohNEH8EiVrAviRRVTcoIgvLSopbvzg2m7i86"#;

const SCREEN_SHARE_OFF: &'static str = r#"eJxtjDEOgCAQBL+ysSdydyAUSG3jI0gsaEwsDO8XGiWRbLGbnWTCle6MY512EshmEoOhW1RdhfR31OZMS38oLkqmGOZmieF1eTBlPwDEIFfMn5zMEGVhB4gcBD16ALRLLkM="#;

const SCREEN_SHARE: &'static str = r#"eJxtjDEOgCAUQ6/SuBP5HxAGZHbxECQOLCYO5p9fWMRE0qFNX9p45bvgWKedDMxmM4Ohm1RNQroX1bnQ8i0UizJTinN7SfH9CmAqYQCIQV7sn5zkEeCUG408THHS0QO6wy7K"#;

const SCROLL_TEXT: &'static str = r#"eJxtjLEOgCAMRH+lYW9sK0RMkJnF1Z3EoaOD4fsFE4kDuenuXl648q1wbmb3IKwsWUCA3ghKQUlM5VsZGC3Q4Xqn1sus1sQwNVcM3cgr8NLZGqzGNCQdeEU3flj+1wOvSiwV"#;

const SCROLL: &'static str = r#"eJxFjDEKgDAQBL9ypD+8WyMqxNRpbO0DFiktJO83ETzZapZhwpXvQufm9oWgRZFBIHkHRmUklfq9Ssqe5JiMpXMdi3cxDL0VgxV1JZ3NbeNWTL/5AA4dHTw="#;

const SEARCH_CHECK: &'static str = r#"eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf29qoSx7WGYZvuZ7g0XwzEAE0ZNCQuWxAmXbix0r2CtIhFAEM4I9//JTw8rdEt1BIQ1Tbdd8iMcbaw=="#;

const SEARCH_CODE: &'static str = r#"eJxNjMEKgDAMQ38l9D6l1YODbf8iVVBQkOFB/95twpS0h+SFuGM8F0yedgtrBEUUXJvz4CrlDunEFH1c16jbDL09MROip4GgV3Gp9OLfjDCETd90+evMA0dTI4U="#;

const SEARCH_SLASH: &'static str = r#"eJw9jEEKgDAMBL8Scm8lrYEe0vxFoqCgIMWD/t5WoYdlDrOMnNO1wpzxoOgZkmfHwKgyNKFiW7F9gZIxIdidkajy+VhPv1bplUAQyI0+tvXMC5TrG5E="#;

const SEARCH_X: &'static str = r#"eJxVzEEKgCAUBNCrDH+v8TXBhXqX+AUFBSEt6vZlgtRiFsMbJuzDMWOMtLHVDl475eAoha5ACo19VfxUlizrBLkiMRNyJE+Q823PqPLnxDAMq17bknZzAztoI4A="#;

const SEARCH: &'static str = r#"eJwlirEKgDAMBX8lvF0l1cEhyb9IFBQUpHRo/76EDscNd+JP9vcirwpmkLfhrNhhsoxs8h/lplPxJabE0zavQRxRrAPOghNp"#;

const SEND_HORIZONTAL: &'static str =
    r#"eJw9irEJACAMwF4p7iJVEAq1H/iAm+DQRXDwf7SDEjIlvPpWGMXNBAb5KyB5ak44WBV+T82AUTH/cgD7URB8"#;

const SEND_TO_BACK: &'static str = r#"eJxVzEsKgDAMBNCrDLmAphR00XoDDyEqxp1I8XN7jZ9WySZM3sTNfRuweWJLkH4cJHgqCfMZGcJ+H9axC6J55TItVO6t3cb8ulE/X2JpaoKg81QXYLtwY2CQX3NuwgqVfCBbFJIgK1wSPACvaTMs"#;

const SEND: &'static str = r#"eJw9yrEJACEQBdFWPubL3e4Jh7DagQ2YCQYmgoH9owbKhPO051FRvGkiEPohL1lyK5tM0GfvoAfFjcAM/u6cOUoQ/w=="#;

const SEPARATOR_HORIZONTAL: &'static str = r#"eJxty0EKwCAQA8CvhP1Au9siHtTf9CCICu1Bf19X6KEgBEJg4lLMF5p4EiY09nQQ+pgso3l2cJuq4GpJffpaYn5uTxYWLDjBBlbdJxZ2EI1Adn2ZH38B+Bwmhg=="#;

const SEPARATOR_VERTICAL: &'static str = r#"eJxtyjEKwCAQRNGrLHuBZDZBLNTbpBBEhaTQ22cVUgSs/jA8l2K+qIlnCFPXCrTwfDA1zDu4bajgakl9+lpifm7PliydBNHADPeJhYVRQ7IPrsP++Av39yaG"#;

const SERVER_COG: &'static str = r#"eJx1kEELwjAMhf9K6D2xabvZwbazF6/exxQqKMiQof/exHnopKOHkL73vjRtx+s03i4wdcYbGF+dYSf1/a19u1vkvn0MzwTnzhwDVcD2EAYHDqwcRofulPUgfeI6vwA3rwLgElKlfOX+00NGt7gOW4XldKvjZlwlcAtfQ53IclHhWJbuXNEe2FNAapB8wdGQkz9RdcMgogwQDnnxlAz+Z0AZw2VChEbjZT2ShwUijlI+6BIK2TLIghB1gM9e+AF+XIcs"#;

const SERVER_CRASH: &'static str = r#"eJx1jbsKgDAMRX8luEeTWIJDdXZxdS84dFBwkH6/7SIpVDKEPM65/g5PhGPuNgWm1QUBAcrFKCi7mSHPkdUuQFIFgESUbvFDkS7eqp1RE9YkFZNVU8lKWBH469bYE7dTp/bp4hEUXSb1LO37eAFErUAW"#;

const SERVER_OFF: &'static str = r#"eJxtT70KAjEMfpVwe2oTm16HeouLiw9x4JBBwUE6+PQmHlwVSijNz/eT1Of6UridpusMrHRcGRiiBdnPLfUaLVOUaakH5yx1Z5JNI3AQf2cn8FaBtC4Yvav5PRBgBpobUscio5nRAJuA0i/ub8nNg3KQO2EQDyxYLmnkmoGKhjgweXwvMLG4zz5i60de"#;

const SERVER: &'static str = r#"eJxVjlEKgCAMhq8iO0CpRPigXqYkhehBhPL2TWdWL/vZ9n1jOrolsTOsyRuQHFjMmMCoXrVGCu/C5pMBBVaPRbO6yoiKqVPN/1x8PbrX7T0cjmVpYMaNoCjdwAU+UAfIFqqxDSoroX6wrJOHvgFlWziy"#;

const SETTINGS_2: &'static str = r#"eJxFyj0KgDAMhuGrhOyi9YciNJ1dPIREoYKDFId6exOVdvpI3sedyxVgJZzbBmyoRvSu1p93uZgejJ2GUniPfGwQCTsEvgmNlU3vCvpyZgqk/zrpUdADHj8kWg=="#;

const SETTINGS: &'static str = r#"eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGekhp0v9L/ra7Hu5nOPZux0IiIGekEA66gSYu1N2DePM7YWRgWvtJ6zxJWyQ06im3SM2mVNDqfEGEfJEBPYWYnfQS8aIECaHcli03JAI12wKwyFjAjLZNcYyTHSBj/V6aUkItqRwBQXlgZUlsLmGCx1oxYWK/tiv89ovkzSjJr38fFREhQdGtJYfvgNva21kpUeyL4EypIfkN7RyZ7ffVx0B5u6FbxQ80dOPlNk4nuPXOOxifvWPR+EpRi3J6+AA5HX87"#;

const SHAPES: &'static str = r#"eJxVjVEKwyAMhq8S8t7M1LZWUE/QXWBvxcoU9jBEWHf7abeHlUDyk3wfMc+1RNgsXmeSwGIlRQpELe5o6qeOSSi9MNMA8u8ITKznjsQgF56qOpM+uaPqG3NDZy7thzM5+AIxpHssFhXCK20lHultkQeEvNeJULtsVuOd8Sn7R4BclzQi+MaqI+2/VNEv5D4TbTI9"#;

const SHARE_2: &'static str = r#"eJxtjDEOgCAMAL9C+gC0KioJ8BnCQEIcmOD3toAJg1Ob9u6Mj9mnILKFHYQvFvCmWS0ocGbpX2cGxXfcOndCl36oURlNdvSMpfgEUTY6K3lQraCFWyoNotKGu1RIK/8veTSTjckbMBMrE63AMeyJkwuf9gKUgzt3"#;

const SHARE: &'static str = r#"eJxNzEsKgDAMBNCrDNmLJmjpou0NPISgoCBaUMTe3hhcSBb58CYhD+eMMVLfguXyg0DQWOk0s/wPlVyVpxTqN5RC3teyLtuEvC/beURiB6dfNODhzH0iBXOFIwmhiNKOcL9d95utqzf7AO7oJlA="#;

const SHEET: &'static str = r#"eJxtjEsKgDAMRK8ScgFJRVBovYwWWxAXUrC9vU3iZ6GrSWbejN39lKA4bBH27NAgHHFOwSH1CMHHJSS9szKFmdE23BvtGjcPmSQqxuFQhURy/QwxycxNqikMdVph1YUvO2j0rpa/1cuUhqzS06zvzZ51Szyd"#;

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

const SHIP_WHEEL: &'static str = r#"eJx9kEELwjAMhf9K6L3VvBlqod3Zy67epQoTFGSI6L833YSJbrskkC8vebyYz12+nCi/kmEYys+hd8lsTR1XA67j7XBv6ZhMwyA8vJMCy3BEVw4kVhwqKuV/oQGI0dpZNYdebmfk+po3TvbAFFyXx1w570n00sRKcAqwW1D3VejL3icejQPq+jemMaA39mdX0A=="#;

const SHIP: &'static str = r#"eJyVT7sKwzAM/JUju1VLVuII3EC3Ll27h3To0KFD8ffXdiBQ8FKEDj3vpPReP088zsNNILzRRCOYBAypUUNf0QmqM4WSM1mp7xP/rwxLOlXZJR3ibBRmiL8w04QGvpkwWF/OnDqDbpXXyBQjBS3xzIgUpy4jONzjWt7aqZw4uf7kkNzdLCf7rP2O5HA0vqsaRnA="#;

const SHIRT: &'static str = r#"eJxVjrEOwjAMRH/l1N3GdtyQSKUzA6zsEQwZGBgQA19PEoaq8ubTe3fLq7wrHqfpasIhIbBHaIQVh0PaKSXIJXC0ERaDjb+QcnAYW3jyPMhjUeg/BOfMyc/xo3JvElbO6KjVtClgZDeVaqzznqUGdy0N7b6Teud3WpdDH7/+AGDDKHk="#;

const SHOPPING_BAG: &'static str = r#"eJxtjDsKgDAQRK8ypF/MJsuSIuYGthZ2AYsUFhbi+V1TBASZZpjPy2e9GvbZLYqACL1ZajDru8y1b0Bh1YMiyeZKnt53yYNh/8bpp2AF+yqQDmFK8GP1ACINICo="#;

const SHOPPING_BASKET: &'static str = r#"eJxtjrEOwzAIRH8FZecaqF0Hyc0f9COidvDQSh3y/wr2EGVATNzjjqv/bW/0eU6/TCKUuExrvXVxrScSc8Yhe6mjpnPgumNECh5UkDYlpXmMdq0ZljfMV8HCKKTs6ldQ2K+DOBthZEGH1D9l5CY5at9r8NV5AFOAPuI="#;

const SHOPPING_CART: &'static str = r#"eJxNjUsKAjEQRK/S9H7a6XLygyRrNx5iiEKELGRwobfXqAOhoRf1HlWx3LbSrlSeiT1TeSWGMm2JlXM8/GiOg6Xhj3d50O7ro9Il8RkyG+qvokGsJYUsWEGg+XsgFeNrEOeHVCWY6QNcU7FmcrIcT0YUfaN35zf/nyub"#;

const SHOVEL: &'static str = r#"eJxljDsKgDAQRK+y2GfdbFw/EHMCPUTAwkJBUCw8vdkUKZSBx8B8/BGvFZaxmhmYbyObGIEk5VMFX2sh+FIbUMA2ihb6f77bDjjvUVCiQ3GQQUk2ndJJ2dM3AzIy6bqcvtw6JLk="#;

const SHOWER_HEAD: &'static str = r#"eJx1zjEKgDAMBdCrfNyt/W3aIlRv4CEEBxfBQTy/reAgNHySIS+E5HO9dmxTdwgEzoRa3ZyHOp/zpwt9oWjCKmYMeJut6RNSaz2ghs1TAqbbWLbMglEzD3rNImg1I5xqCRTFXPmFP3sAkGFQiA=="#;

const SHRINK: &'static str = r#"eJxtzT0LgCAQgOG/crRrmh94YM4tre5Bg4NCQ/T7u4NyEvSG4+G9eB13gXOdmnZAz4Nvwgv/WBmaEjQL/SnFmWGKP98RNMqQtWsK1GblwhtXBQUGmspEMjLmIC+w0p1hudOvi2DAdPkCPSQt8A=="#;

const SHRUB: &'static str = r#"eJxljbEKgDAMRH/lcG9totYOVejm4g+4FRw6KDhIv98UUQcJSTjeceePeCasQzUTgzmrflOsuBp9XcjoP97DZe2ChYWRIVCjHdhkM5GcYHWHsjcUdzKxw6NJ3vKP3akFtUq6X3YBbJUjoQ=="#;

const SHUFFLE: &'static str = r#"eJxtjrEOwzAIRH8FZedasE1syc3cpR9RpUOGVOrQ/1ewo0zJgAQc3L36e/8X+jyGl5LkRRBnQaA7KRLDKCCwYFwNwhk2Y/RRSNvyEJ+qw1RvzWmqh99XMilFiux1lj3OPK14WuppBcX9zDu9uu50nPyBOx8rjB1mR8grN9x8jSHxxLEBS9c7Pw=="#;

const SIGMA_SQUARE: &'static str = r#"eJwly70KgCAUhuFbOZw9w34V1LmltT1SOkJDiJTdfR0avuXjeU0KW4bHYotwR5/JolQIqVhsECjEnfJ/FTbO1Bw4c66ZwFuc5QBK6GWc1NFBX30jdVVSaLas3AtpbBsL"#;

const SIGMA: &'static str = r#"eJyzKUgsyVBIsVXyNbRQMA8z8TDLMVOw0AXiDEOjMl1jJTsbfZASOwDmBwru"#;

const SIGNAL_HIGH: &'static str = r#"eJxtyzkOABAQQNGriN42kVAMN9DqJQqlQpzfUtBM+18+9jIaq4EnYKCb1IZHVCdGfOQ2TWEJMXDJU3Su/GUBNKMdsA=="#;

const SIGNAL_LOW: &'static str =
    r#"eJw9yTEOABAMAMCvNN1RjcRS/YFHSAwdDeL9WLreyRrbYDbsDEwWKaNK+qjiVV+dUHwuj+oO2g=="#;

const SIGNAL_MEDIUM: &'static str =
    r#"eJxtyakNACAMAMBVCJ6vIQFRugFDkCAqEYT5eUQV9g5Hm6x60RUUeLY+aEJ3kVAqnVomfibAqyy1AYslFmk="#;

const SIGNAL_ZERO: &'static str = r#"eJyzKUgsyVBIsVXyNVIwMsjQMzBUsrPRBwnaAQBpcAd7"#;

const SIGNAL: &'static str = r#"eJxtyysOwCAQRdGtTPBtmRfSVkzZQS2eBIFEENbPR6DG3pMrJdZM6TM/CDaflo2Xa0Yvm55B7XCKMBa9Gs0raAKQa3xv6ZzTJRQ="#;

const SIREN: &'static str = r#"eJx1jbEKwzAMRH/lyG7XOiI7ATdzlv5AN0MHL4UORd9fu5TQgos4gXT3uPwoz4rbebokCItCEdoI1KmF7xtqcU/m4nXa8qljWz5gBUMh+AnTscrPAzTuao4jmtK6qwwcWbxibpIFOiL/gr1xYNxnv3LFe/sUUtcYF1o8nBebI0wl"#;

const SKIP_BACK: &'static str = r#"eJwti0EKwCAMBL+y5ANtpD0I6m9KESQR2oP+vjH0NDswm7q2eauga5X3ycQRYUcEB9g84E4lbX9YUqtyYXCmkzB5PYzBdTgsXk35ALQHGMw="#;

const SKIP_FORWARD: &'static str = r#"eJwtikEKwCAMBL+y5ANtxBwK6m9KEcQI7aH5fVP0ssMOk4Y2u7RjaO3PnUkQwQIOEITdJ1JJ26pKarWfsJCJD8LLk+YUv1N7/lflA6LdGJY="#;

const SKULL: &'static str = r#"eJx1jVsKAjEMRbcS8p/aG+lYoR1wAbOIoQoVFGSQQV29qYIjiOQjj3O4SeU4ldOByj0zlGmyxlRumbfcp9Wb9umvhfCtXcZrpX3mIZL6WWucRRtv94WeoS4QNuKCQNpY8fjVBnSWMiop+VfBhU7WTgNFK9vJC+BgfLdY7fUn7Amk8Di/"#;

const SLACK: &'static str = r#"eJxtkEELgzAMhf9KyH3OrBYUWmE3L7t6lymrtyFlq/9+iW7qrIdAaZL35T0zdHcPo8ULQrBICuHdt95Z5NcgP4lGcF3/cN5ijqU5y0Zpno130Fq8UQF5omtKHY9euUAqBZKaerIk46X5wSjbaq6cFR2OYSyuBZZVKoLNvR3sq8iQcXa3sML/GerImhZAUVH24pVm5y3RJ4p4UWRqIue4uSRI2jGNkwJdUVrH1qbWQvoAAcNwLQ=="#;

const SLASH: &'static str = r#"eJyzKUgsyVBIsVXyNTJSAEMjJTsbfZCoHQBr6Adc"#;

const SLICE: &'static str = r#"eJw9jDEKgDAQBL+y2CfmsnJGiHmBfsAuYJFCwUJ8v0bQZouZZeKRz4J1bPYA6YxCy3AZNim21aT4+VmCZQ9aJep1Izh5qUwflr0Vj3ccBM7QcPkrNzZ1GKw="#;

const SLIDERS_HORIZONTAL: &'static str = r#"eJx1kEsOgCAMBa9iuIAUDWGB3MaFiXEtt5d+0KK4agrzpoW4b8c6ZLeY2QwZqJylA6yldWBSHBFKUaN4B5bZqSZbkjTOUIIqcP3V1svQaMvpC8W8jAbPfup7XiVCRrMfMR2q9yPsJdvRBtnP3ltnDna+wctI0YHA7vFe335ibA=="#;

const SLIDERS: &'static str = r#"eJx1kEsOgDAIRK9iuIAFG+OiehsXJsa1vb20UPuxriaQx8wEdx7XPnhcgRCGm1awLBjF84QWNjcGaHOC8nL6kCxoOiSSMFF11rAG5uVSMJoipzUalmSgKh3ncDu3MGU4lUnw1DNW1Irv/PeFoq5+atG6puOKyhK9NSUm130A3zJibA=="#;

const SMARTPHONE_CHARGING: &'static str = r#"eJwli0kOgCAMRa/SdO9QgugCuIGHMEIsiQtDmqi3F2T3h/dsjrvA61AhcEwHS4kjwp2CsEPSCPn539ygUib0dqiet9cmDMHhSqo3ZoYFaARSrM+uDbqylfIfG4scIg=="#;

const SMARTPHONE_NFC: &'static str = r#"eJxti1EKgzAMhq8ScoDMtDVVaL3BDlGmrL4NKWy7/VLL0AcJ+ckfvi9sy6PA9onICN+IgpCX9ZmLPgzCe51LjugRlDA4hVvlp/BKJcMc8c4WBrImeXIW9uh0WNeTlSpU9CwIOQEhw4mZvEDLv6WtH660kUYGR2r19WrZLOr2fmg/tXozzQ=="#;

const SMARTPHONE: &'static str = r#"eJwdi0EKgDAMBL8S8gBtioKHNj/wEWKL6U1KQP29SU+7zO6kXk+FN+OK0C0igtR2iVoNCN8gTysqGWmxzyCcZvc43YcKlIw7RaBNpkC+OeUfdW4Y6Q=="#;

const SMILE_PLUS: &'static str = r#"eJxtzlsKgzAQBdCtXPIfmzsYaSC6gy5CaMGCiKCI7t4k4guEecDM+bi+r8cG31J9REBOrGkQKgxQO02jKv+KqPIHfYP5wMxCkMfWqU/Y/rsfZpbKKSyS1hxXZhgO6R5sVBdLu/8Spt203PQRgQVsUzxko4NM52MFfnE5VQ=="#;

const SMILE: &'static str = r#"eJxdy0sKgDAMBNCrDNn7SVFQaHsDDyFVUBARddHe3qbVjYuQZPKi3Xq6bYbzhlgRzthqggtptbrKd6uP8V4wGRo6cHNx2UKhkSpSCRVi9bbuMwIb6glexVbWHKccBAnEinqtIG6T+t5Ykr9+AAKHK6o="#;

const SNAIL: &'static str = r#"eJxtjdEKwjAMRX/l0vfVJp21g7Z/4KvvpQoTfJAhon9vsoE6GCG5JCfJTff6GHHO5sggXwMCHEiTpfQSc9tFUYbO3Dx3pqSd3pbUrlO7XTBlEw3aKxtyom9Rr0sLLunPiGkkbr3V57HzNkiNp0NdDNROyDP8LL63FOFBgyXsLW9wZuHs7LDiHynPN0s="#;

const SNOWFLAKE: &'static str = r#"eJxtjD0KgDAMRq/ykV00oZQObe8iKCioODjU29sfCx06hBDee7HHfq0I7EgIrzjitLnsEG8R8nZMlrfFrVKBua1R497zs2FxdMoE1oMaFOIknkjDFQwyQ4+yxp92Y4P4PgtN/QFnXzZF"#;

const SOFA: &'static str = r#"eJx1TbsOQEAQ/JWJfuN2HCE5ao1WoZMolArZ70fDXXKyxWbe4VjPHVtfTHTo5mYlCPecUDgmGLSqGEL5RIbwBaFq9We8P3dtYkJoEjnEw9ldbsKUXDL1Htoac7vuT1HCW/cKFwOLOFc="#;

const SOUP: &'static str = r#"eJylzj0OwjAMBeCrWN3ziO38WSqdWbgAWwVDRwbuL5KKoiLChDLZL++Tx/v8WOh2HM4sJDwbGfn1mbOT7meyyzCNh1aYxnct19bCvpOwIVJThVIvTpBIeoVkMAqiItf/0OTgE4o6mNZZHJPAS91G5AINbRmQW5ikJ/O3HDY57mVY2ey02vqy5Zf979HhE34C3W9WWg=="#;

const SPACE: &'static str =
    r#"eJwVizEOABEUBa/yov/WsxENao1DCIVSIc6PZmaaCbOugR5VsRb0m81AO9EOFIL5b6+N8OqCW6hS+N6VDrfeDkU="#;

const SPADE: &'static str = r#"eJxVTssKwkAM/JWh98TdZJetsBb8AH/AW1kPHjx4EL/fiZRCCZP3JNPf6+eJx2W6VZyHZK0gxOFq9FXrlYiI9LfGzNLIOrNwYVW4b9iYMG2w6MHWI5OZECOJqccnKeLReUkLQ7tPSz+FoKXvsjJPz9+yT36hoiZB"#;

const SPARKLE: &'static str = r#"eJxVjjEKgDAMRa8S3FNNirSF2hP0BG4FB4cKDt4fEzNICQSS93lJvttzwrFNFzF4JJdgdbExMCxSJBuOAbTH6oG4C5Yx/RGDXw9VLExdOA4ao2gaJvVoAAeRYbu4TyXP+lt5AdHHIfo="#;

const SPARKLES: &'static str = r#"eJxtjksKwzAMRK8yZO+0I9W4BjcnSC/QXaCLLFLoIuT88YcQE4xAAunpMeE/rTO+r+5HgRr2Pk7bP6mTQHCPxbgVZ5H7qKAsGUCGTww1Fi3CJRPmoitEkY7C5CuIuQhNDaZ4n24ItxR4CEfst4Vuj8aeHnTNi8LOzQ8H+uqyAzJIQrg="#;

const SPEAKER: &'static str = r#"eJwljEEOgCAMBL/S9AFqifGEfKYSISEeCIn4e7dwmmZ3tr5GbdRP3pkq4IBvIMV8p4ZzY5rJm6+WTpaDg19tF7zmqiVSHXvFXuApdNlNmnXwJT+Rusy6O+OyCf4iOgBngG9e+AG+ACen"#;

const SPEECH: &'static str = r#"eJxtj7EOwjAMRH/FYs9hx3GcSKUSO6zsEQwMDAyI7ydpoROyfIvf6c7Ts73udDvszgWFIr9DgjwEFbFFKI3lZSIkp9D1UqBHgzot8r1SQbTrwAqyZVKwJRJKMGsGH7SvtBA41k66lVPsobt52o8e87S1kdrriKM0h9HYNQfMGoSRWf+5nMSadlp/jtCzrD9V3TbDBz0FNY0="#;

const SPELL_CHECK_2: &'static str = r#"eJydjjEKgDAQBL+ypE90TwkpYn7gI0SLNIKg/8eLkFSpbI5lZoqL1/ZkHIs5PejhLQW6xKQ4FJViDdagOIeOmCHc6YgRei0hbrK8C/k2OhaN1Aa/mvbOC4BbK4A="#;

const SPELL_CHECK: &'static str = r#"eJxlizEKACEQA78S7OXORRaL1R/4CMHCRrDw/7gW2kiakMnIKLOhRtMZjsHWEbSRSfJtlOQcctC5hRd09egHabz1ly8+OBfH"#;

const SPLINE: &'static str = r#"eJxlzEEKgEAIBdCriBcoByQCFTpAhwgLClrE0KJunzWbYFAQ+Y8vvmXfF8iKCcEvRerj3oqMJk1JTSrFBQX+qWM6V5gVRwbqBkoQ28ZQvPD1vcIedr4fSg=="#;

const SPLIT_SQUARE_HORIZONTAL: &'static str = r#"eJxNi70KwzAQg19F3G56d2n6A7bnLl27F7fgQikdQkjePjKBkEFIgu+L/+dQ8Upyv8Cut74EgwYPxvjjXJSfG3y1kxwPjc9xs+yEvnaFEhzG+GhalI41B17DTvt+fm/MnsRVMFkSc7avPfMfG9uovAAuqSVF"#;

const SPLIT_SQUARE_VERTICAL: &'static str = r#"eJxNjL0KAkEMhF9lSB9M5jx/YHdrG1t7WYUTRCxk8d7erMVxRUgyfN+k9/Uz4ZblPOJwGaupw5WgcnKrDgPhMWyDlLTpeEmL5Ef4rg3VQvIugad9jdOUGj3Kpivv+XjdMTOLU/D1LNtY8dIEs//jYDtVfjf5JWE="#;

const SPLIT: &'static str = r#"eJxtyzsOgCAQhOGrTOhBd5eXCVJb6CFILGhMLAznNzQ0munmy5/u8lScqzrIQ6prTuU09TOnQRGyya8Qg7npaKRYWMx9mgwF1mxi4F0g3+wihwVe+0EvvWIjqg=="#;

const SPRAY_CAN: &'static str = r#"eJx1jssKwjAURH9luPvETPoy0PQP3LqXtpiCBZHg4+9NNkop2dzFOXOH6e+XGDB5OVWogjaUoT9kNvQ/06EpGBJdQVVF08EVf8itesxjxMdLI3gtUwxeasHbCxMI83INMZMUz8F/00oHBwv7pBkNdKt0DSqCQbWj0i2MYmLp8EzerLL7OSvTnhrHonMb9wXna1ky"#;

const SPROUT: &'static str = r#"eJxtjjEOwzAMA78iZBdr2bIdA2l+0EcE6ZCxQ6e+vlSHZgkMwYRIHLW8tvchz/v06JLTYWlal1vs1uXvWKK1V1TNqJi1waXoZXSgyoDvBsMsMRmZU6Sgaxa4FjJcnRwQgqxo5IYcWriPOL0qzpokNSo/V1c5TNrWpTPFR4BJNA+lKCSikRFgF1PjNb8i/p01oTtMw3MMyWfHF64iPXg="#;

const SQUARE_ASTERISK: &'static str = r#"eJx1y0sKgCAUheGtHO68h2XoQN1Bi4iUdBCESI/dl04cNTrwfxwV3ZpwaxoJ8ZuB4F3YfNLEJOEpcAWbfAlGdflg1LEkD6tpZgPkWSCnCrtsJzAO0fA/7CFQ8QWKHiY4"#;

const SQUARE_CODE: &'static str = r#"eJxNi0EKgCAQRa/ymb2k1qKFepdISRdByEB1+xyDiPdX7/FdTSujXp4s4SyRsyczE5oYCTmVLfNrbjHBDXII7lg4I3rajYbRyqIjXcq/T2izqvP1B7d6H/E="#;

const SQUARE_DASHED_BOTTOM_CODE: &'static str = r#"eJxtjjEOwCAIRa9C3E2F6NDEegPX7iYdWJp0aHr+goskms/Cf3wgP+1luA53YwAMnqDLlbwpKXnwCFLku2ZeExA2DQcR6tCZRq9BxmgNoM8afnm27rKWcQH0GUt+dzwxuQ=="#;

const SQUARE_DASHED_BOTTOM: &'static str = r#"eJxtjLEKgDAQQ38ldD/0QjsIZ//A1b3gcKOD+P2ei1aQLEkePNvb4djmtBRQG0GMERUK1/JuxHbN/QGe/SHRUrXhFlZ7tFNoXX+A5i+5AIJlIUs="#;

const SQUARE_DOT: &'static str = r#"eJwli0EOgDAIBL9C+ICpXjy0fAaJkHgiJNrfS9vLbjI7W1044Gt4IPSZr12hDcuJ4Ml3BBW7NSaiuo0DVTbnR4DTKKlwX+1ZQ1oz/eylGo0="#;

const SQUARE_EQUAL: &'static str = r#"eJxVi0EKgDAMBL8S8gEbFfSQ9Ac+QmwxvUkJqL/X6EUvy+4OwzUvBprLqiZII8JekulbD8EO4Xyy3qPFyI0LkbfZFJLgNAAFpeDEvz/pv+QCc6keJw=="#;

const SQUARE_SLASH: &'static str = r#"eJwli0EKgDAMBL8S9gPSiqDQ9jNabEA8lIDp703obYeZTb2eQpqxglrlu0lG2EEfX9Lm7GYjaHhT0uKHkh5+K41gxWYqZhwgjRM1OFrqUfkBSzAaYA=="#;

const SQUARE_STACK: &'static str = r#"eJyNjDEKwzAQBL+yXC9Fd4gQg6QfpE0fZJNzF4xI7N9bUuHClZuFHXYnfN9FMUZ6erDLhi3DGTF2qCEvn11DraF29bkPILBDDaEUbs2QwuFhB76fRT9z3bRMuWBZIwnhP49FIz0IWyT2BJ3mj5ZO1k7qrR3SDnTEMKk="#;

const SQUARE: &'static str =
    r#"eJwdi0EKABAQRa8y/QsIGwtcBhlbTeH2mM2r1+vF2YoQt9FZEmwA7QQPOsr5xIHWqMJaczR/yBe7jA+D"#;

const SQUIRREL: &'static str = r#"eJxtTjEOwjAM/Mqpu4PtJG2RQhdmVvYIhowMiPdzKRUwVKeL7dg+X3nUZ8P9NFxsxlgTErRDejYRn4rZTSUjrTQNuabAlFQYB46MzsrXnw7G8wRTRJj1hwpsz6SzSTTTYSmH7mApPx8jXHnMArdDlAiyWXX4puziL/F/r5KuaU8q85R7pcQ2yduS9yZnWGxB7dt7A0AdPds="#;

const STAMP: &'static str = r#"eJxtTkEKwkAM/EroPbGzaXYVakE8e+3BW6mHHgXF9zu1WETKsruTTDIz7X14TnI7VpeQlCY0Vdfu5l7XrgwOlorArfgpGed4689BIYRPCvwxjSAsepQBgu+0gAa/DcVLYTHWajmrpczKlT78S7luZaGw93uLMxFDxfLE4OKLqrr6yP2cZ0wPMg9IoqvPiXxVfQPcMDdG"#;

const STAR_HALF: &'static str =
    r#"eJwBNwDI/zxwYXRoIGQ9Ik0xMiAxNy44IDUuOCAyMSA3IDE0LjEgMiA5LjNsNy0xTDEyIDIiPjwvcGF0aD5iWAyo"#;

const STAR_OFF: &'static str = r#"eJxFjUsKwzAMRK8ivPcUyXYkg+MTpIcotNCCKV10kd6+Suhno4HhzVN7nJ5XOs/haEiZ9iNUIToKZZguBSYkTCzEClViA5s3I6LUmJBT6O2weXr72RzKPhDoRPIRxgmVIy9ukhEZ2V9581+P2/1CK89BAq3i4fn65l47ukH9Df98Kqc="#;

const STAR: &'static str = r#"eJw1jUEKgDAQA78SfEB0F2u3UPsdEaQt6MXfW7FCTmEyibUc91Yyatnzda6DKBTiOAUYdYEqAtVDWmbKDDGKQYVT41o8vYejae9+ru+MQT7Tax5SHPtjegDvlRuE"#;

const STEP_BACK: &'static str = r#"eJwtyjEKwCAMheGrhMxCNWTokOY2pQiSCO2gt28Ep/89+KRVu2HQheVEmFFGGGXfKGVUOZZS6d7m4wbdq31vGE6UgVMhiMkLbqI/FmUYNA=="#;

const STEP_FORWARD: &'static str = r#"eJwti0EKwCAMBL8SchZqgvSk+U0pgiRCe2h+3wiedgdm6uh6gVPDgvBxwxPBYzgH0kKpx3KkTht+m8K0ru/TkHIqwDkRQ9wIwtyO/AY5F/Q="#;

const STETHOSCOPE: &'static str = r#"eJx9TcsKAjEM/JWQe2OTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjxr2nCYTclhw5sMC6AWTnq4Ao1zHKeqgR4qoVY7ff2sTDj9/IbLiSkLwSRziWtBmtJb27J+DQ+X9yN/7z2c5Lu5ygPTKyRVgyCkK7K9hhWs/lCdImMtk="#;

const STICKER: &'static str = r#"eJxtjDELgzAQRv/Kh7vXfBcTLKTOHdq1QzfRIaOg+PtNdFAhHAcH790LU79EjK/qSycO9u16hcLkqdO1shkMKJQnMtDI5jSgtf5acZ/j+V914ZGDXbhkYVcffQmlsJ+lBVOPA8XuReYt6Mmz0ZQ6/k42COY04A=="#;

const STICKY_NOTE: &'static str = r#"eJxNjLEOgCAQQ3+lcQe5gyOaILODrg5uRAdGB8P3KzFR06XJa1840pmxD81MogV2lMRgmBp1t0JuMyBNukcFnMl9C7DipdMyPee1iaGtwhh+Wtjis3/RBdRlG0I="#;

const STOP_CIRCLE: &'static str = r#"eJwlikkKwDAIRa8iXqDDolBIvIyVKnQlQpvb1ySbP/BeYXN+BLhV3HYEz1oR+BuXyjI5FRcOULFbo+KB8NoVOlaqJ0LrmX736Ac55him"#;

const STORE: &'static str = r#"eJydkb0KwzAMhF9FZJdqXZzYhTSQvV27Bzp4KXQofv7K6Y9TyBSMbTjpk3RoeMzPRLdTcwcF8uKVyzOBQM6OUpDYElKU1s9VVfGQ7niGYc04HEqZcfgWu3hS5PjLd/YjKdYCI3PcQLUjWKg2c2y5ibEWCNlvsDZMgtsKWCS3df6lgpshgcp9a2qGWPpWwkdyLNGA6S+NtKfiZBeK3WjcTS67cGvvjGtd2gsI8GyR"#;

const STRETCH_HORIZONTAL: &'static str = r#"eJxty0kKgDAQRNGrFHUBB8RVdy6jwc42NKi3z7ALZP3/kxwvx6fcCYvpMVeexK88iDfdbjWtRO5LkKX9Qboa/20GMLACuBUewg=="#;

const STRETCH_VERTICAL: &'static str = r#"eJyFizsOgCAQBa/y8i7gJ8Rql8socWnJJuLtBRtKqilmRko6HVUZiCdfbsqDsJRvc+W+Em8DUWpHlKX3Uf5r9E1uYb59uVsewg=="#;

const STRIKETHROUGH: &'static str = r#"eJxFyr0KgDAMBOBXObL7kzSIQtu5iw9RUFAQcXCwb2/bpYRw3PHZJ74HNkcrT9CwRAODsVwn/Wyg5O1QjLdNKliiQivk/HOYmrvOe0cSRyyEjx1pjlxlJCSuc7ZF+R+lwx7h"#;

const SUBSCRIPT: &'static str = r#"eJxtjjEOhEAIRa9C7GEHBnZmkllvsJUnMFrYmFh4/wgWVoZAfv7jE/oxnxusv2FXMKhQh7F/whv7Q1jA8BX9JQG3DXVJyGSkigIuUMimYEY5Q4jYIC1I7N2y6+oJabMQM9wjRXnyK0518SEkilT0tpofTuX54QJsriqC"#;

const SUBTITLES: &'static str = r#"eJxtjDELgCAYRP/K0S55n4oE5tzS2tAmNDg0NES/P13EQW453oMXnvRmXOu0e9BkO8UwVxRDE3TVyMB4LENOU8QoJQRdEgh0GVV5m7+VhT06DFGSaXsA+ajPVvwByMYvBA=="#;

const SUN_DIM: &'static str = r#"eJyNjU0KgCAQRq8yeADT0XSj3qBDhAUGLUJa1O3zZxOE0uqDee8xxm/R7ytESyQBf1nCMe1d1pmhYmeO+QywWDJxBBko4xnm4wshA44NljJkDSY7maZq1KCokKKv1Gk4pf+jfB893ytNzg=="#;

const SUN_MEDIUM: &'static str = r#"eJx9jEEKwjAQRa8yZO+YmUknLpLewENIFCooSBGxt2/TdFOadjF8mPd4IT379HpAH40zkP7REE87zNuGc8Ft+Ny+HdyjuRKD/Cij/FoDtlUiQNzVANsd8qYLijpoUEVP6K3PV/EUxU15j9r4I28OldyBVTpLbuONN3ZP1g=="#;

const SUN_MOON: &'static str = r#"eJx9TkEOgCAM+8rincnmopAgP+ARJB64kHgwvt+AHkwE07SXtuvcHo8E2zoEYjCR0UxQRVfIAw0EpETJ4N1YKt69i3xyx9AtJwtaKCSUwkaCFlzgll4mMBCn1jDrjpNnnOpV9bNskcpzn8gFd6xNEA=="#;

const SUN_SNOW: &'static str = r#"eJx1j7EOwjAMRH/l1N0hF5ukSKF/wMpeiaELEgPi+3GWdnEHe3n3rHP/rN8Nr/v0YMZtVSgy6JNRp6VfBl76Hipg2RgAGgqfGpEMOwOFPwmuvTVVA+ekNTVJLUhYUkP1JSeJ0ci7zoHLBhP/NEIuNcSsELyKDlMO/gcsEFYT"#;

const SUN: &'static str = r#"eJx9jkEKgDAMBL8Scjc2sVSE1h/4CKmCgoKIiP5ebD1J9bKBzLKM9ePqpx784ZAFwZ/xrg411jaPuLZLuw3QOWxYQHa50f16AZUis6aqgBBMmkMkWlySMfDkd68RYBlSAqI+yGyo0HE5+zWoSJVBNFG7AOPaTrI="#;

const SUNRISE: &'static str = r#"eJx9jrEOgCAMRH+lcQfbShQSZHbhI0gcWEgcjN8vYHQRzSU39F2vtVvYI6xz54mBD90525eRszdISpoBCKtLRdXeMc9AOnIL4AdJZCROV7X4rc6v8dJq0DCCEgqyGns05suhUCwSGvBJnYlYQYE="#;

const SUNSET: &'static str = r#"eJx9jLEKgDAQQ3/lcG+9O4tWqM4uXd0LDl0KDv4/Xiu6WCWQIS+J28MRYZsaTwyEKzeza3M2u5sko8dOWHFtqNi75mVvI9cAfpBEo8bhula/1wzMS/Whh14ZMEpUGQomG4QDZikL+LRO2T9B0w=="#;

const SUPERSCRIPT: &'static str = r#"eJxtjj0KwzAMha9isutVlmVbAjc36NQThHTIUujQ+1OZQqYgENL7nn7GZ/se6XVf3pqyJyNb1nGb4jpOlCUYXbKHcMpykO5MGRWqQuFGJUF9BjSUoimKDmYJF7QLIXeCl2hMY058E2Su6Z95Rixo06Wl7ZEFUpzQi4WsBHOPM9zOl35fri4U"#;

const SWISS_FRANC: &'static str =
    r#"eJxtyzsOABAQANGrbBwAS/yS5QZavUSxpcL9g0alnZeh2RfDyKKiBoPNchSF1K2FnnlAz+kDZ0rScXi0AZPsFpU="#;

const SWITCH_CAMERA: &'static str = r#"eJxlTssKwkAM/JUh92Aza33Abs9evHovq7CCBSki+vfGethKCWEYZiaTeO8fBeckRzPY/rDuCaLxMaXytK0czksrXVx9M12syYC2/BnBpzXzS2DRWTRfx3y7YEwSBPmVxOj4ntBNP7k2DLYDqUEDfJcfDBtvmjRU9QOfMTZx"#;

const SWORD: &'static str = r#"eJxtjFEKgCAQRK+yeIBqrQzBvE0fgqhQH3n7Vlejjz52BmbmrUnRZ+/CASm6cJ27wGVYATeSGRQdaw1KJawZO2NNJbMkahaQkVwLuGVzrDkBn3Ep5dRK1aDinP+Mkcv3o2aI8j5+APFlMl0="#;

const SWORDS: &'static str = r#"eJxtkN0OgyAMhV+F9AG2tShIwnybXZgYNdkuxtuvPxgX8YI2tOejPeRtncs8LS+3rdPyeT8Bu1vvMHLwLvCxqAVpwZjvOzNmJQsx5cF9sWa5J3AFNTPwL5ZiAIXoYWLNVm/FhLWZDJKsEB7iaxNBVh54eUILwWykSxfy5j5isJF99dCd1pJePCyoJtpWjZT0GSF8JbD5mR/B3mPM"#;

const SYRINGE: &'static str = r#"eJx1jjsOg0AMRK8yoreD8X6wtOEEySGipEgBEgX3F14KKADZcjFPnpkyf5Y/fs9mkh4dAkIzlEcVh3KgjAwlPaO3GAw9Z4ixfkkg1HH0qxzQjsTJ1/VtKmkreUlEvAgyiNx0cE8jhV7V8w8kpB2tcwA2Eg=="#;

const TABLE_2: &'static str = r#"eJxNjbEKgDAMRH8ldA82UUsLtbOLq3vBoUvBQfr9JhWk3HK8O+7inZ8C12aOAPO+ZgYGq0JxbakOXSH7Y4KOtdzIV0FjaoGRzyAp07BF2PHX9ibFSU/TC8dLG+Y="#;

const TABLE_PROPERTIES: &'static str = r#"eJxNjE0KgCAQha8yzAVCRSgYXbfpEJHSuAsZ+rl96qLcPHjv4310rMIQHC7KgjnViJ6GunnKcRO4UhB2WHa4HRqEp2UuRSNwTDtLw+VWD54+oVYwzeb39UDZjry5aSUj"#;

const TABLE: &'static str = r#"eJxNjF0KgCAQhK+y7AVCJShYvUGHiJTWt5Cln9unBbJPM8zHfHSswhA9LsaCO82EgYa2BSppE3g8OoQrR2GPlQKnvLP8/f5gqWHbrR0CdaGDmbVPATNq8gLECSVs"#;

const TABLET_SMARTPHONE: &'static str = r#"eJxNjEEOgzAQA79i+R6a3aZVDgk/6CNQQV1uFYpa+D3JBZAvlkeetEzvgi0zEsuaqcR/Hotliidsmj9Wag9EhXf26dYOffoOxTBmvh4Ig0LhawTq1ESvA/Qnz3NwtZnTLjRVk1xUERKt83KgHY4NJ7Q="#;

const TABLET: &'static str = r#"eJwli1EKgDAMQ69SegDdhogf2y6jwxbEj1Jw3t52/jRN8pKl7QryFkwI/5U+hBqfpPYGhIcPpYJxRbBywZpn39V88d2gR6t87roZktxPIVqURmS8k/UDgZEc/g=="#;

const TABLETS: &'static str = r#"eJxVi0EKgDAMBL8Scrc2aq2Htj/wERKFCgpSPOjvNSgWLwu7M+t4TrxMwKdHi5A8GgQ+pARXPjC4V8qUblcu9NO2YY8weuypArKRtEAZM1pr1bRAWpkGrNJdIfFpF0KvJws="#;

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

const TARGET: &'static str =
    r#"eJyNjLsJACAMBVcJWcBPYRWzTLAQrFLp9opBSGn1eNxxJF1lNJBVMWUEPRMRZN7LFIwzPc+A98uf5msbxVghVQ=="#;

const TENT: &'static str = r#"eJx1y0EKgCAURdGtPJxLPu1Hws8duIiggZOgQfunHASBOr2Hq9d+FxybyYzwDnSYTdKp5qQfnlItWi6t5fBa4dq56EGxATIy/O0BP0Alhg=="#;

const TERMINAL_SQUARE: &'static str = r#"eJxFi0EKgCAQRa8yzD5CDWrheIMOESmNiyBkIL19mkT8zYf3nr02YfCE5wxKgR7eobNjA85+eK1QGZ5+ksIuUAgNQsqEGuGOXphQLQgc4sHSf+5OaU6tW+cenIwgqQ=="#;

const TERMINAL: &'static str = r#"eJw1i1EKgCAQBa/y2AvkLkYEq7fpQxAV6qO9fWr4NQzMaKvZcioXWk3luQN58AF2YIbHTlG3lUSdoXEgPgkmP9/h0tld3Bhm/AGxyhja"#;

const TEST_TUBE_2: &'static str = r#"eJxlzDsKgDAQhOGrDPZZMxtZI0RrCz1EwMJGsBDP7wO0UAam+eBPa95mTG0xKlHDJCqUwphVYsB9/hxdkKZx4rn7L8G7amCNUHSpvHpdeqoLDQqD/WmkgtZXrxyfJSFF"#;

const TEST_TUBE: &'static str = r#"eJxtzaEOgDAMBNBfaeZb1mZliLE/wOLJEJUIwvdDCZnC3svdlWM7DfY5LJxIQS7OpC0CU0ImBiFFcSC12J4oQfTAET9cJdQy+FAtfW7ykuUfeY94NNSON2E+ITM="#;

const TEST_TUBES: &'static str = r#"eJx1js0KgCAQhF9l8a65mz8IJnTr0rW70MFjh/D5SxChMpY5zAz7Mf6IZ4J9YqsDymiFnkloKJL3IZjiKMtXrACd0Bux4IeCCL6BSFZSfLzw6rL85kVd1giUbCdH9VM4QLOo/io0ietWXcWDPgo="#;

const TEXT_CURSOR_INPUT: &'static str = r#"eJx1jbEKhTAMRX/l4h5ek9hXhers4kcUHDI6SL/fCkUqKJnuzTlJ3NNh2KZu9eiNk0LhyjC0Tk2kxt0cfxc/x9tihTijRqSCoknQxb+IHvxf+iSQCgpJpqZAKb5eDhZaEJIflyBG4cUcETK7e3ECuCc7lg=="#;

const TEXT_CURSOR: &'static str = r#"eJx1jCEOgDAMRa/yM9+wLg2Yshtg8UsQlQiy89MhKIZ89ZL3vp7tMhxr2nhBKUbcBILsYxKSfQ6Gs3GqOo2o6ps+ZYR5iJ1+zI/oj5Ae3g1sVSJx"#;

const TEXT_QUOTE: &'static str =
    r#"eJx1yzEOABAMQNGrNC4gJcFQnbs4hMRgNIjzY+nE+l8+jTo7tGwKRgjiDZO9iUnBIaCT9JH0FH+WFRQ2v+Acqg=="#;

const TEXT_SELECT: &'static str = r#"eJx1j90KwjAMhV8l5H7oSdfNQrs38CEGChNEvBCZb2/b+dONlF6k7fedhPj7+JjoFPhoyYxCQvt0mnjjwe8SHfzPgSskkC4JCO5vVVpZEpRSo0lx3AT1X6ACtJVEBJWIIfeEvoYODKGtJdbkermdaZbAsEwvBD7EIrnM8dUnMzml2X9ZDkCWBGTjZtjxkjEfqdv2fQNlFn5C"#;

const TEXT: &'static str =
    r#"eJx1yTsOABAQBcCrbByAPOJTLLXGISQKpcL9g0Y02hkedXZqURR4chLZiMTqYOJbGgT9OVi5Nzy3ALf9Fo8="#;

const THEATER: &'static str = r#"eJyVjjEKgDAMRa/ycY82SZUO1Rt4CNGho6B4fm1BdIiD/Cm8l5/EddoTlr4aBew2JYVSqIbYZDDEBxdOWmIJ7CCzg689ad0hUICp+VtD0WBr5Zogp/3+5sJkCwHcJrtY5CCecrO7wrki+fcMOfjj81+rJ8rGV2U="#;

const THERMOMETER_SNOWFLAKE: &'static str = r#"eJxtjbEOgCAMRH/l4q7SUokkyB+4OriROLCYOBi/3+LAIkPb3F1fG650ZxxLtzKIM5kuhrF4MdTEQx5y/+C08LCwvVYDI4bTBa+9BTvQ/LE0TKUaF9iUz2aYJAkEBgTqdW6SGKy6OKr3yr52xjTn"#;

const THERMOMETER_SUN: &'static str = r#"eJxtTrEKQjEM/JXj7Y1Neq0+qG92eauDW8HBRXCQ9/2mIiJYQi6Q3OWuPtrzhutxWtUwN4KIvYJhL3la6q4TlvpLS5v9H+5FCvQgDCqE90BrEdw0SubHSKHB55nNYG9j7fvLQOs/02nguxZJ9KwOlDkhO3xZL3hzN9M="#;

const THERMOMETER: &'static str = r#"eJwBPADD/zxwYXRoIGQ9Ik0xNCA0djEwLjU0YTQgNCAwIDEgMS00IDBWNGEyIDIgMCAwIDEgNCAwWiI+PC9wYXRoPrArDeE="#;

const THUMBS_DOWN: &'static str = r#"eJxNTrsOwjAM/BWre0zu3DapFCqxdYCVgS0SQwYGBsT3Y0AKyJJt3cPncq+PJtf9cEISjGcOa9m9sbV0ZhFkBQXRFduoSJVCiV4I0IWBOs03qlnIh07JrJNwY/ypffKZ/9y+NXenuWPRLyZPVODomWSL1RQmn/Z1WTDN+dJffQEUJiqI"#;

const THUMBS_UP: &'static str = r#"eJxNjrEKwzAMRH9FZLfqk+JYATfQrUP7A90MHTx06FDy/VVacIJA6N6dhMq7fho9z8M9E+IKGZZy2thSuoNEic0Io0eaj1qFhKIXCDy74DS9grAq2eXgZU4kch33fJAga7DDAQdNOE8dOeQ8BzBwg8MWqzKUfu2/swmzR//1C10SKtg="#;

const TICKET: &'static str = r#"eJx1jbEKgDAMRH8luBebq7YItbOLq4NbwaGjg/T7jQ6aQQkhXN4dF/d8FNrGZgYN2ZEjK8OyviKDcGsrF4W9fhhUA50wfgmvwYhh6rQmrE2K7VWY4lPLjvqKb8Dhl7AiJ97sLxs="#;

const TIMER_OFF: &'static str = r#"eJxtjjEKgDAMRa/ycTc2sdgKtTfwEAWHLoKDeH5Th3awhE/ICy8kXOnOOLZhZwPJdohhKiiGurC0gDl5eJivmBx8SSVlUtqRHVloqs1gQ7O2BmgtTP7yKdDSt0znMAtYnrFpLxSyMK8="#;

const TIMER_RESET: &'static str = r#"eJxtizEOgCAQBL+ysQc5OAQTpLbQR5BYUFoY3u8ZE23IVruzk85yVRzLsJOBrTzkND5TTj+wIG6qhxjkSkSEkRCiCngbgZTXTsRNPkFPHXkWsHLzH7oBER0j3w=="#;

const TIMER: &'static str = r#"eJxFzDEKwDAIBdCriBdoDS10MF5GMgRCh0zm9tU4dJKvz8+jvw2sVKQLYfksCEYeT48UUfgIJLxp7IJuU/KFCLPi/rH2qaOBruRqyWfFJ1Ce5QMS2CEM"#;

const TOGGLE_LEFT: &'static str = r#"eJwli0sOgDAIRK9CuICfhXHR9jLYCIkrQmJ7e0FWM5l5r2gng1nxQBgVd4RXLmNvKwJ3udkqbj5rMjoiWlnCa4VE6emgv0kzUXLmDCbf9gGGzRwe"#;

const TOGGLE_RIGHT: &'static str = r#"eJwljEEKwCAMBL8S8oHWHjypn0mlCfQUAq2/N5rLsrAzW7STgf4VM4LnhcBdHraKybuOPUR+chs7cmIrx/JaIVF6O+gWaYRE/pPygmJuE6LkHE0="#;

const TORNADO: &'static str = r#"eJxtyzEKgDAQRNGrDOlFZ5UlgTX1Nh5CsEhp4f2RNCHFtv/x7b2/hudMlxCH76na2lO1AczIrhEUULxEoqC2JZwI2abpBy/qJAk="#;

const TOUCHPAD_OFF: &'static str = r#"eJx1jqEOgDAMRH+lmW9YL8vUmMZg8UsQMyQIsu+nQzBESUWbXu9d01muSvvs1kChgEC+F+vUBGOhHVWiy2nqjpxeH0hCFVjKI7ElieJ8Y4N39Ehonv8hxi1+/2QsMk5veig1VQ=="#;

const TOUCHPAD: &'static str = r#"eJxNi0sKgDAQQ68yZC/aUrrq9AYeQmxxupMy+Lm9LYK4SSAvL9S8KkkumyjDeNBZkgrDTqB6tQa9eTMcYhi7EMO+qFBizJaMk3ZupG8/YizZ6Rj8hx6Qtx5X"#;

const TOWER_CONTROL: &'static str = r#"eJxtzLEKAjEQhOFXGa7PmtmYTQLxaptrLewOLFIoWMg9v9FCOQhb7Xzw1+f6aridpoVZFFTRBPWw8/FOybD+rwThvyclSgqNXuJuNdfn6zTXw6c31181g2ErA6BBdXMDehRYT6eBMMLckBYq7KIjCNDm/vIGRyE+qw=="#;

const TOY_BRICK: &'static str = r#"eJx1y7sKg0AQheFXOUw/ZiebyAZ2rdOkTS8qGbsgi5e3d6cTRE7zw+GL09BlTGsiISxjn7VUIOgw/jSXvhPK6QlbokBNvBlo4r/Nij7RRxzC99k5rmquHiy2d90KBM7GpWZv0MgRvs5Q2V/KHffxKuY="#;

const TRACTOR: &'static str = r#"eJxtjtEKwjAMRX8l5L1x0WbroN0f+Oq7VKFCBRki+vembFjHBqUXek9P4h/nZ4JLwOMBbOozQ4eD35XXwf86C8wnu1E44GazYAcSDbXQGAayevFLqF2Td25UXg71sfCaJKRi4mw0JRmu3+JtjPkK8R2wQ4ifgCwIY0CSAk11xeZ+xhVbUHXbFtglI+s5ReAmQUk17P8MXzvAViA="#;

const TRAFFIC_CONE: &'static str = r#"eJxtUEuOwjAMvYrF3m/iOCat1OEGc4HZVWXBAiSkcn/xHCQ2oEQvifU+dpb7+rjI+ffwN8PliLo2RMiAMlagSTmclp9knpY3v2MWK+gbZkxSyTI6NFieu6Mq0seUzy9yc+odsRpmlwEjTx2Twq7KYtl4SyNDSziSYOgMq+IS4nuku4T6VpRFQhKZ/P8ZeetsyKjVNjw63QOumNAJdLYdTH9dr51x/IYcL8g2IlvMso4yNWxRaZUj2q7DKneqM0MrpncbT9lfT0Q="#;

const TRAIN_FRONT_TUNNEL: &'static str = r#"eJx9TzsKwzAMvcoju1xLVkwMbm7QNUM3QYcshQ7F56/6oQRigoQQeh+e6sOeK27n4SIQWViMI7x9eIkvjeMw19ObN9c/m0fkMDUOagkSpq+AMuLi957ACeMaIvcwPcBcV0yh+IQiJW2ULCP/MrJ4xrRlQK97o3txHxKkDuTPcMEWewHVkkWw"#;

const TRAIN_FRONT: &'static str = r#"eJxtjrEKhEAMRH9lsI+3WTfeCnv+ga3FdYtXXCNYiN/vrCBYLAkJZF4ySVve//h9mimia3V+54AAd0WEmzlrxvQq1Jhudh2gJio1SQ0la9rEtWER3/KwGLtntUNCjsWLodAe7giLA6lCgATsWzGKPCYeXe2Hnhqe2glE3Dol"#;

const TRAIN_TRACK: &'static str = r#"eJx1zrEJwDAMBMBVHi8QJGysQvEGHiKQIk0gRfbHUmFXFnz1x8Prd/0P7jN1BlUPp6aHt02nvWYZAtlQAVFAAgnEJiWiDN5Tt2/sqcsGLq4zUA=="#;

const TRAM_FRONT: &'static str = r#"eJxtzUsKhDAQBNCrFL0PM906koHEG3iIYRTbhSAS/NzeqBvFrArqUZQbm3+ANl2rwRMXhMVTThhjCGH1lBHmrg56aOle+6B0wy8oak9VDmY9Ze8uwoJssk/oLfhrIiaILUSMmIRVcfbRd+qouMsG2iI7fg=="#;

const TRASH_2: &'static str = r#"eJxtjMsKwjAQRX/lkn0wdwxphbRrN267lyhEEHEhxf69M1oVShfDncc5k+/HR8Wpc4ctUmXr+ryxVZ9/B+6QRsYSQE+IF8i+KdoGL55aMqQVrUUaVFKOpnipsagDgT6BjPJ3rpfbGRM7Rzo8RTNo8pOTzY3Bhs3wG4ozpPmVF/ALe+o3dg=="#;

const TRASH: &'static str = r#"eJxtzKEOgDAMBNBfueAb6EHGSMY0BjtPhphEkH0/nZlCNGnaexee6y249+Gc4Yr6IYaxnWLoD93gqi55goqCQvBYs62TUNSGyf0wD5cMWU4bEZYlmwFhJWBlNx/bXyEY"#;

const TREE_DECIDUOUS: &'static str = r#"eJxVT0EKAjEM/Mqw90SzaW0X6oIP8APeynrYowfx/U5EKlJKkplMJmmP/txxP0/XClv22hMSjp/nWmXWioGI6UmS5s2YGYxs0SRJtBCYWfKLs8m0dId/ZS6Rjwq+sYGqUCyaQwEqOIbBRTMCp23/swatL7/1Yt/btLZDXLC2cYfNJF4+mDcVry7n"#;

const TREE_PINE: &'static str = r#"eJx1jj0KhUAMhK8y2G/ey2YlCqtgZ6EXsBMsLBQsxPMbBQUFSTdf5icu/TpiKJKZFRwgEJKewfjbsSNTSetA+hCdic1hGN3r/SQ5cgO+ugEyKEnDHjIFhKcnOys4nT7KWbukjL9jaRmvva1leb85udEO0W4r7w=="#;

const TREES: &'static str = r#"eJxtzr0KwzAMBOBXObJH1cWJf8ANdOvQrh26GTp46NCh+PnrNGAymFuEPjgpftI343Ue7lRQi0wXAwOtIbwE0Ba9LkWzpgYjx0X8g4cVLPQ5rPG09a2xtbqtwHaABgzF9GSqkr2YxNqr/4gbKe5GD865KxYhy3QEv0N98V2nGZSlXfsBkWg53A=="#;

const TRELLO: &'static str = r#"eJx1jEsKgCAURbfyuBsIs7BA3UxJOhUh3X35RILA8fno6I5EdziTNxAbKBtIUCwGM8i7cPnUQGkgV2D1VDurue7WzrVC30mO1M9+FbF863WoP33nLDk="#;

const TRENDING_DOWN: &'static str = r#"eJxlizsKwCAQRK8yeICE3ZCYYuNtUgiiQtJ4e/FXiMUMPHhPYnDJWf8iBuv/71HMIA06thN3XwWGVkb24RtZSrpK2fryNPkZYRMeTQ=="#;

const TRENDING_UP: &'static str = r#"eJxlyzEKgDAQRNGrDDmAOhtiLNbcxiIQkoA23t5VsBCbzxTztLdyllw39Jbrsa9OBBH0QwCDZbnHZBEwuqTjC5L+KGejj7fQf94XPbYeGA=="#;

const TRIANGLE_RIGHT: &'static str = r#"eJwdjDEKgDAUQ68S3H/s/y1WoXZ20Au4lTo4Onh/bCWQPAi89JT3xrUOhxl0LgaDa1FptPkqSoVr7YWTMHTazTEg0Fcuwgj9nz6M55DT2J35A37lE6Y="#;

const TRIANGLE: &'static str = r#"eJxFjDsKgDAQRK8ypN+Y/aBbxIDXsAtYpFCw8P7INsp0w3uv3v0ZONZ0CedFwU5ObF0gKDHSbI5ykoNt+24YhAfPP4jwSffU6hTR9gKUFxOz"#;

const TROPHY: &'static str = r#"eJx1zzEPgjAQhuG/8oWds9+lvWsTZHFhcWVwIzowOhh/v6DCQCBNc8ub53LNc3iNeJyrq6F0UdKgkjD/MD0i1Kmzqm1Oc9c2a82MMnKTh2/OvNNHqI7claY1Ucx6+j1AUqolupRcS3FQlBeXnMAsnuDQIBrnqXtW3Fg/apVowoXiYvEIy9DO3j4Y7H8eFaHX2xp/AGz+Szs="#;

const TRUCK: &'static str = r#"eJx1jr0KAjEQhF9l2D4xo+v9QHK1ja29rEKEK+SQQ9/e5AS10GZgdmY/Jl6Pt4xTkv0W7HYbc75BcHRei/DQWiiXalBsZrCag/BahDMpQ1xVyBDfKCr6rKNCZ7UA37jarojs1p++XSYbz7B7klZgjyTsBFOSpfNKv6nLwv7nP/8Dnts5OrE="#;

const TURTLE: &'static str = r#"eJyNj7EOwjAMRH/l1D3B55iSSqFzF36ALRJDBpAYUL+fENSqQ4fKi6179p3TO38KHtfuRQUFCptDJghpVbui29lxdiFHxCaKYw85sPFUZ8Xu3ZhOP8cxLb4381Fx8UO9SNnRefaMDWgJ94jeD6FKk0rW+sE/hcAmXekv8GQ2BA=="#;

const TV_2: &'static str = r#"eJwly8EJgDAMRuFVwr+AbRW8NNnAIcQW05uUgLq9hl7e4cGXr92UCmNbKUWNAZInf5J7PYzuVkwZKYC0tlONERdQf/4HGn0ZszMH8gG+hhbJ"#;

const TV: &'static str = r#"eJw1i0EKgCAQRa/ymQuUQrhRL1OSA6JiA+XtMyH+4i/ee7aFXXDzIdGRXgnPOEJ3ZAgx8BnFkdoIrU/QJvd2+Tpva0k9cQ6ohbNcQzXQUBpmbHq/4V9deR6w"#;

const TWITCH: &'static str = r#"eJwBOwDE/zxwYXRoIGQ9Ik0yMSAySDN2MTZoNXY0bDQtNGg1bDQtNFYyem0tMTAgOVY3bTUgNFY3Ij48L3BhdGg+5+oQKQ=="#;

const TWITTER: &'static str = r#"eJwdjsEKwkAMRH9l6D1jk83uulB78exHSD14FPTk13daQgJ5eQxZPs/fG6/b9IhAfo0dQbdAYW7OBp9tMOGdxfwKP1hQDpJpWprFvcArVRgCBXWTIauhqlPugNxhKZzWBLsVKk2nGQoWP+d/WpfL8dK6A3M+Hxk="#;

const TYPE: &'static str = r#"eJxly8EJwCAMheFVwlugGixSULfpQRAV2oNu32AoFEog/yFfQm9lllxP6i3X+4pw5MnJsNHlkcL2qhSWHRxhd9CwEQdoSthIeVX818rRMvSHFbuffQAbZCQZ"#;

const UMBRELLA: &'static str = r#"eJxtyjEKgDAQRNGrDOnV3UHEYs0NvIDdgoWlheT8ZiGkCgO/mWevfw/uI50klK4yy4ZWCGRizZWyLSGzda/hy+4Ew9WtkLFj0X78GnkcUw=="#;

const UNDERLINE: &'static str = r#"eJwlikEKgDAMBL+y5AO2JeSU5gdevRcUFEQ8iNjfm7bMYRgYvcuzY800C/iVIhCETkwIC5Pp1BbT87g21JgpBUJNw583u0b62y77AZf3FTk="#;

const UNDO_2: &'static str = r#"eJxVjDEKwCAUQ68S3LX/gxmEX+cuPYTQwaFDh9LzV0EECRleHsSe8lZcuzsTNCIi3fR02bYusk3dTFUJLAxEr7QoBn2y7n7QoTq/fpJgGUc="#;

const UNDO_DOT: &'static str = r#"eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0ulJSEK6qhtCqUbBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtHBx/X"#;

const UNDO: &'static str = r#"eJw9izEKwCAQBL+y2JvkPDg5uPiC5BFCCssU4vvlCmWb3R3G/tobvju8jDykSSh2+ldsk0SgXBWKyxM1el9LkA5+GMRbnbn2FMg="#;

const UNFOLD_HORIZONTAL: &'static str = r#"eJx1yTEOgCAQBdGr/NAT3UUIJEht4yFMLGhMLIznVyiodjPdvHwfT8W5mp0CiGswJU/tlTwk/rCxAMTgV4GoAS2a8CzJRQnk4WxPcI9kHXpDP5hnO50="#;

const UNFOLD_VERTICAL: &'static str = r#"eJx1yzEKgDAQRNGrLOkX3YkJEWJqG1t7wSKNYCGe38RGiw3TzePHc7sy7ZNZBATc7E2KXT1T/FNYocBAglkD6asETXyRzFoEtOgQRzKyJctlujt+kT5+AG84PEE="#;

const UNGROUP: &'static str = r#"eJxNzEEKgDAMBdGrhH8BKVRxkfQyWky3JaDe3lTEdj9vuObNqF6CANJcDjXBAroFEXSW3VSwgjyYkXhqeeIXDXXj7r9NV34J8WcPpRAenw=="#;

const UNLINK_2: &'static str = r#"eJxFizEKACEMBL+y2MtFIZfGE66z8RGCRRrBwv+jqWTZambSbEvRP1cDQzQ2BoPOgp3Ux+FfUJH/AjHR5fRYmzcFyA8+"#;

const UNLINK: &'static str = r#"eJxtjd0KwjAMRl8l9L6xCevawbZ3GShMmOKFF9vbmzT1b0ohIc35cvrbdJ/hOLgLZcwNECNHIEzspdDsMfAUMYQGrAZ9Hol9wpDKZ1urrVrsIoRF46yHyI39QS1j/3JFpAQky1gsBfu1gFjgvwWqRYP+W7KcryfYaHDsYJWWHWw8uCgT6ySgIhVcbbW9wWwx3oFKUGdHqDVGu0Z4D68fsCwtXGHpT/gBo3BXdQ=="#;

const UNLOCK: &'static str = r#"eJwli0sKgCAURbdyefM+rxAT1B00bS4p6SxEKHefEndw4XCOzuEsyK+hhfAkX6Ih3gi1HRMaXwkxpCuWn+TaVaunHlp9uxLhDe0SzId0AgJzG0ONauAudsV+1+4beA=="#;

const UNPLUG: &'static str = r#"eJxtjsEKwkAMRH9l6H3XTdJGhbVfoD/greDBQwUP/j8mOSilYdglZB6Z6e/l88TjMrzojAlSZJj7wZdz/1kM5ty6aRVwq7JwHeGvhcSnKzHotBY1sXM7LMB7cvZYJ5D410CUANTc1MBAY1LawjVksSWJLha9muHddGuWqP/v9QWv+US4"#;

const UPLOAD_CLOUD: &'static str = r#"eJxljDsKgDAQRK8ypHfNLpsfxIAH8BABizSChXh+jUUaeQwMPHj5rFfDvphNwUoxpTUgwII7jgIjNqaQqpJDn/2kvC+SqJiS594oeZRYwHKnvznYg/2kHejwD/RqH/g="#;

const UPLOAD: &'static str = r#"eJxNy7EKgDAMBNBfOboXTbToUDu7+BGCgoJoQSn2702rg2S4O3ixfrwWTJ0amEAm1CODUcqRltab/9YcdK2cLdKTs/7Y4rbuM/yx7tfZKWrQghgVpGT3CWeziyzGKNwkyZL8ZpRdJZ/tA+uUJk4="#;

const USB: &'static str = r#"eJxtjUEKwjAQRa/ymX1iZpI0LSS9QS/gTqKgUEGKC72906pUsIsZGN7783O9THU8oT4KsSPUZ6FEmPSiPu/etM9fS6m4D14y4de6He5nHAsNwSZwZ70uxNmYycqvwvBGKQSy/+dDZ6VBsk2LCJZRNp+wAwcVBN5Go7PltJoHG9auZa9tL0QZQcM="#;

const USER_2: &'static str = r#"eJwlyjEKgDAMRuGr/GQXk4LSIckNPESJgoKDFAe9vZZOb3ifxlHj3FCNJkI8RpL+vkaZXMd+Xa9y71iNlsRIUjIyGAIeZAY32IB/PxwUXg=="#;

const USER_CHECK_2: &'static str = r#"eJw1ykEKgCAQheGrPGYfNRZS4HiDDhEWJEhJucjbl0W8xVv8n4lTWjELjdyBh0lDoymrWKEha+oCrHH+cGHBIdQR3CXUP5eFhkK+aE3cQw5+WxB3v6VTiDWYwT24hVJ48Y/sDQK0IkA="#;

const USER_CHECK: &'static str = r#"eJxNjcEKwjAQRH9l2HvQXZeqkM3Zix9RYsFAaEMtxf69pqVQ5jTMG54v7fTGy+jJDYRnJ61Cca5x6vTRHDt0Fgr+VE/BxzTG3CEuRlfCaKSE+DW6V2Qbgy9DXnLqO5Qh9dPH6O9hBt/AF4hghXco/ABXWyah"#;

const USER_CIRCLE_2: &'static str = r#"eJxlyjEKgDAMheGrPLKLaZDi0PQGHqJEQcFBioPeXqOTyJse/5e2ss8YlYbQQ7hERLCvCQKmnFoHOdlSbZ1gp1JgQlXqCHbcTxy9+cvkYa5/7gIvOx+9"#;

const USER_CIRCLE: &'static str = r#"eJxNzEEKgCAQBdCrDO4r/YERmDdo214sMGgRElG3b7QImcVn+G/G+DX6baE4CCUF+YsTnHdOa5q3t+ZzuWDIvv15wXZ3BJoHMXYEWWuNSfUOBJI8ilAh6HInnCqx9CPd2gc66iVr"#;

const USER_COG_2: &'static str = r#"eJx9z8EKwjAMBuBXCb3vd2nWboV24AP4EFIFBQUZHvTtbVfBHjrJIYd8/El8vC7xdqb4CoonRfGdulG0BCVq9rsynn3FvsqtaKjR4/i80CmoA/cwxAKzt2SpX0sTu4wz+tG7ZozEFkMH10Eagg10DnPbwsISTxghybSAAxNr6D+gJHTpEG6vmHKCbIIBQuWQRFqP9vlRyTE1+AD1fmS3"#;

const USER_COG: &'static str = r#"eJx9z8EKwjAMBuBXCb03Lk3XtdDu7MWHGFVQUJAhom9vYw/20EkPDeTjTxLzZc3XE+RXUuQV5Hf5RwVrUqzmuKvtOTYsVDV9kW3RfXmc4ZjUgQagce8WCxYGebpUTyNWzE/eDOEE5NBqDBq5I2hEA8TS3hIOHZDHCbmYHghIQAbNH1ATdFmE+iO8JPAmsMhQFymkd+ggh7LEtOADOI5lDQ=="#;

const USER_MINUS_2: &'static str = r#"eJwlysEJgDAQRNFWhrmL7iJBIZsOLEKioCAi4iHp3kSZwz/M89f8bFiMk/SQcXZw6OoaUXQMvq0g+Ljf8VhxG3siZuNYkoxDJf8Z/LGfK5IaxRFJjKpELhUp1a+FVxZefgwfYg=="#;

const USER_MINUS: &'static str = r#"eJxNi8EKwjAQRH9lmHvQXZaIkM3Zix9RYsFCESlS0r83aS9lDsNj3qTv8Hvj5XxKhMoadDAYrj3Bgj3imWGrMqdLP+VUpqXMI8rmvBGL04hSnfeuHGNO8/QZUdUpkdiktRC1tWpj3bnpXct/ydsjww=="#;

const USER_PLUS_2: &'static str = r#"eJw9zFEKhDAMBNCrDPlf1ilSVmh6gz2EdIUVRET8aG9vY0XyMYR5SdjG44+fypc9OIweHp3Niw6dxPA2EEOa97RM2FV6QSoqQ42s8jHSyhiWeZ1QnAorKrQW2daKM6+s2tRtr9K30rl2Q0r7wQef5Csqcg=="#;

const USER_PLUS: &'static str = r#"eJxNy9EKwjAMBdBfCfe9aEOoDpo+++JHjDpwMESGjPbvXVqRkYeQe0/ie/w86aG4+0DsN8ejkNDZxomTWzjeJBsjxZM9pZjnNS8T5aq4gFaFgHJRDEZ6meIyvyYqrPADqNreVfWKK6j4Fu/a1M8243vJ3G27LQ9//AXwWC7T"#;

const USER_SQUARE_2: &'static str = r#"eJwli0EKgCAQRa/ymX3UWEQL9QYdIkxSaBEilLdvBpnFH3jv2eeoCaejnTcYPlasmPQGNpjI21EFb0Mu4Y4ojhZC+ByxkW2yrFLH3pYYKoTOhDefNQnfCCnmK9X+F6GSNnUk1MD/OEYkBw=="#;

const USER_SQUARE: &'static str = r#"eJxNjUEKwzAMBL+y6B4aqdD2YPsHfURQTGXooRiTJr+PnFyCDpLYGTbUrA1bpDvBcvlYi8Qvwr/Mzc5zPcLqSyiFWxdS0FL1m6Fu8khQT1mc6qxDZ5zCb2qGOdL7CeFlkEkgGH0YMog9rj9kOQq6k3aXiCgy"#;

const USER_X_2: &'static str = r#"eJxNy0sKhDAQBNCrFLUfxs6Io5DODTyEREFBRMRFvL2JH5ReFE29snOz9miVteSQqilQIEv3EYOMzn4TcNYPix87+E1ZEYsyJ3xQlomcpbPjMHUIRmkMscWUHxEk5j/+cumkXvbpDvqa3nYHunYqQg=="#;

const USER_X: &'static str = r#"eJxVy80KwkAMBOBXGeZedGNoFTZ79uJDlLVgoYgUKd23d+MPWHIIyXwTH/3zhqvxElpIWBrpFYq9T6ONntv/G7oIU9x5KcU8znkakFfjicjF2BGzUZ18whSn8T6gBOORKGIMB2KtW6Tu+g6da1db66Zmb+P22/3ZF8O0LqM="#;

const USER: &'static str = r#"eJxNizEKgDAQBL+yXB804UACudQ2PiKcgoKFBAn6e402YYtl2ZlwpHPFLDRZD2eLcYnB6GsMGx59u8HFUQxdlWLQLeu+IAsxQW+h4a1LyH7M/8YHi0MYxQ=="#;

const USERS_2: &'static str = r#"eJxVjEEKgCAQRa/ymb2kIqLgeIMOIVNQ0CKkRd2+LBDirT7/8dJejgUT02gcTCweHrqhjIWmnIYm5CRrlW2GnEyBIBdTJFQm15TvzKm3rP23vPJwDxrm3aGHb0bZH+I="#;

const USERS: &'static str = r#"eJxljsEKgCAMhl9l7K7lXFmgnrv0EGFBQYeIiHr7ksCCGGOMffv47dJtI/QOW1UCqV1Qx8CQxxIsuCm/O/BO6G0Wn7wN0xrmAVaHjBAOh/U9TocmIs/R2+Qn+vm10LIyr/CbRUulE6ruNtIUCb0AdwYtjA=="#;

const UTENSILS_CROSSED: &'static str = r#"eJxtTjEKwzAM/MqR3aotJVYKbl6QfqCbaYcOMXTo/6kUUggkSKfhdHdS+dTvG69b11IGByaBoQoEcaueeEk0wrDjjUWcmTF2U7l4ylT+Wfc0wFoszFBdusq3yovaxvAkJQXDhx2I8+p7NBcp9BjcmBLYn8nUh0xyIklXDGHv/gFfZjQ5"#;

const UTENSILS: &'static str = r#"eJxtjbEKgDAQQ3/lcG+9u1qLUPsHrh3cig5dBAe577dV6CCSIYHwEn+mK8M+d4sBFrchkCY9AVflIVXHR6w4chd8X4ngG+cKx/hTMAHZyILJgn1HVEkyfk7MepRKXJu4AbKYJM8="#;

const UTILITY_POLE: &'static str = r#"eJx1yTEKgDAQRNGrDOmD7oSwCGtu4CEEizSCheT8mibVht/9Z8/5Vlx7OIRg4xqKLf0VG0Lk6kJCanS+Tr7MgOLDLRtyVGj8G/wBPT8zCw=="#;

const VARIABLE: &'static str = r#"eJxVy1sKgCAQheGtHHyX8FIkmDtoEUFBQURgD7b7RhmLHg4/A9/4c7pWzIMYe2gVpZWG5sATwTdZBP861cFECwMLJ3kf27djQdKDcAK3KkkU1dKpS4lmVCkb/TP8WukDZxUpWA=="#;

const VEGAN: &'static str = r#"eJxtTTkOgDAM+4rFntCkJYBU+AGPqMrAyMD/RQCpE7JsefCRz3Id2JduU2hRY8MrwSGQAA2VZzKeFMIDzTwgkaRuzf1TXXMbEMNUk/eM1GnknnzOaX/xkZMgshV/ke8vICK27A3wxCPm"#;

const VENETIAN_MASK: &'static str = r#"eJxtjaEOgDAMRH+lwR+sXQaIgcZgEbgFxCSC/w/tEsgEuXS7Jveu8Up3pnNqViGWFCiQKzI3qsyzeqk3CFVJhG3ICOkNONRx3SCL7M0cOzs2x+9kT8wHt1bkST9Pos0O3ga/CI/GoEAwSB9RWYNODT2Cby7u"#;

const VIBRATE_OFF: &'static str = r#"eJxti8EKwjAQRH9lyH1jdk1CDmn+wKt3iUIEFQ9S6t+7KZb2UPbwmNk3+X35NFwH8xQkCIQEstCUfOj/kldLta2CXe2UkEZ21cGGYH0A92uxaoQD09wRj7Q35gh29ujPsTrSRbeJ+zXy2q+Tx/11wySDETH4/jmxUuMMVbtUfnY/OD0="#;

const VIBRATE: &'static str = r#"eJxVy8EKgCAQBNBfGfYe4VLgQf2XSGk9BCEL1d+nQVSnXWbeuG1SQfS0MiwY3DH4uRRc3/rgXlXZl+DHSpoVp6eRICkvop7MQDg8WcKeo8r9lRqYtmo+XGokIcw="#;

const VIDEO_OFF: &'static str = r#"eJxNjLEKwzAQQ39FeHfqU83hwfGcoV07dAu0kIIpHUpI/76XEHDQIAn0lD/jd8Kjd1cJnSp0kDgSRDCJOWd251gFciGR5uRKPq1QyQ1ViDbKWxrisXve0vHVc2IVi+He7urr/cTC3pEOPzE32+uyVZuuo/IHClEqnw=="#;

const VIDEO: &'static str = r#"eJwti8EKgCAQRH9l2HtEIuJB/YwO3SKl9RCELFR/3xodhhnm8cK5CiNHOoyBHxwsNLNfKIWxsxRa2QR3JENofz1fXTULR5osgUvdWXTrrdB1u3vpBeovGi0="#;

const VIDEOTAPE: &'static str = r#"eJx9zVEKgCAMBuCrjF2glAgf1Bt0iFBpQg8hg+r2TX0Jgp42/p9vsyUFhnI51AhnjkyyjQg9oJQ3YodqRrgdTujtUIG3x8oE0eGiwZAAKWrkbcgl7AlK80HOGBliVcO9fXEDSpP54fX1xz+BmTO6"#;

const VIEW: &'static str = r#"eJxtjLEOwkAMQ3/F6p5yMRduKZ27sLKfysCIVHQDX09SJApSFVmKk2cPj/q843buLgblwt6yiaGIzbnPlpFQYKFF4nmCiZtZ4mtI4mTo1Y3DIbrG4duohB6rQr0klMQPn20d7oXoRGmsBFdIPcTJfr2wCfej5bqRKcjpz4NtC74BR305uQ=="#;

const VOICEMAIL: &'static str = r#"eJxNzDEKwDAIheGriBcohlI6RC8jGQKhQyZz+1ZtodND/g+r9qmjgRrjgTAZdwRdjFRQ6pZV6qcyhKbz5T82+tXASkajeLn89KXYR7uSGyE1IQA="#;

const VOLUME_1: &'static str = r#"eJwtTFEKgCAUu8rDA5gvfJqg3qBDCEEFoUL+dPueImODjW2+luc7S4Za7tzeIBCBwICDdRC7YeEYXVcS0S9zFH1N7YIjiB1JkoZNapOIDxQDmVYqOwbciz/+Exru"#;

const VOLUME_2: &'static str = r#"eJxNjFEKhDAMRK8SeoBsZzfRLVRv4CEKggpiC/rj7Y0iImEGMryZWPK8D3mhkqdlWxsHkFJFgb6XcD5mFiOcrq6Nn7vUxpK2kfrGdVBWoT9LldQGvB1MNfv6Khj3poPlJBx+CQb6h4cw5CkcR2kmRg=="#;

const VOLUME_X: &'static str = r#"eJw1i0EKwCAMBL+y5AMlgoKg/qYUQVRoD/r7JtYesmGTmdBbmVer6C3X547EDAsHD7OGtUjImb2mpRSOLaVQcj0xjGiOMDmSl6XVEoZUYxRXarObUUV+i1nq5/zsC3qfJiw="#;

const VOLUME: &'static str =
    r#"eJwti1EKABAQRK8yuYBWUQrXkZJV/Li9XfmYV/Pqpcn9VB6Y3MZe2RDBIyDCvZEegWiKSm9Ksj8qFxX0EBo="#;

const VOTE: &'static str = r#"eJxtiyEPgCAQRv/KzX7Id4MxNiRTrAYb00BxMzh/v1zQ5F5876WzXo32aTgiQUhx7IacRhU5vXr2FDbLMDCRe8XSYKvmtgMdb0jxS1h/ZhFCLPKZB93vHd4="#;

const WALLET_2: &'static str = r#"eJxNTKsOgDAM/JVmfrBrttSUaQwWvwRRiSB8P8Vsy4l75U7v9hhdWzgghGxLQqi6/mnV3gmJgRsTU3LAmV+kEURXe5l95LPMg8iG3K8/zgUbpA=="#;

const WALLET_CARDS: &'static str = r#"eJxVjcEKwkAMRH9l2Htjk221hd2evfgRZVvcggcpC+rfm+xBLIEMeTNDwr6mgv0dnTi8tqXk6HhwUOAdPnXndbvnUvkUTlaYwnMuGUt0N49xFghaHYY0krn7BxArWfxQYs4+0WAZOpOHENP4YNtJiVF0xFV7uqA1r1GT+oZ6xV4Pa6lchX8/vnUgMbA="#;

const WALLET: &'static str = r#"eJxtzKEOgDAMhOFXafALbbkBYkxjsAjcEkQlgvT5GWZBkBNnvvzpKrfRuXSbConu0xqLkhLXCXGACRxdTv0Lc2p8oOiCZrm+mowe4g+WubY/lgkGDziafQAcGSIR"#;

const WALLPAPER: &'static str = r#"eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2pghtD2VZhpnv8IV0H9PjhjEKBekTxZf4RumkD4c/7MNreGdcozw7aIvGqik/EIQrp6D1ap2eSGh9qRfiDA2P1baDk7p1KMmszXYwnNWzchWfPajZ7wAtynaqFvIDr0sytQ=="#;

const WAND_2: &'static str = r#"eJxtj7EKwzAMRH/lyB7Xkh3bATdzhvYHugU6ZGihQ8n3V3JJCIkRHEZPvpPyZ/rOeF6bN5MJHk6kJcOpyCRCKGK1pBkZ9sbGBVCS2eOElM689DdWC33soBqUsE5sDvwf8WiGfNHNhrztRx4RDu6M7h3C4it96kG+Tix44QqISGMtQo8Mc1v1Irix38APsIlS9A=="#;

const WAND: &'static str = r#"eJx1zTEOgCAMheGrNOwgLYI0QW7g6m7iwGLiYDy/4kRiWTq8P1+azu0qsM9qQQ/jSiqnoU45tQHDraUUgYu0k+0EnEwExHoY0MnfuNgeDYaq9P9+OCAE1ixRetlHsaEPPOZELw=="#;

const WAREHOUSE: &'static str = r#"eJxtjjELwjAQhf/KI3tick1DhiTg5uLawa3YYgodRALqvzeHUlsot7x77zvehXtfMoYozkTwqmk70j2BoOsYWdXJrndJHVPHxUKjyMGpdvayyj8Lo6yHnj0TK/7XcxEpHLg8heUFB+Ozof3EbpLHeC14TkPJUVQfeZxuuXz1Kwon8K6L5gNG0wea/jfT"#;

const WATCH: &'static str = r#"eJxNjtEOgyAMRX+l4Z1O6lZ4UP/FMBNImBrnw/z7FYkboaHJ7b2n7XzcfJrAf3plSIE/St96xWrobmU8dOuSjhTnCdYlzvs7m8A0kH+pViq7L5f4xz3As1cvwyhji8wandF3bB4jAUGTnyZtkE3QhOxqGbKcNFoHOXGyhVhxLToHAm8ZxfPPQskGQku1ei5K1wk/4BdvIT54"#;

const WAVES: &'static str = r#"eJy1jT0KgDAMRq/y0d1ooq0KtYuzh5A6ODp4f0w6CV2VEMjL34vXfp84FrcJQqZAHkwChli1jrDwmJV8FgrodDJAtCdlx9g3yppMvTKTbZd7l2JrghRfGpbKU336yDT9aXoAe60/UA=="#;

const WEBCAM: &'static str = r#"eJxNy8EJgDAMheFVQu6ijYIekm7gEBKFCh6kiOj2Riy2p8D7/rCuUbcF9BJ0hKC33QYhCg7ouf7Yc8psbnOUnopsn44As+DYA1GwxuzdCnFkdFbdTw+VVyT6"#;

const WEBHOOK: &'static str = r#"eJxtz0sKwlAMBdCthM5zfXn/QC2IYxdR6qATwYH7x9uKOimBQD4ckvE5v1a5n4ebdbEK76sWuC9qMAnMzjJrROYcfsmSJTBMolhbEEzRELmYUZrGYRpPmzmNX/lRuSgJlii3vqAkhTcYUeuKopzNH9cYFd01I9QDyqLUnRJS6WoFlXpE228X9rc30vy/Mkj/OW9lxjQU"#;

const WHEAT_OFF: &'static str = r#"eJx1kstqwzAQRX9l8F5TzehhC5J8gbPtIjvjFlxwSxelJH/fOzZ1H5ERGr2urkZHOrwPHxM9HZtXJVUS78Q3p8ODTZ8O26Jk6pywtGThXnAOHLGmnAIlEpnFehaGwImsehRBjVxiD02Zna27msiZ6FLJo0Ma0P/f4r99C0maN9uRU4I8cWfJa+JibanlL56LUGLNIws2aeYQsRc22XEbegkUdm/F2uH8rBVj9aSTfuoQKa5qh97k9DH/TFF0teueRRas7RLDxqySwkKM/Ap2j+uqudTfV3KN7Gb8lyw0sdASVmvD6sC3domO24j02ZcRXB34gqsDX3DFmblXsS+z8x3sM2TFi/zynl/enukqx0YbuioatLd1eFuHkJro9AXEIaX0"#;

const WHEAT: &'static str = r#"eJydkDEOgzAMRa/yxZ4UOw4QiXKCdu3QDakDA0OHqudvnFR0SQGhyJZlf/k5v3+OrwmPc3VlMIMadNXQn7Q79MvMWWlBbL2DB9FMWmkanfXQqOOjGGKDXKImzEbnpiQyKroXMK1iOt0d0G5BAsgfgRApRdJyB7eFiZr44QMcrsETv3kUSBabWE2Gb82vBTErR1IyRC/42lm4M/FRZ8//XZk1RZDPiIxbLF0BJd8PgIIiUmJaLF3hZOP3gj5MWZvt"#;

const WHOLE_WORD: &'static str = r#"eJx1zMEKwjAQBNBfGfaemkmtUUhy9uJHyCooeJBSQvv3bWhoT73sLju8Cfrt9feGTlHoBH2UVqBjFC8pnNY0hf9z+OAV5UGLW76UqLxSqLyyWlI4D/wZPl93vwXOgT5TLZrONB1oCN5bLbc1XNYymA03OwNZzTKk"#;

const WIFI_OFF: &'static str = r#"eJxtkEEKwjAQRa8ydO83M8lMI7S9gYcIKiiIuHBRb++ktlKhZPEJvM8fXne/PS40St+INPSec2RP/04xdPsKDd2zvK507ptjhhIbtCgpBX9MLYUKVmQFCmVkKez4AiZwuxOYbuAcYEZ6Sgi8QzRvc8KBmBETRbS2VTJkrYhoYV8I85BAhBiWNzrOxz9YIcmvWg98xbgBXkyEZhLF4ufNrsLPzgfgpE5Y"#;

const WIFI: &'static str = r#"eJxtykEKwjAUBNCrDP8A8c/XaBZJbuAhAgoKIi5ctLfvT2lLF2VmM8PLv/Z/4VHkHsFzo8LbQ/AClZpPXdS8uRRcXkNsEXGRt0NoSCFZo/MVmu7l5/19YrAitKAUjCxiKhjYL982b/dd1gnUwCki"#;

const WIND: &'static str = r#"eJxtyz0KgDAMhuGrfPQA0cTaH6gFNxcPUXBwdPD+mAwWBwnvlOcrV7tPHIvbOVKE1oRmWCPYjhI8TZu4WgbDtfRJpqC/sArk1Yz0S1nUcibfsebB4aMfD3kgyQ=="#;

const WINE_OFF: &'static str = r#"eJxtT8sKwkAM/JWh98ZNso8s1IJ49iOKCiuoeBCpf++2PvBQQiaZZBKS7jbcCw7rZmcQKdb03Woq9d2vkcCu6CXBlZZJvS5oWMDhkZamSV3CjBsWUsUb3WzT7iEgfClpZvgJd0ZR602UjbdGKdQ0RoORJam5emRIiXumAA9BrG51B3mXWnKBzOWW2Acwcf577Hy6HjHKuhFp8OQaa/jQcaZVOon6F9E9QqE="#;

const WINE: &'static str = r#"eJx1jLEOgCAQQ3+lYT/kiCeYILOLP+BGcGB0MH6/6EAcNJdL076mYU9HwTapxcPa4lUM3R3F0IADm8Lmg7AFy+n+SBIIzHNCkg1Z0kJ9FT+PmVgLqsFQ37+7kLVNXnRpKBY="#;

const WORKFLOW: &'static str = r#"eJxNjVEKgCAMQK8ydoHSgvpQb9AhJKX5FzLKbp8OCtnHxuPxZnLcGe4UmCyuCI/FCYFiOogFFAG5Lo3ODE135vRMECxuCyh1zV6DhlGmXjQ3sSnOSL2r1bz6c93bIvzrvyajKOo="#;

const WRAP_TEXT: &'static str = r#"eJw1jlEKg0AMRK8y5L90k7WLH7t7gx5CaGEFUaF+6O2bRCWBScIjM3ka5y92KSRMOLhQUhGXXbdINT+NqXkdtoZPoXcES+PXEBERwFoBqT06Qw1SdJkOf7wu47z9CnGCdQfubZDg7EXV7KyZc3/beiYOZxg93zH+GHkugA=="#;

const WRENCH: &'static str = r#"eJxNjrEKgDAMRH/lcE80tqQItX/gRxQcHDo4OPn1Nq2gBF7gchcunvk6sK/DJp4DlF0WCKZ3hH0R1rr1p1cVU3EcAhmyQrtOgRcPQyHlRWDIM8uMhm5y5IodmuULtxz1cH2rMNxDiqN1TA/GsCOZ"#;

const X_CIRCLE: &'static str = r#"eJxNyTsKgDAQhOGrDNuLRjCwsMldZBUUFCRYJLfPq0k1zP+J3kGfE5ocmZUQyiwEje16mbt7+fb/wuHoNRt4srAVaxyIwRglA8gkGc8="#;

const X_OCTAGON: &'static str = r#"eJxNzEEKgCAQheGrPNxXKDUlqHcJggpKhdx0+0QNXQ18//CUd9e7OwvvThsezeZ+IQhw6vkYrxDI8lMJletvkszMqKEsG+XXcGDT7OYTZEegVCM2SUKiLR8PxSW3"#;

const X_SQUARE: &'static str = r#"eJxNi0EKgCAURK8y/H2ERZKg3iVS0kUQ8qG8ff7auJkZ5s3YEnfG42gm1E/vHDg5UishxXwk/nNpm6lZFfN2lJ+318YJwdGpFphBQwuSskMGBj15AUHdH9U="#;

const X: &'static str =
    r#"eJw9ybEJACAMRNFVQhaQWEiKMxs4hGBhI1i4P0ZBuav+w6yrU8tcRCn5RNkQTjU8Gxei/9sGq2sO7w=="#;

const YOUTUBE: &'static str = r#"eJxtjkEKgDAMBL8SvDc2tVGEKvgAH1Hw4EXw4MnXG2uqghKykOwybFjjNsPUFaNDBmqi80gOLrUyBNaQBXnpSeiNLPgWuVZVp0YJDU/UkTB/kfBGJlxGsqo6H2RqWfShPIv3IddfBEgMbCoju9+BAwNZLQw="#;

const ZAP_OFF: &'static str = r#"eJyFy8EKgzAQBNBfGfIB2+6axAjRv+lBEBXaQ/P3jkEP4sHL7gzMy+sylWmcP1iXcf59e6cmXhGlDdAGBn1LaOGlMzfk1zkf8h2mfUjeKUzpoEFiZHiACQkN1NPWS85AGK+womK9M3P4Kz/fUUutnNfpBkv1OoI="#;

const ZAP: &'static str =
    r#"eJwti7EJACEQBFtZbOB/1w992xFBPEETu1fEZIJhJjQrM1lFs1xH/x09BA9+oA4JCSL4HrO5CxfDc8+4AF5nEOw="#;

const ZOOM_IN: &'static str = r#"eJxtjLsNwCAMRFexboBEoARRAMtYFEgoBRVsH/NLleos33vnOBXOkbh5KAXiOrN4WAR3zjq4nJ5IVXloKdtOLbA5zA2q3ylSh5cykAtD7buDWxP2j93McuxE5b3ZF0IzLXU="#;

const ZOOM_OUT: &'static str = r#"eJxNi0EKwCAMBL8S8oCWSCse1M8ED4L04El/36SR4mkXZiZy7dwK8EhIhMDTticMmONpOMdWnwKTEjqB04nkD38jjO0alUjllXz8WommZKty+N0XW+cidQ=="#;

impl LucideIcon {
    fn decompress(&self, input: &str) -> String {
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
            &Self::BookmarkCheck => self.decompress(BOOKMARK_CHECK),
            &Self::BookmarkMinus => self.decompress(BOOKMARK_MINUS),
            &Self::BookmarkPlus => self.decompress(BOOKMARK_PLUS),
            &Self::BookmarkX => self.decompress(BOOKMARK_X),
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
            &Self::Drama => self.decompress(DRAMA),
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
            &Self::Speech => self.decompress(SPEECH),
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
            &Self::Theater => self.decompress(THEATER),
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
