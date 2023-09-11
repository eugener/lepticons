pub struct IconType<'a> {
        pub content: &'a str,
    }
    
pub const ACCESSIBILITY: IconType = IconType{ 
 content: r#"
<circle cx="16" cy="4" r="1"></circle>
<path d="m18 19 1-7-6 1"></path>
<path d="m5 8 3-3 5.5 3-2.36 3.5"></path>
<path d="M4.24 14.5a5 5 0 0 0 6.88 6"></path>
<path d="M13.76 17.5a5 5 0 0 0-6.88-6"></path>"#,
};

pub const ACTIVITY_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" y="3" rx="2" width="18" x="3"></rect>
<path d="M17 12h-2l-2 5-2-10-2 5H7"></path>"#,
};

pub const ACTIVITY: IconType = IconType{ 
 content: r#"
<path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>"#,
};

pub const AIR_VENT: IconType = IconType{ 
 content: r#"
<path d="M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path>
<path d="M6 8h12"></path>
<path d="M18.3 17.7a2.5 2.5 0 0 1-3.16 3.83 2.53 2.53 0 0 1-1.14-2V12"></path>
<path d="M6.6 15.6A2 2 0 1 0 10 17v-5"></path>"#,
};

pub const AIRPLAY: IconType = IconType{ 
 content: r#"
<path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"></path>
<polygon points="12 15 17 21 7 21 12 15"></polygon>"#,
};

pub const ALARM_CHECK: IconType = IconType{ 
 content: r#"
<circle r="8" cy="13" cx="12"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="m9 13 2 2 4-4"></path>"#,
};

pub const ALARM_CLOCK_OFF: IconType = IconType{ 
 content: r#"
<path d="M6.87 6.87a8 8 0 1 0 11.26 11.26"></path>
<path d="M19.9 14.25a8 8 0 0 0-9.15-9.15"></path>
<path d="m22 6-3-3"></path>
<path d="M6.26 18.67 4 21"></path>
<path d="m2 2 20 20"></path>
<path d="M4 4 2 6"></path>"#,
};

pub const ALARM_CLOCK: IconType = IconType{ 
 content: r#"
<circle r="8" cx="12" cy="13"></circle>
<path d="M12 9v4l2 2"></path>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>"#,
};

pub const ALARM_MINUS: IconType = IconType{ 
 content: r#"
<circle cy="13" r="8" cx="12"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="M9 13h6"></path>"#,
};

pub const ALARM_PLUS: IconType = IconType{ 
 content: r#"
<circle cy="13" cx="12" r="8"></circle>
<path d="M5 3 2 6"></path>
<path d="m22 6-3-3"></path>
<path d="M6.38 18.7 4 21"></path>
<path d="M17.64 18.67 20 21"></path>
<path d="M12 10v6"></path>
<path d="M9 13h6"></path>"#,
};

pub const ALBUM: IconType = IconType{ 
 content: r#"
<rect y="3" rx="2" width="18" ry="2" x="3" height="18"></rect>
<polyline points="11 3 11 11 14 8 17 11 17 3"></polyline>"#,
};

pub const ALERT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<line x2="12" x1="12" y1="8" y2="12"></line>
<line x1="12" x2="12.01" y2="16" y1="16"></line>"#,
};

pub const ALERT_OCTAGON: IconType = IconType{ 
 content: r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>
<line y2="12" x1="12" x2="12" y1="8"></line>
<line x2="12.01" y2="16" x1="12" y1="16"></line>"#,
};

pub const ALERT_TRIANGLE: IconType = IconType{ 
 content: r#"
<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>
<path d="M12 9v4"></path>
<path d="M12 17h.01"></path>"#,
};

pub const ALIGN_CENTER_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M2 12h20"></path>
<path d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4"></path>
<path d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4"></path>
<path d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1"></path>
<path d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1"></path>"#,
};

pub const ALIGN_CENTER_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M12 2v20"></path>
<path d="M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4"></path>
<path d="M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4"></path>
<path d="M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1"></path>
<path d="M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1"></path>"#,
};

pub const ALIGN_CENTER: IconType = IconType{ 
 content: r#"
<line x1="21" y2="6" y1="6" x2="3"></line>
<line x2="7" y1="12" x1="17" y2="12"></line>
<line x2="5" y1="18" y2="18" x1="19"></line>"#,
};

pub const ALIGN_END_HORIZONTAL: IconType = IconType{ 
 content: r#"
<rect y="2" x="4" width="6" rx="2" height="16"></rect>
<rect width="6" y="9" rx="2" height="9" x="14"></rect>
<path d="M22 22H2"></path>"#,
};

pub const ALIGN_END_VERTICAL: IconType = IconType{ 
 content: r#"
<rect y="4" rx="2" x="2" width="16" height="6"></rect>
<rect height="6" x="9" y="14" rx="2" width="9"></rect>
<path d="M22 22V2"></path>"#,
};

pub const ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: IconType = IconType{ 
 content: r#"
<rect width="6" y="5" height="14" rx="2" x="4"></rect>
<rect x="14" width="6" y="7" rx="2" height="10"></rect>
<path d="M17 22v-5"></path>
<path d="M17 7V2"></path>
<path d="M7 22v-3"></path>
<path d="M7 5V2"></path>"#,
};

pub const ALIGN_HORIZONTAL_DISTRIBUTE_END: IconType = IconType{ 
 content: r#"
<rect x="4" y="5" rx="2" width="6" height="14"></rect>
<rect width="6" y="7" height="10" rx="2" x="14"></rect>
<path d="M10 2v20"></path>
<path d="M20 2v20"></path>"#,
};

pub const ALIGN_HORIZONTAL_DISTRIBUTE_START: IconType = IconType{ 
 content: r#"
<rect y="5" rx="2" x="4" width="6" height="14"></rect>
<rect x="14" rx="2" width="6" y="7" height="10"></rect>
<path d="M4 2v20"></path>
<path d="M14 2v20"></path>"#,
};

pub const ALIGN_HORIZONTAL_JUSTIFY_CENTER: IconType = IconType{ 
 content: r#"
<rect height="14" y="5" rx="2" x="2" width="6"></rect>
<rect rx="2" height="10" width="6" x="16" y="7"></rect>
<path d="M12 2v20"></path>"#,
};

pub const ALIGN_HORIZONTAL_JUSTIFY_END: IconType = IconType{ 
 content: r#"
<rect height="14" x="2" y="5" width="6" rx="2"></rect>
<rect width="6" height="10" rx="2" y="7" x="12"></rect>
<path d="M22 2v20"></path>"#,
};

pub const ALIGN_HORIZONTAL_JUSTIFY_START: IconType = IconType{ 
 content: r#"
<rect x="6" width="6" y="5" height="14" rx="2"></rect>
<rect y="7" width="6" rx="2" height="10" x="16"></rect>
<path d="M2 2v20"></path>"#,
};

pub const ALIGN_HORIZONTAL_SPACE_AROUND: IconType = IconType{ 
 content: r#"
<rect height="10" y="7" width="6" x="9" rx="2"></rect>
<path d="M4 22V2"></path>
<path d="M20 22V2"></path>"#,
};

pub const ALIGN_HORIZONTAL_SPACE_BETWEEN: IconType = IconType{ 
 content: r#"
<rect width="6" y="5" height="14" x="3" rx="2"></rect>
<rect y="7" rx="2" height="10" x="15" width="6"></rect>
<path d="M3 2v20"></path>
<path d="M21 2v20"></path>"#,
};

pub const ALIGN_JUSTIFY: IconType = IconType{ 
 content: r#"
<line x1="3" x2="21" y2="6" y1="6"></line>
<line x2="21" y2="12" y1="12" x1="3"></line>
<line y1="18" x1="3" y2="18" x2="21"></line>"#,
};

pub const ALIGN_LEFT: IconType = IconType{ 
 content: r#"
<line x1="21" y2="6" x2="3" y1="6"></line>
<line x2="3" y1="12" y2="12" x1="15"></line>
<line x1="17" x2="3" y1="18" y2="18"></line>"#,
};

pub const ALIGN_RIGHT: IconType = IconType{ 
 content: r#"
<line x1="21" x2="3" y1="6" y2="6"></line>
<line y1="12" x2="9" y2="12" x1="21"></line>
<line x1="21" x2="7" y1="18" y2="18"></line>"#,
};

pub const ALIGN_START_HORIZONTAL: IconType = IconType{ 
 content: r#"
<rect height="16" width="6" x="4" rx="2" y="6"></rect>
<rect width="6" height="9" y="6" x="14" rx="2"></rect>
<path d="M22 2H2"></path>"#,
};

pub const ALIGN_START_VERTICAL: IconType = IconType{ 
 content: r#"
<rect y="14" x="6" height="6" rx="2" width="9"></rect>
<rect height="6" y="4" width="16" rx="2" x="6"></rect>
<path d="M2 2v20"></path>"#,
};

pub const ALIGN_VERTICAL_DISTRIBUTE_CENTER: IconType = IconType{ 
 content: r#"
<rect y="14" height="6" x="5" rx="2" width="14"></rect>
<rect width="10" y="4" rx="2" x="7" height="6"></rect>
<path d="M22 7h-5"></path>
<path d="M7 7H1"></path>
<path d="M22 17h-3"></path>
<path d="M5 17H2"></path>"#,
};

pub const ALIGN_VERTICAL_DISTRIBUTE_END: IconType = IconType{ 
 content: r#"
<rect width="14" height="6" rx="2" x="5" y="14"></rect>
<rect y="4" width="10" height="6" rx="2" x="7"></rect>
<path d="M2 20h20"></path>
<path d="M2 10h20"></path>"#,
};

pub const ALIGN_VERTICAL_DISTRIBUTE_START: IconType = IconType{ 
 content: r#"
<rect x="5" y="14" rx="2" width="14" height="6"></rect>
<rect height="6" width="10" y="4" rx="2" x="7"></rect>
<path d="M2 14h20"></path>
<path d="M2 4h20"></path>"#,
};

pub const ALIGN_VERTICAL_JUSTIFY_CENTER: IconType = IconType{ 
 content: r#"
<rect x="5" y="16" height="6" rx="2" width="14"></rect>
<rect width="10" x="7" y="2" height="6" rx="2"></rect>
<path d="M2 12h20"></path>"#,
};

pub const ALIGN_VERTICAL_JUSTIFY_END: IconType = IconType{ 
 content: r#"
<rect width="14" rx="2" x="5" y="12" height="6"></rect>
<rect x="7" y="2" height="6" width="10" rx="2"></rect>
<path d="M2 22h20"></path>"#,
};

pub const ALIGN_VERTICAL_JUSTIFY_START: IconType = IconType{ 
 content: r#"
<rect width="14" height="6" y="16" x="5" rx="2"></rect>
<rect x="7" width="10" y="6" rx="2" height="6"></rect>
<path d="M2 2h20"></path>"#,
};

pub const ALIGN_VERTICAL_SPACE_AROUND: IconType = IconType{ 
 content: r#"
<rect x="7" rx="2" y="9" width="10" height="6"></rect>
<path d="M22 20H2"></path>
<path d="M22 4H2"></path>"#,
};

pub const ALIGN_VERTICAL_SPACE_BETWEEN: IconType = IconType{ 
 content: r#"
<rect height="6" y="15" rx="2" x="5" width="14"></rect>
<rect height="6" x="7" rx="2" width="10" y="3"></rect>
<path d="M2 21h20"></path>
<path d="M2 3h20"></path>"#,
};

pub const AMPERSAND: IconType = IconType{ 
 content: r#"
<path d="M17.5 12c0 4.4-3.6 8-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13"></path>
<path d="M16 12h3"></path>"#,
};

pub const AMPERSANDS: IconType = IconType{ 
 content: r#"
<path d="M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"></path>
<path d="M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"></path>"#,
};

pub const ANCHOR: IconType = IconType{ 
 content: r#"
<circle cy="5" cx="12" r="3"></circle>
<line x1="12" y2="8" x2="12" y1="22"></line>
<path d="M5 12H2a10 10 0 0 0 20 0h-3"></path>"#,
};

pub const ANGRY: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M16 16s-1.5-2-4-2-4 2-4 2"></path>
<path d="M7.5 8 10 9"></path>
<path d="m14 9 2.5-1"></path>
<path d="M9 10h0"></path>
<path d="M15 10h0"></path>"#,
};

pub const ANNOYED: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<path d="M8 15h8"></path>
<path d="M8 9h2"></path>
<path d="M14 9h2"></path>"#,
};

pub const ANTENNA: IconType = IconType{ 
 content: r#"
<path d="M2 12 7 2"></path>
<path d="m7 12 5-10"></path>
<path d="m12 12 5-10"></path>
<path d="m17 12 5-10"></path>
<path d="M4.5 7h15"></path>
<path d="M12 16v6"></path>"#,
};

pub const APERTURE: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<line x2="20.05" y1="8" x1="14.31" y2="17.94"></line>
<line y1="8" x2="21.17" x1="9.69" y2="8"></line>
<line x2="13.12" x1="7.38" y1="12" y2="2.06"></line>
<line y2="6.06" x1="9.69" y1="16" x2="3.95"></line>
<line y2="16" y1="16" x1="14.31" x2="2.83"></line>
<line x1="16.62" x2="10.88" y1="12" y2="21.94"></line>"#,
};

pub const APP_WINDOW: IconType = IconType{ 
 content: r#"
<rect y="4" width="20" rx="2" height="16" x="2"></rect>
<path d="M10 4v4"></path>
<path d="M2 8h20"></path>
<path d="M6 4v4"></path>"#,
};

pub const APPLE: IconType = IconType{ 
 content: r#"
<path d="M12 20.94c1.5 0 2.75 1.06 4 1.06 3 0 6-8 6-12.22A4.91 4.91 0 0 0 17 5c-2.22 0-4 1.44-5 2-1-.56-2.78-2-5-2a4.9 4.9 0 0 0-5 4.78C2 14 5 22 8 22c1.25 0 2.5-1.06 4-1.06Z"></path>
<path d="M10 2c1 .5 2 2 2 5"></path>"#,
};

pub const ARCHIVE_RESTORE: IconType = IconType{ 
 content: r#"
<rect x="2" width="20" y="3" height="5" rx="1"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h2"></path>
<path d="M20 8v11a2 2 0 0 1-2 2h-2"></path>
<path d="m9 15 3-3 3 3"></path>
<path d="M12 12v9"></path>"#,
};

pub const ARCHIVE_X: IconType = IconType{ 
 content: r#"
<rect x="2" height="5" y="3" rx="1" width="20"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
<path d="m9.5 17 5-5"></path>
<path d="m9.5 12 5 5"></path>"#,
};

pub const ARCHIVE: IconType = IconType{ 
 content: r#"
<rect width="20" y="3" rx="1" x="2" height="5"></rect>
<path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>
<path d="M10 12h4"></path>"#,
};

pub const AREA_CHART: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<path d="M7 12v5h12V8l-5 5-4-4Z"></path>"#,
};

pub const ARMCHAIR: IconType = IconType{ 
 content: r#"
<path d="M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3"></path>
<path d="M3 11v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H7v-2a2 2 0 0 0-4 0Z"></path>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#,
};

pub const ARROW_BIG_DOWN_DASH: IconType = IconType{ 
 content: r#"
<path d="M15 5H9"></path>
<path d="M15 9v3h4l-7 7-7-7h4V9h6z"></path>"#,
};

pub const ARROW_BIG_DOWN: IconType = IconType{ 
 content: r#"
<path d="M15 6v6h4l-7 7-7-7h4V6h6z"></path>"#,
};

pub const ARROW_BIG_LEFT_DASH: IconType = IconType{ 
 content: r#"
<path d="M19 15V9"></path>
<path d="M15 15h-3v4l-7-7 7-7v4h3v6z"></path>"#,
};

pub const ARROW_BIG_LEFT: IconType = IconType{ 
 content: r#"
<path d="M18 15h-6v4l-7-7 7-7v4h6v6z"></path>"#,
};

pub const ARROW_BIG_RIGHT_DASH: IconType = IconType{ 
 content: r#"
<path d="M5 9v6"></path>
<path d="M9 9h3V5l7 7-7 7v-4H9V9z"></path>"#,
};

pub const ARROW_BIG_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M6 9h6V5l7 7-7 7v-4H6V9z"></path>"#,
};

pub const ARROW_BIG_UP_DASH: IconType = IconType{ 
 content: r#"
<path d="M9 19h6"></path>
<path d="M9 15v-3H5l7-7 7 7h-4v3H9z"></path>"#,
};

pub const ARROW_BIG_UP: IconType = IconType{ 
 content: r#"
<path d="M9 18v-6H5l7-7 7 7h-4v6H9z"></path>"#,
};

pub const ARROW_DOWN_01: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<rect x="15" ry="2" width="4" height="6" y="4"></rect>
<path d="M17 20v-6h-2"></path>
<path d="M15 20h4"></path>"#,
};

pub const ARROW_DOWN_10: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M17 10V4h-2"></path>
<path d="M15 10h4"></path>
<rect x="15" width="4" y="14" ry="2" height="6"></rect>"#,
};

pub const ARROW_DOWN_AZ: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M20 8h-5"></path>
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"></path>
<path d="M15 14h5l-5 6h5"></path>"#,
};

pub const ARROW_DOWN_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M12 8v8"></path>
<path d="m8 12 4 4 4-4"></path>"#,
};

pub const ARROW_DOWN_FROM_LINE: IconType = IconType{ 
 content: r#"
<path d="M19 3H5"></path>
<path d="M12 21V7"></path>
<path d="m6 15 6 6 6-6"></path>"#,
};

pub const ARROW_DOWN_LEFT_FROM_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M2 12a10 10 0 1 1 10 10"></path>
<path d="m2 22 10-10"></path>
<path d="M8 22H2v-6"></path>"#,
};

pub const ARROW_DOWN_LEFT_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" width="18" y="3" height="18" x="3"></rect>
<path d="m16 8-8 8"></path>
<path d="M16 16H8V8"></path>"#,
};

pub const ARROW_DOWN_LEFT: IconType = IconType{ 
 content: r#"
<path d="M17 7 7 17"></path>
<path d="M17 17H7V7"></path>"#,
};

pub const ARROW_DOWN_NARROW_WIDE: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M11 4h4"></path>
<path d="M11 8h7"></path>
<path d="M11 12h10"></path>"#,
};

pub const ARROW_DOWN_RIGHT_FROM_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M12 22a10 10 0 1 1 10-10"></path>
<path d="M22 22 12 12"></path>
<path d="M22 16v6h-6"></path>"#,
};

pub const ARROW_DOWN_RIGHT_SQUARE: IconType = IconType{ 
 content: r#"
<rect y="3" height="18" rx="2" width="18" x="3"></rect>
<path d="m8 8 8 8"></path>
<path d="M16 8v8H8"></path>"#,
};

pub const ARROW_DOWN_RIGHT: IconType = IconType{ 
 content: r#"
<path d="m7 7 10 10"></path>
<path d="M17 7v10H7"></path>"#,
};

pub const ARROW_DOWN_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" y="3" height="18" rx="2" width="18"></rect>
<path d="M12 8v8"></path>
<path d="m8 12 4 4 4-4"></path>"#,
};

pub const ARROW_DOWN_TO_DOT: IconType = IconType{ 
 content: r#"
<path d="M12 2v14"></path>
<path d="m19 9-7 7-7-7"></path>
<circle cx="12" cy="21" r="1"></circle>"#,
};

pub const ARROW_DOWN_TO_LINE: IconType = IconType{ 
 content: r#"
<path d="M12 17V3"></path>
<path d="m6 11 6 6 6-6"></path>
<path d="M19 21H5"></path>"#,
};

pub const ARROW_DOWN_UP: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="m21 8-4-4-4 4"></path>
<path d="M17 4v16"></path>"#,
};

pub const ARROW_DOWN_WIDE_NARROW: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 20V4"></path>
<path d="M11 4h10"></path>
<path d="M11 8h7"></path>
<path d="M11 12h4"></path>"#,
};

pub const ARROW_DOWN_ZA: IconType = IconType{ 
 content: r#"
<path d="m3 16 4 4 4-4"></path>
<path d="M7 4v16"></path>
<path d="M15 4h5l-5 6h5"></path>
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"></path>
<path d="M20 18h-5"></path>"#,
};

pub const ARROW_DOWN: IconType = IconType{ 
 content: r#"
<path d="M12 5v14"></path>
<path d="m19 12-7 7-7-7"></path>"#,
};

pub const ARROW_LEFT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M16 12H8"></path>
<path d="m12 8-4 4 4 4"></path>"#,
};

pub const ARROW_LEFT_FROM_LINE: IconType = IconType{ 
 content: r#"
<path d="m9 6-6 6 6 6"></path>
<path d="M3 12h14"></path>
<path d="M21 19V5"></path>"#,
};

pub const ARROW_LEFT_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M8 3 4 7l4 4"></path>
<path d="M4 7h16"></path>
<path d="m16 21 4-4-4-4"></path>
<path d="M20 17H4"></path>"#,
};

pub const ARROW_LEFT_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" y="3" x="3" rx="2" width="18"></rect>
<path d="m12 8-4 4 4 4"></path>
<path d="M16 12H8"></path>"#,
};

pub const ARROW_LEFT_TO_LINE: IconType = IconType{ 
 content: r#"
<path d="M3 19V5"></path>
<path d="m13 6-6 6 6 6"></path>
<path d="M7 12h14"></path>"#,
};

pub const ARROW_LEFT: IconType = IconType{ 
 content: r#"
<path d="m12 19-7-7 7-7"></path>
<path d="M19 12H5"></path>"#,
};

pub const ARROW_RIGHT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M8 12h8"></path>
<path d="m12 16 4-4-4-4"></path>"#,
};

pub const ARROW_RIGHT_FROM_LINE: IconType = IconType{ 
 content: r#"
<path d="M3 5v14"></path>
<path d="M21 12H7"></path>
<path d="m15 18 6-6-6-6"></path>"#,
};

pub const ARROW_RIGHT_LEFT: IconType = IconType{ 
 content: r#"
<path d="m16 3 4 4-4 4"></path>
<path d="M20 7H4"></path>
<path d="m8 21-4-4 4-4"></path>
<path d="M4 17h16"></path>"#,
};

pub const ARROW_RIGHT_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" y="3" rx="2" width="18" height="18"></rect>
<path d="M8 12h8"></path>
<path d="m12 16 4-4-4-4"></path>"#,
};

pub const ARROW_RIGHT_TO_LINE: IconType = IconType{ 
 content: r#"
<path d="M17 12H3"></path>
<path d="m11 18 6-6-6-6"></path>
<path d="M21 5v14"></path>"#,
};

pub const ARROW_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M5 12h14"></path>
<path d="m12 5 7 7-7 7"></path>"#,
};

pub const ARROW_UP_01: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<rect x="15" width="4" y="4" height="6" ry="2"></rect>
<path d="M17 20v-6h-2"></path>
<path d="M15 20h4"></path>"#,
};

pub const ARROW_UP_10: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M17 10V4h-2"></path>
<path d="M15 10h4"></path>
<rect y="14" width="4" height="6" ry="2" x="15"></rect>"#,
};

pub const ARROW_UP_AZ: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M20 8h-5"></path>
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"></path>
<path d="M15 14h5l-5 6h5"></path>"#,
};

pub const ARROW_UP_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="m16 12-4-4-4 4"></path>
<path d="M12 16V8"></path>"#,
};

pub const ARROW_UP_DOWN: IconType = IconType{ 
 content: r#"
<path d="m21 16-4 4-4-4"></path>
<path d="M17 20V4"></path>
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>"#,
};

pub const ARROW_UP_FROM_DOT: IconType = IconType{ 
 content: r#"
<path d="m5 9 7-7 7 7"></path>
<path d="M12 16V2"></path>
<circle cx="12" cy="21" r="1"></circle>"#,
};

pub const ARROW_UP_FROM_LINE: IconType = IconType{ 
 content: r#"
<path d="m18 9-6-6-6 6"></path>
<path d="M12 3v14"></path>
<path d="M5 21h14"></path>"#,
};

pub const ARROW_UP_LEFT_FROM_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M2 8V2h6"></path>
<path d="m2 2 10 10"></path>
<path d="M12 2A10 10 0 1 1 2 12"></path>"#,
};

pub const ARROW_UP_LEFT_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" x="3" width="18" height="18" y="3"></rect>
<path d="M8 16V8h8"></path>
<path d="M16 16 8 8"></path>"#,
};

pub const ARROW_UP_LEFT: IconType = IconType{ 
 content: r#"
<path d="M7 17V7h10"></path>
<path d="M17 17 7 7"></path>"#,
};

pub const ARROW_UP_NARROW_WIDE: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M11 12h4"></path>
<path d="M11 16h7"></path>
<path d="M11 20h10"></path>"#,
};

pub const ARROW_UP_RIGHT_FROM_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M22 12A10 10 0 1 1 12 2"></path>
<path d="M22 2 12 12"></path>
<path d="M16 2h6v6"></path>"#,
};

pub const ARROW_UP_RIGHT_SQUARE: IconType = IconType{ 
 content: r#"
<rect width="18" x="3" y="3" rx="2" height="18"></rect>
<path d="M8 8h8v8"></path>
<path d="m8 16 8-8"></path>"#,
};

pub const ARROW_UP_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M7 7h10v10"></path>
<path d="M7 17 17 7"></path>"#,
};

pub const ARROW_UP_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" height="18" y="3" x="3" width="18"></rect>
<path d="m16 12-4-4-4 4"></path>
<path d="M12 16V8"></path>"#,
};

pub const ARROW_UP_TO_LINE: IconType = IconType{ 
 content: r#"
<path d="M5 3h14"></path>
<path d="m18 13-6-6-6 6"></path>
<path d="M12 7v14"></path>"#,
};

pub const ARROW_UP_WIDE_NARROW: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M11 12h10"></path>
<path d="M11 16h7"></path>
<path d="M11 20h4"></path>"#,
};

pub const ARROW_UP_ZA: IconType = IconType{ 
 content: r#"
<path d="m3 8 4-4 4 4"></path>
<path d="M7 4v16"></path>
<path d="M15 4h5l-5 6h5"></path>
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"></path>
<path d="M20 18h-5"></path>"#,
};

pub const ARROW_UP: IconType = IconType{ 
 content: r#"
<path d="m5 12 7-7 7 7"></path>
<path d="M12 19V5"></path>"#,
};

pub const ARROWS_UP_FROM_LINE: IconType = IconType{ 
 content: r#"
<path d="m4 6 3-3 3 3"></path>
<path d="M7 17V3"></path>
<path d="m14 6 3-3 3 3"></path>
<path d="M17 17V3"></path>
<path d="M4 21h16"></path>"#,
};

pub const ASTERISK: IconType = IconType{ 
 content: r#"
<path d="M12 6v12"></path>
<path d="M17.196 9 6.804 15"></path>
<path d="m6.804 9 10.392 6"></path>"#,
};

pub const AT_SIGN: IconType = IconType{ 
 content: r#"
<circle r="4" cx="12" cy="12"></circle>
<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8"></path>"#,
};

pub const ATOM: IconType = IconType{ 
 content: r#"
<circle cy="12" r="1" cx="12"></circle>
<path d="M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z"></path>
<path d="M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z"></path>"#,
};

pub const AWARD: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="8" r="6"></circle>
<path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11"></path>"#,
};

pub const AXE: IconType = IconType{ 
 content: r#"
<path d="m14 12-8.5 8.5a2.12 2.12 0 1 1-3-3L11 9"></path>
<path d="M15 13 9 7l4-4 6 6h3a8 8 0 0 1-7 7z"></path>"#,
};

pub const AXIS_3_D: IconType = IconType{ 
 content: r#"
<path d="M4 4v16h16"></path>
<path d="m4 20 7-7"></path>"#,
};

pub const BABY: IconType = IconType{ 
 content: r#"
<path d="M9 12h.01"></path>
<path d="M15 12h.01"></path>
<path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5"></path>
<path d="M19 6.3a9 9 0 0 1 1.8 3.9 2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1"></path>"#,
};

pub const BACKPACK: IconType = IconType{ 
 content: r#"
<path d="M4 20V10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2Z"></path>
<path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"></path>
<path d="M8 21v-5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v5"></path>
<path d="M8 10h8"></path>
<path d="M8 18h8"></path>"#,
};

pub const BADGE_ALERT: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line y1="8" x1="12" x2="12" y2="12"></line>
<line x1="12" y2="16" x2="12.01" y1="16"></line>"#,
};

pub const BADGE_CENT: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M12 7v10"></path>
<path d="M15.4 10a4 4 0 1 0 0 4"></path>"#,
};

pub const BADGE_CHECK: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m9 12 2 2 4-4"></path>"#,
};

pub const BADGE_DOLLAR_SIGN: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 18V6"></path>"#,
};

pub const BADGE_EURO: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M7 12h5"></path>
<path d="M15 9.4a4 4 0 1 0 0 5.2"></path>"#,
};

pub const BADGE_HELP: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
<line x1="12" y2="17" y1="17" x2="12.01"></line>"#,
};

pub const BADGE_INDIAN_RUPEE: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M8 8h8"></path>
<path d="M8 12h8"></path>
<path d="m13 17-5-1h1a4 4 0 0 0 0-8"></path>"#,
};

pub const BADGE_INFO: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x1="12" y1="16" x2="12" y2="12"></line>
<line x1="12" x2="12.01" y1="8" y2="8"></line>"#,
};

pub const BADGE_JAPANESE_YEN: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m9 8 3 3v7"></path>
<path d="m12 11 3-3"></path>
<path d="M9 12h6"></path>
<path d="M9 16h6"></path>"#,
};

pub const BADGE_MINUS: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x1="8" y1="12" y2="12" x2="16"></line>"#,
};

pub const BADGE_PERCENT: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#,
};

pub const BADGE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x2="12" y2="16" x1="12" y1="8"></line>
<line y1="12" x1="8" y2="12" x2="16"></line>"#,
};

pub const BADGE_POUND_STERLING: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M8 12h4"></path>
<path d="M10 16V9.5a2.5 2.5 0 0 1 5 0"></path>
<path d="M8 16h7"></path>"#,
};

pub const BADGE_RUSSIAN_RUBLE: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M9 16h5"></path>
<path d="M9 12h5a2 2 0 1 0 0-4h-3v9"></path>"#,
};

pub const BADGE_SWISS_FRANC: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<path d="M11 17V8h4"></path>
<path d="M11 12h3"></path>
<path d="M9 16h4"></path>"#,
};

pub const BADGE_X: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>
<line x2="9" x1="15" y1="9" y2="15"></line>
<line x1="9" x2="15" y2="15" y1="9"></line>"#,
};

pub const BADGE: IconType = IconType{ 
 content: r#"
<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"></path>"#,
};

pub const BAGGAGE_CLAIM: IconType = IconType{ 
 content: r#"
<path d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2"></path>
<path d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10"></path>
<rect y="6" x="8" height="8" rx="1" width="13"></rect>
<circle cx="18" cy="20" r="2"></circle>
<circle r="2" cy="20" cx="9"></circle>"#,
};

pub const BAN: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="m4.9 4.9 14.2 14.2"></path>"#,
};

pub const BANANA: IconType = IconType{ 
 content: r#"
<path d="M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5"></path>
<path d="M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z"></path>"#,
};

pub const BANKNOTE: IconType = IconType{ 
 content: r#"
<rect rx="2" x="2" height="12" width="20" y="6"></rect>
<circle cy="12" r="2" cx="12"></circle>
<path d="M6 12h.01M18 12h.01"></path>"#,
};

pub const BAR_CHART_2: IconType = IconType{ 
 content: r#"
<line x1="18" y2="10" y1="20" x2="18"></line>
<line x2="12" y1="20" x1="12" y2="4"></line>
<line x2="6" y1="20" x1="6" y2="14"></line>"#,
};

pub const BAR_CHART_3: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<path d="M18 17V9"></path>
<path d="M13 17V5"></path>
<path d="M8 17v-3"></path>"#,
};

pub const BAR_CHART_4: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<path d="M13 17V9"></path>
<path d="M18 17V5"></path>
<path d="M8 17v-3"></path>"#,
};

pub const BAR_CHART_BIG: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<rect y="10" x="7" rx="1" height="7" width="4"></rect>
<rect rx="1" height="12" x="15" width="4" y="5"></rect>"#,
};

pub const BAR_CHART_HORIZONTAL_BIG: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<rect y="5" width="12" rx="1" x="7" height="4"></rect>
<rect height="4" y="13" rx="1" x="7" width="7"></rect>"#,
};

pub const BAR_CHART_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<path d="M7 16h8"></path>
<path d="M7 11h12"></path>
<path d="M7 6h3"></path>"#,
};

pub const BAR_CHART: IconType = IconType{ 
 content: r#"
<line x2="12" y1="20" y2="10" x1="12"></line>
<line x1="18" y1="20" y2="4" x2="18"></line>
<line y1="20" x1="6" y2="16" x2="6"></line>"#,
};

pub const BASELINE: IconType = IconType{ 
 content: r#"
<path d="M4 20h16"></path>
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>"#,
};

pub const BATH: IconType = IconType{ 
 content: r#"
<path d="M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5"></path>
<line x2="8" y2="7" x1="10" y1="5"></line>
<line x1="2" y1="12" y2="12" x2="22"></line>
<line y2="21" x1="7" y1="19" x2="7"></line>
<line x2="17" y1="19" x1="17" y2="21"></line>"#,
};

pub const BATTERY_CHARGING: IconType = IconType{ 
 content: r#"
<path d="M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2"></path>
<path d="M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1"></path>
<path d="m11 7-3 5h4l-3 5"></path>
<line y1="11" x1="22" y2="13" x2="22"></line>"#,
};

pub const BATTERY_FULL: IconType = IconType{ 
 content: r#"
<rect width="16" x="2" y="7" rx="2" ry="2" height="10"></rect>
<line x1="22" y2="13" y1="11" x2="22"></line>
<line x1="6" y2="13" x2="6" y1="11"></line>
<line y1="11" y2="13" x2="10" x1="10"></line>
<line y2="13" x2="14" y1="11" x1="14"></line>"#,
};

pub const BATTERY_LOW: IconType = IconType{ 
 content: r#"
<rect x="2" height="10" y="7" rx="2" ry="2" width="16"></rect>
<line y1="11" x1="22" x2="22" y2="13"></line>
<line x1="6" y2="13" x2="6" y1="11"></line>"#,
};

pub const BATTERY_MEDIUM: IconType = IconType{ 
 content: r#"
<rect ry="2" height="10" x="2" y="7" rx="2" width="16"></rect>
<line x2="22" x1="22" y2="13" y1="11"></line>
<line y1="11" y2="13" x2="6" x1="6"></line>
<line y1="11" x2="10" x1="10" y2="13"></line>"#,
};

pub const BATTERY_WARNING: IconType = IconType{ 
 content: r#"
<path d="M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2"></path>
<path d="M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2"></path>
<line x1="22" y1="11" y2="13" x2="22"></line>
<line y2="13" x1="10" y1="7" x2="10"></line>
<line y1="17" x1="10" x2="10" y2="17.01"></line>"#,
};

pub const BATTERY: IconType = IconType{ 
 content: r#"
<rect y="7" width="16" height="10" x="2" rx="2" ry="2"></rect>
<line y2="13" y1="11" x1="22" x2="22"></line>"#,
};

pub const BEAKER: IconType = IconType{ 
 content: r#"
<path d="M4.5 3h15"></path>
<path d="M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3"></path>
<path d="M6 14h12"></path>"#,
};

pub const BEAN_OFF: IconType = IconType{ 
 content: r#"
<path d="M9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1"></path>
<path d="M10.75 5.093A6 6 0 0 1 22 8c0 2.411-.61 4.68-1.683 6.66"></path>
<path d="M5.341 10.62a4 4 0 0 0 6.487 1.208M10.62 5.341a4.015 4.015 0 0 1 2.039 2.04"></path>
<line y1="2" x2="22" y2="22" x1="2"></line>"#,
};

pub const BEAN: IconType = IconType{ 
 content: r#"
<path d="M10.165 6.598C9.954 7.478 9.64 8.36 9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22c7.732 0 14-6.268 14-14a6 6 0 0 0-11.835-1.402Z"></path>
<path d="M5.341 10.62a4 4 0 1 0 5.279-5.28"></path>"#,
};

pub const BED_DOUBLE: IconType = IconType{ 
 content: r#"
<path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8"></path>
<path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"></path>
<path d="M12 4v6"></path>
<path d="M2 18h20"></path>"#,
};

pub const BED_SINGLE: IconType = IconType{ 
 content: r#"
<path d="M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8"></path>
<path d="M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4"></path>
<path d="M3 18h18"></path>"#,
};

pub const BED: IconType = IconType{ 
 content: r#"
<path d="M2 4v16"></path>
<path d="M2 8h18a2 2 0 0 1 2 2v10"></path>
<path d="M2 17h20"></path>
<path d="M6 8v9"></path>"#,
};

pub const BEEF: IconType = IconType{ 
 content: r#"
<circle cy="8.5" r="2.5" cx="12.5"></circle>
<path d="M12.5 2a6.5 6.5 0 0 0-6.22 4.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3A6.5 6.5 0 0 0 12.5 2Z"></path>
<path d="m18.5 6 2.19 4.5a6.48 6.48 0 0 1 .31 2 6.49 6.49 0 0 1-2.6 5.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5"></path>"#,
};

pub const BEER: IconType = IconType{ 
 content: r#"
<path d="M17 11h1a3 3 0 0 1 0 6h-1"></path>
<path d="M9 12v6"></path>
<path d="M13 12v6"></path>
<path d="M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z"></path>
<path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8"></path>"#,
};

pub const BELL_DOT: IconType = IconType{ 
 content: r#"
<path d="M19.4 14.9C20.2 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 .7 0 1.3.1 1.9.3"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<circle cx="18" cy="8" r="3"></circle>"#,
};

pub const BELL_MINUS: IconType = IconType{ 
 content: r#"
<path d="M18.4 12c.8 3.8 2.6 5 2.6 5H3s3-2 3-9c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M15 8h6"></path>"#,
};

pub const BELL_OFF: IconType = IconType{ 
 content: r#"
<path d="M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5"></path>
<path d="M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="m2 2 20 20"></path>"#,
};

pub const BELL_PLUS: IconType = IconType{ 
 content: r#"
<path d="M19.3 14.8C20.1 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 1 0 1.9.2 2.8.7"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M15 8h6"></path>
<path d="M18 5v6"></path>"#,
};

pub const BELL_RING: IconType = IconType{ 
 content: r#"
<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
<path d="M4 2C2.8 3.7 2 5.7 2 8"></path>
<path d="M22 8c0-2.3-.8-4.3-2-6"></path>"#,
};

pub const BELL: IconType = IconType{ 
 content: r#"
<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path>
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>"#,
};

pub const BIKE: IconType = IconType{ 
 content: r#"
<circle r="3.5" cy="17.5" cx="18.5"></circle>
<circle cy="17.5" cx="5.5" r="3.5"></circle>
<circle r="1" cx="15" cy="5"></circle>
<path d="M12 17.5V14l-3-3 4-3 2 3h2"></path>"#,
};

pub const BINARY: IconType = IconType{ 
 content: r#"
<rect width="4" x="14" height="6" rx="2" y="14"></rect>
<rect x="6" width="4" rx="2" y="4" height="6"></rect>
<path d="M6 20h4"></path>
<path d="M14 10h4"></path>
<path d="M6 14h2v6"></path>
<path d="M14 4h2v6"></path>"#,
};

pub const BIOHAZARD: IconType = IconType{ 
 content: r#"
<circle cy="11.9" r="2" cx="12"></circle>
<path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6"></path>
<path d="m8.9 10.1 1.4.8"></path>
<path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5"></path>
<path d="m15.1 10.1-1.4.8"></path>
<path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2"></path>
<path d="M12 13.9v1.6"></path>
<path d="M13.5 5.4c-1-.2-2-.2-3 0"></path>
<path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5"></path>
<path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5"></path>"#,
};

pub const BIRD: IconType = IconType{ 
 content: r#"
<path d="M16 7h.01"></path>
<path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20"></path>
<path d="m20 7 2 .5-2 .5"></path>
<path d="M10 18v3"></path>
<path d="M14 17.75V21"></path>
<path d="M7 18a6 6 0 0 0 3.84-10.61"></path>"#,
};

pub const BITCOIN: IconType = IconType{ 
 content: r#"
<path d="M11.767 19.089c4.924.868 6.14-6.025 1.216-6.894m-1.216 6.894L5.86 18.047m5.908 1.042-.347 1.97m1.563-8.864c4.924.869 6.14-6.025 1.215-6.893m-1.215 6.893-3.94-.694m5.155-6.2L8.29 4.26m5.908 1.042.348-1.97M7.48 20.364l3.126-17.727"></path>"#,
};

pub const BLINDS: IconType = IconType{ 
 content: r#"
<path d="M3 3h18"></path>
<path d="M20 7H8"></path>
<path d="M20 11H8"></path>
<path d="M10 19h10"></path>
<path d="M8 15h12"></path>
<path d="M4 3v14"></path>
<circle r="2" cx="4" cy="19"></circle>"#,
};

pub const BLOCKS: IconType = IconType{ 
 content: r#"
<rect x="14" width="7" rx="1" y="3" height="7"></rect>
<path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3"></path>"#,
};

pub const BLUETOOTH_CONNECTED: IconType = IconType{ 
 content: r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>
<line x1="18" y1="12" x2="21" y2="12"></line>
<line x2="6" x1="3" y2="12" y1="12"></line>"#,
};

pub const BLUETOOTH_OFF: IconType = IconType{ 
 content: r#"
<path d="m17 17-5 5V12l-5 5"></path>
<path d="m2 2 20 20"></path>
<path d="M14.5 9.5 17 7l-5-5v4.5"></path>"#,
};

pub const BLUETOOTH_SEARCHING: IconType = IconType{ 
 content: r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>
<path d="M20.83 14.83a4 4 0 0 0 0-5.66"></path>
<path d="M18 12h.01"></path>"#,
};

pub const BLUETOOTH: IconType = IconType{ 
 content: r#"
<path d="m7 7 10 10-5 5V2l5 5L7 17"></path>"#,
};

pub const BOLD: IconType = IconType{ 
 content: r#"
<path d="M14 12a4 4 0 0 0 0-8H6v8"></path>
<path d="M15 20a4 4 0 0 0 0-8H6v8Z"></path>"#,
};

pub const BOMB: IconType = IconType{ 
 content: r#"
<circle r="9" cx="11" cy="13"></circle>
<path d="m19.5 9.5 1.8-1.8a2.4 2.4 0 0 0 0-3.4l-1.6-1.6a2.41 2.41 0 0 0-3.4 0l-1.8 1.8"></path>
<path d="m22 2-1.5 1.5"></path>"#,
};

pub const BONE: IconType = IconType{ 
 content: r#"
<path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z"></path>"#,
};

pub const BOOK_COPY: IconType = IconType{ 
 content: r#"
<path d="M2 16V4a2 2 0 0 1 2-2h11"></path>
<path d="M5 14H4a2 2 0 1 0 0 4h1"></path>
<path d="M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12"></path>"#,
};

pub const BOOK_DOWN: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3 3 3-3"></path>"#,
};

pub const BOOK_KEY: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14"></path>
<path d="M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<circle cx="14" cy="8" r="2"></circle>
<path d="m20 2-4.5 4.5"></path>
<path d="m19 3 1 1"></path>"#,
};

pub const BOOK_LOCK: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10"></path>
<path d="M20 15v7H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<rect x="12" width="8" rx="1" height="5" y="6"></rect>
<path d="M18 6V4a2 2 0 1 0-4 0v2"></path>"#,
};

pub const BOOK_MARKED: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<polyline points="10 2 10 10 13 7 16 10 16 2"></polyline>"#,
};

pub const BOOK_MINUS: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M9 10h6"></path>"#,
};

pub const BOOK_OPEN_CHECK: IconType = IconType{ 
 content: r#"
<path d="M8 3H2v15h7c1.7 0 3 1.3 3 3V7c0-2.2-1.8-4-4-4Z"></path>
<path d="m16 12 2 2 4-4"></path>
<path d="M22 6V3h-6c-2.2 0-4 1.8-4 4v14c0-1.7 1.3-3 3-3h7v-2.3"></path>"#,
};

pub const BOOK_OPEN: IconType = IconType{ 
 content: r#"
<path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path>
<path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path>"#,
};

pub const BOOK_PLUS: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M9 10h6"></path>
<path d="M12 7v6"></path>"#,
};

pub const BOOK_TEMPLATE: IconType = IconType{ 
 content: r#"
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
<path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8"></path>"#,
};

pub const BOOK_UP_2: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2"></path>
<path d="M18 2h2v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3-3 3 3"></path>
<path d="m9 5 3-3 3 3"></path>"#,
};

pub const BOOK_UP: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="M12 13V7"></path>
<path d="m9 10 3-3 3 3"></path>"#,
};

pub const BOOK_X: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
<path d="m14.5 7-5 5"></path>
<path d="m9.5 7 5 5"></path>"#,
};

pub const BOOK: IconType = IconType{ 
 content: r#"
<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>"#,
};

pub const BOOKMARK_MINUS: IconType = IconType{ 
 content: r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>
<line y2="10" x1="15" y1="10" x2="9"></line>"#,
};

pub const BOOKMARK_PLUS: IconType = IconType{ 
 content: r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>
<line y2="13" y1="7" x2="12" x1="12"></line>
<line y2="10" y1="10" x2="9" x1="15"></line>"#,
};

pub const BOOKMARK: IconType = IconType{ 
 content: r#"
<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"></path>"#,
};

pub const BOOM_BOX: IconType = IconType{ 
 content: r#"
<path d="M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"></path>
<path d="M8 8v1"></path>
<path d="M12 8v1"></path>
<path d="M16 8v1"></path>
<rect rx="2" x="2" height="12" y="9" width="20"></rect>
<circle cx="8" cy="15" r="2"></circle>
<circle cy="15" cx="16" r="2"></circle>"#,
};

pub const BOT: IconType = IconType{ 
 content: r#"
<path d="M12 8V4H8"></path>
<rect y="8" height="12" x="4" width="16" rx="2"></rect>
<path d="M2 14h2"></path>
<path d="M20 14h2"></path>
<path d="M15 13v2"></path>
<path d="M9 13v2"></path>"#,
};

pub const BOX_SELECT: IconType = IconType{ 
 content: r#"
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
<path d="M21 14v1"></path>"#,
};

pub const BOX: IconType = IconType{ 
 content: r#"
<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"></path>
<path d="m3.3 7 8.7 5 8.7-5"></path>
<path d="M12 22V12"></path>"#,
};

pub const BOXES: IconType = IconType{ 
 content: r#"
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
<path d="M12 13.5V8"></path>"#,
};

pub const BRACES: IconType = IconType{ 
 content: r#"
<path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1"></path>
<path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1"></path>"#,
};

pub const BRACKETS: IconType = IconType{ 
 content: r#"
<path d="M16 3h3v18h-3"></path>
<path d="M8 21H5V3h3"></path>"#,
};

pub const BRAIN_CIRCUIT: IconType = IconType{ 
 content: r#"
<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08 2.5 2.5 0 0 0 4.91.05L12 20V4.5Z"></path>
<path d="M16 8V5c0-1.1.9-2 2-2"></path>
<path d="M12 13h4"></path>
<path d="M12 18h6a2 2 0 0 1 2 2v1"></path>
<path d="M12 8h8"></path>
<path d="M20.5 8a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M16.5 13a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M20.5 21a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>
<path d="M18.5 3a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z"></path>"#,
};

pub const BRAIN_COG: IconType = IconType{ 
 content: r#"
<circle cy="12" r="3" cx="12"></circle>
<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08A2.5 2.5 0 0 0 12 19.5a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 12 4.5"></path>
<path d="m15.7 10.4-.9.4"></path>
<path d="m9.2 13.2-.9.4"></path>
<path d="m13.6 15.7-.4-.9"></path>
<path d="m10.8 9.2-.4-.9"></path>
<path d="m15.7 13.5-.9-.4"></path>
<path d="m9.2 10.9-.9-.4"></path>
<path d="m10.5 15.7.4-.9"></path>
<path d="m13.1 9.2.4-.9"></path>"#,
};

pub const BRAIN: IconType = IconType{ 
 content: r#"
<path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z"></path>
<path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z"></path>"#,
};

pub const BRIEFCASE: IconType = IconType{ 
 content: r#"
<rect height="14" x="2" ry="2" width="20" rx="2" y="7"></rect>
<path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path>"#,
};

pub const BRING_TO_FRONT: IconType = IconType{ 
 content: r#"
<rect y="8" width="8" rx="2" height="8" x="8"></rect>
<path d="M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2"></path>
<path d="M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2"></path>"#,
};

pub const BRUSH: IconType = IconType{ 
 content: r#"
<path d="m9.06 11.9 8.07-8.06a2.85 2.85 0 1 1 4.03 4.03l-8.06 8.08"></path>
<path d="M7.07 14.94c-1.66 0-3 1.35-3 3.02 0 1.33-2.5 1.52-2 2.02 1.08 1.1 2.49 2.02 4 2.02 2.2 0 4-1.8 4-4.04a3.01 3.01 0 0 0-3-3.02z"></path>"#,
};

pub const BUG_OFF: IconType = IconType{ 
 content: r#"
<path d="M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2"></path>
<path d="M14.12 3.88 16 2"></path>
<path d="M22 13h-4v-2a4 4 0 0 0-4-4h-1.3"></path>
<path d="M20.97 5c0 2.1-1.6 3.8-3.5 4"></path>
<path d="m2 2 20 20"></path>
<path d="M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13"></path>
<path d="M12 20v-8"></path>
<path d="M6 13H2"></path>
<path d="M3 21c0-2.1 1.7-3.9 3.8-4"></path>"#,
};

pub const BUG_PLAY: IconType = IconType{ 
 content: r#"
<path d="m8 2 1.88 1.88"></path>
<path d="M14.12 3.88 16 2"></path>
<path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1"></path>
<path d="M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5"></path>
<path d="M6.53 9C4.6 8.8 3 7.1 3 5"></path>
<path d="M6 13H2"></path>
<path d="M3 21c0-2.1 1.7-3.9 3.8-4"></path>
<path d="M20.97 5c0 2.1-1.6 3.8-3.5 4"></path>
<path d="m12 12 8 5-8 5Z"></path>"#,
};

pub const BUG: IconType = IconType{ 
 content: r#"
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
<path d="M17.2 17c2.1.1 3.8 1.9 3.8 4"></path>"#,
};

pub const BUILDING_2: IconType = IconType{ 
 content: r#"
<path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"></path>
<path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"></path>
<path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"></path>
<path d="M10 6h4"></path>
<path d="M10 10h4"></path>
<path d="M10 14h4"></path>
<path d="M10 18h4"></path>"#,
};

pub const BUILDING: IconType = IconType{ 
 content: r#"
<rect width="16" y="2" rx="2" ry="2" height="20" x="4"></rect>
<path d="M9 22v-4h6v4"></path>
<path d="M8 6h.01"></path>
<path d="M16 6h.01"></path>
<path d="M12 6h.01"></path>
<path d="M12 10h.01"></path>
<path d="M12 14h.01"></path>
<path d="M16 10h.01"></path>
<path d="M16 14h.01"></path>
<path d="M8 10h.01"></path>
<path d="M8 14h.01"></path>"#,
};

pub const BUS_FRONT: IconType = IconType{ 
 content: r#"
<path d="M4 6 2 7"></path>
<path d="M10 6h4"></path>
<path d="m22 7-2-1"></path>
<rect rx="2" height="16" width="16" x="4" y="3"></rect>
<path d="M4 11h16"></path>
<path d="M8 15h.01"></path>
<path d="M16 15h.01"></path>
<path d="M6 19v2"></path>
<path d="M18 21v-2"></path>"#,
};

pub const BUS: IconType = IconType{ 
 content: r#"
<path d="M8 6v6"></path>
<path d="M15 6v6"></path>
<path d="M2 12h19.6"></path>
<path d="M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3"></path>
<circle r="2" cy="18" cx="7"></circle>
<path d="M9 18h5"></path>
<circle r="2" cy="18" cx="16"></circle>"#,
};

pub const CABLE_CAR: IconType = IconType{ 
 content: r#"
<path d="M10 3h.01"></path>
<path d="M14 2h.01"></path>
<path d="m2 9 20-5"></path>
<path d="M12 12V6.5"></path>
<rect height="10" width="16" rx="3" x="4" y="12"></rect>
<path d="M9 12v5"></path>
<path d="M15 12v5"></path>
<path d="M4 17h16"></path>"#,
};

pub const CABLE: IconType = IconType{ 
 content: r#"
<path d="M4 9a2 2 0 0 1-2-2V5h6v2a2 2 0 0 1-2 2Z"></path>
<path d="M3 5V3"></path>
<path d="M7 5V3"></path>
<path d="M19 15V6.5a3.5 3.5 0 0 0-7 0v11a3.5 3.5 0 0 1-7 0V9"></path>
<path d="M17 21v-2"></path>
<path d="M21 21v-2"></path>
<path d="M22 19h-6v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2Z"></path>"#,
};

pub const CAKE_SLICE: IconType = IconType{ 
 content: r#"
<circle cx="9" cy="7" r="2"></circle>
<path d="M7.2 7.9 3 11v9c0 .6.4 1 1 1h16c.6 0 1-.4 1-1v-9c0-2-3-6-7-8l-3.6 2.6"></path>
<path d="M16 13H3"></path>
<path d="M16 17H3"></path>"#,
};

pub const CAKE: IconType = IconType{ 
 content: r#"
<path d="M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8"></path>
<path d="M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1"></path>
<path d="M2 21h20"></path>
<path d="M7 8v2"></path>
<path d="M12 8v2"></path>
<path d="M17 8v2"></path>
<path d="M7 4h.01"></path>
<path d="M12 4h.01"></path>
<path d="M17 4h.01"></path>"#,
};

pub const CALCULATOR: IconType = IconType{ 
 content: r#"
<rect x="4" height="20" width="16" rx="2" y="2"></rect>
<line x2="16" y1="6" y2="6" x1="8"></line>
<line y1="14" y2="18" x2="16" x1="16"></line>
<path d="M16 10h.01"></path>
<path d="M12 10h.01"></path>
<path d="M8 10h.01"></path>
<path d="M12 14h.01"></path>
<path d="M8 14h.01"></path>
<path d="M12 18h.01"></path>
<path d="M8 18h.01"></path>"#,
};

pub const CALENDAR_CHECK_2: IconType = IconType{ 
 content: r#"
<path d="M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x2="16" x1="16" y1="2" y2="6"></line>
<line y1="2" x1="8" x2="8" y2="6"></line>
<line y2="10" x2="21" y1="10" x1="3"></line>
<path d="m16 20 2 2 4-4"></path>"#,
};

pub const CALENDAR_CHECK: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" x="3" rx="2" ry="2" y="4"></rect>
<line y2="6" y1="2" x2="16" x1="16"></line>
<line x1="8" y2="6" x2="8" y1="2"></line>
<line x1="3" y1="10" x2="21" y2="10"></line>
<path d="m9 16 2 2 4-4"></path>"#,
};

pub const CALENDAR_CLOCK: IconType = IconType{ 
 content: r#"
<path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h5"></path>
<path d="M17.5 17.5 16 16.25V14"></path>
<path d="M22 16a6 6 0 1 1-12 0 6 6 0 0 1 12 0Z"></path>"#,
};

pub const CALENDAR_DAYS: IconType = IconType{ 
 content: r#"
<rect y="4" width="18" x="3" height="18" rx="2" ry="2"></rect>
<line x2="16" x1="16" y1="2" y2="6"></line>
<line y1="2" x2="8" y2="6" x1="8"></line>
<line x2="21" y1="10" y2="10" x1="3"></line>
<path d="M8 14h.01"></path>
<path d="M12 14h.01"></path>
<path d="M16 14h.01"></path>
<path d="M8 18h.01"></path>
<path d="M12 18h.01"></path>
<path d="M16 18h.01"></path>"#,
};

pub const CALENDAR_HEART: IconType = IconType{ 
 content: r#"
<path d="M21 10V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h18"></path>
<path d="M21.29 14.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 22l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#,
};

pub const CALENDAR_MINUS: IconType = IconType{ 
 content: r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x2="16" y1="2" x1="16" y2="6"></line>
<line x2="8" y1="2" y2="6" x1="8"></line>
<line y1="10" x2="21" x1="3" y2="10"></line>
<line y1="19" y2="19" x1="16" x2="22"></line>"#,
};

pub const CALENDAR_OFF: IconType = IconType{ 
 content: r#"
<path d="M4.18 4.18A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18"></path>
<path d="M21 15.5V6a2 2 0 0 0-2-2H9.5"></path>
<path d="M16 2v4"></path>
<path d="M3 10h7"></path>
<path d="M21 10h-5.5"></path>
<line x2="22" y2="22" x1="2" y1="2"></line>"#,
};

pub const CALENDAR_PLUS: IconType = IconType{ 
 content: r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line y1="2" x1="16" y2="6" x2="16"></line>
<line x1="8" y1="2" y2="6" x2="8"></line>
<line x2="21" y1="10" y2="10" x1="3"></line>
<line x1="19" x2="19" y2="22" y1="16"></line>
<line y2="19" y1="19" x1="16" x2="22"></line>"#,
};

pub const CALENDAR_RANGE: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" rx="2" x="3" y="4" ry="2"></rect>
<line y2="6" x1="16" y1="2" x2="16"></line>
<line x2="8" y1="2" y2="6" x1="8"></line>
<line y2="10" x2="21" y1="10" x1="3"></line>
<path d="M17 14h-6"></path>
<path d="M13 18H7"></path>
<path d="M7 14h.01"></path>
<path d="M17 18h.01"></path>"#,
};

pub const CALENDAR_SEARCH: IconType = IconType{ 
 content: r#"
<path d="M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7.5"></path>
<path d="M16 2v4"></path>
<path d="M8 2v4"></path>
<path d="M3 10h18"></path>
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z"></path>
<path d="m22 22-1.5-1.5"></path>"#,
};

pub const CALENDAR_X_2: IconType = IconType{ 
 content: r#"
<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8"></path>
<line x1="16" x2="16" y2="6" y1="2"></line>
<line x1="8" x2="8" y1="2" y2="6"></line>
<line y1="10" x2="21" x1="3" y2="10"></line>
<line y1="17" x1="17" x2="22" y2="22"></line>
<line y1="22" x1="17" y2="17" x2="22"></line>"#,
};

pub const CALENDAR_X: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" y="4" rx="2" ry="2" x="3"></rect>
<line x1="16" x2="16" y2="6" y1="2"></line>
<line y2="6" x2="8" x1="8" y1="2"></line>
<line x2="21" y1="10" y2="10" x1="3"></line>
<line x1="10" y1="14" x2="14" y2="18"></line>
<line x2="10" x1="14" y2="18" y1="14"></line>"#,
};

pub const CALENDAR: IconType = IconType{ 
 content: r#"
<rect x="3" y="4" ry="2" height="18" rx="2" width="18"></rect>
<line x1="16" x2="16" y2="6" y1="2"></line>
<line x1="8" x2="8" y1="2" y2="6"></line>
<line y1="10" y2="10" x2="21" x1="3"></line>"#,
};

pub const CAMERA_OFF: IconType = IconType{ 
 content: r#"
<line x2="22" y1="2" x1="2" y2="22"></line>
<path d="M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16"></path>
<path d="M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5"></path>
<path d="M14.121 15.121A3 3 0 1 1 9.88 10.88"></path>"#,
};

pub const CAMERA: IconType = IconType{ 
 content: r#"
<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"></path>
<circle cx="12" cy="13" r="3"></circle>"#,
};

pub const CANDLESTICK_CHART: IconType = IconType{ 
 content: r#"
<path d="M9 5v4"></path>
<rect x="7" height="6" y="9" rx="1" width="4"></rect>
<path d="M9 15v2"></path>
<path d="M17 3v2"></path>
<rect height="8" width="4" y="5" rx="1" x="15"></rect>
<path d="M17 13v3"></path>
<path d="M3 3v18h18"></path>"#,
};

pub const CANDY_CANE: IconType = IconType{ 
 content: r#"
<path d="M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2Z"></path>
<path d="M17.75 7 15 2.1"></path>
<path d="M10.9 4.8 13 9"></path>
<path d="m7.9 9.7 2 4.4"></path>
<path d="M4.9 14.7 7 18.9"></path>"#,
};

pub const CANDY_OFF: IconType = IconType{ 
 content: r#"
<path d="m8.5 8.5-1 1a4.95 4.95 0 0 0 7 7l1-1"></path>
<path d="M11.843 6.187A4.947 4.947 0 0 1 16.5 7.5a4.947 4.947 0 0 1 1.313 4.657"></path>
<path d="M14 16.5V14"></path>
<path d="M14 6.5v1.843"></path>
<path d="M10 10v7.5"></path>
<path d="m16 7 1-5 1.367.683A3 3 0 0 0 19.708 3H21v1.292a3 3 0 0 0 .317 1.341L22 7l-5 1"></path>
<path d="m8 17-1 5-1.367-.683A3 3 0 0 0 4.292 21H3v-1.292a3 3 0 0 0-.317-1.341L2 17l5-1"></path>
<line y2="22" x2="22" y1="2" x1="2"></line>"#,
};

pub const CANDY: IconType = IconType{ 
 content: r#"
<path d="m9.5 7.5-2 2a4.95 4.95 0 1 0 7 7l2-2a4.95 4.95 0 1 0-7-7Z"></path>
<path d="M14 6.5v10"></path>
<path d="M10 7.5v10"></path>
<path d="m16 7 1-5 1.37.68A3 3 0 0 0 19.7 3H21v1.3c0 .46.1.92.32 1.33L22 7l-5 1"></path>
<path d="m8 17-1 5-1.37-.68A3 3 0 0 0 4.3 21H3v-1.3a3 3 0 0 0-.32-1.33L2 17l5-1"></path>"#,
};

pub const CAR_FRONT: IconType = IconType{ 
 content: r#"
<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"></path>
<path d="M7 14h.01"></path>
<path d="M17 14h.01"></path>
<rect rx="2" x="3" width="18" height="8" y="10"></rect>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#,
};

pub const CAR_TAXI_FRONT: IconType = IconType{ 
 content: r#"
<path d="M10 2h4"></path>
<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"></path>
<path d="M7 14h.01"></path>
<path d="M17 14h.01"></path>
<rect width="18" rx="2" x="3" height="8" y="10"></rect>
<path d="M5 18v2"></path>
<path d="M19 18v2"></path>"#,
};

pub const CAR: IconType = IconType{ 
 content: r#"
<path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"></path>
<circle cy="17" r="2" cx="7"></circle>
<path d="M9 17h6"></path>
<circle cy="17" r="2" cx="17"></circle>"#,
};

pub const CARROT: IconType = IconType{ 
 content: r#"
<path d="M2.27 21.7s9.87-3.5 12.73-6.36a4.5 4.5 0 0 0-6.36-6.37C5.77 11.84 2.27 21.7 2.27 21.7zM8.64 14l-2.05-2.04M15.34 15l-2.46-2.46"></path>
<path d="M22 9s-1.33-2-3.5-2C16.86 7 15 9 15 9s1.33 2 3.5 2S22 9 22 9z"></path>
<path d="M15 2s-2 1.33-2 3.5S15 9 15 9s2-1.84 2-3.5C17 3.33 15 2 15 2z"></path>"#,
};

pub const CASE_LOWER: IconType = IconType{ 
 content: r#"
<circle r="3" cy="12" cx="7"></circle>
<path d="M10 9v6"></path>
<circle cx="17" cy="12" r="3"></circle>
<path d="M14 7v8"></path>"#,
};

pub const CASE_SENSITIVE: IconType = IconType{ 
 content: r#"
<path d="m3 15 4-8 4 8"></path>
<path d="M4 13h6"></path>
<circle cx="18" cy="12" r="3"></circle>
<path d="M21 9v6"></path>"#,
};

pub const CASE_UPPER: IconType = IconType{ 
 content: r#"
<path d="m3 15 4-8 4 8"></path>
<path d="M4 13h6"></path>
<path d="M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4"></path>"#,
};

pub const CASSETTE_TAPE: IconType = IconType{ 
 content: r#"
<rect height="16" rx="2" x="2" width="20" y="4"></rect>
<circle cx="8" r="2" cy="10"></circle>
<path d="M8 12h8"></path>
<circle cx="16" r="2" cy="10"></circle>
<path d="m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3"></path>"#,
};

pub const CAST: IconType = IconType{ 
 content: r#"
<path d="M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6"></path>
<path d="M2 12a9 9 0 0 1 8 8"></path>
<path d="M2 16a5 5 0 0 1 4 4"></path>
<line x1="2" x2="2.01" y1="20" y2="20"></line>"#,
};

pub const CASTLE: IconType = IconType{ 
 content: r#"
<path d="M22 20v-9H2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z"></path>
<path d="M18 11V4H6v7"></path>
<path d="M15 22v-4a3 3 0 0 0-3-3v0a3 3 0 0 0-3 3v4"></path>
<path d="M22 11V9"></path>
<path d="M2 11V9"></path>
<path d="M6 4V2"></path>
<path d="M18 4V2"></path>
<path d="M10 4V2"></path>
<path d="M14 4V2"></path>"#,
};

pub const CAT: IconType = IconType{ 
 content: r#"
<path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z"></path>
<path d="M8 14v.5"></path>
<path d="M16 14v.5"></path>
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z"></path>"#,
};

pub const CHECK_CHECK: IconType = IconType{ 
 content: r#"
<path d="M18 6 7 17l-5-5"></path>
<path d="m22 10-7.5 7.5L13 16"></path>"#,
};

pub const CHECK_CIRCLE_2: IconType = IconType{ 
 content: r#"
<path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"></path>
<path d="m9 12 2 2 4-4"></path>"#,
};

pub const CHECK_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
<polyline points="22 4 12 14.01 9 11.01"></polyline>"#,
};

pub const CHECK_SQUARE: IconType = IconType{ 
 content: r#"
<polyline points="9 11 12 14 22 4"></polyline>
<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>"#,
};

pub const CHECK: IconType = IconType{ 
 content: r#"
<polyline points="20 6 9 17 4 12"></polyline>"#,
};

pub const CHEF_HAT: IconType = IconType{ 
 content: r#"
<path d="M6 13.87A4 4 0 0 1 7.41 6a5.11 5.11 0 0 1 1.05-1.54 5 5 0 0 1 7.08 0A5.11 5.11 0 0 1 16.59 6 4 4 0 0 1 18 13.87V21H6Z"></path>
<line x2="18" y1="17" x1="6" y2="17"></line>"#,
};

pub const CHERRY: IconType = IconType{ 
 content: r#"
<path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"></path>
<path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"></path>
<path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12"></path>
<path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z"></path>"#,
};

pub const CHEVRON_DOWN_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="m16 10-4 4-4-4"></path>"#,
};

pub const CHEVRON_DOWN_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" rx="2" y="3" x="3"></rect>
<path d="m16 10-4 4-4-4"></path>"#,
};

pub const CHEVRON_DOWN: IconType = IconType{ 
 content: r#"
<path d="m6 9 6 6 6-6"></path>"#,
};

pub const CHEVRON_FIRST: IconType = IconType{ 
 content: r#"
<path d="m17 18-6-6 6-6"></path>
<path d="M7 6v12"></path>"#,
};

pub const CHEVRON_LAST: IconType = IconType{ 
 content: r#"
<path d="m7 18 6-6-6-6"></path>
<path d="M17 6v12"></path>"#,
};

pub const CHEVRON_LEFT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m14 16-4-4 4-4"></path>"#,
};

pub const CHEVRON_LEFT_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" y="3" x="3" rx="2"></rect>
<path d="m14 16-4-4 4-4"></path>"#,
};

pub const CHEVRON_LEFT: IconType = IconType{ 
 content: r#"
<path d="m15 18-6-6 6-6"></path>"#,
};

pub const CHEVRON_RIGHT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m10 8 4 4-4 4"></path>"#,
};

pub const CHEVRON_RIGHT_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" width="18" height="18" y="3" rx="2"></rect>
<path d="m10 8 4 4-4 4"></path>"#,
};

pub const CHEVRON_RIGHT: IconType = IconType{ 
 content: r#"
<path d="m9 18 6-6-6-6"></path>"#,
};

pub const CHEVRON_UP_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="m8 14 4-4 4 4"></path>"#,
};

pub const CHEVRON_UP_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" rx="2" y="3" x="3" width="18"></rect>
<path d="m8 14 4-4 4 4"></path>"#,
};

pub const CHEVRON_UP: IconType = IconType{ 
 content: r#"
<path d="m18 15-6-6-6 6"></path>"#,
};

pub const CHEVRONS_DOWN_UP: IconType = IconType{ 
 content: r#"
<path d="m7 20 5-5 5 5"></path>
<path d="m7 4 5 5 5-5"></path>"#,
};

pub const CHEVRONS_DOWN: IconType = IconType{ 
 content: r#"
<path d="m7 6 5 5 5-5"></path>
<path d="m7 13 5 5 5-5"></path>"#,
};

pub const CHEVRONS_LEFT_RIGHT: IconType = IconType{ 
 content: r#"
<path d="m9 7-5 5 5 5"></path>
<path d="m15 7 5 5-5 5"></path>"#,
};

pub const CHEVRONS_LEFT: IconType = IconType{ 
 content: r#"
<path d="m11 17-5-5 5-5"></path>
<path d="m18 17-5-5 5-5"></path>"#,
};

pub const CHEVRONS_RIGHT_LEFT: IconType = IconType{ 
 content: r#"
<path d="m20 17-5-5 5-5"></path>
<path d="m4 17 5-5-5-5"></path>"#,
};

pub const CHEVRONS_RIGHT: IconType = IconType{ 
 content: r#"
<path d="m6 17 5-5-5-5"></path>
<path d="m13 17 5-5-5-5"></path>"#,
};

pub const CHEVRONS_UP_DOWN: IconType = IconType{ 
 content: r#"
<path d="m7 15 5 5 5-5"></path>
<path d="m7 9 5-5 5 5"></path>"#,
};

pub const CHEVRONS_UP: IconType = IconType{ 
 content: r#"
<path d="m17 11-5-5-5 5"></path>
<path d="m17 18-5-5-5 5"></path>"#,
};

pub const CHROME: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<circle cx="12" cy="12" r="4"></circle>
<line y2="8" y1="8" x1="21.17" x2="12"></line>
<line x1="3.95" y2="14" x2="8.54" y1="6.06"></line>
<line y1="21.94" x1="10.88" x2="15.46" y2="14"></line>"#,
};

pub const CHURCH: IconType = IconType{ 
 content: r#"
<path d="m18 7 4 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9l4-2"></path>
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4"></path>
<path d="M18 22V5l-6-3-6 3v17"></path>
<path d="M12 7v5"></path>
<path d="M10 9h4"></path>"#,
};

pub const CIGARETTE_OFF: IconType = IconType{ 
 content: r#"
<line x2="22" y2="22" x1="2" y1="2"></line>
<path d="M12 12H2v4h14"></path>
<path d="M22 12v4"></path>
<path d="M18 12h-.5"></path>
<path d="M7 12v4"></path>
<path d="M18 8c0-2.5-2-2.5-2-5"></path>
<path d="M22 8c0-2.5-2-2.5-2-5"></path>"#,
};

pub const CIGARETTE: IconType = IconType{ 
 content: r#"
<path d="M18 12H2v4h16"></path>
<path d="M22 12v4"></path>
<path d="M7 12v4"></path>
<path d="M18 8c0-2.5-2-2.5-2-5"></path>
<path d="M22 8c0-2.5-2-2.5-2-5"></path>"#,
};

pub const CIRCLE_DASHED: IconType = IconType{ 
 content: r#"
<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"></path>
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"></path>
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"></path>
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"></path>
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"></path>
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"></path>
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"></path>
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"></path>"#,
};

pub const CIRCLE_DOLLAR_SIGN: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 18V6"></path>"#,
};

pub const CIRCLE_DOT_DASHED: IconType = IconType{ 
 content: r#"
<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"></path>
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"></path>
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"></path>
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"></path>
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"></path>
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"></path>
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"></path>
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"></path>
<circle cx="12" r="1" cy="12"></circle>"#,
};

pub const CIRCLE_DOT: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<circle cy="12" r="1" cx="12"></circle>"#,
};

pub const CIRCLE_ELLIPSIS: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M17 12h.01"></path>
<path d="M12 12h.01"></path>
<path d="M7 12h.01"></path>"#,
};

pub const CIRCLE_EQUAL: IconType = IconType{ 
 content: r#"
<path d="M7 10h10"></path>
<path d="M7 14h10"></path>
<circle r="10" cx="12" cy="12"></circle>"#,
};

pub const CIRCLE_OFF: IconType = IconType{ 
 content: r#"
<path d="m2 2 20 20"></path>
<path d="M8.35 2.69A10 10 0 0 1 21.3 15.65"></path>
<path d="M19.08 19.08A10 10 0 1 1 4.92 4.92"></path>"#,
};

pub const CIRCLE_SLASH_2: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M22 2 2 22"></path>"#,
};

pub const CIRCLE_SLASH: IconType = IconType{ 
 content: r#"
<line x1="9" y1="15" y2="9" x2="15"></line>
<circle cx="12" r="10" cy="12"></circle>"#,
};

pub const CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>"#,
};

pub const CIRCUIT_BOARD: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" y="3" width="18" rx="2"></rect>
<path d="M11 9h4a2 2 0 0 0 2-2V3"></path>
<circle cy="9" cx="9" r="2"></circle>
<path d="M7 21v-4a2 2 0 0 1 2-2h4"></path>
<circle cy="15" r="2" cx="15"></circle>"#,
};

pub const CITRUS: IconType = IconType{ 
 content: r#"
<path d="M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z"></path>
<path d="M19.65 15.66A8 8 0 0 1 8.35 4.34"></path>
<path d="m14 10-5.5 5.5"></path>
<path d="M14 17.85V10H6.15"></path>"#,
};

pub const CLAPPERBOARD: IconType = IconType{ 
 content: r#"
<path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3Z"></path>
<path d="m6.2 5.3 3.1 3.9"></path>
<path d="m12.4 3.4 3.1 4"></path>
<path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z"></path>"#,
};

pub const CLIPBOARD_CHECK: IconType = IconType{ 
 content: r#"
<rect height="4" rx="1" ry="1" y="2" x="8" width="8"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="m9 14 2 2 4-4"></path>"#,
};

pub const CLIPBOARD_COPY: IconType = IconType{ 
 content: r#"
<rect x="8" height="4" y="2" rx="1" ry="1" width="8"></rect>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"></path>
<path d="M16 4h2a2 2 0 0 1 2 2v4"></path>
<path d="M21 14H11"></path>
<path d="m15 10-4 4 4 4"></path>"#,
};

pub const CLIPBOARD_EDIT: IconType = IconType{ 
 content: r#"
<rect x="8" y="2" rx="1" ry="1" width="8" height="4"></rect>
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z"></path>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-5.5"></path>
<path d="M4 13.5V6a2 2 0 0 1 2-2h2"></path>"#,
};

pub const CLIPBOARD_LIST: IconType = IconType{ 
 content: r#"
<rect y="2" height="4" x="8" rx="1" ry="1" width="8"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="M12 11h4"></path>
<path d="M12 16h4"></path>
<path d="M8 11h.01"></path>
<path d="M8 16h.01"></path>"#,
};

pub const CLIPBOARD_PASTE: IconType = IconType{ 
 content: r#"
<path d="M15 2H9a1 1 0 0 0-1 1v2c0 .6.4 1 1 1h6c.6 0 1-.4 1-1V3c0-.6-.4-1-1-1Z"></path>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M16 4h2a2 2 0 0 1 2 2v2M11 14h10"></path>
<path d="m17 10 4 4-4 4"></path>"#,
};

pub const CLIPBOARD_SIGNATURE: IconType = IconType{ 
 content: r#"
<rect width="8" rx="1" ry="1" x="8" height="4" y="2"></rect>
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5"></path>
<path d="M16 4h2a2 2 0 0 1 1.73 1"></path>
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z"></path>
<path d="M8 18h1"></path>"#,
};

pub const CLIPBOARD_TYPE: IconType = IconType{ 
 content: r#"
<rect y="2" height="4" ry="1" rx="1" width="8" x="8"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="M9 12v-1h6v1"></path>
<path d="M11 17h2"></path>
<path d="M12 11v6"></path>"#,
};

pub const CLIPBOARD_X: IconType = IconType{ 
 content: r#"
<rect ry="1" rx="1" height="4" width="8" x="8" y="2"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
<path d="m15 11-6 6"></path>
<path d="m9 11 6 6"></path>"#,
};

pub const CLIPBOARD: IconType = IconType{ 
 content: r#"
<rect y="2" x="8" ry="1" height="4" width="8" rx="1"></rect>
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>"#,
};

pub const CLOCK_1: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<polyline points="12 6 12 12 14.5 8"></polyline>"#,
};

pub const CLOCK_10: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 8 10"></polyline>"#,
};

pub const CLOCK_11: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 9.5 8"></polyline>"#,
};

pub const CLOCK_12: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12"></polyline>"#,
};

pub const CLOCK_2: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12 16 10"></polyline>"#,
};

pub const CLOCK_3: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<polyline points="12 6 12 12 16.5 12"></polyline>"#,
};

pub const CLOCK_4: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<polyline points="12 6 12 12 16 14"></polyline>"#,
};

pub const CLOCK_5: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12 14.5 16"></polyline>"#,
};

pub const CLOCK_6: IconType = IconType{ 
 content: r#"
<circle cx="12" r="10" cy="12"></circle>
<polyline points="12 6 12 12 12 16.5"></polyline>"#,
};

pub const CLOCK_7: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 9.5 16"></polyline>"#,
};

pub const CLOCK_8: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<polyline points="12 6 12 12 8 14"></polyline>"#,
};

pub const CLOCK_9: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<polyline points="12 6 12 12 7.5 12"></polyline>"#,
};

pub const CLOCK: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<polyline points="12 6 12 12 16 14"></polyline>"#,
};

pub const CLOUD_COG: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="17" r="3"></circle>
<path d="M4.2 15.1A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2"></path>
<path d="m15.7 18.4-.9-.3"></path>
<path d="m9.2 15.9-.9-.3"></path>
<path d="m10.6 20.7.3-.9"></path>
<path d="m13.1 14.2.3-.9"></path>
<path d="m13.6 20.7-.4-1"></path>
<path d="m10.8 14.3-.4-1"></path>
<path d="m8.3 18.6 1-.4"></path>
<path d="m14.7 15.8 1-.4"></path>"#,
};

pub const CLOUD_DRIZZLE: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M8 19v1"></path>
<path d="M8 14v1"></path>
<path d="M16 19v1"></path>
<path d="M16 14v1"></path>
<path d="M12 21v1"></path>
<path d="M12 16v1"></path>"#,
};

pub const CLOUD_FOG: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 17H7"></path>
<path d="M17 21H9"></path>"#,
};

pub const CLOUD_HAIL: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 14v2"></path>
<path d="M8 14v2"></path>
<path d="M16 20h.01"></path>
<path d="M8 20h.01"></path>
<path d="M12 16v2"></path>
<path d="M12 22h.01"></path>"#,
};

pub const CLOUD_LIGHTNING: IconType = IconType{ 
 content: r#"
<path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973"></path>
<path d="m13 12-3 5h4l-3 5"></path>"#,
};

pub const CLOUD_MOON_RAIN: IconType = IconType{ 
 content: r#"
<path d="M10.083 9A6.002 6.002 0 0 1 16 4a4.243 4.243 0 0 0 6 6c0 2.22-1.206 4.16-3 5.197"></path>
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"></path>
<path d="M11 20v2"></path>
<path d="M7 19v2"></path>"#,
};

pub const CLOUD_MOON: IconType = IconType{ 
 content: r#"
<path d="M13 16a3 3 0 1 1 0 6H7a5 5 0 1 1 4.9-6Z"></path>
<path d="M10.1 9A6 6 0 0 1 16 4a4.24 4.24 0 0 0 6 6 6 6 0 0 1-3 5.197"></path>"#,
};

pub const CLOUD_OFF: IconType = IconType{ 
 content: r#"
<path d="m2 2 20 20"></path>
<path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193"></path>
<path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07"></path>"#,
};

pub const CLOUD_RAIN_WIND: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="m9.2 22 3-7"></path>
<path d="m9 13-3 7"></path>
<path d="m17 13-3 7"></path>"#,
};

pub const CLOUD_RAIN: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M16 14v6"></path>
<path d="M8 14v6"></path>
<path d="M12 16v6"></path>"#,
};

pub const CLOUD_SNOW: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M8 15h.01"></path>
<path d="M8 19h.01"></path>
<path d="M12 17h.01"></path>
<path d="M12 21h.01"></path>
<path d="M16 15h.01"></path>
<path d="M16 19h.01"></path>"#,
};

pub const CLOUD_SUN_RAIN: IconType = IconType{ 
 content: r#"
<path d="M12 2v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="M20 12h2"></path>
<path d="m19.07 4.93-1.41 1.41"></path>
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"></path>
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"></path>
<path d="M11 20v2"></path>
<path d="M7 19v2"></path>"#,
};

pub const CLOUD_SUN: IconType = IconType{ 
 content: r#"
<path d="M12 2v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="M20 12h2"></path>
<path d="m19.07 4.93-1.41 1.41"></path>
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"></path>
<path d="M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z"></path>"#,
};

pub const CLOUD: IconType = IconType{ 
 content: r#"
<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>"#,
};

pub const CLOUDY: IconType = IconType{ 
 content: r#"
<path d="M17.5 21H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>
<path d="M22 10a3 3 0 0 0-3-3h-2.207a5.502 5.502 0 0 0-10.702.5"></path>"#,
};

pub const CLOVER: IconType = IconType{ 
 content: r#"
<path d="M16.2 3.8a2.7 2.7 0 0 0-3.81 0l-.4.38-.4-.4a2.7 2.7 0 0 0-3.82 0C6.73 4.85 6.67 6.64 8 8l4 4 4-4c1.33-1.36 1.27-3.15.2-4.2z"></path>
<path d="M8 8c-1.36-1.33-3.15-1.27-4.2-.2a2.7 2.7 0 0 0 0 3.81l.38.4-.4.4a2.7 2.7 0 0 0 0 3.82C4.85 17.27 6.64 17.33 8 16"></path>
<path d="M16 16c1.36 1.33 3.15 1.27 4.2.2a2.7 2.7 0 0 0 0-3.81l-.38-.4.4-.4a2.7 2.7 0 0 0 0-3.82C19.15 6.73 17.36 6.67 16 8"></path>
<path d="M7.8 20.2a2.7 2.7 0 0 0 3.81 0l.4-.38.4.4a2.7 2.7 0 0 0 3.82 0c1.06-1.06 1.12-2.85-.21-4.21l-4-4-4 4c-1.33 1.36-1.27 3.15-.2 4.2z"></path>
<path d="m7 17-5 5"></path>"#,
};

pub const CLUB: IconType = IconType{ 
 content: r#"
<path d="M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z"></path>
<path d="M12 17.66L12 22"></path>"#,
};

pub const CODE_2: IconType = IconType{ 
 content: r#"
<path d="m18 16 4-4-4-4"></path>
<path d="m6 8-4 4 4 4"></path>
<path d="m14.5 4-5 16"></path>"#,
};

pub const CODE: IconType = IconType{ 
 content: r#"
<polyline points="16 18 22 12 16 6"></polyline>
<polyline points="8 6 2 12 8 18"></polyline>"#,
};

pub const CODEPEN: IconType = IconType{ 
 content: r#"
<polygon points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2"></polygon>
<line x1="12" x2="12" y2="15.5" y1="22"></line>
<polyline points="22 8.5 12 15.5 2 8.5"></polyline>
<polyline points="2 15.5 12 8.5 22 15.5"></polyline>
<line y1="2" x2="12" x1="12" y2="8.5"></line>"#,
};

pub const CODESANDBOX: IconType = IconType{ 
 content: r#"
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
<polyline points="7.5 4.21 12 6.81 16.5 4.21"></polyline>
<polyline points="7.5 19.79 7.5 14.6 3 12"></polyline>
<polyline points="21 12 16.5 14.6 16.5 19.79"></polyline>
<polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
<line y1="22.08" x1="12" y2="12" x2="12"></line>"#,
};

pub const COFFEE: IconType = IconType{ 
 content: r#"
<path d="M17 8h1a4 4 0 1 1 0 8h-1"></path>
<path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z"></path>
<line x1="6" y1="2" y2="4" x2="6"></line>
<line x1="10" x2="10" y2="4" y1="2"></line>
<line x1="14" y1="2" y2="4" x2="14"></line>"#,
};

pub const COG: IconType = IconType{ 
 content: r#"
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
<path d="m11 13.73-4 6.93"></path>"#,
};

pub const COINS: IconType = IconType{ 
 content: r#"
<circle cy="8" r="6" cx="8"></circle>
<path d="M18.09 10.37A6 6 0 1 1 10.34 18"></path>
<path d="M7 6h1v4"></path>
<path d="m16.71 13.88.7.71-2.82 2.82"></path>"#,
};

pub const COLUMNS: IconType = IconType{ 
 content: r#"
<rect ry="2" x="3" rx="2" width="18" y="3" height="18"></rect>
<line y2="21" x2="12" x1="12" y1="3"></line>"#,
};

pub const COMBINE: IconType = IconType{ 
 content: r#"
<rect x="2" y="2" width="8" height="8" rx="2"></rect>
<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M10 18H5c-1.7 0-3-1.3-3-3v-1"></path>
<polyline points="7 21 10 18 7 15"></polyline>
<rect height="8" x="14" y="14" width="8" rx="2"></rect>"#,
};

pub const COMMAND: IconType = IconType{ 
 content: r#"
<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"></path>"#,
};

pub const COMPASS: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"></polygon>"#,
};

pub const COMPONENT: IconType = IconType{ 
 content: r#"
<path d="M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z"></path>
<path d="m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z"></path>
<path d="M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z"></path>
<path d="m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z"></path>"#,
};

pub const COMPUTER: IconType = IconType{ 
 content: r#"
<rect y="2" rx="2" x="5" width="14" height="8"></rect>
<rect height="8" rx="2" width="20" y="14" x="2"></rect>
<path d="M6 18h2"></path>
<path d="M12 18h6"></path>"#,
};

pub const CONCIERGE_BELL: IconType = IconType{ 
 content: r#"
<path d="M2 18a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2H2v-2Z"></path>
<path d="M20 16a8 8 0 1 0-16 0"></path>
<path d="M12 4v4"></path>
<path d="M10 4h4"></path>"#,
};

pub const CONSTRUCTION: IconType = IconType{ 
 content: r#"
<rect height="8" y="6" width="20" x="2" rx="1"></rect>
<path d="M17 14v7"></path>
<path d="M7 14v7"></path>
<path d="M17 3v3"></path>
<path d="M7 3v3"></path>
<path d="M10 14 2.3 6.3"></path>
<path d="m14 6 7.7 7.7"></path>
<path d="m8 6 8 8"></path>"#,
};

pub const CONTACT_2: IconType = IconType{ 
 content: r#"
<path d="M16 18a4 4 0 0 0-8 0"></path>
<circle r="3" cy="11" cx="12"></circle>
<rect height="18" rx="2" x="3" width="18" y="4"></rect>
<line y1="2" x1="8" y2="4" x2="8"></line>
<line y2="4" x1="16" x2="16" y1="2"></line>"#,
};

pub const CONTACT: IconType = IconType{ 
 content: r#"
<path d="M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2"></path>
<rect y="4" width="18" x="3" rx="2" height="18"></rect>
<circle cy="10" r="2" cx="12"></circle>
<line y1="2" x1="8" x2="8" y2="4"></line>
<line x2="16" x1="16" y1="2" y2="4"></line>"#,
};

pub const CONTAINER: IconType = IconType{ 
 content: r#"
<path d="M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z"></path>
<path d="M10 21.9V14L2.1 9.1"></path>
<path d="m10 14 11.9-6.9"></path>
<path d="M14 19.8v-8.1"></path>
<path d="M18 17.5V9.4"></path>"#,
};

pub const CONTRAST: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<path d="M12 18a6 6 0 0 0 0-12v12z"></path>"#,
};

pub const COOKIE: IconType = IconType{ 
 content: r#"
<path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5"></path>
<path d="M8.5 8.5v.01"></path>
<path d="M16 15.5v.01"></path>
<path d="M12 12v.01"></path>
<path d="M11 17v.01"></path>
<path d="M7 14v.01"></path>"#,
};

pub const COPY_CHECK: IconType = IconType{ 
 content: r#"
<path d="m12 15 2 2 4-4"></path>
<rect rx="2" width="14" height="14" x="8" y="8" ry="2"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPY_MINUS: IconType = IconType{ 
 content: r#"
<line x2="18" y1="15" x1="12" y2="15"></line>
<rect width="14" x="8" y="8" height="14" rx="2" ry="2"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPY_PLUS: IconType = IconType{ 
 content: r#"
<line y1="12" y2="18" x2="15" x1="15"></line>
<line x1="12" x2="18" y1="15" y2="15"></line>
<rect width="14" y="8" rx="2" height="14" ry="2" x="8"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPY_SLASH: IconType = IconType{ 
 content: r#"
<line x1="12" y1="18" x2="18" y2="12"></line>
<rect height="14" rx="2" ry="2" x="8" y="8" width="14"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPY_X: IconType = IconType{ 
 content: r#"
<line x1="12" x2="18" y2="18" y1="12"></line>
<line x2="18" y2="12" x1="12" y1="18"></line>
<rect rx="2" width="14" x="8" y="8" ry="2" height="14"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPY: IconType = IconType{ 
 content: r#"
<rect rx="2" height="14" y="8" width="14" x="8" ry="2"></rect>
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>"#,
};

pub const COPYLEFT: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M9 9.35a4 4 0 1 1 0 5.3"></path>"#,
};

pub const COPYRIGHT: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M15 9.354a4 4 0 1 0 0 5.292"></path>"#,
};

pub const CORNER_DOWN_LEFT: IconType = IconType{ 
 content: r#"
<polyline points="9 10 4 15 9 20"></polyline>
<path d="M20 4v7a4 4 0 0 1-4 4H4"></path>"#,
};

pub const CORNER_DOWN_RIGHT: IconType = IconType{ 
 content: r#"
<polyline points="15 10 20 15 15 20"></polyline>
<path d="M4 4v7a4 4 0 0 0 4 4h12"></path>"#,
};

pub const CORNER_LEFT_DOWN: IconType = IconType{ 
 content: r#"
<polyline points="14 15 9 20 4 15"></polyline>
<path d="M20 4h-7a4 4 0 0 0-4 4v12"></path>"#,
};

pub const CORNER_LEFT_UP: IconType = IconType{ 
 content: r#"
<polyline points="14 9 9 4 4 9"></polyline>
<path d="M20 20h-7a4 4 0 0 1-4-4V4"></path>"#,
};

pub const CORNER_RIGHT_DOWN: IconType = IconType{ 
 content: r#"
<polyline points="10 15 15 20 20 15"></polyline>
<path d="M4 4h7a4 4 0 0 1 4 4v12"></path>"#,
};

pub const CORNER_RIGHT_UP: IconType = IconType{ 
 content: r#"
<polyline points="10 9 15 4 20 9"></polyline>
<path d="M4 20h7a4 4 0 0 0 4-4V4"></path>"#,
};

pub const CORNER_UP_LEFT: IconType = IconType{ 
 content: r#"
<polyline points="9 14 4 9 9 4"></polyline>
<path d="M20 20v-7a4 4 0 0 0-4-4H4"></path>"#,
};

pub const CORNER_UP_RIGHT: IconType = IconType{ 
 content: r#"
<polyline points="15 14 20 9 15 4"></polyline>
<path d="M4 20v-7a4 4 0 0 1 4-4h12"></path>"#,
};

pub const CPU: IconType = IconType{ 
 content: r#"
<rect y="4" height="16" x="4" width="16" rx="2"></rect>
<rect width="6" height="6" x="9" y="9"></rect>
<path d="M15 2v2"></path>
<path d="M15 20v2"></path>
<path d="M2 15h2"></path>
<path d="M2 9h2"></path>
<path d="M20 15h2"></path>
<path d="M20 9h2"></path>
<path d="M9 2v2"></path>
<path d="M9 20v2"></path>"#,
};

pub const CREATIVE_COMMONS: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"></path>
<path d="M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"></path>"#,
};

pub const CREDIT_CARD: IconType = IconType{ 
 content: r#"
<rect x="2" width="20" height="14" y="5" rx="2"></rect>
<line x2="22" x1="2" y2="10" y1="10"></line>"#,
};

pub const CROISSANT: IconType = IconType{ 
 content: r#"
<path d="m4.6 13.11 5.79-3.21c1.89-1.05 4.79 1.78 3.71 3.71l-3.22 5.81C8.8 23.16.79 15.23 4.6 13.11Z"></path>
<path d="m10.5 9.5-1-2.29C9.2 6.48 8.8 6 8 6H4.5C2.79 6 2 6.5 2 8.5a7.71 7.71 0 0 0 2 4.83"></path>
<path d="M8 6c0-1.55.24-4-2-4-2 0-2.5 2.17-2.5 4"></path>
<path d="m14.5 13.5 2.29 1c.73.3 1.21.7 1.21 1.5v3.5c0 1.71-.5 2.5-2.5 2.5a7.71 7.71 0 0 1-4.83-2"></path>
<path d="M18 16c1.55 0 4-.24 4 2 0 2-2.17 2.5-4 2.5"></path>"#,
};

pub const CROP: IconType = IconType{ 
 content: r#"
<path d="M6 2v14a2 2 0 0 0 2 2h14"></path>
<path d="M18 22V8a2 2 0 0 0-2-2H2"></path>"#,
};

pub const CROSS: IconType = IconType{ 
 content: r#"
<path d="M11 2a2 2 0 0 0-2 2v5H4a2 2 0 0 0-2 2v2c0 1.1.9 2 2 2h5v5c0 1.1.9 2 2 2h2a2 2 0 0 0 2-2v-5h5a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-5V4a2 2 0 0 0-2-2h-2z"></path>"#,
};

pub const CROSSHAIR: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<line x2="18" y1="12" y2="12" x1="22"></line>
<line x2="2" y1="12" y2="12" x1="6"></line>
<line y1="6" x1="12" x2="12" y2="2"></line>
<line x1="12" y2="18" x2="12" y1="22"></line>"#,
};

pub const CROWN: IconType = IconType{ 
 content: r#"
<path d="m2 4 3 12h14l3-12-6 7-4-7-4 7-6-7zm3 16h14"></path>"#,
};

pub const CUP_SODA: IconType = IconType{ 
 content: r#"
<path d="m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8"></path>
<path d="M5 8h14"></path>
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0"></path>
<path d="m12 8 1-6h2"></path>"#,
};

pub const CURRENCY: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="8"></circle>
<line x1="3" y1="3" x2="6" y2="6"></line>
<line y2="6" x1="21" y1="3" x2="18"></line>
<line y2="18" x1="3" x2="6" y1="21"></line>
<line x2="18" y2="18" y1="21" x1="21"></line>"#,
};

pub const DATABASE_BACKUP: IconType = IconType{ 
 content: r#"
<ellipse rx="9" cy="5" ry="3" cx="12"></ellipse>
<path d="M3 12a9 3 0 0 0 5 2.69"></path>
<path d="M21 9.3V5"></path>
<path d="M3 5v14a9 3 0 0 0 6.47 2.88"></path>
<path d="M12 12v4h4"></path>
<path d="M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16"></path>"#,
};

pub const DATABASE_ZAP: IconType = IconType{ 
 content: r#"
<ellipse cy="5" cx="12" rx="9" ry="3"></ellipse>
<path d="M3 5V19A9 3 0 0 0 15 21.84"></path>
<path d="M21 5V8"></path>
<path d="M21 12L18 17H22L19 22"></path>
<path d="M3 12A9 3 0 0 0 14.59 14.87"></path>"#,
};

pub const DATABASE: IconType = IconType{ 
 content: r#"
<ellipse rx="9" cy="5" cx="12" ry="3"></ellipse>
<path d="M3 5V19A9 3 0 0 0 21 19V5"></path>
<path d="M3 12A9 3 0 0 0 21 12"></path>"#,
};

pub const DELETE: IconType = IconType{ 
 content: r#"
<path d="M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z"></path>
<line y1="9" y2="15" x2="12" x1="18"></line>
<line x1="12" y1="9" y2="15" x2="18"></line>"#,
};

pub const DESSERT: IconType = IconType{ 
 content: r#"
<circle cx="12" r="2" cy="4"></circle>
<path d="M10.2 3.2C5.5 4 2 8.1 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4 0c0-4.9-3.5-9-8.2-9.8"></path>
<path d="M3.2 14.8a9 9 0 0 0 17.6 0"></path>"#,
};

pub const DIAMOND: IconType = IconType{ 
 content: r#"
<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z"></path>"#,
};

pub const DICE_1: IconType = IconType{ 
 content: r#"
<rect width="18" ry="2" x="3" y="3" height="18" rx="2"></rect>
<path d="M12 12h.01"></path>"#,
};

pub const DICE_2: IconType = IconType{ 
 content: r#"
<rect width="18" y="3" height="18" x="3" rx="2" ry="2"></rect>
<path d="M15 9h.01"></path>
<path d="M9 15h.01"></path>"#,
};

pub const DICE_3: IconType = IconType{ 
 content: r#"
<rect width="18" ry="2" height="18" x="3" y="3" rx="2"></rect>
<path d="M16 8h.01"></path>
<path d="M12 12h.01"></path>
<path d="M8 16h.01"></path>"#,
};

pub const DICE_4: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" x="3" y="3" ry="2" rx="2"></rect>
<path d="M16 8h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 16h.01"></path>
<path d="M16 16h.01"></path>"#,
};

pub const DICE_5: IconType = IconType{ 
 content: r#"
<rect width="18" ry="2" height="18" rx="2" x="3" y="3"></rect>
<path d="M16 8h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 16h.01"></path>
<path d="M16 16h.01"></path>
<path d="M12 12h.01"></path>"#,
};

pub const DICE_6: IconType = IconType{ 
 content: r#"
<rect height="18" x="3" y="3" width="18" ry="2" rx="2"></rect>
<path d="M16 8h.01"></path>
<path d="M16 12h.01"></path>
<path d="M16 16h.01"></path>
<path d="M8 8h.01"></path>
<path d="M8 12h.01"></path>
<path d="M8 16h.01"></path>"#,
};

pub const DICES: IconType = IconType{ 
 content: r#"
<rect ry="2" width="12" y="10" x="2" rx="2" height="12"></rect>
<path d="m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6"></path>
<path d="M6 18h.01"></path>
<path d="M10 14h.01"></path>
<path d="M15 6h.01"></path>
<path d="M18 9h.01"></path>"#,
};

pub const DIFF: IconType = IconType{ 
 content: r#"
<path d="M12 3v14"></path>
<path d="M5 10h14"></path>
<path d="M5 21h14"></path>"#,
};

pub const DISC_2: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<circle r="4" cx="12" cy="12"></circle>
<path d="M12 12h.01"></path>"#,
};

pub const DISC_3: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M6 12c0-1.7.7-3.2 1.8-4.2"></path>
<circle cy="12" r="2" cx="12"></circle>
<path d="M18 12c0 1.7-.7 3.2-1.8 4.2"></path>"#,
};

pub const DISC: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<circle cy="12" cx="12" r="2"></circle>"#,
};

pub const DIVIDE_CIRCLE: IconType = IconType{ 
 content: r#"
<line x2="16" y1="12" x1="8" y2="12"></line>
<line y2="16" x2="12" y1="16" x1="12"></line>
<line x2="12" y2="8" x1="12" y1="8"></line>
<circle cx="12" r="10" cy="12"></circle>"#,
};

pub const DIVIDE_SQUARE: IconType = IconType{ 
 content: r#"
<rect width="18" x="3" rx="2" ry="2" height="18" y="3"></rect>
<line y2="12" x1="8" x2="16" y1="12"></line>
<line x1="12" y2="16" x2="12" y1="16"></line>
<line x1="12" y1="8" x2="12" y2="8"></line>"#,
};

pub const DIVIDE: IconType = IconType{ 
 content: r#"
<circle r="1" cy="6" cx="12"></circle>
<line x2="19" y1="12" y2="12" x1="5"></line>
<circle cx="12" r="1" cy="18"></circle>"#,
};

pub const DNA_OFF: IconType = IconType{ 
 content: r#"
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
<line y1="2" x2="22" x1="2" y2="22"></line>"#,
};

pub const DNA: IconType = IconType{ 
 content: r#"
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
<path d="m10 16 1.5 1.5"></path>"#,
};

pub const DOG: IconType = IconType{ 
 content: r#"
<path d="M10 5.172C10 3.782 8.423 2.679 6.5 3c-2.823.47-4.113 6.006-4 7 .08.703 1.725 1.722 3.656 1 1.261-.472 1.96-1.45 2.344-2.5"></path>
<path d="M14.267 5.172c0-1.39 1.577-2.493 3.5-2.172 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5"></path>
<path d="M8 14v.5"></path>
<path d="M16 14v.5"></path>
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z"></path>
<path d="M4.42 11.247A13.152 13.152 0 0 0 4 14.556C4 18.728 7.582 21 12 21s8-2.272 8-6.444c0-1.061-.162-2.2-.493-3.309m-9.243-6.082A8.801 8.801 0 0 1 12 5c.78 0 1.5.108 2.161.306"></path>"#,
};

pub const DOLLAR_SIGN: IconType = IconType{ 
 content: r#"
<line x1="12" x2="12" y2="22" y1="2"></line>
<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>"#,
};

pub const DONUT: IconType = IconType{ 
 content: r#"
<path d="M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3"></path>
<circle cy="12" cx="12" r="3"></circle>"#,
};

pub const DOOR_CLOSED: IconType = IconType{ 
 content: r#"
<path d="M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14"></path>
<path d="M2 20h20"></path>
<path d="M14 12v.01"></path>"#,
};

pub const DOOR_OPEN: IconType = IconType{ 
 content: r#"
<path d="M13 4h3a2 2 0 0 1 2 2v14"></path>
<path d="M2 20h3"></path>
<path d="M13 20h9"></path>
<path d="M10 12v.01"></path>
<path d="M13 4.562v16.157a1 1 0 0 1-1.242.97L5 20V5.562a2 2 0 0 1 1.515-1.94l4-1A2 2 0 0 1 13 4.561Z"></path>"#,
};

pub const DOT: IconType = IconType{ 
 content: r#"
<circle r="1" cx="12.1" cy="12.1"></circle>"#,
};

pub const DOWNLOAD_CLOUD: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M12 12v9"></path>
<path d="m8 17 4 4 4-4"></path>"#,
};

pub const DOWNLOAD: IconType = IconType{ 
 content: r#"
<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
<polyline points="7 10 12 15 17 10"></polyline>
<line x1="12" x2="12" y1="15" y2="3"></line>"#,
};

pub const DRIBBBLE: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<path d="M19.13 5.09C15.22 9.14 10 10.44 2.25 10.94"></path>
<path d="M21.75 12.84c-6.62-1.41-12.14 1-16.38 6.32"></path>
<path d="M8.56 2.75c4.37 6 6 9.42 8 17.72"></path>"#,
};

pub const DROPLET: IconType = IconType{ 
 content: r#"
<path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z"></path>"#,
};

pub const DROPLETS: IconType = IconType{ 
 content: r#"
<path d="M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z"></path>
<path d="M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97"></path>"#,
};

pub const DRUMSTICK: IconType = IconType{ 
 content: r#"
<path d="M15.45 15.4c-2.13.65-4.3.32-5.7-1.1-2.29-2.27-1.76-6.5 1.17-9.42 2.93-2.93 7.15-3.46 9.43-1.18 1.41 1.41 1.74 3.57 1.1 5.71-1.4-.51-3.26-.02-4.64 1.36-1.38 1.38-1.87 3.23-1.36 4.63z"></path>
<path d="m11.25 15.6-2.16 2.16a2.5 2.5 0 1 1-4.56 1.73 2.49 2.49 0 0 1-1.41-4.24 2.5 2.5 0 0 1 3.14-.32l2.16-2.16"></path>"#,
};

pub const DUMBBELL: IconType = IconType{ 
 content: r#"
<path d="m6.5 6.5 11 11"></path>
<path d="m21 21-1-1"></path>
<path d="m3 3 1 1"></path>
<path d="m18 22 4-4"></path>
<path d="m2 6 4-4"></path>
<path d="m3 10 7-7"></path>
<path d="m14 21 7-7"></path>"#,
};

pub const EAR_OFF: IconType = IconType{ 
 content: r#"
<path d="M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46"></path>
<path d="M6 8.5c0-.75.13-1.47.36-2.14"></path>
<path d="M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76"></path>
<path d="M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18"></path>
<line x2="22" x1="2" y1="2" y2="22"></line>"#,
};

pub const EAR: IconType = IconType{ 
 content: r#"
<path d="M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0"></path>
<path d="M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4"></path>"#,
};

pub const EGG_FRIED: IconType = IconType{ 
 content: r#"
<circle cx="11.5" cy="12.5" r="3.5"></circle>
<path d="M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z"></path>"#,
};

pub const EGG_OFF: IconType = IconType{ 
 content: r#"
<path d="M6.399 6.399C5.362 8.157 4.65 10.189 4.5 12c-.37 4.43 1.27 9.95 7.5 10 3.256-.026 5.259-1.547 6.375-3.625"></path>
<path d="M19.532 13.875A14.07 14.07 0 0 0 19.5 12c-.36-4.34-3.95-9.96-7.5-10-1.04.012-2.082.502-3.046 1.297"></path>
<line x1="2" x2="22" y1="2" y2="22"></line>"#,
};

pub const EGG: IconType = IconType{ 
 content: r#"
<path d="M12 22c6.23-.05 7.87-5.57 7.5-10-.36-4.34-3.95-9.96-7.5-10-3.55.04-7.14 5.66-7.5 10-.37 4.43 1.27 9.95 7.5 10z"></path>"#,
};

pub const EQUAL_NOT: IconType = IconType{ 
 content: r#"
<line x2="19" y2="9" x1="5" y1="9"></line>
<line x1="5" y1="15" y2="15" x2="19"></line>
<line y1="5" y2="19" x2="5" x1="19"></line>"#,
};

pub const EQUAL: IconType = IconType{ 
 content: r#"
<line y1="9" y2="9" x1="5" x2="19"></line>
<line x1="5" x2="19" y2="15" y1="15"></line>"#,
};

pub const ERASER: IconType = IconType{ 
 content: r#"
<path d="m7 21-4.3-4.3c-1-1-1-2.5 0-3.4l9.6-9.6c1-1 2.5-1 3.4 0l5.6 5.6c1 1 1 2.5 0 3.4L13 21"></path>
<path d="M22 21H7"></path>
<path d="m5 11 9 9"></path>"#,
};

pub const EURO: IconType = IconType{ 
 content: r#"
<path d="M4 10h12"></path>
<path d="M4 14h9"></path>
<path d="M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2"></path>"#,
};

pub const EXPAND: IconType = IconType{ 
 content: r#"
<path d="m21 21-6-6m6 6v-4.8m0 4.8h-4.8"></path>
<path d="M3 16.2V21m0 0h4.8M3 21l6-6"></path>
<path d="M21 7.8V3m0 0h-4.8M21 3l-6 6"></path>
<path d="M3 7.8V3m0 0h4.8M3 3l6 6"></path>"#,
};

pub const EXTERNAL_LINK: IconType = IconType{ 
 content: r#"
<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
<polyline points="15 3 21 3 21 9"></polyline>
<line x2="21" y2="3" x1="10" y1="14"></line>"#,
};

pub const EYE_OFF: IconType = IconType{ 
 content: r#"
<path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path>
<path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"></path>
<path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"></path>
<line x1="2" y2="22" x2="22" y1="2"></line>"#,
};

pub const EYE: IconType = IconType{ 
 content: r#"
<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
<circle r="3" cx="12" cy="12"></circle>"#,
};

pub const FACEBOOK: IconType = IconType{ 
 content: r#"
<path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>"#,
};

pub const FACTORY: IconType = IconType{ 
 content: r#"
<path d="M2 20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8l-7 5V8l-7 5V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"></path>
<path d="M17 18h1"></path>
<path d="M12 18h1"></path>
<path d="M7 18h1"></path>"#,
};

pub const FAN: IconType = IconType{ 
 content: r#"
<path d="M10.827 16.379a6.082 6.082 0 0 1-8.618-7.002l5.412 1.45a6.082 6.082 0 0 1 7.002-8.618l-1.45 5.412a6.082 6.082 0 0 1 8.618 7.002l-5.412-1.45a6.082 6.082 0 0 1-7.002 8.618l1.45-5.412Z"></path>
<path d="M12 12v.01"></path>"#,
};

pub const FAST_FORWARD: IconType = IconType{ 
 content: r#"
<polygon points="13 19 22 12 13 5 13 19"></polygon>
<polygon points="2 19 11 12 2 5 2 19"></polygon>"#,
};

pub const FEATHER: IconType = IconType{ 
 content: r#"
<path d="M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z"></path>
<line x1="16" x2="2" y1="8" y2="22"></line>
<line x2="9" y2="15" y1="15" x1="17.5"></line>"#,
};

pub const FERRIS_WHEEL: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="2"></circle>
<path d="M12 2v4"></path>
<path d="m6.8 15-3.5 2"></path>
<path d="m20.7 7-3.5 2"></path>
<path d="M6.8 9 3.3 7"></path>
<path d="m20.7 17-3.5-2"></path>
<path d="m9 22 3-8 3 8"></path>
<path d="M8 22h8"></path>
<path d="M18 18.7a9 9 0 1 0-12 0"></path>"#,
};

pub const FIGMA: IconType = IconType{ 
 content: r#"
<path d="M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z"></path>
<path d="M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z"></path>
<path d="M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z"></path>
<path d="M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z"></path>
<path d="M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z"></path>"#,
};

pub const FILE_ARCHIVE: IconType = IconType{ 
 content: r#"
<path d="M4 22V4c0-.5.2-1 .6-1.4C5 2.2 5.5 2 6 2h8.5L20 7.5V20c0 .5-.2 1-.6 1.4-.4.4-.9.6-1.4.6h-2"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cy="20" r="2" cx="10"></circle>
<path d="M10 7V6"></path>
<path d="M10 12v-1"></path>
<path d="M10 18v-2"></path>"#,
};

pub const FILE_AUDIO_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v2"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 17v-3a4 4 0 0 1 8 0v3"></path>
<circle cx="9" cy="17" r="1"></circle>
<circle cy="17" r="1" cx="3"></circle>"#,
};

pub const FILE_AUDIO: IconType = IconType{ 
 content: r#"
<path d="M17.5 22h.5c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10 20v-1a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0Z"></path>
<path d="M6 20v-1a2 2 0 1 0-4 0v1a2 2 0 1 0 4 0Z"></path>
<path d="M2 19v-3a6 6 0 0 1 12 0v3"></path>"#,
};

pub const FILE_AXIS_3_D: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M8 10v8h8"></path>
<path d="m8 18 4-4"></path>"#,
};

pub const FILE_BADGE_2: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"></path>
<path d="m14 12.5 1 5.5-3-1-3 1 1-5.5"></path>"#,
};

pub const FILE_BADGE: IconType = IconType{ 
 content: r#"
<path d="M4 7V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-6"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"></path>
<path d="M7 16.5 8 22l-3-1-3 1 1-5.5"></path>"#,
};

pub const FILE_BAR_CHART_2: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-6"></path>
<path d="M8 18v-1"></path>
<path d="M16 18v-3"></path>"#,
};

pub const FILE_BAR_CHART: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-4"></path>
<path d="M8 18v-2"></path>
<path d="M16 18v-6"></path>"#,
};

pub const FILE_BOX: IconType = IconType{ 
 content: r#"
<path d="M14.5 22H18a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2.97 13.12c-.6.36-.97 1.02-.97 1.74v3.28c0 .72.37 1.38.97 1.74l3 1.83c.63.39 1.43.39 2.06 0l3-1.83c.6-.36.97-1.02.97-1.74v-3.28c0-.72-.37-1.38-.97-1.74l-3-1.83a1.97 1.97 0 0 0-2.06 0l-3 1.83Z"></path>
<path d="m7 17-4.74-2.85"></path>
<path d="m7 17 4.74-2.85"></path>
<path d="M7 17v5"></path>"#,
};

pub const FILE_CHECK_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m3 15 2 2 4-4"></path>"#,
};

pub const FILE_CHECK: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m9 15 2 2 4-4"></path>"#,
};

pub const FILE_CLOCK: IconType = IconType{ 
 content: r#"
<path d="M16 22h2c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cy="16" r="6" cx="8"></circle>
<path d="M9.5 17.5 8 16.25V14"></path>"#,
};

pub const FILE_CODE_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m9 18 3-3-3-3"></path>
<path d="m5 12-3 3 3 3"></path>"#,
};

pub const FILE_CODE: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 13-2 2 2 2"></path>
<path d="m14 17 2-2-2-2"></path>"#,
};

pub const FILE_COG: IconType = IconType{ 
 content: r#"
<circle cx="6" cy="13" r="3"></circle>
<path d="m9.7 14.4-.9-.3"></path>
<path d="m3.2 11.9-.9-.3"></path>
<path d="m4.6 16.7.3-.9"></path>
<path d="m7.6 16.7-.4-1"></path>
<path d="m4.8 10.3-.4-1"></path>
<path d="m2.3 14.6 1-.4"></path>
<path d="m8.7 11.8 1-.4"></path>
<path d="m7.4 9.3-.3.9"></path>
<path d="M14 2v6h6"></path>
<path d="M4 5.5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-1.5"></path>"#,
};

pub const FILE_DIFF: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 13V7"></path>
<path d="M9 10h6"></path>
<path d="M9 17h6"></path>"#,
};

pub const FILE_DIGIT: IconType = IconType{ 
 content: r#"
<rect width="4" y="12" rx="2" height="6" x="2"></rect>
<path d="M14 2v6h6"></path>
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<path d="M10 12h2v6"></path>
<path d="M10 18h4"></path>"#,
};

pub const FILE_DOWN: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 18v-6"></path>
<path d="m9 15 3 3 3-3"></path>"#,
};

pub const FILE_EDIT: IconType = IconType{ 
 content: r#"
<path d="M4 13.5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-5.5"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z"></path>"#,
};

pub const FILE_HEART: IconType = IconType{ 
 content: r#"
<path d="M4 6V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10.29 10.7a2.43 2.43 0 0 0-2.66-.52c-.29.12-.56.3-.78.53l-.35.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L6.5 18l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#,
};

pub const FILE_IMAGE: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cy="13" cx="10" r="2"></circle>
<path d="m20 17-1.09-1.09a2 2 0 0 0-2.82 0L10 22"></path>"#,
};

pub const FILE_INPUT: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 15h10"></path>
<path d="m9 18 3-3-3-3"></path>"#,
};

pub const FILE_JSON_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M4 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1"></path>
<path d="M8 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"></path>"#,
};

pub const FILE_JSON: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1"></path>
<path d="M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"></path>"#,
};

pub const FILE_KEY_2: IconType = IconType{ 
 content: r#"
<path d="M4 10V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cx="4" r="2" cy="16"></circle>
<path d="m10 10-4.5 4.5"></path>
<path d="m9 11 1 1"></path>"#,
};

pub const FILE_KEY: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<circle r="2" cx="10" cy="16"></circle>
<path d="m16 10-4.5 4.5"></path>
<path d="m15 11 1 1"></path>"#,
};

pub const FILE_LINE_CHART: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m16 13-3.5 3.5-2-2L8 17"></path>"#,
};

pub const FILE_LOCK_2: IconType = IconType{ 
 content: r#"
<path d="M4 5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<rect x="2" rx="1" y="13" width="8" height="5"></rect>
<path d="M8 13v-2a2 2 0 1 0-4 0v2"></path>"#,
};

pub const FILE_LOCK: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<rect x="8" width="8" rx="1" height="6" y="12"></rect>
<path d="M15 12v-2a3 3 0 1 0-6 0v2"></path>"#,
};

pub const FILE_MINUS_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M3 15h6"></path>"#,
};

pub const FILE_MINUS: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line y1="15" y2="15" x1="9" x2="15"></line>"#,
};

pub const FILE_OUTPUT: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 15h10"></path>
<path d="m5 12-3 3 3 3"></path>"#,
};

pub const FILE_PIE_CHART: IconType = IconType{ 
 content: r#"
<path d="M16 22h2a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M4.04 11.71a5.84 5.84 0 1 0 8.2 8.29"></path>
<path d="M13.83 16A5.83 5.83 0 0 0 8 10.17V16h5.83Z"></path>"#,
};

pub const FILE_PLUS_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M3 15h6"></path>
<path d="M6 12v6"></path>"#,
};

pub const FILE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line x2="12" y1="18" y2="12" x1="12"></line>
<line y1="15" y2="15" x1="9" x2="15"></line>"#,
};

pub const FILE_QUESTION: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M10 10.3c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"></path>
<path d="M12 17h.01"></path>"#,
};

pub const FILE_SCAN: IconType = IconType{ 
 content: r#"
<path d="M20 10V7.5L14.5 2H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h4.5"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M16 22a2 2 0 0 1-2-2"></path>
<path d="M20 22a2 2 0 0 0 2-2"></path>
<path d="M20 14a2 2 0 0 1 2 2"></path>
<path d="M16 14a2 2 0 0 0-2 2"></path>"#,
};

pub const FILE_SEARCH_2: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<circle cx="11.5" r="2.5" cy="14.5"></circle>
<path d="M13.25 16.25 15 18"></path>"#,
};

pub const FILE_SEARCH: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="m9 18-1.5-1.5"></path>"#,
};

pub const FILE_SIGNATURE: IconType = IconType{ 
 content: r#"
<path d="M20 19.5v.5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8.5L18 5.5"></path>
<path d="M8 18h1"></path>
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z"></path>"#,
};

pub const FILE_SPREADSHEET: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M8 13h2"></path>
<path d="M8 17h2"></path>
<path d="M14 13h2"></path>
<path d="M14 17h2"></path>"#,
};

pub const FILE_STACK: IconType = IconType{ 
 content: r#"
<path d="M16 2v5h5"></path>
<path d="M21 6v6.5c0 .8-.7 1.5-1.5 1.5h-7c-.8 0-1.5-.7-1.5-1.5v-9c0-.8.7-1.5 1.5-1.5H17l4 4z"></path>
<path d="M7 8v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H15"></path>
<path d="M3 12v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H11"></path>"#,
};

pub const FILE_SYMLINK: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v7"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 18 3-3-3-3"></path>
<path d="M4 18v-1a2 2 0 0 1 2-2h6"></path>"#,
};

pub const FILE_TERMINAL: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m8 16 2-2-2-2"></path>
<path d="M12 18h4"></path>"#,
};

pub const FILE_TEXT: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line y1="13" y2="13" x2="8" x1="16"></line>
<line x2="8" y1="17" x1="16" y2="17"></line>
<line x2="8" y2="9" x1="10" y1="9"></line>"#,
};

pub const FILE_TYPE_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M2 13v-1h6v1"></path>
<path d="M4 18h2"></path>
<path d="M5 12v6"></path>"#,
};

pub const FILE_TYPE: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M9 13v-1h6v1"></path>
<path d="M11 18h2"></path>
<path d="M12 12v6"></path>"#,
};

pub const FILE_UP: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M12 12v6"></path>
<path d="m15 15-3-3-3 3"></path>"#,
};

pub const FILE_VIDEO_2: IconType = IconType{ 
 content: r#"
<path d="M4 8V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 15.5 4 2.5v-6l-4 2.5"></path>
<rect y="12" height="6" width="8" x="2" rx="1"></rect>"#,
};

pub const FILE_VIDEO: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m10 11 5 3-5 3v-6Z"></path>"#,
};

pub const FILE_VOLUME_2: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="M11.5 13.5c.32.4.5.94.5 1.5s-.18 1.1-.5 1.5"></path>
<path d="M15 12c.64.8 1 1.87 1 3s-.36 2.2-1 3"></path>
<path d="M8 15h.01"></path>"#,
};

pub const FILE_VOLUME: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<path d="m7 10-3 2H2v4h2l3 2v-8Z"></path>
<path d="M11 11c.64.8 1 1.87 1 3s-.36 2.2-1 3"></path>"#,
};

pub const FILE_WARNING: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<path d="M12 9v4"></path>
<path d="M12 17h.01"></path>"#,
};

pub const FILE_X_2: IconType = IconType{ 
 content: r#"
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"></path>
<path d="M14 2v6h6"></path>
<path d="m3 12.5 5 5"></path>
<path d="m8 12.5-5 5"></path>"#,
};

pub const FILE_X: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>
<line x1="9.5" y1="12.5" x2="14.5" y2="17.5"></line>
<line y1="12.5" x1="14.5" x2="9.5" y2="17.5"></line>"#,
};

pub const FILE: IconType = IconType{ 
 content: r#"
<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
<polyline points="14 2 14 8 20 8"></polyline>"#,
};

pub const FILES: IconType = IconType{ 
 content: r#"
<path d="M15.5 2H8.6c-.4 0-.8.2-1.1.5-.3.3-.5.7-.5 1.1v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8c.4 0 .8-.2 1.1-.5.3-.3.5-.7.5-1.1V6.5L15.5 2z"></path>
<path d="M3 7.6v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8"></path>
<path d="M15 2v5h5"></path>"#,
};

pub const FILM: IconType = IconType{ 
 content: r#"
<rect y="3" x="3" rx="2" width="18" height="18"></rect>
<path d="M7 3v18"></path>
<path d="M3 7.5h4"></path>
<path d="M3 12h18"></path>
<path d="M3 16.5h4"></path>
<path d="M17 3v18"></path>
<path d="M17 7.5h4"></path>
<path d="M17 16.5h4"></path>"#,
};

pub const FILTER_X: IconType = IconType{ 
 content: r#"
<path d="M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055"></path>
<path d="m22 3-5 5"></path>
<path d="m17 3 5 5"></path>"#,
};

pub const FILTER: IconType = IconType{ 
 content: r#"
<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>"#,
};

pub const FINGERPRINT: IconType = IconType{ 
 content: r#"
<path d="M2 12C2 6.5 6.5 2 12 2a10 10 0 0 1 8 4"></path>
<path d="M5 19.5C5.5 18 6 15 6 12c0-.7.12-1.37.34-2"></path>
<path d="M17.29 21.02c.12-.6.43-2.3.5-3.02"></path>
<path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4"></path>
<path d="M8.65 22c.21-.66.45-1.32.57-2"></path>
<path d="M14 13.12c0 2.38 0 6.38-1 8.88"></path>
<path d="M2 16h.01"></path>
<path d="M21.8 16c.2-2 .131-5.354 0-6"></path>
<path d="M9 6.8a6 6 0 0 1 9 5.2c0 .47 0 1.17-.02 2"></path>"#,
};

pub const FISH_OFF: IconType = IconType{ 
 content: r#"
<path d="M18 12.47v.03m0-.5v.47m-.475 5.056A6.744 6.744 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.114-2.433 2.121-3.38m3.444-2.088A8.802 8.802 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.177-1.413 3.058"></path>
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.372A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.728-3.618"></path>
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20"></path>"#,
};

pub const FISH_SYMBOL: IconType = IconType{ 
 content: r#"
<path d="M2 16s9-15 20-4C11 23 2 8 2 8"></path>"#,
};

pub const FISH: IconType = IconType{ 
 content: r#"
<path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z"></path>
<path d="M18 12v.5"></path>
<path d="M16 17.93a9.77 9.77 0 0 1 0-11.86"></path>
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33"></path>
<path d="M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4"></path>
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98"></path>"#,
};

pub const FLAG_OFF: IconType = IconType{ 
 content: r#"
<path d="M8 2c3 0 5 2 8 2s4-1 4-1v11"></path>
<path d="M4 22V4"></path>
<path d="M4 15s1-1 4-1 5 2 8 2"></path>
<line y2="22" x1="2" y1="2" x2="22"></line>"#,
};

pub const FLAG_TRIANGLE_LEFT: IconType = IconType{ 
 content: r#"
<path d="M17 22V2L7 7l10 5"></path>"#,
};

pub const FLAG_TRIANGLE_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M7 22V2l10 5-10 5"></path>"#,
};

pub const FLAG: IconType = IconType{ 
 content: r#"
<path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"></path>
<line x1="4" y1="22" x2="4" y2="15"></line>"#,
};

pub const FLAME: IconType = IconType{ 
 content: r#"
<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"></path>"#,
};

pub const FLASHLIGHT_OFF: IconType = IconType{ 
 content: r#"
<path d="M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4"></path>
<path d="M7 2h11v4c0 2-2 2-2 4v1"></path>
<line x1="11" y2="6" x2="18" y1="6"></line>
<line x1="2" y1="2" x2="22" y2="22"></line>"#,
};

pub const FLASHLIGHT: IconType = IconType{ 
 content: r#"
<path d="M18 6c0 2-2 2-2 4v10a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4V2h12z"></path>
<line y2="6" y1="6" x1="6" x2="18"></line>
<line y1="12" x1="12" x2="12" y2="12"></line>"#,
};

pub const FLASK_CONICAL_OFF: IconType = IconType{ 
 content: r#"
<path d="M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542"></path>
<path d="M10 2v2.343"></path>
<path d="M14 2v6.343"></path>
<path d="M8.5 2h7"></path>
<path d="M7 16h9"></path>
<line x1="2" y2="22" x2="22" y1="2"></line>"#,
};

pub const FLASK_CONICAL: IconType = IconType{ 
 content: r#"
<path d="M10 2v7.527a2 2 0 0 1-.211.896L4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-5.069-10.127A2 2 0 0 1 14 9.527V2"></path>
<path d="M8.5 2h7"></path>
<path d="M7 16h10"></path>"#,
};

pub const FLASK_ROUND: IconType = IconType{ 
 content: r#"
<path d="M10 2v7.31"></path>
<path d="M14 9.3V1.99"></path>
<path d="M8.5 2h7"></path>
<path d="M14 9.3a6.5 6.5 0 1 1-4 0"></path>
<path d="M5.52 16h12.96"></path>"#,
};

pub const FLIP_HORIZONTAL_2: IconType = IconType{ 
 content: r#"
<path d="m3 7 5 5-5 5V7"></path>
<path d="m21 7-5 5 5 5V7"></path>
<path d="M12 20v2"></path>
<path d="M12 14v2"></path>
<path d="M12 8v2"></path>
<path d="M12 2v2"></path>"#,
};

pub const FLIP_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3"></path>
<path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3"></path>
<path d="M12 20v2"></path>
<path d="M12 14v2"></path>
<path d="M12 8v2"></path>
<path d="M12 2v2"></path>"#,
};

pub const FLIP_VERTICAL_2: IconType = IconType{ 
 content: r#"
<path d="m17 3-5 5-5-5h10"></path>
<path d="m17 21-5-5-5 5h10"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#,
};

pub const FLIP_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3"></path>
<path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#,
};

pub const FLOWER_2: IconType = IconType{ 
 content: r#"
<path d="M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1"></path>
<circle cx="12" r="2" cy="8"></circle>
<path d="M12 10v12"></path>
<path d="M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z"></path>
<path d="M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z"></path>"#,
};

pub const FLOWER: IconType = IconType{ 
 content: r#"
<path d="M12 7.5a4.5 4.5 0 1 1 4.5 4.5M12 7.5A4.5 4.5 0 1 0 7.5 12M12 7.5V9m-4.5 3a4.5 4.5 0 1 0 4.5 4.5M7.5 12H9m7.5 0a4.5 4.5 0 1 1-4.5 4.5m4.5-4.5H15m-3 4.5V15"></path>
<circle cy="12" cx="12" r="3"></circle>
<path d="m8 16 1.5-1.5"></path>
<path d="M14.5 9.5 16 8"></path>
<path d="m8 8 1.5 1.5"></path>
<path d="M14.5 14.5 16 16"></path>"#,
};

pub const FOCUS: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="3"></circle>
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>"#,
};

pub const FOLD_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M2 12h6"></path>
<path d="M22 12h-6"></path>
<path d="M12 2v2"></path>
<path d="M12 8v2"></path>
<path d="M12 14v2"></path>
<path d="M12 20v2"></path>
<path d="m19 9-3 3 3 3"></path>
<path d="m5 15 3-3-3-3"></path>"#,
};

pub const FOLD_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M12 22v-6"></path>
<path d="M12 8V2"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>
<path d="m15 19-3-3-3 3"></path>
<path d="m15 5-3 3-3-3"></path>"#,
};

pub const FOLDER_ARCHIVE: IconType = IconType{ 
 content: r#"
<path d="M22 20V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2h6"></path>
<circle cx="16" cy="19" r="2"></circle>
<path d="M16 11v-1"></path>
<path d="M16 17v-2"></path>"#,
};

pub const FOLDER_CHECK: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="m9 13 2 2 4-4"></path>"#,
};

pub const FOLDER_CLOCK: IconType = IconType{ 
 content: r#"
<path d="M7 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2"></path>
<circle cx="16" r="6" cy="16"></circle>
<path d="M16 14v2l1 1"></path>"#,
};

pub const FOLDER_CLOSED: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M2 10h20"></path>"#,
};

pub const FOLDER_COG: IconType = IconType{ 
 content: r#"
<circle r="3" cx="18" cy="18"></circle>
<path d="M10.5 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.5"></path>
<path d="m21.7 19.4-.9-.3"></path>
<path d="m15.2 16.9-.9-.3"></path>
<path d="m16.6 21.7.3-.9"></path>
<path d="m19.1 15.2.3-.9"></path>
<path d="m19.6 21.7-.4-1"></path>
<path d="m16.8 15.3-.4-1"></path>
<path d="m14.3 19.6 1-.4"></path>
<path d="m20.7 16.8 1-.4"></path>"#,
};

pub const FOLDER_DOT: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cy="13" cx="12" r="1"></circle>"#,
};

pub const FOLDER_DOWN: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M12 10v6"></path>
<path d="m15 13-3 3-3-3"></path>"#,
};

pub const FOLDER_EDIT: IconType = IconType{ 
 content: r#"
<path d="M8.42 10.61a2.1 2.1 0 1 1 2.97 2.97L5.95 19 2 20l.99-3.95 5.43-5.44Z"></path>
<path d="M2 11.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5"></path>"#,
};

pub const FOLDER_GIT_2: IconType = IconType{ 
 content: r#"
<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5"></path>
<circle r="2" cy="12" cx="13"></circle>
<path d="M18 19c-2.8 0-5-2.2-5-5v8"></path>
<circle cy="19" r="2" cx="20"></circle>"#,
};

pub const FOLDER_GIT: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cy="13" r="2" cx="12"></circle>
<path d="M14 13h3"></path>
<path d="M7 13h3"></path>"#,
};

pub const FOLDER_HEART: IconType = IconType{ 
 content: r#"
<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v1.5"></path>
<path d="M21.29 13.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 21l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z"></path>"#,
};

pub const FOLDER_INPUT: IconType = IconType{ 
 content: r#"
<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1"></path>
<path d="M2 13h10"></path>
<path d="m9 16 3-3-3-3"></path>"#,
};

pub const FOLDER_KANBAN: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M8 10v4"></path>
<path d="M12 10v2"></path>
<path d="M16 10v6"></path>"#,
};

pub const FOLDER_KEY: IconType = IconType{ 
 content: r#"
<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2"></path>
<circle r="2" cy="20" cx="16"></circle>
<path d="m22 14-4.5 4.5"></path>
<path d="m21 15 1 1"></path>"#,
};

pub const FOLDER_LOCK: IconType = IconType{ 
 content: r#"
<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2.5"></path>
<rect width="8" y="17" rx="1" x="14" height="5"></rect>
<path d="M20 17v-2a2 2 0 1 0-4 0v2"></path>"#,
};

pub const FOLDER_MINUS: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<line y1="13" y2="13" x2="15" x1="9"></line>"#,
};

pub const FOLDER_OPEN_DOT: IconType = IconType{ 
 content: r#"
<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2"></path>
<circle cy="15" r="1" cx="14"></circle>"#,
};

pub const FOLDER_OPEN: IconType = IconType{ 
 content: r#"
<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2"></path>"#,
};

pub const FOLDER_OUTPUT: IconType = IconType{ 
 content: r#"
<path d="M2 7.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2"></path>
<path d="M2 13h10"></path>
<path d="m5 10-3 3 3 3"></path>"#,
};

pub const FOLDER_PLUS: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<line y1="10" x1="12" x2="12" y2="16"></line>
<line x2="15" y2="13" x1="9" y1="13"></line>"#,
};

pub const FOLDER_ROOT: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cy="13" r="2" cx="12"></circle>
<path d="M12 15v5"></path>"#,
};

pub const FOLDER_SEARCH_2: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<circle cy="12.5" cx="11.5" r="2.5"></circle>
<path d="M13.27 14.27 15 16"></path>"#,
};

pub const FOLDER_SEARCH: IconType = IconType{ 
 content: r#"
<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v4"></path>
<circle cy="17" cx="17" r="3"></circle>
<path d="m21 21-1.5-1.5"></path>"#,
};

pub const FOLDER_SYMLINK: IconType = IconType{ 
 content: r#"
<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2"></path>
<path d="m8 16 3-3-3-3"></path>
<path d="M2 16v-1a2 2 0 0 1 2-2h6"></path>"#,
};

pub const FOLDER_SYNC: IconType = IconType{ 
 content: r#"
<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1"></path>
<path d="M12 10v4h4"></path>
<path d="m12 14 1.5-1.5c.9-.9 2.2-1.5 3.5-1.5s2.6.6 3.5 1.5c.4.4.8 1 1 1.5"></path>
<path d="M22 22v-4h-4"></path>
<path d="m22 18-1.5 1.5c-.9.9-2.1 1.5-3.5 1.5s-2.6-.6-3.5-1.5c-.4-.4-.8-1-1-1.5"></path>"#,
};

pub const FOLDER_TREE: IconType = IconType{ 
 content: r#"
<path d="M13 10h7a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"></path>
<path d="M13 21h7a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.88a1 1 0 0 1-.9-.55l-.44-.9a1 1 0 0 0-.9-.55H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"></path>
<path d="M3 3v2c0 1.1.9 2 2 2h3"></path>
<path d="M3 3v13c0 1.1.9 2 2 2h3"></path>"#,
};

pub const FOLDER_UP: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="M12 10v6"></path>
<path d="m9 13 3-3 3 3"></path>"#,
};

pub const FOLDER_X: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
<path d="m9.5 10.5 5 5"></path>
<path d="m14.5 10.5-5 5"></path>"#,
};

pub const FOLDER: IconType = IconType{ 
 content: r#"
<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>"#,
};

pub const FOLDERS: IconType = IconType{ 
 content: r#"
<path d="M8 17h12a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3.93a2 2 0 0 1-1.66-.9l-.82-1.2a2 2 0 0 0-1.66-.9H8a2 2 0 0 0-2 2v9c0 1.1.9 2 2 2Z"></path>
<path d="M2 8v11c0 1.1.9 2 2 2h14"></path>"#,
};

pub const FOOTPRINTS: IconType = IconType{ 
 content: r#"
<path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z"></path>
<path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z"></path>
<path d="M16 17h4"></path>
<path d="M4 13h4"></path>"#,
};

pub const FORKLIFT: IconType = IconType{ 
 content: r#"
<path d="M12 12H5a2 2 0 0 0-2 2v5"></path>
<circle cx="13" cy="19" r="2"></circle>
<circle cx="5" cy="19" r="2"></circle>
<path d="M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5"></path>"#,
};

pub const FORM_INPUT: IconType = IconType{ 
 content: r#"
<rect rx="2" y="6" x="2" height="12" width="20"></rect>
<path d="M12 12h.01"></path>
<path d="M17 12h.01"></path>
<path d="M7 12h.01"></path>"#,
};

pub const FORWARD: IconType = IconType{ 
 content: r#"
<polyline points="15 17 20 12 15 7"></polyline>
<path d="M4 18v-2a4 4 0 0 1 4-4h12"></path>"#,
};

pub const FRAME: IconType = IconType{ 
 content: r#"
<line y2="6" x1="22" x2="2" y1="6"></line>
<line y1="18" x1="22" x2="2" y2="18"></line>
<line y2="22" x1="6" x2="6" y1="2"></line>
<line y1="2" x1="18" y2="22" x2="18"></line>"#,
};

pub const FRAMER: IconType = IconType{ 
 content: r#"
<path d="M5 16V9h14V2H5l14 14h-7m-7 0 7 7v-7m-7 0h7"></path>"#,
};

pub const FROWN: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M16 16s-1.5-2-4-2-4 2-4 2"></path>
<line y2="9" y1="9" x1="9" x2="9.01"></line>
<line x1="15" y1="9" x2="15.01" y2="9"></line>"#,
};

pub const FUEL: IconType = IconType{ 
 content: r#"
<line x2="15" y1="22" y2="22" x1="3"></line>
<line x1="4" x2="14" y1="9" y2="9"></line>
<path d="M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18"></path>
<path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5"></path>"#,
};

pub const FUNCTION_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" width="18" height="18" y="3" rx="2" ry="2"></rect>
<path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3"></path>
<path d="M9 11.2h5.7"></path>"#,
};

pub const GALLERY_HORIZONTAL_END: IconType = IconType{ 
 content: r#"
<path d="M2 7v10"></path>
<path d="M6 5v14"></path>
<rect rx="2" width="12" y="3" height="18" x="10"></rect>"#,
};

pub const GALLERY_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M2 3v18"></path>
<rect height="18" width="12" x="6" y="3" rx="2"></rect>
<path d="M22 3v18"></path>"#,
};

pub const GALLERY_THUMBNAILS: IconType = IconType{ 
 content: r#"
<rect rx="2" y="3" width="18" x="3" height="14"></rect>
<path d="M4 21h1"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>
<path d="M19 21h1"></path>"#,
};

pub const GALLERY_VERTICAL_END: IconType = IconType{ 
 content: r#"
<path d="M7 2h10"></path>
<path d="M5 6h14"></path>
<rect x="3" height="12" y="10" rx="2" width="18"></rect>"#,
};

pub const GALLERY_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M3 2h18"></path>
<rect rx="2" width="18" y="6" height="12" x="3"></rect>
<path d="M3 22h18"></path>"#,
};

pub const GAMEPAD_2: IconType = IconType{ 
 content: r#"
<line y1="11" y2="11" x1="6" x2="10"></line>
<line y1="9" x2="8" x1="8" y2="13"></line>
<line x2="15.01" x1="15" y2="12" y1="12"></line>
<line y1="10" x2="18.01" y2="10" x1="18"></line>
<path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"></path>"#,
};

pub const GAMEPAD: IconType = IconType{ 
 content: r#"
<line x1="6" y2="12" x2="10" y1="12"></line>
<line x1="8" x2="8" y1="10" y2="14"></line>
<line x2="15.01" x1="15" y1="13" y2="13"></line>
<line y1="11" x1="18" y2="11" x2="18.01"></line>
<rect y="6" width="20" height="12" rx="2" x="2"></rect>"#,
};

pub const GANTT_CHART_SQUARE: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" x="3" rx="2" y="3"></rect>
<path d="M9 8h7"></path>
<path d="M8 12h6"></path>
<path d="M11 16h5"></path>"#,
};

pub const GANTT_CHART: IconType = IconType{ 
 content: r#"
<path d="M8 6h10"></path>
<path d="M6 12h9"></path>
<path d="M11 18h7"></path>"#,
};

pub const GAUGE_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="M15.6 2.7a10 10 0 1 0 5.7 5.7"></path>
<circle cx="12" cy="12" r="2"></circle>
<path d="M13.4 10.6 19 5"></path>"#,
};

pub const GAUGE: IconType = IconType{ 
 content: r#"
<path d="m12 14 4-4"></path>
<path d="M3.34 19a10 10 0 1 1 17.32 0"></path>"#,
};

pub const GAVEL: IconType = IconType{ 
 content: r#"
<path d="m14 13-7.5 7.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L11 10"></path>
<path d="m16 16 6-6"></path>
<path d="m8 8 6-6"></path>
<path d="m9 7 8 8"></path>
<path d="m21 11-8-8"></path>"#,
};

pub const GEM: IconType = IconType{ 
 content: r#"
<path d="M6 3h12l4 6-10 13L2 9Z"></path>
<path d="M11 3 8 9l4 13 4-13-3-6"></path>
<path d="M2 9h20"></path>"#,
};

pub const GHOST: IconType = IconType{ 
 content: r#"
<path d="M9 10h.01"></path>
<path d="M15 10h.01"></path>
<path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z"></path>"#,
};

pub const GIFT: IconType = IconType{ 
 content: r#"
<polyline points="20 12 20 22 4 22 4 12"></polyline>
<rect width="20" height="5" y="7" x="2"></rect>
<line y1="22" y2="7" x1="12" x2="12"></line>
<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>"#,
};

pub const GIT_BRANCH_PLUS: IconType = IconType{ 
 content: r#"
<path d="M6 3v12"></path>
<path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"></path>
<path d="M15 6a9 9 0 0 0-9 9"></path>
<path d="M18 15v6"></path>
<path d="M21 18h-6"></path>"#,
};

pub const GIT_BRANCH: IconType = IconType{ 
 content: r#"
<line y2="15" y1="3" x1="6" x2="6"></line>
<circle r="3" cy="6" cx="18"></circle>
<circle cx="6" r="3" cy="18"></circle>
<path d="M18 9a9 9 0 0 1-9 9"></path>"#,
};

pub const GIT_COMMIT: IconType = IconType{ 
 content: r#"
<circle r="3" cx="12" cy="12"></circle>
<line x2="9" y2="12" y1="12" x1="3"></line>
<line x1="15" x2="21" y1="12" y2="12"></line>"#,
};

pub const GIT_COMPARE: IconType = IconType{ 
 content: r#"
<circle r="3" cx="18" cy="18"></circle>
<circle cx="6" cy="6" r="3"></circle>
<path d="M13 6h3a2 2 0 0 1 2 2v7"></path>
<path d="M11 18H8a2 2 0 0 1-2-2V9"></path>"#,
};

pub const GIT_FORK: IconType = IconType{ 
 content: r#"
<circle cy="18" cx="12" r="3"></circle>
<circle cy="6" cx="6" r="3"></circle>
<circle cy="6" r="3" cx="18"></circle>
<path d="M18 9v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V9"></path>
<path d="M12 12v3"></path>"#,
};

pub const GIT_MERGE: IconType = IconType{ 
 content: r#"
<circle cx="18" r="3" cy="18"></circle>
<circle cx="6" cy="6" r="3"></circle>
<path d="M6 21V9a9 9 0 0 0 9 9"></path>"#,
};

pub const GIT_PULL_REQUEST_CLOSED: IconType = IconType{ 
 content: r#"
<circle cx="18" r="3" cy="18"></circle>
<circle r="3" cx="6" cy="6"></circle>
<path d="M18 11.5V15"></path>
<path d="m21 3-6 6"></path>
<path d="m21 9-6-6"></path>
<line y2="21" x1="6" y1="9" x2="6"></line>"#,
};

pub const GIT_PULL_REQUEST_DRAFT: IconType = IconType{ 
 content: r#"
<circle r="3" cy="18" cx="18"></circle>
<circle cx="6" r="3" cy="6"></circle>
<path d="M18 6V5"></path>
<path d="M18 11v-1"></path>
<line x1="6" x2="6" y1="9" y2="21"></line>"#,
};

pub const GIT_PULL_REQUEST: IconType = IconType{ 
 content: r#"
<circle r="3" cx="18" cy="18"></circle>
<circle r="3" cx="6" cy="6"></circle>
<path d="M13 6h3a2 2 0 0 1 2 2v7"></path>
<line x1="6" y1="9" y2="21" x2="6"></line>"#,
};

pub const GITHUB: IconType = IconType{ 
 content: r#"
<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
<path d="M9 18c-4.51 2-5-2-7-2"></path>"#,
};

pub const GITLAB: IconType = IconType{ 
 content: r#"
<path d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z"></path>"#,
};

pub const GLASS_WATER: IconType = IconType{ 
 content: r#"
<path d="M15.2 22H8.8a2 2 0 0 1-2-1.79L5 3h14l-1.81 17.21A2 2 0 0 1 15.2 22Z"></path>
<path d="M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0"></path>"#,
};

pub const GLASSES: IconType = IconType{ 
 content: r#"
<circle cx="6" cy="15" r="4"></circle>
<circle cy="15" r="4" cx="18"></circle>
<path d="M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2"></path>
<path d="M2.5 13 5 7c.7-1.3 1.4-2 3-2"></path>
<path d="M21.5 13 19 7c-.7-1.3-1.5-2-3-2"></path>"#,
};

pub const GLOBE_2: IconType = IconType{ 
 content: r#"
<path d="M21.54 15H17a2 2 0 0 0-2 2v4.54"></path>
<path d="M7 3.34V5a3 3 0 0 0 3 3v0a2 2 0 0 1 2 2v0c0 1.1.9 2 2 2v0a2 2 0 0 0 2-2v0c0-1.1.9-2 2-2h3.17"></path>
<path d="M11 21.95V18a2 2 0 0 0-2-2v0a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05"></path>
<circle cy="12" r="10" cx="12"></circle>"#,
};

pub const GLOBE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<line x1="2" y1="12" x2="22" y2="12"></line>
<path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>"#,
};

pub const GOAL: IconType = IconType{ 
 content: r#"
<path d="M12 13V2l8 4-8 4"></path>
<path d="M20.55 10.23A9 9 0 1 1 8 4.94"></path>
<path d="M8 10a5 5 0 1 0 8.9 2.02"></path>"#,
};

pub const GRAB: IconType = IconType{ 
 content: r#"
<path d="M18 11.5V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4"></path>
<path d="M14 10V8a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2"></path>
<path d="M10 9.9V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5"></path>
<path d="M6 14v0a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M18 11v0a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0"></path>"#,
};

pub const GRADUATION_CAP: IconType = IconType{ 
 content: r#"
<path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
<path d="M6 12v5c3 3 9 3 12 0v-5"></path>"#,
};

pub const GRAPE: IconType = IconType{ 
 content: r#"
<path d="M22 5V2l-5.89 5.89"></path>
<circle cy="15.89" r="3" cx="16.6"></circle>
<circle r="3" cx="8.11" cy="7.4"></circle>
<circle cx="12.35" cy="11.65" r="3"></circle>
<circle r="3" cy="5.85" cx="13.91"></circle>
<circle r="3" cx="18.15" cy="10.09"></circle>
<circle cx="6.56" r="3" cy="13.2"></circle>
<circle cx="10.8" cy="17.44" r="3"></circle>
<circle cx="5" cy="19" r="3"></circle>"#,
};

pub const GRID_2_X_2: IconType = IconType{ 
 content: r#"
<rect x="3" y="3" height="18" width="18" rx="2"></rect>
<path d="M3 12h18"></path>
<path d="M12 3v18"></path>"#,
};

pub const GRID_3_X_3: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" width="18" y="3" rx="2"></rect>
<path d="M3 9h18"></path>
<path d="M3 15h18"></path>
<path d="M9 3v18"></path>
<path d="M15 3v18"></path>"#,
};

pub const GRIP_HORIZONTAL: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="9" r="1"></circle>
<circle cy="9" r="1" cx="19"></circle>
<circle cx="5" r="1" cy="9"></circle>
<circle cy="15" cx="12" r="1"></circle>
<circle r="1" cx="19" cy="15"></circle>
<circle cy="15" r="1" cx="5"></circle>"#,
};

pub const GRIP_VERTICAL: IconType = IconType{ 
 content: r#"
<circle cx="9" cy="12" r="1"></circle>
<circle cx="9" cy="5" r="1"></circle>
<circle cx="9" cy="19" r="1"></circle>
<circle cy="12" r="1" cx="15"></circle>
<circle cy="5" cx="15" r="1"></circle>
<circle cx="15" cy="19" r="1"></circle>"#,
};

pub const GRIP: IconType = IconType{ 
 content: r#"
<circle r="1" cx="12" cy="5"></circle>
<circle cx="19" r="1" cy="5"></circle>
<circle r="1" cx="5" cy="5"></circle>
<circle cy="12" cx="12" r="1"></circle>
<circle r="1" cx="19" cy="12"></circle>
<circle cx="5" r="1" cy="12"></circle>
<circle r="1" cx="12" cy="19"></circle>
<circle r="1" cx="19" cy="19"></circle>
<circle cy="19" cx="5" r="1"></circle>"#,
};

pub const GROUP: IconType = IconType{ 
 content: r#"
<path d="M3 7V5c0-1.1.9-2 2-2h2"></path>
<path d="M17 3h2c1.1 0 2 .9 2 2v2"></path>
<path d="M21 17v2c0 1.1-.9 2-2 2h-2"></path>
<path d="M7 21H5c-1.1 0-2-.9-2-2v-2"></path>
<rect height="5" y="7" width="7" x="7" rx="1"></rect>
<rect rx="1" x="10" width="7" height="5" y="12"></rect>"#,
};

pub const HAMMER: IconType = IconType{ 
 content: r#"
<path d="m15 12-8.5 8.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L12 9"></path>
<path d="M17.64 15 22 10.64"></path>
<path d="m20.91 11.7-1.25-1.25c-.6-.6-.93-1.4-.93-2.25v-.86L16.01 4.6a5.56 5.56 0 0 0-3.94-1.64H9l.92.82A6.18 6.18 0 0 1 12 8.4v1.56l2 2h2.47l2.26 1.91"></path>"#,
};

pub const HAND_METAL: IconType = IconType{ 
 content: r#"
<path d="M18 12.5V10a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4"></path>
<path d="M14 11V9a2 2 0 1 0-4 0v2"></path>
<path d="M10 10.5V5a2 2 0 1 0-4 0v9"></path>
<path d="m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5"></path>"#,
};

pub const HAND: IconType = IconType{ 
 content: r#"
<path d="M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2"></path>
<path d="M10 10.5V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v8"></path>
<path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"></path>"#,
};

pub const HARD_DRIVE_DOWNLOAD: IconType = IconType{ 
 content: r#"
<path d="M12 2v8"></path>
<path d="m16 6-4 4-4-4"></path>
<rect x="2" height="8" width="20" y="14" rx="2"></rect>
<path d="M6 18h.01"></path>
<path d="M10 18h.01"></path>"#,
};

pub const HARD_DRIVE_UPLOAD: IconType = IconType{ 
 content: r#"
<path d="m16 6-4-4-4 4"></path>
<path d="M12 2v8"></path>
<rect height="8" x="2" y="14" rx="2" width="20"></rect>
<path d="M6 18h.01"></path>
<path d="M10 18h.01"></path>"#,
};

pub const HARD_DRIVE: IconType = IconType{ 
 content: r#"
<line y1="12" x1="22" y2="12" x2="2"></line>
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>
<line x1="6" x2="6.01" y2="16" y1="16"></line>
<line x1="10" y1="16" x2="10.01" y2="16"></line>"#,
};

pub const HARD_HAT: IconType = IconType{ 
 content: r#"
<path d="M2 18a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v2z"></path>
<path d="M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5"></path>
<path d="M4 15v-3a6 6 0 0 1 6-6h0"></path>
<path d="M14 6h0a6 6 0 0 1 6 6v3"></path>"#,
};

pub const HASH: IconType = IconType{ 
 content: r#"
<line y2="9" y1="9" x2="20" x1="4"></line>
<line y1="15" x1="4" x2="20" y2="15"></line>
<line x1="10" y2="21" y1="3" x2="8"></line>
<line x1="16" y2="21" y1="3" x2="14"></line>"#,
};

pub const HAZE: IconType = IconType{ 
 content: r#"
<path d="m5.2 6.2 1.4 1.4"></path>
<path d="M2 13h2"></path>
<path d="M20 13h2"></path>
<path d="m17.4 7.6 1.4-1.4"></path>
<path d="M22 17H2"></path>
<path d="M22 21H2"></path>
<path d="M16 13a4 4 0 0 0-8 0"></path>
<path d="M12 5V2.5"></path>"#,
};

pub const HDMI_PORT: IconType = IconType{ 
 content: r#"
<path d="M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z"></path>
<path d="M7.5 12h9"></path>"#,
};

pub const HEADING_1: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="m17 12 3-2v8"></path>"#,
};

pub const HEADING_2: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>"#,
};

pub const HEADING_3: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
<path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>"#,
};

pub const HEADING_4: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17 10v4h4"></path>
<path d="M21 10v8"></path>"#,
};

pub const HEADING_5: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<path d="M17 13v-3h4"></path>
<path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"></path>"#,
};

pub const HEADING_6: IconType = IconType{ 
 content: r#"
<path d="M4 12h8"></path>
<path d="M4 18V6"></path>
<path d="M12 18V6"></path>
<circle cx="19" cy="16" r="2"></circle>
<path d="M20 10c-2 2-3 3.5-3 6"></path>"#,
};

pub const HEADING: IconType = IconType{ 
 content: r#"
<path d="M6 12h12"></path>
<path d="M6 20V4"></path>
<path d="M18 20V4"></path>"#,
};

pub const HEADPHONES: IconType = IconType{ 
 content: r#"
<path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"></path>"#,
};

pub const HEART_CRACK: IconType = IconType{ 
 content: r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="m12 13-1-1 2-2-3-3 2-2"></path>"#,
};

pub const HEART_HANDSHAKE: IconType = IconType{ 
 content: r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08v0c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0l2.96 2.66"></path>
<path d="m18 15-2-2"></path>
<path d="m15 18-2-2"></path>"#,
};

pub const HEART_OFF: IconType = IconType{ 
 content: r#"
<line x2="22" x1="2" y2="22" y1="2"></line>
<path d="M16.5 16.5 12 21l-7-7c-1.5-1.45-3-3.2-3-5.5a5.5 5.5 0 0 1 2.14-4.35"></path>
<path d="M8.76 3.1c1.15.22 2.13.78 3.24 1.9 1.5-1.5 2.74-2 4.5-2A5.5 5.5 0 0 1 22 8.5c0 2.12-1.3 3.78-2.67 5.17"></path>"#,
};

pub const HEART_PULSE: IconType = IconType{ 
 content: r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
<path d="M3.22 12H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27"></path>"#,
};

pub const HEART: IconType = IconType{ 
 content: r#"
<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>"#,
};

pub const HELP_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
<path d="M12 17h.01"></path>"#,
};

pub const HELPING_HAND: IconType = IconType{ 
 content: r#"
<path d="m3 15 5.12-5.12A3 3 0 0 1 10.24 9H13a2 2 0 1 1 0 4h-2.5m4-.68 4.17-4.89a1.88 1.88 0 0 1 2.92 2.36l-4.2 5.94A3 3 0 0 1 14.96 17H9.83a2 2 0 0 0-1.42.59L7 19"></path>
<path d="m2 14 6 6"></path>"#,
};

pub const HEXAGON: IconType = IconType{ 
 content: r#"
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>"#,
};

pub const HIGHLIGHTER: IconType = IconType{ 
 content: r#"
<path d="m9 11-6 6v3h9l3-3"></path>
<path d="m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4"></path>"#,
};

pub const HISTORY: IconType = IconType{ 
 content: r#"
<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>
<path d="M12 7v5l4 2"></path>"#,
};

pub const HOME: IconType = IconType{ 
 content: r#"
<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
<polyline points="9 22 9 12 15 12 15 22"></polyline>"#,
};

pub const HOP_OFF: IconType = IconType{ 
 content: r#"
<path d="M17.5 5.5C19 7 20.5 9 21 11c-1.323.265-2.646.39-4.118.226"></path>
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5"></path>
<path d="M17.5 17.5c-2.5 0-4 0-6-1"></path>
<path d="M20 11.5c1 1.5 2 3.5 2 4.5"></path>
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5"></path>
<path d="M22 22c-2 0-3.5-.5-5.5-1.5"></path>
<path d="M4.783 4.782C1.073 8.492 1 14.5 5 18c1-1 2-4.5 1.5-6.5 1.5 1 4 1 5.5.5M8.227 2.57C11.578 1.335 15.453 2.089 18 5c-.88.88-3.7 1.761-5.726 1.618"></path>
<line x1="2" y2="22" x2="22" y1="2"></line>"#,
};

pub const HOP: IconType = IconType{ 
 content: r#"
<path d="M17.5 5.5C19 7 20.5 9 21 11c-2.5.5-5 .5-8.5-1"></path>
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5"></path>
<path d="M16.5 11.5c1 2 1 3.5 1 6-2.5 0-4 0-6-1"></path>
<path d="M20 11.5c1 1.5 2 3.5 2 4.5-1.5.5-3 0-4.5-.5"></path>
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5"></path>
<path d="M20.5 16.5c1 2 1.5 3.5 1.5 5.5-2 0-3.5-.5-5.5-1.5"></path>
<path d="M4.783 4.782C8.493 1.072 14.5 1 18 5c-1 1-4.5 2-6.5 1.5 1 1.5 1 4 .5 5.5-1.5.5-4 .5-5.5-.5C7 13.5 6 17 5 18c-4-3.5-3.927-9.508-.217-13.218Z"></path>
<path d="M4.5 4.5 3 3c-.184-.185-.184-.816 0-1"></path>"#,
};

pub const HOTEL: IconType = IconType{ 
 content: r#"
<path d="M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2Z"></path>
<path d="m9 16 .348-.24c1.465-1.013 3.84-1.013 5.304 0L15 16"></path>
<path d="M8 7h.01"></path>
<path d="M16 7h.01"></path>
<path d="M12 7h.01"></path>
<path d="M12 11h.01"></path>
<path d="M16 11h.01"></path>
<path d="M8 11h.01"></path>
<path d="M10 22v-6.5m4 0V22"></path>"#,
};

pub const HOURGLASS: IconType = IconType{ 
 content: r#"
<path d="M5 22h14"></path>
<path d="M5 2h14"></path>
<path d="M17 22v-4.172a2 2 0 0 0-.586-1.414L12 12l-4.414 4.414A2 2 0 0 0 7 17.828V22"></path>
<path d="M7 2v4.172a2 2 0 0 0 .586 1.414L12 12l4.414-4.414A2 2 0 0 0 17 6.172V2"></path>"#,
};

pub const ICE_CREAM_2: IconType = IconType{ 
 content: r#"
<path d="M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6Zm-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0"></path>
<path d="M12.14 11a3.5 3.5 0 1 1 6.71 0"></path>
<path d="M15.5 6.5a3.5 3.5 0 1 0-7 0"></path>"#,
};

pub const ICE_CREAM: IconType = IconType{ 
 content: r#"
<path d="m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11"></path>
<path d="M17 7A5 5 0 0 0 7 7"></path>
<path d="M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4"></path>"#,
};

pub const IMAGE_MINUS: IconType = IconType{ 
 content: r#"
<path d="M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
<line x2="22" x1="16" y2="5" y1="5"></line>
<circle cx="9" cy="9" r="2"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#,
};

pub const IMAGE_OFF: IconType = IconType{ 
 content: r#"
<line y2="22" y1="2" x1="2" x2="22"></line>
<path d="M10.41 10.41a2 2 0 1 1-2.83-2.83"></path>
<line y1="13.5" x2="6" x1="13.5" y2="21"></line>
<line y2="15" x1="18" x2="21" y1="12"></line>
<path d="M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59"></path>
<path d="M21 15V5a2 2 0 0 0-2-2H9"></path>"#,
};

pub const IMAGE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
<line y2="5" x2="22" y1="5" x1="16"></line>
<line x1="19" y2="8" y1="2" x2="19"></line>
<circle cx="9" cy="9" r="2"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#,
};

pub const IMAGE: IconType = IconType{ 
 content: r#"
<rect rx="2" x="3" y="3" width="18" height="18" ry="2"></rect>
<circle cx="9" r="2" cy="9"></circle>
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>"#,
};

pub const IMPORT: IconType = IconType{ 
 content: r#"
<path d="M12 3v12"></path>
<path d="m8 11 4 4 4-4"></path>
<path d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4"></path>"#,
};

pub const INBOX: IconType = IconType{ 
 content: r#"
<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>"#,
};

pub const INDENT: IconType = IconType{ 
 content: r#"
<polyline points="3 8 7 12 3 16"></polyline>
<line y1="12" x2="11" x1="21" y2="12"></line>
<line x2="11" y1="6" x1="21" y2="6"></line>
<line y2="18" x2="11" y1="18" x1="21"></line>"#,
};

pub const INDIAN_RUPEE: IconType = IconType{ 
 content: r#"
<path d="M6 3h12"></path>
<path d="M6 8h12"></path>
<path d="m6 13 8.5 8"></path>
<path d="M6 13h3"></path>
<path d="M9 13c6.667 0 6.667-10 0-10"></path>"#,
};

pub const INFINITY: IconType = IconType{ 
 content: r#"
<path d="M12 12c-2-2.67-4-4-6-4a4 4 0 1 0 0 8c2 0 4-1.33 6-4Zm0 0c2 2.67 4 4 6 4a4 4 0 0 0 0-8c-2 0-4 1.33-6 4Z"></path>"#,
};

pub const INFO: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M12 16v-4"></path>
<path d="M12 8h.01"></path>"#,
};

pub const INSTAGRAM: IconType = IconType{ 
 content: r#"
<rect height="20" y="2" x="2" rx="5" ry="5" width="20"></rect>
<path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
<line y1="6.5" x2="17.51" y2="6.5" x1="17.5"></line>"#,
};

pub const ITALIC: IconType = IconType{ 
 content: r#"
<line y1="4" x1="19" x2="10" y2="4"></line>
<line y1="20" x2="5" x1="14" y2="20"></line>
<line y1="4" x2="9" y2="20" x1="15"></line>"#,
};

pub const ITERATION_CCW: IconType = IconType{ 
 content: r#"
<path d="M20 10c0-4.4-3.6-8-8-8s-8 3.6-8 8 3.6 8 8 8h8"></path>
<polyline points="16 14 20 18 16 22"></polyline>"#,
};

pub const ITERATION_CW: IconType = IconType{ 
 content: r#"
<path d="M4 10c0-4.4 3.6-8 8-8s8 3.6 8 8-3.6 8-8 8H4"></path>
<polyline points="8 22 4 18 8 14"></polyline>"#,
};

pub const JAPANESE_YEN: IconType = IconType{ 
 content: r#"
<path d="M12 9.5V21m0-11.5L6 3m6 6.5L18 3"></path>
<path d="M6 15h12"></path>
<path d="M6 11h12"></path>"#,
};

pub const JOYSTICK: IconType = IconType{ 
 content: r#"
<path d="M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z"></path>
<path d="M6 15v-2"></path>
<path d="M12 15V9"></path>
<circle r="3" cy="6" cx="12"></circle>"#,
};

pub const KANBAN_SQUARE_DASHED: IconType = IconType{ 
 content: r#"
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
<path d="M3 9v1"></path>"#,
};

pub const KANBAN_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" rx="2" height="18" y="3" width="18"></rect>
<path d="M8 7v7"></path>
<path d="M12 7v4"></path>
<path d="M16 7v9"></path>"#,
};

pub const KANBAN: IconType = IconType{ 
 content: r#"
<path d="M6 5v11"></path>
<path d="M12 5v6"></path>
<path d="M18 5v14"></path>"#,
};

pub const KEY_ROUND: IconType = IconType{ 
 content: r#"
<path d="M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z"></path>
<circle cx="16.5" cy="7.5" r=".5"></circle>"#,
};

pub const KEY_SQUARE: IconType = IconType{ 
 content: r#"
<path d="M12.4 2.7c.9-.9 2.5-.9 3.4 0l5.5 5.5c.9.9.9 2.5 0 3.4l-3.7 3.7c-.9.9-2.5.9-3.4 0L8.7 9.8c-.9-.9-.9-2.5 0-3.4Z"></path>
<path d="m14 7 3 3"></path>
<path d="M9.4 10.6 2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4"></path>"#,
};

pub const KEY: IconType = IconType{ 
 content: r#"
<circle cx="7.5" cy="15.5" r="5.5"></circle>
<path d="m21 2-9.6 9.6"></path>
<path d="m15.5 7.5 3 3L22 7l-3-3"></path>"#,
};

pub const KEYBOARD: IconType = IconType{ 
 content: r#"
<rect x="2" y="4" rx="2" ry="2" width="20" height="16"></rect>
<path d="M6 8h.001"></path>
<path d="M10 8h.001"></path>
<path d="M14 8h.001"></path>
<path d="M18 8h.001"></path>
<path d="M8 12h.001"></path>
<path d="M12 12h.001"></path>
<path d="M16 12h.001"></path>
<path d="M7 16h10"></path>"#,
};

pub const LAMP_CEILING: IconType = IconType{ 
 content: r#"
<path d="M12 2v5"></path>
<path d="M6 7h12l4 9H2l4-9Z"></path>
<path d="M9.17 16a3 3 0 1 0 5.66 0"></path>"#,
};

pub const LAMP_DESK: IconType = IconType{ 
 content: r#"
<path d="m14 5-3 3 2 7 8-8-7-2Z"></path>
<path d="m14 5-3 3-3-3 3-3 3 3Z"></path>
<path d="M9.5 6.5 4 12l3 6"></path>
<path d="M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z"></path>"#,
};

pub const LAMP_FLOOR: IconType = IconType{ 
 content: r#"
<path d="M9 2h6l3 7H6l3-7Z"></path>
<path d="M12 9v13"></path>
<path d="M9 22h6"></path>"#,
};

pub const LAMP_WALL_DOWN: IconType = IconType{ 
 content: r#"
<path d="M11 13h6l3 7H8l3-7Z"></path>
<path d="M14 13V8a2 2 0 0 0-2-2H8"></path>
<path d="M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z"></path>"#,
};

pub const LAMP_WALL_UP: IconType = IconType{ 
 content: r#"
<path d="M11 4h6l3 7H8l3-7Z"></path>
<path d="M14 11v5a2 2 0 0 1-2 2H8"></path>
<path d="M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z"></path>"#,
};

pub const LAMP: IconType = IconType{ 
 content: r#"
<path d="M8 2h8l4 10H4L8 2Z"></path>
<path d="M12 12v6"></path>
<path d="M8 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H8Z"></path>"#,
};

pub const LANDMARK: IconType = IconType{ 
 content: r#"
<line y1="22" x2="21" x1="3" y2="22"></line>
<line y1="18" x2="6" x1="6" y2="11"></line>
<line y1="18" x1="10" y2="11" x2="10"></line>
<line x2="14" y2="11" x1="14" y1="18"></line>
<line x2="18" y1="18" x1="18" y2="11"></line>
<polygon points="12 2 20 7 4 7"></polygon>"#,
};

pub const LANGUAGES: IconType = IconType{ 
 content: r#"
<path d="m5 8 6 6"></path>
<path d="m4 14 6-6 2-3"></path>
<path d="M2 5h12"></path>
<path d="M7 2h1"></path>
<path d="m22 22-5-10-5 10"></path>
<path d="M14 18h6"></path>"#,
};

pub const LAPTOP_2: IconType = IconType{ 
 content: r#"
<rect width="18" height="12" x="3" rx="2" y="4" ry="2"></rect>
<line x2="22" y2="20" x1="2" y1="20"></line>"#,
};

pub const LAPTOP: IconType = IconType{ 
 content: r#"
<path d="M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16"></path>"#,
};

pub const LASSO_SELECT: IconType = IconType{ 
 content: r#"
<path d="M7 22a5 5 0 0 1-2-4"></path>
<path d="M7 16.93c.96.43 1.96.74 2.99.91"></path>
<path d="M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2"></path>
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"></path>
<path d="M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14v0z"></path>"#,
};

pub const LASSO: IconType = IconType{ 
 content: r#"
<path d="M7 22a5 5 0 0 1-2-4"></path>
<path d="M3.3 14A6.8 6.8 0 0 1 2 10c0-4.4 4.5-8 10-8s10 3.6 10 8-4.5 8-10 8a12 12 0 0 1-5-1"></path>
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"></path>"#,
};

pub const LAUGH: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z"></path>
<line x2="9.01" y1="9" y2="9" x1="9"></line>
<line x1="15" y1="9" y2="9" x2="15.01"></line>"#,
};

pub const LAYERS: IconType = IconType{ 
 content: r#"
<polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
<polyline points="2 17 12 22 22 17"></polyline>
<polyline points="2 12 12 17 22 12"></polyline>"#,
};

pub const LAYOUT_DASHBOARD: IconType = IconType{ 
 content: r#"
<rect y="3" rx="1" height="9" width="7" x="3"></rect>
<rect y="3" height="5" x="14" rx="1" width="7"></rect>
<rect x="14" width="7" height="9" y="12" rx="1"></rect>
<rect x="3" height="5" width="7" y="16" rx="1"></rect>"#,
};

pub const LAYOUT_GRID: IconType = IconType{ 
 content: r#"
<rect x="3" rx="1" y="3" height="7" width="7"></rect>
<rect rx="1" height="7" x="14" width="7" y="3"></rect>
<rect height="7" width="7" x="14" rx="1" y="14"></rect>
<rect height="7" x="3" y="14" rx="1" width="7"></rect>"#,
};

pub const LAYOUT_LIST: IconType = IconType{ 
 content: r#"
<rect height="7" y="3" width="7" rx="1" x="3"></rect>
<rect height="7" width="7" rx="1" x="3" y="14"></rect>
<path d="M14 4h7"></path>
<path d="M14 9h7"></path>
<path d="M14 15h7"></path>
<path d="M14 20h7"></path>"#,
};

pub const LAYOUT_PANEL_LEFT: IconType = IconType{ 
 content: r#"
<rect x="3" y="3" rx="1" height="18" width="7"></rect>
<rect rx="1" width="7" height="7" x="14" y="3"></rect>
<rect y="14" height="7" rx="1" x="14" width="7"></rect>"#,
};

pub const LAYOUT_PANEL_TOP: IconType = IconType{ 
 content: r#"
<rect width="18" height="7" y="3" rx="1" x="3"></rect>
<rect width="7" x="3" y="14" rx="1" height="7"></rect>
<rect y="14" rx="1" x="14" width="7" height="7"></rect>"#,
};

pub const LAYOUT_TEMPLATE: IconType = IconType{ 
 content: r#"
<rect width="18" x="3" height="7" rx="1" y="3"></rect>
<rect y="14" rx="1" width="9" x="3" height="7"></rect>
<rect height="7" y="14" x="16" rx="1" width="5"></rect>"#,
};

pub const LAYOUT: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" x="3" y="3" rx="2" ry="2"></rect>
<line y2="9" x1="3" x2="21" y1="9"></line>
<line x1="9" y1="21" x2="9" y2="9"></line>"#,
};

pub const LEAF: IconType = IconType{ 
 content: r#"
<path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z"></path>
<path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12"></path>"#,
};

pub const LEAFY_GREEN: IconType = IconType{ 
 content: r#"
<path d="M2 22c1.25-.987 2.27-1.975 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.187-2.353 3.5 3.5 0 0 0 3.69-5.116A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.831 1.373 3.5 3.5 0 0 0-5.116 3.69 4 4 0 0 0-2.348 6.155C3.499 15.42 4.409 16.712 4.2 18.1 3.926 19.743 3.014 20.732 2 22"></path>
<path d="M2 22 17 7"></path>"#,
};

pub const LIBRARY: IconType = IconType{ 
 content: r#"
<path d="m16 6 4 14"></path>
<path d="M12 6v14"></path>
<path d="M8 8v12"></path>
<path d="M4 4v16"></path>"#,
};

pub const LIFE_BUOY: IconType = IconType{ 
 content: r#"
<circle cx="12" r="10" cy="12"></circle>
<path d="m4.93 4.93 4.24 4.24"></path>
<path d="m14.83 9.17 4.24-4.24"></path>
<path d="m14.83 14.83 4.24 4.24"></path>
<path d="m9.17 14.83-4.24 4.24"></path>
<circle r="4" cx="12" cy="12"></circle>"#,
};

pub const LIGATURE: IconType = IconType{ 
 content: r#"
<path d="M8 20V8c0-2.2 1.8-4 4-4 1.5 0 2.8.8 3.5 2"></path>
<path d="M6 12h4"></path>
<path d="M14 12h2v8"></path>
<path d="M6 20h4"></path>
<path d="M14 20h4"></path>"#,
};

pub const LIGHTBULB_OFF: IconType = IconType{ 
 content: r#"
<path d="M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5"></path>
<path d="m2 2 20 20"></path>
<path d="M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5"></path>
<path d="M9 18h6"></path>
<path d="M10 22h4"></path>"#,
};

pub const LIGHTBULB: IconType = IconType{ 
 content: r#"
<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"></path>
<path d="M9 18h6"></path>
<path d="M10 22h4"></path>"#,
};

pub const LINE_CHART: IconType = IconType{ 
 content: r#"
<path d="M3 3v18h18"></path>
<path d="m19 9-5 5-4-4-3 3"></path>"#,
};

pub const LINK_2_OFF: IconType = IconType{ 
 content: r#"
<path d="M9 17H7A5 5 0 0 1 7 7"></path>
<path d="M15 7h2a5 5 0 0 1 4 8"></path>
<line y2="12" x1="8" x2="12" y1="12"></line>
<line x1="2" x2="22" y2="22" y1="2"></line>"#,
};

pub const LINK_2: IconType = IconType{ 
 content: r#"
<path d="M9 17H7A5 5 0 0 1 7 7h2"></path>
<path d="M15 7h2a5 5 0 1 1 0 10h-2"></path>
<line y2="12" x1="8" y1="12" x2="16"></line>"#,
};

pub const LINK: IconType = IconType{ 
 content: r#"
<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>"#,
};

pub const LINKEDIN: IconType = IconType{ 
 content: r#"
<path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
<rect height="12" width="4" y="9" x="2"></rect>
<circle cy="4" r="2" cx="4"></circle>"#,
};

pub const LIST_CHECKS: IconType = IconType{ 
 content: r#"
<path d="m3 17 2 2 4-4"></path>
<path d="m3 7 2 2 4-4"></path>
<path d="M13 6h8"></path>
<path d="M13 12h8"></path>
<path d="M13 18h8"></path>"#,
};

pub const LIST_END: IconType = IconType{ 
 content: r#"
<path d="M16 12H3"></path>
<path d="M16 6H3"></path>
<path d="M10 18H3"></path>
<path d="M21 6v10a2 2 0 0 1-2 2h-5"></path>
<path d="m16 16-2 2 2 2"></path>"#,
};

pub const LIST_FILTER: IconType = IconType{ 
 content: r#"
<path d="M3 6h18"></path>
<path d="M7 12h10"></path>
<path d="M10 18h4"></path>"#,
};

pub const LIST_MINUS: IconType = IconType{ 
 content: r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="M21 12h-6"></path>"#,
};

pub const LIST_MUSIC: IconType = IconType{ 
 content: r#"
<path d="M21 15V6"></path>
<path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
<path d="M12 12H3"></path>
<path d="M16 6H3"></path>
<path d="M12 18H3"></path>"#,
};

pub const LIST_ORDERED: IconType = IconType{ 
 content: r#"
<line x1="10" y2="6" x2="21" y1="6"></line>
<line x1="10" y1="12" y2="12" x2="21"></line>
<line y1="18" y2="18" x1="10" x2="21"></line>
<path d="M4 6h1v4"></path>
<path d="M4 10h2"></path>
<path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>"#,
};

pub const LIST_PLUS: IconType = IconType{ 
 content: r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="M18 9v6"></path>
<path d="M21 12h-6"></path>"#,
};

pub const LIST_RESTART: IconType = IconType{ 
 content: r#"
<path d="M21 6H3"></path>
<path d="M7 12H3"></path>
<path d="M7 18H3"></path>
<path d="M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14"></path>
<path d="M11 10v4h4"></path>"#,
};

pub const LIST_START: IconType = IconType{ 
 content: r#"
<path d="M16 12H3"></path>
<path d="M16 18H3"></path>
<path d="M10 6H3"></path>
<path d="M21 18V8a2 2 0 0 0-2-2h-5"></path>
<path d="m16 8-2-2 2-2"></path>"#,
};

pub const LIST_TODO: IconType = IconType{ 
 content: r#"
<rect y="5" x="3" width="6" height="6" rx="1"></rect>
<path d="m3 17 2 2 4-4"></path>
<path d="M13 6h8"></path>
<path d="M13 12h8"></path>
<path d="M13 18h8"></path>"#,
};

pub const LIST_TREE: IconType = IconType{ 
 content: r#"
<path d="M21 12h-8"></path>
<path d="M21 6H8"></path>
<path d="M21 18h-8"></path>
<path d="M3 6v4c0 1.1.9 2 2 2h3"></path>
<path d="M3 10v6c0 1.1.9 2 2 2h3"></path>"#,
};

pub const LIST_VIDEO: IconType = IconType{ 
 content: r#"
<path d="M12 12H3"></path>
<path d="M16 6H3"></path>
<path d="M12 18H3"></path>
<path d="m16 12 5 3-5 3v-6Z"></path>"#,
};

pub const LIST_X: IconType = IconType{ 
 content: r#"
<path d="M11 12H3"></path>
<path d="M16 6H3"></path>
<path d="M16 18H3"></path>
<path d="m19 10-4 4"></path>
<path d="m15 10 4 4"></path>"#,
};

pub const LIST: IconType = IconType{ 
 content: r#"
<line y2="6" y1="6" x1="8" x2="21"></line>
<line x2="21" y2="12" y1="12" x1="8"></line>
<line y2="18" x2="21" x1="8" y1="18"></line>
<line y2="6" x2="3.01" y1="6" x1="3"></line>
<line y2="12" y1="12" x1="3" x2="3.01"></line>
<line y1="18" x1="3" x2="3.01" y2="18"></line>"#,
};

pub const LOADER_2: IconType = IconType{ 
 content: r#"
<path d="M21 12a9 9 0 1 1-6.219-8.56"></path>"#,
};

pub const LOADER: IconType = IconType{ 
 content: r#"
<line x1="12" y2="6" x2="12" y1="2"></line>
<line x1="12" y1="18" y2="22" x2="12"></line>
<line y1="4.93" x1="4.93" x2="7.76" y2="7.76"></line>
<line x2="19.07" y2="19.07" y1="16.24" x1="16.24"></line>
<line x1="2" y1="12" x2="6" y2="12"></line>
<line y1="12" x2="22" y2="12" x1="18"></line>
<line y1="19.07" x1="4.93" y2="16.24" x2="7.76"></line>
<line y2="4.93" x2="19.07" y1="7.76" x1="16.24"></line>"#,
};

pub const LOCATE_FIXED: IconType = IconType{ 
 content: r#"
<line x1="2" x2="5" y1="12" y2="12"></line>
<line y1="12" x1="19" x2="22" y2="12"></line>
<line y1="2" x1="12" y2="5" x2="12"></line>
<line y1="19" y2="22" x2="12" x1="12"></line>
<circle cy="12" cx="12" r="7"></circle>
<circle cx="12" cy="12" r="3"></circle>"#,
};

pub const LOCATE_OFF: IconType = IconType{ 
 content: r#"
<line x1="2" y1="12" y2="12" x2="5"></line>
<line x1="19" y2="12" x2="22" y1="12"></line>
<line y1="2" y2="5" x2="12" x1="12"></line>
<line x1="12" y1="19" x2="12" y2="22"></line>
<path d="M7.11 7.11C5.83 8.39 5 10.1 5 12c0 3.87 3.13 7 7 7 1.9 0 3.61-.83 4.89-2.11"></path>
<path d="M18.71 13.96c.19-.63.29-1.29.29-1.96 0-3.87-3.13-7-7-7-.67 0-1.33.1-1.96.29"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#,
};

pub const LOCATE: IconType = IconType{ 
 content: r#"
<line x2="5" x1="2" y1="12" y2="12"></line>
<line x2="22" x1="19" y2="12" y1="12"></line>
<line y1="2" x2="12" y2="5" x1="12"></line>
<line x1="12" x2="12" y1="19" y2="22"></line>
<circle cx="12" cy="12" r="7"></circle>"#,
};

pub const LOCK: IconType = IconType{ 
 content: r#"
<rect y="11" ry="2" x="3" width="18" height="11" rx="2"></rect>
<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>"#,
};

pub const LOG_IN: IconType = IconType{ 
 content: r#"
<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
<polyline points="10 17 15 12 10 7"></polyline>
<line x2="3" y1="12" x1="15" y2="12"></line>"#,
};

pub const LOG_OUT: IconType = IconType{ 
 content: r#"
<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
<polyline points="16 17 21 12 16 7"></polyline>
<line x1="21" x2="9" y1="12" y2="12"></line>"#,
};

pub const LOLLIPOP: IconType = IconType{ 
 content: r#"
<circle cx="11" cy="11" r="8"></circle>
<path d="m21 21-4.3-4.3"></path>
<path d="M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0"></path>"#,
};

pub const LUGGAGE: IconType = IconType{ 
 content: r#"
<path d="M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0"></path>
<path d="M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14"></path>
<path d="M10 20h4"></path>
<circle r="2" cy="20" cx="16"></circle>
<circle cx="8" r="2" cy="20"></circle>"#,
};

pub const M_SQUARE: IconType = IconType{ 
 content: r#"
<rect y="3" width="18" height="18" x="3" rx="2"></rect>
<path d="M8 16V8l4 4 4-4v8"></path>"#,
};

pub const MAGNET: IconType = IconType{ 
 content: r#"
<path d="m6 15-4-4 6.75-6.77a7.79 7.79 0 0 1 11 11L13 22l-4-4 6.39-6.36a2.14 2.14 0 0 0-3-3L6 15"></path>
<path d="m5 8 4 4"></path>
<path d="m12 15 4 4"></path>"#,
};

pub const MAIL_CHECK: IconType = IconType{ 
 content: r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="m16 19 2 2 4-4"></path>"#,
};

pub const MAIL_MINUS: IconType = IconType{ 
 content: r#"
<path d="M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M16 19h6"></path>"#,
};

pub const MAIL_OPEN: IconType = IconType{ 
 content: r#"
<path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z"></path>
<path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10"></path>"#,
};

pub const MAIL_PLUS: IconType = IconType{ 
 content: r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M19 16v6"></path>
<path d="M16 19h6"></path>"#,
};

pub const MAIL_QUESTION: IconType = IconType{ 
 content: r#"
<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"></path>
<path d="M20 22v.01"></path>"#,
};

pub const MAIL_SEARCH: IconType = IconType{ 
 content: r#"
<path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z"></path>
<circle r="3" cy="18" cx="18"></circle>
<path d="m22 22-1.5-1.5"></path>"#,
};

pub const MAIL_WARNING: IconType = IconType{ 
 content: r#"
<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="M20 14v4"></path>
<path d="M20 22v.01"></path>"#,
};

pub const MAIL_X: IconType = IconType{ 
 content: r#"
<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h9"></path>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
<path d="m17 17 4 4"></path>
<path d="m21 17-4 4"></path>"#,
};

pub const MAIL: IconType = IconType{ 
 content: r#"
<rect x="2" height="16" width="20" rx="2" y="4"></rect>
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>"#,
};

pub const MAILBOX: IconType = IconType{ 
 content: r#"
<path d="M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z"></path>
<polyline points="15,9 18,9 18,11"></polyline>
<path d="M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2v0"></path>
<line x2="7" x1="6" y2="10" y1="10"></line>"#,
};

pub const MAILS: IconType = IconType{ 
 content: r#"
<rect height="13" x="6" width="16" y="4" rx="2"></rect>
<path d="m22 7-7.1 3.78c-.57.3-1.23.3-1.8 0L6 7"></path>
<path d="M2 8v11c0 1.1.9 2 2 2h14"></path>"#,
};

pub const MAP_PIN_OFF: IconType = IconType{ 
 content: r#"
<path d="M5.43 5.43A8.06 8.06 0 0 0 4 10c0 6 8 12 8 12a29.94 29.94 0 0 0 5-5"></path>
<path d="M19.18 13.52A8.66 8.66 0 0 0 20 10a8 8 0 0 0-8-8 7.88 7.88 0 0 0-3.52.82"></path>
<path d="M9.13 9.13A2.78 2.78 0 0 0 9 10a3 3 0 0 0 3 3 2.78 2.78 0 0 0 .87-.13"></path>
<path d="M14.9 9.25a3 3 0 0 0-2.15-2.16"></path>
<line y1="2" x2="22" y2="22" x1="2"></line>"#,
};

pub const MAP_PIN: IconType = IconType{ 
 content: r#"
<path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"></path>
<circle cx="12" cy="10" r="3"></circle>"#,
};

pub const MAP: IconType = IconType{ 
 content: r#"
<polygon points="3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21"></polygon>
<line y1="3" x2="9" x1="9" y2="18"></line>
<line x1="15" y1="6" x2="15" y2="21"></line>"#,
};

pub const MARTINI: IconType = IconType{ 
 content: r#"
<path d="M8 22h8"></path>
<path d="M12 11v11"></path>
<path d="m19 3-7 8-7-8Z"></path>"#,
};

pub const MAXIMIZE_2: IconType = IconType{ 
 content: r#"
<polyline points="15 3 21 3 21 9"></polyline>
<polyline points="9 21 3 21 3 15"></polyline>
<line x2="14" x1="21" y1="3" y2="10"></line>
<line y1="21" y2="14" x2="10" x1="3"></line>"#,
};

pub const MAXIMIZE: IconType = IconType{ 
 content: r#"
<path d="M8 3H5a2 2 0 0 0-2 2v3"></path>
<path d="M21 8V5a2 2 0 0 0-2-2h-3"></path>
<path d="M3 16v3a2 2 0 0 0 2 2h3"></path>
<path d="M16 21h3a2 2 0 0 0 2-2v-3"></path>"#,
};

pub const MEDAL: IconType = IconType{ 
 content: r#"
<path d="M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15"></path>
<path d="M11 12 5.12 2.2"></path>
<path d="m13 12 5.88-9.8"></path>
<path d="M8 7h8"></path>
<circle r="5" cx="12" cy="17"></circle>
<path d="M12 18v-2h-.5"></path>"#,
};

pub const MEGAPHONE_OFF: IconType = IconType{ 
 content: r#"
<path d="M9.26 9.26 3 11v3l14.14 3.14"></path>
<path d="M21 15.34V6l-7.31 2.03"></path>
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"></path>
<line x2="22" y1="2" x1="2" y2="22"></line>"#,
};

pub const MEGAPHONE: IconType = IconType{ 
 content: r#"
<path d="m3 11 18-5v12L3 14v-3z"></path>
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"></path>"#,
};

pub const MEH: IconType = IconType{ 
 content: r#"
<circle cx="12" r="10" cy="12"></circle>
<line x2="16" y2="15" y1="15" x1="8"></line>
<line x1="9" x2="9.01" y1="9" y2="9"></line>
<line y2="9" y1="9" x2="15.01" x1="15"></line>"#,
};

pub const MEMORY_STICK: IconType = IconType{ 
 content: r#"
<path d="M6 19v-3"></path>
<path d="M10 19v-3"></path>
<path d="M14 19v-3"></path>
<path d="M18 19v-3"></path>
<path d="M8 11V9"></path>
<path d="M16 11V9"></path>
<path d="M12 11V9"></path>
<path d="M2 15h20"></path>
<path d="M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z"></path>"#,
};

pub const MENU_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" x="3" width="18" y="3" height="18"></rect>
<path d="M7 8h10"></path>
<path d="M7 12h10"></path>
<path d="M7 16h10"></path>"#,
};

pub const MENU: IconType = IconType{ 
 content: r#"
<line x2="20" y1="12" y2="12" x1="4"></line>
<line y1="6" y2="6" x1="4" x2="20"></line>
<line y2="18" x1="4" x2="20" y1="18"></line>"#,
};

pub const MERGE: IconType = IconType{ 
 content: r#"
<path d="m8 6 4-4 4 4"></path>
<path d="M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22"></path>
<path d="m20 22-5-5"></path>"#,
};

pub const MESSAGE_CIRCLE: IconType = IconType{ 
 content: r#"
<path d="m3 21 1.9-5.7a8.5 8.5 0 1 1 3.8 3.8z"></path>"#,
};

pub const MESSAGE_SQUARE_DASHED: IconType = IconType{ 
 content: r#"
<path d="M3 6V5c0-1.1.9-2 2-2h2"></path>
<path d="M11 3h3"></path>
<path d="M18 3h1c1.1 0 2 .9 2 2"></path>
<path d="M21 9v2"></path>
<path d="M21 15c0 1.1-.9 2-2 2h-1"></path>
<path d="M14 17h-3"></path>
<path d="m7 17-4 4v-5"></path>
<path d="M3 12v-2"></path>"#,
};

pub const MESSAGE_SQUARE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
<line y1="10" x1="9" y2="10" x2="15"></line>
<line y1="7" x1="12" y2="13" x2="12"></line>"#,
};

pub const MESSAGE_SQUARE: IconType = IconType{ 
 content: r#"
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>"#,
};

pub const MESSAGES_SQUARE: IconType = IconType{ 
 content: r#"
<path d="M14 9a2 2 0 0 1-2 2H6l-4 4V4c0-1.1.9-2 2-2h8a2 2 0 0 1 2 2v5Z"></path>
<path d="M18 9h2a2 2 0 0 1 2 2v11l-4-4h-6a2 2 0 0 1-2-2v-1"></path>"#,
};

pub const MIC_2: IconType = IconType{ 
 content: r#"
<path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12"></path>
<circle r="5" cy="7" cx="17"></circle>"#,
};

pub const MIC_OFF: IconType = IconType{ 
 content: r#"
<line x1="2" y2="22" y1="2" x2="22"></line>
<path d="M18.89 13.23A7.12 7.12 0 0 0 19 12v-2"></path>
<path d="M5 10v2a7 7 0 0 0 12 5"></path>
<path d="M15 9.34V5a3 3 0 0 0-5.68-1.33"></path>
<path d="M9 9v3a3 3 0 0 0 5.12 2.12"></path>
<line x1="12" y2="22" x2="12" y1="19"></line>"#,
};

pub const MIC: IconType = IconType{ 
 content: r#"
<path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"></path>
<path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
<line x2="12" y1="19" x1="12" y2="22"></line>"#,
};

pub const MICROSCOPE: IconType = IconType{ 
 content: r#"
<path d="M6 18h8"></path>
<path d="M3 22h18"></path>
<path d="M14 22a7 7 0 1 0 0-14h-1"></path>
<path d="M9 14h2"></path>
<path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z"></path>
<path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"></path>"#,
};

pub const MICROWAVE: IconType = IconType{ 
 content: r#"
<rect x="2" height="15" y="4" rx="2" width="20"></rect>
<rect width="8" height="7" y="8" rx="1" x="6"></rect>
<path d="M18 8v7"></path>
<path d="M6 19v2"></path>
<path d="M18 19v2"></path>"#,
};

pub const MILESTONE: IconType = IconType{ 
 content: r#"
<path d="M18 6H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h13l4-3.5L18 6Z"></path>
<path d="M12 13v8"></path>
<path d="M12 3v3"></path>"#,
};

pub const MILK_OFF: IconType = IconType{ 
 content: r#"
<path d="M8 2h8"></path>
<path d="M9 2v1.343M15 2v2.789a4 4 0 0 0 .672 2.219l.656.984a4 4 0 0 1 .672 2.22v1.131M7.8 7.8l-.128.192A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3"></path>
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.472 6.472 0 0 0 3.435.435"></path>
<line y1="2" y2="22" x2="22" x1="2"></line>"#,
};

pub const MILK: IconType = IconType{ 
 content: r#"
<path d="M8 2h8"></path>
<path d="M9 2v2.789a4 4 0 0 1-.672 2.219l-.656.984A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.789a4 4 0 0 0-.672-2.219l-.656-.984A4 4 0 0 1 15 4.788V2"></path>
<path d="M7 15a6.472 6.472 0 0 1 5 0 6.47 6.47 0 0 0 5 0"></path>"#,
};

pub const MINIMIZE_2: IconType = IconType{ 
 content: r#"
<polyline points="4 14 10 14 10 20"></polyline>
<polyline points="20 10 14 10 14 4"></polyline>
<line x1="14" y1="10" x2="21" y2="3"></line>
<line y1="21" x1="3" x2="10" y2="14"></line>"#,
};

pub const MINIMIZE: IconType = IconType{ 
 content: r#"
<path d="M8 3v3a2 2 0 0 1-2 2H3"></path>
<path d="M21 8h-3a2 2 0 0 1-2-2V3"></path>
<path d="M3 16h3a2 2 0 0 1 2 2v3"></path>
<path d="M16 21v-3a2 2 0 0 1 2-2h3"></path>"#,
};

pub const MINUS_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<path d="M8 12h8"></path>"#,
};

pub const MINUS_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" x="3" y="3" rx="2" width="18"></rect>
<path d="M8 12h8"></path>"#,
};

pub const MINUS: IconType = IconType{ 
 content: r#"
<path d="M5 12h14"></path>"#,
};

pub const MONITOR_CHECK: IconType = IconType{ 
 content: r#"
<path d="m9 10 2 2 4-4"></path>
<rect width="20" y="3" rx="2" height="14" x="2"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_DOT: IconType = IconType{ 
 content: r#"
<circle cx="19" r="3" cy="6"></circle>
<path d="M22 12v3a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9"></path>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_DOWN: IconType = IconType{ 
 content: r#"
<path d="M12 13V7"></path>
<path d="m15 10-3 3-3-3"></path>
<rect height="14" y="3" rx="2" width="20" x="2"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_OFF: IconType = IconType{ 
 content: r#"
<path d="M17 17H4a2 2 0 0 1-2-2V5c0-1.5 1-2 1-2"></path>
<path d="M22 15V5a2 2 0 0 0-2-2H9"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m2 2 20 20"></path>"#,
};

pub const MONITOR_PAUSE: IconType = IconType{ 
 content: r#"
<path d="M10 13V7"></path>
<path d="M14 13V7"></path>
<rect x="2" y="3" rx="2" width="20" height="14"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_PLAY: IconType = IconType{ 
 content: r#"
<path d="m10 7 5 3-5 3Z"></path>
<rect rx="2" x="2" y="3" width="20" height="14"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_SMARTPHONE: IconType = IconType{ 
 content: r#"
<path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8"></path>
<path d="M10 19v-3.96 3.15"></path>
<path d="M7 19h5"></path>
<rect height="10" width="6" y="12" rx="2" x="16"></rect>"#,
};

pub const MONITOR_SPEAKER: IconType = IconType{ 
 content: r#"
<path d="M5.5 20H8"></path>
<path d="M17 9h.01"></path>
<rect height="16" y="4" rx="2" width="10" x="12"></rect>
<path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4"></path>
<circle r="1" cy="15" cx="17"></circle>"#,
};

pub const MONITOR_STOP: IconType = IconType{ 
 content: r#"
<rect y="7" height="6" width="6" x="9"></rect>
<rect x="2" height="14" y="3" width="20" rx="2"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_UP: IconType = IconType{ 
 content: r#"
<path d="m9 10 3-3 3 3"></path>
<path d="M12 13V7"></path>
<rect height="14" y="3" rx="2" width="20" x="2"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR_X: IconType = IconType{ 
 content: r#"
<path d="m14.5 12.5-5-5"></path>
<path d="m9.5 12.5 5-5"></path>
<rect y="3" width="20" rx="2" height="14" x="2"></rect>
<path d="M12 17v4"></path>
<path d="M8 21h8"></path>"#,
};

pub const MONITOR: IconType = IconType{ 
 content: r#"
<rect width="20" height="14" y="3" rx="2" x="2"></rect>
<line y1="21" y2="21" x1="8" x2="16"></line>
<line x2="12" y2="21" x1="12" y1="17"></line>"#,
};

pub const MOON_STAR: IconType = IconType{ 
 content: r#"
<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
<path d="M19 3v4"></path>
<path d="M21 5h-4"></path>"#,
};

pub const MOON: IconType = IconType{ 
 content: r#"
<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>"#,
};

pub const MORE_HORIZONTAL: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="1"></circle>
<circle cy="12" cx="19" r="1"></circle>
<circle r="1" cx="5" cy="12"></circle>"#,
};

pub const MORE_VERTICAL: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="1"></circle>
<circle cy="5" r="1" cx="12"></circle>
<circle cx="12" cy="19" r="1"></circle>"#,
};

pub const MOUNTAIN_SNOW: IconType = IconType{ 
 content: r#"
<path d="m8 3 4 8 5-5 5 15H2L8 3z"></path>
<path d="M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19"></path>"#,
};

pub const MOUNTAIN: IconType = IconType{ 
 content: r#"
<path d="m8 3 4 8 5-5 5 15H2L8 3z"></path>"#,
};

pub const MOUSE_POINTER_2: IconType = IconType{ 
 content: r#"
<path d="m4 4 7.07 17 2.51-7.39L21 11.07z"></path>"#,
};

pub const MOUSE_POINTER_CLICK: IconType = IconType{ 
 content: r#"
<path d="m9 9 5 12 1.774-5.226L21 14 9 9z"></path>
<path d="m16.071 16.071 4.243 4.243"></path>
<path d="m7.188 2.239.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656-2.12 2.122"></path>"#,
};

pub const MOUSE_POINTER_SQUARE_DASHED: IconType = IconType{ 
 content: r#"
<path d="M5 3a2 2 0 0 0-2 2"></path>
<path d="M19 3a2 2 0 0 1 2 2"></path>
<path d="m12 12 4 10 1.7-4.3L22 16Z"></path>
<path d="M5 21a2 2 0 0 1-2-2"></path>
<path d="M9 3h1"></path>
<path d="M9 21h2"></path>
<path d="M14 3h1"></path>
<path d="M3 9v1"></path>
<path d="M21 9v2"></path>
<path d="M3 14v1"></path>"#,
};

pub const MOUSE_POINTER_SQUARE: IconType = IconType{ 
 content: r#"
<path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"></path>
<path d="m12 12 4 10 1.7-4.3L22 16Z"></path>"#,
};

pub const MOUSE_POINTER: IconType = IconType{ 
 content: r#"
<path d="m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z"></path>
<path d="m13 13 6 6"></path>"#,
};

pub const MOUSE: IconType = IconType{ 
 content: r#"
<rect x="5" y="2" height="20" rx="7" width="14"></rect>
<path d="M12 6v4"></path>"#,
};

pub const MOVE_3_D: IconType = IconType{ 
 content: r#"
<path d="M5 3v16h16"></path>
<path d="m5 19 6-6"></path>
<path d="m2 6 3-3 3 3"></path>
<path d="m18 16 3 3-3 3"></path>"#,
};

pub const MOVE_DIAGONAL_2: IconType = IconType{ 
 content: r#"
<polyline points="5 11 5 5 11 5"></polyline>
<polyline points="19 13 19 19 13 19"></polyline>
<line y1="5" x1="5" x2="19" y2="19"></line>"#,
};

pub const MOVE_DIAGONAL: IconType = IconType{ 
 content: r#"
<polyline points="13 5 19 5 19 11"></polyline>
<polyline points="11 19 5 19 5 13"></polyline>
<line y2="19" y1="5" x1="19" x2="5"></line>"#,
};

pub const MOVE_DOWN_LEFT: IconType = IconType{ 
 content: r#"
<path d="M11 19H5V13"></path>
<path d="M19 5L5 19"></path>"#,
};

pub const MOVE_DOWN_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M19 13V19H13"></path>
<path d="M5 5L19 19"></path>"#,
};

pub const MOVE_DOWN: IconType = IconType{ 
 content: r#"
<path d="M8 18L12 22L16 18"></path>
<path d="M12 2V22"></path>"#,
};

pub const MOVE_HORIZONTAL: IconType = IconType{ 
 content: r#"
<polyline points="18 8 22 12 18 16"></polyline>
<polyline points="6 8 2 12 6 16"></polyline>
<line x2="22" y1="12" x1="2" y2="12"></line>"#,
};

pub const MOVE_LEFT: IconType = IconType{ 
 content: r#"
<path d="M6 8L2 12L6 16"></path>
<path d="M2 12H22"></path>"#,
};

pub const MOVE_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M18 8L22 12L18 16"></path>
<path d="M2 12H22"></path>"#,
};

pub const MOVE_UP_LEFT: IconType = IconType{ 
 content: r#"
<path d="M5 11V5H11"></path>
<path d="M5 5L19 19"></path>"#,
};

pub const MOVE_UP_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M13 5H19V11"></path>
<path d="M19 5L5 19"></path>"#,
};

pub const MOVE_UP: IconType = IconType{ 
 content: r#"
<path d="M8 6L12 2L16 6"></path>
<path d="M12 2V22"></path>"#,
};

pub const MOVE_VERTICAL: IconType = IconType{ 
 content: r#"
<polyline points="8 18 12 22 16 18"></polyline>
<polyline points="8 6 12 2 16 6"></polyline>
<line y1="2" x2="12" x1="12" y2="22"></line>"#,
};

pub const MOVE: IconType = IconType{ 
 content: r#"
<polyline points="5 9 2 12 5 15"></polyline>
<polyline points="9 5 12 2 15 5"></polyline>
<polyline points="15 19 12 22 9 19"></polyline>
<polyline points="19 9 22 12 19 15"></polyline>
<line y2="12" x2="22" x1="2" y1="12"></line>
<line x2="12" x1="12" y1="2" y2="22"></line>"#,
};

pub const MUSIC_2: IconType = IconType{ 
 content: r#"
<circle cy="18" r="4" cx="8"></circle>
<path d="M12 18V2l7 4"></path>"#,
};

pub const MUSIC_3: IconType = IconType{ 
 content: r#"
<circle r="4" cx="12" cy="18"></circle>
<path d="M16 18V2"></path>"#,
};

pub const MUSIC_4: IconType = IconType{ 
 content: r#"
<path d="M9 18V5l12-2v13"></path>
<path d="m9 9 12-2"></path>
<circle cx="6" cy="18" r="3"></circle>
<circle r="3" cy="16" cx="18"></circle>"#,
};

pub const MUSIC: IconType = IconType{ 
 content: r#"
<path d="M9 18V5l12-2v13"></path>
<circle cx="6" cy="18" r="3"></circle>
<circle cy="16" cx="18" r="3"></circle>"#,
};

pub const NAVIGATION_2_OFF: IconType = IconType{ 
 content: r#"
<path d="M9.31 9.31 5 21l7-4 7 4-1.17-3.17"></path>
<path d="M14.53 8.88 12 2l-1.17 3.17"></path>
<line x1="2" y1="2" y2="22" x2="22"></line>"#,
};

pub const NAVIGATION_2: IconType = IconType{ 
 content: r#"
<polygon points="12 2 19 21 12 17 5 21 12 2"></polygon>"#,
};

pub const NAVIGATION_OFF: IconType = IconType{ 
 content: r#"
<path d="M8.43 8.43 3 11l8 2 2 8 2.57-5.43"></path>
<path d="M17.39 11.73 22 2l-9.73 4.61"></path>
<line x2="22" y1="2" y2="22" x1="2"></line>"#,
};

pub const NAVIGATION: IconType = IconType{ 
 content: r#"
<polygon points="3 11 22 2 13 21 11 13 3 11"></polygon>"#,
};

pub const NETWORK: IconType = IconType{ 
 content: r#"
<rect height="6" width="6" y="16" rx="1" x="16"></rect>
<rect width="6" rx="1" x="2" height="6" y="16"></rect>
<rect x="9" width="6" height="6" y="2" rx="1"></rect>
<path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"></path>
<path d="M12 12V8"></path>"#,
};

pub const NEWSPAPER: IconType = IconType{ 
 content: r#"
<path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2"></path>
<path d="M18 14h-8"></path>
<path d="M15 18h-5"></path>
<path d="M10 6h8v4h-8V6Z"></path>"#,
};

pub const NFC: IconType = IconType{ 
 content: r#"
<path d="M6 8.32a7.43 7.43 0 0 1 0 7.36"></path>
<path d="M9.46 6.21a11.76 11.76 0 0 1 0 11.58"></path>
<path d="M12.91 4.1a15.91 15.91 0 0 1 .01 15.8"></path>
<path d="M16.37 2a20.16 20.16 0 0 1 0 20"></path>"#,
};

pub const NUT_OFF: IconType = IconType{ 
 content: r#"
<path d="M12 4V2"></path>
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592a7.01 7.01 0 0 0 4.125-2.939"></path>
<path d="M19 10v3.343"></path>
<path d="M12 12c-1.349-.573-1.905-1.005-2.5-2-.546.902-1.048 1.353-2.5 2-1.018-.644-1.46-1.08-2-2-1.028.71-1.69.918-3 1 1.081-1.048 1.757-2.03 2-3 .194-.776.84-1.551 1.79-2.21m11.654 5.997c.887-.457 1.28-.891 1.556-1.787 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4-.74 0-1.461.068-2.15.192"></path>
<line x1="2" x2="22" y2="22" y1="2"></line>"#,
};

pub const NUT: IconType = IconType{ 
 content: r#"
<path d="M12 4V2"></path>
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592A7.003 7.003 0 0 0 19 14v-4"></path>
<path d="M12 4C8 4 4.5 6 4 8c-.243.97-.919 1.952-2 3 1.31-.082 1.972-.29 3-1 .54.92.982 1.356 2 2 1.452-.647 1.954-1.098 2.5-2 .595.995 1.151 1.427 2.5 2 1.31-.621 1.862-1.058 2.5-2 .629.977 1.162 1.423 2.5 2 1.209-.548 1.68-.967 2-2 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4Z"></path>"#,
};

pub const OCTAGON: IconType = IconType{ 
 content: r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>"#,
};

pub const OPTION: IconType = IconType{ 
 content: r#"
<path d="M3 3h6l6 18h6"></path>
<path d="M14 3h7"></path>"#,
};

pub const ORBIT: IconType = IconType{ 
 content: r#"
<circle r="3" cy="12" cx="12"></circle>
<circle cy="5" r="2" cx="19"></circle>
<circle cy="19" cx="5" r="2"></circle>
<path d="M10.4 21.9a10 10 0 0 0 9.941-15.416"></path>
<path d="M13.5 2.1a10 10 0 0 0-9.841 15.416"></path>"#,
};

pub const OUTDENT: IconType = IconType{ 
 content: r#"
<polyline points="7 8 3 12 7 16"></polyline>
<line x2="11" y2="12" x1="21" y1="12"></line>
<line y1="6" y2="6" x2="11" x1="21"></line>
<line x2="11" x1="21" y1="18" y2="18"></line>"#,
};

pub const PACKAGE_2: IconType = IconType{ 
 content: r#"
<path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z"></path>
<path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9"></path>
<path d="M12 3v6"></path>"#,
};

pub const PACKAGE_CHECK: IconType = IconType{ 
 content: r#"
<path d="m16 16 2 2 4-4"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line x2="12" y2="12" x1="12" y1="22"></line>"#,
};

pub const PACKAGE_MINUS: IconType = IconType{ 
 content: r#"
<path d="M16 16h6"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line x1="12" x2="12" y1="22" y2="12"></line>"#,
};

pub const PACKAGE_OPEN: IconType = IconType{ 
 content: r#"
<path d="M20.91 8.84 8.56 2.23a1.93 1.93 0 0 0-1.81 0L3.1 4.13a2.12 2.12 0 0 0-.05 3.69l12.22 6.93a2 2 0 0 0 1.94 0L21 12.51a2.12 2.12 0 0 0-.09-3.67Z"></path>
<path d="m3.09 8.84 12.35-6.61a1.93 1.93 0 0 1 1.81 0l3.65 1.9a2.12 2.12 0 0 1 .1 3.69L8.73 14.75a2 2 0 0 1-1.94 0L3 12.51a2.12 2.12 0 0 1 .09-3.67Z"></path>
<line x1="12" y1="22" x2="12" y2="13"></line>
<path d="M20 13.5v3.37a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13.5"></path>"#,
};

pub const PACKAGE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M16 16h6"></path>
<path d="M19 13v6"></path>
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y2="12" x2="12" x1="12" y1="22"></line>"#,
};

pub const PACKAGE_SEARCH: IconType = IconType{ 
 content: r#"
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y2="12" x1="12" x2="12" y1="22"></line>
<circle cx="18.5" cy="15.5" r="2.5"></circle>
<path d="M20.27 17.27 22 19"></path>"#,
};

pub const PACKAGE_X: IconType = IconType{ 
 content: r#"
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14"></path>
<path d="m7.5 4.27 9 5.15"></path>
<polyline points="3.29 7 12 12 20.71 7"></polyline>
<line y1="22" x1="12" y2="12" x2="12"></line>
<path d="m17 13 5 5m-5 0 5-5"></path>"#,
};

pub const PACKAGE: IconType = IconType{ 
 content: r#"
<path d="m7.5 4.27 9 5.15"></path>
<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"></path>
<path d="m3.3 7 8.7 5 8.7-5"></path>
<path d="M12 22V12"></path>"#,
};

pub const PAINT_BUCKET: IconType = IconType{ 
 content: r#"
<path d="m19 11-8-8-8.6 8.6a2 2 0 0 0 0 2.8l5.2 5.2c.8.8 2 .8 2.8 0L19 11Z"></path>
<path d="m5 2 5 5"></path>
<path d="M2 13h15"></path>
<path d="M22 20a2 2 0 1 1-4 0c0-1.6 1.7-2.4 2-4 .3 1.6 2 2.4 2 4Z"></path>"#,
};

pub const PAINTBRUSH_2: IconType = IconType{ 
 content: r#"
<path d="M14 19.9V16h3a2 2 0 0 0 2-2v-2H5v2c0 1.1.9 2 2 2h3v3.9a2 2 0 1 0 4 0Z"></path>
<path d="M6 12V2h12v10"></path>
<path d="M14 2v4"></path>
<path d="M10 2v2"></path>"#,
};

pub const PAINTBRUSH: IconType = IconType{ 
 content: r#"
<path d="M18.37 2.63 14 7l-1.59-1.59a2 2 0 0 0-2.82 0L8 7l9 9 1.59-1.59a2 2 0 0 0 0-2.82L17 10l4.37-4.37a2.12 2.12 0 1 0-3-3Z"></path>
<path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7"></path>
<path d="M14.5 17.5 4.5 15"></path>"#,
};

pub const PALETTE: IconType = IconType{ 
 content: r#"
<circle r=".5" cx="13.5" cy="6.5"></circle>
<circle cy="10.5" r=".5" cx="17.5"></circle>
<circle r=".5" cx="8.5" cy="7.5"></circle>
<circle cx="6.5" r=".5" cy="12.5"></circle>
<path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"></path>"#,
};

pub const PALMTREE: IconType = IconType{ 
 content: r#"
<path d="M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4"></path>
<path d="M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3"></path>
<path d="M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35z"></path>
<path d="M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14"></path>"#,
};

pub const PANEL_BOTTOM_CLOSE: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" rx="2" ry="2" width="18" y="3"></rect>
<line y2="15" x1="3" x2="21" y1="15"></line>
<path d="m15 8-3 3-3-3"></path>"#,
};

pub const PANEL_BOTTOM_INACTIVE: IconType = IconType{ 
 content: r#"
<rect x="3" rx="2" height="18" width="18" y="3"></rect>
<path d="M14 15h1"></path>
<path d="M19 15h2"></path>
<path d="M3 15h2"></path>
<path d="M9 15h1"></path>"#,
};

pub const PANEL_BOTTOM_OPEN: IconType = IconType{ 
 content: r#"
<rect y="3" rx="2" width="18" ry="2" height="18" x="3"></rect>
<line x1="3" x2="21" y1="15" y2="15"></line>
<path d="m9 10 3-3 3 3"></path>"#,
};

pub const PANEL_BOTTOM: IconType = IconType{ 
 content: r#"
<rect height="18" ry="2" rx="2" width="18" x="3" y="3"></rect>
<line x1="3" y1="15" x2="21" y2="15"></line>"#,
};

pub const PANEL_LEFT_CLOSE: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" ry="2" width="18" y="3" rx="2"></rect>
<path d="M9 3v18"></path>
<path d="m16 15-3-3 3-3"></path>"#,
};

pub const PANEL_LEFT_INACTIVE: IconType = IconType{ 
 content: r#"
<rect height="18" y="3" rx="2" width="18" x="3"></rect>
<path d="M9 14v1"></path>
<path d="M9 19v2"></path>
<path d="M9 3v2"></path>
<path d="M9 9v1"></path>"#,
};

pub const PANEL_LEFT_OPEN: IconType = IconType{ 
 content: r#"
<rect x="3" ry="2" y="3" width="18" rx="2" height="18"></rect>
<path d="M9 3v18"></path>
<path d="m14 9 3 3-3 3"></path>"#,
};

pub const PANEL_LEFT: IconType = IconType{ 
 content: r#"
<rect rx="2" width="18" y="3" ry="2" x="3" height="18"></rect>
<line y1="3" x2="9" x1="9" y2="21"></line>"#,
};

pub const PANEL_RIGHT_CLOSE: IconType = IconType{ 
 content: r#"
<rect width="18" y="3" rx="2" x="3" height="18" ry="2"></rect>
<line x2="15" y2="21" x1="15" y1="3"></line>
<path d="m8 9 3 3-3 3"></path>"#,
};

pub const PANEL_RIGHT_INACTIVE: IconType = IconType{ 
 content: r#"
<rect rx="2" height="18" width="18" x="3" y="3"></rect>
<path d="M15 14v1"></path>
<path d="M15 19v2"></path>
<path d="M15 3v2"></path>
<path d="M15 9v1"></path>"#,
};

pub const PANEL_RIGHT_OPEN: IconType = IconType{ 
 content: r#"
<rect y="3" height="18" ry="2" rx="2" x="3" width="18"></rect>
<line x1="15" y1="3" y2="21" x2="15"></line>
<path d="m10 15-3-3 3-3"></path>"#,
};

pub const PANEL_RIGHT: IconType = IconType{ 
 content: r#"
<rect ry="2" x="3" rx="2" y="3" height="18" width="18"></rect>
<line x2="15" y2="21" y1="3" x1="15"></line>"#,
};

pub const PANEL_TOP_CLOSE: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" y="3" rx="2" ry="2" x="3"></rect>
<line x1="3" y1="9" x2="21" y2="9"></line>
<path d="m9 16 3-3 3 3"></path>"#,
};

pub const PANEL_TOP_INACTIVE: IconType = IconType{ 
 content: r#"
<rect y="3" rx="2" width="18" height="18" x="3"></rect>
<path d="M14 9h1"></path>
<path d="M19 9h2"></path>
<path d="M3 9h2"></path>
<path d="M9 9h1"></path>"#,
};

pub const PANEL_TOP_OPEN: IconType = IconType{ 
 content: r#"
<rect rx="2" height="18" x="3" y="3" width="18" ry="2"></rect>
<line y2="9" x2="21" y1="9" x1="3"></line>
<path d="m15 14-3 3-3-3"></path>"#,
};

pub const PANEL_TOP: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" rx="2" ry="2" width="18" y="3"></rect>
<line y2="9" x2="21" y1="9" x1="3"></line>"#,
};

pub const PAPERCLIP: IconType = IconType{ 
 content: r#"
<path d="m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>"#,
};

pub const PARENTHESES: IconType = IconType{ 
 content: r#"
<path d="M8 21s-4-3-4-9 4-9 4-9"></path>
<path d="M16 3s4 3 4 9-4 9-4 9"></path>"#,
};

pub const PARKING_CIRCLE_OFF: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m5 5 14 14"></path>
<path d="M13 13a3 3 0 1 0 0-6H9v2"></path>
<path d="M9 17v-2.34"></path>"#,
};

pub const PARKING_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M9 17V7h4a3 3 0 0 1 0 6H9"></path>"#,
};

pub const PARKING_METER: IconType = IconType{ 
 content: r#"
<path d="M9 9a3 3 0 1 1 6 0"></path>
<path d="M12 12v3"></path>
<path d="M11 15h2"></path>
<path d="M19 9a7 7 0 1 0-13.6 2.3C6.4 14.4 8 19 8 19h8s1.6-4.6 2.6-7.7c.3-.8.4-1.5.4-2.3"></path>
<path d="M12 19v3"></path>"#,
};

pub const PARKING_SQUARE_OFF: IconType = IconType{ 
 content: r#"
<path d="M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41"></path>
<path d="M3 8.7V19a2 2 0 0 0 2 2h10.3"></path>
<path d="m2 2 20 20"></path>
<path d="M13 13a3 3 0 1 0 0-6H9v2"></path>
<path d="M9 17v-2.3"></path>"#,
};

pub const PARKING_SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" width="18" rx="2" height="18" y="3"></rect>
<path d="M9 17V7h4a3 3 0 0 1 0 6H9"></path>"#,
};

pub const PARTY_POPPER: IconType = IconType{ 
 content: r#"
<path d="M5.8 11.3 2 22l10.7-3.79"></path>
<path d="M4 3h.01"></path>
<path d="M22 8h.01"></path>
<path d="M15 2h.01"></path>
<path d="M22 20h.01"></path>
<path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12v0c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10"></path>
<path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11v0c-.11.7-.72 1.22-1.43 1.22H17"></path>
<path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98v0C9.52 4.9 9 5.52 9 6.23V7"></path>
<path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z"></path>"#,
};

pub const PAUSE_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<line y2="9" x1="10" x2="10" y1="15"></line>
<line x2="14" x1="14" y1="15" y2="9"></line>"#,
};

pub const PAUSE_OCTAGON: IconType = IconType{ 
 content: r#"
<path d="M10 15V9"></path>
<path d="M14 15V9"></path>
<path d="M7.714 2h8.572L22 7.714v8.572L16.286 22H7.714L2 16.286V7.714L7.714 2z"></path>"#,
};

pub const PAUSE: IconType = IconType{ 
 content: r#"
<rect height="16" x="6" y="4" width="4"></rect>
<rect y="4" height="16" x="14" width="4"></rect>"#,
};

pub const PAW_PRINT: IconType = IconType{ 
 content: r#"
<circle cx="11" cy="4" r="2"></circle>
<circle cx="18" cy="8" r="2"></circle>
<circle r="2" cx="20" cy="16"></circle>
<path d="M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z"></path>"#,
};

pub const PC_CASE: IconType = IconType{ 
 content: r#"
<rect height="20" rx="2" y="2" x="5" width="14"></rect>
<path d="M15 14h.01"></path>
<path d="M9 6h6"></path>
<path d="M9 10h6"></path>"#,
};

pub const PEN_LINE: IconType = IconType{ 
 content: r#"
<path d="M12 20h9"></path>
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"></path>"#,
};

pub const PEN_SQUARE: IconType = IconType{ 
 content: r#"
<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
<path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z"></path>"#,
};

pub const PEN_TOOL: IconType = IconType{ 
 content: r#"
<path d="m12 19 7-7 3 3-7 7-3-3z"></path>
<path d="m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path>
<path d="m2 2 7.586 7.586"></path>
<circle cy="11" r="2" cx="11"></circle>"#,
};

pub const PEN: IconType = IconType{ 
 content: r#"
<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>"#,
};

pub const PENCIL_LINE: IconType = IconType{ 
 content: r#"
<path d="M12 20h9"></path>
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"></path>
<path d="m15 5 3 3"></path>"#,
};

pub const PENCIL_RULER: IconType = IconType{ 
 content: r#"
<path d="m15 5 4 4"></path>
<path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13"></path>
<path d="m8 6 2-2"></path>
<path d="m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z"></path>
<path d="m18 16 2-2"></path>
<path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17"></path>"#,
};

pub const PENCIL: IconType = IconType{ 
 content: r#"
<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
<path d="m15 5 4 4"></path>"#,
};

pub const PERCENT_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#,
};

pub const PERCENT_DIAMOND: IconType = IconType{ 
 content: r#"
<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z"></path>
<path d="M9.2 9.2h.01"></path>
<path d="m14.5 9.5-5 5"></path>
<path d="M14.7 14.8h.01"></path>"#,
};

pub const PERCENT_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" x="3" y="3" rx="2"></rect>
<path d="m15 9-6 6"></path>
<path d="M9 9h.01"></path>
<path d="M15 15h.01"></path>"#,
};

pub const PERCENT: IconType = IconType{ 
 content: r#"
<line y1="5" y2="19" x2="5" x1="19"></line>
<circle cy="6.5" r="2.5" cx="6.5"></circle>
<circle cy="17.5" cx="17.5" r="2.5"></circle>"#,
};

pub const PERSON_STANDING: IconType = IconType{ 
 content: r#"
<circle r="1" cx="12" cy="5"></circle>
<path d="m9 20 3-6 3 6"></path>
<path d="m6 8 6 2 6-2"></path>
<path d="M12 10v4"></path>"#,
};

pub const PHONE_CALL: IconType = IconType{ 
 content: r#"
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
<path d="M14.05 2a9 9 0 0 1 8 7.94"></path>
<path d="M14.05 6A5 5 0 0 1 18 10"></path>"#,
};

pub const PHONE_FORWARDED: IconType = IconType{ 
 content: r#"
<polyline points="18 2 22 6 18 10"></polyline>
<line x1="14" y1="6" x2="22" y2="6"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#,
};

pub const PHONE_INCOMING: IconType = IconType{ 
 content: r#"
<polyline points="16 2 16 8 22 8"></polyline>
<line x1="22" y1="2" y2="8" x2="16"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#,
};

pub const PHONE_MISSED: IconType = IconType{ 
 content: r#"
<line x2="16" y2="8" y1="2" x1="22"></line>
<line y2="8" x2="22" y1="2" x1="16"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#,
};

pub const PHONE_OFF: IconType = IconType{ 
 content: r#"
<path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91"></path>
<line y1="2" y2="22" x1="22" x2="2"></line>"#,
};

pub const PHONE_OUTGOING: IconType = IconType{ 
 content: r#"
<polyline points="22 8 22 2 16 2"></polyline>
<line y1="8" x1="16" y2="2" x2="22"></line>
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#,
};

pub const PHONE: IconType = IconType{ 
 content: r#"
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>"#,
};

pub const PI_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" y="3" rx="2" x="3"></rect>
<path d="M7 7h10"></path>
<path d="M10 7v10"></path>
<path d="M16 17a2 2 0 0 1-2-2V7"></path>"#,
};

pub const PI: IconType = IconType{ 
 content: r#"
<line x2="9" x1="9" y2="20" y1="4"></line>
<path d="M4 7c0-1.7 1.3-3 3-3h13"></path>
<path d="M18 20c-1.7 0-3-1.3-3-3V4"></path>"#,
};

pub const PICTURE_IN_PICTURE_2: IconType = IconType{ 
 content: r#"
<path d="M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4"></path>
<rect x="12" rx="2" y="13" width="10" height="7"></rect>"#,
};

pub const PICTURE_IN_PICTURE: IconType = IconType{ 
 content: r#"
<path d="M8 4.5v5H3m-1-6 6 6m13 0v-3c0-1.16-.84-2-2-2h-7m-9 9v2c0 1.05.95 2 2 2h3"></path>
<rect width="10" x="12" y="13.5" ry="2" height="7"></rect>"#,
};

pub const PIE_CHART: IconType = IconType{ 
 content: r#"
<path d="M21.21 15.89A10 10 0 1 1 8 2.83"></path>
<path d="M22 12A10 10 0 0 0 12 2v10z"></path>"#,
};

pub const PIGGY_BANK: IconType = IconType{ 
 content: r#"
<path d="M19 5c-1.5 0-2.8 1.4-3 2-3.5-1.5-11-.3-11 5 0 1.8 0 3 2 4.5V20h4v-2h3v2h4v-4c1-.5 1.7-1 2-2h2v-4h-2c0-1-.5-1.5-1-2h0V5z"></path>
<path d="M2 9v1c0 1.1.9 2 2 2h1"></path>
<path d="M16 11h0"></path>"#,
};

pub const PILCROW_SQUARE: IconType = IconType{ 
 content: r#"
<rect y="3" height="18" width="18" x="3" rx="2"></rect>
<path d="M12 12H9.5a2.5 2.5 0 0 1 0-5H17"></path>
<path d="M12 7v10"></path>
<path d="M16 7v10"></path>"#,
};

pub const PILCROW: IconType = IconType{ 
 content: r#"
<path d="M13 4v16"></path>
<path d="M17 4v16"></path>
<path d="M19 4H9.5a4.5 4.5 0 0 0 0 9H13"></path>"#,
};

pub const PILL: IconType = IconType{ 
 content: r#"
<path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z"></path>
<path d="m8.5 8.5 7 7"></path>"#,
};

pub const PIN_OFF: IconType = IconType{ 
 content: r#"
<line x1="2" x2="22" y2="22" y1="2"></line>
<line x2="12" x1="12" y1="17" y2="22"></line>
<path d="M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12"></path>
<path d="M15 9.34V6h1a2 2 0 0 0 0-4H7.89"></path>"#,
};

pub const PIN: IconType = IconType{ 
 content: r#"
<line y1="17" x2="12" x1="12" y2="22"></line>
<path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"></path>"#,
};

pub const PIPETTE: IconType = IconType{ 
 content: r#"
<path d="m2 22 1-1h3l9-9"></path>
<path d="M3 21v-3l9-9"></path>
<path d="m15 6 3.4-3.4a2.1 2.1 0 1 1 3 3L18 9l.4.4a2.1 2.1 0 1 1-3 3l-3.8-3.8a2.1 2.1 0 1 1 3-3l.4.4Z"></path>"#,
};

pub const PIZZA: IconType = IconType{ 
 content: r#"
<path d="M15 11h.01"></path>
<path d="M11 15h.01"></path>
<path d="M16 16h.01"></path>
<path d="m2 16 20 6-6-20A20 20 0 0 0 2 16"></path>
<path d="M5.71 17.11a17.04 17.04 0 0 1 11.4-11.4"></path>"#,
};

pub const PLANE_LANDING: IconType = IconType{ 
 content: r#"
<path d="M2 22h20"></path>
<path d="M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z"></path>"#,
};

pub const PLANE_TAKEOFF: IconType = IconType{ 
 content: r#"
<path d="M2 22h20"></path>
<path d="M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z"></path>"#,
};

pub const PLANE: IconType = IconType{ 
 content: r#"
<path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z"></path>"#,
};

pub const PLAY_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cy="12" cx="12"></circle>
<polygon points="10 8 16 12 10 16 10 8"></polygon>"#,
};

pub const PLAY_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" width="18" y="3" height="18" x="3"></rect>
<path d="m9 8 6 4-6 4Z"></path>"#,
};

pub const PLAY: IconType = IconType{ 
 content: r#"
<polygon points="5 3 19 12 5 21 5 3"></polygon>"#,
};

pub const PLUG_2: IconType = IconType{ 
 content: r#"
<path d="M9 2v6"></path>
<path d="M15 2v6"></path>
<path d="M12 17v5"></path>
<path d="M5 8h14"></path>
<path d="M6 11V8h12v3a6 6 0 1 1-12 0v0Z"></path>"#,
};

pub const PLUG_ZAP_2: IconType = IconType{ 
 content: r#"
<path d="m13 2-2 2.5h3L12 7"></path>
<path d="M10 14v-3"></path>
<path d="M14 14v-3"></path>
<path d="M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z"></path>
<path d="M12 22v-3"></path>"#,
};

pub const PLUG_ZAP: IconType = IconType{ 
 content: r#"
<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"></path>
<path d="m2 22 3-3"></path>
<path d="M7.5 13.5 10 11"></path>
<path d="M10.5 16.5 13 14"></path>
<path d="m18 3-4 4h6l-4 4"></path>"#,
};

pub const PLUG: IconType = IconType{ 
 content: r#"
<path d="M12 22v-5"></path>
<path d="M9 8V2"></path>
<path d="M15 8V2"></path>
<path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z"></path>"#,
};

pub const PLUS_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cy="12" r="10" cx="12"></circle>
<path d="M8 12h8"></path>
<path d="M12 8v8"></path>"#,
};

pub const PLUS_SQUARE: IconType = IconType{ 
 content: r#"
<rect rx="2" width="18" x="3" height="18" y="3"></rect>
<path d="M8 12h8"></path>
<path d="M12 8v8"></path>"#,
};

pub const PLUS: IconType = IconType{ 
 content: r#"
<path d="M5 12h14"></path>
<path d="M12 5v14"></path>"#,
};

pub const POCKET_KNIFE: IconType = IconType{ 
 content: r#"
<path d="M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2"></path>
<path d="M18 6h.01"></path>
<path d="M6 18h.01"></path>
<path d="M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z"></path>
<path d="M18 11.66V22a4 4 0 0 0 4-4V6"></path>"#,
};

pub const POCKET: IconType = IconType{ 
 content: r#"
<path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z"></path>
<polyline points="8 10 12 14 16 10"></polyline>"#,
};

pub const PODCAST: IconType = IconType{ 
 content: r#"
<circle cy="11" r="1" cx="12"></circle>
<path d="M11 17a1 1 0 0 1 2 0c0 .5-.34 3-.5 4.5a.5.5 0 0 1-1 0c-.16-1.5-.5-4-.5-4.5Z"></path>
<path d="M8 14a5 5 0 1 1 8 0"></path>
<path d="M17 18.5a9 9 0 1 0-10 0"></path>"#,
};

pub const POINTER: IconType = IconType{ 
 content: r#"
<path d="M22 14a8 8 0 0 1-8 8"></path>
<path d="M18 11v-1a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"></path>
<path d="M14 10V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1"></path>
<path d="M10 9.5V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v10"></path>
<path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"></path>"#,
};

pub const POPCORN: IconType = IconType{ 
 content: r#"
<path d="M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4"></path>
<path d="M10 22 9 8"></path>
<path d="m14 22 1-14"></path>
<path d="M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z"></path>"#,
};

pub const POPSICLE: IconType = IconType{ 
 content: r#"
<path d="M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z"></path>
<path d="m22 22-5.5-5.5"></path>"#,
};

pub const POUND_STERLING: IconType = IconType{ 
 content: r#"
<path d="M18 7c0-5.333-8-5.333-8 0"></path>
<path d="M10 7v14"></path>
<path d="M6 21h12"></path>
<path d="M6 13h10"></path>"#,
};

pub const POWER_OFF: IconType = IconType{ 
 content: r#"
<path d="M18.36 6.64A9 9 0 0 1 20.77 15"></path>
<path d="M6.16 6.16a9 9 0 1 0 12.68 12.68"></path>
<path d="M12 2v4"></path>
<path d="m2 2 20 20"></path>"#,
};

pub const POWER: IconType = IconType{ 
 content: r#"
<path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path>
<line x2="12" y2="12" x1="12" y1="2"></line>"#,
};

pub const PRESENTATION: IconType = IconType{ 
 content: r#"
<path d="M2 3h20"></path>
<path d="M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3"></path>
<path d="m7 21 5-5 5 5"></path>"#,
};

pub const PRINTER: IconType = IconType{ 
 content: r#"
<polyline points="6 9 6 2 18 2 18 9"></polyline>
<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path>
<rect width="12" x="6" y="14" height="8"></rect>"#,
};

pub const PROJECTOR: IconType = IconType{ 
 content: r#"
<path d="M5 7 3 5"></path>
<path d="M9 6V3"></path>
<path d="m13 7 2-2"></path>
<circle cy="13" r="3" cx="9"></circle>
<path d="M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17"></path>
<path d="M16 16h2"></path>"#,
};

pub const PUZZLE: IconType = IconType{ 
 content: r#"
<path d="M19.439 7.85c-.049.322.059.648.289.878l1.568 1.568c.47.47.706 1.087.706 1.704s-.235 1.233-.706 1.704l-1.611 1.611a.98.98 0 0 1-.837.276c-.47-.07-.802-.48-.968-.925a2.501 2.501 0 1 0-3.214 3.214c.446.166.855.497.925.968a.979.979 0 0 1-.276.837l-1.61 1.61a2.404 2.404 0 0 1-1.705.707 2.402 2.402 0 0 1-1.704-.706l-1.568-1.568a1.026 1.026 0 0 0-.877-.29c-.493.074-.84.504-1.02.968a2.5 2.5 0 1 1-3.237-3.237c.464-.18.894-.527.967-1.02a1.026 1.026 0 0 0-.289-.877l-1.568-1.568A2.402 2.402 0 0 1 1.998 12c0-.617.236-1.234.706-1.704L4.23 8.77c.24-.24.581-.353.917-.303.515.077.877.528 1.073 1.01a2.5 2.5 0 1 0 3.259-3.259c-.482-.196-.933-.558-1.01-1.073-.05-.336.062-.676.303-.917l1.525-1.525A2.402 2.402 0 0 1 12 1.998c.617 0 1.234.236 1.704.706l1.568 1.568c.23.23.556.338.877.29.493-.074.84-.504 1.02-.968a2.5 2.5 0 1 1 3.237 3.237c-.464.18-.894.527-.967 1.02Z"></path>"#,
};

pub const QR_CODE: IconType = IconType{ 
 content: r#"
<rect rx="1" x="3" height="5" y="3" width="5"></rect>
<rect y="3" rx="1" width="5" height="5" x="16"></rect>
<rect width="5" y="16" x="3" height="5" rx="1"></rect>
<path d="M21 16h-3a2 2 0 0 0-2 2v3"></path>
<path d="M21 21v.01"></path>
<path d="M12 7v3a2 2 0 0 1-2 2H7"></path>
<path d="M3 12h.01"></path>
<path d="M12 3h.01"></path>
<path d="M12 16v.01"></path>
<path d="M16 12h1"></path>
<path d="M21 12v.01"></path>
<path d="M12 21v-1"></path>"#,
};

pub const QUOTE: IconType = IconType{ 
 content: r#"
<path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
<path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>"#,
};

pub const RABBIT: IconType = IconType{ 
 content: r#"
<path d="M20 8.54V4a2 2 0 1 0-4 0v3"></path>
<path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1.93 1.93 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1c-1.7 0-3 1.3-3 3"></path>
<path d="M7.61 12.53a3 3 0 1 0-1.6 4.3"></path>
<path d="M13 16a3 3 0 0 1 2.24 5"></path>
<path d="M18 12h.01"></path>"#,
};

pub const RADAR: IconType = IconType{ 
 content: r#"
<path d="M19.07 4.93A10 10 0 0 0 6.99 3.34"></path>
<path d="M4 6h.01"></path>
<path d="M2.29 9.62A10 10 0 1 0 21.31 8.35"></path>
<path d="M16.24 7.76A6 6 0 1 0 8.23 16.67"></path>
<path d="M12 18h.01"></path>
<path d="M17.99 11.66A6 6 0 0 1 15.77 16.67"></path>
<circle cx="12" cy="12" r="2"></circle>
<path d="m13.41 10.59 5.66-5.66"></path>"#,
};

pub const RADIATION: IconType = IconType{ 
 content: r#"
<path d="M12 12h0"></path>
<path d="M7.5 4.2c-.3-.5-.9-.7-1.3-.4C3.9 5.5 2.3 8.1 2 11c-.1.5.4 1 1 1h5c0-1.5.8-2.8 2-3.4-1.1-1.9-2-3.5-2.5-4.4z"></path>
<path d="M21 12c.6 0 1-.4 1-1-.3-2.9-1.8-5.5-4.1-7.1-.4-.3-1.1-.2-1.3.3-.6.9-1.5 2.5-2.6 4.3 1.2.7 2 2 2 3.5h5z"></path>
<path d="M7.5 19.8c-.3.5-.1 1.1.4 1.3 2.6 1.2 5.6 1.2 8.2 0 .5-.2.7-.8.4-1.3-.5-.9-1.4-2.5-2.5-4.3-1.2.7-2.8.7-4 0-1.1 1.8-2 3.4-2.5 4.3z"></path>"#,
};

pub const RADIO_RECEIVER: IconType = IconType{ 
 content: r#"
<path d="M5 16v2"></path>
<path d="M19 16v2"></path>
<rect x="2" y="8" rx="2" width="20" height="8"></rect>
<path d="M18 12h0"></path>"#,
};

pub const RADIO_TOWER: IconType = IconType{ 
 content: r#"
<path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9"></path>
<path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5"></path>
<circle cx="12" cy="9" r="2"></circle>
<path d="M16.2 4.8c2 2 2.26 5.11.8 7.47"></path>
<path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1"></path>
<path d="M9.5 18h5"></path>
<path d="m8 22 4-11 4 11"></path>"#,
};

pub const RADIO: IconType = IconType{ 
 content: r#"
<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9"></path>
<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5"></path>
<circle cy="12" r="2" cx="12"></circle>
<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5"></path>
<path d="M19.1 4.9C23 8.8 23 15.1 19.1 19"></path>"#,
};

pub const RAIL_SYMBOL: IconType = IconType{ 
 content: r#"
<path d="M5 15h14"></path>
<path d="M5 9h14"></path>
<path d="m14 20-5-5 6-6-5-5"></path>"#,
};

pub const RAINBOW: IconType = IconType{ 
 content: r#"
<path d="M22 17a10 10 0 0 0-20 0"></path>
<path d="M6 17a6 6 0 0 1 12 0"></path>
<path d="M10 17a2 2 0 0 1 4 0"></path>"#,
};

pub const RAT: IconType = IconType{ 
 content: r#"
<path d="M17 5c0-1.7-1.3-3-3-3s-3 1.3-3 3c0 .8.3 1.5.8 2H11c-3.9 0-7 3.1-7 7v0c0 2.2 1.8 4 4 4"></path>
<path d="M16.8 3.9c.3-.3.6-.5 1-.7 1.5-.6 3.3.1 3.9 1.6.6 1.5-.1 3.3-1.6 3.9l1.6 2.8c.2.3.2.7.2 1-.2.8-.9 1.2-1.7 1.1 0 0-1.6-.3-2.7-.6H17c-1.7 0-3 1.3-3 3"></path>
<path d="M13.2 18a3 3 0 0 0-2.2-5"></path>
<path d="M13 22H4a2 2 0 0 1 0-4h12"></path>
<path d="M16 9h.01"></path>"#,
};

pub const RATIO: IconType = IconType{ 
 content: r#"
<rect width="12" y="2" height="20" x="6" rx="2"></rect>
<rect x="2" y="6" width="20" rx="2" height="12"></rect>"#,
};

pub const RECEIPT: IconType = IconType{ 
 content: r#"
<path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1-2-1Z"></path>
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"></path>
<path d="M12 17V7"></path>"#,
};

pub const RECTANGLE_HORIZONTAL: IconType = IconType{ 
 content: r#"
<rect x="2" width="20" rx="2" height="12" y="6"></rect>"#,
};

pub const RECTANGLE_VERTICAL: IconType = IconType{ 
 content: r#"
<rect x="6" y="2" width="12" height="20" rx="2"></rect>"#,
};

pub const RECYCLE: IconType = IconType{ 
 content: r#"
<path d="M7 19H4.815a1.83 1.83 0 0 1-1.57-.881 1.785 1.785 0 0 1-.004-1.784L7.196 9.5"></path>
<path d="M11 19h8.203a1.83 1.83 0 0 0 1.556-.89 1.784 1.784 0 0 0 0-1.775l-1.226-2.12"></path>
<path d="m14 16-3 3 3 3"></path>
<path d="M8.293 13.596 7.196 9.5 3.1 10.598"></path>
<path d="m9.344 5.811 1.093-1.892A1.83 1.83 0 0 1 11.985 3a1.784 1.784 0 0 1 1.546.888l3.943 6.843"></path>
<path d="m13.378 9.633 4.096 1.098 1.097-4.096"></path>"#,
};

pub const REDO_2: IconType = IconType{ 
 content: r#"
<path d="m15 14 5-5-5-5"></path>
<path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13"></path>"#,
};

pub const REDO_DOT: IconType = IconType{ 
 content: r#"
<circle cy="17" r="1" cx="12"></circle>
<path d="M21 7v6h-6"></path>
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"></path>"#,
};

pub const REDO: IconType = IconType{ 
 content: r#"
<path d="M21 7v6h-6"></path>
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"></path>"#,
};

pub const REFRESH_CCW_DOT: IconType = IconType{ 
 content: r#"
<path d="M3 2v6h6"></path>
<path d="M21 12A9 9 0 0 0 6 5.3L3 8"></path>
<path d="M21 22v-6h-6"></path>
<path d="M3 12a9 9 0 0 0 15 6.7l3-2.7"></path>
<circle cy="12" r="1" cx="12"></circle>"#,
};

pub const REFRESH_CCW: IconType = IconType{ 
 content: r#"
<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>
<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
<path d="M16 16h5v5"></path>"#,
};

pub const REFRESH_CW_OFF: IconType = IconType{ 
 content: r#"
<path d="M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47"></path>
<path d="M8 16H3v5"></path>
<path d="M3 12C3 9.51 4 7.26 5.64 5.64"></path>
<path d="m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64"></path>
<path d="M21 12c0 1-.16 1.97-.47 2.87"></path>
<path d="M21 3v5h-5"></path>
<path d="M22 22 2 2"></path>"#,
};

pub const REFRESH_CW: IconType = IconType{ 
 content: r#"
<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
<path d="M21 3v5h-5"></path>
<path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
<path d="M8 16H3v5"></path>"#,
};

pub const REFRIGERATOR: IconType = IconType{ 
 content: r#"
<path d="M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z"></path>
<path d="M5 10h14"></path>
<path d="M15 7v6"></path>"#,
};

pub const REGEX: IconType = IconType{ 
 content: r#"
<path d="M17 3v10"></path>
<path d="m12.67 5.5 8.66 5"></path>
<path d="m12.67 10.5 8.66-5"></path>
<path d="M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z"></path>"#,
};

pub const REMOVE_FORMATTING: IconType = IconType{ 
 content: r#"
<path d="M4 7V4h16v3"></path>
<path d="M5 20h6"></path>
<path d="M13 4 8 20"></path>
<path d="m15 15 5 5"></path>
<path d="m20 15-5 5"></path>"#,
};

pub const REPEAT_1: IconType = IconType{ 
 content: r#"
<path d="m17 2 4 4-4 4"></path>
<path d="M3 11v-1a4 4 0 0 1 4-4h14"></path>
<path d="m7 22-4-4 4-4"></path>
<path d="M21 13v1a4 4 0 0 1-4 4H3"></path>
<path d="M11 10h1v4"></path>"#,
};

pub const REPEAT_2: IconType = IconType{ 
 content: r#"
<path d="m2 9 3-3 3 3"></path>
<path d="M13 18H7a2 2 0 0 1-2-2V6"></path>
<path d="m22 15-3 3-3-3"></path>
<path d="M11 6h6a2 2 0 0 1 2 2v10"></path>"#,
};

pub const REPEAT: IconType = IconType{ 
 content: r#"
<path d="m17 2 4 4-4 4"></path>
<path d="M3 11v-1a4 4 0 0 1 4-4h14"></path>
<path d="m7 22-4-4 4-4"></path>
<path d="M21 13v1a4 4 0 0 1-4 4H3"></path>"#,
};

pub const REPLACE_ALL: IconType = IconType{ 
 content: r#"
<path d="M14 4c0-1.1.9-2 2-2"></path>
<path d="M20 2c1.1 0 2 .9 2 2"></path>
<path d="M22 8c0 1.1-.9 2-2 2"></path>
<path d="M16 10c-1.1 0-2-.9-2-2"></path>
<path d="m3 7 3 3 3-3"></path>
<path d="M6 10V5c0-1.7 1.3-3 3-3h1"></path>
<rect rx="2" x="2" height="8" width="8" y="14"></rect>
<path d="M14 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>
<path d="M20 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2"></path>"#,
};

pub const REPLACE: IconType = IconType{ 
 content: r#"
<path d="M14 4c0-1.1.9-2 2-2"></path>
<path d="M20 2c1.1 0 2 .9 2 2"></path>
<path d="M22 8c0 1.1-.9 2-2 2"></path>
<path d="M16 10c-1.1 0-2-.9-2-2"></path>
<path d="m3 7 3 3 3-3"></path>
<path d="M6 10V5c0-1.7 1.3-3 3-3h1"></path>
<rect x="2" y="14" width="8" rx="2" height="8"></rect>"#,
};

pub const REPLY_ALL: IconType = IconType{ 
 content: r#"
<polyline points="7 17 2 12 7 7"></polyline>
<polyline points="12 17 7 12 12 7"></polyline>
<path d="M22 18v-2a4 4 0 0 0-4-4H7"></path>"#,
};

pub const REPLY: IconType = IconType{ 
 content: r#"
<polyline points="9 17 4 12 9 7"></polyline>
<path d="M20 18v-2a4 4 0 0 0-4-4H4"></path>"#,
};

pub const REWIND: IconType = IconType{ 
 content: r#"
<polygon points="11 19 2 12 11 5 11 19"></polygon>
<polygon points="22 19 13 12 22 5 22 19"></polygon>"#,
};

pub const ROCKET: IconType = IconType{ 
 content: r#"
<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"></path>
<path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"></path>
<path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"></path>
<path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"></path>"#,
};

pub const ROCKING_CHAIR: IconType = IconType{ 
 content: r#"
<polyline points="3.5 2 6.5 12.5 18 12.5"></polyline>
<line y2="20" x2="5.5" x1="9.5" y1="12.5"></line>
<line x2="18.5" x1="15" y1="12.5" y2="20"></line>
<path d="M2.75 18a13 13 0 0 0 18.5 0"></path>"#,
};

pub const ROLLER_COASTER: IconType = IconType{ 
 content: r#"
<path d="M6 19V5"></path>
<path d="M10 19V6.8"></path>
<path d="M14 19v-7.8"></path>
<path d="M18 5v4"></path>
<path d="M18 19v-6"></path>
<path d="M22 19V9"></path>
<path d="M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65"></path>"#,
};

pub const ROTATE_3_D: IconType = IconType{ 
 content: r#"
<path d="M16.466 7.5C15.643 4.237 13.952 2 12 2 9.239 2 7 6.477 7 12s2.239 10 5 10c.342 0 .677-.069 1-.2"></path>
<path d="m15.194 13.707 3.814 1.86-1.86 3.814"></path>
<path d="M19 15.57c-1.804.885-4.274 1.43-7 1.43-5.523 0-10-2.239-10-5s4.477-5 10-5c4.838 0 8.873 1.718 9.8 4"></path>"#,
};

pub const ROTATE_CCW: IconType = IconType{ 
 content: r#"
<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
<path d="M3 3v5h5"></path>"#,
};

pub const ROTATE_CW: IconType = IconType{ 
 content: r#"
<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
<path d="M21 3v5h-5"></path>"#,
};

pub const ROUTER: IconType = IconType{ 
 content: r#"
<rect y="14" x="2" rx="2" width="20" height="8"></rect>
<path d="M6.01 18H6"></path>
<path d="M10.01 18H10"></path>
<path d="M15 10v4"></path>
<path d="M17.84 7.17a4 4 0 0 0-5.66 0"></path>
<path d="M20.66 4.34a8 8 0 0 0-11.31 0"></path>"#,
};

pub const ROWS: IconType = IconType{ 
 content: r#"
<rect ry="2" x="3" width="18" y="3" rx="2" height="18"></rect>
<line y1="12" x2="21" x1="3" y2="12"></line>"#,
};

pub const RSS: IconType = IconType{ 
 content: r#"
<path d="M4 11a9 9 0 0 1 9 9"></path>
<path d="M4 4a16 16 0 0 1 16 16"></path>
<circle r="1" cy="19" cx="5"></circle>"#,
};

pub const RULER: IconType = IconType{ 
 content: r#"
<path d="M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z"></path>
<path d="m14.5 12.5 2-2"></path>
<path d="m11.5 9.5 2-2"></path>
<path d="m8.5 6.5 2-2"></path>
<path d="m17.5 15.5 2-2"></path>"#,
};

pub const RUSSIAN_RUBLE: IconType = IconType{ 
 content: r#"
<path d="M6 11h8a4 4 0 0 0 0-8H9v18"></path>
<path d="M6 15h8"></path>"#,
};

pub const SAILBOAT: IconType = IconType{ 
 content: r#"
<path d="M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z"></path>
<path d="M21 14 10 2 3 14h18Z"></path>
<path d="M10 2v16"></path>"#,
};

pub const SALAD: IconType = IconType{ 
 content: r#"
<path d="M7 21h10"></path>
<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z"></path>
<path d="M11.38 12a2.4 2.4 0 0 1-.4-4.77 2.4 2.4 0 0 1 3.2-2.77 2.4 2.4 0 0 1 3.47-.63 2.4 2.4 0 0 1 3.37 3.37 2.4 2.4 0 0 1-1.1 3.7 2.51 2.51 0 0 1 .03 1.1"></path>
<path d="m13 12 4-4"></path>
<path d="M10.9 7.25A3.99 3.99 0 0 0 4 10c0 .73.2 1.41.54 2"></path>"#,
};

pub const SANDWICH: IconType = IconType{ 
 content: r#"
<path d="M3 11v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3"></path>
<path d="M12 19H4a1 1 0 0 1-1-1v-2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3.83"></path>
<path d="m3 11 7.77-6.04a2 2 0 0 1 2.46 0L21 11H3Z"></path>
<path d="M12.97 19.77 7 15h12.5l-3.75 4.5a2 2 0 0 1-2.78.27Z"></path>"#,
};

pub const SATELLITE_DISH: IconType = IconType{ 
 content: r#"
<path d="M4 10a7.31 7.31 0 0 0 10 10Z"></path>
<path d="m9 15 3-3"></path>
<path d="M17 13a6 6 0 0 0-6-6"></path>
<path d="M21 13A10 10 0 0 0 11 3"></path>"#,
};

pub const SATELLITE: IconType = IconType{ 
 content: r#"
<path d="M13 7 9 3 5 7l4 4"></path>
<path d="m17 11 4 4-4 4-4-4"></path>
<path d="m8 12 4 4 6-6-4-4Z"></path>
<path d="m16 8 3-3"></path>
<path d="M9 21a6 6 0 0 0-6-6"></path>"#,
};

pub const SAVE_ALL: IconType = IconType{ 
 content: r#"
<path d="M6 4a2 2 0 0 1 2-2h10l4 4v10.2a2 2 0 0 1-2 1.8H8a2 2 0 0 1-2-2Z"></path>
<path d="M10 2v4h6"></path>
<path d="M18 18v-7h-8v7"></path>
<path d="M18 22H4a2 2 0 0 1-2-2V6"></path>"#,
};

pub const SAVE: IconType = IconType{ 
 content: r#"
<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
<polyline points="17 21 17 13 7 13 7 21"></polyline>
<polyline points="7 3 7 8 15 8"></polyline>"#,
};

pub const SCALE_3_D: IconType = IconType{ 
 content: r#"
<circle cx="19" r="2" cy="19"></circle>
<circle r="2" cy="5" cx="5"></circle>
<path d="M5 7v12h12"></path>
<path d="m5 19 6-6"></path>"#,
};

pub const SCALE: IconType = IconType{ 
 content: r#"
<path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"></path>
<path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"></path>
<path d="M7 21h10"></path>
<path d="M12 3v18"></path>
<path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2"></path>"#,
};

pub const SCALING: IconType = IconType{ 
 content: r#"
<path d="M21 3 9 15"></path>
<path d="M12 3H3v18h18v-9"></path>
<path d="M16 3h5v5"></path>
<path d="M14 15H9v-5"></path>"#,
};

pub const SCAN_FACE: IconType = IconType{ 
 content: r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<path d="M9 9h.01"></path>
<path d="M15 9h.01"></path>"#,
};

pub const SCAN_LINE: IconType = IconType{ 
 content: r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
<line y2="12" x1="7" x2="17" y1="12"></line>"#,
};

pub const SCAN: IconType = IconType{ 
 content: r#"
<path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
<path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
<path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
<path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>"#,
};

pub const SCATTER_CHART: IconType = IconType{ 
 content: r#"
<circle r=".5" cx="7.5" cy="7.5"></circle>
<circle r=".5" cx="18.5" cy="5.5"></circle>
<circle r=".5" cx="11.5" cy="11.5"></circle>
<circle cx="7.5" cy="16.5" r=".5"></circle>
<circle r=".5" cy="14.5" cx="17.5"></circle>
<path d="M3 3v18h18"></path>"#,
};

pub const SCHOOL_2: IconType = IconType{ 
 content: r#"
<circle r="1" cx="12" cy="10"></circle>
<path d="M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z"></path>
<path d="M6 17v.01"></path>
<path d="M6 13v.01"></path>
<path d="M18 17v.01"></path>
<path d="M18 13v.01"></path>
<path d="M14 22v-5a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5"></path>"#,
};

pub const SCHOOL: IconType = IconType{ 
 content: r#"
<path d="m4 6 8-4 8 4"></path>
<path d="m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2"></path>
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4"></path>
<path d="M18 5v17"></path>
<path d="M6 5v17"></path>
<circle r="2" cx="12" cy="9"></circle>"#,
};

pub const SCISSORS_LINE_DASHED: IconType = IconType{ 
 content: r#"
<path d="M5.42 9.42 8 12"></path>
<circle r="2" cx="4" cy="8"></circle>
<path d="m14 6-8.58 8.58"></path>
<circle cx="4" cy="16" r="2"></circle>
<path d="M10.8 14.8 14 18"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>"#,
};

pub const SCISSORS_SQUARE_DASHED_BOTTOM: IconType = IconType{ 
 content: r#"
<path d="M4 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2"></path>
<path d="M10 22H8"></path>
<path d="M16 22h-2"></path>
<circle r="2" cx="8" cy="8"></circle>
<path d="M9.414 9.414 12 12"></path>
<path d="M14.8 14.8 18 18"></path>
<circle cy="16" r="2" cx="8"></circle>
<path d="m18 6-8.586 8.586"></path>"#,
};

pub const SCISSORS_SQUARE: IconType = IconType{ 
 content: r#"
<rect y="2" rx="2" x="2" width="20" height="20"></rect>
<circle cx="8" cy="8" r="2"></circle>
<path d="M9.414 9.414 12 12"></path>
<path d="M14.8 14.8 18 18"></path>
<circle r="2" cy="16" cx="8"></circle>
<path d="m18 6-8.586 8.586"></path>"#,
};

pub const SCISSORS: IconType = IconType{ 
 content: r#"
<circle cx="6" cy="6" r="3"></circle>
<path d="M8.12 8.12 12 12"></path>
<path d="M20 4 8.12 15.88"></path>
<circle cy="18" cx="6" r="3"></circle>
<path d="M14.8 14.8 20 20"></path>"#,
};

pub const SCREEN_SHARE_OFF: IconType = IconType{ 
 content: r#"
<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m22 3-5 5"></path>
<path d="m17 3 5 5"></path>"#,
};

pub const SCREEN_SHARE: IconType = IconType{ 
 content: r#"
<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>
<path d="m17 8 5-5"></path>
<path d="M17 3h5v5"></path>"#,
};

pub const SCROLL_TEXT: IconType = IconType{ 
 content: r#"
<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4"></path>
<path d="M19 17V5a2 2 0 0 0-2-2H4"></path>
<path d="M15 8h-5"></path>
<path d="M15 12h-5"></path>"#,
};

pub const SCROLL: IconType = IconType{ 
 content: r#"
<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4"></path>
<path d="M19 17V5a2 2 0 0 0-2-2H4"></path>"#,
};

pub const SEARCH_CHECK: IconType = IconType{ 
 content: r#"
<path d="m8 11 2 2 4-4"></path>
<circle cy="11" r="8" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#,
};

pub const SEARCH_CODE: IconType = IconType{ 
 content: r#"
<path d="m9 9-2 2 2 2"></path>
<path d="m13 13 2-2-2-2"></path>
<circle r="8" cx="11" cy="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#,
};

pub const SEARCH_SLASH: IconType = IconType{ 
 content: r#"
<path d="m13.5 8.5-5 5"></path>
<circle r="8" cy="11" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#,
};

pub const SEARCH_X: IconType = IconType{ 
 content: r#"
<path d="m13.5 8.5-5 5"></path>
<path d="m8.5 8.5 5 5"></path>
<circle cx="11" r="8" cy="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#,
};

pub const SEARCH: IconType = IconType{ 
 content: r#"
<circle cy="11" r="8" cx="11"></circle>
<path d="m21 21-4.3-4.3"></path>"#,
};

pub const SEND_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="m3 3 3 9-3 9 19-9Z"></path>
<path d="M6 12h16"></path>"#,
};

pub const SEND_TO_BACK: IconType = IconType{ 
 content: r#"
<rect rx="2" y="14" width="8" x="14" height="8"></rect>
<rect y="2" x="2" width="8" rx="2" height="8"></rect>
<path d="M7 14v1a2 2 0 0 0 2 2h1"></path>
<path d="M14 7h1a2 2 0 0 1 2 2v1"></path>"#,
};

pub const SEND: IconType = IconType{ 
 content: r#"
<path d="m22 2-7 20-4-9-9-4Z"></path>
<path d="M22 2 11 13"></path>"#,
};

pub const SEPARATOR_HORIZONTAL: IconType = IconType{ 
 content: r#"
<line y2="12" x1="3" x2="21" y1="12"></line>
<polyline points="8 8 12 4 16 8"></polyline>
<polyline points="16 16 12 20 8 16"></polyline>"#,
};

pub const SEPARATOR_VERTICAL: IconType = IconType{ 
 content: r#"
<line x2="12" x1="12" y1="3" y2="21"></line>
<polyline points="8 8 4 12 8 16"></polyline>
<polyline points="16 16 20 12 16 8"></polyline>"#,
};

pub const SERVER_COG: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="3"></circle>
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
<path d="m13.4 8.3-.3.9"></path>"#,
};

pub const SERVER_CRASH: IconType = IconType{ 
 content: r#"
<path d="M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2"></path>
<path d="M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2"></path>
<path d="M6 6h.01"></path>
<path d="M6 18h.01"></path>
<path d="m13 6-4 6h6l-4 6"></path>"#,
};

pub const SERVER_OFF: IconType = IconType{ 
 content: r#"
<path d="M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5"></path>
<path d="M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z"></path>
<path d="M22 17v-1a2 2 0 0 0-2-2h-1"></path>
<path d="M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z"></path>
<path d="M6 18h.01"></path>
<path d="m2 2 20 20"></path>"#,
};

pub const SERVER: IconType = IconType{ 
 content: r#"
<rect width="20" ry="2" x="2" y="2" height="8" rx="2"></rect>
<rect x="2" y="14" rx="2" width="20" ry="2" height="8"></rect>
<line y1="6" x1="6" x2="6.01" y2="6"></line>
<line y1="18" x2="6.01" x1="6" y2="18"></line>"#,
};

pub const SETTINGS_2: IconType = IconType{ 
 content: r#"
<path d="M20 7h-9"></path>
<path d="M14 17H5"></path>
<circle cy="17" r="3" cx="17"></circle>
<circle cx="7" cy="7" r="3"></circle>"#,
};

pub const SETTINGS: IconType = IconType{ 
 content: r#"
<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
<circle r="3" cx="12" cy="12"></circle>"#,
};

pub const SHAPES: IconType = IconType{ 
 content: r#"
<path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z"></path>
<rect rx="1" width="7" x="3" y="14" height="7"></rect>
<circle cx="17.5" cy="17.5" r="3.5"></circle>"#,
};

pub const SHARE_2: IconType = IconType{ 
 content: r#"
<circle cx="18" cy="5" r="3"></circle>
<circle cy="12" cx="6" r="3"></circle>
<circle cy="19" r="3" cx="18"></circle>
<line x2="15.42" y2="17.49" y1="13.51" x1="8.59"></line>
<line x1="15.41" y2="10.49" x2="8.59" y1="6.51"></line>"#,
};

pub const SHARE: IconType = IconType{ 
 content: r#"
<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
<polyline points="16 6 12 2 8 6"></polyline>
<line y1="2" x1="12" x2="12" y2="15"></line>"#,
};

pub const SHEET: IconType = IconType{ 
 content: r#"
<rect y="3" height="18" x="3" ry="2" width="18" rx="2"></rect>
<line y2="9" x1="3" y1="9" x2="21"></line>
<line x1="3" y2="15" x2="21" y1="15"></line>
<line x1="9" y1="9" y2="21" x2="9"></line>
<line x2="15" y1="9" y2="21" x1="15"></line>"#,
};

pub const SHELL: IconType = IconType{ 
 content: r#"
<path d="M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44"></path>"#,
};

pub const SHIELD_ALERT: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M12 8v4"></path>
<path d="M12 16h.01"></path>"#,
};

pub const SHIELD_BAN: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m4 5 14 12"></path>"#,
};

pub const SHIELD_CHECK: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m9 12 2 2 4-4"></path>"#,
};

pub const SHIELD_ELLIPSIS: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h.01"></path>
<path d="M12 11h.01"></path>
<path d="M16 11h.01"></path>"#,
};

pub const SHIELD_HALF: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M12 22V2"></path>"#,
};

pub const SHIELD_MINUS: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h8"></path>"#,
};

pub const SHIELD_OFF: IconType = IconType{ 
 content: r#"
<path d="M19.7 14a6.9 6.9 0 0 0 .3-2V5l-8-3-3.2 1.2"></path>
<path d="m2 2 20 20"></path>
<path d="M4.7 4.7 4 5v7c0 6 8 10 8 10a20.3 20.3 0 0 0 5.62-4.38"></path>"#,
};

pub const SHIELD_PLUS: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M8 11h8"></path>
<path d="M12 15V7"></path>"#,
};

pub const SHIELD_QUESTION: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3"></path>
<path d="M12 17h.01"></path>"#,
};

pub const SHIELD_X: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>
<path d="m14.5 9-5 5"></path>
<path d="m9.5 9 5 5"></path>"#,
};

pub const SHIELD: IconType = IconType{ 
 content: r#"
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"></path>"#,
};

pub const SHIP_WHEEL: IconType = IconType{ 
 content: r#"
<circle r="8" cy="12" cx="12"></circle>
<path d="M12 2v7.5"></path>
<path d="m19 5-5.23 5.23"></path>
<path d="M22 12h-7.5"></path>
<path d="m19 19-5.23-5.23"></path>
<path d="M12 14.5V22"></path>
<path d="M10.23 13.77 5 19"></path>
<path d="M9.5 12H2"></path>
<path d="M10.23 10.23 5 5"></path>
<circle cy="12" r="2.5" cx="12"></circle>"#,
};

pub const SHIP: IconType = IconType{ 
 content: r#"
<path d="M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1 .6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M19.38 20A11.6 11.6 0 0 0 21 14l-9-4-9 4c0 2.9.94 5.34 2.81 7.76"></path>
<path d="M19 13V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v6"></path>
<path d="M12 10v4"></path>
<path d="M12 2v3"></path>"#,
};

pub const SHIRT: IconType = IconType{ 
 content: r#"
<path d="M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z"></path>"#,
};

pub const SHOPPING_BAG: IconType = IconType{ 
 content: r#"
<path d="M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z"></path>
<path d="M3 6h18"></path>
<path d="M16 10a4 4 0 0 1-8 0"></path>"#,
};

pub const SHOPPING_BASKET: IconType = IconType{ 
 content: r#"
<path d="m5 11 4-7"></path>
<path d="m19 11-4-7"></path>
<path d="M2 11h20"></path>
<path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8c.9 0 1.8-.7 2-1.6l1.7-7.4"></path>
<path d="m9 11 1 9"></path>
<path d="M4.5 15.5h15"></path>
<path d="m15 11-1 9"></path>"#,
};

pub const SHOPPING_CART: IconType = IconType{ 
 content: r#"
<circle r="1" cx="8" cy="21"></circle>
<circle cx="19" cy="21" r="1"></circle>
<path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12"></path>"#,
};

pub const SHOVEL: IconType = IconType{ 
 content: r#"
<path d="M2 22v-5l5-5 5 5-5 5z"></path>
<path d="M9.5 14.5 16 8"></path>
<path d="m17 2 5 5-.5.5a3.53 3.53 0 0 1-5 0s0 0 0 0a3.53 3.53 0 0 1 0-5L17 2"></path>"#,
};

pub const SHOWER_HEAD: IconType = IconType{ 
 content: r#"
<path d="m4 4 2.5 2.5"></path>
<path d="M13.5 6.5a4.95 4.95 0 0 0-7 7"></path>
<path d="M15 5 5 15"></path>
<path d="M14 17v.01"></path>
<path d="M10 16v.01"></path>
<path d="M13 13v.01"></path>
<path d="M16 10v.01"></path>
<path d="M11 20v.01"></path>
<path d="M17 14v.01"></path>
<path d="M20 11v.01"></path>"#,
};

pub const SHRINK: IconType = IconType{ 
 content: r#"
<path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8"></path>
<path d="M9 19.8V15m0 0H4.2M9 15l-6 6"></path>
<path d="M15 4.2V9m0 0h4.8M15 9l6-6"></path>
<path d="M9 4.2V9m0 0H4.2M9 9 3 3"></path>"#,
};

pub const SHRUB: IconType = IconType{ 
 content: r#"
<path d="M12 22v-7l-2-2"></path>
<path d="M17 8v.8A6 6 0 0 1 13.8 20v0H10v0A6.5 6.5 0 0 1 7 8h0a5 5 0 0 1 10 0Z"></path>
<path d="m14 14-2 2"></path>"#,
};

pub const SHUFFLE: IconType = IconType{ 
 content: r#"
<path d="M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22"></path>
<path d="m18 2 4 4-4 4"></path>
<path d="M2 6h1.9c1.5 0 2.9.9 3.6 2.2"></path>
<path d="M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8"></path>
<path d="m18 14 4 4-4 4"></path>"#,
};

pub const SIGMA_SQUARE: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" rx="2" y="3" x="3"></rect>
<path d="M16 8.9V7H8l4 5-4 5h8v-1.9"></path>"#,
};

pub const SIGMA: IconType = IconType{ 
 content: r#"
<path d="M18 7V4H6l6 8-6 8h12v-3"></path>"#,
};

pub const SIGNAL_HIGH: IconType = IconType{ 
 content: r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>
<path d="M17 20V8"></path>"#,
};

pub const SIGNAL_LOW: IconType = IconType{ 
 content: r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>"#,
};

pub const SIGNAL_MEDIUM: IconType = IconType{ 
 content: r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>"#,
};

pub const SIGNAL_ZERO: IconType = IconType{ 
 content: r#"
<path d="M2 20h.01"></path>"#,
};

pub const SIGNAL: IconType = IconType{ 
 content: r#"
<path d="M2 20h.01"></path>
<path d="M7 20v-4"></path>
<path d="M12 20v-8"></path>
<path d="M17 20V8"></path>
<path d="M22 4v16"></path>"#,
};

pub const SIREN: IconType = IconType{ 
 content: r#"
<path d="M7 12a5 5 0 0 1 5-5v0a5 5 0 0 1 5 5v6H7v-6Z"></path>
<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v2H5v-2Z"></path>
<path d="M21 12h1"></path>
<path d="M18.5 4.5 18 5"></path>
<path d="M2 12h1"></path>
<path d="M12 2v1"></path>
<path d="m4.929 4.929.707.707"></path>
<path d="M12 12v6"></path>"#,
};

pub const SKIP_BACK: IconType = IconType{ 
 content: r#"
<polygon points="19 20 9 12 19 4 19 20"></polygon>
<line y1="19" x2="5" y2="5" x1="5"></line>"#,
};

pub const SKIP_FORWARD: IconType = IconType{ 
 content: r#"
<polygon points="5 4 15 12 5 20 5 4"></polygon>
<line y2="19" x2="19" y1="5" x1="19"></line>"#,
};

pub const SKULL: IconType = IconType{ 
 content: r#"
<circle r="1" cx="9" cy="12"></circle>
<circle cx="15" cy="12" r="1"></circle>
<path d="M8 20v2h8v-2"></path>
<path d="m12.5 17-.5-1-.5 1h1z"></path>
<path d="M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20"></path>"#,
};

pub const SLACK: IconType = IconType{ 
 content: r#"
<rect width="3" x="13" height="8" y="2" rx="1.5"></rect>
<path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5"></path>
<rect y="14" x="8" rx="1.5" width="3" height="8"></rect>
<path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5"></path>
<rect x="14" width="8" height="3" y="13" rx="1.5"></rect>
<path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5"></path>
<rect rx="1.5" x="2" width="8" height="3" y="8"></rect>
<path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5"></path>"#,
};

pub const SLASH: IconType = IconType{ 
 content: r#"
<path d="M22 2 2 22"></path>"#,
};

pub const SLICE: IconType = IconType{ 
 content: r#"
<path d="m8 14-6 6h9v-3"></path>
<path d="M18.37 3.63 8 14l3 3L21.37 6.63a2.12 2.12 0 1 0-3-3Z"></path>"#,
};

pub const SLIDERS_HORIZONTAL: IconType = IconType{ 
 content: r#"
<line y2="4" x1="21" x2="14" y1="4"></line>
<line x1="10" y2="4" x2="3" y1="4"></line>
<line x2="12" y1="12" x1="21" y2="12"></line>
<line x1="8" y1="12" x2="3" y2="12"></line>
<line x2="16" x1="21" y2="20" y1="20"></line>
<line x1="12" x2="3" y1="20" y2="20"></line>
<line y1="2" x2="14" y2="6" x1="14"></line>
<line x2="8" y2="14" y1="10" x1="8"></line>
<line x2="16" y1="18" x1="16" y2="22"></line>"#,
};

pub const SLIDERS: IconType = IconType{ 
 content: r#"
<line x1="4" y1="21" y2="14" x2="4"></line>
<line y1="10" y2="3" x2="4" x1="4"></line>
<line x2="12" x1="12" y1="21" y2="12"></line>
<line y2="3" x1="12" y1="8" x2="12"></line>
<line x2="20" x1="20" y2="16" y1="21"></line>
<line x2="20" x1="20" y2="3" y1="12"></line>
<line x1="2" x2="6" y1="14" y2="14"></line>
<line x2="14" x1="10" y1="8" y2="8"></line>
<line y1="16" y2="16" x2="22" x1="18"></line>"#,
};

pub const SMARTPHONE_CHARGING: IconType = IconType{ 
 content: r#"
<rect height="20" width="14" x="5" rx="2" y="2" ry="2"></rect>
<path d="M12.667 8 10 12h4l-2.667 4"></path>"#,
};

pub const SMARTPHONE_NFC: IconType = IconType{ 
 content: r#"
<rect width="7" x="2" rx="1" y="6" height="12"></rect>
<path d="M13 8.32a7.43 7.43 0 0 1 0 7.36"></path>
<path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58"></path>
<path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8"></path>"#,
};

pub const SMARTPHONE: IconType = IconType{ 
 content: r#"
<rect height="20" x="5" y="2" width="14" rx="2" ry="2"></rect>
<path d="M12 18h.01"></path>"#,
};

pub const SMILE_PLUS: IconType = IconType{ 
 content: r#"
<path d="M22 11v1a10 10 0 1 1-9-10"></path>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<line x2="9.01" y1="9" y2="9" x1="9"></line>
<line y1="9" y2="9" x2="15.01" x1="15"></line>
<path d="M16 5h6"></path>
<path d="M19 2v6"></path>"#,
};

pub const SMILE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
<line x1="9" x2="9.01" y1="9" y2="9"></line>
<line x1="15" y2="9" x2="15.01" y1="9"></line>"#,
};

pub const SNAIL: IconType = IconType{ 
 content: r#"
<path d="M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0"></path>
<circle r="8" cx="10" cy="13"></circle>
<path d="M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6"></path>
<path d="M18 3 19.1 5.2"></path>
<path d="M22 3 20.9 5.2"></path>"#,
};

pub const SNOWFLAKE: IconType = IconType{ 
 content: r#"
<line y1="12" x2="22" x1="2" y2="12"></line>
<line x1="12" x2="12" y1="2" y2="22"></line>
<path d="m20 16-4-4 4-4"></path>
<path d="m4 8 4 4-4 4"></path>
<path d="m16 4-4 4-4-4"></path>
<path d="m8 20 4-4 4 4"></path>"#,
};

pub const SOFA: IconType = IconType{ 
 content: r#"
<path d="M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3"></path>
<path d="M2 11v5a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H6v-2a2 2 0 0 0-4 0Z"></path>
<path d="M4 18v2"></path>
<path d="M20 18v2"></path>
<path d="M12 4v9"></path>"#,
};

pub const SOUP: IconType = IconType{ 
 content: r#"
<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z"></path>
<path d="M7 21h10"></path>
<path d="M19.5 12 22 6"></path>
<path d="M16.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.73 1.62"></path>
<path d="M11.25 3c.27.1.8.53.74 1.36-.05.83-.93 1.2-.98 2.02-.06.78.33 1.24.72 1.62"></path>
<path d="M6.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.74 1.62"></path>"#,
};

pub const SPACE: IconType = IconType{ 
 content: r#"
<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"></path>"#,
};

pub const SPADE: IconType = IconType{ 
 content: r#"
<path d="M5 9c-1.5 1.5-3 3.2-3 5.5A5.5 5.5 0 0 0 7.5 20c1.8 0 3-.5 4.5-2 1.5 1.5 2.7 2 4.5 2a5.5 5.5 0 0 0 5.5-5.5c0-2.3-1.5-4-3-5.5l-7-7-7 7Z"></path>
<path d="M12 18v4"></path>"#,
};

pub const SPARKLE: IconType = IconType{ 
 content: r#"
<path d="m12 3-1.9 5.8a2 2 0 0 1-1.287 1.288L3 12l5.8 1.9a2 2 0 0 1 1.288 1.287L12 21l1.9-5.8a2 2 0 0 1 1.287-1.288L21 12l-5.8-1.9a2 2 0 0 1-1.288-1.287Z"></path>"#,
};

pub const SPARKLES: IconType = IconType{ 
 content: r#"
<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"></path>
<path d="M5 3v4"></path>
<path d="M19 17v4"></path>
<path d="M3 5h4"></path>
<path d="M17 19h4"></path>"#,
};

pub const SPEAKER: IconType = IconType{ 
 content: r#"
<rect ry="2" width="16" y="2" height="20" x="4" rx="2"></rect>
<circle cy="14" cx="12" r="4"></circle>
<line x2="12.01" x1="12" y1="6" y2="6"></line>"#,
};

pub const SPELL_CHECK_2: IconType = IconType{ 
 content: r#"
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>
<path d="M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1"></path>"#,
};

pub const SPELL_CHECK: IconType = IconType{ 
 content: r#"
<path d="m6 16 6-12 6 12"></path>
<path d="M8 12h8"></path>
<path d="m16 20 2 2 4-4"></path>"#,
};

pub const SPLINE: IconType = IconType{ 
 content: r#"
<circle cx="19" r="2" cy="5"></circle>
<circle r="2" cx="5" cy="19"></circle>
<path d="M5 17A12 12 0 0 1 17 5"></path>"#,
};

pub const SPLIT_SQUARE_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3"></path>
<path d="M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3"></path>
<line x1="12" x2="12" y1="4" y2="20"></line>"#,
};

pub const SPLIT_SQUARE_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3"></path>
<path d="M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3"></path>
<line y1="12" x1="4" x2="20" y2="12"></line>"#,
};

pub const SPLIT: IconType = IconType{ 
 content: r#"
<path d="M16 3h5v5"></path>
<path d="M8 3H3v5"></path>
<path d="M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3"></path>
<path d="m15 9 6-6"></path>"#,
};

pub const SPRAY_CAN: IconType = IconType{ 
 content: r#"
<path d="M3 3h.01"></path>
<path d="M7 5h.01"></path>
<path d="M11 7h.01"></path>
<path d="M3 7h.01"></path>
<path d="M7 9h.01"></path>
<path d="M3 11h.01"></path>
<rect height="4" x="15" y="5" width="4"></rect>
<path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2"></path>
<path d="m13 14 8-2"></path>
<path d="m13 19 8-2"></path>"#,
};

pub const SPROUT: IconType = IconType{ 
 content: r#"
<path d="M7 20h10"></path>
<path d="M10 20c5.5-2.5.8-6.4 3-10"></path>
<path d="M9.5 9.4c1.1.8 1.8 2.2 2.3 3.7-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2 2.8-.5 4.4 0 5.5.8z"></path>
<path d="M14.1 6a7 7 0 0 0-1.1 4c1.9-.1 3.3-.6 4.3-1.4 1-1 1.6-2.3 1.7-4.6-2.7.1-4 1-4.9 2z"></path>"#,
};

pub const SQUARE_ASTERISK: IconType = IconType{ 
 content: r#"
<rect height="18" x="3" width="18" y="3" rx="2"></rect>
<path d="M12 8v8"></path>
<path d="m8.5 14 7-4"></path>
<path d="m8.5 10 7 4"></path>"#,
};

pub const SQUARE_CODE: IconType = IconType{ 
 content: r#"
<rect rx="2" width="18" y="3" height="18" x="3"></rect>
<path d="m10 10-2 2 2 2"></path>
<path d="m14 14 2-2-2-2"></path>"#,
};

pub const SQUARE_DASHED_BOTTOM_CODE: IconType = IconType{ 
 content: r#"
<path d="m10 10-2 2 2 2"></path>
<path d="m14 14 2-2-2-2"></path>
<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>"#,
};

pub const SQUARE_DASHED_BOTTOM: IconType = IconType{ 
 content: r#"
<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"></path>
<path d="M9 21h1"></path>
<path d="M14 21h1"></path>"#,
};

pub const SQUARE_DOT: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" x="3" y="3" rx="2"></rect>
<circle cx="12" cy="12" r="1"></circle>"#,
};

pub const SQUARE_EQUAL: IconType = IconType{ 
 content: r#"
<rect width="18" height="18" y="3" rx="2" x="3"></rect>
<path d="M7 10h10"></path>
<path d="M7 14h10"></path>"#,
};

pub const SQUARE_SLASH: IconType = IconType{ 
 content: r#"
<rect width="18" rx="2" y="3" height="18" x="3"></rect>
<line y2="9" x2="15" y1="15" x1="9"></line>"#,
};

pub const SQUARE_STACK: IconType = IconType{ 
 content: r#"
<path d="M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"></path>
<path d="M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"></path>
<rect x="14" width="8" rx="2" height="8" y="14"></rect>"#,
};

pub const SQUARE: IconType = IconType{ 
 content: r#"
<rect x="3" height="18" width="18" y="3" rx="2"></rect>"#,
};

pub const SQUIRREL: IconType = IconType{ 
 content: r#"
<path d="M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10"></path>
<path d="M16 20c0-1.7 1.3-3 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4"></path>
<path d="M15.2 22a3 3 0 0 0-2.2-5"></path>
<path d="M18 13h.01"></path>"#,
};

pub const STAMP: IconType = IconType{ 
 content: r#"
<path d="M5 22h14"></path>
<path d="M19.27 13.73A2.5 2.5 0 0 0 17.5 13h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-1.5c0-.66-.26-1.3-.73-1.77Z"></path>
<path d="M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-3-3c-1.66 0-3 1-3 3s1 2 1 3.5V13"></path>"#,
};

pub const STAR_HALF: IconType = IconType{ 
 content: r#"
<path d="M12 17.8 5.8 21 7 14.1 2 9.3l7-1L12 2"></path>"#,
};

pub const STAR_OFF: IconType = IconType{ 
 content: r#"
<path d="M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43"></path>
<path d="M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91"></path>
<line x1="2" y2="22" y1="2" x2="22"></line>"#,
};

pub const STAR: IconType = IconType{ 
 content: r#"
<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>"#,
};

pub const STEP_BACK: IconType = IconType{ 
 content: r#"
<line y1="20" x2="18" x1="18" y2="4"></line>
<polygon points="14,20 4,12 14,4"></polygon>"#,
};

pub const STEP_FORWARD: IconType = IconType{ 
 content: r#"
<line x2="6" x1="6" y2="20" y1="4"></line>
<polygon points="10,4 20,12 10,20"></polygon>"#,
};

pub const STETHOSCOPE: IconType = IconType{ 
 content: r#"
<path d="M4.8 2.3A.3.3 0 1 0 5 2H4a2 2 0 0 0-2 2v5a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6V4a2 2 0 0 0-2-2h-1a.2.2 0 1 0 .3.3"></path>
<path d="M8 15v1a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6v-4"></path>
<circle cy="10" cx="20" r="2"></circle>"#,
};

pub const STICKER: IconType = IconType{ 
 content: r#"
<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"></path>
<path d="M15 3v6h6"></path>
<path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"></path>
<path d="M8 13h0"></path>
<path d="M16 13h0"></path>"#,
};

pub const STICKY_NOTE: IconType = IconType{ 
 content: r#"
<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z"></path>
<path d="M15 3v6h6"></path>"#,
};

pub const STOP_CIRCLE: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<rect width="6" x="9" y="9" height="6"></rect>"#,
};

pub const STORE: IconType = IconType{ 
 content: r#"
<path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7"></path>
<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
<path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4"></path>
<path d="M2 7h20"></path>
<path d="M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7"></path>"#,
};

pub const STRETCH_HORIZONTAL: IconType = IconType{ 
 content: r#"
<rect y="4" width="20" x="2" height="6" rx="2"></rect>
<rect y="14" x="2" rx="2" width="20" height="6"></rect>"#,
};

pub const STRETCH_VERTICAL: IconType = IconType{ 
 content: r#"
<rect height="20" width="6" x="4" y="2" rx="2"></rect>
<rect y="2" height="20" x="14" width="6" rx="2"></rect>"#,
};

pub const STRIKETHROUGH: IconType = IconType{ 
 content: r#"
<path d="M16 4H9a3 3 0 0 0-2.83 4"></path>
<path d="M14 12a4 4 0 0 1 0 8H6"></path>
<line x1="4" y1="12" y2="12" x2="20"></line>"#,
};

pub const SUBSCRIPT: IconType = IconType{ 
 content: r#"
<path d="m4 5 8 8"></path>
<path d="m12 5-8 8"></path>
<path d="M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07"></path>"#,
};

pub const SUBTITLES: IconType = IconType{ 
 content: r#"
<path d="M7 13h4"></path>
<path d="M15 13h2"></path>
<path d="M7 9h2"></path>
<path d="M13 9h4"></path>
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10Z"></path>"#,
};

pub const SUN_DIM: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="4"></circle>
<path d="M12 4h.01"></path>
<path d="M20 12h.01"></path>
<path d="M12 20h.01"></path>
<path d="M4 12h.01"></path>
<path d="M17.657 6.343h.01"></path>
<path d="M17.657 17.657h.01"></path>
<path d="M6.343 17.657h.01"></path>
<path d="M6.343 6.343h.01"></path>"#,
};

pub const SUN_MEDIUM: IconType = IconType{ 
 content: r#"
<circle r="4" cx="12" cy="12"></circle>
<path d="M12 3v1"></path>
<path d="M12 20v1"></path>
<path d="M3 12h1"></path>
<path d="M20 12h1"></path>
<path d="m18.364 5.636-.707.707"></path>
<path d="m6.343 17.657-.707.707"></path>
<path d="m5.636 5.636.707.707"></path>
<path d="m17.657 17.657.707.707"></path>"#,
};

pub const SUN_MOON: IconType = IconType{ 
 content: r#"
<path d="M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4"></path>
<path d="M12 2v2"></path>
<path d="M12 20v2"></path>
<path d="m4.9 4.9 1.4 1.4"></path>
<path d="m17.7 17.7 1.4 1.4"></path>
<path d="M2 12h2"></path>
<path d="M20 12h2"></path>
<path d="m6.3 17.7-1.4 1.4"></path>
<path d="m19.1 4.9-1.4 1.4"></path>"#,
};

pub const SUN_SNOW: IconType = IconType{ 
 content: r#"
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
<path d="m21 15-3-3 3-3"></path>"#,
};

pub const SUN: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="4"></circle>
<path d="M12 2v2"></path>
<path d="M12 20v2"></path>
<path d="m4.93 4.93 1.41 1.41"></path>
<path d="m17.66 17.66 1.41 1.41"></path>
<path d="M2 12h2"></path>
<path d="M20 12h2"></path>
<path d="m6.34 17.66-1.41 1.41"></path>
<path d="m19.07 4.93-1.41 1.41"></path>"#,
};

pub const SUNRISE: IconType = IconType{ 
 content: r#"
<path d="M12 2v8"></path>
<path d="m4.93 10.93 1.41 1.41"></path>
<path d="M2 18h2"></path>
<path d="M20 18h2"></path>
<path d="m19.07 10.93-1.41 1.41"></path>
<path d="M22 22H2"></path>
<path d="m8 6 4-4 4 4"></path>
<path d="M16 18a4 4 0 0 0-8 0"></path>"#,
};

pub const SUNSET: IconType = IconType{ 
 content: r#"
<path d="M12 10V2"></path>
<path d="m4.93 10.93 1.41 1.41"></path>
<path d="M2 18h2"></path>
<path d="M20 18h2"></path>
<path d="m19.07 10.93-1.41 1.41"></path>
<path d="M22 22H2"></path>
<path d="m16 6-4 4-4-4"></path>
<path d="M16 18a4 4 0 0 0-8 0"></path>"#,
};

pub const SUPERSCRIPT: IconType = IconType{ 
 content: r#"
<path d="m4 19 8-8"></path>
<path d="m12 19-8-8"></path>
<path d="M20 12h-4c0-1.5.442-2 1.5-2.5S20 8.334 20 7.002c0-.472-.17-.93-.484-1.29a2.105 2.105 0 0 0-2.617-.436c-.42.239-.738.614-.899 1.06"></path>"#,
};

pub const SWISS_FRANC: IconType = IconType{ 
 content: r#"
<path d="M10 21V3h8"></path>
<path d="M6 16h9"></path>
<path d="M10 9.5h7"></path>"#,
};

pub const SWITCH_CAMERA: IconType = IconType{ 
 content: r#"
<path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5"></path>
<path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5"></path>
<circle cy="12" r="3" cx="12"></circle>
<path d="m18 22-3-3 3-3"></path>
<path d="m6 2 3 3-3 3"></path>"#,
};

pub const SWORD: IconType = IconType{ 
 content: r#"
<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5"></polyline>
<line y2="13" x1="13" x2="19" y1="19"></line>
<line x1="16" x2="20" y1="16" y2="20"></line>
<line x2="21" x1="19" y1="21" y2="19"></line>"#,
};

pub const SWORDS: IconType = IconType{ 
 content: r#"
<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5"></polyline>
<line y1="19" y2="13" x2="19" x1="13"></line>
<line x2="20" y1="16" y2="20" x1="16"></line>
<line x1="19" x2="21" y1="21" y2="19"></line>
<polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5"></polyline>
<line y1="14" x2="9" y2="18" x1="5"></line>
<line x1="7" y1="17" y2="20" x2="4"></line>
<line x1="3" x2="5" y1="19" y2="21"></line>"#,
};

pub const SYRINGE: IconType = IconType{ 
 content: r#"
<path d="m18 2 4 4"></path>
<path d="m17 7 3-3"></path>
<path d="M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5"></path>
<path d="m9 11 4 4"></path>
<path d="m5 19-3 3"></path>
<path d="m14 4 6 6"></path>"#,
};

pub const TABLE_2: IconType = IconType{ 
 content: r#"
<path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18"></path>"#,
};

pub const TABLE_PROPERTIES: IconType = IconType{ 
 content: r#"
<path d="M15 3v18"></path>
<rect width="18" x="3" y="3" height="18" rx="2"></rect>
<path d="M21 9H3"></path>
<path d="M21 15H3"></path>"#,
};

pub const TABLE: IconType = IconType{ 
 content: r#"
<path d="M12 3v18"></path>
<rect width="18" height="18" x="3" y="3" rx="2"></rect>
<path d="M3 9h18"></path>
<path d="M3 15h18"></path>"#,
};

pub const TABLET_SMARTPHONE: IconType = IconType{ 
 content: r#"
<rect x="3" rx="2" y="8" width="10" height="14"></rect>
<path d="M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4"></path>
<path d="M8 18h.01"></path>"#,
};

pub const TABLET: IconType = IconType{ 
 content: r#"
<rect ry="2" width="16" height="20" x="4" rx="2" y="2"></rect>
<line y1="18" x1="12" y2="18" x2="12.01"></line>"#,
};

pub const TABLETS: IconType = IconType{ 
 content: r#"
<circle cy="7" r="5" cx="7"></circle>
<circle r="5" cy="17" cx="17"></circle>
<path d="M12 17h10"></path>
<path d="m3.46 10.54 7.08-7.08"></path>"#,
};

pub const TAG: IconType = IconType{ 
 content: r#"
<path d="M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z"></path>
<path d="M7 7h.01"></path>"#,
};

pub const TAGS: IconType = IconType{ 
 content: r#"
<path d="M9 5H2v7l6.29 6.29c.94.94 2.48.94 3.42 0l3.58-3.58c.94-.94.94-2.48 0-3.42L9 5Z"></path>
<path d="M6 9.01V9"></path>
<path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4L17 19"></path>"#,
};

pub const TALLY_1: IconType = IconType{ 
 content: r#"
<path d="M4 4v16"></path>"#,
};

pub const TALLY_2: IconType = IconType{ 
 content: r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>"#,
};

pub const TALLY_3: IconType = IconType{ 
 content: r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>"#,
};

pub const TALLY_4: IconType = IconType{ 
 content: r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>
<path d="M19 4v16"></path>"#,
};

pub const TALLY_5: IconType = IconType{ 
 content: r#"
<path d="M4 4v16"></path>
<path d="M9 4v16"></path>
<path d="M14 4v16"></path>
<path d="M19 4v16"></path>
<path d="M22 6 2 18"></path>"#,
};

pub const TARGET: IconType = IconType{ 
 content: r#"
<circle r="10" cx="12" cy="12"></circle>
<circle r="6" cx="12" cy="12"></circle>
<circle cy="12" r="2" cx="12"></circle>"#,
};

pub const TENT: IconType = IconType{ 
 content: r#"
<path d="M19 20 10 4"></path>
<path d="m5 20 9-16"></path>
<path d="M3 20h18"></path>
<path d="m12 15-3 5"></path>
<path d="m12 15 3 5"></path>"#,
};

pub const TERMINAL_SQUARE: IconType = IconType{ 
 content: r#"
<path d="m7 11 2-2-2-2"></path>
<path d="M11 13h4"></path>
<rect y="3" rx="2" ry="2" width="18" x="3" height="18"></rect>"#,
};

pub const TERMINAL: IconType = IconType{ 
 content: r#"
<polyline points="4 17 10 11 4 5"></polyline>
<line y1="19" x1="12" y2="19" x2="20"></line>"#,
};

pub const TEST_TUBE_2: IconType = IconType{ 
 content: r#"
<path d="M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01v0a2.83 2.83 0 0 1 0-4L17 3"></path>
<path d="m16 2 6 6"></path>
<path d="M12 16H4"></path>"#,
};

pub const TEST_TUBE: IconType = IconType{ 
 content: r#"
<path d="M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5h0c-1.4 0-2.5-1.1-2.5-2.5V2"></path>
<path d="M8.5 2h7"></path>
<path d="M14.5 16h-5"></path>"#,
};

pub const TEST_TUBES: IconType = IconType{ 
 content: r#"
<path d="M9 2v17.5A2.5 2.5 0 0 1 6.5 22v0A2.5 2.5 0 0 1 4 19.5V2"></path>
<path d="M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5v0a2.5 2.5 0 0 1-2.5-2.5V2"></path>
<path d="M3 2h7"></path>
<path d="M14 2h7"></path>
<path d="M9 16H4"></path>
<path d="M20 16h-5"></path>"#,
};

pub const TEXT_CURSOR_INPUT: IconType = IconType{ 
 content: r#"
<path d="M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1"></path>
<path d="M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5"></path>
<path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"></path>
<path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7"></path>
<path d="M9 7v10"></path>"#,
};

pub const TEXT_CURSOR: IconType = IconType{ 
 content: r#"
<path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1"></path>
<path d="M7 22h1a4 4 0 0 0 4-4v-1"></path>
<path d="M7 2h1a4 4 0 0 1 4 4v1"></path>"#,
};

pub const TEXT_QUOTE: IconType = IconType{ 
 content: r#"
<path d="M17 6H3"></path>
<path d="M21 12H8"></path>
<path d="M21 18H8"></path>
<path d="M3 12v6"></path>"#,
};

pub const TEXT_SELECT: IconType = IconType{ 
 content: r#"
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
<line x2="15" y2="8" x1="7" y1="8"></line>
<line x1="7" y1="12" x2="17" y2="12"></line>
<line x1="7" x2="13" y2="16" y1="16"></line>"#,
};

pub const TEXT: IconType = IconType{ 
 content: r#"
<path d="M17 6.1H3"></path>
<path d="M21 12.1H3"></path>
<path d="M15.1 18H3"></path>"#,
};

pub const THERMOMETER_SNOWFLAKE: IconType = IconType{ 
 content: r#"
<path d="M2 12h10"></path>
<path d="M9 4v16"></path>
<path d="m3 9 3 3-3 3"></path>
<path d="M12 6 9 9 6 6"></path>
<path d="m6 18 3-3 1.5 1.5"></path>
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>"#,
};

pub const THERMOMETER_SUN: IconType = IconType{ 
 content: r#"
<path d="M12 9a4 4 0 0 0-2 7.5"></path>
<path d="M12 3v2"></path>
<path d="m6.6 18.4-1.4 1.4"></path>
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>
<path d="M4 13H2"></path>
<path d="M6.34 7.34 4.93 5.93"></path>"#,
};

pub const THERMOMETER: IconType = IconType{ 
 content: r#"
<path d="M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"></path>"#,
};

pub const THUMBS_DOWN: IconType = IconType{ 
 content: r#"
<path d="M17 14V2"></path>
<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z"></path>"#,
};

pub const THUMBS_UP: IconType = IconType{ 
 content: r#"
<path d="M7 10v12"></path>
<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z"></path>"#,
};

pub const TICKET: IconType = IconType{ 
 content: r#"
<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"></path>
<path d="M13 5v2"></path>
<path d="M13 17v2"></path>
<path d="M13 11v2"></path>"#,
};

pub const TIMER_OFF: IconType = IconType{ 
 content: r#"
<path d="M10 2h4"></path>
<path d="M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7"></path>
<path d="M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2"></path>
<path d="m2 2 20 20"></path>
<path d="M12 12v-2"></path>"#,
};

pub const TIMER_RESET: IconType = IconType{ 
 content: r#"
<path d="M10 2h4"></path>
<path d="M12 14v-4"></path>
<path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6"></path>
<path d="M9 17H4v5"></path>"#,
};

pub const TIMER: IconType = IconType{ 
 content: r#"
<line y1="2" y2="2" x1="10" x2="14"></line>
<line y2="11" x1="12" x2="15" y1="14"></line>
<circle cy="14" r="8" cx="12"></circle>"#,
};

pub const TOGGLE_LEFT: IconType = IconType{ 
 content: r#"
<rect rx="6" width="20" height="12" y="6" ry="6" x="2"></rect>
<circle cx="8" cy="12" r="2"></circle>"#,
};

pub const TOGGLE_RIGHT: IconType = IconType{ 
 content: r#"
<rect y="6" ry="6" width="20" height="12" x="2" rx="6"></rect>
<circle cx="16" cy="12" r="2"></circle>"#,
};

pub const TORNADO: IconType = IconType{ 
 content: r#"
<path d="M21 4H3"></path>
<path d="M18 8H6"></path>
<path d="M19 12H9"></path>
<path d="M16 16h-6"></path>
<path d="M11 20H9"></path>"#,
};

pub const TOUCHPAD_OFF: IconType = IconType{ 
 content: r#"
<path d="M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16"></path>
<path d="M2 14h12"></path>
<path d="M22 14h-2"></path>
<path d="M12 20v-6"></path>
<path d="m2 2 20 20"></path>
<path d="M22 16V6a2 2 0 0 0-2-2H10"></path>"#,
};

pub const TOUCHPAD: IconType = IconType{ 
 content: r#"
<rect height="16" y="4" width="20" rx="2" x="2"></rect>
<path d="M2 14h20"></path>
<path d="M12 20v-6"></path>"#,
};

pub const TOWER_CONTROL: IconType = IconType{ 
 content: r#"
<path d="M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z"></path>
<path d="M8 13v9"></path>
<path d="M16 22v-9"></path>
<path d="m9 6 1 7"></path>
<path d="m15 6-1 7"></path>
<path d="M12 6V2"></path>
<path d="M13 2h-2"></path>"#,
};

pub const TOY_BRICK: IconType = IconType{ 
 content: r#"
<rect x="3" rx="1" y="8" height="12" width="18"></rect>
<path d="M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3"></path>
<path d="M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3"></path>"#,
};

pub const TRACTOR: IconType = IconType{ 
 content: r#"
<path d="M3 4h9l1 7"></path>
<path d="M4 11V4"></path>
<path d="M8 10V4"></path>
<path d="M18 5c-.6 0-1 .4-1 1v5.6"></path>
<path d="m10 11 11 .9c.6 0 .9.5.8 1.1l-.8 5h-1"></path>
<circle cx="7" cy="15" r=".5"></circle>
<circle cx="7" cy="15" r="5"></circle>
<path d="M16 18h-5"></path>
<circle cx="18" cy="18" r="2"></circle>"#,
};

pub const TRAFFIC_CONE: IconType = IconType{ 
 content: r#"
<path d="M9.3 6.2a4.55 4.55 0 0 0 5.4 0"></path>
<path d="M7.9 10.7c.9.8 2.4 1.3 4.1 1.3s3.2-.5 4.1-1.3"></path>
<path d="M13.9 3.5a1.93 1.93 0 0 0-3.8-.1l-3 10c-.1.2-.1.4-.1.6 0 1.7 2.2 3 5 3s5-1.3 5-3c0-.2 0-.4-.1-.5Z"></path>
<path d="m7.5 12.2-4.7 2.7c-.5.3-.8.7-.8 1.1s.3.8.8 1.1l7.6 4.5c.9.5 2.1.5 3 0l7.6-4.5c.7-.3 1-.7 1-1.1s-.3-.8-.8-1.1l-4.7-2.8"></path>"#,
};

pub const TRAIN_FRONT_TUNNEL: IconType = IconType{ 
 content: r#"
<path d="M2 22V12a10 10 0 1 1 20 0v10"></path>
<path d="M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8"></path>
<path d="M10 15h.01"></path>
<path d="M14 15h.01"></path>
<path d="M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z"></path>
<path d="m9 19-2 3"></path>
<path d="m15 19 2 3"></path>"#,
};

pub const TRAIN_FRONT: IconType = IconType{ 
 content: r#"
<path d="M8 3.1V7a4 4 0 0 0 8 0V3.1"></path>
<path d="m9 15-1-1"></path>
<path d="m15 15 1-1"></path>
<path d="M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z"></path>
<path d="m8 19-2 3"></path>
<path d="m16 19 2 3"></path>"#,
};

pub const TRAIN_TRACK: IconType = IconType{ 
 content: r#"
<path d="M2 17 17 2"></path>
<path d="m2 14 8 8"></path>
<path d="m5 11 8 8"></path>
<path d="m8 8 8 8"></path>
<path d="m11 5 8 8"></path>
<path d="m14 2 8 8"></path>
<path d="M7 22 22 7"></path>"#,
};

pub const TRAM_FRONT: IconType = IconType{ 
 content: r#"
<rect y="3" width="16" height="16" rx="2" x="4"></rect>
<path d="M4 11h16"></path>
<path d="M12 3v8"></path>
<path d="m8 19-2 3"></path>
<path d="m18 22-2-3"></path>
<path d="M8 15h0"></path>
<path d="M16 15h0"></path>"#,
};

pub const TRASH_2: IconType = IconType{ 
 content: r#"
<path d="M3 6h18"></path>
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
<line x2="10" x1="10" y1="11" y2="17"></line>
<line x2="14" y2="17" x1="14" y1="11"></line>"#,
};

pub const TRASH: IconType = IconType{ 
 content: r#"
<path d="M3 6h18"></path>
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>"#,
};

pub const TREE_DECIDUOUS: IconType = IconType{ 
 content: r#"
<path d="M8 19h8a4 4 0 0 0 3.8-2.8 4 4 0 0 0-1.6-4.5c1-1.1 1-2.7.4-4-.7-1.2-2.2-2-3.6-1.7a3 3 0 0 0-3-3 3 3 0 0 0-3 3c-1.4-.2-2.9.5-3.6 1.7-.7 1.3-.5 2.9.4 4a4 4 0 0 0-1.6 4.5A4 4 0 0 0 8 19Z"></path>
<path d="M12 19v3"></path>"#,
};

pub const TREE_PINE: IconType = IconType{ 
 content: r#"
<path d="m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z"></path>
<path d="M12 22v-3"></path>"#,
};

pub const TREES: IconType = IconType{ 
 content: r#"
<path d="M10 10v.2A3 3 0 0 1 8.9 16v0H5v0h0a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z"></path>
<path d="M7 16v6"></path>
<path d="M13 19v3"></path>
<path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5"></path>"#,
};

pub const TRELLO: IconType = IconType{ 
 content: r#"
<rect width="18" rx="2" height="18" x="3" y="3" ry="2"></rect>
<rect x="7" width="3" height="9" y="7"></rect>
<rect y="7" x="14" width="3" height="5"></rect>"#,
};

pub const TRENDING_DOWN: IconType = IconType{ 
 content: r#"
<polyline points="22 17 13.5 8.5 8.5 13.5 2 7"></polyline>
<polyline points="16 17 22 17 22 11"></polyline>"#,
};

pub const TRENDING_UP: IconType = IconType{ 
 content: r#"
<polyline points="22 7 13.5 15.5 8.5 10.5 2 17"></polyline>
<polyline points="16 7 22 7 22 13"></polyline>"#,
};

pub const TRIANGLE_RIGHT: IconType = IconType{ 
 content: r#"
<path d="M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z"></path>"#,
};

pub const TRIANGLE: IconType = IconType{ 
 content: r#"
<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path>"#,
};

pub const TROPHY: IconType = IconType{ 
 content: r#"
<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"></path>
<path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"></path>
<path d="M4 22h16"></path>
<path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"></path>
<path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"></path>
<path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"></path>"#,
};

pub const TRUCK: IconType = IconType{ 
 content: r#"
<path d="M5 18H3c-.6 0-1-.4-1-1V7c0-.6.4-1 1-1h10c.6 0 1 .4 1 1v11"></path>
<path d="M14 9h4l4 4v4c0 .6-.4 1-1 1h-2"></path>
<circle cx="7" cy="18" r="2"></circle>
<path d="M15 18H9"></path>
<circle cx="17" cy="18" r="2"></circle>"#,
};

pub const TURTLE: IconType = IconType{ 
 content: r#"
<path d="m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z"></path>
<path d="M4.82 7.9 8 10"></path>
<path d="M15.18 7.9 12 10"></path>
<path d="M16.93 10H20a2 2 0 0 1 0 4H2"></path>"#,
};

pub const TV_2: IconType = IconType{ 
 content: r#"
<path d="M7 21h10"></path>
<rect y="3" height="14" width="20" x="2" rx="2"></rect>"#,
};

pub const TV: IconType = IconType{ 
 content: r#"
<rect width="20" y="7" ry="2" rx="2" height="15" x="2"></rect>
<polyline points="17 2 12 7 7 2"></polyline>"#,
};

pub const TWITCH: IconType = IconType{ 
 content: r#"
<path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7"></path>"#,
};

pub const TWITTER: IconType = IconType{ 
 content: r#"
<path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z"></path>"#,
};

pub const TYPE: IconType = IconType{ 
 content: r#"
<polyline points="4 7 4 4 20 4 20 7"></polyline>
<line x1="9" y2="20" x2="15" y1="20"></line>
<line x2="12" y1="4" y2="20" x1="12"></line>"#,
};

pub const UMBRELLA: IconType = IconType{ 
 content: r#"
<path d="M22 12a10.06 10.06 1 0 0-20 0Z"></path>
<path d="M12 12v8a2 2 0 0 0 4 0"></path>
<path d="M12 2v1"></path>"#,
};

pub const UNDERLINE: IconType = IconType{ 
 content: r#"
<path d="M6 4v6a6 6 0 0 0 12 0V4"></path>
<line x1="4" x2="20" y2="20" y1="20"></line>"#,
};

pub const UNDO_2: IconType = IconType{ 
 content: r#"
<path d="M9 14 4 9l5-5"></path>
<path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11"></path>"#,
};

pub const UNDO_DOT: IconType = IconType{ 
 content: r#"
<circle r="1" cy="17" cx="12"></circle>
<path d="M3 7v6h6"></path>
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"></path>"#,
};

pub const UNDO: IconType = IconType{ 
 content: r#"
<path d="M3 7v6h6"></path>
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"></path>"#,
};

pub const UNFOLD_HORIZONTAL: IconType = IconType{ 
 content: r#"
<path d="M16 12h6"></path>
<path d="M8 12H2"></path>
<path d="M12 2v2"></path>
<path d="M12 8v2"></path>
<path d="M12 14v2"></path>
<path d="M12 20v2"></path>
<path d="m19 15 3-3-3-3"></path>
<path d="m5 9-3 3 3 3"></path>"#,
};

pub const UNFOLD_VERTICAL: IconType = IconType{ 
 content: r#"
<path d="M12 22v-6"></path>
<path d="M12 8V2"></path>
<path d="M4 12H2"></path>
<path d="M10 12H8"></path>
<path d="M16 12h-2"></path>
<path d="M22 12h-2"></path>
<path d="m15 19-3 3-3-3"></path>
<path d="m15 5-3-3-3 3"></path>"#,
};

pub const UNGROUP: IconType = IconType{ 
 content: r#"
<rect height="6" width="8" x="5" y="4" rx="1"></rect>
<rect width="8" y="14" x="11" height="6" rx="1"></rect>"#,
};

pub const UNLINK_2: IconType = IconType{ 
 content: r#"
<path d="M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2"></path>"#,
};

pub const UNLINK: IconType = IconType{ 
 content: r#"
<path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71"></path>
<path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71"></path>
<line x1="8" y1="2" y2="5" x2="8"></line>
<line y1="8" x1="2" x2="5" y2="8"></line>
<line x1="16" x2="16" y1="19" y2="22"></line>
<line y1="16" x1="19" y2="16" x2="22"></line>"#,
};

pub const UNLOCK: IconType = IconType{ 
 content: r#"
<rect ry="2" y="11" height="11" width="18" rx="2" x="3"></rect>
<path d="M7 11V7a5 5 0 0 1 9.9-1"></path>"#,
};

pub const UNPLUG: IconType = IconType{ 
 content: r#"
<path d="m19 5 3-3"></path>
<path d="m2 22 3-3"></path>
<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"></path>
<path d="M7.5 13.5 10 11"></path>
<path d="M10.5 16.5 13 14"></path>
<path d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z"></path>"#,
};

pub const UPLOAD_CLOUD: IconType = IconType{ 
 content: r#"
<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
<path d="M12 12v9"></path>
<path d="m16 16-4-4-4 4"></path>"#,
};

pub const UPLOAD: IconType = IconType{ 
 content: r#"
<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
<polyline points="17 8 12 3 7 8"></polyline>
<line x1="12" x2="12" y1="3" y2="15"></line>"#,
};

pub const USB: IconType = IconType{ 
 content: r#"
<circle cx="10" cy="7" r="1"></circle>
<circle cx="4" cy="20" r="1"></circle>
<path d="M4.7 19.3 19 5"></path>
<path d="m21 3-3 1 2 2Z"></path>
<path d="M9.26 7.68 5 12l2 5"></path>
<path d="m10 14 5 2 3.5-3.5"></path>
<path d="m18 12 1-1 1 1-1 1Z"></path>"#,
};

pub const USER_2: IconType = IconType{ 
 content: r#"
<circle cy="8" r="5" cx="12"></circle>
<path d="M20 21a8 8 0 1 0-16 0"></path>"#,
};

pub const USER_CHECK_2: IconType = IconType{ 
 content: r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle cx="8" cy="9" r="4"></circle>
<polyline points="16 11 18 13 22 9"></polyline>"#,
};

pub const USER_CHECK: IconType = IconType{ 
 content: r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cy="7" r="4" cx="9"></circle>
<polyline points="16 11 18 13 22 9"></polyline>"#,
};

pub const USER_CIRCLE_2: IconType = IconType{ 
 content: r#"
<path d="M18 20a6 6 0 0 0-12 0"></path>
<circle r="4" cx="12" cy="10"></circle>
<circle cx="12" r="10" cy="12"></circle>"#,
};

pub const USER_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cy="12" cx="12" r="10"></circle>
<circle cx="12" r="3" cy="10"></circle>
<path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662"></path>"#,
};

pub const USER_COG_2: IconType = IconType{ 
 content: r#"
<circle cy="15" r="3" cx="18"></circle>
<circle r="4" cx="8" cy="9"></circle>
<path d="M10.5 13.5A6 6 0 0 0 2 19"></path>
<path d="m21.7 16.4-.9-.3"></path>
<path d="m15.2 13.9-.9-.3"></path>
<path d="m16.6 18.7.3-.9"></path>
<path d="m19.1 12.2.3-.9"></path>
<path d="m19.6 18.7-.4-1"></path>
<path d="m16.8 12.3-.4-1"></path>
<path d="m14.3 16.6 1-.4"></path>
<path d="m20.7 13.8 1-.4"></path>"#,
};

pub const USER_COG: IconType = IconType{ 
 content: r#"
<circle cy="15" r="3" cx="18"></circle>
<circle r="4" cx="9" cy="7"></circle>
<path d="M10 15H6a4 4 0 0 0-4 4v2"></path>
<path d="m21.7 16.4-.9-.3"></path>
<path d="m15.2 13.9-.9-.3"></path>
<path d="m16.6 18.7.3-.9"></path>
<path d="m19.1 12.2.3-.9"></path>
<path d="m19.6 18.7-.4-1"></path>
<path d="m16.8 12.3-.4-1"></path>
<path d="m14.3 16.6 1-.4"></path>
<path d="m20.7 13.8 1-.4"></path>"#,
};

pub const USER_MINUS_2: IconType = IconType{ 
 content: r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cx="8" cy="9"></circle>
<line y2="11" y1="11" x1="22" x2="16"></line>"#,
};

pub const USER_MINUS: IconType = IconType{ 
 content: r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cx="9" cy="7" r="4"></circle>
<line x1="22" y1="11" y2="11" x2="16"></line>"#,
};

pub const USER_PLUS_2: IconType = IconType{ 
 content: r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cy="9" cx="8"></circle>
<line x1="19" y2="14" y1="8" x2="19"></line>
<line x2="16" y2="11" x1="22" y1="11"></line>"#,
};

pub const USER_PLUS: IconType = IconType{ 
 content: r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cx="9" cy="7" r="4"></circle>
<line x2="19" x1="19" y2="14" y1="8"></line>
<line x2="16" x1="22" y1="11" y2="11"></line>"#,
};

pub const USER_SQUARE_2: IconType = IconType{ 
 content: r#"
<path d="M18 21a6 6 0 0 0-12 0"></path>
<circle cy="11" r="4" cx="12"></circle>
<rect height="18" rx="2" y="3" x="3" width="18"></rect>"#,
};

pub const USER_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" rx="2" y="3" x="3" width="18"></rect>
<circle r="3" cx="12" cy="10"></circle>
<path d="M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2"></path>"#,
};

pub const USER_X_2: IconType = IconType{ 
 content: r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle cy="9" r="4" cx="8"></circle>
<line y2="13" x1="17" x2="22" y1="8"></line>
<line x2="17" y2="13" x1="22" y1="8"></line>"#,
};

pub const USER_X: IconType = IconType{ 
 content: r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cy="7" cx="9" r="4"></circle>
<line y2="13" x2="22" y1="8" x1="17"></line>
<line x1="22" y1="8" x2="17" y2="13"></line>"#,
};

pub const USER: IconType = IconType{ 
 content: r#"
<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
<circle cx="12" cy="7" r="4"></circle>"#,
};

pub const USERS_2: IconType = IconType{ 
 content: r#"
<path d="M14 19a6 6 0 0 0-12 0"></path>
<circle r="4" cx="8" cy="9"></circle>
<path d="M22 19a6 6 0 0 0-6-6 4 4 0 1 0 0-8"></path>"#,
};

pub const USERS: IconType = IconType{ 
 content: r#"
<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
<circle cx="9" cy="7" r="4"></circle>
<path d="M22 21v-2a4 4 0 0 0-3-3.87"></path>
<path d="M16 3.13a4 4 0 0 1 0 7.75"></path>"#,
};

pub const UTENSILS_CROSSED: IconType = IconType{ 
 content: r#"
<path d="m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8"></path>
<path d="M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7"></path>
<path d="m2.1 21.8 6.4-6.3"></path>
<path d="m19 5-7 7"></path>"#,
};

pub const UTENSILS: IconType = IconType{ 
 content: r#"
<path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2"></path>
<path d="M7 2v20"></path>
<path d="M21 15V2v0a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7"></path>"#,
};

pub const UTILITY_POLE: IconType = IconType{ 
 content: r#"
<path d="M12 2v20"></path>
<path d="M2 5h20"></path>
<path d="M3 3v2"></path>
<path d="M7 3v2"></path>
<path d="M17 3v2"></path>
<path d="M21 3v2"></path>
<path d="m19 5-7 7-7-7"></path>"#,
};

pub const VARIABLE: IconType = IconType{ 
 content: r#"
<path d="M8 21s-4-3-4-9 4-9 4-9"></path>
<path d="M16 3s4 3 4 9-4 9-4 9"></path>
<line y1="9" y2="15" x1="15" x2="9"></line>
<line x1="9" y1="9" y2="15" x2="15"></line>"#,
};

pub const VEGAN: IconType = IconType{ 
 content: r#"
<path d="M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14"></path>
<path d="M16 8c4 0 6-2 6-6-4 0-6 2-6 6"></path>
<path d="M17.41 3.6a10 10 0 1 0 3 3"></path>"#,
};

pub const VENETIAN_MASK: IconType = IconType{ 
 content: r#"
<path d="M2 12a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V7h-5a8 8 0 0 0-5 2 8 8 0 0 0-5-2H2Z"></path>
<path d="M6 11c1.5 0 3 .5 3 2-2 0-3 0-3-2Z"></path>
<path d="M18 11c-1.5 0-3 .5-3 2 2 0 3 0 3-2Z"></path>"#,
};

pub const VIBRATE_OFF: IconType = IconType{ 
 content: r#"
<path d="m2 8 2 2-2 2 2 2-2 2"></path>
<path d="m22 8-2 2 2 2-2 2 2 2"></path>
<path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2"></path>
<path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34"></path>
<line x1="2" y1="2" x2="22" y2="22"></line>"#,
};

pub const VIBRATE: IconType = IconType{ 
 content: r#"
<path d="m2 8 2 2-2 2 2 2-2 2"></path>
<path d="m22 8-2 2 2 2-2 2 2 2"></path>
<rect y="5" width="8" x="8" height="14" rx="1"></rect>"#,
};

pub const VIDEO_OFF: IconType = IconType{ 
 content: r#"
<path d="M10.66 6H14a2 2 0 0 1 2 2v2.34l1 1L22 8v8"></path>
<path d="M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2l10 10Z"></path>
<line y2="22" x2="22" y1="2" x1="2"></line>"#,
};

pub const VIDEO: IconType = IconType{ 
 content: r#"
<path d="m22 8-6 4 6 4V8Z"></path>
<rect y="6" height="12" rx="2" width="14" x="2" ry="2"></rect>"#,
};

pub const VIDEOTAPE: IconType = IconType{ 
 content: r#"
<rect height="16" y="4" width="20" x="2" rx="2"></rect>
<path d="M2 8h20"></path>
<circle r="2" cx="8" cy="14"></circle>
<path d="M8 12h8"></path>
<circle r="2" cx="16" cy="14"></circle>"#,
};

pub const VIEW: IconType = IconType{ 
 content: r#"
<path d="M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z"></path>
<path d="M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"></path>
<path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2"></path>
<path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2"></path>"#,
};

pub const VOICEMAIL: IconType = IconType{ 
 content: r#"
<circle cx="6" cy="12" r="4"></circle>
<circle r="4" cy="12" cx="18"></circle>
<line x1="6" x2="18" y1="16" y2="16"></line>"#,
};

pub const VOLUME_1: IconType = IconType{ 
 content: r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>"#,
};

pub const VOLUME_2: IconType = IconType{ 
 content: r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
<path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>"#,
};

pub const VOLUME_X: IconType = IconType{ 
 content: r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
<line x2="16" y2="15" x1="22" y1="9"></line>
<line y1="9" y2="15" x2="22" x1="16"></line>"#,
};

pub const VOLUME: IconType = IconType{ 
 content: r#"
<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>"#,
};

pub const VOTE: IconType = IconType{ 
 content: r#"
<path d="m9 12 2 2 4-4"></path>
<path d="M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z"></path>
<path d="M22 19H2"></path>"#,
};

pub const WALLET_2: IconType = IconType{ 
 content: r#"
<path d="M17 14h.01"></path>
<path d="M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14"></path>"#,
};

pub const WALLET_CARDS: IconType = IconType{ 
 content: r#"
<rect width="18" x="3" rx="2" y="3" height="18"></rect>
<path d="M3 9a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2"></path>
<path d="M3 11h3c.8 0 1.6.3 2.1.9l1.1.9c1.6 1.6 4.1 1.6 5.7 0l1.1-.9c.5-.5 1.3-.9 2.1-.9H21"></path>"#,
};

pub const WALLET: IconType = IconType{ 
 content: r#"
<path d="M21 12V7H5a2 2 0 0 1 0-4h14v4"></path>
<path d="M3 5v14a2 2 0 0 0 2 2h16v-5"></path>
<path d="M18 12a2 2 0 0 0 0 4h4v-4Z"></path>"#,
};

pub const WALLPAPER: IconType = IconType{ 
 content: r#"
<circle cy="9" r="2" cx="8"></circle>
<path d="m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2"></path>
<path d="M8 21h8"></path>
<path d="M12 17v4"></path>"#,
};

pub const WAND_2: IconType = IconType{ 
 content: r#"
<path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72Z"></path>
<path d="m14 7 3 3"></path>
<path d="M5 6v4"></path>
<path d="M19 14v4"></path>
<path d="M10 2v2"></path>
<path d="M7 8H3"></path>
<path d="M21 16h-4"></path>
<path d="M11 3H9"></path>"#,
};

pub const WAND: IconType = IconType{ 
 content: r#"
<path d="M15 4V2"></path>
<path d="M15 16v-2"></path>
<path d="M8 9h2"></path>
<path d="M20 9h2"></path>
<path d="M17.8 11.8 19 13"></path>
<path d="M15 9h0"></path>
<path d="M17.8 6.2 19 5"></path>
<path d="m3 21 9-9"></path>
<path d="M12.2 6.2 11 5"></path>"#,
};

pub const WAREHOUSE: IconType = IconType{ 
 content: r#"
<path d="M22 8.35V20a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8.35A2 2 0 0 1 3.26 6.5l8-3.2a2 2 0 0 1 1.48 0l8 3.2A2 2 0 0 1 22 8.35Z"></path>
<path d="M6 18h12"></path>
<path d="M6 14h12"></path>
<rect width="12" height="12" y="10" x="6"></rect>"#,
};

pub const WATCH: IconType = IconType{ 
 content: r#"
<circle r="6" cy="12" cx="12"></circle>
<polyline points="12 10 12 12 13 13"></polyline>
<path d="m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05"></path>
<path d="m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05"></path>"#,
};

pub const WAVES: IconType = IconType{ 
 content: r#"
<path d="M2 6c.6.5 1.2 1 2.5 1C7 7 7 5 9.5 5c2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M2 12c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>
<path d="M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"></path>"#,
};

pub const WEBCAM: IconType = IconType{ 
 content: r#"
<circle r="8" cy="10" cx="12"></circle>
<circle cx="12" r="3" cy="10"></circle>
<path d="M7 22h10"></path>
<path d="M12 22v-4"></path>"#,
};

pub const WEBHOOK: IconType = IconType{ 
 content: r#"
<path d="M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2"></path>
<path d="m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06"></path>
<path d="m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8"></path>"#,
};

pub const WHEAT_OFF: IconType = IconType{ 
 content: r#"
<path d="m2 22 10-10"></path>
<path d="m16 8-1.17 1.17"></path>
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97"></path>
<path d="M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62"></path>
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"></path>
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98"></path>
<path d="M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28"></path>
<line y1="2" x2="22" y2="22" x1="2"></line>"#,
};

pub const WHEAT: IconType = IconType{ 
 content: r#"
<path d="M2 22 16 8"></path>
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"></path>
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"></path>
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>
<path d="M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"></path>"#,
};

pub const WHOLE_WORD: IconType = IconType{ 
 content: r#"
<circle cx="7" r="3" cy="12"></circle>
<path d="M10 9v6"></path>
<circle cy="12" cx="17" r="3"></circle>
<path d="M14 7v8"></path>
<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"></path>"#,
};

pub const WIFI_OFF: IconType = IconType{ 
 content: r#"
<line y2="22" y1="2" x1="2" x2="22"></line>
<path d="M8.5 16.5a5 5 0 0 1 7 0"></path>
<path d="M2 8.82a15 15 0 0 1 4.17-2.65"></path>
<path d="M10.66 5c4.01-.36 8.14.9 11.34 3.76"></path>
<path d="M16.85 11.25a10 10 0 0 1 2.22 1.68"></path>
<path d="M5 13a10 10 0 0 1 5.24-2.76"></path>
<line x2="12.01" y1="20" y2="20" x1="12"></line>"#,
};

pub const WIFI: IconType = IconType{ 
 content: r#"
<path d="M5 13a10 10 0 0 1 14 0"></path>
<path d="M8.5 16.5a5 5 0 0 1 7 0"></path>
<path d="M2 8.82a15 15 0 0 1 20 0"></path>
<line x2="12.01" y2="20" y1="20" x1="12"></line>"#,
};

pub const WIND: IconType = IconType{ 
 content: r#"
<path d="M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2"></path>
<path d="M9.6 4.6A2 2 0 1 1 11 8H2"></path>
<path d="M12.6 19.4A2 2 0 1 0 14 16H2"></path>"#,
};

pub const WINE_OFF: IconType = IconType{ 
 content: r#"
<path d="M8 22h8"></path>
<path d="M7 10h3m7 0h-1.343"></path>
<path d="M12 15v7"></path>
<path d="M7.307 7.307A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.391 4.391M8.638 2.981C8.75 2.668 8.872 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.809-.145 1.198"></path>
<line x2="22" x1="2" y1="2" y2="22"></line>"#,
};

pub const WINE: IconType = IconType{ 
 content: r#"
<path d="M8 22h8"></path>
<path d="M7 10h10"></path>
<path d="M12 15v7"></path>
<path d="M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z"></path>"#,
};

pub const WORKFLOW: IconType = IconType{ 
 content: r#"
<rect height="8" width="8" y="3" rx="2" x="3"></rect>
<path d="M7 11v4a2 2 0 0 0 2 2h4"></path>
<rect y="13" rx="2" width="8" height="8" x="13"></rect>"#,
};

pub const WRAP_TEXT: IconType = IconType{ 
 content: r#"
<line y1="6" x2="21" y2="6" x1="3"></line>
<path d="M3 12h15a3 3 0 1 1 0 6h-4"></path>
<polyline points="16 16 14 18 16 20"></polyline>
<line x1="3" y1="18" y2="18" x2="10"></line>"#,
};

pub const WRENCH: IconType = IconType{ 
 content: r#"
<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>"#,
};

pub const X_CIRCLE: IconType = IconType{ 
 content: r#"
<circle cx="12" cy="12" r="10"></circle>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#,
};

pub const X_OCTAGON: IconType = IconType{ 
 content: r#"
<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#,
};

pub const X_SQUARE: IconType = IconType{ 
 content: r#"
<rect height="18" width="18" y="3" ry="2" rx="2" x="3"></rect>
<path d="m15 9-6 6"></path>
<path d="m9 9 6 6"></path>"#,
};

pub const X: IconType = IconType{ 
 content: r#"
<path d="M18 6 6 18"></path>
<path d="m6 6 12 12"></path>"#,
};

pub const YOUTUBE: IconType = IconType{ 
 content: r#"
<path d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17"></path>
<path d="m10 15 5-3-5-3z"></path>"#,
};

pub const ZAP_OFF: IconType = IconType{ 
 content: r#"
<polyline points="12.41 6.75 13 2 10.57 4.92"></polyline>
<polyline points="18.57 12.91 21 10 15.66 10"></polyline>
<polyline points="8 8 3 14 12 14 11 22 16 16"></polyline>
<line y2="22" x2="22" x1="2" y1="2"></line>"#,
};

pub const ZAP: IconType = IconType{ 
 content: r#"
<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>"#,
};

pub const ZOOM_IN: IconType = IconType{ 
 content: r#"
<circle cx="11" cy="11" r="8"></circle>
<line x1="21" y1="21" y2="16.65" x2="16.65"></line>
<line y2="14" x1="11" y1="8" x2="11"></line>
<line x2="14" y1="11" y2="11" x1="8"></line>"#,
};

pub const ZOOM_OUT: IconType = IconType{ 
 content: r#"
<circle cy="11" r="8" cx="11"></circle>
<line y1="21" x1="21" x2="16.65" y2="16.65"></line>
<line y1="11" x2="14" x1="8" y2="11"></line>"#,
};
