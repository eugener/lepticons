use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, Debug, PartialEq)]
pub enum LucideIconType {
    #[strum(props(File = "accessibility"))]
    Accessibility,
    #[strum(props(File = "activity"))]
    Activity,
    #[strum(props(File = "activity-square"))]
    ActivitySquare,
    #[strum(props(File = "air-vent"))]
    AirVent,
    #[strum(props(File = "airplay"))]
    Airplay,
    #[strum(props(File = "alarm-check"))]
    AlarmCheck,
    #[strum(props(File = "alarm-clock"))]
    AlarmClock,
    #[strum(props(File = "alarm-clock-off"))]
    AlarmClockOff,
    #[strum(props(File = "alarm-minus"))]
    AlarmMinus,
    #[strum(props(File = "alarm-plus"))]
    AlarmPlus,
    #[strum(props(File = "album"))]
    Album,
    #[strum(props(File = "alert-circle"))]
    AlertCircle,
    #[strum(props(File = "alert-octagon"))]
    AlertOctagon,
    #[strum(props(File = "alert-triangle"))]
    AlertTriangle,
    #[strum(props(File = "align-center"))]
    AlignCenter,
    #[strum(props(File = "align-center-horizontal"))]
    AlignCenterHorizontal,
    #[strum(props(File = "align-center-vertical"))]
    AlignCenterVertical,
    #[strum(props(File = "align-end-horizontal"))]
    AlignEndHorizontal,
    #[strum(props(File = "align-end-vertical"))]
    AlignEndVertical,
    #[strum(props(File = "align-horizontal-distribute-center"))]
    AlignHorizontalDistributeCenter,
    #[strum(props(File = "align-horizontal-distribute-end"))]
    AlignHorizontalDistributeEnd,
    #[strum(props(File = "align-horizontal-distribute-start"))]
    AlignHorizontalDistributeStart,
    #[strum(props(File = "align-horizontal-justify-center"))]
    AlignHorizontalJustifyCenter,
    #[strum(props(File = "align-horizontal-justify-end"))]
    AlignHorizontalJustifyEnd,
    #[strum(props(File = "align-horizontal-justify-start"))]
    AlignHorizontalJustifyStart,
    #[strum(props(File = "align-horizontal-space-around"))]
    AlignHorizontalSpaceAround,
    #[strum(props(File = "align-horizontal-space-between"))]
    AlignHorizontalSpaceBetween,
    #[strum(props(File = "align-justify"))]
    AlignJustify,
    #[strum(props(File = "align-left"))]
    AlignLeft,
    #[strum(props(File = "align-right"))]
    AlignRight,
    #[strum(props(File = "align-start-horizontal"))]
    AlignStartHorizontal,
    #[strum(props(File = "align-start-vertical"))]
    AlignStartVertical,
    #[strum(props(File = "align-vertical-distribute-center"))]
    AlignVerticalDistributeCenter,
    #[strum(props(File = "align-vertical-distribute-end"))]
    AlignVerticalDistributeEnd,
    #[strum(props(File = "align-vertical-distribute-start"))]
    AlignVerticalDistributeStart,
    #[strum(props(File = "align-vertical-justify-center"))]
    AlignVerticalJustifyCenter,
    #[strum(props(File = "align-vertical-justify-end"))]
    AlignVerticalJustifyEnd,
    #[strum(props(File = "align-vertical-justify-start"))]
    AlignVerticalJustifyStart,
    #[strum(props(File = "align-vertical-space-around"))]
    AlignVerticalSpaceAround,
    #[strum(props(File = "align-vertical-space-between"))]
    AlignVerticalSpaceBetween,
    #[strum(props(File = "ampersand"))]
    Ampersand,
    #[strum(props(File = "ampersands"))]
    Ampersands,
    #[strum(props(File = "anchor"))]
    Anchor,
    #[strum(props(File = "angry"))]
    Angry,
    #[strum(props(File = "annoyed"))]
    Annoyed,
    #[strum(props(File = "antenna"))]
    Antenna,
    #[strum(props(File = "aperture"))]
    Aperture,
    #[strum(props(File = "app-window"))]
    AppWindow,
    #[strum(props(File = "apple"))]
    Apple,
    #[strum(props(File = "archive"))]
    Archive,
    #[strum(props(File = "archive-restore"))]
    ArchiveRestore,
    #[strum(props(File = "archive-x"))]
    ArchiveX,
    #[strum(props(File = "area-chart"))]
    AreaChart,
    #[strum(props(File = "armchair"))]
    Armchair,
    #[strum(props(File = "arrow-big-down"))]
    ArrowBigDown,
    #[strum(props(File = "arrow-big-down-dash"))]
    ArrowBigDownDash,
    #[strum(props(File = "arrow-big-left"))]
    ArrowBigLeft,
    #[strum(props(File = "arrow-big-left-dash"))]
    ArrowBigLeftDash,
    #[strum(props(File = "arrow-big-right"))]
    ArrowBigRight,
    #[strum(props(File = "arrow-big-right-dash"))]
    ArrowBigRightDash,
    #[strum(props(File = "arrow-big-up"))]
    ArrowBigUp,
    #[strum(props(File = "arrow-big-up-dash"))]
    ArrowBigUpDash,
    #[strum(props(File = "arrow-down"))]
    ArrowDown,
    #[strum(props(File = "arrow-down-0-1"))]
    ArrowDown01,
    #[strum(props(File = "arrow-down-1-0"))]
    ArrowDown10,
    #[strum(props(File = "arrow-down-a-z"))]
    ArrowDownAZ,
    #[strum(props(File = "arrow-down-circle"))]
    ArrowDownCircle,
    #[strum(props(File = "arrow-down-from-line"))]
    ArrowDownFromLine,
    #[strum(props(File = "arrow-down-left"))]
    ArrowDownLeft,
    #[strum(props(File = "arrow-down-left-from-circle"))]
    ArrowDownLeftFromCircle,
    #[strum(props(File = "arrow-down-left-square"))]
    ArrowDownLeftSquare,
    #[strum(props(File = "arrow-down-narrow-wide"))]
    ArrowDownNarrowWide,
    #[strum(props(File = "arrow-down-right"))]
    ArrowDownRight,
    #[strum(props(File = "arrow-down-right-from-circle"))]
    ArrowDownRightFromCircle,
    #[strum(props(File = "arrow-down-right-square"))]
    ArrowDownRightSquare,
    #[strum(props(File = "arrow-down-square"))]
    ArrowDownSquare,
    #[strum(props(File = "arrow-down-to-dot"))]
    ArrowDownToDot,
    #[strum(props(File = "arrow-down-to-line"))]
    ArrowDownToLine,
    #[strum(props(File = "arrow-down-up"))]
    ArrowDownUp,
    #[strum(props(File = "arrow-down-wide-narrow"))]
    ArrowDownWideNarrow,
    #[strum(props(File = "arrow-down-z-a"))]
    ArrowDownZA,
    #[strum(props(File = "arrow-left"))]
    ArrowLeft,
    #[strum(props(File = "arrow-left-circle"))]
    ArrowLeftCircle,
    #[strum(props(File = "arrow-left-from-line"))]
    ArrowLeftFromLine,
    #[strum(props(File = "arrow-left-right"))]
    ArrowLeftRight,
    #[strum(props(File = "arrow-left-square"))]
    ArrowLeftSquare,
    #[strum(props(File = "arrow-left-to-line"))]
    ArrowLeftToLine,
    #[strum(props(File = "arrow-right"))]
    ArrowRight,
    #[strum(props(File = "arrow-right-circle"))]
    ArrowRightCircle,
    #[strum(props(File = "arrow-right-from-line"))]
    ArrowRightFromLine,
    #[strum(props(File = "arrow-right-left"))]
    ArrowRightLeft,
    #[strum(props(File = "arrow-right-square"))]
    ArrowRightSquare,
    #[strum(props(File = "arrow-right-to-line"))]
    ArrowRightToLine,
    #[strum(props(File = "arrow-up"))]
    ArrowUp,
    #[strum(props(File = "arrow-up-0-1"))]
    ArrowUp01,
    #[strum(props(File = "arrow-up-1-0"))]
    ArrowUp10,
    #[strum(props(File = "arrow-up-a-z"))]
    ArrowUpAZ,
    #[strum(props(File = "arrow-up-circle"))]
    ArrowUpCircle,
    #[strum(props(File = "arrow-up-down"))]
    ArrowUpDown,
    #[strum(props(File = "arrow-up-from-dot"))]
    ArrowUpFromDot,
    #[strum(props(File = "arrow-up-from-line"))]
    ArrowUpFromLine,
    #[strum(props(File = "arrow-up-left"))]
    ArrowUpLeft,
    #[strum(props(File = "arrow-up-left-from-circle"))]
    ArrowUpLeftFromCircle,
    #[strum(props(File = "arrow-up-left-square"))]
    ArrowUpLeftSquare,
    #[strum(props(File = "arrow-up-narrow-wide"))]
    ArrowUpNarrowWide,
    #[strum(props(File = "arrow-up-right"))]
    ArrowUpRight,
    #[strum(props(File = "arrow-up-right-from-circle"))]
    ArrowUpRightFromCircle,
    #[strum(props(File = "arrow-up-right-square"))]
    ArrowUpRightSquare,
    #[strum(props(File = "arrow-up-square"))]
    ArrowUpSquare,
    #[strum(props(File = "arrow-up-to-line"))]
    ArrowUpToLine,
    #[strum(props(File = "arrow-up-wide-narrow"))]
    ArrowUpWideNarrow,
    #[strum(props(File = "arrow-up-z-a"))]
    ArrowUpZA,
    #[strum(props(File = "arrows-up-from-line"))]
    ArrowsUpFromLine,
    #[strum(props(File = "asterisk"))]
    Asterisk,
    #[strum(props(File = "at-sign"))]
    AtSign,
    #[strum(props(File = "atom"))]
    Atom,
    #[strum(props(File = "award"))]
    Award,
    #[strum(props(File = "axe"))]
    Axe,
    #[strum(props(File = "axis-3d"))]
    Axis3d,
    #[strum(props(File = "baby"))]
    Baby,
    #[strum(props(File = "backpack"))]
    Backpack,
    #[strum(props(File = "badge"))]
    Badge,
    #[strum(props(File = "badge-alert"))]
    BadgeAlert,
    #[strum(props(File = "badge-cent"))]
    BadgeCent,
    #[strum(props(File = "badge-check"))]
    BadgeCheck,
    #[strum(props(File = "badge-dollar-sign"))]
    BadgeDollarSign,
    #[strum(props(File = "badge-euro"))]
    BadgeEuro,
    #[strum(props(File = "badge-help"))]
    BadgeHelp,
    #[strum(props(File = "badge-indian-rupee"))]
    BadgeIndianRupee,
    #[strum(props(File = "badge-info"))]
    BadgeInfo,
    #[strum(props(File = "badge-japanese-yen"))]
    BadgeJapaneseYen,
    #[strum(props(File = "badge-minus"))]
    BadgeMinus,
    #[strum(props(File = "badge-percent"))]
    BadgePercent,
    #[strum(props(File = "badge-plus"))]
    BadgePlus,
    #[strum(props(File = "badge-pound-sterling"))]
    BadgePoundSterling,
    #[strum(props(File = "badge-russian-ruble"))]
    BadgeRussianRuble,
    #[strum(props(File = "badge-swiss-franc"))]
    BadgeSwissFranc,
    #[strum(props(File = "badge-x"))]
    BadgeX,
    #[strum(props(File = "baggage-claim"))]
    BaggageClaim,
    #[strum(props(File = "ban"))]
    Ban,
    #[strum(props(File = "banana"))]
    Banana,
    #[strum(props(File = "banknote"))]
    Banknote,
    #[strum(props(File = "bar-chart"))]
    BarChart,
    #[strum(props(File = "bar-chart-2"))]
    BarChart2,
    #[strum(props(File = "bar-chart-3"))]
    BarChart3,
    #[strum(props(File = "bar-chart-4"))]
    BarChart4,
    #[strum(props(File = "bar-chart-big"))]
    BarChartBig,
    #[strum(props(File = "bar-chart-horizontal"))]
    BarChartHorizontal,
    #[strum(props(File = "bar-chart-horizontal-big"))]
    BarChartHorizontalBig,
    #[strum(props(File = "baseline"))]
    Baseline,
    #[strum(props(File = "bath"))]
    Bath,
    #[strum(props(File = "battery"))]
    Battery,
    #[strum(props(File = "battery-charging"))]
    BatteryCharging,
    #[strum(props(File = "battery-full"))]
    BatteryFull,
    #[strum(props(File = "battery-low"))]
    BatteryLow,
    #[strum(props(File = "battery-medium"))]
    BatteryMedium,
    #[strum(props(File = "battery-warning"))]
    BatteryWarning,
    #[strum(props(File = "beaker"))]
    Beaker,
    #[strum(props(File = "bean"))]
    Bean,
    #[strum(props(File = "bean-off"))]
    BeanOff,
    #[strum(props(File = "bed"))]
    Bed,
    #[strum(props(File = "bed-double"))]
    BedDouble,
    #[strum(props(File = "bed-single"))]
    BedSingle,
    #[strum(props(File = "beef"))]
    Beef,
    #[strum(props(File = "beer"))]
    Beer,
    #[strum(props(File = "bell"))]
    Bell,
    #[strum(props(File = "bell-dot"))]
    BellDot,
    #[strum(props(File = "bell-minus"))]
    BellMinus,
    #[strum(props(File = "bell-off"))]
    BellOff,
    #[strum(props(File = "bell-plus"))]
    BellPlus,
    #[strum(props(File = "bell-ring"))]
    BellRing,
    #[strum(props(File = "bike"))]
    Bike,
    #[strum(props(File = "binary"))]
    Binary,
    #[strum(props(File = "biohazard"))]
    Biohazard,
    #[strum(props(File = "bird"))]
    Bird,
    #[strum(props(File = "bitcoin"))]
    Bitcoin,
    #[strum(props(File = "blinds"))]
    Blinds,
    #[strum(props(File = "blocks"))]
    Blocks,
    #[strum(props(File = "bluetooth"))]
    Bluetooth,
    #[strum(props(File = "bluetooth-connected"))]
    BluetoothConnected,
    #[strum(props(File = "bluetooth-off"))]
    BluetoothOff,
    #[strum(props(File = "bluetooth-searching"))]
    BluetoothSearching,
    #[strum(props(File = "bold"))]
    Bold,
    #[strum(props(File = "bomb"))]
    Bomb,
    #[strum(props(File = "bone"))]
    Bone,
    #[strum(props(File = "book"))]
    Book,
    #[strum(props(File = "book-copy"))]
    BookCopy,
    #[strum(props(File = "book-down"))]
    BookDown,
    #[strum(props(File = "book-key"))]
    BookKey,
    #[strum(props(File = "book-lock"))]
    BookLock,
    #[strum(props(File = "book-marked"))]
    BookMarked,
    #[strum(props(File = "book-minus"))]
    BookMinus,
    #[strum(props(File = "book-open"))]
    BookOpen,
    #[strum(props(File = "book-open-check"))]
    BookOpenCheck,
    #[strum(props(File = "book-plus"))]
    BookPlus,
    #[strum(props(File = "book-template"))]
    BookTemplate,
    #[strum(props(File = "book-up"))]
    BookUp,
    #[strum(props(File = "book-up-2"))]
    BookUp2,
    #[strum(props(File = "book-x"))]
    BookX,
    #[strum(props(File = "bookmark"))]
    Bookmark,
    #[strum(props(File = "bookmark-minus"))]
    BookmarkMinus,
    #[strum(props(File = "bookmark-plus"))]
    BookmarkPlus,
    #[strum(props(File = "boom-box"))]
    BoomBox,
    #[strum(props(File = "bot"))]
    Bot,
    #[strum(props(File = "box"))]
    Box,
    #[strum(props(File = "box-select"))]
    BoxSelect,
    #[strum(props(File = "boxes"))]
    Boxes,
    #[strum(props(File = "braces"))]
    Braces,
    #[strum(props(File = "brackets"))]
    Brackets,
    #[strum(props(File = "brain"))]
    Brain,
    #[strum(props(File = "brain-circuit"))]
    BrainCircuit,
    #[strum(props(File = "brain-cog"))]
    BrainCog,
    #[strum(props(File = "briefcase"))]
    Briefcase,
    #[strum(props(File = "bring-to-front"))]
    BringToFront,
    #[strum(props(File = "brush"))]
    Brush,
    #[strum(props(File = "bug"))]
    Bug,
    #[strum(props(File = "bug-off"))]
    BugOff,
    #[strum(props(File = "bug-play"))]
    BugPlay,
    #[strum(props(File = "building"))]
    Building,
    #[strum(props(File = "building-2"))]
    Building2,
    #[strum(props(File = "bus"))]
    Bus,
    #[strum(props(File = "bus-front"))]
    BusFront,
    #[strum(props(File = "cable"))]
    Cable,
    #[strum(props(File = "cable-car"))]
    CableCar,
    #[strum(props(File = "cake"))]
    Cake,
    #[strum(props(File = "cake-slice"))]
    CakeSlice,
    #[strum(props(File = "calculator"))]
    Calculator,
    #[strum(props(File = "calendar"))]
    Calendar,
    #[strum(props(File = "calendar-check"))]
    CalendarCheck,
    #[strum(props(File = "calendar-check-2"))]
    CalendarCheck2,
    #[strum(props(File = "calendar-clock"))]
    CalendarClock,
    #[strum(props(File = "calendar-days"))]
    CalendarDays,
    #[strum(props(File = "calendar-heart"))]
    CalendarHeart,
    #[strum(props(File = "calendar-minus"))]
    CalendarMinus,
    #[strum(props(File = "calendar-off"))]
    CalendarOff,
    #[strum(props(File = "calendar-plus"))]
    CalendarPlus,
    #[strum(props(File = "calendar-range"))]
    CalendarRange,
    #[strum(props(File = "calendar-search"))]
    CalendarSearch,
    #[strum(props(File = "calendar-x"))]
    CalendarX,
    #[strum(props(File = "calendar-x-2"))]
    CalendarX2,
    #[strum(props(File = "camera"))]
    Camera,
    #[strum(props(File = "camera-off"))]
    CameraOff,
    #[strum(props(File = "candlestick-chart"))]
    CandlestickChart,
    #[strum(props(File = "candy"))]
    Candy,
    #[strum(props(File = "candy-cane"))]
    CandyCane,
    #[strum(props(File = "candy-off"))]
    CandyOff,
    #[strum(props(File = "car"))]
    Car,
    #[strum(props(File = "car-front"))]
    CarFront,
    #[strum(props(File = "car-taxi-front"))]
    CarTaxiFront,
    #[strum(props(File = "carrot"))]
    Carrot,
    #[strum(props(File = "case-lower"))]
    CaseLower,
    #[strum(props(File = "case-sensitive"))]
    CaseSensitive,
    #[strum(props(File = "case-upper"))]
    CaseUpper,
    #[strum(props(File = "cassette-tape"))]
    CassetteTape,
    #[strum(props(File = "cast"))]
    Cast,
    #[strum(props(File = "castle"))]
    Castle,
    #[strum(props(File = "cat"))]
    Cat,
    #[strum(props(File = "check"))]
    Check,
    #[strum(props(File = "check-check"))]
    CheckCheck,
    #[strum(props(File = "check-circle"))]
    CheckCircle,
    #[strum(props(File = "check-circle-2"))]
    CheckCircle2,
    #[strum(props(File = "check-square"))]
    CheckSquare,
    #[strum(props(File = "chef-hat"))]
    ChefHat,
    #[strum(props(File = "cherry"))]
    Cherry,
    #[strum(props(File = "chevron-down"))]
    ChevronDown,
    #[strum(props(File = "chevron-down-circle"))]
    ChevronDownCircle,
    #[strum(props(File = "chevron-down-square"))]
    ChevronDownSquare,
    #[strum(props(File = "chevron-first"))]
    ChevronFirst,
    #[strum(props(File = "chevron-last"))]
    ChevronLast,
    #[strum(props(File = "chevron-left"))]
    ChevronLeft,
    #[strum(props(File = "chevron-left-circle"))]
    ChevronLeftCircle,
    #[strum(props(File = "chevron-left-square"))]
    ChevronLeftSquare,
    #[strum(props(File = "chevron-right"))]
    ChevronRight,
    #[strum(props(File = "chevron-right-circle"))]
    ChevronRightCircle,
    #[strum(props(File = "chevron-right-square"))]
    ChevronRightSquare,
    #[strum(props(File = "chevron-up"))]
    ChevronUp,
    #[strum(props(File = "chevron-up-circle"))]
    ChevronUpCircle,
    #[strum(props(File = "chevron-up-square"))]
    ChevronUpSquare,
    #[strum(props(File = "chevrons-down"))]
    ChevronsDown,
    #[strum(props(File = "chevrons-down-up"))]
    ChevronsDownUp,
    #[strum(props(File = "chevrons-left"))]
    ChevronsLeft,
    #[strum(props(File = "chevrons-left-right"))]
    ChevronsLeftRight,
    #[strum(props(File = "chevrons-right"))]
    ChevronsRight,
    #[strum(props(File = "chevrons-right-left"))]
    ChevronsRightLeft,
    #[strum(props(File = "chevrons-up"))]
    ChevronsUp,
    #[strum(props(File = "chevrons-up-down"))]
    ChevronsUpDown,
    #[strum(props(File = "chrome"))]
    Chrome,
    #[strum(props(File = "church"))]
    Church,
    #[strum(props(File = "cigarette"))]
    Cigarette,
    #[strum(props(File = "cigarette-off"))]
    CigaretteOff,
    #[strum(props(File = "circle"))]
    Circle,
    #[strum(props(File = "circle-dashed"))]
    CircleDashed,
    #[strum(props(File = "circle-dollar-sign"))]
    CircleDollarSign,
    #[strum(props(File = "circle-dot"))]
    CircleDot,
    #[strum(props(File = "circle-dot-dashed"))]
    CircleDotDashed,
    #[strum(props(File = "circle-ellipsis"))]
    CircleEllipsis,
    #[strum(props(File = "circle-equal"))]
    CircleEqual,
    #[strum(props(File = "circle-off"))]
    CircleOff,
    #[strum(props(File = "circle-slash"))]
    CircleSlash,
    #[strum(props(File = "circle-slash-2"))]
    CircleSlash2,
    #[strum(props(File = "circuit-board"))]
    CircuitBoard,
    #[strum(props(File = "citrus"))]
    Citrus,
    #[strum(props(File = "clapperboard"))]
    Clapperboard,
    #[strum(props(File = "clipboard"))]
    Clipboard,
    #[strum(props(File = "clipboard-check"))]
    ClipboardCheck,
    #[strum(props(File = "clipboard-copy"))]
    ClipboardCopy,
    #[strum(props(File = "clipboard-edit"))]
    ClipboardEdit,
    #[strum(props(File = "clipboard-list"))]
    ClipboardList,
    #[strum(props(File = "clipboard-paste"))]
    ClipboardPaste,
    #[strum(props(File = "clipboard-signature"))]
    ClipboardSignature,
    #[strum(props(File = "clipboard-type"))]
    ClipboardType,
    #[strum(props(File = "clipboard-x"))]
    ClipboardX,
    #[strum(props(File = "clock"))]
    Clock,
    #[strum(props(File = "clock-1"))]
    Clock1,
    #[strum(props(File = "clock-10"))]
    Clock10,
    #[strum(props(File = "clock-11"))]
    Clock11,
    #[strum(props(File = "clock-12"))]
    Clock12,
    #[strum(props(File = "clock-2"))]
    Clock2,
    #[strum(props(File = "clock-3"))]
    Clock3,
    #[strum(props(File = "clock-4"))]
    Clock4,
    #[strum(props(File = "clock-5"))]
    Clock5,
    #[strum(props(File = "clock-6"))]
    Clock6,
    #[strum(props(File = "clock-7"))]
    Clock7,
    #[strum(props(File = "clock-8"))]
    Clock8,
    #[strum(props(File = "clock-9"))]
    Clock9,
    #[strum(props(File = "cloud"))]
    Cloud,
    #[strum(props(File = "cloud-cog"))]
    CloudCog,
    #[strum(props(File = "cloud-drizzle"))]
    CloudDrizzle,
    #[strum(props(File = "cloud-fog"))]
    CloudFog,
    #[strum(props(File = "cloud-hail"))]
    CloudHail,
    #[strum(props(File = "cloud-lightning"))]
    CloudLightning,
    #[strum(props(File = "cloud-moon"))]
    CloudMoon,
    #[strum(props(File = "cloud-moon-rain"))]
    CloudMoonRain,
    #[strum(props(File = "cloud-off"))]
    CloudOff,
    #[strum(props(File = "cloud-rain"))]
    CloudRain,
    #[strum(props(File = "cloud-rain-wind"))]
    CloudRainWind,
    #[strum(props(File = "cloud-snow"))]
    CloudSnow,
    #[strum(props(File = "cloud-sun"))]
    CloudSun,
    #[strum(props(File = "cloud-sun-rain"))]
    CloudSunRain,
    #[strum(props(File = "cloudy"))]
    Cloudy,
    #[strum(props(File = "clover"))]
    Clover,
    #[strum(props(File = "club"))]
    Club,
    #[strum(props(File = "code"))]
    Code,
    #[strum(props(File = "code-2"))]
    Code2,
    #[strum(props(File = "codepen"))]
    Codepen,
    #[strum(props(File = "codesandbox"))]
    Codesandbox,
    #[strum(props(File = "coffee"))]
    Coffee,
    #[strum(props(File = "cog"))]
    Cog,
    #[strum(props(File = "coins"))]
    Coins,
    #[strum(props(File = "columns"))]
    Columns,
    #[strum(props(File = "combine"))]
    Combine,
    #[strum(props(File = "command"))]
    Command,
    #[strum(props(File = "compass"))]
    Compass,
    #[strum(props(File = "component"))]
    Component,
    #[strum(props(File = "computer"))]
    Computer,
    #[strum(props(File = "concierge-bell"))]
    ConciergeBell,
    #[strum(props(File = "construction"))]
    Construction,
    #[strum(props(File = "contact"))]
    Contact,
    #[strum(props(File = "contact-2"))]
    Contact2,
    #[strum(props(File = "container"))]
    Container,
    #[strum(props(File = "contrast"))]
    Contrast,
    #[strum(props(File = "cookie"))]
    Cookie,
    #[strum(props(File = "copy"))]
    Copy,
    #[strum(props(File = "copy-check"))]
    CopyCheck,
    #[strum(props(File = "copy-minus"))]
    CopyMinus,
    #[strum(props(File = "copy-plus"))]
    CopyPlus,
    #[strum(props(File = "copy-slash"))]
    CopySlash,
    #[strum(props(File = "copy-x"))]
    CopyX,
    #[strum(props(File = "copyleft"))]
    Copyleft,
    #[strum(props(File = "copyright"))]
    Copyright,
    #[strum(props(File = "corner-down-left"))]
    CornerDownLeft,
    #[strum(props(File = "corner-down-right"))]
    CornerDownRight,
    #[strum(props(File = "corner-left-down"))]
    CornerLeftDown,
    #[strum(props(File = "corner-left-up"))]
    CornerLeftUp,
    #[strum(props(File = "corner-right-down"))]
    CornerRightDown,
    #[strum(props(File = "corner-right-up"))]
    CornerRightUp,
    #[strum(props(File = "corner-up-left"))]
    CornerUpLeft,
    #[strum(props(File = "corner-up-right"))]
    CornerUpRight,
    #[strum(props(File = "cpu"))]
    Cpu,
    #[strum(props(File = "creative-commons"))]
    CreativeCommons,
    #[strum(props(File = "credit-card"))]
    CreditCard,
    #[strum(props(File = "croissant"))]
    Croissant,
    #[strum(props(File = "crop"))]
    Crop,
    #[strum(props(File = "cross"))]
    Cross,
    #[strum(props(File = "crosshair"))]
    Crosshair,
    #[strum(props(File = "crown"))]
    Crown,
    #[strum(props(File = "cup-soda"))]
    CupSoda,
    #[strum(props(File = "currency"))]
    Currency,
    #[strum(props(File = "database"))]
    Database,
    #[strum(props(File = "database-backup"))]
    DatabaseBackup,
    #[strum(props(File = "database-zap"))]
    DatabaseZap,
    #[strum(props(File = "delete"))]
    Delete,
    #[strum(props(File = "dessert"))]
    Dessert,
    #[strum(props(File = "diamond"))]
    Diamond,
    #[strum(props(File = "dice-1"))]
    Dice1,
    #[strum(props(File = "dice-2"))]
    Dice2,
    #[strum(props(File = "dice-3"))]
    Dice3,
    #[strum(props(File = "dice-4"))]
    Dice4,
    #[strum(props(File = "dice-5"))]
    Dice5,
    #[strum(props(File = "dice-6"))]
    Dice6,
    #[strum(props(File = "dices"))]
    Dices,
    #[strum(props(File = "diff"))]
    Diff,
    #[strum(props(File = "disc"))]
    Disc,
    #[strum(props(File = "disc-2"))]
    Disc2,
    #[strum(props(File = "disc-3"))]
    Disc3,
    #[strum(props(File = "divide"))]
    Divide,
    #[strum(props(File = "divide-circle"))]
    DivideCircle,
    #[strum(props(File = "divide-square"))]
    DivideSquare,
    #[strum(props(File = "dna"))]
    Dna,
    #[strum(props(File = "dna-off"))]
    DnaOff,
    #[strum(props(File = "dog"))]
    Dog,
    #[strum(props(File = "dollar-sign"))]
    DollarSign,
    #[strum(props(File = "donut"))]
    Donut,
    #[strum(props(File = "door-closed"))]
    DoorClosed,
    #[strum(props(File = "door-open"))]
    DoorOpen,
    #[strum(props(File = "dot"))]
    Dot,
    #[strum(props(File = "download"))]
    Download,
    #[strum(props(File = "download-cloud"))]
    DownloadCloud,
    #[strum(props(File = "dribbble"))]
    Dribbble,
    #[strum(props(File = "droplet"))]
    Droplet,
    #[strum(props(File = "droplets"))]
    Droplets,
    #[strum(props(File = "drumstick"))]
    Drumstick,
    #[strum(props(File = "dumbbell"))]
    Dumbbell,
    #[strum(props(File = "ear"))]
    Ear,
    #[strum(props(File = "ear-off"))]
    EarOff,
    #[strum(props(File = "egg"))]
    Egg,
    #[strum(props(File = "egg-fried"))]
    EggFried,
    #[strum(props(File = "egg-off"))]
    EggOff,
    #[strum(props(File = "equal"))]
    Equal,
    #[strum(props(File = "equal-not"))]
    EqualNot,
    #[strum(props(File = "eraser"))]
    Eraser,
    #[strum(props(File = "euro"))]
    Euro,
    #[strum(props(File = "expand"))]
    Expand,
    #[strum(props(File = "external-link"))]
    ExternalLink,
    #[strum(props(File = "eye"))]
    Eye,
    #[strum(props(File = "eye-off"))]
    EyeOff,
    #[strum(props(File = "facebook"))]
    Facebook,
    #[strum(props(File = "factory"))]
    Factory,
    #[strum(props(File = "fan"))]
    Fan,
    #[strum(props(File = "fast-forward"))]
    FastForward,
    #[strum(props(File = "feather"))]
    Feather,
    #[strum(props(File = "ferris-wheel"))]
    FerrisWheel,
    #[strum(props(File = "figma"))]
    Figma,
    #[strum(props(File = "file"))]
    File,
    #[strum(props(File = "file-archive"))]
    FileArchive,
    #[strum(props(File = "file-audio"))]
    FileAudio,
    #[strum(props(File = "file-audio-2"))]
    FileAudio2,
    #[strum(props(File = "file-axis-3d"))]
    FileAxis3d,
    #[strum(props(File = "file-badge"))]
    FileBadge,
    #[strum(props(File = "file-badge-2"))]
    FileBadge2,
    #[strum(props(File = "file-bar-chart"))]
    FileBarChart,
    #[strum(props(File = "file-bar-chart-2"))]
    FileBarChart2,
    #[strum(props(File = "file-box"))]
    FileBox,
    #[strum(props(File = "file-check"))]
    FileCheck,
    #[strum(props(File = "file-check-2"))]
    FileCheck2,
    #[strum(props(File = "file-clock"))]
    FileClock,
    #[strum(props(File = "file-code"))]
    FileCode,
    #[strum(props(File = "file-code-2"))]
    FileCode2,
    #[strum(props(File = "file-cog"))]
    FileCog,
    #[strum(props(File = "file-diff"))]
    FileDiff,
    #[strum(props(File = "file-digit"))]
    FileDigit,
    #[strum(props(File = "file-down"))]
    FileDown,
    #[strum(props(File = "file-edit"))]
    FileEdit,
    #[strum(props(File = "file-heart"))]
    FileHeart,
    #[strum(props(File = "file-image"))]
    FileImage,
    #[strum(props(File = "file-input"))]
    FileInput,
    #[strum(props(File = "file-json"))]
    FileJson,
    #[strum(props(File = "file-json-2"))]
    FileJson2,
    #[strum(props(File = "file-key"))]
    FileKey,
    #[strum(props(File = "file-key-2"))]
    FileKey2,
    #[strum(props(File = "file-line-chart"))]
    FileLineChart,
    #[strum(props(File = "file-lock"))]
    FileLock,
    #[strum(props(File = "file-lock-2"))]
    FileLock2,
    #[strum(props(File = "file-minus"))]
    FileMinus,
    #[strum(props(File = "file-minus-2"))]
    FileMinus2,
    #[strum(props(File = "file-output"))]
    FileOutput,
    #[strum(props(File = "file-pie-chart"))]
    FilePieChart,
    #[strum(props(File = "file-plus"))]
    FilePlus,
    #[strum(props(File = "file-plus-2"))]
    FilePlus2,
    #[strum(props(File = "file-question"))]
    FileQuestion,
    #[strum(props(File = "file-scan"))]
    FileScan,
    #[strum(props(File = "file-search"))]
    FileSearch,
    #[strum(props(File = "file-search-2"))]
    FileSearch2,
    #[strum(props(File = "file-signature"))]
    FileSignature,
    #[strum(props(File = "file-spreadsheet"))]
    FileSpreadsheet,
    #[strum(props(File = "file-stack"))]
    FileStack,
    #[strum(props(File = "file-symlink"))]
    FileSymlink,
    #[strum(props(File = "file-terminal"))]
    FileTerminal,
    #[strum(props(File = "file-text"))]
    FileText,
    #[strum(props(File = "file-type"))]
    FileType,
    #[strum(props(File = "file-type-2"))]
    FileType2,
    #[strum(props(File = "file-up"))]
    FileUp,
    #[strum(props(File = "file-video"))]
    FileVideo,
    #[strum(props(File = "file-video-2"))]
    FileVideo2,
    #[strum(props(File = "file-volume"))]
    FileVolume,
    #[strum(props(File = "file-volume-2"))]
    FileVolume2,
    #[strum(props(File = "file-warning"))]
    FileWarning,
    #[strum(props(File = "file-x"))]
    FileX,
    #[strum(props(File = "file-x-2"))]
    FileX2,
    #[strum(props(File = "files"))]
    Files,
    #[strum(props(File = "film"))]
    Film,
    #[strum(props(File = "filter"))]
    Filter,
    #[strum(props(File = "filter-x"))]
    FilterX,
    #[strum(props(File = "fingerprint"))]
    Fingerprint,
    #[strum(props(File = "fish"))]
    Fish,
    #[strum(props(File = "fish-off"))]
    FishOff,
    #[strum(props(File = "fish-symbol"))]
    FishSymbol,
    #[strum(props(File = "flag"))]
    Flag,
    #[strum(props(File = "flag-off"))]
    FlagOff,
    #[strum(props(File = "flag-triangle-left"))]
    FlagTriangleLeft,
    #[strum(props(File = "flag-triangle-right"))]
    FlagTriangleRight,
    #[strum(props(File = "flame"))]
    Flame,
    #[strum(props(File = "flashlight"))]
    Flashlight,
    #[strum(props(File = "flashlight-off"))]
    FlashlightOff,
    #[strum(props(File = "flask-conical"))]
    FlaskConical,
    #[strum(props(File = "flask-conical-off"))]
    FlaskConicalOff,
    #[strum(props(File = "flask-round"))]
    FlaskRound,
    #[strum(props(File = "flip-horizontal"))]
    FlipHorizontal,
    #[strum(props(File = "flip-horizontal-2"))]
    FlipHorizontal2,
    #[strum(props(File = "flip-vertical"))]
    FlipVertical,
    #[strum(props(File = "flip-vertical-2"))]
    FlipVertical2,
    #[strum(props(File = "flower"))]
    Flower,
    #[strum(props(File = "flower-2"))]
    Flower2,
    #[strum(props(File = "focus"))]
    Focus,
    #[strum(props(File = "fold-horizontal"))]
    FoldHorizontal,
    #[strum(props(File = "fold-vertical"))]
    FoldVertical,
    #[strum(props(File = "folder"))]
    Folder,
    #[strum(props(File = "folder-archive"))]
    FolderArchive,
    #[strum(props(File = "folder-check"))]
    FolderCheck,
    #[strum(props(File = "folder-clock"))]
    FolderClock,
    #[strum(props(File = "folder-closed"))]
    FolderClosed,
    #[strum(props(File = "folder-cog"))]
    FolderCog,
    #[strum(props(File = "folder-dot"))]
    FolderDot,
    #[strum(props(File = "folder-down"))]
    FolderDown,
    #[strum(props(File = "folder-edit"))]
    FolderEdit,
    #[strum(props(File = "folder-git"))]
    FolderGit,
    #[strum(props(File = "folder-git-2"))]
    FolderGit2,
    #[strum(props(File = "folder-heart"))]
    FolderHeart,
    #[strum(props(File = "folder-input"))]
    FolderInput,
    #[strum(props(File = "folder-kanban"))]
    FolderKanban,
    #[strum(props(File = "folder-key"))]
    FolderKey,
    #[strum(props(File = "folder-lock"))]
    FolderLock,
    #[strum(props(File = "folder-minus"))]
    FolderMinus,
    #[strum(props(File = "folder-open"))]
    FolderOpen,
    #[strum(props(File = "folder-open-dot"))]
    FolderOpenDot,
    #[strum(props(File = "folder-output"))]
    FolderOutput,
    #[strum(props(File = "folder-plus"))]
    FolderPlus,
    #[strum(props(File = "folder-root"))]
    FolderRoot,
    #[strum(props(File = "folder-search"))]
    FolderSearch,
    #[strum(props(File = "folder-search-2"))]
    FolderSearch2,
    #[strum(props(File = "folder-symlink"))]
    FolderSymlink,
    #[strum(props(File = "folder-sync"))]
    FolderSync,
    #[strum(props(File = "folder-tree"))]
    FolderTree,
    #[strum(props(File = "folder-up"))]
    FolderUp,
    #[strum(props(File = "folder-x"))]
    FolderX,
    #[strum(props(File = "folders"))]
    Folders,
    #[strum(props(File = "footprints"))]
    Footprints,
    #[strum(props(File = "forklift"))]
    Forklift,
    #[strum(props(File = "form-input"))]
    FormInput,
    #[strum(props(File = "forward"))]
    Forward,
    #[strum(props(File = "frame"))]
    Frame,
    #[strum(props(File = "framer"))]
    Framer,
    #[strum(props(File = "frown"))]
    Frown,
    #[strum(props(File = "fuel"))]
    Fuel,
    #[strum(props(File = "function-square"))]
    FunctionSquare,
    #[strum(props(File = "gallery-horizontal"))]
    GalleryHorizontal,
    #[strum(props(File = "gallery-horizontal-end"))]
    GalleryHorizontalEnd,
    #[strum(props(File = "gallery-thumbnails"))]
    GalleryThumbnails,
    #[strum(props(File = "gallery-vertical"))]
    GalleryVertical,
    #[strum(props(File = "gallery-vertical-end"))]
    GalleryVerticalEnd,
    #[strum(props(File = "gamepad"))]
    Gamepad,
    #[strum(props(File = "gamepad-2"))]
    Gamepad2,
    #[strum(props(File = "gantt-chart"))]
    GanttChart,
    #[strum(props(File = "gantt-chart-square"))]
    GanttChartSquare,
    #[strum(props(File = "gauge"))]
    Gauge,
    #[strum(props(File = "gauge-circle"))]
    GaugeCircle,
    #[strum(props(File = "gavel"))]
    Gavel,
    #[strum(props(File = "gem"))]
    Gem,
    #[strum(props(File = "ghost"))]
    Ghost,
    #[strum(props(File = "gift"))]
    Gift,
    #[strum(props(File = "git-branch"))]
    GitBranch,
    #[strum(props(File = "git-branch-plus"))]
    GitBranchPlus,
    #[strum(props(File = "git-commit"))]
    GitCommit,
    #[strum(props(File = "git-compare"))]
    GitCompare,
    #[strum(props(File = "git-fork"))]
    GitFork,
    #[strum(props(File = "git-merge"))]
    GitMerge,
    #[strum(props(File = "git-pull-request"))]
    GitPullRequest,
    #[strum(props(File = "git-pull-request-closed"))]
    GitPullRequestClosed,
    #[strum(props(File = "git-pull-request-draft"))]
    GitPullRequestDraft,
    #[strum(props(File = "github"))]
    Github,
    #[strum(props(File = "gitlab"))]
    Gitlab,
    #[strum(props(File = "glass-water"))]
    GlassWater,
    #[strum(props(File = "glasses"))]
    Glasses,
    #[strum(props(File = "globe"))]
    Globe,
    #[strum(props(File = "globe-2"))]
    Globe2,
    #[strum(props(File = "goal"))]
    Goal,
    #[strum(props(File = "grab"))]
    Grab,
    #[strum(props(File = "graduation-cap"))]
    GraduationCap,
    #[strum(props(File = "grape"))]
    Grape,
    #[strum(props(File = "grid-2x2"))]
    Grid2x2,
    #[strum(props(File = "grid-3x3"))]
    Grid3x3,
    #[strum(props(File = "grip"))]
    Grip,
    #[strum(props(File = "grip-horizontal"))]
    GripHorizontal,
    #[strum(props(File = "grip-vertical"))]
    GripVertical,
    #[strum(props(File = "group"))]
    Group,
    #[strum(props(File = "hammer"))]
    Hammer,
    #[strum(props(File = "hand"))]
    Hand,
    #[strum(props(File = "hand-metal"))]
    HandMetal,
    #[strum(props(File = "hard-drive"))]
    HardDrive,
    #[strum(props(File = "hard-drive-download"))]
    HardDriveDownload,
    #[strum(props(File = "hard-drive-upload"))]
    HardDriveUpload,
    #[strum(props(File = "hard-hat"))]
    HardHat,
    #[strum(props(File = "hash"))]
    Hash,
    #[strum(props(File = "haze"))]
    Haze,
    #[strum(props(File = "hdmi-port"))]
    HdmiPort,
    #[strum(props(File = "heading"))]
    Heading,
    #[strum(props(File = "heading-1"))]
    Heading1,
    #[strum(props(File = "heading-2"))]
    Heading2,
    #[strum(props(File = "heading-3"))]
    Heading3,
    #[strum(props(File = "heading-4"))]
    Heading4,
    #[strum(props(File = "heading-5"))]
    Heading5,
    #[strum(props(File = "heading-6"))]
    Heading6,
    #[strum(props(File = "headphones"))]
    Headphones,
    #[strum(props(File = "heart"))]
    Heart,
    #[strum(props(File = "heart-crack"))]
    HeartCrack,
    #[strum(props(File = "heart-handshake"))]
    HeartHandshake,
    #[strum(props(File = "heart-off"))]
    HeartOff,
    #[strum(props(File = "heart-pulse"))]
    HeartPulse,
    #[strum(props(File = "help-circle"))]
    HelpCircle,
    #[strum(props(File = "helping-hand"))]
    HelpingHand,
    #[strum(props(File = "hexagon"))]
    Hexagon,
    #[strum(props(File = "highlighter"))]
    Highlighter,
    #[strum(props(File = "history"))]
    History,
    #[strum(props(File = "home"))]
    Home,
    #[strum(props(File = "hop"))]
    Hop,
    #[strum(props(File = "hop-off"))]
    HopOff,
    #[strum(props(File = "hotel"))]
    Hotel,
    #[strum(props(File = "hourglass"))]
    Hourglass,
    #[strum(props(File = "ice-cream"))]
    IceCream,
    #[strum(props(File = "ice-cream-2"))]
    IceCream2,
    #[strum(props(File = "image"))]
    Image,
    #[strum(props(File = "image-minus"))]
    ImageMinus,
    #[strum(props(File = "image-off"))]
    ImageOff,
    #[strum(props(File = "image-plus"))]
    ImagePlus,
    #[strum(props(File = "import"))]
    Import,
    #[strum(props(File = "inbox"))]
    Inbox,
    #[strum(props(File = "indent"))]
    Indent,
    #[strum(props(File = "indian-rupee"))]
    IndianRupee,
    #[strum(props(File = "infinity"))]
    Infinity,
    #[strum(props(File = "info"))]
    Info,
    #[strum(props(File = "instagram"))]
    Instagram,
    #[strum(props(File = "italic"))]
    Italic,
    #[strum(props(File = "iteration-ccw"))]
    IterationCcw,
    #[strum(props(File = "iteration-cw"))]
    IterationCw,
    #[strum(props(File = "japanese-yen"))]
    JapaneseYen,
    #[strum(props(File = "joystick"))]
    Joystick,
    #[strum(props(File = "kanban"))]
    Kanban,
    #[strum(props(File = "kanban-square"))]
    KanbanSquare,
    #[strum(props(File = "kanban-square-dashed"))]
    KanbanSquareDashed,
    #[strum(props(File = "key"))]
    Key,
    #[strum(props(File = "key-round"))]
    KeyRound,
    #[strum(props(File = "key-square"))]
    KeySquare,
    #[strum(props(File = "keyboard"))]
    Keyboard,
    #[strum(props(File = "lamp"))]
    Lamp,
    #[strum(props(File = "lamp-ceiling"))]
    LampCeiling,
    #[strum(props(File = "lamp-desk"))]
    LampDesk,
    #[strum(props(File = "lamp-floor"))]
    LampFloor,
    #[strum(props(File = "lamp-wall-down"))]
    LampWallDown,
    #[strum(props(File = "lamp-wall-up"))]
    LampWallUp,
    #[strum(props(File = "landmark"))]
    Landmark,
    #[strum(props(File = "languages"))]
    Languages,
    #[strum(props(File = "laptop"))]
    Laptop,
    #[strum(props(File = "laptop-2"))]
    Laptop2,
    #[strum(props(File = "lasso"))]
    Lasso,
    #[strum(props(File = "lasso-select"))]
    LassoSelect,
    #[strum(props(File = "laugh"))]
    Laugh,
    #[strum(props(File = "layers"))]
    Layers,
    #[strum(props(File = "layout"))]
    Layout,
    #[strum(props(File = "layout-dashboard"))]
    LayoutDashboard,
    #[strum(props(File = "layout-grid"))]
    LayoutGrid,
    #[strum(props(File = "layout-list"))]
    LayoutList,
    #[strum(props(File = "layout-panel-left"))]
    LayoutPanelLeft,
    #[strum(props(File = "layout-panel-top"))]
    LayoutPanelTop,
    #[strum(props(File = "layout-template"))]
    LayoutTemplate,
    #[strum(props(File = "leaf"))]
    Leaf,
    #[strum(props(File = "leafy-green"))]
    LeafyGreen,
    #[strum(props(File = "library"))]
    Library,
    #[strum(props(File = "life-buoy"))]
    LifeBuoy,
    #[strum(props(File = "ligature"))]
    Ligature,
    #[strum(props(File = "lightbulb"))]
    Lightbulb,
    #[strum(props(File = "lightbulb-off"))]
    LightbulbOff,
    #[strum(props(File = "line-chart"))]
    LineChart,
    #[strum(props(File = "link"))]
    Link,
    #[strum(props(File = "link-2"))]
    Link2,
    #[strum(props(File = "link-2-off"))]
    Link2Off,
    #[strum(props(File = "linkedin"))]
    Linkedin,
    #[strum(props(File = "list"))]
    List,
    #[strum(props(File = "list-checks"))]
    ListChecks,
    #[strum(props(File = "list-end"))]
    ListEnd,
    #[strum(props(File = "list-filter"))]
    ListFilter,
    #[strum(props(File = "list-minus"))]
    ListMinus,
    #[strum(props(File = "list-music"))]
    ListMusic,
    #[strum(props(File = "list-ordered"))]
    ListOrdered,
    #[strum(props(File = "list-plus"))]
    ListPlus,
    #[strum(props(File = "list-restart"))]
    ListRestart,
    #[strum(props(File = "list-start"))]
    ListStart,
    #[strum(props(File = "list-todo"))]
    ListTodo,
    #[strum(props(File = "list-tree"))]
    ListTree,
    #[strum(props(File = "list-video"))]
    ListVideo,
    #[strum(props(File = "list-x"))]
    ListX,
    #[strum(props(File = "loader"))]
    Loader,
    #[strum(props(File = "loader-2"))]
    Loader2,
    #[strum(props(File = "locate"))]
    Locate,
    #[strum(props(File = "locate-fixed"))]
    LocateFixed,
    #[strum(props(File = "locate-off"))]
    LocateOff,
    #[strum(props(File = "lock"))]
    Lock,
    #[strum(props(File = "log-in"))]
    LogIn,
    #[strum(props(File = "log-out"))]
    LogOut,
    #[strum(props(File = "lollipop"))]
    Lollipop,
    #[strum(props(File = "luggage"))]
    Luggage,
    #[strum(props(File = "m-square"))]
    MSquare,
    #[strum(props(File = "magnet"))]
    Magnet,
    #[strum(props(File = "mail"))]
    Mail,
    #[strum(props(File = "mail-check"))]
    MailCheck,
    #[strum(props(File = "mail-minus"))]
    MailMinus,
    #[strum(props(File = "mail-open"))]
    MailOpen,
    #[strum(props(File = "mail-plus"))]
    MailPlus,
    #[strum(props(File = "mail-question"))]
    MailQuestion,
    #[strum(props(File = "mail-search"))]
    MailSearch,
    #[strum(props(File = "mail-warning"))]
    MailWarning,
    #[strum(props(File = "mail-x"))]
    MailX,
    #[strum(props(File = "mailbox"))]
    Mailbox,
    #[strum(props(File = "mails"))]
    Mails,
    #[strum(props(File = "map"))]
    Map,
    #[strum(props(File = "map-pin"))]
    MapPin,
    #[strum(props(File = "map-pin-off"))]
    MapPinOff,
    #[strum(props(File = "martini"))]
    Martini,
    #[strum(props(File = "maximize"))]
    Maximize,
    #[strum(props(File = "maximize-2"))]
    Maximize2,
    #[strum(props(File = "medal"))]
    Medal,
    #[strum(props(File = "megaphone"))]
    Megaphone,
    #[strum(props(File = "megaphone-off"))]
    MegaphoneOff,
    #[strum(props(File = "meh"))]
    Meh,
    #[strum(props(File = "memory-stick"))]
    MemoryStick,
    #[strum(props(File = "menu"))]
    Menu,
    #[strum(props(File = "menu-square"))]
    MenuSquare,
    #[strum(props(File = "merge"))]
    Merge,
    #[strum(props(File = "message-circle"))]
    MessageCircle,
    #[strum(props(File = "message-square"))]
    MessageSquare,
    #[strum(props(File = "message-square-dashed"))]
    MessageSquareDashed,
    #[strum(props(File = "message-square-plus"))]
    MessageSquarePlus,
    #[strum(props(File = "messages-square"))]
    MessagesSquare,
    #[strum(props(File = "mic"))]
    Mic,
    #[strum(props(File = "mic-2"))]
    Mic2,
    #[strum(props(File = "mic-off"))]
    MicOff,
    #[strum(props(File = "microscope"))]
    Microscope,
    #[strum(props(File = "microwave"))]
    Microwave,
    #[strum(props(File = "milestone"))]
    Milestone,
    #[strum(props(File = "milk"))]
    Milk,
    #[strum(props(File = "milk-off"))]
    MilkOff,
    #[strum(props(File = "minimize"))]
    Minimize,
    #[strum(props(File = "minimize-2"))]
    Minimize2,
    #[strum(props(File = "minus"))]
    Minus,
    #[strum(props(File = "minus-circle"))]
    MinusCircle,
    #[strum(props(File = "minus-square"))]
    MinusSquare,
    #[strum(props(File = "monitor"))]
    Monitor,
    #[strum(props(File = "monitor-check"))]
    MonitorCheck,
    #[strum(props(File = "monitor-dot"))]
    MonitorDot,
    #[strum(props(File = "monitor-down"))]
    MonitorDown,
    #[strum(props(File = "monitor-off"))]
    MonitorOff,
    #[strum(props(File = "monitor-pause"))]
    MonitorPause,
    #[strum(props(File = "monitor-play"))]
    MonitorPlay,
    #[strum(props(File = "monitor-smartphone"))]
    MonitorSmartphone,
    #[strum(props(File = "monitor-speaker"))]
    MonitorSpeaker,
    #[strum(props(File = "monitor-stop"))]
    MonitorStop,
    #[strum(props(File = "monitor-up"))]
    MonitorUp,
    #[strum(props(File = "monitor-x"))]
    MonitorX,
    #[strum(props(File = "moon"))]
    Moon,
    #[strum(props(File = "moon-star"))]
    MoonStar,
    #[strum(props(File = "more-horizontal"))]
    MoreHorizontal,
    #[strum(props(File = "more-vertical"))]
    MoreVertical,
    #[strum(props(File = "mountain"))]
    Mountain,
    #[strum(props(File = "mountain-snow"))]
    MountainSnow,
    #[strum(props(File = "mouse"))]
    Mouse,
    #[strum(props(File = "mouse-pointer"))]
    MousePointer,
    #[strum(props(File = "mouse-pointer-2"))]
    MousePointer2,
    #[strum(props(File = "mouse-pointer-click"))]
    MousePointerClick,
    #[strum(props(File = "mouse-pointer-square"))]
    MousePointerSquare,
    #[strum(props(File = "mouse-pointer-square-dashed"))]
    MousePointerSquareDashed,
    #[strum(props(File = "move"))]
    Move,
    #[strum(props(File = "move-3d"))]
    Move3d,
    #[strum(props(File = "move-diagonal"))]
    MoveDiagonal,
    #[strum(props(File = "move-diagonal-2"))]
    MoveDiagonal2,
    #[strum(props(File = "move-down"))]
    MoveDown,
    #[strum(props(File = "move-down-left"))]
    MoveDownLeft,
    #[strum(props(File = "move-down-right"))]
    MoveDownRight,
    #[strum(props(File = "move-horizontal"))]
    MoveHorizontal,
    #[strum(props(File = "move-left"))]
    MoveLeft,
    #[strum(props(File = "move-right"))]
    MoveRight,
    #[strum(props(File = "move-up"))]
    MoveUp,
    #[strum(props(File = "move-up-left"))]
    MoveUpLeft,
    #[strum(props(File = "move-up-right"))]
    MoveUpRight,
    #[strum(props(File = "move-vertical"))]
    MoveVertical,
    #[strum(props(File = "music"))]
    Music,
    #[strum(props(File = "music-2"))]
    Music2,
    #[strum(props(File = "music-3"))]
    Music3,
    #[strum(props(File = "music-4"))]
    Music4,
    #[strum(props(File = "navigation"))]
    Navigation,
    #[strum(props(File = "navigation-2"))]
    Navigation2,
    #[strum(props(File = "navigation-2-off"))]
    Navigation2Off,
    #[strum(props(File = "navigation-off"))]
    NavigationOff,
    #[strum(props(File = "network"))]
    Network,
    #[strum(props(File = "newspaper"))]
    Newspaper,
    #[strum(props(File = "nfc"))]
    Nfc,
    #[strum(props(File = "nut"))]
    Nut,
    #[strum(props(File = "nut-off"))]
    NutOff,
    #[strum(props(File = "octagon"))]
    Octagon,
    #[strum(props(File = "option"))]
    Option,
    #[strum(props(File = "orbit"))]
    Orbit,
    #[strum(props(File = "outdent"))]
    Outdent,
    #[strum(props(File = "package"))]
    Package,
    #[strum(props(File = "package-2"))]
    Package2,
    #[strum(props(File = "package-check"))]
    PackageCheck,
    #[strum(props(File = "package-minus"))]
    PackageMinus,
    #[strum(props(File = "package-open"))]
    PackageOpen,
    #[strum(props(File = "package-plus"))]
    PackagePlus,
    #[strum(props(File = "package-search"))]
    PackageSearch,
    #[strum(props(File = "package-x"))]
    PackageX,
    #[strum(props(File = "paint-bucket"))]
    PaintBucket,
    #[strum(props(File = "paintbrush"))]
    Paintbrush,
    #[strum(props(File = "paintbrush-2"))]
    Paintbrush2,
    #[strum(props(File = "palette"))]
    Palette,
    #[strum(props(File = "palmtree"))]
    Palmtree,
    #[strum(props(File = "panel-bottom"))]
    PanelBottom,
    #[strum(props(File = "panel-bottom-close"))]
    PanelBottomClose,
    #[strum(props(File = "panel-bottom-inactive"))]
    PanelBottomInactive,
    #[strum(props(File = "panel-bottom-open"))]
    PanelBottomOpen,
    #[strum(props(File = "panel-left"))]
    PanelLeft,
    #[strum(props(File = "panel-left-close"))]
    PanelLeftClose,
    #[strum(props(File = "panel-left-inactive"))]
    PanelLeftInactive,
    #[strum(props(File = "panel-left-open"))]
    PanelLeftOpen,
    #[strum(props(File = "panel-right"))]
    PanelRight,
    #[strum(props(File = "panel-right-close"))]
    PanelRightClose,
    #[strum(props(File = "panel-right-inactive"))]
    PanelRightInactive,
    #[strum(props(File = "panel-right-open"))]
    PanelRightOpen,
    #[strum(props(File = "panel-top"))]
    PanelTop,
    #[strum(props(File = "panel-top-close"))]
    PanelTopClose,
    #[strum(props(File = "panel-top-inactive"))]
    PanelTopInactive,
    #[strum(props(File = "panel-top-open"))]
    PanelTopOpen,
    #[strum(props(File = "paperclip"))]
    Paperclip,
    #[strum(props(File = "parentheses"))]
    Parentheses,
    #[strum(props(File = "parking-circle"))]
    ParkingCircle,
    #[strum(props(File = "parking-circle-off"))]
    ParkingCircleOff,
    #[strum(props(File = "parking-meter"))]
    ParkingMeter,
    #[strum(props(File = "parking-square"))]
    ParkingSquare,
    #[strum(props(File = "parking-square-off"))]
    ParkingSquareOff,
    #[strum(props(File = "party-popper"))]
    PartyPopper,
    #[strum(props(File = "pause"))]
    Pause,
    #[strum(props(File = "pause-circle"))]
    PauseCircle,
    #[strum(props(File = "pause-octagon"))]
    PauseOctagon,
    #[strum(props(File = "paw-print"))]
    PawPrint,
    #[strum(props(File = "pc-case"))]
    PcCase,
    #[strum(props(File = "pen"))]
    Pen,
    #[strum(props(File = "pen-line"))]
    PenLine,
    #[strum(props(File = "pen-square"))]
    PenSquare,
    #[strum(props(File = "pen-tool"))]
    PenTool,
    #[strum(props(File = "pencil"))]
    Pencil,
    #[strum(props(File = "pencil-line"))]
    PencilLine,
    #[strum(props(File = "pencil-ruler"))]
    PencilRuler,
    #[strum(props(File = "percent"))]
    Percent,
    #[strum(props(File = "percent-circle"))]
    PercentCircle,
    #[strum(props(File = "percent-diamond"))]
    PercentDiamond,
    #[strum(props(File = "percent-square"))]
    PercentSquare,
    #[strum(props(File = "person-standing"))]
    PersonStanding,
    #[strum(props(File = "phone"))]
    Phone,
    #[strum(props(File = "phone-call"))]
    PhoneCall,
    #[strum(props(File = "phone-forwarded"))]
    PhoneForwarded,
    #[strum(props(File = "phone-incoming"))]
    PhoneIncoming,
    #[strum(props(File = "phone-missed"))]
    PhoneMissed,
    #[strum(props(File = "phone-off"))]
    PhoneOff,
    #[strum(props(File = "phone-outgoing"))]
    PhoneOutgoing,
    #[strum(props(File = "pi"))]
    Pi,
    #[strum(props(File = "pi-square"))]
    PiSquare,
    #[strum(props(File = "picture-in-picture"))]
    PictureInPicture,
    #[strum(props(File = "picture-in-picture-2"))]
    PictureInPicture2,
    #[strum(props(File = "pie-chart"))]
    PieChart,
    #[strum(props(File = "piggy-bank"))]
    PiggyBank,
    #[strum(props(File = "pilcrow"))]
    Pilcrow,
    #[strum(props(File = "pilcrow-square"))]
    PilcrowSquare,
    #[strum(props(File = "pill"))]
    Pill,
    #[strum(props(File = "pin"))]
    Pin,
    #[strum(props(File = "pin-off"))]
    PinOff,
    #[strum(props(File = "pipette"))]
    Pipette,
    #[strum(props(File = "pizza"))]
    Pizza,
    #[strum(props(File = "plane"))]
    Plane,
    #[strum(props(File = "plane-landing"))]
    PlaneLanding,
    #[strum(props(File = "plane-takeoff"))]
    PlaneTakeoff,
    #[strum(props(File = "play"))]
    Play,
    #[strum(props(File = "play-circle"))]
    PlayCircle,
    #[strum(props(File = "play-square"))]
    PlaySquare,
    #[strum(props(File = "plug"))]
    Plug,
    #[strum(props(File = "plug-2"))]
    Plug2,
    #[strum(props(File = "plug-zap"))]
    PlugZap,
    #[strum(props(File = "plug-zap-2"))]
    PlugZap2,
    #[strum(props(File = "plus"))]
    Plus,
    #[strum(props(File = "plus-circle"))]
    PlusCircle,
    #[strum(props(File = "plus-square"))]
    PlusSquare,
    #[strum(props(File = "pocket"))]
    Pocket,
    #[strum(props(File = "pocket-knife"))]
    PocketKnife,
    #[strum(props(File = "podcast"))]
    Podcast,
    #[strum(props(File = "pointer"))]
    Pointer,
    #[strum(props(File = "popcorn"))]
    Popcorn,
    #[strum(props(File = "popsicle"))]
    Popsicle,
    #[strum(props(File = "pound-sterling"))]
    PoundSterling,
    #[strum(props(File = "power"))]
    Power,
    #[strum(props(File = "power-off"))]
    PowerOff,
    #[strum(props(File = "presentation"))]
    Presentation,
    #[strum(props(File = "printer"))]
    Printer,
    #[strum(props(File = "projector"))]
    Projector,
    #[strum(props(File = "puzzle"))]
    Puzzle,
    #[strum(props(File = "qr-code"))]
    QrCode,
    #[strum(props(File = "quote"))]
    Quote,
    #[strum(props(File = "rabbit"))]
    Rabbit,
    #[strum(props(File = "radar"))]
    Radar,
    #[strum(props(File = "radiation"))]
    Radiation,
    #[strum(props(File = "radio"))]
    Radio,
    #[strum(props(File = "radio-receiver"))]
    RadioReceiver,
    #[strum(props(File = "radio-tower"))]
    RadioTower,
    #[strum(props(File = "rail-symbol"))]
    RailSymbol,
    #[strum(props(File = "rainbow"))]
    Rainbow,
    #[strum(props(File = "rat"))]
    Rat,
    #[strum(props(File = "ratio"))]
    Ratio,
    #[strum(props(File = "receipt"))]
    Receipt,
    #[strum(props(File = "rectangle-horizontal"))]
    RectangleHorizontal,
    #[strum(props(File = "rectangle-vertical"))]
    RectangleVertical,
    #[strum(props(File = "recycle"))]
    Recycle,
    #[strum(props(File = "redo"))]
    Redo,
    #[strum(props(File = "redo-2"))]
    Redo2,
    #[strum(props(File = "redo-dot"))]
    RedoDot,
    #[strum(props(File = "refresh-ccw"))]
    RefreshCcw,
    #[strum(props(File = "refresh-ccw-dot"))]
    RefreshCcwDot,
    #[strum(props(File = "refresh-cw"))]
    RefreshCw,
    #[strum(props(File = "refresh-cw-off"))]
    RefreshCwOff,
    #[strum(props(File = "refrigerator"))]
    Refrigerator,
    #[strum(props(File = "regex"))]
    Regex,
    #[strum(props(File = "remove-formatting"))]
    RemoveFormatting,
    #[strum(props(File = "repeat"))]
    Repeat,
    #[strum(props(File = "repeat-1"))]
    Repeat1,
    #[strum(props(File = "repeat-2"))]
    Repeat2,
    #[strum(props(File = "replace"))]
    Replace,
    #[strum(props(File = "replace-all"))]
    ReplaceAll,
    #[strum(props(File = "reply"))]
    Reply,
    #[strum(props(File = "reply-all"))]
    ReplyAll,
    #[strum(props(File = "rewind"))]
    Rewind,
    #[strum(props(File = "rocket"))]
    Rocket,
    #[strum(props(File = "rocking-chair"))]
    RockingChair,
    #[strum(props(File = "roller-coaster"))]
    RollerCoaster,
    #[strum(props(File = "rotate-3d"))]
    Rotate3d,
    #[strum(props(File = "rotate-ccw"))]
    RotateCcw,
    #[strum(props(File = "rotate-cw"))]
    RotateCw,
    #[strum(props(File = "router"))]
    Router,
    #[strum(props(File = "rows"))]
    Rows,
    #[strum(props(File = "rss"))]
    Rss,
    #[strum(props(File = "ruler"))]
    Ruler,
    #[strum(props(File = "russian-ruble"))]
    RussianRuble,
    #[strum(props(File = "sailboat"))]
    Sailboat,
    #[strum(props(File = "salad"))]
    Salad,
    #[strum(props(File = "sandwich"))]
    Sandwich,
    #[strum(props(File = "satellite"))]
    Satellite,
    #[strum(props(File = "satellite-dish"))]
    SatelliteDish,
    #[strum(props(File = "save"))]
    Save,
    #[strum(props(File = "save-all"))]
    SaveAll,
    #[strum(props(File = "scale"))]
    Scale,
    #[strum(props(File = "scale-3d"))]
    Scale3d,
    #[strum(props(File = "scaling"))]
    Scaling,
    #[strum(props(File = "scan"))]
    Scan,
    #[strum(props(File = "scan-face"))]
    ScanFace,
    #[strum(props(File = "scan-line"))]
    ScanLine,
    #[strum(props(File = "scatter-chart"))]
    ScatterChart,
    #[strum(props(File = "school"))]
    School,
    #[strum(props(File = "school-2"))]
    School2,
    #[strum(props(File = "scissors"))]
    Scissors,
    #[strum(props(File = "scissors-line-dashed"))]
    ScissorsLineDashed,
    #[strum(props(File = "scissors-square"))]
    ScissorsSquare,
    #[strum(props(File = "scissors-square-dashed-bottom"))]
    ScissorsSquareDashedBottom,
    #[strum(props(File = "screen-share"))]
    ScreenShare,
    #[strum(props(File = "screen-share-off"))]
    ScreenShareOff,
    #[strum(props(File = "scroll"))]
    Scroll,
    #[strum(props(File = "scroll-text"))]
    ScrollText,
    #[strum(props(File = "search"))]
    Search,
    #[strum(props(File = "search-check"))]
    SearchCheck,
    #[strum(props(File = "search-code"))]
    SearchCode,
    #[strum(props(File = "search-slash"))]
    SearchSlash,
    #[strum(props(File = "search-x"))]
    SearchX,
    #[strum(props(File = "send"))]
    Send,
    #[strum(props(File = "send-horizontal"))]
    SendHorizontal,
    #[strum(props(File = "send-to-back"))]
    SendToBack,
    #[strum(props(File = "separator-horizontal"))]
    SeparatorHorizontal,
    #[strum(props(File = "separator-vertical"))]
    SeparatorVertical,
    #[strum(props(File = "server"))]
    Server,
    #[strum(props(File = "server-cog"))]
    ServerCog,
    #[strum(props(File = "server-crash"))]
    ServerCrash,
    #[strum(props(File = "server-off"))]
    ServerOff,
    #[strum(props(File = "settings"))]
    Settings,
    #[strum(props(File = "settings-2"))]
    Settings2,
    #[strum(props(File = "shapes"))]
    Shapes,
    #[strum(props(File = "share"))]
    Share,
    #[strum(props(File = "share-2"))]
    Share2,
    #[strum(props(File = "sheet"))]
    Sheet,
    #[strum(props(File = "shell"))]
    Shell,
    #[strum(props(File = "shield"))]
    Shield,
    #[strum(props(File = "shield-alert"))]
    ShieldAlert,
    #[strum(props(File = "shield-ban"))]
    ShieldBan,
    #[strum(props(File = "shield-check"))]
    ShieldCheck,
    #[strum(props(File = "shield-ellipsis"))]
    ShieldEllipsis,
    #[strum(props(File = "shield-half"))]
    ShieldHalf,
    #[strum(props(File = "shield-minus"))]
    ShieldMinus,
    #[strum(props(File = "shield-off"))]
    ShieldOff,
    #[strum(props(File = "shield-plus"))]
    ShieldPlus,
    #[strum(props(File = "shield-question"))]
    ShieldQuestion,
    #[strum(props(File = "shield-x"))]
    ShieldX,
    #[strum(props(File = "ship"))]
    Ship,
    #[strum(props(File = "ship-wheel"))]
    ShipWheel,
    #[strum(props(File = "shirt"))]
    Shirt,
    #[strum(props(File = "shopping-bag"))]
    ShoppingBag,
    #[strum(props(File = "shopping-basket"))]
    ShoppingBasket,
    #[strum(props(File = "shopping-cart"))]
    ShoppingCart,
    #[strum(props(File = "shovel"))]
    Shovel,
    #[strum(props(File = "shower-head"))]
    ShowerHead,
    #[strum(props(File = "shrink"))]
    Shrink,
    #[strum(props(File = "shrub"))]
    Shrub,
    #[strum(props(File = "shuffle"))]
    Shuffle,
    #[strum(props(File = "sigma"))]
    Sigma,
    #[strum(props(File = "sigma-square"))]
    SigmaSquare,
    #[strum(props(File = "signal"))]
    Signal,
    #[strum(props(File = "signal-high"))]
    SignalHigh,
    #[strum(props(File = "signal-low"))]
    SignalLow,
    #[strum(props(File = "signal-medium"))]
    SignalMedium,
    #[strum(props(File = "signal-zero"))]
    SignalZero,
    #[strum(props(File = "siren"))]
    Siren,
    #[strum(props(File = "skip-back"))]
    SkipBack,
    #[strum(props(File = "skip-forward"))]
    SkipForward,
    #[strum(props(File = "skull"))]
    Skull,
    #[strum(props(File = "slack"))]
    Slack,
    #[strum(props(File = "slash"))]
    Slash,
    #[strum(props(File = "slice"))]
    Slice,
    #[strum(props(File = "sliders"))]
    Sliders,
    #[strum(props(File = "sliders-horizontal"))]
    SlidersHorizontal,
    #[strum(props(File = "smartphone"))]
    Smartphone,
    #[strum(props(File = "smartphone-charging"))]
    SmartphoneCharging,
    #[strum(props(File = "smartphone-nfc"))]
    SmartphoneNfc,
    #[strum(props(File = "smile"))]
    Smile,
    #[strum(props(File = "smile-plus"))]
    SmilePlus,
    #[strum(props(File = "snail"))]
    Snail,
    #[strum(props(File = "snowflake"))]
    Snowflake,
    #[strum(props(File = "sofa"))]
    Sofa,
    #[strum(props(File = "soup"))]
    Soup,
    #[strum(props(File = "space"))]
    Space,
    #[strum(props(File = "spade"))]
    Spade,
    #[strum(props(File = "sparkle"))]
    Sparkle,
    #[strum(props(File = "sparkles"))]
    Sparkles,
    #[strum(props(File = "speaker"))]
    Speaker,
    #[strum(props(File = "spell-check"))]
    SpellCheck,
    #[strum(props(File = "spell-check-2"))]
    SpellCheck2,
    #[strum(props(File = "spline"))]
    Spline,
    #[strum(props(File = "split"))]
    Split,
    #[strum(props(File = "split-square-horizontal"))]
    SplitSquareHorizontal,
    #[strum(props(File = "split-square-vertical"))]
    SplitSquareVertical,
    #[strum(props(File = "spray-can"))]
    SprayCan,
    #[strum(props(File = "sprout"))]
    Sprout,
    #[strum(props(File = "square"))]
    Square,
    #[strum(props(File = "square-asterisk"))]
    SquareAsterisk,
    #[strum(props(File = "square-code"))]
    SquareCode,
    #[strum(props(File = "square-dashed-bottom"))]
    SquareDashedBottom,
    #[strum(props(File = "square-dashed-bottom-code"))]
    SquareDashedBottomCode,
    #[strum(props(File = "square-dot"))]
    SquareDot,
    #[strum(props(File = "square-equal"))]
    SquareEqual,
    #[strum(props(File = "square-slash"))]
    SquareSlash,
    #[strum(props(File = "square-stack"))]
    SquareStack,
    #[strum(props(File = "squirrel"))]
    Squirrel,
    #[strum(props(File = "stamp"))]
    Stamp,
    #[strum(props(File = "star"))]
    Star,
    #[strum(props(File = "star-half"))]
    StarHalf,
    #[strum(props(File = "star-off"))]
    StarOff,
    #[strum(props(File = "step-back"))]
    StepBack,
    #[strum(props(File = "step-forward"))]
    StepForward,
    #[strum(props(File = "stethoscope"))]
    Stethoscope,
    #[strum(props(File = "sticker"))]
    Sticker,
    #[strum(props(File = "sticky-note"))]
    StickyNote,
    #[strum(props(File = "stop-circle"))]
    StopCircle,
    #[strum(props(File = "store"))]
    Store,
    #[strum(props(File = "stretch-horizontal"))]
    StretchHorizontal,
    #[strum(props(File = "stretch-vertical"))]
    StretchVertical,
    #[strum(props(File = "strikethrough"))]
    Strikethrough,
    #[strum(props(File = "subscript"))]
    Subscript,
    #[strum(props(File = "subtitles"))]
    Subtitles,
    #[strum(props(File = "sun"))]
    Sun,
    #[strum(props(File = "sun-dim"))]
    SunDim,
    #[strum(props(File = "sun-medium"))]
    SunMedium,
    #[strum(props(File = "sun-moon"))]
    SunMoon,
    #[strum(props(File = "sun-snow"))]
    SunSnow,
    #[strum(props(File = "sunrise"))]
    Sunrise,
    #[strum(props(File = "sunset"))]
    Sunset,
    #[strum(props(File = "superscript"))]
    Superscript,
    #[strum(props(File = "swiss-franc"))]
    SwissFranc,
    #[strum(props(File = "switch-camera"))]
    SwitchCamera,
    #[strum(props(File = "sword"))]
    Sword,
    #[strum(props(File = "swords"))]
    Swords,
    #[strum(props(File = "syringe"))]
    Syringe,
    #[strum(props(File = "table"))]
    Table,
    #[strum(props(File = "table-2"))]
    Table2,
    #[strum(props(File = "table-properties"))]
    TableProperties,
    #[strum(props(File = "tablet"))]
    Tablet,
    #[strum(props(File = "tablet-smartphone"))]
    TabletSmartphone,
    #[strum(props(File = "tablets"))]
    Tablets,
    #[strum(props(File = "tag"))]
    Tag,
    #[strum(props(File = "tags"))]
    Tags,
    #[strum(props(File = "tally-1"))]
    Tally1,
    #[strum(props(File = "tally-2"))]
    Tally2,
    #[strum(props(File = "tally-3"))]
    Tally3,
    #[strum(props(File = "tally-4"))]
    Tally4,
    #[strum(props(File = "tally-5"))]
    Tally5,
    #[strum(props(File = "target"))]
    Target,
    #[strum(props(File = "tent"))]
    Tent,
    #[strum(props(File = "terminal"))]
    Terminal,
    #[strum(props(File = "terminal-square"))]
    TerminalSquare,
    #[strum(props(File = "test-tube"))]
    TestTube,
    #[strum(props(File = "test-tube-2"))]
    TestTube2,
    #[strum(props(File = "test-tubes"))]
    TestTubes,
    #[strum(props(File = "text"))]
    Text,
    #[strum(props(File = "text-cursor"))]
    TextCursor,
    #[strum(props(File = "text-cursor-input"))]
    TextCursorInput,
    #[strum(props(File = "text-quote"))]
    TextQuote,
    #[strum(props(File = "text-select"))]
    TextSelect,
    #[strum(props(File = "thermometer"))]
    Thermometer,
    #[strum(props(File = "thermometer-snowflake"))]
    ThermometerSnowflake,
    #[strum(props(File = "thermometer-sun"))]
    ThermometerSun,
    #[strum(props(File = "thumbs-down"))]
    ThumbsDown,
    #[strum(props(File = "thumbs-up"))]
    ThumbsUp,
    #[strum(props(File = "ticket"))]
    Ticket,
    #[strum(props(File = "timer"))]
    Timer,
    #[strum(props(File = "timer-off"))]
    TimerOff,
    #[strum(props(File = "timer-reset"))]
    TimerReset,
    #[strum(props(File = "toggle-left"))]
    ToggleLeft,
    #[strum(props(File = "toggle-right"))]
    ToggleRight,
    #[strum(props(File = "tornado"))]
    Tornado,
    #[strum(props(File = "touchpad"))]
    Touchpad,
    #[strum(props(File = "touchpad-off"))]
    TouchpadOff,
    #[strum(props(File = "tower-control"))]
    TowerControl,
    #[strum(props(File = "toy-brick"))]
    ToyBrick,
    #[strum(props(File = "tractor"))]
    Tractor,
    #[strum(props(File = "traffic-cone"))]
    TrafficCone,
    #[strum(props(File = "train-front"))]
    TrainFront,
    #[strum(props(File = "train-front-tunnel"))]
    TrainFrontTunnel,
    #[strum(props(File = "train-track"))]
    TrainTrack,
    #[strum(props(File = "tram-front"))]
    TramFront,
    #[strum(props(File = "trash"))]
    Trash,
    #[strum(props(File = "trash-2"))]
    Trash2,
    #[strum(props(File = "tree-deciduous"))]
    TreeDeciduous,
    #[strum(props(File = "tree-pine"))]
    TreePine,
    #[strum(props(File = "trees"))]
    Trees,
    #[strum(props(File = "trello"))]
    Trello,
    #[strum(props(File = "trending-down"))]
    TrendingDown,
    #[strum(props(File = "trending-up"))]
    TrendingUp,
    #[strum(props(File = "triangle"))]
    Triangle,
    #[strum(props(File = "triangle-right"))]
    TriangleRight,
    #[strum(props(File = "trophy"))]
    Trophy,
    #[strum(props(File = "truck"))]
    Truck,
    #[strum(props(File = "turtle"))]
    Turtle,
    #[strum(props(File = "tv"))]
    Tv,
    #[strum(props(File = "tv-2"))]
    Tv2,
    #[strum(props(File = "twitch"))]
    Twitch,
    #[strum(props(File = "twitter"))]
    Twitter,
    #[strum(props(File = "type"))]
    Type,
    #[strum(props(File = "umbrella"))]
    Umbrella,
    #[strum(props(File = "underline"))]
    Underline,
    #[strum(props(File = "undo"))]
    Undo,
    #[strum(props(File = "undo-2"))]
    Undo2,
    #[strum(props(File = "undo-dot"))]
    UndoDot,
    #[strum(props(File = "unfold-horizontal"))]
    UnfoldHorizontal,
    #[strum(props(File = "unfold-vertical"))]
    UnfoldVertical,
    #[strum(props(File = "ungroup"))]
    Ungroup,
    #[strum(props(File = "unlink"))]
    Unlink,
    #[strum(props(File = "unlink-2"))]
    Unlink2,
    #[strum(props(File = "unlock"))]
    Unlock,
    #[strum(props(File = "unplug"))]
    Unplug,
    #[strum(props(File = "upload"))]
    Upload,
    #[strum(props(File = "upload-cloud"))]
    UploadCloud,
    #[strum(props(File = "usb"))]
    Usb,
    #[strum(props(File = "user"))]
    User,
    #[strum(props(File = "user-2"))]
    User2,
    #[strum(props(File = "user-check"))]
    UserCheck,
    #[strum(props(File = "user-check-2"))]
    UserCheck2,
    #[strum(props(File = "user-circle"))]
    UserCircle,
    #[strum(props(File = "user-circle-2"))]
    UserCircle2,
    #[strum(props(File = "user-cog"))]
    UserCog,
    #[strum(props(File = "user-cog-2"))]
    UserCog2,
    #[strum(props(File = "user-minus"))]
    UserMinus,
    #[strum(props(File = "user-minus-2"))]
    UserMinus2,
    #[strum(props(File = "user-plus"))]
    UserPlus,
    #[strum(props(File = "user-plus-2"))]
    UserPlus2,
    #[strum(props(File = "user-square"))]
    UserSquare,
    #[strum(props(File = "user-square-2"))]
    UserSquare2,
    #[strum(props(File = "user-x"))]
    UserX,
    #[strum(props(File = "user-x-2"))]
    UserX2,
    #[strum(props(File = "users"))]
    Users,
    #[strum(props(File = "users-2"))]
    Users2,
    #[strum(props(File = "utensils"))]
    Utensils,
    #[strum(props(File = "utensils-crossed"))]
    UtensilsCrossed,
    #[strum(props(File = "utility-pole"))]
    UtilityPole,
    #[strum(props(File = "variable"))]
    Variable,
    #[strum(props(File = "vegan"))]
    Vegan,
    #[strum(props(File = "venetian-mask"))]
    VenetianMask,
    #[strum(props(File = "vibrate"))]
    Vibrate,
    #[strum(props(File = "vibrate-off"))]
    VibrateOff,
    #[strum(props(File = "video"))]
    Video,
    #[strum(props(File = "video-off"))]
    VideoOff,
    #[strum(props(File = "videotape"))]
    Videotape,
    #[strum(props(File = "view"))]
    View,
    #[strum(props(File = "voicemail"))]
    Voicemail,
    #[strum(props(File = "volume"))]
    Volume,
    #[strum(props(File = "volume-1"))]
    Volume1,
    #[strum(props(File = "volume-2"))]
    Volume2,
    #[strum(props(File = "volume-x"))]
    VolumeX,
    #[strum(props(File = "vote"))]
    Vote,
    #[strum(props(File = "wallet"))]
    Wallet,
    #[strum(props(File = "wallet-2"))]
    Wallet2,
    #[strum(props(File = "wallet-cards"))]
    WalletCards,
    #[strum(props(File = "wallpaper"))]
    Wallpaper,
    #[strum(props(File = "wand"))]
    Wand,
    #[strum(props(File = "wand-2"))]
    Wand2,
    #[strum(props(File = "warehouse"))]
    Warehouse,
    #[strum(props(File = "watch"))]
    Watch,
    #[strum(props(File = "waves"))]
    Waves,
    #[strum(props(File = "webcam"))]
    Webcam,
    #[strum(props(File = "webhook"))]
    Webhook,
    #[strum(props(File = "wheat"))]
    Wheat,
    #[strum(props(File = "wheat-off"))]
    WheatOff,
    #[strum(props(File = "whole-word"))]
    WholeWord,
    #[strum(props(File = "wifi"))]
    Wifi,
    #[strum(props(File = "wifi-off"))]
    WifiOff,
    #[strum(props(File = "wind"))]
    Wind,
    #[strum(props(File = "wine"))]
    Wine,
    #[strum(props(File = "wine-off"))]
    WineOff,
    #[strum(props(File = "workflow"))]
    Workflow,
    #[strum(props(File = "wrap-text"))]
    WrapText,
    #[strum(props(File = "wrench"))]
    Wrench,
    #[strum(props(File = "x"))]
    X,
    #[strum(props(File = "x-circle"))]
    XCircle,
    #[strum(props(File = "x-octagon"))]
    XOctagon,
    #[strum(props(File = "x-square"))]
    XSquare,
    #[strum(props(File = "youtube"))]
    Youtube,
    #[strum(props(File = "zap"))]
    Zap,
    #[strum(props(File = "zap-off"))]
    ZapOff,
    #[strum(props(File = "zoom-in"))]
    ZoomIn,
    #[strum(props(File = "zoom-out"))]
    ZoomOut,
}