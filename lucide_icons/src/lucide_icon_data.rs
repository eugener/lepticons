use strum_macros::{EnumProperty,EnumIter};
use core::fmt;

use fmt::Result;

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone )]
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
const ACCESSIBILITY: &str = r#"
<circle cx="16" cy="4" r="1"></circle>
<path d="m18 19 1-7-6 1"></path>
<path d="m5 8 3-3 5.5 3-2.36 3.5"></path>
<path d="M4.24 14.5a5 5 0 0 0 6.88 6"></path>
<path d="M13.76 17.5a5 5 0 0 0-6.88-6"></path>"#;

const ACTIVITY_SQUARE: &str = r#"
<rect rx="2" height="18" y="3" width="18" x="3"></rect>
<path d="M17 12h-2l-2 5-2-10-2 5H7"></path>"#;

const ACTIVITY: &str = r#"
<path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>"#;

const AIR_VENT: &str = r#"
<path d="M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path>
<path d="M6 8h12"></path>
<path d="M18.3 17.7a2.5 2.5 0 0 1-3.16 3.83 2.53 2.53 0 0 1-1.14-2V12"></path>
<path d="M6.6 15.6A2 2 0 1 0 10 17v-5"></path>"#;

const AIRPLAY: &str = r#"
<path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"></path>
<polygon points="12 15 17 21 7 21 12 15"></polygon>"#;

const ALARM_CHECK: &str = r#"
<circle cx="12" cy="13" r="8"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="m9 13 2 2 4-4"></path>"#;

const ALARM_CLOCK_OFF: &str = r#"
<path d="M6.87 6.87a8 8 0 1 0 11.26 11.26"></path>
<path d="M19.9 14.25a8 8 0 0 0-9.15-9.15"></path>
<path d="m22 6-3-3"></path>
<path d="M6.26 18.67 4 21"></path>
<path d="m2 2 20 20"></path>
<path d="M4 4 2 6"></path>"#;

const ALARM_CLOCK: &str = r#"
<circle cy="13" cx="12" r="8"></circle>
<path d="M12 9v4l2 2"></path>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>"#;

const ALARM_MINUS: &str = r#"
<circle cy="13" r="8" cx="12"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="M9 13h6"></path>"#;

const ALARM_PLUS: &str = r#"
<circle cx="12" cy="13" r="8"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="M12 10v6"></path>
<path d="M9 13h6"></path>"#;

const ALBUM: &str = r#"
<rect width="18" ry="2" height="18" x="3" y="3" rx="2"></rect>
<polyline points="11 3 11 11 14 8 17 11 17 3"></polyline>"#;

const ALERT_CIRCLE: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<line y1="8" x2="12" x1="12" y2="12"></line>
<line x2="12.01" y2="16" x1="12" y1="16"></line>"#;

const ALERT_OCTAGON: &str = r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>
<line x1="12" y1="8" y2="12" x2="12"></line>
<line x2="12.01" x1="12" y2="16" y1="16"></line>"#;

const ALERT_TRIANGLE: &str = r#"
<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>
<path d="M12 9v4"></path>
<path d="M12 17h.01"></path>"#;

const ALIGN_CENTER_HORIZONTAL: &str = r#"
<path d="M2 12h20"></path>
<path d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4"></path>
<path d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4"></path>
<path d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1"></path>
<path d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1"></path>"#;

const ALIGN_CENTER_VERTICAL: &str = r#"
<path d="M12 2v20"></path>
<path d="M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4"></path>
<path d="M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4"></path>
<path d="M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1"></path>
<path d="M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1"></path>"#;

const ALIGN_CENTER: &str = r#"
<line x1="21" y1="6" y2="6" x2="3"></line>
<line x2="7" x1="17" y1="12" y2="12"></line>
<line x1="19" y1="18" x2="5" y2="18"></line>"#;

const ALIGN_END_HORIZONTAL: &str = r#"
<rect rx="2" height="16" width="6" x="4" y="2"></rect>
<rect width="6" height="9" y="9" rx="2" x="14"></rect>
<path d="M22 22H2"></path>"#;

const ALIGN_END_VERTICAL: &str = r#"
<rect y="4" height="6" x="2" width="16" rx="2"></rect>
<rect y="14" height="6" width="9" x="9" rx="2"></rect>
<path d="M22 22V2"></path>"#;

const ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: &str = r#"
<rect rx="2" y="5" x="4" height="14" width="6"></rect>
<rect height="10" x="14" y="7" width="6" rx="2"></rect>
<path d="M17 22v-5"></path>
<path d="M17 7V2"></path>
<path d="M7 22v-3"></path>
<path d="M7 5V2"></path>"#;

const ALIGN_HORIZONTAL_DISTRIBUTE_END: &str = r#"
<rect width="6" height="14" y="5" x="4" rx="2"></rect>
<rect width="6" rx="2" y="7" height="10" x="14"></rect>
<path d="M10 2v20"></path>
<path d="M20 2v20"></path>"#;

const ALIGN_HORIZONTAL_DISTRIBUTE_START: &str = r#"
<rect rx="2" height="14" width="6" x="4" y="5"></rect>
<rect width="6" height="10" x="14" rx="2" y="7"></rect>
<path d="M4 2v20"></path>
<path d="M14 2v20"></path>"#;

const ALIGN_HORIZONTAL_JUSTIFY_CENTER: &str = r#"
<rect width="6" x="2" rx="2" height="14" y="5"></rect>
<rect rx="2" width="6" height="10" x="16" y="7"></rect>
<path d="M12 2v20"></path>"#;

const ALIGN_HORIZONTAL_JUSTIFY_END: &str = r#"
<rect rx="2" y="5" width="6" x="2" height="14"></rect>
<rect rx="2" width="6" height="10" x="12" y="7"></rect>
<path d="M22 2v20"></path>"#;

const ALIGN_HORIZONTAL_JUSTIFY_START: &str = r#"
<rect width="6" x="6" height="14" y="5" rx="2"></rect>
<rect x="16" width="6" height="10" rx="2" y="7"></rect>
<path d="M2 2v20"></path>"#;

const ALIGN_HORIZONTAL_SPACE_AROUND: &str = r#"
<rect rx="2" x="9" width="6" y="7" height="10"></rect>
<path d="M4 22V2"></path>
<path d="M20 22V2"></path>"#;

const ALIGN_HORIZONTAL_SPACE_BETWEEN: &str = r#"
<rect x="3" y="5" width="6" height="14" rx="2"></rect>
<rect x="15" height="10" y="7" width="6" rx="2"></rect>
<path d="M3 2v20"></path>
<path d="M21 2v20"></path>"#;

const ALIGN_JUSTIFY: &str = r#"
<line y1="6" y2="6" x2="21" x1="3"></line>
<line x1="3" y1="12" y2="12" x2="21"></line>
<line x1="3" y1="18" x2="21" y2="18"></line>"#;

const ALIGN_LEFT: &str = r#"
<line y2="6" x2="3" x1="21" y1="6"></line>
<line y2="12" x2="3" y1="12" x1="15"></line>
<line y2="18" y1="18" x1="17" x2="3"></line>"#;

const ALIGN_RIGHT: &str = r#"
<line y2="6" x1="21" x2="3" y1="6"></line>
<line x1="21" y1="12" x2="9" y2="12"></line>
<line x2="7" x1="21" y1="18" y2="18"></line>"#;

const ALIGN_START_HORIZONTAL: &str = r#"
<rect y="6" height="16" width="6" x="4" rx="2"></rect>
<rect width="6" height="9" x="14" rx="2" y="6"></rect>
<path d="M22 2H2"></path>"#;

const ALIGN_START_VERTICAL: &str = r#"
<rect x="6" height="6" width="9" rx="2" y="14"></rect>
<rect rx="2" x="6" y="4" width="16" height="6"></rect>
<path d="M2 2v20"></path>"#;

const ALIGN_VERTICAL_DISTRIBUTE_CENTER: &str = r#"
<rect x="5" rx="2" width="14" height="6" y="14"></rect>
<rect width="10" rx="2" height="6" y="4" x="7"></rect>
<path d="M22 7h-5"></path>
<path d="M7 7H1"></path>
<path d="M22 17h-3"></path>
<path d="M5 17H2"></path>"#;

const ALIGN_VERTICAL_DISTRIBUTE_END: &str = r#"
<rect y="14" width="14" height="6" x="5" rx="2"></rect>
<rect width="10" height="6" x="7" y="4" rx="2"></rect>
<path d="M2 20h20"></path>
<path d="M2 10h20"></path>"#;

const ALIGN_VERTICAL_DISTRIBUTE_START: &str = r#"
<rect rx="2" x="5" y="14" height="6" width="14"></rect>
<rect rx="2" height="6" x="7" y="4" width="10"></rect>
<path d="M2 14h20"></path>
<path d="M2 4h20"></path>"#;

const ALIGN_VERTICAL_JUSTIFY_CENTER: &str = r#"
<rect width="14" x="5" y="16" height="6" rx="2"></rect>
<rect rx="2" height="6" x="7" y="2" width="10"></rect>
<path d="M2 12h20"></path>"#;

const ALIGN_VERTICAL_JUSTIFY_END: &str = r#"
<rect rx="2" y="12" width="14" height="6" x="5"></rect>
<rect width="10" x="7" y="2" rx="2" height="6"></rect>
<path d="M2 22h20"></path>"#;

const ALIGN_VERTICAL_JUSTIFY_START: &str = r#"
<rect width="14" height="6" y="16" rx="2" x="5"></rect>
<rect rx="2" width="10" x="7" height="6" y="6"></rect>
<path d="M2 2h20"></path>"#;

const ALIGN_VERTICAL_SPACE_AROUND: &str = r#"
<rect y="9" x="7" width="10" height="6" rx="2"></rect>
<path d="M22 20H2"></path>
<path d="M22 4H2"></path>"#;

const ALIGN_VERTICAL_SPACE_BETWEEN: &str = r#"
<rect y="15" rx="2" height="6" x="5" width="14"></rect>
<rect width="10" y="3" rx="2" height="6" x="7"></rect>
<path d="M2 21h20"></path>
<path d="M2 3h20"></path>"#;

const AMPERSAND: &str = r#"
<path d="M17.5 12c0 4.4-3.6 8-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13"></path>
<path d="M16 12h3"></path>"#;

const AMPERSANDS: &str = r#"
<path d="M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"></path>
<path d="M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"></path>"#;

const ANCHOR: &str = r#"
<circle r="3" cx="12" cy="5"></circle>
<line x1="12" y1="22" x2="12" y2="8"></line>
<path d="M5 12H2a10 10 0 0 0 20 0h-3"></path>"#;

const ANGRY: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M16 16s-1.5-2-4-2-4 2-4 2"></path>
<path d="M7.5 8 10 9"></path>
<path d="m14 9 2.5-1"></path>
<path d="M9 10h0"></path>
<path d="M15 10h0"></path>"#;

const ANNOYED: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M8 15h8"></path>
<path d="M8 9h2"></path>
<path d="M14 9h2"></path>"#;

const ANTENNA: &str = r#"
<path d="M2 12 7 2"></path>
<path d="m7 12 5-10"></path>
<path d="m12 12 5-10"></path>
<path d="m17 12 5-10"></path>
<path d="M4.5 7h15"></path>
<path d="M12 16v6"></path>"#;

const APERTURE: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<line x2="20.05" x1="14.31" y1="8" y2="17.94"></line>
<line y2="8" x1="9.69" x2="21.17" y1="8"></line>
<line y1="12" x2="13.12" x1="7.38" y2="2.06"></line>
<line x1="9.69" x2="3.95" y1="16" y2="6.06"></line>
<line y1="16" x2="2.83" y2="16" x1="14.31"></line>
<line y1="12" x1="16.62" y2="21.94" x2="10.88"></line>"#;

const APP_WINDOW: &str = r#"
<rect y="4" x="2" height="16" width="20" rx="2"></rect>
<path d="M10 4v4"></path>
<path d="M2 8h20"></path>
<path d="M6 4v4"></path>"#;

const APPLE: &str = r#"
<path d="M12 20.94c1.5 0 2.75 1.06 4 1.06 3 0 6-8 6-12.22A4.91 4.91 0 0 0 17 5c-2.22 0-4 1.44-5 2-1-.56-2.78-2-5-2a4.9 4.9 0 0 0-5 4.78C2 14 5 22 8 22c1.25 0 2.5-1.06 4-1.06Z"></path>
<path d="M10 2c1 .5 2 2 2 5"></path>"#;

const ARCHIVE_RESTORE: &str = r#"
<rect y="3" rx="1" width="20" x="2" height="5"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h2"></path>
<path d="M20 8v11a2 2 0 0 1-2 2h-2"></path>
<path d="m9 15 3-3 3 3"></path>
<path d="M12 12v9"></path>"#;

const ARCHIVE_X: &str = r#"
<rect y="3" rx="1" width="20" height="5" x="2"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
<path d="m9.5 17 5-5"></path>
<path d="m9.5 12 5 5"></path>"#;

const ARCHIVE: &str = r#"
<rect height="5" y="3" width="20" x="2" rx="1"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
<path d="M10 12h4"></path>"#;

const AREA_CHART: &str = r#"
<path d="M3 3v18h18"></path>
<path d="M7 12v5h12V8l-5 5-4-4Z"></path>"#;

const ARMCHAIR: &str = r#"
<path d="M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3"></path>
<path d="M3 11v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H7v-2a2 2 0 0 0-4 0Z"></path>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#;

const ARROW_BIG_DOWN_DASH: &str = r#"
<path d="M15 5H9"></path>
<path d="M15 9v3h4l-7 7-7-7h4V9h6z"></path>"#;

const ARROW_BIG_DOWN: &str = r#"
<path d="M15 6v6h4l-7 7-7-7h4V6h6z"></path>"#;

const ARROW_BIG_LEFT_DASH: &str = r#"
<path d="M19 15V9"></path>
<path d="M15 15h-3v4l-7-7 7-7v4h3v6z"></path>"#;

const ARROW_BIG_LEFT: &str = r#"
<path d="M18 15h-6v4l-7-7 7-7v4h6v6z"></path>"#;

const ARROW_BIG_RIGHT_DASH: &str = r#"
<path d="M5 9v6"></path>
<path d="M9 9h3V5l7 7-7 7v-4H9V9z"></path>"#;

const ARROW_BIG_RIGHT: &str = r#"
<path d="M6 9h6V5l7 7-7 7v-4H6V9z"></path>"#;

const ARROW_BIG_UP_DASH: &str = r#"
<path d="M9 19h6"></path>
<path d="M9 15v-3H5l7-7 7 7h-4v3H9z"></path>"#;

const ARROW_BIG_UP: &str = r#"
<path d="M9 18v-6H5l7-7 7 7h-4v6H9z"></path>"#;

const ARROW_DOWN_01: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<rect y="4" height="6" ry="2" x="15" width="4"></rect>
<path d="M17 20v-6h-2"></path>
<path d="M15 20h4"></path>"#;

const ARROW_DOWN_10: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M17 10V4h-2"></path>
<path d="M15 10h4"></path>
<rect x="15" height="6" y="14" ry="2" width="4"></rect>"#;

const ARROW_DOWN_AZ: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M20 8h-5"></path>
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"></path>
<path d="M15 14h5l-5 6h5"></path>"#;

const ARROW_DOWN_CIRCLE: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<path d="M12 8v8"></path>
<path d="m8 12 4 4 4-4"></path>"#;

const ARROW_DOWN_FROM_LINE: &str = r#"
<path d="M19 3H5"></path>
<path d="M12 21V7"></path>
<path d="m6 15 6 6 6-6"></path>"#;

const ARROW_DOWN_LEFT_FROM_CIRCLE: &str = r#"
<path d="M2 12a10 10 0 1 1 10 10"></path>
<path d="m2 22 10-10"></path>
<path d="M8 22H2v-6"></path>"#;

const ARROW_DOWN_LEFT_SQUARE: &str = r#"
<rect x="3" width="18" height="18" y="3" rx="2"></rect>
<path d="m16 8-8 8"></path>
<path d="M16 16H8V8"></path>"#;

const ARROW_DOWN_LEFT: &str = r#"
<path d="M17 7 7 17"></path>
<path d="M17 17H7V7"></path>"#;

const ARROW_DOWN_NARROW_WIDE: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M11 4h4"></path>
<path d="M11 8h7"></path>
<path d="M11 12h10"></path>"#;

const ARROW_DOWN_RIGHT_FROM_CIRCLE: &str = r#"
<path d="M12 22a10 10 0 1 1 10-10"></path>
<path d="M22 22 12 12"></path>
<path d="M22 16v6h-6"></path>"#;

const ARROW_DOWN_RIGHT_SQUARE: &str = r#"
<rect x="3" width="18" height="18" y="3" rx="2"></rect>
<path d="m8 8 8 8"></path>
<path d="M16 8v8H8"></path>"#;

const ARROW_DOWN_RIGHT: &str = r#"
<path d="m7 7 10 10"></path>
<path d="M17 7v10H7"></path>"#;

const ARROW_DOWN_SQUARE: &str = r#"
<rect y="3" width="18" x="3" height="18" rx="2"></rect>
<path d="M12 8v8"></path>
<path d="m8 12 4 4 4-4"></path>"#;

const ARROW_DOWN_TO_DOT: &str = r#"
<path d="M12 2v14"></path>
<path d="m19 9-7 7-7-7"></path>
<circle r="1" cx="12" cy="21"></circle>"#;

const ARROW_DOWN_TO_LINE: &str = r#"
<path d="M12 17V3"></path>
<path d="m6 11 6 6 6-6"></path>
<path d="M19 21H5"></path>"#;

const ARROW_DOWN_UP: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="m21 8-4-4-4 4"></path>
<path d="M17 4v16"></path>"#;

const ARROW_DOWN_WIDE_NARROW: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M11 4h10"></path>
<path d="M11 8h7"></path>
<path d="M11 12h4"></path>"#;

const ARROW_DOWN_ZA: &str = r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 4v16"></path>
<path d="M15 4h5l-5 6h5"></path>
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"></path>
<path d="M20 18h-5"></path>"#;

const ARROW_DOWN: &str = r#"
<path d="M12 5v14"></path>
<path d="m19 12-7 7-7-7"></path>"#;

const ARROW_LEFT_CIRCLE: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M16 12H8"></path>
<path d="m12 8-4 4 4 4"></path>"#;

const ARROW_LEFT_FROM_LINE: &str = r#"
<path d="m9 6-6 6 6 6"></path>
<path d="M3 12h14"></path>
<path d="M21 19V5"></path>"#;

const ARROW_LEFT_RIGHT: &str = r#"
<path d="M8 3 4 7l4 4"></path>
<path d="M4 7h16"></path>
<path d="m16 21 4-4-4-4"></path>
<path d="M20 17H4"></path>"#;

const ARROW_LEFT_SQUARE: &str = r#"
<rect width="18" x="3" rx="2" y="3" height="18"></rect>
<path d="m12 8-4 4 4 4"></path>
<path d="M16 12H8"></path>"#;

const ARROW_LEFT_TO_LINE: &str = r#"
<path d="M3 19V5"></path>
<path d="m13 6-6 6 6 6"></path>
<path d="M7 12h14"></path>"#;

const ARROW_LEFT: &str = r#"
<path d="m12 19-7-7 7-7"></path>
<path d="M19 12H5"></path>"#;

const ARROW_RIGHT_CIRCLE: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<path d="M8 12h8"></path>
<path d="m12 16 4-4-4-4"></path>"#;

const ARROW_RIGHT_FROM_LINE: &str = r#"
<path d="M3 5v14"></path>
<path d="M21 12H7"></path>
<path d="m15 18 6-6-6-6"></path>"#;

const ARROW_RIGHT_LEFT: &str = r#"
<path d="m16 3 4 4-4 4"></path>
<path d="M20 7H4"></path>
<path d="m8 21-4-4 4-4"></path>
<path d="M4 17h16"></path>"#;

const ARROW_RIGHT_SQUARE: &str = r#"
<rect y="3" x="3" width="18" height="18" rx="2"></rect>
<path d="M8 12h8"></path>
<path d="m12 16 4-4-4-4"></path>"#;

const ARROW_RIGHT_TO_LINE: &str = r#"
<path d="M17 12H3"></path>
<path d="m11 18 6-6-6-6"></path>
<path d="M21 5v14"></path>"#;

const ARROW_RIGHT: &str = r#"
<path d="M5 12h14"></path>
<path d="m12 5 7 7-7 7"></path>"#;

const ARROW_UP_01: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<rect width="4" x="15" height="6" ry="2" y="4"></rect>
<path d="M17 20v-6h-2"></path>
<path d="M15 20h4"></path>"#;

const ARROW_UP_10: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M17 10V4h-2"></path>
<path d="M15 10h4"></path>
<rect width="4" ry="2" height="6" y="14" x="15"></rect>"#;

const ARROW_UP_AZ: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M20 8h-5"></path>
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"></path>
<path d="M15 14h5l-5 6h5"></path>"#;

const ARROW_UP_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m16 12-4-4-4 4"></path>
<path d="M12 16V8"></path>"#;

const ARROW_UP_DOWN: &str = r#"
<path d="m21 16-4 4-4-4"></path>
<path d="M17 20V4"></path>
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>"#;

const ARROW_UP_FROM_DOT: &str = r#"
<path d="m5 9 7-7 7 7"></path>
<path d="M12 16V2"></path>
<circle cy="21" r="1" cx="12"></circle>"#;

const ARROW_UP_FROM_LINE: &str = r#"
<path d="m18 9-6-6-6 6"></path>
<path d="M12 3v14"></path>
<path d="M5 21h14"></path>"#;

const ARROW_UP_LEFT_FROM_CIRCLE: &str = r#"
<path d="M2 8V2h6"></path>
<path d="m2 2 10 10"></path>
<path d="M12 2A10 10 0 1 1 2 12"></path>"#;

const ARROW_UP_LEFT_SQUARE: &str = r#"
<rect x="3" height="18" y="3" width="18" rx="2"></rect>
<path d="M8 16V8h8"></path>
<path d="M16 16 8 8"></path>"#;

const ARROW_UP_LEFT: &str = r#"
<path d="M7 17V7h10"></path>
<path d="M17 17 7 7"></path>"#;

const ARROW_UP_NARROW_WIDE: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M11 12h4"></path>
<path d="M11 16h7"></path>
<path d="M11 20h10"></path>"#;

const ARROW_UP_RIGHT_FROM_CIRCLE: &str = r#"
<path d="M22 12A10 10 0 1 1 12 2"></path>
<path d="M22 2 12 12"></path>
<path d="M16 2h6v6"></path>"#;

const ARROW_UP_RIGHT_SQUARE: &str = r#"
<rect width="18" x="3" y="3" height="18" rx="2"></rect>
<path d="M8 8h8v8"></path>
<path d="m8 16 8-8"></path>"#;

const ARROW_UP_RIGHT: &str = r#"
<path d="M7 7h10v10"></path>
<path d="M7 17 17 7"></path>"#;

const ARROW_UP_SQUARE: &str = r#"
<rect width="18" x="3" height="18" y="3" rx="2"></rect>
<path d="m16 12-4-4-4 4"></path>
<path d="M12 16V8"></path>"#;

const ARROW_UP_TO_LINE: &str = r#"
<path d="M5 3h14"></path>
<path d="m18 13-6-6-6 6"></path>
<path d="M12 7v14"></path>"#;

const ARROW_UP_WIDE_NARROW: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M11 12h10"></path>
<path d="M11 16h7"></path>
<path d="M11 20h4"></path>"#;

const ARROW_UP_ZA: &str = r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M15 4h5l-5 6h5"></path>
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"></path>
<path d="M20 18h-5"></path>"#;

const ARROW_UP: &str = r#"
<path d="m5 12 7-7 7 7"></path>
<path d="M12 19V5"></path>"#;

const ARROWS_UP_FROM_LINE: &str = r#"
<path d="m4 6 3-3 3 3"></path>
<path d="M7 17V3"></path>
<path d="m14 6 3-3 3 3"></path>
<path d="M17 17V3"></path>
<path d="M4 21h16"></path>"#;

const ASTERISK: &str = r#"
<path d="M12 6v12"></path>
<path d="M17.196 9 6.804 15"></path>
<path d="m6.804 9 10.392 6"></path>"#;

const AT_SIGN: &str = r#"
<circle cy="12" r="4" cx="12"></circle>
<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8"></path>"#;

const ATOM: &str = r#"
<circle cx="12" cy="12" r="1"></circle>
<path d="M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z"></path>
<path d="M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z"></path>"#;

const AWARD: &str = r#"
<circle cy="8" r="6" cx="12"></circle>
<path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11"></path>"#;

const AXE: &str = r#"
<path d="m14 12-8.5 8.5a2.12 2.12 0 1 1-3-3L11 9"></path>
<path d="M15 13 9 7l4-4 6 6h3a8 8 0 0 1-7 7z"></path>"#;

const AXIS_3_D: &str = r#"
<path d="M4 4v16h16"></path>
<path d="m4 20 7-7"></path>"#;

const BABY: &str = r#"
<path d="M9 12h.01"></path>
<path d="M15 12h.01"></path>
<path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5"></path>
<path d="M19 6.3a9 9 0 0 1 1.8 3.9 2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1"></path>"#;

const BACKPACK: &str = r#"
<path d="M4 20V10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2Z"></path>
<path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"></path>
<path d="M8 21v-5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v5"></path>
<path d="M8 10h8"></path>
<path d="M8 18h8"></path>"#;

const BADGE_ALERT: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line y1="8" x2="12" y2="12" x1="12"></line>
<line y1="16" x2="12.01" x1="12" y2="16"></line>"#;

const BADGE_CENT: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M12 7v10"></path>
<path d="M15.4 10a4 4 0 1 0 0 4"></path>"#;

const BADGE_CHECK: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m9 12 2 2 4-4"></path>"#;

const BADGE_DOLLAR_SIGN: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 18V6"></path>"#;

const BADGE_EURO: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M7 12h5"></path>
<path d="M15 9.4a4 4 0 1 0 0 5.2"></path>"#;

const BADGE_HELP: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
<line x1="12" x2="12.01" y2="17" y1="17"></line>"#;

const BADGE_INDIAN_RUPEE: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M8 8h8"></path>
<path d="M8 12h8"></path>
<path d="m13 17-5-1h1a4 4 0 0 0 0-8"></path>"#;

const BADGE_INFO: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line y1="16" y2="12" x2="12" x1="12"></line>
<line y2="8" x1="12" y1="8" x2="12.01"></line>"#;

const BADGE_JAPANESE_YEN: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m9 8 3 3v7"></path>
<path d="m12 11 3-3"></path>
<path d="M9 12h6"></path>
<path d="M9 16h6"></path>"#;

const BADGE_MINUS: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x2="16" y2="12" y1="12" x1="8"></line>"#;

const BADGE_PERCENT: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#;

const BADGE_PLUS: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x1="12" x2="12" y2="16" y1="8"></line>
<line x1="8" x2="16" y1="12" y2="12"></line>"#;

const BADGE_POUND_STERLING: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M8 12h4"></path>
<path d="M10 16V9.5a2.5 2.5 0 0 1 5 0"></path>
<path d="M8 16h7"></path>"#;

const BADGE_RUSSIAN_RUBLE: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M9 16h5"></path>
<path d="M9 12h5a2 2 0 1 0 0-4h-3v9"></path>"#;

const BADGE_SWISS_FRANC: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M11 17V8h4"></path>
<path d="M11 12h3"></path>
<path d="M9 16h4"></path>"#;

const BADGE_X: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line y2="15" x2="9" x1="15" y1="9"></line>
<line y1="9" x1="9" y2="15" x2="15"></line>"#;

const BADGE: &str = r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>"#;

const BAGGAGE_CLAIM: &str = r#"
<path d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2"></path>
<path d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10"></path>
<rect width="13" height="8" x="8" y="6" rx="1"></rect>
<circle r="2" cx="18" cy="20"></circle>
<circle r="2" cy="20" cx="9"></circle>"#;

const BAN: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="m4.9 4.9 14.2 14.2"></path>"#;

const BANANA: &str = r#"
<path d="M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5"></path>
<path d="M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z"></path>"#;

const BANKNOTE: &str = r#"
<rect y="6" rx="2" height="12" x="2" width="20"></rect>
<circle r="2" cx="12" cy="12"></circle>
<path d="M6 12h.01M18 12h.01"></path>"#;

const BAR_CHART_2: &str = r#"
<line y2="10" y1="20" x1="18" x2="18"></line>
<line y2="4" x1="12" y1="20" x2="12"></line>
<line y1="20" x2="6" x1="6" y2="14"></line>"#;

const BAR_CHART_3: &str = r#"
<path d="M3 3v18h18"></path>
<path d="M18 17V9"></path>
<path d="M13 17V5"></path>
<path d="M8 17v-3"></path>"#;

const BAR_CHART_4: &str = r#"
<path d="M3 3v18h18"></path>
<path d="M13 17V9"></path>
<path d="M18 17V5"></path>
<path d="M8 17v-3"></path>"#;

const BAR_CHART_BIG: &str = r#"
<path d="M3 3v18h18"></path>
<rect x="7" height="7" y="10" width="4" rx="1"></rect>
<rect width="4" height="12" y="5" x="15" rx="1"></rect>"#;

const BAR_CHART_HORIZONTAL_BIG: &str = r#"
<path d="M3 3v18h18"></path>
<rect height="4" y="5" width="12" rx="1" x="7"></rect>
<rect width="7" height="4" x="7" y="13" rx="1"></rect>"#;

const BAR_CHART_HORIZONTAL: &str = r#"
<path d="M3 3v18h18"></path>
<path d="M7 16h8"></path>
<path d="M7 11h12"></path>
<path d="M7 6h3"></path>"#;

const BAR_CHART: &str = r#"
<line x1="12" y1="20" y2="10" x2="12"></line>
<line y2="4" x2="18" y1="20" x1="18"></line>
<line x2="6" y1="20" x1="6" y2="16"></line>"#;

const BASELINE: &str = r#"
<path d="M4 20h16"></path>
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>"#;

const BATH: &str = r#"
<path d="M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5"></path>
<line x1="10" y1="5" y2="7" x2="8"></line>
<line x1="2" y1="12" x2="22" y2="12"></line>
<line y2="21" y1="19" x1="7" x2="7"></line>
<line x1="17" y1="19" y2="21" x2="17"></line>"#;

const BATTERY_CHARGING: &str = r#"
<path d="M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2"></path>
<path d="M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1"></path>
<path d="m11 7-3 5h4l-3 5"></path>
<line y2="13" x1="22" x2="22" y1="11"></line>"#;

const BATTERY_FULL: &str = r#"
<rect height="10" ry="2" width="16" x="2" rx="2" y="7"></rect>
<line y1="11" y2="13" x1="22" x2="22"></line>
<line y1="11" y2="13" x2="6" x1="6"></line>
<line x1="10" y2="13" y1="11" x2="10"></line>
<line y1="11" x1="14" y2="13" x2="14"></line>"#;

const BATTERY_LOW: &str = r#"
<rect rx="2" x="2" ry="2" y="7" width="16" height="10"></rect>
<line y1="11" y2="13" x2="22" x1="22"></line>
<line x2="6" y2="13" x1="6" y1="11"></line>"#;

const BATTERY_MEDIUM: &str = r#"
<rect ry="2" width="16" height="10" x="2" y="7" rx="2"></rect>
<line y2="13" y1="11" x1="22" x2="22"></line>
<line x2="6" y2="13" x1="6" y1="11"></line>
<line y2="13" x2="10" y1="11" x1="10"></line>"#;

const BATTERY_WARNING: &str = r#"
<path d="M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2"></path>
<path d="M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2"></path>
<line x1="22" y1="11" y2="13" x2="22"></line>
<line y1="7" x1="10" x2="10" y2="13"></line>
<line y2="17.01" x2="10" x1="10" y1="17"></line>"#;

const BATTERY: &str = r#"
<rect x="2" height="10" y="7" rx="2" width="16" ry="2"></rect>
<line x1="22" x2="22" y1="11" y2="13"></line>"#;

const BEAKER: &str = r#"
<path d="M4.5 3h15"></path>
<path d="M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3"></path>
<path d="M6 14h12"></path>"#;

const BEAN_OFF: &str = r#"
<path d="M9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1"></path>
<path d="M10.75 5.093A6 6 0 0 1 22 8c0 2.411-.61 4.68-1.683 6.66"></path>
<path d="M5.341 10.62a4 4 0 0 0 6.487 1.208M10.62 5.341a4.015 4.015 0 0 1 2.039 2.04"></path>
<line y1="2" y2="22" x2="22" x1="2"></line>"#;

const BEAN: &str = r#"
<path d="M10.165 6.598C9.954 7.478 9.64 8.36 9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22c7.732 0 14-6.268 14-14a6 6 0 0 0-11.835-1.402Z"></path>
<path d="M5.341 10.62a4 4 0 1 0 5.279-5.28"></path>"#;

const BED_DOUBLE: &str = r#"
<path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8"></path>
<path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"></path>
<path d="M12 4v6"></path>
<path d="M2 18h20"></path>"#;

const BED_SINGLE: &str = r#"
<path d="M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8"></path>
<path d="M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4"></path>
<path d="M3 18h18"></path>"#;

const BED: &str = r#"
<path d="M2 4v16"></path>
<path d="M2 8h18a2 2 0 0 1 2 2v10"></path>
<path d="M2 17h20"></path>
<path d="M6 8v9"></path>"#;

const BEEF: &str = r#"
<circle cy="8.5" cx="12.5" r="2.5"></circle>
<path d="M12.5 2a6.5 6.5 0 0 0-6.22 4.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3A6.5 6.5 0 0 0 12.5 2Z"></path>
<path d="m18.5 6 2.19 4.5a6.48 6.48 0 0 1 .31 2 6.49 6.49 0 0 1-2.6 5.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5"></path>"#;

const BEER: &str = r#"
<path d="M17 11h1a3 3 0 0 1 0 6h-1"></path>
<path d="M9 12v6"></path>
<path d="M13 12v6"></path>
<path d="M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z"></path>
<path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8"></path>"#;

const BELL_DOT: &str = r#"
<path d="M19.4 14.9C20.2 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 .7 0 1.3.1 1.9.3"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<circle r="3" cx="18" cy="8"></circle>"#;

const BELL_MINUS: &str = r#"
<path d="M18.4 12c.8 3.8 2.6 5 2.6 5H3s3-2 3-9c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M15 8h6"></path>"#;

const BELL_OFF: &str = r#"
<path d="M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5"></path>
<path d="M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="m2 2 20 20"></path>"#;

const BELL_PLUS: &str = r#"
<path d="M19.3 14.8C20.1 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 1 0 1.9.2 2.8.7"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M15 8h6"></path>
<path d="M18 5v6"></path>"#;

const BELL_RING: &str = r#"
<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M4 2C2.8 3.7 2 5.7 2 8"></path>
<path d="M22 8c0-2.3-.8-4.3-2-6"></path>"#;

const BELL: &str = r#"
<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>"#;

const BIKE: &str = r#"
<circle cx="18.5" r="3.5" cy="17.5"></circle>
<circle r="3.5" cx="5.5" cy="17.5"></circle>
<circle cy="5" r="1" cx="15"></circle>
<path d="M12 17.5V14l-3-3 4-3 2 3h2"></path>"#;

const BINARY: &str = r#"
<rect rx="2" width="4" x="14" height="6" y="14"></rect>
<rect height="6" rx="2" width="4" y="4" x="6"></rect>
<path d="M6 20h4"></path>
<path d="M14 10h4"></path>
<path d="M6 14h2v6"></path>
<path d="M14 4h2v6"></path>"#;

const BIOHAZARD: &str = r#"
<circle cx="12" cy="11.9" r="2"></circle>
<path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6"></path>
<path d="m8.9 10.1 1.4.8"></path>
<path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5"></path>
<path d="m15.1 10.1-1.4.8"></path>
<path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2"></path>
<path d="M12 13.9v1.6"></path>
<path d="M13.5 5.4c-1-.2-2-.2-3 0"></path>
<path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5"></path>
<path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5"></path>"#;

const BIRD: &str = r#"
<path d="M16 7h.01"></path>
<path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20"></path>
<path d="m20 7 2 .5-2 .5"></path>
<path d="M10 18v3"></path>
<path d="M14 17.75V21"></path>
<path d="M7 18a6 6 0 0 0 3.84-10.61"></path>"#;

const BITCOIN: &str = r#"
<path d="M11.767 19.089c4.924.868 6.14-6.025 1.216-6.894m-1.216 6.894L5.86 18.047m5.908 1.042-.347 1.97m1.563-8.864c4.924.869 6.14-6.025 1.215-6.893m-1.215 6.893-3.94-.694m5.155-6.2L8.29 4.26m5.908 1.042.348-1.97M7.48 20.364l3.126-17.727"></path>"#;

const BLINDS: &str = r#"
<path d="M3 3h18"></path>
<path d="M20 7H8"></path>
<path d="M20 11H8"></path>
<path d="M10 19h10"></path>
<path d="M8 15h12"></path>
<path d="M4 3v14"></path>
<circle r="2" cy="19" cx="4"></circle>"#;

const BLOCKS: &str = r#"
<rect y="3" rx="1" height="7" x="14" width="7"></rect>
<path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3"></path>"#;

const BLUETOOTH_CONNECTED: &str = r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>
<line x1="18" y2="12" x2="21" y1="12"></line>
<line x2="6" y1="12" x1="3" y2="12"></line>"#;

const BLUETOOTH_OFF: &str = r#"
<path d="m17 17-5 5V12l-5 5"></path>
<path d="m2 2 20 20"></path>
<path d="M14.5 9.5 17 7l-5-5v4.5"></path>"#;

const BLUETOOTH_SEARCHING: &str = r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>
<path d="M20.83 14.83a4 4 0 0 0 0-5.66"></path>
<path d="M18 12h.01"></path>"#;

const BLUETOOTH: &str = r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>"#;

const BOLD: &str = r#"
<path d="M14 12a4 4 0 0 0 0-8H6v8"></path>
<path d="M15 20a4 4 0 0 0 0-8H6v8Z"></path>"#;

const BOMB: &str = r#"
<circle r="9" cx="11" cy="13"></circle>
<path d="m19.5 9.5 1.8-1.8a2.4 2.4 0 0 0 0-3.4l-1.6-1.6a2.41 2.41 0 0 0-3.4 0l-1.8 1.8"></path>
<path d="m22 2-1.5 1.5"></path>"#;

const BONE: &str = r#"
<path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z"></path>"#;

const BOOK_COPY: &str = r#"
<path d="M2 16V4a2 2 0 0 1 2-2h11"></path>
<path d="M5 14H4a2 2 0 1 0 0 4h1"></path>
<path d="M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12"></path>"#;

const BOOK_DOWN: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3 3 3-3"></path>"#;

const BOOK_KEY: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14"></path>
<path d="M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<circle r="2" cy="8" cx="14"></circle>
<path d="m20 2-4.5 4.5"></path>
<path d="m19 3 1 1"></path>"#;

const BOOK_LOCK: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10"></path>
<path d="M20 15v7H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<rect rx="1" y="6" height="5" x="12" width="8"></rect>
<path d="M18 6V4a2 2 0 1 0-4 0v2"></path>"#;

const BOOK_MARKED: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<polyline points="10 2 10 10 13 7 16 10 16 2"></polyline>"#;

const BOOK_MINUS: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M9 10h6"></path>"#;

const BOOK_OPEN_CHECK: &str = r#"
<path d="M8 3H2v15h7c1.7 0 3 1.3 3 3V7c0-2.2-1.8-4-4-4Z"></path>
<path d="m16 12 2 2 4-4"></path>
<path d="M22 6V3h-6c-2.2 0-4 1.8-4 4v14c0-1.7 1.3-3 3-3h7v-2.3"></path>"#;

const BOOK_OPEN: &str = r#"
<path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path>
<path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path>"#;

const BOOK_PLUS: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M9 10h6"></path>
<path d="M12 7v6"></path>"#;

const BOOK_TEMPLATE: &str = r#"
<path d="M20 22h-2"></path>
<path d="M20 15v2h-2"></path>
<path d="M4 19.5V15"></path>
<path d="M20 8v3"></path>
<path d="M18 2h2v2"></path>
<path d="M4 11V9"></path>
<path d="M12 2h2"></path>
<path d="M12 22h2"></path>
<path d="M12 17h2"></path>
<path d="M8 22H6.5a2.5 2.5 0 0 1 0-5H8"></path>
<path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8"></path>"#;

const BOOK_UP_2: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2"></path>
<path d="M18 2h2v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3-3 3 3"></path>
<path d="m9 5 3-3 3 3"></path>"#;

const BOOK_UP: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3-3 3 3"></path>"#;

const BOOK_X: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="m14.5 7-5 5"></path>
<path d="m9.5 7 5 5"></path>"#;

const BOOK: &str = r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>"#;

const BOOKMARK_MINUS: &str = r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>
<line y1="10" y2="10" x1="15" x2="9"></line>"#;

const BOOKMARK_PLUS: &str = r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>
<line x1="12" y2="13" y1="7" x2="12"></line>
<line x2="9" y2="10" x1="15" y1="10"></line>"#;

const BOOKMARK: &str = r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>"#;

const BOOM_BOX: &str = r#"
<path d="M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"></path>
<path d="M8 8v1"></path>
<path d="M12 8v1"></path>
<path d="M16 8v1"></path>
<rect y="9" width="20" height="12" x="2" rx="2"></rect>
<circle cy="15" r="2" cx="8"></circle>
<circle cy="15" r="2" cx="16"></circle>"#;

const BOT: &str = r#"
<path d="M12 8V4H8"></path>
<rect width="16" y="8" rx="2" height="12" x="4"></rect>
<path d="M2 14h2"></path>
<path d="M20 14h2"></path>
<path d="M15 13v2"></path>
<path d="M9 13v2"></path>"#;

const BOX_SELECT: &str = r#"
<path d="M5 3a2 2 0 0 0-2 2"></path>
<path d="M19 3a2 2 0 0 1 2 2"></path>
<path d="M21 19a2 2 0 0 1-2 2"></path>
<path d="M5 21a2 2 0 0 1-2-2"></path>
<path d="M9 3h1"></path>
<path d="M9 21h1"></path>
<path d="M14 3h1"></path>
<path d="M14 21h1"></path>
<path d="M3 9v1"></path>
<path d="M21 9v1"></path>
<path d="M3 14v1"></path>
<path d="M21 14v1"></path>"#;

const BOX: &str = r#"
<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"></path>
<path d="m3.3 7 8.7 5 8.7-5"></path>
<path d="M12 22V12"></path>"#;

const BOXES: &str = r#"
<path d="M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z"></path>
<path d="m7 16.5-4.74-2.85"></path>
<path d="m7 16.5 5-3"></path>
<path d="M7 16.5v5.17"></path>
<path d="M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z"></path>
<path d="m17 16.5-5-3"></path>
<path d="m17 16.5 4.74-2.85"></path>
<path d="M17 16.5v5.17"></path>
<path d="M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z"></path>
<path d="M12 8 7.26 5.15"></path>
<path d="m12 8 4.74-2.85"></path>
<path d="M12 13.5V8"></path>"#;

const BRACES: &str = r#"
<path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1"></path>
<path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1"></path>"#;

const BRACKETS: &str = r#"
<path d="M16 3h3v18h-3"></path>
<path d="M8 21H5V3h3"></path>"#;

const BRAIN_CIRCUIT: &str = r#"
<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08 2.5 2.5 0 0 0 4.91.05L12 20V4.5Z"></path>
<path d="M16 8V5c0-1.1.9-2 2-2"></path>
<path d="M12 13h4"></path>
<path d="M12 18h6a2 2 0 0 1 2 2v1"></path>
<path d="M12 8h8"></path>
<path d="M20.5 8a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M16.5 13a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M20.5 21a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M18.5 3a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>"#;

const BRAIN_COG: &str = r#"
<circle r="3" cx="12" cy="12"></circle>
<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08A2.5 2.5 0 0 0 12 19.5a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 12 4.5"></path>
<path d="m15.7 10.4-.9.4"></path>
<path d="m9.2 13.2-.9.4"></path>
<path d="m13.6 15.7-.4-.9"></path>
<path d="m10.8 9.2-.4-.9"></path>
<path d="m15.7 13.5-.9-.4"></path>
<path d="m9.2 10.9-.9-.4"></path>
<path d="m10.5 15.7.4-.9"></path>
<path d="m13.1 9.2.4-.9"></path>"#;

const BRAIN: &str = r#"
<path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z"></path>
<path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z"></path>"#;

const BRIEFCASE: &str = r#"
<rect width="20" height="14" x="2" y="7" rx="2" ry="2"></rect>
<path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path>"#;

const BRING_TO_FRONT: &str = r#"
<rect rx="2" x="8" width="8" height="8" y="8"></rect>
<path d="M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2"></path>
<path d="M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2"></path>"#;

const BRUSH: &str = r#"
<path d="m9.06 11.9 8.07-8.06a2.85 2.85 0 1 1 4.03 4.03l-8.06 8.08"></path>
<path d="M7.07 14.94c-1.66 0-3 1.35-3 3.02 0 1.33-2.5 1.52-2 2.02 1.08 1.1 2.49 2.02 4 2.02 2.2 0 4-1.8 4-4.04a3.01 3.01 0 0 0-3-3.02z"></path>"#;

const BUG_OFF: &str = r#"
<path d="M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2"></path>
<path d="M14.12 3.88 16 2"></path>
<path d="M22 13h-4v-2a4 4 0 0 0-4-4h-1.3"></path>
<path d="M20.97 5c0 2.1-1.6 3.8-3.5 4"></path>
<path d="m2 2 20 20"></path>
<path d="M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13"></path>
<path d="M12 20v-8"></path>
<path d="M6 13H2"></path>
<path d="M3 21c0-2.1 1.7-3.9 3.8-4"></path>"#;

const BUG_PLAY: &str = r#"
<path d="m8 2 1.88 1.88"></path>
<path d="M14.12 3.88 16 2"></path>
<path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1"></path>
<path d="M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5"></path>
<path d="M6.53 9C4.6 8.8 3 7.1 3 5"></path>
<path d="M6 13H2"></path>
<path d="M3 21c0-2.1 1.7-3.9 3.8-4"></path>
<path d="M20.97 5c0 2.1-1.6 3.8-3.5 4"></path>
<path d="m12 12 8 5-8 5Z"></path>"#;

const BUG: &str = r#"
<path d="m8 2 1.88 1.88"></path>
<path d="M14.12 3.88 16 2"></path>
<path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1"></path>
<path d="M12 20c-3.3 0-6-2.7-6-6v-3a4 4 0 0 1 4-4h4a4 4 0 0 1 4 4v3c0 3.3-2.7 6-6 6"></path>
<path d="M12 20v-9"></path>
<path d="M6.53 9C4.6 8.8 3 7.1 3 5"></path>
<path d="M6 13H2"></path>
<path d="M3 21c0-2.1 1.7-3.9 3.8-4"></path>
<path d="M20.97 5c0 2.1-1.6 3.8-3.5 4"></path>
<path d="M22 13h-4"></path>
<path d="M17.2 17c2.1.1 3.8 1.9 3.8 4"></path>"#;

const BUILDING_2: &str = r#"
<path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"></path>
<path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"></path>
<path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"></path>
<path d="M10 6h4"></path>
<path d="M10 10h4"></path>
<path d="M10 14h4"></path>
<path d="M10 18h4"></path>"#;

const BUILDING: &str = r#"
<rect x="4" y="2" ry="2" rx="2" width="16" height="20"></rect>
<path d="M9 22v-4h6v4"></path>
<path d="M8 6h.01"></path>
<path d="M16 6h.01"></path>
<path d="M12 6h.01"></path>
<path d="M12 10h.01"></path>
<path d="M12 14h.01"></path>
<path d="M16 10h.01"></path>
<path d="M16 14h.01"></path>
<path d="M8 10h.01"></path>
<path d="M8 14h.01"></path>"#;

const BUS_FRONT: &str = r#"
<path d="M4 6 2 7"></path>
<path d="M10 6h4"></path>
<path d="m22 7-2-1"></path>
<rect width="16" height="16" x="4" y="3" rx="2"></rect>
<path d="M4 11h16"></path>
<path d="M8 15h.01"></path>
<path d="M16 15h.01"></path>
<path d="M6 19v2"></path>
<path d="M18 21v-2"></path>"#;

const BUS: &str = r#"
<path d="M8 6v6"></path>
<path d="M15 6v6"></path>
<path d="M2 12h19.6"></path>
<path d="M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3"></path>
<circle cx="7" r="2" cy="18"></circle>
<path d="M9 18h5"></path>
<circle cx="16" r="2" cy="18"></circle>"#;

const CABLE_CAR: &str = r#"
<path d="M10 3h.01"></path>
<path d="M14 2h.01"></path>
<path d="m2 9 20-5"></path>
<path d="M12 12V6.5"></path>
<rect x="4" width="16" y="12" height="10" rx="3"></rect>
<path d="M9 12v5"></path>
<path d="M15 12v5"></path>
<path d="M4 17h16"></path>"#;

const CABLE: &str = r#"
<path d="M4 9a2 2 0 0 1-2-2V5h6v2a2 2 0 0 1-2 2Z"></path>
<path d="M3 5V3"></path>
<path d="M7 5V3"></path>
<path d="M19 15V6.5a3.5 3.5 0 0 0-7 0v11a3.5 3.5 0 0 1-7 0V9"></path>
<path d="M17 21v-2"></path>
<path d="M21 21v-2"></path>
<path d="M22 19h-6v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2Z"></path>"#;

const CAKE_SLICE: &str = r#"
<circle cy="7" cx="9" r="2"></circle>
<path d="M7.2 7.9 3 11v9c0 .6.4 1 1 1h16c.6 0 1-.4 1-1v-9c0-2-3-6-7-8l-3.6 2.6"></path>
<path d="M16 13H3"></path>
<path d="M16 17H3"></path>"#;

const CAKE: &str = r#"
<path d="M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8"></path>
<path d="M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1"></path>
<path d="M2 21h20"></path>
<path d="M7 8v2"></path>
<path d="M12 8v2"></path>
<path d="M17 8v2"></path>
<path d="M7 4h.01"></path>
<path d="M12 4h.01"></path>
<path d="M17 4h.01"></path>"#;

const CALCULATOR: &str = r#"
<rect width="16" y="2" rx="2" height="20" x="4"></rect>
<line x1="8" y1="6" x2="16" y2="6"></line>
<line y2="18" x2="16" y1="14" x1="16"></line>
<path d="M16 10h.01"></path>
<path d="M12 10h.01"></path>
<path d="M8 10h.01"></path>
<path d="M12 14h.01"></path>
<path d="M8 14h.01"></path>
<path d="M12 18h.01"></path>
<path d="M8 18h.01"></path>"#;

const CALENDAR_CHECK_2: &str = r#"
<path d="M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x2="16" x1="16" y1="2" y2="6"></line>
<line x1="8" y1="2" y2="6" x2="8"></line>
<line x1="3" y2="10" y1="10" x2="21"></line>
<path d="m16 20 2 2 4-4"></path>"#;

const CALENDAR_CHECK: &str = r#"
<rect width="18" y="4" rx="2" height="18" x="3" ry="2"></rect>
<line x2="16" y2="6" x1="16" y1="2"></line>
<line x1="8" y1="2" x2="8" y2="6"></line>
<line x2="21" y1="10" x1="3" y2="10"></line>
<path d="m9 16 2 2 4-4"></path>"#;

const CALENDAR_CLOCK: &str = r#"
<path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h5"></path>
<path d="M17.5 17.5 16 16.25V14"></path>
<path d="M22 16a6 6 0 1 1-12 0 6 6 0 0 1 12 0Z"></path>"#;

const CALENDAR_DAYS: &str = r#"
<rect x="3" ry="2" y="4" rx="2" width="18" height="18"></rect>
<line x1="16" y1="2" x2="16" y2="6"></line>
<line y2="6" x1="8" y1="2" x2="8"></line>
<line x2="21" y2="10" y1="10" x1="3"></line>
<path d="M8 14h.01"></path>
<path d="M12 14h.01"></path>
<path d="M16 14h.01"></path>
<path d="M8 18h.01"></path>
<path d="M12 18h.01"></path>
<path d="M16 18h.01"></path>"#;

const CALENDAR_HEART: &str = r#"
<path d="M21 10V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h18"></path>
<path d="M21.29 14.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 22l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#;

const CALENDAR_MINUS: &str = r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x2="16" y2="6" y1="2" x1="16"></line>
<line y2="6" x2="8" x1="8" y1="2"></line>
<line x2="21" x1="3" y2="10" y1="10"></line>
<line y1="19" y2="19" x2="22" x1="16"></line>"#;

const CALENDAR_OFF: &str = r#"
<path d="M4.18 4.18A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18"></path>
<path d="M21 15.5V6a2 2 0 0 0-2-2H9.5"></path>
<path d="M16 2v4"></path>
<path d="M3 10h7"></path>
<path d="M21 10h-5.5"></path>
<line x1="2" x2="22" y1="2" y2="22"></line>"#;

const CALENDAR_PLUS: &str = r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line y2="6" x2="16" x1="16" y1="2"></line>
<line x1="8" y1="2" x2="8" y2="6"></line>
<line x2="21" y2="10" x1="3" y1="10"></line>
<line x2="19" y1="16" x1="19" y2="22"></line>
<line x1="16" x2="22" y2="19" y1="19"></line>"#;

const CALENDAR_RANGE: &str = r#"
<rect rx="2" y="4" width="18" height="18" x="3" ry="2"></rect>
<line x1="16" x2="16" y1="2" y2="6"></line>
<line y1="2" x2="8" y2="6" x1="8"></line>
<line x2="21" y2="10" x1="3" y1="10"></line>
<path d="M17 14h-6"></path>
<path d="M13 18H7"></path>
<path d="M7 14h.01"></path>
<path d="M17 18h.01"></path>"#;

const CALENDAR_SEARCH: &str = r#"
<path d="M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7.5"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h18"></path>
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z"></path>
<path d="m22 22-1.5-1.5"></path>"#;

const CALENDAR_X_2: &str = r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x1="16" y1="2" x2="16" y2="6"></line>
<line x2="8" y2="6" x1="8" y1="2"></line>
<line x1="3" x2="21" y1="10" y2="10"></line>
<line x1="17" y1="17" x2="22" y2="22"></line>
<line x1="17" y1="22" x2="22" y2="17"></line>"#;

const CALENDAR_X: &str = r#"
<rect width="18" rx="2" y="4" ry="2" height="18" x="3"></rect>
<line x1="16" y1="2" y2="6" x2="16"></line>
<line x1="8" x2="8" y2="6" y1="2"></line>
<line y1="10" y2="10" x1="3" x2="21"></line>
<line x1="10" y1="14" x2="14" y2="18"></line>
<line y2="18" x2="10" y1="14" x1="14"></line>"#;

const CALENDAR: &str = r#"
<rect y="4" width="18" x="3" rx="2" ry="2" height="18"></rect>
<line y1="2" x2="16" x1="16" y2="6"></line>
<line y2="6" y1="2" x2="8" x1="8"></line>
<line x2="21" x1="3" y2="10" y1="10"></line>"#;

const CAMERA_OFF: &str = r#"
<line x1="2" y1="2" y2="22" x2="22"></line>
<path d="M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16"></path>
<path d="M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5"></path>
<path d="M14.121 15.121A3 3 0 1 1 9.88 10.88"></path>"#;

const CAMERA: &str = r#"
<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"></path>
<circle cx="12" cy="13" r="3"></circle>"#;

const CANDLESTICK_CHART: &str = r#"
<path d="M9 5v4"></path>
<rect height="6" x="7" width="4" rx="1" y="9"></rect>
<path d="M9 15v2"></path>
<path d="M17 3v2"></path>
<rect y="5" rx="1" height="8" x="15" width="4"></rect>
<path d="M17 13v3"></path>
<path d="M3 3v18h18"></path>"#;

const CANDY_CANE: &str = r#"
<path d="M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2Z"></path>
<path d="M17.75 7 15 2.1"></path>
<path d="M10.9 4.8 13 9"></path>
<path d="m7.9 9.7 2 4.4"></path>
<path d="M4.9 14.7 7 18.9"></path>"#;

const CANDY_OFF: &str = r#"
<path d="m8.5 8.5-1 1a4.95 4.95 0 0 0 7 7l1-1"></path>
<path d="M11.843 6.187A4.947 4.947 0 0 1 16.5 7.5a4.947 4.947 0 0 1 1.313 4.657"></path>
<path d="M14 16.5V14"></path>
<path d="M14 6.5v1.843"></path>
<path d="M10 10v7.5"></path>
<path d="m16 7 1-5 1.367.683A3 3 0 0 0 19.708 3H21v1.292a3 3 0 0 0 .317 1.341L22 7l-5 1"></path>
<path d="m8 17-1 5-1.367-.683A3 3 0 0 0 4.292 21H3v-1.292a3 3 0 0 0-.317-1.341L2 17l5-1"></path>
<line y2="22" x2="22" x1="2" y1="2"></line>"#;

const CANDY: &str = r#"
<path d="m9.5 7.5-2 2a4.95 4.95 0 1 0 7 7l2-2a4.95 4.95 0 1 0-7-7Z"></path>
<path d="M14 6.5v10"></path>
<path d="M10 7.5v10"></path>
<path d="m16 7 1-5 1.37.68A3 3 0 0 0 19.7 3H21v1.3c0 .46.1.92.32 1.33L22 7l-5 1"></path>
<path d="m8 17-1 5-1.37-.68A3 3 0 0 0 4.3 21H3v-1.3a3 3 0 0 0-.32-1.33L2 17l5-1"></path>"#;

const CAR_FRONT: &str = r#"
<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"></path>
<path d="M7 14h.01"></path>
<path d="M17 14h.01"></path>
<rect height="8" x="3" rx="2" y="10" width="18"></rect>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#;

const CAR_TAXI_FRONT: &str = r#"
<path d="M10 2h4"></path>
<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"></path>
<path d="M7 14h.01"></path>
<path d="M17 14h.01"></path>
<rect width="18" x="3" rx="2" y="10" height="8"></rect>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#;

const CAR: &str = r#"
<path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"></path>
<circle cx="7" r="2" cy="17"></circle>
<path d="M9 17h6"></path>
<circle cx="17" cy="17" r="2"></circle>"#;

const CARROT: &str = r#"
<path d="M2.27 21.7s9.87-3.5 12.73-6.36a4.5 4.5 0 0 0-6.36-6.37C5.77 11.84 2.27 21.7 2.27 21.7zM8.64 14l-2.05-2.04M15.34 15l-2.46-2.46"></path>
<path d="M22 9s-1.33-2-3.5-2C16.86 7 15 9 15 9s1.33 2 3.5 2S22 9 22 9z"></path>
<path d="M15 2s-2 1.33-2 3.5S15 9 15 9s2-1.84 2-3.5C17 3.33 15 2 15 2z"></path>"#;

const CASE_LOWER: &str = r#"
<circle r="3" cx="7" cy="12"></circle>
<path d="M10 9v6"></path>
<circle r="3" cy="12" cx="17"></circle>
<path d="M14 7v8"></path>"#;

const CASE_SENSITIVE: &str = r#"
<path d="m3 15 4-8 4 8"></path>
<path d="M4 13h6"></path>
<circle r="3" cx="18" cy="12"></circle>
<path d="M21 9v6"></path>"#;

const CASE_UPPER: &str = r#"
<path d="m3 15 4-8 4 8"></path>
<path d="M4 13h6"></path>
<path d="M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4"></path>"#;

const CASSETTE_TAPE: &str = r#"
<rect height="16" rx="2" width="20" x="2" y="4"></rect>
<circle cy="10" cx="8" r="2"></circle>
<path d="M8 12h8"></path>
<circle r="2" cy="10" cx="16"></circle>
<path d="m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3"></path>"#;

const CAST: &str = r#"
<path d="M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6"></path>
<path d="M2 12a9 9 0 0 1 8 8"></path>
<path d="M2 16a5 5 0 0 1 4 4"></path>
<line x2="2.01" y1="20" x1="2" y2="20"></line>"#;

const CASTLE: &str = r#"
<path d="M22 20v-9H2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z"></path>
<path d="M18 11V4H6v7"></path>
<path d="M15 22v-4a3 3 0 0 0-3-3v0a3 3 0 0 0-3 3v4"></path>
<path d="M22 11V9"></path>
<path d="M2 11V9"></path>
<path d="M6 4V2"></path>
<path d="M18 4V2"></path>
<path d="M10 4V2"></path>
<path d="M14 4V2"></path>"#;

const CAT: &str = r#"
<path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z"></path>
<path d="M8 14v.5"></path>
<path d="M16 14v.5"></path>
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z"></path>"#;

const CHECK_CHECK: &str = r#"
<path d="M18 6 7 17l-5-5"></path>
<path d="m22 10-7.5 7.5L13 16"></path>"#;

const CHECK_CIRCLE_2: &str = r#"
<path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"></path>
<path d="m9 12 2 2 4-4"></path>"#;

const CHECK_CIRCLE: &str = r#"
<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
<polyline points="22 4 12 14.01 9 11.01"></polyline>"#;

const CHECK_SQUARE: &str = r#"
<polyline points="9 11 12 14 22 4"></polyline>
<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>"#;

const CHECK: &str = r#"
<polyline points="20 6 9 17 4 12"></polyline>"#;

const CHEF_HAT: &str = r#"
<path d="M6 13.87A4 4 0 0 1 7.41 6a5.11 5.11 0 0 1 1.05-1.54 5 5 0 0 1 7.08 0A5.11 5.11 0 0 1 16.59 6 4 4 0 0 1 18 13.87V21H6Z"></path>
<line y2="17" y1="17" x1="6" x2="18"></line>"#;

const CHERRY: &str = r#"
<path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"></path>
<path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"></path>
<path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12"></path>
<path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z"></path>"#;

const CHEVRON_DOWN_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m16 10-4 4-4-4"></path>"#;

const CHEVRON_DOWN_SQUARE: &str = r#"
<rect y="3" x="3" width="18" height="18" rx="2"></rect>
<path d="m16 10-4 4-4-4"></path>"#;

const CHEVRON_DOWN: &str = r#"
<path d="m6 9 6 6 6-6"></path>"#;

const CHEVRON_FIRST: &str = r#"
<path d="m17 18-6-6 6-6"></path>
<path d="M7 6v12"></path>"#;

const CHEVRON_LAST: &str = r#"
<path d="m7 18 6-6-6-6"></path>
<path d="M17 6v12"></path>"#;

const CHEVRON_LEFT_CIRCLE: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<path d="m14 16-4-4 4-4"></path>"#;

const CHEVRON_LEFT_SQUARE: &str = r#"
<rect x="3" height="18" width="18" y="3" rx="2"></rect>
<path d="m14 16-4-4 4-4"></path>"#;

const CHEVRON_LEFT: &str = r#"
<path d="m15 18-6-6 6-6"></path>"#;

const CHEVRON_RIGHT_CIRCLE: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="m10 8 4 4-4 4"></path>"#;

const CHEVRON_RIGHT_SQUARE: &str = r#"
<rect x="3" height="18" rx="2" y="3" width="18"></rect>
<path d="m10 8 4 4-4 4"></path>"#;

const CHEVRON_RIGHT: &str = r#"
<path d="m9 18 6-6-6-6"></path>"#;

const CHEVRON_UP_CIRCLE: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="m8 14 4-4 4 4"></path>"#;

const CHEVRON_UP_SQUARE: &str = r#"
<rect y="3" x="3" rx="2" height="18" width="18"></rect>
<path d="m8 14 4-4 4 4"></path>"#;

const CHEVRON_UP: &str = r#"
<path d="m18 15-6-6-6 6"></path>"#;

const CHEVRONS_DOWN_UP: &str = r#"
<path d="m7 20 5-5 5 5"></path>
<path d="m7 4 5 5 5-5"></path>"#;

const CHEVRONS_DOWN: &str = r#"
<path d="m7 6 5 5 5-5"></path>
<path d="m7 13 5 5 5-5"></path>"#;

const CHEVRONS_LEFT_RIGHT: &str = r#"
<path d="m9 7-5 5 5 5"></path>
<path d="m15 7 5 5-5 5"></path>"#;

const CHEVRONS_LEFT: &str = r#"
<path d="m11 17-5-5 5-5"></path>
<path d="m18 17-5-5 5-5"></path>"#;

const CHEVRONS_RIGHT_LEFT: &str = r#"
<path d="m20 17-5-5 5-5"></path>
<path d="m4 17 5-5-5-5"></path>"#;

const CHEVRONS_RIGHT: &str = r#"
<path d="m6 17 5-5-5-5"></path>
<path d="m13 17 5-5-5-5"></path>"#;

const CHEVRONS_UP_DOWN: &str = r#"
<path d="m7 15 5 5 5-5"></path>
<path d="m7 9 5-5 5 5"></path>"#;

const CHEVRONS_UP: &str = r#"
<path d="m17 11-5-5-5 5"></path>
<path d="m17 18-5-5-5 5"></path>"#;

const CHROME: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<circle cx="12" r="4" cy="12"></circle>
<line x1="21.17" y1="8" x2="12" y2="8"></line>
<line y1="6.06" y2="14" x2="8.54" x1="3.95"></line>
<line y2="14" x2="15.46" x1="10.88" y1="21.94"></line>"#;

const CHURCH: &str = r#"
<path d="m18 7 4 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9l4-2"></path>
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4"></path>
<path d="M18 22V5l-6-3-6 3v17"></path>
<path d="M12 7v5"></path>
<path d="M10 9h4"></path>"#;

const CIGARETTE_OFF: &str = r#"
<line y1="2" x2="22" x1="2" y2="22"></line>
<path d="M12 12H2v4h14"></path>
<path d="M22 12v4"></path>
<path d="M18 12h-.5"></path>
<path d="M7 12v4"></path>
<path d="M18 8c0-2.5-2-2.5-2-5"></path>
<path d="M22 8c0-2.5-2-2.5-2-5"></path>"#;

const CIGARETTE: &str = r#"
<path d="M18 12H2v4h16"></path>
<path d="M22 12v4"></path>
<path d="M7 12v4"></path>
<path d="M18 8c0-2.5-2-2.5-2-5"></path>
<path d="M22 8c0-2.5-2-2.5-2-5"></path>"#;

const CIRCLE_DASHED: &str = r#"
<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"></path>
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"></path>
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"></path>
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"></path>
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"></path>
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"></path>
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"></path>
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"></path>"#;

const CIRCLE_DOLLAR_SIGN: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 18V6"></path>"#;

const CIRCLE_DOT_DASHED: &str = r#"
<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"></path>
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"></path>
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"></path>
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"></path>
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"></path>
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"></path>
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"></path>
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"></path>
<circle cy="12" r="1" cx="12"></circle>"#;

const CIRCLE_DOT: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<circle r="1" cx="12" cy="12"></circle>"#;

const CIRCLE_ELLIPSIS: &str = r#"
<circle r="10" cy="12" cx="12"></circle>
<path d="M17 12h.01"></path>
<path d="M12 12h.01"></path>
<path d="M7 12h.01"></path>"#;

const CIRCLE_EQUAL: &str = r#"
<path d="M7 10h10"></path>
<path d="M7 14h10"></path>
<circle r="10" cx="12" cy="12"></circle>"#;

const CIRCLE_OFF: &str = r#"
<path d="m2 2 20 20"></path>
<path d="M8.35 2.69A10 10 0 0 1 21.3 15.65"></path>
<path d="M19.08 19.08A10 10 0 1 1 4.92 4.92"></path>"#;

const CIRCLE_SLASH_2: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M22 2 2 22"></path>"#;

const CIRCLE_SLASH: &str = r#"
<line x1="9" x2="15" y1="15" y2="9"></line>
<circle cy="12" cx="12" r="10"></circle>"#;

const CIRCLE: &str = r#"
<circle cy="12" r="10" cx="12"></circle>"#;

const CIRCUIT_BOARD: &str = r#"
<rect width="18" height="18" rx="2" x="3" y="3"></rect>
<path d="M11 9h4a2 2 0 0 0 2-2V3"></path>
<circle r="2" cy="9" cx="9"></circle>
<path d="M7 21v-4a2 2 0 0 1 2-2h4"></path>
<circle cy="15" cx="15" r="2"></circle>"#;

const CITRUS: &str = r#"
<path d="M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z"></path>
<path d="M19.65 15.66A8 8 0 0 1 8.35 4.34"></path>
<path d="m14 10-5.5 5.5"></path>
<path d="M14 17.85V10H6.15"></path>"#;

const CLAPPERBOARD: &str = r#"
<path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3Z"></path>
<path d="m6.2 5.3 3.1 3.9"></path>
<path d="m12.4 3.4 3.1 4"></path>
<path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z"></path>"#;

const CLIPBOARD_CHECK: &str = r#"
<rect height="4" ry="1" x="8" rx="1" y="2" width="8"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="m9 14 2 2 4-4"></path>"#;

const CLIPBOARD_COPY: &str = r#"
<rect width="8" height="4" x="8" y="2" rx="1" ry="1"></rect>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"></path>
<path d="M16 4h2a2 2 0 0 1 2 2v4"></path>
<path d="M21 14H11"></path>
<path d="m15 10-4 4 4 4"></path>"#;

const CLIPBOARD_EDIT: &str = r#"
<rect ry="1" width="8" y="2" x="8" rx="1" height="4"></rect>
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z"></path>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-5.5"></path>
<path d="M4 13.5V6a2 2 0 0 1 2-2h2"></path>"#;

const CLIPBOARD_LIST: &str = r#"
<rect rx="1" height="4" ry="1" x="8" y="2" width="8"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="M12 11h4"></path>
<path d="M12 16h4"></path>
<path d="M8 11h.01"></path>
<path d="M8 16h.01"></path>"#;

const CLIPBOARD_PASTE: &str = r#"
<path d="M15 2H9a1 1 0 0 0-1 1v2c0 .6.4 1 1 1h6c.6 0 1-.4 1-1V3c0-.6-.4-1-1-1Z"></path>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M16 4h2a2 2 0 0 1 2 2v2M11 14h10"></path>
<path d="m17 10 4 4-4 4"></path>"#;

const CLIPBOARD_SIGNATURE: &str = r#"
<rect ry="1" y="2" x="8" width="8" height="4" rx="1"></rect>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5"></path>
<path d="M16 4h2a2 2 0 0 1 1.73 1"></path>
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z"></path>
<path d="M8 18h1"></path>"#;

const CLIPBOARD_TYPE: &str = r#"
<rect rx="1" x="8" height="4" ry="1" width="8" y="2"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="M9 12v-1h6v1"></path>
<path d="M11 17h2"></path>
<path d="M12 11v6"></path>"#;

const CLIPBOARD_X: &str = r#"
<rect y="2" rx="1" ry="1" width="8" x="8" height="4"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="m15 11-6 6"></path>
<path d="m9 11 6 6"></path>"#;

const CLIPBOARD: &str = r#"
<rect rx="1" ry="1" width="8" x="8" y="2" height="4"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>"#;

const CLOCK_1: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 14.5 8"></polyline>"#;

const CLOCK_10: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 8 10"></polyline>"#;

const CLOCK_11: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<polyline points="12 6 12 12 9.5 8"></polyline>"#;

const CLOCK_12: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12"></polyline>"#;

const CLOCK_2: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 16 10"></polyline>"#;

const CLOCK_3: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<polyline points="12 6 12 12 16.5 12"></polyline>"#;

const CLOCK_4: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 16 14"></polyline>"#;

const CLOCK_5: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<polyline points="12 6 12 12 14.5 16"></polyline>"#;

const CLOCK_6: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<polyline points="12 6 12 12 12 16.5"></polyline>"#;

const CLOCK_7: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<polyline points="12 6 12 12 9.5 16"></polyline>"#;

const CLOCK_8: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<polyline points="12 6 12 12 8 14"></polyline>"#;

const CLOCK_9: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12 7.5 12"></polyline>"#;

const CLOCK: &str = r#"
<circle r="10" cy="12" cx="12"></circle>
<polyline points="12 6 12 12 16 14"></polyline>"#;

const CLOUD_COG: &str = r#"
<circle r="3" cx="12" cy="17"></circle>
<path d="M4.2 15.1A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2"></path>
<path d="m15.7 18.4-.9-.3"></path>
<path d="m9.2 15.9-.9-.3"></path>
<path d="m10.6 20.7.3-.9"></path>
<path d="m13.1 14.2.3-.9"></path>
<path d="m13.6 20.7-.4-1"></path>
<path d="m10.8 14.3-.4-1"></path>
<path d="m8.3 18.6 1-.4"></path>
<path d="m14.7 15.8 1-.4"></path>"#;

const CLOUD_DRIZZLE: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M8 19v1"></path>
<path d="M8 14v1"></path>
<path d="M16 19v1"></path>
<path d="M16 14v1"></path>
<path d="M12 21v1"></path>
<path d="M12 16v1"></path>"#;

const CLOUD_FOG: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 17H7"></path>
<path d="M17 21H9"></path>"#;

const CLOUD_HAIL: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 14v2"></path>
<path d="M8 14v2"></path>
<path d="M16 20h.01"></path>
<path d="M8 20h.01"></path>
<path d="M12 16v2"></path>
<path d="M12 22h.01"></path>"#;

const CLOUD_LIGHTNING: &str = r#"
<path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973"></path>
<path d="m13 12-3 5h4l-3 5"></path>"#;

const CLOUD_MOON_RAIN: &str = r#"
<path d="M10.083 9A6.002 6.002 0 0 1 16 4a4.243 4.243 0 0 0 6 6c0 2.22-1.206 4.16-3 5.197"></path>
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"></path>
<path d="M11 20v2"></path>
<path d="M7 19v2"></path>"#;

const CLOUD_MOON: &str = r#"
<path d="M13 16a3 3 0 1 1 0 6H7a5 5 0 1 1 4.9-6Z"></path>
<path d="M10.1 9A6 6 0 0 1 16 4a4.24 4.24 0 0 0 6 6 6 6 0 0 1-3 5.197"></path>"#;

const CLOUD_OFF: &str = r#"
<path d="m2 2 20 20"></path>
<path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193"></path>
<path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07"></path>"#;

const CLOUD_RAIN_WIND: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="m9.2 22 3-7"></path>
<path d="m9 13-3 7"></path>
<path d="m17 13-3 7"></path>"#;

const CLOUD_RAIN: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 14v6"></path>
<path d="M8 14v6"></path>
<path d="M12 16v6"></path>"#;

const CLOUD_SNOW: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M8 15h.01"></path>
<path d="M8 19h.01"></path>
<path d="M12 17h.01"></path>
<path d="M12 21h.01"></path>
<path d="M16 15h.01"></path>
<path d="M16 19h.01"></path>"#;

const CLOUD_SUN_RAIN: &str = r#"
<path d="M12 2v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="M20 12h2"></path>
<path d="m19.07 4.93-1.41 1.41"></path>
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"></path>
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"></path>
<path d="M11 20v2"></path>
<path d="M7 19v2"></path>"#;

const CLOUD_SUN: &str = r#"
<path d="M12 2v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="M20 12h2"></path>
<path d="m19.07 4.93-1.41 1.41"></path>
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"></path>
<path d="M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z"></path>"#;

const CLOUD: &str = r#"
<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>"#;

const CLOUDY: &str = r#"
<path d="M17.5 21H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>
<path d="M22 10a3 3 0 0 0-3-3h-2.207a5.502 5.502 0 0 0-10.702.5"></path>"#;

const CLOVER: &str = r#"
<path d="M16.2 3.8a2.7 2.7 0 0 0-3.81 0l-.4.38-.4-.4a2.7 2.7 0 0 0-3.82 0C6.73 4.85 6.67 6.64 8 8l4 4 4-4c1.33-1.36 1.27-3.15.2-4.2z"></path>
<path d="M8 8c-1.36-1.33-3.15-1.27-4.2-.2a2.7 2.7 0 0 0 0 3.81l.38.4-.4.4a2.7 2.7 0 0 0 0 3.82C4.85 17.27 6.64 17.33 8 16"></path>
<path d="M16 16c1.36 1.33 3.15 1.27 4.2.2a2.7 2.7 0 0 0 0-3.81l-.38-.4.4-.4a2.7 2.7 0 0 0 0-3.82C19.15 6.73 17.36 6.67 16 8"></path>
<path d="M7.8 20.2a2.7 2.7 0 0 0 3.81 0l.4-.38.4.4a2.7 2.7 0 0 0 3.82 0c1.06-1.06 1.12-2.85-.21-4.21l-4-4-4 4c-1.33 1.36-1.27 3.15-.2 4.2z"></path>
<path d="m7 17-5 5"></path>"#;

const CLUB: &str = r#"
<path d="M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z"></path>
<path d="M12 17.66L12 22"></path>"#;

const CODE_2: &str = r#"
<path d="m18 16 4-4-4-4"></path>
<path d="m6 8-4 4 4 4"></path>
<path d="m14.5 4-5 16"></path>"#;

const CODE: &str = r#"
<polyline points="16 18 22 12 16 6"></polyline>
<polyline points="8 6 2 12 8 18"></polyline>"#;

const CODEPEN: &str = r#"
<polygon points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2"></polygon>
<line x2="12" x1="12" y1="22" y2="15.5"></line>
<polyline points="22 8.5 12 15.5 2 8.5"></polyline>
<polyline points="2 15.5 12 8.5 22 15.5"></polyline>
<line x2="12" y2="8.5" x1="12" y1="2"></line>"#;

const CODESANDBOX: &str = r#"
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
<polyline points="7.5 4.21 12 6.81 16.5 4.21"></polyline>
<polyline points="7.5 19.79 7.5 14.6 3 12"></polyline>
<polyline points="21 12 16.5 14.6 16.5 19.79"></polyline>
<polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
<line x1="12" y1="22.08" y2="12" x2="12"></line>"#;

const COFFEE: &str = r#"
<path d="M17 8h1a4 4 0 1 1 0 8h-1"></path>
<path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z"></path>
<line y1="2" y2="4" x2="6" x1="6"></line>
<line y2="4" x1="10" x2="10" y1="2"></line>
<line y1="2" y2="4" x1="14" x2="14"></line>"#;

const COG: &str = r#"
<path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"></path>
<path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"></path>
<path d="M12 2v2"></path>
<path d="M12 22v-2"></path>
<path d="m17 20.66-1-1.73"></path>
<path d="M11 10.27 7 3.34"></path>
<path d="m20.66 17-1.73-1"></path>
<path d="m3.34 7 1.73 1"></path>
<path d="M14 12h8"></path>
<path d="M2 12h2"></path>
<path d="m20.66 7-1.73 1"></path>
<path d="m3.34 17 1.73-1"></path>
<path d="m17 3.34-1 1.73"></path>
<path d="m11 13.73-4 6.93"></path>"#;

const COINS: &str = r#"
<circle cy="8" r="6" cx="8"></circle>
<path d="M18.09 10.37A6 6 0 1 1 10.34 18"></path>
<path d="M7 6h1v4"></path>
<path d="m16.71 13.88.7.71-2.82 2.82"></path>"#;

const COLUMNS: &str = r#"
<rect width="18" x="3" y="3" rx="2" ry="2" height="18"></rect>
<line x2="12" y2="21" x1="12" y1="3"></line>"#;

const COMBINE: &str = r#"
<rect x="2" rx="2" height="8" width="8" y="2"></rect>
<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M10 18H5c-1.7 0-3-1.3-3-3v-1"></path>
<polyline points="7 21 10 18 7 15"></polyline>
<rect y="14" x="14" width="8" height="8" rx="2"></rect>"#;

const COMMAND: &str = r#"
<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"></path>"#;

const COMPASS: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"></polygon>"#;

const COMPONENT: &str = r#"
<path d="M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z"></path>
<path d="m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z"></path>
<path d="M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z"></path>
<path d="m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z"></path>"#;

const COMPUTER: &str = r#"
<rect width="14" x="5" height="8" y="2" rx="2"></rect>
<rect height="8" width="20" y="14" x="2" rx="2"></rect>
<path d="M6 18h2"></path>
<path d="M12 18h6"></path>"#;

const CONCIERGE_BELL: &str = r#"
<path d="M2 18a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2H2v-2Z"></path>
<path d="M20 16a8 8 0 1 0-16 0"></path>
<path d="M12 4v4"></path>
<path d="M10 4h4"></path>"#;

const CONSTRUCTION: &str = r#"
<rect height="8" rx="1" width="20" x="2" y="6"></rect>
<path d="M17 14v7"></path>
<path d="M7 14v7"></path>
<path d="M17 3v3"></path>
<path d="M7 3v3"></path>
<path d="M10 14 2.3 6.3"></path>
<path d="m14 6 7.7 7.7"></path>
<path d="m8 6 8 8"></path>"#;

const CONTACT_2: &str = r#"
<path d="M16 18a4 4 0 0 0-8 0"></path>
<circle r="3" cx="12" cy="11"></circle>
<rect width="18" x="3" rx="2" height="18" y="4"></rect>
<line x1="8" x2="8" y2="4" y1="2"></line>
<line y2="4" x2="16" x1="16" y1="2"></line>"#;

const CONTACT: &str = r#"
<path d="M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2"></path>
<rect width="18" height="18" rx="2" y="4" x="3"></rect>
<circle r="2" cx="12" cy="10"></circle>
<line y1="2" x1="8" x2="8" y2="4"></line>
<line x2="16" y1="2" y2="4" x1="16"></line>"#;

const CONTAINER: &str = r#"
<path d="M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z"></path>
<path d="M10 21.9V14L2.1 9.1"></path>
<path d="m10 14 11.9-6.9"></path>
<path d="M14 19.8v-8.1"></path>
<path d="M18 17.5V9.4"></path>"#;

const CONTRAST: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<path d="M12 18a6 6 0 0 0 0-12v12z"></path>"#;

const COOKIE: &str = r#"
<path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5"></path>
<path d="M8.5 8.5v.01"></path>
<path d="M16 15.5v.01"></path>
<path d="M12 12v.01"></path>
<path d="M11 17v.01"></path>
<path d="M7 14v.01"></path>"#;

const COPY_CHECK: &str = r#"
<path d="m12 15 2 2 4-4"></path>
<rect y="8" ry="2" height="14" rx="2" x="8" width="14"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPY_MINUS: &str = r#"
<line y2="15" y1="15" x1="12" x2="18"></line>
<rect rx="2" ry="2" x="8" y="8" width="14" height="14"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPY_PLUS: &str = r#"
<line x2="15" y1="12" y2="18" x1="15"></line>
<line x2="18" y2="15" y1="15" x1="12"></line>
<rect y="8" rx="2" ry="2" x="8" width="14" height="14"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPY_SLASH: &str = r#"
<line y1="18" y2="12" x2="18" x1="12"></line>
<rect rx="2" ry="2" x="8" width="14" height="14" y="8"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPY_X: &str = r#"
<line x2="18" y2="18" y1="12" x1="12"></line>
<line x2="18" y2="12" x1="12" y1="18"></line>
<rect height="14" x="8" ry="2" rx="2" width="14" y="8"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPY: &str = r#"
<rect y="8" height="14" rx="2" x="8" width="14" ry="2"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#;

const COPYLEFT: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M9 9.35a4 4 0 1 1 0 5.3"></path>"#;

const COPYRIGHT: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M15 9.354a4 4 0 1 0 0 5.292"></path>"#;

const CORNER_DOWN_LEFT: &str = r#"
<polyline points="9 10 4 15 9 20"></polyline>
<path d="M20 4v7a4 4 0 0 1-4 4H4"></path>"#;

const CORNER_DOWN_RIGHT: &str = r#"
<polyline points="15 10 20 15 15 20"></polyline>
<path d="M4 4v7a4 4 0 0 0 4 4h12"></path>"#;

const CORNER_LEFT_DOWN: &str = r#"
<polyline points="14 15 9 20 4 15"></polyline>
<path d="M20 4h-7a4 4 0 0 0-4 4v12"></path>"#;

const CORNER_LEFT_UP: &str = r#"
<polyline points="14 9 9 4 4 9"></polyline>
<path d="M20 20h-7a4 4 0 0 1-4-4V4"></path>"#;

const CORNER_RIGHT_DOWN: &str = r#"
<polyline points="10 15 15 20 20 15"></polyline>
<path d="M4 4h7a4 4 0 0 1 4 4v12"></path>"#;

const CORNER_RIGHT_UP: &str = r#"
<polyline points="10 9 15 4 20 9"></polyline>
<path d="M4 20h7a4 4 0 0 0 4-4V4"></path>"#;

const CORNER_UP_LEFT: &str = r#"
<polyline points="9 14 4 9 9 4"></polyline>
<path d="M20 20v-7a4 4 0 0 0-4-4H4"></path>"#;

const CORNER_UP_RIGHT: &str = r#"
<polyline points="15 14 20 9 15 4"></polyline>
<path d="M4 20v-7a4 4 0 0 1 4-4h12"></path>"#;

const CPU: &str = r#"
<rect x="4" rx="2" width="16" height="16" y="4"></rect>
<rect width="6" x="9" y="9" height="6"></rect>
<path d="M15 2v2"></path>
<path d="M15 20v2"></path>
<path d="M2 15h2"></path>
<path d="M2 9h2"></path>
<path d="M20 15h2"></path>
<path d="M20 9h2"></path>
<path d="M9 2v2"></path>
<path d="M9 20v2"></path>"#;

const CREATIVE_COMMONS: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"></path>
<path d="M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"></path>"#;

const CREDIT_CARD: &str = r#"
<rect x="2" height="14" rx="2" y="5" width="20"></rect>
<line y2="10" x1="2" x2="22" y1="10"></line>"#;

const CROISSANT: &str = r#"
<path d="m4.6 13.11 5.79-3.21c1.89-1.05 4.79 1.78 3.71 3.71l-3.22 5.81C8.8 23.16.79 15.23 4.6 13.11Z"></path>
<path d="m10.5 9.5-1-2.29C9.2 6.48 8.8 6 8 6H4.5C2.79 6 2 6.5 2 8.5a7.71 7.71 0 0 0 2 4.83"></path>
<path d="M8 6c0-1.55.24-4-2-4-2 0-2.5 2.17-2.5 4"></path>
<path d="m14.5 13.5 2.29 1c.73.3 1.21.7 1.21 1.5v3.5c0 1.71-.5 2.5-2.5 2.5a7.71 7.71 0 0 1-4.83-2"></path>
<path d="M18 16c1.55 0 4-.24 4 2 0 2-2.17 2.5-4 2.5"></path>"#;

const CROP: &str = r#"
<path d="M6 2v14a2 2 0 0 0 2 2h14"></path>
<path d="M18 22V8a2 2 0 0 0-2-2H2"></path>"#;

const CROSS: &str = r#"
<path d="M11 2a2 2 0 0 0-2 2v5H4a2 2 0 0 0-2 2v2c0 1.1.9 2 2 2h5v5c0 1.1.9 2 2 2h2a2 2 0 0 0 2-2v-5h5a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-5V4a2 2 0 0 0-2-2h-2z"></path>"#;

const CROSSHAIR: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<line y1="12" x2="18" y2="12" x1="22"></line>
<line x1="6" y2="12" x2="2" y1="12"></line>
<line x2="12" y1="6" y2="2" x1="12"></line>
<line y2="18" x2="12" x1="12" y1="22"></line>"#;

const CROWN: &str = r#"
<path d="m2 4 3 12h14l3-12-6 7-4-7-4 7-6-7zm3 16h14"></path>"#;

const CUP_SODA: &str = r#"
<path d="m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8"></path>
<path d="M5 8h14"></path>
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0"></path>
<path d="m12 8 1-6h2"></path>"#;

const CURRENCY: &str = r#"
<circle cy="12" cx="12" r="8"></circle>
<line x2="6" y2="6" x1="3" y1="3"></line>
<line x2="18" y2="6" y1="3" x1="21"></line>
<line x2="6" y2="18" x1="3" y1="21"></line>
<line x2="18" x1="21" y1="21" y2="18"></line>"#;

const DATABASE_BACKUP: &str = r#"
<ellipse cy="5" rx="9" cx="12" ry="3"></ellipse>
<path d="M3 12a9 3 0 0 0 5 2.69"></path>
<path d="M21 9.3V5"></path>
<path d="M3 5v14a9 3 0 0 0 6.47 2.88"></path>
<path d="M12 12v4h4"></path>
<path d="M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16"></path>"#;

const DATABASE_ZAP: &str = r#"
<ellipse cy="5" cx="12" ry="3" rx="9"></ellipse>
<path d="M3 5V19A9 3 0 0 0 15 21.84"></path>
<path d="M21 5V8"></path>
<path d="M21 12L18 17H22L19 22"></path>
<path d="M3 12A9 3 0 0 0 14.59 14.87"></path>"#;

const DATABASE: &str = r#"
<ellipse cy="5" ry="3" rx="9" cx="12"></ellipse>
<path d="M3 5V19A9 3 0 0 0 21 19V5"></path>
<path d="M3 12A9 3 0 0 0 21 12"></path>"#;

const DELETE: &str = r#"
<path d="M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z"></path>
<line y2="15" y1="9" x2="12" x1="18"></line>
<line x2="18" x1="12" y1="9" y2="15"></line>"#;

const DESSERT: &str = r#"
<circle cy="4" r="2" cx="12"></circle>
<path d="M10.2 3.2C5.5 4 2 8.1 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4 0c0-4.9-3.5-9-8.2-9.8"></path>
<path d="M3.2 14.8a9 9 0 0 0 17.6 0"></path>"#;

const DIAMOND: &str = r#"
<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z"></path>"#;

const DICE_1: &str = r#"
<rect y="3" rx="2" ry="2" height="18" width="18" x="3"></rect>
<path d="M12 12h.01"></path>"#;

const DICE_2: &str = r#"
<rect y="3" rx="2" width="18" x="3" height="18" ry="2"></rect>
<path d="M15 9h.01"></path>
<path d="M9 15h.01"></path>"#;

const DICE_3: &str = r#"
<rect x="3" rx="2" ry="2" y="3" height="18" width="18"></rect>
<path d="M16 8h.01"></path>
<path d="M12 12h.01"></path>
<path d="M8 16h.01"></path>"#;

const DICE_4: &str = r#"
<rect x="3" width="18" height="18" ry="2" y="3" rx="2"></rect>
<path d="M16 8h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 16h.01"></path>
<path d="M16 16h.01"></path>"#;

const DICE_5: &str = r#"
<rect height="18" ry="2" y="3" x="3" width="18" rx="2"></rect>
<path d="M16 8h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 16h.01"></path>
<path d="M16 16h.01"></path>
<path d="M12 12h.01"></path>"#;

const DICE_6: &str = r#"
<rect width="18" height="18" y="3" rx="2" x="3" ry="2"></rect>
<path d="M16 8h.01"></path>
<path d="M16 12h.01"></path>
<path d="M16 16h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 12h.01"></path>
<path d="M8 16h.01"></path>"#;

const DICES: &str = r#"
<rect height="12" y="10" x="2" ry="2" rx="2" width="12"></rect>
<path d="m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6"></path>
<path d="M6 18h.01"></path>
<path d="M10 14h.01"></path>
<path d="M15 6h.01"></path>
<path d="M18 9h.01"></path>"#;

const DIFF: &str = r#"
<path d="M12 3v14"></path>
<path d="M5 10h14"></path>
<path d="M5 21h14"></path>"#;

const DISC_2: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<circle r="4" cx="12" cy="12"></circle>
<path d="M12 12h.01"></path>"#;

const DISC_3: &str = r#"
<circle r="10" cy="12" cx="12"></circle>
<path d="M6 12c0-1.7.7-3.2 1.8-4.2"></path>
<circle cy="12" r="2" cx="12"></circle>
<path d="M18 12c0 1.7-.7 3.2-1.8 4.2"></path>"#;

const DISC: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<circle r="2" cy="12" cx="12"></circle>"#;

const DIVIDE_CIRCLE: &str = r#"
<line y1="12" x1="8" y2="12" x2="16"></line>
<line x2="12" y1="16" x1="12" y2="16"></line>
<line y2="8" x1="12" x2="12" y1="8"></line>
<circle cx="12" cy="12" r="10"></circle>"#;

const DIVIDE_SQUARE: &str = r#"
<rect height="18" rx="2" ry="2" x="3" width="18" y="3"></rect>
<line y1="12" y2="12" x2="16" x1="8"></line>
<line x1="12" y2="16" x2="12" y1="16"></line>
<line x1="12" x2="12" y1="8" y2="8"></line>"#;

const DIVIDE: &str = r#"
<circle r="1" cx="12" cy="6"></circle>
<line x1="5" y1="12" y2="12" x2="19"></line>
<circle cy="18" r="1" cx="12"></circle>"#;

const DNA_OFF: &str = r#"
<path d="M15 2c-1.35 1.5-2.092 3-2.5 4.5M9 22c1.35-1.5 2.092-3 2.5-4.5"></path>
<path d="M2 15c3.333-3 6.667-3 10-3m10-3c-1.5 1.35-3 2.092-4.5 2.5"></path>
<path d="m17 6-2.5-2.5"></path>
<path d="m14 8-1.5-1.5"></path>
<path d="m7 18 2.5 2.5"></path>
<path d="m3.5 14.5.5.5"></path>
<path d="m20 9 .5.5"></path>
<path d="m6.5 12.5 1 1"></path>
<path d="m16.5 10.5 1 1"></path>
<path d="m10 16 1.5 1.5"></path>
<line y1="2" x2="22" y2="22" x1="2"></line>"#;

const DNA: &str = r#"
<path d="M2 15c6.667-6 13.333 0 20-6"></path>
<path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993"></path>
<path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993"></path>
<path d="m17 6-2.5-2.5"></path>
<path d="m14 8-1-1"></path>
<path d="m7 18 2.5 2.5"></path>
<path d="m3.5 14.5.5.5"></path>
<path d="m20 9 .5.5"></path>
<path d="m6.5 12.5 1 1"></path>
<path d="m16.5 10.5 1 1"></path>
<path d="m10 16 1.5 1.5"></path>"#;

const DOG: &str = r#"
<path d="M10 5.172C10 3.782 8.423 2.679 6.5 3c-2.823.47-4.113 6.006-4 7 .08.703 1.725 1.722 3.656 1 1.261-.472 1.96-1.45 2.344-2.5"></path>
<path d="M14.267 5.172c0-1.39 1.577-2.493 3.5-2.172 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5"></path>
<path d="M8 14v.5"></path>
<path d="M16 14v.5"></path>
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z"></path>
<path d="M4.42 11.247A13.152 13.152 0 0 0 4 14.556C4 18.728 7.582 21 12 21s8-2.272 8-6.444c0-1.061-.162-2.2-.493-3.309m-9.243-6.082A8.801 8.801 0 0 1 12 5c.78 0 1.5.108 2.161.306"></path>"#;

const DOLLAR_SIGN: &str = r#"
<line y1="2" y2="22" x2="12" x1="12"></line>
<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>"#;

const DONUT: &str = r#"
<path d="M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3"></path>
<circle cx="12" cy="12" r="3"></circle>"#;

const DOOR_CLOSED: &str = r#"
<path d="M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14"></path>
<path d="M2 20h20"></path>
<path d="M14 12v.01"></path>"#;

const DOOR_OPEN: &str = r#"
<path d="M13 4h3a2 2 0 0 1 2 2v14"></path>
<path d="M2 20h3"></path>
<path d="M13 20h9"></path>
<path d="M10 12v.01"></path>
<path d="M13 4.562v16.157a1 1 0 0 1-1.242.97L5 20V5.562a2 2 0 0 1 1.515-1.94l4-1A2 2 0 0 1 13 4.561Z"></path>"#;

const DOT: &str = r#"
<circle r="1" cx="12.1" cy="12.1"></circle>"#;

const DOWNLOAD_CLOUD: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M12 12v9"></path>
<path d="m8 17 4 4 4-4"></path>"#;

const DOWNLOAD: &str = r#"
<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
<polyline points="7 10 12 15 17 10"></polyline>
<line x2="12" y2="3" x1="12" y1="15"></line>"#;

const DRIBBBLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M19.13 5.09C15.22 9.14 10 10.44 2.25 10.94"></path>
<path d="M21.75 12.84c-6.62-1.41-12.14 1-16.38 6.32"></path>
<path d="M8.56 2.75c4.37 6 6 9.42 8 17.72"></path>"#;

const DROPLET: &str = r#"
<path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z"></path>"#;

const DROPLETS: &str = r#"
<path d="M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z"></path>
<path d="M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97"></path>"#;

const DRUMSTICK: &str = r#"
<path d="M15.45 15.4c-2.13.65-4.3.32-5.7-1.1-2.29-2.27-1.76-6.5 1.17-9.42 2.93-2.93 7.15-3.46 9.43-1.18 1.41 1.41 1.74 3.57 1.1 5.71-1.4-.51-3.26-.02-4.64 1.36-1.38 1.38-1.87 3.23-1.36 4.63z"></path>
<path d="m11.25 15.6-2.16 2.16a2.5 2.5 0 1 1-4.56 1.73 2.49 2.49 0 0 1-1.41-4.24 2.5 2.5 0 0 1 3.14-.32l2.16-2.16"></path>"#;

const DUMBBELL: &str = r#"
<path d="m6.5 6.5 11 11"></path>
<path d="m21 21-1-1"></path>
<path d="m3 3 1 1"></path>
<path d="m18 22 4-4"></path>
<path d="m2 6 4-4"></path>
<path d="m3 10 7-7"></path>
<path d="m14 21 7-7"></path>"#;

const EAR_OFF: &str = r#"
<path d="M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46"></path>
<path d="M6 8.5c0-.75.13-1.47.36-2.14"></path>
<path d="M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76"></path>
<path d="M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#;

const EAR: &str = r#"
<path d="M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0"></path>
<path d="M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4"></path>"#;

const EGG_FRIED: &str = r#"
<circle cx="11.5" cy="12.5" r="3.5"></circle>
<path d="M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z"></path>"#;

const EGG_OFF: &str = r#"
<path d="M6.399 6.399C5.362 8.157 4.65 10.189 4.5 12c-.37 4.43 1.27 9.95 7.5 10 3.256-.026 5.259-1.547 6.375-3.625"></path>
<path d="M19.532 13.875A14.07 14.07 0 0 0 19.5 12c-.36-4.34-3.95-9.96-7.5-10-1.04.012-2.082.502-3.046 1.297"></path>
<line x1="2" x2="22" y1="2" y2="22"></line>"#;

const EGG: &str = r#"
<path d="M12 22c6.23-.05 7.87-5.57 7.5-10-.36-4.34-3.95-9.96-7.5-10-3.55.04-7.14 5.66-7.5 10-.37 4.43 1.27 9.95 7.5 10z"></path>"#;

const EQUAL_NOT: &str = r#"
<line y1="9" y2="9" x2="19" x1="5"></line>
<line y1="15" x1="5" y2="15" x2="19"></line>
<line x1="19" y2="19" y1="5" x2="5"></line>"#;

const EQUAL: &str = r#"
<line x1="5" y1="9" y2="9" x2="19"></line>
<line x1="5" y1="15" y2="15" x2="19"></line>"#;

const ERASER: &str = r#"
<path d="m7 21-4.3-4.3c-1-1-1-2.5 0-3.4l9.6-9.6c1-1 2.5-1 3.4 0l5.6 5.6c1 1 1 2.5 0 3.4L13 21"></path>
<path d="M22 21H7"></path>
<path d="m5 11 9 9"></path>"#;

const EURO: &str = r#"
<path d="M4 10h12"></path>
<path d="M4 14h9"></path>
<path d="M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2"></path>"#;

const EXPAND: &str = r#"
<path d="m21 21-6-6m6 6v-4.8m0 4.8h-4.8"></path>
<path d="M3 16.2V21m0 0h4.8M3 21l6-6"></path>
<path d="M21 7.8V3m0 0h-4.8M21 3l-6 6"></path>
<path d="M3 7.8V3m0 0h4.8M3 3l6 6"></path>"#;

const EXTERNAL_LINK: &str = r#"
<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
<polyline points="15 3 21 3 21 9"></polyline>
<line x1="10" y1="14" y2="3" x2="21"></line>"#;

const EYE_OFF: &str = r#"
<path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path>
<path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"></path>
<path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"></path>
<line y1="2" x1="2" x2="22" y2="22"></line>"#;

const EYE: &str = r#"
<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
<circle r="3" cy="12" cx="12"></circle>"#;

const FACEBOOK: &str = r#"
<path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>"#;

const FACTORY: &str = r#"
<path d="M2 20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8l-7 5V8l-7 5V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"></path>
<path d="M17 18h1"></path>
<path d="M12 18h1"></path>
<path d="M7 18h1"></path>"#;

const FAN: &str = r#"
<path d="M10.827 16.379a6.082 6.082 0 0 1-8.618-7.002l5.412 1.45a6.082 6.082 0 0 1 7.002-8.618l-1.45 5.412a6.082 6.082 0 0 1 8.618 7.002l-5.412-1.45a6.082 6.082 0 0 1-7.002 8.618l1.45-5.412Z"></path>
<path d="M12 12v.01"></path>"#;

const FAST_FORWARD: &str = r#"
<polygon points="13 19 22 12 13 5 13 19"></polygon>
<polygon points="2 19 11 12 2 5 2 19"></polygon>"#;

const FEATHER: &str = r#"
<path d="M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z"></path>
<line x2="2" y1="8" y2="22" x1="16"></line>
<line x1="17.5" y2="15" x2="9" y1="15"></line>"#;

const FERRIS_WHEEL: &str = r#"
<circle cx="12" cy="12" r="2"></circle>
<path d="M12 2v4"></path>
<path d="m6.8 15-3.5 2"></path>
<path d="m20.7 7-3.5 2"></path>
<path d="M6.8 9 3.3 7"></path>
<path d="m20.7 17-3.5-2"></path>
<path d="m9 22 3-8 3 8"></path>
<path d="M8 22h8"></path>
<path d="M18 18.7a9 9 0 1 0-12 0"></path>"#;

const FIGMA: &str = r#"
<path d="M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z"></path>
<path d="M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z"></path>
<path d="M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z"></path>
<path d="M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z"></path>
<path d="M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z"></path>"#;

const FILE_ARCHIVE: &str = r#"
<path d="M4 22V4c0-.5.2-1 .6-1.4C5 2.2 5.5 2 6 2h8.5L20 7.5V20c0 .5-.2 1-.6 1.4-.4.4-.9.6-1.4.6h-2"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle r="2" cx="10" cy="20"></circle>
<path d="M10 7V6"></path>
<path d="M10 12v-1"></path>
<path d="M10 18v-2"></path>"#;

const FILE_AUDIO_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v2"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 17v-3a4 4 0 0 1 8 0v3"></path>
<circle cy="17" r="1" cx="9"></circle>
<circle cy="17" r="1" cx="3"></circle>"#;

const FILE_AUDIO: &str = r#"
<path d="M17.5 22h.5c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10 20v-1a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0Z"></path>
<path d="M6 20v-1a2 2 0 1 0-4 0v1a2 2 0 1 0 4 0Z"></path>
<path d="M2 19v-3a6 6 0 0 1 12 0v3"></path>"#;

const FILE_AXIS_3_D: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M8 10v8h8"></path>
<path d="m8 18 4-4"></path>"#;

const FILE_BADGE_2: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"></path>
<path d="m14 12.5 1 5.5-3-1-3 1 1-5.5"></path>"#;

const FILE_BADGE: &str = r#"
<path d="M4 7V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-6"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"></path>
<path d="M7 16.5 8 22l-3-1-3 1 1-5.5"></path>"#;

const FILE_BAR_CHART_2: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-6"></path>
<path d="M8 18v-1"></path>
<path d="M16 18v-3"></path>"#;

const FILE_BAR_CHART: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-4"></path>
<path d="M8 18v-2"></path>
<path d="M16 18v-6"></path>"#;

const FILE_BOX: &str = r#"
<path d="M14.5 22H18a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2.97 13.12c-.6.36-.97 1.02-.97 1.74v3.28c0 .72.37 1.38.97 1.74l3 1.83c.63.39 1.43.39 2.06 0l3-1.83c.6-.36.97-1.02.97-1.74v-3.28c0-.72-.37-1.38-.97-1.74l-3-1.83a1.97 1.97 0 0 0-2.06 0l-3 1.83Z"></path>
<path d="m7 17-4.74-2.85"></path>
<path d="m7 17 4.74-2.85"></path>
<path d="M7 17v5"></path>"#;

const FILE_CHECK_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m3 15 2 2 4-4"></path>"#;

const FILE_CHECK: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m9 15 2 2 4-4"></path>"#;

const FILE_CLOCK: &str = r#"
<path d="M16 22h2c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cx="8" r="6" cy="16"></circle>
<path d="M9.5 17.5 8 16.25V14"></path>"#;

const FILE_CODE_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m9 18 3-3-3-3"></path>
<path d="m5 12-3 3 3 3"></path>"#;

const FILE_CODE: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 13-2 2 2 2"></path>
<path d="m14 17 2-2-2-2"></path>"#;

const FILE_COG: &str = r#"
<circle r="3" cx="6" cy="13"></circle>
<path d="m9.7 14.4-.9-.3"></path>
<path d="m3.2 11.9-.9-.3"></path>
<path d="m4.6 16.7.3-.9"></path>
<path d="m7.6 16.7-.4-1"></path>
<path d="m4.8 10.3-.4-1"></path>
<path d="m2.3 14.6 1-.4"></path>
<path d="m8.7 11.8 1-.4"></path>
<path d="m7.4 9.3-.3.9"></path>
<path d="M14 2v6h6"></path>
<path d="M4 5.5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-1.5"></path>"#;

const FILE_DIFF: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 13V7"></path>
<path d="M9 10h6"></path>
<path d="M9 17h6"></path>"#;

const FILE_DIGIT: &str = r#"
<rect rx="2" height="6" x="2" y="12" width="4"></rect>
<path d="M14 2v6h6"></path>
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<path d="M10 12h2v6"></path>
<path d="M10 18h4"></path>"#;

const FILE_DOWN: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-6"></path>
<path d="m9 15 3 3 3-3"></path>"#;

const FILE_EDIT: &str = r#"
<path d="M4 13.5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-5.5"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z"></path>"#;

const FILE_HEART: &str = r#"
<path d="M4 6V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10.29 10.7a2.43 2.43 0 0 0-2.66-.52c-.29.12-.56.3-.78.53l-.35.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L6.5 18l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#;

const FILE_IMAGE: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cx="10" cy="13" r="2"></circle>
<path d="m20 17-1.09-1.09a2 2 0 0 0-2.82 0L10 22"></path>"#;

const FILE_INPUT: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 15h10"></path>
<path d="m9 18 3-3-3-3"></path>"#;

const FILE_JSON_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M4 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1"></path>
<path d="M8 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"></path>"#;

const FILE_JSON: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1"></path>
<path d="M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"></path>"#;

const FILE_KEY_2: &str = r#"
<path d="M4 10V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cy="16" cx="4" r="2"></circle>
<path d="m10 10-4.5 4.5"></path>
<path d="m9 11 1 1"></path>"#;

const FILE_KEY: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<circle r="2" cx="10" cy="16"></circle>
<path d="m16 10-4.5 4.5"></path>
<path d="m15 11 1 1"></path>"#;

const FILE_LINE_CHART: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m16 13-3.5 3.5-2-2L8 17"></path>"#;

const FILE_LOCK_2: &str = r#"
<path d="M4 5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<rect width="8" height="5" rx="1" x="2" y="13"></rect>
<path d="M8 13v-2a2 2 0 1 0-4 0v2"></path>"#;

const FILE_LOCK: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<rect width="8" x="8" y="12" rx="1" height="6"></rect>
<path d="M15 12v-2a3 3 0 1 0-6 0v2"></path>"#;

const FILE_MINUS_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M3 15h6"></path>"#;

const FILE_MINUS: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line x1="9" y2="15" y1="15" x2="15"></line>"#;

const FILE_OUTPUT: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 15h10"></path>
<path d="m5 12-3 3 3 3"></path>"#;

const FILE_PIE_CHART: &str = r#"
<path d="M16 22h2a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M4.04 11.71a5.84 5.84 0 1 0 8.2 8.29"></path>
<path d="M13.83 16A5.83 5.83 0 0 0 8 10.17V16h5.83Z"></path>"#;

const FILE_PLUS_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M3 15h6"></path>
<path d="M6 12v6"></path>"#;

const FILE_PLUS: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line x2="12" x1="12" y1="18" y2="12"></line>
<line y1="15" x1="9" x2="15" y2="15"></line>"#;

const FILE_QUESTION: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M10 10.3c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"></path>
<path d="M12 17h.01"></path>"#;

const FILE_SCAN: &str = r#"
<path d="M20 10V7.5L14.5 2H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h4.5"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M16 22a2 2 0 0 1-2-2"></path>
<path d="M20 22a2 2 0 0 0 2-2"></path>
<path d="M20 14a2 2 0 0 1 2 2"></path>
<path d="M16 14a2 2 0 0 0-2 2"></path>"#;

const FILE_SEARCH_2: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cx="11.5" cy="14.5" r="2.5"></circle>
<path d="M13.25 16.25 15 18"></path>"#;

const FILE_SEARCH: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="m9 18-1.5-1.5"></path>"#;

const FILE_SIGNATURE: &str = r#"
<path d="M20 19.5v.5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8.5L18 5.5"></path>
<path d="M8 18h1"></path>
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z"></path>"#;

const FILE_SPREADSHEET: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M8 13h2"></path>
<path d="M8 17h2"></path>
<path d="M14 13h2"></path>
<path d="M14 17h2"></path>"#;

const FILE_STACK: &str = r#"
<path d="M16 2v5h5"></path>
<path d="M21 6v6.5c0 .8-.7 1.5-1.5 1.5h-7c-.8 0-1.5-.7-1.5-1.5v-9c0-.8.7-1.5 1.5-1.5H17l4 4z"></path>
<path d="M7 8v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H15"></path>
<path d="M3 12v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H11"></path>"#;

const FILE_SYMLINK: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v7"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 18 3-3-3-3"></path>
<path d="M4 18v-1a2 2 0 0 1 2-2h6"></path>"#;

const FILE_TERMINAL: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m8 16 2-2-2-2"></path>
<path d="M12 18h4"></path>"#;

const FILE_TEXT: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line y2="13" x1="16" x2="8" y1="13"></line>
<line x1="16" x2="8" y2="17" y1="17"></line>
<line y1="9" x2="8" y2="9" x1="10"></line>"#;

const FILE_TYPE_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 13v-1h6v1"></path>
<path d="M4 18h2"></path>
<path d="M5 12v6"></path>"#;

const FILE_TYPE: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M9 13v-1h6v1"></path>
<path d="M11 18h2"></path>
<path d="M12 12v6"></path>"#;

const FILE_UP: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 12v6"></path>
<path d="m15 15-3-3-3 3"></path>"#;

const FILE_VIDEO_2: &str = r#"
<path d="M4 8V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 15.5 4 2.5v-6l-4 2.5"></path>
<rect height="6" x="2" y="12" width="8" rx="1"></rect>"#;

const FILE_VIDEO: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 11 5 3-5 3v-6Z"></path>"#;

const FILE_VOLUME_2: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M11.5 13.5c.32.4.5.94.5 1.5s-.18 1.1-.5 1.5"></path>
<path d="M15 12c.64.8 1 1.87 1 3s-.36 2.2-1 3"></path>
<path d="M8 15h.01"></path>"#;

const FILE_VOLUME: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m7 10-3 2H2v4h2l3 2v-8Z"></path>
<path d="M11 11c.64.8 1 1.87 1 3s-.36 2.2-1 3"></path>"#;

const FILE_WARNING: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 9v4"></path>
<path d="M12 17h.01"></path>"#;

const FILE_X_2: &str = r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<path d="M14 2v6h6"></path>
<path d="m3 12.5 5 5"></path>
<path d="m8 12.5-5 5"></path>"#;

const FILE_X: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line y2="17.5" x1="9.5" y1="12.5" x2="14.5"></line>
<line x2="9.5" y2="17.5" x1="14.5" y1="12.5"></line>"#;

const FILE: &str = r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>"#;

const FILES: &str = r#"
<path d="M15.5 2H8.6c-.4 0-.8.2-1.1.5-.3.3-.5.7-.5 1.1v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8c.4 0 .8-.2 1.1-.5.3-.3.5-.7.5-1.1V6.5L15.5 2z"></path>
<path d="M3 7.6v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8"></path>
<path d="M15 2v5h5"></path>"#;

const FILM: &str = r#"
<rect height="18" x="3" rx="2" width="18" y="3"></rect>
<path d="M7 3v18"></path>
<path d="M3 7.5h4"></path>
<path d="M3 12h18"></path>
<path d="M3 16.5h4"></path>
<path d="M17 3v18"></path>
<path d="M17 7.5h4"></path>
<path d="M17 16.5h4"></path>"#;

const FILTER_X: &str = r#"
<path d="M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055"></path>
<path d="m22 3-5 5"></path>
<path d="m17 3 5 5"></path>"#;

const FILTER: &str = r#"
<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>"#;

const FINGERPRINT: &str = r#"
<path d="M2 12C2 6.5 6.5 2 12 2a10 10 0 0 1 8 4"></path>
<path d="M5 19.5C5.5 18 6 15 6 12c0-.7.12-1.37.34-2"></path>
<path d="M17.29 21.02c.12-.6.43-2.3.5-3.02"></path>
<path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4"></path>
<path d="M8.65 22c.21-.66.45-1.32.57-2"></path>
<path d="M14 13.12c0 2.38 0 6.38-1 8.88"></path>
<path d="M2 16h.01"></path>
<path d="M21.8 16c.2-2 .131-5.354 0-6"></path>
<path d="M9 6.8a6 6 0 0 1 9 5.2c0 .47 0 1.17-.02 2"></path>"#;

const FISH_OFF: &str = r#"
<path d="M18 12.47v.03m0-.5v.47m-.475 5.056A6.744 6.744 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.114-2.433 2.121-3.38m3.444-2.088A8.802 8.802 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.177-1.413 3.058"></path>
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.372A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.728-3.618"></path>
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20"></path>"#;

const FISH_SYMBOL: &str = r#"
<path d="M2 16s9-15 20-4C11 23 2 8 2 8"></path>"#;

const FISH: &str = r#"
<path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z"></path>
<path d="M18 12v.5"></path>
<path d="M16 17.93a9.77 9.77 0 0 1 0-11.86"></path>
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33"></path>
<path d="M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4"></path>
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98"></path>"#;

const FLAG_OFF: &str = r#"
<path d="M8 2c3 0 5 2 8 2s4-1 4-1v11"></path>
<path d="M4 22V4"></path>
<path d="M4 15s1-1 4-1 5 2 8 2"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#;

const FLAG_TRIANGLE_LEFT: &str = r#"
<path d="M17 22V2L7 7l10 5"></path>"#;

const FLAG_TRIANGLE_RIGHT: &str = r#"
<path d="M7 22V2l10 5-10 5"></path>"#;

const FLAG: &str = r#"
<path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"></path>
<line y1="22" x1="4" y2="15" x2="4"></line>"#;

const FLAME: &str = r#"
<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"></path>"#;

const FLASHLIGHT_OFF: &str = r#"
<path d="M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4"></path>
<path d="M7 2h11v4c0 2-2 2-2 4v1"></path>
<line x2="18" x1="11" y2="6" y1="6"></line>
<line x1="2" y2="22" x2="22" y1="2"></line>"#;

const FLASHLIGHT: &str = r#"
<path d="M18 6c0 2-2 2-2 4v10a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4V2h12z"></path>
<line x1="6" y2="6" y1="6" x2="18"></line>
<line x1="12" x2="12" y1="12" y2="12"></line>"#;

const FLASK_CONICAL_OFF: &str = r#"
<path d="M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542"></path>
<path d="M10 2v2.343"></path>
<path d="M14 2v6.343"></path>
<path d="M8.5 2h7"></path>
<path d="M7 16h9"></path>
<line x2="22" y1="2" x1="2" y2="22"></line>"#;

const FLASK_CONICAL: &str = r#"
<path d="M10 2v7.527a2 2 0 0 1-.211.896L4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-5.069-10.127A2 2 0 0 1 14 9.527V2"></path>
<path d="M8.5 2h7"></path>
<path d="M7 16h10"></path>"#;

const FLASK_ROUND: &str = r#"
<path d="M10 2v7.31"></path>
<path d="M14 9.3V1.99"></path>
<path d="M8.5 2h7"></path>
<path d="M14 9.3a6.5 6.5 0 1 1-4 0"></path>
<path d="M5.52 16h12.96"></path>"#;

const FLIP_HORIZONTAL_2: &str = r#"
<path d="m3 7 5 5-5 5V7"></path>
<path d="m21 7-5 5 5 5V7"></path>
<path d="M12 20v2"></path>
<path d="M12 14v2"></path>
<path d="M12 8v2"></path>
<path d="M12 2v2"></path>"#;

const FLIP_HORIZONTAL: &str = r#"
<path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3"></path>
<path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3"></path>
<path d="M12 20v2"></path>
<path d="M12 14v2"></path>
<path d="M12 8v2"></path>
<path d="M12 2v2"></path>"#;

const FLIP_VERTICAL_2: &str = r#"
<path d="m17 3-5 5-5-5h10"></path>
<path d="m17 21-5-5-5 5h10"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#;

const FLIP_VERTICAL: &str = r#"
<path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3"></path>
<path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#;

const FLOWER_2: &str = r#"
<path d="M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1"></path>
<circle cy="8" cx="12" r="2"></circle>
<path d="M12 10v12"></path>
<path d="M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z"></path>
<path d="M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z"></path>"#;

const FLOWER: &str = r#"
<path d="M12 7.5a4.5 4.5 0 1 1 4.5 4.5M12 7.5A4.5 4.5 0 1 0 7.5 12M12 7.5V9m-4.5 3a4.5 4.5 0 1 0 4.5 4.5M7.5 12H9m7.5 0a4.5 4.5 0 1 1-4.5 4.5m4.5-4.5H15m-3 4.5V15"></path>
<circle cx="12" r="3" cy="12"></circle>
<path d="m8 16 1.5-1.5"></path>
<path d="M14.5 9.5 16 8"></path>
<path d="m8 8 1.5 1.5"></path>
<path d="M14.5 14.5 16 16"></path>"#;

const FOCUS: &str = r#"
<circle cx="12" cy="12" r="3"></circle>
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>"#;

const FOLD_HORIZONTAL: &str = r#"
<path d="M2 12h6"></path>
<path d="M22 12h-6"></path>
<path d="M12 2v2"></path>
<path d="M12 8v2"></path>
<path d="M12 14v2"></path>
<path d="M12 20v2"></path>
<path d="m19 9-3 3 3 3"></path>
<path d="m5 15 3-3-3-3"></path>"#;

const FOLD_VERTICAL: &str = r#"
<path d="M12 22v-6"></path>
<path d="M12 8V2"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>
<path d="m15 19-3-3-3 3"></path>
<path d="m15 5-3 3-3-3"></path>"#;

const FOLDER_ARCHIVE: &str = r#"
<path d="M22 20V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2h6"></path>
<circle r="2" cx="16" cy="19"></circle>
<path d="M16 11v-1"></path>
<path d="M16 17v-2"></path>"#;

const FOLDER_CHECK: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="m9 13 2 2 4-4"></path>"#;

const FOLDER_CLOCK: &str = r#"
<path d="M7 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2"></path>
<circle cx="16" r="6" cy="16"></circle>
<path d="M16 14v2l1 1"></path>"#;

const FOLDER_CLOSED: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M2 10h20"></path>"#;

const FOLDER_COG: &str = r#"
<circle r="3" cx="18" cy="18"></circle>
<path d="M10.5 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.5"></path>
<path d="m21.7 19.4-.9-.3"></path>
<path d="m15.2 16.9-.9-.3"></path>
<path d="m16.6 21.7.3-.9"></path>
<path d="m19.1 15.2.3-.9"></path>
<path d="m19.6 21.7-.4-1"></path>
<path d="m16.8 15.3-.4-1"></path>
<path d="m14.3 19.6 1-.4"></path>
<path d="m20.7 16.8 1-.4"></path>"#;

const FOLDER_DOT: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cy="13" cx="12" r="1"></circle>"#;

const FOLDER_DOWN: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M12 10v6"></path>
<path d="m15 13-3 3-3-3"></path>"#;

const FOLDER_EDIT: &str = r#"
<path d="M8.42 10.61a2.1 2.1 0 1 1 2.97 2.97L5.95 19 2 20l.99-3.95 5.43-5.44Z"></path>
<path d="M2 11.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5"></path>"#;

const FOLDER_GIT_2: &str = r#"
<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5"></path>
<circle cy="12" cx="13" r="2"></circle>
<path d="M18 19c-2.8 0-5-2.2-5-5v8"></path>
<circle cx="20" r="2" cy="19"></circle>"#;

const FOLDER_GIT: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle r="2" cx="12" cy="13"></circle>
<path d="M14 13h3"></path>
<path d="M7 13h3"></path>"#;

const FOLDER_HEART: &str = r#"
<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v1.5"></path>
<path d="M21.29 13.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 21l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#;

const FOLDER_INPUT: &str = r#"
<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1"></path>
<path d="M2 13h10"></path>
<path d="m9 16 3-3-3-3"></path>"#;

const FOLDER_KANBAN: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M8 10v4"></path>
<path d="M12 10v2"></path>
<path d="M16 10v6"></path>"#;

const FOLDER_KEY: &str = r#"
<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2"></path>
<circle r="2" cy="20" cx="16"></circle>
<path d="m22 14-4.5 4.5"></path>
<path d="m21 15 1 1"></path>"#;

const FOLDER_LOCK: &str = r#"
<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2.5"></path>
<rect height="5" width="8" y="17" rx="1" x="14"></rect>
<path d="M20 17v-2a2 2 0 1 0-4 0v2"></path>"#;

const FOLDER_MINUS: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<line y2="13" x2="15" x1="9" y1="13"></line>"#;

const FOLDER_OPEN_DOT: &str = r#"
<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2"></path>
<circle cy="15" r="1" cx="14"></circle>"#;

const FOLDER_OPEN: &str = r#"
<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2"></path>"#;

const FOLDER_OUTPUT: &str = r#"
<path d="M2 7.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2"></path>
<path d="M2 13h10"></path>
<path d="m5 10-3 3 3 3"></path>"#;

const FOLDER_PLUS: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<line x2="12" y2="16" y1="10" x1="12"></line>
<line y1="13" y2="13" x2="15" x1="9"></line>"#;

const FOLDER_ROOT: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cx="12" r="2" cy="13"></circle>
<path d="M12 15v5"></path>"#;

const FOLDER_SEARCH_2: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cx="11.5" r="2.5" cy="12.5"></circle>
<path d="M13.27 14.27 15 16"></path>"#;

const FOLDER_SEARCH: &str = r#"
<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v4"></path>
<circle r="3" cy="17" cx="17"></circle>
<path d="m21 21-1.5-1.5"></path>"#;

const FOLDER_SYMLINK: &str = r#"
<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2"></path>
<path d="m8 16 3-3-3-3"></path>
<path d="M2 16v-1a2 2 0 0 1 2-2h6"></path>"#;

const FOLDER_SYNC: &str = r#"
<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1"></path>
<path d="M12 10v4h4"></path>
<path d="m12 14 1.5-1.5c.9-.9 2.2-1.5 3.5-1.5s2.6.6 3.5 1.5c.4.4.8 1 1 1.5"></path>
<path d="M22 22v-4h-4"></path>
<path d="m22 18-1.5 1.5c-.9.9-2.1 1.5-3.5 1.5s-2.6-.6-3.5-1.5c-.4-.4-.8-1-1-1.5"></path>"#;

const FOLDER_TREE: &str = r#"
<path d="M13 10h7a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"></path>
<path d="M13 21h7a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.88a1 1 0 0 1-.9-.55l-.44-.9a1 1 0 0 0-.9-.55H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"></path>
<path d="M3 3v2c0 1.1.9 2 2 2h3"></path>
<path d="M3 3v13c0 1.1.9 2 2 2h3"></path>"#;

const FOLDER_UP: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M12 10v6"></path>
<path d="m9 13 3-3 3 3"></path>"#;

const FOLDER_X: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="m9.5 10.5 5 5"></path>
<path d="m14.5 10.5-5 5"></path>"#;

const FOLDER: &str = r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>"#;

const FOLDERS: &str = r#"
<path d="M8 17h12a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3.93a2 2 0 0 1-1.66-.9l-.82-1.2a2 2 0 0 0-1.66-.9H8a2 2 0 0 0-2 2v9c0 1.1.9 2 2 2Z"></path>
<path d="M2 8v11c0 1.1.9 2 2 2h14"></path>"#;

const FOOTPRINTS: &str = r#"
<path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z"></path>
<path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z"></path>
<path d="M16 17h4"></path>
<path d="M4 13h4"></path>"#;

const FORKLIFT: &str = r#"
<path d="M12 12H5a2 2 0 0 0-2 2v5"></path>
<circle r="2" cy="19" cx="13"></circle>
<circle cy="19" r="2" cx="5"></circle>
<path d="M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5"></path>"#;

const FORM_INPUT: &str = r#"
<rect x="2" y="6" rx="2" width="20" height="12"></rect>
<path d="M12 12h.01"></path>
<path d="M17 12h.01"></path>
<path d="M7 12h.01"></path>"#;

const FORWARD: &str = r#"
<polyline points="15 17 20 12 15 7"></polyline>
<path d="M4 18v-2a4 4 0 0 1 4-4h12"></path>"#;

const FRAME: &str = r#"
<line y1="6" y2="6" x2="2" x1="22"></line>
<line x2="2" y2="18" x1="22" y1="18"></line>
<line x1="6" y2="22" x2="6" y1="2"></line>
<line y1="2" y2="22" x2="18" x1="18"></line>"#;

const FRAMER: &str = r#"
<path d="M5 16V9h14V2H5l14 14h-7m-7 0 7 7v-7m-7 0h7"></path>"#;

const FROWN: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M16 16s-1.5-2-4-2-4 2-4 2"></path>
<line y2="9" y1="9" x2="9.01" x1="9"></line>
<line x1="15" y1="9" y2="9" x2="15.01"></line>"#;

const FUEL: &str = r#"
<line x1="3" x2="15" y1="22" y2="22"></line>
<line x2="14" x1="4" y1="9" y2="9"></line>
<path d="M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18"></path>
<path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5"></path>"#;

const FUNCTION_SQUARE: &str = r#"
<rect width="18" height="18" rx="2" y="3" ry="2" x="3"></rect>
<path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3"></path>
<path d="M9 11.2h5.7"></path>"#;

const GALLERY_HORIZONTAL_END: &str = r#"
<path d="M2 7v10"></path>
<path d="M6 5v14"></path>
<rect width="12" x="10" y="3" rx="2" height="18"></rect>"#;

const GALLERY_HORIZONTAL: &str = r#"
<path d="M2 3v18"></path>
<rect width="12" height="18" rx="2" y="3" x="6"></rect>
<path d="M22 3v18"></path>"#;

const GALLERY_THUMBNAILS: &str = r#"
<rect height="14" x="3" y="3" width="18" rx="2"></rect>
<path d="M4 21h1"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>
<path d="M19 21h1"></path>"#;

const GALLERY_VERTICAL_END: &str = r#"
<path d="M7 2h10"></path>
<path d="M5 6h14"></path>
<rect height="12" width="18" x="3" y="10" rx="2"></rect>"#;

const GALLERY_VERTICAL: &str = r#"
<path d="M3 2h18"></path>
<rect width="18" height="12" rx="2" x="3" y="6"></rect>
<path d="M3 22h18"></path>"#;

const GAMEPAD_2: &str = r#"
<line x1="6" x2="10" y1="11" y2="11"></line>
<line y2="13" y1="9" x1="8" x2="8"></line>
<line x2="15.01" x1="15" y1="12" y2="12"></line>
<line y2="10" x2="18.01" x1="18" y1="10"></line>
<path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"></path>"#;

const GAMEPAD: &str = r#"
<line x1="6" y2="12" x2="10" y1="12"></line>
<line x2="8" x1="8" y1="10" y2="14"></line>
<line x1="15" x2="15.01" y2="13" y1="13"></line>
<line x1="18" y1="11" y2="11" x2="18.01"></line>
<rect width="20" y="6" height="12" x="2" rx="2"></rect>"#;

const GANTT_CHART_SQUARE: &str = r#"
<rect y="3" width="18" rx="2" height="18" x="3"></rect>
<path d="M9 8h7"></path>
<path d="M8 12h6"></path>
<path d="M11 16h5"></path>"#;

const GANTT_CHART: &str = r#"
<path d="M8 6h10"></path>
<path d="M6 12h9"></path>
<path d="M11 18h7"></path>"#;

const GAUGE_CIRCLE: &str = r#"
<path d="M15.6 2.7a10 10 0 1 0 5.7 5.7"></path>
<circle cx="12" cy="12" r="2"></circle>
<path d="M13.4 10.6 19 5"></path>"#;

const GAUGE: &str = r#"
<path d="m12 14 4-4"></path>
<path d="M3.34 19a10 10 0 1 1 17.32 0"></path>"#;

const GAVEL: &str = r#"
<path d="m14 13-7.5 7.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L11 10"></path>
<path d="m16 16 6-6"></path>
<path d="m8 8 6-6"></path>
<path d="m9 7 8 8"></path>
<path d="m21 11-8-8"></path>"#;

const GEM: &str = r#"
<path d="M6 3h12l4 6-10 13L2 9Z"></path>
<path d="M11 3 8 9l4 13 4-13-3-6"></path>
<path d="M2 9h20"></path>"#;

const GHOST: &str = r#"
<path d="M9 10h.01"></path>
<path d="M15 10h.01"></path>
<path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z"></path>"#;

const GIFT: &str = r#"
<polyline points="20 12 20 22 4 22 4 12"></polyline>
<rect width="20" height="5" x="2" y="7"></rect>
<line y1="22" x1="12" x2="12" y2="7"></line>
<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>"#;

const GIT_BRANCH_PLUS: &str = r#"
<path d="M6 3v12"></path>
<path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="M15 6a9 9 0 0 0-9 9"></path>
<path d="M18 15v6"></path>
<path d="M21 18h-6"></path>"#;

const GIT_BRANCH: &str = r#"
<line x2="6" y1="3" y2="15" x1="6"></line>
<circle r="3" cx="18" cy="6"></circle>
<circle cx="6" r="3" cy="18"></circle>
<path d="M18 9a9 9 0 0 1-9 9"></path>"#;

const GIT_COMMIT: &str = r#"
<circle cx="12" cy="12" r="3"></circle>
<line y2="12" x2="9" y1="12" x1="3"></line>
<line y1="12" x1="15" y2="12" x2="21"></line>"#;

const GIT_COMPARE: &str = r#"
<circle cy="18" cx="18" r="3"></circle>
<circle cx="6" r="3" cy="6"></circle>
<path d="M13 6h3a2 2 0 0 1 2 2v7"></path>
<path d="M11 18H8a2 2 0 0 1-2-2V9"></path>"#;

const GIT_FORK: &str = r#"
<circle cx="12" r="3" cy="18"></circle>
<circle cx="6" r="3" cy="6"></circle>
<circle r="3" cx="18" cy="6"></circle>
<path d="M18 9v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V9"></path>
<path d="M12 12v3"></path>"#;

const GIT_MERGE: &str = r#"
<circle r="3" cx="18" cy="18"></circle>
<circle cx="6" r="3" cy="6"></circle>
<path d="M6 21V9a9 9 0 0 0 9 9"></path>"#;

const GIT_PULL_REQUEST_CLOSED: &str = r#"
<circle cx="18" r="3" cy="18"></circle>
<circle cy="6" r="3" cx="6"></circle>
<path d="M18 11.5V15"></path>
<path d="m21 3-6 6"></path>
<path d="m21 9-6-6"></path>
<line y1="9" y2="21" x1="6" x2="6"></line>"#;

const GIT_PULL_REQUEST_DRAFT: &str = r#"
<circle r="3" cy="18" cx="18"></circle>
<circle cx="6" cy="6" r="3"></circle>
<path d="M18 6V5"></path>
<path d="M18 11v-1"></path>
<line x1="6" x2="6" y2="21" y1="9"></line>"#;

const GIT_PULL_REQUEST: &str = r#"
<circle cy="18" r="3" cx="18"></circle>
<circle cy="6" cx="6" r="3"></circle>
<path d="M13 6h3a2 2 0 0 1 2 2v7"></path>
<line x2="6" x1="6" y1="9" y2="21"></line>"#;

const GITHUB: &str = r#"
<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
<path d="M9 18c-4.51 2-5-2-7-2"></path>"#;

const GITLAB: &str = r#"
<path d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z"></path>"#;

const GLASS_WATER: &str = r#"
<path d="M15.2 22H8.8a2 2 0 0 1-2-1.79L5 3h14l-1.81 17.21A2 2 0 0 1 15.2 22Z"></path>
<path d="M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0"></path>"#;

const GLASSES: &str = r#"
<circle r="4" cx="6" cy="15"></circle>
<circle r="4" cy="15" cx="18"></circle>
<path d="M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2"></path>
<path d="M2.5 13 5 7c.7-1.3 1.4-2 3-2"></path>
<path d="M21.5 13 19 7c-.7-1.3-1.5-2-3-2"></path>"#;

const GLOBE_2: &str = r#"
<path d="M21.54 15H17a2 2 0 0 0-2 2v4.54"></path>
<path d="M7 3.34V5a3 3 0 0 0 3 3v0a2 2 0 0 1 2 2v0c0 1.1.9 2 2 2v0a2 2 0 0 0 2-2v0c0-1.1.9-2 2-2h3.17"></path>
<path d="M11 21.95V18a2 2 0 0 0-2-2v0a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05"></path>
<circle cy="12" cx="12" r="10"></circle>"#;

const GLOBE: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<line y1="12" y2="12" x2="22" x1="2"></line>
<path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>"#;

const GOAL: &str = r#"
<path d="M12 13V2l8 4-8 4"></path>
<path d="M20.55 10.23A9 9 0 1 1 8 4.94"></path>
<path d="M8 10a5 5 0 1 0 8.9 2.02"></path>"#;

const GRAB: &str = r#"
<path d="M18 11.5V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4"></path>
<path d="M14 10V8a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2"></path>
<path d="M10 9.9V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5"></path>
<path d="M6 14v0a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M18 11v0a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0"></path>"#;

const GRADUATION_CAP: &str = r#"
<path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
<path d="M6 12v5c3 3 9 3 12 0v-5"></path>"#;

const GRAPE: &str = r#"
<path d="M22 5V2l-5.89 5.89"></path>
<circle r="3" cx="16.6" cy="15.89"></circle>
<circle r="3" cy="7.4" cx="8.11"></circle>
<circle r="3" cx="12.35" cy="11.65"></circle>
<circle cx="13.91" cy="5.85" r="3"></circle>
<circle cy="10.09" cx="18.15" r="3"></circle>
<circle cy="13.2" r="3" cx="6.56"></circle>
<circle cx="10.8" cy="17.44" r="3"></circle>
<circle cy="19" cx="5" r="3"></circle>"#;

const GRID_2_X_2: &str = r#"
<rect width="18" height="18" rx="2" x="3" y="3"></rect>
<path d="M3 12h18"></path>
<path d="M12 3v18"></path>"#;

const GRID_3_X_3: &str = r#"
<rect y="3" height="18" width="18" x="3" rx="2"></rect>
<path d="M3 9h18"></path>
<path d="M3 15h18"></path>
<path d="M9 3v18"></path>
<path d="M15 3v18"></path>"#;

const GRIP_HORIZONTAL: &str = r#"
<circle r="1" cy="9" cx="12"></circle>
<circle cx="19" r="1" cy="9"></circle>
<circle cx="5" cy="9" r="1"></circle>
<circle cx="12" r="1" cy="15"></circle>
<circle cx="19" r="1" cy="15"></circle>
<circle cx="5" cy="15" r="1"></circle>"#;

const GRIP_VERTICAL: &str = r#"
<circle r="1" cy="12" cx="9"></circle>
<circle cy="5" cx="9" r="1"></circle>
<circle cx="9" cy="19" r="1"></circle>
<circle cy="12" r="1" cx="15"></circle>
<circle r="1" cy="5" cx="15"></circle>
<circle cy="19" r="1" cx="15"></circle>"#;

const GRIP: &str = r#"
<circle cx="12" r="1" cy="5"></circle>
<circle cy="5" cx="19" r="1"></circle>
<circle cy="5" cx="5" r="1"></circle>
<circle r="1" cx="12" cy="12"></circle>
<circle r="1" cy="12" cx="19"></circle>
<circle cy="12" cx="5" r="1"></circle>
<circle cy="19" cx="12" r="1"></circle>
<circle r="1" cy="19" cx="19"></circle>
<circle r="1" cx="5" cy="19"></circle>"#;

const GROUP: &str = r#"
<path d="M3 7V5c0-1.1.9-2 2-2h2"></path>
<path d="M17 3h2c1.1 0 2 .9 2 2v2"></path>
<path d="M21 17v2c0 1.1-.9 2-2 2h-2"></path>
<path d="M7 21H5c-1.1 0-2-.9-2-2v-2"></path>
<rect y="7" x="7" width="7" height="5" rx="1"></rect>
<rect y="12" width="7" x="10" rx="1" height="5"></rect>"#;

const HAMMER: &str = r#"
<path d="m15 12-8.5 8.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L12 9"></path>
<path d="M17.64 15 22 10.64"></path>
<path d="m20.91 11.7-1.25-1.25c-.6-.6-.93-1.4-.93-2.25v-.86L16.01 4.6a5.56 5.56 0 0 0-3.94-1.64H9l.92.82A6.18 6.18 0 0 1 12 8.4v1.56l2 2h2.47l2.26 1.91"></path>"#;

const HAND_METAL: &str = r#"
<path d="M18 12.5V10a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4"></path>
<path d="M14 11V9a2 2 0 1 0-4 0v2"></path>
<path d="M10 10.5V5a2 2 0 1 0-4 0v9"></path>
<path d="m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5"></path>"#;

const HAND: &str = r#"
<path d="M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2"></path>
<path d="M10 10.5V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v8"></path>
<path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"></path>"#;

const HARD_DRIVE_DOWNLOAD: &str = r#"
<path d="M12 2v8"></path>
<path d="m16 6-4 4-4-4"></path>
<rect rx="2" x="2" width="20" height="8" y="14"></rect>
<path d="M6 18h.01"></path>
<path d="M10 18h.01"></path>"#;

const HARD_DRIVE_UPLOAD: &str = r#"
<path d="m16 6-4-4-4 4"></path>
<path d="M12 2v8"></path>
<rect x="2" rx="2" y="14" height="8" width="20"></rect>
<path d="M6 18h.01"></path>
<path d="M10 18h.01"></path>"#;

const HARD_DRIVE: &str = r#"
<line x1="22" x2="2" y1="12" y2="12"></line>
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>
<line y2="16" y1="16" x1="6" x2="6.01"></line>
<line y2="16" x2="10.01" x1="10" y1="16"></line>"#;

const HARD_HAT: &str = r#"
<path d="M2 18a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v2z"></path>
<path d="M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5"></path>
<path d="M4 15v-3a6 6 0 0 1 6-6h0"></path>
<path d="M14 6h0a6 6 0 0 1 6 6v3"></path>"#;

const HASH: &str = r#"
<line y1="9" x1="4" x2="20" y2="9"></line>
<line x1="4" x2="20" y1="15" y2="15"></line>
<line x1="10" y1="3" y2="21" x2="8"></line>
<line x1="16" x2="14" y1="3" y2="21"></line>"#;

const HAZE: &str = r#"
<path d="m5.2 6.2 1.4 1.4"></path>
<path d="M2 13h2"></path>
<path d="M20 13h2"></path>
<path d="m17.4 7.6 1.4-1.4"></path>
<path d="M22 17H2"></path>
<path d="M22 21H2"></path>
<path d="M16 13a4 4 0 0 0-8 0"></path>
<path d="M12 5V2.5"></path>"#;

const HDMI_PORT: &str = r#"
<path d="M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z"></path>
<path d="M7.5 12h9"></path>"#;

const HEADING_1: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="m17 12 3-2v8"></path>"#;

const HEADING_2: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>"#;

const HEADING_3: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
<path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>"#;

const HEADING_4: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17 10v4h4"></path>
<path d="M21 10v8"></path>"#;

const HEADING_5: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17 13v-3h4"></path>
<path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"></path>"#;

const HEADING_6: &str = r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<circle r="2" cy="16" cx="19"></circle>
<path d="M20 10c-2 2-3 3.5-3 6"></path>"#;

const HEADING: &str = r#"
<path d="M6 12h12"></path>
<path d="M6 20V4"></path>
<path d="M18 20V4"></path>"#;

const HEADPHONES: &str = r#"
<path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"></path>"#;

const HEART_CRACK: &str = r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="m12 13-1-1 2-2-3-3 2-2"></path>"#;

const HEART_HANDSHAKE: &str = r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08v0c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0l2.96 2.66"></path>
<path d="m18 15-2-2"></path>
<path d="m15 18-2-2"></path>"#;

const HEART_OFF: &str = r#"
<line y2="22" y1="2" x1="2" x2="22"></line>
<path d="M16.5 16.5 12 21l-7-7c-1.5-1.45-3-3.2-3-5.5a5.5 5.5 0 0 1 2.14-4.35"></path>
<path d="M8.76 3.1c1.15.22 2.13.78 3.24 1.9 1.5-1.5 2.74-2 4.5-2A5.5 5.5 0 0 1 22 8.5c0 2.12-1.3 3.78-2.67 5.17"></path>"#;

const HEART_PULSE: &str = r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="M3.22 12H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27"></path>"#;

const HEART: &str = r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>"#;

const HELP_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
<path d="M12 17h.01"></path>"#;

const HELPING_HAND: &str = r#"
<path d="m3 15 5.12-5.12A3 3 0 0 1 10.24 9H13a2 2 0 1 1 0 4h-2.5m4-.68 4.17-4.89a1.88 1.88 0 0 1 2.92 2.36l-4.2 5.94A3 3 0 0 1 14.96 17H9.83a2 2 0 0 0-1.42.59L7 19"></path>
<path d="m2 14 6 6"></path>"#;

const HEXAGON: &str = r#"
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>"#;

const HIGHLIGHTER: &str = r#"
<path d="m9 11-6 6v3h9l3-3"></path>
<path d="m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4"></path>"#;

const HISTORY: &str = r#"
<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>
<path d="M12 7v5l4 2"></path>"#;

const HOME: &str = r#"
<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
<polyline points="9 22 9 12 15 12 15 22"></polyline>"#;

const HOP_OFF: &str = r#"
<path d="M17.5 5.5C19 7 20.5 9 21 11c-1.323.265-2.646.39-4.118.226"></path>
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5"></path>
<path d="M17.5 17.5c-2.5 0-4 0-6-1"></path>
<path d="M20 11.5c1 1.5 2 3.5 2 4.5"></path>
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5"></path>
<path d="M22 22c-2 0-3.5-.5-5.5-1.5"></path>
<path d="M4.783 4.782C1.073 8.492 1 14.5 5 18c1-1 2-4.5 1.5-6.5 1.5 1 4 1 5.5.5M8.227 2.57C11.578 1.335 15.453 2.089 18 5c-.88.88-3.7 1.761-5.726 1.618"></path>
<line x1="2" y1="2" x2="22" y2="22"></line>"#;

const HOP: &str = r#"
<path d="M17.5 5.5C19 7 20.5 9 21 11c-2.5.5-5 .5-8.5-1"></path>
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5"></path>
<path d="M16.5 11.5c1 2 1 3.5 1 6-2.5 0-4 0-6-1"></path>
<path d="M20 11.5c1 1.5 2 3.5 2 4.5-1.5.5-3 0-4.5-.5"></path>
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5"></path>
<path d="M20.5 16.5c1 2 1.5 3.5 1.5 5.5-2 0-3.5-.5-5.5-1.5"></path>
<path d="M4.783 4.782C8.493 1.072 14.5 1 18 5c-1 1-4.5 2-6.5 1.5 1 1.5 1 4 .5 5.5-1.5.5-4 .5-5.5-.5C7 13.5 6 17 5 18c-4-3.5-3.927-9.508-.217-13.218Z"></path>
<path d="M4.5 4.5 3 3c-.184-.185-.184-.816 0-1"></path>"#;

const HOTEL: &str = r#"
<path d="M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2Z"></path>
<path d="m9 16 .348-.24c1.465-1.013 3.84-1.013 5.304 0L15 16"></path>
<path d="M8 7h.01"></path>
<path d="M16 7h.01"></path>
<path d="M12 7h.01"></path>
<path d="M12 11h.01"></path>
<path d="M16 11h.01"></path>
<path d="M8 11h.01"></path>
<path d="M10 22v-6.5m4 0V22"></path>"#;

const HOURGLASS: &str = r#"
<path d="M5 22h14"></path>
<path d="M5 2h14"></path>
<path d="M17 22v-4.172a2 2 0 0 0-.586-1.414L12 12l-4.414 4.414A2 2 0 0 0 7 17.828V22"></path>
<path d="M7 2v4.172a2 2 0 0 0 .586 1.414L12 12l4.414-4.414A2 2 0 0 0 17 6.172V2"></path>"#;

const ICE_CREAM_2: &str = r#"
<path d="M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6Zm-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0"></path>
<path d="M12.14 11a3.5 3.5 0 1 1 6.71 0"></path>
<path d="M15.5 6.5a3.5 3.5 0 1 0-7 0"></path>"#;

const ICE_CREAM: &str = r#"
<path d="m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11"></path>
<path d="M17 7A5 5 0 0 0 7 7"></path>
<path d="M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4"></path>"#;

const IMAGE_MINUS: &str = r#"
<path d="M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
<line x1="16" y2="5" x2="22" y1="5"></line>
<circle r="2" cx="9" cy="9"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#;

const IMAGE_OFF: &str = r#"
<line y1="2" x2="22" y2="22" x1="2"></line>
<path d="M10.41 10.41a2 2 0 1 1-2.83-2.83"></path>
<line y2="21" x1="13.5" x2="6" y1="13.5"></line>
<line x2="21" y1="12" x1="18" y2="15"></line>
<path d="M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59"></path>
<path d="M21 15V5a2 2 0 0 0-2-2H9"></path>"#;

const IMAGE_PLUS: &str = r#"
<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
<line x1="16" x2="22" y2="5" y1="5"></line>
<line y2="8" y1="2" x1="19" x2="19"></line>
<circle cx="9" cy="9" r="2"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#;

const IMAGE: &str = r#"
<rect y="3" x="3" ry="2" rx="2" height="18" width="18"></rect>
<circle r="2" cx="9" cy="9"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#;

const IMPORT: &str = r#"
<path d="M12 3v12"></path>
<path d="m8 11 4 4 4-4"></path>
<path d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4"></path>"#;

const INBOX: &str = r#"
<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>"#;

const INDENT: &str = r#"
<polyline points="3 8 7 12 3 16"></polyline>
<line y1="12" x1="21" y2="12" x2="11"></line>
<line y1="6" y2="6" x2="11" x1="21"></line>
<line x1="21" x2="11" y1="18" y2="18"></line>"#;

const INDIAN_RUPEE: &str = r#"
<path d="M6 3h12"></path>
<path d="M6 8h12"></path>
<path d="m6 13 8.5 8"></path>
<path d="M6 13h3"></path>
<path d="M9 13c6.667 0 6.667-10 0-10"></path>"#;

const INFINITY: &str = r#"
<path d="M12 12c-2-2.67-4-4-6-4a4 4 0 1 0 0 8c2 0 4-1.33 6-4Zm0 0c2 2.67 4 4 6 4a4 4 0 0 0 0-8c-2 0-4 1.33-6 4Z"></path>"#;

const INFO: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M12 16v-4"></path>
<path d="M12 8h.01"></path>"#;

const INSTAGRAM: &str = r#"
<rect y="2" x="2" width="20" rx="5" ry="5" height="20"></rect>
<path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
<line x2="17.51" y2="6.5" y1="6.5" x1="17.5"></line>"#;

const ITALIC: &str = r#"
<line x1="19" x2="10" y1="4" y2="4"></line>
<line y1="20" x1="14" x2="5" y2="20"></line>
<line y2="20" x2="9" x1="15" y1="4"></line>"#;

const ITERATION_CCW: &str = r#"
<path d="M20 10c0-4.4-3.6-8-8-8s-8 3.6-8 8 3.6 8 8 8h8"></path>
<polyline points="16 14 20 18 16 22"></polyline>"#;

const ITERATION_CW: &str = r#"
<path d="M4 10c0-4.4 3.6-8 8-8s8 3.6 8 8-3.6 8-8 8H4"></path>
<polyline points="8 22 4 18 8 14"></polyline>"#;

const JAPANESE_YEN: &str = r#"
<path d="M12 9.5V21m0-11.5L6 3m6 6.5L18 3"></path>
<path d="M6 15h12"></path>
<path d="M6 11h12"></path>"#;

const JOYSTICK: &str = r#"
<path d="M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z"></path>
<path d="M6 15v-2"></path>
<path d="M12 15V9"></path>
<circle r="3" cx="12" cy="6"></circle>"#;

const KANBAN_SQUARE_DASHED: &str = r#"
<path d="M8 7v7"></path>
<path d="M12 7v4"></path>
<path d="M16 7v9"></path>
<path d="M5 3a2 2 0 0 0-2 2"></path>
<path d="M9 3h1"></path>
<path d="M14 3h1"></path>
<path d="M19 3a2 2 0 0 1 2 2"></path>
<path d="M21 9v1"></path>
<path d="M21 14v1"></path>
<path d="M21 19a2 2 0 0 1-2 2"></path>
<path d="M14 21h1"></path>
<path d="M9 21h1"></path>
<path d="M5 21a2 2 0 0 1-2-2"></path>
<path d="M3 14v1"></path>
<path d="M3 9v1"></path>"#;

const KANBAN_SQUARE: &str = r#"
<rect x="3" width="18" y="3" height="18" rx="2"></rect>
<path d="M8 7v7"></path>
<path d="M12 7v4"></path>
<path d="M16 7v9"></path>"#;

const KANBAN: &str = r#"
<path d="M6 5v11"></path>
<path d="M12 5v6"></path>
<path d="M18 5v14"></path>"#;

const KEY_ROUND: &str = r#"
<path d="M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z"></path>
<circle r=".5" cx="16.5" cy="7.5"></circle>"#;

const KEY_SQUARE: &str = r#"
<path d="M12.4 2.7c.9-.9 2.5-.9 3.4 0l5.5 5.5c.9.9.9 2.5 0 3.4l-3.7 3.7c-.9.9-2.5.9-3.4 0L8.7 9.8c-.9-.9-.9-2.5 0-3.4Z"></path>
<path d="m14 7 3 3"></path>
<path d="M9.4 10.6 2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4"></path>"#;

const KEY: &str = r#"
<circle cx="7.5" cy="15.5" r="5.5"></circle>
<path d="m21 2-9.6 9.6"></path>
<path d="m15.5 7.5 3 3L22 7l-3-3"></path>"#;

const KEYBOARD: &str = r#"
<rect y="4" x="2" rx="2" width="20" height="16" ry="2"></rect>
<path d="M6 8h.001"></path>
<path d="M10 8h.001"></path>
<path d="M14 8h.001"></path>
<path d="M18 8h.001"></path>
<path d="M8 12h.001"></path>
<path d="M12 12h.001"></path>
<path d="M16 12h.001"></path>
<path d="M7 16h10"></path>"#;

const LAMP_CEILING: &str = r#"
<path d="M12 2v5"></path>
<path d="M6 7h12l4 9H2l4-9Z"></path>
<path d="M9.17 16a3 3 0 1 0 5.66 0"></path>"#;

const LAMP_DESK: &str = r#"
<path d="m14 5-3 3 2 7 8-8-7-2Z"></path>
<path d="m14 5-3 3-3-3 3-3 3 3Z"></path>
<path d="M9.5 6.5 4 12l3 6"></path>
<path d="M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z"></path>"#;

const LAMP_FLOOR: &str = r#"
<path d="M9 2h6l3 7H6l3-7Z"></path>
<path d="M12 9v13"></path>
<path d="M9 22h6"></path>"#;

const LAMP_WALL_DOWN: &str = r#"
<path d="M11 13h6l3 7H8l3-7Z"></path>
<path d="M14 13V8a2 2 0 0 0-2-2H8"></path>
<path d="M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z"></path>"#;

const LAMP_WALL_UP: &str = r#"
<path d="M11 4h6l3 7H8l3-7Z"></path>
<path d="M14 11v5a2 2 0 0 1-2 2H8"></path>
<path d="M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z"></path>"#;

const LAMP: &str = r#"
<path d="M8 2h8l4 10H4L8 2Z"></path>
<path d="M12 12v6"></path>
<path d="M8 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H8Z"></path>"#;

const LANDMARK: &str = r#"
<line x1="3" x2="21" y2="22" y1="22"></line>
<line x2="6" x1="6" y2="11" y1="18"></line>
<line x2="10" y1="18" y2="11" x1="10"></line>
<line x1="14" x2="14" y1="18" y2="11"></line>
<line y1="18" x1="18" y2="11" x2="18"></line>
<polygon points="12 2 20 7 4 7"></polygon>"#;

const LANGUAGES: &str = r#"
<path d="m5 8 6 6"></path>
<path d="m4 14 6-6 2-3"></path>
<path d="M2 5h12"></path>
<path d="M7 2h1"></path>
<path d="m22 22-5-10-5 10"></path>
<path d="M14 18h6"></path>"#;

const LAPTOP_2: &str = r#"
<rect width="18" x="3" y="4" rx="2" ry="2" height="12"></rect>
<line y1="20" y2="20" x2="22" x1="2"></line>"#;

const LAPTOP: &str = r#"
<path d="M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16"></path>"#;

const LASSO_SELECT: &str = r#"
<path d="M7 22a5 5 0 0 1-2-4"></path>
<path d="M7 16.93c.96.43 1.96.74 2.99.91"></path>
<path d="M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2"></path>
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"></path>
<path d="M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14v0z"></path>"#;

const LASSO: &str = r#"
<path d="M7 22a5 5 0 0 1-2-4"></path>
<path d="M3.3 14A6.8 6.8 0 0 1 2 10c0-4.4 4.5-8 10-8s10 3.6 10 8-4.5 8-10 8a12 12 0 0 1-5-1"></path>
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"></path>"#;

const LAUGH: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z"></path>
<line y1="9" x1="9" y2="9" x2="9.01"></line>
<line y2="9" x1="15" y1="9" x2="15.01"></line>"#;

const LAYERS: &str = r#"
<polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
<polyline points="2 17 12 22 22 17"></polyline>
<polyline points="2 12 12 17 22 12"></polyline>"#;

const LAYOUT_DASHBOARD: &str = r#"
<rect height="9" width="7" y="3" x="3" rx="1"></rect>
<rect width="7" rx="1" height="5" x="14" y="3"></rect>
<rect x="14" width="7" height="9" y="12" rx="1"></rect>
<rect x="3" height="5" rx="1" y="16" width="7"></rect>"#;

const LAYOUT_GRID: &str = r#"
<rect x="3" rx="1" y="3" width="7" height="7"></rect>
<rect rx="1" height="7" y="3" width="7" x="14"></rect>
<rect x="14" y="14" rx="1" width="7" height="7"></rect>
<rect x="3" width="7" rx="1" height="7" y="14"></rect>"#;

const LAYOUT_LIST: &str = r#"
<rect width="7" x="3" rx="1" y="3" height="7"></rect>
<rect rx="1" y="14" x="3" width="7" height="7"></rect>
<path d="M14 4h7"></path>
<path d="M14 9h7"></path>
<path d="M14 15h7"></path>
<path d="M14 20h7"></path>"#;

const LAYOUT_PANEL_LEFT: &str = r#"
<rect height="18" x="3" y="3" width="7" rx="1"></rect>
<rect x="14" width="7" height="7" y="3" rx="1"></rect>
<rect y="14" x="14" rx="1" width="7" height="7"></rect>"#;

const LAYOUT_PANEL_TOP: &str = r#"
<rect x="3" rx="1" width="18" height="7" y="3"></rect>
<rect width="7" y="14" height="7" x="3" rx="1"></rect>
<rect height="7" x="14" width="7" rx="1" y="14"></rect>"#;

const LAYOUT_TEMPLATE: &str = r#"
<rect rx="1" height="7" x="3" y="3" width="18"></rect>
<rect x="3" y="14" rx="1" height="7" width="9"></rect>
<rect y="14" height="7" x="16" width="5" rx="1"></rect>"#;

const LAYOUT: &str = r#"
<rect height="18" width="18" x="3" y="3" rx="2" ry="2"></rect>
<line y2="9" y1="9" x1="3" x2="21"></line>
<line x2="9" x1="9" y2="9" y1="21"></line>"#;

const LEAF: &str = r#"
<path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z"></path>
<path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12"></path>"#;

const LEAFY_GREEN: &str = r#"
<path d="M2 22c1.25-.987 2.27-1.975 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.187-2.353 3.5 3.5 0 0 0 3.69-5.116A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.831 1.373 3.5 3.5 0 0 0-5.116 3.69 4 4 0 0 0-2.348 6.155C3.499 15.42 4.409 16.712 4.2 18.1 3.926 19.743 3.014 20.732 2 22"></path>
<path d="M2 22 17 7"></path>"#;

const LIBRARY: &str = r#"
<path d="m16 6 4 14"></path>
<path d="M12 6v14"></path>
<path d="M8 8v12"></path>
<path d="M4 4v16"></path>"#;

const LIFE_BUOY: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m4.93 4.93 4.24 4.24"></path>
<path d="m14.83 9.17 4.24-4.24"></path>
<path d="m14.83 14.83 4.24 4.24"></path>
<path d="m9.17 14.83-4.24 4.24"></path>
<circle cy="12" r="4" cx="12"></circle>"#;

const LIGATURE: &str = r#"
<path d="M8 20V8c0-2.2 1.8-4 4-4 1.5 0 2.8.8 3.5 2"></path>
<path d="M6 12h4"></path>
<path d="M14 12h2v8"></path>
<path d="M6 20h4"></path>
<path d="M14 20h4"></path>"#;

const LIGHTBULB_OFF: &str = r#"
<path d="M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5"></path>
<path d="m2 2 20 20"></path>
<path d="M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5"></path>
<path d="M9 18h6"></path>
<path d="M10 22h4"></path>"#;

const LIGHTBULB: &str = r#"
<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"></path>
<path d="M9 18h6"></path>
<path d="M10 22h4"></path>"#;

const LINE_CHART: &str = r#"
<path d="M3 3v18h18"></path>
<path d="m19 9-5 5-4-4-3 3"></path>"#;

const LINK_2_OFF: &str = r#"
<path d="M9 17H7A5 5 0 0 1 7 7"></path>
<path d="M15 7h2a5 5 0 0 1 4 8"></path>
<line x2="12" x1="8" y1="12" y2="12"></line>
<line x1="2" x2="22" y2="22" y1="2"></line>"#;

const LINK_2: &str = r#"
<path d="M9 17H7A5 5 0 0 1 7 7h2"></path>
<path d="M15 7h2a5 5 0 1 1 0 10h-2"></path>
<line x1="8" x2="16" y2="12" y1="12"></line>"#;

const LINK: &str = r#"
<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>"#;

const LINKEDIN: &str = r#"
<path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
<rect y="9" x="2" height="12" width="4"></rect>
<circle cy="4" r="2" cx="4"></circle>"#;

const LIST_CHECKS: &str = r#"
<path d="m3 17 2 2 4-4"></path>
<path d="m3 7 2 2 4-4"></path>
<path d="M13 6h8"></path>
<path d="M13 12h8"></path>
<path d="M13 18h8"></path>"#;

const LIST_END: &str = r#"
<path d="M16 12H3"></path>
<path d="M16 6H3"></path>
<path d="M10 18H3"></path>
<path d="M21 6v10a2 2 0 0 1-2 2h-5"></path>
<path d="m16 16-2 2 2 2"></path>"#;

const LIST_FILTER: &str = r#"
<path d="M3 6h18"></path>
<path d="M7 12h10"></path>
<path d="M10 18h4"></path>"#;

const LIST_MINUS: &str = r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="M21 12h-6"></path>"#;

const LIST_MUSIC: &str = r#"
<path d="M21 15V6"></path>
<path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
<path d="M12 12H3"></path>
<path d="M16 6H3"></path>
<path d="M12 18H3"></path>"#;

const LIST_ORDERED: &str = r#"
<line y2="6" y1="6" x1="10" x2="21"></line>
<line x2="21" y1="12" y2="12" x1="10"></line>
<line y2="18" x1="10" x2="21" y1="18"></line>
<path d="M4 6h1v4"></path>
<path d="M4 10h2"></path>
<path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>"#;

const LIST_PLUS: &str = r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="M18 9v6"></path>
<path d="M21 12h-6"></path>"#;

const LIST_RESTART: &str = r#"
<path d="M21 6H3"></path>
<path d="M7 12H3"></path>
<path d="M7 18H3"></path>
<path d="M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14"></path>
<path d="M11 10v4h4"></path>"#;

const LIST_START: &str = r#"
<path d="M16 12H3"></path>
<path d="M16 18H3"></path>
<path d="M10 6H3"></path>
<path d="M21 18V8a2 2 0 0 0-2-2h-5"></path>
<path d="m16 8-2-2 2-2"></path>"#;

const LIST_TODO: &str = r#"
<rect width="6" height="6" rx="1" y="5" x="3"></rect>
<path d="m3 17 2 2 4-4"></path>
<path d="M13 6h8"></path>
<path d="M13 12h8"></path>
<path d="M13 18h8"></path>"#;

const LIST_TREE: &str = r#"
<path d="M21 12h-8"></path>
<path d="M21 6H8"></path>
<path d="M21 18h-8"></path>
<path d="M3 6v4c0 1.1.9 2 2 2h3"></path>
<path d="M3 10v6c0 1.1.9 2 2 2h3"></path>"#;

const LIST_VIDEO: &str = r#"
<path d="M12 12H3"></path>
<path d="M16 6H3"></path>
<path d="M12 18H3"></path>
<path d="m16 12 5 3-5 3v-6Z"></path>"#;

const LIST_X: &str = r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="m19 10-4 4"></path>
<path d="m15 10 4 4"></path>"#;

const LIST: &str = r#"
<line y2="6" x1="8" x2="21" y1="6"></line>
<line x1="8" y1="12" y2="12" x2="21"></line>
<line y1="18" y2="18" x2="21" x1="8"></line>
<line y2="6" y1="6" x2="3.01" x1="3"></line>
<line x2="3.01" x1="3" y1="12" y2="12"></line>
<line y1="18" x1="3" x2="3.01" y2="18"></line>"#;

const LOADER_2: &str = r#"
<path d="M21 12a9 9 0 1 1-6.219-8.56"></path>"#;

const LOADER: &str = r#"
<line x2="12" y2="6" x1="12" y1="2"></line>
<line y2="22" x1="12" x2="12" y1="18"></line>
<line x2="7.76" y1="4.93" y2="7.76" x1="4.93"></line>
<line y1="16.24" y2="19.07" x1="16.24" x2="19.07"></line>
<line y1="12" x2="6" y2="12" x1="2"></line>
<line y1="12" y2="12" x1="18" x2="22"></line>
<line y1="19.07" x2="7.76" x1="4.93" y2="16.24"></line>
<line x2="19.07" y1="7.76" y2="4.93" x1="16.24"></line>"#;

const LOCATE_FIXED: &str = r#"
<line x1="2" x2="5" y1="12" y2="12"></line>
<line y1="12" y2="12" x1="19" x2="22"></line>
<line x2="12" y1="2" x1="12" y2="5"></line>
<line x2="12" y1="19" x1="12" y2="22"></line>
<circle r="7" cx="12" cy="12"></circle>
<circle cx="12" cy="12" r="3"></circle>"#;

const LOCATE_OFF: &str = r#"
<line y1="12" x1="2" y2="12" x2="5"></line>
<line x2="22" y1="12" y2="12" x1="19"></line>
<line x1="12" y2="5" x2="12" y1="2"></line>
<line x2="12" x1="12" y2="22" y1="19"></line>
<path d="M7.11 7.11C5.83 8.39 5 10.1 5 12c0 3.87 3.13 7 7 7 1.9 0 3.61-.83 4.89-2.11"></path>
<path d="M18.71 13.96c.19-.63.29-1.29.29-1.96 0-3.87-3.13-7-7-7-.67 0-1.33.1-1.96.29"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#;

const LOCATE: &str = r#"
<line x2="5" y1="12" x1="2" y2="12"></line>
<line x2="22" y1="12" x1="19" y2="12"></line>
<line y2="5" y1="2" x1="12" x2="12"></line>
<line y2="22" x2="12" x1="12" y1="19"></line>
<circle r="7" cx="12" cy="12"></circle>"#;

const LOCK: &str = r#"
<rect height="11" x="3" y="11" rx="2" width="18" ry="2"></rect>
<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>"#;

const LOG_IN: &str = r#"
<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
<polyline points="10 17 15 12 10 7"></polyline>
<line x1="15" x2="3" y2="12" y1="12"></line>"#;

const LOG_OUT: &str = r#"
<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
<polyline points="16 17 21 12 16 7"></polyline>
<line x1="21" y1="12" x2="9" y2="12"></line>"#;

const LOLLIPOP: &str = r#"
<circle cx="11" cy="11" r="8"></circle>
<path d="m21 21-4.3-4.3"></path>
<path d="M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0"></path>"#;

const LUGGAGE: &str = r#"
<path d="M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0"></path>
<path d="M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14"></path>
<path d="M10 20h4"></path>
<circle r="2" cx="16" cy="20"></circle>
<circle cx="8" r="2" cy="20"></circle>"#;

const M_SQUARE: &str = r#"
<rect height="18" x="3" y="3" rx="2" width="18"></rect>
<path d="M8 16V8l4 4 4-4v8"></path>"#;

const MAGNET: &str = r#"
<path d="m6 15-4-4 6.75-6.77a7.79 7.79 0 0 1 11 11L13 22l-4-4 6.39-6.36a2.14 2.14 0 0 0-3-3L6 15"></path>
<path d="m5 8 4 4"></path>
<path d="m12 15 4 4"></path>"#;

const MAIL_CHECK: &str = r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="m16 19 2 2 4-4"></path>"#;

const MAIL_MINUS: &str = r#"
<path d="M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M16 19h6"></path>"#;

const MAIL_OPEN: &str = r#"
<path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z"></path>
<path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10"></path>"#;

const MAIL_PLUS: &str = r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M19 16v6"></path>
<path d="M16 19h6"></path>"#;

const MAIL_QUESTION: &str = r#"
<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"></path>
<path d="M20 22v.01"></path>"#;

const MAIL_SEARCH: &str = r#"
<path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z"></path>
<circle cx="18" cy="18" r="3"></circle>
<path d="m22 22-1.5-1.5"></path>"#;

const MAIL_WARNING: &str = r#"
<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M20 14v4"></path>
<path d="M20 22v.01"></path>"#;

const MAIL_X: &str = r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h9"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="m17 17 4 4"></path>
<path d="m21 17-4 4"></path>"#;

const MAIL: &str = r#"
<rect y="4" width="20" height="16" x="2" rx="2"></rect>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>"#;

const MAILBOX: &str = r#"
<path d="M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z"></path>
<polyline points="15,9 18,9 18,11"></polyline>
<path d="M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2v0"></path>
<line y2="10" x1="6" x2="7" y1="10"></line>"#;

const MAILS: &str = r#"
<rect x="6" width="16" height="13" rx="2" y="4"></rect>
<path d="m22 7-7.1 3.78c-.57.3-1.23.3-1.8 0L6 7"></path>
<path d="M2 8v11c0 1.1.9 2 2 2h14"></path>"#;

const MAP_PIN_OFF: &str = r#"
<path d="M5.43 5.43A8.06 8.06 0 0 0 4 10c0 6 8 12 8 12a29.94 29.94 0 0 0 5-5"></path>
<path d="M19.18 13.52A8.66 8.66 0 0 0 20 10a8 8 0 0 0-8-8 7.88 7.88 0 0 0-3.52.82"></path>
<path d="M9.13 9.13A2.78 2.78 0 0 0 9 10a3 3 0 0 0 3 3 2.78 2.78 0 0 0 .87-.13"></path>
<path d="M14.9 9.25a3 3 0 0 0-2.15-2.16"></path>
<line y1="2" x2="22" y2="22" x1="2"></line>"#;

const MAP_PIN: &str = r#"
<path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"></path>
<circle cx="12" cy="10" r="3"></circle>"#;

const MAP: &str = r#"
<polygon points="3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21"></polygon>
<line y1="3" y2="18" x1="9" x2="9"></line>
<line x2="15" y2="21" x1="15" y1="6"></line>"#;

const MARTINI: &str = r#"
<path d="M8 22h8"></path>
<path d="M12 11v11"></path>
<path d="m19 3-7 8-7-8Z"></path>"#;

const MAXIMIZE_2: &str = r#"
<polyline points="15 3 21 3 21 9"></polyline>
<polyline points="9 21 3 21 3 15"></polyline>
<line x1="21" y2="10" x2="14" y1="3"></line>
<line y2="14" x1="3" x2="10" y1="21"></line>"#;

const MAXIMIZE: &str = r#"
<path d="M8 3H5a2 2 0 0 0-2 2v3"></path>
<path d="M21 8V5a2 2 0 0 0-2-2h-3"></path>
<path d="M3 16v3a2 2 0 0 0 2 2h3"></path>
<path d="M16 21h3a2 2 0 0 0 2-2v-3"></path>"#;

const MEDAL: &str = r#"
<path d="M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15"></path>
<path d="M11 12 5.12 2.2"></path>
<path d="m13 12 5.88-9.8"></path>
<path d="M8 7h8"></path>
<circle cx="12" cy="17" r="5"></circle>
<path d="M12 18v-2h-.5"></path>"#;

const MEGAPHONE_OFF: &str = r#"
<path d="M9.26 9.26 3 11v3l14.14 3.14"></path>
<path d="M21 15.34V6l-7.31 2.03"></path>
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"></path>
<line y2="22" x2="22" x1="2" y1="2"></line>"#;

const MEGAPHONE: &str = r#"
<path d="m3 11 18-5v12L3 14v-3z"></path>
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"></path>"#;

const MEH: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<line x1="8" y1="15" x2="16" y2="15"></line>
<line y1="9" y2="9" x1="9" x2="9.01"></line>
<line x2="15.01" y1="9" x1="15" y2="9"></line>"#;

const MEMORY_STICK: &str = r#"
<path d="M6 19v-3"></path>
<path d="M10 19v-3"></path>
<path d="M14 19v-3"></path>
<path d="M18 19v-3"></path>
<path d="M8 11V9"></path>
<path d="M16 11V9"></path>
<path d="M12 11V9"></path>
<path d="M2 15h20"></path>
<path d="M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z"></path>"#;

const MENU_SQUARE: &str = r#"
<rect rx="2" height="18" x="3" width="18" y="3"></rect>
<path d="M7 8h10"></path>
<path d="M7 12h10"></path>
<path d="M7 16h10"></path>"#;

const MENU: &str = r#"
<line y2="12" x1="4" x2="20" y1="12"></line>
<line y1="6" y2="6" x1="4" x2="20"></line>
<line y2="18" x2="20" x1="4" y1="18"></line>"#;

const MERGE: &str = r#"
<path d="m8 6 4-4 4 4"></path>
<path d="M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22"></path>
<path d="m20 22-5-5"></path>"#;

const MESSAGE_CIRCLE: &str = r#"
<path d="m3 21 1.9-5.7a8.5 8.5 0 1 1 3.8 3.8z"></path>"#;

const MESSAGE_SQUARE_DASHED: &str = r#"
<path d="M3 6V5c0-1.1.9-2 2-2h2"></path>
<path d="M11 3h3"></path>
<path d="M18 3h1c1.1 0 2 .9 2 2"></path>
<path d="M21 9v2"></path>
<path d="M21 15c0 1.1-.9 2-2 2h-1"></path>
<path d="M14 17h-3"></path>
<path d="m7 17-4 4v-5"></path>
<path d="M3 12v-2"></path>"#;

const MESSAGE_SQUARE_PLUS: &str = r#"
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
<line x2="15" x1="9" y1="10" y2="10"></line>
<line y2="13" x1="12" y1="7" x2="12"></line>"#;

const MESSAGE_SQUARE: &str = r#"
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>"#;

const MESSAGES_SQUARE: &str = r#"
<path d="M14 9a2 2 0 0 1-2 2H6l-4 4V4c0-1.1.9-2 2-2h8a2 2 0 0 1 2 2v5Z"></path>
<path d="M18 9h2a2 2 0 0 1 2 2v11l-4-4h-6a2 2 0 0 1-2-2v-1"></path>"#;

const MIC_2: &str = r#"
<path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12"></path>
<circle cx="17" r="5" cy="7"></circle>"#;

const MIC_OFF: &str = r#"
<line y2="22" x2="22" y1="2" x1="2"></line>
<path d="M18.89 13.23A7.12 7.12 0 0 0 19 12v-2"></path>
<path d="M5 10v2a7 7 0 0 0 12 5"></path>
<path d="M15 9.34V5a3 3 0 0 0-5.68-1.33"></path>
<path d="M9 9v3a3 3 0 0 0 5.12 2.12"></path>
<line x1="12" x2="12" y1="19" y2="22"></line>"#;

const MIC: &str = r#"
<path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"></path>
<path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
<line y1="19" x1="12" y2="22" x2="12"></line>"#;

const MICROSCOPE: &str = r#"
<path d="M6 18h8"></path>
<path d="M3 22h18"></path>
<path d="M14 22a7 7 0 1 0 0-14h-1"></path>
<path d="M9 14h2"></path>
<path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z"></path>
<path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"></path>"#;

const MICROWAVE: &str = r#"
<rect width="20" y="4" height="15" x="2" rx="2"></rect>
<rect width="8" height="7" x="6" y="8" rx="1"></rect>
<path d="M18 8v7"></path>
<path d="M6 19v2"></path>
<path d="M18 19v2"></path>"#;

const MILESTONE: &str = r#"
<path d="M18 6H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h13l4-3.5L18 6Z"></path>
<path d="M12 13v8"></path>
<path d="M12 3v3"></path>"#;

const MILK_OFF: &str = r#"
<path d="M8 2h8"></path>
<path d="M9 2v1.343M15 2v2.789a4 4 0 0 0 .672 2.219l.656.984a4 4 0 0 1 .672 2.22v1.131M7.8 7.8l-.128.192A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3"></path>
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.472 6.472 0 0 0 3.435.435"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#;

const MILK: &str = r#"
<path d="M8 2h8"></path>
<path d="M9 2v2.789a4 4 0 0 1-.672 2.219l-.656.984A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.789a4 4 0 0 0-.672-2.219l-.656-.984A4 4 0 0 1 15 4.788V2"></path>
<path d="M7 15a6.472 6.472 0 0 1 5 0 6.47 6.47 0 0 0 5 0"></path>"#;

const MINIMIZE_2: &str = r#"
<polyline points="4 14 10 14 10 20"></polyline>
<polyline points="20 10 14 10 14 4"></polyline>
<line y1="10" x2="21" x1="14" y2="3"></line>
<line y1="21" x2="10" y2="14" x1="3"></line>"#;

const MINIMIZE: &str = r#"
<path d="M8 3v3a2 2 0 0 1-2 2H3"></path>
<path d="M21 8h-3a2 2 0 0 1-2-2V3"></path>
<path d="M3 16h3a2 2 0 0 1 2 2v3"></path>
<path d="M16 21v-3a2 2 0 0 1 2-2h3"></path>"#;

const MINUS_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M8 12h8"></path>"#;

const MINUS_SQUARE: &str = r#"
<rect width="18" x="3" height="18" y="3" rx="2"></rect>
<path d="M8 12h8"></path>"#;

const MINUS: &str = r#"
<path d="M5 12h14"></path>"#;

const MONITOR_CHECK: &str = r#"
<path d="m9 10 2 2 4-4"></path>
<rect x="2" width="20" y="3" rx="2" height="14"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_DOT: &str = r#"
<circle cy="6" r="3" cx="19"></circle>
<path d="M22 12v3a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9"></path>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_DOWN: &str = r#"
<path d="M12 13V7"></path>
<path d="m15 10-3 3-3-3"></path>
<rect y="3" height="14" rx="2" x="2" width="20"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_OFF: &str = r#"
<path d="M17 17H4a2 2 0 0 1-2-2V5c0-1.5 1-2 1-2"></path>
<path d="M22 15V5a2 2 0 0 0-2-2H9"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m2 2 20 20"></path>"#;

const MONITOR_PAUSE: &str = r#"
<path d="M10 13V7"></path>
<path d="M14 13V7"></path>
<rect x="2" y="3" rx="2" width="20" height="14"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_PLAY: &str = r#"
<path d="m10 7 5 3-5 3Z"></path>
<rect rx="2" height="14" width="20" x="2" y="3"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_SMARTPHONE: &str = r#"
<path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8"></path>
<path d="M10 19v-3.96 3.15"></path>
<path d="M7 19h5"></path>
<rect x="16" height="10" width="6" y="12" rx="2"></rect>"#;

const MONITOR_SPEAKER: &str = r#"
<path d="M5.5 20H8"></path>
<path d="M17 9h.01"></path>
<rect width="10" y="4" rx="2" x="12" height="16"></rect>
<path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4"></path>
<circle r="1" cx="17" cy="15"></circle>"#;

const MONITOR_STOP: &str = r#"
<rect y="7" width="6" x="9" height="6"></rect>
<rect width="20" height="14" rx="2" x="2" y="3"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_UP: &str = r#"
<path d="m9 10 3-3 3 3"></path>
<path d="M12 13V7"></path>
<rect x="2" y="3" width="20" rx="2" height="14"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR_X: &str = r#"
<path d="m14.5 12.5-5-5"></path>
<path d="m9.5 12.5 5-5"></path>
<rect rx="2" height="14" x="2" width="20" y="3"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#;

const MONITOR: &str = r#"
<rect y="3" width="20" rx="2" height="14" x="2"></rect>
<line x1="8" y1="21" x2="16" y2="21"></line>
<line x1="12" x2="12" y2="21" y1="17"></line>"#;

const MOON_STAR: &str = r#"
<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
<path d="M19 3v4"></path>
<path d="M21 5h-4"></path>"#;

const MOON: &str = r#"
<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>"#;

const MORE_HORIZONTAL: &str = r#"
<circle r="1" cy="12" cx="12"></circle>
<circle cx="19" cy="12" r="1"></circle>
<circle r="1" cx="5" cy="12"></circle>"#;

const MORE_VERTICAL: &str = r#"
<circle cx="12" cy="12" r="1"></circle>
<circle cy="5" cx="12" r="1"></circle>
<circle r="1" cx="12" cy="19"></circle>"#;

const MOUNTAIN_SNOW: &str = r#"
<path d="m8 3 4 8 5-5 5 15H2L8 3z"></path>
<path d="M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19"></path>"#;

const MOUNTAIN: &str = r#"
<path d="m8 3 4 8 5-5 5 15H2L8 3z"></path>"#;

const MOUSE_POINTER_2: &str = r#"
<path d="m4 4 7.07 17 2.51-7.39L21 11.07z"></path>"#;

const MOUSE_POINTER_CLICK: &str = r#"
<path d="m9 9 5 12 1.774-5.226L21 14 9 9z"></path>
<path d="m16.071 16.071 4.243 4.243"></path>
<path d="m7.188 2.239.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656-2.12 2.122"></path>"#;

const MOUSE_POINTER_SQUARE_DASHED: &str = r#"
<path d="M5 3a2 2 0 0 0-2 2"></path>
<path d="M19 3a2 2 0 0 1 2 2"></path>
<path d="m12 12 4 10 1.7-4.3L22 16Z"></path>
<path d="M5 21a2 2 0 0 1-2-2"></path>
<path d="M9 3h1"></path>
<path d="M9 21h2"></path>
<path d="M14 3h1"></path>
<path d="M3 9v1"></path>
<path d="M21 9v2"></path>
<path d="M3 14v1"></path>"#;

const MOUSE_POINTER_SQUARE: &str = r#"
<path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"></path>
<path d="m12 12 4 10 1.7-4.3L22 16Z"></path>"#;

const MOUSE_POINTER: &str = r#"
<path d="m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z"></path>
<path d="m13 13 6 6"></path>"#;

const MOUSE: &str = r#"
<rect x="5" height="20" rx="7" y="2" width="14"></rect>
<path d="M12 6v4"></path>"#;

const MOVE_3_D: &str = r#"
<path d="M5 3v16h16"></path>
<path d="m5 19 6-6"></path>
<path d="m2 6 3-3 3 3"></path>
<path d="m18 16 3 3-3 3"></path>"#;

const MOVE_DIAGONAL_2: &str = r#"
<polyline points="5 11 5 5 11 5"></polyline>
<polyline points="19 13 19 19 13 19"></polyline>
<line x1="5" y1="5" x2="19" y2="19"></line>"#;

const MOVE_DIAGONAL: &str = r#"
<polyline points="13 5 19 5 19 11"></polyline>
<polyline points="11 19 5 19 5 13"></polyline>
<line x2="5" x1="19" y2="19" y1="5"></line>"#;

const MOVE_DOWN_LEFT: &str = r#"
<path d="M11 19H5V13"></path>
<path d="M19 5L5 19"></path>"#;

const MOVE_DOWN_RIGHT: &str = r#"
<path d="M19 13V19H13"></path>
<path d="M5 5L19 19"></path>"#;

const MOVE_DOWN: &str = r#"
<path d="M8 18L12 22L16 18"></path>
<path d="M12 2V22"></path>"#;

const MOVE_HORIZONTAL: &str = r#"
<polyline points="18 8 22 12 18 16"></polyline>
<polyline points="6 8 2 12 6 16"></polyline>
<line y1="12" y2="12" x2="22" x1="2"></line>"#;

const MOVE_LEFT: &str = r#"
<path d="M6 8L2 12L6 16"></path>
<path d="M2 12H22"></path>"#;

const MOVE_RIGHT: &str = r#"
<path d="M18 8L22 12L18 16"></path>
<path d="M2 12H22"></path>"#;

const MOVE_UP_LEFT: &str = r#"
<path d="M5 11V5H11"></path>
<path d="M5 5L19 19"></path>"#;

const MOVE_UP_RIGHT: &str = r#"
<path d="M13 5H19V11"></path>
<path d="M19 5L5 19"></path>"#;

const MOVE_UP: &str = r#"
<path d="M8 6L12 2L16 6"></path>
<path d="M12 2V22"></path>"#;

const MOVE_VERTICAL: &str = r#"
<polyline points="8 18 12 22 16 18"></polyline>
<polyline points="8 6 12 2 16 6"></polyline>
<line y1="2" x1="12" y2="22" x2="12"></line>"#;

const MOVE: &str = r#"
<polyline points="5 9 2 12 5 15"></polyline>
<polyline points="9 5 12 2 15 5"></polyline>
<polyline points="15 19 12 22 9 19"></polyline>
<polyline points="19 9 22 12 19 15"></polyline>
<line x1="2" y2="12" y1="12" x2="22"></line>
<line y2="22" x2="12" x1="12" y1="2"></line>"#;

const MUSIC_2: &str = r#"
<circle cy="18" cx="8" r="4"></circle>
<path d="M12 18V2l7 4"></path>"#;

const MUSIC_3: &str = r#"
<circle cx="12" cy="18" r="4"></circle>
<path d="M16 18V2"></path>"#;

const MUSIC_4: &str = r#"
<path d="M9 18V5l12-2v13"></path>
<path d="m9 9 12-2"></path>
<circle r="3" cx="6" cy="18"></circle>
<circle r="3" cx="18" cy="16"></circle>"#;

const MUSIC: &str = r#"
<path d="M9 18V5l12-2v13"></path>
<circle cx="6" cy="18" r="3"></circle>
<circle cy="16" r="3" cx="18"></circle>"#;

const NAVIGATION_2_OFF: &str = r#"
<path d="M9.31 9.31 5 21l7-4 7 4-1.17-3.17"></path>
<path d="M14.53 8.88 12 2l-1.17 3.17"></path>
<line y1="2" x1="2" x2="22" y2="22"></line>"#;

const NAVIGATION_2: &str = r#"
<polygon points="12 2 19 21 12 17 5 21 12 2"></polygon>"#;

const NAVIGATION_OFF: &str = r#"
<path d="M8.43 8.43 3 11l8 2 2 8 2.57-5.43"></path>
<path d="M17.39 11.73 22 2l-9.73 4.61"></path>
<line y2="22" x1="2" x2="22" y1="2"></line>"#;

const NAVIGATION: &str = r#"
<polygon points="3 11 22 2 13 21 11 13 3 11"></polygon>"#;

const NETWORK: &str = r#"
<rect height="6" x="16" y="16" rx="1" width="6"></rect>
<rect y="16" rx="1" width="6" x="2" height="6"></rect>
<rect x="9" width="6" rx="1" y="2" height="6"></rect>
<path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"></path>
<path d="M12 12V8"></path>"#;

const NEWSPAPER: &str = r#"
<path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2"></path>
<path d="M18 14h-8"></path>
<path d="M15 18h-5"></path>
<path d="M10 6h8v4h-8V6Z"></path>"#;

const NFC: &str = r#"
<path d="M6 8.32a7.43 7.43 0 0 1 0 7.36"></path>
<path d="M9.46 6.21a11.76 11.76 0 0 1 0 11.58"></path>
<path d="M12.91 4.1a15.91 15.91 0 0 1 .01 15.8"></path>
<path d="M16.37 2a20.16 20.16 0 0 1 0 20"></path>"#;

const NUT_OFF: &str = r#"
<path d="M12 4V2"></path>
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592a7.01 7.01 0 0 0 4.125-2.939"></path>
<path d="M19 10v3.343"></path>
<path d="M12 12c-1.349-.573-1.905-1.005-2.5-2-.546.902-1.048 1.353-2.5 2-1.018-.644-1.46-1.08-2-2-1.028.71-1.69.918-3 1 1.081-1.048 1.757-2.03 2-3 .194-.776.84-1.551 1.79-2.21m11.654 5.997c.887-.457 1.28-.891 1.556-1.787 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4-.74 0-1.461.068-2.15.192"></path>
<line x1="2" x2="22" y1="2" y2="22"></line>"#;

const NUT: &str = r#"
<path d="M12 4V2"></path>
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592A7.003 7.003 0 0 0 19 14v-4"></path>
<path d="M12 4C8 4 4.5 6 4 8c-.243.97-.919 1.952-2 3 1.31-.082 1.972-.29 3-1 .54.92.982 1.356 2 2 1.452-.647 1.954-1.098 2.5-2 .595.995 1.151 1.427 2.5 2 1.31-.621 1.862-1.058 2.5-2 .629.977 1.162 1.423 2.5 2 1.209-.548 1.68-.967 2-2 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4Z"></path>"#;

const OCTAGON: &str = r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>"#;

const OPTION: &str = r#"
<path d="M3 3h6l6 18h6"></path>
<path d="M14 3h7"></path>"#;

const ORBIT: &str = r#"
<circle cx="12" cy="12" r="3"></circle>
<circle cx="19" cy="5" r="2"></circle>
<circle cx="5" cy="19" r="2"></circle>
<path d="M10.4 21.9a10 10 0 0 0 9.941-15.416"></path>
<path d="M13.5 2.1a10 10 0 0 0-9.841 15.416"></path>"#;

const OUTDENT: &str = r#"
<polyline points="7 8 3 12 7 16"></polyline>
<line y2="12" y1="12" x2="11" x1="21"></line>
<line y2="6" x1="21" y1="6" x2="11"></line>
<line y1="18" x2="11" y2="18" x1="21"></line>"#;

const PACKAGE_2: &str = r#"
<path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z"></path>
<path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9"></path>
<path d="M12 3v6"></path>"#;

const PACKAGE_CHECK: &str = r#"
<path d="m16 16 2 2 4-4"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y1="22" x1="12" x2="12" y2="12"></line>"#;

const PACKAGE_MINUS: &str = r#"
<path d="M16 16h6"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line x1="12" x2="12" y1="22" y2="12"></line>"#;

const PACKAGE_OPEN: &str = r#"
<path d="M20.91 8.84 8.56 2.23a1.93 1.93 0 0 0-1.81 0L3.1 4.13a2.12 2.12 0 0 0-.05 3.69l12.22 6.93a2 2 0 0 0 1.94 0L21 12.51a2.12 2.12 0 0 0-.09-3.67Z"></path>
<path d="m3.09 8.84 12.35-6.61a1.93 1.93 0 0 1 1.81 0l3.65 1.9a2.12 2.12 0 0 1 .1 3.69L8.73 14.75a2 2 0 0 1-1.94 0L3 12.51a2.12 2.12 0 0 1 .09-3.67Z"></path>
<line x1="12" y2="13" y1="22" x2="12"></line>
<path d="M20 13.5v3.37a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13.5"></path>"#;

const PACKAGE_PLUS: &str = r#"
<path d="M16 16h6"></path>
<path d="M19 13v6"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y2="12" x1="12" x2="12" y1="22"></line>"#;

const PACKAGE_SEARCH: &str = r#"
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y1="22" x2="12" y2="12" x1="12"></line>
<circle cx="18.5" cy="15.5" r="2.5"></circle>
<path d="M20.27 17.27 22 19"></path>"#;

const PACKAGE_X: &str = r#"
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y2="12" x1="12" x2="12" y1="22"></line>
<path d="m17 13 5 5m-5 0 5-5"></path>"#;

const PACKAGE: &str = r#"
<path d="m7.5 4.27 9 5.15"></path>
<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"></path>
<path d="m3.3 7 8.7 5 8.7-5"></path>
<path d="M12 22V12"></path>"#;

const PAINT_BUCKET: &str = r#"
<path d="m19 11-8-8-8.6 8.6a2 2 0 0 0 0 2.8l5.2 5.2c.8.8 2 .8 2.8 0L19 11Z"></path>
<path d="m5 2 5 5"></path>
<path d="M2 13h15"></path>
<path d="M22 20a2 2 0 1 1-4 0c0-1.6 1.7-2.4 2-4 .3 1.6 2 2.4 2 4Z"></path>"#;

const PAINTBRUSH_2: &str = r#"
<path d="M14 19.9V16h3a2 2 0 0 0 2-2v-2H5v2c0 1.1.9 2 2 2h3v3.9a2 2 0 1 0 4 0Z"></path>
<path d="M6 12V2h12v10"></path>
<path d="M14 2v4"></path>
<path d="M10 2v2"></path>"#;

const PAINTBRUSH: &str = r#"
<path d="M18.37 2.63 14 7l-1.59-1.59a2 2 0 0 0-2.82 0L8 7l9 9 1.59-1.59a2 2 0 0 0 0-2.82L17 10l4.37-4.37a2.12 2.12 0 1 0-3-3Z"></path>
<path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7"></path>
<path d="M14.5 17.5 4.5 15"></path>"#;

const PALETTE: &str = r#"
<circle cy="6.5" r=".5" cx="13.5"></circle>
<circle cx="17.5" cy="10.5" r=".5"></circle>
<circle r=".5" cy="7.5" cx="8.5"></circle>
<circle cx="6.5" cy="12.5" r=".5"></circle>
<path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"></path>"#;

const PALMTREE: &str = r#"
<path d="M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4"></path>
<path d="M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3"></path>
<path d="M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35z"></path>
<path d="M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14"></path>"#;

const PANEL_BOTTOM_CLOSE: &str = r#"
<rect x="3" y="3" ry="2" rx="2" height="18" width="18"></rect>
<line y2="15" x2="21" y1="15" x1="3"></line>
<path d="m15 8-3 3-3-3"></path>"#;

const PANEL_BOTTOM_INACTIVE: &str = r#"
<rect height="18" x="3" width="18" y="3" rx="2"></rect>
<path d="M14 15h1"></path>
<path d="M19 15h2"></path>
<path d="M3 15h2"></path>
<path d="M9 15h1"></path>"#;

const PANEL_BOTTOM_OPEN: &str = r#"
<rect rx="2" ry="2" width="18" y="3" x="3" height="18"></rect>
<line x1="3" y1="15" x2="21" y2="15"></line>
<path d="m9 10 3-3 3 3"></path>"#;

const PANEL_BOTTOM: &str = r#"
<rect ry="2" x="3" rx="2" y="3" width="18" height="18"></rect>
<line y2="15" x1="3" x2="21" y1="15"></line>"#;

const PANEL_LEFT_CLOSE: &str = r#"
<rect width="18" y="3" rx="2" x="3" ry="2" height="18"></rect>
<path d="M9 3v18"></path>
<path d="m16 15-3-3 3-3"></path>"#;

const PANEL_LEFT_INACTIVE: &str = r#"
<rect y="3" width="18" height="18" rx="2" x="3"></rect>
<path d="M9 14v1"></path>
<path d="M9 19v2"></path>
<path d="M9 3v2"></path>
<path d="M9 9v1"></path>"#;

const PANEL_LEFT_OPEN: &str = r#"
<rect width="18" height="18" y="3" x="3" rx="2" ry="2"></rect>
<path d="M9 3v18"></path>
<path d="m14 9 3 3-3 3"></path>"#;

const PANEL_LEFT: &str = r#"
<rect y="3" rx="2" ry="2" width="18" height="18" x="3"></rect>
<line x2="9" y1="3" x1="9" y2="21"></line>"#;

const PANEL_RIGHT_CLOSE: &str = r#"
<rect height="18" rx="2" x="3" width="18" y="3" ry="2"></rect>
<line x1="15" x2="15" y1="3" y2="21"></line>
<path d="m8 9 3 3-3 3"></path>"#;

const PANEL_RIGHT_INACTIVE: &str = r#"
<rect x="3" height="18" y="3" rx="2" width="18"></rect>
<path d="M15 14v1"></path>
<path d="M15 19v2"></path>
<path d="M15 3v2"></path>
<path d="M15 9v1"></path>"#;

const PANEL_RIGHT_OPEN: &str = r#"
<rect height="18" x="3" y="3" ry="2" rx="2" width="18"></rect>
<line x2="15" y1="3" y2="21" x1="15"></line>
<path d="m10 15-3-3 3-3"></path>"#;

const PANEL_RIGHT: &str = r#"
<rect width="18" height="18" ry="2" y="3" rx="2" x="3"></rect>
<line x1="15" y2="21" x2="15" y1="3"></line>"#;

const PANEL_TOP_CLOSE: &str = r#"
<rect x="3" ry="2" rx="2" height="18" y="3" width="18"></rect>
<line y2="9" x1="3" y1="9" x2="21"></line>
<path d="m9 16 3-3 3 3"></path>"#;

const PANEL_TOP_INACTIVE: &str = r#"
<rect rx="2" x="3" y="3" width="18" height="18"></rect>
<path d="M14 9h1"></path>
<path d="M19 9h2"></path>
<path d="M3 9h2"></path>
<path d="M9 9h1"></path>"#;

const PANEL_TOP_OPEN: &str = r#"
<rect ry="2" width="18" height="18" rx="2" y="3" x="3"></rect>
<line y2="9" y1="9" x2="21" x1="3"></line>
<path d="m15 14-3 3-3-3"></path>"#;

const PANEL_TOP: &str = r#"
<rect rx="2" ry="2" width="18" height="18" y="3" x="3"></rect>
<line x2="21" y2="9" y1="9" x1="3"></line>"#;

const PAPERCLIP: &str = r#"
<path d="m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>"#;

const PARENTHESES: &str = r#"
<path d="M8 21s-4-3-4-9 4-9 4-9"></path>
<path d="M16 3s4 3 4 9-4 9-4 9"></path>"#;

const PARKING_CIRCLE_OFF: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="m5 5 14 14"></path>
<path d="M13 13a3 3 0 1 0 0-6H9v2"></path>
<path d="M9 17v-2.34"></path>"#;

const PARKING_CIRCLE: &str = r#"
<circle r="10" cy="12" cx="12"></circle>
<path d="M9 17V7h4a3 3 0 0 1 0 6H9"></path>"#;

const PARKING_METER: &str = r#"
<path d="M9 9a3 3 0 1 1 6 0"></path>
<path d="M12 12v3"></path>
<path d="M11 15h2"></path>
<path d="M19 9a7 7 0 1 0-13.6 2.3C6.4 14.4 8 19 8 19h8s1.6-4.6 2.6-7.7c.3-.8.4-1.5.4-2.3"></path>
<path d="M12 19v3"></path>"#;

const PARKING_SQUARE_OFF: &str = r#"
<path d="M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41"></path>
<path d="M3 8.7V19a2 2 0 0 0 2 2h10.3"></path>
<path d="m2 2 20 20"></path>
<path d="M13 13a3 3 0 1 0 0-6H9v2"></path>
<path d="M9 17v-2.3"></path>"#;

const PARKING_SQUARE: &str = r#"
<rect height="18" y="3" width="18" rx="2" x="3"></rect>
<path d="M9 17V7h4a3 3 0 0 1 0 6H9"></path>"#;

const PARTY_POPPER: &str = r#"
<path d="M5.8 11.3 2 22l10.7-3.79"></path>
<path d="M4 3h.01"></path>
<path d="M22 8h.01"></path>
<path d="M15 2h.01"></path>
<path d="M22 20h.01"></path>
<path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12v0c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10"></path>
<path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11v0c-.11.7-.72 1.22-1.43 1.22H17"></path>
<path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98v0C9.52 4.9 9 5.52 9 6.23V7"></path>
<path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z"></path>"#;

const PAUSE_CIRCLE: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<line x1="10" y1="15" x2="10" y2="9"></line>
<line x2="14" y1="15" y2="9" x1="14"></line>"#;

const PAUSE_OCTAGON: &str = r#"
<path d="M10 15V9"></path>
<path d="M14 15V9"></path>
<path d="M7.714 2h8.572L22 7.714v8.572L16.286 22H7.714L2 16.286V7.714L7.714 2z"></path>"#;

const PAUSE: &str = r#"
<rect width="4" y="4" height="16" x="6"></rect>
<rect x="14" width="4" height="16" y="4"></rect>"#;

const PAW_PRINT: &str = r#"
<circle cx="11" cy="4" r="2"></circle>
<circle cx="18" cy="8" r="2"></circle>
<circle r="2" cx="20" cy="16"></circle>
<path d="M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z"></path>"#;

const PC_CASE: &str = r#"
<rect width="14" x="5" height="20" y="2" rx="2"></rect>
<path d="M15 14h.01"></path>
<path d="M9 6h6"></path>
<path d="M9 10h6"></path>"#;

const PEN_LINE: &str = r#"
<path d="M12 20h9"></path>
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"></path>"#;

const PEN_SQUARE: &str = r#"
<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
<path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"></path>"#;

const PEN_TOOL: &str = r#"
<path d="m12 19 7-7 3 3-7 7-3-3z"></path>
<path d="m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path>
<path d="m2 2 7.586 7.586"></path>
<circle cx="11" r="2" cy="11"></circle>"#;

const PEN: &str = r#"
<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>"#;

const PENCIL_LINE: &str = r#"
<path d="M12 20h9"></path>
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"></path>
<path d="m15 5 3 3"></path>"#;

const PENCIL_RULER: &str = r#"
<path d="m15 5 4 4"></path>
<path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13"></path>
<path d="m8 6 2-2"></path>
<path d="m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z"></path>
<path d="m18 16 2-2"></path>
<path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17"></path>"#;

const PENCIL: &str = r#"
<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
<path d="m15 5 4 4"></path>"#;

const PERCENT_CIRCLE: &str = r#"
<circle cx="12" r="10" cy="12"></circle>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#;

const PERCENT_DIAMOND: &str = r#"
<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z"></path>
<path d="M9.2 9.2h.01"></path>
<path d="m14.5 9.5-5 5"></path>
<path d="M14.7 14.8h.01"></path>"#;

const PERCENT_SQUARE: &str = r#"
<rect y="3" x="3" height="18" rx="2" width="18"></rect>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#;

const PERCENT: &str = r#"
<line x1="19" x2="5" y1="5" y2="19"></line>
<circle cy="6.5" cx="6.5" r="2.5"></circle>
<circle r="2.5" cx="17.5" cy="17.5"></circle>"#;

const PERSON_STANDING: &str = r#"
<circle cx="12" r="1" cy="5"></circle>
<path d="m9 20 3-6 3 6"></path>
<path d="m6 8 6 2 6-2"></path>
<path d="M12 10v4"></path>"#;

const PHONE_CALL: &str = r#"
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
<path d="M14.05 2a9 9 0 0 1 8 7.94"></path>
<path d="M14.05 6A5 5 0 0 1 18 10"></path>"#;

const PHONE_FORWARDED: &str = r#"
<polyline points="18 2 22 6 18 10"></polyline>
<line x2="22" x1="14" y1="6" y2="6"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#;

const PHONE_INCOMING: &str = r#"
<polyline points="16 2 16 8 22 8"></polyline>
<line y1="2" x1="22" y2="8" x2="16"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#;

const PHONE_MISSED: &str = r#"
<line y1="2" y2="8" x2="16" x1="22"></line>
<line y1="2" x1="16" y2="8" x2="22"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#;

const PHONE_OFF: &str = r#"
<path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91"></path>
<line x2="2" y1="2" y2="22" x1="22"></line>"#;

const PHONE_OUTGOING: &str = r#"
<polyline points="22 8 22 2 16 2"></polyline>
<line y1="8" x1="16" x2="22" y2="2"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#;

const PHONE: &str = r#"
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#;

const PI_SQUARE: &str = r#"
<rect width="18" rx="2" x="3" height="18" y="3"></rect>
<path d="M7 7h10"></path>
<path d="M10 7v10"></path>
<path d="M16 17a2 2 0 0 1-2-2V7"></path>"#;

const PI: &str = r#"
<line x2="9" y1="4" y2="20" x1="9"></line>
<path d="M4 7c0-1.7 1.3-3 3-3h13"></path>
<path d="M18 20c-1.7 0-3-1.3-3-3V4"></path>"#;

const PICTURE_IN_PICTURE_2: &str = r#"
<path d="M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4"></path>
<rect x="12" width="10" y="13" rx="2" height="7"></rect>"#;

const PICTURE_IN_PICTURE: &str = r#"
<path d="M8 4.5v5H3m-1-6 6 6m13 0v-3c0-1.16-.84-2-2-2h-7m-9 9v2c0 1.05.95 2 2 2h3"></path>
<rect ry="2" y="13.5" x="12" width="10" height="7"></rect>"#;

const PIE_CHART: &str = r#"
<path d="M21.21 15.89A10 10 0 1 1 8 2.83"></path>
<path d="M22 12A10 10 0 0 0 12 2v10z"></path>"#;

const PIGGY_BANK: &str = r#"
<path d="M19 5c-1.5 0-2.8 1.4-3 2-3.5-1.5-11-.3-11 5 0 1.8 0 3 2 4.5V20h4v-2h3v2h4v-4c1-.5 1.7-1 2-2h2v-4h-2c0-1-.5-1.5-1-2h0V5z"></path>
<path d="M2 9v1c0 1.1.9 2 2 2h1"></path>
<path d="M16 11h0"></path>"#;

const PILCROW_SQUARE: &str = r#"
<rect height="18" rx="2" y="3" width="18" x="3"></rect>
<path d="M12 12H9.5a2.5 2.5 0 0 1 0-5H17"></path>
<path d="M12 7v10"></path>
<path d="M16 7v10"></path>"#;

const PILCROW: &str = r#"
<path d="M13 4v16"></path>
<path d="M17 4v16"></path>
<path d="M19 4H9.5a4.5 4.5 0 0 0 0 9H13"></path>"#;

const PILL: &str = r#"
<path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z"></path>
<path d="m8.5 8.5 7 7"></path>"#;

const PIN_OFF: &str = r#"
<line y1="2" x1="2" y2="22" x2="22"></line>
<line x2="12" y1="17" y2="22" x1="12"></line>
<path d="M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12"></path>
<path d="M15 9.34V6h1a2 2 0 0 0 0-4H7.89"></path>"#;

const PIN: &str = r#"
<line x2="12" y2="22" y1="17" x1="12"></line>
<path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"></path>"#;

const PIPETTE: &str = r#"
<path d="m2 22 1-1h3l9-9"></path>
<path d="M3 21v-3l9-9"></path>
<path d="m15 6 3.4-3.4a2.1 2.1 0 1 1 3 3L18 9l.4.4a2.1 2.1 0 1 1-3 3l-3.8-3.8a2.1 2.1 0 1 1 3-3l.4.4Z"></path>"#;

const PIZZA: &str = r#"
<path d="M15 11h.01"></path>
<path d="M11 15h.01"></path>
<path d="M16 16h.01"></path>
<path d="m2 16 20 6-6-20A20 20 0 0 0 2 16"></path>
<path d="M5.71 17.11a17.04 17.04 0 0 1 11.4-11.4"></path>"#;

const PLANE_LANDING: &str = r#"
<path d="M2 22h20"></path>
<path d="M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z"></path>"#;

const PLANE_TAKEOFF: &str = r#"
<path d="M2 22h20"></path>
<path d="M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z"></path>"#;

const PLANE: &str = r#"
<path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z"></path>"#;

const PLAY_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<polygon points="10 8 16 12 10 16 10 8"></polygon>"#;

const PLAY_SQUARE: &str = r#"
<rect height="18" y="3" x="3" rx="2" width="18"></rect>
<path d="m9 8 6 4-6 4Z"></path>"#;

const PLAY: &str = r#"
<polygon points="5 3 19 12 5 21 5 3"></polygon>"#;

const PLUG_2: &str = r#"
<path d="M9 2v6"></path>
<path d="M15 2v6"></path>
<path d="M12 17v5"></path>
<path d="M5 8h14"></path>
<path d="M6 11V8h12v3a6 6 0 1 1-12 0v0Z"></path>"#;

const PLUG_ZAP_2: &str = r#"
<path d="m13 2-2 2.5h3L12 7"></path>
<path d="M10 14v-3"></path>
<path d="M14 14v-3"></path>
<path d="M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z"></path>
<path d="M12 22v-3"></path>"#;

const PLUG_ZAP: &str = r#"
<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"></path>
<path d="m2 22 3-3"></path>
<path d="M7.5 13.5 10 11"></path>
<path d="M10.5 16.5 13 14"></path>
<path d="m18 3-4 4h6l-4 4"></path>"#;

const PLUG: &str = r#"
<path d="M12 22v-5"></path>
<path d="M9 8V2"></path>
<path d="M15 8V2"></path>
<path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z"></path>"#;

const PLUS_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M8 12h8"></path>
<path d="M12 8v8"></path>"#;

const PLUS_SQUARE: &str = r#"
<rect height="18" x="3" rx="2" width="18" y="3"></rect>
<path d="M8 12h8"></path>
<path d="M12 8v8"></path>"#;

const PLUS: &str = r#"
<path d="M5 12h14"></path>
<path d="M12 5v14"></path>"#;

const POCKET_KNIFE: &str = r#"
<path d="M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2"></path>
<path d="M18 6h.01"></path>
<path d="M6 18h.01"></path>
<path d="M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z"></path>
<path d="M18 11.66V22a4 4 0 0 0 4-4V6"></path>"#;

const POCKET: &str = r#"
<path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z"></path>
<polyline points="8 10 12 14 16 10"></polyline>"#;

const PODCAST: &str = r#"
<circle cy="11" cx="12" r="1"></circle>
<path d="M11 17a1 1 0 0 1 2 0c0 .5-.34 3-.5 4.5a.5.5 0 0 1-1 0c-.16-1.5-.5-4-.5-4.5Z"></path>
<path d="M8 14a5 5 0 1 1 8 0"></path>
<path d="M17 18.5a9 9 0 1 0-10 0"></path>"#;

const POINTER: &str = r#"
<path d="M22 14a8 8 0 0 1-8 8"></path>
<path d="M18 11v-1a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M14 10V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1"></path>
<path d="M10 9.5V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v10"></path>
<path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"></path>"#;

const POPCORN: &str = r#"
<path d="M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4"></path>
<path d="M10 22 9 8"></path>
<path d="m14 22 1-14"></path>
<path d="M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z"></path>"#;

const POPSICLE: &str = r#"
<path d="M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z"></path>
<path d="m22 22-5.5-5.5"></path>"#;

const POUND_STERLING: &str = r#"
<path d="M18 7c0-5.333-8-5.333-8 0"></path>
<path d="M10 7v14"></path>
<path d="M6 21h12"></path>
<path d="M6 13h10"></path>"#;

const POWER_OFF: &str = r#"
<path d="M18.36 6.64A9 9 0 0 1 20.77 15"></path>
<path d="M6.16 6.16a9 9 0 1 0 12.68 12.68"></path>
<path d="M12 2v4"></path>
<path d="m2 2 20 20"></path>"#;

const POWER: &str = r#"
<path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path>
<line y1="2" x1="12" y2="12" x2="12"></line>"#;

const PRESENTATION: &str = r#"
<path d="M2 3h20"></path>
<path d="M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3"></path>
<path d="m7 21 5-5 5 5"></path>"#;

const PRINTER: &str = r#"
<polyline points="6 9 6 2 18 2 18 9"></polyline>
<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path>
<rect height="8" width="12" x="6" y="14"></rect>"#;

const PROJECTOR: &str = r#"
<path d="M5 7 3 5"></path>
<path d="M9 6V3"></path>
<path d="m13 7 2-2"></path>
<circle cx="9" r="3" cy="13"></circle>
<path d="M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17"></path>
<path d="M16 16h2"></path>"#;

const PUZZLE: &str = r#"
<path d="M19.439 7.85c-.049.322.059.648.289.878l1.568 1.568c.47.47.706 1.087.706 1.704s-.235 1.233-.706 1.704l-1.611 1.611a.98.98 0 0 1-.837.276c-.47-.07-.802-.48-.968-.925a2.501 2.501 0 1 0-3.214 3.214c.446.166.855.497.925.968a.979.979 0 0 1-.276.837l-1.61 1.61a2.404 2.404 0 0 1-1.705.707 2.402 2.402 0 0 1-1.704-.706l-1.568-1.568a1.026 1.026 0 0 0-.877-.29c-.493.074-.84.504-1.02.968a2.5 2.5 0 1 1-3.237-3.237c.464-.18.894-.527.967-1.02a1.026 1.026 0 0 0-.289-.877l-1.568-1.568A2.402 2.402 0 0 1 1.998 12c0-.617.236-1.234.706-1.704L4.23 8.77c.24-.24.581-.353.917-.303.515.077.877.528 1.073 1.01a2.5 2.5 0 1 0 3.259-3.259c-.482-.196-.933-.558-1.01-1.073-.05-.336.062-.676.303-.917l1.525-1.525A2.402 2.402 0 0 1 12 1.998c.617 0 1.234.236 1.704.706l1.568 1.568c.23.23.556.338.877.29.493-.074.84-.504 1.02-.968a2.5 2.5 0 1 1 3.237 3.237c-.464.18-.894.527-.967 1.02Z"></path>"#;

const QR_CODE: &str = r#"
<rect width="5" x="3" y="3" height="5" rx="1"></rect>
<rect x="16" y="3" rx="1" width="5" height="5"></rect>
<rect x="3" height="5" width="5" rx="1" y="16"></rect>
<path d="M21 16h-3a2 2 0 0 0-2 2v3"></path>
<path d="M21 21v.01"></path>
<path d="M12 7v3a2 2 0 0 1-2 2H7"></path>
<path d="M3 12h.01"></path>
<path d="M12 3h.01"></path>
<path d="M12 16v.01"></path>
<path d="M16 12h1"></path>
<path d="M21 12v.01"></path>
<path d="M12 21v-1"></path>"#;

const QUOTE: &str = r#"
<path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
<path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>"#;

const RABBIT: &str = r#"
<path d="M20 8.54V4a2 2 0 1 0-4 0v3"></path>
<path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1.93 1.93 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1c-1.7 0-3 1.3-3 3"></path>
<path d="M7.61 12.53a3 3 0 1 0-1.6 4.3"></path>
<path d="M13 16a3 3 0 0 1 2.24 5"></path>
<path d="M18 12h.01"></path>"#;

const RADAR: &str = r#"
<path d="M19.07 4.93A10 10 0 0 0 6.99 3.34"></path>
<path d="M4 6h.01"></path>
<path d="M2.29 9.62A10 10 0 1 0 21.31 8.35"></path>
<path d="M16.24 7.76A6 6 0 1 0 8.23 16.67"></path>
<path d="M12 18h.01"></path>
<path d="M17.99 11.66A6 6 0 0 1 15.77 16.67"></path>
<circle cy="12" r="2" cx="12"></circle>
<path d="m13.41 10.59 5.66-5.66"></path>"#;

const RADIATION: &str = r#"
<path d="M12 12h0"></path>
<path d="M7.5 4.2c-.3-.5-.9-.7-1.3-.4C3.9 5.5 2.3 8.1 2 11c-.1.5.4 1 1 1h5c0-1.5.8-2.8 2-3.4-1.1-1.9-2-3.5-2.5-4.4z"></path>
<path d="M21 12c.6 0 1-.4 1-1-.3-2.9-1.8-5.5-4.1-7.1-.4-.3-1.1-.2-1.3.3-.6.9-1.5 2.5-2.6 4.3 1.2.7 2 2 2 3.5h5z"></path>
<path d="M7.5 19.8c-.3.5-.1 1.1.4 1.3 2.6 1.2 5.6 1.2 8.2 0 .5-.2.7-.8.4-1.3-.5-.9-1.4-2.5-2.5-4.3-1.2.7-2.8.7-4 0-1.1 1.8-2 3.4-2.5 4.3z"></path>"#;

const RADIO_RECEIVER: &str = r#"
<path d="M5 16v2"></path>
<path d="M19 16v2"></path>
<rect x="2" width="20" y="8" rx="2" height="8"></rect>
<path d="M18 12h0"></path>"#;

const RADIO_TOWER: &str = r#"
<path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9"></path>
<path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5"></path>
<circle r="2" cy="9" cx="12"></circle>
<path d="M16.2 4.8c2 2 2.26 5.11.8 7.47"></path>
<path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1"></path>
<path d="M9.5 18h5"></path>
<path d="m8 22 4-11 4 11"></path>"#;

const RADIO: &str = r#"
<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9"></path>
<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5"></path>
<circle cy="12" cx="12" r="2"></circle>
<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5"></path>
<path d="M19.1 4.9C23 8.8 23 15.1 19.1 19"></path>"#;

const RAIL_SYMBOL: &str = r#"
<path d="M5 15h14"></path>
<path d="M5 9h14"></path>
<path d="m14 20-5-5 6-6-5-5"></path>"#;

const RAINBOW: &str = r#"
<path d="M22 17a10 10 0 0 0-20 0"></path>
<path d="M6 17a6 6 0 0 1 12 0"></path>
<path d="M10 17a2 2 0 0 1 4 0"></path>"#;

const RAT: &str = r#"
<path d="M17 5c0-1.7-1.3-3-3-3s-3 1.3-3 3c0 .8.3 1.5.8 2H11c-3.9 0-7 3.1-7 7v0c0 2.2 1.8 4 4 4"></path>
<path d="M16.8 3.9c.3-.3.6-.5 1-.7 1.5-.6 3.3.1 3.9 1.6.6 1.5-.1 3.3-1.6 3.9l1.6 2.8c.2.3.2.7.2 1-.2.8-.9 1.2-1.7 1.1 0 0-1.6-.3-2.7-.6H17c-1.7 0-3 1.3-3 3"></path>
<path d="M13.2 18a3 3 0 0 0-2.2-5"></path>
<path d="M13 22H4a2 2 0 0 1 0-4h12"></path>
<path d="M16 9h.01"></path>"#;

const RATIO: &str = r#"
<rect rx="2" x="6" y="2" height="20" width="12"></rect>
<rect rx="2" width="20" x="2" y="6" height="12"></rect>"#;

const RECEIPT: &str = r#"
<path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1-2-1Z"></path>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 17V7"></path>"#;

const RECTANGLE_HORIZONTAL: &str = r#"
<rect height="12" width="20" x="2" y="6" rx="2"></rect>"#;

const RECTANGLE_VERTICAL: &str = r#"
<rect width="12" height="20" y="2" x="6" rx="2"></rect>"#;

const RECYCLE: &str = r#"
<path d="M7 19H4.815a1.83 1.83 0 0 1-1.57-.881 1.785 1.785 0 0 1-.004-1.784L7.196 9.5"></path>
<path d="M11 19h8.203a1.83 1.83 0 0 0 1.556-.89 1.784 1.784 0 0 0 0-1.775l-1.226-2.12"></path>
<path d="m14 16-3 3 3 3"></path>
<path d="M8.293 13.596 7.196 9.5 3.1 10.598"></path>
<path d="m9.344 5.811 1.093-1.892A1.83 1.83 0 0 1 11.985 3a1.784 1.784 0 0 1 1.546.888l3.943 6.843"></path>
<path d="m13.378 9.633 4.096 1.098 1.097-4.096"></path>"#;

const REDO_2: &str = r#"
<path d="m15 14 5-5-5-5"></path>
<path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13"></path>"#;

const REDO_DOT: &str = r#"
<circle cx="12" cy="17" r="1"></circle>
<path d="M21 7v6h-6"></path>
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"></path>"#;

const REDO: &str = r#"
<path d="M21 7v6h-6"></path>
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"></path>"#;

const REFRESH_CCW_DOT: &str = r#"
<path d="M3 2v6h6"></path>
<path d="M21 12A9 9 0 0 0 6 5.3L3 8"></path>
<path d="M21 22v-6h-6"></path>
<path d="M3 12a9 9 0 0 0 15 6.7l3-2.7"></path>
<circle cy="12" r="1" cx="12"></circle>"#;

const REFRESH_CCW: &str = r#"
<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>
<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
<path d="M16 16h5v5"></path>"#;

const REFRESH_CW_OFF: &str = r#"
<path d="M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47"></path>
<path d="M8 16H3v5"></path>
<path d="M3 12C3 9.51 4 7.26 5.64 5.64"></path>
<path d="m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64"></path>
<path d="M21 12c0 1-.16 1.97-.47 2.87"></path>
<path d="M21 3v5h-5"></path>
<path d="M22 22 2 2"></path>"#;

const REFRESH_CW: &str = r#"
<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
<path d="M21 3v5h-5"></path>
<path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
<path d="M8 16H3v5"></path>"#;

const REFRIGERATOR: &str = r#"
<path d="M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z"></path>
<path d="M5 10h14"></path>
<path d="M15 7v6"></path>"#;

const REGEX: &str = r#"
<path d="M17 3v10"></path>
<path d="m12.67 5.5 8.66 5"></path>
<path d="m12.67 10.5 8.66-5"></path>
<path d="M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z"></path>"#;

const REMOVE_FORMATTING: &str = r#"
<path d="M4 7V4h16v3"></path>
<path d="M5 20h6"></path>
<path d="M13 4 8 20"></path>
<path d="m15 15 5 5"></path>
<path d="m20 15-5 5"></path>"#;

const REPEAT_1: &str = r#"
<path d="m17 2 4 4-4 4"></path>
<path d="M3 11v-1a4 4 0 0 1 4-4h14"></path>
<path d="m7 22-4-4 4-4"></path>
<path d="M21 13v1a4 4 0 0 1-4 4H3"></path>
<path d="M11 10h1v4"></path>"#;

const REPEAT_2: &str = r#"
<path d="m2 9 3-3 3 3"></path>
<path d="M13 18H7a2 2 0 0 1-2-2V6"></path>
<path d="m22 15-3 3-3-3"></path>
<path d="M11 6h6a2 2 0 0 1 2 2v10"></path>"#;

const REPEAT: &str = r#"
<path d="m17 2 4 4-4 4"></path>
<path d="M3 11v-1a4 4 0 0 1 4-4h14"></path>
<path d="m7 22-4-4 4-4"></path>
<path d="M21 13v1a4 4 0 0 1-4 4H3"></path>"#;

const REPLACE_ALL: &str = r#"
<path d="M14 4c0-1.1.9-2 2-2"></path>
<path d="M20 2c1.1 0 2 .9 2 2"></path>
<path d="M22 8c0 1.1-.9 2-2 2"></path>
<path d="M16 10c-1.1 0-2-.9-2-2"></path>
<path d="m3 7 3 3 3-3"></path>
<path d="M6 10V5c0-1.7 1.3-3 3-3h1"></path>
<rect rx="2" x="2" width="8" height="8" y="14"></rect>
<path d="M14 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M20 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>"#;

const REPLACE: &str = r#"
<path d="M14 4c0-1.1.9-2 2-2"></path>
<path d="M20 2c1.1 0 2 .9 2 2"></path>
<path d="M22 8c0 1.1-.9 2-2 2"></path>
<path d="M16 10c-1.1 0-2-.9-2-2"></path>
<path d="m3 7 3 3 3-3"></path>
<path d="M6 10V5c0-1.7 1.3-3 3-3h1"></path>
<rect width="8" rx="2" height="8" x="2" y="14"></rect>"#;

const REPLY_ALL: &str = r#"
<polyline points="7 17 2 12 7 7"></polyline>
<polyline points="12 17 7 12 12 7"></polyline>
<path d="M22 18v-2a4 4 0 0 0-4-4H7"></path>"#;

const REPLY: &str = r#"
<polyline points="9 17 4 12 9 7"></polyline>
<path d="M20 18v-2a4 4 0 0 0-4-4H4"></path>"#;

const REWIND: &str = r#"
<polygon points="11 19 2 12 11 5 11 19"></polygon>
<polygon points="22 19 13 12 22 5 22 19"></polygon>"#;

const ROCKET: &str = r#"
<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"></path>
<path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"></path>
<path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"></path>
<path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"></path>"#;

const ROCKING_CHAIR: &str = r#"
<polyline points="3.5 2 6.5 12.5 18 12.5"></polyline>
<line x1="9.5" x2="5.5" y2="20" y1="12.5"></line>
<line x2="18.5" x1="15" y2="20" y1="12.5"></line>
<path d="M2.75 18a13 13 0 0 0 18.5 0"></path>"#;

const ROLLER_COASTER: &str = r#"
<path d="M6 19V5"></path>
<path d="M10 19V6.8"></path>
<path d="M14 19v-7.8"></path>
<path d="M18 5v4"></path>
<path d="M18 19v-6"></path>
<path d="M22 19V9"></path>
<path d="M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65"></path>"#;

const ROTATE_3_D: &str = r#"
<path d="M16.466 7.5C15.643 4.237 13.952 2 12 2 9.239 2 7 6.477 7 12s2.239 10 5 10c.342 0 .677-.069 1-.2"></path>
<path d="m15.194 13.707 3.814 1.86-1.86 3.814"></path>
<path d="M19 15.57c-1.804.885-4.274 1.43-7 1.43-5.523 0-10-2.239-10-5s4.477-5 10-5c4.838 0 8.873 1.718 9.8 4"></path>"#;

const ROTATE_CCW: &str = r#"
<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>"#;

const ROTATE_CW: &str = r#"
<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
<path d="M21 3v5h-5"></path>"#;

const ROUTER: &str = r#"
<rect y="14" x="2" rx="2" width="20" height="8"></rect>
<path d="M6.01 18H6"></path>
<path d="M10.01 18H10"></path>
<path d="M15 10v4"></path>
<path d="M17.84 7.17a4 4 0 0 0-5.66 0"></path>
<path d="M20.66 4.34a8 8 0 0 0-11.31 0"></path>"#;

const ROWS: &str = r#"
<rect y="3" x="3" rx="2" ry="2" width="18" height="18"></rect>
<line x1="3" x2="21" y2="12" y1="12"></line>"#;

const RSS: &str = r#"
<path d="M4 11a9 9 0 0 1 9 9"></path>
<path d="M4 4a16 16 0 0 1 16 16"></path>
<circle cx="5" cy="19" r="1"></circle>"#;

const RULER: &str = r#"
<path d="M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z"></path>
<path d="m14.5 12.5 2-2"></path>
<path d="m11.5 9.5 2-2"></path>
<path d="m8.5 6.5 2-2"></path>
<path d="m17.5 15.5 2-2"></path>"#;

const RUSSIAN_RUBLE: &str = r#"
<path d="M6 11h8a4 4 0 0 0 0-8H9v18"></path>
<path d="M6 15h8"></path>"#;

const SAILBOAT: &str = r#"
<path d="M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z"></path>
<path d="M21 14 10 2 3 14h18Z"></path>
<path d="M10 2v16"></path>"#;

const SALAD: &str = r#"
<path d="M7 21h10"></path>
<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z"></path>
<path d="M11.38 12a2.4 2.4 0 0 1-.4-4.77 2.4 2.4 0 0 1 3.2-2.77 2.4 2.4 0 0 1 3.47-.63 2.4 2.4 0 0 1 3.37 3.37 2.4 2.4 0 0 1-1.1 3.7 2.51 2.51 0 0 1 .03 1.1"></path>
<path d="m13 12 4-4"></path>
<path d="M10.9 7.25A3.99 3.99 0 0 0 4 10c0 .73.2 1.41.54 2"></path>"#;

const SANDWICH: &str = r#"
<path d="M3 11v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3"></path>
<path d="M12 19H4a1 1 0 0 1-1-1v-2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3.83"></path>
<path d="m3 11 7.77-6.04a2 2 0 0 1 2.46 0L21 11H3Z"></path>
<path d="M12.97 19.77 7 15h12.5l-3.75 4.5a2 2 0 0 1-2.78.27Z"></path>"#;

const SATELLITE_DISH: &str = r#"
<path d="M4 10a7.31 7.31 0 0 0 10 10Z"></path>
<path d="m9 15 3-3"></path>
<path d="M17 13a6 6 0 0 0-6-6"></path>
<path d="M21 13A10 10 0 0 0 11 3"></path>"#;

const SATELLITE: &str = r#"
<path d="M13 7 9 3 5 7l4 4"></path>
<path d="m17 11 4 4-4 4-4-4"></path>
<path d="m8 12 4 4 6-6-4-4Z"></path>
<path d="m16 8 3-3"></path>
<path d="M9 21a6 6 0 0 0-6-6"></path>"#;

const SAVE_ALL: &str = r#"
<path d="M6 4a2 2 0 0 1 2-2h10l4 4v10.2a2 2 0 0 1-2 1.8H8a2 2 0 0 1-2-2Z"></path>
<path d="M10 2v4h6"></path>
<path d="M18 18v-7h-8v7"></path>
<path d="M18 22H4a2 2 0 0 1-2-2V6"></path>"#;

const SAVE: &str = r#"
<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
<polyline points="17 21 17 13 7 13 7 21"></polyline>
<polyline points="7 3 7 8 15 8"></polyline>"#;

const SCALE_3_D: &str = r#"
<circle r="2" cy="19" cx="19"></circle>
<circle cy="5" r="2" cx="5"></circle>
<path d="M5 7v12h12"></path>
<path d="m5 19 6-6"></path>"#;

const SCALE: &str = r#"
<path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"></path>
<path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"></path>
<path d="M7 21h10"></path>
<path d="M12 3v18"></path>
<path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2"></path>"#;

const SCALING: &str = r#"
<path d="M21 3 9 15"></path>
<path d="M12 3H3v18h18v-9"></path>
<path d="M16 3h5v5"></path>
<path d="M14 15H9v-5"></path>"#;

const SCAN_FACE: &str = r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<path d="M9 9h.01"></path>
<path d="M15 9h.01"></path>"#;

const SCAN_LINE: &str = r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
<line y2="12" x2="17" x1="7" y1="12"></line>"#;

const SCAN: &str = r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>"#;

const SCATTER_CHART: &str = r#"
<circle r=".5" cy="7.5" cx="7.5"></circle>
<circle cx="18.5" r=".5" cy="5.5"></circle>
<circle cy="11.5" r=".5" cx="11.5"></circle>
<circle cx="7.5" cy="16.5" r=".5"></circle>
<circle r=".5" cy="14.5" cx="17.5"></circle>
<path d="M3 3v18h18"></path>"#;

const SCHOOL_2: &str = r#"
<circle cx="12" r="1" cy="10"></circle>
<path d="M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z"></path>
<path d="M6 17v.01"></path>
<path d="M6 13v.01"></path>
<path d="M18 17v.01"></path>
<path d="M18 13v.01"></path>
<path d="M14 22v-5a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5"></path>"#;

const SCHOOL: &str = r#"
<path d="m4 6 8-4 8 4"></path>
<path d="m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2"></path>
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4"></path>
<path d="M18 5v17"></path>
<path d="M6 5v17"></path>
<circle r="2" cy="9" cx="12"></circle>"#;

const SCISSORS_LINE_DASHED: &str = r#"
<path d="M5.42 9.42 8 12"></path>
<circle cx="4" cy="8" r="2"></circle>
<path d="m14 6-8.58 8.58"></path>
<circle cy="16" cx="4" r="2"></circle>
<path d="M10.8 14.8 14 18"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#;

const SCISSORS_SQUARE_DASHED_BOTTOM: &str = r#"
<path d="M4 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2"></path>
<path d="M10 22H8"></path>
<path d="M16 22h-2"></path>
<circle cx="8" cy="8" r="2"></circle>
<path d="M9.414 9.414 12 12"></path>
<path d="M14.8 14.8 18 18"></path>
<circle cx="8" r="2" cy="16"></circle>
<path d="m18 6-8.586 8.586"></path>"#;

const SCISSORS_SQUARE: &str = r#"
<rect width="20" height="20" y="2" rx="2" x="2"></rect>
<circle r="2" cy="8" cx="8"></circle>
<path d="M9.414 9.414 12 12"></path>
<path d="M14.8 14.8 18 18"></path>
<circle r="2" cy="16" cx="8"></circle>
<path d="m18 6-8.586 8.586"></path>"#;

const SCISSORS: &str = r#"
<circle cx="6" r="3" cy="6"></circle>
<path d="M8.12 8.12 12 12"></path>
<path d="M20 4 8.12 15.88"></path>
<circle cy="18" cx="6" r="3"></circle>
<path d="M14.8 14.8 20 20"></path>"#;

const SCREEN_SHARE_OFF: &str = r#"
<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m22 3-5 5"></path>
<path d="m17 3 5 5"></path>"#;

const SCREEN_SHARE: &str = r#"
<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m17 8 5-5"></path>
<path d="M17 3h5v5"></path>"#;

const SCROLL_TEXT: &str = r#"
<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4"></path>
<path d="M19 17V5a2 2 0 0 0-2-2H4"></path>
<path d="M15 8h-5"></path>
<path d="M15 12h-5"></path>"#;

const SCROLL: &str = r#"
<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4"></path>
<path d="M19 17V5a2 2 0 0 0-2-2H4"></path>"#;

const SEARCH_CHECK: &str = r#"
<path d="m8 11 2 2 4-4"></path>
<circle r="8" cy="11" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#;

const SEARCH_CODE: &str = r#"
<path d="m9 9-2 2 2 2"></path>
<path d="m13 13 2-2-2-2"></path>
<circle cx="11" cy="11" r="8"></circle>
<path d="m21 21-4.3-4.3"></path>"#;

const SEARCH_SLASH: &str = r#"
<path d="m13.5 8.5-5 5"></path>
<circle cy="11" r="8" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#;

const SEARCH_X: &str = r#"
<path d="m13.5 8.5-5 5"></path>
<path d="m8.5 8.5 5 5"></path>
<circle r="8" cy="11" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#;

const SEARCH: &str = r#"
<circle cx="11" cy="11" r="8"></circle>
<path d="m21 21-4.3-4.3"></path>"#;

const SEND_HORIZONTAL: &str = r#"
<path d="m3 3 3 9-3 9 19-9Z"></path>
<path d="M6 12h16"></path>"#;

const SEND_TO_BACK: &str = r#"
<rect rx="2" width="8" y="14" height="8" x="14"></rect>
<rect x="2" rx="2" width="8" height="8" y="2"></rect>
<path d="M7 14v1a2 2 0 0 0 2 2h1"></path>
<path d="M14 7h1a2 2 0 0 1 2 2v1"></path>"#;

const SEND: &str = r#"
<path d="m22 2-7 20-4-9-9-4Z"></path>
<path d="M22 2 11 13"></path>"#;

const SEPARATOR_HORIZONTAL: &str = r#"
<line x2="21" y1="12" x1="3" y2="12"></line>
<polyline points="8 8 12 4 16 8"></polyline>
<polyline points="16 16 12 20 8 16"></polyline>"#;

const SEPARATOR_VERTICAL: &str = r#"
<line y2="21" x2="12" x1="12" y1="3"></line>
<polyline points="8 8 4 12 8 16"></polyline>
<polyline points="16 16 20 12 16 8"></polyline>"#;

const SERVER_COG: &str = r#"
<circle cy="12" r="3" cx="12"></circle>
<path d="M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5"></path>
<path d="M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5"></path>
<path d="M6 6h.01"></path>
<path d="M6 18h.01"></path>
<path d="m15.7 13.4-.9-.3"></path>
<path d="m9.2 10.9-.9-.3"></path>
<path d="m10.6 15.7.3-.9"></path>
<path d="m13.6 15.7-.4-1"></path>
<path d="m10.8 9.3-.4-1"></path>
<path d="m8.3 13.6 1-.4"></path>
<path d="m14.7 10.8 1-.4"></path>
<path d="m13.4 8.3-.3.9"></path>"#;

const SERVER_CRASH: &str = r#"
<path d="M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2"></path>
<path d="M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2"></path>
<path d="M6 6h.01"></path>
<path d="M6 18h.01"></path>
<path d="m13 6-4 6h6l-4 6"></path>"#;

const SERVER_OFF: &str = r#"
<path d="M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5"></path>
<path d="M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z"></path>
<path d="M22 17v-1a2 2 0 0 0-2-2h-1"></path>
<path d="M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z"></path>
<path d="M6 18h.01"></path>
<path d="m2 2 20 20"></path>"#;

const SERVER: &str = r#"
<rect height="8" x="2" y="2" width="20" rx="2" ry="2"></rect>
<rect height="8" x="2" width="20" y="14" ry="2" rx="2"></rect>
<line x2="6.01" x1="6" y1="6" y2="6"></line>
<line x1="6" y2="18" x2="6.01" y1="18"></line>"#;

const SETTINGS_2: &str = r#"
<path d="M20 7h-9"></path>
<path d="M14 17H5"></path>
<circle cy="17" r="3" cx="17"></circle>
<circle r="3" cx="7" cy="7"></circle>"#;

const SETTINGS: &str = r#"
<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
<circle cx="12" cy="12" r="3"></circle>"#;

const SHAPES: &str = r#"
<path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z"></path>
<rect x="3" y="14" width="7" height="7" rx="1"></rect>
<circle cx="17.5" r="3.5" cy="17.5"></circle>"#;

const SHARE_2: &str = r#"
<circle cy="5" cx="18" r="3"></circle>
<circle cx="6" cy="12" r="3"></circle>
<circle r="3" cx="18" cy="19"></circle>
<line x2="15.42" y1="13.51" y2="17.49" x1="8.59"></line>
<line y1="6.51" y2="10.49" x2="8.59" x1="15.41"></line>"#;

const SHARE: &str = r#"
<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
<polyline points="16 6 12 2 8 6"></polyline>
<line x1="12" y1="2" x2="12" y2="15"></line>"#;

const SHEET: &str = r#"
<rect height="18" rx="2" ry="2" y="3" x="3" width="18"></rect>
<line y2="9" x2="21" x1="3" y1="9"></line>
<line x1="3" y1="15" x2="21" y2="15"></line>
<line x1="9" y2="21" y1="9" x2="9"></line>
<line y1="9" y2="21" x2="15" x1="15"></line>"#;

const SHELL: &str = r#"
<path d="M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44"></path>"#;

const SHIELD_ALERT: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M12 8v4"></path>
<path d="M12 16h.01"></path>"#;

const SHIELD_BAN: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m4 5 14 12"></path>"#;

const SHIELD_CHECK: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m9 12 2 2 4-4"></path>"#;

const SHIELD_ELLIPSIS: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h.01"></path>
<path d="M12 11h.01"></path>
<path d="M16 11h.01"></path>"#;

const SHIELD_HALF: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M12 22V2"></path>"#;

const SHIELD_MINUS: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h8"></path>"#;

const SHIELD_OFF: &str = r#"
<path d="M19.7 14a6.9 6.9 0 0 0 .3-2V5l-8-3-3.2 1.2"></path>
<path d="m2 2 20 20"></path>
<path d="M4.7 4.7 4 5v7c0 6 8 10 8 10a20.3 20.3 0 0 0 5.62-4.38"></path>"#;

const SHIELD_PLUS: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h8"></path>
<path d="M12 15V7"></path>"#;

const SHIELD_QUESTION: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3"></path>
<path d="M12 17h.01"></path>"#;

const SHIELD_X: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m14.5 9-5 5"></path>
<path d="m9.5 9 5 5"></path>"#;

const SHIELD: &str = r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>"#;

const SHIP_WHEEL: &str = r#"
<circle cx="12" r="8" cy="12"></circle>
<path d="M12 2v7.5"></path>
<path d="m19 5-5.23 5.23"></path>
<path d="M22 12h-7.5"></path>
<path d="m19 19-5.23-5.23"></path>
<path d="M12 14.5V22"></path>
<path d="M10.23 13.77 5 19"></path>
<path d="M9.5 12H2"></path>
<path d="M10.23 10.23 5 5"></path>
<circle cx="12" cy="12" r="2.5"></circle>"#;

const SHIP: &str = r#"
<path d="M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1 .6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M19.38 20A11.6 11.6 0 0 0 21 14l-9-4-9 4c0 2.9.94 5.34 2.81 7.76"></path>
<path d="M19 13V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v6"></path>
<path d="M12 10v4"></path>
<path d="M12 2v3"></path>"#;

const SHIRT: &str = r#"
<path d="M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z"></path>"#;

const SHOPPING_BAG: &str = r#"
<path d="M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z"></path>
<path d="M3 6h18"></path>
<path d="M16 10a4 4 0 0 1-8 0"></path>"#;

const SHOPPING_BASKET: &str = r#"
<path d="m5 11 4-7"></path>
<path d="m19 11-4-7"></path>
<path d="M2 11h20"></path>
<path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8c.9 0 1.8-.7 2-1.6l1.7-7.4"></path>
<path d="m9 11 1 9"></path>
<path d="M4.5 15.5h15"></path>
<path d="m15 11-1 9"></path>"#;

const SHOPPING_CART: &str = r#"
<circle r="1" cx="8" cy="21"></circle>
<circle r="1" cx="19" cy="21"></circle>
<path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12"></path>"#;

const SHOVEL: &str = r#"
<path d="M2 22v-5l5-5 5 5-5 5z"></path>
<path d="M9.5 14.5 16 8"></path>
<path d="m17 2 5 5-.5.5a3.53 3.53 0 0 1-5 0s0 0 0 0a3.53 3.53 0 0 1 0-5L17 2"></path>"#;

const SHOWER_HEAD: &str = r#"
<path d="m4 4 2.5 2.5"></path>
<path d="M13.5 6.5a4.95 4.95 0 0 0-7 7"></path>
<path d="M15 5 5 15"></path>
<path d="M14 17v.01"></path>
<path d="M10 16v.01"></path>
<path d="M13 13v.01"></path>
<path d="M16 10v.01"></path>
<path d="M11 20v.01"></path>
<path d="M17 14v.01"></path>
<path d="M20 11v.01"></path>"#;

const SHRINK: &str = r#"
<path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8"></path>
<path d="M9 19.8V15m0 0H4.2M9 15l-6 6"></path>
<path d="M15 4.2V9m0 0h4.8M15 9l6-6"></path>
<path d="M9 4.2V9m0 0H4.2M9 9 3 3"></path>"#;

const SHRUB: &str = r#"
<path d="M12 22v-7l-2-2"></path>
<path d="M17 8v.8A6 6 0 0 1 13.8 20v0H10v0A6.5 6.5 0 0 1 7 8h0a5 5 0 0 1 10 0Z"></path>
<path d="m14 14-2 2"></path>"#;

const SHUFFLE: &str = r#"
<path d="M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22"></path>
<path d="m18 2 4 4-4 4"></path>
<path d="M2 6h1.9c1.5 0 2.9.9 3.6 2.2"></path>
<path d="M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8"></path>
<path d="m18 14 4 4-4 4"></path>"#;

const SIGMA_SQUARE: &str = r#"
<rect height="18" rx="2" y="3" x="3" width="18"></rect>
<path d="M16 8.9V7H8l4 5-4 5h8v-1.9"></path>"#;

const SIGMA: &str = r#"
<path d="M18 7V4H6l6 8-6 8h12v-3"></path>"#;

const SIGNAL_HIGH: &str = r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>
<path d="M17 20V8"></path>"#;

const SIGNAL_LOW: &str = r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>"#;

const SIGNAL_MEDIUM: &str = r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>"#;

const SIGNAL_ZERO: &str = r#"
<path d="M2 20h.01"></path>"#;

const SIGNAL: &str = r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>
<path d="M17 20V8"></path>
<path d="M22 4v16"></path>"#;

const SIREN: &str = r#"
<path d="M7 12a5 5 0 0 1 5-5v0a5 5 0 0 1 5 5v6H7v-6Z"></path>
<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v2H5v-2Z"></path>
<path d="M21 12h1"></path>
<path d="M18.5 4.5 18 5"></path>
<path d="M2 12h1"></path>
<path d="M12 2v1"></path>
<path d="m4.929 4.929.707.707"></path>
<path d="M12 12v6"></path>"#;

const SKIP_BACK: &str = r#"
<polygon points="19 20 9 12 19 4 19 20"></polygon>
<line y2="5" x2="5" y1="19" x1="5"></line>"#;

const SKIP_FORWARD: &str = r#"
<polygon points="5 4 15 12 5 20 5 4"></polygon>
<line y1="5" x2="19" y2="19" x1="19"></line>"#;

const SKULL: &str = r#"
<circle cy="12" cx="9" r="1"></circle>
<circle r="1" cy="12" cx="15"></circle>
<path d="M8 20v2h8v-2"></path>
<path d="m12.5 17-.5-1-.5 1h1z"></path>
<path d="M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20"></path>"#;

const SLACK: &str = r#"
<rect x="13" rx="1.5" height="8" width="3" y="2"></rect>
<path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5"></path>
<rect width="3" y="14" height="8" x="8" rx="1.5"></rect>
<path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5"></path>
<rect width="8" y="13" x="14" rx="1.5" height="3"></rect>
<path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5"></path>
<rect rx="1.5" height="3" x="2" y="8" width="8"></rect>
<path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5"></path>"#;

const SLASH: &str = r#"
<path d="M22 2 2 22"></path>"#;

const SLICE: &str = r#"
<path d="m8 14-6 6h9v-3"></path>
<path d="M18.37 3.63 8 14l3 3L21.37 6.63a2.12 2.12 0 1 0-3-3Z"></path>"#;

const SLIDERS_HORIZONTAL: &str = r#"
<line y1="4" x1="21" y2="4" x2="14"></line>
<line y1="4" x2="3" x1="10" y2="4"></line>
<line y1="12" y2="12" x2="12" x1="21"></line>
<line y2="12" x1="8" x2="3" y1="12"></line>
<line x2="16" x1="21" y1="20" y2="20"></line>
<line y2="20" x1="12" y1="20" x2="3"></line>
<line x1="14" x2="14" y1="2" y2="6"></line>
<line y1="10" y2="14" x1="8" x2="8"></line>
<line x1="16" y2="22" y1="18" x2="16"></line>"#;

const SLIDERS: &str = r#"
<line y1="21" x2="4" x1="4" y2="14"></line>
<line x1="4" x2="4" y2="3" y1="10"></line>
<line y2="12" x2="12" x1="12" y1="21"></line>
<line y1="8" y2="3" x1="12" x2="12"></line>
<line y1="21" x1="20" x2="20" y2="16"></line>
<line x1="20" y1="12" y2="3" x2="20"></line>
<line x2="6" y2="14" y1="14" x1="2"></line>
<line y1="8" x1="10" y2="8" x2="14"></line>
<line y2="16" x1="18" x2="22" y1="16"></line>"#;

const SMARTPHONE_CHARGING: &str = r#"
<rect width="14" height="20" y="2" ry="2" rx="2" x="5"></rect>
<path d="M12.667 8 10 12h4l-2.667 4"></path>"#;

const SMARTPHONE_NFC: &str = r#"
<rect x="2" width="7" height="12" y="6" rx="1"></rect>
<path d="M13 8.32a7.43 7.43 0 0 1 0 7.36"></path>
<path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58"></path>
<path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8"></path>"#;

const SMARTPHONE: &str = r#"
<rect width="14" x="5" height="20" rx="2" ry="2" y="2"></rect>
<path d="M12 18h.01"></path>"#;

const SMILE_PLUS: &str = r#"
<path d="M22 11v1a10 10 0 1 1-9-10"></path>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<line x2="9.01" y2="9" y1="9" x1="9"></line>
<line x1="15" y1="9" y2="9" x2="15.01"></line>
<path d="M16 5h6"></path>
<path d="M19 2v6"></path>"#;

const SMILE: &str = r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<line x1="9" x2="9.01" y2="9" y1="9"></line>
<line y1="9" y2="9" x1="15" x2="15.01"></line>"#;

const SNAIL: &str = r#"
<path d="M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0"></path>
<circle r="8" cy="13" cx="10"></circle>
<path d="M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6"></path>
<path d="M18 3 19.1 5.2"></path>
<path d="M22 3 20.9 5.2"></path>"#;

const SNOWFLAKE: &str = r#"
<line x1="2" y1="12" x2="22" y2="12"></line>
<line x2="12" y2="22" y1="2" x1="12"></line>
<path d="m20 16-4-4 4-4"></path>
<path d="m4 8 4 4-4 4"></path>
<path d="m16 4-4 4-4-4"></path>
<path d="m8 20 4-4 4 4"></path>"#;

const SOFA: &str = r#"
<path d="M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3"></path>
<path d="M2 11v5a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H6v-2a2 2 0 0 0-4 0Z"></path>
<path d="M4 18v2"></path>
<path d="M20 18v2"></path>
<path d="M12 4v9"></path>"#;

const SOUP: &str = r#"
<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z"></path>
<path d="M7 21h10"></path>
<path d="M19.5 12 22 6"></path>
<path d="M16.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.73 1.62"></path>
<path d="M11.25 3c.27.1.8.53.74 1.36-.05.83-.93 1.2-.98 2.02-.06.78.33 1.24.72 1.62"></path>
<path d="M6.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.74 1.62"></path>"#;

const SPACE: &str = r#"
<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"></path>"#;

const SPADE: &str = r#"
<path d="M5 9c-1.5 1.5-3 3.2-3 5.5A5.5 5.5 0 0 0 7.5 20c1.8 0 3-.5 4.5-2 1.5 1.5 2.7 2 4.5 2a5.5 5.5 0 0 0 5.5-5.5c0-2.3-1.5-4-3-5.5l-7-7-7 7Z"></path>
<path d="M12 18v4"></path>"#;

const SPARKLE: &str = r#"
<path d="m12 3-1.9 5.8a2 2 0 0 1-1.287 1.288L3 12l5.8 1.9a2 2 0 0 1 1.288 1.287L12 21l1.9-5.8a2 2 0 0 1 1.287-1.288L21 12l-5.8-1.9a2 2 0 0 1-1.288-1.287Z"></path>"#;

const SPARKLES: &str = r#"
<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"></path>
<path d="M5 3v4"></path>
<path d="M19 17v4"></path>
<path d="M3 5h4"></path>
<path d="M17 19h4"></path>"#;

const SPEAKER: &str = r#"
<rect x="4" width="16" y="2" height="20" rx="2" ry="2"></rect>
<circle cy="14" cx="12" r="4"></circle>
<line y1="6" y2="6" x1="12" x2="12.01"></line>"#;

const SPELL_CHECK_2: &str = r#"
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>
<path d="M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1"></path>"#;

const SPELL_CHECK: &str = r#"
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>
<path d="m16 20 2 2 4-4"></path>"#;

const SPLINE: &str = r#"
<circle cy="5" cx="19" r="2"></circle>
<circle r="2" cy="19" cx="5"></circle>
<path d="M5 17A12 12 0 0 1 17 5"></path>"#;

const SPLIT_SQUARE_HORIZONTAL: &str = r#"
<path d="M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3"></path>
<path d="M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3"></path>
<line x1="12" y2="20" x2="12" y1="4"></line>"#;

const SPLIT_SQUARE_VERTICAL: &str = r#"
<path d="M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3"></path>
<path d="M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3"></path>
<line y2="12" x1="4" x2="20" y1="12"></line>"#;

const SPLIT: &str = r#"
<path d="M16 3h5v5"></path>
<path d="M8 3H3v5"></path>
<path d="M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3"></path>
<path d="m15 9 6-6"></path>"#;

const SPRAY_CAN: &str = r#"
<path d="M3 3h.01"></path>
<path d="M7 5h.01"></path>
<path d="M11 7h.01"></path>
<path d="M3 7h.01"></path>
<path d="M7 9h.01"></path>
<path d="M3 11h.01"></path>
<rect height="4" x="15" y="5" width="4"></rect>
<path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2"></path>
<path d="m13 14 8-2"></path>
<path d="m13 19 8-2"></path>"#;

const SPROUT: &str = r#"
<path d="M7 20h10"></path>
<path d="M10 20c5.5-2.5.8-6.4 3-10"></path>
<path d="M9.5 9.4c1.1.8 1.8 2.2 2.3 3.7-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2 2.8-.5 4.4 0 5.5.8z"></path>
<path d="M14.1 6a7 7 0 0 0-1.1 4c1.9-.1 3.3-.6 4.3-1.4 1-1 1.6-2.3 1.7-4.6-2.7.1-4 1-4.9 2z"></path>"#;

const SQUARE_ASTERISK: &str = r#"
<rect width="18" y="3" height="18" x="3" rx="2"></rect>
<path d="M12 8v8"></path>
<path d="m8.5 14 7-4"></path>
<path d="m8.5 10 7 4"></path>"#;

const SQUARE_CODE: &str = r#"
<rect width="18" x="3" rx="2" height="18" y="3"></rect>
<path d="m10 10-2 2 2 2"></path>
<path d="m14 14 2-2-2-2"></path>"#;

const SQUARE_DASHED_BOTTOM_CODE: &str = r#"
<path d="m10 10-2 2 2 2"></path>
<path d="m14 14 2-2-2-2"></path>
<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>"#;

const SQUARE_DASHED_BOTTOM: &str = r#"
<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>"#;

const SQUARE_DOT: &str = r#"
<rect height="18" rx="2" y="3" x="3" width="18"></rect>
<circle cy="12" cx="12" r="1"></circle>"#;

const SQUARE_EQUAL: &str = r#"
<rect y="3" height="18" x="3" width="18" rx="2"></rect>
<path d="M7 10h10"></path>
<path d="M7 14h10"></path>"#;

const SQUARE_SLASH: &str = r#"
<rect height="18" x="3" y="3" rx="2" width="18"></rect>
<line x2="15" y2="9" x1="9" y1="15"></line>"#;

const SQUARE_STACK: &str = r#"
<path d="M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"></path>
<path d="M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"></path>
<rect width="8" height="8" x="14" rx="2" y="14"></rect>"#;

const SQUARE: &str = r#"
<rect rx="2" width="18" x="3" height="18" y="3"></rect>"#;

const SQUIRREL: &str = r#"
<path d="M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10"></path>
<path d="M16 20c0-1.7 1.3-3 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4"></path>
<path d="M15.2 22a3 3 0 0 0-2.2-5"></path>
<path d="M18 13h.01"></path>"#;

const STAMP: &str = r#"
<path d="M5 22h14"></path>
<path d="M19.27 13.73A2.5 2.5 0 0 0 17.5 13h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-1.5c0-.66-.26-1.3-.73-1.77Z"></path>
<path d="M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-3-3c-1.66 0-3 1-3 3s1 2 1 3.5V13"></path>"#;

const STAR_HALF: &str = r#"
<path d="M12 17.8 5.8 21 7 14.1 2 9.3l7-1L12 2"></path>"#;

const STAR_OFF: &str = r#"
<path d="M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43"></path>
<path d="M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91"></path>
<line x1="2" x2="22" y1="2" y2="22"></line>"#;

const STAR: &str = r#"
<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>"#;

const STEP_BACK: &str = r#"
<line x2="18" y1="20" x1="18" y2="4"></line>
<polygon points="14,20 4,12 14,4"></polygon>"#;

const STEP_FORWARD: &str = r#"
<line y2="20" x1="6" x2="6" y1="4"></line>
<polygon points="10,4 20,12 10,20"></polygon>"#;

const STETHOSCOPE: &str = r#"
<path d="M4.8 2.3A.3.3 0 1 0 5 2H4a2 2 0 0 0-2 2v5a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6V4a2 2 0 0 0-2-2h-1a.2.2 0 1 0 .3.3"></path>
<path d="M8 15v1a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6v-4"></path>
<circle cy="10" cx="20" r="2"></circle>"#;

const STICKER: &str = r#"
<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"></path>
<path d="M15 3v6h6"></path>
<path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"></path>
<path d="M8 13h0"></path>
<path d="M16 13h0"></path>"#;

const STICKY_NOTE: &str = r#"
<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"></path>
<path d="M15 3v6h6"></path>"#;

const STOP_CIRCLE: &str = r#"
<circle cy="12" cx="12" r="10"></circle>
<rect height="6" width="6" y="9" x="9"></rect>"#;

const STORE: &str = r#"
<path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7"></path>
<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
<path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4"></path>
<path d="M2 7h20"></path>
<path d="M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7"></path>"#;

const STRETCH_HORIZONTAL: &str = r#"
<rect y="4" height="6" x="2" width="20" rx="2"></rect>
<rect x="2" y="14" rx="2" height="6" width="20"></rect>"#;

const STRETCH_VERTICAL: &str = r#"
<rect rx="2" width="6" x="4" height="20" y="2"></rect>
<rect y="2" rx="2" height="20" x="14" width="6"></rect>"#;

const STRIKETHROUGH: &str = r#"
<path d="M16 4H9a3 3 0 0 0-2.83 4"></path>
<path d="M14 12a4 4 0 0 1 0 8H6"></path>
<line x2="20" y1="12" y2="12" x1="4"></line>"#;

const SUBSCRIPT: &str = r#"
<path d="m4 5 8 8"></path>
<path d="m12 5-8 8"></path>
<path d="M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07"></path>"#;

const SUBTITLES: &str = r#"
<path d="M7 13h4"></path>
<path d="M15 13h2"></path>
<path d="M7 9h2"></path>
<path d="M13 9h4"></path>
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10Z"></path>"#;

const SUN_DIM: &str = r#"
<circle cx="12" cy="12" r="4"></circle>
<path d="M12 4h.01"></path>
<path d="M20 12h.01"></path>
<path d="M12 20h.01"></path>
<path d="M4 12h.01"></path>
<path d="M17.657 6.343h.01"></path>
<path d="M17.657 17.657h.01"></path>
<path d="M6.343 17.657h.01"></path>
<path d="M6.343 6.343h.01"></path>"#;

const SUN_MEDIUM: &str = r#"
<circle cx="12" cy="12" r="4"></circle>
<path d="M12 3v1"></path>
<path d="M12 20v1"></path>
<path d="M3 12h1"></path>
<path d="M20 12h1"></path>
<path d="m18.364 5.636-.707.707"></path>
<path d="m6.343 17.657-.707.707"></path>
<path d="m5.636 5.636.707.707"></path>
<path d="m17.657 17.657.707.707"></path>"#;

const SUN_MOON: &str = r#"
<path d="M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4"></path>
<path d="M12 2v2"></path>
<path d="M12 20v2"></path>
<path d="m4.9 4.9 1.4 1.4"></path>
<path d="m17.7 17.7 1.4 1.4"></path>
<path d="M2 12h2"></path>
<path d="M20 12h2"></path>
<path d="m6.3 17.7-1.4 1.4"></path>
<path d="m19.1 4.9-1.4 1.4"></path>"#;

const SUN_SNOW: &str = r#"
<path d="M10 9a3 3 0 1 0 0 6"></path>
<path d="M2 12h1"></path>
<path d="M14 21V3"></path>
<path d="M10 4V3"></path>
<path d="M10 21v-1"></path>
<path d="m3.64 18.36.7-.7"></path>
<path d="m4.34 6.34-.7-.7"></path>
<path d="M14 12h8"></path>
<path d="m17 4-3 3"></path>
<path d="m14 17 3 3"></path>
<path d="m21 15-3-3 3-3"></path>"#;

const SUN: &str = r#"
<circle cy="12" r="4" cx="12"></circle>
<path d="M12 2v2"></path>
<path d="M12 20v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="m17.66 17.66 1.41 1.41"></path>
<path d="M2 12h2"></path>
<path d="M20 12h2"></path>
<path d="m6.34 17.66-1.41 1.41"></path>
<path d="m19.07 4.93-1.41 1.41"></path>"#;

const SUNRISE: &str = r#"
<path d="M12 2v8"></path>
<path d="m4.93 10.93 1.41 1.41"></path>
<path d="M2 18h2"></path>
<path d="M20 18h2"></path>
<path d="m19.07 10.93-1.41 1.41"></path>
<path d="M22 22H2"></path>
<path d="m8 6 4-4 4 4"></path>
<path d="M16 18a4 4 0 0 0-8 0"></path>"#;

const SUNSET: &str = r#"
<path d="M12 10V2"></path>
<path d="m4.93 10.93 1.41 1.41"></path>
<path d="M2 18h2"></path>
<path d="M20 18h2"></path>
<path d="m19.07 10.93-1.41 1.41"></path>
<path d="M22 22H2"></path>
<path d="m16 6-4 4-4-4"></path>
<path d="M16 18a4 4 0 0 0-8 0"></path>"#;

const SUPERSCRIPT: &str = r#"
<path d="m4 19 8-8"></path>
<path d="m12 19-8-8"></path>
<path d="M20 12h-4c0-1.5.442-2 1.5-2.5S20 8.334 20 7.002c0-.472-.17-.93-.484-1.29a2.105 2.105 0 0 0-2.617-.436c-.42.239-.738.614-.899 1.06"></path>"#;

const SWISS_FRANC: &str = r#"
<path d="M10 21V3h8"></path>
<path d="M6 16h9"></path>
<path d="M10 9.5h7"></path>"#;

const SWITCH_CAMERA: &str = r#"
<path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5"></path>
<path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5"></path>
<circle cx="12" r="3" cy="12"></circle>
<path d="m18 22-3-3 3-3"></path>
<path d="m6 2 3 3-3 3"></path>"#;

const SWORD: &str = r#"
<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5"></polyline>
<line x1="13" y1="19" x2="19" y2="13"></line>
<line x1="16" y1="16" x2="20" y2="20"></line>
<line y2="19" x2="21" x1="19" y1="21"></line>"#;

const SWORDS: &str = r#"
<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5"></polyline>
<line x1="13" x2="19" y1="19" y2="13"></line>
<line x1="16" y2="20" y1="16" x2="20"></line>
<line y1="21" x1="19" x2="21" y2="19"></line>
<polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5"></polyline>
<line x2="9" y1="14" y2="18" x1="5"></line>
<line x2="4" y2="20" x1="7" y1="17"></line>
<line y1="19" x1="3" x2="5" y2="21"></line>"#;

const SYRINGE: &str = r#"
<path d="m18 2 4 4"></path>
<path d="m17 7 3-3"></path>
<path d="M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5"></path>
<path d="m9 11 4 4"></path>
<path d="m5 19-3 3"></path>
<path d="m14 4 6 6"></path>"#;

const TABLE_2: &str = r#"
<path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18"></path>"#;

const TABLE_PROPERTIES: &str = r#"
<path d="M15 3v18"></path>
<rect rx="2" x="3" width="18" height="18" y="3"></rect>
<path d="M21 9H3"></path>
<path d="M21 15H3"></path>"#;

const TABLE: &str = r#"
<path d="M12 3v18"></path>
<rect width="18" rx="2" x="3" height="18" y="3"></rect>
<path d="M3 9h18"></path>
<path d="M3 15h18"></path>"#;

const TABLET_SMARTPHONE: &str = r#"
<rect x="3" width="10" height="14" y="8" rx="2"></rect>
<path d="M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4"></path>
<path d="M8 18h.01"></path>"#;

const TABLET: &str = r#"
<rect ry="2" width="16" y="2" x="4" rx="2" height="20"></rect>
<line x1="12" y2="18" x2="12.01" y1="18"></line>"#;

const TABLETS: &str = r#"
<circle cy="7" r="5" cx="7"></circle>
<circle cx="17" cy="17" r="5"></circle>
<path d="M12 17h10"></path>
<path d="m3.46 10.54 7.08-7.08"></path>"#;

const TAG: &str = r#"
<path d="M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z"></path>
<path d="M7 7h.01"></path>"#;

const TAGS: &str = r#"
<path d="M9 5H2v7l6.29 6.29c.94.94 2.48.94 3.42 0l3.58-3.58c.94-.94.94-2.48 0-3.42L9 5Z"></path>
<path d="M6 9.01V9"></path>
<path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4L17 19"></path>"#;

const TALLY_1: &str = r#"
<path d="M4 4v16"></path>"#;

const TALLY_2: &str = r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>"#;

const TALLY_3: &str = r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>"#;

const TALLY_4: &str = r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>
<path d="M19 4v16"></path>"#;

const TALLY_5: &str = r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>
<path d="M19 4v16"></path>
<path d="M22 6 2 18"></path>"#;

const TARGET: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<circle cx="12" cy="12" r="6"></circle>
<circle cy="12" cx="12" r="2"></circle>"#;

const TENT: &str = r#"
<path d="M19 20 10 4"></path>
<path d="m5 20 9-16"></path>
<path d="M3 20h18"></path>
<path d="m12 15-3 5"></path>
<path d="m12 15 3 5"></path>"#;

const TERMINAL_SQUARE: &str = r#"
<path d="m7 11 2-2-2-2"></path>
<path d="M11 13h4"></path>
<rect rx="2" height="18" ry="2" width="18" x="3" y="3"></rect>"#;

const TERMINAL: &str = r#"
<polyline points="4 17 10 11 4 5"></polyline>
<line x2="20" x1="12" y2="19" y1="19"></line>"#;

const TEST_TUBE_2: &str = r#"
<path d="M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01v0a2.83 2.83 0 0 1 0-4L17 3"></path>
<path d="m16 2 6 6"></path>
<path d="M12 16H4"></path>"#;

const TEST_TUBE: &str = r#"
<path d="M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5h0c-1.4 0-2.5-1.1-2.5-2.5V2"></path>
<path d="M8.5 2h7"></path>
<path d="M14.5 16h-5"></path>"#;

const TEST_TUBES: &str = r#"
<path d="M9 2v17.5A2.5 2.5 0 0 1 6.5 22v0A2.5 2.5 0 0 1 4 19.5V2"></path>
<path d="M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5v0a2.5 2.5 0 0 1-2.5-2.5V2"></path>
<path d="M3 2h7"></path>
<path d="M14 2h7"></path>
<path d="M9 16H4"></path>
<path d="M20 16h-5"></path>"#;

const TEXT_CURSOR_INPUT: &str = r#"
<path d="M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1"></path>
<path d="M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5"></path>
<path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"></path>
<path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7"></path>
<path d="M9 7v10"></path>"#;

const TEXT_CURSOR: &str = r#"
<path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1"></path>
<path d="M7 22h1a4 4 0 0 0 4-4v-1"></path>
<path d="M7 2h1a4 4 0 0 1 4 4v1"></path>"#;

const TEXT_QUOTE: &str = r#"
<path d="M17 6H3"></path>
<path d="M21 12H8"></path>
<path d="M21 18H8"></path>
<path d="M3 12v6"></path>"#;

const TEXT_SELECT: &str = r#"
<path d="M5 3a2 2 0 0 0-2 2"></path>
<path d="M19 3a2 2 0 0 1 2 2"></path>
<path d="M21 19a2 2 0 0 1-2 2"></path>
<path d="M5 21a2 2 0 0 1-2-2"></path>
<path d="M9 3h1"></path>
<path d="M9 21h1"></path>
<path d="M14 3h1"></path>
<path d="M14 21h1"></path>
<path d="M3 9v1"></path>
<path d="M21 9v1"></path>
<path d="M3 14v1"></path>
<path d="M21 14v1"></path>
<line y1="8" x2="15" x1="7" y2="8"></line>
<line y2="12" x2="17" x1="7" y1="12"></line>
<line x2="13" y2="16" y1="16" x1="7"></line>"#;

const TEXT: &str = r#"
<path d="M17 6.1H3"></path>
<path d="M21 12.1H3"></path>
<path d="M15.1 18H3"></path>"#;

const THERMOMETER_SNOWFLAKE: &str = r#"
<path d="M2 12h10"></path>
<path d="M9 4v16"></path>
<path d="m3 9 3 3-3 3"></path>
<path d="M12 6 9 9 6 6"></path>
<path d="m6 18 3-3 1.5 1.5"></path>
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>"#;

const THERMOMETER_SUN: &str = r#"
<path d="M12 9a4 4 0 0 0-2 7.5"></path>
<path d="M12 3v2"></path>
<path d="m6.6 18.4-1.4 1.4"></path>
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>
<path d="M4 13H2"></path>
<path d="M6.34 7.34 4.93 5.93"></path>"#;

const THERMOMETER: &str = r#"
<path d="M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>"#;

const THUMBS_DOWN: &str = r#"
<path d="M17 14V2"></path>
<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z"></path>"#;

const THUMBS_UP: &str = r#"
<path d="M7 10v12"></path>
<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z"></path>"#;

const TICKET: &str = r#"
<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"></path>
<path d="M13 5v2"></path>
<path d="M13 17v2"></path>
<path d="M13 11v2"></path>"#;

const TIMER_OFF: &str = r#"
<path d="M10 2h4"></path>
<path d="M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7"></path>
<path d="M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2"></path>
<path d="m2 2 20 20"></path>
<path d="M12 12v-2"></path>"#;

const TIMER_RESET: &str = r#"
<path d="M10 2h4"></path>
<path d="M12 14v-4"></path>
<path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6"></path>
<path d="M9 17H4v5"></path>"#;

const TIMER: &str = r#"
<line y1="2" x2="14" y2="2" x1="10"></line>
<line x1="12" x2="15" y1="14" y2="11"></line>
<circle cy="14" r="8" cx="12"></circle>"#;

const TOGGLE_LEFT: &str = r#"
<rect x="2" rx="6" y="6" ry="6" width="20" height="12"></rect>
<circle r="2" cx="8" cy="12"></circle>"#;

const TOGGLE_RIGHT: &str = r#"
<rect height="12" width="20" x="2" y="6" ry="6" rx="6"></rect>
<circle cx="16" cy="12" r="2"></circle>"#;

const TORNADO: &str = r#"
<path d="M21 4H3"></path>
<path d="M18 8H6"></path>
<path d="M19 12H9"></path>
<path d="M16 16h-6"></path>
<path d="M11 20H9"></path>"#;

const TOUCHPAD_OFF: &str = r#"
<path d="M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16"></path>
<path d="M2 14h12"></path>
<path d="M22 14h-2"></path>
<path d="M12 20v-6"></path>
<path d="m2 2 20 20"></path>
<path d="M22 16V6a2 2 0 0 0-2-2H10"></path>"#;

const TOUCHPAD: &str = r#"
<rect y="4" rx="2" width="20" x="2" height="16"></rect>
<path d="M2 14h20"></path>
<path d="M12 20v-6"></path>"#;

const TOWER_CONTROL: &str = r#"
<path d="M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z"></path>
<path d="M8 13v9"></path>
<path d="M16 22v-9"></path>
<path d="m9 6 1 7"></path>
<path d="m15 6-1 7"></path>
<path d="M12 6V2"></path>
<path d="M13 2h-2"></path>"#;

const TOY_BRICK: &str = r#"
<rect rx="1" x="3" width="18" height="12" y="8"></rect>
<path d="M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3"></path>
<path d="M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3"></path>"#;

const TRACTOR: &str = r#"
<path d="M3 4h9l1 7"></path>
<path d="M4 11V4"></path>
<path d="M8 10V4"></path>
<path d="M18 5c-.6 0-1 .4-1 1v5.6"></path>
<path d="m10 11 11 .9c.6 0 .9.5.8 1.1l-.8 5h-1"></path>
<circle r=".5" cy="15" cx="7"></circle>
<circle cx="7" r="5" cy="15"></circle>
<path d="M16 18h-5"></path>
<circle cy="18" r="2" cx="18"></circle>"#;

const TRAFFIC_CONE: &str = r#"
<path d="M9.3 6.2a4.55 4.55 0 0 0 5.4 0"></path>
<path d="M7.9 10.7c.9.8 2.4 1.3 4.1 1.3s3.2-.5 4.1-1.3"></path>
<path d="M13.9 3.5a1.93 1.93 0 0 0-3.8-.1l-3 10c-.1.2-.1.4-.1.6 0 1.7 2.2 3 5 3s5-1.3 5-3c0-.2 0-.4-.1-.5Z"></path>
<path d="m7.5 12.2-4.7 2.7c-.5.3-.8.7-.8 1.1s.3.8.8 1.1l7.6 4.5c.9.5 2.1.5 3 0l7.6-4.5c.7-.3 1-.7 1-1.1s-.3-.8-.8-1.1l-4.7-2.8"></path>"#;

const TRAIN_FRONT_TUNNEL: &str = r#"
<path d="M2 22V12a10 10 0 1 1 20 0v10"></path>
<path d="M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8"></path>
<path d="M10 15h.01"></path>
<path d="M14 15h.01"></path>
<path d="M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z"></path>
<path d="m9 19-2 3"></path>
<path d="m15 19 2 3"></path>"#;

const TRAIN_FRONT: &str = r#"
<path d="M8 3.1V7a4 4 0 0 0 8 0V3.1"></path>
<path d="m9 15-1-1"></path>
<path d="m15 15 1-1"></path>
<path d="M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z"></path>
<path d="m8 19-2 3"></path>
<path d="m16 19 2 3"></path>"#;

const TRAIN_TRACK: &str = r#"
<path d="M2 17 17 2"></path>
<path d="m2 14 8 8"></path>
<path d="m5 11 8 8"></path>
<path d="m8 8 8 8"></path>
<path d="m11 5 8 8"></path>
<path d="m14 2 8 8"></path>
<path d="M7 22 22 7"></path>"#;

const TRAM_FRONT: &str = r#"
<rect height="16" y="3" x="4" rx="2" width="16"></rect>
<path d="M4 11h16"></path>
<path d="M12 3v8"></path>
<path d="m8 19-2 3"></path>
<path d="m18 22-2-3"></path>
<path d="M8 15h0"></path>
<path d="M16 15h0"></path>"#;

const TRASH_2: &str = r#"
<path d="M3 6h18"></path>
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
<line x1="10" x2="10" y1="11" y2="17"></line>
<line x1="14" y1="11" y2="17" x2="14"></line>"#;

const TRASH: &str = r#"
<path d="M3 6h18"></path>
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>"#;

const TREE_DECIDUOUS: &str = r#"
<path d="M8 19h8a4 4 0 0 0 3.8-2.8 4 4 0 0 0-1.6-4.5c1-1.1 1-2.7.4-4-.7-1.2-2.2-2-3.6-1.7a3 3 0 0 0-3-3 3 3 0 0 0-3 3c-1.4-.2-2.9.5-3.6 1.7-.7 1.3-.5 2.9.4 4a4 4 0 0 0-1.6 4.5A4 4 0 0 0 8 19Z"></path>
<path d="M12 19v3"></path>"#;

const TREE_PINE: &str = r#"
<path d="m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z"></path>
<path d="M12 22v-3"></path>"#;

const TREES: &str = r#"
<path d="M10 10v.2A3 3 0 0 1 8.9 16v0H5v0h0a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z"></path>
<path d="M7 16v6"></path>
<path d="M13 19v3"></path>
<path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5"></path>"#;

const TRELLO: &str = r#"
<rect height="18" x="3" width="18" y="3" ry="2" rx="2"></rect>
<rect height="9" width="3" x="7" y="7"></rect>
<rect width="3" height="5" x="14" y="7"></rect>"#;

const TRENDING_DOWN: &str = r#"
<polyline points="22 17 13.5 8.5 8.5 13.5 2 7"></polyline>
<polyline points="16 17 22 17 22 11"></polyline>"#;

const TRENDING_UP: &str = r#"
<polyline points="22 7 13.5 15.5 8.5 10.5 2 17"></polyline>
<polyline points="16 7 22 7 22 13"></polyline>"#;

const TRIANGLE_RIGHT: &str = r#"
<path d="M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z"></path>"#;

const TRIANGLE: &str = r#"
<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>"#;

const TROPHY: &str = r#"
<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"></path>
<path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"></path>
<path d="M4 22h16"></path>
<path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"></path>
<path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"></path>
<path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"></path>"#;

const TRUCK: &str = r#"
<path d="M5 18H3c-.6 0-1-.4-1-1V7c0-.6.4-1 1-1h10c.6 0 1 .4 1 1v11"></path>
<path d="M14 9h4l4 4v4c0 .6-.4 1-1 1h-2"></path>
<circle r="2" cx="7" cy="18"></circle>
<path d="M15 18H9"></path>
<circle r="2" cy="18" cx="17"></circle>"#;

const TURTLE: &str = r#"
<path d="m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z"></path>
<path d="M4.82 7.9 8 10"></path>
<path d="M15.18 7.9 12 10"></path>
<path d="M16.93 10H20a2 2 0 0 1 0 4H2"></path>"#;

const TV_2: &str = r#"
<path d="M7 21h10"></path>
<rect height="14" x="2" width="20" rx="2" y="3"></rect>"#;

const TV: &str = r#"
<rect rx="2" ry="2" y="7" width="20" height="15" x="2"></rect>
<polyline points="17 2 12 7 7 2"></polyline>"#;

const TWITCH: &str = r#"
<path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7"></path>"#;

const TWITTER: &str = r#"
<path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z"></path>"#;

const TYPE: &str = r#"
<polyline points="4 7 4 4 20 4 20 7"></polyline>
<line y1="20" y2="20" x1="9" x2="15"></line>
<line x1="12" y1="4" x2="12" y2="20"></line>"#;

const UMBRELLA: &str = r#"
<path d="M22 12a10.06 10.06 1 0 0-20 0Z"></path>
<path d="M12 12v8a2 2 0 0 0 4 0"></path>
<path d="M12 2v1"></path>"#;

const UNDERLINE: &str = r#"
<path d="M6 4v6a6 6 0 0 0 12 0V4"></path>
<line x2="20" x1="4" y1="20" y2="20"></line>"#;

const UNDO_2: &str = r#"
<path d="M9 14 4 9l5-5"></path>
<path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11"></path>"#;

const UNDO_DOT: &str = r#"
<circle r="1" cy="17" cx="12"></circle>
<path d="M3 7v6h6"></path>
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"></path>"#;

const UNDO: &str = r#"
<path d="M3 7v6h6"></path>
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"></path>"#;

const UNFOLD_HORIZONTAL: &str = r#"
<path d="M16 12h6"></path>
<path d="M8 12H2"></path>
<path d="M12 2v2"></path>
<path d="M12 8v2"></path>
<path d="M12 14v2"></path>
<path d="M12 20v2"></path>
<path d="m19 15 3-3-3-3"></path>
<path d="m5 9-3 3 3 3"></path>"#;

const UNFOLD_VERTICAL: &str = r#"
<path d="M12 22v-6"></path>
<path d="M12 8V2"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>
<path d="m15 19-3 3-3-3"></path>
<path d="m15 5-3-3-3 3"></path>"#;

const UNGROUP: &str = r#"
<rect y="4" width="8" x="5" height="6" rx="1"></rect>
<rect width="8" y="14" height="6" x="11" rx="1"></rect>"#;

const UNLINK_2: &str = r#"
<path d="M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2"></path>"#;

const UNLINK: &str = r#"
<path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71"></path>
<path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71"></path>
<line y1="2" x1="8" y2="5" x2="8"></line>
<line y2="8" y1="8" x1="2" x2="5"></line>
<line x1="16" y1="19" y2="22" x2="16"></line>
<line y1="16" x1="19" y2="16" x2="22"></line>"#;

const UNLOCK: &str = r#"
<rect ry="2" x="3" width="18" height="11" rx="2" y="11"></rect>
<path d="M7 11V7a5 5 0 0 1 9.9-1"></path>"#;

const UNPLUG: &str = r#"
<path d="m19 5 3-3"></path>
<path d="m2 22 3-3"></path>
<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"></path>
<path d="M7.5 13.5 10 11"></path>
<path d="M10.5 16.5 13 14"></path>
<path d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z"></path>"#;

const UPLOAD_CLOUD: &str = r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M12 12v9"></path>
<path d="m16 16-4-4-4 4"></path>"#;

const UPLOAD: &str = r#"
<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
<polyline points="17 8 12 3 7 8"></polyline>
<line y2="15" x2="12" x1="12" y1="3"></line>"#;

const USB: &str = r#"
<circle cy="7" r="1" cx="10"></circle>
<circle cy="20" r="1" cx="4"></circle>
<path d="M4.7 19.3 19 5"></path>
<path d="m21 3-3 1 2 2Z"></path>
<path d="M9.26 7.68 5 12l2 5"></path>
<path d="m10 14 5 2 3.5-3.5"></path>
<path d="m18 12 1-1 1 1-1 1Z"></path>"#;

const USER_2: &str = r#"
<circle cx="12" cy="8" r="5"></circle>
<path d="M20 21a8 8 0 1 0-16 0"></path>"#;

const USER_CHECK_2: &str = r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cy="9" cx="8"></circle>
<polyline points="16 11 18 13 22 9"></polyline>"#;

const USER_CHECK: &str = r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cx="9" r="4" cy="7"></circle>
<polyline points="16 11 18 13 22 9"></polyline>"#;

const USER_CIRCLE_2: &str = r#"
<path d="M18 20a6 6 0 0 0-12 0"></path>
<circle r="4" cx="12" cy="10"></circle>
<circle cx="12" r="10" cy="12"></circle>"#;

const USER_CIRCLE: &str = r#"
<circle cx="12" cy="12" r="10"></circle>
<circle cy="10" cx="12" r="3"></circle>
<path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662"></path>"#;

const USER_COG_2: &str = r#"
<circle cx="18" r="3" cy="15"></circle>
<circle cy="9" cx="8" r="4"></circle>
<path d="M10.5 13.5A6 6 0 0 0 2 19"></path>
<path d="m21.7 16.4-.9-.3"></path>
<path d="m15.2 13.9-.9-.3"></path>
<path d="m16.6 18.7.3-.9"></path>
<path d="m19.1 12.2.3-.9"></path>
<path d="m19.6 18.7-.4-1"></path>
<path d="m16.8 12.3-.4-1"></path>
<path d="m14.3 16.6 1-.4"></path>
<path d="m20.7 13.8 1-.4"></path>"#;

const USER_COG: &str = r#"
<circle cx="18" cy="15" r="3"></circle>
<circle r="4" cy="7" cx="9"></circle>
<path d="M10 15H6a4 4 0 0 0-4 4v2"></path>
<path d="m21.7 16.4-.9-.3"></path>
<path d="m15.2 13.9-.9-.3"></path>
<path d="m16.6 18.7.3-.9"></path>
<path d="m19.1 12.2.3-.9"></path>
<path d="m19.6 18.7-.4-1"></path>
<path d="m16.8 12.3-.4-1"></path>
<path d="m14.3 16.6 1-.4"></path>
<path d="m20.7 13.8 1-.4"></path>"#;

const USER_MINUS_2: &str = r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cx="8" cy="9"></circle>
<line y2="11" x1="22" y1="11" x2="16"></line>"#;

const USER_MINUS: &str = r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle r="4" cy="7" cx="9"></circle>
<line x1="22" y2="11" y1="11" x2="16"></line>"#;

const USER_PLUS_2: &str = r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cx="8" cy="9"></circle>
<line y2="14" x2="19" x1="19" y1="8"></line>
<line x2="16" y2="11" x1="22" y1="11"></line>"#;

const USER_PLUS: &str = r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle r="4" cy="7" cx="9"></circle>
<line x2="19" y1="8" x1="19" y2="14"></line>
<line y1="11" x2="16" y2="11" x1="22"></line>"#;

const USER_SQUARE_2: &str = r#"
<path d="M18 21a6 6 0 0 0-12 0"></path>
<circle cy="11" r="4" cx="12"></circle>
<rect x="3" y="3" width="18" rx="2" height="18"></rect>"#;

const USER_SQUARE: &str = r#"
<rect width="18" x="3" rx="2" height="18" y="3"></rect>
<circle cy="10" cx="12" r="3"></circle>
<path d="M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2"></path>"#;

const USER_X_2: &str = r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cx="8" cy="9"></circle>
<line x2="22" y2="13" y1="8" x1="17"></line>
<line y2="13" x1="22" y1="8" x2="17"></line>"#;

const USER_X: &str = r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle r="4" cx="9" cy="7"></circle>
<line y1="8" x2="22" x1="17" y2="13"></line>
<line x2="17" y1="8" y2="13" x1="22"></line>"#;

const USER: &str = r#"
<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
<circle r="4" cx="12" cy="7"></circle>"#;

const USERS_2: &str = r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle cx="8" cy="9" r="4"></circle>
<path d="M22 19a6 6 0 0 0-6-6 4 4 0 1 0 0-8"></path>"#;

const USERS: &str = r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle r="4" cy="7" cx="9"></circle>
<path d="M22 21v-2a4 4 0 0 0-3-3.87"></path>
<path d="M16 3.13a4 4 0 0 1 0 7.75"></path>"#;

const UTENSILS_CROSSED: &str = r#"
<path d="m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8"></path>
<path d="M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7"></path>
<path d="m2.1 21.8 6.4-6.3"></path>
<path d="m19 5-7 7"></path>"#;

const UTENSILS: &str = r#"
<path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2"></path>
<path d="M7 2v20"></path>
<path d="M21 15V2v0a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7"></path>"#;

const UTILITY_POLE: &str = r#"
<path d="M12 2v20"></path>
<path d="M2 5h20"></path>
<path d="M3 3v2"></path>
<path d="M7 3v2"></path>
<path d="M17 3v2"></path>
<path d="M21 3v2"></path>
<path d="m19 5-7 7-7-7"></path>"#;

const VARIABLE: &str = r#"
<path d="M8 21s-4-3-4-9 4-9 4-9"></path>
<path d="M16 3s4 3 4 9-4 9-4 9"></path>
<line x1="15" x2="9" y1="9" y2="15"></line>
<line x2="15" y1="9" y2="15" x1="9"></line>"#;

const VEGAN: &str = r#"
<path d="M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14"></path>
<path d="M16 8c4 0 6-2 6-6-4 0-6 2-6 6"></path>
<path d="M17.41 3.6a10 10 0 1 0 3 3"></path>"#;

const VENETIAN_MASK: &str = r#"
<path d="M2 12a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V7h-5a8 8 0 0 0-5 2 8 8 0 0 0-5-2H2Z"></path>
<path d="M6 11c1.5 0 3 .5 3 2-2 0-3 0-3-2Z"></path>
<path d="M18 11c-1.5 0-3 .5-3 2 2 0 3 0 3-2Z"></path>"#;

const VIBRATE_OFF: &str = r#"
<path d="m2 8 2 2-2 2 2 2-2 2"></path>
<path d="m22 8-2 2 2 2-2 2 2 2"></path>
<path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2"></path>
<path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34"></path>
<line y1="2" y2="22" x1="2" x2="22"></line>"#;

const VIBRATE: &str = r#"
<path d="m2 8 2 2-2 2 2 2-2 2"></path>
<path d="m22 8-2 2 2 2-2 2 2 2"></path>
<rect height="14" width="8" x="8" y="5" rx="1"></rect>"#;

const VIDEO_OFF: &str = r#"
<path d="M10.66 6H14a2 2 0 0 1 2 2v2.34l1 1L22 8v8"></path>
<path d="M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2l10 10Z"></path>
<line y2="22" x1="2" x2="22" y1="2"></line>"#;

const VIDEO: &str = r#"
<path d="m22 8-6 4 6 4V8Z"></path>
<rect width="14" ry="2" height="12" rx="2" x="2" y="6"></rect>"#;

const VIDEOTAPE: &str = r#"
<rect y="4" x="2" height="16" rx="2" width="20"></rect>
<path d="M2 8h20"></path>
<circle cy="14" r="2" cx="8"></circle>
<path d="M8 12h8"></path>
<circle cy="14" cx="16" r="2"></circle>"#;

const VIEW: &str = r#"
<path d="M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z"></path>
<path d="M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"></path>
<path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2"></path>
<path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2"></path>"#;

const VOICEMAIL: &str = r#"
<circle cx="6" cy="12" r="4"></circle>
<circle cx="18" cy="12" r="4"></circle>
<line y2="16" x1="6" x2="18" y1="16"></line>"#;

const VOLUME_1: &str = r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>"#;

const VOLUME_2: &str = r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
<path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>"#;

const VOLUME_X: &str = r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<line y1="9" x1="22" y2="15" x2="16"></line>
<line x1="16" y1="9" x2="22" y2="15"></line>"#;

const VOLUME: &str = r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>"#;

const VOTE: &str = r#"
<path d="m9 12 2 2 4-4"></path>
<path d="M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z"></path>
<path d="M22 19H2"></path>"#;

const WALLET_2: &str = r#"
<path d="M17 14h.01"></path>
<path d="M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14"></path>"#;

const WALLET_CARDS: &str = r#"
<rect height="18" width="18" rx="2" y="3" x="3"></rect>
<path d="M3 9a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2"></path>
<path d="M3 11h3c.8 0 1.6.3 2.1.9l1.1.9c1.6 1.6 4.1 1.6 5.7 0l1.1-.9c.5-.5 1.3-.9 2.1-.9H21"></path>"#;

const WALLET: &str = r#"
<path d="M21 12V7H5a2 2 0 0 1 0-4h14v4"></path>
<path d="M3 5v14a2 2 0 0 0 2 2h16v-5"></path>
<path d="M18 12a2 2 0 0 0 0 4h4v-4Z"></path>"#;

const WALLPAPER: &str = r#"
<circle cx="8" cy="9" r="2"></circle>
<path d="m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>"#;

const WAND_2: &str = r#"
<path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72Z"></path>
<path d="m14 7 3 3"></path>
<path d="M5 6v4"></path>
<path d="M19 14v4"></path>
<path d="M10 2v2"></path>
<path d="M7 8H3"></path>
<path d="M21 16h-4"></path>
<path d="M11 3H9"></path>"#;

const WAND: &str = r#"
<path d="M15 4V2"></path>
<path d="M15 16v-2"></path>
<path d="M8 9h2"></path>
<path d="M20 9h2"></path>
<path d="M17.8 11.8 19 13"></path>
<path d="M15 9h0"></path>
<path d="M17.8 6.2 19 5"></path>
<path d="m3 21 9-9"></path>
<path d="M12.2 6.2 11 5"></path>"#;

const WAREHOUSE: &str = r#"
<path d="M22 8.35V20a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8.35A2 2 0 0 1 3.26 6.5l8-3.2a2 2 0 0 1 1.48 0l8 3.2A2 2 0 0 1 22 8.35Z"></path>
<path d="M6 18h12"></path>
<path d="M6 14h12"></path>
<rect width="12" y="10" height="12" x="6"></rect>"#;

const WATCH: &str = r#"
<circle r="6" cy="12" cx="12"></circle>
<polyline points="12 10 12 12 13 13"></polyline>
<path d="m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05"></path>
<path d="m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05"></path>"#;

const WAVES: &str = r#"
<path d="M2 6c.6.5 1.2 1 2.5 1C7 7 7 5 9.5 5c2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M2 12c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>"#;

const WEBCAM: &str = r#"
<circle r="8" cy="10" cx="12"></circle>
<circle cx="12" cy="10" r="3"></circle>
<path d="M7 22h10"></path>
<path d="M12 22v-4"></path>"#;

const WEBHOOK: &str = r#"
<path d="M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2"></path>
<path d="m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06"></path>
<path d="m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8"></path>"#;

const WHEAT_OFF: &str = r#"
<path d="m2 22 10-10"></path>
<path d="m16 8-1.17 1.17"></path>
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97"></path>
<path d="M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62"></path>
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"></path>
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98"></path>
<path d="M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28"></path>
<line x1="2" x2="22" y2="22" y1="2"></line>"#;

const WHEAT: &str = r#"
<path d="M2 22 16 8"></path>
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"></path>
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>"#;

const WHOLE_WORD: &str = r#"
<circle cx="7" cy="12" r="3"></circle>
<path d="M10 9v6"></path>
<circle cy="12" r="3" cx="17"></circle>
<path d="M14 7v8"></path>
<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"></path>"#;

const WIFI_OFF: &str = r#"
<line x2="22" y2="22" y1="2" x1="2"></line>
<path d="M8.5 16.5a5 5 0 0 1 7 0"></path>
<path d="M2 8.82a15 15 0 0 1 4.17-2.65"></path>
<path d="M10.66 5c4.01-.36 8.14.9 11.34 3.76"></path>
<path d="M16.85 11.25a10 10 0 0 1 2.22 1.68"></path>
<path d="M5 13a10 10 0 0 1 5.24-2.76"></path>
<line y1="20" x1="12" y2="20" x2="12.01"></line>"#;

const WIFI: &str = r#"
<path d="M5 13a10 10 0 0 1 14 0"></path>
<path d="M8.5 16.5a5 5 0 0 1 7 0"></path>
<path d="M2 8.82a15 15 0 0 1 20 0"></path>
<line y1="20" x1="12" y2="20" x2="12.01"></line>"#;

const WIND: &str = r#"
<path d="M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2"></path>
<path d="M9.6 4.6A2 2 0 1 1 11 8H2"></path>
<path d="M12.6 19.4A2 2 0 1 0 14 16H2"></path>"#;

const WINE_OFF: &str = r#"
<path d="M8 22h8"></path>
<path d="M7 10h3m7 0h-1.343"></path>
<path d="M12 15v7"></path>
<path d="M7.307 7.307A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.391 4.391M8.638 2.981C8.75 2.668 8.872 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.809-.145 1.198"></path>
<line x1="2" y1="2" y2="22" x2="22"></line>"#;

const WINE: &str = r#"
<path d="M8 22h8"></path>
<path d="M7 10h10"></path>
<path d="M12 15v7"></path>
<path d="M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z"></path>"#;

const WORKFLOW: &str = r#"
<rect y="3" rx="2" width="8" x="3" height="8"></rect>
<path d="M7 11v4a2 2 0 0 0 2 2h4"></path>
<rect y="13" rx="2" x="13" width="8" height="8"></rect>"#;

const WRAP_TEXT: &str = r#"
<line x2="21" y1="6" y2="6" x1="3"></line>
<path d="M3 12h15a3 3 0 1 1 0 6h-4"></path>
<polyline points="16 16 14 18 16 20"></polyline>
<line y2="18" y1="18" x1="3" x2="10"></line>"#;

const WRENCH: &str = r#"
<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>"#;

const X_CIRCLE: &str = r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#;

const X_OCTAGON: &str = r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#;

const X_SQUARE: &str = r#"
<rect rx="2" y="3" width="18" ry="2" x="3" height="18"></rect>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#;

const X: &str = r#"
<path d="M18 6 6 18"></path>
<path d="m6 6 12 12"></path>"#;

const YOUTUBE: &str = r#"
<path d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17"></path>
<path d="m10 15 5-3-5-3z"></path>"#;

const ZAP_OFF: &str = r#"
<polyline points="12.41 6.75 13 2 10.57 4.92"></polyline>
<polyline points="18.57 12.91 21 10 15.66 10"></polyline>
<polyline points="8 8 3 14 12 14 11 22 16 16"></polyline>
<line y1="2" x1="2" y2="22" x2="22"></line>"#;

const ZAP: &str = r#"
<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>"#;

const ZOOM_IN: &str = r#"
<circle cy="11" r="8" cx="11"></circle>
<line y1="21" y2="16.65" x1="21" x2="16.65"></line>
<line x1="11" x2="11" y1="8" y2="14"></line>
<line x2="14" x1="8" y2="11" y1="11"></line>"#;

const ZOOM_OUT: &str = r#"
<circle cx="11" r="8" cy="11"></circle>
<line x1="21" x2="16.65" y1="21" y2="16.65"></line>
<line y2="11" x1="8" x2="14" y1="11"></line>"#;

impl LucideIcon {
        pub fn svg(&self) -> String {
           match self {
    
 &Self::Accessibility => ACCESSIBILITY.to_string(),
 &Self::ActivitySquare => ACTIVITY_SQUARE.to_string(),
 &Self::Activity => ACTIVITY.to_string(),
 &Self::AirVent => AIR_VENT.to_string(),
 &Self::Airplay => AIRPLAY.to_string(),
 &Self::AlarmCheck => ALARM_CHECK.to_string(),
 &Self::AlarmClockOff => ALARM_CLOCK_OFF.to_string(),
 &Self::AlarmClock => ALARM_CLOCK.to_string(),
 &Self::AlarmMinus => ALARM_MINUS.to_string(),
 &Self::AlarmPlus => ALARM_PLUS.to_string(),
 &Self::Album => ALBUM.to_string(),
 &Self::AlertCircle => ALERT_CIRCLE.to_string(),
 &Self::AlertOctagon => ALERT_OCTAGON.to_string(),
 &Self::AlertTriangle => ALERT_TRIANGLE.to_string(),
 &Self::AlignCenterHorizontal => ALIGN_CENTER_HORIZONTAL.to_string(),
 &Self::AlignCenterVertical => ALIGN_CENTER_VERTICAL.to_string(),
 &Self::AlignCenter => ALIGN_CENTER.to_string(),
 &Self::AlignEndHorizontal => ALIGN_END_HORIZONTAL.to_string(),
 &Self::AlignEndVertical => ALIGN_END_VERTICAL.to_string(),
 &Self::AlignHorizontalDistributeCenter => ALIGN_HORIZONTAL_DISTRIBUTE_CENTER.to_string(),
 &Self::AlignHorizontalDistributeEnd => ALIGN_HORIZONTAL_DISTRIBUTE_END.to_string(),
 &Self::AlignHorizontalDistributeStart => ALIGN_HORIZONTAL_DISTRIBUTE_START.to_string(),
 &Self::AlignHorizontalJustifyCenter => ALIGN_HORIZONTAL_JUSTIFY_CENTER.to_string(),
 &Self::AlignHorizontalJustifyEnd => ALIGN_HORIZONTAL_JUSTIFY_END.to_string(),
 &Self::AlignHorizontalJustifyStart => ALIGN_HORIZONTAL_JUSTIFY_START.to_string(),
 &Self::AlignHorizontalSpaceAround => ALIGN_HORIZONTAL_SPACE_AROUND.to_string(),
 &Self::AlignHorizontalSpaceBetween => ALIGN_HORIZONTAL_SPACE_BETWEEN.to_string(),
 &Self::AlignJustify => ALIGN_JUSTIFY.to_string(),
 &Self::AlignLeft => ALIGN_LEFT.to_string(),
 &Self::AlignRight => ALIGN_RIGHT.to_string(),
 &Self::AlignStartHorizontal => ALIGN_START_HORIZONTAL.to_string(),
 &Self::AlignStartVertical => ALIGN_START_VERTICAL.to_string(),
 &Self::AlignVerticalDistributeCenter => ALIGN_VERTICAL_DISTRIBUTE_CENTER.to_string(),
 &Self::AlignVerticalDistributeEnd => ALIGN_VERTICAL_DISTRIBUTE_END.to_string(),
 &Self::AlignVerticalDistributeStart => ALIGN_VERTICAL_DISTRIBUTE_START.to_string(),
 &Self::AlignVerticalJustifyCenter => ALIGN_VERTICAL_JUSTIFY_CENTER.to_string(),
 &Self::AlignVerticalJustifyEnd => ALIGN_VERTICAL_JUSTIFY_END.to_string(),
 &Self::AlignVerticalJustifyStart => ALIGN_VERTICAL_JUSTIFY_START.to_string(),
 &Self::AlignVerticalSpaceAround => ALIGN_VERTICAL_SPACE_AROUND.to_string(),
 &Self::AlignVerticalSpaceBetween => ALIGN_VERTICAL_SPACE_BETWEEN.to_string(),
 &Self::Ampersand => AMPERSAND.to_string(),
 &Self::Ampersands => AMPERSANDS.to_string(),
 &Self::Anchor => ANCHOR.to_string(),
 &Self::Angry => ANGRY.to_string(),
 &Self::Annoyed => ANNOYED.to_string(),
 &Self::Antenna => ANTENNA.to_string(),
 &Self::Aperture => APERTURE.to_string(),
 &Self::AppWindow => APP_WINDOW.to_string(),
 &Self::Apple => APPLE.to_string(),
 &Self::ArchiveRestore => ARCHIVE_RESTORE.to_string(),
 &Self::ArchiveX => ARCHIVE_X.to_string(),
 &Self::Archive => ARCHIVE.to_string(),
 &Self::AreaChart => AREA_CHART.to_string(),
 &Self::Armchair => ARMCHAIR.to_string(),
 &Self::ArrowBigDownDash => ARROW_BIG_DOWN_DASH.to_string(),
 &Self::ArrowBigDown => ARROW_BIG_DOWN.to_string(),
 &Self::ArrowBigLeftDash => ARROW_BIG_LEFT_DASH.to_string(),
 &Self::ArrowBigLeft => ARROW_BIG_LEFT.to_string(),
 &Self::ArrowBigRightDash => ARROW_BIG_RIGHT_DASH.to_string(),
 &Self::ArrowBigRight => ARROW_BIG_RIGHT.to_string(),
 &Self::ArrowBigUpDash => ARROW_BIG_UP_DASH.to_string(),
 &Self::ArrowBigUp => ARROW_BIG_UP.to_string(),
 &Self::ArrowDown01 => ARROW_DOWN_01.to_string(),
 &Self::ArrowDown10 => ARROW_DOWN_10.to_string(),
 &Self::ArrowDownAZ => ARROW_DOWN_AZ.to_string(),
 &Self::ArrowDownCircle => ARROW_DOWN_CIRCLE.to_string(),
 &Self::ArrowDownFromLine => ARROW_DOWN_FROM_LINE.to_string(),
 &Self::ArrowDownLeftFromCircle => ARROW_DOWN_LEFT_FROM_CIRCLE.to_string(),
 &Self::ArrowDownLeftSquare => ARROW_DOWN_LEFT_SQUARE.to_string(),
 &Self::ArrowDownLeft => ARROW_DOWN_LEFT.to_string(),
 &Self::ArrowDownNarrowWide => ARROW_DOWN_NARROW_WIDE.to_string(),
 &Self::ArrowDownRightFromCircle => ARROW_DOWN_RIGHT_FROM_CIRCLE.to_string(),
 &Self::ArrowDownRightSquare => ARROW_DOWN_RIGHT_SQUARE.to_string(),
 &Self::ArrowDownRight => ARROW_DOWN_RIGHT.to_string(),
 &Self::ArrowDownSquare => ARROW_DOWN_SQUARE.to_string(),
 &Self::ArrowDownToDot => ARROW_DOWN_TO_DOT.to_string(),
 &Self::ArrowDownToLine => ARROW_DOWN_TO_LINE.to_string(),
 &Self::ArrowDownUp => ARROW_DOWN_UP.to_string(),
 &Self::ArrowDownWideNarrow => ARROW_DOWN_WIDE_NARROW.to_string(),
 &Self::ArrowDownZA => ARROW_DOWN_ZA.to_string(),
 &Self::ArrowDown => ARROW_DOWN.to_string(),
 &Self::ArrowLeftCircle => ARROW_LEFT_CIRCLE.to_string(),
 &Self::ArrowLeftFromLine => ARROW_LEFT_FROM_LINE.to_string(),
 &Self::ArrowLeftRight => ARROW_LEFT_RIGHT.to_string(),
 &Self::ArrowLeftSquare => ARROW_LEFT_SQUARE.to_string(),
 &Self::ArrowLeftToLine => ARROW_LEFT_TO_LINE.to_string(),
 &Self::ArrowLeft => ARROW_LEFT.to_string(),
 &Self::ArrowRightCircle => ARROW_RIGHT_CIRCLE.to_string(),
 &Self::ArrowRightFromLine => ARROW_RIGHT_FROM_LINE.to_string(),
 &Self::ArrowRightLeft => ARROW_RIGHT_LEFT.to_string(),
 &Self::ArrowRightSquare => ARROW_RIGHT_SQUARE.to_string(),
 &Self::ArrowRightToLine => ARROW_RIGHT_TO_LINE.to_string(),
 &Self::ArrowRight => ARROW_RIGHT.to_string(),
 &Self::ArrowUp01 => ARROW_UP_01.to_string(),
 &Self::ArrowUp10 => ARROW_UP_10.to_string(),
 &Self::ArrowUpAZ => ARROW_UP_AZ.to_string(),
 &Self::ArrowUpCircle => ARROW_UP_CIRCLE.to_string(),
 &Self::ArrowUpDown => ARROW_UP_DOWN.to_string(),
 &Self::ArrowUpFromDot => ARROW_UP_FROM_DOT.to_string(),
 &Self::ArrowUpFromLine => ARROW_UP_FROM_LINE.to_string(),
 &Self::ArrowUpLeftFromCircle => ARROW_UP_LEFT_FROM_CIRCLE.to_string(),
 &Self::ArrowUpLeftSquare => ARROW_UP_LEFT_SQUARE.to_string(),
 &Self::ArrowUpLeft => ARROW_UP_LEFT.to_string(),
 &Self::ArrowUpNarrowWide => ARROW_UP_NARROW_WIDE.to_string(),
 &Self::ArrowUpRightFromCircle => ARROW_UP_RIGHT_FROM_CIRCLE.to_string(),
 &Self::ArrowUpRightSquare => ARROW_UP_RIGHT_SQUARE.to_string(),
 &Self::ArrowUpRight => ARROW_UP_RIGHT.to_string(),
 &Self::ArrowUpSquare => ARROW_UP_SQUARE.to_string(),
 &Self::ArrowUpToLine => ARROW_UP_TO_LINE.to_string(),
 &Self::ArrowUpWideNarrow => ARROW_UP_WIDE_NARROW.to_string(),
 &Self::ArrowUpZA => ARROW_UP_ZA.to_string(),
 &Self::ArrowUp => ARROW_UP.to_string(),
 &Self::ArrowsUpFromLine => ARROWS_UP_FROM_LINE.to_string(),
 &Self::Asterisk => ASTERISK.to_string(),
 &Self::AtSign => AT_SIGN.to_string(),
 &Self::Atom => ATOM.to_string(),
 &Self::Award => AWARD.to_string(),
 &Self::Axe => AXE.to_string(),
 &Self::Axis3D => AXIS_3_D.to_string(),
 &Self::Baby => BABY.to_string(),
 &Self::Backpack => BACKPACK.to_string(),
 &Self::BadgeAlert => BADGE_ALERT.to_string(),
 &Self::BadgeCent => BADGE_CENT.to_string(),
 &Self::BadgeCheck => BADGE_CHECK.to_string(),
 &Self::BadgeDollarSign => BADGE_DOLLAR_SIGN.to_string(),
 &Self::BadgeEuro => BADGE_EURO.to_string(),
 &Self::BadgeHelp => BADGE_HELP.to_string(),
 &Self::BadgeIndianRupee => BADGE_INDIAN_RUPEE.to_string(),
 &Self::BadgeInfo => BADGE_INFO.to_string(),
 &Self::BadgeJapaneseYen => BADGE_JAPANESE_YEN.to_string(),
 &Self::BadgeMinus => BADGE_MINUS.to_string(),
 &Self::BadgePercent => BADGE_PERCENT.to_string(),
 &Self::BadgePlus => BADGE_PLUS.to_string(),
 &Self::BadgePoundSterling => BADGE_POUND_STERLING.to_string(),
 &Self::BadgeRussianRuble => BADGE_RUSSIAN_RUBLE.to_string(),
 &Self::BadgeSwissFranc => BADGE_SWISS_FRANC.to_string(),
 &Self::BadgeX => BADGE_X.to_string(),
 &Self::Badge => BADGE.to_string(),
 &Self::BaggageClaim => BAGGAGE_CLAIM.to_string(),
 &Self::Ban => BAN.to_string(),
 &Self::Banana => BANANA.to_string(),
 &Self::Banknote => BANKNOTE.to_string(),
 &Self::BarChart2 => BAR_CHART_2.to_string(),
 &Self::BarChart3 => BAR_CHART_3.to_string(),
 &Self::BarChart4 => BAR_CHART_4.to_string(),
 &Self::BarChartBig => BAR_CHART_BIG.to_string(),
 &Self::BarChartHorizontalBig => BAR_CHART_HORIZONTAL_BIG.to_string(),
 &Self::BarChartHorizontal => BAR_CHART_HORIZONTAL.to_string(),
 &Self::BarChart => BAR_CHART.to_string(),
 &Self::Baseline => BASELINE.to_string(),
 &Self::Bath => BATH.to_string(),
 &Self::BatteryCharging => BATTERY_CHARGING.to_string(),
 &Self::BatteryFull => BATTERY_FULL.to_string(),
 &Self::BatteryLow => BATTERY_LOW.to_string(),
 &Self::BatteryMedium => BATTERY_MEDIUM.to_string(),
 &Self::BatteryWarning => BATTERY_WARNING.to_string(),
 &Self::Battery => BATTERY.to_string(),
 &Self::Beaker => BEAKER.to_string(),
 &Self::BeanOff => BEAN_OFF.to_string(),
 &Self::Bean => BEAN.to_string(),
 &Self::BedDouble => BED_DOUBLE.to_string(),
 &Self::BedSingle => BED_SINGLE.to_string(),
 &Self::Bed => BED.to_string(),
 &Self::Beef => BEEF.to_string(),
 &Self::Beer => BEER.to_string(),
 &Self::BellDot => BELL_DOT.to_string(),
 &Self::BellMinus => BELL_MINUS.to_string(),
 &Self::BellOff => BELL_OFF.to_string(),
 &Self::BellPlus => BELL_PLUS.to_string(),
 &Self::BellRing => BELL_RING.to_string(),
 &Self::Bell => BELL.to_string(),
 &Self::Bike => BIKE.to_string(),
 &Self::Binary => BINARY.to_string(),
 &Self::Biohazard => BIOHAZARD.to_string(),
 &Self::Bird => BIRD.to_string(),
 &Self::Bitcoin => BITCOIN.to_string(),
 &Self::Blinds => BLINDS.to_string(),
 &Self::Blocks => BLOCKS.to_string(),
 &Self::BluetoothConnected => BLUETOOTH_CONNECTED.to_string(),
 &Self::BluetoothOff => BLUETOOTH_OFF.to_string(),
 &Self::BluetoothSearching => BLUETOOTH_SEARCHING.to_string(),
 &Self::Bluetooth => BLUETOOTH.to_string(),
 &Self::Bold => BOLD.to_string(),
 &Self::Bomb => BOMB.to_string(),
 &Self::Bone => BONE.to_string(),
 &Self::BookCopy => BOOK_COPY.to_string(),
 &Self::BookDown => BOOK_DOWN.to_string(),
 &Self::BookKey => BOOK_KEY.to_string(),
 &Self::BookLock => BOOK_LOCK.to_string(),
 &Self::BookMarked => BOOK_MARKED.to_string(),
 &Self::BookMinus => BOOK_MINUS.to_string(),
 &Self::BookOpenCheck => BOOK_OPEN_CHECK.to_string(),
 &Self::BookOpen => BOOK_OPEN.to_string(),
 &Self::BookPlus => BOOK_PLUS.to_string(),
 &Self::BookTemplate => BOOK_TEMPLATE.to_string(),
 &Self::BookUp2 => BOOK_UP_2.to_string(),
 &Self::BookUp => BOOK_UP.to_string(),
 &Self::BookX => BOOK_X.to_string(),
 &Self::Book => BOOK.to_string(),
 &Self::BookmarkMinus => BOOKMARK_MINUS.to_string(),
 &Self::BookmarkPlus => BOOKMARK_PLUS.to_string(),
 &Self::Bookmark => BOOKMARK.to_string(),
 &Self::BoomBox => BOOM_BOX.to_string(),
 &Self::Bot => BOT.to_string(),
 &Self::BoxSelect => BOX_SELECT.to_string(),
 &Self::Box => BOX.to_string(),
 &Self::Boxes => BOXES.to_string(),
 &Self::Braces => BRACES.to_string(),
 &Self::Brackets => BRACKETS.to_string(),
 &Self::BrainCircuit => BRAIN_CIRCUIT.to_string(),
 &Self::BrainCog => BRAIN_COG.to_string(),
 &Self::Brain => BRAIN.to_string(),
 &Self::Briefcase => BRIEFCASE.to_string(),
 &Self::BringToFront => BRING_TO_FRONT.to_string(),
 &Self::Brush => BRUSH.to_string(),
 &Self::BugOff => BUG_OFF.to_string(),
 &Self::BugPlay => BUG_PLAY.to_string(),
 &Self::Bug => BUG.to_string(),
 &Self::Building2 => BUILDING_2.to_string(),
 &Self::Building => BUILDING.to_string(),
 &Self::BusFront => BUS_FRONT.to_string(),
 &Self::Bus => BUS.to_string(),
 &Self::CableCar => CABLE_CAR.to_string(),
 &Self::Cable => CABLE.to_string(),
 &Self::CakeSlice => CAKE_SLICE.to_string(),
 &Self::Cake => CAKE.to_string(),
 &Self::Calculator => CALCULATOR.to_string(),
 &Self::CalendarCheck2 => CALENDAR_CHECK_2.to_string(),
 &Self::CalendarCheck => CALENDAR_CHECK.to_string(),
 &Self::CalendarClock => CALENDAR_CLOCK.to_string(),
 &Self::CalendarDays => CALENDAR_DAYS.to_string(),
 &Self::CalendarHeart => CALENDAR_HEART.to_string(),
 &Self::CalendarMinus => CALENDAR_MINUS.to_string(),
 &Self::CalendarOff => CALENDAR_OFF.to_string(),
 &Self::CalendarPlus => CALENDAR_PLUS.to_string(),
 &Self::CalendarRange => CALENDAR_RANGE.to_string(),
 &Self::CalendarSearch => CALENDAR_SEARCH.to_string(),
 &Self::CalendarX2 => CALENDAR_X_2.to_string(),
 &Self::CalendarX => CALENDAR_X.to_string(),
 &Self::Calendar => CALENDAR.to_string(),
 &Self::CameraOff => CAMERA_OFF.to_string(),
 &Self::Camera => CAMERA.to_string(),
 &Self::CandlestickChart => CANDLESTICK_CHART.to_string(),
 &Self::CandyCane => CANDY_CANE.to_string(),
 &Self::CandyOff => CANDY_OFF.to_string(),
 &Self::Candy => CANDY.to_string(),
 &Self::CarFront => CAR_FRONT.to_string(),
 &Self::CarTaxiFront => CAR_TAXI_FRONT.to_string(),
 &Self::Car => CAR.to_string(),
 &Self::Carrot => CARROT.to_string(),
 &Self::CaseLower => CASE_LOWER.to_string(),
 &Self::CaseSensitive => CASE_SENSITIVE.to_string(),
 &Self::CaseUpper => CASE_UPPER.to_string(),
 &Self::CassetteTape => CASSETTE_TAPE.to_string(),
 &Self::Cast => CAST.to_string(),
 &Self::Castle => CASTLE.to_string(),
 &Self::Cat => CAT.to_string(),
 &Self::CheckCheck => CHECK_CHECK.to_string(),
 &Self::CheckCircle2 => CHECK_CIRCLE_2.to_string(),
 &Self::CheckCircle => CHECK_CIRCLE.to_string(),
 &Self::CheckSquare => CHECK_SQUARE.to_string(),
 &Self::Check => CHECK.to_string(),
 &Self::ChefHat => CHEF_HAT.to_string(),
 &Self::Cherry => CHERRY.to_string(),
 &Self::ChevronDownCircle => CHEVRON_DOWN_CIRCLE.to_string(),
 &Self::ChevronDownSquare => CHEVRON_DOWN_SQUARE.to_string(),
 &Self::ChevronDown => CHEVRON_DOWN.to_string(),
 &Self::ChevronFirst => CHEVRON_FIRST.to_string(),
 &Self::ChevronLast => CHEVRON_LAST.to_string(),
 &Self::ChevronLeftCircle => CHEVRON_LEFT_CIRCLE.to_string(),
 &Self::ChevronLeftSquare => CHEVRON_LEFT_SQUARE.to_string(),
 &Self::ChevronLeft => CHEVRON_LEFT.to_string(),
 &Self::ChevronRightCircle => CHEVRON_RIGHT_CIRCLE.to_string(),
 &Self::ChevronRightSquare => CHEVRON_RIGHT_SQUARE.to_string(),
 &Self::ChevronRight => CHEVRON_RIGHT.to_string(),
 &Self::ChevronUpCircle => CHEVRON_UP_CIRCLE.to_string(),
 &Self::ChevronUpSquare => CHEVRON_UP_SQUARE.to_string(),
 &Self::ChevronUp => CHEVRON_UP.to_string(),
 &Self::ChevronsDownUp => CHEVRONS_DOWN_UP.to_string(),
 &Self::ChevronsDown => CHEVRONS_DOWN.to_string(),
 &Self::ChevronsLeftRight => CHEVRONS_LEFT_RIGHT.to_string(),
 &Self::ChevronsLeft => CHEVRONS_LEFT.to_string(),
 &Self::ChevronsRightLeft => CHEVRONS_RIGHT_LEFT.to_string(),
 &Self::ChevronsRight => CHEVRONS_RIGHT.to_string(),
 &Self::ChevronsUpDown => CHEVRONS_UP_DOWN.to_string(),
 &Self::ChevronsUp => CHEVRONS_UP.to_string(),
 &Self::Chrome => CHROME.to_string(),
 &Self::Church => CHURCH.to_string(),
 &Self::CigaretteOff => CIGARETTE_OFF.to_string(),
 &Self::Cigarette => CIGARETTE.to_string(),
 &Self::CircleDashed => CIRCLE_DASHED.to_string(),
 &Self::CircleDollarSign => CIRCLE_DOLLAR_SIGN.to_string(),
 &Self::CircleDotDashed => CIRCLE_DOT_DASHED.to_string(),
 &Self::CircleDot => CIRCLE_DOT.to_string(),
 &Self::CircleEllipsis => CIRCLE_ELLIPSIS.to_string(),
 &Self::CircleEqual => CIRCLE_EQUAL.to_string(),
 &Self::CircleOff => CIRCLE_OFF.to_string(),
 &Self::CircleSlash2 => CIRCLE_SLASH_2.to_string(),
 &Self::CircleSlash => CIRCLE_SLASH.to_string(),
 &Self::Circle => CIRCLE.to_string(),
 &Self::CircuitBoard => CIRCUIT_BOARD.to_string(),
 &Self::Citrus => CITRUS.to_string(),
 &Self::Clapperboard => CLAPPERBOARD.to_string(),
 &Self::ClipboardCheck => CLIPBOARD_CHECK.to_string(),
 &Self::ClipboardCopy => CLIPBOARD_COPY.to_string(),
 &Self::ClipboardEdit => CLIPBOARD_EDIT.to_string(),
 &Self::ClipboardList => CLIPBOARD_LIST.to_string(),
 &Self::ClipboardPaste => CLIPBOARD_PASTE.to_string(),
 &Self::ClipboardSignature => CLIPBOARD_SIGNATURE.to_string(),
 &Self::ClipboardType => CLIPBOARD_TYPE.to_string(),
 &Self::ClipboardX => CLIPBOARD_X.to_string(),
 &Self::Clipboard => CLIPBOARD.to_string(),
 &Self::Clock1 => CLOCK_1.to_string(),
 &Self::Clock10 => CLOCK_10.to_string(),
 &Self::Clock11 => CLOCK_11.to_string(),
 &Self::Clock12 => CLOCK_12.to_string(),
 &Self::Clock2 => CLOCK_2.to_string(),
 &Self::Clock3 => CLOCK_3.to_string(),
 &Self::Clock4 => CLOCK_4.to_string(),
 &Self::Clock5 => CLOCK_5.to_string(),
 &Self::Clock6 => CLOCK_6.to_string(),
 &Self::Clock7 => CLOCK_7.to_string(),
 &Self::Clock8 => CLOCK_8.to_string(),
 &Self::Clock9 => CLOCK_9.to_string(),
 &Self::Clock => CLOCK.to_string(),
 &Self::CloudCog => CLOUD_COG.to_string(),
 &Self::CloudDrizzle => CLOUD_DRIZZLE.to_string(),
 &Self::CloudFog => CLOUD_FOG.to_string(),
 &Self::CloudHail => CLOUD_HAIL.to_string(),
 &Self::CloudLightning => CLOUD_LIGHTNING.to_string(),
 &Self::CloudMoonRain => CLOUD_MOON_RAIN.to_string(),
 &Self::CloudMoon => CLOUD_MOON.to_string(),
 &Self::CloudOff => CLOUD_OFF.to_string(),
 &Self::CloudRainWind => CLOUD_RAIN_WIND.to_string(),
 &Self::CloudRain => CLOUD_RAIN.to_string(),
 &Self::CloudSnow => CLOUD_SNOW.to_string(),
 &Self::CloudSunRain => CLOUD_SUN_RAIN.to_string(),
 &Self::CloudSun => CLOUD_SUN.to_string(),
 &Self::Cloud => CLOUD.to_string(),
 &Self::Cloudy => CLOUDY.to_string(),
 &Self::Clover => CLOVER.to_string(),
 &Self::Club => CLUB.to_string(),
 &Self::Code2 => CODE_2.to_string(),
 &Self::Code => CODE.to_string(),
 &Self::Codepen => CODEPEN.to_string(),
 &Self::Codesandbox => CODESANDBOX.to_string(),
 &Self::Coffee => COFFEE.to_string(),
 &Self::Cog => COG.to_string(),
 &Self::Coins => COINS.to_string(),
 &Self::Columns => COLUMNS.to_string(),
 &Self::Combine => COMBINE.to_string(),
 &Self::Command => COMMAND.to_string(),
 &Self::Compass => COMPASS.to_string(),
 &Self::Component => COMPONENT.to_string(),
 &Self::Computer => COMPUTER.to_string(),
 &Self::ConciergeBell => CONCIERGE_BELL.to_string(),
 &Self::Construction => CONSTRUCTION.to_string(),
 &Self::Contact2 => CONTACT_2.to_string(),
 &Self::Contact => CONTACT.to_string(),
 &Self::Container => CONTAINER.to_string(),
 &Self::Contrast => CONTRAST.to_string(),
 &Self::Cookie => COOKIE.to_string(),
 &Self::CopyCheck => COPY_CHECK.to_string(),
 &Self::CopyMinus => COPY_MINUS.to_string(),
 &Self::CopyPlus => COPY_PLUS.to_string(),
 &Self::CopySlash => COPY_SLASH.to_string(),
 &Self::CopyX => COPY_X.to_string(),
 &Self::Copy => COPY.to_string(),
 &Self::Copyleft => COPYLEFT.to_string(),
 &Self::Copyright => COPYRIGHT.to_string(),
 &Self::CornerDownLeft => CORNER_DOWN_LEFT.to_string(),
 &Self::CornerDownRight => CORNER_DOWN_RIGHT.to_string(),
 &Self::CornerLeftDown => CORNER_LEFT_DOWN.to_string(),
 &Self::CornerLeftUp => CORNER_LEFT_UP.to_string(),
 &Self::CornerRightDown => CORNER_RIGHT_DOWN.to_string(),
 &Self::CornerRightUp => CORNER_RIGHT_UP.to_string(),
 &Self::CornerUpLeft => CORNER_UP_LEFT.to_string(),
 &Self::CornerUpRight => CORNER_UP_RIGHT.to_string(),
 &Self::Cpu => CPU.to_string(),
 &Self::CreativeCommons => CREATIVE_COMMONS.to_string(),
 &Self::CreditCard => CREDIT_CARD.to_string(),
 &Self::Croissant => CROISSANT.to_string(),
 &Self::Crop => CROP.to_string(),
 &Self::Cross => CROSS.to_string(),
 &Self::Crosshair => CROSSHAIR.to_string(),
 &Self::Crown => CROWN.to_string(),
 &Self::CupSoda => CUP_SODA.to_string(),
 &Self::Currency => CURRENCY.to_string(),
 &Self::DatabaseBackup => DATABASE_BACKUP.to_string(),
 &Self::DatabaseZap => DATABASE_ZAP.to_string(),
 &Self::Database => DATABASE.to_string(),
 &Self::Delete => DELETE.to_string(),
 &Self::Dessert => DESSERT.to_string(),
 &Self::Diamond => DIAMOND.to_string(),
 &Self::Dice1 => DICE_1.to_string(),
 &Self::Dice2 => DICE_2.to_string(),
 &Self::Dice3 => DICE_3.to_string(),
 &Self::Dice4 => DICE_4.to_string(),
 &Self::Dice5 => DICE_5.to_string(),
 &Self::Dice6 => DICE_6.to_string(),
 &Self::Dices => DICES.to_string(),
 &Self::Diff => DIFF.to_string(),
 &Self::Disc2 => DISC_2.to_string(),
 &Self::Disc3 => DISC_3.to_string(),
 &Self::Disc => DISC.to_string(),
 &Self::DivideCircle => DIVIDE_CIRCLE.to_string(),
 &Self::DivideSquare => DIVIDE_SQUARE.to_string(),
 &Self::Divide => DIVIDE.to_string(),
 &Self::DnaOff => DNA_OFF.to_string(),
 &Self::Dna => DNA.to_string(),
 &Self::Dog => DOG.to_string(),
 &Self::DollarSign => DOLLAR_SIGN.to_string(),
 &Self::Donut => DONUT.to_string(),
 &Self::DoorClosed => DOOR_CLOSED.to_string(),
 &Self::DoorOpen => DOOR_OPEN.to_string(),
 &Self::Dot => DOT.to_string(),
 &Self::DownloadCloud => DOWNLOAD_CLOUD.to_string(),
 &Self::Download => DOWNLOAD.to_string(),
 &Self::Dribbble => DRIBBBLE.to_string(),
 &Self::Droplet => DROPLET.to_string(),
 &Self::Droplets => DROPLETS.to_string(),
 &Self::Drumstick => DRUMSTICK.to_string(),
 &Self::Dumbbell => DUMBBELL.to_string(),
 &Self::EarOff => EAR_OFF.to_string(),
 &Self::Ear => EAR.to_string(),
 &Self::EggFried => EGG_FRIED.to_string(),
 &Self::EggOff => EGG_OFF.to_string(),
 &Self::Egg => EGG.to_string(),
 &Self::EqualNot => EQUAL_NOT.to_string(),
 &Self::Equal => EQUAL.to_string(),
 &Self::Eraser => ERASER.to_string(),
 &Self::Euro => EURO.to_string(),
 &Self::Expand => EXPAND.to_string(),
 &Self::ExternalLink => EXTERNAL_LINK.to_string(),
 &Self::EyeOff => EYE_OFF.to_string(),
 &Self::Eye => EYE.to_string(),
 &Self::Facebook => FACEBOOK.to_string(),
 &Self::Factory => FACTORY.to_string(),
 &Self::Fan => FAN.to_string(),
 &Self::FastForward => FAST_FORWARD.to_string(),
 &Self::Feather => FEATHER.to_string(),
 &Self::FerrisWheel => FERRIS_WHEEL.to_string(),
 &Self::Figma => FIGMA.to_string(),
 &Self::FileArchive => FILE_ARCHIVE.to_string(),
 &Self::FileAudio2 => FILE_AUDIO_2.to_string(),
 &Self::FileAudio => FILE_AUDIO.to_string(),
 &Self::FileAxis3D => FILE_AXIS_3_D.to_string(),
 &Self::FileBadge2 => FILE_BADGE_2.to_string(),
 &Self::FileBadge => FILE_BADGE.to_string(),
 &Self::FileBarChart2 => FILE_BAR_CHART_2.to_string(),
 &Self::FileBarChart => FILE_BAR_CHART.to_string(),
 &Self::FileBox => FILE_BOX.to_string(),
 &Self::FileCheck2 => FILE_CHECK_2.to_string(),
 &Self::FileCheck => FILE_CHECK.to_string(),
 &Self::FileClock => FILE_CLOCK.to_string(),
 &Self::FileCode2 => FILE_CODE_2.to_string(),
 &Self::FileCode => FILE_CODE.to_string(),
 &Self::FileCog => FILE_COG.to_string(),
 &Self::FileDiff => FILE_DIFF.to_string(),
 &Self::FileDigit => FILE_DIGIT.to_string(),
 &Self::FileDown => FILE_DOWN.to_string(),
 &Self::FileEdit => FILE_EDIT.to_string(),
 &Self::FileHeart => FILE_HEART.to_string(),
 &Self::FileImage => FILE_IMAGE.to_string(),
 &Self::FileInput => FILE_INPUT.to_string(),
 &Self::FileJson2 => FILE_JSON_2.to_string(),
 &Self::FileJson => FILE_JSON.to_string(),
 &Self::FileKey2 => FILE_KEY_2.to_string(),
 &Self::FileKey => FILE_KEY.to_string(),
 &Self::FileLineChart => FILE_LINE_CHART.to_string(),
 &Self::FileLock2 => FILE_LOCK_2.to_string(),
 &Self::FileLock => FILE_LOCK.to_string(),
 &Self::FileMinus2 => FILE_MINUS_2.to_string(),
 &Self::FileMinus => FILE_MINUS.to_string(),
 &Self::FileOutput => FILE_OUTPUT.to_string(),
 &Self::FilePieChart => FILE_PIE_CHART.to_string(),
 &Self::FilePlus2 => FILE_PLUS_2.to_string(),
 &Self::FilePlus => FILE_PLUS.to_string(),
 &Self::FileQuestion => FILE_QUESTION.to_string(),
 &Self::FileScan => FILE_SCAN.to_string(),
 &Self::FileSearch2 => FILE_SEARCH_2.to_string(),
 &Self::FileSearch => FILE_SEARCH.to_string(),
 &Self::FileSignature => FILE_SIGNATURE.to_string(),
 &Self::FileSpreadsheet => FILE_SPREADSHEET.to_string(),
 &Self::FileStack => FILE_STACK.to_string(),
 &Self::FileSymlink => FILE_SYMLINK.to_string(),
 &Self::FileTerminal => FILE_TERMINAL.to_string(),
 &Self::FileText => FILE_TEXT.to_string(),
 &Self::FileType2 => FILE_TYPE_2.to_string(),
 &Self::FileType => FILE_TYPE.to_string(),
 &Self::FileUp => FILE_UP.to_string(),
 &Self::FileVideo2 => FILE_VIDEO_2.to_string(),
 &Self::FileVideo => FILE_VIDEO.to_string(),
 &Self::FileVolume2 => FILE_VOLUME_2.to_string(),
 &Self::FileVolume => FILE_VOLUME.to_string(),
 &Self::FileWarning => FILE_WARNING.to_string(),
 &Self::FileX2 => FILE_X_2.to_string(),
 &Self::FileX => FILE_X.to_string(),
 &Self::File => FILE.to_string(),
 &Self::Files => FILES.to_string(),
 &Self::Film => FILM.to_string(),
 &Self::FilterX => FILTER_X.to_string(),
 &Self::Filter => FILTER.to_string(),
 &Self::Fingerprint => FINGERPRINT.to_string(),
 &Self::FishOff => FISH_OFF.to_string(),
 &Self::FishSymbol => FISH_SYMBOL.to_string(),
 &Self::Fish => FISH.to_string(),
 &Self::FlagOff => FLAG_OFF.to_string(),
 &Self::FlagTriangleLeft => FLAG_TRIANGLE_LEFT.to_string(),
 &Self::FlagTriangleRight => FLAG_TRIANGLE_RIGHT.to_string(),
 &Self::Flag => FLAG.to_string(),
 &Self::Flame => FLAME.to_string(),
 &Self::FlashlightOff => FLASHLIGHT_OFF.to_string(),
 &Self::Flashlight => FLASHLIGHT.to_string(),
 &Self::FlaskConicalOff => FLASK_CONICAL_OFF.to_string(),
 &Self::FlaskConical => FLASK_CONICAL.to_string(),
 &Self::FlaskRound => FLASK_ROUND.to_string(),
 &Self::FlipHorizontal2 => FLIP_HORIZONTAL_2.to_string(),
 &Self::FlipHorizontal => FLIP_HORIZONTAL.to_string(),
 &Self::FlipVertical2 => FLIP_VERTICAL_2.to_string(),
 &Self::FlipVertical => FLIP_VERTICAL.to_string(),
 &Self::Flower2 => FLOWER_2.to_string(),
 &Self::Flower => FLOWER.to_string(),
 &Self::Focus => FOCUS.to_string(),
 &Self::FoldHorizontal => FOLD_HORIZONTAL.to_string(),
 &Self::FoldVertical => FOLD_VERTICAL.to_string(),
 &Self::FolderArchive => FOLDER_ARCHIVE.to_string(),
 &Self::FolderCheck => FOLDER_CHECK.to_string(),
 &Self::FolderClock => FOLDER_CLOCK.to_string(),
 &Self::FolderClosed => FOLDER_CLOSED.to_string(),
 &Self::FolderCog => FOLDER_COG.to_string(),
 &Self::FolderDot => FOLDER_DOT.to_string(),
 &Self::FolderDown => FOLDER_DOWN.to_string(),
 &Self::FolderEdit => FOLDER_EDIT.to_string(),
 &Self::FolderGit2 => FOLDER_GIT_2.to_string(),
 &Self::FolderGit => FOLDER_GIT.to_string(),
 &Self::FolderHeart => FOLDER_HEART.to_string(),
 &Self::FolderInput => FOLDER_INPUT.to_string(),
 &Self::FolderKanban => FOLDER_KANBAN.to_string(),
 &Self::FolderKey => FOLDER_KEY.to_string(),
 &Self::FolderLock => FOLDER_LOCK.to_string(),
 &Self::FolderMinus => FOLDER_MINUS.to_string(),
 &Self::FolderOpenDot => FOLDER_OPEN_DOT.to_string(),
 &Self::FolderOpen => FOLDER_OPEN.to_string(),
 &Self::FolderOutput => FOLDER_OUTPUT.to_string(),
 &Self::FolderPlus => FOLDER_PLUS.to_string(),
 &Self::FolderRoot => FOLDER_ROOT.to_string(),
 &Self::FolderSearch2 => FOLDER_SEARCH_2.to_string(),
 &Self::FolderSearch => FOLDER_SEARCH.to_string(),
 &Self::FolderSymlink => FOLDER_SYMLINK.to_string(),
 &Self::FolderSync => FOLDER_SYNC.to_string(),
 &Self::FolderTree => FOLDER_TREE.to_string(),
 &Self::FolderUp => FOLDER_UP.to_string(),
 &Self::FolderX => FOLDER_X.to_string(),
 &Self::Folder => FOLDER.to_string(),
 &Self::Folders => FOLDERS.to_string(),
 &Self::Footprints => FOOTPRINTS.to_string(),
 &Self::Forklift => FORKLIFT.to_string(),
 &Self::FormInput => FORM_INPUT.to_string(),
 &Self::Forward => FORWARD.to_string(),
 &Self::Frame => FRAME.to_string(),
 &Self::Framer => FRAMER.to_string(),
 &Self::Frown => FROWN.to_string(),
 &Self::Fuel => FUEL.to_string(),
 &Self::FunctionSquare => FUNCTION_SQUARE.to_string(),
 &Self::GalleryHorizontalEnd => GALLERY_HORIZONTAL_END.to_string(),
 &Self::GalleryHorizontal => GALLERY_HORIZONTAL.to_string(),
 &Self::GalleryThumbnails => GALLERY_THUMBNAILS.to_string(),
 &Self::GalleryVerticalEnd => GALLERY_VERTICAL_END.to_string(),
 &Self::GalleryVertical => GALLERY_VERTICAL.to_string(),
 &Self::Gamepad2 => GAMEPAD_2.to_string(),
 &Self::Gamepad => GAMEPAD.to_string(),
 &Self::GanttChartSquare => GANTT_CHART_SQUARE.to_string(),
 &Self::GanttChart => GANTT_CHART.to_string(),
 &Self::GaugeCircle => GAUGE_CIRCLE.to_string(),
 &Self::Gauge => GAUGE.to_string(),
 &Self::Gavel => GAVEL.to_string(),
 &Self::Gem => GEM.to_string(),
 &Self::Ghost => GHOST.to_string(),
 &Self::Gift => GIFT.to_string(),
 &Self::GitBranchPlus => GIT_BRANCH_PLUS.to_string(),
 &Self::GitBranch => GIT_BRANCH.to_string(),
 &Self::GitCommit => GIT_COMMIT.to_string(),
 &Self::GitCompare => GIT_COMPARE.to_string(),
 &Self::GitFork => GIT_FORK.to_string(),
 &Self::GitMerge => GIT_MERGE.to_string(),
 &Self::GitPullRequestClosed => GIT_PULL_REQUEST_CLOSED.to_string(),
 &Self::GitPullRequestDraft => GIT_PULL_REQUEST_DRAFT.to_string(),
 &Self::GitPullRequest => GIT_PULL_REQUEST.to_string(),
 &Self::Github => GITHUB.to_string(),
 &Self::Gitlab => GITLAB.to_string(),
 &Self::GlassWater => GLASS_WATER.to_string(),
 &Self::Glasses => GLASSES.to_string(),
 &Self::Globe2 => GLOBE_2.to_string(),
 &Self::Globe => GLOBE.to_string(),
 &Self::Goal => GOAL.to_string(),
 &Self::Grab => GRAB.to_string(),
 &Self::GraduationCap => GRADUATION_CAP.to_string(),
 &Self::Grape => GRAPE.to_string(),
 &Self::Grid2X2 => GRID_2_X_2.to_string(),
 &Self::Grid3X3 => GRID_3_X_3.to_string(),
 &Self::GripHorizontal => GRIP_HORIZONTAL.to_string(),
 &Self::GripVertical => GRIP_VERTICAL.to_string(),
 &Self::Grip => GRIP.to_string(),
 &Self::Group => GROUP.to_string(),
 &Self::Hammer => HAMMER.to_string(),
 &Self::HandMetal => HAND_METAL.to_string(),
 &Self::Hand => HAND.to_string(),
 &Self::HardDriveDownload => HARD_DRIVE_DOWNLOAD.to_string(),
 &Self::HardDriveUpload => HARD_DRIVE_UPLOAD.to_string(),
 &Self::HardDrive => HARD_DRIVE.to_string(),
 &Self::HardHat => HARD_HAT.to_string(),
 &Self::Hash => HASH.to_string(),
 &Self::Haze => HAZE.to_string(),
 &Self::HdmiPort => HDMI_PORT.to_string(),
 &Self::Heading1 => HEADING_1.to_string(),
 &Self::Heading2 => HEADING_2.to_string(),
 &Self::Heading3 => HEADING_3.to_string(),
 &Self::Heading4 => HEADING_4.to_string(),
 &Self::Heading5 => HEADING_5.to_string(),
 &Self::Heading6 => HEADING_6.to_string(),
 &Self::Heading => HEADING.to_string(),
 &Self::Headphones => HEADPHONES.to_string(),
 &Self::HeartCrack => HEART_CRACK.to_string(),
 &Self::HeartHandshake => HEART_HANDSHAKE.to_string(),
 &Self::HeartOff => HEART_OFF.to_string(),
 &Self::HeartPulse => HEART_PULSE.to_string(),
 &Self::Heart => HEART.to_string(),
 &Self::HelpCircle => HELP_CIRCLE.to_string(),
 &Self::HelpingHand => HELPING_HAND.to_string(),
 &Self::Hexagon => HEXAGON.to_string(),
 &Self::Highlighter => HIGHLIGHTER.to_string(),
 &Self::History => HISTORY.to_string(),
 &Self::Home => HOME.to_string(),
 &Self::HopOff => HOP_OFF.to_string(),
 &Self::Hop => HOP.to_string(),
 &Self::Hotel => HOTEL.to_string(),
 &Self::Hourglass => HOURGLASS.to_string(),
 &Self::IceCream2 => ICE_CREAM_2.to_string(),
 &Self::IceCream => ICE_CREAM.to_string(),
 &Self::ImageMinus => IMAGE_MINUS.to_string(),
 &Self::ImageOff => IMAGE_OFF.to_string(),
 &Self::ImagePlus => IMAGE_PLUS.to_string(),
 &Self::Image => IMAGE.to_string(),
 &Self::Import => IMPORT.to_string(),
 &Self::Inbox => INBOX.to_string(),
 &Self::Indent => INDENT.to_string(),
 &Self::IndianRupee => INDIAN_RUPEE.to_string(),
 &Self::Infinity => INFINITY.to_string(),
 &Self::Info => INFO.to_string(),
 &Self::Instagram => INSTAGRAM.to_string(),
 &Self::Italic => ITALIC.to_string(),
 &Self::IterationCcw => ITERATION_CCW.to_string(),
 &Self::IterationCw => ITERATION_CW.to_string(),
 &Self::JapaneseYen => JAPANESE_YEN.to_string(),
 &Self::Joystick => JOYSTICK.to_string(),
 &Self::KanbanSquareDashed => KANBAN_SQUARE_DASHED.to_string(),
 &Self::KanbanSquare => KANBAN_SQUARE.to_string(),
 &Self::Kanban => KANBAN.to_string(),
 &Self::KeyRound => KEY_ROUND.to_string(),
 &Self::KeySquare => KEY_SQUARE.to_string(),
 &Self::Key => KEY.to_string(),
 &Self::Keyboard => KEYBOARD.to_string(),
 &Self::LampCeiling => LAMP_CEILING.to_string(),
 &Self::LampDesk => LAMP_DESK.to_string(),
 &Self::LampFloor => LAMP_FLOOR.to_string(),
 &Self::LampWallDown => LAMP_WALL_DOWN.to_string(),
 &Self::LampWallUp => LAMP_WALL_UP.to_string(),
 &Self::Lamp => LAMP.to_string(),
 &Self::Landmark => LANDMARK.to_string(),
 &Self::Languages => LANGUAGES.to_string(),
 &Self::Laptop2 => LAPTOP_2.to_string(),
 &Self::Laptop => LAPTOP.to_string(),
 &Self::LassoSelect => LASSO_SELECT.to_string(),
 &Self::Lasso => LASSO.to_string(),
 &Self::Laugh => LAUGH.to_string(),
 &Self::Layers => LAYERS.to_string(),
 &Self::LayoutDashboard => LAYOUT_DASHBOARD.to_string(),
 &Self::LayoutGrid => LAYOUT_GRID.to_string(),
 &Self::LayoutList => LAYOUT_LIST.to_string(),
 &Self::LayoutPanelLeft => LAYOUT_PANEL_LEFT.to_string(),
 &Self::LayoutPanelTop => LAYOUT_PANEL_TOP.to_string(),
 &Self::LayoutTemplate => LAYOUT_TEMPLATE.to_string(),
 &Self::Layout => LAYOUT.to_string(),
 &Self::Leaf => LEAF.to_string(),
 &Self::LeafyGreen => LEAFY_GREEN.to_string(),
 &Self::Library => LIBRARY.to_string(),
 &Self::LifeBuoy => LIFE_BUOY.to_string(),
 &Self::Ligature => LIGATURE.to_string(),
 &Self::LightbulbOff => LIGHTBULB_OFF.to_string(),
 &Self::Lightbulb => LIGHTBULB.to_string(),
 &Self::LineChart => LINE_CHART.to_string(),
 &Self::Link2Off => LINK_2_OFF.to_string(),
 &Self::Link2 => LINK_2.to_string(),
 &Self::Link => LINK.to_string(),
 &Self::Linkedin => LINKEDIN.to_string(),
 &Self::ListChecks => LIST_CHECKS.to_string(),
 &Self::ListEnd => LIST_END.to_string(),
 &Self::ListFilter => LIST_FILTER.to_string(),
 &Self::ListMinus => LIST_MINUS.to_string(),
 &Self::ListMusic => LIST_MUSIC.to_string(),
 &Self::ListOrdered => LIST_ORDERED.to_string(),
 &Self::ListPlus => LIST_PLUS.to_string(),
 &Self::ListRestart => LIST_RESTART.to_string(),
 &Self::ListStart => LIST_START.to_string(),
 &Self::ListTodo => LIST_TODO.to_string(),
 &Self::ListTree => LIST_TREE.to_string(),
 &Self::ListVideo => LIST_VIDEO.to_string(),
 &Self::ListX => LIST_X.to_string(),
 &Self::List => LIST.to_string(),
 &Self::Loader2 => LOADER_2.to_string(),
 &Self::Loader => LOADER.to_string(),
 &Self::LocateFixed => LOCATE_FIXED.to_string(),
 &Self::LocateOff => LOCATE_OFF.to_string(),
 &Self::Locate => LOCATE.to_string(),
 &Self::Lock => LOCK.to_string(),
 &Self::LogIn => LOG_IN.to_string(),
 &Self::LogOut => LOG_OUT.to_string(),
 &Self::Lollipop => LOLLIPOP.to_string(),
 &Self::Luggage => LUGGAGE.to_string(),
 &Self::MSquare => M_SQUARE.to_string(),
 &Self::Magnet => MAGNET.to_string(),
 &Self::MailCheck => MAIL_CHECK.to_string(),
 &Self::MailMinus => MAIL_MINUS.to_string(),
 &Self::MailOpen => MAIL_OPEN.to_string(),
 &Self::MailPlus => MAIL_PLUS.to_string(),
 &Self::MailQuestion => MAIL_QUESTION.to_string(),
 &Self::MailSearch => MAIL_SEARCH.to_string(),
 &Self::MailWarning => MAIL_WARNING.to_string(),
 &Self::MailX => MAIL_X.to_string(),
 &Self::Mail => MAIL.to_string(),
 &Self::Mailbox => MAILBOX.to_string(),
 &Self::Mails => MAILS.to_string(),
 &Self::MapPinOff => MAP_PIN_OFF.to_string(),
 &Self::MapPin => MAP_PIN.to_string(),
 &Self::Map => MAP.to_string(),
 &Self::Martini => MARTINI.to_string(),
 &Self::Maximize2 => MAXIMIZE_2.to_string(),
 &Self::Maximize => MAXIMIZE.to_string(),
 &Self::Medal => MEDAL.to_string(),
 &Self::MegaphoneOff => MEGAPHONE_OFF.to_string(),
 &Self::Megaphone => MEGAPHONE.to_string(),
 &Self::Meh => MEH.to_string(),
 &Self::MemoryStick => MEMORY_STICK.to_string(),
 &Self::MenuSquare => MENU_SQUARE.to_string(),
 &Self::Menu => MENU.to_string(),
 &Self::Merge => MERGE.to_string(),
 &Self::MessageCircle => MESSAGE_CIRCLE.to_string(),
 &Self::MessageSquareDashed => MESSAGE_SQUARE_DASHED.to_string(),
 &Self::MessageSquarePlus => MESSAGE_SQUARE_PLUS.to_string(),
 &Self::MessageSquare => MESSAGE_SQUARE.to_string(),
 &Self::MessagesSquare => MESSAGES_SQUARE.to_string(),
 &Self::Mic2 => MIC_2.to_string(),
 &Self::MicOff => MIC_OFF.to_string(),
 &Self::Mic => MIC.to_string(),
 &Self::Microscope => MICROSCOPE.to_string(),
 &Self::Microwave => MICROWAVE.to_string(),
 &Self::Milestone => MILESTONE.to_string(),
 &Self::MilkOff => MILK_OFF.to_string(),
 &Self::Milk => MILK.to_string(),
 &Self::Minimize2 => MINIMIZE_2.to_string(),
 &Self::Minimize => MINIMIZE.to_string(),
 &Self::MinusCircle => MINUS_CIRCLE.to_string(),
 &Self::MinusSquare => MINUS_SQUARE.to_string(),
 &Self::Minus => MINUS.to_string(),
 &Self::MonitorCheck => MONITOR_CHECK.to_string(),
 &Self::MonitorDot => MONITOR_DOT.to_string(),
 &Self::MonitorDown => MONITOR_DOWN.to_string(),
 &Self::MonitorOff => MONITOR_OFF.to_string(),
 &Self::MonitorPause => MONITOR_PAUSE.to_string(),
 &Self::MonitorPlay => MONITOR_PLAY.to_string(),
 &Self::MonitorSmartphone => MONITOR_SMARTPHONE.to_string(),
 &Self::MonitorSpeaker => MONITOR_SPEAKER.to_string(),
 &Self::MonitorStop => MONITOR_STOP.to_string(),
 &Self::MonitorUp => MONITOR_UP.to_string(),
 &Self::MonitorX => MONITOR_X.to_string(),
 &Self::Monitor => MONITOR.to_string(),
 &Self::MoonStar => MOON_STAR.to_string(),
 &Self::Moon => MOON.to_string(),
 &Self::MoreHorizontal => MORE_HORIZONTAL.to_string(),
 &Self::MoreVertical => MORE_VERTICAL.to_string(),
 &Self::MountainSnow => MOUNTAIN_SNOW.to_string(),
 &Self::Mountain => MOUNTAIN.to_string(),
 &Self::MousePointer2 => MOUSE_POINTER_2.to_string(),
 &Self::MousePointerClick => MOUSE_POINTER_CLICK.to_string(),
 &Self::MousePointerSquareDashed => MOUSE_POINTER_SQUARE_DASHED.to_string(),
 &Self::MousePointerSquare => MOUSE_POINTER_SQUARE.to_string(),
 &Self::MousePointer => MOUSE_POINTER.to_string(),
 &Self::Mouse => MOUSE.to_string(),
 &Self::Move3D => MOVE_3_D.to_string(),
 &Self::MoveDiagonal2 => MOVE_DIAGONAL_2.to_string(),
 &Self::MoveDiagonal => MOVE_DIAGONAL.to_string(),
 &Self::MoveDownLeft => MOVE_DOWN_LEFT.to_string(),
 &Self::MoveDownRight => MOVE_DOWN_RIGHT.to_string(),
 &Self::MoveDown => MOVE_DOWN.to_string(),
 &Self::MoveHorizontal => MOVE_HORIZONTAL.to_string(),
 &Self::MoveLeft => MOVE_LEFT.to_string(),
 &Self::MoveRight => MOVE_RIGHT.to_string(),
 &Self::MoveUpLeft => MOVE_UP_LEFT.to_string(),
 &Self::MoveUpRight => MOVE_UP_RIGHT.to_string(),
 &Self::MoveUp => MOVE_UP.to_string(),
 &Self::MoveVertical => MOVE_VERTICAL.to_string(),
 &Self::Move => MOVE.to_string(),
 &Self::Music2 => MUSIC_2.to_string(),
 &Self::Music3 => MUSIC_3.to_string(),
 &Self::Music4 => MUSIC_4.to_string(),
 &Self::Music => MUSIC.to_string(),
 &Self::Navigation2Off => NAVIGATION_2_OFF.to_string(),
 &Self::Navigation2 => NAVIGATION_2.to_string(),
 &Self::NavigationOff => NAVIGATION_OFF.to_string(),
 &Self::Navigation => NAVIGATION.to_string(),
 &Self::Network => NETWORK.to_string(),
 &Self::Newspaper => NEWSPAPER.to_string(),
 &Self::Nfc => NFC.to_string(),
 &Self::NutOff => NUT_OFF.to_string(),
 &Self::Nut => NUT.to_string(),
 &Self::Octagon => OCTAGON.to_string(),
 &Self::Option => OPTION.to_string(),
 &Self::Orbit => ORBIT.to_string(),
 &Self::Outdent => OUTDENT.to_string(),
 &Self::Package2 => PACKAGE_2.to_string(),
 &Self::PackageCheck => PACKAGE_CHECK.to_string(),
 &Self::PackageMinus => PACKAGE_MINUS.to_string(),
 &Self::PackageOpen => PACKAGE_OPEN.to_string(),
 &Self::PackagePlus => PACKAGE_PLUS.to_string(),
 &Self::PackageSearch => PACKAGE_SEARCH.to_string(),
 &Self::PackageX => PACKAGE_X.to_string(),
 &Self::Package => PACKAGE.to_string(),
 &Self::PaintBucket => PAINT_BUCKET.to_string(),
 &Self::Paintbrush2 => PAINTBRUSH_2.to_string(),
 &Self::Paintbrush => PAINTBRUSH.to_string(),
 &Self::Palette => PALETTE.to_string(),
 &Self::Palmtree => PALMTREE.to_string(),
 &Self::PanelBottomClose => PANEL_BOTTOM_CLOSE.to_string(),
 &Self::PanelBottomInactive => PANEL_BOTTOM_INACTIVE.to_string(),
 &Self::PanelBottomOpen => PANEL_BOTTOM_OPEN.to_string(),
 &Self::PanelBottom => PANEL_BOTTOM.to_string(),
 &Self::PanelLeftClose => PANEL_LEFT_CLOSE.to_string(),
 &Self::PanelLeftInactive => PANEL_LEFT_INACTIVE.to_string(),
 &Self::PanelLeftOpen => PANEL_LEFT_OPEN.to_string(),
 &Self::PanelLeft => PANEL_LEFT.to_string(),
 &Self::PanelRightClose => PANEL_RIGHT_CLOSE.to_string(),
 &Self::PanelRightInactive => PANEL_RIGHT_INACTIVE.to_string(),
 &Self::PanelRightOpen => PANEL_RIGHT_OPEN.to_string(),
 &Self::PanelRight => PANEL_RIGHT.to_string(),
 &Self::PanelTopClose => PANEL_TOP_CLOSE.to_string(),
 &Self::PanelTopInactive => PANEL_TOP_INACTIVE.to_string(),
 &Self::PanelTopOpen => PANEL_TOP_OPEN.to_string(),
 &Self::PanelTop => PANEL_TOP.to_string(),
 &Self::Paperclip => PAPERCLIP.to_string(),
 &Self::Parentheses => PARENTHESES.to_string(),
 &Self::ParkingCircleOff => PARKING_CIRCLE_OFF.to_string(),
 &Self::ParkingCircle => PARKING_CIRCLE.to_string(),
 &Self::ParkingMeter => PARKING_METER.to_string(),
 &Self::ParkingSquareOff => PARKING_SQUARE_OFF.to_string(),
 &Self::ParkingSquare => PARKING_SQUARE.to_string(),
 &Self::PartyPopper => PARTY_POPPER.to_string(),
 &Self::PauseCircle => PAUSE_CIRCLE.to_string(),
 &Self::PauseOctagon => PAUSE_OCTAGON.to_string(),
 &Self::Pause => PAUSE.to_string(),
 &Self::PawPrint => PAW_PRINT.to_string(),
 &Self::PcCase => PC_CASE.to_string(),
 &Self::PenLine => PEN_LINE.to_string(),
 &Self::PenSquare => PEN_SQUARE.to_string(),
 &Self::PenTool => PEN_TOOL.to_string(),
 &Self::Pen => PEN.to_string(),
 &Self::PencilLine => PENCIL_LINE.to_string(),
 &Self::PencilRuler => PENCIL_RULER.to_string(),
 &Self::Pencil => PENCIL.to_string(),
 &Self::PercentCircle => PERCENT_CIRCLE.to_string(),
 &Self::PercentDiamond => PERCENT_DIAMOND.to_string(),
 &Self::PercentSquare => PERCENT_SQUARE.to_string(),
 &Self::Percent => PERCENT.to_string(),
 &Self::PersonStanding => PERSON_STANDING.to_string(),
 &Self::PhoneCall => PHONE_CALL.to_string(),
 &Self::PhoneForwarded => PHONE_FORWARDED.to_string(),
 &Self::PhoneIncoming => PHONE_INCOMING.to_string(),
 &Self::PhoneMissed => PHONE_MISSED.to_string(),
 &Self::PhoneOff => PHONE_OFF.to_string(),
 &Self::PhoneOutgoing => PHONE_OUTGOING.to_string(),
 &Self::Phone => PHONE.to_string(),
 &Self::PiSquare => PI_SQUARE.to_string(),
 &Self::Pi => PI.to_string(),
 &Self::PictureInPicture2 => PICTURE_IN_PICTURE_2.to_string(),
 &Self::PictureInPicture => PICTURE_IN_PICTURE.to_string(),
 &Self::PieChart => PIE_CHART.to_string(),
 &Self::PiggyBank => PIGGY_BANK.to_string(),
 &Self::PilcrowSquare => PILCROW_SQUARE.to_string(),
 &Self::Pilcrow => PILCROW.to_string(),
 &Self::Pill => PILL.to_string(),
 &Self::PinOff => PIN_OFF.to_string(),
 &Self::Pin => PIN.to_string(),
 &Self::Pipette => PIPETTE.to_string(),
 &Self::Pizza => PIZZA.to_string(),
 &Self::PlaneLanding => PLANE_LANDING.to_string(),
 &Self::PlaneTakeoff => PLANE_TAKEOFF.to_string(),
 &Self::Plane => PLANE.to_string(),
 &Self::PlayCircle => PLAY_CIRCLE.to_string(),
 &Self::PlaySquare => PLAY_SQUARE.to_string(),
 &Self::Play => PLAY.to_string(),
 &Self::Plug2 => PLUG_2.to_string(),
 &Self::PlugZap2 => PLUG_ZAP_2.to_string(),
 &Self::PlugZap => PLUG_ZAP.to_string(),
 &Self::Plug => PLUG.to_string(),
 &Self::PlusCircle => PLUS_CIRCLE.to_string(),
 &Self::PlusSquare => PLUS_SQUARE.to_string(),
 &Self::Plus => PLUS.to_string(),
 &Self::PocketKnife => POCKET_KNIFE.to_string(),
 &Self::Pocket => POCKET.to_string(),
 &Self::Podcast => PODCAST.to_string(),
 &Self::Pointer => POINTER.to_string(),
 &Self::Popcorn => POPCORN.to_string(),
 &Self::Popsicle => POPSICLE.to_string(),
 &Self::PoundSterling => POUND_STERLING.to_string(),
 &Self::PowerOff => POWER_OFF.to_string(),
 &Self::Power => POWER.to_string(),
 &Self::Presentation => PRESENTATION.to_string(),
 &Self::Printer => PRINTER.to_string(),
 &Self::Projector => PROJECTOR.to_string(),
 &Self::Puzzle => PUZZLE.to_string(),
 &Self::QrCode => QR_CODE.to_string(),
 &Self::Quote => QUOTE.to_string(),
 &Self::Rabbit => RABBIT.to_string(),
 &Self::Radar => RADAR.to_string(),
 &Self::Radiation => RADIATION.to_string(),
 &Self::RadioReceiver => RADIO_RECEIVER.to_string(),
 &Self::RadioTower => RADIO_TOWER.to_string(),
 &Self::Radio => RADIO.to_string(),
 &Self::RailSymbol => RAIL_SYMBOL.to_string(),
 &Self::Rainbow => RAINBOW.to_string(),
 &Self::Rat => RAT.to_string(),
 &Self::Ratio => RATIO.to_string(),
 &Self::Receipt => RECEIPT.to_string(),
 &Self::RectangleHorizontal => RECTANGLE_HORIZONTAL.to_string(),
 &Self::RectangleVertical => RECTANGLE_VERTICAL.to_string(),
 &Self::Recycle => RECYCLE.to_string(),
 &Self::Redo2 => REDO_2.to_string(),
 &Self::RedoDot => REDO_DOT.to_string(),
 &Self::Redo => REDO.to_string(),
 &Self::RefreshCcwDot => REFRESH_CCW_DOT.to_string(),
 &Self::RefreshCcw => REFRESH_CCW.to_string(),
 &Self::RefreshCwOff => REFRESH_CW_OFF.to_string(),
 &Self::RefreshCw => REFRESH_CW.to_string(),
 &Self::Refrigerator => REFRIGERATOR.to_string(),
 &Self::Regex => REGEX.to_string(),
 &Self::RemoveFormatting => REMOVE_FORMATTING.to_string(),
 &Self::Repeat1 => REPEAT_1.to_string(),
 &Self::Repeat2 => REPEAT_2.to_string(),
 &Self::Repeat => REPEAT.to_string(),
 &Self::ReplaceAll => REPLACE_ALL.to_string(),
 &Self::Replace => REPLACE.to_string(),
 &Self::ReplyAll => REPLY_ALL.to_string(),
 &Self::Reply => REPLY.to_string(),
 &Self::Rewind => REWIND.to_string(),
 &Self::Rocket => ROCKET.to_string(),
 &Self::RockingChair => ROCKING_CHAIR.to_string(),
 &Self::RollerCoaster => ROLLER_COASTER.to_string(),
 &Self::Rotate3D => ROTATE_3_D.to_string(),
 &Self::RotateCcw => ROTATE_CCW.to_string(),
 &Self::RotateCw => ROTATE_CW.to_string(),
 &Self::Router => ROUTER.to_string(),
 &Self::Rows => ROWS.to_string(),
 &Self::Rss => RSS.to_string(),
 &Self::Ruler => RULER.to_string(),
 &Self::RussianRuble => RUSSIAN_RUBLE.to_string(),
 &Self::Sailboat => SAILBOAT.to_string(),
 &Self::Salad => SALAD.to_string(),
 &Self::Sandwich => SANDWICH.to_string(),
 &Self::SatelliteDish => SATELLITE_DISH.to_string(),
 &Self::Satellite => SATELLITE.to_string(),
 &Self::SaveAll => SAVE_ALL.to_string(),
 &Self::Save => SAVE.to_string(),
 &Self::Scale3D => SCALE_3_D.to_string(),
 &Self::Scale => SCALE.to_string(),
 &Self::Scaling => SCALING.to_string(),
 &Self::ScanFace => SCAN_FACE.to_string(),
 &Self::ScanLine => SCAN_LINE.to_string(),
 &Self::Scan => SCAN.to_string(),
 &Self::ScatterChart => SCATTER_CHART.to_string(),
 &Self::School2 => SCHOOL_2.to_string(),
 &Self::School => SCHOOL.to_string(),
 &Self::ScissorsLineDashed => SCISSORS_LINE_DASHED.to_string(),
 &Self::ScissorsSquareDashedBottom => SCISSORS_SQUARE_DASHED_BOTTOM.to_string(),
 &Self::ScissorsSquare => SCISSORS_SQUARE.to_string(),
 &Self::Scissors => SCISSORS.to_string(),
 &Self::ScreenShareOff => SCREEN_SHARE_OFF.to_string(),
 &Self::ScreenShare => SCREEN_SHARE.to_string(),
 &Self::ScrollText => SCROLL_TEXT.to_string(),
 &Self::Scroll => SCROLL.to_string(),
 &Self::SearchCheck => SEARCH_CHECK.to_string(),
 &Self::SearchCode => SEARCH_CODE.to_string(),
 &Self::SearchSlash => SEARCH_SLASH.to_string(),
 &Self::SearchX => SEARCH_X.to_string(),
 &Self::Search => SEARCH.to_string(),
 &Self::SendHorizontal => SEND_HORIZONTAL.to_string(),
 &Self::SendToBack => SEND_TO_BACK.to_string(),
 &Self::Send => SEND.to_string(),
 &Self::SeparatorHorizontal => SEPARATOR_HORIZONTAL.to_string(),
 &Self::SeparatorVertical => SEPARATOR_VERTICAL.to_string(),
 &Self::ServerCog => SERVER_COG.to_string(),
 &Self::ServerCrash => SERVER_CRASH.to_string(),
 &Self::ServerOff => SERVER_OFF.to_string(),
 &Self::Server => SERVER.to_string(),
 &Self::Settings2 => SETTINGS_2.to_string(),
 &Self::Settings => SETTINGS.to_string(),
 &Self::Shapes => SHAPES.to_string(),
 &Self::Share2 => SHARE_2.to_string(),
 &Self::Share => SHARE.to_string(),
 &Self::Sheet => SHEET.to_string(),
 &Self::Shell => SHELL.to_string(),
 &Self::ShieldAlert => SHIELD_ALERT.to_string(),
 &Self::ShieldBan => SHIELD_BAN.to_string(),
 &Self::ShieldCheck => SHIELD_CHECK.to_string(),
 &Self::ShieldEllipsis => SHIELD_ELLIPSIS.to_string(),
 &Self::ShieldHalf => SHIELD_HALF.to_string(),
 &Self::ShieldMinus => SHIELD_MINUS.to_string(),
 &Self::ShieldOff => SHIELD_OFF.to_string(),
 &Self::ShieldPlus => SHIELD_PLUS.to_string(),
 &Self::ShieldQuestion => SHIELD_QUESTION.to_string(),
 &Self::ShieldX => SHIELD_X.to_string(),
 &Self::Shield => SHIELD.to_string(),
 &Self::ShipWheel => SHIP_WHEEL.to_string(),
 &Self::Ship => SHIP.to_string(),
 &Self::Shirt => SHIRT.to_string(),
 &Self::ShoppingBag => SHOPPING_BAG.to_string(),
 &Self::ShoppingBasket => SHOPPING_BASKET.to_string(),
 &Self::ShoppingCart => SHOPPING_CART.to_string(),
 &Self::Shovel => SHOVEL.to_string(),
 &Self::ShowerHead => SHOWER_HEAD.to_string(),
 &Self::Shrink => SHRINK.to_string(),
 &Self::Shrub => SHRUB.to_string(),
 &Self::Shuffle => SHUFFLE.to_string(),
 &Self::SigmaSquare => SIGMA_SQUARE.to_string(),
 &Self::Sigma => SIGMA.to_string(),
 &Self::SignalHigh => SIGNAL_HIGH.to_string(),
 &Self::SignalLow => SIGNAL_LOW.to_string(),
 &Self::SignalMedium => SIGNAL_MEDIUM.to_string(),
 &Self::SignalZero => SIGNAL_ZERO.to_string(),
 &Self::Signal => SIGNAL.to_string(),
 &Self::Siren => SIREN.to_string(),
 &Self::SkipBack => SKIP_BACK.to_string(),
 &Self::SkipForward => SKIP_FORWARD.to_string(),
 &Self::Skull => SKULL.to_string(),
 &Self::Slack => SLACK.to_string(),
 &Self::Slash => SLASH.to_string(),
 &Self::Slice => SLICE.to_string(),
 &Self::SlidersHorizontal => SLIDERS_HORIZONTAL.to_string(),
 &Self::Sliders => SLIDERS.to_string(),
 &Self::SmartphoneCharging => SMARTPHONE_CHARGING.to_string(),
 &Self::SmartphoneNfc => SMARTPHONE_NFC.to_string(),
 &Self::Smartphone => SMARTPHONE.to_string(),
 &Self::SmilePlus => SMILE_PLUS.to_string(),
 &Self::Smile => SMILE.to_string(),
 &Self::Snail => SNAIL.to_string(),
 &Self::Snowflake => SNOWFLAKE.to_string(),
 &Self::Sofa => SOFA.to_string(),
 &Self::Soup => SOUP.to_string(),
 &Self::Space => SPACE.to_string(),
 &Self::Spade => SPADE.to_string(),
 &Self::Sparkle => SPARKLE.to_string(),
 &Self::Sparkles => SPARKLES.to_string(),
 &Self::Speaker => SPEAKER.to_string(),
 &Self::SpellCheck2 => SPELL_CHECK_2.to_string(),
 &Self::SpellCheck => SPELL_CHECK.to_string(),
 &Self::Spline => SPLINE.to_string(),
 &Self::SplitSquareHorizontal => SPLIT_SQUARE_HORIZONTAL.to_string(),
 &Self::SplitSquareVertical => SPLIT_SQUARE_VERTICAL.to_string(),
 &Self::Split => SPLIT.to_string(),
 &Self::SprayCan => SPRAY_CAN.to_string(),
 &Self::Sprout => SPROUT.to_string(),
 &Self::SquareAsterisk => SQUARE_ASTERISK.to_string(),
 &Self::SquareCode => SQUARE_CODE.to_string(),
 &Self::SquareDashedBottomCode => SQUARE_DASHED_BOTTOM_CODE.to_string(),
 &Self::SquareDashedBottom => SQUARE_DASHED_BOTTOM.to_string(),
 &Self::SquareDot => SQUARE_DOT.to_string(),
 &Self::SquareEqual => SQUARE_EQUAL.to_string(),
 &Self::SquareSlash => SQUARE_SLASH.to_string(),
 &Self::SquareStack => SQUARE_STACK.to_string(),
 &Self::Square => SQUARE.to_string(),
 &Self::Squirrel => SQUIRREL.to_string(),
 &Self::Stamp => STAMP.to_string(),
 &Self::StarHalf => STAR_HALF.to_string(),
 &Self::StarOff => STAR_OFF.to_string(),
 &Self::Star => STAR.to_string(),
 &Self::StepBack => STEP_BACK.to_string(),
 &Self::StepForward => STEP_FORWARD.to_string(),
 &Self::Stethoscope => STETHOSCOPE.to_string(),
 &Self::Sticker => STICKER.to_string(),
 &Self::StickyNote => STICKY_NOTE.to_string(),
 &Self::StopCircle => STOP_CIRCLE.to_string(),
 &Self::Store => STORE.to_string(),
 &Self::StretchHorizontal => STRETCH_HORIZONTAL.to_string(),
 &Self::StretchVertical => STRETCH_VERTICAL.to_string(),
 &Self::Strikethrough => STRIKETHROUGH.to_string(),
 &Self::Subscript => SUBSCRIPT.to_string(),
 &Self::Subtitles => SUBTITLES.to_string(),
 &Self::SunDim => SUN_DIM.to_string(),
 &Self::SunMedium => SUN_MEDIUM.to_string(),
 &Self::SunMoon => SUN_MOON.to_string(),
 &Self::SunSnow => SUN_SNOW.to_string(),
 &Self::Sun => SUN.to_string(),
 &Self::Sunrise => SUNRISE.to_string(),
 &Self::Sunset => SUNSET.to_string(),
 &Self::Superscript => SUPERSCRIPT.to_string(),
 &Self::SwissFranc => SWISS_FRANC.to_string(),
 &Self::SwitchCamera => SWITCH_CAMERA.to_string(),
 &Self::Sword => SWORD.to_string(),
 &Self::Swords => SWORDS.to_string(),
 &Self::Syringe => SYRINGE.to_string(),
 &Self::Table2 => TABLE_2.to_string(),
 &Self::TableProperties => TABLE_PROPERTIES.to_string(),
 &Self::Table => TABLE.to_string(),
 &Self::TabletSmartphone => TABLET_SMARTPHONE.to_string(),
 &Self::Tablet => TABLET.to_string(),
 &Self::Tablets => TABLETS.to_string(),
 &Self::Tag => TAG.to_string(),
 &Self::Tags => TAGS.to_string(),
 &Self::Tally1 => TALLY_1.to_string(),
 &Self::Tally2 => TALLY_2.to_string(),
 &Self::Tally3 => TALLY_3.to_string(),
 &Self::Tally4 => TALLY_4.to_string(),
 &Self::Tally5 => TALLY_5.to_string(),
 &Self::Target => TARGET.to_string(),
 &Self::Tent => TENT.to_string(),
 &Self::TerminalSquare => TERMINAL_SQUARE.to_string(),
 &Self::Terminal => TERMINAL.to_string(),
 &Self::TestTube2 => TEST_TUBE_2.to_string(),
 &Self::TestTube => TEST_TUBE.to_string(),
 &Self::TestTubes => TEST_TUBES.to_string(),
 &Self::TextCursorInput => TEXT_CURSOR_INPUT.to_string(),
 &Self::TextCursor => TEXT_CURSOR.to_string(),
 &Self::TextQuote => TEXT_QUOTE.to_string(),
 &Self::TextSelect => TEXT_SELECT.to_string(),
 &Self::Text => TEXT.to_string(),
 &Self::ThermometerSnowflake => THERMOMETER_SNOWFLAKE.to_string(),
 &Self::ThermometerSun => THERMOMETER_SUN.to_string(),
 &Self::Thermometer => THERMOMETER.to_string(),
 &Self::ThumbsDown => THUMBS_DOWN.to_string(),
 &Self::ThumbsUp => THUMBS_UP.to_string(),
 &Self::Ticket => TICKET.to_string(),
 &Self::TimerOff => TIMER_OFF.to_string(),
 &Self::TimerReset => TIMER_RESET.to_string(),
 &Self::Timer => TIMER.to_string(),
 &Self::ToggleLeft => TOGGLE_LEFT.to_string(),
 &Self::ToggleRight => TOGGLE_RIGHT.to_string(),
 &Self::Tornado => TORNADO.to_string(),
 &Self::TouchpadOff => TOUCHPAD_OFF.to_string(),
 &Self::Touchpad => TOUCHPAD.to_string(),
 &Self::TowerControl => TOWER_CONTROL.to_string(),
 &Self::ToyBrick => TOY_BRICK.to_string(),
 &Self::Tractor => TRACTOR.to_string(),
 &Self::TrafficCone => TRAFFIC_CONE.to_string(),
 &Self::TrainFrontTunnel => TRAIN_FRONT_TUNNEL.to_string(),
 &Self::TrainFront => TRAIN_FRONT.to_string(),
 &Self::TrainTrack => TRAIN_TRACK.to_string(),
 &Self::TramFront => TRAM_FRONT.to_string(),
 &Self::Trash2 => TRASH_2.to_string(),
 &Self::Trash => TRASH.to_string(),
 &Self::TreeDeciduous => TREE_DECIDUOUS.to_string(),
 &Self::TreePine => TREE_PINE.to_string(),
 &Self::Trees => TREES.to_string(),
 &Self::Trello => TRELLO.to_string(),
 &Self::TrendingDown => TRENDING_DOWN.to_string(),
 &Self::TrendingUp => TRENDING_UP.to_string(),
 &Self::TriangleRight => TRIANGLE_RIGHT.to_string(),
 &Self::Triangle => TRIANGLE.to_string(),
 &Self::Trophy => TROPHY.to_string(),
 &Self::Truck => TRUCK.to_string(),
 &Self::Turtle => TURTLE.to_string(),
 &Self::Tv2 => TV_2.to_string(),
 &Self::Tv => TV.to_string(),
 &Self::Twitch => TWITCH.to_string(),
 &Self::Twitter => TWITTER.to_string(),
 &Self::Type => TYPE.to_string(),
 &Self::Umbrella => UMBRELLA.to_string(),
 &Self::Underline => UNDERLINE.to_string(),
 &Self::Undo2 => UNDO_2.to_string(),
 &Self::UndoDot => UNDO_DOT.to_string(),
 &Self::Undo => UNDO.to_string(),
 &Self::UnfoldHorizontal => UNFOLD_HORIZONTAL.to_string(),
 &Self::UnfoldVertical => UNFOLD_VERTICAL.to_string(),
 &Self::Ungroup => UNGROUP.to_string(),
 &Self::Unlink2 => UNLINK_2.to_string(),
 &Self::Unlink => UNLINK.to_string(),
 &Self::Unlock => UNLOCK.to_string(),
 &Self::Unplug => UNPLUG.to_string(),
 &Self::UploadCloud => UPLOAD_CLOUD.to_string(),
 &Self::Upload => UPLOAD.to_string(),
 &Self::Usb => USB.to_string(),
 &Self::User2 => USER_2.to_string(),
 &Self::UserCheck2 => USER_CHECK_2.to_string(),
 &Self::UserCheck => USER_CHECK.to_string(),
 &Self::UserCircle2 => USER_CIRCLE_2.to_string(),
 &Self::UserCircle => USER_CIRCLE.to_string(),
 &Self::UserCog2 => USER_COG_2.to_string(),
 &Self::UserCog => USER_COG.to_string(),
 &Self::UserMinus2 => USER_MINUS_2.to_string(),
 &Self::UserMinus => USER_MINUS.to_string(),
 &Self::UserPlus2 => USER_PLUS_2.to_string(),
 &Self::UserPlus => USER_PLUS.to_string(),
 &Self::UserSquare2 => USER_SQUARE_2.to_string(),
 &Self::UserSquare => USER_SQUARE.to_string(),
 &Self::UserX2 => USER_X_2.to_string(),
 &Self::UserX => USER_X.to_string(),
 &Self::User => USER.to_string(),
 &Self::Users2 => USERS_2.to_string(),
 &Self::Users => USERS.to_string(),
 &Self::UtensilsCrossed => UTENSILS_CROSSED.to_string(),
 &Self::Utensils => UTENSILS.to_string(),
 &Self::UtilityPole => UTILITY_POLE.to_string(),
 &Self::Variable => VARIABLE.to_string(),
 &Self::Vegan => VEGAN.to_string(),
 &Self::VenetianMask => VENETIAN_MASK.to_string(),
 &Self::VibrateOff => VIBRATE_OFF.to_string(),
 &Self::Vibrate => VIBRATE.to_string(),
 &Self::VideoOff => VIDEO_OFF.to_string(),
 &Self::Video => VIDEO.to_string(),
 &Self::Videotape => VIDEOTAPE.to_string(),
 &Self::View => VIEW.to_string(),
 &Self::Voicemail => VOICEMAIL.to_string(),
 &Self::Volume1 => VOLUME_1.to_string(),
 &Self::Volume2 => VOLUME_2.to_string(),
 &Self::VolumeX => VOLUME_X.to_string(),
 &Self::Volume => VOLUME.to_string(),
 &Self::Vote => VOTE.to_string(),
 &Self::Wallet2 => WALLET_2.to_string(),
 &Self::WalletCards => WALLET_CARDS.to_string(),
 &Self::Wallet => WALLET.to_string(),
 &Self::Wallpaper => WALLPAPER.to_string(),
 &Self::Wand2 => WAND_2.to_string(),
 &Self::Wand => WAND.to_string(),
 &Self::Warehouse => WAREHOUSE.to_string(),
 &Self::Watch => WATCH.to_string(),
 &Self::Waves => WAVES.to_string(),
 &Self::Webcam => WEBCAM.to_string(),
 &Self::Webhook => WEBHOOK.to_string(),
 &Self::WheatOff => WHEAT_OFF.to_string(),
 &Self::Wheat => WHEAT.to_string(),
 &Self::WholeWord => WHOLE_WORD.to_string(),
 &Self::WifiOff => WIFI_OFF.to_string(),
 &Self::Wifi => WIFI.to_string(),
 &Self::Wind => WIND.to_string(),
 &Self::WineOff => WINE_OFF.to_string(),
 &Self::Wine => WINE.to_string(),
 &Self::Workflow => WORKFLOW.to_string(),
 &Self::WrapText => WRAP_TEXT.to_string(),
 &Self::Wrench => WRENCH.to_string(),
 &Self::XCircle => X_CIRCLE.to_string(),
 &Self::XOctagon => X_OCTAGON.to_string(),
 &Self::XSquare => X_SQUARE.to_string(),
 &Self::X => X.to_string(),
 &Self::Youtube => YOUTUBE.to_string(),
 &Self::ZapOff => ZAP_OFF.to_string(),
 &Self::Zap => ZAP.to_string(),
 &Self::ZoomIn => ZOOM_IN.to_string(),
 &Self::ZoomOut => ZOOM_OUT.to_string(),
}
}
}
