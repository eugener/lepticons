
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideGlyph {
    #[cfg(feature = "a_arrow_down")]
    #[strum(props(
        svg = "<path d=\"m14 12 4 4 4-4\"></path><path d=\"M18 16V7\"></path><path d=\"m2 16 4.03-9.69a.5.5 0 0 1 .923 0L11 16\"></path><path d=\"M3.30 13h6.39\"></path>",
        categories = "text,design",
        tags = "letter,font size,text,formatting,smaller",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowDown,
    #[cfg(feature = "a_arrow_up")]
    #[strum(props(
        svg = "<path d=\"m14 11 4-4 4 4\"></path><path d=\"M18 16V7\"></path><path d=\"m2 16 4.03-9.69a.5.5 0 0 1 .923 0L11 16\"></path><path d=\"M3.30 13h6.39\"></path>",
        categories = "text,design",
        tags = "letter,font size,text,formatting,larger,bigger",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis"
    ))]
    AArrowUp,
    #[cfg(feature = "a_large_small")]
    #[strum(props(
        svg = "<path d=\"m15 16 2.53-7.32a1.02 1.02 1 0 1 1.92 0L22 16\"></path><path d=\"M15.69 14h5.60\"></path><path d=\"m2 16 4.03-9.69a.5.5 0 0 1 .923 0L11 16\"></path><path d=\"M3.30 13h6.39\"></path>",
        categories = "text,design",
        tags = "letter,font size,text,formatting",
        contributors = "it-is-not,jguddas,danielbayley,ericfennis,vichotech,karsa-mistmere"
    ))]
    ALargeSmall,
    #[cfg(feature = "accessibility")]
    #[strum(props(
        svg = "<circle cx=\"16\" cy=\"4\" r=\"1\"></circle><path d=\"m18 19 1-7-6 1\"></path><path d=\"m5 8 3-3 5.5 3-2.36 3.5\"></path><path d=\"M4.24 14.5a5 5 0 0 0 6.88 6\"></path><path d=\"M13.76 17.5a5 5 0 0 0-6.88-6\"></path>",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[cfg(feature = "activity")]
    #[strum(props(
        svg = "<path d=\"M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48 0L9.24 2.18a.25.25 0 0 0-.48 0l-2.35 8.36A2 2 0 0 1 4.49 12H2\"></path>",
        categories = "medical,account,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "colebemis,jguddas"
    ))]
    Activity,
    #[cfg(feature = "air_vent")]
    #[strum(props(
        svg = "<path d=\"M18 17.5a2.5 2.5 0 1 1-4 2.03V12\"></path><path d=\"M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2\"></path><path d=\"M6 8h12\"></path><path d=\"M6.6 15.57A2 2 0 1 0 10 17v-5\"></path>",
        categories = "home",
        tags = "air conditioner,ac,central air,cooling,climate-control",
        contributors = "karsa-mistmere,jguddas"
    ))]
    AirVent,
    #[cfg(feature = "airplay")]
    #[strum(props(
        svg = "<path d=\"M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1\"></path><path d=\"m12 15 5 6H7Z\"></path>",
        categories = "multimedia,connectivity",
        tags = "stream,cast,mirroring,screen,monitor,macos,osx",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Airplay,
    #[cfg(feature = "alarm_clock_check")]
    #[strum(props(
        svg = "<circle cy=\"13\" r=\"8\" cx=\"12\"></circle><path d=\"M5 3 2 6\"></path><path d=\"m22 6-3-3\"></path><path d=\"M6.38 18.7 4 21\"></path><path d=\"M17.64 18.67 20 21\"></path><path d=\"m9 13 2 2 4-4\"></path>",
        categories = "devices,notifications,time",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockCheck,
    #[cfg(feature = "alarm_clock_minus")]
    #[strum(props(
        svg = "<circle cy=\"13\" r=\"8\" cx=\"12\"></circle><path d=\"M5 3 2 6\"></path><path d=\"m22 6-3-3\"></path><path d=\"M6.38 18.7 4 21\"></path><path d=\"M17.64 18.67 20 21\"></path><path d=\"M9 13h6\"></path>",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockMinus,
    #[cfg(feature = "alarm_clock_off")]
    #[strum(props(
        svg = "<path d=\"M6.87 6.87a8 8 0 1 0 11.26 11.26\"></path><path d=\"M19.9 14.25a8 8 0 0 0-9.15-9.15\"></path><path d=\"m22 6-3-3\"></path><path d=\"M6.26 18.67 4 21\"></path><path d=\"m2 2 20 20\"></path><path d=\"M4 4 2 6\"></path>",
        categories = "devices,notifications,time",
        tags = "morning,turn-off",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    AlarmClockOff,
    #[cfg(feature = "alarm_clock_plus")]
    #[strum(props(
        svg = "<circle r=\"8\" cx=\"12\" cy=\"13\"></circle><path d=\"M5 3 2 6\"></path><path d=\"m22 6-3-3\"></path><path d=\"M6.38 18.7 4 21\"></path><path d=\"M17.64 18.67 20 21\"></path><path d=\"M12 10v6\"></path><path d=\"M9 13h6\"></path>",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClockPlus,
    #[cfg(feature = "alarm_clock")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"8\" cy=\"13\"></circle><path d=\"M12 9v4l2 2\"></path><path d=\"M5 3 2 6\"></path><path d=\"m22 6-3-3\"></path><path d=\"M6.38 18.7 4 21\"></path><path d=\"M17.64 18.67 20 21\"></path>",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_smoke")]
    #[strum(props(
        svg = "<path d=\"M11 21c0-2.5 2-2.5 2-5\"></path><path d=\"M16 21c0-2.5 2-2.5 2-5\"></path><path d=\"m19 8-.8 3a1.25 1.25 0 0 1-1.2 1H7a1.25 1.25 0 0 1-1.2-1L5 8\"></path><path d=\"M21 3a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a1 1 0 0 1 1-1z\"></path><path d=\"M6 21c0-2.5 2-2.5 2-5\"></path>",
        categories = "home,devices,travel",
        tags = "fire,alert,warning,detector,carbon monoxide,safety,equipment,amenities",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    AlarmSmoke,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" x=\"3\" y=\"3\" width=\"18\" ry=\"2\"></rect><polyline points=\"11 3 11 11 14 8 17 11 17 3\"></polyline>",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "align_center_horizontal")]
    #[strum(props(
        svg = "<path d=\"M2 12h20\"></path><path d=\"M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4\"></path><path d=\"M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4\"></path><path d=\"M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1\"></path><path d=\"M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1\"></path>",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterHorizontal,
    #[cfg(feature = "align_center_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 2v20\"></path><path d=\"M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4\"></path><path d=\"M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4\"></path><path d=\"M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1\"></path><path d=\"M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1\"></path>",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterVertical,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "<rect height=\"16\" y=\"2\" x=\"4\" rx=\"2\" width=\"6\"></rect><rect y=\"9\" height=\"9\" rx=\"2\" width=\"6\" x=\"14\"></rect><path d=\"M22 22H2\"></path>",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "<rect rx=\"2\" y=\"4\" height=\"6\" width=\"16\" x=\"2\"></rect><rect rx=\"2\" y=\"14\" width=\"9\" x=\"9\" height=\"6\"></rect><path d=\"M22 22V2\"></path>",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "<rect y=\"5\" x=\"4\" width=\"6\" rx=\"2\" height=\"14\"></rect><rect height=\"10\" width=\"6\" x=\"14\" y=\"7\" rx=\"2\"></rect><path d=\"M17 22v-5\"></path><path d=\"M17 7V2\"></path><path d=\"M7 22v-3\"></path><path d=\"M7 5V2\"></path>",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "<rect rx=\"2\" x=\"4\" y=\"5\" width=\"6\" height=\"14\"></rect><rect x=\"14\" height=\"10\" rx=\"2\" width=\"6\" y=\"7\"></rect><path d=\"M10 2v20\"></path><path d=\"M20 2v20\"></path>",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "<rect height=\"14\" x=\"4\" rx=\"2\" y=\"5\" width=\"6\"></rect><rect x=\"14\" width=\"6\" height=\"10\" y=\"7\" rx=\"2\"></rect><path d=\"M4 2v20\"></path><path d=\"M14 2v20\"></path>",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "<rect y=\"5\" rx=\"2\" x=\"2\" width=\"6\" height=\"14\"></rect><rect rx=\"2\" width=\"6\" y=\"7\" height=\"10\" x=\"16\"></rect><path d=\"M12 2v20\"></path>",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"14\" y=\"5\" x=\"2\" width=\"6\"></rect><rect x=\"12\" rx=\"2\" y=\"7\" height=\"10\" width=\"6\"></rect><path d=\"M22 2v20\"></path>",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "<rect width=\"6\" height=\"14\" x=\"6\" y=\"5\" rx=\"2\"></rect><rect x=\"16\" y=\"7\" rx=\"2\" width=\"6\" height=\"10\"></rect><path d=\"M2 2v20\"></path>",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "<rect width=\"6\" x=\"9\" y=\"7\" rx=\"2\" height=\"10\"></rect><path d=\"M4 22V2\"></path><path d=\"M20 22V2\"></path>",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"5\" rx=\"2\" width=\"6\" height=\"14\"></rect><rect height=\"10\" x=\"15\" width=\"6\" rx=\"2\" y=\"7\"></rect><path d=\"M3 2v20\"></path><path d=\"M21 2v20\"></path>",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"16\" x=\"4\" y=\"6\" width=\"6\"></rect><rect y=\"6\" width=\"6\" rx=\"2\" height=\"9\" x=\"14\"></rect><path d=\"M22 2H2\"></path>",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "<rect y=\"14\" width=\"9\" x=\"6\" rx=\"2\" height=\"6\"></rect><rect width=\"16\" x=\"6\" y=\"4\" rx=\"2\" height=\"6\"></rect><path d=\"M2 2v20\"></path>",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "<path d=\"M22 17h-3\"></path><path d=\"M22 7h-5\"></path><path d=\"M5 17H2\"></path><path d=\"M7 7H2\"></path><rect x=\"5\" rx=\"2\" width=\"14\" y=\"14\" height=\"6\"></rect><rect y=\"4\" height=\"6\" rx=\"2\" x=\"7\" width=\"10\"></rect>",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis,jguddas"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "<rect y=\"14\" rx=\"2\" height=\"6\" x=\"5\" width=\"14\"></rect><rect width=\"10\" y=\"4\" x=\"7\" rx=\"2\" height=\"6\"></rect><path d=\"M2 20h20\"></path><path d=\"M2 10h20\"></path>",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "<rect width=\"14\" y=\"14\" rx=\"2\" height=\"6\" x=\"5\"></rect><rect y=\"4\" height=\"6\" rx=\"2\" width=\"10\" x=\"7\"></rect><path d=\"M2 14h20\"></path><path d=\"M2 4h20\"></path>",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "<rect height=\"6\" x=\"5\" y=\"16\" width=\"14\" rx=\"2\"></rect><rect y=\"2\" width=\"10\" height=\"6\" x=\"7\" rx=\"2\"></rect><path d=\"M2 12h20\"></path>",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "<rect x=\"5\" height=\"6\" y=\"12\" rx=\"2\" width=\"14\"></rect><rect width=\"10\" y=\"2\" x=\"7\" rx=\"2\" height=\"6\"></rect><path d=\"M2 22h20\"></path>",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "<rect width=\"14\" y=\"16\" height=\"6\" rx=\"2\" x=\"5\"></rect><rect x=\"7\" y=\"6\" rx=\"2\" height=\"6\" width=\"10\"></rect><path d=\"M2 2h20\"></path>",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "<rect width=\"10\" rx=\"2\" x=\"7\" height=\"6\" y=\"9\"></rect><path d=\"M22 20H2\"></path><path d=\"M22 4H2\"></path>",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "<rect width=\"14\" y=\"15\" x=\"5\" rx=\"2\" height=\"6\"></rect><rect width=\"10\" height=\"6\" x=\"7\" y=\"3\" rx=\"2\"></rect><path d=\"M2 21h20\"></path><path d=\"M2 3h20\"></path>",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceBetween,
    #[cfg(feature = "ambulance")]
    #[strum(props(
        svg = "<path d=\"M10 10H6\"></path><path d=\"M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2\"></path><path d=\"M19 18h2a1 1 0 0 0 1-1v-3.28a1 1 0 0 0-.684-.948l-1.92-.641a1 1 0 0 1-.578-.502l-1.53-3.07A1 1 0 0 0 16.38 8H14\"></path><path d=\"M8 8v4\"></path><path d=\"M9 18h6\"></path><circle r=\"2\" cx=\"17\" cy=\"18\"></circle><circle r=\"2\" cx=\"7\" cy=\"18\"></circle>",
        categories = "medical,transportation",
        tags = "ambulance,emergency,medical,vehicle,siren,healthcare,transportation,rescue,urgent,first aid",
        contributors = "jordan808,jguddas,colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley"
    ))]
    Ambulance,
    #[cfg(feature = "ampersand")]
    #[strum(props(
        svg = "<path d=\"M16 12h3\"></path><path d=\"M17.5 12a8 8 0 0 1-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13\"></path>",
        categories = "text,development",
        tags = "and,typography,operator,join,concatenate,code,&",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Ampersand,
    #[cfg(feature = "ampersands")]
    #[strum(props(
        svg = "<path d=\"M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5\"></path><path d=\"M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5\"></path>",
        categories = "text,development",
        tags = "and,operator,then,code,&&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersands,
    #[cfg(feature = "amphora")]
    #[strum(props(
        svg = "<path d=\"M10 2v5.63c0 .424-.272.79-.653.98A6 6 0 0 0 6 14c.006 4 3 7 5 8\"></path><path d=\"M10 5H8a2 2 0 0 0 0 4h.68\"></path><path d=\"M14 2v5.63c0 .424.27.795.65.982A6 6 0 0 1 18 14c0 4-3 7-5 8\"></path><path d=\"M14 5h2a2 2 0 0 1 0 4h-.68\"></path><path d=\"M18 22H6\"></path><path d=\"M9 2h6\"></path>",
        categories = "food-beverage,gaming",
        tags = "pottery,artifact,artefact,vase,ceramics,clay,archaeology,museum,wine,oil",
        contributors = "karsa-mistmere"
    ))]
    Amphora,
    #[cfg(feature = "anchor")]
    #[strum(props(
        svg = "<path d=\"M12 6v16\"></path><path d=\"m19 13 2-1a9 9 0 0 1-18 0l2 1\"></path><path d=\"M9 11h6\"></path><circle cy=\"4\" cx=\"12\" r=\"2\"></circle>",
        categories = "transportation,text",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Anchor,
    #[cfg(feature = "angry")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M16 16s-1.5-2-4-2-4 2-4 2\"></path><path d=\"M7.5 8 10 9\"></path><path d=\"m14 9 2.5-1\"></path><path d=\"M9 10h.01\"></path><path d=\"M15 10h.01\"></path>",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[cfg(feature = "annoyed")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M8 15h8\"></path><path d=\"M8 9h2\"></path><path d=\"M14 9h2\"></path>",
        categories = "emoji",
        tags = "emoji,nuisance,face,emotion",
        contributors = "karsa-mistmere"
    ))]
    Annoyed,
    #[cfg(feature = "antenna")]
    #[strum(props(
        svg = "<path d=\"M2 12 7 2\"></path><path d=\"m7 12 5-10\"></path><path d=\"m12 12 5-10\"></path><path d=\"m17 12 5-10\"></path><path d=\"M4.5 7h15\"></path><path d=\"M12 16v6\"></path>",
        categories = "devices,multimedia,communication",
        tags = "signal,connection,connectivity,tv,television,broadcast,live,frequency,tune,scan,channels,aerial,receiver,transmission,transducer,terrestrial,satellite,cable",
        contributors = "danielbayley"
    ))]
    Antenna,
    #[cfg(feature = "anvil")]
    #[strum(props(
        svg = "<path d=\"M7 10H6a4 4 0 0 1-4-4 1 1 0 0 1 1-1h4\"></path><path d=\"M7 5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1 7 7 0 0 1-7 7H8a1 1 0 0 1-1-1z\"></path><path d=\"M9 12v5\"></path><path d=\"M15 12v5\"></path><path d=\"M5 20a3 3 0 0 1 3-3h8a3 3 0 0 1 3 3 1 1 0 0 1-1 1H6a1 1 0 0 1-1-1\"></path>",
        categories = "buildings,tools,gaming",
        tags = "metal,iron,alloy,materials,heavy,weight,blacksmith,forge,acme",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Anvil,
    #[cfg(feature = "aperture")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"m14.31 8 5.74 9.94\"></path><path d=\"M9.69 8h11.48\"></path><path d=\"m7.38 12 5.74-9.94\"></path><path d=\"M9.69 16 3.95 6.06\"></path><path d=\"M14.31 16H2.83\"></path><path d=\"m16.62 12-5.74 9.94\"></path>",
        categories = "photography",
        tags = "camera,photo,pictures,shutter,exposure",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window_mac")]
    #[strum(props(
        svg = "<rect height=\"16\" width=\"20\" x=\"2\" rx=\"2\" y=\"4\"></rect><path d=\"M6 8h.01\"></path><path d=\"M10 8h.01\"></path><path d=\"M14 8h.01\"></path>",
        categories = "layout,design,development,files",
        tags = "application,menu bar,pane,preferences,macos,osx,executable",
        contributors = "danielbayley"
    ))]
    AppWindowMac,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "<rect y=\"4\" x=\"2\" height=\"16\" rx=\"2\" width=\"20\"></rect><path d=\"M10 4v4\"></path><path d=\"M2 8h20\"></path><path d=\"M6 4v4\"></path>",
        categories = "layout,design,development,files",
        tags = "application,menu bar,pane,executable",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    AppWindow,
    #[cfg(feature = "apple")]
    #[strum(props(
        svg = "<path d=\"M12 6.52V3a1 1 0 0 1 1-1h0\"></path><path d=\"M18.23 21A15 15 0 0 0 22 11a6 6 0 0 0-10-4.47A6 6 0 0 0 2 11a15.1 15.1 0 0 0 3.76 10 3 3 0 0 0 3.64.648 5.5 5.5 0 0 1 5.17 0A3 3 0 0 0 18.23 21\"></path>",
        categories = "food-beverage",
        tags = "fruit,food,healthy,snack,nutrition,fresh,produce,grocery,organic,harvest,vitamin,red,green,juicy,sweet,tart,bite,orchard,plant,core,raw,diet",
        contributors = "karsa-mistmere"
    ))]
    Apple,
    #[cfg(feature = "archive_restore")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"20\" rx=\"1\" x=\"2\" height=\"5\"></rect><path d=\"M4 8v11a2 2 0 0 0 2 2h2\"></path><path d=\"M20 8v11a2 2 0 0 1-2 2h-2\"></path><path d=\"m9 15 3-3 3 3\"></path><path d=\"M12 12v9\"></path>",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "<rect height=\"5\" rx=\"1\" width=\"20\" x=\"2\" y=\"3\"></rect><path d=\"M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8\"></path><path d=\"m9.5 17 5-5\"></path><path d=\"m9.5 12 5 5\"></path>",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "<rect rx=\"1\" y=\"3\" width=\"20\" height=\"5\" x=\"2\"></rect><path d=\"M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8\"></path><path d=\"M10 12h4\"></path>",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Archive,
    #[cfg(feature = "armchair")]
    #[strum(props(
        svg = "<path d=\"M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3\"></path><path d=\"M3 16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v1.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V11a2 2 0 0 0-4 0z\"></path><path d=\"M5 18v2\"></path><path d=\"M19 18v2\"></path>",
        categories = "home",
        tags = "sofa,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Armchair,
    #[cfg(feature = "arrow_big_down_dash")]
    #[strum(props(
        svg = "<path d=\"M14 8a1 1 0 0 1 1 1v2a1 1 0 0 0 1 1h3.29a.707.70 0 0 1 .5 1.20l-6.93 6.93a1.20 1.20 0 0 1-1.70 0l-6.94-6.94a.707.70 0 0 1 .5-1.20H8a1 1 0 0 0 1-1V9a1 1 0 0 1 1-1z\"></path><path d=\"M9 4h6\"></path>",
        categories = "arrows,gaming,files",
        tags = "backwards,reverse,slow,direction,south,download",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigDownDash,
    #[cfg(feature = "arrow_big_down")]
    #[strum(props(
        svg = "<path d=\"M9 5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v6a1 1 0 0 0 1 1h3.29a.707.70 0 0 1 .5 1.20l-7.08 7.08a1 1 0 0 1-1.41 0l-7.08-7.08a.707.70 0 0 1 .5-1.20H8a1 1 0 0 0 1-1z\"></path>",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south",
        contributors = "Andreto,mittalyashu,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigDown,
    #[cfg(feature = "arrow_big_left_dash")]
    #[strum(props(
        svg = "<path d=\"M13 9a1 1 0 0 1-1-1V4.70a.707.70 0 0 0-1.20-.5l-6.94 6.94a1.20 1.20 0 0 0 0 1.70l6.94 6.94a.707.70 0 0 0 1.20-.5V16a1 1 0 0 1 1-1h2a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1z\"></path><path d=\"M20 9v6\"></path>",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,turn,corner",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigLeftDash,
    #[cfg(feature = "arrow_big_left")]
    #[strum(props(
        svg = "<path d=\"M10.79 19.79a.707.70 0 0 0 1.20-.5V16a1 1 0 0 1 1-1h6a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1h-6a1 1 0 0 1-1-1V4.70a.707.70 0 0 0-1.20-.5l-6.94 6.94a1.20 1.20 0 0 0 0 1.70z\"></path>",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigLeft,
    #[cfg(feature = "arrow_big_right_dash")]
    #[strum(props(
        svg = "<path d=\"M11 9a1 1 0 0 0 1-1V4.70a.707.70 0 0 1 1.20-.5l6.94 6.94a1.20 1.20 0 0 1 0 1.70l-6.94 6.94a.707.70 0 0 1-1.20-.5V16a1 1 0 0 0-1-1H9a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1z\"></path><path d=\"M4 9v6\"></path>",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,turn,corner",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigRightDash,
    #[cfg(feature = "arrow_big_right")]
    #[strum(props(
        svg = "<path d=\"M13.20 19.79a.707.70 0 0 1-1.20-.5V16a1 1 0 0 0-1-1H5a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1h6a1 1 0 0 0 1-1V4.70a.707.70 0 0 1 1.20-.5l6.94 6.94a1.20 1.20 0 0 1 0 1.70z\"></path>",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigRight,
    #[cfg(feature = "arrow_big_up_dash")]
    #[strum(props(
        svg = "<path d=\"M14 16a1 1 0 0 0 1-1v-2a1 1 0 0 1 1-1h3.29a.707.70 0 0 0 .5-1.20l-6.93-6.93a1.20 1.20 0 0 0-1.70 0l-6.94 6.94a.707.70 0 0 0 .5 1.20H8a1 1 0 0 1 1 1v2a1 1 0 0 0 1 1z\"></path><path d=\"M9 20h6\"></path>",
        categories = "arrows,text,development,gaming",
        tags = "caps lock,capitals,keyboard,button,mac,forward,direction,north,faster,speed,boost",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigUpDash,
    #[cfg(feature = "arrow_big_up")]
    #[strum(props(
        svg = "<path d=\"M9 19a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-6a1 1 0 0 1 1-1h3.29a.707.70 0 0 0 .5-1.20l-7.08-7.08a1 1 0 0 0-1.41 0l-7.08 7.08a.707.70 0 0 0 .5 1.20H8a1 1 0 0 1 1 1z\"></path>",
        categories = "arrows,text,development,gaming",
        tags = "shift,keyboard,button,mac,capitalize,capitalise,forward,direction,north",
        contributors = "Andreto,mittalyashu,danielbayley,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    ArrowBigUp,
    #[cfg(feature = "arrow_down_01")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><rect y=\"4\" height=\"6\" width=\"4\" ry=\"2\" x=\"15\"></rect><path d=\"M17 20v-6h-2\"></path><path d=\"M15 20h4\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><path d=\"M17 10V4h-2\"></path><path d=\"M15 10h4\"></path><rect y=\"14\" height=\"6\" ry=\"2\" x=\"15\" width=\"4\"></rect>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDown10,
    #[cfg(feature = "arrow_down_az")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><path d=\"M20 8h-5\"></path><path d=\"M15 10V6.5a2.5 2.5 0 0 1 5 0V10\"></path><path d=\"M15 14h5l-5 6h5\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDownAZ,
    #[cfg(feature = "arrow_down_from_line")]
    #[strum(props(
        svg = "<path d=\"M19 3H5\"></path><path d=\"M12 21V7\"></path><path d=\"m6 15 6 6 6-6\"></path>",
        categories = "arrows,files",
        tags = "backwards,reverse,direction,south,download,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownFromLine,
    #[cfg(feature = "arrow_down_left")]
    #[strum(props(
        svg = "<path d=\"M17 7 7 17\"></path><path d=\"M17 17H7V7\"></path>",
        categories = "arrows",
        tags = "direction,south-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownLeft,
    #[cfg(feature = "arrow_down_narrow_wide")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><path d=\"M11 4h4\"></path><path d=\"M11 8h7\"></path><path d=\"M11 12h10\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownNarrowWide,
    #[cfg(feature = "arrow_down_right")]
    #[strum(props(
        svg = "<path d=\"m7 7 10 10\"></path><path d=\"M17 7v10H7\"></path>",
        categories = "arrows",
        tags = "direction,south-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownRight,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "<path d=\"M12 2v14\"></path><path d=\"m19 9-7 7-7-7\"></path><circle cx=\"12\" r=\"1\" cy=\"21\"></circle>",
        categories = "arrows",
        tags = "direction,south,waypoint,location,step,into",
        contributors = "danielbayley"
    ))]
    ArrowDownToDot,
    #[cfg(feature = "arrow_down_to_line")]
    #[strum(props(
        svg = "<path d=\"M12 17V3\"></path><path d=\"m6 11 6 6 6-6\"></path><path d=\"M19 21H5\"></path>",
        categories = "arrows,files,development",
        tags = "behind,direction,south,download,save,git,version control,pull,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownToLine,
    #[cfg(feature = "arrow_down_up")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><path d=\"m21 8-4-4-4 4\"></path><path d=\"M17 4v16\"></path>",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,network,traffic,flow,mobile data,internet,sort,reorder,move",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownUp,
    #[cfg(feature = "arrow_down_wide_narrow")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 20V4\"></path><path d=\"M11 4h10\"></path><path d=\"M11 8h7\"></path><path d=\"M11 12h4\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowDownWideNarrow,
    #[cfg(feature = "arrow_down_za")]
    #[strum(props(
        svg = "<path d=\"m3 16 4 4 4-4\"></path><path d=\"M7 4v16\"></path><path d=\"M15 4h5l-5 6h5\"></path><path d=\"M15 20v-3.5a2.5 2.5 0 0 1 5 0V20\"></path><path d=\"M20 18h-5\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical,reverse",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowDownZA,
    #[cfg(feature = "arrow_down")]
    #[strum(props(
        svg = "<path d=\"M12 5v14\"></path><path d=\"m19 12-7 7-7-7\"></path>",
        categories = "arrows",
        tags = "backwards,reverse,direction,south",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDown,
    #[cfg(feature = "arrow_left_from_line")]
    #[strum(props(
        svg = "<path d=\"m9 6-6 6 6 6\"></path><path d=\"M3 12h14\"></path><path d=\"M21 19V5\"></path>",
        categories = "arrows",
        tags = "previous,back,direction,west,expand,fold,horizontal,<-|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftFromLine,
    #[cfg(feature = "arrow_left_right")]
    #[strum(props(
        svg = "<path d=\"M8 3 4 7l4 4\"></path><path d=\"M4 7h16\"></path><path d=\"m16 21 4-4-4-4\"></path><path d=\"M20 17H4\"></path>",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ArrowLeftRight,
    #[cfg(feature = "arrow_left_to_line")]
    #[strum(props(
        svg = "<path d=\"M3 19V5\"></path><path d=\"m13 6-6 6 6 6\"></path><path d=\"M7 12h14\"></path>",
        categories = "arrows",
        tags = "previous,back,direction,west,collapse,fold,horizontal,|<-",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftToLine,
    #[cfg(feature = "arrow_left")]
    #[strum(props(
        svg = "<path d=\"m12 19-7-7 7-7\"></path><path d=\"M19 12H5\"></path>",
        categories = "arrows",
        tags = "previous,back,direction,west,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeft,
    #[cfg(feature = "arrow_right_from_line")]
    #[strum(props(
        svg = "<path d=\"M3 5v14\"></path><path d=\"M21 12H7\"></path><path d=\"m15 18 6-6-6-6\"></path>",
        categories = "arrows",
        tags = "next,forward,direction,east,export,expand,fold,horizontal,|->",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightFromLine,
    #[cfg(feature = "arrow_right_left")]
    #[strum(props(
        svg = "<path d=\"m16 3 4 4-4 4\"></path><path d=\"M20 7H4\"></path><path d=\"m8 21-4-4 4-4\"></path><path d=\"M4 17h16\"></path>",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "karsa-mistmere"
    ))]
    ArrowRightLeft,
    #[cfg(feature = "arrow_right_to_line")]
    #[strum(props(
        svg = "<path d=\"M17 12H3\"></path><path d=\"m11 18 6-6-6-6\"></path><path d=\"M21 5v14\"></path>",
        categories = "arrows,development",
        tags = "next,forward,direction,east,tab,keyboard,mac,indent,collapse,fold,horizontal,->|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightToLine,
    #[cfg(feature = "arrow_right")]
    #[strum(props(
        svg = "<path d=\"M5 12h14\"></path><path d=\"m12 5 7 7-7 7\"></path>",
        categories = "arrows",
        tags = "forward,next,direction,east,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRight,
    #[cfg(feature = "arrow_up_01")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><rect y=\"4\" width=\"4\" ry=\"2\" height=\"6\" x=\"15\"></rect><path d=\"M17 20v-6h-2\"></path><path d=\"M15 20h4\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUp01,
    #[cfg(feature = "arrow_up_10")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><path d=\"M17 10V4h-2\"></path><path d=\"M15 10h4\"></path><rect width=\"4\" height=\"6\" ry=\"2\" y=\"14\" x=\"15\"></rect>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,numerical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUp10,
    #[cfg(feature = "arrow_up_az")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><path d=\"M20 8h-5\"></path><path d=\"M15 10V6.5a2.5 2.5 0 0 1 5 0V10\"></path><path d=\"M15 14h5l-5 6h5\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUpAZ,
    #[cfg(feature = "arrow_up_down")]
    #[strum(props(
        svg = "<path d=\"m21 16-4 4-4-4\"></path><path d=\"M17 20V4\"></path><path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path>",
        categories = "arrows",
        tags = "bidirectional,two-way,2-way,swap,switch,network,mobile data,internet,sort,reorder,move",
        contributors = "it-is-not,karsa-mistmere,ericfennis"
    ))]
    ArrowUpDown,
    #[cfg(feature = "arrow_up_from_dot")]
    #[strum(props(
        svg = "<path d=\"m5 9 7-7 7 7\"></path><path d=\"M12 16V2\"></path><circle cx=\"12\" cy=\"21\" r=\"1\"></circle>",
        categories = "arrows",
        tags = "direction,north,step,out",
        contributors = "danielbayley"
    ))]
    ArrowUpFromDot,
    #[cfg(feature = "arrow_up_from_line")]
    #[strum(props(
        svg = "<path d=\"m18 9-6-6-6 6\"></path><path d=\"M12 3v14\"></path><path d=\"M5 21h14\"></path>",
        categories = "arrows,files,development",
        tags = "forward,direction,north,upload,git,version control,push,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpFromLine,
    #[cfg(feature = "arrow_up_left")]
    #[strum(props(
        svg = "<path d=\"M7 17V7h10\"></path><path d=\"M17 17 7 7\"></path>",
        categories = "arrows",
        tags = "direction,north-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpLeft,
    #[cfg(feature = "arrow_up_narrow_wide")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><path d=\"M11 12h4\"></path><path d=\"M11 16h7\"></path><path d=\"M11 20h10\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "lukesmurray,ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowUpNarrowWide,
    #[cfg(feature = "arrow_up_right")]
    #[strum(props(
        svg = "<path d=\"M7 7h10v10\"></path><path d=\"M7 17 17 7\"></path>",
        categories = "arrows",
        tags = "direction,north-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpRight,
    #[cfg(feature = "arrow_up_to_line")]
    #[strum(props(
        svg = "<path d=\"M5 3h14\"></path><path d=\"m18 13-6-6-6 6\"></path><path d=\"M12 7v14\"></path>",
        categories = "arrows,files",
        tags = "forward,direction,north,upload,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpToLine,
    #[cfg(feature = "arrow_up_wide_narrow")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><path d=\"M11 12h10\"></path><path d=\"M11 16h7\"></path><path d=\"M11 20h4\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpWideNarrow,
    #[cfg(feature = "arrow_up_za")]
    #[strum(props(
        svg = "<path d=\"m3 8 4-4 4 4\"></path><path d=\"M7 4v16\"></path><path d=\"M15 4h5l-5 6h5\"></path><path d=\"M15 20v-3.5a2.5 2.5 0 0 1 5 0V20\"></path><path d=\"M20 18h-5\"></path>",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,descending,increasing,decreasing,rising,falling,alphabetical,reverse",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ArrowUpZA,
    #[cfg(feature = "arrow_up")]
    #[strum(props(
        svg = "<path d=\"m5 12 7-7 7 7\"></path><path d=\"M12 19V5\"></path>",
        categories = "arrows",
        tags = "forward,direction,north",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUp,
    #[cfg(feature = "arrows_up_from_line")]
    #[strum(props(
        svg = "<path d=\"m4 6 3-3 3 3\"></path><path d=\"M7 17V3\"></path><path d=\"m14 6 3-3 3 3\"></path><path d=\"M17 17V3\"></path><path d=\"M4 21h16\"></path>",
        categories = "arrows,transportation,mail",
        tags = "direction,orientation,this way up,vertical,package,box,fragile,postage,shipping",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowsUpFromLine,
    #[cfg(feature = "asterisk")]
    #[strum(props(
        svg = "<path d=\"M12 6v12\"></path><path d=\"M17.19 9 6.80 15\"></path><path d=\"m6.80 9 10.39 6\"></path>",
        categories = "text,math,development",
        tags = "reference,times,multiply,multiplication,operator,code,glob pattern,wildcard,*",
        contributors = "mittalyashu,ericfennis"
    ))]
    Asterisk,
    #[cfg(feature = "at_sign")]
    #[strum(props(
        svg = "<circle r=\"4\" cx=\"12\" cy=\"12\"></circle><path d=\"M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8\"></path>",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "<circle r=\"1\" cx=\"12\" cy=\"12\"></circle><path d=\"M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z\"></path><path d=\"M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z\"></path>",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule,electricity,energy,chemistry",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[cfg(feature = "audio_lines")]
    #[strum(props(
        svg = "<path d=\"M2 10v3\"></path><path d=\"M6 6v11\"></path><path d=\"M10 3v18\"></path><path d=\"M14 8v7\"></path><path d=\"M18 5v13\"></path><path d=\"M22 10v3\"></path>",
        categories = "multimedia,communication",
        tags = "graphic equaliser,sound,noise,listen,hearing,hertz,frequency,wavelength,vibrate,sine,synthesizer,synthesiser,levels,track,music,playback,radio,broadcast,airwaves,voice,vocals,singer,song",
        contributors = "danielbayley"
    ))]
    AudioLines,
    #[cfg(feature = "audio_waveform")]
    #[strum(props(
        svg = "<path d=\"M2 13a2 2 0 0 0 2-2V7a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0V4a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0v-4a2 2 0 0 1 2-2\"></path>",
        categories = "multimedia,communication",
        tags = "sound,noise,listen,hearing,hertz,frequency,wavelength,vibrate,sine,synthesizer,synthesiser,levels,track,music,playback,radio,broadcast,airwaves,voice,vocals,singer,song",
        contributors = "danielbayley"
    ))]
    AudioWaveform,
    #[cfg(feature = "award")]
    #[strum(props(
        svg = "<path d=\"m15.47 12.89 1.51 8.52a.5.5 0 0 1-.81.47l-3.58-2.68a1 1 0 0 0-1.19 0l-3.58 2.68a.5.5 0 0 1-.81-.469l1.51-8.52\"></path><circle cy=\"8\" r=\"6\" cx=\"12\"></circle>",
        categories = "account,sports,gaming",
        tags = "achievement,badge,rosette,prize,winner",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Award,
    #[cfg(feature = "axe")]
    #[strum(props(
        svg = "<path d=\"m14 12-8.38 8.38a1 1 0 0 1-3.00-3L11 9\"></path><path d=\"M15 15.5a.5.5 0 0 0 .5.5A6.5 6.5 0 0 0 22 9.5a.5.5 0 0 0-.5-.5h-1.67a2 2 0 0 1-1.41-.586l-5.06-5.06a1.20 1.20 0 0 0-1.70 0L9.35 5.64a1.20 1.20 0 0 0 0 1.70l5.06 5.06A2 2 0 0 1 15 13.82z\"></path>",
        categories = "tools,gaming",
        tags = "hatchet,weapon,chop,sharp,equipment,fireman,firefighter,brigade,lumberjack,woodcutter,logger,forestry",
        contributors = "Andreto,ericfennis,csandman,jguddas,karsa-mistmere"
    ))]
    Axe,
    #[cfg(feature = "axis_3_d")]
    #[strum(props(
        svg = "<path d=\"M13.5 10.5 15 9\"></path><path d=\"M4 4v15a1 1 0 0 0 1 1h15\"></path><path d=\"M4.29 19.70 6 18\"></path><path d=\"m9 15 1.5-1.5\"></path>",
        categories = "design",
        tags = "gizmo,coordinates",
        contributors = "lscheibel,jguddas"
    ))]
    Axis3D,
    #[cfg(feature = "baby")]
    #[strum(props(
        svg = "<path d=\"M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5\"></path><path d=\"M15 12h.01\"></path><path d=\"M19.38 6.81A9 9 0 0 1 20.8 10.2a2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1\"></path><path d=\"M9 12h.01\"></path>",
        categories = "accessibility,people",
        tags = "child,childproof,children",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Baby,
    #[cfg(feature = "backpack")]
    #[strum(props(
        svg = "<path d=\"M4 10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z\"></path><path d=\"M8 10h8\"></path><path d=\"M8 18h8\"></path><path d=\"M8 22v-6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v6\"></path><path d=\"M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2\"></path>",
        categories = "gaming,photography,travel",
        tags = "bag,hiking,travel,camping,school,childhood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Backpack,
    #[cfg(feature = "badge_alert")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><line x1=\"12\" y2=\"12\" y1=\"8\" x2=\"12\"></line><line x2=\"12.01\" y2=\"16\" y1=\"16\" x1=\"12\"></line>",
        categories = "account,social",
        tags = "check,verified,unverified,security,safety,issue",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeAlert,
    #[cfg(feature = "badge_cent")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M12 7v10\"></path><path d=\"M15.4 10a4 4 0 1 0 0 4\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    BadgeCent,
    #[cfg(feature = "badge_check")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "social",
        tags = "verified,check",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeCheck,
    #[cfg(feature = "badge_dollar_sign")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8\"></path><path d=\"M12 18V6\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,usd,$",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeDollarSign,
    #[cfg(feature = "badge_euro")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M7 12h5\"></path><path d=\"M15 9.4a4 4 0 1 0 0 5.2\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,€",
        contributors = "danielbayley"
    ))]
    BadgeEuro,
    #[cfg(feature = "badge_indian_rupee")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M8 8h8\"></path><path d=\"M8 12h8\"></path><path d=\"m13 17-5-1h1a4 4 0 0 0 0-8\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,inr,₹",
        contributors = "danielbayley"
    ))]
    BadgeIndianRupee,
    #[cfg(feature = "badge_info")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><line x2=\"12\" x1=\"12\" y1=\"16\" y2=\"12\"></line><line y1=\"8\" x1=\"12\" x2=\"12.01\" y2=\"8\"></line>",
        categories = "account,accessibility,social",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeInfo,
    #[cfg(feature = "badge_japanese_yen")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"m9 8 3 3v7\"></path><path d=\"m12 11 3-3\"></path><path d=\"M9 12h6\"></path><path d=\"M9 16h6\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,jpy,¥",
        contributors = "danielbayley"
    ))]
    BadgeJapaneseYen,
    #[cfg(feature = "badge_minus")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><line x1=\"8\" x2=\"16\" y1=\"12\" y2=\"12\"></line>",
        categories = "social",
        tags = "verified,unverified,delete,remove,erase",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeMinus,
    #[cfg(feature = "badge_percent")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"m15 9-6 6\"></path><path d=\"M9 9h.01\"></path><path d=\"M15 15h.01\"></path>",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePercent,
    #[cfg(feature = "badge_plus")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><line x1=\"12\" x2=\"12\" y1=\"8\" y2=\"16\"></line><line y1=\"12\" y2=\"12\" x2=\"16\" x1=\"8\"></line>",
        categories = "social",
        tags = "verified,unverified,add,create,new",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePlus,
    #[cfg(feature = "badge_pound_sterling")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M8 12h4\"></path><path d=\"M10 16V9.5a2.5 2.5 0 0 1 5 0\"></path><path d=\"M8 16h7\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,british,gbp,£",
        contributors = "danielbayley"
    ))]
    BadgePoundSterling,
    #[cfg(feature = "badge_question_mark")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\"></path><line y1=\"17\" x2=\"12.01\" x1=\"12\" y2=\"17\"></line>",
        categories = "accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeQuestionMark,
    #[cfg(feature = "badge_russian_ruble")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M9 16h5\"></path><path d=\"M9 12h5a2 2 0 1 0 0-4h-3v9\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,rub,₽",
        contributors = "danielbayley"
    ))]
    BadgeRussianRuble,
    #[cfg(feature = "badge_swiss_franc")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><path d=\"M11 17V8h4\"></path><path d=\"M11 12h3\"></path><path d=\"M9 16h4\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,chf,₣",
        contributors = "danielbayley"
    ))]
    BadgeSwissFranc,
    #[cfg(feature = "badge_turkish_lira")]
    #[strum(props(
        svg = "<path d=\"M11 7v10a5 5 0 0 0 5-5\"></path><path d=\"m15 8-6 3\"></path><path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76\"></path>",
        categories = "shopping,finance",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,try,₺",
        contributors = "danielbayley,jamiemlaw"
    ))]
    BadgeTurkishLira,
    #[cfg(feature = "badge_x")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path><line x2=\"9\" y1=\"9\" y2=\"15\" x1=\"15\"></line><line x2=\"15\" y1=\"9\" x1=\"9\" y2=\"15\"></line>",
        categories = "social",
        tags = "verified,unverified,lost,delete,remove",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeX,
    #[cfg(feature = "badge")]
    #[strum(props(
        svg = "<path d=\"M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z\"></path>",
        categories = "account,social,shapes",
        tags = "check,verified,unverified",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Badge,
    #[cfg(feature = "baggage_claim")]
    #[strum(props(
        svg = "<path d=\"M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2\"></path><path d=\"M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10\"></path><rect width=\"13\" height=\"8\" y=\"6\" rx=\"1\" x=\"8\"></rect><circle cy=\"20\" r=\"2\" cx=\"18\"></circle><circle r=\"2\" cx=\"9\" cy=\"20\"></circle>",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "balloon")]
    #[strum(props(
        svg = "<path d=\"M12 16v1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v1\"></path><path d=\"M12 6a2 2 0 0 1 2 2\"></path><path d=\"M18 8c0 4-3.5 8-6 8s-6-4-6-8a6 6 0 0 1 12 0\"></path>",
        categories = "emoji",
        tags = "party,festival,congratulations,celebration,decoration,colorful,floating,fun,birthday,event,entertainment",
        contributors = "peteruithoven"
    ))]
    Balloon,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M4.92 4.92 19.07 19.07\"></path>",
        categories = "account",
        tags = "cancel,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,circle,slash,null,void",
        contributors = "colebemis"
    ))]
    Ban,
    #[cfg(feature = "banana")]
    #[strum(props(
        svg = "<path d=\"M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5\"></path><path d=\"M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z\"></path>",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Banana,
    #[cfg(feature = "bandage")]
    #[strum(props(
        svg = "<path d=\"M10 10.01h.01\"></path><path d=\"M10 14.01h.01\"></path><path d=\"M14 10.01h.01\"></path><path d=\"M14 14.01h.01\"></path><path d=\"M18 6v12\"></path><path d=\"M6 6v12\"></path><rect rx=\"2\" width=\"20\" x=\"2\" y=\"6\" height=\"12\"></rect>",
        categories = "medical",
        tags = "plaster,band-aid,first aid,medical,health,wound,injury,care,treatment,healing,protection,emergency,aid,safety,patch",
        contributors = "karsa-mistmere,jamiemlaw,jguddas"
    ))]
    Bandage,
    #[cfg(feature = "banknote_arrow_down")]
    #[strum(props(
        svg = "<path d=\"M12 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5\"></path><path d=\"m16 19 3 3 3-3\"></path><path d=\"M18 12h.01\"></path><path d=\"M19 16v6\"></path><path d=\"M6 12h.01\"></path><circle cx=\"12\" r=\"2\" cy=\"12\"></circle>",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,withdraw,expense,out,payout,refund,debit,spending,decrease",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteArrowDown,
    #[cfg(feature = "banknote_arrow_up")]
    #[strum(props(
        svg = "<path d=\"M12 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5\"></path><path d=\"M18 12h.01\"></path><path d=\"M19 22v-6\"></path><path d=\"m22 19-3-3-3 3\"></path><path d=\"M6 12h.01\"></path><circle cy=\"12\" r=\"2\" cx=\"12\"></circle>",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,deposit,earnings,income,in,credit,prepaid,growth,increase",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteArrowUp,
    #[cfg(feature = "banknote_x")]
    #[strum(props(
        svg = "<path d=\"M13 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5\"></path><path d=\"m17 17 5 5\"></path><path d=\"M18 12h.01\"></path><path d=\"m22 17-5 5\"></path><path d=\"M6 12h.01\"></path><circle r=\"2\" cy=\"12\" cx=\"12\"></circle>",
        categories = "finance",
        tags = "bill,currency,money,payment,funds,transaction,cash,finance,error,failed,rejected,canceled,declined,lost,delete,remove",
        contributors = "AnnaSasDev,joffx,ericfennis,mittalyashu"
    ))]
    BanknoteX,
    #[cfg(feature = "banknote")]
    #[strum(props(
        svg = "<rect width=\"20\" y=\"6\" rx=\"2\" height=\"12\" x=\"2\"></rect><circle cy=\"12\" r=\"2\" cx=\"12\"></circle><path d=\"M6 12h.01M18 12h.01\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "barcode")]
    #[strum(props(
        svg = "<path d=\"M3 5v14\"></path><path d=\"M8 5v14\"></path><path d=\"M12 5v14\"></path><path d=\"M17 5v14\"></path><path d=\"M21 5v14\"></path>",
        categories = "shopping",
        tags = "scan,checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer",
        contributors = "danielbayley"
    ))]
    Barcode,
    #[cfg(feature = "barrel")]
    #[strum(props(
        svg = "<path d=\"M10 3a41 41 0 0 0 0 18\"></path><path d=\"M14 3a41 41 0 0 1 0 18\"></path><path d=\"M17 3a2 2 0 0 1 1.68.92 15.25 15.25 0 0 1 0 16.16A2 2 0 0 1 17 21H7a2 2 0 0 1-1.68-.92 15.25 15.25 0 0 1 0-16.16A2 2 0 0 1 7 3z\"></path><path d=\"M3.84 17h16.32\"></path><path d=\"M3.84 7h16.32\"></path>",
        categories = "food-beverage,navigation",
        tags = "keg,drum,tank,wine,beer,oak,wood,firkin,hogshead,kilderkin,barrique,solera,aging,whiskey,brewery,distillery,winery,vineyard",
        contributors = "karsa-mistmere,jamiemlaw"
    ))]
    Barrel,
    #[cfg(feature = "baseline")]
    #[strum(props(
        svg = "<path d=\"M4 20h16\"></path><path d=\"m6 16 6-12 6 12\"></path><path d=\"M8 12h8\"></path>",
        categories = "text",
        tags = "text,format,color",
        contributors = "ericfennis"
    ))]
    Baseline,
    #[cfg(feature = "bath")]
    #[strum(props(
        svg = "<path d=\"M10 4 8 6\"></path><path d=\"M17 19v2\"></path><path d=\"M2 12h20\"></path><path d=\"M7 19v2\"></path><path d=\"M9 5 7.62 3.62A2.12 2.12 0 0 0 4 5v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5\"></path>",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    Bath,
    #[cfg(feature = "battery_charging")]
    #[strum(props(
        svg = "<path d=\"m11 7-3 5h4l-3 5\"></path><path d=\"M14.85 6H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.93\"></path><path d=\"M22 14v-4\"></path><path d=\"M5.14 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2.93\"></path>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey,jguddas"
    ))]
    BatteryCharging,
    #[cfg(feature = "battery_full")]
    #[strum(props(
        svg = "<path d=\"M10 10v4\"></path><path d=\"M14 10v4\"></path><path d=\"M22 14v-4\"></path><path d=\"M6 10v4\"></path><rect width=\"16\" y=\"6\" height=\"12\" rx=\"2\" x=\"2\"></rect>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey,jguddas"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "<path d=\"M22 14v-4\"></path><path d=\"M6 14v-4\"></path><rect height=\"12\" rx=\"2\" x=\"2\" y=\"6\" width=\"16\"></rect>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere,jguddas"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "<path d=\"M10 14v-4\"></path><path d=\"M22 14v-4\"></path><path d=\"M6 14v-4\"></path><rect y=\"6\" height=\"12\" width=\"16\" x=\"2\" rx=\"2\"></rect>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere,jguddas"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_plus")]
    #[strum(props(
        svg = "<path d=\"M10 9v6\"></path><path d=\"M12.54 6H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.60\"></path><path d=\"M22 14v-4\"></path><path d=\"M7 12h6\"></path><path d=\"M7.60 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.60\"></path>",
        categories = "devices",
        tags = "power,electricity,energy,accumulator,charge,plus,economy,health,add,new,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis,jguddas,johnletey,Footagesus"
    ))]
    BatteryPlus,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "<path d=\"M10 17h.01\"></path><path d=\"M10 7v6\"></path><path d=\"M14 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2\"></path><path d=\"M22 14v-4\"></path><path d=\"M6 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2\"></path>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis,jguddas"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "<path d=\"M 22 14 L 22 10\"></path><rect x=\"2\" width=\"16\" height=\"12\" y=\"6\" rx=\"2\"></rect>",
        categories = "connectivity,devices",
        tags = "power,electricity,energy,accumulator,charge",
        contributors = "colebemis,ericfennis,johnletey,jguddas"
    ))]
    Battery,
    #[cfg(feature = "beaker")]
    #[strum(props(
        svg = "<path d=\"M4.5 3h15\"></path><path d=\"M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3\"></path><path d=\"M6 14h12\"></path>",
        categories = "science,gaming",
        tags = "cup,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis"
    ))]
    Beaker,
    #[cfg(feature = "bean_off")]
    #[strum(props(
        svg = "<path d=\"M9 9c-.64.64-1.52.954-2.40 1.16A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1\"></path><path d=\"M10.75 5.09A6 6 0 0 1 22 8c0 2.41-.61 4.68-1.68 6.66\"></path><path d=\"M5.34 10.62a4 4 0 0 0 6.48 1.20M10.62 5.34a4.01 4.01 0 0 1 2.03 2.04\"></path><line y2=\"22\" x2=\"22\" x1=\"2\" y1=\"2\"></line>",
        categories = "food-beverage",
        tags = "soy free,legume,soy,food,seed,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BeanOff,
    #[cfg(feature = "bean")]
    #[strum(props(
        svg = "<path d=\"M10.16 6.59C9.95 7.47 9.64 8.36 9 9c-.64.64-1.52.954-2.40 1.16A6 6 0 0 0 8 22c7.73 0 14-6.26 14-14a6 6 0 0 0-11.83-1.40Z\"></path><path d=\"M5.34 10.62a4 4 0 1 0 5.27-5.28\"></path>",
        categories = "food-beverage",
        tags = "legume,soy,food,seed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bean,
    #[cfg(feature = "bed_double")]
    #[strum(props(
        svg = "<path d=\"M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8\"></path><path d=\"M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4\"></path><path d=\"M12 4v6\"></path><path d=\"M2 18h20\"></path>",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedDouble,
    #[cfg(feature = "bed_single")]
    #[strum(props(
        svg = "<path d=\"M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8\"></path><path d=\"M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4\"></path><path d=\"M3 18h18\"></path>",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedSingle,
    #[cfg(feature = "bed")]
    #[strum(props(
        svg = "<path d=\"M2 4v16\"></path><path d=\"M2 8h18a2 2 0 0 1 2 2v10\"></path><path d=\"M2 17h20\"></path><path d=\"M6 8v9\"></path>",
        categories = "home",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bed,
    #[cfg(feature = "beef_off")]
    #[strum(props(
        svg = "<path d=\"M11.77 6.10a2.5 2.5 0 0 1 3.12 3.12\"></path><path d=\"M17.85 12.18a6.5 6.5 0 0 0-9.03-9.04\"></path><path d=\"M18.01 18.01C15.02 20.34 10.83 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5\"></path><path d=\"m18.5 6 2.19 4.5a6.48 6.48 0 0 1-.139 4.39\"></path><path d=\"m2 2 20 20\"></path><path d=\"M6.35 6.37a7 7 0 0 0-.075.23c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c3.35 0 6.99-1.26 9.85-3.15\"></path>",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak,vegetarian",
        contributors = "kemie,ericfennis,karsa-mistmere,jguddas"
    ))]
    BeefOff,
    #[cfg(feature = "beef")]
    #[strum(props(
        svg = "<path d=\"M16.4 13.7A6.5 6.5 0 1 0 6.28 6.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3\"></path><path d=\"m18.5 6 2.19 4.5a6.48 6.48 0 0 1-2.29 7.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5\"></path><circle cx=\"12.5\" cy=\"8.5\" r=\"2.5\"></circle>",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak",
        contributors = "kemie,ericfennis,karsa-mistmere"
    ))]
    Beef,
    #[cfg(feature = "beer_off")]
    #[strum(props(
        svg = "<path d=\"M13 13v5\"></path><path d=\"M17 11.47V8\"></path><path d=\"M17 11h1a3 3 0 0 1 2.74 4.21\"></path><path d=\"m2 2 20 20\"></path><path d=\"M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-3\"></path><path d=\"M7.53 7.53C6.76 7.64 6.15 8 5.5 8a2.5 2.5 0 0 1-1.76-4.26\"></path><path d=\"M8.72 3.20C9.30 2.76 9.88 2 11 2c1.56 0 2 1.5 3 1.5s1.72-.5 2.5-.5a1 1 0 1 1 0 5c-.78 0-1.5-.5-2.5-.5a3.14 3.14 0 0 0-.842.12\"></path><path d=\"M9 14.6V18\"></path>",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    BeerOff,
    #[cfg(feature = "beer")]
    #[strum(props(
        svg = "<path d=\"M17 11h1a3 3 0 0 1 0 6h-1\"></path><path d=\"M9 12v6\"></path><path d=\"M13 12v6\"></path><path d=\"M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z\"></path><path d=\"M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8\"></path>",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Beer,
    #[cfg(feature = "bell_dot")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M11.68 2.00A6 6 0 0 0 6 8c0 4.49-1.41 5.95-2.73 7.32A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.67c-.824-.85-1.67-1.73-2.21-3.34\"></path><circle r=\"3\" cx=\"18\" cy=\"5\"></circle>",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley,jguddas"
    ))]
    BellDot,
    #[cfg(feature = "bell_electric")]
    #[strum(props(
        svg = "<path d=\"M18.51 17.34A7 7 0 0 1 14 19\"></path><path d=\"M18.8 4A11 11 0 0 1 20 9\"></path><path d=\"M9 9h.01\"></path><circle cx=\"20\" r=\"2\" cy=\"16\"></circle><circle cx=\"9\" cy=\"9\" r=\"7\"></circle><rect height=\"6\" y=\"16\" width=\"10\" x=\"4\" rx=\"2\"></rect>",
        categories = "devices,notifications,home",
        tags = "fire alarm,flames,smoke,firefighter,fireman,department,brigade,station,emergency,alert,safety,school bell,period break,recess,doorbell,entrance,entry,ring,reception",
        contributors = "danielbayley,jguddas"
    ))]
    BellElectric,
    #[cfg(feature = "bell_minus")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M15 8h6\"></path><path d=\"M16.24 3.75A6 6 0 0 0 6 8c0 4.49-1.41 5.95-2.73 7.32A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.67A9.4 9.4 0 0 1 18.66 12\"></path>",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder,delete,remove,erase",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellMinus,
    #[cfg(feature = "bell_off")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M17 17H4a1 1 0 0 1-.74-1.67C4.59 13.95 6 12.49 6 8a6 6 0 0 1 .258-1.74\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8.66 3.01A6 6 0 0 1 18 8c0 2.68.77 4.65 1.70 6.05\"></path>",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    BellOff,
    #[cfg(feature = "bell_plus")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M15 8h6\"></path><path d=\"M18 5v6\"></path><path d=\"M20.00 14.46a9 9 0 0 0 .738.86A1 1 0 0 1 20 17H4a1 1 0 0 1-.74-1.67C4.59 13.95 6 12.49 6 8a6 6 0 0 1 8.75-5.33\"></path>",
        categories = "notifications",
        tags = "notification,silent,reminder,add,create,new",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellPlus,
    #[cfg(feature = "bell_ring")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M22 8c0-2.3-.8-4.3-2-6\"></path><path d=\"M3.26 15.32A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.67C19.41 13.95 18 12.49 18 8A6 6 0 0 0 6 8c0 4.49-1.41 5.95-2.73 7.32\"></path><path d=\"M4 2C2.8 3.7 2 5.7 2 8\"></path>",
        categories = "notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "ericfennis,danielbayley"
    ))]
    BellRing,
    #[cfg(feature = "bell")]
    #[strum(props(
        svg = "<path d=\"M10.26 21a2 2 0 0 0 3.46 0\"></path><path d=\"M3.26 15.32A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.67C19.41 13.95 18 12.49 18 8A6 6 0 0 0 6 8c0 4.49-1.41 5.95-2.73 7.32\"></path>",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    Bell,
    #[cfg(feature = "between_horizontal_end")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" height=\"7\" rx=\"1\" width=\"13\"></rect><path d=\"m22 15-3-3 3-3\"></path><rect height=\"7\" width=\"13\" rx=\"1\" x=\"3\" y=\"14\"></rect>",
        categories = "layout,design,tools",
        tags = "insert,add,left,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalEnd,
    #[cfg(feature = "between_horizontal_start")]
    #[strum(props(
        svg = "<rect rx=\"1\" width=\"13\" height=\"7\" x=\"8\" y=\"3\"></rect><path d=\"m2 9 3 3-3 3\"></path><rect width=\"13\" height=\"7\" rx=\"1\" x=\"8\" y=\"14\"></rect>",
        categories = "layout,design,tools",
        tags = "insert,add,right,slot,squeeze,space,vertical,grid,table,rows,cells,excel,spreadsheet,accountancy,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenHorizontalStart,
    #[cfg(feature = "between_vertical_end")]
    #[strum(props(
        svg = "<rect rx=\"1\" width=\"7\" height=\"13\" x=\"3\" y=\"3\"></rect><path d=\"m9 22 3-3 3 3\"></path><rect y=\"3\" width=\"7\" height=\"13\" rx=\"1\" x=\"14\"></rect>",
        categories = "layout,design,tools",
        tags = "insert,add,top,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalEnd,
    #[cfg(feature = "between_vertical_start")]
    #[strum(props(
        svg = "<rect height=\"13\" y=\"8\" rx=\"1\" x=\"3\" width=\"7\"></rect><path d=\"m15 2-3 3-3-3\"></path><rect rx=\"1\" y=\"8\" height=\"13\" width=\"7\" x=\"14\"></rect>",
        categories = "layout,design,tools",
        tags = "insert,add,bottom,slot,squeeze,space,vertical,grid,table,columns,cells,data,enter,entry,entries,blocks,rectangles,chevron",
        contributors = "danielbayley"
    ))]
    BetweenVerticalStart,
    #[cfg(feature = "biceps_flexed")]
    #[strum(props(
        svg = "<path d=\"M12.40 13.01A5 5 0 0 1 22 15c0 3.86-4 7-9 7-4.07 0-8.15-.82-10.37-2.46-.426-.316-.631-.832-.62-1.36C2.11 12.72 2.62 2 10 2a3 3 0 0 1 3 3 2 2 0 0 1-2 2c-1.10 0-1.64-.444-2-1\"></path><path d=\"M15 14a5 5 0 0 0-7.58 2\"></path><path d=\"M9.96 6.82C8.01 7.97 9.5 13 8 15\"></path>",
        categories = "emoji",
        tags = "arm,muscle,strong,working out,athletic,toned,muscular,forelimb,curled",
        contributors = "karsa-mistmere"
    ))]
    BicepsFlexed,
    #[cfg(feature = "bike")]
    #[strum(props(
        svg = "<circle r=\"3.5\" cx=\"18.5\" cy=\"17.5\"></circle><circle cx=\"5.5\" cy=\"17.5\" r=\"3.5\"></circle><circle cx=\"15\" cy=\"5\" r=\"1\"></circle><path d=\"M12 17.5V14l-3-3 4-3 2 3h2\"></path>",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "<rect height=\"6\" y=\"14\" x=\"14\" width=\"4\" rx=\"2\"></rect><rect height=\"6\" y=\"4\" x=\"6\" width=\"4\" rx=\"2\"></rect><path d=\"M6 20h4\"></path><path d=\"M14 10h4\"></path><path d=\"M6 14h2v6\"></path><path d=\"M14 4h2v6\"></path>",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "binoculars")]
    #[strum(props(
        svg = "<path d=\"M10 10h4\"></path><path d=\"M19 7V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3\"></path><path d=\"M20 21a2 2 0 0 0 2-2v-3.85c0-1.39-2-2.96-2-4.82V8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v11a2 2 0 0 0 2 2z\"></path><path d=\"M 22 16 L 2 16\"></path><path d=\"M4 21a2 2 0 0 1-2-2v-3.85c0-1.39 2-2.96 2-4.82V8a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v11a2 2 0 0 1-2 2z\"></path><path d=\"M9 7V4a1 1 0 0 0-1-1H6a1 1 0 0 0-1 1v3\"></path>",
        categories = "navigation,nature,photography,science,travel,development",
        tags = "field glasses,lorgnette,pince-nez,observation,sightseeing,nature,wildlife,birdwatching,scouting,surveillance,search,discovery,monitoring,lookout,viewpoint,travel,tourism,research",
        contributors = "karsa-mistmere"
    ))]
    Binoculars,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "<circle r=\"2\" cx=\"12\" cy=\"11.9\"></circle><path d=\"M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6\"></path><path d=\"m8.9 10.1 1.4.8\"></path><path d=\"M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5\"></path><path d=\"m15.1 10.1-1.4.8\"></path><path d=\"M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2\"></path><path d=\"M12 13.9v1.6\"></path><path d=\"M13.5 5.4c-1-.2-2-.2-3 0\"></path><path d=\"M17 16.4c.7-.7 1.2-1.6 1.5-2.5\"></path><path d=\"M5.5 13.9c.3.9.8 1.8 1.5 2.5\"></path>",
        categories = "science",
        tags = "fallout,waste,biology,chemistry,chemical,element",
        contributors = "karsa-mistmere,danielbayley,ericfennis"
    ))]
    Biohazard,
    #[cfg(feature = "bird")]
    #[strum(props(
        svg = "<path d=\"M16 7h.01\"></path><path d=\"M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20\"></path><path d=\"m20 7 2 .5-2 .5\"></path><path d=\"M10 18v3\"></path><path d=\"M14 17.75V21\"></path><path d=\"M7 18a6 6 0 0 0 3.84-10.61\"></path>",
        categories = "animals",
        tags = "peace,freedom,wing,avian,tweet",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Bird,
    #[cfg(feature = "birdhouse")]
    #[strum(props(
        svg = "<path d=\"M12 18v4\"></path><path d=\"m17 18 1.95-11.46\"></path><path d=\"m3 8 7.82-5.61a2 2 0 0 1 2.36 0L21 8\"></path><path d=\"M4 18h16\"></path><path d=\"M7 18 5.04 6.53\"></path><circle r=\"2\" cx=\"12\" cy=\"10\"></circle>",
        categories = "nature,animals,navigation,home",
        tags = "birdhouse,bird,garden,home,house,woodwork",
        contributors = "hieu-onefold,karsa-mistmere"
    ))]
    Birdhouse,
    #[cfg(feature = "bitcoin")]
    #[strum(props(
        svg = "<path d=\"M11.76 19.08c4.92.868 6.14-6.02 1.21-6.89m-1.21 6.89L5.86 18.04m5.90 1.04-.347 1.97m1.56-8.86c4.92.869 6.14-6.02 1.21-6.89m-1.21 6.89-3.94-.694m5.15-6.2L8.29 4.26m5.90 1.04.348-1.97M7.48 20.36l3.12-17.72\"></path>",
        categories = "development,finance",
        tags = "cryptocurrency,digital,blockchain,finance,coin,market,decentralized,investment,crypto,currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Bitcoin,
    #[cfg(feature = "blend")]
    #[strum(props(
        svg = "<circle cy=\"9\" r=\"7\" cx=\"9\"></circle><circle r=\"7\" cy=\"15\" cx=\"15\"></circle>",
        categories = "design,photography,tools,development",
        tags = "mode,overlay,multiply,screen,opacity,transparency,alpha,filters,lenses,mixed,shades,tints,hues,saturation,brightness,overlap,colors,colours",
        contributors = "danielbayley"
    ))]
    Blend,
    #[cfg(feature = "blinds")]
    #[strum(props(
        svg = "<path d=\"M3 3h18\"></path><path d=\"M20 7H8\"></path><path d=\"M20 11H8\"></path><path d=\"M10 19h10\"></path><path d=\"M8 15h12\"></path><path d=\"M4 3v14\"></path><circle cx=\"4\" cy=\"19\" r=\"2\"></circle>",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[cfg(feature = "blocks")]
    #[strum(props(
        svg = "<path d=\"M10 22V7a1 1 0 0 0-1-1H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5a1 1 0 0 0-1-1H2\"></path><rect rx=\"1\" height=\"8\" width=\"8\" x=\"14\" y=\"2\"></rect>",
        categories = "development,layout,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning,squares,corner",
        contributors = "danielbayley,jguddas"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "<path d=\"m7 7 10 10-5 5V2l5 5L7 17\"></path><line y2=\"12\" y1=\"12\" x1=\"18\" x2=\"21\"></line><line x1=\"3\" y1=\"12\" x2=\"6\" y2=\"12\"></line>",
        categories = "connectivity,devices",
        tags = "paired",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothConnected,
    #[cfg(feature = "bluetooth_off")]
    #[strum(props(
        svg = "<path d=\"m17 17-5 5V12l-5 5\"></path><path d=\"m2 2 20 20\"></path><path d=\"M14.5 9.5 17 7l-5-5v4.5\"></path>",
        categories = "connectivity,devices",
        tags = "lost",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothOff,
    #[cfg(feature = "bluetooth_searching")]
    #[strum(props(
        svg = "<path d=\"m7 7 10 10-5 5V2l5 5L7 17\"></path><path d=\"M20.83 14.83a4 4 0 0 0 0-5.66\"></path><path d=\"M18 12h.01\"></path>",
        categories = "connectivity,devices",
        tags = "pairing",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothSearching,
    #[cfg(feature = "bluetooth")]
    #[strum(props(
        svg = "<path d=\"m7 7 10 10-5 5V2l5 5L7 17\"></path>",
        categories = "connectivity,devices",
        tags = "wireless",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    Bluetooth,
    #[cfg(feature = "bold")]
    #[strum(props(
        svg = "<path d=\"M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8\"></path>",
        categories = "text",
        tags = "text,strong,format",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Bold,
    #[cfg(feature = "bolt")]
    #[strum(props(
        svg = "<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\"></path><circle r=\"4\" cy=\"12\" cx=\"12\"></circle>",
        categories = "tools,home",
        tags = "nut,screw,settings,preferences,configuration,controls,edit,diy,fixed,build,construction,parts",
        contributors = "danielbayley"
    ))]
    Bolt,
    #[cfg(feature = "bomb")]
    #[strum(props(
        svg = "<circle cx=\"11\" cy=\"13\" r=\"9\"></circle><path d=\"M14.35 4.65 16.3 2.7a2.41 2.41 0 0 1 3.4 0l1.6 1.6a2.4 2.4 0 0 1 0 3.4l-1.95 1.95\"></path><path d=\"m22 2-1.5 1.5\"></path>",
        categories = "security,tools",
        tags = "fatal,error,crash,blockbuster,mine,explosion,explode,explosive",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bomb,
    #[cfg(feature = "bone")]
    #[strum(props(
        svg = "<path d=\"M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z\"></path>",
        categories = "animals,medical,gaming",
        tags = "health,skeleton,skull,death,pets,dog",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bone,
    #[cfg(feature = "book_a")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"m8 13 4-7 4 7\"></path><path d=\"M9.1 11h5.7\"></path>",
        categories = "text,gaming",
        tags = "dictionary,define,definition,thesaurus,encyclopedia,encyclopaedia,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,alphabetical,a-z,ordered",
        contributors = "danielbayley"
    ))]
    BookA,
    #[cfg(feature = "book_alert")]
    #[strum(props(
        svg = "<path d=\"M12 13h.01\"></path><path d=\"M12 6v3\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path>",
        categories = "text,development,gaming",
        tags = "reading,paperback,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,warning,alert,danger,exclamation mark",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,lscheibel,domasmark"
    ))]
    BookAlert,
    #[cfg(feature = "book_audio")]
    #[strum(props(
        svg = "<path d=\"M12 6v7\"></path><path d=\"M16 8v3\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M8 8v3\"></path>",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookAudio,
    #[cfg(feature = "book_check")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"m9 9.5 2 2 4-4\"></path>",
        categories = "text,development,gaming",
        tags = "read,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "danielbayley"
    ))]
    BookCheck,
    #[cfg(feature = "book_copy")]
    #[strum(props(
        svg = "<path d=\"M5 7a2 2 0 0 0-2 2v11\"></path><path d=\"M5.80 18H5a2 2 0 0 0 0 4h9.5a.5.5 0 0 0 .5-.5V21\"></path><path d=\"M9 15V4a2 2 0 0 1 2-2h9.5a.5.5 0 0 1 .5.5v14a.5.5 0 0 1-.5.5H11a2 2 0 0 1 0-4h10\"></path>",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,clone,fork,duplicate,multiple,books,library,copies,copied,plagiarism,plagiarised,plagiarized,reading list,information,informed,knowledge,knowledgeable,knowledgable,education,high school,university,college,academy,student,study,learning,research,smart,intelligent,intellectual",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookCopy,
    #[cfg(feature = "book_dashed")]
    #[strum(props(
        svg = "<path d=\"M12 17h1.5\"></path><path d=\"M12 22h1.5\"></path><path d=\"M12 2h1.5\"></path><path d=\"M17.5 22H19a1 1 0 0 0 1-1\"></path><path d=\"M17.5 2H19a1 1 0 0 1 1 1v1.5\"></path><path d=\"M20 14v3h-2.5\"></path><path d=\"M20 8.5V10\"></path><path d=\"M4 10V8.5\"></path><path d=\"M4 19.5V14\"></path><path d=\"M4 4.5A2.5 2.5 0 0 1 6.5 2H8\"></path><path d=\"M8 22H6.5a1 1 0 0 1 0-5H8\"></path>",
        categories = "development",
        tags = "code,coding,version control,git,repository,template,draft,script,screenplay,writing,writer,author,unwritten,unpublished,untold",
        contributors = "danielbayley,jguddas"
    ))]
    BookDashed,
    #[cfg(feature = "book_down")]
    #[strum(props(
        svg = "<path d=\"M12 13V7\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"m9 10 3 3 3-3\"></path>",
        categories = "development",
        tags = "code,coding,version control,git,repository,pull",
        contributors = "danielbayley"
    ))]
    BookDown,
    #[cfg(feature = "book_headphones")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M8 12v-2a4 4 0 0 1 8 0v2\"></path><circle r=\"1\" cx=\"15\" cy=\"12\"></circle><circle r=\"1\" cy=\"12\" cx=\"9\"></circle>",
        categories = "multimedia,text",
        tags = "audiobook,reading,listening,sound,story,fiction,novel,information,knowledge,education,student,study,learning,research",
        contributors = "danielbayley"
    ))]
    BookHeadphones,
    #[cfg(feature = "book_heart")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M8.62 9.8A2.25 2.25 0 1 1 12 6.83a2.25 2.25 0 1 1 3.38 2.96l-2.62 2.85a.998.99 0 0 1-1.50 0z\"></path>",
        categories = "social,text,gaming",
        tags = "diary,romance,novel,journal,entry,entries,personal,private,secret,crush,like,love,emotion,feminine,girls,teens,teenager,therapy,theraputic,therapist,planner,organizer,organiser,notes,notepad,stationery,sketchbook,writing,written,reading,favorite,favourite,high school",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BookHeart,
    #[cfg(feature = "book_image")]
    #[strum(props(
        svg = "<path d=\"m20 13.7-2.1-2.1a2 2 0 0 0-2.8 0L9.7 17\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><circle r=\"2\" cy=\"8\" cx=\"10\"></circle>",
        categories = "photography,text,multimedia,files,social,shopping,travel",
        tags = "images,pictures,photos,album,collection,event,magazine,catalog,catalogue,brochure,browse,gallery",
        contributors = "danielbayley,jguddas"
    ))]
    BookImage,
    #[cfg(feature = "book_key")]
    #[strum(props(
        svg = "<path d=\"M13 2H6.5A2.5 2.5 0 0 0 4 4.5v15\"></path><path d=\"M17 2v6\"></path><path d=\"M17 4h2\"></path><path d=\"M20 15.2V21a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><circle cx=\"17\" r=\"2\" cy=\"10\"></circle>",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,public,secret,unlocked,hidden,revealed,knowledge,learning",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "<path d=\"M18 6V4a2 2 0 1 0-4 0v2\"></path><path d=\"M20 15v6a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10\"></path><rect width=\"8\" x=\"12\" y=\"6\" height=\"5\" rx=\"1\"></rect>",
        categories = "development,security,gaming",
        tags = "code,coding,version control,git,repository,private,secret,hidden,knowledge",
        contributors = "danielbayley"
    ))]
    BookLock,
    #[cfg(feature = "book_marked")]
    #[strum(props(
        svg = "<path d=\"M10 2v8l3-3 3 3V2\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path>",
        categories = "text,development,gaming",
        tags = "dictionary,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,saved,later,future,reference,index,code,coding,version control,git,repository",
        contributors = "danielbayley"
    ))]
    BookMarked,
    #[cfg(feature = "book_minus")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M9 10h6\"></path>",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,censor,cancel,forbid,prohibit,ban,uneducated,re-educate,unlearn,downgrade",
        contributors = "danielbayley"
    ))]
    BookMinus,
    #[cfg(feature = "book_open_check")]
    #[strum(props(
        svg = "<path d=\"M12 21V7\"></path><path d=\"m16 12 2 2 4-4\"></path><path d=\"M22 6V4a1 1 0 0 0-1-1h-5a4 4 0 0 0-4 4 4 4 0 0 0-4-4H3a1 1 0 0 0-1 1v13a1 1 0 0 0 1 1h6a3 3 0 0 1 3 3 3 3 0 0 1 3-3h6a1 1 0 0 0 1-1v-1.3\"></path>",
        categories = "text,development,gaming",
        tags = "read,pages,booklet,magazine,leaflet,pamphlet,library,written,authored,published,informed,knowledgeable,educated,schooled,homework,examined,tested,marked,passed,graduated,studied,learned,lesson,researched,documented,revealed,blank,plain language,true,truth,verified,corrected,task,todo,done,completed,finished,ticked",
        contributors = "schmidt-oliver,karsa-mistmere,ericfennis"
    ))]
    BookOpenCheck,
    #[cfg(feature = "book_open_text")]
    #[strum(props(
        svg = "<path d=\"M12 7v14\"></path><path d=\"M16 12h2\"></path><path d=\"M16 8h2\"></path><path d=\"M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z\"></path><path d=\"M6 12h2\"></path><path d=\"M6 8h2\"></path>",
        categories = "text,development",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed",
        contributors = "danielbayley"
    ))]
    BookOpenText,
    #[cfg(feature = "book_open")]
    #[strum(props(
        svg = "<path d=\"M12 7v14\"></path><path d=\"M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z\"></path>",
        categories = "text,development,gaming",
        tags = "reading,pages,booklet,magazine,leaflet,pamphlet,library,writing,written,writer,author,story,script,screenplay,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation,revealed,blank,plain",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    BookOpen,
    #[cfg(feature = "book_plus")]
    #[strum(props(
        svg = "<path d=\"M12 7v6\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M9 10h6\"></path>",
        categories = "development,text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,read,write,author,publish,inform,graduate,re-educate,study,learn,research,knowledge,improve,upgrade,level up",
        contributors = "danielbayley"
    ))]
    BookPlus,
    #[cfg(feature = "book_search")]
    #[strum(props(
        svg = "<path d=\"M11 22H5.5a1 1 0 0 1 0-5h4.50\"></path><path d=\"m21 22-1.87-1.87\"></path><path d=\"M3 19.5v-15A2.5 2.5 0 0 1 5.5 2H18a1 1 0 0 1 1 1v8\"></path><circle r=\"3\" cx=\"17\" cy=\"18\"></circle>",
        categories = "text,development,gaming",
        tags = "reading,library,study,education,research,knowledge,discover,browsing,lookup,finding,scanning",
        contributors = "karsa-mistmere,Muhammad-Aqib-Bashir"
    ))]
    BookSearch,
    #[cfg(feature = "book_text")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M8 11h8\"></path><path d=\"M8 7h6\"></path>",
        categories = "text,gaming",
        tags = "reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "danielbayley"
    ))]
    BookText,
    #[cfg(feature = "book_type")]
    #[strum(props(
        svg = "<path d=\"M10 13h4\"></path><path d=\"M12 6v7\"></path><path d=\"M16 8V6H8v2\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path>",
        categories = "text,design,gaming",
        tags = "thesaurus,synonym,reading,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,language,translate,typography,fonts,collection",
        contributors = "danielbayley"
    ))]
    BookType,
    #[cfg(feature = "book_up_2")]
    #[strum(props(
        svg = "<path d=\"M12 13V7\"></path><path d=\"M18 2h1a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2\"></path><path d=\"m9 10 3-3 3 3\"></path><path d=\"m9 5 3-3 3 3\"></path>",
        categories = "development",
        tags = "code,coding,version control,git,repository,push,force",
        contributors = "danielbayley"
    ))]
    BookUp2,
    #[cfg(feature = "book_up")]
    #[strum(props(
        svg = "<path d=\"M12 13V7\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"m9 10 3-3 3 3\"></path>",
        categories = "development",
        tags = "code,coding,version control,git,repository,push",
        contributors = "danielbayley"
    ))]
    BookUp,
    #[cfg(feature = "book_user")]
    #[strum(props(
        svg = "<path d=\"M15 13a3 3 0 1 0-6 0\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><circle cx=\"12\" cy=\"8\" r=\"2\"></circle>",
        categories = "account,connectivity,communication,social",
        tags = "person,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking",
        contributors = "danielbayley"
    ))]
    BookUser,
    #[cfg(feature = "book_x")]
    #[strum(props(
        svg = "<path d=\"m14.5 7-5 5\"></path><path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path><path d=\"m9.5 7 5 5\"></path>",
        categories = "text,gaming",
        tags = "code,coding,version control,git,repository,remove,delete,reading,misinformation,disinformation,misinformed,charlatan,sophistry,false,lies,untruth,propaganda,censored,cancelled,forbidden,prohibited,banned,uneducated,re-education,unlearn",
        contributors = "danielbayley"
    ))]
    BookX,
    #[cfg(feature = "book")]
    #[strum(props(
        svg = "<path d=\"M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20\"></path>",
        categories = "text,development,gaming",
        tags = "reading,paperback,booklet,magazine,leaflet,pamphlet,tome,library,writing,written,writer,author,story,script,fiction,novel,information,knowledge,education,high school,university,college,academy,student,study,learning,homework,research,documentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Book,
    #[cfg(feature = "bookmark_check")]
    #[strum(props(
        svg = "<path d=\"M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.49.868l-4.51-2.57a2 2 0 0 0-1.98 0l-4.51 2.57A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z\"></path><path d=\"m9 10 2 2 4-4\"></path>",
        categories = "account",
        tags = "read,finished,complete,clip,marker,tag,task,todo",
        contributors = "danielbayley,jguddas"
    ))]
    BookmarkCheck,
    #[cfg(feature = "bookmark_minus")]
    #[strum(props(
        svg = "<path d=\"M15 10H9\"></path><path d=\"M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.49.868l-4.51-2.57a2 2 0 0 0-1.98 0l-4.51 2.57A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z\"></path>",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis,jguddas"
    ))]
    BookmarkMinus,
    #[cfg(feature = "bookmark_plus")]
    #[strum(props(
        svg = "<path d=\"M12 7v6\"></path><path d=\"M15 10H9\"></path><path d=\"M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.49.868l-4.51-2.57a2 2 0 0 0-1.98 0l-4.51 2.57A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z\"></path>",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis,jguddas"
    ))]
    BookmarkPlus,
    #[cfg(feature = "bookmark_x")]
    #[strum(props(
        svg = "<path d=\"m14.5 7.5-5 5\"></path><path d=\"M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.49.868l-4.51-2.57a2 2 0 0 0-1.98 0l-4.51 2.57A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z\"></path><path d=\"m9.5 7.5 5 5\"></path>",
        categories = "account",
        tags = "read,clip,marker,tag,cancel,close,delete,remove,clear",
        contributors = "danielbayley,jguddas"
    ))]
    BookmarkX,
    #[cfg(feature = "bookmark")]
    #[strum(props(
        svg = "<path d=\"M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.49.868l-4.51-2.57a2 2 0 0 0-1.98 0l-4.51 2.57A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z\"></path>",
        categories = "account",
        tags = "save,favorite,mark,label,attachment,file,stick,pin,read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis,jguddas"
    ))]
    Bookmark,
    #[cfg(feature = "boom_box")]
    #[strum(props(
        svg = "<path d=\"M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4\"></path><path d=\"M8 8v1\"></path><path d=\"M12 8v1\"></path><path d=\"M16 8v1\"></path><rect rx=\"2\" y=\"9\" width=\"20\" height=\"12\" x=\"2\"></rect><circle r=\"2\" cy=\"15\" cx=\"8\"></circle><circle cx=\"16\" cy=\"15\" r=\"2\"></circle>",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = "danielbayley"
    ))]
    BoomBox,
    #[cfg(feature = "bot_message_square")]
    #[strum(props(
        svg = "<path d=\"M12 6V2H8\"></path><path d=\"M15 11v2\"></path><path d=\"M2 12h2\"></path><path d=\"M20 12h2\"></path><path d=\"M20 16a2 2 0 0 1-2 2H8.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 4 20.28V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2z\"></path><path d=\"M9 11v2\"></path>",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BotMessageSquare,
    #[cfg(feature = "bot_off")]
    #[strum(props(
        svg = "<path d=\"M13.67 8H18a2 2 0 0 1 2 2v4.33\"></path><path d=\"M2 14h2\"></path><path d=\"M20 14h2\"></path><path d=\"M22 22 2 2\"></path><path d=\"M8 8H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 1.41-.586\"></path><path d=\"M9 13v2\"></path><path d=\"M9.67 4H12v2.33\"></path>",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "taichimaeda,ericfennis"
    ))]
    BotOff,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "<path d=\"M12 8V4H8\"></path><rect x=\"4\" width=\"16\" rx=\"2\" y=\"8\" height=\"12\"></rect><path d=\"M2 14h2\"></path><path d=\"M20 14h2\"></path><path d=\"M15 13v2\"></path><path d=\"M9 13v2\"></path>",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "ericfennis"
    ))]
    Bot,
    #[cfg(feature = "bottle_wine")]
    #[strum(props(
        svg = "<path d=\"M10 3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v2a6 6 0 0 0 1.2 3.6l.6.8A6 6 0 0 1 17 13v8a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1v-8a6 6 0 0 1 1.2-3.6l.6-.8A6 6 0 0 0 10 5z\"></path><path d=\"M17 13h-4a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h4\"></path>",
        categories = "food-beverage",
        tags = "alcohol,drink,glass,goblet,chalice,vineyard,winery,red,white,rose,dry,sparkling,bar,party,nightclub,nightlife,sommelier,restaurant,dinner,meal",
        contributors = "jamiemlaw"
    ))]
    BottleWine,
    #[cfg(feature = "bow_arrow")]
    #[strum(props(
        svg = "<path d=\"M17 3h4v4\"></path><path d=\"M18.57 11.08a13 13 0 0 1 1.04 9.02 1.17 1.17 0 0 1-1.91.597L14 17\"></path><path d=\"M7 10 3.29 6.29a1.17 1.17 0 0 1 .6-1.91 13 13 0 0 1 9.03 1.05\"></path><path d=\"M7 14a1.7 1.7 0 0 0-1.20.5l-2.64 2.64A.5.5 0 0 0 3.5 18H5a1 1 0 0 1 1 1v1.5a.5.5 0 0 0 .854.35L9.5 18.20A1.7 1.7 0 0 0 10 17v-2a1 1 0 0 0-1-1z\"></path><path d=\"M9.70 14.29 21 3\"></path>",
        categories = "gaming,tools",
        tags = "archer,archery,game,war,weapon",
        contributors = "jamiemlaw"
    ))]
    BowArrow,
    #[cfg(feature = "box")]
    #[strum(props(
        svg = "<path d=\"M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z\"></path><path d=\"m3.3 7 8.7 5 8.7-5\"></path><path d=\"M12 22V12\"></path>",
        categories = "shapes,gaming,development,math",
        tags = "cube,package,container,storage,geometry,3d,isometric",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Box,
    #[cfg(feature = "boxes")]
    #[strum(props(
        svg = "<path d=\"M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z\"></path><path d=\"m7 16.5-4.74-2.85\"></path><path d=\"m7 16.5 5-3\"></path><path d=\"M7 16.5v5.17\"></path><path d=\"M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z\"></path><path d=\"m17 16.5-5-3\"></path><path d=\"m17 16.5 4.74-2.85\"></path><path d=\"M17 16.5v5.17\"></path><path d=\"M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z\"></path><path d=\"M12 8 7.26 5.15\"></path><path d=\"m12 8 4.74-2.85\"></path><path d=\"M12 13.5V8\"></path>",
        categories = "shapes,gaming,development",
        tags = "cubes,packages,parts,group,units,collection,cluster,geometry",
        contributors = "karsa-mistmere"
    ))]
    Boxes,
    #[cfg(feature = "braces")]
    #[strum(props(
        svg = "<path d=\"M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1\"></path><path d=\"M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1\"></path>",
        categories = "development,files",
        tags = "json,code,token,curly brackets,data,{,}",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Braces,
    #[cfg(feature = "brackets")]
    #[strum(props(
        svg = "<path d=\"M16 3h3a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-3\"></path><path d=\"M8 21H5a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h3\"></path>",
        categories = "development,files",
        tags = "code,token,array,list,square,[,]",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Brackets,
    #[cfg(feature = "brain_circuit")]
    #[strum(props(
        svg = "<path d=\"M12 5a3 3 0 1 0-5.99.125 4 4 0 0 0-2.52 5.77 4 4 0 0 0 .556 6.58A4 4 0 1 0 12 18Z\"></path><path d=\"M9 13a4.5 4.5 0 0 0 3-4\"></path><path d=\"M6.00 5.12A3 3 0 0 0 6.40 6.5\"></path><path d=\"M3.47 10.89a4 4 0 0 1 .585-.396\"></path><path d=\"M6 18a4 4 0 0 1-1.96-.516\"></path><path d=\"M12 13h4\"></path><path d=\"M12 18h6a2 2 0 0 1 2 2v1\"></path><path d=\"M12 8h8\"></path><path d=\"M16 8V5a2 2 0 0 1 2-2\"></path><circle cy=\"13\" r=\".5\" cx=\"16\"></circle><circle cy=\"3\" cx=\"18\" r=\".5\"></circle><circle cx=\"20\" cy=\"21\" r=\".5\"></circle><circle cx=\"20\" cy=\"8\" r=\".5\"></circle>",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,ericfennis"
    ))]
    BrainCircuit,
    #[cfg(feature = "brain_cog")]
    #[strum(props(
        svg = "<path d=\"m10.85 14.77-.383.92\"></path><path d=\"m10.85 9.22-.383-.923\"></path><path d=\"m13.14 14.77.382.92\"></path><path d=\"m13.53 8.30-.383.92\"></path><path d=\"m14.77 10.85.923-.383\"></path><path d=\"m14.77 13.14.923.38\"></path><path d=\"M17.59 6.5A3 3 0 1 0 12 5a3 3 0 0 0-5.63-1.44 3 3 0 0 0-.368 1.57 4 4 0 0 0-2.52 5.77\"></path><path d=\"M17.99 5.12a4 4 0 0 1 2.52 5.77\"></path><path d=\"M19.50 10.29a4 4 0 0 1-1.5 7.70\"></path><path d=\"M4.03 17.48A4 4 0 0 0 11.46 20c.18-.311.89-.311 1.07 0a4 4 0 0 0 7.43-2.51\"></path><path d=\"M4.5 10.29A4 4 0 0 0 6 18\"></path><path d=\"M6.00 5.12a3 3 0 0 0 .4 1.37\"></path><path d=\"m9.22 10.85-.923-.383\"></path><path d=\"m9.22 13.14-.923.38\"></path><circle cy=\"12\" r=\"3\" cx=\"12\"></circle>",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,UsamaKhan"
    ))]
    BrainCog,
    #[cfg(feature = "brain")]
    #[strum(props(
        svg = "<path d=\"M12 18V5\"></path><path d=\"M15 13a4.17 4.17 0 0 1-3-4 4.17 4.17 0 0 1-3 4\"></path><path d=\"M17.59 6.5A3 3 0 1 0 12 5a3 3 0 1 0-5.59 1.5\"></path><path d=\"M17.99 5.12a4 4 0 0 1 2.52 5.77\"></path><path d=\"M18 18a4 4 0 0 0 2-7.46\"></path><path d=\"M19.96 17.48A4 4 0 1 1 12 18a4 4 0 1 1-7.96-.517\"></path><path d=\"M6 18a4 4 0 0 1-2-7.46\"></path><path d=\"M6.00 5.12a4 4 0 0 0-2.52 5.77\"></path>",
        categories = "medical,science",
        tags = "medical,mind,mental,intellect,cerebral,consciousness,genius,artificial intelligence,ai,think,thought,insight,intelligent,smart",
        contributors = "karsa-mistmere,jguddas,it-is-not,jamiemlaw"
    ))]
    Brain,
    #[cfg(feature = "brick_wall_fire")]
    #[strum(props(
        svg = "<path d=\"M16 3v2.10\"></path><path d=\"M17 9c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 22 17a5 5 0 0 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C13 11.5 16 9 17 9\"></path><path d=\"M21 8.27V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.93\"></path><path d=\"M3 15h5.25\"></path><path d=\"M3 9h8.22\"></path><path d=\"M8 15v6\"></path><path d=\"M8 3v6\"></path>",
        categories = "security,home,connectivity",
        tags = "firewall,security,bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone,campfire,camping,wilderness,outdoors,lit,warmth,wood,twigs,sticks",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWallFire,
    #[cfg(feature = "brick_wall_shield")]
    #[strum(props(
        svg = "<path d=\"M12 9v1.25\"></path><path d=\"M16 3v5.46\"></path><path d=\"M21 9.11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h5.75\"></path><path d=\"M22 17.5c0 2.49-1.75 3.74-3.83 4.47a.5.5 0 0 1-.335-.005c-2.08-.72-3.83-1.97-3.83-4.47V14a.5.5 0 0 1 .5-.499c1 0 2.25-.6 3.12-1.36a.6.6 0 0 1 .76-.001c.875.76 2.12 1.36 3.12 1.36a.5.5 0 0 1 .5.5z\"></path><path d=\"M3 15h7\"></path><path d=\"M3 9h12.14\"></path><path d=\"M8 15v6\"></path><path d=\"M8 3v6\"></path>",
        categories = "security,home,connectivity",
        tags = "firewall,security,bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone,cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWallShield,
    #[cfg(feature = "brick_wall")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" x=\"3\" rx=\"2\" y=\"3\"></rect><path d=\"M12 9v6\"></path><path d=\"M16 15v6\"></path><path d=\"M16 3v6\"></path><path d=\"M3 15h18\"></path><path d=\"M3 9h18\"></path><path d=\"M8 15v6\"></path><path d=\"M8 3v6\"></path>",
        categories = "buildings,home",
        tags = "bricks,mortar,cement,materials,construction,builder,labourer,quantity surveyor,blocks,stone",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BrickWall,
    #[cfg(feature = "briefcase_business")]
    #[strum(props(
        svg = "<path d=\"M12 12h.01\"></path><path d=\"M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2\"></path><path d=\"M22 13a18.15 18.15 0 0 1-20 0\"></path><rect rx=\"2\" width=\"20\" x=\"2\" height=\"14\" y=\"6\"></rect>",
        categories = "transportation",
        tags = "work,bag,baggage,folder,portfolio",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    BriefcaseBusiness,
    #[cfg(feature = "briefcase_conveyor_belt")]
    #[strum(props(
        svg = "<path d=\"M10 20v2\"></path><path d=\"M14 20v2\"></path><path d=\"M18 20v2\"></path><path d=\"M21 20H3\"></path><path d=\"M6 20v2\"></path><path d=\"M8 16V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v12\"></path><rect x=\"4\" y=\"6\" rx=\"2\" width=\"16\" height=\"10\"></rect>",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase,conveyor,carousel",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BriefcaseConveyorBelt,
    #[cfg(feature = "briefcase_medical")]
    #[strum(props(
        svg = "<path d=\"M12 11v4\"></path><path d=\"M14 13h-4\"></path><path d=\"M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2\"></path><path d=\"M18 6v14\"></path><path d=\"M6 6v14\"></path><rect width=\"20\" x=\"2\" height=\"14\" y=\"6\" rx=\"2\"></rect>",
        categories = "medical,transportation",
        tags = "doctor,medicine,first aid",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    BriefcaseMedical,
    #[cfg(feature = "briefcase")]
    #[strum(props(
        svg = "<path d=\"M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16\"></path><rect y=\"6\" rx=\"2\" x=\"2\" width=\"20\" height=\"14\"></rect>",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "<rect width=\"8\" rx=\"2\" x=\"8\" height=\"8\" y=\"8\"></rect><path d=\"M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2\"></path><path d=\"M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2\"></path>",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[cfg(feature = "brush_cleaning")]
    #[strum(props(
        svg = "<path d=\"m16 22-1-4\"></path><path d=\"M19 14a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2h-3a1 1 0 0 1-1-1V4a2 2 0 0 0-4 0v5a1 1 0 0 1-1 1H6a2 2 0 0 0-2 2v1a1 1 0 0 0 1 1\"></path><path d=\"M19 14H5l-1.97 6.76A1 1 0 0 0 4 22h16a1 1 0 0 0 .973-1.23z\"></path><path d=\"m8 22 1-4\"></path>",
        categories = "home,tools,design",
        tags = "cleaning,utensil,housekeeping,tool,sweeping,scrubbing,hygiene,maintenance,household,cleaner,chores,equipment,sanitation,bristles,handle,home care,sanitize,purify,wash,disinfect,sterilize,scrub,polish,decontaminate,wipe,spotless,remove,empty,erase,purge,eliminate",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BrushCleaning,
    #[cfg(feature = "brush")]
    #[strum(props(
        svg = "<path d=\"m11 10 3 3\"></path><path d=\"M6.5 21A3.5 3.5 0 1 0 3 17.5a2.62 2.62 0 0 1-.708 1.79A1 1 0 0 0 3 21z\"></path><path d=\"M9.96 17.03 21.37 5.62a1 1 0 0 0-3.00-3.00L6.96 14.03\"></path>",
        categories = "text,design,tools",
        tags = "clean,sweep,refactor,remove,draw,paint,color,artist",
        contributors = "ericfennis,chessurisme,jguddas,karsa-mistmere"
    ))]
    Brush,
    #[cfg(feature = "bubbles")]
    #[strum(props(
        svg = "<path d=\"M7.00 15.08A1.5 1.5 0 0 1 9 16.5\"></path><circle r=\"3.5\" cx=\"18.5\" cy=\"8.5\"></circle><circle cx=\"7.5\" cy=\"16.5\" r=\"5.5\"></circle><circle r=\"2.5\" cy=\"4.5\" cx=\"7.5\"></circle>",
        categories = "weather",
        tags = "water,cleaning,soap,bath,hygiene,freshness,wash,foam,cleanliness,shampoo,purity,splash,lightness,airy,relaxation,spa,bubbly,fluid,floating,drop",
        contributors = "vqh2602,jguddas,karsa-mistmere"
    ))]
    Bubbles,
    #[cfg(feature = "bug_off")]
    #[strum(props(
        svg = "<path d=\"M12 20v-8\"></path><path d=\"M12.65 7H14a4 4 0 0 1 4 4v1.34\"></path><path d=\"M14.12 3.88 16 2\"></path><path d=\"M17.12 17.12A6 6 0 0 1 6 14v-3a4 4 0 0 1 1.72-3.28\"></path><path d=\"m2 2 20 20\"></path><path d=\"M21 5a4 4 0 0 1-3.55 3.97\"></path><path d=\"M22 13h-3.34\"></path><path d=\"M3 21a4 4 0 0 1 3.81-4\"></path><path d=\"M3 5a4 4 0 0 0 3.55 3.97\"></path><path d=\"M6 13H2\"></path><path d=\"m8 2 1.88 1.88\"></path><path d=\"M9.71 4.06A3 3 0 0 1 15 6v1.13\"></path>",
        categories = "development,animals",
        tags = "issue,fixed,resolved,testing,debug,code,insect,kill,exterminate,pest control",
        contributors = "danielbayley,jamiemlaw,jguddas"
    ))]
    BugOff,
    #[cfg(feature = "bug_play")]
    #[strum(props(
        svg = "<path d=\"M10 19.65A6 6 0 0 1 6 14v-3a4 4 0 0 1 4-4h4a4 4 0 0 1 4 3.97\"></path><path d=\"M14 15.00a1 1 0 0 1 1.51-.859l4.99 2.99a1 1 0 0 1 0 1.71l-4.99 2.99a1 1 0 0 1-1.51-.86z\"></path><path d=\"M14.12 3.88 16 2\"></path><path d=\"M21 5a4 4 0 0 1-3.55 3.97\"></path><path d=\"M3 21a4 4 0 0 1 3.81-4\"></path><path d=\"M3 5a4 4 0 0 0 3.55 3.97\"></path><path d=\"M6 13H2\"></path><path d=\"m8 2 1.88 1.88\"></path><path d=\"M9 7.13V6a3 3 0 1 1 6 0v1.13\"></path>",
        categories = "development,animals",
        tags = "issue,testing,debug,reproduce,code,insect",
        contributors = "danielbayley,jguddas,karsa-mistmere,jamiemlaw"
    ))]
    BugPlay,
    #[cfg(feature = "bug")]
    #[strum(props(
        svg = "<path d=\"M12 20v-9\"></path><path d=\"M14 7a4 4 0 0 1 4 4v3a6 6 0 0 1-12 0v-3a4 4 0 0 1 4-4z\"></path><path d=\"M14.12 3.88 16 2\"></path><path d=\"M21 21a4 4 0 0 0-3.81-4\"></path><path d=\"M21 5a4 4 0 0 1-3.55 3.97\"></path><path d=\"M22 13h-4\"></path><path d=\"M3 21a4 4 0 0 1 3.81-4\"></path><path d=\"M3 5a4 4 0 0 0 3.55 3.97\"></path><path d=\"M6 13H2\"></path><path d=\"m8 2 1.88 1.88\"></path><path d=\"M9 7.13V6a3 3 0 1 1 6 0v1.13\"></path>",
        categories = "development,animals",
        tags = "issue,error,defect,testing,troubleshoot,problem,report,debug,code,insect,beetle",
        contributors = "danielbayley,jamiemlaw"
    ))]
    Bug,
    #[cfg(feature = "building_2")]
    #[strum(props(
        svg = "<path d=\"M10 12h4\"></path><path d=\"M10 8h4\"></path><path d=\"M14 21v-3a2 2 0 0 0-4 0v3\"></path><path d=\"M6 10H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-2\"></path><path d=\"M6 21V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v16\"></path>",
        categories = "account,buildings",
        tags = "business,company,enterprise,skyscraper,organisation,organization,city",
        contributors = "maxim-s-barabash,ericfennis,karsa-mistmere,jguddas"
    ))]
    Building2,
    #[cfg(feature = "building")]
    #[strum(props(
        svg = "<path d=\"M12 10h.01\"></path><path d=\"M12 14h.01\"></path><path d=\"M12 6h.01\"></path><path d=\"M16 10h.01\"></path><path d=\"M16 14h.01\"></path><path d=\"M16 6h.01\"></path><path d=\"M8 10h.01\"></path><path d=\"M8 14h.01\"></path><path d=\"M8 6h.01\"></path><path d=\"M9 22v-3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3\"></path><rect y=\"2\" height=\"20\" width=\"16\" rx=\"2\" x=\"4\"></rect>",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "<path d=\"M4 6 2 7\"></path><path d=\"M10 6h4\"></path><path d=\"m22 7-2-1\"></path><rect rx=\"2\" height=\"16\" x=\"4\" y=\"3\" width=\"16\"></rect><path d=\"M4 11h16\"></path><path d=\"M8 15h.01\"></path><path d=\"M16 15h.01\"></path><path d=\"M6 19v2\"></path><path d=\"M18 21v-2\"></path>",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "<path d=\"M8 6v6\"></path><path d=\"M15 6v6\"></path><path d=\"M2 12h19.6\"></path><path d=\"M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3\"></path><circle cx=\"7\" cy=\"18\" r=\"2\"></circle><path d=\"M9 18h5\"></path><circle cy=\"18\" r=\"2\" cx=\"16\"></circle>",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "<path d=\"M10 3h.01\"></path><path d=\"M14 2h.01\"></path><path d=\"m2 9 20-5\"></path><path d=\"M12 12V6.5\"></path><rect width=\"16\" x=\"4\" y=\"12\" height=\"10\" rx=\"3\"></rect><path d=\"M9 12v5\"></path><path d=\"M15 12v5\"></path><path d=\"M4 17h16\"></path>",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[cfg(feature = "cable")]
    #[strum(props(
        svg = "<path d=\"M17 19a1 1 0 0 1-1-1v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2a1 1 0 0 1-1 1z\"></path><path d=\"M17 21v-2\"></path><path d=\"M19 14V6.5a1 1 0 0 0-7 0v11a1 1 0 0 1-7 0V10\"></path><path d=\"M21 21v-2\"></path><path d=\"M3 5V3\"></path><path d=\"M4 10a2 2 0 0 1-2-2V6a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2z\"></path><path d=\"M7 5V3\"></path>",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,energy,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley,jguddas"
    ))]
    Cable,
    #[cfg(feature = "cake_slice")]
    #[strum(props(
        svg = "<path d=\"M16 13H3\"></path><path d=\"M16 17H3\"></path><path d=\"m7.2 7.9-3.38 2.5A2 2 0 0 0 3 12.01V20a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-8.65c0-2-2.44-6.02-6.44-8.02a1 1 0 0 0-1.08.057L10.4 5.6\"></path><circle cx=\"9\" r=\"2\" cy=\"7\"></circle>",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = "danielbayley,jguddas"
    ))]
    CakeSlice,
    #[cfg(feature = "cake")]
    #[strum(props(
        svg = "<path d=\"M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8\"></path><path d=\"M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1\"></path><path d=\"M2 21h20\"></path><path d=\"M7 8v3\"></path><path d=\"M12 8v3\"></path><path d=\"M17 8v3\"></path><path d=\"M7 4h.01\"></path><path d=\"M12 4h.01\"></path><path d=\"M17 4h.01\"></path>",
        categories = "food-beverage,social,account",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,fondant,icing sugar,sweet,baking",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Cake,
    #[cfg(feature = "calculator")]
    #[strum(props(
        svg = "<rect width=\"16\" x=\"4\" height=\"20\" y=\"2\" rx=\"2\"></rect><line x2=\"16\" y1=\"6\" y2=\"6\" x1=\"8\"></line><line x2=\"16\" y1=\"14\" x1=\"16\" y2=\"18\"></line><path d=\"M16 10h.01\"></path><path d=\"M12 10h.01\"></path><path d=\"M8 10h.01\"></path><path d=\"M12 14h.01\"></path><path d=\"M8 14h.01\"></path><path d=\"M12 18h.01\"></path><path d=\"M8 18h.01\"></path>",
        categories = "math,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[cfg(feature = "calendar_1")]
    #[strum(props(
        svg = "<path d=\"M11 14h1v4\"></path><path d=\"M16 2v4\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path><rect height=\"18\" rx=\"2\" x=\"3\" y=\"4\" width=\"18\"></rect>",
        categories = "time",
        tags = "date,month,year,event,single,singular,once,1,first",
        contributors = "colebemis,ericfennis,peteruithoven"
    ))]
    Calendar1,
    #[cfg(feature = "calendar_arrow_down")]
    #[strum(props(
        svg = "<path d=\"m14 18 4 4 4-4\"></path><path d=\"M16 2v4\"></path><path d=\"M18 14v8\"></path><path d=\"M21 11.35V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.34\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path>",
        categories = "time",
        tags = "date,month,year,event,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    CalendarArrowDown,
    #[cfg(feature = "calendar_arrow_up")]
    #[strum(props(
        svg = "<path d=\"m14 18 4-4 4 4\"></path><path d=\"M16 2v4\"></path><path d=\"M18 22v-8\"></path><path d=\"M21 11.34V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h9\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path>",
        categories = "time",
        tags = "date,month,year,event,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    CalendarArrowUp,
    #[cfg(feature = "calendar_check_2")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><path d=\"M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8\"></path><path d=\"M3 10h18\"></path><path d=\"m16 20 2 2 4-4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[cfg(feature = "calendar_check")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect rx=\"2\" height=\"18\" width=\"18\" x=\"3\" y=\"4\"></rect><path d=\"M3 10h18\"></path><path d=\"m9 16 2 2 4-4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[cfg(feature = "calendar_clock")]
    #[strum(props(
        svg = "<path d=\"M16 14v2.2l1.6 1\"></path><path d=\"M16 2v4\"></path><path d=\"M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5\"></path><path d=\"M3 10h5\"></path><path d=\"M8 2v4\"></path><circle cy=\"16\" cx=\"16\" r=\"6\"></circle>",
        categories = "time",
        tags = "date,day,month,year,event,clock,hour",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    CalendarClock,
    #[cfg(feature = "calendar_cog")]
    #[strum(props(
        svg = "<path d=\"m15.22 16.85-.923-.383\"></path><path d=\"m15.22 19.14-.923.38\"></path><path d=\"M16 2v4\"></path><path d=\"m16.47 14.30.382.92\"></path><path d=\"m16.85 20.77-.383.92\"></path><path d=\"m19.14 15.22.383-.923\"></path><path d=\"m19.53 21.69-.382-.924\"></path><path d=\"m20.77 16.85.924-.383\"></path><path d=\"m20.77 19.14.924.38\"></path><path d=\"M21 10.59V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path><circle cy=\"18\" r=\"3\" cx=\"18\"></circle>",
        categories = "time",
        tags = "date,day,month,year,events,settings,gear,cog",
        contributors = "karsa-mistmere,ericfennis,AlexandrePhilibert,UsamaKhan,jguddas"
    ))]
    CalendarCog,
    #[cfg(feature = "calendar_days")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect y=\"4\" height=\"18\" width=\"18\" rx=\"2\" x=\"3\"></rect><path d=\"M3 10h18\"></path><path d=\"M8 14h.01\"></path><path d=\"M12 14h.01\"></path><path d=\"M16 14h.01\"></path><path d=\"M8 18h.01\"></path><path d=\"M12 18h.01\"></path><path d=\"M16 18h.01\"></path>",
        categories = "time",
        tags = "date,month,year,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[cfg(feature = "calendar_fold")]
    #[strum(props(
        svg = "<path d=\"M3 20a2 2 0 0 0 2 2h10a2.4 2.4 0 0 0 1.70-.706l3.58-3.58A2.4 2.4 0 0 0 21 16V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2z\"></path><path d=\"M15 22v-5a1 1 0 0 1 1-1h5\"></path><path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><path d=\"M3 10h18\"></path>",
        categories = "time,files",
        tags = "date,month,year,event,birthday,birthdate,ics",
        contributors = "danielbayley"
    ))]
    CalendarFold,
    #[cfg(feature = "calendar_heart")]
    #[strum(props(
        svg = "<path d=\"M12.12 22H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v5.12\"></path><path d=\"M14.62 18.8A2.25 2.25 0 1 1 18 15.83a2.25 2.25 0 1 1 3.38 2.96l-2.62 2.85a.998.99 0 0 1-1.50 0z\"></path><path d=\"M16 2v4\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path>",
        categories = "time",
        tags = "date,month,year,event,heart,favourite,subscribe,valentines day",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[cfg(feature = "calendar_minus_2")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect y=\"4\" rx=\"2\" x=\"3\" width=\"18\" height=\"18\"></rect><path d=\"M3 10h18\"></path><path d=\"M10 16h4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "danielbayley"
    ))]
    CalendarMinus2,
    #[cfg(feature = "calendar_minus")]
    #[strum(props(
        svg = "<path d=\"M16 19h6\"></path><path d=\"M16 2v4\"></path><path d=\"M21 15V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8.5\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarMinus,
    #[cfg(feature = "calendar_off")]
    #[strum(props(
        svg = "<path d=\"M4.2 4.2A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18\"></path><path d=\"M21 15.5V6a2 2 0 0 0-2-2H9.5\"></path><path d=\"M16 2v4\"></path><path d=\"M3 10h7\"></path><path d=\"M21 10h-5.5\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[cfg(feature = "calendar_plus_2")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect width=\"18\" y=\"4\" x=\"3\" height=\"18\" rx=\"2\"></rect><path d=\"M3 10h18\"></path><path d=\"M10 16h4\"></path><path d=\"M12 14v4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "danielbayley"
    ))]
    CalendarPlus2,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "<path d=\"M16 19h6\"></path><path d=\"M16 2v4\"></path><path d=\"M19 16v6\"></path><path d=\"M21 12.59V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8.5\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "<rect height=\"18\" y=\"4\" rx=\"2\" width=\"18\" x=\"3\"></rect><path d=\"M16 2v4\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path><path d=\"M17 14h-6\"></path><path d=\"M13 18H7\"></path><path d=\"M7 14h.01\"></path><path d=\"M17 18h.01\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[cfg(feature = "calendar_search")]
    #[strum(props(
        svg = "<path d=\"M16 2v4\"></path><path d=\"M21 11.75V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.25\"></path><path d=\"m22 22-1.87-1.87\"></path><path d=\"M3 10h18\"></path><path d=\"M8 2v4\"></path><circle cx=\"18\" cy=\"18\" r=\"3\"></circle>",
        categories = "time",
        tags = "date,day,month,year,events,search,lens",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CalendarSearch,
    #[cfg(feature = "calendar_sync")]
    #[strum(props(
        svg = "<path d=\"M11 10v4h4\"></path><path d=\"m11 14 1.53-1.60a5 5 0 0 1 8 1.5\"></path><path d=\"M16 2v4\"></path><path d=\"m21 18-1.53 1.60a5 5 0 0 1-8-1.5\"></path><path d=\"M21 22v-4h-4\"></path><path d=\"M21 8.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h4.3\"></path><path d=\"M3 10h4\"></path><path d=\"M8 2v4\"></path>",
        categories = "arrows,time",
        tags = "repeat,refresh,reconnect,transfer,backup,date,month,year,event,subscribe,recurring,schedule,reminder,automatic,auto",
        contributors = "danielbayley,jguddas,karsa-mistmere,chessurisme"
    ))]
    CalendarSync,
    #[cfg(feature = "calendar_x_2")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><path d=\"M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8\"></path><path d=\"M3 10h18\"></path><path d=\"m17 22 5-5\"></path><path d=\"m17 17 5 5\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[cfg(feature = "calendar_x")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect width=\"18\" y=\"4\" x=\"3\" height=\"18\" rx=\"2\"></rect><path d=\"M3 10h18\"></path><path d=\"m14 14-4 4\"></path><path d=\"m10 14 4 4\"></path>",
        categories = "time",
        tags = "date,day,month,year,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M16 2v4\"></path><rect y=\"4\" x=\"3\" width=\"18\" rx=\"2\" height=\"18\"></rect><path d=\"M3 10h18\"></path>",
        categories = "time",
        tags = "date,month,year,event,birthday,birthdate",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "calendars")]
    #[strum(props(
        svg = "<path d=\"M12 2v2\"></path><path d=\"M15.72 21.01A2 2 0 0 1 14 22H4a2 2 0 0 1-2-2V10a2 2 0 0 1 2-2\"></path><path d=\"M18 2v2\"></path><path d=\"M2 13h2\"></path><path d=\"M8 8h14\"></path><rect rx=\"2\" width=\"14\" height=\"14\" x=\"8\" y=\"3\"></rect>",
        categories = "time",
        tags = "date,month,year,event,dates,months,years,events",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere"
    ))]
    Calendars,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "<path d=\"M14.56 14.55a3 3 0 1 1-4.12-4.12\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20 20H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h1.99a2 2 0 0 0 .819-.175\"></path><path d=\"M9.69 4.02A2 2 0 0 1 10.00 4h3.99a2 2 0 0 1 1.76 1.05l.486.9A2 2 0 0 0 18.00 7H20a2 2 0 0 1 2 2v7.34\"></path>",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "<path d=\"M13.99 4a2 2 0 0 1 1.76 1.05l.486.9A2 2 0 0 0 18.00 7H20a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h1.99a2 2 0 0 0 1.75-1.04l.489-.904A2 2 0 0 1 10.00 4z\"></path><circle cx=\"12\" cy=\"13\" r=\"3\"></circle>",
        categories = "photography,devices,communication",
        tags = "photography,lens,focus,capture,shot,visual,image,device,equipment,photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis,karsa-mistmere"
    ))]
    Camera,
    #[cfg(feature = "candy_cane")]
    #[strum(props(
        svg = "<path d=\"M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.46-2 2 2 0 1 0-3.46-2Z\"></path><path d=\"M17.75 7 15 2.1\"></path><path d=\"M10.9 4.8 13 9\"></path><path d=\"m7.9 9.7 2 4.4\"></path><path d=\"M4.9 14.7 7 18.9\"></path>",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CandyCane,
    #[cfg(feature = "candy_off")]
    #[strum(props(
        svg = "<path d=\"M10 10v7.9\"></path><path d=\"M11.80 6.14a5 5 0 0 1 6.05 6.05\"></path><path d=\"M14 6.1v2.24\"></path><path d=\"m15.5 15.57-.964.96a5 5 0 0 1-7.07 0 5 5 0 0 1 0-7.07l.964-.965\"></path><path d=\"M16 7V3a1 1 0 0 1 1.70-.707 2.5 2.5 0 0 0 2.15.717 1 1 0 0 1 1.13 1.13 2.5 2.5 0 0 0 .717 2.15A1 1 0 0 1 21 8h-4\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8 17v4a1 1 0 0 1-1.70.707 2.5 2.5 0 0 0-2.15-.717 1 1 0 0 1-1.13-1.13 2.5 2.5 0 0 0-.717-2.15A1 1 0 0 1 3 16h4\"></path>",
        categories = "food-beverage",
        tags = "sugar free,food,sweet,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CandyOff,
    #[cfg(feature = "candy")]
    #[strum(props(
        svg = "<path d=\"M10 7v10.9\"></path><path d=\"M14 6.1V17\"></path><path d=\"M16 7V3a1 1 0 0 1 1.70-.707 2.5 2.5 0 0 0 2.15.717 1 1 0 0 1 1.13 1.13 2.5 2.5 0 0 0 .717 2.15A1 1 0 0 1 21 8h-4\"></path><path d=\"M16.53 7.46a5 5 0 0 0-7.07 0l-2 2a5 5 0 0 0 0 7.07 5 5 0 0 0 7.07 0l2-2a5 5 0 0 0 0-7.07\"></path><path d=\"M8 17v4a1 1 0 0 1-1.70.707 2.5 2.5 0 0 0-2.15-.717 1 1 0 0 1-1.13-1.13 2.5 2.5 0 0 0-.717-2.15A1 1 0 0 1 3 16h4\"></path>",
        categories = "food-beverage",
        tags = "sugar,food,sweet",
        contributors = "karsa-mistmere"
    ))]
    Candy,
    #[cfg(feature = "cannabis_off")]
    #[strum(props(
        svg = "<path d=\"M12 22v-4c1.5 1.5 3.5 3 6 3 0-1.5-.5-3.5-2-5\"></path><path d=\"M13.98 8.32C13.90 6.05 13.36 3.82 12 2a9.3 9.3 0 0 0-1.44 2.9\"></path><path d=\"M17.37 11.72C18.88 10.53 21 7.84 21 6c-2.32 0-5.08 1.29-6.66 2.68\"></path><path d=\"m2 2 20 20\"></path><path d=\"M21.02 15.37A15 15 0 0 0 22 15c-.426-1.27-2.67-2.55-4.25-2.90\"></path><path d=\"M6.99 6.99C5.71 6.4 4.29 6 3 6c0 2 2.5 5 4 6-1.5 0-4.5 1.5-5 3 3.5 1.5 6 1 6 1-1.5 1.5-2 3.5-2 5 2.5 0 4.5-1.5 6-3\"></path>",
        categories = "nature",
        tags = "cannabis,weed,leaf",
        contributors = "nickveles"
    ))]
    CannabisOff,
    #[cfg(feature = "cannabis")]
    #[strum(props(
        svg = "<path d=\"M12 22v-4\"></path><path d=\"M7 12c-1.5 0-4.5 1.5-5 3 3.5 1.5 6 1 6 1-1.5 1.5-2 3.5-2 5 2.5 0 4.5-1.5 6-3 1.5 1.5 3.5 3 6 3 0-1.5-.5-3.5-2-5 0 0 2.5.5 6-1-.5-1.5-3.5-3-5-3 1.5-1 4-4 4-6-2.5 0-5.5 1.5-7 3 0-2.5-.5-5-2-7-1.5 2-2 4.5-2 7-1.5-1.5-4.5-3-7-3 0 2 2.5 5 4 6\"></path>",
        categories = "nature",
        tags = "cannabis,weed,leaf",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Cannabis,
    #[cfg(feature = "captions_off")]
    #[strum(props(
        svg = "<path d=\"M10.5 5H19a2 2 0 0 1 2 2v8.5\"></path><path d=\"M17 11h-.5\"></path><path d=\"M19 19H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2\"></path><path d=\"m2 2 20 20\"></path><path d=\"M7 11h4\"></path><path d=\"M7 15h2.5\"></path>",
        categories = "multimedia",
        tags = "closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "DefaultLP"
    ))]
    CaptionsOff,
    #[cfg(feature = "captions")]
    #[strum(props(
        svg = "<rect ry=\"2\" x=\"3\" width=\"18\" height=\"14\" y=\"5\" rx=\"2\"></rect><path d=\"M7 15h4M15 15h2M7 11h2M13 11h4\"></path>",
        categories = "multimedia",
        tags = "closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "DefaultLP"
    ))]
    Captions,
    #[cfg(feature = "car_front")]
    #[strum(props(
        svg = "<path d=\"m21 8-2 2-1.5-3.7A2 2 0 0 0 15.64 5H8.4a2 2 0 0 0-1.90 1.25L5 10 3 8\"></path><path d=\"M7 14h.01\"></path><path d=\"M17 14h.01\"></path><rect height=\"8\" x=\"3\" rx=\"2\" width=\"18\" y=\"10\"></rect><path d=\"M5 18v2\"></path><path d=\"M19 18v2\"></path>",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "<path d=\"M10 2h4\"></path><path d=\"m21 8-2 2-1.5-3.7A2 2 0 0 0 15.64 5H8.4a2 2 0 0 0-1.90 1.25L5 10 3 8\"></path><path d=\"M7 14h.01\"></path><path d=\"M17 14h.01\"></path><rect width=\"18\" height=\"8\" x=\"3\" y=\"10\" rx=\"2\"></rect><path d=\"M5 18v2\"></path><path d=\"M19 18v2\"></path>",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "<path d=\"M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2\"></path><circle cy=\"17\" r=\"2\" cx=\"7\"></circle><path d=\"M9 17h6\"></path><circle cx=\"17\" r=\"2\" cy=\"17\"></circle>",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[cfg(feature = "caravan")]
    #[strum(props(
        svg = "<path d=\"M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2\"></path><path d=\"M2 9h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H2\"></path><path d=\"M22 17v1a1 1 0 0 1-1 1H10v-9a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v9\"></path><circle cx=\"8\" r=\"2\" cy=\"19\"></circle>",
        categories = "transportation,travel,nature",
        tags = "trailer,tow,camping,campsite,mobile home,holiday,nomadic,wilderness,outdoors",
        contributors = "danielbayley,jguddas"
    ))]
    Caravan,
    #[cfg(feature = "card_sim")]
    #[strum(props(
        svg = "<path d=\"M12 14v4\"></path><path d=\"M14.17 2a2 2 0 0 1 1.41.586l3.82 3.82A2 2 0 0 1 20 7.82V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z\"></path><path d=\"M8 14h8\"></path><rect height=\"8\" x=\"8\" y=\"10\" width=\"8\" rx=\"1\"></rect>",
        categories = "connectivity,communication,multimedia,devices",
        tags = "cellphone,smartphone,mobile,network,cellular,service,provider,signal,coverage,disk,data,format,storage,flash,digital,contacts,phone book,contractual,circuit board,chip",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    CardSim,
    #[cfg(feature = "carrot")]
    #[strum(props(
        svg = "<path d=\"M2.27 21.7s9.87-3.5 12.73-6.36a4.5 4.5 0 0 0-6.36-6.37C5.77 11.84 2.27 21.7 2.27 21.7zM8.64 14l-2.05-2.04M15.34 15l-2.46-2.46\"></path><path d=\"M22 9s-1.33-2-3.5-2C16.86 7 15 9 15 9s1.33 2 3.5 2S22 9 22 9z\"></path><path d=\"M15 2s-2 1.33-2 3.5S15 9 15 9s2-1.84 2-3.5C17 3.33 15 2 15 2z\"></path>",
        categories = "food-beverage",
        tags = "vegetable,food,eat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Carrot,
    #[cfg(feature = "case_lower")]
    #[strum(props(
        svg = "<path d=\"M10 9v7\"></path><path d=\"M14 6v10\"></path><circle cx=\"17.5\" cy=\"12.5\" r=\"3.5\"></circle><circle cx=\"6.5\" r=\"3.5\" cy=\"12.5\"></circle>",
        categories = "text,development",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,vichotech,karsa-mistmere"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "<path d=\"m2 16 4.03-9.69a.5.5 0 0 1 .923 0L11 16\"></path><path d=\"M22 9v7\"></path><path d=\"M3.30 13h6.39\"></path><circle cy=\"12.5\" cx=\"18.5\" r=\"3.5\"></circle>",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,vichotech,karsa-mistmere"
    ))]
    CaseSensitive,
    #[cfg(feature = "case_upper")]
    #[strum(props(
        svg = "<path d=\"M15 11h4.5a1 1 0 0 1 0 5h-4a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h3a1 1 0 0 1 0 5\"></path><path d=\"m2 16 4.03-9.69a.5.5 0 0 1 .923 0L11 16\"></path><path d=\"M3.30 13h6.39\"></path>",
        categories = "text,development",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,jguddas,vichotech,karsa-mistmere"
    ))]
    CaseUpper,
    #[cfg(feature = "cassette_tape")]
    #[strum(props(
        svg = "<rect y=\"4\" height=\"16\" width=\"20\" x=\"2\" rx=\"2\"></rect><circle cy=\"10\" cx=\"8\" r=\"2\"></circle><path d=\"M8 12h8\"></path><circle cx=\"16\" r=\"2\" cy=\"10\"></circle><path d=\"m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3\"></path>",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = "danielbayley"
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "<path d=\"M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6\"></path><path d=\"M2 12a9 9 0 0 1 8 8\"></path><path d=\"M2 16a5 5 0 0 1 4 4\"></path><line x2=\"2.01\" y2=\"20\" x1=\"2\" y1=\"20\"></line>",
        categories = "devices,connectivity",
        tags = "chromecast,airplay,screen",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Cast,
    #[cfg(feature = "castle")]
    #[strum(props(
        svg = "<path d=\"M10 5V3\"></path><path d=\"M14 5V3\"></path><path d=\"M15 21v-3a3 3 0 0 0-6 0v3\"></path><path d=\"M18 3v8\"></path><path d=\"M18 5H6\"></path><path d=\"M22 11H2\"></path><path d=\"M22 9v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9\"></path><path d=\"M6 3v8\"></path>",
        categories = "buildings,gaming,navigation",
        tags = "fortress,stronghold,palace,chateau,building",
        contributors = "karsa-mistmere"
    ))]
    Castle,
    #[cfg(feature = "cat")]
    #[strum(props(
        svg = "<path d=\"M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z\"></path><path d=\"M8 14v.5\"></path><path d=\"M16 14v.5\"></path><path d=\"M11.25 16.25h1.5L12 17l-.75-.75Z\"></path>",
        categories = "animals",
        tags = "animal,pet,kitten,feline",
        contributors = "kemie"
    ))]
    Cat,
    #[cfg(feature = "cctv_off")]
    #[strum(props(
        svg = "<path d=\"m12.30 6.65 4.79 2.40a1 1 0 0 1 .447 1.34l-.501 1.00.605.60h2.72a1 1 0 0 1 .894 1.44l-.724 1.44\"></path><path d=\"m15.16 15.16-.719 1.43a1 1 0 0 1-1.34.447L3.61 12.3a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.9 2.9 0 0 1 .873-1.03\"></path><path d=\"M2 19h3.76a2 2 0 0 0 1.8-1.1l1.44-2.90\"></path><path d=\"m2 2 20 20\"></path><path d=\"M2 21v-4\"></path><path d=\"M7 9h.01\"></path>",
        categories = "security,devices,communication,connectivity,photography",
        tags = "camera,surveillance,recording,film,videotape,crime,watching",
        contributors = "danielbayley,karsa-mistmere,rrod497"
    ))]
    CctvOff,
    #[cfg(feature = "cctv")]
    #[strum(props(
        svg = "<path d=\"M16.75 12h3.63a1 1 0 0 1 .894 1.44l-2.03 4.06a1 1 0 0 1-1.70.134l-2.12-2.97\"></path><path d=\"M17.10 9.05a1 1 0 0 1 .447 1.34l-3.10 6.21a1 1 0 0 1-1.34.447L3.61 12.3a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.92 2.92 0 0 1 3.92-1.3z\"></path><path d=\"M2 19h3.76a2 2 0 0 0 1.8-1.1L9 15\"></path><path d=\"M2 21v-4\"></path><path d=\"M7 9h.01\"></path>",
        categories = "security,devices,communication,connectivity,photography",
        tags = "camera,surveillance,recording,film,videotape,crime,watching",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cctv,
    #[cfg(feature = "chart_area")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M7 11.20a.5.5 0 0 1 .146-.353l2-2a.5.5 0 0 1 .708 0l3.29 3.29a.5.5 0 0 0 .708 0l4.29-4.29a.5.5 0 0 1 .854.35V16a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1z\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,area",
        contributors = "nstokoe"
    ))]
    ChartArea,
    #[cfg(feature = "chart_bar_big")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><rect y=\"13\" x=\"7\" rx=\"1\" width=\"9\" height=\"4\"></rect><rect width=\"12\" rx=\"1\" x=\"7\" y=\"5\" height=\"4\"></rect>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartBarBig,
    #[cfg(feature = "chart_bar_decreasing")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M7 11h8\"></path><path d=\"M7 16h3\"></path><path d=\"M7 6h12\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartBarDecreasing,
    #[cfg(feature = "chart_bar_increasing")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M7 11h8\"></path><path d=\"M7 16h12\"></path><path d=\"M7 6h3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere"
    ))]
    ChartBarIncreasing,
    #[cfg(feature = "chart_bar_stacked")]
    #[strum(props(
        svg = "<path d=\"M11 13v4\"></path><path d=\"M15 5v4\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><rect x=\"7\" rx=\"1\" height=\"4\" y=\"13\" width=\"9\"></rect><rect y=\"5\" x=\"7\" width=\"12\" rx=\"1\" height=\"4\"></rect>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,multivariate,categorical,comparison",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartBarStacked,
    #[cfg(feature = "chart_bar")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M7 16h8\"></path><path d=\"M7 11h12\"></path><path d=\"M7 6h3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartBar,
    #[cfg(feature = "chart_candlestick")]
    #[strum(props(
        svg = "<path d=\"M9 5v4\"></path><rect width=\"4\" rx=\"1\" x=\"7\" y=\"9\" height=\"6\"></rect><path d=\"M9 15v2\"></path><path d=\"M17 3v2\"></path><rect rx=\"1\" height=\"8\" x=\"15\" y=\"5\" width=\"4\"></rect><path d=\"M17 13v3\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path>",
        categories = "charts,finance",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,stocks,exchange,hedge fund,statistics,analytics,diagram,graph",
        contributors = "danielbayley"
    ))]
    ChartCandlestick,
    #[cfg(feature = "chart_column_big")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><rect height=\"12\" x=\"15\" rx=\"1\" width=\"4\" y=\"5\"></rect><rect rx=\"1\" width=\"4\" y=\"8\" x=\"7\" height=\"9\"></rect>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartColumnBig,
    #[cfg(feature = "chart_column_decreasing")]
    #[strum(props(
        svg = "<path d=\"M13 17V9\"></path><path d=\"M18 17v-3\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M8 17V5\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartColumnDecreasing,
    #[cfg(feature = "chart_column_increasing")]
    #[strum(props(
        svg = "<path d=\"M13 17V9\"></path><path d=\"M18 17V5\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M8 17v-3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumnIncreasing,
    #[cfg(feature = "chart_column_stacked")]
    #[strum(props(
        svg = "<path d=\"M11 13H7\"></path><path d=\"M19 9h-4\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><rect y=\"5\" height=\"12\" x=\"15\" width=\"4\" rx=\"1\"></rect><rect rx=\"1\" x=\"7\" y=\"8\" width=\"4\" height=\"9\"></rect>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,multivariate,categorical,comparison",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumnStacked,
    #[cfg(feature = "chart_column")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M18 17V9\"></path><path d=\"M13 17V5\"></path><path d=\"M8 17v-3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ChartColumn,
    #[cfg(feature = "chart_gantt")]
    #[strum(props(
        svg = "<path d=\"M10 6h8\"></path><path d=\"M12 16h6\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M8 11h7\"></path>",
        categories = "charts",
        tags = "diagram,graph,timeline,planning",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    ChartGantt,
    #[cfg(feature = "chart_line")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"m19 9-5 5-4-4-3 3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "karsa-mistmere"
    ))]
    ChartLine,
    #[cfg(feature = "chart_network")]
    #[strum(props(
        svg = "<path d=\"m13.11 7.66 1.78 2.67\"></path><path d=\"m14.16 12.78-3.32 1.42\"></path><path d=\"m20 4-6.06 1.51\"></path><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><circle cy=\"6\" cx=\"12\" r=\"2\"></circle><circle r=\"2\" cx=\"16\" cy=\"12\"></circle><circle cx=\"9\" r=\"2\" cy=\"15\"></circle>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,topology,cluster,web,nodes,connections,edges",
        contributors = "karsa-mistmere"
    ))]
    ChartNetwork,
    #[cfg(feature = "chart_no_axes_column_decreasing")]
    #[strum(props(
        svg = "<path d=\"M5 21V3\"></path><path d=\"M12 21V9\"></path><path d=\"M19 21v-6\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending down",
        contributors = "karsa-mistmere"
    ))]
    ChartNoAxesColumnDecreasing,
    #[cfg(feature = "chart_no_axes_column_increasing")]
    #[strum(props(
        svg = "<path d=\"M5 21v-6\"></path><path d=\"M12 21V9\"></path><path d=\"M19 21V3\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    ChartNoAxesColumnIncreasing,
    #[cfg(feature = "chart_no_axes_column")]
    #[strum(props(
        svg = "<path d=\"M5 21v-6\"></path><path d=\"M12 21V3\"></path><path d=\"M19 21V9\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    ChartNoAxesColumn,
    #[cfg(feature = "chart_no_axes_combined")]
    #[strum(props(
        svg = "<path d=\"M12 16v5\"></path><path d=\"M16 14v7\"></path><path d=\"M20 10v11\"></path><path d=\"m22 3-8.64 8.64a.5.5 0 0 1-.708 0L9.35 8.35a.5.5 0 0 0-.707 0L2 15\"></path><path d=\"M4 18v3\"></path><path d=\"M8 14v7\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,trending up",
        contributors = "karsa-mistmere"
    ))]
    ChartNoAxesCombined,
    #[cfg(feature = "chart_no_axes_gantt")]
    #[strum(props(
        svg = "<path d=\"M6 5h12\"></path><path d=\"M4 12h10\"></path><path d=\"M12 19h8\"></path>",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ChartNoAxesGantt,
    #[cfg(feature = "chart_pie")]
    #[strum(props(
        svg = "<path d=\"M21 12c.552 0 1.00-.449.95-.998a10 10 0 0 0-8.95-8.95c-.55-.055-.998.39-.998.95v8a1 1 0 0 0 1 1z\"></path><path d=\"M21.21 15.89A10 10 0 1 1 8 2.83\"></path>",
        categories = "charts,files",
        tags = "statistics,analytics,diagram,presentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ChartPie,
    #[cfg(feature = "chart_scatter")]
    #[strum(props(
        svg = "<circle r=\".5\" cy=\"7.5\" fill=\"currentColor\" cx=\"7.5\"></circle><circle cy=\"5.5\" cx=\"18.5\" r=\".5\" fill=\"currentColor\"></circle><circle fill=\"currentColor\" r=\".5\" cx=\"11.5\" cy=\"11.5\"></circle><circle cy=\"16.5\" fill=\"currentColor\" r=\".5\" cx=\"7.5\"></circle><circle cx=\"17.5\" r=\".5\" cy=\"14.5\" fill=\"currentColor\"></circle><path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ChartScatter,
    #[cfg(feature = "chart_spline")]
    #[strum(props(
        svg = "<path d=\"M3 3v16a2 2 0 0 0 2 2h16\"></path><path d=\"M7 16c.5-2 1.5-7 4-7 2 0 2 3 4 3 2.5 0 4.5-5 5-7\"></path>",
        categories = "charts",
        tags = "statistics,analytics,diagram,graph,curve,continuous,smooth,polynomial,quadratic,function,interpolation",
        contributors = "karsa-mistmere"
    ))]
    ChartSpline,
    #[cfg(feature = "check_check")]
    #[strum(props(
        svg = "<path d=\"M18 6 7 17l-5-5\"></path><path d=\"m22 10-7.5 7.5L13 16\"></path>",
        categories = "notifications",
        tags = "done,received,double,todo,tick,complete,task",
        contributors = "ericfennis"
    ))]
    CheckCheck,
    #[cfg(feature = "check_line")]
    #[strum(props(
        svg = "<path d=\"M20 4L9 15\"></path><path d=\"M21 19L3 19\"></path><path d=\"M9 15L4 10\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,oosawy"
    ))]
    CheckLine,
    #[cfg(feature = "check")]
    #[strum(props(
        svg = "<path d=\"M20 6 9 17l-5-5\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis"
    ))]
    Check,
    #[cfg(feature = "chef_hat")]
    #[strum(props(
        svg = "<path d=\"M17 21a1 1 0 0 0 1-1v-5.35c0-.457.31-.844.72-1.04a4 4 0 0 0-2.13-7.58 5 5 0 0 0-9.18 0 4 4 0 0 0-2.13 7.58c.411.19.727.58.727 1.04V20a1 1 0 0 0 1 1Z\"></path><path d=\"M6 17h12\"></path>",
        categories = "food-beverage",
        tags = "cooking,food,kitchen,restaurant",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    ChefHat,
    #[cfg(feature = "cherry")]
    #[strum(props(
        svg = "<path d=\"M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z\"></path><path d=\"M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z\"></path><path d=\"M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12\"></path><path d=\"M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z\"></path>",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Cherry,
    #[cfg(feature = "chess_bishop")]
    #[strum(props(
        svg = "<path d=\"M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z\"></path><path d=\"M15 18c1.5-.615 3-2.46 3-4.92C18 8.76 14.5 4.46 12 2 9.5 4.46 6 8.77 6 13.07 6 15.53 7.5 17.38 9 18\"></path><path d=\"m16 7-2.5 2.5\"></path><path d=\"M9 2h6\"></path>",
        categories = "gaming,emoji",
        tags = "mitre,miter,piece,board game,religion",
        contributors = "karsa-mistmere"
    ))]
    ChessBishop,
    #[cfg(feature = "chess_king")]
    #[strum(props(
        svg = "<path d=\"M4 20a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1z\"></path><path d=\"m6.7 18-1-1C4.35 15.68 3 14.09 3 12a5 5 0 0 1 4.95-5c1.58 0 2.7.45 4.05 1.81C13.35 7.45 14.46 7 16.05 7A5 5 0 0 1 21 12c0 2.08-1.35 3.67-2.7 5l-1 1\"></path><path d=\"M10 4h4\"></path><path d=\"M12 2v6.81\"></path>",
        categories = "gaming,emoji",
        tags = "ruler,crown,piece,board game,stalemate",
        contributors = "karsa-mistmere"
    ))]
    ChessKing,
    #[cfg(feature = "chess_knight")]
    #[strum(props(
        svg = "<path d=\"M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z\"></path><path d=\"M16.5 18c1-2 2.5-5 2.5-9a7 7 0 0 0-7-7H6.63a1 1 0 0 0-.768 1.64L7 5l-2.32 5.80a2 2 0 0 0 .95 2.52l2.87 1.45\"></path><path d=\"m15 5 1.42-1.42\"></path><path d=\"m17 8 1.53-1.53\"></path><path d=\"M9.71 12.18 7 18\"></path>",
        categories = "gaming,emoji",
        tags = "piece,horse,board game",
        contributors = "karsa-mistmere"
    ))]
    ChessKnight,
    #[cfg(feature = "chess_pawn")]
    #[strum(props(
        svg = "<path d=\"M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z\"></path><path d=\"m14.5 10 1.5 8\"></path><path d=\"M7 10h10\"></path><path d=\"m8 18 1.5-8\"></path><circle r=\"4\" cx=\"12\" cy=\"6\"></circle>",
        categories = "gaming,emoji",
        tags = "piece,board game",
        contributors = "karsa-mistmere"
    ))]
    ChessPawn,
    #[cfg(feature = "chess_queen")]
    #[strum(props(
        svg = "<path d=\"M4 20a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1z\"></path><path d=\"m12.47 5.94 1.56 5.34a1 1 0 0 0 1.75.32l2.61-3.40\"></path><path d=\"m20 9-3 9\"></path><path d=\"m5.59 8.20 2.61 3.40a1 1 0 0 0 1.75-.329l1.56-5.34\"></path><path d=\"M7 18 4 9\"></path><circle r=\"2\" cy=\"4\" cx=\"12\"></circle><circle cy=\"7\" cx=\"20\" r=\"2\"></circle><circle cy=\"7\" r=\"2\" cx=\"4\"></circle>",
        categories = "gaming,emoji",
        tags = "ruler,crown,piece,board game,stalemate",
        contributors = "karsa-mistmere"
    ))]
    ChessQueen,
    #[cfg(feature = "chess_rook")]
    #[strum(props(
        svg = "<path d=\"M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z\"></path><path d=\"M10 2v2\"></path><path d=\"M14 2v2\"></path><path d=\"m17 18-1-9\"></path><path d=\"M6 2v5a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2\"></path><path d=\"M6 4h12\"></path><path d=\"m7 18 1-9\"></path>",
        categories = "gaming,emoji",
        tags = "castle,piece,board game",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ChessRook,
    #[cfg(feature = "chevron_down")]
    #[strum(props(
        svg = "<path d=\"m6 9 6 6 6-6\"></path>",
        categories = "arrows,gaming",
        tags = "backwards,reverse,slow,dropdown",
        contributors = "colebemis"
    ))]
    ChevronDown,
    #[cfg(feature = "chevron_first")]
    #[strum(props(
        svg = "<path d=\"m17 18-6-6 6-6\"></path><path d=\"M7 6v12\"></path>",
        categories = "arrows,multimedia",
        tags = "previous,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronFirst,
    #[cfg(feature = "chevron_last")]
    #[strum(props(
        svg = "<path d=\"m7 18 6-6-6-6\"></path><path d=\"M17 6v12\"></path>",
        categories = "arrows,multimedia",
        tags = "skip,next,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronLast,
    #[cfg(feature = "chevron_left")]
    #[strum(props(
        svg = "<path d=\"m15 18-6-6 6-6\"></path>",
        categories = "arrows",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "colebemis"
    ))]
    ChevronLeft,
    #[cfg(feature = "chevron_right")]
    #[strum(props(
        svg = "<path d=\"m9 18 6-6-6-6\"></path>",
        categories = "arrows,math,development",
        tags = "forward,next,more than,greater,menu,code,coding,command line,terminal,prompt,shell,>",
        contributors = "colebemis"
    ))]
    ChevronRight,
    #[cfg(feature = "chevron_up")]
    #[strum(props(
        svg = "<path d=\"m18 15-6-6-6 6\"></path>",
        categories = "arrows,math,gaming",
        tags = "caret,keyboard,mac,control,ctrl,superscript,exponential,power,ahead,fast,^,dropdown",
        contributors = "colebemis"
    ))]
    ChevronUp,
    #[cfg(feature = "chevrons_down_up")]
    #[strum(props(
        svg = "<path d=\"m7 20 5-5 5 5\"></path><path d=\"m7 4 5 5 5-5\"></path>",
        categories = "arrows",
        tags = "collapse,fold,vertical",
        contributors = "PeterlitsZo,mittalyashu,ericfennis"
    ))]
    ChevronsDownUp,
    #[cfg(feature = "chevrons_down")]
    #[strum(props(
        svg = "<path d=\"m7 6 5 5 5-5\"></path><path d=\"m7 13 5 5 5-5\"></path>",
        categories = "arrows,gaming",
        tags = "backwards,reverse,slower",
        contributors = "colebemis"
    ))]
    ChevronsDown,
    #[cfg(feature = "chevrons_left_right_ellipsis")]
    #[strum(props(
        svg = "<path d=\"M12 12h.01\"></path><path d=\"M16 12h.01\"></path><path d=\"m17 7 5 5-5 5\"></path><path d=\"m7 7-5 5 5 5\"></path><path d=\"M8 12h.01\"></path>",
        categories = "communication,devices,multimedia,gaming",
        tags = "internet,network,connection,cable,lan,port,router,switch,hub,modem,web,online,networking,communication,socket,plug,slot,controller,connector,interface,console,signal,data,input,output",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ChevronsLeftRightEllipsis,
    #[cfg(feature = "chevrons_left_right")]
    #[strum(props(
        svg = "<path d=\"m9 7-5 5 5 5\"></path><path d=\"m15 7 5 5-5 5\"></path>",
        categories = "arrows",
        tags = "expand,horizontal,unfold",
        contributors = "karsa-mistmere"
    ))]
    ChevronsLeftRight,
    #[cfg(feature = "chevrons_left")]
    #[strum(props(
        svg = "<path d=\"m11 17-5-5 5-5\"></path><path d=\"m18 17-5-5 5-5\"></path>",
        categories = "arrows,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsLeft,
    #[cfg(feature = "chevrons_right_left")]
    #[strum(props(
        svg = "<path d=\"m20 17-5-5 5-5\"></path><path d=\"m4 17 5-5-5-5\"></path>",
        categories = "arrows",
        tags = "collapse,fold,horizontal",
        contributors = "karsa-mistmere"
    ))]
    ChevronsRightLeft,
    #[cfg(feature = "chevrons_right")]
    #[strum(props(
        svg = "<path d=\"m6 17 5-5-5-5\"></path><path d=\"m13 17 5-5-5-5\"></path>",
        categories = "arrows,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsRight,
    #[cfg(feature = "chevrons_up_down")]
    #[strum(props(
        svg = "<path d=\"m7 15 5 5 5-5\"></path><path d=\"m7 9 5-5 5 5\"></path>",
        categories = "arrows",
        tags = "expand,unfold,vertical",
        contributors = "mittalyashu,ericfennis"
    ))]
    ChevronsUpDown,
    #[cfg(feature = "chevrons_up")]
    #[strum(props(
        svg = "<path d=\"m17 11-5-5-5 5\"></path><path d=\"m17 18-5-5-5 5\"></path>",
        categories = "arrows,gaming",
        tags = "forward,ahead,faster,speed,boost",
        contributors = "colebemis"
    ))]
    ChevronsUp,
    #[cfg(feature = "church")]
    #[strum(props(
        svg = "<path d=\"M10 9h4\"></path><path d=\"M12 7v5\"></path><path d=\"M14 21v-3a2 2 0 0 0-4 0v3\"></path><path d=\"m18 9 3.52 2.14a1 1 0 0 1 .48.85V19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-6.99a1 1 0 0 1 .48-.854L6 9\"></path><path d=\"M6 21V7a1 1 0 0 1 .376-.782l5-3.99a1 1 0 0 1 1.24.001l5 4A1 1 0 0 1 18 7v14\"></path>",
        categories = "buildings,navigation",
        tags = "temple,building",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Church,
    #[cfg(feature = "cigarette_off")]
    #[strum(props(
        svg = "<path d=\"M12 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h13\"></path><path d=\"M18 8c0-2.5-2-2.5-2-5\"></path><path d=\"m2 2 20 20\"></path><path d=\"M21 12a1 1 0 0 1 1 1v2a1 1 0 0 1-.5.86\"></path><path d=\"M22 8c0-2.5-2-2.5-2-5\"></path><path d=\"M7 12v4\"></path>",
        categories = "travel,transportation,medical",
        tags = "smoking,no-smoking",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CigaretteOff,
    #[cfg(feature = "cigarette")]
    #[strum(props(
        svg = "<path d=\"M17 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h14\"></path><path d=\"M18 8c0-2.5-2-2.5-2-5\"></path><path d=\"M21 16a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1\"></path><path d=\"M22 8c0-2.5-2-2.5-2-5\"></path><path d=\"M7 12v4\"></path>",
        categories = "travel,transportation,medical",
        tags = "smoking",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Cigarette,
    #[cfg(feature = "circle_alert")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><line y2=\"12\" y1=\"8\" x2=\"12\" x1=\"12\"></line><line x2=\"12.01\" y1=\"16\" x1=\"12\" y2=\"16\"></line>",
        categories = "notifications",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    CircleAlert,
    #[cfg(feature = "circle_arrow_down")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M12 8v8\"></path><path d=\"m8 12 4 4 4-4\"></path>",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowDown,
    #[cfg(feature = "circle_arrow_left")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"m12 8-4 4 4 4\"></path><path d=\"M16 12H8\"></path>",
        categories = "arrows,gaming",
        tags = "previous,back,direction,west,sign,turn,button,<-",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowLeft,
    #[cfg(feature = "circle_arrow_out_down_left")]
    #[strum(props(
        svg = "<path d=\"M2 12a10 10 0 1 1 10 10\"></path><path d=\"m2 22 10-10\"></path><path d=\"M8 22H2v-6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutDownLeft,
    #[cfg(feature = "circle_arrow_out_down_right")]
    #[strum(props(
        svg = "<path d=\"M12 22a10 10 0 1 1 10-10\"></path><path d=\"M22 22 12 12\"></path><path d=\"M22 16v6h-6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutDownRight,
    #[cfg(feature = "circle_arrow_out_up_left")]
    #[strum(props(
        svg = "<path d=\"M2 8V2h6\"></path><path d=\"m2 2 10 10\"></path><path d=\"M12 2A10 10 0 1 1 2 12\"></path>",
        categories = "arrows,development",
        tags = "outwards,direction,north-west,diagonal,keyboard,button,escape",
        contributors = "danielbayley"
    ))]
    CircleArrowOutUpLeft,
    #[cfg(feature = "circle_arrow_out_up_right")]
    #[strum(props(
        svg = "<path d=\"M22 12A10 10 0 1 1 12 2\"></path><path d=\"M22 2 12 12\"></path><path d=\"M16 2h6v6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,north-east,diagonal",
        contributors = "danielbayley"
    ))]
    CircleArrowOutUpRight,
    #[cfg(feature = "circle_arrow_right")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"m12 16 4-4-4-4\"></path><path d=\"M8 12h8\"></path>",
        categories = "arrows,gaming",
        tags = "next,forward,direction,east,sign,turn,button,->",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowRight,
    #[cfg(feature = "circle_arrow_up")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m16 12-4-4-4 4\"></path><path d=\"M12 16V8\"></path>",
        categories = "arrows,gaming",
        tags = "forward,direction,north,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    CircleArrowUp,
    #[cfg(feature = "circle_check_big")]
    #[strum(props(
        svg = "<path d=\"M21.80 10A10 10 0 1 1 17 3.33\"></path><path d=\"m9 11 3 3L22 4\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    CircleCheckBig,
    #[cfg(feature = "circle_check")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis"
    ))]
    CircleCheck,
    #[cfg(feature = "circle_chevron_down")]
    #[strum(props(
        svg = "<circle r=\"10\" cy=\"12\" cx=\"12\"></circle><path d=\"m16 10-4 4-4-4\"></path>",
        categories = "arrows",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    CircleChevronDown,
    #[cfg(feature = "circle_chevron_left")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m14 16-4-4 4-4\"></path>",
        categories = "arrows",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    CircleChevronLeft,
    #[cfg(feature = "circle_chevron_right")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m10 8 4 4-4 4\"></path>",
        categories = "arrows",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    CircleChevronRight,
    #[cfg(feature = "circle_chevron_up")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m8 14 4-4 4 4\"></path>",
        categories = "arrows",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    CircleChevronUp,
    #[cfg(feature = "circle_dashed")]
    #[strum(props(
        svg = "<path d=\"M10.1 2.18a10 10 0 0 1 3.8 0\"></path><path d=\"M13.9 21.81a10 10 0 0 1-3.8 0\"></path><path d=\"M17.60 3.72a10 10 0 0 1 2.69 2.7\"></path><path d=\"M2.18 13.9a10 10 0 0 1 0-3.8\"></path><path d=\"M20.27 17.60a10 10 0 0 1-2.7 2.69\"></path><path d=\"M21.81 10.1a10 10 0 0 1 0 3.8\"></path><path d=\"M3.72 6.39a10 10 0 0 1 2.7-2.69\"></path><path d=\"M6.39 20.27a10 10 0 0 1-2.69-2.7\"></path>",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = "danielbayley,jguddas"
    ))]
    CircleDashed,
    #[cfg(feature = "circle_divide")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><line y2=\"12\" y1=\"12\" x2=\"16\" x1=\"8\"></line><line x1=\"12\" x2=\"12\" y2=\"16\" y1=\"16\"></line><line y1=\"8\" x1=\"12\" x2=\"12\" y2=\"8\"></line>",
        categories = "math",
        tags = "calculate,math,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    CircleDivide,
    #[cfg(feature = "circle_dollar_sign")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8\"></path><path d=\"M12 18V6\"></path>",
        categories = "finance",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[cfg(feature = "circle_dot_dashed")]
    #[strum(props(
        svg = "<path d=\"M10.1 2.18a9.93 9.93 0 0 1 3.8 0\"></path><path d=\"M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7\"></path><path d=\"M21.82 10.1a9.93 9.93 0 0 1 0 3.8\"></path><path d=\"M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69\"></path><path d=\"M13.9 21.82a9.94 9.94 0 0 1-3.8 0\"></path><path d=\"M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7\"></path><path d=\"M2.18 13.9a9.93 9.93 0 0 1 0-3.8\"></path><path d=\"M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69\"></path><circle r=\"1\" cy=\"12\" cx=\"12\"></circle>",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = "danielbayley"
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "<circle r=\"10\" cy=\"12\" cx=\"12\"></circle><circle cx=\"12\" cy=\"12\" r=\"1\"></circle>",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M17 12h.01\"></path><path d=\"M12 12h.01\"></path><path d=\"M7 12h.01\"></path>",
        categories = "layout,development",
        tags = "ellipsis,et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"10\" cy=\"12\"></circle><path d=\"M7 10h10\"></path><path d=\"M7 14h10\"></path>",
        categories = "math",
        tags = "calculate,shape,=",
        contributors = "danielbayley"
    ))]
    CircleEqual,
    #[cfg(feature = "circle_fading_arrow_up")]
    #[strum(props(
        svg = "<path d=\"M12 2a10 10 0 0 1 7.38 16.75\"></path><path d=\"m16 12-4-4-4 4\"></path><path d=\"M12 16V8\"></path><path d=\"M2.5 8.87a10 10 0 0 0-.5 3\"></path><path d=\"M2.83 16a10 10 0 0 0 2.43 3.4\"></path><path d=\"M4.63 5.23a10 10 0 0 1 .891-.857\"></path><path d=\"M8.64 21.42a10 10 0 0 0 7.63-.38\"></path>",
        categories = "arrows,development",
        tags = "north,up,upgrade,improve,circle,button",
        contributors = "jordan808,jguddas,colebemis,ericfennis,mosch"
    ))]
    CircleFadingArrowUp,
    #[cfg(feature = "circle_fading_plus")]
    #[strum(props(
        svg = "<path d=\"M12 2a10 10 0 0 1 7.38 16.75\"></path><path d=\"M12 8v8\"></path><path d=\"M16 12H8\"></path><path d=\"M2.5 8.87a10 10 0 0 0-.5 3\"></path><path d=\"M2.83 16a10 10 0 0 0 2.43 3.4\"></path><path d=\"M4.63 5.23a10 10 0 0 1 .891-.857\"></path><path d=\"M8.64 21.42a10 10 0 0 0 7.63-.38\"></path>",
        categories = "communication,social",
        tags = "stories,social media,instagram,facebook,meta,snapchat,sharing,content",
        contributors = "jordan808,jguddas"
    ))]
    CircleFadingPlus,
    #[cfg(feature = "circle_gauge")]
    #[strum(props(
        svg = "<path d=\"M15.6 2.7a10 10 0 1 0 5.7 5.7\"></path><circle cy=\"12\" cx=\"12\" r=\"2\"></circle><path d=\"M13.4 10.6 19 5\"></path>",
        categories = "transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "danielbayley"
    ))]
    CircleGauge,
    #[cfg(feature = "circle_minus")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M8 12h8\"></path>",
        categories = "math",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    CircleMinus,
    #[cfg(feature = "circle_off")]
    #[strum(props(
        svg = "<path d=\"m2 2 20 20\"></path><path d=\"M8.35 2.69A10 10 0 0 1 21.3 15.65\"></path><path d=\"M19.08 19.08A10 10 0 1 1 4.92 4.92\"></path>",
        categories = "shapes",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure",
        contributors = "danielbayley"
    ))]
    CircleOff,
    #[cfg(feature = "circle_parking_off")]
    #[strum(props(
        svg = "<path d=\"M12.65 7H13a3 3 0 0 1 2.98 3.30\"></path><path d=\"M13 13H9\"></path><path d=\"M19.07 19.07A1 1 0 0 1 4.93 4.93\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8.35 2.68a10 10 0 0 1 12.95 12.95\"></path><path d=\"M9 17V9\"></path>",
        categories = "transportation,navigation",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    CircleParkingOff,
    #[cfg(feature = "circle_parking")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M9 17V7h4a3 3 0 0 1 0 6H9\"></path>",
        categories = "transportation,navigation",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleParking,
    #[cfg(feature = "circle_pause")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><line y2=\"9\" x1=\"10\" x2=\"10\" y1=\"15\"></line><line x2=\"14\" y1=\"15\" y2=\"9\" x1=\"14\"></line>",
        categories = "multimedia",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis"
    ))]
    CirclePause,
    #[cfg(feature = "circle_percent")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"m15 9-6 6\"></path><path d=\"M9 9h.01\"></path><path d=\"M15 15h.01\"></path>",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    CirclePercent,
    #[cfg(feature = "circle_pile")]
    #[strum(props(
        svg = "<circle cy=\"19\" r=\"2\" cx=\"12\"></circle><circle r=\"2\" cy=\"5\" cx=\"12\"></circle><circle cx=\"16\" cy=\"12\" r=\"2\"></circle><circle cx=\"20\" r=\"2\" cy=\"19\"></circle><circle cx=\"4\" cy=\"19\" r=\"2\"></circle><circle cy=\"12\" r=\"2\" cx=\"8\"></circle>",
        categories = "shapes",
        tags = "off,zero,record,shape,circle-pile,circle,pile,stack,layer,structure,form,group,collection,stock,inventory,materials,warehouse",
        contributors = "colebemis,nathan-de-pachtere"
    ))]
    CirclePile,
    #[cfg(feature = "circle_play")]
    #[strum(props(
        svg = "<path d=\"M9 9.00a1 1 0 0 1 1.51-.859l4.99 2.99a1 1 0 0 1 0 1.71l-4.99 2.99A1 1 0 0 1 9 14.99z\"></path><circle cx=\"12\" cy=\"12\" r=\"10\"></circle>",
        categories = "multimedia",
        tags = "music,start,run",
        contributors = "colebemis,karsa-mistmere"
    ))]
    CirclePlay,
    #[cfg(feature = "circle_plus")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M8 12h8\"></path><path d=\"M12 8v8\"></path>",
        categories = "math,development,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    CirclePlus,
    #[cfg(feature = "circle_pound_sterling")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M10 16V9.5a1 1 0 0 1 5 0\"></path><path d=\"M8 12h4\"></path><path d=\"M8 16h7\"></path>",
        categories = "finance",
        tags = "monetization,coin,penny,marketing,currency,money,payment,british,gbp,£",
        contributors = "karsa-mistmere,jguddas,danielbayley,LieOnLion"
    ))]
    CirclePoundSterling,
    #[cfg(feature = "circle_power")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M12 7v4\"></path><path d=\"M7.99 9.00a5 5 0 1 0 8-.005\"></path>",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley,jguddas"
    ))]
    CirclePower,
    #[cfg(feature = "circle_question_mark")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\"></path><path d=\"M12 17h.01\"></path>",
        categories = "accessibility,text,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    CircleQuestionMark,
    #[cfg(feature = "circle_slash_2")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M22 2 2 22\"></path>",
        categories = "shapes,math,development",
        tags = "diameter,zero,ø,nothing,null,void,ban,math,divide,division,half,split,/,average,avg,mean,median,normal",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><line y1=\"15\" x1=\"9\" x2=\"15\" y2=\"9\"></line>",
        categories = "development,math",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[cfg(feature = "circle_small")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"6\" cy=\"12\"></circle>",
        categories = "shapes,medical",
        tags = "shape,bullet,gender,genderless",
        contributors = "jamiemlaw"
    ))]
    CircleSmall,
    #[cfg(feature = "circle_star")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M11.05 7.61a1 1 0 0 1 1.90.024l.737 1.45a1 1 0 0 0 .737.53l1.63.256a1 1 0 0 1 .588 1.80l-1.17 1.16a1 1 0 0 0-.282.86l.259 1.61a1 1 0 0 1-1.54 1.13l-1.46-.75a1 1 0 0 0-.912 0l-1.46.75a1 1 0 0 1-1.53-1.13l.258-1.61a1 1 0 0 0-.282-.867l-1.15-1.15a1 1 0 0 1 .572-1.82l1.63-.256a1 1 0 0 0 .737-.535z\"></path>",
        categories = "sports,gaming",
        tags = "badge,medal,honour,decoration,order,pin,laurel,trophy,medallion,insignia,bronze,silver,gold",
        contributors = "karsa-mistmere"
    ))]
    CircleStar,
    #[cfg(feature = "circle_stop")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><rect width=\"6\" rx=\"1\" x=\"9\" height=\"6\" y=\"9\"></rect>",
        categories = "multimedia",
        tags = "media,music",
        contributors = "colebemis,ericfennis"
    ))]
    CircleStop,
    #[cfg(feature = "circle_user_round")]
    #[strum(props(
        svg = "<path d=\"M17.92 20.05a6 6 0 0 0-11.85.001\"></path><circle cy=\"11\" r=\"4\" cx=\"12\"></circle><circle r=\"10\" cx=\"12\" cy=\"12\"></circle>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUserRound,
    #[cfg(feature = "circle_user")]
    #[strum(props(
        svg = "<circle r=\"10\" cy=\"12\" cx=\"12\"></circle><circle r=\"3\" cy=\"10\" cx=\"12\"></circle><path d=\"M7 20.66V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.66\"></path>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    CircleUser,
    #[cfg(feature = "circle_x")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"m15 9-6 6\"></path><path d=\"m9 9 6 6\"></path>",
        categories = "math,development",
        tags = "cancel,close,delete,remove,times,clear,error,incorrect,wrong,mistake,failure,linter,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    CircleX,
    #[cfg(feature = "circle")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle>",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[cfg(feature = "circuit_board")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" y=\"3\" rx=\"2\" x=\"3\"></rect><path d=\"M11 9h4a2 2 0 0 0 2-2V3\"></path><circle cy=\"9\" cx=\"9\" r=\"2\"></circle><path d=\"M7 21v-4a2 2 0 0 1 2-2h4\"></path><circle cx=\"15\" cy=\"15\" r=\"2\"></circle>",
        categories = "science,development",
        tags = "computing,electricity,electronics",
        contributors = "danielbayley,jguddas"
    ))]
    CircuitBoard,
    #[cfg(feature = "citrus")]
    #[strum(props(
        svg = "<path d=\"M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z\"></path><path d=\"M19.65 15.66A8 8 0 0 1 8.35 4.34\"></path><path d=\"m14 10-5.5 5.5\"></path><path d=\"M14 17.85V10H6.15\"></path>",
        categories = "food-beverage",
        tags = "lemon,orange,grapefruit,fruit",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Citrus,
    #[cfg(feature = "clapperboard")]
    #[strum(props(
        svg = "<path d=\"m12.29 3.46 3.02 3.95\"></path><path d=\"M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3z\"></path><path d=\"M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"></path><path d=\"m6.18 5.27 3.1 3.89\"></path>",
        categories = "multimedia",
        tags = "movie,film,video,camera,cinema,cut,action,television,tv,show,entertainment",
        contributors = "it-is-not,ericfennis,danielbayley,torfmuer"
    ))]
    Clapperboard,
    #[cfg(feature = "clipboard_check")]
    #[strum(props(
        svg = "<rect y=\"2\" rx=\"1\" height=\"4\" x=\"8\" ry=\"1\" width=\"8\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"m9 14 2 2 4-4\"></path>",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_clock")]
    #[strum(props(
        svg = "<path d=\"M16 14v2.2l1.6 1\"></path><path d=\"M16 4h2a2 2 0 0 1 2 2v.832\"></path><path d=\"M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2\"></path><circle r=\"6\" cx=\"16\" cy=\"16\"></circle><rect height=\"4\" width=\"8\" x=\"8\" y=\"2\" rx=\"1\"></rect>",
        categories = "time,text",
        tags = "copy,paste,history,log,clock,time,watch,alarm,hour,minute,reminder,scheduled,deadline,pending,time tracking,timesheets,appointment,logbook",
        contributors = "beanduong,colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    ClipboardClock,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "<rect height=\"4\" ry=\"1\" width=\"8\" y=\"2\" x=\"8\" rx=\"1\"></rect><path d=\"M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2\"></path><path d=\"M16 4h2a2 2 0 0 1 2 2v4\"></path><path d=\"M21 14H11\"></path><path d=\"m15 10-4 4 4 4\"></path>",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "<rect y=\"2\" ry=\"1\" height=\"4\" width=\"8\" x=\"8\" rx=\"1\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"M12 11h4\"></path><path d=\"M12 16h4\"></path><path d=\"M8 11h.01\"></path><path d=\"M8 16h.01\"></path>",
        categories = "text",
        tags = "copy,paste,tasks",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardList,
    #[cfg(feature = "clipboard_minus")]
    #[strum(props(
        svg = "<rect y=\"2\" x=\"8\" rx=\"1\" height=\"4\" ry=\"1\" width=\"8\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"M9 14h6\"></path>",
        categories = "text,medical",
        tags = "copy,delete,remove,erase,document,medical,report,doctor",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ClipboardMinus,
    #[cfg(feature = "clipboard_paste")]
    #[strum(props(
        svg = "<path d=\"M11 14h10\"></path><path d=\"M16 4h2a2 2 0 0 1 2 2v1.34\"></path><path d=\"m17 18 4-4-4-4\"></path><path d=\"M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 1.79-1.11\"></path><rect height=\"4\" width=\"8\" rx=\"1\" y=\"2\" x=\"8\"></rect>",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "xnousnow,ericfennis,jguddas"
    ))]
    ClipboardPaste,
    #[cfg(feature = "clipboard_pen_line")]
    #[strum(props(
        svg = "<rect rx=\"1\" width=\"8\" height=\"4\" y=\"2\" x=\"8\"></rect><path d=\"M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5\"></path><path d=\"M16 4h2a2 2 0 0 1 1.73 1\"></path><path d=\"M8 18h1\"></path><path d=\"M21.37 12.62a1 1 0 0 0-3.00-3.00l-4.01 4.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path>",
        categories = "text",
        tags = "paste",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardPenLine,
    #[cfg(feature = "clipboard_pen")]
    #[strum(props(
        svg = "<path d=\"M16 4h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21.34 15.66a1 1 0 1 0-3.00-3.00l-5.01 5.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path><path d=\"M8 22H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><rect rx=\"1\" y=\"2\" height=\"4\" width=\"8\" x=\"8\"></rect>",
        categories = "text",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis,Spleefies"
    ))]
    ClipboardPen,
    #[cfg(feature = "clipboard_plus")]
    #[strum(props(
        svg = "<rect width=\"8\" ry=\"1\" x=\"8\" height=\"4\" y=\"2\" rx=\"1\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"M9 14h6\"></path><path d=\"M12 17v-6\"></path>",
        categories = "text,medical",
        tags = "copy,paste,add,create,new,document,medical,report,doctor",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ClipboardPlus,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "<rect x=\"8\" height=\"4\" width=\"8\" y=\"2\" rx=\"1\" ry=\"1\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"M9 12v-1h6v1\"></path><path d=\"M11 17h2\"></path><path d=\"M12 11v6\"></path>",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "<rect ry=\"1\" width=\"8\" height=\"4\" x=\"8\" rx=\"1\" y=\"2\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path><path d=\"m15 11-6 6\"></path><path d=\"m9 11 6 6\"></path>",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "<rect x=\"8\" rx=\"1\" ry=\"1\" y=\"2\" width=\"8\" height=\"4\"></rect><path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\"></path>",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M12 6v6l2-4\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M12 6v6l-4-2\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M12 6v6l-2-4\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M12 6v6\"></path>",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "<circle r=\"10\" cy=\"12\" cx=\"12\"></circle><path d=\"M12 6v6l4-2\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M12 6v6h4\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock3,
    #[cfg(feature = "clock_4")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M12 6v6l4 2\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[cfg(feature = "clock_5")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M12 6v6l2 4\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"10\" cy=\"12\"></circle><path d=\"M12 6v10\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M12 6v6l-2 4\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M12 6v6l-4 2\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[cfg(feature = "clock_9")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><path d=\"M12 6v6H8\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley,jamiemlaw"
    ))]
    Clock9,
    #[cfg(feature = "clock_alert")]
    #[strum(props(
        svg = "<path d=\"M12 6v6l4 2\"></path><path d=\"M20 12v5\"></path><path d=\"M20 21h.01\"></path><path d=\"M21.25 8.2A10 10 0 1 0 16 21.16\"></path>",
        categories = "time",
        tags = "time,watch,alarm,warning,wrong",
        contributors = "colebemis,jguddas,jamiemlaw"
    ))]
    ClockAlert,
    #[cfg(feature = "clock_arrow_down")]
    #[strum(props(
        svg = "<path d=\"M12 6v6l2 1\"></path><path d=\"M12.33 21.99a10 10 0 1 1 9.58-8.76\"></path><path d=\"m14 18 4 4 4-4\"></path><path d=\"M18 14v8\"></path>",
        categories = "time",
        tags = "time,watch,alarm,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis"
    ))]
    ClockArrowDown,
    #[cfg(feature = "clock_arrow_up")]
    #[strum(props(
        svg = "<path d=\"M12 6v6l1.56.78\"></path><path d=\"M13.22 21.92a10 10 0 1 1 8.76-9.58\"></path><path d=\"m14 18 4-4 4 4\"></path><path d=\"M18 22v-8\"></path>",
        categories = "time",
        tags = "time,watch,alarm,sort,order,ascending,descending,increasing,decreasing,rising,falling",
        contributors = "karsa-mistmere,colebemis"
    ))]
    ClockArrowUp,
    #[cfg(feature = "clock_check")]
    #[strum(props(
        svg = "<path d=\"M12 6v6l4 2\"></path><path d=\"M22 12a10 10 0 1 0-11 9.95\"></path><path d=\"m22 16-5.5 5.5L14 19\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis,jguddas,karsa-mistmere"
    ))]
    ClockCheck,
    #[cfg(feature = "clock_fading")]
    #[strum(props(
        svg = "<path d=\"M12 2a10 10 0 0 1 7.38 16.75\"></path><path d=\"M12 6v6l4 2\"></path><path d=\"M2.5 8.87a10 10 0 0 0-.5 3\"></path><path d=\"M2.83 16a10 10 0 0 0 2.43 3.4\"></path><path d=\"M4.63 5.23a10 10 0 0 1 .891-.857\"></path><path d=\"M8.64 21.42a10 10 0 0 0 7.63-.38\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis,jguddas"
    ))]
    ClockFading,
    #[cfg(feature = "clock_plus")]
    #[strum(props(
        svg = "<path d=\"M12 6v6l3.64 1.82\"></path><path d=\"M16 19h6\"></path><path d=\"M19 16v6\"></path><path d=\"M21.92 13.26a10 10 0 1 0-8.65 8.65\"></path>",
        categories = "time",
        tags = "time,watch,alarm,add,create,new",
        contributors = "karsa-mistmere,colebemis,gubser"
    ))]
    ClockPlus,
    #[cfg(feature = "clock")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M12 6v6l4 2\"></path>",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "closed_caption")]
    #[strum(props(
        svg = "<path d=\"M10 9.17a3 3 0 1 0 0 5.66\"></path><path d=\"M17 9.17a3 3 0 1 0 0 5.66\"></path><rect x=\"2\" y=\"5\" width=\"20\" height=\"14\" rx=\"2\"></rect>",
        categories = "accessibility,multimedia",
        tags = "tv,movie,video,closed captions,subtitles,subhead,transcription,transcribe,dialogue,accessibility",
        contributors = "ericfennis,UsamaKhan"
    ))]
    ClosedCaption,
    #[cfg(feature = "cloud_alert")]
    #[strum(props(
        svg = "<path d=\"M12 12v4\"></path><path d=\"M12 20h.01\"></path><path d=\"M8.12 16.94A7 7 0 1 1 15.71 8h1.79a1 1 0 0 1 0 9h-1.64\"></path>",
        categories = "development",
        tags = "weather,danger,warning,alert,error,sync,network,exclamation",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,lscheibel,jguddas"
    ))]
    CloudAlert,
    #[cfg(feature = "cloud_backup")]
    #[strum(props(
        svg = "<path d=\"M21 15.25A4.5 4.5 0 0 0 17.5 8h-1.79A7 7 0 1 0 3 13.60\"></path><path d=\"M7 11v4h4\"></path><path d=\"M8 19a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5 4.82 4.82 0 0 0-3.41 1.41L7 15\"></path>",
        categories = "arrows,files",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,synchronize,synchronise,refresh,reconnect,transfer,data,security,upload,save,remote,safety",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    CloudBackup,
    #[cfg(feature = "cloud_check")]
    #[strum(props(
        svg = "<path d=\"m17 15-5.5 5.5L9 18\"></path><path d=\"M5.51 16.07A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 3.50 7.32\"></path>",
        categories = "development",
        tags = "sync,network,success,done,completed,saved,persisted",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas,lscheibel"
    ))]
    CloudCheck,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "<path d=\"m10.85 19.77-.383.92\"></path><path d=\"m13.14 14.22.383-.923\"></path><path d=\"M13.14 19.77a3 3 0 1 0-2.29-5.54l-.383-.923\"></path><path d=\"m13.53 20.69-.382-.924a3 3 0 1 1-2.29-5.54\"></path><path d=\"m14.77 15.85.923-.383\"></path><path d=\"m14.77 18.14.923.38\"></path><path d=\"M4.2 15.1a7 7 0 1 1 9.93-9.85A7 7 0 0 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2\"></path><path d=\"m9.22 15.85-.923-.383\"></path><path d=\"m9.22 18.14-.923.38\"></path>",
        categories = "development",
        tags = "computing,ai,cluster,network",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    CloudCog,
    #[cfg(feature = "cloud_download")]
    #[strum(props(
        svg = "<path d=\"M12 13v8l-4-4\"></path><path d=\"m12 21 4-4\"></path><path d=\"M4.39 15.26A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.43 8.28\"></path>",
        categories = "arrows,files",
        tags = "import",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDownload,
    #[cfg(feature = "cloud_drizzle")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"M8 19v1\"></path><path d=\"M8 14v1\"></path><path d=\"M16 19v1\"></path><path d=\"M16 14v1\"></path><path d=\"M12 21v1\"></path><path d=\"M12 16v1\"></path>",
        categories = "weather",
        tags = "weather,shower",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDrizzle,
    #[cfg(feature = "cloud_fog")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"M16 17H7\"></path><path d=\"M17 21H9\"></path>",
        categories = "weather",
        tags = "weather,mist",
        contributors = "ericfennis,karsa-mistmere,mittalyashu"
    ))]
    CloudFog,
    #[cfg(feature = "cloud_hail")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"M16 14v2\"></path><path d=\"M8 14v2\"></path><path d=\"M16 20h.01\"></path><path d=\"M8 20h.01\"></path><path d=\"M12 16v2\"></path><path d=\"M12 22h.01\"></path>",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudHail,
    #[cfg(feature = "cloud_lightning")]
    #[strum(props(
        svg = "<path d=\"M6 16.32A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.97\"></path><path d=\"m13 12-3 5h4l-3 5\"></path>",
        categories = "weather",
        tags = "weather,bolt",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudLightning,
    #[cfg(feature = "cloud_moon_rain")]
    #[strum(props(
        svg = "<path d=\"M11 20v2\"></path><path d=\"M18.37 14.51a6 6 0 0 0 3.46-4.12c.148-.625-.659-.97-1.24-.714a4 4 0 0 1-5.25-5.26c.255-.589-.09-1.39-.716-1.24a6 6 0 0 0-4.59 5.36\"></path><path d=\"M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24\"></path><path d=\"M7 19v2\"></path>",
        categories = "weather",
        tags = "weather,partly,night,rainfall",
        contributors = "it-is-not,karsa-mistmere,jguddas"
    ))]
    CloudMoonRain,
    #[cfg(feature = "cloud_moon")]
    #[strum(props(
        svg = "<path d=\"M13 16a3 3 0 0 1 0 6H7a5 5 0 1 1 4.9-6z\"></path><path d=\"M18.37 14.51a6 6 0 0 0 3.46-4.12c.148-.625-.659-.97-1.24-.714a4 4 0 0 1-5.25-5.26c.255-.589-.09-1.39-.716-1.24a6 6 0 0 0-4.59 5.36\"></path>",
        categories = "weather",
        tags = "weather,night",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudMoon,
    #[cfg(feature = "cloud_off")]
    #[strum(props(
        svg = "<path d=\"M10.94 5.27A7 7 0 0 1 15.71 10h1.79a4.5 4.5 0 0 1 4.22 6.05\"></path><path d=\"M18.79 18.81A4.5 4.5 0 0 1 17.5 19H9A7 7 0 0 1 5.79 5.78\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "connectivity,weather",
        tags = "disconnect",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudOff,
    #[cfg(feature = "cloud_rain_wind")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"m9.2 22 3-7\"></path><path d=\"m9 13-3 7\"></path><path d=\"m17 13-3 7\"></path>",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudRainWind,
    #[cfg(feature = "cloud_rain")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"M16 14v6\"></path><path d=\"M8 14v6\"></path><path d=\"M12 16v6\"></path>",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudRain,
    #[cfg(feature = "cloud_snow")]
    #[strum(props(
        svg = "<path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"M8 15h.01\"></path><path d=\"M8 19h.01\"></path><path d=\"M12 17h.01\"></path><path d=\"M12 21h.01\"></path><path d=\"M16 15h.01\"></path><path d=\"M16 19h.01\"></path>",
        categories = "weather",
        tags = "weather,blizzard",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSnow,
    #[cfg(feature = "cloud_sun_rain")]
    #[strum(props(
        svg = "<path d=\"M12 2v2\"></path><path d=\"m4.93 4.93 1.41 1.41\"></path><path d=\"M20 12h2\"></path><path d=\"m19.07 4.93-1.41 1.41\"></path><path d=\"M15.94 12.65a4 4 0 0 0-5.92-4.12\"></path><path d=\"M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24\"></path><path d=\"M11 20v2\"></path><path d=\"M7 19v2\"></path>",
        categories = "weather",
        tags = "weather,partly,rainfall",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    CloudSunRain,
    #[cfg(feature = "cloud_sun")]
    #[strum(props(
        svg = "<path d=\"M12 2v2\"></path><path d=\"m4.93 4.93 1.41 1.41\"></path><path d=\"M20 12h2\"></path><path d=\"m19.07 4.93-1.41 1.41\"></path><path d=\"M15.94 12.65a4 4 0 0 0-5.92-4.12\"></path><path d=\"M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z\"></path>",
        categories = "weather",
        tags = "weather,partly",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudSun,
    #[cfg(feature = "cloud_sync")]
    #[strum(props(
        svg = "<path d=\"m17 18-1.53 1.60a5 5 0 0 1-8-1.5\"></path><path d=\"M17 22v-4h-4\"></path><path d=\"M20.99 15.25A4.5 4.5 0 0 0 17.49 8h-1.79a7 7 0 1 0-12.70 5.60\"></path><path d=\"M7 10v4h4\"></path><path d=\"m7 14 1.53-1.60a5 5 0 0 1 8 1.5\"></path>",
        categories = "arrows,files",
        tags = "synchronize,synchronise,refresh,reconnect,transfer,backup,storage,upload,download,connection,network,data",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSync,
    #[cfg(feature = "cloud_upload")]
    #[strum(props(
        svg = "<path d=\"M12 13v8\"></path><path d=\"M4 14.89A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.24\"></path><path d=\"m8 17 4-4 4 4\"></path>",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudUpload,
    #[cfg(feature = "cloud")]
    #[strum(props(
        svg = "<path d=\"M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z\"></path>",
        categories = "weather",
        tags = "weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cloud,
    #[cfg(feature = "cloudy")]
    #[strum(props(
        svg = "<path d=\"M17.5 12a1 1 0 1 1 0 9H9.00a7 7 0 1 1 6.70-9z\"></path><path d=\"M21.83 9A3 3 0 0 0 19 7h-2.20a5.5 5.5 0 0 0-10.72.61\"></path>",
        categories = "weather",
        tags = "weather,clouds",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    Cloudy,
    #[cfg(feature = "clover")]
    #[strum(props(
        svg = "<path d=\"M16.17 7.83 2 22\"></path><path d=\"M4.02 12a2.82 2.82 0 1 1 3.81-4.17A2.82 2.82 0 1 1 12 4.02a2.82 2.82 0 1 1 4.17 3.81A2.82 2.82 0 1 1 19.98 12a2.82 2.82 0 1 1-3.81 4.17A2.82 2.82 0 1 1 12 19.98a2.82 2.82 0 1 1-4.17-3.81A1 1 0 1 1 4 12\"></path><path d=\"m7.83 7.83 8.34 8.34\"></path>",
        categories = "gaming",
        tags = "leaf,luck,plant",
        contributors = "ericfennis,csandman,jguddas"
    ))]
    Clover,
    #[cfg(feature = "club")]
    #[strum(props(
        svg = "<path d=\"M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z\"></path><path d=\"M12 17.66L12 22\"></path>",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Club,
    #[cfg(feature = "code_xml")]
    #[strum(props(
        svg = "<path d=\"m18 16 4-4-4-4\"></path><path d=\"m6 8-4 4 4 4\"></path><path d=\"m14.5 4-5 16\"></path>",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "ericfennis,mittalyashu"
    ))]
    CodeXml,
    #[cfg(feature = "code")]
    #[strum(props(
        svg = "<path d=\"m16 18 6-6-6-6\"></path><path d=\"m8 6-6 6 6 6\"></path>",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "colebemis"
    ))]
    Code,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "<path d=\"M10 2v2\"></path><path d=\"M14 2v2\"></path><path d=\"M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1\"></path><path d=\"M6 2v2\"></path>",
        categories = "food-beverage",
        tags = "drink,cup,mug,tea,cafe,hot,beverage",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    Coffee,
    #[cfg(feature = "cog")]
    #[strum(props(
        svg = "<path d=\"M11 10.27 7 3.34\"></path><path d=\"m11 13.73-4 6.93\"></path><path d=\"M12 22v-2\"></path><path d=\"M12 2v2\"></path><path d=\"M14 12h8\"></path><path d=\"m17 20.66-1-1.73\"></path><path d=\"m17 3.34-1 1.73\"></path><path d=\"M2 12h2\"></path><path d=\"m20.66 17-1.73-1\"></path><path d=\"m20.66 7-1.73 1\"></path><path d=\"m3.34 17 1.73-1\"></path><path d=\"m3.34 7 1.73 1\"></path><circle r=\"2\" cx=\"12\" cy=\"12\"></circle><circle cy=\"12\" cx=\"12\" r=\"8\"></circle>",
        categories = "account",
        tags = "computing,settings,cog,edit,gear,preferences,controls,configuration,fixed,build,construction,parts",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Cog,
    #[cfg(feature = "coins")]
    #[strum(props(
        svg = "<path d=\"M13.74 17.73a6 6 0 1 1-7.48-7.48\"></path><path d=\"M15 6h1v4\"></path><path d=\"m6.13 14.76.866-.5 2 3.46\"></path><circle cx=\"16\" cy=\"8\" r=\"6\"></circle>",
        categories = "gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere,jguddas"
    ))]
    Coins,
    #[cfg(feature = "columns_2")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" y=\"3\" x=\"3\" rx=\"2\"></rect><path d=\"M12 3v18\"></path>",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,parallel,series,split,vertical,horizontal,half,center,middle,even,sidebar,drawer,gutter,fold,reflow,typography,pagination,pages",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns2,
    #[cfg(feature = "columns_3_cog")]
    #[strum(props(
        svg = "<path d=\"M10.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v5.5\"></path><path d=\"m14.3 19.6 1-.4\"></path><path d=\"M15 3v7.5\"></path><path d=\"m15.2 16.9-.9-.3\"></path><path d=\"m16.6 21.7.3-.9\"></path><path d=\"m16.8 15.3-.4-1\"></path><path d=\"m19.1 15.2.3-.9\"></path><path d=\"m19.6 21.7-.4-1\"></path><path d=\"m20.7 16.8 1-.4\"></path><path d=\"m21.7 19.4-.9-.3\"></path><path d=\"M9 3v18\"></path><circle cx=\"18\" cy=\"18\" r=\"3\"></circle>",
        categories = "layout,design",
        tags = "columns,settings,customize,table,grid,adjust,configuration,panel,layout",
        contributors = "irvineacosta,danielbayley,karsa-mistmere"
    ))]
    Columns3Cog,
    #[cfg(feature = "columns_3")]
    #[strum(props(
        svg = "<rect width=\"18\" rx=\"2\" y=\"3\" height=\"18\" x=\"3\"></rect><path d=\"M9 3v18\"></path><path d=\"M15 3v18\"></path>",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages",
        contributors = "danielbayley"
    ))]
    Columns3,
    #[cfg(feature = "columns_4")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"18\" rx=\"2\" x=\"3\" height=\"18\"></rect><path d=\"M7.5 3v18\"></path><path d=\"M12 3v18\"></path><path d=\"M16.5 3v18\"></path>",
        categories = "layout,design,text,security",
        tags = "lines,list,queue,preview,parallel,series,split,vertical,horizontal,thirds,triple,center,middle,alignment,even,sidebars,drawers,gutters,fold,reflow,typography,pagination,pages,prison,jail,bars,sentence,police,cops,cell,crime,criminal,justice,law,enforcement,grill",
        contributors = "danielbayley"
    ))]
    Columns4,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "<path d=\"M14 3a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1\"></path><path d=\"M19 3a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1\"></path><path d=\"m7 15 3 3\"></path><path d=\"m7 21 3-3H5a2 2 0 0 1-2-2v-2\"></path><rect y=\"14\" x=\"14\" rx=\"1\" width=\"7\" height=\"7\"></rect><rect height=\"7\" x=\"3\" y=\"3\" rx=\"1\" width=\"7\"></rect>",
        categories = "development,files",
        tags = "cubes,packages,parts,units,collection,cluster,combine,gather,merge",
        contributors = "danielbayley,ericfennis,jguddas"
    ))]
    Combine,
    #[cfg(feature = "command")]
    #[strum(props(
        svg = "<path d=\"M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3\"></path>",
        categories = "development",
        tags = "keyboard,key,mac,cmd,button",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Command,
    #[cfg(feature = "compass")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"m16.24 7.76-1.80 5.41a2 2 0 0 1-1.26 1.26L7.76 16.24l1.80-5.41a2 2 0 0 1 1.26-1.26z\"></path>",
        categories = "navigation,travel",
        tags = "direction,north,east,south,west,safari,browser",
        contributors = "colebemis,jguddas"
    ))]
    Compass,
    #[cfg(feature = "component")]
    #[strum(props(
        svg = "<path d=\"M15.53 11.29a1 1 0 0 0 0 1.41l2.37 2.37a1 1 0 0 0 1.41 0l2.37-2.37a1 1 0 0 0 0-1.41l-2.37-2.37a1 1 0 0 0-1.41 0z\"></path><path d=\"M2.29 11.29a1 1 0 0 0 0 1.41l2.37 2.37a1 1 0 0 0 1.41 0l2.37-2.37a1 1 0 0 0 0-1.41L6.08 8.91a1 1 0 0 0-1.41 0z\"></path><path d=\"M8.91 17.91a1 1 0 0 0 0 1.41l2.37 2.37a1 1 0 0 0 1.41 0l2.37-2.37a1 1 0 0 0 0-1.41l-2.37-2.37a1 1 0 0 0-1.41 0z\"></path><path d=\"M8.91 4.67a1 1 0 0 0 0 1.41l2.37 2.37a1 1 0 0 0 1.41 0l2.37-2.37a1 1 0 0 0 0-1.41l-2.37-2.37a1 1 0 0 0-1.41 0z\"></path>",
        categories = "design,development",
        tags = "design,element,group,module,part,symbol",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Component,
    #[cfg(feature = "computer")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"14\" x=\"5\" y=\"2\" height=\"8\"></rect><rect height=\"8\" width=\"20\" y=\"14\" rx=\"2\" x=\"2\"></rect><path d=\"M6 18h2\"></path><path d=\"M12 18h6\"></path>",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = "danielbayley"
    ))]
    Computer,
    #[cfg(feature = "concierge_bell")]
    #[strum(props(
        svg = "<path d=\"M3 20a1 1 0 0 1-1-1v-1a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1Z\"></path><path d=\"M20 16a8 8 0 1 0-16 0\"></path><path d=\"M12 4v4\"></path><path d=\"M10 4h4\"></path>",
        categories = "travel",
        tags = "reception,bell,porter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ConciergeBell,
    #[cfg(feature = "cone")]
    #[strum(props(
        svg = "<path d=\"m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98\"></path><ellipse rx=\"9\" cy=\"19\" cx=\"12\" ry=\"3\"></ellipse>",
        categories = "shapes,math",
        tags = "conical,triangle,triangular,geometry,filter,funnel,hopper,spotlight,searchlight",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cone,
    #[cfg(feature = "construction")]
    #[strum(props(
        svg = "<rect y=\"6\" width=\"20\" x=\"2\" height=\"8\" rx=\"1\"></rect><path d=\"M17 14v7\"></path><path d=\"M7 14v7\"></path><path d=\"M17 3v3\"></path><path d=\"M7 3v3\"></path><path d=\"M10 14 2.3 6.3\"></path><path d=\"m14 6 7.7 7.7\"></path><path d=\"m8 6 8 8\"></path>",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_round")]
    #[strum(props(
        svg = "<path d=\"M16 2v2\"></path><path d=\"M17.91 22a6 6 0 0 0-12 0\"></path><path d=\"M8 2v2\"></path><circle cx=\"12\" cy=\"12\" r=\"4\"></circle><rect x=\"3\" y=\"4\" height=\"18\" rx=\"2\" width=\"18\"></rect>",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ContactRound,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "<path d=\"M16 2v2\"></path><path d=\"M7 22v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2\"></path><path d=\"M8 2v2\"></path><circle cx=\"12\" cy=\"11\" r=\"3\"></circle><rect height=\"18\" rx=\"2\" x=\"3\" y=\"4\" width=\"18\"></rect>",
        categories = "account,connectivity,communication,social",
        tags = "user,person,family,friend,acquaintance,listing,networking",
        contributors = "lscheibel,karsa-mistmere,FPDK,ericfennis,jguddas"
    ))]
    Contact,
    #[cfg(feature = "container")]
    #[strum(props(
        svg = "<path d=\"M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z\"></path><path d=\"M10 21.9V14L2.1 9.1\"></path><path d=\"m10 14 11.9-6.9\"></path><path d=\"M14 19.8v-8.1\"></path><path d=\"M18 17.5V9.4\"></path>",
        categories = "development,transportation,mail",
        tags = "storage,shipping,freight,supply chain,docker,environment,devops,code,coding",
        contributors = "danielbayley"
    ))]
    Container,
    #[cfg(feature = "contrast")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M12 18a6 6 0 0 0 0-12v12z\"></path>",
        categories = "photography,accessibility,design",
        tags = "display,accessibility",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Contrast,
    #[cfg(feature = "cookie")]
    #[strum(props(
        svg = "<path d=\"M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5\"></path><path d=\"M8.5 8.5v.01\"></path><path d=\"M16 15.5v.01\"></path><path d=\"M12 12v.01\"></path><path d=\"M11 17v.01\"></path><path d=\"M7 14v.01\"></path>",
        categories = "account,food-beverage",
        tags = "biscuit,privacy,legal,food",
        contributors = "it-is-not,ericfennis"
    ))]
    Cookie,
    #[cfg(feature = "cooking_pot")]
    #[strum(props(
        svg = "<path d=\"M2 12h20\"></path><path d=\"M20 12v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8\"></path><path d=\"m4 8 16-4\"></path><path d=\"m8.86 6.78-.45-1.81a2 2 0 0 1 1.45-2.43l1.94-.48a2 2 0 0 1 2.43 1.46l.45 1.8\"></path>",
        categories = "food-beverage,home",
        tags = "pod,cooking,recipe,food,kitchen,chef,restaurant,dinner,lunch,breakfast,meal,eat",
        contributors = "guillermo-angeles,ericfennis"
    ))]
    CookingPot,
    #[cfg(feature = "copy_check")]
    #[strum(props(
        svg = "<path d=\"m12 15 2 2 4-4\"></path><rect ry=\"2\" x=\"8\" width=\"14\" height=\"14\" y=\"8\" rx=\"2\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "<line x2=\"18\" x1=\"12\" y1=\"15\" y2=\"15\"></line><rect x=\"8\" ry=\"2\" rx=\"2\" height=\"14\" y=\"8\" width=\"14\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "text,math",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "<line x2=\"15\" y2=\"18\" x1=\"15\" y1=\"12\"></line><line y1=\"15\" y2=\"15\" x2=\"18\" x1=\"12\"></line><rect y=\"8\" rx=\"2\" height=\"14\" width=\"14\" ry=\"2\" x=\"8\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "text,math",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "<line x2=\"18\" y2=\"12\" x1=\"12\" y1=\"18\"></line><rect height=\"14\" rx=\"2\" width=\"14\" x=\"8\" y=\"8\" ry=\"2\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "text,development,math",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "<line y1=\"12\" x1=\"12\" y2=\"18\" x2=\"18\"></line><line x2=\"18\" x1=\"12\" y1=\"18\" y2=\"12\"></line><rect width=\"14\" x=\"8\" height=\"14\" y=\"8\" ry=\"2\" rx=\"2\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "notifications,math",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "<rect rx=\"2\" y=\"8\" height=\"14\" ry=\"2\" width=\"14\" x=\"8\"></rect><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"></path>",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[cfg(feature = "copyleft")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M9.17 14.83a4 4 0 1 0 0-5.66\"></path>",
        categories = "text",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[cfg(feature = "copyright")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M14.83 14.83a4 4 0 1 1 0-5.66\"></path>",
        categories = "text",
        tags = "licence,license",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyright,
    #[cfg(feature = "corner_down_left")]
    #[strum(props(
        svg = "<path d=\"M20 4v7a4 4 0 0 1-4 4H4\"></path><path d=\"m9 10-5 5 5 5\"></path>",
        categories = "arrows",
        tags = "arrow,return",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownLeft,
    #[cfg(feature = "corner_down_right")]
    #[strum(props(
        svg = "<path d=\"m15 10 5 5-5 5\"></path><path d=\"M4 4v7a4 4 0 0 0 4 4h12\"></path>",
        categories = "arrows,text,development",
        tags = "arrow,indent,tab",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownRight,
    #[cfg(feature = "corner_left_down")]
    #[strum(props(
        svg = "<path d=\"m14 15-5 5-5-5\"></path><path d=\"M20 4h-7a4 4 0 0 0-4 4v12\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftDown,
    #[cfg(feature = "corner_left_up")]
    #[strum(props(
        svg = "<path d=\"M14 9 9 4 4 9\"></path><path d=\"M20 20h-7a4 4 0 0 1-4-4V4\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftUp,
    #[cfg(feature = "corner_right_down")]
    #[strum(props(
        svg = "<path d=\"m10 15 5 5 5-5\"></path><path d=\"M4 4h7a4 4 0 0 1 4 4v12\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightDown,
    #[cfg(feature = "corner_right_up")]
    #[strum(props(
        svg = "<path d=\"m10 9 5-5 5 5\"></path><path d=\"M4 20h7a4 4 0 0 0 4-4V4\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightUp,
    #[cfg(feature = "corner_up_left")]
    #[strum(props(
        svg = "<path d=\"M20 20v-7a4 4 0 0 0-4-4H4\"></path><path d=\"M9 14 4 9l5-5\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpLeft,
    #[cfg(feature = "corner_up_right")]
    #[strum(props(
        svg = "<path d=\"m15 14 5-5-5-5\"></path><path d=\"M4 20v-7a4 4 0 0 1 4-4h12\"></path>",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpRight,
    #[cfg(feature = "cpu")]
    #[strum(props(
        svg = "<path d=\"M12 20v2\"></path><path d=\"M12 2v2\"></path><path d=\"M17 20v2\"></path><path d=\"M17 2v2\"></path><path d=\"M2 12h2\"></path><path d=\"M2 17h2\"></path><path d=\"M2 7h2\"></path><path d=\"M20 12h2\"></path><path d=\"M20 17h2\"></path><path d=\"M20 7h2\"></path><path d=\"M7 20v2\"></path><path d=\"M7 2v2\"></path><rect x=\"4\" width=\"16\" y=\"4\" rx=\"2\" height=\"16\"></rect><rect width=\"8\" x=\"8\" height=\"8\" rx=\"1\" y=\"8\"></rect>",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1\"></path><path d=\"M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1\"></path>",
        categories = "text",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "<rect width=\"20\" x=\"2\" y=\"5\" rx=\"2\" height=\"14\"></rect><line y2=\"10\" x2=\"22\" y1=\"10\" x1=\"2\"></line>",
        categories = "account,finance",
        tags = "bank,purchase,payment,cc",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    CreditCard,
    #[cfg(feature = "croissant")]
    #[strum(props(
        svg = "<path d=\"M10.2 18H4.77a1.5 1.5 0 0 1-1.35-.97 11 11 0 0 1 .132-6.48\"></path><path d=\"M18 10.2V4.77a1.5 1.5 0 0 0-.97-1.35 11 11 0 0 0-6.48.132\"></path><path d=\"M18 5a4 3 0 0 1 4 3 2 2 0 0 1-2 2 10 10 0 0 0-5.13 1.42\"></path><path d=\"M5 18a3 4 0 0 0 3 4 2 2 0 0 0 2-2 10 10 0 0 1 1.42-5.14\"></path><path d=\"M8.70 2.55a10 10 0 0 0-6.15 6.15 1.5 1.5 0 0 0 .676 1.62l9.80 5.42a2 2 0 0 0 2.71-2.71l-5.42-9.80a1.5 1.5 0 0 0-1.62-.676\"></path>",
        categories = "food-beverage",
        tags = "bakery,cooking,food,pastry",
        contributors = "karsa-mistmere"
    ))]
    Croissant,
    #[cfg(feature = "crop")]
    #[strum(props(
        svg = "<path d=\"M6 2v14a2 2 0 0 0 2 2h14\"></path><path d=\"M18 22V8a2 2 0 0 0-2-2H2\"></path>",
        categories = "photography,design",
        tags = "photo,image",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Crop,
    #[cfg(feature = "cross")]
    #[strum(props(
        svg = "<path d=\"M4 9a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h4a1 1 0 0 1 1 1v4a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-4a1 1 0 0 1 1-1h4a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-4a1 1 0 0 1-1-1V4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4a1 1 0 0 1-1 1z\"></path>",
        categories = "shapes",
        tags = "healthcare,first aid",
        contributors = "lscheibel,ericfennis"
    ))]
    Cross,
    #[cfg(feature = "crosshair")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"10\" cy=\"12\"></circle><line x2=\"18\" y1=\"12\" x1=\"22\" y2=\"12\"></line><line y2=\"12\" y1=\"12\" x1=\"6\" x2=\"2\"></line><line x1=\"12\" y2=\"2\" x2=\"12\" y1=\"6\"></line><line x2=\"12\" y1=\"22\" y2=\"18\" x1=\"12\"></line>",
        categories = "photography",
        tags = "aim,target",
        contributors = "colebemis,ericfennis"
    ))]
    Crosshair,
    #[cfg(feature = "crown")]
    #[strum(props(
        svg = "<path d=\"M11.56 3.26a.5.5 0 0 1 .876 0L15.39 8.87a1 1 0 0 0 1.51.294L21.18 5.5a.5.5 0 0 1 .798.51l-2.83 10.24a1 1 0 0 1-.956.73H5.81a1 1 0 0 1-.957-.734L2.02 6.02a.5.5 0 0 1 .798-.519l4.27 3.66a1 1 0 0 0 1.51-.294z\"></path><path d=\"M5 21h14\"></path>",
        categories = "gaming",
        tags = "diadem,tiara,circlet,corona,king,ruler,winner,favourite",
        contributors = "ahtohbi4,ericfennis,csandman,karsa-mistmere"
    ))]
    Crown,
    #[cfg(feature = "cuboid")]
    #[strum(props(
        svg = "<path d=\"M10 22v-8\"></path><path d=\"M2.33 8.89 10 14l11.71-7.02\"></path><path d=\"M22 14a2 2 0 0 1-.971 1.71l-10 6a2 2 0 0 1-2.13-.05l-6-4A2 2 0 0 1 2 16v-6a2 2 0 0 1 .971-1.71l10-6a2 2 0 0 1 2.13.05l6 4A2 2 0 0 1 22 8z\"></path>",
        categories = "shapes,math,food-beverage",
        tags = "brick,block,box,3d,solid,volume,container,storage,shipping,carton,geometry,rectangular,hexahedron,butter,tofu,soap,cheese,package,parcel,crate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Cuboid,
    #[cfg(feature = "cup_soda")]
    #[strum(props(
        svg = "<path d=\"m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8\"></path><path d=\"M5 8h14\"></path><path d=\"M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0\"></path><path d=\"m12 8 1-6h2\"></path>",
        categories = "food-beverage",
        tags = "beverage,cup,drink,soda,straw,water",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CupSoda,
    #[cfg(feature = "currency")]
    #[strum(props(
        svg = "<circle r=\"8\" cx=\"12\" cy=\"12\"></circle><line y1=\"3\" x1=\"3\" x2=\"6\" y2=\"6\"></line><line y1=\"3\" x1=\"21\" x2=\"18\" y2=\"6\"></line><line y1=\"21\" x1=\"3\" x2=\"6\" y2=\"18\"></line><line y2=\"18\" y1=\"21\" x1=\"21\" x2=\"18\"></line>",
        categories = "finance",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "cylinder")]
    #[strum(props(
        svg = "<ellipse cy=\"5\" ry=\"3\" rx=\"9\" cx=\"12\"></ellipse><path d=\"M3 5v14a9 3 0 0 0 18 0V5\"></path>",
        categories = "shapes,design,math",
        tags = "shape,elliptical,geometry,container,storage,tin,pot",
        contributors = "danielbayley"
    ))]
    Cylinder,
    #[cfg(feature = "dam")]
    #[strum(props(
        svg = "<path d=\"M11 11.31c1.17.56 1.54 1.69 3.5 1.69 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M11.75 18c.35.5 1.45 1 2.75 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M2 10h4\"></path><path d=\"M2 14h4\"></path><path d=\"M2 18h4\"></path><path d=\"M2 6h4\"></path><path d=\"M7 3a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1L10 4a1 1 0 0 0-1-1z\"></path>",
        categories = "buildings,sustainability,navigation",
        tags = "electricity,energy,water",
        contributors = "AnnaSasDev"
    ))]
    Dam,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "<ellipse rx=\"9\" ry=\"3\" cy=\"5\" cx=\"12\"></ellipse><path d=\"M3 12a9 3 0 0 0 5 2.69\"></path><path d=\"M21 9.3V5\"></path><path d=\"M3 5v14a9 3 0 0 0 6.47 2.88\"></path><path d=\"M12 12v4h4\"></path><path d=\"M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16\"></path>",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_search")]
    #[strum(props(
        svg = "<path d=\"M21 11.69V5\"></path><path d=\"m22 22-1.87-1.87\"></path><path d=\"M3 12a9 3 0 0 0 8.69 2.99\"></path><path d=\"M3 5v14a9 3 0 0 0 9.28 2.99\"></path><circle cx=\"18\" cy=\"18\" r=\"3\"></circle><ellipse cy=\"5\" rx=\"9\" ry=\"3\" cx=\"12\"></ellipse>",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas,Spleefies"
    ))]
    DatabaseSearch,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "<ellipse rx=\"9\" ry=\"3\" cx=\"12\" cy=\"5\"></ellipse><path d=\"M3 5V19A9 3 0 0 0 15 21.84\"></path><path d=\"M21 5V8\"></path><path d=\"M21 12L18 17H22L19 22\"></path><path d=\"M3 12A9 3 0 0 0 14.59 14.87\"></path>",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "<ellipse cy=\"5\" cx=\"12\" ry=\"3\" rx=\"9\"></ellipse><path d=\"M3 5V19A9 3 0 0 0 21 19V5\"></path><path d=\"M3 12A9 3 0 0 0 21 12\"></path>",
        categories = "devices,development",
        tags = "storage,memory,container,tin,pot,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "decimals_arrow_left")]
    #[strum(props(
        svg = "<path d=\"m13 21-3-3 3-3\"></path><path d=\"M20 18H10\"></path><path d=\"M3 11h.01\"></path><rect rx=\"2.5\" y=\"3\" width=\"5\" height=\"8\" x=\"6\"></rect>",
        categories = "design,text,arrows,math",
        tags = "numerical,decimal,decrease,less,fewer,precision,rounding,digits,fraction,float,number",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    DecimalsArrowLeft,
    #[cfg(feature = "decimals_arrow_right")]
    #[strum(props(
        svg = "<path d=\"M10 18h10\"></path><path d=\"m17 21 3-3-3-3\"></path><path d=\"M3 11h.01\"></path><rect x=\"15\" y=\"3\" width=\"5\" rx=\"2.5\" height=\"8\"></rect><rect width=\"5\" y=\"3\" x=\"6\" height=\"8\" rx=\"2.5\"></rect>",
        categories = "design,text,arrows,math",
        tags = "numerical,decimal,increase,more,precision,rounding,digits,fraction,float,number",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    DecimalsArrowRight,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "<path d=\"M10 5a2 2 0 0 0-1.34.519l-6.32 5.74a1 1 0 0 0 0 1.48l6.32 5.74A2 2 0 0 0 10 19h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2z\"></path><path d=\"m12 9 6 6\"></path><path d=\"m18 9-6 6\"></path>",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "<path d=\"M10.16 3.16A10 10 0 0 0 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4-.006 10 10 0 0 0-8.16-9.82\"></path><path d=\"M20.80 14.86a9 9 0 0 1-17.60 0\"></path><circle r=\"2\" cx=\"12\" cy=\"4\"></circle>",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = "danielbayley,jguddas"
    ))]
    Dessert,
    #[cfg(feature = "diameter")]
    #[strum(props(
        svg = "<circle cy=\"19\" r=\"2\" cx=\"19\"></circle><circle r=\"2\" cx=\"5\" cy=\"5\"></circle><path d=\"M6.48 3.66a10 10 0 0 1 13.86 13.86\"></path><path d=\"m6.41 6.41 11.18 11.18\"></path><path d=\"M3.66 6.48a10 10 0 0 0 13.86 13.86\"></path>",
        categories = "shapes,math,design,tools",
        tags = "shape,circle,geometry,trigonometry,width,height,size,calculate,measure",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Diameter,
    #[cfg(feature = "diamond_minus")]
    #[strum(props(
        svg = "<path d=\"M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z\"></path><path d=\"M8 12h8\"></path>",
        categories = "multimedia,photography,tools,devices",
        tags = "keyframe,subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "chessurisme"
    ))]
    DiamondMinus,
    #[cfg(feature = "diamond_percent")]
    #[strum(props(
        svg = "<path d=\"M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z\"></path><path d=\"M9.2 9.2h.01\"></path><path d=\"m14.5 9.5-5 5\"></path><path d=\"M14.7 14.8h.01\"></path>",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    DiamondPercent,
    #[cfg(feature = "diamond_plus")]
    #[strum(props(
        svg = "<path d=\"M12 8v8\"></path><path d=\"M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z\"></path><path d=\"M8 12h8\"></path>",
        categories = "multimedia,photography,tools,devices",
        tags = "keyframe,add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "chessurisme"
    ))]
    DiamondPlus,
    #[cfg(feature = "diamond")]
    #[strum(props(
        svg = "<path d=\"M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z\"></path>",
        categories = "shapes,gaming",
        tags = "square,rectangle,oblique,rhombus,shape,suit,playing,cards",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Diamond,
    #[cfg(feature = "dice_1")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" ry=\"2\" rx=\"2\" x=\"3\" y=\"3\"></rect><path d=\"M12 12h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"18\" height=\"18\" ry=\"2\" x=\"3\" rx=\"2\"></rect><path d=\"M15 9h.01\"></path><path d=\"M9 15h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" ry=\"2\" y=\"3\" height=\"18\" rx=\"2\"></rect><path d=\"M16 8h.01\"></path><path d=\"M12 12h.01\"></path><path d=\"M8 16h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" x=\"3\" y=\"3\" ry=\"2\" width=\"18\"></rect><path d=\"M16 8h.01\"></path><path d=\"M8 8h.01\"></path><path d=\"M8 16h.01\"></path><path d=\"M16 16h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" y=\"3\" height=\"18\" rx=\"2\" ry=\"2\"></rect><path d=\"M16 8h.01\"></path><path d=\"M8 8h.01\"></path><path d=\"M8 16h.01\"></path><path d=\"M16 16h.01\"></path><path d=\"M12 12h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" height=\"18\" rx=\"2\" y=\"3\" ry=\"2\"></rect><path d=\"M16 8h.01\"></path><path d=\"M16 12h.01\"></path><path d=\"M16 16h.01\"></path><path d=\"M8 8h.01\"></path><path d=\"M8 12h.01\"></path><path d=\"M8 16h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "<rect x=\"2\" height=\"12\" y=\"10\" rx=\"2\" ry=\"2\" width=\"12\"></rect><path d=\"m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6\"></path><path d=\"M6 18h.01\"></path><path d=\"M10 14h.01\"></path><path d=\"M15 6h.01\"></path><path d=\"M18 9h.01\"></path>",
        categories = "gaming",
        tags = "dice,random,tabletop,board,game",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dices,
    #[cfg(feature = "diff")]
    #[strum(props(
        svg = "<path d=\"M12 3v14\"></path><path d=\"M5 10h14\"></path><path d=\"M5 21h14\"></path>",
        categories = "development,files",
        tags = "patch,difference,compare,plus,minus,plus-minus,math",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Diff,
    #[cfg(feature = "disc_2")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"10\" cy=\"12\"></circle><circle cx=\"12\" cy=\"12\" r=\"4\"></circle><path d=\"M12 12h.01\"></path>",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M6 12c0-1.7.7-3.2 1.8-4.2\"></path><circle cx=\"12\" r=\"2\" cy=\"12\"></circle><path d=\"M18 12c0 1.7-.7 3.2-1.8 4.2\"></path>",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc_album")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"18\" rx=\"2\" height=\"18\" x=\"3\"></rect><circle cx=\"12\" cy=\"12\" r=\"5\"></circle><path d=\"M12 12h.01\"></path>",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "danielbayley"
    ))]
    DiscAlbum,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><circle cx=\"12\" cy=\"12\" r=\"2\"></circle>",
        categories = "devices,multimedia",
        tags = "album,music,songs,format,cd,dvd,vinyl,sleeve,cover,platinum,compilation,ep,recording,playback,spin,rotate,rpm,dj",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"6\" r=\"1\"></circle><line x2=\"19\" x1=\"5\" y2=\"12\" y1=\"12\"></line><circle cy=\"18\" cx=\"12\" r=\"1\"></circle>",
        categories = "math,development",
        tags = "calculate,math,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "<path d=\"M15 2c-1.35 1.5-2.09 3-2.5 4.5L14 8\"></path><path d=\"m17 6-2.89-2.89\"></path><path d=\"M2 15c3.33-3 6.66-3 10-3\"></path><path d=\"m2 2 20 20\"></path><path d=\"m20 9 .891.89\"></path><path d=\"M22 9c-1.5 1.35-3 2.09-4.5 2.5l-1-1\"></path><path d=\"M3.10 14.10 4 15\"></path><path d=\"m6.5 12.5 1 1\"></path><path d=\"m7 18 2.89 2.89\"></path><path d=\"M9 22c1.35-1.5 2.09-3 2.5-4.5L10 16\"></path>",
        categories = "medical,food-beverage",
        tags = "gene,gmo free,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    DnaOff,
    #[cfg(feature = "dna")]
    #[strum(props(
        svg = "<path d=\"m10 16 1.5 1.5\"></path><path d=\"m14 8-1.5-1.5\"></path><path d=\"M15 2c-1.79 1.99-2.51 3.99-2.80 5.99\"></path><path d=\"m16.5 10.5 1 1\"></path><path d=\"m17 6-2.89-2.89\"></path><path d=\"M2 15c6.66-6 13.33 0 20-6\"></path><path d=\"m20 9 .891.89\"></path><path d=\"M3.10 14.10 4 15\"></path><path d=\"m6.5 12.5 1 1\"></path><path d=\"m7 18 2.89 2.89\"></path><path d=\"M9 22c1.79-1.99 2.51-3.99 2.80-5.99\"></path>",
        categories = "medical",
        tags = "gene,gmo,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere"
    ))]
    Dna,
    #[cfg(feature = "dock")]
    #[strum(props(
        svg = "<path d=\"M2 8h20\"></path><rect width=\"20\" height=\"16\" y=\"4\" x=\"2\" rx=\"2\"></rect><path d=\"M6 16h12\"></path>",
        categories = "layout,design,development,files",
        tags = "desktop,applications,launch,home,menu bar,bottom,line,macos,osx",
        contributors = "danielbayley"
    ))]
    Dock,
    #[cfg(feature = "dog")]
    #[strum(props(
        svg = "<path d=\"M11.25 16.25h1.5L12 17z\"></path><path d=\"M16 14v.5\"></path><path d=\"M4.42 11.24A13.15 13.15 0 0 0 4 14.55C4 18.72 7.58 21 12 21s8-2.27 8-6.44a11.70 11.70 0 0 0-.493-3.30\"></path><path d=\"M8 14v.5\"></path><path d=\"M8.5 8.5c-.384 1.05-1.08 2.02-2.34 2.5-1.93.722-3.57-.297-3.65-1-.113-.994 1.17-6.53 4-7 1.92-.321 3.65.845 3.65 2.23A7.49 7.49 0 0 1 14 5.27c0-1.39 1.84-2.59 3.76-2.27 2.82.47 4.11 6.00 4 7-.08.70-1.72 1.72-3.65 1-1.26-.472-1.85-1.45-2.23-2.5\"></path>",
        categories = "animals",
        tags = "animal,pet,puppy,hound,canine",
        contributors = "kemie,jguddas"
    ))]
    Dog,
    #[cfg(feature = "dollar_sign")]
    #[strum(props(
        svg = "<line x2=\"12\" y1=\"2\" y2=\"22\" x1=\"12\"></line><path d=\"M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "<path d=\"M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3\"></path><circle r=\"3\" cy=\"12\" cx=\"12\"></circle>",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = "danielbayley"
    ))]
    Donut,
    #[cfg(feature = "door_closed_locked")]
    #[strum(props(
        svg = "<path d=\"M10 12h.01\"></path><path d=\"M18 9V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14\"></path><path d=\"M2 20h8\"></path><path d=\"M20 17v-2a2 2 0 1 0-4 0v2\"></path><rect x=\"14\" rx=\"1\" height=\"5\" y=\"17\" width=\"8\"></rect>",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit,lock",
        contributors = "karsa-mistmere,lukedukeus"
    ))]
    DoorClosedLocked,
    #[cfg(feature = "door_closed")]
    #[strum(props(
        svg = "<path d=\"M10 12h.01\"></path><path d=\"M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14\"></path><path d=\"M2 20h20\"></path>",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorClosed,
    #[cfg(feature = "door_open")]
    #[strum(props(
        svg = "<path d=\"M11 20H2\"></path><path d=\"M11 4.56v16.15a1 1 0 0 0 1.24.97L19 20V5.56a2 2 0 0 0-1.51-1.94l-4-1A2 2 0 0 0 11 4.56z\"></path><path d=\"M11 4H8a2 2 0 0 0-2 2v14\"></path><path d=\"M14 12h.01\"></path><path d=\"M22 20h-3\"></path>",
        categories = "home,travel,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorOpen,
    #[cfg(feature = "dot")]
    #[strum(props(
        svg = "<circle cy=\"12.1\" cx=\"12.1\" r=\"1\"></circle>",
        categories = "shapes,text",
        tags = "interpunct,interpoint,middot,step,punctuation,period,full stop,end,finish,final,characters,font,typography,type,center,.",
        contributors = "danielbayley"
    ))]
    Dot,
    #[cfg(feature = "download")]
    #[strum(props(
        svg = "<path d=\"M12 15V3\"></path><path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"></path><path d=\"m7 10 5 5 5-5\"></path>",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[cfg(feature = "drafting_compass")]
    #[strum(props(
        svg = "<path d=\"m12.99 6.74 1.93 3.44\"></path><path d=\"M19.13 12a10 10 0 0 1-14.27 0\"></path><path d=\"m21 21-2.16-3.84\"></path><path d=\"m3 21 8.02-14.26\"></path><circle cx=\"12\" r=\"2\" cy=\"5\"></circle>",
        categories = "math,design,tools",
        tags = "geometry,trigonometry,radius,diameter,circumference,calculate,measure,arc,curve,draw,sketch",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    DraftingCompass,
    #[cfg(feature = "drama")]
    #[strum(props(
        svg = "<path d=\"M10 11h.01\"></path><path d=\"M14 6h.01\"></path><path d=\"M18 6h.01\"></path><path d=\"M6.5 13.1h.01\"></path><path d=\"M22 5c0 9-4 12-6 12s-6-3-6-12c0-2 2-3 6-3s6 1 6 3\"></path><path d=\"M17.4 9.9c-.8.8-2 .8-2.8 0\"></path><path d=\"M10.1 7.1C9 7.2 7.7 7.7 6 8.6c-3.5 2-4.7 3.9-3.7 5.6 4.5 7.8 9.5 8.4 11.2 7.4.9-.5 1.9-2.1 1.9-4.7\"></path><path d=\"M9.1 16.5c.3-1.1 1.4-1.7 2.4-1.4\"></path>",
        categories = "multimedia",
        tags = "drama,masks,theater,theatre,entertainment,show",
        contributors = "danielbayley"
    ))]
    Drama,
    #[cfg(feature = "drill")]
    #[strum(props(
        svg = "<path d=\"M10 18a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H5a3 3 0 0 1-3-3 1 1 0 0 1 1-1z\"></path><path d=\"M13 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1l-.81 3.24a1 1 0 0 1-.97.75H8\"></path><path d=\"M14 4h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3\"></path><path d=\"M18 6h4\"></path><path d=\"m5 10-2 8\"></path><path d=\"m7 18 2-8\"></path>",
        categories = "tools,home,devices",
        tags = "power,bit,head,hole,diy,toolbox,build,construction",
        contributors = "danielbayley,jguddas"
    ))]
    Drill,
    #[cfg(feature = "drone")]
    #[strum(props(
        svg = "<path d=\"M10 10 7 7\"></path><path d=\"m10 14-3 3\"></path><path d=\"m14 10 3-3\"></path><path d=\"m14 14 3 3\"></path><path d=\"M14.20 4.13a4 4 0 1 1 5.43 5.86\"></path><path d=\"M19.63 14a4 4 0 1 1-5.43 5.86\"></path><path d=\"M4.36 10a4 4 0 1 1 5.43-5.86\"></path><path d=\"M9.79 19.86a4 4 0 1 1-5.42-5.87\"></path><rect width=\"4\" height=\"8\" y=\"8\" rx=\"1\" x=\"10\"></rect>",
        categories = "transportation,devices",
        tags = "quadcopter,uav,aerial,flight,flying,technology,airborne,robotics",
        contributors = "bernatfortet,shopped,karsa-mistmere,jguddas"
    ))]
    Drone,
    #[cfg(feature = "droplet_off")]
    #[strum(props(
        svg = "<path d=\"M18.71 13.18C18.29 11.85 17.38 10.60 16 9.5c-2-1.6-3.5-4-4-6.5a10.7 10.7 0 0 1-.884 2.58\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8.79 8.79A11 11 0 0 1 8 9.5C6 11.1 5 13 5 15a7 7 0 0 0 13.22 3.20\"></path>",
        categories = "weather,gaming",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "colebemis,ericfennis,csandman,johnletey,jguddas,Footagesus"
    ))]
    DropletOff,
    #[cfg(feature = "droplet")]
    #[strum(props(
        svg = "<path d=\"M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z\"></path>",
        categories = "weather,gaming",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Droplet,
    #[cfg(feature = "droplets")]
    #[strum(props(
        svg = "<path d=\"M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z\"></path><path d=\"M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97\"></path>",
        categories = "weather",
        tags = "water,weather,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "Andreto,ericfennis"
    ))]
    Droplets,
    #[cfg(feature = "drum")]
    #[strum(props(
        svg = "<path d=\"m2 2 8 8\"></path><path d=\"m22 2-8 8\"></path><ellipse cy=\"9\" ry=\"5\" cx=\"12\" rx=\"10\"></ellipse><path d=\"M7 13.4v7.9\"></path><path d=\"M12 14v8\"></path><path d=\"M17 13.4v7.9\"></path><path d=\"M2 9v8a10 5 0 0 0 20 0V9\"></path>",
        categories = "multimedia,devices",
        tags = "drummer,kit,sticks,instrument,beat,bang,bass,backing track,band,play,performance,concert,march,music,audio,sound,noise,loud",
        contributors = "danielbayley"
    ))]
    Drum,
    #[cfg(feature = "drumstick")]
    #[strum(props(
        svg = "<path d=\"M15.4 15.63a7.87 6 135 1 1 6.23-6.23 4.5 3.43 135 0 0-6.23 6.23\"></path><path d=\"m8.29 12.71-2.6 2.6a2.5 2.5 0 1 0-1.65 4.65A2.5 2.5 0 1 0 8.7 18.3l2.59-2.59\"></path>",
        categories = "food-beverage",
        tags = "food,chicken,meat",
        contributors = "Andreto,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Drumstick,
    #[cfg(feature = "dumbbell")]
    #[strum(props(
        svg = "<path d=\"M17.59 12.76a2 2 0 1 0 2.82-2.82l-1.76-1.76a2 2 0 0 0 2.82-2.82l-2.82-2.82a2 2 0 0 0-2.82 2.82l-1.76-1.76a2 2 0 1 0-2.82 2.82z\"></path><path d=\"m2.5 21.5 1.4-1.4\"></path><path d=\"m20.1 3.9 1.4-1.4\"></path><path d=\"M5.34 21.48a2 2 0 1 0 2.82-2.82l1.76 1.76a2 2 0 1 0 2.82-2.82l-6.36-6.36a2 2 0 1 0-2.82 2.82l1.76 1.76a2 2 0 0 0-2.82 2.82z\"></path><path d=\"m9.6 14.4 4.8-4.8\"></path>",
        categories = "navigation,sports",
        tags = "barbell,weight,workout,gym",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Dumbbell,
    #[cfg(feature = "ear_off")]
    #[strum(props(
        svg = "<path d=\"M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46\"></path><path d=\"M6 8.5c0-.75.13-1.47.36-2.14\"></path><path d=\"M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76\"></path><path d=\"M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18\"></path><line y2=\"22\" x1=\"2\" y1=\"2\" x2=\"22\"></line>",
        categories = "medical,accessibility",
        tags = "hearing,hard of hearing,hearing loss,deafness,noise,silence,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EarOff,
    #[cfg(feature = "ear")]
    #[strum(props(
        svg = "<path d=\"M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0\"></path><path d=\"M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4\"></path>",
        categories = "medical,accessibility",
        tags = "hearing,noise,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Ear,
    #[cfg(feature = "earth_lock")]
    #[strum(props(
        svg = "<path d=\"M7 3.34V5a3 3 0 0 0 3 3\"></path><path d=\"M11 21.95V18a2 2 0 0 0-2-2 2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05\"></path><path d=\"M21.54 15H17a2 2 0 0 0-2 2v4.54\"></path><path d=\"M12 2a10 10 0 1 0 9.54 13\"></path><path d=\"M20 6V4a2 2 0 1 0-4 0v2\"></path><rect x=\"14\" width=\"8\" rx=\"1\" height=\"5\" y=\"6\"></rect>",
        categories = "security,development,devices",
        tags = "vpn,private,privacy,network,world,browser,security,encryption,protection,connection",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    EarthLock,
    #[cfg(feature = "earth")]
    #[strum(props(
        svg = "<path d=\"M21.54 15H17a2 2 0 0 0-2 2v4.54\"></path><path d=\"M7 3.34V5a3 3 0 0 0 3 3a2 2 0 0 1 2 2c0 1.1.9 2 2 2a2 2 0 0 0 2-2c0-1.1.9-2 2-2h3.17\"></path><path d=\"M11 21.95V18a2 2 0 0 0-2-2a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05\"></path><circle cx=\"12\" cy=\"12\" r=\"10\"></circle>",
        categories = "navigation",
        tags = "world,browser,language,translate,globe",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Earth,
    #[cfg(feature = "eclipse")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M12 2a7 7 0 1 0 10 10\"></path>",
        categories = "science,design,development,accessibility,photography",
        tags = "lunar,solar,crescent moon,sun,earth,day,night,planet,space,mode,dark,light,toggle,switch,color,css,styles,display,accessibility,contrast,brightness,blend,shade",
        contributors = "danielbayley"
    ))]
    Eclipse,
    #[cfg(feature = "egg_fried")]
    #[strum(props(
        svg = "<circle cx=\"11.5\" cy=\"12.5\" r=\"3.5\"></circle><path d=\"M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z\"></path>",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "<path d=\"m2 2 20 20\"></path><path d=\"M20 14.34V14c0-6-4-12-8-12-1.07 0-2.15.436-3.15 1.19\"></path><path d=\"M6.20 6.21C4.87 8.4 4 11.2 4 14a8 8 0 0 0 14.56 4.56\"></path>",
        categories = "food-beverage",
        tags = "egg free,vegan,hatched,bad egg",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    EggOff,
    #[cfg(feature = "egg")]
    #[strum(props(
        svg = "<path d=\"M12 2C8 2 4 8 4 14a8 8 0 0 0 16 0c0-6-4-12-8-12\"></path>",
        categories = "food-beverage,animals",
        tags = "bird,chicken,nest,hatch,shell,incubate,soft boiled,hard,breakfast,brunch,morning,easter",
        contributors = "mittalyashu,Andreto,ericfennis,jamiemlaw"
    ))]
    Egg,
    #[cfg(feature = "ellipse")]
    #[strum(props(
        svg = "<ellipse rx=\"10\" ry=\"6\" cy=\"12\" cx=\"12\"></ellipse>",
        categories = "shapes",
        tags = "shape,geometry,rounded,smooth,outline,form,boundary,curve,shapes,ellipse,oval",
        contributors = "KISHORE-KUMAR-S"
    ))]
    Ellipse,
    #[cfg(feature = "ellipsis_vertical")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"1\"></circle><circle r=\"1\" cx=\"12\" cy=\"5\"></circle><circle r=\"1\" cy=\"19\" cx=\"12\"></circle>",
        categories = "layout",
        tags = "menu,options,spread,more,further,extra,overflow,dots,…,...",
        contributors = "colebemis"
    ))]
    EllipsisVertical,
    #[cfg(feature = "ellipsis")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"1\"></circle><circle cy=\"12\" r=\"1\" cx=\"19\"></circle><circle cx=\"5\" r=\"1\" cy=\"12\"></circle>",
        categories = "layout,development",
        tags = "et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,coding,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "colebemis"
    ))]
    Ellipsis,
    #[cfg(feature = "equal_approximately")]
    #[strum(props(
        svg = "<path d=\"M5 15a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0\"></path><path d=\"M5 9a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0\"></path>",
        categories = "math",
        tags = "about,calculate,math,operater",
        contributors = "ksk3110"
    ))]
    EqualApproximately,
    #[cfg(feature = "equal_not")]
    #[strum(props(
        svg = "<line x2=\"19\" y1=\"9\" y2=\"9\" x1=\"5\"></line><line y1=\"15\" x1=\"5\" y2=\"15\" x2=\"19\"></line><line x2=\"5\" x1=\"19\" y1=\"5\" y2=\"19\"></line>",
        categories = "math,development",
        tags = "calculate,off,math,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "<line x1=\"5\" y1=\"9\" x2=\"19\" y2=\"9\"></line><line y1=\"15\" x1=\"5\" y2=\"15\" x2=\"19\"></line>",
        categories = "math,development",
        tags = "calculate,math,operator,assignment,code,=",
        contributors = "ericfennis"
    ))]
    Equal,
    #[cfg(feature = "eraser")]
    #[strum(props(
        svg = "<path d=\"M21 21H8a2 2 0 0 1-1.42-.587l-3.99-3.99a2 2 0 0 1 0-2.82l10-10a2 2 0 0 1 2.82 0l5.99 6a2 2 0 0 1 0 2.82L12.83 21\"></path><path d=\"m5.08 11.09 8.82 8.82\"></path>",
        categories = "text",
        tags = "pencil,drawing,undo,delete,clear,trash,remove",
        contributors = "maxwellito,karsa-mistmere,jguddas"
    ))]
    Eraser,
    #[cfg(feature = "ethernet_port")]
    #[strum(props(
        svg = "<path d=\"m15 20 3-3h2a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h2l3 3z\"></path><path d=\"M6 8v1\"></path><path d=\"M10 8v1\"></path><path d=\"M14 8v1\"></path><path d=\"M18 8v1\"></path>",
        categories = "communication,devices,multimedia,gaming",
        tags = "internet,network,connection,cable,lan,port,router,switch,hub,modem,web,online,networking,communication,socket,plug,slot,controller,connector,interface,console,signal,data,input,output",
        contributors = "ericfennis"
    ))]
    EthernetPort,
    #[cfg(feature = "euro")]
    #[strum(props(
        svg = "<path d=\"M4 10h12\"></path><path d=\"M4 14h9\"></path><path d=\"M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Euro,
    #[cfg(feature = "ev_charger")]
    #[strum(props(
        svg = "<path d=\"M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 4 0v-6.99a2 2 0 0 0-.59-1.42L18 5\"></path><path d=\"M14 21V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16\"></path><path d=\"M2 21h13\"></path><path d=\"M3 7h11\"></path><path d=\"m9 11-2 3h3l-2 3\"></path>",
        categories = "transportation,navigation",
        tags = "electric,charger,station,vehicle,fast,plug,ev,power,electricity,energy,accumulator,charge",
        contributors = "UsamaKhan,karsa-mistmere,ericfennis,colebemis,csandman,johnletey"
    ))]
    EvCharger,
    #[cfg(feature = "expand")]
    #[strum(props(
        svg = "<path d=\"m15 15 6 6\"></path><path d=\"m15 9 6-6\"></path><path d=\"M21 16v5h-5\"></path><path d=\"M21 8V3h-5\"></path><path d=\"M3 16v5h5\"></path><path d=\"m3 21 6-6\"></path><path d=\"M3 8V3h5\"></path><path d=\"M9 9 3 3\"></path>",
        categories = "text,arrows",
        tags = "scale,fullscreen,maximize,minimize,contract",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    Expand,
    #[cfg(feature = "external_link")]
    #[strum(props(
        svg = "<path d=\"M15 3h6v6\"></path><path d=\"M10 14 21 3\"></path><path d=\"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6\"></path>",
        categories = "arrows,text,social",
        tags = "outbound,open,share",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_closed")]
    #[strum(props(
        svg = "<path d=\"m15 18-.722-3.25\"></path><path d=\"M2 8a10.64 10.64 0 0 0 20 0\"></path><path d=\"m20 15-1.72-2.05\"></path><path d=\"m4 15 1.72-2.05\"></path><path d=\"m9 18 .722-3.25\"></path>",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,hide,conceal,mask,hidden,visibility,vision",
        contributors = "karsa-mistmere"
    ))]
    EyeClosed,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "<path d=\"M10.73 5.07a10.74 10.74 0 0 1 11.20 6.57 1 1 0 0 1 0 .696 10.74 10.74 0 0 1-1.44 2.49\"></path><path d=\"M14.08 14.15a3 3 0 0 1-4.24-4.24\"></path><path d=\"M17.47 17.49a10.75 10.75 0 0 1-15.41-5.15 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.44-5.14\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,hide,conceal,mask,hidden,visibility,vision",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "<path d=\"M2.06 12.34a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.87 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.87 0\"></path><circle r=\"3\" cx=\"12\" cy=\"12\"></circle>",
        categories = "accessibility,photography,design,security",
        tags = "view,watch,see,show,expose,reveal,display,visible,visibility,vision,preview,read",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Eye,
    #[cfg(feature = "factory")]
    #[strum(props(
        svg = "<path d=\"M12 16h.01\"></path><path d=\"M16 16h.01\"></path><path d=\"M3 19a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.5a.5.5 0 0 0-.769-.422l-4.46 2.84A.5.5 0 0 1 15 10.5v-2a.5.5 0 0 0-.769-.422L9.77 10.92A.5.5 0 0 1 9 10.5V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2z\"></path><path d=\"M8 16h.01\"></path>",
        categories = "buildings,navigation",
        tags = "building,business,energy,industry,manufacture,sector",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Factory,
    #[cfg(feature = "fan")]
    #[strum(props(
        svg = "<path d=\"M10.82 16.37a6.08 6.08 0 0 1-8.61-7.00l5.41 1.45a6.08 6.08 0 0 1 7.00-8.61l-1.45 5.41a6.08 6.08 0 0 1 8.61 7.00l-5.41-1.45a6.08 6.08 0 0 1-7.00 8.61l1.45-5.41Z\"></path><path d=\"M12 12v.01\"></path>",
        categories = "home",
        tags = "air,cooler,ventilation,ventilator,blower",
        contributors = "karsa-mistmere"
    ))]
    Fan,
    #[cfg(feature = "fast_forward")]
    #[strum(props(
        svg = "<path d=\"M12 6a2 2 0 0 1 3.41-1.41l6 6a2 2 0 0 1 0 2.82l-6 6A2 2 0 0 1 12 18z\"></path><path d=\"M2 6a2 2 0 0 1 3.41-1.41l6 6a2 2 0 0 1 0 2.82l-6 6A2 2 0 0 1 2 18z\"></path>",
        categories = "multimedia,arrows",
        tags = "music",
        contributors = "colebemis,karsa-mistmere"
    ))]
    FastForward,
    #[cfg(feature = "feather")]
    #[strum(props(
        svg = "<path d=\"M12.67 19a2 2 0 0 0 1.41-.588l6.15-6.17a6 6 0 0 0-8.49-8.49L5.58 9.91A2 2 0 0 0 5 11.32V18a1 1 0 0 0 1 1z\"></path><path d=\"M16 8 2 22\"></path><path d=\"M17.5 15H9\"></path>",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[cfg(feature = "fence")]
    #[strum(props(
        svg = "<path d=\"M4 3 2 5v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z\"></path><path d=\"M6 8h4\"></path><path d=\"M6 18h4\"></path><path d=\"m12 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z\"></path><path d=\"M14 8h4\"></path><path d=\"M14 18h4\"></path><path d=\"m20 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z\"></path>",
        categories = "home,buildings",
        tags = "picket,panels,woodwork,diy,materials,suburban,garden,property,territory",
        contributors = "danielbayley"
    ))]
    Fence,
    #[cfg(feature = "ferris_wheel")]
    #[strum(props(
        svg = "<circle r=\"2\" cy=\"12\" cx=\"12\"></circle><path d=\"M12 2v4\"></path><path d=\"m6.8 15-3.5 2\"></path><path d=\"m20.7 7-3.5 2\"></path><path d=\"M6.8 9 3.3 7\"></path><path d=\"m20.7 17-3.5-2\"></path><path d=\"m9 22 3-8 3 8\"></path><path d=\"M8 22h8\"></path><path d=\"M18 18.7a9 9 0 1 0-12 0\"></path>",
        categories = "navigation",
        tags = "big wheel,daisy wheel,observation,attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    FerrisWheel,
    #[cfg(feature = "file_archive")]
    #[strum(props(
        svg = "<path d=\"M13.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v11.5\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 12v-1\"></path><path d=\"M8 18v-2\"></path><path d=\"M8 7V6\"></path><circle cx=\"8\" r=\"2\" cy=\"20\"></circle>",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileArchive,
    #[cfg(feature = "file_axis_3_d")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m8 18 4-4\"></path><path d=\"M8 10v8h8\"></path>",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileAxis3D,
    #[cfg(feature = "file_badge")]
    #[strum(props(
        svg = "<path d=\"M13 22h5a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.3\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m7.69 16.47 1.29 4.88a.5.5 0 0 1-.698.59l-1.84-.849a1 1 0 0 0-.879.00l-1.84.85a.5.5 0 0 1-.692-.593l1.29-4.88\"></path><circle cy=\"14\" r=\"3\" cx=\"6\"></circle>",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBadge,
    #[cfg(feature = "file_box")]
    #[strum(props(
        svg = "<path d=\"M14.5 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.8\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M11.7 14.2 7 17l-4.7-2.8\"></path><path d=\"M3 13.1a2 2 0 0 0-.999 1.76v3.24a2 2 0 0 0 .969 1.78L6 21.7a2 2 0 0 0 2.03.01L11 19.9a2 2 0 0 0 1-1.76V14.9a2 2 0 0 0-.97-1.78L8 11.3a2 2 0 0 0-2.03-.01z\"></path><path d=\"M7 17v5\"></path>",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBox,
    #[cfg(feature = "file_braces_corner")]
    #[strum(props(
        svg = "<path d=\"M14 22h4a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v6\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M5 14a1 1 0 0 0-1 1v2a1 1 0 0 1-1 1 1 1 0 0 1 1 1v2a1 1 0 0 0 1 1\"></path><path d=\"M9 22a1 1 0 0 0 1-1v-2a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-2a1 1 0 0 0-1-1\"></path>",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBracesCorner,
    #[cfg(feature = "file_braces")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1\"></path><path d=\"M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1\"></path>",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileBraces,
    #[cfg(feature = "file_chart_column_increasing")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 18v-2\"></path><path d=\"M12 18v-4\"></path><path d=\"M16 18v-6\"></path>",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation,trending up",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartColumnIncreasing,
    #[cfg(feature = "file_chart_column")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 18v-1\"></path><path d=\"M12 18v-6\"></path><path d=\"M16 18v-3\"></path>",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartColumn,
    #[cfg(feature = "file_chart_line")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m16 13-3.5 3.5-2-2L8 17\"></path>",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileChartLine,
    #[cfg(feature = "file_chart_pie")]
    #[strum(props(
        svg = "<path d=\"M15.94 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.51\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M4.01 11.51a6 6 0 1 0 8.46 8.47\"></path><path d=\"M9 16a1 1 0 0 1-1-1v-4c0-.552.45-1.00.995-.917a6 6 0 0 1 4.92 4.92c.091.54-.365.99-.917.99z\"></path>",
        categories = "files",
        tags = "statistics,analytics,diagram,graph,presentation",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileChartPie,
    #[cfg(feature = "file_check_corner")]
    #[strum(props(
        svg = "<path d=\"M10.5 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v6\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m14 20 2 2 4-4\"></path>",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCheckCorner,
    #[cfg(feature = "file_check")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m9 15 2 2 4-4\"></path>",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,danielbayley"
    ))]
    FileCheck,
    #[cfg(feature = "file_clock")]
    #[strum(props(
        svg = "<path d=\"M16 22h2a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v2.85\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 14v2.2l1.6 1\"></path><circle cx=\"8\" cy=\"16\" r=\"6\"></circle>",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    FileClock,
    #[cfg(feature = "file_code_corner")]
    #[strum(props(
        svg = "<path d=\"M4 12.15V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-3.35\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m5 16-3 3 3 3\"></path><path d=\"m9 22 3-3-3-3\"></path>",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCodeCorner,
    #[cfg(feature = "file_code")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10 12.5 8 15l2 2.5\"></path><path d=\"m14 12.5 2 2.5-2 2.5\"></path>",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[cfg(feature = "file_cog")]
    #[strum(props(
        svg = "<path d=\"M15 8a1 1 0 0 1-1-1V2a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8z\"></path><path d=\"M20 8v12a2 2 0 0 1-2 2h-4.18\"></path><path d=\"m3.30 19.53.92-.382\"></path><path d=\"M4 10.59V4a2 2 0 0 1 2-2h8\"></path><path d=\"m4.22 16.85-.924-.383\"></path><path d=\"m5.85 15.22-.383-.923\"></path><path d=\"m5.85 20.77-.383.92\"></path><path d=\"m8.14 15.22.383-.923\"></path><path d=\"m8.53 21.69-.382-.924\"></path><path d=\"m9.77 16.85.922-.383\"></path><path d=\"m9.77 19.14.922.38\"></path><circle cy=\"18\" r=\"3\" cx=\"7\"></circle>",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere,danielbayley,jguddas,UsamaKhan"
    ))]
    FileCog,
    #[cfg(feature = "file_diff")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M9 10h6\"></path><path d=\"M12 13V7\"></path><path d=\"M9 17h6\"></path>",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDiff,
    #[cfg(feature = "file_digit")]
    #[strum(props(
        svg = "<path d=\"M4 12V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10 16h2v6\"></path><path d=\"M10 22h4\"></path><rect y=\"16\" x=\"2\" height=\"6\" rx=\"2\" width=\"4\"></rect>",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileDigit,
    #[cfg(feature = "file_down")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M12 18v-6\"></path><path d=\"m9 15 3 3 3-3\"></path>",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileDown,
    #[cfg(feature = "file_exclamation_point")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M12 9v4\"></path><path d=\"M12 17h.01\"></path>",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileExclamationPoint,
    #[cfg(feature = "file_headphone")]
    #[strum(props(
        svg = "<path d=\"M4 6.83V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-.343\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M2 19a2 2 0 0 1 4 0v1a2 2 0 0 1-4 0v-4a6 6 0 0 1 12 0v4a2 2 0 0 1-4 0v-1a2 2 0 0 1 4 0\"></path>",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileHeadphone,
    #[cfg(feature = "file_heart")]
    #[strum(props(
        svg = "<path d=\"M13 22h5a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v7\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M3.62 18.8A2.25 2.25 0 1 1 7 15.83a2.25 2.25 0 1 1 3.38 2.96l-2.62 2.85a1 1 0 0 1-1.50 0z\"></path>",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileHeart,
    #[cfg(feature = "file_image")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><circle cx=\"10\" cy=\"12\" r=\"2\"></circle><path d=\"m20 17-1.29-1.29a2.41 2.41 0 0 0-3.40 0L9 22\"></path>",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileImage,
    #[cfg(feature = "file_input")]
    #[strum(props(
        svg = "<path d=\"M4 11V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-1\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M2 15h10\"></path><path d=\"m9 18 3-3-3-3\"></path>",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "<path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M4 12v6\"></path><path d=\"M4 14h2\"></path><path d=\"M9.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v4\"></path><circle cy=\"20\" cx=\"4\" r=\"2\"></circle>",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileKey,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "<path d=\"M4 9.8V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-3\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M9 17v-2a2 2 0 0 0-4 0v2\"></path><rect width=\"8\" y=\"17\" rx=\"1\" x=\"3\" height=\"5\"></rect>",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileLock,
    #[cfg(feature = "file_minus_corner")]
    #[strum(props(
        svg = "<path d=\"M20 14V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M14 18h6\"></path>",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileMinusCorner,
    #[cfg(feature = "file_minus")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M9 15h6\"></path>",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileMinus,
    #[cfg(feature = "file_music")]
    #[strum(props(
        svg = "<path d=\"M11.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v10.35\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 20v-7l3 1.47\"></path><circle r=\"2\" cx=\"6\" cy=\"20\"></circle>",
        categories = "files,multimedia",
        tags = "audio,sound,noise,track,digital,recording,playback,piano,keyboard,keys,notes,chord,midi,octave",
        contributors = "danielbayley,jguddas"
    ))]
    FileMusic,
    #[cfg(feature = "file_output")]
    #[strum(props(
        svg = "<path d=\"M4.22 20.92A2 2 0 0 0 6 22h12a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.12\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m5 11-3 3\"></path><path d=\"m5 17-3-3h10\"></path>",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    FileOutput,
    #[cfg(feature = "file_pen_line")]
    #[strum(props(
        svg = "<path d=\"M14.36 13.63a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506l4.01-4.00a1 1 0 0 0-3.00-3.00z\"></path><path d=\"M14.48 7.85A1 1 0 0 1 14 7V2\"></path><path d=\"M20 19.64V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l2.51 2.51\"></path><path d=\"M8 18h1\"></path>",
        categories = "files",
        tags = "edit",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    FilePenLine,
    #[cfg(feature = "file_pen")]
    #[strum(props(
        svg = "<path d=\"M12.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v9.34\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10.37 12.62a1 1 0 0 1 3 3.00L8.36 20.63a2 2 0 0 1-.854.50l-2.86.837a.5.5 0 0 1-.62-.62l.836-2.86a2 2 0 0 1 .506-.853z\"></path>",
        categories = "files",
        tags = "signature",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePen,
    #[cfg(feature = "file_play")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M15.03 13.44a.647.64 0 0 1 0 1.12l-4.06 2.35a.645.64 0 0 1-.968-.56v-4.70a.645.64 0 0 1 .967-.56z\"></path>",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FilePlay,
    #[cfg(feature = "file_plus_corner")]
    #[strum(props(
        svg = "<path d=\"M11.35 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v5.35\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M14 19h6\"></path><path d=\"M17 16v6\"></path>",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FilePlusCorner,
    #[cfg(feature = "file_plus")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M9 15h6\"></path><path d=\"M12 18v-6\"></path>",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FilePlus,
    #[cfg(feature = "file_question_mark")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M12 17h.01\"></path><path d=\"M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3\"></path>",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileQuestionMark,
    #[cfg(feature = "file_scan")]
    #[strum(props(
        svg = "<path d=\"M20 10V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4.35\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M16 14a2 2 0 0 0-2 2\"></path><path d=\"M16 22a2 2 0 0 1-2-2\"></path><path d=\"M20 14a2 2 0 0 1 2 2\"></path><path d=\"M20 22a2 2 0 0 0 2-2\"></path>",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileScan,
    #[cfg(feature = "file_search_corner")]
    #[strum(props(
        svg = "<path d=\"M11.1 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v3.25\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m21 22-2.88-2.88\"></path><circle cy=\"17\" cx=\"16\" r=\"3\"></circle>",
        categories = "files",
        tags = "lost,document,find,browser,lens",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSearchCorner,
    #[cfg(feature = "file_search")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><circle cy=\"14.5\" cx=\"11.5\" r=\"2.5\"></circle><path d=\"M13.3 16.3 15 18\"></path>",
        categories = "files",
        tags = "lost,document,find,browser,lens",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    FileSearch,
    #[cfg(feature = "file_signal")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 15h.01\"></path><path d=\"M11.5 13.5a2.5 2.5 0 0 1 0 3\"></path><path d=\"M15 12a5 5 0 0 1 0 6\"></path>",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileSignal,
    #[cfg(feature = "file_sliders")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 12h8\"></path><path d=\"M10 11v2\"></path><path d=\"M8 17h8\"></path><path d=\"M14 16v2\"></path>",
        categories = "files,development",
        tags = "cogged,gear,mechanical,machinery,configuration,controls,preferences,settings,system,admin,edit,executable",
        contributors = "danielbayley"
    ))]
    FileSliders,
    #[cfg(feature = "file_spreadsheet")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 13h2\"></path><path d=\"M14 13h2\"></path><path d=\"M8 17h2\"></path><path d=\"M14 17h2\"></path>",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileSpreadsheet,
    #[cfg(feature = "file_stack")]
    #[strum(props(
        svg = "<path d=\"M11 21a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-8a1 1 0 0 1 1-1\"></path><path d=\"M16 16a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1V8a1 1 0 0 1 1-1\"></path><path d=\"M21 6a2 2 0 0 0-.586-1.41l-2-2A2 2 0 0 0 17 2h-3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1z\"></path>",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FileStack,
    #[cfg(feature = "file_symlink")]
    #[strum(props(
        svg = "<path d=\"M4 11V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m10 18 3-3-3-3\"></path>",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileSymlink,
    #[cfg(feature = "file_terminal")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m8 16 2-2-2-2\"></path><path d=\"M12 18h4\"></path>",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileTerminal,
    #[cfg(feature = "file_text")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10 9H8\"></path><path d=\"M16 13H8\"></path><path d=\"M16 17H8\"></path>",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    FileText,
    #[cfg(feature = "file_type_corner")]
    #[strum(props(
        svg = "<path d=\"M12 22h6a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v6\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M3 16v-1.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5V16\"></path><path d=\"M6 22h2\"></path><path d=\"M7 14v8\"></path>",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileTypeCorner,
    #[cfg(feature = "file_type")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M11 18h2\"></path><path d=\"M12 12v6\"></path><path d=\"M9 13v-.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 .5.5v.5\"></path>",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileType,
    #[cfg(feature = "file_up")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M12 12v6\"></path><path d=\"m15 15-3-3-3 3\"></path>",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    FileUp,
    #[cfg(feature = "file_user")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M16 22a4 4 0 0 0-8 0\"></path><circle cx=\"12\" cy=\"15\" r=\"3\"></circle>",
        categories = "account,files",
        tags = "person,personal information,people,listing,networking,document,contact,cover letter,resume,cv,curriculum vitae,application form",
        contributors = "danielbayley,colebemis,ericfennis,jguddas"
    ))]
    FileUser,
    #[cfg(feature = "file_video_camera")]
    #[strum(props(
        svg = "<path d=\"M4 12V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m10 17.84 3.03-1.75a.64.64 0 0 1 .967.56v4.70a.65.65 0 0 1-.967.56L10 20.15\"></path><rect rx=\"1\" height=\"6\" x=\"3\" y=\"16\" width=\"7\"></rect>",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    FileVideoCamera,
    #[cfg(feature = "file_volume")]
    #[strum(props(
        svg = "<path d=\"M4 11.55V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-1.95\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M12 15a5 5 0 0 1 0 6\"></path><path d=\"M8 14.50a.5.5 0 0 0-.826-.381l-1.89 1.63a1 1 0 0 1-.651.24H3.5a.5.5 0 0 0-.5.50v3.00a.5.5 0 0 0 .5.50h1.12a1 1 0 0 1 .652.24l1.89 1.63a.5.5 0 0 0 .826-.38z\"></path>",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    FileVolume,
    #[cfg(feature = "file_x_corner")]
    #[strum(props(
        svg = "<path d=\"M11 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v5\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m15 17 5 5\"></path><path d=\"m20 17-5 5\"></path>",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileXCorner,
    #[cfg(feature = "file_x")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"m14.5 12.5-5 5\"></path><path d=\"m9.5 12.5 5 5\"></path>",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis,danielbayley"
    ))]
    FileX,
    #[cfg(feature = "file")]
    #[strum(props(
        svg = "<path d=\"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path>",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    File,
    #[cfg(feature = "files")]
    #[strum(props(
        svg = "<path d=\"M15 2h-4a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8\"></path><path d=\"M16.70 2.70A2.4 2.4 0 0 0 15 2v5a1 1 0 0 0 1 1h5a2.4 2.4 0 0 0-.706-1.70z\"></path><path d=\"M5 7a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h8a2 2 0 0 0 1.73-1\"></path>",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Files,
    #[cfg(feature = "film")]
    #[strum(props(
        svg = "<rect y=\"3\" height=\"18\" rx=\"2\" width=\"18\" x=\"3\"></rect><path d=\"M7 3v18\"></path><path d=\"M3 7.5h4\"></path><path d=\"M3 12h18\"></path><path d=\"M3 16.5h4\"></path><path d=\"M17 3v18\"></path><path d=\"M17 7.5h4\"></path><path d=\"M17 16.5h4\"></path>",
        categories = "photography,multimedia",
        tags = "movie,video,reel,camera,cinema,entertainment",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Film,
    #[cfg(feature = "fingerprint_pattern")]
    #[strum(props(
        svg = "<path d=\"M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4\"></path><path d=\"M14 13.12c0 2.38 0 6.38-1 8.88\"></path><path d=\"M17.29 21.02c.12-.6.43-2.3.5-3.02\"></path><path d=\"M2 12a10 10 0 0 1 18-6\"></path><path d=\"M2 16h.01\"></path><path d=\"M21.8 16c.2-2 .131-5.35 0-6\"></path><path d=\"M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2\"></path><path d=\"M8.65 22c.21-.66.45-1.32.57-2\"></path><path d=\"M9 6.8a6 6 0 0 1 9 5.2v2\"></path>",
        categories = "account,security,medical,devices",
        tags = "2fa,authentication,biometric,identity,security",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FingerprintPattern,
    #[cfg(feature = "fire_extinguisher")]
    #[strum(props(
        svg = "<path d=\"M15 6.5V3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3.5\"></path><path d=\"M9 18h8\"></path><path d=\"M18 3h-3\"></path><path d=\"M11 3a6 6 0 0 0-6 6v11\"></path><path d=\"M5 13h4\"></path><path d=\"M17 10a4 4 0 0 0-8 0v10a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2Z\"></path>",
        categories = "home,tools,travel",
        tags = "flames,smoke,foam,water,spray,hose,firefighter,fireman,department,brigade,station,emergency,suppress,compressed,tank,cylinder,safety,equipment,amenities",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FireExtinguisher,
    #[cfg(feature = "fish_off")]
    #[strum(props(
        svg = "<path d=\"M18 12.47v.03m0-.5v.47m-.475 5.05A6.74 6.74 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.11-2.43 2.12-3.38m3.44-2.08A8.80 8.80 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.17-1.41 3.05\"></path><path d=\"M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.37A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.72-3.61\"></path><path d=\"m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20\"></path>",
        categories = "food-beverage,animals",
        tags = "food,dish,restaurant,course,meal,seafood,animal,pet,sea,marine,allergy,intolerance,diet",
        contributors = "jguddas,kemie,ericfennis"
    ))]
    FishOff,
    #[cfg(feature = "fish_symbol")]
    #[strum(props(
        svg = "<path d=\"M2 16s9-15 20-4C11 23 2 8 2 8\"></path>",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "danielbayley"
    ))]
    FishSymbol,
    #[cfg(feature = "fish")]
    #[strum(props(
        svg = "<path d=\"M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z\"></path><path d=\"M18 12v.5\"></path><path d=\"M16 17.93a9.77 9.77 0 0 1 0-11.86\"></path><path d=\"M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33\"></path><path d=\"M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4\"></path><path d=\"m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98\"></path>",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "kemie"
    ))]
    Fish,
    #[cfg(feature = "fishing_hook")]
    #[strum(props(
        svg = "<path d=\"m17.58 11.41-5.93 5.93a1 1 0 0 1-8-8l3.13-3.13a.707.70 0 0 1 1.20.5V10\"></path><path d=\"M20.41 8.58 22 7\"></path><circle r=\"2\" cx=\"19\" cy=\"10\"></circle>",
        categories = "sports,travel",
        tags = "sea,boating,angler,bait,reel,tackle,marine,outdoors,fish,fishing,hook,sports,travel",
        contributors = "7ender,jguddas,karsa-mistmere,jamiemlaw"
    ))]
    FishingHook,
    #[cfg(feature = "fishing_rod")]
    #[strum(props(
        svg = "<path d=\"M4 11h1\"></path><path d=\"M8 15a2 2 0 0 1-4 0V3a1 1 0 0 1 1-1h.5C14 2 20 9 20 18v4\"></path><circle r=\"2\" cx=\"18\" cy=\"18\"></circle>",
        categories = "sports,travel",
        tags = "fishing,rod,hobby,equipment,reel",
        contributors = "7ender,jguddas,karsa-mistmere"
    ))]
    FishingRod,
    #[cfg(feature = "flag_off")]
    #[strum(props(
        svg = "<path d=\"M16 16c-3 0-5-2-8-2a6 6 0 0 0-4 1.52\"></path><path d=\"m2 2 20 20\"></path><path d=\"M4 22V4\"></path><path d=\"M7.65 2H8c3 0 5 2 7.33 2q2 0 3.06-.8A1 1 0 0 1 20 4v10.34\"></path>",
        categories = "account,social",
        tags = "unflag,unmark,report,marker,notification,warning,milestone,goal,notice,signal,attention,banner",
        contributors = "karsa-mistmere,cyberalien,ericfennis,jamiemlaw"
    ))]
    FlagOff,
    #[cfg(feature = "flag_triangle_left")]
    #[strum(props(
        svg = "<path d=\"M18 22V2.8a.8.8 0 0 0-1.17-.71L5.45 7.78a.8.8 0 0 0 0 1.44L18 15.5\"></path>",
        categories = "development,navigation",
        tags = "report,timeline,marker,pin",
        contributors = "tidoni,ericfennis,jamiemlaw"
    ))]
    FlagTriangleLeft,
    #[cfg(feature = "flag_triangle_right")]
    #[strum(props(
        svg = "<path d=\"M6 22V2.8a.8.8 0 0 1 1.17-.71l11.38 5.69a.8.8 0 0 1 0 1.44L6 15.5\"></path>",
        categories = "development,navigation",
        tags = "report,timeline,marker,pin",
        contributors = "tidoni,ericfennis,jamiemlaw"
    ))]
    FlagTriangleRight,
    #[cfg(feature = "flag")]
    #[strum(props(
        svg = "<path d=\"M4 22V4a1 1 0 0 1 .4-.8A6 6 0 0 1 8 2c3 0 5 2 7.33 2q2 0 3.06-.8A1 1 0 0 1 20 4v10a1 1 0 0 1-.4.8A6 6 0 0 1 16 16c-3 0-5-2-8-2a6 6 0 0 0-4 1.52\"></path>",
        categories = "account,social",
        tags = "report,marker,notification,warning,milestone,goal,notice,signal,attention,banner",
        contributors = "colebemis,ericfennis,jamiemlaw"
    ))]
    Flag,
    #[cfg(feature = "flame_kindling")]
    #[strum(props(
        svg = "<path d=\"M12 2c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 17 10a5 5 0 1 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C8 4.5 11 2 12 2Z\"></path><path d=\"m5 22 14-4\"></path><path d=\"m5 18 14 4\"></path>",
        categories = "nature,social,gaming",
        tags = "campfire,camping,wilderness,outdoors,lit,warmth,wood,twigs,sticks",
        contributors = "danielbayley"
    ))]
    FlameKindling,
    #[cfg(feature = "flame")]
    #[strum(props(
        svg = "<path d=\"M12 3q1 4 4 6.5t3 5.5a1 1 0 0 1-14 0 5 5 0 0 1 1-3 1 1 0 0 0 5 0c0-2-1.5-3-1.5-5q0-2 2.5-4\"></path>",
        categories = "weather,social,gaming",
        tags = "heat,burn,light,glow,ignite,passion,ember,fire,lit,burning,spark,embers,smoke,firefighter,fireman,department,brigade,station,emergency",
        contributors = "ericfennis,johnletey,csandman,jamiemlaw"
    ))]
    Flame,
    #[cfg(feature = "flashlight_off")]
    #[strum(props(
        svg = "<path d=\"M11.65 6H18\"></path><path d=\"M12 13v1\"></path><path d=\"M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-8a4 4 0 0 0-.8-2.4l-.6-.8A3 3 0 0 1 6 7V6\"></path><path d=\"m2 2 20 20\"></path><path d=\"M7.64 2H17a1 1 0 0 1 1 1v4a3 3 0 0 1-.6 1.8l-.6.8a4 4 0 0 0-.55 1.00\"></path>",
        categories = "photography,devices",
        tags = "torch,light,beam,emergency,safety,tool,bright",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman,jamiemlaw"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "<path d=\"M12 13v1\"></path><path d=\"M17 2a1 1 0 0 1 1 1v4a3 3 0 0 1-.6 1.8l-.6.8A4 4 0 0 0 16 12v8a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2v-8a4 4 0 0 0-.8-2.4l-.6-.8A3 3 0 0 1 6 7V3a1 1 0 0 1 1-1z\"></path><path d=\"M6 6h12\"></path>",
        categories = "photography,devices",
        tags = "torch,light,beam,emergency,safety,tool,bright",
        contributors = "csandman,ericfennis,jamiemlaw"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "<path d=\"M10 2v2.34\"></path><path d=\"M14 2v6.34\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20 20a2 2 0 0 1-2 2H6a2 2 0 0 1-1.75-2.96l5.22-9.56\"></path><path d=\"M6.45 15H15\"></path><path d=\"M8.5 2h7\"></path>",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,non toxic,lab,chemistry,experiment,test",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    FlaskConicalOff,
    #[cfg(feature = "flask_conical")]
    #[strum(props(
        svg = "<path d=\"M14 2v6a2 2 0 0 0 .245.96l5.51 10.08A2 2 0 0 1 18 22H6a2 2 0 0 1-1.75-2.96l5.51-10.08A2 2 0 0 0 10 8V2\"></path><path d=\"M6.45 15h11.09\"></path><path d=\"M8.5 2h7\"></path>",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    FlaskConical,
    #[cfg(feature = "flask_round")]
    #[strum(props(
        svg = "<path d=\"M10 2v6.29a7 7 0 1 0 4 0V2\"></path><path d=\"M5 15h14\"></path><path d=\"M8.5 2h7\"></path>",
        categories = "science,gaming",
        tags = "beaker,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,danielbayley,jamiemlaw"
    ))]
    FlaskRound,
    #[cfg(feature = "flip_horizontal_2")]
    #[strum(props(
        svg = "<path d=\"m3 7 5 5-5 5V7\"></path><path d=\"m21 7-5 5 5 5V7\"></path><path d=\"M12 20v2\"></path><path d=\"M12 14v2\"></path><path d=\"M12 8v2\"></path><path d=\"M12 2v2\"></path>",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal2,
    #[cfg(feature = "flip_vertical_2")]
    #[strum(props(
        svg = "<path d=\"m17 3-5 5-5-5h10\"></path><path d=\"m17 21-5-5-5 5h10\"></path><path d=\"M4 12H2\"></path><path d=\"M10 12H8\"></path><path d=\"M16 12h-2\"></path><path d=\"M22 12h-2\"></path>",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    FlipVertical2,
    #[cfg(feature = "flower_2")]
    #[strum(props(
        svg = "<path d=\"M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1\"></path><circle cx=\"12\" cy=\"8\" r=\"2\"></circle><path d=\"M12 10v12\"></path><path d=\"M12 22c4.2 0 7-1.66 7-5-4.2 0-7 1.66-7 5Z\"></path><path d=\"M12 22c-4.2 0-7-1.66-7-5 4.2 0 7 1.66 7 5Z\"></path>",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"3\"></circle><path d=\"M12 16.5A4.5 4.5 0 1 1 7.5 12 4.5 4.5 0 1 1 12 7.5a4.5 4.5 0 1 1 4.5 4.5 4.5 4.5 0 1 1-4.5 4.5\"></path><path d=\"M12 7.5V9\"></path><path d=\"M7.5 12H9\"></path><path d=\"M16.5 12H15\"></path><path d=\"M12 16.5V15\"></path><path d=\"m8 8 1.88 1.88\"></path><path d=\"M14.12 9.88 16 8\"></path><path d=\"m8 16 1.88-1.88\"></path><path d=\"M14.12 14.12 16 16\"></path>",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"12\" cy=\"12\"></circle><path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path>",
        categories = "photography",
        tags = "camera,lens,photo,dashed",
        contributors = "karsa-mistmere,danielbayley,jguddas,ericfennis"
    ))]
    Focus,
    #[cfg(feature = "fold_horizontal")]
    #[strum(props(
        svg = "<path d=\"M2 12h6\"></path><path d=\"M22 12h-6\"></path><path d=\"M12 2v2\"></path><path d=\"M12 8v2\"></path><path d=\"M12 14v2\"></path><path d=\"M12 20v2\"></path><path d=\"m19 9-3 3 3 3\"></path><path d=\"m5 15 3-3-3-3\"></path>",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    FoldHorizontal,
    #[cfg(feature = "fold_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 22v-6\"></path><path d=\"M12 8V2\"></path><path d=\"M4 12H2\"></path><path d=\"M10 12H8\"></path><path d=\"M16 12h-2\"></path><path d=\"M22 12h-2\"></path><path d=\"m15 19-3-3-3 3\"></path><path d=\"m15 5-3 3-3-3\"></path>",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    FoldVertical,
    #[cfg(feature = "folder_archive")]
    #[strum(props(
        svg = "<circle cx=\"15\" cy=\"19\" r=\"2\"></circle><path d=\"M20.9 19.8A2 2 0 0 0 22 18V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2h5.1\"></path><path d=\"M15 11v-1\"></path><path d=\"M15 17v-2\"></path>",
        categories = "files",
        tags = "archive,zip,package",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderArchive,
    #[cfg(feature = "folder_check")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"m9 13 2 2 4-4\"></path>",
        categories = "files",
        tags = "done,directory,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderCheck,
    #[cfg(feature = "folder_clock")]
    #[strum(props(
        svg = "<path d=\"M16 14v2.2l1.6 1\"></path><path d=\"M7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2\"></path><circle cx=\"16\" r=\"6\" cy=\"16\"></circle>",
        categories = "files,time",
        tags = "history,directory,clock",
        contributors = "karsa-mistmere,jguddas,jamiemlaw"
    ))]
    FolderClock,
    #[cfg(feature = "folder_closed")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"M2 10h20\"></path>",
        categories = "files",
        tags = "directory,closed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderClosed,
    #[cfg(feature = "folder_code")]
    #[strum(props(
        svg = "<path d=\"M10 10.5 8 13l2 2.5\"></path><path d=\"m14 10.5 2 2.5-2 2.5\"></path><path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2z\"></path>",
        categories = "files,development",
        tags = "directory,coding,develop,software",
        contributors = "jguddas,colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderCode,
    #[cfg(feature = "folder_cog")]
    #[strum(props(
        svg = "<path d=\"M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.98a2 2 0 0 1 1.69.9l.66 1.2A2 2 0 0 0 12 6h8a2 2 0 0 1 2 2v3.3\"></path><path d=\"m14.30 19.53.92-.382\"></path><path d=\"m15.22 16.85-.923-.383\"></path><path d=\"m16.85 15.22-.383-.923\"></path><path d=\"m16.85 20.77-.383.92\"></path><path d=\"m19.14 15.22.383-.923\"></path><path d=\"m19.53 21.69-.382-.924\"></path><path d=\"m20.77 16.85.924-.383\"></path><path d=\"m20.77 19.14.924.38\"></path><circle cy=\"18\" r=\"3\" cx=\"18\"></circle>",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    FolderCog,
    #[cfg(feature = "folder_dot")]
    #[strum(props(
        svg = "<path d=\"M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z\"></path><circle cy=\"13\" r=\"1\" cx=\"12\"></circle>",
        categories = "files,development",
        tags = "directory,root,project,pinned,active,current,cogged,gear,mechanical,machinery,configuration,controls,preferences,settings,system,admin,edit",
        contributors = "danielbayley"
    ))]
    FolderDot,
    #[cfg(feature = "folder_down")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"M12 10v6\"></path><path d=\"m15 13-3 3-3-3\"></path>",
        categories = "files,arrows",
        tags = "directory,download,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderDown,
    #[cfg(feature = "folder_git_2")]
    #[strum(props(
        svg = "<path d=\"M18 19a5 5 0 0 1-5-5v8\"></path><path d=\"M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5\"></path><circle cy=\"12\" cx=\"13\" r=\"2\"></circle><circle cx=\"20\" cy=\"19\" r=\"2\"></circle>",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis,jguddas"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"13\" r=\"2\"></circle><path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"M14 13h3\"></path><path d=\"M7 13h3\"></path>",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit,
    #[cfg(feature = "folder_heart")]
    #[strum(props(
        svg = "<path d=\"M10.63 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.41\"></path><path d=\"M14.62 18.8A2.25 2.25 0 1 1 18 15.83a2.25 2.25 0 1 1 3.38 2.96l-2.62 2.85a.998.99 0 0 1-1.50 0z\"></path>",
        categories = "files",
        tags = "directory,heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FolderHeart,
    #[cfg(feature = "folder_input")]
    #[strum(props(
        svg = "<path d=\"M2 9V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1\"></path><path d=\"M2 13h10\"></path><path d=\"m9 16 3-3-3-3\"></path>",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderInput,
    #[cfg(feature = "folder_kanban")]
    #[strum(props(
        svg = "<path d=\"M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z\"></path><path d=\"M8 10v4\"></path><path d=\"M12 10v2\"></path><path d=\"M16 10v6\"></path>",
        categories = "charts,development,design,files",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,directory,project,root",
        contributors = "danielbayley"
    ))]
    FolderKanban,
    #[cfg(feature = "folder_key")]
    #[strum(props(
        svg = "<path d=\"M13 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1.36\"></path><path d=\"M19 12v6\"></path><path d=\"M19 14h2\"></path><circle r=\"2\" cy=\"20\" cx=\"19\"></circle>",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "<rect y=\"17\" x=\"14\" rx=\"1\" height=\"5\" width=\"8\"></rect><path d=\"M10 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v2.5\"></path><path d=\"M20 17v-2a2 2 0 1 0-4 0v2\"></path>",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[cfg(feature = "folder_minus")]
    #[strum(props(
        svg = "<path d=\"M9 13h6\"></path><path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path>",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[cfg(feature = "folder_open_dot")]
    #[strum(props(
        svg = "<path d=\"m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2\"></path><circle cy=\"15\" cx=\"14\" r=\"1\"></circle>",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = "danielbayley"
    ))]
    FolderOpenDot,
    #[cfg(feature = "folder_open")]
    #[strum(props(
        svg = "<path d=\"m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2\"></path>",
        categories = "files",
        tags = "directory",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FolderOpen,
    #[cfg(feature = "folder_output")]
    #[strum(props(
        svg = "<path d=\"M2 7.5V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-1.5\"></path><path d=\"M2 13h10\"></path><path d=\"m5 10-3 3 3 3\"></path>",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
    #[cfg(feature = "folder_pen")]
    #[strum(props(
        svg = "<path d=\"M2 11.5V5a2 2 0 0 1 2-2h3.9c.7 0 1.3.3 1.7.9l.8 1.2c.4.6 1 .9 1.7.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5\"></path><path d=\"M11.37 13.62a1 1 0 1 0-3.00-3.00l-5.01 5.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path>",
        categories = "files",
        tags = "directory,rename",
        contributors = "karsa-mistmere"
    ))]
    FolderPen,
    #[cfg(feature = "folder_plus")]
    #[strum(props(
        svg = "<path d=\"M12 10v6\"></path><path d=\"M9 13h6\"></path><path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path>",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[cfg(feature = "folder_root")]
    #[strum(props(
        svg = "<path d=\"M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z\"></path><circle cy=\"13\" r=\"2\" cx=\"12\"></circle><path d=\"M12 15v5\"></path>",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley"
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "<circle r=\"2.5\" cy=\"12.5\" cx=\"11.5\"></circle><path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"M13.3 14.3 15 16\"></path>",
        categories = "files",
        tags = "directory,search,find,lost,browser,lens",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "<path d=\"M10.7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v4.1\"></path><path d=\"m21 21-1.9-1.9\"></path><circle cx=\"17\" r=\"3\" cy=\"17\"></circle>",
        categories = "files",
        tags = "directory,search,find,lost,browser,lens",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSearch,
    #[cfg(feature = "folder_symlink")]
    #[strum(props(
        svg = "<path d=\"M2 9.35V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7\"></path><path d=\"m8 16 3-3-3-3\"></path>",
        categories = "files",
        tags = "directory,symlink,symbolic,link",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSymlink,
    #[cfg(feature = "folder_sync")]
    #[strum(props(
        svg = "<path d=\"M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v.5\"></path><path d=\"M12 10v4h4\"></path><path d=\"m12 14 1.53-1.60a5 5 0 0 1 8 1.5\"></path><path d=\"M22 22v-4h-4\"></path><path d=\"m22 18-1.53 1.60a5 5 0 0 1-8-1.5\"></path>",
        categories = "files,arrows",
        tags = "directory,synchronize,synchronise,refresh,reconnect,transfer,backup",
        contributors = "danielbayley,jguddas"
    ))]
    FolderSync,
    #[cfg(feature = "folder_tree")]
    #[strum(props(
        svg = "<path d=\"M20 10a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z\"></path><path d=\"M20 21a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.9a1 1 0 0 1-.88-.55l-.42-.85a1 1 0 0 0-.92-.6H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z\"></path><path d=\"M3 5a2 2 0 0 0 2 2h3\"></path><path d=\"M3 3v13a2 2 0 0 0 2 2h3\"></path>",
        categories = "files",
        tags = "directory,tree,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderTree,
    #[cfg(feature = "folder_up")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"M12 10v6\"></path><path d=\"m9 13 3-3 3 3\"></path>",
        categories = "files,arrows",
        tags = "directory,upload,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderUp,
    #[cfg(feature = "folder_x")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path><path d=\"m9.5 10.5 5 5\"></path><path d=\"m14.5 10.5-5 5\"></path>",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderX,
    #[cfg(feature = "folder")]
    #[strum(props(
        svg = "<path d=\"M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z\"></path>",
        categories = "files",
        tags = "directory",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Folder,
    #[cfg(feature = "folders")]
    #[strum(props(
        svg = "<path d=\"M20 5a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h2.5a1.5 1.5 0 0 1 1.2.6l.6.8a1.5 1.5 0 0 0 1.2.6z\"></path><path d=\"M3 8.26a2 2 0 0 0-1 1.73V19a2 2 0 0 0 2 2h11a2 2 0 0 0 1.73-1\"></path>",
        categories = "files",
        tags = "multiple,copy,directories",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Folders,
    #[cfg(feature = "footprints")]
    #[strum(props(
        svg = "<path d=\"M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z\"></path><path d=\"M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z\"></path><path d=\"M16 17h4\"></path><path d=\"M4 13h4\"></path>",
        categories = "navigation",
        tags = "steps,walking,foot,feet,trail,shoe",
        contributors = "karsa-mistmere"
    ))]
    Footprints,
    #[cfg(feature = "forklift")]
    #[strum(props(
        svg = "<path d=\"M12 12H5a2 2 0 0 0-2 2v5\"></path><path d=\"M15 19h7\"></path><path d=\"M16 19V2\"></path><path d=\"M6 12V7a2 2 0 0 1 2-2h2.17a2 2 0 0 1 1.41.586l3.82 3.82A2 2 0 0 1 16 10.82\"></path><path d=\"M7 19h4\"></path><circle r=\"2\" cx=\"13\" cy=\"19\"></circle><circle cy=\"19\" r=\"2\" cx=\"5\"></circle>",
        categories = "transportation",
        tags = "machinery,industrial,warehouse,lifting,storage,equipment,heavy-duty,moving,vehicle,transport,logistics",
        contributors = "ericfennis,jguddas"
    ))]
    Forklift,
    #[cfg(feature = "form")]
    #[strum(props(
        svg = "<path d=\"M4 14h6\"></path><path d=\"M4 2h10\"></path><rect height=\"4\" y=\"18\" width=\"16\" x=\"4\" rx=\"1\"></rect><rect height=\"4\" rx=\"1\" x=\"4\" y=\"6\" width=\"16\"></rect>",
        categories = "development",
        tags = "document,page,file,layout,paper,stub,formality,structure,template,inputs,design,components",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Form,
    #[cfg(feature = "forward")]
    #[strum(props(
        svg = "<path d=\"m15 17 5-5-5-5\"></path><path d=\"M4 18v-2a4 4 0 0 1 4-4h12\"></path>",
        categories = "mail",
        tags = "send,share,email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Forward,
    #[cfg(feature = "frame")]
    #[strum(props(
        svg = "<line x1=\"22\" y1=\"6\" x2=\"2\" y2=\"6\"></line><line y1=\"18\" x1=\"22\" y2=\"18\" x2=\"2\"></line><line x1=\"6\" y1=\"2\" x2=\"6\" y2=\"22\"></line><line y2=\"22\" x2=\"18\" y1=\"2\" x1=\"18\"></line>",
        categories = "design,photography",
        tags = "logo,design,tool",
        contributors = "Bowero,ericfennis"
    ))]
    Frame,
    #[cfg(feature = "frown")]
    #[strum(props(
        svg = "<circle r=\"10\" cy=\"12\" cx=\"12\"></circle><path d=\"M16 16s-1.5-2-4-2-4 2-4 2\"></path><line y1=\"9\" y2=\"9\" x1=\"9\" x2=\"9.01\"></line><line y1=\"9\" x1=\"15\" x2=\"15.01\" y2=\"9\"></line>",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "<path d=\"M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 4 0v-6.99a2 2 0 0 0-.59-1.42L18 5\"></path><path d=\"M14 21V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16\"></path><path d=\"M2 21h13\"></path><path d=\"M3 9h11\"></path>",
        categories = "transportation,navigation",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis,UsamaKhan"
    ))]
    Fuel,
    #[cfg(feature = "fullscreen")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><rect width=\"10\" height=\"8\" rx=\"1\" y=\"8\" x=\"7\"></rect>",
        categories = "layout,multimedia,design,photography",
        tags = "expand,zoom,preview,focus,camera,lens,image",
        contributors = "danielbayley"
    ))]
    Fullscreen,
    #[cfg(feature = "funnel_plus")]
    #[strum(props(
        svg = "<path d=\"M13.35 3H3a1 1 0 0 0-.742 1.67l7.22 7.98A2 2 0 0 1 10 14v6a1 1 0 0 0 .553.89l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.34l1.21-1.34\"></path><path d=\"M16 6h6\"></path><path d=\"M19 3v6\"></path>",
        categories = "layout",
        tags = "filter,hopper,add,create,new",
        contributors = "gubser,ericfennis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    FunnelPlus,
    #[cfg(feature = "funnel_x")]
    #[strum(props(
        svg = "<path d=\"M12.53 3H3a1 1 0 0 0-.742 1.67l7.22 7.98A2 2 0 0 1 10 14v6a1 1 0 0 0 .553.89l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.34l.427-.473\"></path><path d=\"m16.5 3.5 5 5\"></path><path d=\"m21.5 3.5-5 5\"></path>",
        categories = "layout",
        tags = "filter,hopper,remove,delete",
        contributors = "gubser,ericfennis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    FunnelX,
    #[cfg(feature = "funnel")]
    #[strum(props(
        svg = "<path d=\"M10 20a1 1 0 0 0 .553.89l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.34L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.22 7.98A2 2 0 0 1 10 14z\"></path>",
        categories = "layout",
        tags = "filter,hopper",
        contributors = "colebemis,lukedukeus,jguddas,karsa-mistmere"
    ))]
    Funnel,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "<path d=\"M2 7v10\"></path><path d=\"M6 5v14\"></path><rect x=\"10\" rx=\"2\" height=\"18\" y=\"3\" width=\"12\"></rect>",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "<path d=\"M2 3v18\"></path><rect x=\"6\" rx=\"2\" height=\"18\" y=\"3\" width=\"12\"></rect><path d=\"M22 3v18\"></path>",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" height=\"14\" width=\"18\" x=\"3\"></rect><path d=\"M4 21h1\"></path><path d=\"M9 21h1\"></path><path d=\"M14 21h1\"></path><path d=\"M19 21h1\"></path>",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "<path d=\"M7 2h10\"></path><path d=\"M5 6h14\"></path><rect rx=\"2\" width=\"18\" height=\"12\" x=\"3\" y=\"10\"></rect>",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "<path d=\"M3 2h18\"></path><rect width=\"18\" rx=\"2\" height=\"12\" y=\"6\" x=\"3\"></rect><path d=\"M3 22h18\"></path>",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "<line y2=\"11\" x1=\"6\" x2=\"10\" y1=\"11\"></line><line y2=\"13\" x2=\"8\" x1=\"8\" y1=\"9\"></line><line y2=\"12\" x1=\"15\" y1=\"12\" x2=\"15.01\"></line><line x2=\"18.01\" y1=\"10\" y2=\"10\" x1=\"18\"></line><path d=\"M17.32 5H6.68a4 4 0 0 0-3.97 3.59c-.006.05-.01.10-.017.15C2.60 9.41 2 14.45 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.41-1.41A2 2 0 0 1 9.82 16h4.34a2 2 0 0 1 1.41.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.54-.604-6.58-.685-7.25-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z\"></path>",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad_directional")]
    #[strum(props(
        svg = "<path d=\"M11.14 15.85a1.20 1.20 0 0 1 1.70 0l1.56 1.56A2 2 0 0 1 15 18.82V21a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-2.17a2 2 0 0 1 .586-1.41z\"></path><path d=\"M18.82 15a2 2 0 0 1-1.41-.586l-1.56-1.56a1.20 1.20 0 0 1 0-1.70l1.56-1.56A2 2 0 0 1 18.82 9H21a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1z\"></path><path d=\"M6.58 14.41A2 2 0 0 1 5.17 15H3a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1h2.17a2 2 0 0 1 1.41.586l1.56 1.56a1.20 1.20 0 0 1 0 1.70z\"></path><path d=\"M9 3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2.17a2 2 0 0 1-.586 1.41l-1.56 1.56a1.20 1.20 0 0 1-1.70 0l-1.56-1.56A2 2 0 0 1 9 5.17z\"></path>",
        categories = "gaming,devices",
        tags = "direction,arrow,controller,navigation,button,move,pointer,arrowhead,console,game,gaming",
        contributors = "felipeajzanetti,jguddas"
    ))]
    GamepadDirectional,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "<line x1=\"6\" x2=\"10\" y2=\"12\" y1=\"12\"></line><line x2=\"8\" y1=\"10\" y2=\"14\" x1=\"8\"></line><line x1=\"15\" x2=\"15.01\" y2=\"13\" y1=\"13\"></line><line x2=\"18.01\" x1=\"18\" y2=\"11\" y1=\"11\"></line><rect x=\"2\" rx=\"2\" height=\"12\" y=\"6\" width=\"20\"></rect>",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gauge")]
    #[strum(props(
        svg = "<path d=\"m12 14 4-4\"></path><path d=\"M3.34 19a10 10 0 1 1 17.32 0\"></path>",
        categories = "transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "ericfennis,Andreto,danielbayley,karsa-mistmere"
    ))]
    Gauge,
    #[cfg(feature = "gavel")]
    #[strum(props(
        svg = "<path d=\"m14 13-8.38 8.38a1 1 0 0 1-3.00-3l8.38-8.38\"></path><path d=\"m16 16 6-6\"></path><path d=\"m21.5 10.5-8-8\"></path><path d=\"m8 8 6-6\"></path><path d=\"m8.5 7.5 8 8\"></path>",
        categories = "navigation,tools",
        tags = "justice,law,court,judgment,legal,hands,penalty,decision,authority,hammer,mallet",
        contributors = "Andreto,ericfennis,jguddas,karsa-mistmere"
    ))]
    Gavel,
    #[cfg(feature = "gem")]
    #[strum(props(
        svg = "<path d=\"M10.5 3 8 9l4 13 4-13-2.5-6\"></path><path d=\"M17 3a2 2 0 0 1 1.6.8l3 4a2 2 0 0 1 .013 2.38l-7.99 10.98a2 2 0 0 1-3.24 0l-7.99-10.98A2 2 0 0 1 2.4 7.8l2.99-3.99A2 2 0 0 1 7 3z\"></path><path d=\"M2 9h20\"></path>",
        categories = "gaming,development,finance",
        tags = "diamond,crystal,ruby,jewellery,price,special,present,gift,ring,wedding,proposal,marriage,rubygems",
        contributors = "connium,ericfennis,karsa-mistmere"
    ))]
    Gem,
    #[cfg(feature = "georgian_lari")]
    #[strum(props(
        svg = "<path d=\"M11.5 21a7.5 7.5 0 1 1 7.35-9\"></path><path d=\"M13 12V3\"></path><path d=\"M4 21h16\"></path><path d=\"M9 12V3\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "kivicode"
    ))]
    GeorgianLari,
    #[cfg(feature = "ghost")]
    #[strum(props(
        svg = "<path d=\"M9 10h.01\"></path><path d=\"M15 10h.01\"></path><path d=\"M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z\"></path>",
        categories = "gaming",
        tags = "pac-man,spooky",
        contributors = "mittalyashu,ericfennis"
    ))]
    Ghost,
    #[cfg(feature = "gift")]
    #[strum(props(
        svg = "<path d=\"M12 7v14\"></path><path d=\"M20 11v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8\"></path><path d=\"M7.5 7a1 1 0 0 1 0-5A4.8 8 0 0 1 12 7a4.8 8 0 0 1 4.5-5 1 1 0 0 1 0 5\"></path><rect width=\"18\" y=\"7\" rx=\"1\" height=\"4\" x=\"3\"></rect>",
        categories = "gaming,account",
        tags = "present,box,birthday,party",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Gift,
    #[cfg(feature = "git_branch_minus")]
    #[strum(props(
        svg = "<path d=\"M15 6a9 9 0 0 0-9 9V3\"></path><path d=\"M21 18h-6\"></path><circle cx=\"18\" cy=\"6\" r=\"3\"></circle><circle r=\"3\" cx=\"6\" cy=\"18\"></circle>",
        categories = "development",
        tags = "code,version control,vcs,repository,delete,remove,-",
        contributors = "joris-gallot"
    ))]
    GitBranchMinus,
    #[cfg(feature = "git_branch_plus")]
    #[strum(props(
        svg = "<path d=\"M6 3v12\"></path><path d=\"M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z\"></path><path d=\"M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z\"></path><path d=\"M15 6a9 9 0 0 0-9 9\"></path><path d=\"M18 15v6\"></path><path d=\"M21 18h-6\"></path>",
        categories = "development",
        tags = "code,version control,vcs,repository,add,create,+",
        contributors = "mittalyashu,ericfennis"
    ))]
    GitBranchPlus,
    #[cfg(feature = "git_branch")]
    #[strum(props(
        svg = "<path d=\"M15 6a9 9 0 0 0-9 9V3\"></path><circle cx=\"18\" cy=\"6\" r=\"3\"></circle><circle r=\"3\" cy=\"18\" cx=\"6\"></circle>",
        categories = "development",
        tags = "code,version control,vcs,repository",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit_horizontal")]
    #[strum(props(
        svg = "<circle r=\"3\" cy=\"12\" cx=\"12\"></circle><line x1=\"3\" x2=\"9\" y1=\"12\" y2=\"12\"></line><line x2=\"21\" y2=\"12\" y1=\"12\" x1=\"15\"></line>",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommitHorizontal,
    #[cfg(feature = "git_commit_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 3v6\"></path><circle r=\"3\" cx=\"12\" cy=\"12\"></circle><path d=\"M12 15v6\"></path>",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station",
        contributors = "danielbayley"
    ))]
    GitCommitVertical,
    #[cfg(feature = "git_compare_arrows")]
    #[strum(props(
        svg = "<circle cx=\"5\" cy=\"6\" r=\"3\"></circle><path d=\"M12 6h5a2 2 0 0 1 2 2v7\"></path><path d=\"m15 9-3-3 3-3\"></path><circle cx=\"19\" cy=\"18\" r=\"3\"></circle><path d=\"M12 18H7a2 2 0 0 1-2-2V9\"></path><path d=\"m9 15 3 3-3 3\"></path>",
        categories = "development,arrows",
        tags = "code,version control,diff",
        contributors = "danielbayley"
    ))]
    GitCompareArrows,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "<circle cy=\"18\" cx=\"18\" r=\"3\"></circle><circle r=\"3\" cx=\"6\" cy=\"6\"></circle><path d=\"M13 6h3a2 2 0 0 1 2 2v7\"></path><path d=\"M11 18H8a2 2 0 0 1-2-2V9\"></path>",
        categories = "development",
        tags = "code,version control,diff",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "<circle cy=\"18\" r=\"3\" cx=\"12\"></circle><circle cx=\"6\" cy=\"6\" r=\"3\"></circle><circle cx=\"18\" cy=\"6\" r=\"3\"></circle><path d=\"M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9\"></path><path d=\"M12 12v3\"></path>",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis,danielbayley"
    ))]
    GitFork,
    #[cfg(feature = "git_graph")]
    #[strum(props(
        svg = "<circle cy=\"6\" r=\"3\" cx=\"5\"></circle><path d=\"M5 9v6\"></path><circle cy=\"18\" cx=\"5\" r=\"3\"></circle><path d=\"M12 3v18\"></path><circle r=\"3\" cx=\"19\" cy=\"6\"></circle><path d=\"M16 15.7A9 9 0 0 0 19 9\"></path>",
        categories = "development",
        tags = "code,version control,commit graph,commits,gitlens",
        contributors = "danielbayley"
    ))]
    GitGraph,
    #[cfg(feature = "git_merge_conflict")]
    #[strum(props(
        svg = "<path d=\"M12 6h4a2 2 0 0 1 2 2v7\"></path><path d=\"M6 12v9\"></path><path d=\"M9 3 3 9\"></path><path d=\"M9 9 3 3\"></path><circle cx=\"18\" cy=\"18\" r=\"3\"></circle>",
        categories = "development",
        tags = "code,version control,commits,diff,error,conflict",
        contributors = "timmy471,colebemis,csandman,karsa-mistmere,ericfennis"
    ))]
    GitMergeConflict,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"18\" cy=\"18\"></circle><circle cy=\"6\" r=\"3\" cx=\"6\"></circle><path d=\"M6 21V9a9 9 0 0 0 9 9\"></path>",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_arrow")]
    #[strum(props(
        svg = "<circle cy=\"6\" cx=\"5\" r=\"3\"></circle><path d=\"M5 9v12\"></path><circle cx=\"19\" cy=\"18\" r=\"3\"></circle><path d=\"m15 9-3-3 3-3\"></path><path d=\"M12 6h5a2 2 0 0 1 2 2v7\"></path>",
        categories = "development,arrows",
        tags = "code,version control,open",
        contributors = "danielbayley"
    ))]
    GitPullRequestArrow,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "<circle cy=\"6\" cx=\"6\" r=\"3\"></circle><path d=\"M6 9v12\"></path><path d=\"m21 3-6 6\"></path><path d=\"m21 9-6-6\"></path><path d=\"M18 11.5V15\"></path><circle cy=\"18\" r=\"3\" cx=\"18\"></circle>",
        categories = "development",
        tags = "code,version control,rejected,closed,cancelled,x",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_create_arrow")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"5\" cy=\"6\"></circle><path d=\"M5 9v12\"></path><path d=\"m15 9-3-3 3-3\"></path><path d=\"M12 6h5a2 2 0 0 1 2 2v3\"></path><path d=\"M19 15v6\"></path><path d=\"M22 18h-6\"></path>",
        categories = "development,arrows",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreateArrow,
    #[cfg(feature = "git_pull_request_create")]
    #[strum(props(
        svg = "<circle cx=\"6\" r=\"3\" cy=\"6\"></circle><path d=\"M6 9v12\"></path><path d=\"M13 6h3a2 2 0 0 1 2 2v3\"></path><path d=\"M18 15v6\"></path><path d=\"M21 18h-6\"></path>",
        categories = "development",
        tags = "code,version control,open,plus,add,+",
        contributors = "danielbayley"
    ))]
    GitPullRequestCreate,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "<circle r=\"3\" cy=\"18\" cx=\"18\"></circle><circle r=\"3\" cy=\"6\" cx=\"6\"></circle><path d=\"M18 6V5\"></path><path d=\"M18 11v-1\"></path><line x1=\"6\" y1=\"9\" y2=\"21\" x2=\"6\"></line>",
        categories = "development",
        tags = "code,version control,open,draft,dashed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "<circle cy=\"18\" r=\"3\" cx=\"18\"></circle><circle r=\"3\" cy=\"6\" cx=\"6\"></circle><path d=\"M13 6h3a2 2 0 0 1 2 2v7\"></path><line y1=\"9\" x1=\"6\" y2=\"21\" x2=\"6\"></line>",
        categories = "development",
        tags = "code,version control,open",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitPullRequest,
    #[cfg(feature = "glass_water")]
    #[strum(props(
        svg = "<path d=\"M5.11 4.10A1 1 0 0 1 6.11 3h11.78a1 1 0 0 1 .994 1.10L17.19 20.21A2 2 0 0 1 15.2 22H8.8a2 2 0 0 1-2-1.79z\"></path><path d=\"M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0\"></path>",
        categories = "food-beverage",
        tags = "beverage,drink,glass,water",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GlassWater,
    #[cfg(feature = "glasses")]
    #[strum(props(
        svg = "<circle cy=\"15\" cx=\"6\" r=\"4\"></circle><circle cy=\"15\" r=\"4\" cx=\"18\"></circle><path d=\"M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2\"></path><path d=\"M2.5 13 5 7c.7-1.3 1.4-2 3-2\"></path><path d=\"M21.5 13 19 7c-.7-1.3-1.5-2-3-2\"></path>",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_lock")]
    #[strum(props(
        svg = "<path d=\"M15.68 15A14.5 14.5 0 0 1 12 22a14.5 14.5 0 0 1 0-20 10 10 0 1 0 9.54 13\"></path><path d=\"M2 12h8.5\"></path><path d=\"M20 6V4a2 2 0 1 0-4 0v2\"></path><rect x=\"14\" y=\"6\" rx=\"1\" width=\"8\" height=\"5\"></rect>",
        categories = "security,development,devices",
        tags = "vpn,private,privacy,network,world,browser,security,encryption,protection,connection",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    GlobeLock,
    #[cfg(feature = "globe_off")]
    #[strum(props(
        svg = "<path d=\"M10.11 4.46A14.5 14.5 0 0 1 12 2a10 10 0 0 1 9.31 13.64\"></path><path d=\"M15.55 15.55A14.5 14.5 0 0 1 12 22 10 10 0 0 1 4.92 4.92\"></path><path d=\"M15.89 10.23A14.5 14.5 0 0 0 12 2a10 10 0 0 0-3.64.687\"></path><path d=\"M17.65 12H22\"></path><path d=\"M19.07 19.07A10 10 0 0 1 12 22 14.5 14.5 0 0 1 8.44 8.45\"></path><path d=\"M2 12h10\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "navigation,connectivity,devices",
        tags = "globe,earth,planet,disable,mute,off,hide,avoid,world,browser,language,translate,internet,offline,disconnected,network,connection,no connection,network failure,signal off",
        contributors = "TimNekk,karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    GlobeOff,
    #[cfg(feature = "globe_x")]
    #[strum(props(
        svg = "<path d=\"m16 3 5 5\"></path><path d=\"M2 12h20A10 10 0 1 1 12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 4-10\"></path><path d=\"m21 3-5 5\"></path>",
        categories = "connectivity,devices,navigation",
        tags = "globe,internet,offline,disconnected,network,connection,world,no connection,network failure,signal off",
        contributors = "karsa-mistmere,Muhammad-Aqib-Bashir"
    ))]
    GlobeX,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "<circle r=\"10\" cx=\"12\" cy=\"12\"></circle><path d=\"M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20\"></path><path d=\"M2 12h20\"></path>",
        categories = "navigation",
        tags = "world,browser,language,translate",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Globe,
    #[cfg(feature = "goal")]
    #[strum(props(
        svg = "<path d=\"M12 13V2l8 4-8 4\"></path><path d=\"M20.56 10.22a9 9 0 1 1-12.55-5.29\"></path><path d=\"M8.00 9.99a5 5 0 1 0 8.9 2.02\"></path>",
        categories = "gaming",
        tags = "flag,bullseye",
        contributors = "guillermo-angeles,jguddas"
    ))]
    Goal,
    #[cfg(feature = "gpu")]
    #[strum(props(
        svg = "<path d=\"M2 17h18a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H2\"></path><path d=\"M2 21V3\"></path><path d=\"M7 17v3a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1v-3\"></path><circle cy=\"11\" r=\"2\" cx=\"16\"></circle><circle cx=\"8\" r=\"2\" cy=\"11\"></circle>",
        categories = "devices,gaming",
        tags = "processor,cores,technology,computer,chip,circuit,specs,graphics processing unit,video card,display adapter,gddr,rendering,digital image processing,crypto mining",
        contributors = "xandykati98,karsa-mistmere"
    ))]
    Gpu,
    #[cfg(feature = "graduation_cap")]
    #[strum(props(
        svg = "<path d=\"M21.42 10.92a1 1 0 0 0-.019-1.83L12.83 5.18a2 2 0 0 0-1.66 0L2.6 9.08a1 1 0 0 0 0 1.83l8.57 3.90a2 2 0 0 0 1.66 0z\"></path><path d=\"M22 10v6\"></path><path d=\"M6 12.5V16a6 3 0 0 0 12 0v-3.5\"></path>",
        categories = "buildings",
        tags = "school,university,learn,study,mortarboard,education,ceremony,academic,hat,diploma,bachlor's,master's,doctorate",
        contributors = "Tummerhore,ericfennis,karsa-mistmere,jguddas"
    ))]
    GraduationCap,
    #[cfg(feature = "grape")]
    #[strum(props(
        svg = "<path d=\"M22 5V2l-5.89 5.89\"></path><circle r=\"3\" cx=\"16.6\" cy=\"15.89\"></circle><circle r=\"3\" cx=\"8.11\" cy=\"7.4\"></circle><circle cx=\"12.35\" cy=\"11.65\" r=\"3\"></circle><circle r=\"3\" cy=\"5.85\" cx=\"13.91\"></circle><circle cy=\"10.09\" r=\"3\" cx=\"18.15\"></circle><circle r=\"3\" cy=\"13.2\" cx=\"6.56\"></circle><circle r=\"3\" cy=\"17.44\" cx=\"10.8\"></circle><circle r=\"3\" cx=\"5\" cy=\"19\"></circle>",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2_check")]
    #[strum(props(
        svg = "<path d=\"M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3\"></path><path d=\"m16 19 2 2 4-4\"></path>",
        categories = "text,layout,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,data,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme"
    ))]
    Grid2X2Check,
    #[cfg(feature = "grid_2_x_2_plus")]
    #[strum(props(
        svg = "<path d=\"M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3\"></path><path d=\"M16 19h6\"></path><path d=\"M19 22v-6\"></path>",
        categories = "text,layout,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,data,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme,jguddas"
    ))]
    Grid2X2Plus,
    #[cfg(feature = "grid_2_x_2_x")]
    #[strum(props(
        svg = "<path d=\"M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3\"></path><path d=\"m16 16 5 5\"></path><path d=\"m16 21 5-5\"></path>",
        categories = "text,layout,math",
        tags = "table,rows,columns,data,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley,chessurisme"
    ))]
    Grid2X2X,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "<path d=\"M12 3v18\"></path><path d=\"M3 12h18\"></path><rect x=\"3\" rx=\"2\" height=\"18\" width=\"18\" y=\"3\"></rect>",
        categories = "text,layout,design,math",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre,window,skylight",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_2")]
    #[strum(props(
        svg = "<path d=\"M15 3v18\"></path><path d=\"M3 12h18\"></path><path d=\"M9 3v18\"></path><rect y=\"3\" width=\"18\" rx=\"2\" x=\"3\" height=\"18\"></rect>",
        categories = "text,math,layout,design",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre,window",
        contributors = "qubrat"
    ))]
    Grid3X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "<rect rx=\"2\" y=\"3\" width=\"18\" height=\"18\" x=\"3\"></rect><path d=\"M3 9h18\"></path><path d=\"M3 15h18\"></path><path d=\"M9 3v18\"></path><path d=\"M15 3v18\"></path>",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"9\" r=\"1\"></circle><circle cy=\"9\" r=\"1\" cx=\"19\"></circle><circle cy=\"9\" cx=\"5\" r=\"1\"></circle><circle r=\"1\" cx=\"12\" cy=\"15\"></circle><circle cx=\"19\" r=\"1\" cy=\"15\"></circle><circle cy=\"15\" r=\"1\" cx=\"5\"></circle>",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "<circle cx=\"9\" r=\"1\" cy=\"12\"></circle><circle r=\"1\" cx=\"9\" cy=\"5\"></circle><circle cx=\"9\" cy=\"19\" r=\"1\"></circle><circle r=\"1\" cx=\"15\" cy=\"12\"></circle><circle cy=\"5\" r=\"1\" cx=\"15\"></circle><circle cy=\"19\" r=\"1\" cx=\"15\"></circle>",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"5\" r=\"1\"></circle><circle cy=\"5\" cx=\"19\" r=\"1\"></circle><circle cx=\"5\" cy=\"5\" r=\"1\"></circle><circle cx=\"12\" cy=\"12\" r=\"1\"></circle><circle cy=\"12\" r=\"1\" cx=\"19\"></circle><circle r=\"1\" cx=\"5\" cy=\"12\"></circle><circle cy=\"19\" cx=\"12\" r=\"1\"></circle><circle cx=\"19\" cy=\"19\" r=\"1\"></circle><circle cy=\"19\" cx=\"5\" r=\"1\"></circle>",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "<path d=\"M3 7V5c0-1.1.9-2 2-2h2\"></path><path d=\"M17 3h2c1.1 0 2 .9 2 2v2\"></path><path d=\"M21 17v2c0 1.1-.9 2-2 2h-2\"></path><path d=\"M7 21H5c-1.1 0-2-.9-2-2v-2\"></path><rect y=\"7\" rx=\"1\" width=\"7\" height=\"5\" x=\"7\"></rect><rect rx=\"1\" height=\"5\" x=\"10\" y=\"12\" width=\"7\"></rect>",
        categories = "files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[cfg(feature = "guitar")]
    #[strum(props(
        svg = "<path d=\"m11.9 12.1 4.51-4.51\"></path><path d=\"M20.1 2.3a1 1 0 0 0-1.4 0l-1.11 1.11A2 2 0 0 0 17 4.82v1.34a2 2 0 0 1-.586 1.41A2 2 0 0 1 17.82 7h1.34a2 2 0 0 0 1.41-.586L21.7 5.3a1 1 0 0 0 0-1.4z\"></path><path d=\"m6 16 2 2\"></path><path d=\"M8.23 9.85A3 3 0 0 1 11 8a5 5 0 0 1 5 5 3 3 0 0 1-1.85 2.77l-.92.38A2 2 0 0 0 12 18a4 4 0 0 1-4 4 6 6 0 0 1-6-6 4 4 0 0 1 4-4 2 2 0 0 0 1.85-1.23z\"></path>",
        categories = "multimedia",
        tags = "acoustic,instrument,strings,riff,rock,band,country,concert,performance,play,lead,loud,music,audio,sound,noise",
        contributors = "danielbayley,jguddas"
    ))]
    Guitar,
    #[cfg(feature = "ham")]
    #[strum(props(
        svg = "<path d=\"M13.14 21.14A7.27 10.44 45 1 0 2.85 10.85\"></path><path d=\"M13.14 21.14A7.27 4.36 45 0 0 2.85 10.85a7.27 4.36 45 0 0 10.28 10.28\"></path><path d=\"M16.56 10.43 18.6 8.4a2.50 2.50 0 1 0 1.65-4.65 2.5 2.5 0 1 0-4.66 1.66l-2.02 2.02\"></path><path d=\"m8.5 16.5-1-1\"></path>",
        categories = "food-beverage",
        tags = "food,pork,pig,meat,bone,hock,knuckle,gammon,cured",
        contributors = "karsa-mistmere"
    ))]
    Ham,
    #[cfg(feature = "hamburger")]
    #[strum(props(
        svg = "<path d=\"M12 16H4a2 2 0 1 1 0-4h16a2 2 0 1 1 0 4h-4.25\"></path><path d=\"M5 12a2 2 0 0 1-2-2 9 7 0 0 1 18 0 2 2 0 0 1-2 2\"></path><path d=\"M5 16a2 2 0 0 0-2 2 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 2 2 0 0 0-2-2q0 0 0 0\"></path><path d=\"m6.67 12 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2\"></path>",
        categories = "food-beverage",
        tags = "burger,cheeseburger,meat,beef,patty,bun,fast food,junk food,takeaway,takeout,snack,dish,restaurant,lunch,meal,savory,savoury,cookery,cooking,grilled,barbecue,barbeque,bbq,lettuce,tomato,relish,pickles,onions,ketchup,mustard,mayonnaise",
        contributors = "danielbayley,kemie,karsa-mistmere,jguddas,jamiemlaw"
    ))]
    Hamburger,
    #[cfg(feature = "hammer")]
    #[strum(props(
        svg = "<path d=\"m15 12-9.37 9.37a1 1 0 0 1-3.00-3L12 9\"></path><path d=\"m18 15 4-4\"></path><path d=\"m21.5 11.5-1.91-1.91A2 2 0 0 1 19 8.17v-.344a2 2 0 0 0-.586-1.41l-1.65-1.65A6 6 0 0 0 12.51 3H9l1.24 1.24A6 6 0 0 1 12 8.48V10l2 2h1.17a2 2 0 0 1 1.41.586L18.5 14.5\"></path>",
        categories = "tools,home",
        tags = "mallet,nails,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Hammer,
    #[cfg(feature = "hand_coins")]
    #[strum(props(
        svg = "<path d=\"M11 15h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 17\"></path><path d=\"m7 21 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9\"></path><path d=\"m2 16 6 6\"></path><circle cy=\"9\" cx=\"16\" r=\"2.9\"></circle><circle r=\"3\" cy=\"5\" cx=\"6\"></circle>",
        categories = "finance,account",
        tags = "savings,banking,money,finance,offers,mortgage,payment,received,wage,payroll,allowance,pocket money,handout,pennies",
        contributors = "danielbayley,kayleyhill"
    ))]
    HandCoins,
    #[cfg(feature = "hand_fist")]
    #[strum(props(
        svg = "<path d=\"M12.03 17.01a3 3 0 0 0-3-3l-.311-.002a.72.72 0 0 1-.505-1.22l1.19-1.19A2 2 0 0 1 10.82 11H12a2 2 0 0 0 0-4H9.24a3 3 0 0 0-2.12.879l-2.70 2.70A4.83 4.83 0 0 0 3 14a8 8 0 0 0 8 8h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v2a2 2 0 1 0 4 0\"></path><path d=\"M13.88 9.66A2 2 0 0 0 17 8V5A2 2 0 1 0 13 5\"></path><path d=\"M9 5A2 2 0 1 0 5 5V10\"></path><path d=\"M9 7V4A2 2 0 1 1 13 4V7.26\"></path>",
        categories = "social,emoji,communication,sports",
        tags = "clench,strength,power,unity,solidarity,rebellion,victory,triumph,support,fight,combat,brawl",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    HandFist,
    #[cfg(feature = "hand_grab")]
    #[strum(props(
        svg = "<path d=\"M18 11.5V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1.4\"></path><path d=\"M14 10V8a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2\"></path><path d=\"M10 9.9V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v5\"></path><path d=\"M6 14a2 2 0 0 0-2-2a2 2 0 0 0-2 2\"></path><path d=\"M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0\"></path>",
        categories = "cursors,design,layout",
        tags = "hand",
        contributors = "ericfennis"
    ))]
    HandGrab,
    #[cfg(feature = "hand_heart")]
    #[strum(props(
        svg = "<path d=\"M11 14h2a2 2 0 0 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 16\"></path><path d=\"m14.45 13.39 5.05-4.69C20.19 8 21 6.85 21 5.75a2.75 2.75 0 0 0-4.79-1.83.276.27 0 0 1-.406 0A2.75 2.75 0 0 0 11 5.75c0 1.2.80 2.24 1.5 2.94L16 11.95\"></path><path d=\"m2 15 6 6\"></path><path d=\"m7 20 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a1 1 0 0 0-2.75-2.91\"></path>",
        categories = "social",
        tags = "love,like,emotion",
        contributors = "danielbayley,kayleyhill,karsa-mistmere"
    ))]
    HandHeart,
    #[cfg(feature = "hand_helping")]
    #[strum(props(
        svg = "<path d=\"M11 12h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 14\"></path><path d=\"m7 18 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9\"></path><path d=\"m2 13 6 6\"></path>",
        categories = "emoji",
        tags = "agreement,help,proposal,charity,begging,terms",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    HandHelping,
    #[cfg(feature = "hand_metal")]
    #[strum(props(
        svg = "<path d=\"M18 12.5V10a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1.4\"></path><path d=\"M14 11V9a2 2 0 1 0-4 0v2\"></path><path d=\"M10 10.5V5a2 2 0 1 0-4 0v9\"></path><path d=\"m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5\"></path>",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[cfg(feature = "hand_platter")]
    #[strum(props(
        svg = "<path d=\"M12 3V2\"></path><path d=\"m15.4 17.4 3.2-2.8a2 2 0 1 1 2.8 2.9l-3.6 3.3c-.7.8-1.7 1.2-2.8 1.2h-4c-1.1 0-2.1-.4-2.8-1.2l-1.30-1.46A1 1 0 0 0 6.15 19H5\"></path><path d=\"M2 14h12a2 2 0 0 1 0 4h-2\"></path><path d=\"M4 10h16\"></path><path d=\"M5 10a7 7 0 0 1 14 0\"></path><path d=\"M5 14v6a1 1 0 0 1-1 1H2\"></path>",
        categories = "food-beverage,people",
        tags = "waiter,waitress,restaurant,table service,served,dinner,dining,meal,course,luxury",
        contributors = "danielbayley"
    ))]
    HandPlatter,
    #[cfg(feature = "hand")]
    #[strum(props(
        svg = "<path d=\"M18 11V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2\"></path><path d=\"M14 10V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2\"></path><path d=\"M10 10.5V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2v8\"></path><path d=\"M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15\"></path>",
        categories = "cursors,accessibility",
        tags = "wave,move,mouse,grab",
        contributors = "ericfennis"
    ))]
    Hand,
    #[cfg(feature = "handbag")]
    #[strum(props(
        svg = "<path d=\"M2.04 18.56A2 2 0 0 0 4 21h16a2 2 0 0 0 1.95-2.43l-2-9A2 2 0 0 0 18 8H6a2 2 0 0 0-1.95 1.56z\"></path><path d=\"M8 11V6a4 4 0 0 1 8 0v5\"></path>",
        categories = "shopping,transportation",
        tags = "bag,baggage,carry,clutch,fashion,luggage,purse,tote,travel",
        contributors = "jamiemlaw,karsa-mistmere"
    ))]
    Handbag,
    #[cfg(feature = "handshake")]
    #[strum(props(
        svg = "<path d=\"m11 17 2 2a1 1 0 1 0 3-3\"></path><path d=\"m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4\"></path><path d=\"m21 3 1 11h-2\"></path><path d=\"M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3\"></path><path d=\"M3 4h8\"></path>",
        categories = "account,social,communication,finance,security",
        tags = "agreement,partnership,deal,business,assistance,cooperation,friendship,union,terms",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Handshake,
    #[cfg(feature = "hard_drive_download")]
    #[strum(props(
        svg = "<path d=\"M12 2v8\"></path><path d=\"m16 6-4 4-4-4\"></path><rect width=\"20\" height=\"8\" x=\"2\" y=\"14\" rx=\"2\"></rect><path d=\"M6 18h.01\"></path><path d=\"M10 18h.01\"></path>",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "<path d=\"m16 6-4-4-4 4\"></path><path d=\"M12 2v8\"></path><rect y=\"14\" width=\"20\" rx=\"2\" height=\"8\" x=\"2\"></rect><path d=\"M6 18h.01\"></path><path d=\"M10 18h.01\"></path>",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "<path d=\"M10 16h.01\"></path><path d=\"M2.21 11.57a2 2 0 0 0-.212.89V18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5.52a2 2 0 0 0-.212-.896L18.55 5.11A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\"></path><path d=\"M21.94 12.01H2.05\"></path><path d=\"M6 16h.01\"></path>",
        categories = "development,devices",
        tags = "computer,server,memory,data,ssd,disk,hard disk,storage,hardware,backup,media",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    HardDrive,
    #[cfg(feature = "hard_hat")]
    #[strum(props(
        svg = "<path d=\"M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5\"></path><path d=\"M14 6a6 6 0 0 1 6 6v3\"></path><path d=\"M4 15v-3a6 6 0 0 1 6-6\"></path><rect y=\"15\" x=\"2\" width=\"20\" height=\"4\" rx=\"1\"></rect>",
        categories = "tools",
        tags = "helmet,construction,safety,savety",
        contributors = "Andreto,ericfennis"
    ))]
    HardHat,
    #[cfg(feature = "hash")]
    #[strum(props(
        svg = "<line x2=\"20\" y1=\"9\" y2=\"9\" x1=\"4\"></line><line x2=\"20\" y1=\"15\" y2=\"15\" x1=\"4\"></line><line y1=\"3\" x2=\"8\" y2=\"21\" x1=\"10\"></line><line x1=\"16\" y1=\"3\" x2=\"14\" y2=\"21\"></line>",
        categories = "text,social",
        tags = "hashtag,number,pound",
        contributors = "colebemis,ericfennis"
    ))]
    Hash,
    #[cfg(feature = "hat_glasses")]
    #[strum(props(
        svg = "<path d=\"M14 18a2 2 0 0 0-4 0\"></path><path d=\"m19 11-2.11-6.65a2 2 0 0 0-2.75-1.14l-1.27.61A2 2 0 0 1 12 4H8.5a2 2 0 0 0-1.92 1.45L5 11\"></path><path d=\"M2 11h20\"></path><circle r=\"3\" cy=\"18\" cx=\"17\"></circle><circle cx=\"7\" r=\"3\" cy=\"18\"></circle>",
        categories = "social,account,security",
        tags = "incognito,disguise,costume,masked,anonymous,anonymity,privacy,private browsing,stealth,hidden,undercover,cloak,invisible,ghost,spy,agent,detective,identity,cap,fedora,spectacles,shades,sunglasses,eyewear",
        contributors = "karsa-mistmere"
    ))]
    HatGlasses,
    #[cfg(feature = "haze")]
    #[strum(props(
        svg = "<path d=\"m5.2 6.2 1.4 1.4\"></path><path d=\"M2 13h2\"></path><path d=\"M20 13h2\"></path><path d=\"m17.4 7.6 1.4-1.4\"></path><path d=\"M22 17H2\"></path><path d=\"M22 21H2\"></path><path d=\"M16 13a4 4 0 0 0-8 0\"></path><path d=\"M12 5V2.5\"></path>",
        categories = "weather",
        tags = "mist,fog",
        contributors = "ericfennis"
    ))]
    Haze,
    #[cfg(feature = "hd")]
    #[strum(props(
        svg = "<path d=\"M10 12H6\"></path><path d=\"M10 15V9\"></path><path d=\"M14 14.5a.5.5 0 0 0 .5.5h1a2.5 2.5 0 0 0 2.5-2.5v-1A2.5 2.5 0 0 0 15.5 9h-1a.5.5 0 0 0-.5.5z\"></path><path d=\"M6 15V9\"></path><rect width=\"20\" x=\"2\" rx=\"2\" height=\"14\" y=\"5\"></rect>",
        categories = "devices,multimedia",
        tags = "tv,resolution,video,high definition,720p,1080p",
        contributors = "ahtohbi4,jamiemlaw,karsa-mistmere,jguddas"
    ))]
    Hd,
    #[cfg(feature = "hdmi_port")]
    #[strum(props(
        svg = "<path d=\"M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z\"></path><path d=\"M7.5 12h9\"></path>",
        categories = "devices,multimedia,gaming",
        tags = "socket,plug,slot,controller,connector,interface,console,signal,audio,video,visual,av,data,input,output",
        contributors = "danielbayley"
    ))]
    HdmiPort,
    #[cfg(feature = "heading_1")]
    #[strum(props(
        svg = "<path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path><path d=\"M12 18V6\"></path><path d=\"m17 12 3-2v8\"></path>",
        categories = "text",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading1,
    #[cfg(feature = "heading_2")]
    #[strum(props(
        svg = "<path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path><path d=\"M12 18V6\"></path><path d=\"M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1\"></path>",
        categories = "text",
        tags = "h2,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading2,
    #[cfg(feature = "heading_3")]
    #[strum(props(
        svg = "<path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path><path d=\"M12 18V6\"></path><path d=\"M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2\"></path><path d=\"M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2\"></path>",
        categories = "text",
        tags = "h3,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading3,
    #[cfg(feature = "heading_4")]
    #[strum(props(
        svg = "<path d=\"M12 18V6\"></path><path d=\"M17 10v3a1 1 0 0 0 1 1h3\"></path><path d=\"M21 10v8\"></path><path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path>",
        categories = "text",
        tags = "h4,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading4,
    #[cfg(feature = "heading_5")]
    #[strum(props(
        svg = "<path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path><path d=\"M12 18V6\"></path><path d=\"M17 13v-3h4\"></path><path d=\"M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17\"></path>",
        categories = "text",
        tags = "h5,html,markup,markdown",
        contributors = "ericfennis,jguddas"
    ))]
    Heading5,
    #[cfg(feature = "heading_6")]
    #[strum(props(
        svg = "<path d=\"M4 12h8\"></path><path d=\"M4 18V6\"></path><path d=\"M12 18V6\"></path><circle r=\"2\" cy=\"16\" cx=\"19\"></circle><path d=\"M20 10c-2 2-3 3.5-3 6\"></path>",
        categories = "text",
        tags = "h6,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading6,
    #[cfg(feature = "heading")]
    #[strum(props(
        svg = "<path d=\"M6 12h12\"></path><path d=\"M6 20V4\"></path><path d=\"M18 20V4\"></path>",
        categories = "text",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading,
    #[cfg(feature = "headphone_off")]
    #[strum(props(
        svg = "<path d=\"M21 14h-1.34\"></path><path d=\"M9.12 3.47A9 9 0 0 1 21 12v3.34\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20.41 20.41A2 2 0 0 1 19 21h-1a2 2 0 0 1-2-2v-3\"></path><path d=\"M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 2.63-6.36\"></path>",
        categories = "multimedia,connectivity,communication,devices,gaming",
        tags = "music,audio,sound,mute,off",
        contributors = "colebemis,csandman,ericfennis,jguddas,Need-an-AwP"
    ))]
    HeadphoneOff,
    #[cfg(feature = "headphones")]
    #[strum(props(
        svg = "<path d=\"M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3\"></path>",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Headphones,
    #[cfg(feature = "headset")]
    #[strum(props(
        svg = "<path d=\"M3 11h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5Zm0 0a9 9 0 1 1 18 0m0 0v5a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3Z\"></path><path d=\"M21 16v2a4 4 0 0 1-4 4h-5\"></path>",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound,gaming,headphones,headset,call,center,phone,telephone,voip,video",
        contributors = "ericfennis"
    ))]
    Headset,
    #[cfg(feature = "heart_crack")]
    #[strum(props(
        svg = "<path d=\"M12.40 5.82c-.702.79-1.15 1.49-1.41 2.16l2.15 2.15a.5.5 0 0 1 0 .707l-2.29 2.29a.5.5 0 0 0 0 .707L12 15\"></path><path d=\"M13.50 20.31a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.59-3.67.6.6 0 0 0 .818.00A5.5 5.5 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5z\"></path>",
        categories = "emoji",
        tags = "heartbreak,sadness,emotion",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartCrack,
    #[cfg(feature = "heart_handshake")]
    #[strum(props(
        svg = "<path d=\"M19.41 14.41C21 12.82 22 11.5 22 9.5a5.5 5.5 0 0 0-9.59-3.67.6.6 0 0 1-.818.00A5.5 5.5 0 0 0 2 9.5c0 2.3 1.5 4 3 5.5l5.53 5.36a2 2 0 0 0 2.87.052 2.12 2.12 0 0 0-.004-3 2.12 2.12 0 1 0 3-3 2.12 2.12 0 0 0 3.00 0 2 2 0 0 0 0-2.82l-1.88-1.88a2.41 2.41 0 0 0-3.40 0l-1.71 1.71a2 2 0 0 1-2.82 0 2 2 0 0 1 0-2.82l2.82-2.76\"></path>",
        categories = "emoji,account,security",
        tags = "agreement,charity,help,deal,terms,emotion,together,handshake",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartHandshake,
    #[cfg(feature = "heart_minus")]
    #[strum(props(
        svg = "<path d=\"m14.87 18.99-1.36 1.32a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.59-3.67.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5a5.2 5.2 0 0 1-.244 1.57\"></path><path d=\"M15 15h6\"></path>",
        categories = "medical,account,multimedia,gaming,social",
        tags = "unlike,unfavorite,remove,delete,damage",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,Ayberkyvs,jguddas"
    ))]
    HeartMinus,
    #[cfg(feature = "heart_off")]
    #[strum(props(
        svg = "<path d=\"M10.5 4.89a5.5 5.5 0 0 1 1.09.931.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 1.87-1.00 3.35-2.18 4.65\"></path><path d=\"m16.96 16.96-3.45 3.34a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 2.74-4.76\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "social,multimedia",
        tags = "unlike,dislike,hate,emotion",
        contributors = "karsa-mistmere,ericfennis,danielbayley,jguddas"
    ))]
    HeartOff,
    #[cfg(feature = "heart_plus")]
    #[strum(props(
        svg = "<path d=\"m14.47 19.37-.971.93a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.59-3.67.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5a5.2 5.2 0 0 1-.219 1.49\"></path><path d=\"M15 15h6\"></path><path d=\"M18 12v6\"></path>",
        categories = "medical,account,multimedia,gaming,social",
        tags = "plus,like,favorite,add,health,support",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas,Ayberkyvs,UsamaKhan"
    ))]
    HeartPlus,
    #[cfg(feature = "heart_pulse")]
    #[strum(props(
        svg = "<path d=\"M2 9.5a5.5 5.5 0 0 1 9.59-3.67.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.49 5.31a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5\"></path><path d=\"M3.22 13H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27\"></path>",
        categories = "medical",
        tags = "heartbeat,pulse,health,medical,blood pressure,cardiac,systole,diastole",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    HeartPulse,
    #[cfg(feature = "heart")]
    #[strum(props(
        svg = "<path d=\"M2 9.5a5.5 5.5 0 0 1 9.59-3.67.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.49 5.31a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5\"></path>",
        categories = "medical,social,multimedia,emoji,gaming,shapes",
        tags = "like,love,emotion,suit,playing,cards",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Heart,
    #[cfg(feature = "heater")]
    #[strum(props(
        svg = "<path d=\"M11 8c2-3-2-3 0-6\"></path><path d=\"M15.5 8c2-3-2-3 0-6\"></path><path d=\"M6 10h.01\"></path><path d=\"M6 14h.01\"></path><path d=\"M10 16v-4\"></path><path d=\"M14 16v-4\"></path><path d=\"M18 16v-4\"></path><path d=\"M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3\"></path><path d=\"M5 20v2\"></path><path d=\"M19 20v2\"></path>",
        categories = "home,devices,travel",
        tags = "heating,warmth,comfort,fire,stove,electric,electronics,amenities",
        contributors = "danielbayley"
    ))]
    Heater,
    #[cfg(feature = "helicopter")]
    #[strum(props(
        svg = "<path d=\"M11 17v4\"></path><path d=\"M14 3v8a2 2 0 0 0 2 2h5.86\"></path><path d=\"M17 17v4\"></path><path d=\"M18 17a4 4 0 0 0 4-4 8 6 0 0 0-8-6 6 5 0 0 0-6 5v3a2 2 0 0 0 2 2z\"></path><path d=\"M2 10v5\"></path><path d=\"M6 3h16\"></path><path d=\"M7 21h14\"></path><path d=\"M8 13H2\"></path>",
        categories = "transportation,travel",
        tags = "transport,flying,rotor,aviation,helipad,gear,flyer,technology,helicopter,aircraft,vehicle",
        contributors = "liloudreams,ericfennis,jamiemlaw"
    ))]
    Helicopter,
    #[cfg(feature = "hexagon")]
    #[strum(props(
        svg = "<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\"></path>",
        categories = "shapes,development",
        tags = "shape,node.js,logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Hexagon,
    #[cfg(feature = "highlighter")]
    #[strum(props(
        svg = "<path d=\"m9 11-6 6v3h9l3-3\"></path><path d=\"m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4\"></path>",
        categories = "text,design",
        tags = "mark,text",
        contributors = "lscheibel,Andreto,ericfennis"
    ))]
    Highlighter,
    #[cfg(feature = "history")]
    #[strum(props(
        svg = "<path d=\"M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8\"></path><path d=\"M3 3v5h5\"></path><path d=\"M12 7v5l4 2\"></path>",
        categories = "arrows,time",
        tags = "time,redo,undo,rewind,timeline,version,time machine,backup,rotate,ccw",
        contributors = "ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    History,
    #[cfg(feature = "hop_off")]
    #[strum(props(
        svg = "<path d=\"M10.82 16.12c1.69.6 3.91.79 5.18.85.28.01.53-.09.7-.27\"></path><path d=\"M11.14 20.57c.52.24 2.44 1.12 4.08 1.37.46.06.86-.25.9-.71.12-1.52-.3-3.43-.5-4.28\"></path><path d=\"M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .7-.26\"></path><path d=\"M17.99 5.52a20.83 20.83 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-1.17.1-2.5.02-3.9-.25\"></path><path d=\"M20.57 11.14c.24.52 1.12 2.44 1.37 4.08.04.3-.08.59-.31.75\"></path><path d=\"M4.93 4.93a10 10 0 0 0-.67 13.4c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.85.85 0 0 0 .48-.24\"></path><path d=\"M5.52 17.99c1.05.95 2.91 2.42 4.5 3.15a.8.8 0 0 0 1.13-.68c.2-2.34-.33-5.3-1.57-8.28\"></path><path d=\"M8.35 2.68a10 10 0 0 1 9.98 1.58c.43.35.4.96-.12 1.17-1.5.6-4.3.98-6.07 1.05\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "food-beverage",
        tags = "beer,brewery,drink,hop free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    HopOff,
    #[cfg(feature = "hop")]
    #[strum(props(
        svg = "<path d=\"M10.82 16.12c1.69.6 3.91.79 5.18.85.55.03 1-.42.97-.97-.06-1.27-.26-3.5-.85-5.18\"></path><path d=\"M11.5 6.5c1.64 0 5-.38 6.71-1.07.52-.2.55-.82.12-1.17A10 10 0 0 0 4.26 18.33c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.88.88 0 0 0 .73-.74c.3-2.14-.15-3.5-.61-4.88\"></path><path d=\"M15.62 16.95c.2.85.62 2.76.5 4.28a.77.77 0 0 1-.9.7 16.64 16.64 0 0 1-4.08-1.36\"></path><path d=\"M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .96-.96 17.68 17.68 0 0 0-.9-4.87\"></path><path d=\"M16.94 15.62c.86.2 2.77.62 4.29.5a.77.77 0 0 0 .7-.9 16.64 16.64 0 0 0-1.36-4.08\"></path><path d=\"M17.99 5.52a20.82 20.82 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-2.33.2-5.3-.32-8.27-1.57\"></path><path d=\"M4.93 4.93 3 3a.7.7 0 0 1 0-1\"></path><path d=\"M9.58 12.18c1.24 2.98 1.77 5.95 1.57 8.28a.8.8 0 0 1-1.13.68 20.82 20.82 0 0 1-4.5-3.15\"></path>",
        categories = "food-beverage",
        tags = "beer,brewery,drink",
        contributors = "karsa-mistmere"
    ))]
    Hop,
    #[cfg(feature = "hospital")]
    #[strum(props(
        svg = "<path d=\"M12 7v4\"></path><path d=\"M14 21v-3a2 2 0 0 0-4 0v3\"></path><path d=\"M14 9h-4\"></path><path d=\"M18 11h2a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2\"></path><path d=\"M18 21V5a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16\"></path>",
        categories = "medical,buildings,navigation,travel",
        tags = "infirmary,sanatorium,healthcare,doctor,hospice,clinic,emergency room,ward,building,medical,vet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Hospital,
    #[cfg(feature = "hotel")]
    #[strum(props(
        svg = "<path d=\"M10 22v-6.57\"></path><path d=\"M12 11h.01\"></path><path d=\"M12 7h.01\"></path><path d=\"M14 15.43V22\"></path><path d=\"M15 16a5 5 0 0 0-6 0\"></path><path d=\"M16 11h.01\"></path><path d=\"M16 7h.01\"></path><path d=\"M8 11h.01\"></path><path d=\"M8 7h.01\"></path><rect height=\"20\" x=\"4\" rx=\"2\" y=\"2\" width=\"16\"></rect>",
        categories = "buildings,navigation,travel",
        tags = "building,hostel,motel,inn",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Hotel,
    #[cfg(feature = "hourglass")]
    #[strum(props(
        svg = "<path d=\"M5 22h14\"></path><path d=\"M5 2h14\"></path><path d=\"M17 22v-4.17a2 2 0 0 0-.586-1.41L12 12l-4.41 4.41A2 2 0 0 0 7 17.82V22\"></path><path d=\"M7 2v4.17a2 2 0 0 0 .586 1.41L12 12l4.41-4.41A2 2 0 0 0 17 6.17V2\"></path>",
        categories = "time,gaming",
        tags = "timer,time,sandglass",
        contributors = "karsa-mistmere"
    ))]
    Hourglass,
    #[cfg(feature = "house_heart")]
    #[strum(props(
        svg = "<path d=\"M8.62 13.8A2.25 2.25 0 1 1 12 10.83a2.25 2.25 0 1 1 3.38 2.96l-2.62 2.85a.998.99 0 0 1-1.50 0z\"></path><path d=\"M3 10a2 2 0 0 1 .709-1.52l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"></path>",
        categories = "home,buildings,medical",
        tags = "home sweet home,abode,building,residence,healthy living,lifestyle",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    HouseHeart,
    #[cfg(feature = "house_plug")]
    #[strum(props(
        svg = "<path d=\"M10 12V8.96\"></path><path d=\"M14 12V8.96\"></path><path d=\"M15 12a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-2a1 1 0 0 1 1-1z\"></path><path d=\"M8.5 21H5a2 2 0 0 1-2-2v-9a2 2 0 0 1 .709-1.52l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2h-5a2 2 0 0 1-2-2v-2\"></path>",
        categories = "buildings,home,sustainability",
        tags = "home,living,building,residence,architecture,autarky,energy",
        contributors = "jguddas,karsa-mistmere"
    ))]
    HousePlug,
    #[cfg(feature = "house_plus")]
    #[strum(props(
        svg = "<path d=\"M12.35 21H5a2 2 0 0 1-2-2v-9a2 2 0 0 1 .71-1.53l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v2.35\"></path><path d=\"M14.8 12.4A1 1 0 0 0 14 12h-4a1 1 0 0 0-1 1v8\"></path><path d=\"M15 18h6\"></path><path d=\"M18 15v6\"></path>",
        categories = "buildings,medical",
        tags = "home,living,medical,new,addition,building,residence,architecture",
        contributors = "karsa-mistmere,jguddas"
    ))]
    HousePlus,
    #[cfg(feature = "house_wifi")]
    #[strum(props(
        svg = "<path d=\"M9.5 13.86a4 4 0 0 1 5 .01\"></path><path d=\"M12 17h.01\"></path><path d=\"M3 10a2 2 0 0 1 .709-1.52l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"></path><path d=\"M7 10.75a8 8 0 0 1 10 0\"></path>",
        categories = "home,buildings,connectivity",
        tags = "home,living,building,wifi,connectivity",
        contributors = "akshaymemane,jguddas,karsa-mistmere"
    ))]
    HouseWifi,
    #[cfg(feature = "house")]
    #[strum(props(
        svg = "<path d=\"M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8\"></path><path d=\"M3 10a2 2 0 0 1 .709-1.52l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"></path>",
        categories = "buildings,home,navigation",
        tags = "home,living,building,residence,architecture",
        contributors = "jguddas,karsa-mistmere"
    ))]
    House,
    #[cfg(feature = "ice_cream_bowl")]
    #[strum(props(
        svg = "<path d=\"M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6m-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0\"></path><path d=\"M12.14 11a3.5 3.5 0 1 1 6.71 0\"></path><path d=\"M15.5 6.5a3.5 3.5 0 1 0-7 0\"></path>",
        categories = "food-beverage",
        tags = "gelato,food,dessert,dish,restaurant,course,meal",
        contributors = "kemie,jguddas,karsa-mistmere,ericfennis"
    ))]
    IceCreamBowl,
    #[cfg(feature = "ice_cream_cone")]
    #[strum(props(
        svg = "<path d=\"m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11\"></path><path d=\"M17 7A5 5 0 0 0 7 7\"></path><path d=\"M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4\"></path>",
        categories = "food-beverage",
        tags = "gelato,food",
        contributors = "karsa-mistmere"
    ))]
    IceCreamCone,
    #[cfg(feature = "id_card_lanyard")]
    #[strum(props(
        svg = "<path d=\"M13.5 8h-3\"></path><path d=\"m15 2-1 2h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h3\"></path><path d=\"M16.89 22A5 5 0 0 0 7.1 22\"></path><path d=\"m9 2 3 6\"></path><circle cx=\"12\" cy=\"15\" r=\"3\"></circle>",
        categories = "security,account",
        tags = "id-card,id-card-lanyard,identity,employee,gate-pass,badge",
        contributors = "python2911,UsamaKhan,jguddas"
    ))]
    IdCardLanyard,
    #[cfg(feature = "id_card")]
    #[strum(props(
        svg = "<path d=\"M16 10h2\"></path><path d=\"M16 14h2\"></path><path d=\"M6.17 15a3 3 0 0 1 5.66 0\"></path><circle cx=\"9\" cy=\"11\" r=\"2\"></circle><rect x=\"2\" height=\"14\" y=\"5\" width=\"20\" rx=\"2\"></rect>",
        categories = "security,account",
        tags = "card,badge,identity,authentication,secure",
        contributors = "jguddas,karsa-mistmere"
    ))]
    IdCard,
    #[cfg(feature = "image_down")]
    #[strum(props(
        svg = "<path d=\"M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.81.014L6 21\"></path><path d=\"m14 19 3 3v-5.5\"></path><path d=\"m17 22 3-3\"></path><circle cx=\"9\" cy=\"9\" r=\"2\"></circle>",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,download,save,export",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImageDown,
    #[cfg(feature = "image_minus")]
    #[strum(props(
        svg = "<path d=\"M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7\"></path><line x2=\"22\" x1=\"16\" y1=\"5\" y2=\"5\"></line><circle r=\"2\" cy=\"9\" cx=\"9\"></circle><path d=\"m21 15-3.08-3.08a2 2 0 0 0-2.82 0L6 21\"></path>",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "<line y2=\"22\" x1=\"2\" x2=\"22\" y1=\"2\"></line><path d=\"M10.41 10.41a2 2 0 1 1-2.83-2.83\"></path><line x1=\"13.5\" x2=\"6\" y1=\"13.5\" y2=\"21\"></line><line y2=\"15\" x1=\"18\" y1=\"12\" x2=\"21\"></line><path d=\"M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.05-.22 1.41-.59\"></path><path d=\"M21 15V5a2 2 0 0 0-2-2H9\"></path>",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_play")]
    #[strum(props(
        svg = "<path d=\"M15 15.00a1 1 0 0 1 1.51-.859l4.99 2.99a1 1 0 0 1 0 1.71l-4.99 2.99a1 1 0 0 1-1.51-.86z\"></path><path d=\"M21 12.17V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6\"></path><path d=\"m6 21 5-5\"></path><circle r=\"2\" cy=\"9\" cx=\"9\"></circle>",
        categories = "photography,text,multimedia,files",
        tags = "picture,gif,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImagePlay,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "<path d=\"M16 5h6\"></path><path d=\"M19 2v6\"></path><path d=\"M21 11.5V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7.5\"></path><path d=\"m21 15-3.08-3.08a2 2 0 0 0-2.82 0L6 21\"></path><circle cy=\"9\" cx=\"9\" r=\"2\"></circle>",
        categories = "photography,multimedia,files",
        tags = "add,create,picture",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    ImagePlus,
    #[cfg(feature = "image_up")]
    #[strum(props(
        svg = "<path d=\"M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.81.014L6 21\"></path><path d=\"m14 19.5 3-3 3 3\"></path><path d=\"M17 22v-5.5\"></path><circle cy=\"9\" r=\"2\" cx=\"9\"></circle>",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,upload,import",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    ImageUp,
    #[cfg(feature = "image_upscale")]
    #[strum(props(
        svg = "<path d=\"M16 3h5v5\"></path><path d=\"M17 21h2a2 2 0 0 0 2-2\"></path><path d=\"M21 12v3\"></path><path d=\"m21 3-5 5\"></path><path d=\"M3 7V5a2 2 0 0 1 2-2\"></path><path d=\"m5 21 4.14-4.14a1.21 1.21 0 0 1 1.71 0L13 19\"></path><path d=\"M9 3h3\"></path><rect width=\"10\" y=\"11\" height=\"10\" rx=\"1\" x=\"3\"></rect>",
        categories = "photography,multimedia",
        tags = "resize,picture,sharpen,increase",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,jguddas"
    ))]
    ImageUpscale,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "<rect ry=\"2\" y=\"3\" rx=\"2\" height=\"18\" width=\"18\" x=\"3\"></rect><circle cy=\"9\" r=\"2\" cx=\"9\"></circle><path d=\"m21 15-3.08-3.08a2 2 0 0 0-2.82 0L6 21\"></path>",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Image,
    #[cfg(feature = "images")]
    #[strum(props(
        svg = "<path d=\"m22 11-1.29-1.29a2.4 2.4 0 0 0-3.40 0L11 16\"></path><path d=\"M4 8a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2\"></path><circle fill=\"currentColor\" r=\"1\" cx=\"13\" cy=\"7\"></circle><rect y=\"2\" width=\"14\" height=\"14\" rx=\"2\" x=\"8\"></rect>",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo,multiple,copy,gallery,album,collection,slideshow,showcase",
        contributors = "karsa-mistmere"
    ))]
    Images,
    #[cfg(feature = "import")]
    #[strum(props(
        svg = "<path d=\"M12 3v12\"></path><path d=\"m8 11 4 4 4-4\"></path><path d=\"M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4\"></path>",
        categories = "arrows,files",
        tags = "save",
        contributors = "mittalyashu,ericfennis"
    ))]
    Import,
    #[cfg(feature = "inbox")]
    #[strum(props(
        svg = "<polyline points=\"22 12 16 12 14 15 10 15 8 12 2 12\"></polyline><path d=\"M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\"></path>",
        categories = "account,mail",
        tags = "email",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Inbox,
    #[cfg(feature = "indian_rupee")]
    #[strum(props(
        svg = "<path d=\"M6 3h12\"></path><path d=\"M6 8h12\"></path><path d=\"m6 13 8.5 8\"></path><path d=\"M6 13h3\"></path><path d=\"M9 13c6.66 0 6.66-10 0-10\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    IndianRupee,
    #[cfg(feature = "infinity")]
    #[strum(props(
        svg = "<path d=\"M6 16c5 0 7-8 12-8a4 4 0 0 1 0 8c-5 0-7-8-12-8a4 4 0 1 0 0 8\"></path>",
        categories = "multimedia",
        tags = "unlimited,forever,loop,math",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,jamiemlaw"
    ))]
    Infinity,
    #[cfg(feature = "info")]
    #[strum(props(
        svg = "<circle cx=\"12\" r=\"10\" cy=\"12\"></circle><path d=\"M12 16v-4\"></path><path d=\"M12 8h.01\"></path>",
        categories = "accessibility,notifications",
        tags = "about,advice,clue,details,help,hint,indicator,information,knowledge,notice,status,support,tooltip",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Info,
    #[cfg(feature = "inspection_panel")]
    #[strum(props(
        svg = "<rect y=\"3\" height=\"18\" x=\"3\" width=\"18\" rx=\"2\"></rect><path d=\"M7 7h.01\"></path><path d=\"M17 7h.01\"></path><path d=\"M7 17h.01\"></path><path d=\"M17 17h.01\"></path>",
        categories = "tools",
        tags = "access,cover,tile,metal,materials,screws",
        contributors = "danielbayley"
    ))]
    InspectionPanel,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "<line y1=\"4\" x1=\"19\" x2=\"10\" y2=\"4\"></line><line x2=\"5\" y1=\"20\" y2=\"20\" x1=\"14\"></line><line x2=\"9\" y1=\"4\" x1=\"15\" y2=\"20\"></line>",
        categories = "text",
        tags = "oblique,text,format",
        contributors = "colebemis,ericfennis"
    ))]
    Italic,
    #[cfg(feature = "iteration_ccw")]
    #[strum(props(
        svg = "<path d=\"m16 14 4 4-4 4\"></path><path d=\"M20 10a8 8 0 1 0-8 8h8\"></path>",
        categories = "arrows,design",
        tags = "arrow,right",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCcw,
    #[cfg(feature = "iteration_cw")]
    #[strum(props(
        svg = "<path d=\"M4 10a8 8 0 1 1 8 8H4\"></path><path d=\"m8 22-4-4 4-4\"></path>",
        categories = "arrows,design",
        tags = "arrow,left",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCw,
    #[cfg(feature = "japanese_yen")]
    #[strum(props(
        svg = "<path d=\"M12 9.5V21m0-11.5L6 3m6 6.5L18 3\"></path><path d=\"M6 15h12\"></path><path d=\"M6 11h12\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis"
    ))]
    JapaneseYen,
    #[cfg(feature = "joystick")]
    #[strum(props(
        svg = "<path d=\"M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z\"></path><path d=\"M6 15v-2\"></path><path d=\"M12 15V9\"></path><circle cx=\"12\" r=\"3\" cy=\"6\"></circle>",
        categories = "gaming,devices",
        tags = "game,console,control stick",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Joystick,
    #[cfg(feature = "kanban")]
    #[strum(props(
        svg = "<path d=\"M5 3v14\"></path><path d=\"M12 3v8\"></path><path d=\"M19 3v18\"></path>",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Kanban,
    #[cfg(feature = "kayak")]
    #[strum(props(
        svg = "<path d=\"M18 17a1 1 0 0 0-1 1v1a2 2 0 1 0 2-2z\"></path><path d=\"M20.97 3.61a.45.45 0 0 0-.58-.58C10.2 6.6 6.6 10.2 3.03 20.39a.45.45 0 0 0 .58.58C13.8 17.4 17.4 13.8 20.97 3.61\"></path><path d=\"m6.70 6.70 10.58 10.58\"></path><path d=\"M7 5a2 2 0 1 0-2 2h1a1 1 0 0 0 1-1z\"></path>",
        categories = "transportation",
        tags = "kayak,boat,paddle,water,sport,recreation,adventure,outdoors,equipment,lake,ocean",
        contributors = "jguddas,jpjacobpadilla"
    ))]
    Kayak,
    #[cfg(feature = "key_round")]
    #[strum(props(
        svg = "<path d=\"M2.58 17.41A2 2 0 0 0 2 18.82V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.41-.586l.814-.814a6.5 6.5 0 1 0-4-4z\"></path><circle cy=\"7.5\" fill=\"currentColor\" cx=\"16.5\" r=\".5\"></circle>",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = "danielbayley,jguddas"
    ))]
    KeyRound,
    #[cfg(feature = "key_square")]
    #[strum(props(
        svg = "<path d=\"M12.4 2.7a2.5 2.5 0 0 1 3.4 0l5.5 5.5a2.5 2.5 0 0 1 0 3.4l-3.7 3.7a2.5 2.5 0 0 1-3.4 0L8.7 9.8a2.5 2.5 0 0 1 0-3.4z\"></path><path d=\"m14 7 3 3\"></path><path d=\"m9.4 10.6-6.81 6.81A2 2 0 0 0 2 18.82V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.41-.586l.814-.814\"></path>",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = "danielbayley,jguddas"
    ))]
    KeySquare,
    #[cfg(feature = "key")]
    #[strum(props(
        svg = "<path d=\"m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4\"></path><path d=\"m21 2-9.6 9.6\"></path><circle r=\"5.5\" cx=\"7.5\" cy=\"15.5\"></circle>",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis"
    ))]
    Key,
    #[cfg(feature = "keyboard_music")]
    #[strum(props(
        svg = "<rect height=\"16\" width=\"20\" y=\"4\" rx=\"2\" x=\"2\"></rect><path d=\"M6 8h4\"></path><path d=\"M14 8h.01\"></path><path d=\"M18 8h.01\"></path><path d=\"M2 12h20\"></path><path d=\"M6 12v4\"></path><path d=\"M10 12v4\"></path><path d=\"M14 12v4\"></path><path d=\"M18 12v4\"></path>",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,keys,chord,octave,midi,controller,instrument,electric,signal,digital,studio,production,producer,pianist,piano,play,performance,concert",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    KeyboardMusic,
    #[cfg(feature = "keyboard_off")]
    #[strum(props(
        svg = "<path d=\"M 20 4 A2 2 0 0 1 22 6\"></path><path d=\"M 22 6 L 22 16.41\"></path><path d=\"M 7 16 L 16 16\"></path><path d=\"M 9.69 4 L 20 4\"></path><path d=\"M14 8h.01\"></path><path d=\"M18 8h.01\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2\"></path><path d=\"M6 8h.01\"></path><path d=\"M8 12h.01\"></path>",
        categories = "devices,text,development",
        tags = "unkeys,layout,spell,settings,mouse",
        contributors = "Diottodev,karsa-mistmere"
    ))]
    KeyboardOff,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "<path d=\"M10 8h.01\"></path><path d=\"M12 12h.01\"></path><path d=\"M14 8h.01\"></path><path d=\"M16 12h.01\"></path><path d=\"M18 8h.01\"></path><path d=\"M6 8h.01\"></path><path d=\"M7 16h10\"></path><path d=\"M8 12h.01\"></path><rect width=\"20\" height=\"16\" rx=\"2\" y=\"4\" x=\"2\"></rect>",
        categories = "text,devices,development",
        tags = "layout,spell,settings,mouse",
        contributors = "it-is-not,ericfennis"
    ))]
    Keyboard,
    #[cfg(feature = "lamp_ceiling")]
    #[strum(props(
        svg = "<path d=\"M12 2v5\"></path><path d=\"M14.82 15.99a3 3 0 1 1-5.65 0\"></path><path d=\"M20.92 14.60A1 1 0 0 1 20 16H4a1 1 0 0 1-.92-1.39l3-7A1 1 0 0 1 7 7h10a1 1 0 0 1 .92.60z\"></path>",
        categories = "home",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    LampCeiling,
    #[cfg(feature = "lamp_desk")]
    #[strum(props(
        svg = "<path d=\"M10.29 2.29a1 1 0 0 1 1.41 0l2.5 2.5 5.99 1.22a1 1 0 0 1 .506 1.68l-7 7a1 1 0 0 1-1.68-.506l-1.22-5.99-2.5-2.5a1 1 0 0 1 0-1.41z\"></path><path d=\"m14.20 4.79-3.41 3.41\"></path><path d=\"M3 20a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1z\"></path><path d=\"m9.08 6.5-4.79 4.79a1 1 0 0 0-.18 1.17L7 18\"></path>",
        categories = "home",
        tags = "lighting,household,office,desk,home,furniture",
        contributors = "karsa-mistmere,jguddas,jamiemlaw"
    ))]
    LampDesk,
    #[cfg(feature = "lamp_floor")]
    #[strum(props(
        svg = "<path d=\"M12 10v12\"></path><path d=\"M17.92 7.62A1 1 0 0 1 17 9H7a1 1 0 0 1-.928-1.37l2-5A1 1 0 0 1 9 2h6a1 1 0 0 1 .928.62z\"></path><path d=\"M9 22h6\"></path>",
        categories = "home",
        tags = "lighting,household,floor,home,furniture",
        contributors = "karsa-mistmere,danielbayley,jamiemlaw"
    ))]
    LampFloor,
    #[cfg(feature = "lamp_wall_down")]
    #[strum(props(
        svg = "<path d=\"M19.92 18.62A1 1 0 0 1 19 20H9a1 1 0 0 1-.928-1.37l2-5A1 1 0 0 1 11 13h6a1 1 0 0 1 .928.62z\"></path><path d=\"M6 3a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z\"></path><path d=\"M8 6h4a2 2 0 0 1 2 2v5\"></path>",
        categories = "home",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    LampWallDown,
    #[cfg(feature = "lamp_wall_up")]
    #[strum(props(
        svg = "<path d=\"M19.92 9.62A1 1 0 0 1 19 11H9a1 1 0 0 1-.928-1.37l2-5A1 1 0 0 1 11 4h6a1 1 0 0 1 .928.62z\"></path><path d=\"M6 15a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1z\"></path><path d=\"M8 18h4a2 2 0 0 0 2-2v-5\"></path>",
        categories = "home",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    LampWallUp,
    #[cfg(feature = "lamp")]
    #[strum(props(
        svg = "<path d=\"M12 12v6\"></path><path d=\"M4.07 10.61A1 1 0 0 0 5 12h14a1 1 0 0 0 .923-1.38l-3.07-7.38A2 2 0 0 0 15 2H9a2 2 0 0 0-1.84 1.23Z\"></path><path d=\"M8 20a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1z\"></path>",
        categories = "home",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Lamp,
    #[cfg(feature = "land_plot")]
    #[strum(props(
        svg = "<path d=\"m12 8 6-3-6-3v10\"></path><path d=\"m8 11.99-5.5 3.14a1 1 0 0 0 0 1.74l8.5 4.86a2 2 0 0 0 2 0l8.5-4.86a1 1 0 0 0 0-1.74L16 12\"></path><path d=\"m6.49 12.85 11.02 6.3\"></path><path d=\"M17.51 12.85 6.5 19.15\"></path>",
        categories = "design,tools,math,sports,gaming",
        tags = "area,surface,square metres,allotment,parcel,property,plane,acres,measure,distance,isometric,flag,golf course,hole",
        contributors = "danielbayley"
    ))]
    LandPlot,
    #[cfg(feature = "landmark")]
    #[strum(props(
        svg = "<path d=\"M10 18v-7\"></path><path d=\"M11.12 2.19a2 2 0 0 1 1.76.00l7.86 3.84c.476.23.31.94-.22.94H3.47c-.53 0-.695-.716-.22-.949z\"></path><path d=\"M14 18v-7\"></path><path d=\"M18 18v-7\"></path><path d=\"M3 22h18\"></path><path d=\"M6 18v-7\"></path>",
        categories = "finance,navigation,buildings",
        tags = "bank,building,capitol,finance,money,museum,art gallery,hall,institute,pediment,portico,columns,pillars,classical,architecture,government,institution",
        contributors = "connium,ericfennis,karsa-mistmere"
    ))]
    Landmark,
    #[cfg(feature = "languages")]
    #[strum(props(
        svg = "<path d=\"m5 8 6 6\"></path><path d=\"m4 14 6-6 2-3\"></path><path d=\"M2 5h12\"></path><path d=\"M7 2h1\"></path><path d=\"m22 22-5-10-5 10\"></path><path d=\"M14 18h6\"></path>",
        categories = "text",
        tags = "translate",
        contributors = "ericfennis,mittalyashu,johnletey"
    ))]
    Languages,
    #[cfg(feature = "laptop_minimal_check")]
    #[strum(props(
        svg = "<path d=\"M2 20h20\"></path><path d=\"m9 10 2 2 4-4\"></path><rect rx=\"2\" width=\"18\" y=\"4\" x=\"3\" height=\"12\"></rect>",
        categories = "devices,notifications",
        tags = "computer,screen,remote,success,done,todo,tick,complete,task",
        contributors = "ericfennis,jguddas"
    ))]
    LaptopMinimalCheck,
    #[cfg(feature = "laptop_minimal")]
    #[strum(props(
        svg = "<rect rx=\"2\" ry=\"2\" x=\"3\" width=\"18\" y=\"4\" height=\"12\"></rect><line y1=\"20\" x2=\"22\" y2=\"20\" x1=\"2\"></line>",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis"
    ))]
    LaptopMinimal,
    #[cfg(feature = "laptop")]
    #[strum(props(
        svg = "<path d=\"M18 5a2 2 0 0 1 2 2v8.52a2 2 0 0 0 .212.89l1.06 2.12a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45l1.06-2.12A2 2 0 0 0 4 15.52V7a2 2 0 0 1 2-2z\"></path><path d=\"M20.05 15.98H3.94\"></path>",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis,csandman"
    ))]
    Laptop,
    #[cfg(feature = "lasso_select")]
    #[strum(props(
        svg = "<path d=\"M7 22a5 5 0 0 1-2-4\"></path><path d=\"M7 16.93c.96.43 1.96.74 2.99.91\"></path><path d=\"M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2\"></path><path d=\"M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z\"></path><path d=\"M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14z\"></path>",
        categories = "arrows,design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    LassoSelect,
    #[cfg(feature = "lasso")]
    #[strum(props(
        svg = "<path d=\"M3.70 14.46a10 8 0 1 1 3.11 2.37\"></path><path d=\"M7 22a5 5 0 0 1-2-3.99\"></path><circle cy=\"16\" r=\"2\" cx=\"5\"></circle>",
        categories = "design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman,jguddas"
    ))]
    Lasso,
    #[cfg(feature = "laugh")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z\"></path><line x2=\"9.01\" y1=\"9\" y2=\"9\" x1=\"9\"></line><line y2=\"9\" x2=\"15.01\" x1=\"15\" y1=\"9\"></line>",
        categories = "emoji",
        tags = "emoji,face,happy,good,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Laugh,
    #[cfg(feature = "layers_2")]
    #[strum(props(
        svg = "<path d=\"M13 13.74a2 2 0 0 1-2 0L2.5 8.87a1 1 0 0 1 0-1.74L11 2.26a2 2 0 0 1 2 0l8.5 4.87a1 1 0 0 1 0 1.74z\"></path><path d=\"m20 14.28 1.5.84a1 1 0 0 1 0 1.74L13 21.74a2 2 0 0 1-2 0l-8.5-4.87a1 1 0 0 1 0-1.74l1.5-.845\"></path>",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy,duplicate,double,shortcuts",
        contributors = "danielbayley,jguddas"
    ))]
    Layers2,
    #[cfg(feature = "layers_plus")]
    #[strum(props(
        svg = "<path d=\"M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 .83.18 2 2 0 0 0 .83-.18l8.58-3.9a1 1 0 0 0 0-1.83z\"></path><path d=\"M16 17h6\"></path><path d=\"M19 14v6\"></path><path d=\"M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 .825.17\"></path><path d=\"M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l2.11-.962\"></path>",
        categories = "design,layout",
        tags = "stack,layers,add,new,increase,create,positive,copy,upgrade",
        contributors = "juanisidoro,karsa-mistmere"
    ))]
    LayersPlus,
    #[cfg(feature = "layers")]
    #[strum(props(
        svg = "<path d=\"M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z\"></path><path d=\"M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12\"></path><path d=\"M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17\"></path>",
        categories = "design,layout",
        tags = "stack,pile,pages,sheets,paperwork,copies,copy",
        contributors = "colebemis,danielbayley,jguddas"
    ))]
    Layers,
    #[cfg(feature = "layout_dashboard")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"7\" height=\"9\" x=\"3\" rx=\"1\"></rect><rect width=\"7\" rx=\"1\" height=\"5\" x=\"14\" y=\"3\"></rect><rect y=\"12\" rx=\"1\" height=\"9\" x=\"14\" width=\"7\"></rect><rect width=\"7\" height=\"5\" y=\"16\" rx=\"1\" x=\"3\"></rect>",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "<rect height=\"7\" y=\"3\" rx=\"1\" width=\"7\" x=\"3\"></rect><rect height=\"7\" x=\"14\" width=\"7\" y=\"3\" rx=\"1\"></rect><rect height=\"7\" x=\"14\" width=\"7\" y=\"14\" rx=\"1\"></rect><rect y=\"14\" x=\"3\" height=\"7\" width=\"7\" rx=\"1\"></rect>",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "<rect rx=\"1\" height=\"7\" x=\"3\" width=\"7\" y=\"3\"></rect><rect x=\"3\" y=\"14\" height=\"7\" rx=\"1\" width=\"7\"></rect><path d=\"M14 4h7\"></path><path d=\"M14 9h7\"></path><path d=\"M14 15h7\"></path><path d=\"M14 20h7\"></path>",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "<rect width=\"7\" x=\"3\" rx=\"1\" y=\"3\" height=\"18\"></rect><rect width=\"7\" rx=\"1\" y=\"3\" height=\"7\" x=\"14\"></rect><rect rx=\"1\" height=\"7\" x=\"14\" y=\"14\" width=\"7\"></rect>",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "<rect rx=\"1\" width=\"18\" x=\"3\" height=\"7\" y=\"3\"></rect><rect rx=\"1\" height=\"7\" x=\"3\" y=\"14\" width=\"7\"></rect><rect width=\"7\" x=\"14\" height=\"7\" rx=\"1\" y=\"14\"></rect>",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"7\" x=\"3\" y=\"3\" rx=\"1\"></rect><rect x=\"3\" y=\"14\" width=\"9\" height=\"7\" rx=\"1\"></rect><rect width=\"5\" height=\"7\" x=\"16\" rx=\"1\" y=\"14\"></rect>",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[cfg(feature = "leaf")]
    #[strum(props(
        svg = "<path d=\"M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z\"></path><path d=\"M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12\"></path>",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,energy,plant,autumn",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Leaf,
    #[cfg(feature = "leafy_green")]
    #[strum(props(
        svg = "<path d=\"M2 22c1.25-.987 2.27-1.97 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.18-2.35 3.5 3.5 0 0 0 3.69-5.11A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.83 1.37 3.5 3.5 0 0 0-5.11 3.69 4 4 0 0 0-2.34 6.15C3.49 15.42 4.40 16.71 4.2 18.1 3.92 19.74 3.01 20.73 2 22\"></path><path d=\"M2 22 17 7\"></path>",
        categories = "food-beverage,emoji,sustainability",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[cfg(feature = "lectern")]
    #[strum(props(
        svg = "<path d=\"M16 12h3a2 2 0 0 0 1.90-1.38l1.05-3.33A1 1 0 0 0 21 6H3a1 1 0 0 0-.958 1.28l1.05 3.33A2 2 0 0 0 5 12h3\"></path><path d=\"M18 6V3a1 1 0 0 0-1-1h-3\"></path><rect x=\"8\" rx=\"1\" y=\"10\" width=\"8\" height=\"12\"></rect>",
        categories = "communication,multimedia",
        tags = "pulpit,podium,stand",
        contributors = "gurtt,karsa-mistmere"
    ))]
    Lectern,
    #[cfg(feature = "lens_concave")]
    #[strum(props(
        svg = "<path d=\"M7 2a1 1 0 0 0-.8 1.6 14 14 0 0 1 0 16.8A1 1 0 0 0 7 22h10a1 1 0 0 0 .8-1.6 14 14 0 0 1 0-16.8A1 1 0 0 0 17 2z\"></path>",
        categories = "science,tools,shapes",
        tags = "concave,lens,optics,light,magnification,curved,focus,refraction,science,physics,eyeglass,telescope,microscope",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    LensConcave,
    #[cfg(feature = "lens_convex")]
    #[strum(props(
        svg = "<path d=\"M13.43 2a1 1 0 0 1 .824.44 18 18 0 0 1 0 19.10 1 1 0 0 1-.824.44h-2.86a1 1 0 0 1-.824-.448 18 18 0 0 1 0-19.10A1 1 0 0 1 10.56 2z\"></path>",
        categories = "science,tools,shapes",
        tags = "convex,lens,optics,magnification,focus,light,refraction,physics,eyeglass,telescope,microscope,curved,science",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    LensConvex,
    #[cfg(feature = "library_big")]
    #[strum(props(
        svg = "<rect width=\"8\" x=\"3\" y=\"3\" rx=\"1\" height=\"18\"></rect><path d=\"M7 3v18\"></path><path d=\"M20.4 18.9c.2.5-.1 1.1-.6 1.3l-1.9.7c-.5.2-1.1-.1-1.3-.6L11.1 5.1c-.2-.5.1-1.1.6-1.3l1.9-.7c.5-.2 1.1.1 1.3.6Z\"></path>",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    LibraryBig,
    #[cfg(feature = "library")]
    #[strum(props(
        svg = "<path d=\"m16 6 4 14\"></path><path d=\"M12 6v14\"></path><path d=\"M8 8v12\"></path><path d=\"M4 4v16\"></path>",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "johnletey,csandman,ericfennis"
    ))]
    Library,
    #[cfg(feature = "life_buoy")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"m4.93 4.93 4.24 4.24\"></path><path d=\"m14.83 9.17 4.24-4.24\"></path><path d=\"m14.83 14.83 4.24 4.24\"></path><path d=\"m9.17 14.83-4.24 4.24\"></path><circle cy=\"12\" r=\"4\" cx=\"12\"></circle>",
        categories = "accessibility,medical",
        tags = "preserver,life belt,lifesaver,help,rescue,ship,ring,raft,inflatable,wheel,donut",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    LifeBuoy,
    #[cfg(feature = "ligature")]
    #[strum(props(
        svg = "<path d=\"M14 12h2v8\"></path><path d=\"M14 20h4\"></path><path d=\"M6 12h4\"></path><path d=\"M6 20h4\"></path><path d=\"M8 20V8a4 4 0 0 1 7.46-2\"></path>",
        categories = "text",
        tags = "text,font,typography,alternates,alternatives",
        contributors = "danielbayley"
    ))]
    Ligature,
    #[cfg(feature = "lightbulb_off")]
    #[strum(props(
        svg = "<path d=\"M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5\"></path><path d=\"m2 2 20 20\"></path><path d=\"M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5\"></path><path d=\"M9 18h6\"></path><path d=\"M10 22h4\"></path>",
        categories = "photography",
        tags = "lights",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,danielbayley"
    ))]
    LightbulbOff,
    #[cfg(feature = "lightbulb")]
    #[strum(props(
        svg = "<path d=\"M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5\"></path><path d=\"M9 18h6\"></path><path d=\"M10 22h4\"></path>",
        categories = "photography",
        tags = "idea,bright,lights",
        contributors = "ericfennis,danielbayley"
    ))]
    Lightbulb,
    #[cfg(feature = "line_dot_right_horizontal")]
    #[strum(props(
        svg = "<path d=\"M 3 12 L 15 12\"></path><circle r=\"3\" cx=\"18\" cy=\"12\"></circle>",
        categories = "development,navigation",
        tags = "code,version control,waypoint,stop,station,last,end",
        contributors = "colebemis,ericfennis,johnletey,nathan-de-pachtere"
    ))]
    LineDotRightHorizontal,
    #[cfg(feature = "line_squiggle")]
    #[strum(props(
        svg = "<path d=\"M7 3.5c5-2 7 2.5 3 4C1.5 10 2 15 5 16c5 2 9-10 14-7s.5 13.5-4 12c-5-2.5.5-11 6-2\"></path>",
        categories = "shapes,math,design",
        tags = "line,snakes,annotate,curve,doodle,stroke,pen,tool,gesture,draw,wave,art,road",
        contributors = "chessurisme,jguddas"
    ))]
    LineSquiggle,
    #[cfg(feature = "line_style")]
    #[strum(props(
        svg = "<path d=\"M11 5h2\"></path><path d=\"M15 12h6\"></path><path d=\"M19 5h2\"></path><path d=\"M3 12h6\"></path><path d=\"M3 19h18\"></path><path d=\"M3 5h2\"></path>",
        categories = "design,tools",
        tags = "line,stroke,style,dashed,border",
        contributors = "dg-ac"
    ))]
    LineStyle,
    #[cfg(feature = "link_2_off")]
    #[strum(props(
        svg = "<path d=\"M9 17H7A5 5 0 0 1 7 7\"></path><path d=\"M15 7h2a5 5 0 0 1 4 8\"></path><line y1=\"12\" x1=\"8\" x2=\"12\" y2=\"12\"></line><line x2=\"22\" y2=\"22\" y1=\"2\" x1=\"2\"></line>",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "<path d=\"M9 17H7A5 5 0 0 1 7 7h2\"></path><path d=\"M15 7h2a5 5 0 1 1 0 10h-2\"></path><line y2=\"12\" x2=\"16\" y1=\"12\" x1=\"8\"></line>",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    Link2,
    #[cfg(feature = "link")]
    #[strum(props(
        svg = "<path d=\"M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71\"></path><path d=\"M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71\"></path>",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Link,
    #[cfg(feature = "list_check")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M16 12H3\"></path><path d=\"M11 19H3\"></path><path d=\"m15 18 2 2 4-4\"></path>",
        categories = "text",
        tags = "done,check,tick,complete,list,to-do,bom",
        contributors = "guanboo-yang,karsa-mistmere"
    ))]
    ListCheck,
    #[cfg(feature = "list_checks")]
    #[strum(props(
        svg = "<path d=\"M13 5h8\"></path><path d=\"M13 12h8\"></path><path d=\"M13 19h8\"></path><path d=\"m3 17 2 2 4-4\"></path><path d=\"m3 7 2 2 4-4\"></path>",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListChecks,
    #[cfg(feature = "list_chevrons_down_up")]
    #[strum(props(
        svg = "<path d=\"M3 5h8\"></path><path d=\"M3 12h8\"></path><path d=\"M3 19h8\"></path><path d=\"m15 5 3 3 3-3\"></path><path d=\"m15 19 3-3 3 3\"></path>",
        categories = "text,arrows",
        tags = "options,items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold,vertical",
        contributors = "colebemis,ericfennis,ocavue,jguddas,PeterlitsZo,mittalyashu,juliankellydesign,karsa-mistmere"
    ))]
    ListChevronsDownUp,
    #[cfg(feature = "list_chevrons_up_down")]
    #[strum(props(
        svg = "<path d=\"M3 5h8\"></path><path d=\"M3 12h8\"></path><path d=\"M3 19h8\"></path><path d=\"m15 8 3-3 3 3\"></path><path d=\"m15 16 3 3 3-3\"></path>",
        categories = "text,arrows",
        tags = "options,items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold,vertical",
        contributors = "colebemis,ericfennis,ocavue,jguddas,PeterlitsZo,mittalyashu,juliankellydesign,karsa-mistmere"
    ))]
    ListChevronsUpDown,
    #[cfg(feature = "list_collapse")]
    #[strum(props(
        svg = "<path d=\"M10 5h11\"></path><path d=\"M10 12h11\"></path><path d=\"M10 19h11\"></path><path d=\"m3 10 3-3-3-3\"></path><path d=\"m3 20 3-3-3-3\"></path>",
        categories = "text",
        tags = "items,collapse,expand,details,disclosure,show,hide,toggle,accordion,more,less,fold,unfold",
        contributors = "ocavue,jguddas,karsa-mistmere"
    ))]
    ListCollapse,
    #[cfg(feature = "list_end")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M16 12H3\"></path><path d=\"M9 19H3\"></path><path d=\"m16 16-3 3 3 3\"></path><path d=\"M21 5v12a2 2 0 0 1-2 2h-6\"></path>",
        categories = "multimedia,text",
        tags = "queue,bottom,end,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListEnd,
    #[cfg(feature = "list_filter_plus")]
    #[strum(props(
        svg = "<path d=\"M12 5H2\"></path><path d=\"M6 12h12\"></path><path d=\"M9 19h6\"></path><path d=\"M16 5h6\"></path><path d=\"M19 8V2\"></path>",
        categories = "text,layout",
        tags = "filter,plus,options,add",
        contributors = "abdeniz,karsa-mistmere"
    ))]
    ListFilterPlus,
    #[cfg(feature = "list_filter")]
    #[strum(props(
        svg = "<path d=\"M2 5h20\"></path><path d=\"M6 12h12\"></path><path d=\"M9 19h6\"></path>",
        categories = "text",
        tags = "options",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListFilter,
    #[cfg(feature = "list_indent_decrease")]
    #[strum(props(
        svg = "<path d=\"M21 5H11\"></path><path d=\"M21 12H11\"></path><path d=\"M21 19H11\"></path><path d=\"m7 8-4 4 4 4\"></path>",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis,karsa-mistmere"
    ))]
    ListIndentDecrease,
    #[cfg(feature = "list_indent_increase")]
    #[strum(props(
        svg = "<path d=\"M21 5H11\"></path><path d=\"M21 12H11\"></path><path d=\"M21 19H11\"></path><path d=\"m3 8 4 4-4 4\"></path>",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis,karsa-mistmere"
    ))]
    ListIndentIncrease,
    #[cfg(feature = "list_minus")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M11 12H3\"></path><path d=\"M16 19H3\"></path><path d=\"M21 12h-6\"></path>",
        categories = "multimedia,text",
        tags = "playlist,remove,song,subtract,delete,unqueue",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListMinus,
    #[cfg(feature = "list_music")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M11 12H3\"></path><path d=\"M11 19H3\"></path><path d=\"M21 16V5\"></path><circle cx=\"18\" cy=\"16\" r=\"3\"></circle>",
        categories = "multimedia",
        tags = "playlist,queue,music,audio,playback",
        contributors = "karsa-mistmere"
    ))]
    ListMusic,
    #[cfg(feature = "list_ordered")]
    #[strum(props(
        svg = "<path d=\"M11 5h10\"></path><path d=\"M11 12h10\"></path><path d=\"M11 19h10\"></path><path d=\"M4 4h1v5\"></path><path d=\"M4 9h2\"></path><path d=\"M6.5 20H3.4c0-1 2.6-1.92 2.6-3.5a1.5 1.5 0 0 0-2.6-1.02\"></path>",
        categories = "text",
        tags = "number,order,queue",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ListOrdered,
    #[cfg(feature = "list_plus")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M11 12H3\"></path><path d=\"M16 19H3\"></path><path d=\"M18 9v6\"></path><path d=\"M21 12h-6\"></path>",
        categories = "multimedia,text",
        tags = "playlist,add,song,track,new",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListPlus,
    #[cfg(feature = "list_restart")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M7 12H3\"></path><path d=\"M7 19H3\"></path><path d=\"M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14\"></path><path d=\"M11 10v4h4\"></path>",
        categories = "multimedia,text",
        tags = "reset,refresh,reload,playlist,replay",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListRestart,
    #[cfg(feature = "list_start")]
    #[strum(props(
        svg = "<path d=\"M3 5h6\"></path><path d=\"M3 12h13\"></path><path d=\"M3 19h13\"></path><path d=\"m16 8-3-3 3-3\"></path><path d=\"M21 19V7a2 2 0 0 0-2-2h-6\"></path>",
        categories = "multimedia,text",
        tags = "queue,top,start,next,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListStart,
    #[cfg(feature = "list_todo")]
    #[strum(props(
        svg = "<path d=\"M13 5h8\"></path><path d=\"M13 12h8\"></path><path d=\"M13 19h8\"></path><path d=\"m3 17 2 2 4-4\"></path><rect y=\"4\" height=\"6\" width=\"6\" rx=\"1\" x=\"3\"></rect>",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    ListTodo,
    #[cfg(feature = "list_tree")]
    #[strum(props(
        svg = "<path d=\"M8 5h13\"></path><path d=\"M13 12h8\"></path><path d=\"M13 19h8\"></path><path d=\"M3 10a2 2 0 0 0 2 2h3\"></path><path d=\"M3 5v12a2 2 0 0 0 2 2h3\"></path>",
        categories = "files,text,layout",
        tags = "tree,browser",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListTree,
    #[cfg(feature = "list_video")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M10 12H3\"></path><path d=\"M10 19H3\"></path><path d=\"M15 12.00a1 1 0 0 1 1.51-.859l4.99 2.99a1 1 0 0 1 0 1.71l-4.99 2.99a1 1 0 0 1-1.51-.86z\"></path>",
        categories = "multimedia",
        tags = "playlist,video,playback",
        contributors = "karsa-mistmere"
    ))]
    ListVideo,
    #[cfg(feature = "list_x")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M11 12H3\"></path><path d=\"M16 19H3\"></path><path d=\"m15.5 9.5 5 5\"></path><path d=\"m20.5 9.5-5 5\"></path>",
        categories = "multimedia,text",
        tags = "playlist,subtract,remove,delete,unqueue",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ListX,
    #[cfg(feature = "list")]
    #[strum(props(
        svg = "<path d=\"M3 5h.01\"></path><path d=\"M3 12h.01\"></path><path d=\"M3 19h.01\"></path><path d=\"M8 5h13\"></path><path d=\"M8 12h13\"></path><path d=\"M8 19h13\"></path>",
        categories = "text",
        tags = "options",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    List,
    #[cfg(feature = "loader_circle")]
    #[strum(props(
        svg = "<path d=\"M21 12a9 9 0 1 1-6.21-8.56\"></path>",
        categories = "cursors,multimedia,layout",
        tags = "loading,wait,busy,progress,spinner,spinning,throbber,circle",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,ericfennis"
    ))]
    LoaderCircle,
    #[cfg(feature = "loader_pinwheel")]
    #[strum(props(
        svg = "<path d=\"M22 12a1 1 0 0 1-10 0 1 1 0 0 0-10 0\"></path><path d=\"M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6\"></path><path d=\"M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6\"></path><circle cx=\"12\" r=\"10\" cy=\"12\"></circle>",
        categories = "cursors,design",
        tags = "loading,wait,busy,progress,throbber,spinner,spinning,beach ball,frozen,freeze",
        contributors = "danielbayley,jguddas"
    ))]
    LoaderPinwheel,
    #[cfg(feature = "loader")]
    #[strum(props(
        svg = "<path d=\"M12 2v4\"></path><path d=\"m16.2 7.8 2.9-2.9\"></path><path d=\"M18 12h4\"></path><path d=\"m16.2 16.2 2.9 2.9\"></path><path d=\"M12 18v4\"></path><path d=\"m4.9 19.1 2.9-2.9\"></path><path d=\"M2 12h4\"></path><path d=\"m4.9 4.9 2.9 2.9\"></path>",
        categories = "cursors,multimedia,layout,design",
        tags = "loading,wait,busy,progress,spinner,spinning,throbber",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "<line x1=\"2\" y2=\"12\" x2=\"5\" y1=\"12\"></line><line y1=\"12\" x2=\"22\" x1=\"19\" y2=\"12\"></line><line x2=\"12\" x1=\"12\" y2=\"5\" y1=\"2\"></line><line y1=\"19\" x1=\"12\" y2=\"22\" x2=\"12\"></line><circle cy=\"12\" r=\"7\" cx=\"12\"></circle><circle cy=\"12\" r=\"3\" cx=\"12\"></circle>",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "<path d=\"M12 19v3\"></path><path d=\"M12 2v3\"></path><path d=\"M18.89 13.24a7 7 0 0 0-8.13-8.13\"></path><path d=\"M19 12h3\"></path><path d=\"M2 12h3\"></path><path d=\"m2 2 20 20\"></path><path d=\"M7.05 7.05a7 7 0 0 0 9.9 9.9\"></path>",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "fdev,jamiemlaw"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "<line x1=\"2\" x2=\"5\" y1=\"12\" y2=\"12\"></line><line x1=\"19\" y1=\"12\" x2=\"22\" y2=\"12\"></line><line x2=\"12\" y2=\"5\" y1=\"2\" x1=\"12\"></line><line y1=\"19\" x2=\"12\" y2=\"22\" x1=\"12\"></line><circle r=\"7\" cx=\"12\" cy=\"12\"></circle>",
        categories = "navigation",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[cfg(feature = "lock_keyhole_open")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"16\" r=\"1\"></circle><rect rx=\"2\" height=\"12\" width=\"18\" y=\"10\" x=\"3\"></rect><path d=\"M7 10V7a5 5 0 0 1 9.33-2.5\"></path>",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis,cd16b,danielbayley,karsa-mistmere"
    ))]
    LockKeyholeOpen,
    #[cfg(feature = "lock_keyhole")]
    #[strum(props(
        svg = "<circle r=\"1\" cy=\"16\" cx=\"12\"></circle><rect x=\"3\" width=\"18\" height=\"12\" y=\"10\" rx=\"2\"></rect><path d=\"M7 10V7a5 5 0 0 1 10 0v3\"></path>",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis,cd16b,danielbayley,karsa-mistmere"
    ))]
    LockKeyhole,
    #[cfg(feature = "lock_open")]
    #[strum(props(
        svg = "<rect rx=\"2\" ry=\"2\" x=\"3\" width=\"18\" y=\"11\" height=\"11\"></rect><path d=\"M7 11V7a5 5 0 0 1 9.9-1\"></path>",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LockOpen,
    #[cfg(feature = "lock")]
    #[strum(props(
        svg = "<rect y=\"11\" width=\"18\" ry=\"2\" x=\"3\" height=\"11\" rx=\"2\"></rect><path d=\"M7 11V7a5 5 0 0 1 10 0v4\"></path>",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[cfg(feature = "log_in")]
    #[strum(props(
        svg = "<path d=\"m10 17 5-5-5-5\"></path><path d=\"M15 12H3\"></path><path d=\"M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4\"></path>",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[cfg(feature = "log_out")]
    #[strum(props(
        svg = "<path d=\"m16 17 5-5-5-5\"></path><path d=\"M21 12H9\"></path><path d=\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4\"></path>",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[cfg(feature = "logs")]
    #[strum(props(
        svg = "<path d=\"M3 5h1\"></path><path d=\"M3 12h1\"></path><path d=\"M3 19h1\"></path><path d=\"M8 5h1\"></path><path d=\"M8 12h1\"></path><path d=\"M8 19h1\"></path><path d=\"M13 5h8\"></path><path d=\"M13 12h8\"></path><path d=\"M13 19h8\"></path>",
        categories = "text",
        tags = "options,list,menu,order,queue,tasks,logs",
        contributors = "AnnaSasDev,karsa-mistmere"
    ))]
    Logs,
    #[cfg(feature = "lollipop")]
    #[strum(props(
        svg = "<circle r=\"8\" cx=\"11\" cy=\"11\"></circle><path d=\"m21 21-4.3-4.3\"></path><path d=\"M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0\"></path>",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = "danielbayley"
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "<path d=\"M6 20a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2\"></path><path d=\"M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14\"></path><path d=\"M10 20h4\"></path><circle cx=\"16\" cy=\"20\" r=\"2\"></circle><circle cx=\"8\" cy=\"20\" r=\"2\"></circle>",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "magnet")]
    #[strum(props(
        svg = "<path d=\"m12 15 4 4\"></path><path d=\"M2.35 10.64a1.20 1.20 0 0 0 0 1.70l2.29 2.29a1.20 1.20 0 0 0 1.70 0l6.02-6.02a1 1 0 1 1 3 3l-6.02 6.02a1.20 1.20 0 0 0 0 1.70l2.29 2.29a1.20 1.20 0 0 0 1.70 0l6.36-6.36A1 1 0 0 0 8.71 4.28z\"></path><path d=\"m5 8 4 4\"></path>",
        categories = "design",
        tags = "horseshoe,lock,science,snap",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Magnet,
    #[cfg(feature = "mail_check")]
    #[strum(props(
        svg = "<path d=\"M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"m16 19 2 2 4-4\"></path>",
        categories = "mail",
        tags = "email,message,letter,subscribe,delivered,success,read,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailCheck,
    #[cfg(feature = "mail_minus")]
    #[strum(props(
        svg = "<path d=\"M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"M16 19h6\"></path>",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailMinus,
    #[cfg(feature = "mail_open")]
    #[strum(props(
        svg = "<path d=\"M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z\"></path><path d=\"m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10\"></path>",
        categories = "mail",
        tags = "email,message,letter,read",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailOpen,
    #[cfg(feature = "mail_plus")]
    #[strum(props(
        svg = "<path d=\"M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"M19 16v6\"></path><path d=\"M16 19h6\"></path>",
        categories = "mail",
        tags = "email,message,letter,add,create,new,compose",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailPlus,
    #[cfg(feature = "mail_question_mark")]
    #[strum(props(
        svg = "<path d=\"M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2\"></path><path d=\"M20 22v.01\"></path>",
        categories = "mail",
        tags = "email,message,letter,delivery,undelivered",
        contributors = "karsa-mistmere"
    ))]
    MailQuestionMark,
    #[cfg(feature = "mail_search")]
    #[strum(props(
        svg = "<path d=\"M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z\"></path><circle r=\"3\" cx=\"18\" cy=\"18\"></circle><path d=\"m22 22-1.5-1.5\"></path>",
        categories = "mail",
        tags = "email,message,letter,search,lens",
        contributors = "karsa-mistmere"
    ))]
    MailSearch,
    #[cfg(feature = "mail_warning")]
    #[strum(props(
        svg = "<path d=\"M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"M20 14v4\"></path><path d=\"M20 22v.01\"></path>",
        categories = "mail",
        tags = "email,message,letter,delivery error,exclamation mark",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailWarning,
    #[cfg(feature = "mail_x")]
    #[strum(props(
        svg = "<path d=\"M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h9\"></path><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"></path><path d=\"m17 17 4 4\"></path><path d=\"m21 17-4 4\"></path>",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailX,
    #[cfg(feature = "mail")]
    #[strum(props(
        svg = "<path d=\"m22 7-8.99 5.72a2 2 0 0 1-2.00 0L2 7\"></path><rect height=\"16\" x=\"2\" y=\"4\" width=\"20\" rx=\"2\"></rect>",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z\"></path><polyline points=\"15,9 18,9 18,11\"></polyline><path d=\"M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2\"></path><line x2=\"7\" y2=\"10\" y1=\"10\" x1=\"6\"></line>",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "<path d=\"M17 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 1-1.73\"></path><path d=\"m22 5.5-6.41 4.17a2 2 0 0 1-2.16 0L7 5.5\"></path><rect height=\"12\" y=\"3\" x=\"7\" width=\"15\" rx=\"2\"></rect>",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Mails,
    #[cfg(feature = "map_minus")]
    #[strum(props(
        svg = "<path d=\"m11 19-1.10-.552a2 2 0 0 0-1.78 0l-3.65 1.83A1 1 0 0 1 3 19.38V6.61a1 1 0 0 1 .553-.894l4.55-2.27a2 2 0 0 1 1.78 0l4.21 2.10a2 2 0 0 0 1.78 0l3.65-1.83A1 1 0 0 1 21 4.61V14\"></path><path d=\"M15 5.76V14\"></path><path d=\"M21 18h-6\"></path><path d=\"M9 3.23v15\"></path>",
        categories = "navigation,travel",
        tags = "location,navigation,travel,drop,delete,remove,erase",
        contributors = "colebemis,karsa-mistmere,ericfennis,MarianoFranzese,jguddas"
    ))]
    MapMinus,
    #[cfg(feature = "map_pin_check_inside")]
    #[strum(props(
        svg = "<path d=\"M20 10c0 4.99-5.53 10.19-7.39 11.79a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 16 0\"></path><path d=\"m9 10 2 2 4-4\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,done,tick,complete,task,added",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinCheckInside,
    #[cfg(feature = "map_pin_check")]
    #[strum(props(
        svg = "<path d=\"M19.43 12.93c.357-.967.57-1.95.57-2.93a8 8 0 0 0-16 0c0 4.99 5.53 10.19 7.39 11.79a1 1 0 0 0 1.20 0 32.19 32.19 0 0 0 .813-.728\"></path><circle r=\"3\" cx=\"12\" cy=\"10\"></circle><path d=\"m16 18 2 2 4-4\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,done,tick,complete,task,added",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinCheck,
    #[cfg(feature = "map_pin_house")]
    #[strum(props(
        svg = "<path d=\"M15 22a1 1 0 0 1-1-1v-4a1 1 0 0 1 .445-.832l3-2a1 1 0 0 1 1.11 0l3 2A1 1 0 0 1 22 17v4a1 1 0 0 1-1 1z\"></path><path d=\"M18 10a8 8 0 0 0-16 0c0 4.99 5.53 10.19 7.39 11.79a1 1 0 0 0 .601.2\"></path><path d=\"M18 22v-3\"></path><circle cx=\"10\" cy=\"10\" r=\"3\"></circle>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,home,living,building,residence,architecture,address,poi,real estate,property,navigation,destination,geolocation,place,landmark",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinHouse,
    #[cfg(feature = "map_pin_minus_inside")]
    #[strum(props(
        svg = "<path d=\"M20 10c0 4.99-5.53 10.19-7.39 11.79a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 16 0\"></path><path d=\"M9 10h6\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinMinusInside,
    #[cfg(feature = "map_pin_minus")]
    #[strum(props(
        svg = "<path d=\"M18.97 14C19.6 12.70 20 11.34 20 10a8 8 0 0 0-16 0c0 4.99 5.53 10.19 7.39 11.79a1 1 0 0 0 1.20 0 32 32 0 0 0 .824-.738\"></path><circle r=\"3\" cy=\"10\" cx=\"12\"></circle><path d=\"M16 18h6\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinMinus,
    #[cfg(feature = "map_pin_off")]
    #[strum(props(
        svg = "<path d=\"M12.75 7.09a3 3 0 0 1 2.16 2.16\"></path><path d=\"M17.07 17.07c-1.63 2.17-3.52 3.91-4.47 4.72a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 1.43-4.56\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8.47 2.81A8 8 0 0 1 20 10c0 1.18-.31 2.37-.81 3.53\"></path><path d=\"M9.13 9.13a3 3 0 0 0 3.74 3.74\"></path>",
        categories = "navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[cfg(feature = "map_pin_pen")]
    #[strum(props(
        svg = "<path d=\"M17.97 9.30A8 8 0 0 0 2 10c0 4.69 4.88 9.56 7.02 11.46\"></path><path d=\"M21.37 16.62a1 1 0 0 0-3.00-3.00l-4.01 4.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path><circle r=\"3\" cy=\"10\" cx=\"10\"></circle>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,edit",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,sachinkr7368"
    ))]
    MapPinPen,
    #[cfg(feature = "map_pin_plus_inside")]
    #[strum(props(
        svg = "<path d=\"M20 10c0 4.99-5.53 10.19-7.39 11.79a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 16 0\"></path><path d=\"M12 7v6\"></path><path d=\"M9 10h6\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,add,create,new",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinPlusInside,
    #[cfg(feature = "map_pin_plus")]
    #[strum(props(
        svg = "<path d=\"M19.91 11.10A7.29 7.29 0 0 0 20 10a8 8 0 0 0-16 0c0 4.99 5.53 10.19 7.39 11.79a1 1 0 0 0 1.20 0 32 32 0 0 0 .824-.738\"></path><circle cx=\"12\" cy=\"10\" r=\"3\"></circle><path d=\"M16 18h6\"></path><path d=\"M19 15v6\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,add,create,new",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinPlus,
    #[cfg(feature = "map_pin_search")]
    #[strum(props(
        svg = "<path d=\"M 12.24 21.96 a 1 1 0 0 1 -0.84 -0.17 C 9.53 20.19 4 14.99 4 10 a 8 8 0 0 1 16 0 C 20 10.42 19.96 10.84 19.88 11.26\"></path><path d=\"m22 22-1.88-1.88\"></path><circle cy=\"10\" cx=\"12\" r=\"3\"></circle><circle cy=\"18\" r=\"3\" cx=\"18\"></circle>",
        categories = "text,navigation,travel,account",
        tags = "location,navigation,travel,waypoint,marker,drop",
        contributors = "colebemis,karsa-mistmere,ericfennis,csandman,TonySullivan"
    ))]
    MapPinSearch,
    #[cfg(feature = "map_pin_x_inside")]
    #[strum(props(
        svg = "<path d=\"M20 10c0 4.99-5.53 10.19-7.39 11.79a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 16 0\"></path><path d=\"m14.5 7.5-5 5\"></path><path d=\"m9.5 7.5 5 5\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinXInside,
    #[cfg(feature = "map_pin_x")]
    #[strum(props(
        svg = "<path d=\"M19.75 11.90A7.78 7.78 0 0 0 20 10a8 8 0 0 0-16 0c0 4.99 5.53 10.19 7.39 11.79a1 1 0 0 0 1.20 0 19 19 0 0 0 .09-.077\"></path><circle cy=\"10\" r=\"3\" cx=\"12\"></circle><path d=\"m21.5 15.5-5 5\"></path><path d=\"m21.5 20.5-5-5\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop,delete,remove,erase",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    MapPinX,
    #[cfg(feature = "map_pin")]
    #[strum(props(
        svg = "<path d=\"M20 10c0 4.99-5.53 10.19-7.39 11.79a1 1 0 0 1-1.20 0C9.53 20.19 4 14.99 4 10a8 8 0 0 1 16 0\"></path><circle cy=\"10\" cx=\"12\" r=\"3\"></circle>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[cfg(feature = "map_pinned")]
    #[strum(props(
        svg = "<path d=\"M18 8c0 3.61-3.86 7.42-5.39 8.79a1 1 0 0 1-1.21 0C9.87 15.42 6 11.61 6 8a6 6 0 0 1 12 0\"></path><circle cx=\"12\" cy=\"8\" r=\"2\"></circle><path d=\"M8.71 14h-3.71a1 1 0 0 0-.948.68l-2.00 6A1 1 0 0 0 3 22h18a1 1 0 0 0 .948-1.31l-2-6a1 1 0 0 0-.949-.684h-3.71\"></path>",
        categories = "navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MapPinned,
    #[cfg(feature = "map_plus")]
    #[strum(props(
        svg = "<path d=\"m11 19-1.10-.552a2 2 0 0 0-1.78 0l-3.65 1.83A1 1 0 0 1 3 19.38V6.61a1 1 0 0 1 .553-.894l4.55-2.27a2 2 0 0 1 1.78 0l4.21 2.10a2 2 0 0 0 1.78 0l3.65-1.83A1 1 0 0 1 21 4.61V12\"></path><path d=\"M15 5.76V12\"></path><path d=\"M18 15v6\"></path><path d=\"M21 18h-6\"></path><path d=\"M9 3.23v15\"></path>",
        categories = "navigation",
        tags = "location,navigation,travel,new,add,create",
        contributors = "colebemis,karsa-mistmere,ericfennis,Seanw265"
    ))]
    MapPlus,
    #[cfg(feature = "map")]
    #[strum(props(
        svg = "<path d=\"M14.10 5.55a2 2 0 0 0 1.78 0l3.65-1.83A1 1 0 0 1 21 4.61v12.76a1 1 0 0 1-.553.89l-4.55 2.27a2 2 0 0 1-1.78 0l-4.21-2.10a2 2 0 0 0-1.78 0l-3.65 1.83A1 1 0 0 1 3 19.38V6.61a1 1 0 0 1 .553-.894l4.55-2.27a2 2 0 0 1 1.78 0z\"></path><path d=\"M15 5.76v15\"></path><path d=\"M9 3.23v15\"></path>",
        categories = "text,navigation",
        tags = "location,navigation,travel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Map,
    #[cfg(feature = "mars_stroke")]
    #[strum(props(
        svg = "<path d=\"m14 6 4 4\"></path><path d=\"M17 3h4v4\"></path><path d=\"m21 3-7.75 7.75\"></path><circle cx=\"9\" cy=\"15\" r=\"6\"></circle>",
        categories = "medical",
        tags = "gender,androgyne,transgender",
        contributors = "jamiemlaw"
    ))]
    MarsStroke,
    #[cfg(feature = "mars")]
    #[strum(props(
        svg = "<path d=\"M16 3h5v5\"></path><path d=\"m21 3-6.75 6.75\"></path><circle cx=\"10\" cy=\"14\" r=\"6\"></circle>",
        categories = "medical",
        tags = "gender,sex,male,masculine,man,boy",
        contributors = "jguddas,jamiemlaw"
    ))]
    Mars,
    #[cfg(feature = "martini")]
    #[strum(props(
        svg = "<path d=\"M8 22h8\"></path><path d=\"M12 11v11\"></path><path d=\"m19 3-7 8-7-8Z\"></path>",
        categories = "food-beverage",
        tags = "cocktail,alcohol,beverage,bar,drink,glass",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    Martini,
    #[cfg(feature = "maximize_2")]
    #[strum(props(
        svg = "<path d=\"M15 3h6v6\"></path><path d=\"m21 3-7 7\"></path><path d=\"m3 21 7-7\"></path><path d=\"M9 21H3v-6\"></path>",
        categories = "arrows,layout,design",
        tags = "fullscreen,arrows,expand",
        contributors = "colebemis,ericfennis"
    ))]
    Maximize2,
    #[cfg(feature = "maximize")]
    #[strum(props(
        svg = "<path d=\"M8 3H5a2 2 0 0 0-2 2v3\"></path><path d=\"M21 8V5a2 2 0 0 0-2-2h-3\"></path><path d=\"M3 16v3a2 2 0 0 0 2 2h3\"></path><path d=\"M16 21h3a2 2 0 0 0 2-2v-3\"></path>",
        categories = "layout,design",
        tags = "fullscreen,expand,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Maximize,
    #[cfg(feature = "medal")]
    #[strum(props(
        svg = "<path d=\"M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15\"></path><path d=\"M11 12 5.12 2.2\"></path><path d=\"m13 12 5.88-9.8\"></path><path d=\"M8 7h8\"></path><circle cy=\"17\" r=\"5\" cx=\"12\"></circle><path d=\"M12 18v-2h-.5\"></path>",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[cfg(feature = "megaphone_off")]
    #[strum(props(
        svg = "<path d=\"M11.63 6A13 13 0 0 0 19.4 3.2 1 1 0 0 1 21 4v11.34\"></path><path d=\"M14.37 14.35A13 13 0 0 0 11 14H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h1\"></path><path d=\"m2 2 20 20\"></path><path d=\"M6 14a12 12 0 0 0 2.4 7.2 2 2 0 0 0 3.2-2.4A8 8 0 0 1 10 14\"></path><path d=\"M8 8v6\"></path>",
        categories = "multimedia,notifications",
        tags = "advertisement,announcement,attention,alert,loudspeaker,megaphone,notification,disable,silent",
        contributors = "jamiemlaw"
    ))]
    MegaphoneOff,
    #[cfg(feature = "megaphone")]
    #[strum(props(
        svg = "<path d=\"M11 6a13 13 0 0 0 8.4-2.8A1 1 0 0 1 21 4v12a1 1 0 0 1-1.6.8A13 13 0 0 0 11 14H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z\"></path><path d=\"M6 14a12 12 0 0 0 2.4 7.2 2 2 0 0 0 3.2-2.4A8 8 0 0 1 10 14\"></path><path d=\"M8 6v8\"></path>",
        categories = "multimedia,notifications",
        tags = "advertisement,announcement,attention,alert,loudspeaker,megaphone,notification",
        contributors = "jamiemlaw"
    ))]
    Megaphone,
    #[cfg(feature = "meh")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"10\"></circle><line x1=\"8\" x2=\"16\" y2=\"15\" y1=\"15\"></line><line x1=\"9\" y1=\"9\" y2=\"9\" x2=\"9.01\"></line><line x1=\"15\" y1=\"9\" y2=\"9\" x2=\"15.01\"></line>",
        categories = "emoji",
        tags = "emoji,face,neutral,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Meh,
    #[cfg(feature = "memory_stick")]
    #[strum(props(
        svg = "<path d=\"M12 12v-2\"></path><path d=\"M12 18v-2\"></path><path d=\"M16 12v-2\"></path><path d=\"M16 18v-2\"></path><path d=\"M2 11h1.5\"></path><path d=\"M20 18v-2\"></path><path d=\"M20.5 11H22\"></path><path d=\"M4 18v-2\"></path><path d=\"M8 12v-2\"></path><path d=\"M8 18v-2\"></path><rect height=\"10\" rx=\"2\" x=\"2\" y=\"6\" width=\"20\"></rect>",
        categories = "devices,gaming",
        tags = "ram,random access,technology,computer,chip,circuit,specs,capacity,gigabytes,gb",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MemoryStick,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "<path d=\"M4 5h16\"></path><path d=\"M4 12h16\"></path><path d=\"M4 19h16\"></path>",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Menu,
    #[cfg(feature = "merge")]
    #[strum(props(
        svg = "<path d=\"m8 6 4-4 4 4\"></path><path d=\"M12 2v10.3a4 4 0 0 1-1.17 2.87L4 22\"></path><path d=\"m20 22-5-5\"></path>",
        categories = "development,arrows",
        tags = "combine,join,unite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Merge,
    #[cfg(feature = "message_circle_check")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "social,account",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,moderate,check,done,todo,complete",
        contributors = "Shrinks99"
    ))]
    MessageCircleCheck,
    #[cfg(feature = "message_circle_code")]
    #[strum(props(
        svg = "<path d=\"m10 9-3 3 3 3\"></path><path d=\"m14 15 3-3-3-3\"></path><path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path>",
        categories = "development,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleCode,
    #[cfg(feature = "message_circle_dashed")]
    #[strum(props(
        svg = "<path d=\"M10.1 2.18a10 10 0 0 1 3.8 0\"></path><path d=\"M13.9 21.81a10 10 0 0 1-3.8 0\"></path><path d=\"M17.60 3.72a10 10 0 0 1 2.69 2.7\"></path><path d=\"M2.18 13.9a10 10 0 0 1 0-3.8\"></path><path d=\"M20.28 17.61a10 10 0 0 1-2.7 2.69\"></path><path d=\"M21.81 10.1a10 10 0 0 1 0 3.8\"></path><path d=\"M3.72 6.39a10 10 0 0 1 2.7-2.69\"></path><path d=\"m6.16 21.11-2.90.85a1 1 0 0 1-1.23-1.16l.965-2.98\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleDashed,
    #[cfg(feature = "message_circle_heart")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"M7.82 13.07A3 3 0 0 1 12 8.76a3 3 0 0 1 5.00 2.22 3 3 0 0 1-.832 2.08l-3.44 3.62a1 1 0 0 1-1.45-.001z\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleHeart,
    #[cfg(feature = "message_circle_more")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"M8 12h.01\"></path><path d=\"M12 12h.01\"></path><path d=\"M16 12h.01\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleMore,
    #[cfg(feature = "message_circle_off")]
    #[strum(props(
        svg = "<path d=\"m2 2 20 20\"></path><path d=\"M4.93 4.92a10 10 0 0 0-1.93 11.41 2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 0 0 11.30-1.98\"></path><path d=\"M8.35 2.69A10 10 0 0 1 21.3 15.65\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleOff,
    #[cfg(feature = "message_circle_plus")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"M8 12h8\"></path><path d=\"M12 8v8\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCirclePlus,
    #[cfg(feature = "message_circle_question_mark")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\"></path><path d=\"M12 17h.01\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,help",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleQuestionMark,
    #[cfg(feature = "message_circle_reply")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"m10 15-3-3 3-3\"></path><path d=\"M7 12h8a2 2 0 0 1 2 2v1\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleReply,
    #[cfg(feature = "message_circle_warning")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"M12 8v4\"></path><path d=\"M12 16h.01\"></path>",
        categories = "social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleWarning,
    #[cfg(feature = "message_circle_x")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path><path d=\"m15 9-6 6\"></path><path d=\"m9 9 6 6\"></path>",
        categories = "account,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageCircleX,
    #[cfg(feature = "message_circle")]
    #[strum(props(
        svg = "<path d=\"M2.99 16.34a2 2 0 0 1 .094 1.16l-1.06 3.29a1 1 0 0 0 1.23 1.16l3.41-.998a2 2 0 0 1 1.09.092 10 10 0 1 0-4.77-4.71\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[cfg(feature = "message_square_check")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.7.7 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"m9 11 2 2 4-4\"></path>",
        categories = "social,account",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,moderate,check,done,todo,complete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MessageSquareCheck,
    #[cfg(feature = "message_square_code")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"m10 8-3 3 3 3\"></path><path d=\"m14 14 3-3-3-3\"></path>",
        categories = "development,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,code review,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareCode,
    #[cfg(feature = "message_square_dashed")]
    #[strum(props(
        svg = "<path d=\"M14 3h2\"></path><path d=\"M16 19h-2\"></path><path d=\"M2 12v-2\"></path><path d=\"M2 16v5.28a.71.71 0 0 0 1.21.502l1.14-1.14\"></path><path d=\"M20 19a2 2 0 0 0 2-2v-1\"></path><path d=\"M22 10v2\"></path><path d=\"M22 6V5a2 2 0 0 0-2-2\"></path><path d=\"M4 3a2 2 0 0 0-2 2v1\"></path><path d=\"M8 19h2\"></path><path d=\"M8 3h2\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,draft",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MessageSquareDashed,
    #[cfg(feature = "message_square_diff")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M10 15h4\"></path><path d=\"M10 9h4\"></path><path d=\"M12 7v4\"></path>",
        categories = "development,files,social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add,patch,difference,plus,minus,plus-minus,math,code review,coding,version control,git",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareDiff,
    #[cfg(feature = "message_square_dot")]
    #[strum(props(
        svg = "<path d=\"M12.7 3H4a2 2 0 0 0-2 2v16.28a.71.71 0 0 0 1.21.502l2.20-2.20A2 2 0 0 1 6.82 19H20a2 2 0 0 0 2-2v-4.7\"></path><circle cy=\"6\" cx=\"19\" r=\"3\"></circle>",
        categories = "social,notifications",
        tags = "unread,unresolved,comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareDot,
    #[cfg(feature = "message_square_heart")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M7.5 9.5c0 .687.26 1.38.697 1.84l3.00 3.26a1.14 1.14 0 0 0 .407.31 1 1 0 0 0 .783-.004 1.14 1.14 0 0 0 .398-.31l3.00-3.26A2.77 2.77 0 0 0 16.5 9.5 2.5 2.5 0 0 0 12 8a2.5 2.5 0 0 0-4.5 1.5\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,positive,like,love,interest,valentine,dating,date,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareHeart,
    #[cfg(feature = "message_square_lock")]
    #[strum(props(
        svg = "<path d=\"M22 8.5V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v16.28a.71.71 0 0 0 1.21.502l2.20-2.20A2 2 0 0 1 6.82 19H10\"></path><path d=\"M20 15v-2a2 2 0 0 0-4 0v2\"></path><rect y=\"15\" width=\"8\" height=\"5\" rx=\"1\" x=\"14\"></rect>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,secure,encrypted",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageSquareLock,
    #[cfg(feature = "message_square_more")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M12 11h.01\"></path><path d=\"M16 11h.01\"></path><path d=\"M8 11h.01\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,typing,writing,responding,ellipsis,etc,et cetera,...,…",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareMore,
    #[cfg(feature = "message_square_off")]
    #[strum(props(
        svg = "<path d=\"M19 19H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.7.7 0 0 1 2 21.28V5a2 2 0 0 1 1.18-1.82\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8.65 3H20a2 2 0 0 1 2 2v11.34\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareOff,
    #[cfg(feature = "message_square_plus")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M12 8v6\"></path><path d=\"M9 11h6\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,add",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquarePlus,
    #[cfg(feature = "message_square_quote")]
    #[strum(props(
        svg = "<path d=\"M14 14a2 2 0 0 0 2-2V8h-2\"></path><path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M8 14a2 2 0 0 0 2-2V8H8\"></path>",
        categories = "social,text",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MessageSquareQuote,
    #[cfg(feature = "message_square_reply")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"m10 8-3 3 3 3\"></path><path d=\"M17 14v-1a2 2 0 0 0-2-2H7\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareReply,
    #[cfg(feature = "message_square_share")]
    #[strum(props(
        svg = "<path d=\"M12 3H4a2 2 0 0 0-2 2v16.28a.71.71 0 0 0 1.21.502l2.20-2.20A2 2 0 0 1 6.82 19H20a2 2 0 0 0 2-2v-4\"></path><path d=\"M16 3h6v6\"></path><path d=\"m16 9 6-6\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,network,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareShare,
    #[cfg(feature = "message_square_text")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M7 11h10\"></path><path d=\"M7 15h6\"></path><path d=\"M7 7h8\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareText,
    #[cfg(feature = "message_square_warning")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"M12 15h.01\"></path><path d=\"M12 7v4\"></path>",
        categories = "social,notifications",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,report,abuse,offense,alert,danger,caution,protected,exclamation mark",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareWarning,
    #[cfg(feature = "message_square_x")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path><path d=\"m14.5 8.5-5 5\"></path><path d=\"m9.5 8.5 5 5\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble,clear,close,delete,remove,cancel,silence,mute,moderate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessageSquareX,
    #[cfg(feature = "message_square")]
    #[strum(props(
        svg = "<path d=\"M22 17a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 21.28V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubble",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MessageSquare,
    #[cfg(feature = "messages_square")]
    #[strum(props(
        svg = "<path d=\"M16 10a2 2 0 0 1-2 2H6.82a2 2 0 0 0-1.41.586l-2.20 2.20A.71.71 0 0 1 2 14.28V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z\"></path><path d=\"M20 9a2 2 0 0 1 2 2v10.28a.71.71 0 0 1-1.21.502l-2.20-2.20A2 2 0 0 0 17.17 19H10a2 2 0 0 1-2-2v-1\"></path>",
        categories = "social",
        tags = "comment,chat,conversation,dialog,feedback,speech bubbles,copy,multiple,discussion,interview,debate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    MessagesSquare,
    #[cfg(feature = "metronome")]
    #[strum(props(
        svg = "<path d=\"M12 11.4V9.1\"></path><path d=\"m12 17 6.59-6.59\"></path><path d=\"m15.05 5.7-.218-.691a3 3 0 0 0-5.66 0L4.41 19.69A1 1 0 0 0 5.37 21h13.25a1 1 0 0 0 .951-1.31L18.45 16.2\"></path><circle cx=\"20\" cy=\"9\" r=\"2\"></circle>",
        categories = "multimedia,time",
        tags = "metronome,tempo,rhythm,beat,bpm,music,audio,sound,practice,timing,timer,time,pulse,sync,cadence,control,playback,studio,tool",
        contributors = "jguddas,edwloef"
    ))]
    Metronome,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "<path d=\"M12 19v3\"></path><path d=\"M15 9.34V5a3 3 0 0 0-5.68-1.33\"></path><path d=\"M16.95 16.95A7 7 0 0 1 5 12v-2\"></path><path d=\"M18.89 13.23A7 7 0 0 0 19 12v-2\"></path><path d=\"m2 2 20 20\"></path><path d=\"M9 9v3a3 3 0 0 0 5.12 2.12\"></path>",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,LieOnLion"
    ))]
    MicOff,
    #[cfg(feature = "mic_vocal")]
    #[strum(props(
        svg = "<path d=\"m11 7.60-5.99 8.19a1 1 0 0 0 .1 1.29l.817.81a1 1 0 0 0 1.31.087L15.09 12\"></path><path d=\"M16.5 21.17C15.5 20.5 14.37 20 13 20c-2.05 0-3.92 2.35-6 2-2.07-.356-2.77-3.36-1.5-4.5\"></path><circle cy=\"7\" r=\"5\" cx=\"16\"></circle>",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "jguddas"
    ))]
    MicVocal,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "<path d=\"M12 19v3\"></path><path d=\"M19 10v2a7 7 0 0 1-14 0v-2\"></path><rect width=\"6\" x=\"9\" y=\"2\" height=\"13\" rx=\"3\"></rect>",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,listen,radio,podcast,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Mic,
    #[cfg(feature = "microchip")]
    #[strum(props(
        svg = "<path d=\"M10 12h4\"></path><path d=\"M10 17h4\"></path><path d=\"M10 7h4\"></path><path d=\"M18 12h2\"></path><path d=\"M18 18h2\"></path><path d=\"M18 6h2\"></path><path d=\"M4 12h2\"></path><path d=\"M4 18h2\"></path><path d=\"M4 6h2\"></path><rect rx=\"2\" y=\"2\" x=\"6\" height=\"20\" width=\"12\"></rect>",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,integrated circuit,memory,ram,specs,gpu,gigahertz,ghz",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    Microchip,
    #[cfg(feature = "microscope")]
    #[strum(props(
        svg = "<path d=\"M6 18h8\"></path><path d=\"M3 22h18\"></path><path d=\"M14 22a7 7 0 1 0 0-14h-1\"></path><path d=\"M9 14h2\"></path><path d=\"M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z\"></path><path d=\"M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3\"></path>",
        categories = "science,medical",
        tags = "medical,education,science,imaging,research",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Microscope,
    #[cfg(feature = "microwave")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"20\" x=\"2\" height=\"15\" y=\"4\"></rect><rect width=\"8\" height=\"7\" x=\"6\" rx=\"1\" y=\"8\"></rect><path d=\"M18 8v7\"></path><path d=\"M6 19v2\"></path><path d=\"M18 19v2\"></path>",
        categories = "food-beverage,home",
        tags = "oven,cooker,toaster oven,bake",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Microwave,
    #[cfg(feature = "milestone")]
    #[strum(props(
        svg = "<path d=\"M12 13v8\"></path><path d=\"M12 3v3\"></path><path d=\"M18.17 6a2 2 0 0 1 1.41.586l2.06 2.06a1.20 1.20 0 0 1 0 1.70l-2.06 2.06a2 2 0 0 1-1.41.586H4a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1z\"></path>",
        categories = "arrows,navigation,development,gaming",
        tags = "signpost,direction,right,east,forward,version control,waypoint",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Milestone,
    #[cfg(feature = "milk_off")]
    #[strum(props(
        svg = "<path d=\"M8 2h8\"></path><path d=\"M9 2v1.34M15 2v2.78a4 4 0 0 0 .672 2.21l.656.98a4 4 0 0 1 .672 2.22v1.13M7.8 7.8l-.128.19A4 4 0 0 0 7 10.21V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3\"></path><path d=\"M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 3.43.435\"></path><line y1=\"2\" x1=\"2\" x2=\"22\" y2=\"22\"></line>",
        categories = "food-beverage",
        tags = "lactose free,bottle,beverage,drink,water,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MilkOff,
    #[cfg(feature = "milk")]
    #[strum(props(
        svg = "<path d=\"M8 2h8\"></path><path d=\"M9 2v2.78a4 4 0 0 1-.672 2.21l-.656.98A4 4 0 0 0 7 10.21V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.78a4 4 0 0 0-.672-2.21l-.656-.984A4 4 0 0 1 15 4.78V2\"></path><path d=\"M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0\"></path>",
        categories = "food-beverage",
        tags = "lactose,bottle,beverage,drink,water,diet",
        contributors = "karsa-mistmere"
    ))]
    Milk,
    #[cfg(feature = "minimize_2")]
    #[strum(props(
        svg = "<path d=\"m14 10 7-7\"></path><path d=\"M20 10h-6V4\"></path><path d=\"m3 21 7-7\"></path><path d=\"M4 14h6v6\"></path>",
        categories = "arrows,layout,design",
        tags = "exit fullscreen,arrows,close,shrink",
        contributors = "colebemis,ericfennis"
    ))]
    Minimize2,
    #[cfg(feature = "minimize")]
    #[strum(props(
        svg = "<path d=\"M8 3v3a2 2 0 0 1-2 2H3\"></path><path d=\"M21 8h-3a2 2 0 0 1-2-2V3\"></path><path d=\"M3 16h3a2 2 0 0 1 2 2v3\"></path><path d=\"M16 21v-3a2 2 0 0 1 2-2h3\"></path>",
        categories = "layout,design",
        tags = "exit fullscreen,close,shrink",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Minimize,
    #[cfg(feature = "minus")]
    #[strum(props(
        svg = "<path d=\"M5 12h14\"></path>",
        categories = "math,development,text,tools",
        tags = "subtract,remove,decrease,decrement,reduce,negative,calculate,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    Minus,
    #[cfg(feature = "mirror_rectangular")]
    #[strum(props(
        svg = "<path d=\"M11 6 8 9\"></path><path d=\"m16 7-8 8\"></path><rect height=\"20\" rx=\"2\" y=\"2\" x=\"4\" width=\"16\"></rect>",
        categories = "science,home,tools",
        tags = "reflection,optics,glass,surface,image,physics,science,bathroom,decor,cosmetic,shiny,periscope,vanity",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    MirrorRectangular,
    #[cfg(feature = "mirror_round")]
    #[strum(props(
        svg = "<path d=\"M10 6.6 8.6 8\"></path><path d=\"M12 18v4\"></path><path d=\"M15 7.5 9.5 13\"></path><path d=\"M7 22h10\"></path><circle cy=\"10\" cx=\"12\" r=\"8\"></circle>",
        categories = "science,home,tools",
        tags = "reflection,optics,glass,surface,image,physics,science,bathroom,vanity,makeup,decor,cosmetic,shiny,periscope",
        contributors = "Muhammad-Aqib-Bashir,jamiemlaw,karsa-mistmere"
    ))]
    MirrorRound,
    #[cfg(feature = "monitor_check")]
    #[strum(props(
        svg = "<path d=\"m9 10 2 2 4-4\"></path><rect x=\"2\" y=\"3\" rx=\"2\" width=\"20\" height=\"14\"></rect><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_cloud")]
    #[strum(props(
        svg = "<path d=\"M11 13a3 3 0 1 1 2.83-4H14a2 2 0 0 1 0 4z\"></path><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path><rect y=\"3\" width=\"20\" x=\"2\" rx=\"2\" height=\"14\"></rect>",
        categories = "connectivity,devices,development",
        tags = "virtual machine,virtual desktop,vm,vdi,computing,remote work,monitoring,infrastructure,software as a service,saas,workstation,environment,tv,screen,display",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCloud,
    #[cfg(feature = "monitor_cog")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"m14.30 7.53.92-.382\"></path><path d=\"m15.22 4.85-.923-.383\"></path><path d=\"m16.85 3.22-.383-.924\"></path><path d=\"m16.85 8.77-.383.92\"></path><path d=\"m19.14 3.22.383-.924\"></path><path d=\"m19.53 9.69-.382-.924\"></path><path d=\"m20.77 4.85.924-.383\"></path><path d=\"m20.77 7.14.924.38\"></path><path d=\"M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7\"></path><path d=\"M8 21h8\"></path><circle r=\"3\" cx=\"18\" cy=\"6\"></circle>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,virtual machine,vm,executable,settings,cog,edit,gear,configuration,preferences,system,control panel,network,computing",
        contributors = "karsa-mistmere,colebemis,UsamaKhan"
    ))]
    MonitorCog,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"M22 12.30V15a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h8.69\"></path><path d=\"M8 21h8\"></path><circle cy=\"6\" r=\"3\" cx=\"19\"></circle>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "<path d=\"M12 13V7\"></path><path d=\"m15 10-3 3-3-3\"></path><rect height=\"14\" width=\"20\" y=\"3\" rx=\"2\" x=\"2\"></rect><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,download",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorDown,
    #[cfg(feature = "monitor_off")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"M17 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 1.18-1.82\"></path><path d=\"m2 2 20 20\"></path><path d=\"M8 21h8\"></path><path d=\"M8.65 3H20a2 2 0 0 1 2 2v10a2 2 0 0 1-.293 1.04\"></path>",
        categories = "connectivity,devices",
        tags = "share",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    MonitorOff,
    #[cfg(feature = "monitor_pause")]
    #[strum(props(
        svg = "<path d=\"M10 13V7\"></path><path d=\"M14 13V7\"></path><rect width=\"20\" x=\"2\" rx=\"2\" y=\"3\" height=\"14\"></rect><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path>",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "<path d=\"M15.03 9.44a.647.64 0 0 1 0 1.12l-4.06 2.35a.645.64 0 0 1-.968-.56V7.64a.645.64 0 0 1 .967-.56z\"></path><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path><rect width=\"20\" x=\"2\" height=\"14\" y=\"3\" rx=\"2\"></rect>",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "<path d=\"M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8\"></path><path d=\"M10 19v-3.96 3.15\"></path><path d=\"M7 19h5\"></path><rect y=\"12\" width=\"6\" height=\"10\" rx=\"2\" x=\"16\"></rect>",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "<path d=\"M5.5 20H8\"></path><path d=\"M17 9h.01\"></path><rect rx=\"2\" width=\"10\" height=\"16\" x=\"12\" y=\"4\"></rect><path d=\"M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4\"></path><circle cx=\"17\" cy=\"15\" r=\"1\"></circle>",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path><rect x=\"2\" rx=\"2\" y=\"3\" width=\"20\" height=\"14\"></rect><rect width=\"6\" rx=\"1\" y=\"7\" x=\"9\" height=\"6\"></rect>",
        categories = "connectivity,devices,multimedia",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "<path d=\"m9 10 3-3 3 3\"></path><path d=\"M12 13V7\"></path><rect rx=\"2\" width=\"20\" height=\"14\" x=\"2\" y=\"3\"></rect><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "<path d=\"m14.5 12.5-5-5\"></path><path d=\"m9.5 12.5 5-5\"></path><rect rx=\"2\" y=\"3\" height=\"14\" width=\"20\" x=\"2\"></rect><path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" width=\"20\" x=\"2\" height=\"14\"></rect><line x1=\"8\" y2=\"21\" x2=\"16\" y1=\"21\"></line><line x1=\"12\" x2=\"12\" y1=\"17\" y2=\"21\"></line>",
        categories = "connectivity,devices",
        tags = "tv,screen,display,virtual machine,vm",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    Monitor,
    #[cfg(feature = "moon_star")]
    #[strum(props(
        svg = "<path d=\"M18 5h4\"></path><path d=\"M20 3v4\"></path><path d=\"M20.98 12.48a9 9 0 1 1-9.47-9.47c.405-.022.61.46.40.803a6 6 0 0 0 8.26 8.26c.344-.215.82-.004.80.401\"></path>",
        categories = "accessibility,weather",
        tags = "dark,night,star",
        contributors = "karsa-mistmere"
    ))]
    MoonStar,
    #[cfg(feature = "moon")]
    #[strum(props(
        svg = "<path d=\"M20.98 12.48a9 9 0 1 1-9.47-9.47c.405-.022.61.46.40.803a6 6 0 0 0 8.26 8.26c.344-.215.82-.004.80.401\"></path>",
        categories = "accessibility",
        tags = "dark,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Moon,
    #[cfg(feature = "motorbike")]
    #[strum(props(
        svg = "<path d=\"m18 14-1-3\"></path><path d=\"m3 9 6 2a2 2 0 0 1 2-2h2a2 2 0 0 1 1.99 1.81\"></path><path d=\"M8 17h3a1 1 0 0 0 1-1 6 6 0 0 1 6-6 1 1 0 0 0 1-1v-.75A5 5 0 0 0 17 5\"></path><circle r=\"3\" cx=\"19\" cy=\"17\"></circle><circle r=\"3\" cy=\"17\" cx=\"5\"></circle>",
        categories = "transportation",
        tags = "moto,motorcycle,transport,vehicle,drive,ride,trip,race,racing,journey,delivery",
        contributors = "jamiemlaw"
    ))]
    Motorbike,
    #[cfg(feature = "mountain_snow")]
    #[strum(props(
        svg = "<path d=\"m8 3 4 8 5-5 5 15H2L8 3z\"></path><path d=\"M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19\"></path>",
        categories = "nature",
        tags = "alpine,climb,snow",
        contributors = "kerkeslager,ericfennis"
    ))]
    MountainSnow,
    #[cfg(feature = "mountain")]
    #[strum(props(
        svg = "<path d=\"m8 3 4 8 5-5 5 15H2L8 3z\"></path>",
        categories = "nature,gaming",
        tags = "climb,hike,rock",
        contributors = "kerkeslager,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Mountain,
    #[cfg(feature = "mouse_left")]
    #[strum(props(
        svg = "<path d=\"M12 7.31V10\"></path><path d=\"M5 10v5a7 7 0 0 0 14 0V9c0-3.52-2.60-6.51-6-7\"></path><circle cx=\"7\" r=\"2\" cy=\"4\"></circle>",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,marvfash"
    ))]
    MouseLeft,
    #[cfg(feature = "mouse_off")]
    #[strum(props(
        svg = "<path d=\"M12 6v.343\"></path><path d=\"M18.21 18.21A7 7 0 0 1 5 15V9a7 7 0 0 1 .782-3.21\"></path><path d=\"M19 13.34V9A7 7 0 0 0 8.56 2.90\"></path><path d=\"M22 22 2 2\"></path>",
        categories = "devices",
        tags = "device,scroll,click,disabled",
        contributors = "karsa-mistmere,mittalyashu,ericfennis"
    ))]
    MouseOff,
    #[cfg(feature = "mouse_pointer_2_off")]
    #[strum(props(
        svg = "<path d=\"m15.55 8.45 5.13 2.08a.5.5 0 0 1-.063.94l-6.12 1.58a2 2 0 0 0-1.43 1.43l-1.57 6.12a.5.5 0 0 1-.947.06L8.45 15.55\"></path><path d=\"M22 2 2 22\"></path><path d=\"m6.81 11.52-2.77-6.84a.495.49 0 0 1 .651-.651l6.84 2.77\"></path>",
        categories = "arrows,cursors",
        tags = "pointer,mouse,cursor,off,disable,arrow,navigation,selection,select,click,no-click,interaction",
        contributors = "ericfennis,csandman,domingasp,jguddas"
    ))]
    MousePointer2Off,
    #[cfg(feature = "mouse_pointer_2")]
    #[strum(props(
        svg = "<path d=\"M4.03 4.68a.495.49 0 0 1 .651-.651l16 6.5a.5.5 0 0 1-.063.94l-6.12 1.58a2 2 0 0 0-1.43 1.43l-1.57 6.12a.5.5 0 0 1-.947.06z\"></path>",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "ericfennis,csandman"
    ))]
    MousePointer2,
    #[cfg(feature = "mouse_pointer_ban")]
    #[strum(props(
        svg = "<path d=\"M2.03 2.68a.498.49 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.94L8.20 7.54a1 1 0 0 0-.66.66l-1.06 3.44a.5.5 0 0 1-.944.03z\"></path><circle r=\"6\" cy=\"16\" cx=\"16\"></circle><path d=\"m11.8 11.8 8.4 8.4\"></path>",
        categories = "arrows,cursors",
        tags = "wait,busy,loading,blocked,frozen,freeze",
        contributors = "danielbayley"
    ))]
    MousePointerBan,
    #[cfg(feature = "mouse_pointer_click")]
    #[strum(props(
        svg = "<path d=\"M14 4.1 12 6\"></path><path d=\"m5.1 8-2.9-.8\"></path><path d=\"m6 12-1.9 2\"></path><path d=\"M7.2 2.2 8 5.1\"></path><path d=\"M9.03 9.69a.498.49 0 0 1 .653-.653l11 4.5a.5.5 0 0 1-.074.94l-4.34 1.04a1 1 0 0 0-.74.73l-1.04 4.35a.5.5 0 0 1-.95.07z\"></path>",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    MousePointerClick,
    #[cfg(feature = "mouse_pointer")]
    #[strum(props(
        svg = "<path d=\"M12.58 12.58 19 19\"></path><path d=\"M3.68 3.03a.497.49 0 0 0-.651.65l6.5 15.99a.501.50 0 0 0 .947-.062l1.56-6.08a2 2 0 0 1 1.44-1.47l6.12-1.57a.5.5 0 0 0 .063-.947z\"></path>",
        categories = "arrows,cursors",
        tags = "click,select",
        contributors = "ashygee,ericfennis"
    ))]
    MousePointer,
    #[cfg(feature = "mouse_right")]
    #[strum(props(
        svg = "<path d=\"M12 7.31V10\"></path><path d=\"M19 10v5a7 7 0 0 1-14 0V9c0-3.52 2.60-6.51 6-7\"></path><circle cy=\"4\" cx=\"17\" r=\"2\"></circle>",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,marvfash"
    ))]
    MouseRight,
    #[cfg(feature = "mouse")]
    #[strum(props(
        svg = "<rect width=\"14\" x=\"5\" height=\"20\" rx=\"7\" y=\"2\"></rect><path d=\"M12 6v4\"></path>",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Mouse,
    #[cfg(feature = "move_3_d")]
    #[strum(props(
        svg = "<path d=\"M5 3v16h16\"></path><path d=\"m5 19 6-6\"></path><path d=\"m2 6 3-3 3 3\"></path><path d=\"m18 16 3 3-3 3\"></path>",
        categories = "design",
        tags = "arrows,axis,gizmo,coordinates,transform,translate",
        contributors = "lscheibel,ericfennis"
    ))]
    Move3D,
    #[cfg(feature = "move_diagonal_2")]
    #[strum(props(
        svg = "<path d=\"M19 13v6h-6\"></path><path d=\"M5 11V5h6\"></path><path d=\"m5 5 14 14\"></path>",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "<path d=\"M11 19H5v-6\"></path><path d=\"M13 5h6v6\"></path><path d=\"M19 5 5 19\"></path>",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal,
    #[cfg(feature = "move_down_left")]
    #[strum(props(
        svg = "<path d=\"M11 19H5V13\"></path><path d=\"M19 5L5 19\"></path>",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "siarie,ericfennis,karsa-mistmere,jonas-hoebenreich"
    ))]
    MoveDownLeft,
    #[cfg(feature = "move_down_right")]
    #[strum(props(
        svg = "<path d=\"M19 13V19H13\"></path><path d=\"M5 5L19 19\"></path>",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDownRight,
    #[cfg(feature = "move_down")]
    #[strum(props(
        svg = "<path d=\"M8 18L12 22L16 18\"></path><path d=\"M12 2V22\"></path>",
        categories = "arrows",
        tags = "arrow,direction,downwards,south",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDown,
    #[cfg(feature = "move_horizontal")]
    #[strum(props(
        svg = "<path d=\"m18 8 4 4-4 4\"></path><path d=\"M2 12h20\"></path><path d=\"m6 8-4 4 4 4\"></path>",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis,csandman"
    ))]
    MoveHorizontal,
    #[cfg(feature = "move_left")]
    #[strum(props(
        svg = "<path d=\"M6 8L2 12L6 16\"></path><path d=\"M2 12H22\"></path>",
        categories = "arrows",
        tags = "arrow,direction,back,west",
        contributors = "jonas-hoebenreich"
    ))]
    MoveLeft,
    #[cfg(feature = "move_right")]
    #[strum(props(
        svg = "<path d=\"M18 8L22 12L18 16\"></path><path d=\"M2 12H22\"></path>",
        categories = "arrows",
        tags = "arrow,direction,trend flat,east",
        contributors = "jonas-hoebenreich"
    ))]
    MoveRight,
    #[cfg(feature = "move_up_left")]
    #[strum(props(
        svg = "<path d=\"M5 11V5H11\"></path><path d=\"M5 5L19 19\"></path>",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpLeft,
    #[cfg(feature = "move_up_right")]
    #[strum(props(
        svg = "<path d=\"M13 5H19V11\"></path><path d=\"M19 5L5 19\"></path>",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpRight,
    #[cfg(feature = "move_up")]
    #[strum(props(
        svg = "<path d=\"M8 6L12 2L16 6\"></path><path d=\"M12 2V22\"></path>",
        categories = "arrows",
        tags = "arrow,direction,upwards,north",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUp,
    #[cfg(feature = "move_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 2v20\"></path><path d=\"m8 18 4 4 4-4\"></path><path d=\"m8 6 4-4 4 4\"></path>",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "<path d=\"M12 2v20\"></path><path d=\"m15 19-3 3-3-3\"></path><path d=\"m19 9 3 3-3 3\"></path><path d=\"M2 12h20\"></path><path d=\"m5 9-3 3 3 3\"></path><path d=\"m9 5 3-3 3 3\"></path>",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "<circle cy=\"18\" cx=\"8\" r=\"4\"></circle><path d=\"M12 18V2l7 4\"></path>",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "<circle cy=\"18\" r=\"4\" cx=\"12\"></circle><path d=\"M16 18V2\"></path>",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "<path d=\"M9 18V5l12-2v13\"></path><path d=\"m9 9 12-2\"></path><circle cx=\"6\" cy=\"18\" r=\"3\"></circle><circle cx=\"18\" cy=\"16\" r=\"3\"></circle>",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "<path d=\"M9 18V5l12-2v13\"></path><circle cy=\"18\" cx=\"6\" r=\"3\"></circle><circle cy=\"16\" r=\"3\" cx=\"18\"></circle>",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "<path d=\"M9.31 9.31 5 21l7-4 7 4-1.17-3.17\"></path><path d=\"M14.53 8.88 12 2l-1.17 3.17\"></path><line y1=\"2\" x2=\"22\" y2=\"22\" x1=\"2\"></line>",
        categories = "navigation",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Navigation2Off,
    #[cfg(feature = "navigation_2")]
    #[strum(props(
        svg = "<polygon points=\"12 2 19 21 12 17 5 21 12 2\"></polygon>",
        categories = "navigation",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation2,
    #[cfg(feature = "navigation_off")]
    #[strum(props(
        svg = "<path d=\"M8.43 8.43 3 11l8 2 2 8 2.57-5.43\"></path><path d=\"M17.39 11.73 22 2l-9.73 4.61\"></path><line y1=\"2\" x2=\"22\" y2=\"22\" x1=\"2\"></line>",
        categories = "navigation",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NavigationOff,
    #[cfg(feature = "navigation")]
    #[strum(props(
        svg = "<polygon points=\"3 11 22 2 13 21 11 13 3 11\"></polygon>",
        categories = "navigation",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation,
    #[cfg(feature = "network")]
    #[strum(props(
        svg = "<rect x=\"16\" y=\"16\" width=\"6\" height=\"6\" rx=\"1\"></rect><rect y=\"16\" x=\"2\" width=\"6\" rx=\"1\" height=\"6\"></rect><rect y=\"2\" rx=\"1\" x=\"9\" width=\"6\" height=\"6\"></rect><path d=\"M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3\"></path><path d=\"M12 12V8\"></path>",
        categories = "development",
        tags = "tree",
        contributors = "ericfennis,johnletey,csandman,karsa-mistmere"
    ))]
    Network,
    #[cfg(feature = "newspaper")]
    #[strum(props(
        svg = "<path d=\"M15 18h-5\"></path><path d=\"M18 14h-8\"></path><path d=\"M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-4 0v-9a2 2 0 0 1 2-2h2\"></path><rect y=\"6\" rx=\"1\" width=\"8\" height=\"4\" x=\"10\"></rect>",
        categories = "multimedia,communication",
        tags = "news,feed,home,magazine,article,headline",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Newspaper,
    #[cfg(feature = "nfc")]
    #[strum(props(
        svg = "<path d=\"M6 8.32a7.43 7.43 0 0 1 0 7.36\"></path><path d=\"M9.46 6.21a11.76 11.76 0 0 1 0 11.58\"></path><path d=\"M12.91 4.1a15.91 15.91 0 0 1 .01 15.8\"></path><path d=\"M16.37 2a20.16 20.16 0 0 1 0 20\"></path>",
        categories = "communication,finance,devices",
        tags = "contactless,payment,near-field communication",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Nfc,
    #[cfg(feature = "non_binary")]
    #[strum(props(
        svg = "<path d=\"M12 2v10\"></path><path d=\"m8.5 4 7 4\"></path><path d=\"m8.5 8 7-4\"></path><circle cy=\"17\" r=\"5\" cx=\"12\"></circle>",
        categories = "medical",
        tags = "gender,nonbinary,enby",
        contributors = "jamiemlaw"
    ))]
    NonBinary,
    #[cfg(feature = "notebook_pen")]
    #[strum(props(
        svg = "<path d=\"M13.4 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7.4\"></path><path d=\"M2 6h4\"></path><path d=\"M2 10h4\"></path><path d=\"M2 14h4\"></path><path d=\"M2 18h4\"></path><path d=\"M21.37 5.62a1 1 0 1 0-3.00-3.00l-5.01 5.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path>",
        categories = "text,social",
        tags = "pencil,notepad,notes,noted,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,eraser,rubber",
        contributors = "danielbayley"
    ))]
    NotebookPen,
    #[cfg(feature = "notebook_tabs")]
    #[strum(props(
        svg = "<path d=\"M2 6h4\"></path><path d=\"M2 10h4\"></path><path d=\"M2 14h4\"></path><path d=\"M2 18h4\"></path><rect height=\"20\" width=\"16\" x=\"4\" rx=\"2\" y=\"2\"></rect><path d=\"M15 2v20\"></path><path d=\"M15 7h5\"></path><path d=\"M15 12h5\"></path><path d=\"M15 17h5\"></path>",
        categories = "account,communication,social",
        tags = "notepad,notes,people,family,friends,acquaintances,contacts,details,addresses,phone numbers,directory,listing,networking,alphabetical,a-z,organizer,organiser,planner,diary,stationery",
        contributors = "danielbayley"
    ))]
    NotebookTabs,
    #[cfg(feature = "notebook_text")]
    #[strum(props(
        svg = "<path d=\"M2 6h4\"></path><path d=\"M2 10h4\"></path><path d=\"M2 14h4\"></path><path d=\"M2 18h4\"></path><rect height=\"20\" x=\"4\" y=\"2\" rx=\"2\" width=\"16\"></rect><path d=\"M9.5 8h5\"></path><path d=\"M9.5 12H16\"></path><path d=\"M9.5 16H14\"></path>",
        categories = "text,social",
        tags = "notepad,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,research,homework,lines,opened",
        contributors = "danielbayley"
    ))]
    NotebookText,
    #[cfg(feature = "notebook")]
    #[strum(props(
        svg = "<path d=\"M2 6h4\"></path><path d=\"M2 10h4\"></path><path d=\"M2 14h4\"></path><path d=\"M2 18h4\"></path><rect x=\"4\" width=\"16\" height=\"20\" y=\"2\" rx=\"2\"></rect><path d=\"M16 2v20\"></path>",
        categories = "text,communication,social,design",
        tags = "notepad,notes,stationery,sketchbook,moleskine,closure,strap,band,elastic,organizer,organiser,planner,diary,journal,writing,written,writer,reading,high school,university,college,academy,student,study,homework,research",
        contributors = "danielbayley"
    ))]
    Notebook,
    #[cfg(feature = "notepad_text_dashed")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M12 2v4\"></path><path d=\"M16 2v4\"></path><path d=\"M16 4h2a2 2 0 0 1 2 2v2\"></path><path d=\"M20 12v2\"></path><path d=\"M20 18v2a2 2 0 0 1-2 2h-1\"></path><path d=\"M13 22h-2\"></path><path d=\"M7 22H6a2 2 0 0 1-2-2v-2\"></path><path d=\"M4 14v-2\"></path><path d=\"M4 8V6a2 2 0 0 1 2-2h2\"></path><path d=\"M8 10h6\"></path><path d=\"M8 14h8\"></path><path d=\"M8 18h5\"></path>",
        categories = "text,social",
        tags = "notebook,notes,pages,paper,stationery,diary,journal,writing,write,written,draft,template,lines",
        contributors = "danielbayley"
    ))]
    NotepadTextDashed,
    #[cfg(feature = "notepad_text")]
    #[strum(props(
        svg = "<path d=\"M8 2v4\"></path><path d=\"M12 2v4\"></path><path d=\"M16 2v4\"></path><rect x=\"4\" rx=\"2\" y=\"4\" height=\"18\" width=\"16\"></rect><path d=\"M8 10h6\"></path><path d=\"M8 14h8\"></path><path d=\"M8 18h5\"></path>",
        categories = "text,social",
        tags = "notebook,notes,pages,paper,stationery,sketchbook,organizer,organiser,planner,diary,journal,writing,write,written,reading,high school,university,college,academy,student,study,homework,research,lines,opened",
        contributors = "danielbayley"
    ))]
    NotepadText,
    #[cfg(feature = "nut_off")]
    #[strum(props(
        svg = "<path d=\"M12 4V2\"></path><path d=\"M5 10v4a7.00 7.00 0 0 0 5.27 6.78c.412.10.802.29 1.10.592L12 22l.621-.621c.3-.3.69-.488 1.10-.592a7.01 7.01 0 0 0 4.12-2.93\"></path><path d=\"M19 10v3.34\"></path><path d=\"M12 12c-1.34-.573-1.90-1.00-2.5-2-.546.90-1.04 1.35-2.5 2-1.01-.644-1.46-1.08-2-2-1.02.71-1.69.91-3 1 1.08-1.04 1.75-2.03 2-3 .194-.776.84-1.55 1.79-2.21m11.65 5.99c.887-.457 1.28-.891 1.55-1.78 1.03.916 1.68 1.15 3 1-1.29-1.03-1.75-2.03-2-3-.5-2-4-4-8-4-.74 0-1.46.068-2.15.19\"></path><line y1=\"2\" y2=\"22\" x1=\"2\" x2=\"22\"></line>",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NutOff,
    #[cfg(feature = "nut")]
    #[strum(props(
        svg = "<path d=\"M12 4V2\"></path><path d=\"M5 10v4a7.00 7.00 0 0 0 5.27 6.78c.412.10.802.29 1.10.592L12 22l.621-.621c.3-.3.69-.488 1.10-.592A7.00 7.00 0 0 0 19 14v-4\"></path><path d=\"M12 4C8 4 4.5 6 4 8c-.243.97-.919 1.95-2 3 1.31-.082 1.97-.29 3-1 .54.92.98 1.35 2 2 1.45-.647 1.95-1.09 2.5-2 .595.99 1.15 1.42 2.5 2 1.31-.621 1.86-1.05 2.5-2 .629.97 1.16 1.42 2.5 2 1.20-.548 1.68-.967 2-2 1.03.916 1.68 1.15 3 1-1.29-1.03-1.75-2.03-2-3-.5-2-4-4-8-4Z\"></path>",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,diet",
        contributors = "karsa-mistmere"
    ))]
    Nut,
    #[cfg(feature = "octagon_alert")]
    #[strum(props(
        svg = "<path d=\"M12 16h.01\"></path><path d=\"M12 8v4\"></path><path d=\"M15.31 2a2 2 0 0 1 1.41.586l4.68 4.68A2 2 0 0 1 22 8.68v6.62a2 2 0 0 1-.586 1.41l-4.68 4.68a2 2 0 0 1-1.41.586H8.68a2 2 0 0 1-1.41-.586l-4.68-4.68A2 2 0 0 1 2 15.31V8.68a2 2 0 0 1 .586-1.41l4.68-4.68A2 2 0 0 1 8.68 2z\"></path>",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    OctagonAlert,
    #[cfg(feature = "octagon_minus")]
    #[strum(props(
        svg = "<path d=\"M2.58 16.72A2 2 0 0 1 2 15.31V8.68a2 2 0 0 1 .586-1.41l4.68-4.68A2 2 0 0 1 8.68 2h6.62a2 2 0 0 1 1.41.586l4.68 4.68A2 2 0 0 1 22 8.68v6.62a2 2 0 0 1-.586 1.41l-4.68 4.68a2 2 0 0 1-1.41.586H8.68a2 2 0 0 1-1.41-.586z\"></path><path d=\"M8 12h8\"></path>",
        categories = "transportation",
        tags = "stop,forbidden,subtract,remove,decrease,reduce,-,traffic,halt,restricted",
        contributors = "colebemis,jguddas"
    ))]
    OctagonMinus,
    #[cfg(feature = "octagon_pause")]
    #[strum(props(
        svg = "<path d=\"M10 15V9\"></path><path d=\"M14 15V9\"></path><path d=\"M2.58 16.72A2 2 0 0 1 2 15.31V8.68a2 2 0 0 1 .586-1.41l4.68-4.68A2 2 0 0 1 8.68 2h6.62a2 2 0 0 1 1.41.586l4.68 4.68A2 2 0 0 1 22 8.68v6.62a2 2 0 0 1-.586 1.41l-4.68 4.68a2 2 0 0 1-1.41.586H8.68a2 2 0 0 1-1.41-.586z\"></path>",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "mittalyashu,jguddas"
    ))]
    OctagonPause,
    #[cfg(feature = "octagon_x")]
    #[strum(props(
        svg = "<path d=\"m15 9-6 6\"></path><path d=\"M2.58 16.72A2 2 0 0 1 2 15.31V8.68a2 2 0 0 1 .586-1.41l4.68-4.68A2 2 0 0 1 8.68 2h6.62a2 2 0 0 1 1.41.586l4.68 4.68A2 2 0 0 1 22 8.68v6.62a2 2 0 0 1-.586 1.41l-4.68 4.68a2 2 0 0 1-1.41.586H8.68a2 2 0 0 1-1.41-.586z\"></path><path d=\"m9 9 6 6\"></path>",
        categories = "math,notifications",
        tags = "delete,stop,alert,warning,times,clear,math",
        contributors = "colebemis,ericfennis"
    ))]
    OctagonX,
    #[cfg(feature = "octagon")]
    #[strum(props(
        svg = "<path d=\"M2.58 16.72A2 2 0 0 1 2 15.31V8.68a2 2 0 0 1 .586-1.41l4.68-4.68A2 2 0 0 1 8.68 2h6.62a2 2 0 0 1 1.41.586l4.68 4.68A2 2 0 0 1 22 8.68v6.62a2 2 0 0 1-.586 1.41l-4.68 4.68a2 2 0 0 1-1.41.586H8.68a2 2 0 0 1-1.41-.586z\"></path>",
        categories = "shapes",
        tags = "stop,shape",
        contributors = "colebemis,jguddas"
    ))]
    Octagon,
    #[cfg(feature = "omega")]
    #[strum(props(
        svg = "<path d=\"M3 20h4.5a.5.5 0 0 0 .5-.5v-.282a.52.52 0 0 0-.247-.437 8 8 0 1 1 8.49-.001.52.52 0 0 0-.247.43v.282a.5.5 0 0 0 .5.5H21\"></path>",
        categories = "math,development,text,science",
        tags = "greek,symbol,mathematics,education,physics,engineering,ohms,electrical resistance,angular frequency,dynamical systems,astronomy,constellations,philosophy",
        contributors = "karsa-mistmere"
    ))]
    Omega,
    #[cfg(feature = "option")]
    #[strum(props(
        svg = "<path d=\"M3 3h6l6 18h6\"></path><path d=\"M14 3h7\"></path>",
        categories = "development",
        tags = "keyboard,key,mac,alt,button",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Option,
    #[cfg(feature = "orbit")]
    #[strum(props(
        svg = "<path d=\"M20.34 6.48A10 10 0 0 1 10.26 21.85\"></path><path d=\"M3.65 17.51A10 10 0 0 1 13.74 2.15\"></path><circle cx=\"12\" r=\"3\" cy=\"12\"></circle><circle cy=\"5\" r=\"2\" cx=\"19\"></circle><circle cy=\"19\" r=\"2\" cx=\"5\"></circle>",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "origami")]
    #[strum(props(
        svg = "<path d=\"M12 12V4a1 1 0 0 1 1-1h6.29a1 1 0 0 1 .651 1.75l-4.69 4.02\"></path><path d=\"m12 21-7.41-7.41A2 2 0 0 1 4 12.17V6.41a1.00 1.00 0 0 1 1.70-.707L20 20.00\"></path><path d=\"m12.21 3.38 8.41 14.96a1 1 0 0 1-.167 1.19l-1.16 1.16a1 1 0 0 1-.706.29H6.35a1 1 0 0 1-.625-.219L3.25 18.8a1 1 0 0 1 .631-1.78l4.16.027\"></path>",
        categories = "animals,design",
        tags = "paper,bird",
        contributors = "gurtt"
    ))]
    Origami,
    #[cfg(feature = "package_2")]
    #[strum(props(
        svg = "<path d=\"M12 3v6\"></path><path d=\"M16.76 3a2 2 0 0 1 1.8 1.1l2.23 4.47a2 2 0 0 1 .21.89V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9.47a2 2 0 0 1 .211-.894L5.45 4.1A2 2 0 0 1 7.24 3z\"></path><path d=\"M3.05 9.01h17.89\"></path>",
        categories = "files,development",
        tags = "box,container,storage,sealed,packed,unopened,undelivered,archive,zip",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Package2,
    #[cfg(feature = "package_check")]
    #[strum(props(
        svg = "<path d=\"M12 22V12\"></path><path d=\"m16 17 2 2 4-4\"></path><path d=\"M21 11.12V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.72l7 4a2 2 0 0 0 2 .001l1.32-.753\"></path><path d=\"M3.29 7 12 12l8.71-5\"></path><path d=\"m7.5 4.27 8.99 5.14\"></path>",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task,delivered",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[cfg(feature = "package_minus")]
    #[strum(props(
        svg = "<path d=\"M12 22V12\"></path><path d=\"M16 17h6\"></path><path d=\"M21 13V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.72l7 4a2 2 0 0 0 2 .001l1.67-.955\"></path><path d=\"M3.29 7 12 12l8.71-5\"></path><path d=\"m7.5 4.27 8.99 5.14\"></path>",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[cfg(feature = "package_open")]
    #[strum(props(
        svg = "<path d=\"M12 22v-9\"></path><path d=\"M15.17 2.21a1.67 1.67 0 0 1 1.63 0L21 4.57a1.93 1.93 0 0 1 0 3.36L8.82 14.79a1.65 1.65 0 0 1-1.64 0L3 12.43a1.93 1.93 0 0 1 0-3.36z\"></path><path d=\"M20 13v3.87a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13\"></path><path d=\"M21 12.43a1.93 1.93 0 0 0 0-3.36L8.83 2.2a1.64 1.64 0 0 0-1.63 0L3 4.57a1.93 1.93 0 0 0 0 3.36l12.18 6.86a1.63 1.63 0 0 0 1.63 0z\"></path>",
        categories = "files,development",
        tags = "box,container,storage,unpack,unarchive,unzip,opened,delivered",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "<path d=\"M12 22V12\"></path><path d=\"M16 17h6\"></path><path d=\"M19 14v6\"></path><path d=\"M21 10.53V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.72l7 4a2 2 0 0 0 2 .001l1.67-.955\"></path><path d=\"M3.29 7 12 12l8.71-5\"></path><path d=\"m7.5 4.27 8.99 5.14\"></path>",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "<path d=\"M12 22V12\"></path><path d=\"M20.27 18.27 22 20\"></path><path d=\"M21 10.49V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.72l7 4a2 2 0 0 0 2 .001l.98-.559\"></path><path d=\"M3.29 7 12 12l8.71-5\"></path><path d=\"m7.5 4.27 8.99 5.14\"></path><circle cx=\"18.5\" cy=\"16.5\" r=\"2.5\"></circle>",
        categories = "files,development",
        tags = "find,product process,lens",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "<path d=\"M12 22V12\"></path><path d=\"m16.5 14.5 5 5\"></path><path d=\"m16.5 19.5 5-5\"></path><path d=\"M21 10.5V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.72l7 4a2 2 0 0 0 2 .001l.13-.074\"></path><path d=\"M3.29 7 12 12l8.71-5\"></path><path d=\"m7.5 4.27 8.99 5.14\"></path>",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageX,
    #[cfg(feature = "package")]
    #[strum(props(
        svg = "<path d=\"M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z\"></path><path d=\"M12 22V12\"></path><polyline points=\"3.29 7 12 12 20.71 7\"></polyline><path d=\"m7.5 4.27 9 5.15\"></path>",
        categories = "files,development",
        tags = "box,container,storage,sealed,delivery,undelivered,unopened,packed,archive,zip,module",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley,jguddas,sezze"
    ))]
    Package,
    #[cfg(feature = "paint_bucket")]
    #[strum(props(
        svg = "<path d=\"M11 7 6 2\"></path><path d=\"M18.99 12H2.04\"></path><path d=\"M21.14 18.38A3.34 3.34 0 0 1 20 16.5a3.3 3.3 0 0 1-1.14 1.88c-.575.46-.855 1.02-.855 1.59A2 2 0 0 0 20 22a2 2 0 0 0 2-2.02c0-.58-.285-1.13-.855-1.59\"></path><path d=\"m8.5 4.5 2.14-2.14a1.20 1.20 0 0 1 1.70 0l7.29 7.29a1.20 1.20 0 0 1 0 1.70l-7.59 7.59a3.61 3.61 0 0 1-5.11 0l-3.88-3.88a3.61 3.61 0 0 1 0-5.11L5.67 7.33\"></path>",
        categories = "design,tools",
        tags = "fill,paint,bucket,color,colour",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[cfg(feature = "paint_roller")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"16\" height=\"6\" x=\"2\" y=\"2\"></rect><path d=\"M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2\"></path><rect height=\"6\" width=\"4\" rx=\"1\" y=\"16\" x=\"8\"></rect>",
        categories = "text,design,home,tools",
        tags = "brush,color,colour,decoration,diy",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PaintRoller,
    #[cfg(feature = "paintbrush_vertical")]
    #[strum(props(
        svg = "<path d=\"M10 2v2\"></path><path d=\"M14 2v4\"></path><path d=\"M17 2a1 1 0 0 1 1 1v9H6V3a1 1 0 0 1 1-1z\"></path><path d=\"M6 12a1 1 0 0 0-1 1v1a2 2 0 0 0 2 2h2a1 1 0 0 1 1 1v2.9a2 2 0 1 0 4 0V17a1 1 0 0 1 1-1h2a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1\"></path>",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PaintbrushVertical,
    #[cfg(feature = "paintbrush")]
    #[strum(props(
        svg = "<path d=\"m14.62 17.89-10.68-2.91\"></path><path d=\"M18.37 2.62a1 1 0 1 1 3.00 3.00L17.36 9.64a.5.5 0 0 0 0 .707l.944.94a2.41 2.41 0 0 1 0 3.40l-.944.94a.5.5 0 0 1-.707 0L8.35 7.34a.5.5 0 0 1 0-.707l.944-.944a2.41 2.41 0 0 1 3.40 0l.944.94a.5.5 0 0 0 .707 0z\"></path><path d=\"M9 8c-1.80 2.71-3.97 3.46-6.58 3.94a.507.50 0 0 0-.302.81l7.32 8.88a1 1 0 0 0 1.18.204C12.73 20.40 16 16.79 16 15\"></path>",
        categories = "text,design,photography,home,tools",
        tags = "brush,paintbrush,design,color,colour,decoration,diy",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[cfg(feature = "palette")]
    #[strum(props(
        svg = "<path d=\"M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z\"></path><circle r=\".5\" cy=\"6.5\" fill=\"currentColor\" cx=\"13.5\"></circle><circle r=\".5\" fill=\"currentColor\" cy=\"10.5\" cx=\"17.5\"></circle><circle cx=\"6.5\" cy=\"12.5\" r=\".5\" fill=\"currentColor\"></circle><circle cx=\"8.5\" fill=\"currentColor\" cy=\"7.5\" r=\".5\"></circle>",
        categories = "text,design,photography",
        tags = "colors,colours,theme,scheme,paint,watercolor,watercolour,artist",
        contributors = "ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    Palette,
    #[cfg(feature = "panda")]
    #[strum(props(
        svg = "<path d=\"M11.25 17.25h1.5L12 18z\"></path><path d=\"m15 12 2 2\"></path><path d=\"M18 6.5a.5.5 0 0 0-.5-.5\"></path><path d=\"M20.69 9.67a4.5 4.5 0 1 0-7.04-5.5 8.35 8.35 0 0 0-3.3 0 4.5 4.5 0 1 0-7.04 5.5C2.49 11.2 2 12.88 2 14.5 2 19.47 6.48 22 12 22s10-2.53 10-7.5c0-1.62-.48-3.3-1.3-4.83\"></path><path d=\"M6 6.5a.495.49 0 0 1 .5-.5\"></path><path d=\"m9 12-2 2\"></path>",
        categories = "animals",
        tags = "animal,wildlife,bear,zoo,bamboo",
        contributors = "chessurisme,karsa-mistmere"
    ))]
    Panda,
    #[cfg(feature = "panel_bottom_close")]
    #[strum(props(
        svg = "<rect width=\"18\" y=\"3\" x=\"3\" rx=\"2\" height=\"18\"></rect><path d=\"M3 15h18\"></path><path d=\"m15 8-3 3-3-3\"></path>",
        categories = "layout,arrows",
        tags = "drawer,dock,hide,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_dashed")]
    #[strum(props(
        svg = "<rect width=\"18\" rx=\"2\" x=\"3\" y=\"3\" height=\"18\"></rect><path d=\"M14 15h1\"></path><path d=\"M19 15h2\"></path><path d=\"M3 15h2\"></path><path d=\"M9 15h1\"></path>",
        categories = "layout",
        tags = "drawer,dock,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelBottomDashed,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" height=\"18\" x=\"3\" width=\"18\"></rect><path d=\"M3 15h18\"></path><path d=\"m9 10 3-3 3 3\"></path>",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "<rect x=\"3\" height=\"18\" rx=\"2\" y=\"3\" width=\"18\"></rect><path d=\"M3 15h18\"></path>",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "<rect y=\"3\" x=\"3\" width=\"18\" height=\"18\" rx=\"2\"></rect><path d=\"M9 3v18\"></path><path d=\"m16 15-3-3 3-3\"></path>",
        categories = "layout,arrows",
        tags = "primary,drawer,hide,chevron,<",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_dashed")]
    #[strum(props(
        svg = "<rect y=\"3\" x=\"3\" rx=\"2\" height=\"18\" width=\"18\"></rect><path d=\"M9 14v1\"></path><path d=\"M9 19v2\"></path><path d=\"M9 3v2\"></path><path d=\"M9 9v1\"></path>",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelLeftDashed,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" height=\"18\" y=\"3\" rx=\"2\"></rect><path d=\"M9 3v18\"></path><path d=\"m14 9 3 3-3 3\"></path>",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal,chevron,right,>",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left_right_dashed")]
    #[strum(props(
        svg = "<path d=\"M15 10V9\"></path><path d=\"M15 15v-1\"></path><path d=\"M15 21v-2\"></path><path d=\"M15 5V3\"></path><path d=\"M9 10V9\"></path><path d=\"M9 15v-1\"></path><path d=\"M9 21v-2\"></path><path d=\"M9 5V3\"></path><rect rx=\"2\" height=\"18\" x=\"3\" width=\"18\" y=\"3\"></rect>",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,vertical,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta,jguddas"
    ))]
    PanelLeftRightDashed,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" rx=\"2\" height=\"18\" width=\"18\"></rect><path d=\"M9 3v18\"></path>",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "<rect height=\"18\" x=\"3\" y=\"3\" rx=\"2\" width=\"18\"></rect><path d=\"M15 3v18\"></path><path d=\"m8 9 3 3-3 3\"></path>",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide,chevron,>",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_dashed")]
    #[strum(props(
        svg = "<rect rx=\"2\" y=\"3\" width=\"18\" height=\"18\" x=\"3\"></rect><path d=\"M15 14v1\"></path><path d=\"M15 19v2\"></path><path d=\"M15 3v2\"></path><path d=\"M15 9v1\"></path>",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelRightDashed,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "<rect x=\"3\" width=\"18\" y=\"3\" rx=\"2\" height=\"18\"></rect><path d=\"M15 3v18\"></path><path d=\"m10 15-3-3 3-3\"></path>",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal,chevron,left,<",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"18\" height=\"18\" y=\"3\" x=\"3\"></rect><path d=\"M15 3v18\"></path>",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_bottom_dashed")]
    #[strum(props(
        svg = "<path d=\"M14 15h1\"></path><path d=\"M14 9h1\"></path><path d=\"M19 15h2\"></path><path d=\"M19 9h2\"></path><path d=\"M3 15h2\"></path><path d=\"M3 9h2\"></path><path d=\"M9 15h1\"></path><path d=\"M9 9h1\"></path><rect y=\"3\" width=\"18\" height=\"18\" x=\"3\" rx=\"2\"></rect>",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,padding,margin,guide,layout,horizontal,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta,jguddas"
    ))]
    PanelTopBottomDashed,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "<rect y=\"3\" x=\"3\" rx=\"2\" height=\"18\" width=\"18\"></rect><path d=\"M3 9h18\"></path><path d=\"m9 16 3-3 3 3\"></path>",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide,chevron,up",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_dashed")]
    #[strum(props(
        svg = "<rect height=\"18\" rx=\"2\" x=\"3\" y=\"3\" width=\"18\"></rect><path d=\"M14 9h1\"></path><path d=\"M19 9h2\"></path><path d=\"M3 9h2\"></path><path d=\"M9 9h1\"></path>",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,padding,margin,guide,layout,bleed",
        contributors = "danielbayley,ericfennis,irvineacosta"
    ))]
    PanelTopDashed,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "<rect y=\"3\" x=\"3\" rx=\"2\" height=\"18\" width=\"18\"></rect><path d=\"M3 9h18\"></path><path d=\"m15 14-3 3-3-3\"></path>",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal,chevron,down",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" width=\"18\" height=\"18\" x=\"3\"></rect><path d=\"M3 9h18\"></path>",
        categories = "layout,design,development",
        tags = "drawer,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[cfg(feature = "panels_left_bottom")]
    #[strum(props(
        svg = "<rect x=\"3\" rx=\"2\" height=\"18\" width=\"18\" y=\"3\"></rect><path d=\"M9 3v18\"></path><path d=\"M9 15h12\"></path>",
        categories = "layout",
        tags = "drawers,sidebar,primary",
        contributors = "danielbayley"
    ))]
    PanelsLeftBottom,
    #[cfg(feature = "panels_right_bottom")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" x=\"3\" width=\"18\" y=\"3\"></rect><path d=\"M3 15h12\"></path><path d=\"M15 3v18\"></path>",
        categories = "layout",
        tags = "drawers,sidebar,secondary",
        contributors = "danielbayley"
    ))]
    PanelsRightBottom,
    #[cfg(feature = "panels_top_left")]
    #[strum(props(
        svg = "<rect y=\"3\" x=\"3\" rx=\"2\" width=\"18\" height=\"18\"></rect><path d=\"M3 9h18\"></path><path d=\"M9 21V9\"></path>",
        categories = "layout,design,development",
        tags = "menu bar,sidebar,primary,drawers,window,webpage,projects,overview",
        contributors = "colebemis,ericfennis"
    ))]
    PanelsTopLeft,
    #[cfg(feature = "paperclip")]
    #[strum(props(
        svg = "<path d=\"m16 6-8.41 8.58a2 2 0 0 0 2.82 2.82l8.41-8.58a4 4 0 1 0-5.65-5.65l-8.37 8.55a6 6 0 1 0 8.48 8.48l8.37-8.55\"></path>",
        categories = "text,design,files,mail",
        tags = "attachment,file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Paperclip,
    #[cfg(feature = "parentheses")]
    #[strum(props(
        svg = "<path d=\"M8 21s-4-3-4-9 4-9 4-9\"></path><path d=\"M16 3s4 3 4 9-4 9-4 9\"></path>",
        categories = "development,files,math",
        tags = "code,token,parenthesis,parens,brackets,parameters,arguments,args,input,call,math,formula,function,(,)",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Parentheses,
    #[cfg(feature = "parking_meter")]
    #[strum(props(
        svg = "<path d=\"M11 15h2\"></path><path d=\"M12 12v3\"></path><path d=\"M12 19v3\"></path><path d=\"M15.28 19a1 1 0 0 0 .948-.68l2.37-6.98a7 7 0 1 0-13.2 0l2.37 6.98a1 1 0 0 0 .948.68z\"></path><path d=\"M9 9a3 3 0 1 1 6 0\"></path>",
        categories = "transportation,navigation",
        tags = "driving,car park,pay,sidewalk,pavement",
        contributors = "danielbayley,jguddas"
    ))]
    ParkingMeter,
    #[cfg(feature = "party_popper")]
    #[strum(props(
        svg = "<path d=\"M5.8 11.3 2 22l10.7-3.79\"></path><path d=\"M4 3h.01\"></path><path d=\"M22 8h.01\"></path><path d=\"M15 2h.01\"></path><path d=\"M22 20h.01\"></path><path d=\"m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10\"></path><path d=\"m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11c-.11.7-.72 1.22-1.43 1.22H17\"></path><path d=\"m11 2 .33.82c.34.86-.2 1.82-1.11 1.98C9.52 4.9 9 5.52 9 6.23V7\"></path><path d=\"M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z\"></path>",
        categories = "emoji",
        tags = "emoji,congratulations,celebration,party,tada,🎉,🎊,excitement,exciting,excites,confetti",
        contributors = "karsa-mistmere"
    ))]
    PartyPopper,
    #[cfg(feature = "pause")]
    #[strum(props(
        svg = "<rect rx=\"1\" x=\"14\" y=\"3\" height=\"18\" width=\"5\"></rect><rect x=\"5\" y=\"3\" rx=\"1\" width=\"5\" height=\"18\"></rect>",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "<circle r=\"2\" cx=\"11\" cy=\"4\"></circle><circle cx=\"18\" cy=\"8\" r=\"2\"></circle><circle cy=\"16\" cx=\"20\" r=\"2\"></circle><path d=\"M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.04Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z\"></path>",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "<rect x=\"5\" width=\"14\" height=\"20\" rx=\"2\" y=\"2\"></rect><path d=\"M15 14h.01\"></path><path d=\"M9 6h6\"></path><path d=\"M9 10h6\"></path>",
        categories = "devices,gaming",
        tags = "computer,chassis",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PcCase,
    #[cfg(feature = "pen_line")]
    #[strum(props(
        svg = "<path d=\"M13 21h8\"></path><path d=\"M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .623.62l4.35-1.32a2 2 0 0 0 .83-.497z\"></path>",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    PenLine,
    #[cfg(feature = "pen_off")]
    #[strum(props(
        svg = "<path d=\"m10 10-6.15 6.16a2 2 0 0 0-.5.83l-1.32 4.36a.5.5 0 0 0 .622.62l4.35-1.32a2 2 0 0 0 .83-.5L14 13.98\"></path><path d=\"m12.82 7.17 4.35-4.34a1 1 0 1 1 3.98 3.98l-4.35 4.35\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "text,design,tools",
        tags = "disabled,inactive,non-editable,locked,read-only,unmodifiable,frozen,restricted,pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PenOff,
    #[cfg(feature = "pen_tool")]
    #[strum(props(
        svg = "<path d=\"M15.70 21.29a1 1 0 0 1-1.41 0l-1.58-1.58a1 1 0 0 1 0-1.41l5.58-5.58a1 1 0 0 1 1.41 0l1.58 1.58a1 1 0 0 1 0 1.41z\"></path><path d=\"m18 13-1.37-6.87a1 1 0 0 0-.746-.776L3.23 2.02a1 1 0 0 0-1.20 1.20L5.35 15.87a1 1 0 0 0 .776.74L13 18\"></path><path d=\"m2.3 2.3 7.28 7.28\"></path><circle cx=\"11\" cy=\"11\" r=\"2\"></circle>",
        categories = "text,design,cursors",
        tags = "vector,drawing,path",
        contributors = "ashygee,mittalyashu,ericfennis,jguddas"
    ))]
    PenTool,
    #[cfg(feature = "pen")]
    #[strum(props(
        svg = "<path d=\"M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .623.62l4.35-1.32a2 2 0 0 0 .83-.497z\"></path>",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pen,
    #[cfg(feature = "pencil_line")]
    #[strum(props(
        svg = "<path d=\"M13 21h8\"></path><path d=\"m15 5 4 4\"></path><path d=\"M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .623.62l4.35-1.32a2 2 0 0 0 .83-.497z\"></path>",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    PencilLine,
    #[cfg(feature = "pencil_off")]
    #[strum(props(
        svg = "<path d=\"m10 10-6.15 6.16a2 2 0 0 0-.5.83l-1.32 4.36a.5.5 0 0 0 .622.62l4.35-1.32a2 2 0 0 0 .83-.5L14 13.98\"></path><path d=\"m12.82 7.17 4.35-4.34a1 1 0 1 1 3.98 3.98l-4.35 4.35\"></path><path d=\"m15 5 4 4\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "design,cursors,tools,text",
        tags = "disabled,inactive,non-editable,locked,read-only,unmodifiable,frozen,restricted,rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley,karsa-mistmere"
    ))]
    PencilOff,
    #[cfg(feature = "pencil_ruler")]
    #[strum(props(
        svg = "<path d=\"M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13\"></path><path d=\"m8 6 2-2\"></path><path d=\"m18 16 2-2\"></path><path d=\"m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17\"></path><path d=\"M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .623.62l4.35-1.32a2 2 0 0 0 .83-.497z\"></path><path d=\"m15 5 4 4\"></path>",
        categories = "tools,design,layout,text",
        tags = "edit,create,draw,sketch,draft,writer,writing,stationery,artist,measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PencilRuler,
    #[cfg(feature = "pencil")]
    #[strum(props(
        svg = "<path d=\"M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .623.62l4.35-1.32a2 2 0 0 0 .83-.497z\"></path><path d=\"m15 5 4 4\"></path>",
        categories = "design,cursors,tools,text",
        tags = "rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Pencil,
    #[cfg(feature = "pentagon")]
    #[strum(props(
        svg = "<path d=\"M10.83 2.38a2 2 0 0 1 2.34 0l8 5.74a2 2 0 0 1 .73 2.25l-3.04 9.26a2 2 0 0 1-1.9 1.37H7.04a2 2 0 0 1-1.9-1.37L2.1 10.37a2 2 0 0 1 .73-2.25z\"></path>",
        categories = "shapes",
        tags = "shape",
        contributors = "danielbayley,jguddas"
    ))]
    Pentagon,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "<line x2=\"5\" y2=\"19\" x1=\"19\" y1=\"5\"></line><circle cy=\"6.5\" r=\"2.5\" cx=\"6.5\"></circle><circle cx=\"17.5\" r=\"2.5\" cy=\"17.5\"></circle>",
        categories = "math,development,finance,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "<circle cy=\"5\" cx=\"12\" r=\"1\"></circle><path d=\"m9 20 3-6 3 6\"></path><path d=\"m6 8 6 2 6-2\"></path><path d=\"M12 10v4\"></path>",
        categories = "accessibility,people",
        tags = "people,human,accessibility,stick figure",
        contributors = "mittalyashu,ericfennis"
    ))]
    PersonStanding,
    #[cfg(feature = "philippine_peso")]
    #[strum(props(
        svg = "<path d=\"M20 11H4\"></path><path d=\"M20 7H4\"></path><path d=\"M7 21V4a1 1 0 0 1 1-1h4a1 1 0 0 1 0 12H7\"></path>",
        categories = "finance",
        tags = "currency,peso,money,php",
        contributors = "jguddas,kasutu,karsa-mistmere"
    ))]
    PhilippinePeso,
    #[cfg(feature = "phone_call")]
    #[strum(props(
        svg = "<path d=\"M13 2a9 9 0 0 1 9 9\"></path><path d=\"M13 6a5 5 0 0 1 5 5\"></path><path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "connectivity,devices,communication",
        tags = "ring",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw,jguddas"
    ))]
    PhoneCall,
    #[cfg(feature = "phone_forwarded")]
    #[strum(props(
        svg = "<path d=\"M14 6h8\"></path><path d=\"m18 2 4 4-4 4\"></path><path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneForwarded,
    #[cfg(feature = "phone_incoming")]
    #[strum(props(
        svg = "<path d=\"M16 2v6h6\"></path><path d=\"m22 2-6 6\"></path><path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "<path d=\"m16 2 6 6\"></path><path d=\"m22 2-6 6\"></path><path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "<path d=\"M10.1 13.9a14 14 0 0 0 3.73 2.66 1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2 18 18 0 0 1-12.72-5.27\"></path><path d=\"M22 2 2 22\"></path><path d=\"M4.76 13.58A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 .244.47\"></path>",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "<path d=\"m16 8 6-6\"></path><path d=\"M22 8V2h-6\"></path><path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,jamiemlaw"
    ))]
    PhoneOutgoing,
    #[cfg(feature = "phone")]
    #[strum(props(
        svg = "<path d=\"M13.83 16.56a1 1 0 0 0 1.21-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.35a1 1 0 0 0-.292 1.23 14 14 0 0 0 6.39 6.38\"></path>",
        categories = "text,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Phone,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "<line y2=\"20\" x1=\"9\" x2=\"9\" y1=\"4\"></line><path d=\"M4 7c0-1.7 1.3-3 3-3h13\"></path><path d=\"M18 20c-1.7 0-3-1.3-3-3V4\"></path>",
        categories = "development,math",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "piano")]
    #[strum(props(
        svg = "<path d=\"M18.5 8c-1.4 0-2.6-.8-3.2-2A6.87 6.87 0 0 0 2 9v11a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-8.5C22 9.6 20.4 8 18.5 8\"></path><path d=\"M2 14h20\"></path><path d=\"M6 14v4\"></path><path d=\"M10 14v4\"></path><path d=\"M14 14v4\"></path><path d=\"M18 14v4\"></path>",
        categories = "multimedia,devices",
        tags = "music,audio,sound,noise,notes,chord,keys,octave,acoustic,instrument,play,pianist,performance,concert",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Piano,
    #[cfg(feature = "pickaxe")]
    #[strum(props(
        svg = "<path d=\"m14 13-8.38 8.38a1 1 0 0 1-3.00-3L11 9.99\"></path><path d=\"M15.97 4.02A13 13 0 0 0 5.90 2.37c-1.39.342-1.09 2.15.277 2.60a19.9 19.9 0 0 1 5.82 3.02\"></path><path d=\"M16.00 11.99a19.9 19.9 0 0 1 3.02 5.82c.444 1.36 2.26 1.67 2.60.278A13 13 0 0 0 20 8.06\"></path><path d=\"M18.35 3.35a1.20 1.20 0 0 0-1.70 0l-5.29 5.29a1.20 1.20 0 0 0 0 1.70l2.29 2.29a1.20 1.20 0 0 0 1.70 0l5.29-5.29a1.20 1.20 0 0 0 0-1.70z\"></path>",
        categories = "tools,gaming",
        tags = "mining,mine,land worker,extraction,labor,construction,progress,advancement,crafting,building,creation",
        contributors = "karsa-mistmere"
    ))]
    Pickaxe,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "<path d=\"M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4\"></path><rect y=\"13\" width=\"10\" rx=\"2\" x=\"12\" height=\"7\"></rect>",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "<path d=\"M2 10h6V4\"></path><path d=\"m2 4 6 6\"></path><path d=\"M21 10V7a2 2 0 0 0-2-2h-7\"></path><path d=\"M3 14v2a2 2 0 0 0 2 2h3\"></path><rect width=\"10\" x=\"12\" y=\"14\" height=\"7\" rx=\"1\"></rect>",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis,jguddas,karsa-mistmere"
    ))]
    PictureInPicture,
    #[cfg(feature = "piggy_bank")]
    #[strum(props(
        svg = "<path d=\"M11 17h3v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a3.16 3.16 0 0 0 2-2h1a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1h-1a5 5 0 0 0-2-4V3a4 4 0 0 0-3.2 1.6l-.3.4H11a6 6 0 0 0-6 6v1a5 5 0 0 0 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1z\"></path><path d=\"M16 10h.01\"></path><path d=\"M2 8v1a2 2 0 0 0 2 2h1\"></path>",
        categories = "finance",
        tags = "money,savings",
        contributors = "ericfennis,jamiemlaw"
    ))]
    PiggyBank,
    #[cfg(feature = "pilcrow_left")]
    #[strum(props(
        svg = "<path d=\"M14 3v11\"></path><path d=\"M14 9h-3a3 3 0 0 1 0-6h9\"></path><path d=\"M18 3v11\"></path><path d=\"M22 18H2l4-4\"></path><path d=\"m6 22-4-4\"></path>",
        categories = "text",
        tags = "direction,paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "zaaakher,danielbayley,jonas-hoebenreich"
    ))]
    PilcrowLeft,
    #[cfg(feature = "pilcrow_right")]
    #[strum(props(
        svg = "<path d=\"M10 3v11\"></path><path d=\"M10 9H7a1 1 0 0 1 0-6h8\"></path><path d=\"M14 3v11\"></path><path d=\"m18 14 4 4H2\"></path><path d=\"m22 18-4 4\"></path>",
        categories = "text",
        tags = "direction,paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "zaaakher,danielbayley,jonas-hoebenreich"
    ))]
    PilcrowRight,
    #[cfg(feature = "pilcrow")]
    #[strum(props(
        svg = "<path d=\"M13 4v16\"></path><path d=\"M17 4v16\"></path><path d=\"M19 4H9.5a4.5 4.5 0 0 0 0 9H13\"></path>",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Pilcrow,
    #[cfg(feature = "pill_bottle")]
    #[strum(props(
        svg = "<path d=\"M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4\"></path><path d=\"M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7\"></path><rect rx=\"1\" width=\"16\" y=\"2\" x=\"4\" height=\"5\"></rect>",
        categories = "medical",
        tags = "medicine,medication,prescription,drug,supplement,vitamin,capsule,jar,container,healthcare,pharmaceutical,tablet",
        contributors = "karsa-mistmere"
    ))]
    PillBottle,
    #[cfg(feature = "pill")]
    #[strum(props(
        svg = "<path d=\"m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z\"></path><path d=\"m8.5 8.5 7 7\"></path>",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,tablet,pharmacy",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Pill,
    #[cfg(feature = "pin_off")]
    #[strum(props(
        svg = "<path d=\"M12 17v5\"></path><path d=\"M15 9.34V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H7.89\"></path><path d=\"m2 2 20 20\"></path><path d=\"M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h11\"></path>",
        categories = "navigation",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "<path d=\"M12 17v5\"></path><path d=\"M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z\"></path>",
        categories = "navigation,account",
        tags = "save,map,lock,fix",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pin,
    #[cfg(feature = "pipette")]
    #[strum(props(
        svg = "<path d=\"m12 9-8.41 8.41A2 2 0 0 0 3 18.82v1.34a2 2 0 0 1-.586 1.41A2 2 0 0 1 3.82 21h1.34a2 2 0 0 0 1.41-.586L15 12\"></path><path d=\"m18 9 .4.4a1 1 0 1 1-3 3l-3.8-3.8a1 1 0 1 1 3-3l.4.4 3.4-3.4a1 1 0 1 1 3 3z\"></path><path d=\"m2 22 .414-.414\"></path>",
        categories = "text,design,science",
        tags = "eye dropper,color picker,lab,chemistry",
        contributors = "Andreto,ericfennis,karsa-mistmere,jguddas"
    ))]
    Pipette,
    #[cfg(feature = "pizza")]
    #[strum(props(
        svg = "<path d=\"m12 14-1 1\"></path><path d=\"m13.75 18.25-1.25 1.42\"></path><path d=\"M17.77 5.65a15.68 15.68 0 0 0-12.12 12.12\"></path><path d=\"M18.8 9.3a1 1 0 0 0 2.1 7.7\"></path><path d=\"M21.96 20.73a1 1 0 0 1-1.23 1.23l-18-5a1 1 0 0 1-.695-1.23A19.68 19.68 0 0 1 15.73 2.03a1 1 0 0 1 1.23.695z\"></path>",
        categories = "food-beverage",
        tags = "pie,quiche,food",
        contributors = "karsa-mistmere,ericfennis,jguddas,jamiemlaw"
    ))]
    Pizza,
    #[cfg(feature = "plane_landing")]
    #[strum(props(
        svg = "<path d=\"M2 22h20\"></path><path d=\"M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z\"></path>",
        categories = "transportation,travel",
        tags = "arrival,plane,trip,airplane,landing",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneLanding,
    #[cfg(feature = "plane_takeoff")]
    #[strum(props(
        svg = "<path d=\"M2 22h20\"></path><path d=\"M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z\"></path>",
        categories = "transportation,travel",
        tags = "departure,plane,trip,airplane,takeoff",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneTakeoff,
    #[cfg(feature = "plane")]
    #[strum(props(
        svg = "<path d=\"M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z\"></path>",
        categories = "transportation,travel,navigation",
        tags = "plane,trip,airplane",
        contributors = "ahtohbi4,csandman,ericfennis"
    ))]
    Plane,
    #[cfg(feature = "play")]
    #[strum(props(
        svg = "<path d=\"M5 5a2 2 0 0 1 3.00-1.72l11.99 6.99a2 2 0 0 1 .003 3.45l-12 7A2 2 0 0 1 5 19z\"></path>",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Play,
    #[cfg(feature = "plug_2")]
    #[strum(props(
        svg = "<path d=\"M9 2v6\"></path><path d=\"M15 2v6\"></path><path d=\"M12 17v5\"></path><path d=\"M5 8h14\"></path><path d=\"M6 11V8h12v3a6 6 0 1 1-12 0Z\"></path>",
        categories = "devices,development",
        tags = "electricity,energy,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[cfg(feature = "plug_zap")]
    #[strum(props(
        svg = "<path d=\"M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z\"></path><path d=\"m2 22 3-3\"></path><path d=\"M7.5 13.5 10 11\"></path><path d=\"M10.5 16.5 13 14\"></path><path d=\"m18 3-4 4h6l-4 4\"></path>",
        categories = "devices",
        tags = "electricity,energy,electronics,charge,charging,battery,connect",
        contributors = "mittalyashu,ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    PlugZap,
    #[cfg(feature = "plug")]
    #[strum(props(
        svg = "<path d=\"M12 22v-5\"></path><path d=\"M15 8V2\"></path><path d=\"M17 8a1 1 0 0 1 1 1v4a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1z\"></path><path d=\"M9 8V2\"></path>",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet,power,voltage,current,charger",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Plug,
    #[cfg(feature = "plus")]
    #[strum(props(
        svg = "<path d=\"M5 12h14\"></path><path d=\"M12 5v14\"></path>",
        categories = "math,tools,development,text,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis"
    ))]
    Plus,
    #[cfg(feature = "pocket_knife")]
    #[strum(props(
        svg = "<path d=\"M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2\"></path><path d=\"M18 6h.01\"></path><path d=\"M6 18h.01\"></path><path d=\"M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z\"></path><path d=\"M18 11.66V22a4 4 0 0 0 4-4V6\"></path>",
        categories = "tools",
        tags = "swiss army knife,penknife,multi-tool,multitask,blade,cutter,gadget,corkscrew",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PocketKnife,
    #[cfg(feature = "podcast")]
    #[strum(props(
        svg = "<path d=\"M13 17a1 1 0 1 0-2 0l.5 4.5a0.5 0.5 0 0 0 1 0z\" fill=\"currentColor\"></path><path d=\"M16.85 18.58a9 9 0 1 0-9.7 0\"></path><path d=\"M8 14a5 5 0 1 1 8 0\"></path><circle cx=\"12\" cy=\"11\" fill=\"currentColor\" r=\"1\"></circle>",
        categories = "multimedia,social",
        tags = "audio,music,mic,talk,voice,subscribe,subscription,stream",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere,jguddas"
    ))]
    Podcast,
    #[cfg(feature = "pointer_off")]
    #[strum(props(
        svg = "<path d=\"M10 4.5V4a2 2 0 0 0-2.41-1.95\"></path><path d=\"M13.9 8.4a2 2 0 0 0-1.26-1.29\"></path><path d=\"M21.7 16.2A8 8 0 0 0 22 14v-3a2 2 0 1 0-4 0v-1a2 2 0 0 0-3.63-1.15\"></path><path d=\"m7 15-1.8-1.8a2 2 0 0 0-2.79 2.86L6 19.7a7.74 7.74 0 0 0 6 2.3h2a8 8 0 0 0 5.65-2.34\"></path><path d=\"M6 6v8\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis,jguddas"
    ))]
    PointerOff,
    #[cfg(feature = "pointer")]
    #[strum(props(
        svg = "<path d=\"M22 14a8 8 0 0 1-8 8\"></path><path d=\"M18 11v-1a2 2 0 0 0-2-2a2 2 0 0 0-2 2\"></path><path d=\"M14 10V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1\"></path><path d=\"M10 9.5V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v10\"></path><path d=\"M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15\"></path>",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis"
    ))]
    Pointer,
    #[cfg(feature = "popcorn")]
    #[strum(props(
        svg = "<path d=\"M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4\"></path><path d=\"M10 22 9 8\"></path><path d=\"m14 22 1-14\"></path><path d=\"M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z\"></path>",
        categories = "food-beverage,multimedia",
        tags = "cinema,movies,films,salted,sweet,sugar,candy,snack",
        contributors = "danielbayley"
    ))]
    Popcorn,
    #[cfg(feature = "popsicle")]
    #[strum(props(
        svg = "<path d=\"M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z\"></path><path d=\"m22 22-5.5-5.5\"></path>",
        categories = "food-beverage",
        tags = "ice lolly,ice cream,sweet,food",
        contributors = "danielbayley"
    ))]
    Popsicle,
    #[cfg(feature = "pound_sterling")]
    #[strum(props(
        svg = "<path d=\"M18 7c0-5.33-8-5.33-8 0\"></path><path d=\"M10 7v14\"></path><path d=\"M6 21h12\"></path><path d=\"M6 13h10\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[cfg(feature = "power_off")]
    #[strum(props(
        svg = "<path d=\"M18.36 6.64A9 9 0 0 1 20.77 15\"></path><path d=\"M6.16 6.16a9 9 0 1 0 12.68 12.68\"></path><path d=\"M12 2v4\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "connectivity",
        tags = "on,off,device,switch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PowerOff,
    #[cfg(feature = "power")]
    #[strum(props(
        svg = "<path d=\"M12 2v10\"></path><path d=\"M18.4 6.6a9 9 0 1 1-12.77.04\"></path>",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Power,
    #[cfg(feature = "presentation")]
    #[strum(props(
        svg = "<path d=\"M2 3h20\"></path><path d=\"M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3\"></path><path d=\"m7 21 5-5 5 5\"></path>",
        categories = "multimedia,photography,devices,communication,design",
        tags = "screen,whiteboard,marker pens,markers,blackboard,chalk,easel,school,learning,lesson,office,meeting,project,planning",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Presentation,
    #[cfg(feature = "printer_check")]
    #[strum(props(
        svg = "<path d=\"M13.5 22H7a1 1 0 0 1-1-1v-6a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v.5\"></path><path d=\"m16 19 2 2 4-4\"></path><path d=\"M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2\"></path><path d=\"M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6\"></path>",
        categories = "devices",
        tags = "fax,office,device,success,printed",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    PrinterCheck,
    #[cfg(feature = "printer_x")]
    #[strum(props(
        svg = "<path d=\"M12.53 22H7a1 1 0 0 1-1-1v-6a1 1 0 0 1 1-1h6.37\"></path><path d=\"m16.5 16.5 5 5\"></path><path d=\"m16.5 21.5 5-5\"></path><path d=\"M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.5\"></path><path d=\"M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6\"></path>",
        categories = "devices",
        tags = "fax,office,device,cross,cancel,remove,error",
        contributors = "colebemis,csandman,ericfennis,jguddas,lt25106"
    ))]
    PrinterX,
    #[cfg(feature = "printer")]
    #[strum(props(
        svg = "<path d=\"M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2\"></path><path d=\"M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6\"></path><rect y=\"14\" width=\"12\" rx=\"1\" x=\"6\" height=\"8\"></rect>",
        categories = "devices",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "<path d=\"M5 7 3 5\"></path><path d=\"M9 6V3\"></path><path d=\"m13 7 2-2\"></path><circle cx=\"9\" cy=\"13\" r=\"3\"></circle><path d=\"M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17\"></path><path d=\"M16 16h2\"></path>",
        categories = "multimedia,photography,devices,communication",
        tags = "cinema,film,movie,home video,presentation,slideshow,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Projector,
    #[cfg(feature = "proportions")]
    #[strum(props(
        svg = "<rect rx=\"2\" y=\"4\" x=\"2\" width=\"20\" height=\"16\"></rect><path d=\"M12 9v11\"></path><path d=\"M2 9h13a2 2 0 0 1 2 2v9\"></path>",
        categories = "layout,design,photography,devices",
        tags = "screens,sizes,rotate,rotation,adjust,aspect ratio,16:9,widescreen,4:3,resolution,responsive,mobile,desktop,dimensions,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Proportions,
    #[cfg(feature = "puzzle")]
    #[strum(props(
        svg = "<path d=\"M15.39 4.39a1 1 0 0 0 1.68-.474 2.5 2.5 0 1 1 3.01 3.01 1 1 0 0 0-.474 1.68l1.68 1.68a2.41 2.41 0 0 1 0 3.41L19.61 15.39a1 1 0 0 1-1.68-.474 2.5 2.5 0 1 0-3.01 3.01 1 1 0 0 1 .474 1.68l-1.68 1.68a2.41 2.41 0 0 1-3.41 0L8.61 19.61a1 1 0 0 0-1.68.47 2.5 2.5 0 1 1-3.01-3.01 1 1 0 0 0 .474-1.68l-1.68-1.68a2.41 2.41 0 0 1 0-3.41L4.39 8.61a1 1 0 0 1 1.68.47 2.5 2.5 0 1 0 3.01-3.01 1 1 0 0 1-.474-1.68l1.68-1.68a2.41 2.41 0 0 1 3.41 0z\"></path>",
        categories = "development,gaming",
        tags = "component,module,part,piece",
        contributors = "karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    Puzzle,
    #[cfg(feature = "pyramid")]
    #[strum(props(
        svg = "<path d=\"M2.5 16.88a1 1 0 0 1-.32-1.43l9-13.02a1 1 0 0 1 1.64 0l9 13.01a1 1 0 0 1-.32 1.44l-8.51 4.86a2 2 0 0 1-1.98 0Z\"></path><path d=\"M12 2v20\"></path>",
        categories = "shapes,math,travel",
        tags = "prism,triangle,triangular,hierarchy,structure,geometry,ancient,egyptian,landmark,tourism",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Pyramid,
    #[cfg(feature = "qr_code")]
    #[strum(props(
        svg = "<rect y=\"3\" height=\"5\" width=\"5\" rx=\"1\" x=\"3\"></rect><rect width=\"5\" y=\"3\" rx=\"1\" x=\"16\" height=\"5\"></rect><rect width=\"5\" rx=\"1\" height=\"5\" x=\"3\" y=\"16\"></rect><path d=\"M21 16h-3a2 2 0 0 0-2 2v3\"></path><path d=\"M21 21v.01\"></path><path d=\"M12 7v3a2 2 0 0 1-2 2H7\"></path><path d=\"M3 12h.01\"></path><path d=\"M12 3h.01\"></path><path d=\"M12 16v.01\"></path><path d=\"M16 12h1\"></path><path d=\"M21 12v.01\"></path><path d=\"M12 21v-1\"></path>",
        categories = "development,social",
        tags = "barcode,scan,link,url,information,digital",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    QrCode,
    #[cfg(feature = "quote")]
    #[strum(props(
        svg = "<path d=\"M16 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z\"></path><path d=\"M5 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z\"></path>",
        categories = "text",
        tags = "quotation",
        contributors = "Billiam,jguddas,karsa-mistmere"
    ))]
    Quote,
    #[cfg(feature = "rabbit")]
    #[strum(props(
        svg = "<path d=\"M13 16a3 3 0 0 1 2.24 5\"></path><path d=\"M18 12h.01\"></path><path d=\"M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1 1 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1a3 3 0 0 0-3 3\"></path><path d=\"M20 8.54V4a2 2 0 1 0-4 0v3\"></path><path d=\"M7.61 12.52a3 3 0 1 0-1.6 4.3\"></path>",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = "danielbayley"
    ))]
    Rabbit,
    #[cfg(feature = "radar")]
    #[strum(props(
        svg = "<path d=\"M19.07 4.93A10 10 0 0 0 6.99 3.34\"></path><path d=\"M4 6h.01\"></path><path d=\"M2.29 9.62A10 10 0 1 0 21.31 8.35\"></path><path d=\"M16.24 7.76A6 6 0 1 0 8.23 16.67\"></path><path d=\"M12 18h.01\"></path><path d=\"M17.99 11.66A6 6 0 0 1 15.77 16.67\"></path><circle r=\"2\" cx=\"12\" cy=\"12\"></circle><path d=\"m13.41 10.59 5.66-5.66\"></path>",
        categories = "navigation,security,communication",
        tags = "scan,sonar,detect,find,locate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radar,
    #[cfg(feature = "radiation")]
    #[strum(props(
        svg = "<path d=\"M12 12h.01\"></path><path d=\"M14 15.46a4 4 0 0 1-4 0L7.52 19.74 A 1 1 0 0 0 7.99 21.16 10 10 0 0 0 16.00 21.16 1 1 0 0 0 16.47 19.74z\"></path><path d=\"M16 12a4 4 0 0 0-2-3.46l2.47-4.28a1 1 0 0 1 1.46-.305 10 10 0 0 1 4.00 6.94A1 1 0 0 1 21 12z\"></path><path d=\"M8 12a4 4 0 0 1 2-3.46L7.52 4.25a1 1 0 0 0-1.46-.305 10 10 0 0 0-4.00 6.94A1 1 0 0 0 3 12z\"></path>",
        categories = "science",
        tags = "radioactive,nuclear,fallout,waste,atomic,physics,particle,element,molecule",
        contributors = "karsa-mistmere,danielbayley,ericfennis"
    ))]
    Radiation,
    #[cfg(feature = "radical")]
    #[strum(props(
        svg = "<path d=\"M3 12h3.28a1 1 0 0 1 .948.68l2.29 7.93a.5.5 0 0 0 .96-.044L13.82 4.77A1 1 0 0 1 14.79 4H21\"></path>",
        categories = "development,math",
        tags = "calculate,formula,math,operator,root,square,symbol",
        contributors = "smnandre,jguddas"
    ))]
    Radical,
    #[cfg(feature = "radio_off")]
    #[strum(props(
        svg = "<path d=\"M13.41 13.41a2 2 0 1 1-2.82-2.82\"></path><path d=\"M16.24 7.76a6 6 0 0 1 1.74 4.57\"></path><path d=\"M19.07 4.93a10 10 0 0 1 2.23 10.72\"></path><path d=\"m2 2 20 20\"></path><path d=\"M4.92 19.06a10 10 0 0 1 0-14.13\"></path><path d=\"M7.75 16.23a6 6 0 0 1 0-8.47\"></path>",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "kongsgard"
    ))]
    RadioOff,
    #[cfg(feature = "radio_receiver")]
    #[strum(props(
        svg = "<path d=\"M5 16v2\"></path><path d=\"M19 16v2\"></path><rect height=\"8\" rx=\"2\" width=\"20\" y=\"8\" x=\"2\"></rect><path d=\"M18 12h.01\"></path>",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[cfg(feature = "radio_tower")]
    #[strum(props(
        svg = "<path d=\"M4.9 16.1C1 12.2 1 5.8 4.9 1.9\"></path><path d=\"M7.8 4.7a6.14 6.14 0 0 0-.8 7.5\"></path><circle r=\"2\" cy=\"9\" cx=\"12\"></circle><path d=\"M16.2 4.8c2 2 2.26 5.11.8 7.47\"></path><path d=\"M19.1 1.9a9.96 9.96 0 0 1 0 14.1\"></path><path d=\"M9.5 18h5\"></path><path d=\"m8 22 4-11 4 11\"></path>",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[cfg(feature = "radio")]
    #[strum(props(
        svg = "<path d=\"M16.24 7.76a6 6 0 0 1 0 8.47\"></path><path d=\"M19.07 4.93a10 10 0 0 1 0 14.13\"></path><path d=\"M4.92 19.06a10 10 0 0 1 0-14.13\"></path><path d=\"M7.75 16.23a6 6 0 0 1 0-8.47\"></path><circle r=\"2\" cy=\"12\" cx=\"12\"></circle>",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    Radio,
    #[cfg(feature = "radius")]
    #[strum(props(
        svg = "<path d=\"M20.34 17.52a10 10 0 1 0-2.82 2.82\"></path><circle cy=\"19\" cx=\"19\" r=\"2\"></circle><path d=\"m13.41 13.41 4.18 4.18\"></path><circle r=\"2\" cx=\"12\" cy=\"12\"></circle>",
        categories = "shapes,math,design,tools",
        tags = "shape,circle,geometry,trigonometry,radii,calculate,measure,size",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radius,
    #[cfg(feature = "rainbow")]
    #[strum(props(
        svg = "<path d=\"M22 17a10 10 0 0 0-20 0\"></path><path d=\"M6 17a6 6 0 0 1 12 0\"></path><path d=\"M10 17a2 2 0 0 1 4 0\"></path>",
        categories = "weather",
        tags = "colors,colours,spectrum,light,prism,arc,clear,sunshine",
        contributors = "danielbayley"
    ))]
    Rainbow,
    #[cfg(feature = "rat")]
    #[strum(props(
        svg = "<path d=\"M13 22H4a2 2 0 0 1 0-4h12\"></path><path d=\"M13.23 18a3 3 0 0 0-2.2-5\"></path><path d=\"M16 9h.01\"></path><path d=\"M16.82 3.94a3 3 0 1 1 3.23 4.86l1.81 2.58a1.5 1.5 0 0 1-1.5 2.1l-2.87-.453a3 3 0 0 0-3.5 3\"></path><path d=\"M17 4.98a3 3 0 1 0-5.2 2.05A7 7 0 0 0 4 14.01 4 4 0 0 0 8 18\"></path>",
        categories = "animals",
        tags = "mouse,mice,gerbil,rodent,pet,pest,plague,disease",
        contributors = "henri42,jguddas,karsa-mistmere,danielbayley"
    ))]
    Rat,
    #[cfg(feature = "ratio")]
    #[strum(props(
        svg = "<rect x=\"6\" height=\"20\" width=\"12\" y=\"2\" rx=\"2\"></rect><rect x=\"2\" y=\"6\" rx=\"2\" height=\"12\" width=\"20\"></rect>",
        categories = "layout,design,photography",
        tags = "screens,sizes,rotate,rotation,adjust,aspect ratio,proportions,16:9,widescreen,4:3,resolution,responsive,mobile,desktop,dimensions,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[cfg(feature = "receipt_cent")]
    #[strum(props(
        svg = "<path d=\"M12 7v10\"></path><path d=\"M14.82 14.82a4 4 0 0 1-5.65 0 4 4 0 0 1 0-5.65 4 4 0 0 1 5.65 0\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,cents,dollar,usd,$,¢",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptCent,
    #[cfg(feature = "receipt_euro")]
    #[strum(props(
        svg = "<path d=\"M15.82 14.82a4 4 0 0 1-5.65 0 4 4 0 0 1 0-5.65 4 4 0 0 1 5.65 0\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M8 12h5\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,€",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptEuro,
    #[cfg(feature = "receipt_indian_rupee")]
    #[strum(props(
        svg = "<path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M8 11h8\"></path><path d=\"M8 7h8\"></path><path d=\"M9 7a4 4 0 0 1 0 8H8l3 2\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,inr,₹",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptIndianRupee,
    #[cfg(feature = "receipt_japanese_yen")]
    #[strum(props(
        svg = "<path d=\"m12 10 3-3\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M9 11h6\"></path><path d=\"M9 15h6\"></path><path d=\"m9 7 3 3v7\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,jpy,¥",
        contributors = "it-is-not,ericfennis,karsa-mistmere"
    ))]
    ReceiptJapaneseYen,
    #[cfg(feature = "receipt_pound_sterling")]
    #[strum(props(
        svg = "<path d=\"M10 17V9.5a1 1 0 0 1 5 0\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M8 13h5\"></path><path d=\"M8 17h7\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,british,currency,gbp,£",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptPoundSterling,
    #[cfg(feature = "receipt_russian_ruble")]
    #[strum(props(
        svg = "<path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M8 11h5a2 2 0 0 0 0-4h-3v10\"></path><path d=\"M8 15h5\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,rub,₽",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptRussianRuble,
    #[cfg(feature = "receipt_swiss_franc")]
    #[strum(props(
        svg = "<path d=\"M10 11h4\"></path><path d=\"M10 17V7h5\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path><path d=\"M8 15h5\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,chf,₣",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ReceiptSwissFranc,
    #[cfg(feature = "receipt_text")]
    #[strum(props(
        svg = "<path d=\"M13 16H8\"></path><path d=\"M14 8H8\"></path><path d=\"M16 12H8\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,details,small print,terms,conditions,contract",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReceiptText,
    #[cfg(feature = "receipt_turkish_lira")]
    #[strum(props(
        svg = "<path d=\"M10 7v10a5 5 0 0 0 5-5\"></path><path d=\"m14 8-6 3\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,try,₺",
        contributors = "danielbayley,jamiemlaw,karsa-mistmere"
    ))]
    ReceiptTurkishLira,
    #[cfg(feature = "receipt")]
    #[strum(props(
        svg = "<path d=\"M12 17V7\"></path><path d=\"M16 8h-6a2 2 0 0 0 0 4h4a2 2 0 0 1 0 4H8\"></path><path d=\"M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z\"></path>",
        categories = "finance,travel",
        tags = "bill,voucher,slip,check,counterfoil,currency,dollar,usd,$",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[cfg(feature = "rectangle_circle")]
    #[strum(props(
        svg = "<path d=\"M14 4v16H3a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z\"></path><circle cx=\"14\" cy=\"12\" r=\"8\"></circle>",
        categories = "development,text",
        tags = "compose,keyboard,key,button",
        contributors = "zefir-git,jguddas"
    ))]
    RectangleCircle,
    #[cfg(feature = "rectangle_ellipsis")]
    #[strum(props(
        svg = "<rect y=\"6\" rx=\"2\" x=\"2\" height=\"12\" width=\"20\"></rect><path d=\"M12 12h.01\"></path><path d=\"M17 12h.01\"></path><path d=\"M7 12h.01\"></path>",
        categories = "text,development",
        tags = "login,password,authenticate,2fa,field,fill,ellipsis,et cetera,etc,loader,loading,progress,pending,throbber,menu,options,operator,code,spread,rest,more,further,extra,overflow,dots,…,...",
        contributors = "mittalyashu,ericfennis"
    ))]
    RectangleEllipsis,
    #[cfg(feature = "rectangle_goggles")]
    #[strum(props(
        svg = "<path d=\"M20 6a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-4a2 2 0 0 1-1.6-.8l-1.6-2.13a1 1 0 0 0-1.6 0L9.6 17.2A2 2 0 0 1 8 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z\"></path>",
        categories = "devices,gaming,multimedia,connectivity",
        tags = "vr,virtual,augmented,reality,headset,goggles",
        contributors = "EthanHazel,jguddas"
    ))]
    RectangleGoggles,
    #[cfg(feature = "rectangle_horizontal")]
    #[strum(props(
        svg = "<rect height=\"12\" y=\"6\" width=\"20\" x=\"2\" rx=\"2\"></rect>",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "<rect width=\"12\" rx=\"2\" height=\"20\" x=\"6\" y=\"2\"></rect>",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,9:16,vertical,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleVertical,
    #[cfg(feature = "recycle")]
    #[strum(props(
        svg = "<path d=\"M7 19H4.81a1.83 1.83 0 0 1-1.57-.881 1.78 1.78 0 0 1-.004-1.78L7.19 9.5\"></path><path d=\"M11 19h8.20a1.83 1.83 0 0 0 1.55-.89 1.78 1.78 0 0 0 0-1.77l-1.22-2.12\"></path><path d=\"m14 16-3 3 3 3\"></path><path d=\"M8.29 13.59 7.19 9.5 3.1 10.59\"></path><path d=\"m9.34 5.81 1.09-1.89A1.83 1.83 0 0 1 11.98 3a1.78 1.78 0 0 1 1.54.888l3.94 6.84\"></path><path d=\"m13.37 9.63 4.09 1.09 1.09-4.09\"></path>",
        categories = "sustainability",
        tags = "sustainability,salvage,arrows",
        contributors = "karsa-mistmere"
    ))]
    Recycle,
    #[cfg(feature = "redo_2")]
    #[strum(props(
        svg = "<path d=\"m15 14 5-5-5-5\"></path><path d=\"M20 9H9.5A5.5 5.5 0 0 0 4 14.5A5.5 5.5 0 0 0 9.5 20H13\"></path>",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Redo2,
    #[cfg(feature = "redo_dot")]
    #[strum(props(
        svg = "<circle r=\"1\" cx=\"12\" cy=\"17\"></circle><path d=\"M21 7v6h-6\"></path><path d=\"M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7\"></path>",
        categories = "text,arrows",
        tags = "redo,history,step,over,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RedoDot,
    #[cfg(feature = "redo")]
    #[strum(props(
        svg = "<path d=\"M21 7v6h-6\"></path><path d=\"M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7\"></path>",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Redo,
    #[cfg(feature = "refresh_ccw_dot")]
    #[strum(props(
        svg = "<path d=\"M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8\"></path><path d=\"M3 3v5h5\"></path><path d=\"M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16\"></path><path d=\"M16 16h5v5\"></path><circle r=\"1\" cx=\"12\" cy=\"12\"></circle>",
        categories = "arrows,development",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = "csandman,ericfennis,danielbayley,jamiemlaw"
    ))]
    RefreshCcwDot,
    #[cfg(feature = "refresh_ccw")]
    #[strum(props(
        svg = "<path d=\"M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8\"></path><path d=\"M3 3v5h5\"></path><path d=\"M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16\"></path><path d=\"M16 16h5v5\"></path>",
        categories = "arrows",
        tags = "arrows,rotate,reload,rerun,synchronise,synchronize,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCcw,
    #[cfg(feature = "refresh_cw_off")]
    #[strum(props(
        svg = "<path d=\"M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47\"></path><path d=\"M8 16H3v5\"></path><path d=\"M3 12C3 9.51 4 7.26 5.64 5.64\"></path><path d=\"m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64\"></path><path d=\"M21 12c0 1-.16 1.97-.47 2.87\"></path><path d=\"M21 3v5h-5\"></path><path d=\"M22 22 2 2\"></path>",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle,cancel,no,stop,error,disconnect,ignore",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCwOff,
    #[cfg(feature = "refresh_cw")]
    #[strum(props(
        svg = "<path d=\"M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8\"></path><path d=\"M21 3v5h-5\"></path><path d=\"M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16\"></path><path d=\"M8 16H3v5\"></path>",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCw,
    #[cfg(feature = "refrigerator")]
    #[strum(props(
        svg = "<path d=\"M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z\"></path><path d=\"M5 10h14\"></path><path d=\"M15 7v6\"></path>",
        categories = "food-beverage,home",
        tags = "frigerator,fridge,freezer,cooler,icebox,chiller,cold storage",
        contributors = "karsa-mistmere"
    ))]
    Refrigerator,
    #[cfg(feature = "regex")]
    #[strum(props(
        svg = "<path d=\"M17 3v10\"></path><path d=\"m12.67 5.5 8.66 5\"></path><path d=\"m12.67 10.5 8.66-5\"></path><path d=\"M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z\"></path>",
        categories = "text,development",
        tags = "search,text,code",
        contributors = "mittalyashu,ericfennis"
    ))]
    Regex,
    #[cfg(feature = "remove_formatting")]
    #[strum(props(
        svg = "<path d=\"M4 7V4h16v3\"></path><path d=\"M5 20h6\"></path><path d=\"M13 4 8 20\"></path><path d=\"m15 15 5 5\"></path><path d=\"m20 15-5 5\"></path>",
        categories = "text",
        tags = "text,font,typography,format,x,remove,delete,times,clear",
        contributors = "ericfennis"
    ))]
    RemoveFormatting,
    #[cfg(feature = "repeat_1")]
    #[strum(props(
        svg = "<path d=\"m17 2 4 4-4 4\"></path><path d=\"M3 11v-1a4 4 0 0 1 4-4h14\"></path><path d=\"m7 22-4-4 4-4\"></path><path d=\"M21 13v1a4 4 0 0 1-4 4H3\"></path><path d=\"M11 10h1v4\"></path>",
        categories = "multimedia",
        tags = "replay",
        contributors = "ericfennis"
    ))]
    Repeat1,
    #[cfg(feature = "repeat_2")]
    #[strum(props(
        svg = "<path d=\"m2 9 3-3 3 3\"></path><path d=\"M13 18H7a2 2 0 0 1-2-2V6\"></path><path d=\"m22 15-3 3-3-3\"></path><path d=\"M11 6h6a2 2 0 0 1 2 2v10\"></path>",
        categories = "arrows,social,multimedia",
        tags = "arrows,retweet,repost,share,repeat,loop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Repeat2,
    #[cfg(feature = "repeat")]
    #[strum(props(
        svg = "<path d=\"m17 2 4 4-4 4\"></path><path d=\"M3 11v-1a4 4 0 0 1 4-4h14\"></path><path d=\"m7 22-4-4 4-4\"></path><path d=\"M21 13v1a4 4 0 0 1-4 4H3\"></path>",
        categories = "arrows,multimedia",
        tags = "loop,arrows",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Repeat,
    #[cfg(feature = "replace_all")]
    #[strum(props(
        svg = "<path d=\"M14 14a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1\"></path><path d=\"M14 4a1 1 0 0 1 1-1\"></path><path d=\"M15 10a1 1 0 0 1-1-1\"></path><path d=\"M19 14a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1\"></path><path d=\"M21 4a1 1 0 0 0-1-1\"></path><path d=\"M21 9a1 1 0 0 1-1 1\"></path><path d=\"m3 7 3 3 3-3\"></path><path d=\"M6 10V5a2 2 0 0 1 2-2h2\"></path><rect width=\"7\" x=\"3\" rx=\"1\" y=\"14\" height=\"7\"></rect>",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "<path d=\"M14 4a1 1 0 0 1 1-1\"></path><path d=\"M15 10a1 1 0 0 1-1-1\"></path><path d=\"M21 4a1 1 0 0 0-1-1\"></path><path d=\"M21 9a1 1 0 0 1-1 1\"></path><path d=\"m3 7 3 3 3-3\"></path><path d=\"M6 10V5a2 2 0 0 1 2-2h2\"></path><rect height=\"7\" width=\"7\" x=\"3\" rx=\"1\" y=\"14\"></rect>",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Replace,
    #[cfg(feature = "reply_all")]
    #[strum(props(
        svg = "<path d=\"m12 17-5-5 5-5\"></path><path d=\"M22 18v-2a4 4 0 0 0-4-4H7\"></path><path d=\"m7 17-5-5 5-5\"></path>",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    ReplyAll,
    #[cfg(feature = "reply")]
    #[strum(props(
        svg = "<path d=\"M20 18v-2a4 4 0 0 0-4-4H4\"></path><path d=\"m9 17-5-5 5-5\"></path>",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Reply,
    #[cfg(feature = "rewind")]
    #[strum(props(
        svg = "<path d=\"M12 6a2 2 0 0 0-3.41-1.41l-6 6a2 2 0 0 0 0 2.82l6 6A2 2 0 0 0 12 18z\"></path><path d=\"M22 6a2 2 0 0 0-3.41-1.41l-6 6a2 2 0 0 0 0 2.82l6 6A2 2 0 0 0 22 18z\"></path>",
        categories = "arrows,multimedia",
        tags = "music",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Rewind,
    #[cfg(feature = "ribbon")]
    #[strum(props(
        svg = "<path d=\"M12 11.22C11 9.99 10 9 10 8a2 2 0 0 1 4 0c0 1-.998 2.00-2.01 3.22\"></path><path d=\"m12 18 2.57-3.5\"></path><path d=\"M6.24 9.01a7 7 0 0 1 11.50-.009\"></path><path d=\"M9.35 14.53 12 11.22\"></path><path d=\"M9.35 14.53C7.72 12.24 6 10.22 6 7a6 5 0 0 1 12 0c-.005 3.22-1.77 5.23-3.43 7.5l3.55 4.52a1 1 0 0 1-.203 1.43l-1.89 1.36a1 1 0 0 1-1.38-.215L12 18l-2.67 3.59a1 1 0 0 1-1.39.21l-1.86-1.35a1 1 0 0 1-.203-1.42z\"></path>",
        categories = "social,medical,emoji",
        tags = "awareness,strip,band,tape,strap,cordon",
        contributors = "karsa-mistmere"
    ))]
    Ribbon,
    #[cfg(feature = "road")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"M12 5V3\"></path><path d=\"M12 9v3\"></path><path d=\"M2.07 18.44A2 2 0 0 0 4 21h16a2 2 0 0 0 1.92-2.55l-4-14A2 2 0 0 0 16 3H8a2 2 0 0 0-1.92 1.45z\"></path>",
        categories = "transportation",
        tags = "road,street,highway,route,path,transport,traffic,drive,map",
        contributors = "uibalint,karsa-mistmere,jguddas"
    ))]
    Road,
    #[cfg(feature = "rocket")]
    #[strum(props(
        svg = "<path d=\"M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5\"></path><path d=\"M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09\"></path><path d=\"M9 12a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.4 22.4 0 0 1-4 2z\"></path><path d=\"M9 12H4s.55-3.03 2-4c1.62-1.08 5 .05 5 .05\"></path>",
        categories = "gaming,development",
        tags = "release,boost,launch,space,version",
        contributors = "ericfennis,jguddas"
    ))]
    Rocket,
    #[cfg(feature = "rocking_chair")]
    #[strum(props(
        svg = "<path d=\"m15 13 3.70 7.41\"></path><path d=\"M3 19a15 15 0 0 0 18 0\"></path><path d=\"m3 2 3.21 9.63A2 2 0 0 0 8.10 13H18\"></path><path d=\"m9 13-3.70 7.41\"></path>",
        categories = "home",
        tags = "chair,furniture,seat,comfort,relax",
        contributors = "connium,ericfennis,jamiemlaw"
    ))]
    RockingChair,
    #[cfg(feature = "roller_coaster")]
    #[strum(props(
        svg = "<path d=\"M6 19V5\"></path><path d=\"M10 19V6.8\"></path><path d=\"M14 19v-7.8\"></path><path d=\"M18 5v4\"></path><path d=\"M18 19v-6\"></path><path d=\"M22 19V9\"></path><path d=\"M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65\"></path>",
        categories = "navigation",
        tags = "attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    RollerCoaster,
    #[cfg(feature = "rose")]
    #[strum(props(
        svg = "<path d=\"M17 10h-1a4 4 0 1 1 4-4v.534\"></path><path d=\"M17 6h1a4 4 0 0 1 1.42 7.74l-2.29.87a6 6 0 0 1-5.33-10.68l2.06-1.31\"></path><path d=\"M4.5 17c2.8-.5 4.4 0 5.5.8s1.8 2.2 2.3 3.7c-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2\"></path><path d=\"M9.77 12C4 15 2 22 2 22\"></path><circle cx=\"17\" cy=\"8\" r=\"2\"></circle>",
        categories = "nature,seasons,sustainability,home,social",
        tags = "roses,thorns,petals,plant,stem,leaves,spring,bloom,blossom,gardening,botanical,flora,florist,bouquet,bunch,gift,date,romance,romantic,valentines day,special occasion",
        contributors = "danielbayley,jguddas,jamiemlaw,mittalyashu"
    ))]
    Rose,
    #[cfg(feature = "rotate_3_d")]
    #[strum(props(
        svg = "<path d=\"M16.46 7.5C15.64 4.23 13.95 2 12 2 9.23 2 7 6.47 7 12s2.23 10 5 10c.342 0 .677-.069 1-.2\"></path><path d=\"m15.19 13.70 3.81 1.86-1.86 3.81\"></path><path d=\"M19 15.57c-1.80.885-4.27 1.43-7 1.43-5.52 0-10-2.23-10-5s4.47-5 10-5c4.83 0 8.87 1.71 9.8 4\"></path>",
        categories = "design",
        tags = "gizmo,transform,orientation,orbit,axis",
        contributors = "lscheibel"
    ))]
    Rotate3D,
    #[cfg(feature = "rotate_ccw_key")]
    #[strum(props(
        svg = "<path d=\"M12 7v6\"></path><path d=\"M12 9h2\"></path><path d=\"M3 12a9 9 0 1 0 9-9 9.74 9.74 0 0 0-6.74 2.74L3 8\"></path><path d=\"M3 3v5h5\"></path><circle r=\"2\" cx=\"12\" cy=\"15\"></circle>",
        categories = "security,account",
        tags = "password,key,refresh,change",
        contributors = "karsa-mistmere,pgbradbury,jguddas"
    ))]
    RotateCcwKey,
    #[cfg(feature = "rotate_ccw_square")]
    #[strum(props(
        svg = "<path d=\"M20 9V7a2 2 0 0 0-2-2h-6\"></path><path d=\"m15 2-3 3 3 3\"></path><path d=\"M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2\"></path>",
        categories = "layout,design,photography,tools,arrows",
        tags = "left,counter-clockwise,rotate,image,90,45,degrees,°",
        contributors = "danielbayley"
    ))]
    RotateCcwSquare,
    #[cfg(feature = "rotate_ccw")]
    #[strum(props(
        svg = "<path d=\"M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8\"></path><path d=\"M3 3v5h5\"></path>",
        categories = "arrows,design,photography",
        tags = "arrow,left,counter-clockwise,restart,reload,rerun,refresh,backup,undo,replay,redo,retry,rewind,reverse",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCcw,
    #[cfg(feature = "rotate_cw_square")]
    #[strum(props(
        svg = "<path d=\"M12 5H6a2 2 0 0 0-2 2v3\"></path><path d=\"m9 8 3-3-3-3\"></path><path d=\"M4 14v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2\"></path>",
        categories = "layout,design,photography,tools,arrows",
        tags = "right,clockwise,rotate,image,90,45,degrees,°",
        contributors = "danielbayley"
    ))]
    RotateCwSquare,
    #[cfg(feature = "rotate_cw")]
    #[strum(props(
        svg = "<path d=\"M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8\"></path><path d=\"M21 3v5h-5\"></path>",
        categories = "arrows,design,photography",
        tags = "arrow,right,clockwise,refresh,reload,rerun,redo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCw,
    #[cfg(feature = "route_off")]
    #[strum(props(
        svg = "<circle r=\"3\" cy=\"19\" cx=\"6\"></circle><path d=\"M9 19h8.5c.4 0 .9-.1 1.3-.2\"></path><path d=\"M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12\"></path><path d=\"m2 2 20 20\"></path><path d=\"M21 15.3a3.5 3.5 0 0 0-3.3-3.3\"></path><path d=\"M15 5h-4.3\"></path><circle cx=\"18\" cy=\"5\" r=\"3\"></circle>",
        categories = "navigation",
        tags = "path,journey,planner,points,stops,stations,reset,clear,cancelled,closed,blocked",
        contributors = "danielbayley"
    ))]
    RouteOff,
    #[cfg(feature = "route")]
    #[strum(props(
        svg = "<circle cy=\"19\" r=\"3\" cx=\"6\"></circle><path d=\"M9 19h8.5a3.5 3.5 0 0 0 0-7h-11a3.5 3.5 0 0 1 0-7H15\"></path><circle cy=\"5\" cx=\"18\" r=\"3\"></circle>",
        categories = "navigation",
        tags = "path,journey,planner,points,stops,stations",
        contributors = "danielbayley"
    ))]
    Route,
    #[cfg(feature = "router")]
    #[strum(props(
        svg = "<rect height=\"8\" rx=\"2\" width=\"20\" x=\"2\" y=\"14\"></rect><path d=\"M6.01 18H6\"></path><path d=\"M10.01 18H10\"></path><path d=\"M15 10v4\"></path><path d=\"M17.84 7.17a4 4 0 0 0-5.66 0\"></path><path d=\"M20.66 4.34a8 8 0 0 0-11.31 0\"></path>",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows_2")]
    #[strum(props(
        svg = "<rect x=\"3\" rx=\"2\" width=\"18\" y=\"3\" height=\"18\"></rect><path d=\"M3 12h18\"></path>",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,panel,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawer",
        contributors = "danielbayley"
    ))]
    Rows2,
    #[cfg(feature = "rows_3")]
    #[strum(props(
        svg = "<rect width=\"18\" y=\"3\" rx=\"2\" x=\"3\" height=\"18\"></rect><path d=\"M21 9H3\"></path><path d=\"M21 15H3\"></path>",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers",
        contributors = "danielbayley"
    ))]
    Rows3,
    #[cfg(feature = "rows_4")]
    #[strum(props(
        svg = "<rect x=\"3\" rx=\"2\" width=\"18\" height=\"18\" y=\"3\"></rect><path d=\"M21 7.5H3\"></path><path d=\"M21 12H3\"></path><path d=\"M21 16.5H3\"></path>",
        categories = "layout,design,text",
        tags = "lines,list,queue,preview,paragraphs,parallel,series,split,vertical,horizontal,half,center,middle,even,drawers,grill",
        contributors = "danielbayley"
    ))]
    Rows4,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "<path d=\"M4 11a9 9 0 0 1 9 9\"></path><path d=\"M4 4a16 16 0 0 1 16 16\"></path><circle cx=\"5\" cy=\"19\" r=\"1\"></circle>",
        categories = "development,social",
        tags = "feed,subscribe,news,updates,notifications,content,blog,articles,broadcast,syndication,reader,channels,posts,publishing,digest,alert,following,inbox,newsletter,weblog,podcast",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Rss,
    #[cfg(feature = "ruler_dimension_line")]
    #[strum(props(
        svg = "<path d=\"M10 15v-3\"></path><path d=\"M14 15v-3\"></path><path d=\"M18 15v-3\"></path><path d=\"M2 8V4\"></path><path d=\"M22 6H2\"></path><path d=\"M22 8V4\"></path><path d=\"M6 15v-3\"></path><rect rx=\"2\" x=\"2\" y=\"12\" width=\"20\" height=\"8\"></rect>",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "jguddas,karsa-mistmere"
    ))]
    RulerDimensionLine,
    #[cfg(feature = "ruler")]
    #[strum(props(
        svg = "<path d=\"M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z\"></path><path d=\"m14.5 12.5 2-2\"></path><path d=\"m11.5 9.5 2-2\"></path><path d=\"m8.5 6.5 2-2\"></path><path d=\"m17.5 15.5 2-2\"></path>",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Ruler,
    #[cfg(feature = "russian_ruble")]
    #[strum(props(
        svg = "<path d=\"M6 11h8a4 4 0 0 0 0-8H9v18\"></path><path d=\"M6 15h8\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    RussianRuble,
    #[cfg(feature = "sailboat")]
    #[strum(props(
        svg = "<path d=\"M10 2v15\"></path><path d=\"M7 22a4 4 0 0 1-4-4 1 1 0 0 1 1-1h16a1 1 0 0 1 1 1 4 4 0 0 1-4 4z\"></path><path d=\"M9.15 2.46a1 1 0 0 1 1.52-.193l9.97 8.98A1 1 0 0 1 20 13H4a1 1 0 0 1-.824-1.56z\"></path>",
        categories = "transportation,travel",
        tags = "ship,boat,harbor,harbour,dock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Sailboat,
    #[cfg(feature = "salad")]
    #[strum(props(
        svg = "<path d=\"M7 21h10\"></path><path d=\"M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z\"></path><path d=\"M11.38 12a2.4 2.4 0 0 1-.4-4.77 2.4 2.4 0 0 1 3.2-2.77 2.4 2.4 0 0 1 3.47-.63 2.4 2.4 0 0 1 3.37 3.37 2.4 2.4 0 0 1-1.1 3.7 2.51 2.51 0 0 1 .03 1.1\"></path><path d=\"m13 12 4-4\"></path><path d=\"M10.9 7.25A3.99 3.99 0 0 0 4 10c0 .73.2 1.41.54 2\"></path>",
        categories = "food-beverage,emoji",
        tags = "food,vegetarian,dish,restaurant,course,meal,side,vegetables,health",
        contributors = "kemie"
    ))]
    Salad,
    #[cfg(feature = "sandwich")]
    #[strum(props(
        svg = "<path d=\"m2.37 11.22 8.37-6.77a2 2 0 0 1 2.51 0l8.37 6.77\"></path><path d=\"M21 15a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-5.25\"></path><path d=\"M3 15a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h9\"></path><path d=\"m6.67 15 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2\"></path><rect width=\"20\" x=\"2\" y=\"11\" rx=\"1\" height=\"4\"></rect>",
        categories = "food-beverage",
        tags = "food,snack,dish,restaurant,lunch,meal",
        contributors = "kemie,jamiemlaw"
    ))]
    Sandwich,
    #[cfg(feature = "satellite_dish")]
    #[strum(props(
        svg = "<path d=\"M4 10a7.31 7.31 0 0 0 10 10Z\"></path><path d=\"m9 15 3-3\"></path><path d=\"M17 13a6 6 0 0 0-6-6\"></path><path d=\"M21 13A10 10 0 0 0 11 3\"></path>",
        categories = "connectivity,devices,multimedia",
        tags = "antenna,receiver,dish aerial,saucer",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SatelliteDish,
    #[cfg(feature = "satellite")]
    #[strum(props(
        svg = "<path d=\"m13.5 6.5-3.14-3.14a1.20 1.20 0 0 0-1.70 0L6.35 5.64a1.20 1.20 0 0 0 0 1.70L9.5 10.5\"></path><path d=\"M16.5 7.5 19 5\"></path><path d=\"m17.5 10.5 3.14 3.14a1.20 1.20 0 0 1 0 1.70l-2.29 2.29a1.20 1.20 0 0 1-1.70 0L13.5 14.5\"></path><path d=\"M9 21a6 6 0 0 0-6-6\"></path><path d=\"M9.35 10.64a1.20 1.20 0 0 0 0 1.70l2.29 2.29a1.20 1.20 0 0 0 1.70 0l4.29-4.29a1.20 1.20 0 0 0 0-1.70l-2.29-2.29a1.20 1.20 0 0 0-1.70 0z\"></path>",
        categories = "connectivity,science",
        tags = "space station,orbit,transmitter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Satellite,
    #[cfg(feature = "saudi_riyal")]
    #[strum(props(
        svg = "<path d=\"m20 19.5-5.5 1.2\"></path><path d=\"M14.5 4v11.22a1 1 0 0 0 1.24.97L20 15.2\"></path><path d=\"m2.97 19.35 5.54-1.36A2 2 0 0 0 10 16V2\"></path><path d=\"M20 10 4 13.5\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "null78,jguddas"
    ))]
    SaudiRiyal,
    #[cfg(feature = "save_all")]
    #[strum(props(
        svg = "<path d=\"M10 2v3a1 1 0 0 0 1 1h5\"></path><path d=\"M18 18v-6a1 1 0 0 0-1-1h-6a1 1 0 0 0-1 1v6\"></path><path d=\"M18 22H4a2 2 0 0 1-2-2V6\"></path><path d=\"M8 18a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9.17a2 2 0 0 1 1.41.586l2.82 2.82A2 2 0 0 1 22 6.82V16a2 2 0 0 1-2.01 2z\"></path>",
        categories = "text,files",
        tags = "floppy disks,copy",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    SaveAll,
    #[cfg(feature = "save_off")]
    #[strum(props(
        svg = "<path d=\"M13 13H8a1 1 0 0 0-1 1v7\"></path><path d=\"M14 8h1\"></path><path d=\"M17 21v-4\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20.41 20.41A2 2 0 0 1 19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 .59-1.41\"></path><path d=\"M29.5 11.5s5 5 4 5\"></path><path d=\"M9 3h6.2a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V15\"></path>",
        categories = "text,files",
        tags = "floppy disk,unsalvageable",
        contributors = "AnnaSasDev"
    ))]
    SaveOff,
    #[cfg(feature = "save")]
    #[strum(props(
        svg = "<path d=\"M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z\"></path><path d=\"M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7\"></path><path d=\"M7 3v4a1 1 0 0 0 1 1h7\"></path>",
        categories = "text,files",
        tags = "floppy disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Save,
    #[cfg(feature = "scale_3_d")]
    #[strum(props(
        svg = "<path d=\"M5 7v11a1 1 0 0 0 1 1h11\"></path><path d=\"M5.29 18.70 11 13\"></path><circle cy=\"19\" r=\"2\" cx=\"19\"></circle><circle r=\"2\" cx=\"5\" cy=\"5\"></circle>",
        categories = "design",
        tags = "gizmo,transform,size,axis",
        contributors = "lscheibel,ericfennis,jguddas"
    ))]
    Scale3D,
    #[cfg(feature = "scale")]
    #[strum(props(
        svg = "<path d=\"M12 3v18\"></path><path d=\"m19 8 3 8a5 5 0 0 1-6 0zV7\"></path><path d=\"M3 7h1a17 17 0 0 0 8-2 17 17 0 0 0 8 2h1\"></path><path d=\"m5 8 3 8a5 5 0 0 1-6 0zV7\"></path><path d=\"M7 21h10\"></path>",
        categories = "navigation,science,finance",
        tags = "balance,legal,license,right,rule,law,justice,weight,measure,compare,judge,fair,ethics,decision",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Scale,
    #[cfg(feature = "scaling")]
    #[strum(props(
        svg = "<path d=\"M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7\"></path><path d=\"M14 15H9v-5\"></path><path d=\"M16 3h5v5\"></path><path d=\"M21 3 9 15\"></path>",
        categories = "design",
        tags = "scale,resize,design",
        contributors = "karsa-mistmere"
    ))]
    Scaling,
    #[cfg(feature = "scan_barcode")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M8 7v10\"></path><path d=\"M12 7v10\"></path><path d=\"M17 7v10\"></path>",
        categories = "shopping,devices",
        tags = "checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer",
        contributors = "danielbayley"
    ))]
    ScanBarcode,
    #[cfg(feature = "scan_eye")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><circle cx=\"12\" cy=\"12\" r=\"1\"></circle><path d=\"M18.94 12.33a1 1 0 0 0 0-.66 7.5 7.5 0 0 0-13.88 0 1 1 0 0 0 0 .66 7.5 7.5 0 0 0 13.88 0\"></path>",
        categories = "photography,multimedia,accessibility,security,devices,account",
        tags = "preview,zoom,expand,fullscreen,gallery,image,camera,watch,surveillance,retina,focus,lens,biometric,identification,authentication,access,login",
        contributors = "danielbayley"
    ))]
    ScanEye,
    #[cfg(feature = "scan_face")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M8 14s1.5 2 4 2 4-2 4-2\"></path><path d=\"M9 9h.01\"></path><path d=\"M15 9h.01\"></path>",
        categories = "account,security,devices,social",
        tags = "face,biometric,identification,authentication,2fa,access,login,dashed",
        contributors = "karsa-mistmere"
    ))]
    ScanFace,
    #[cfg(feature = "scan_heart")]
    #[strum(props(
        svg = "<path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M7.82 13.07A3 3 0 0 1 12 8.76a3 3 0 0 1 4.17 4.30l-3.44 3.62a1 1 0 0 1-1.44 0z\"></path>",
        categories = "medical",
        tags = "health,heart rate,pulse,monitoring,healthiness,screening,dashed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ScanHeart,
    #[cfg(feature = "scan_line")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M7 12h10\"></path>",
        categories = "devices,shopping",
        tags = "checkout,till,cart,transaction,purchase,buy,product,packaging,retail,consumer,qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ScanLine,
    #[cfg(feature = "scan_qr_code")]
    #[strum(props(
        svg = "<path d=\"M17 12v4a1 1 0 0 1-1 1h-4\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M17 8V7\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M7 17h.01\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><rect x=\"7\" y=\"7\" height=\"5\" width=\"5\" rx=\"1\"></rect>",
        categories = "account,shopping,devices,security",
        tags = "barcode,scan,qrcode,url,information,digital,scanner",
        contributors = "jguddas,vexkiddy"
    ))]
    ScanQrCode,
    #[cfg(feature = "scan_search")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><circle cx=\"12\" cy=\"12\" r=\"3\"></circle><path d=\"m16 16-1.9-1.9\"></path>",
        categories = "photography,multimedia,accessibility",
        tags = "preview,zoom,expand,fullscreen,gallery,image,focus,lens",
        contributors = "danielbayley"
    ))]
    ScanSearch,
    #[cfg(feature = "scan_text")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M7 8h8\"></path><path d=\"M7 12h10\"></path><path d=\"M7 16h6\"></path>",
        categories = "text,devices",
        tags = "recognition,read,translate,copy,lines",
        contributors = "danielbayley"
    ))]
    ScanText,
    #[cfg(feature = "scan")]
    #[strum(props(
        svg = "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"></path><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"></path><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"></path><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"></path>",
        categories = "devices,shopping,security,social,gaming",
        tags = "qr-code,barcode,checkout,augmented reality,ar,target,surveillance,camera,lens,focus,frame,select,box,boundary,bounds,area,square,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    Scan,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "<path d=\"M14 21v-3a2 2 0 0 0-4 0v3\"></path><path d=\"M18 4.93V21\"></path><path d=\"m4 6 7.10-3.79a2 2 0 0 1 1.78 0L20 6\"></path><path d=\"m6 11-3.52 2.14a1 1 0 0 0-.48.85V19a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a1 1 0 0 0-.48-.853L18 11\"></path><path d=\"M6 4.93V21\"></path><circle r=\"2\" cx=\"12\" cy=\"9\"></circle>",
        categories = "buildings,navigation",
        tags = "building,education,childhood,university,learning,campus,scholar,student,lecture,degree,course,academia,study,knowledge,classroom,research,diploma,graduation,professor,tutorial,homework,assignment,exam",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "<path d=\"M5.42 9.42 8 12\"></path><circle r=\"2\" cx=\"4\" cy=\"8\"></circle><path d=\"m14 6-8.58 8.58\"></path><circle cy=\"16\" r=\"2\" cx=\"4\"></circle><path d=\"M10.8 14.8 14 18\"></path><path d=\"M16 12h-2\"></path><path d=\"M22 12h-2\"></path>",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"6\" cy=\"6\"></circle><path d=\"M8.12 8.12 12 12\"></path><path d=\"M20 4 8.12 15.88\"></path><circle cx=\"6\" cy=\"18\" r=\"3\"></circle><path d=\"M14.8 14.8 20 20\"></path>",
        categories = "text,design,tools",
        tags = "cut,snip,chop,stationery,crafts",
        contributors = "colebemis,ericfennis"
    ))]
    Scissors,
    #[cfg(feature = "scooter")]
    #[strum(props(
        svg = "<path d=\"M21 4h-3.5l2 11.05\"></path><path d=\"M6.95 17h5.14c.523 0 .95-.406 1.06-.916a6.5 6.5 0 0 1 5.34-5.00\"></path><circle cx=\"19.5\" cy=\"17.5\" r=\"2.5\"></circle><circle cx=\"4.5\" cy=\"17.5\" r=\"2.5\"></circle>",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey,transport,electric,ride,urban,commute,speed",
        contributors = "Ahmed-Dghaies,karsa-mistmere"
    ))]
    Scooter,
    #[cfg(feature = "screen_share_off")]
    #[strum(props(
        svg = "<path d=\"M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3\"></path><path d=\"M8 21h8\"></path><path d=\"M12 17v4\"></path><path d=\"m22 3-5 5\"></path><path d=\"m17 3 5 5\"></path>",
        categories = "connectivity,devices,communication",
        tags = "desktop,disconnect,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShareOff,
    #[cfg(feature = "screen_share")]
    #[strum(props(
        svg = "<path d=\"M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3\"></path><path d=\"M8 21h8\"></path><path d=\"M12 17v4\"></path><path d=\"m17 8 5-5\"></path><path d=\"M17 3h5v5\"></path>",
        categories = "connectivity,devices,communication",
        tags = "host,desktop,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShare,
    #[cfg(feature = "scroll_text")]
    #[strum(props(
        svg = "<path d=\"M15 12h-5\"></path><path d=\"M15 8h-5\"></path><path d=\"M19 17V5a2 2 0 0 0-2-2H4\"></path><path d=\"M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3\"></path>",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ScrollText,
    #[cfg(feature = "scroll")]
    #[strum(props(
        svg = "<path d=\"M19 17V5a2 2 0 0 0-2-2H4\"></path><path d=\"M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3\"></path>",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Scroll,
    #[cfg(feature = "search_alert")]
    #[strum(props(
        svg = "<circle cx=\"11\" cy=\"11\" r=\"8\"></circle><path d=\"m21 21-4.3-4.3\"></path><path d=\"M11 7v4\"></path><path d=\"M11 15h.01\"></path>",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,warning,alert,error,anomaly,lens",
        contributors = "colebemis,ericfennis,jguddas,Veatec22"
    ))]
    SearchAlert,
    #[cfg(feature = "search_check")]
    #[strum(props(
        svg = "<path d=\"m8 11 2 2 4-4\"></path><circle r=\"8\" cy=\"11\" cx=\"11\"></circle><path d=\"m21 21-4.3-4.3\"></path>",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick,lens",
        contributors = "danielbayley"
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "<path d=\"m13 13.5 2-2.5-2-2.5\"></path><path d=\"m21 21-4.3-4.3\"></path><path d=\"M9 8.5 7 11l2 2.5\"></path><circle cx=\"11\" r=\"8\" cy=\"11\"></circle>",
        categories = "text,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>,lens",
        contributors = "danielbayley,jguddas"
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "<path d=\"m13.5 8.5-5 5\"></path><circle cx=\"11\" cy=\"11\" r=\"8\"></circle><path d=\"m21 21-4.3-4.3\"></path>",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/,lens",
        contributors = "danielbayley"
    ))]
    SearchSlash,
    #[cfg(feature = "search_x")]
    #[strum(props(
        svg = "<path d=\"m13.5 8.5-5 5\"></path><path d=\"m8.5 8.5 5 5\"></path><circle cx=\"11\" r=\"8\" cy=\"11\"></circle><path d=\"m21 21-4.3-4.3\"></path>",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,lens",
        contributors = "danielbayley"
    ))]
    SearchX,
    #[cfg(feature = "search")]
    #[strum(props(
        svg = "<path d=\"m21 21-4.34-4.34\"></path><circle cy=\"11\" r=\"8\" cx=\"11\"></circle>",
        categories = "text,social",
        tags = "find,scan,magnifier,magnifying glass,lens",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    Search,
    #[cfg(feature = "section")]
    #[strum(props(
        svg = "<path d=\"M16 5a4 3 0 0 0-8 0c0 4 8 3 8 7a4 3 0 0 1-8 0\"></path><path d=\"M8 19a4 3 0 0 0 8 0c0-4-8-3-8-7a4 3 0 0 1 8 0\"></path>",
        categories = "text",
        tags = "mark,typography,punctuation,legal,type,text,prose,symbol",
        contributors = "gurtt,karsa-mistmere"
    ))]
    Section,
    #[cfg(feature = "send_horizontal")]
    #[strum(props(
        svg = "<path d=\"M3.71 3.04a.498.49 0 0 0-.683.62l2.84 7.62a2 2 0 0 1 0 1.39l-2.84 7.62a.498.49 0 0 0 .682.62l18-8.5a.5.5 0 0 0 0-.904z\"></path><path d=\"M6 12h16\"></path>",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SendHorizontal,
    #[cfg(feature = "send_to_back")]
    #[strum(props(
        svg = "<rect height=\"8\" x=\"14\" rx=\"2\" width=\"8\" y=\"14\"></rect><rect height=\"8\" width=\"8\" y=\"2\" rx=\"2\" x=\"2\"></rect><path d=\"M7 14v1a2 2 0 0 0 2 2h1\"></path><path d=\"M14 7h1a2 2 0 0 1 2 2v1\"></path>",
        categories = "design,layout",
        tags = "bring,send,move,under,back,backwards,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    SendToBack,
    #[cfg(feature = "send")]
    #[strum(props(
        svg = "<path d=\"M14.53 21.68a.5.5 0 0 0 .937-.024l6.5-19a.496.49 0 0 0-.635-.635l-19 6.5a.5.5 0 0 0-.024.93l7.93 3.18a2 2 0 0 1 1.11 1.11z\"></path><path d=\"m21.85 2.14-10.94 10.93\"></path>",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Send,
    #[cfg(feature = "separator_horizontal")]
    #[strum(props(
        svg = "<path d=\"m16 16-4 4-4-4\"></path><path d=\"M3 12h18\"></path><path d=\"m8 8 4-4 4 4\"></path>",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 3v18\"></path><path d=\"m16 16 4-4-4-4\"></path><path d=\"m8 8-4 4 4 4\"></path>",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[cfg(feature = "server_cog")]
    #[strum(props(
        svg = "<path d=\"m10.85 14.77-.383.92\"></path><path d=\"M13.14 14.77a3 3 0 1 0-2.29-5.54l-.383-.923\"></path><path d=\"m13.14 9.22.383-.923\"></path><path d=\"m13.53 15.69-.382-.924a3 3 0 1 1-2.29-5.54\"></path><path d=\"m14.77 10.85.923-.383\"></path><path d=\"m14.77 13.14.923.38\"></path><path d=\"M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5\"></path><path d=\"M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5\"></path><path d=\"M6 18h.01\"></path><path d=\"M6 6h.01\"></path><path d=\"m9.22 10.85-.923-.383\"></path><path d=\"m9.22 13.14-.923.38\"></path>",
        categories = "development,devices",
        tags = "cloud,storage,computing,cog,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    ServerCog,
    #[cfg(feature = "server_crash")]
    #[strum(props(
        svg = "<path d=\"M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2\"></path><path d=\"M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2\"></path><path d=\"M6 6h.01\"></path><path d=\"M6 18h.01\"></path><path d=\"m13 6-4 6h6l-4 6\"></path>",
        categories = "development,devices",
        tags = "cloud,storage,problem,error",
        contributors = "mittalyashu,ericfennis"
    ))]
    ServerCrash,
    #[cfg(feature = "server_off")]
    #[strum(props(
        svg = "<path d=\"M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5\"></path><path d=\"M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z\"></path><path d=\"M22 17v-1a2 2 0 0 0-2-2h-1\"></path><path d=\"M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z\"></path><path d=\"M6 18h.01\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    ServerOff,
    #[cfg(feature = "server")]
    #[strum(props(
        svg = "<rect ry=\"2\" width=\"20\" height=\"8\" rx=\"2\" x=\"2\" y=\"2\"></rect><rect x=\"2\" rx=\"2\" height=\"8\" width=\"20\" ry=\"2\" y=\"14\"></rect><line x2=\"6.01\" y1=\"6\" x1=\"6\" y2=\"6\"></line><line x1=\"6\" x2=\"6.01\" y1=\"18\" y2=\"18\"></line>",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "<path d=\"M14 17H5\"></path><path d=\"M19 7h-9\"></path><circle r=\"3\" cx=\"17\" cy=\"17\"></circle><circle cx=\"7\" cy=\"7\" r=\"3\"></circle>",
        categories = "account",
        tags = "cog,edit,gear,preferences,slider",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "<path d=\"M9.67 4.13a2.34 2.34 0 0 1 4.65 0 2.34 2.34 0 0 0 3.31 1.91 2.34 2.34 0 0 1 2.33 4.03 2.34 2.34 0 0 0 0 3.83 2.34 2.34 0 0 1-2.33 4.03 2.34 2.34 0 0 0-3.31 1.91 2.34 2.34 0 0 1-4.65 0 2.34 2.34 0 0 0-3.32-1.91 2.34 2.34 0 0 1-2.33-4.03 2.34 2.34 0 0 0 0-3.83A2.34 2.34 0 0 1 6.35 6.05a2.34 2.34 0 0 0 3.31-1.91\"></path><circle cy=\"12\" r=\"3\" cx=\"12\"></circle>",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "<path d=\"M8.3 10a.7.7 0 0 1-.626-1.07L11.4 3a.7.7 0 0 1 1.19-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z\"></path><rect x=\"3\" width=\"7\" height=\"7\" y=\"14\" rx=\"1\"></rect><circle cx=\"17.5\" r=\"3.5\" cy=\"17.5\"></circle>",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = "danielbayley"
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "<circle cy=\"5\" r=\"3\" cx=\"18\"></circle><circle cx=\"6\" cy=\"12\" r=\"3\"></circle><circle cy=\"19\" r=\"3\" cx=\"18\"></circle><line y1=\"13.51\" x2=\"15.42\" x1=\"8.59\" y2=\"17.49\"></line><line x1=\"15.41\" x2=\"8.59\" y2=\"10.49\" y1=\"6.51\"></line>",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "<path d=\"M12 2v13\"></path><path d=\"m16 6-4-4-4 4\"></path><path d=\"M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8\"></path>",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" ry=\"2\" y=\"3\" width=\"18\" x=\"3\"></rect><line y2=\"9\" x1=\"3\" x2=\"21\" y1=\"9\"></line><line x1=\"3\" y2=\"15\" y1=\"15\" x2=\"21\"></line><line x2=\"9\" y2=\"21\" x1=\"9\" y1=\"9\"></line><line x2=\"15\" y2=\"21\" y1=\"9\" x1=\"15\"></line>",
        categories = "text,files",
        tags = "spreadsheets,table,excel",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Sheet,
    #[cfg(feature = "shell")]
    #[strum(props(
        svg = "<path d=\"M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44\"></path>",
        categories = "animals,development,nature,science,travel,food-beverage,home",
        tags = "beach,sand,holiday,sealife,fossil,ammonite,biology,ocean,terminal,command line,session,bash,zsh,roll,wrap,chewing gum,bubble gum,sweet,sugar,hosepipe,carpet,string,spiral,spinner,hypnotise,hypnosis",
        contributors = "danielbayley"
    ))]
    Shell,
    #[cfg(feature = "shelving_unit")]
    #[strum(props(
        svg = "<path d=\"M12 12V9a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3\"></path><path d=\"M16 20v-3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3\"></path><path d=\"M20 22V2\"></path><path d=\"M4 12h16\"></path><path d=\"M4 20h16\"></path><path d=\"M4 2v20\"></path><path d=\"M4 4h16\"></path>",
        categories = "home",
        tags = "ledge,rack,storage,inventory,furniture,sill,shelves,shelf,organize,display,store,arrange,unit,cabinet,fixture,retail,warehouse",
        contributors = "karsa-mistmere"
    ))]
    ShelvingUnit,
    #[cfg(feature = "shield_alert")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M12 8v4\"></path><path d=\"M12 16h.01\"></path>",
        categories = "account,security,development,notifications,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,warning,emergency,attention,urgent,alarm,crest,bravery,strength,tough,attacked,damaged,injured,hit,expired,disabled,inactive,error,exclamation mark,!",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldAlert,
    #[cfg(feature = "shield_ban")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"m4.24 5.21 14.39 12.47\"></path>",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,cancel,error,crest,bravery,attacked,damaged,injured,hit,expired,eliminated,disabled,inactive,/",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldBan,
    #[cfg(feature = "shield_check")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secured,safety,protection,protected,guardian,guarded,armored,armoured,defense,defence,defended,blocked,threat,prevention,prevented,antivirus,vigilance,vigilant,active,activated,enabled,detection,scanned,found,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audited,admin,verification,verified,certification,certified,tested,passed,qualified,cleared,cleaned,disinfected,uninfected,task,completed,todo,done,ticked,checked,crest,bravery",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldCheck,
    #[cfg(feature = "shield_cog_corner")]
    #[strum(props(
        svg = "<path d=\"M11 22c-3.80-1.45-7-3.96-7-9V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v4\"></path><path d=\"M14.92 16.54 14 16.16\"></path><path d=\"m14.92 18.84-.923.38\"></path><path d=\"M16.54 14.92 16.16 14\"></path><path d=\"m16.54 20.46-.383.92\"></path><path d=\"m18.84 14.92.383-.923\"></path><path d=\"m19.22 21.39-.382-.924\"></path><path d=\"m20.46 16.54.923-.383\"></path><path d=\"m20.46 18.84.923.38\"></path><circle cx=\"17.69\" r=\"3\" cy=\"17.69\"></circle>",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,shieldcog,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere,RajnishKMehta"
    ))]
    ShieldCogCorner,
    #[cfg(feature = "shield_cog")]
    #[strum(props(
        svg = "<path d=\"m10.92 14.46-.383.92\"></path><path d=\"M10.92 8.92 10.54 8\"></path><path d=\"M13.22 8.92 13.60 8\"></path><path d=\"m13.60 15.39-.382-.924\"></path><path d=\"m14.84 10.54.923-.383\"></path><path d=\"m14.84 12.84.923.38\"></path><path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"m9.30 10.54-.923-.383\"></path><path d=\"m9.30 12.84-.923.38\"></path><circle cy=\"11.69\" cx=\"12.07\" r=\"3\"></circle>",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere,RajnishKMehta"
    ))]
    ShieldCog,
    #[cfg(feature = "shield_ellipsis")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M8 12h.01\"></path><path d=\"M12 12h.01\"></path><path d=\"M16 12h.01\"></path>",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,securing,protecting,guarding,armoring,armouring,defending,blocking,preventing,antivirus,detecting,scanning,finding,auditing,admin,verifying,crest,upgrading,loader,loading,throbber,progress,dots,more,etc,...,…",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldEllipsis,
    #[cfg(feature = "shield_half")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M12 22V2\"></path>",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,logo,sigil,flag,team,faction,fraternity,university,college,academy,school,education,uniform,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,ranking,army,cadet,scout",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldHalf,
    #[cfg(feature = "shield_minus")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M9 12h6\"></path>",
        categories = "account,security,development,gaming",
        tags = "unshield,cybersecurity,unsecure,unguard,unblock,antivirus,clean,clear,disinfect,patch,fix,stop,cancel,remove,relax,admin,crest,bravery,weakened,damaged,hit,unarm,disable,deactivate,decommission,downgraded,minimum,-",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldMinus,
    #[cfg(feature = "shield_off")]
    #[strum(props(
        svg = "<path d=\"m2 2 20 20\"></path><path d=\"M5 5a1 1 0 0 0-1 1v7c0 5 3.5 7.5 7.67 8.94a1 1 0 0 0 .67.01c2.35-.82 4.48-1.97 5.9-3.71\"></path><path d=\"M9.30 3.65A12.25 12.25 0 0 0 11.24 2.28a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v7a9.78 9.78 0 0 1-.08 1.26\"></path>",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,interception,threat,prevention,unprevented,antivirus,detection,undetected,exploit,vulnerability,vulnerable,weakness,infected,infection,comprimised,data leak,unaudited,admin,verification,unverified,inactive,cancelled,error,crest,bravery,damaged,injured,hit,expired,eliminated",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShieldOff,
    #[cfg(feature = "shield_plus")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M9 12h6\"></path><path d=\"M12 9v6\"></path>",
        categories = "account,security,development,gaming,medical",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,extra,added,professional,enterprise,full,maximum,upgraded,ultra,activate,enable,audit,admin,verification,crest,medic,+",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ShieldPlus,
    #[cfg(feature = "shield_question_mark")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3\"></path><path d=\"M12 17h.01\"></path>",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,undetected,scan,find,exploit,vulnerability,vulnerable,weakness,infection,comprimised,data leak,audit,admin,verification,unverified,uncertified,uncertain,unknown,inactive,crest,question mark,?",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ShieldQuestionMark,
    #[cfg(feature = "shield_user")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"M6.37 18.91a6 6 0 0 1 11.24.003\"></path><circle cx=\"12\" cy=\"11\" r=\"4\"></circle>",
        categories = "account,security,development",
        tags = "shield,user,admin,protection,protected,safety,guard",
        contributors = "sebinemeth,ksk3110,karsa-mistmere,colebemis"
    ))]
    ShieldUser,
    #[cfg(feature = "shield_x")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path><path d=\"m14.5 9.5-5 5\"></path><path d=\"m9.5 9.5 5 5\"></path>",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,prevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,inactive,cancel,error,wrong,false,crest,bravery,attacked,damaged,injured,hit,dead,deceased,expired,eliminated,exterminated",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ShieldX,
    #[cfg(feature = "shield")]
    #[strum(props(
        svg = "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"></path>",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Shield,
    #[cfg(feature = "ship_wheel")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"8\"></circle><path d=\"M12 2v7.5\"></path><path d=\"m19 5-5.23 5.23\"></path><path d=\"M22 12h-7.5\"></path><path d=\"m19 19-5.23-5.23\"></path><path d=\"M12 14.5V22\"></path><path d=\"M10.23 13.77 5 19\"></path><path d=\"M9.5 12H2\"></path><path d=\"M10.23 10.23 5 5\"></path><circle cy=\"12\" cx=\"12\" r=\"2.5\"></circle>",
        categories = "transportation,navigation,travel",
        tags = "steering,rudder,boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "danielbayley"
    ))]
    ShipWheel,
    #[cfg(feature = "ship")]
    #[strum(props(
        svg = "<path d=\"M12 10.18V14\"></path><path d=\"M12 2v3\"></path><path d=\"M19 13V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v6\"></path><path d=\"M19.38 20A11.6 11.6 0 0 0 21 14l-8.18-3.63a2 2 0 0 0-1.62 0L3 14a11.6 11.6 0 0 0 2.81 7.76\"></path><path d=\"M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1s1.2 1 2.5 1c2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path>",
        categories = "transportation,navigation,travel",
        tags = "boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip,releases",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Ship,
    #[cfg(feature = "shirt")]
    #[strum(props(
        svg = "<path d=\"M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z\"></path>",
        categories = "shopping",
        tags = "t-shirt,shopping,store,clothing,clothes",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Shirt,
    #[cfg(feature = "shopping_bag")]
    #[strum(props(
        svg = "<path d=\"M16 10a4 4 0 0 1-8 0\"></path><path d=\"M3.10 6.03h17.79\"></path><path d=\"M3.4 5.46a2 2 0 0 0-.4 1.2V20a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6.66a2 2 0 0 0-.4-1.2l-2-2.66A2 2 0 0 0 17 2H7a2 2 0 0 0-1.6.8z\"></path>",
        categories = "shopping",
        tags = "ecommerce,cart,purchase,store",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ShoppingBag,
    #[cfg(feature = "shopping_basket")]
    #[strum(props(
        svg = "<path d=\"m15 11-1 9\"></path><path d=\"m19 11-4-7\"></path><path d=\"M2 11h20\"></path><path d=\"m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8a2 2 0 0 0 2-1.6l1.7-7.4\"></path><path d=\"M4.5 15.5h15\"></path><path d=\"m5 11 4-7\"></path><path d=\"m9 11 1 9\"></path>",
        categories = "shopping",
        tags = "cart,e-commerce,store,purchase,products,items,ingredients",
        contributors = "danielbayley"
    ))]
    ShoppingBasket,
    #[cfg(feature = "shopping_cart")]
    #[strum(props(
        svg = "<circle cx=\"8\" cy=\"21\" r=\"1\"></circle><circle cx=\"19\" r=\"1\" cy=\"21\"></circle><path d=\"M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12\"></path>",
        categories = "shopping",
        tags = "trolley,cart,basket,e-commerce,store,purchase,products,items,ingredients",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShoppingCart,
    #[cfg(feature = "shovel")]
    #[strum(props(
        svg = "<path d=\"M21.56 4.56a1.5 1.5 0 0 1 0 2.12l-.47.47a3 3 0 0 1-4.21-.03 3 3 0 0 1 0-4.24l.44-.44a1.5 1.5 0 0 1 2.12 0z\"></path><path d=\"M3 22a1 1 0 0 1-1-1v-3.58a1 1 0 0 1 .293-.707l3.35-3.35a1.20 1.20 0 0 1 1.70 0l3.29 3.29a1.20 1.20 0 0 1 0 1.70l-3.35 3.35a1 1 0 0 1-.707.29z\"></path><path d=\"m9 15 7.87-7.87\"></path>",
        categories = "nature,tools,gaming",
        tags = "dig,spade,treasure",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Shovel,
    #[cfg(feature = "shower_head")]
    #[strum(props(
        svg = "<path d=\"m4 4 2.5 2.5\"></path><path d=\"M13.5 6.5a4.95 4.95 0 0 0-7 7\"></path><path d=\"M15 5 5 15\"></path><path d=\"M14 17v.01\"></path><path d=\"M10 16v.01\"></path><path d=\"M13 13v.01\"></path><path d=\"M16 10v.01\"></path><path d=\"M11 20v.01\"></path><path d=\"M17 14v.01\"></path><path d=\"M20 11v.01\"></path>",
        categories = "home,travel",
        tags = "shower,bath,bathroom,amenities,services",
        contributors = "karsa-mistmere"
    ))]
    ShowerHead,
    #[cfg(feature = "shredder")]
    #[strum(props(
        svg = "<path d=\"M4 13V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.70.706l3.58 3.58A2.4 2.4 0 0 1 20 8v5\"></path><path d=\"M14 2v5a1 1 0 0 0 1 1h5\"></path><path d=\"M10 22v-5\"></path><path d=\"M14 19v-2\"></path><path d=\"M18 20v-3\"></path><path d=\"M2 13h20\"></path><path d=\"M6 20v-3\"></path>",
        categories = "mail,files",
        tags = "file,paper,tear,cut,delete,destroy,remove,erase,document,destruction,secure,security,confidential,data,trash,dispose,disposal,information,waste,permanent",
        contributors = "Alirashidy,colebemis,danielbayley,ericfennis,jguddas,karsa-mistmere"
    ))]
    Shredder,
    #[cfg(feature = "shrimp")]
    #[strum(props(
        svg = "<path d=\"M11 12h.01\"></path><path d=\"M13 22c.5-.5 1.12-1 2.5-1-1.38 0-2-.5-2.5-1\"></path><path d=\"M14 2a3.28 3.28 0 0 1-3.22 1.79l-6.17-.561A2.38 2.38 0 1 0 4.38 8H15.5a1 1 0 0 1 0 13 1 1 0 0 0 0-5H12a7 7 0 0 1-7-7V8\"></path><path d=\"M14 8a8.5 8.5 0 0 1 0 8\"></path><path d=\"M16 16c2 0 4.5-4 4-6\"></path>",
        categories = "animals",
        tags = "seafood,shellfish,crustacean,prawn,scallop,whelk,arthropod,littleneck,quahog,cherrystone",
        contributors = "karsa-mistmere"
    ))]
    Shrimp,
    #[cfg(feature = "shrink")]
    #[strum(props(
        svg = "<path d=\"m15 15 6 6m-6-6v4.8m0-4.8h4.8\"></path><path d=\"M9 19.8V15m0 0H4.2M9 15l-6 6\"></path><path d=\"M15 4.2V9m0 0h4.8M15 9l6-6\"></path><path d=\"M9 4.2V9m0 0H4.2M9 9 3 3\"></path>",
        categories = "layout,arrows",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Shrink,
    #[cfg(feature = "shrub")]
    #[strum(props(
        svg = "<path d=\"M12 22v-5.17a2 2 0 0 0-.586-1.41L9.5 13.5\"></path><path d=\"M14.5 14.5 12 17\"></path><path d=\"M17 8.8A6 6 0 0 1 13.8 20H10A6.5 6.5 0 0 1 7 8a5 5 0 0 1 10 0z\"></path>",
        categories = "nature",
        tags = "forest,undergrowth,park,nature",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Shrub,
    #[cfg(feature = "shuffle")]
    #[strum(props(
        svg = "<path d=\"m18 14 4 4-4 4\"></path><path d=\"m18 2 4 4-4 4\"></path><path d=\"M2 18h1.97a4 4 0 0 0 3.3-1.7l5.45-8.6a4 4 0 0 1 3.3-1.7H22\"></path><path d=\"M2 6h1.97a4 4 0 0 1 3.6 2.2\"></path><path d=\"M22 18h-6.04a4 4 0 0 1-3.3-1.8l-.359-.45\"></path>",
        categories = "multimedia,arrows",
        tags = "music,random,reorder",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere,jguddas"
    ))]
    Shuffle,
    #[cfg(feature = "sigma")]
    #[strum(props(
        svg = "<path d=\"M18 7V5a1 1 0 0 0-1-1H6.5a.5.5 0 0 0-.4.8l4.5 6a2 2 0 0 1 0 2.4l-4.5 6a.5.5 0 0 0 .4.8H17a1 1 0 0 0 1-1v-2\"></path>",
        categories = "text,math,science",
        tags = "sum,calculate,formula,math,enumeration,enumerate",
        contributors = "mittalyashu,johnletey,ericfennis"
    ))]
    Sigma,
    #[cfg(feature = "signal_high")]
    #[strum(props(
        svg = "<path d=\"M2 20h.01\"></path><path d=\"M7 20v-4\"></path><path d=\"M12 20v-8\"></path><path d=\"M17 20V8\"></path>",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalHigh,
    #[cfg(feature = "signal_low")]
    #[strum(props(
        svg = "<path d=\"M2 20h.01\"></path><path d=\"M7 20v-4\"></path>",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalLow,
    #[cfg(feature = "signal_medium")]
    #[strum(props(
        svg = "<path d=\"M2 20h.01\"></path><path d=\"M7 20v-4\"></path><path d=\"M12 20v-8\"></path>",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalMedium,
    #[cfg(feature = "signal_zero")]
    #[strum(props(
        svg = "<path d=\"M2 20h.01\"></path>",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g,lost",
        contributors = "ericfennis,azdle"
    ))]
    SignalZero,
    #[cfg(feature = "signal")]
    #[strum(props(
        svg = "<path d=\"M2 20h.01\"></path><path d=\"M7 20v-4\"></path><path d=\"M12 20v-8\"></path><path d=\"M17 20V8\"></path><path d=\"M22 4v16\"></path>",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    Signal,
    #[cfg(feature = "signature")]
    #[strum(props(
        svg = "<path d=\"m21 17-2.15-1.86A.5.5 0 0 0 18 15.5v.5a1 1 0 0 1-1 1h-2a1 1 0 0 1-1-1c0-2.54-3.99-3.97-8.5-4a1 1 0 0 0 0 5c4.15 0 4.74-11.29 5.70-13.5a2.5 2.5 0 1 1 3.31 3.28\"></path><path d=\"M3 21h18\"></path>",
        categories = "text",
        tags = "text,format,input,contract,autograph,handwriting,sign,cursive,ink,scribble,authorize,personal,agreement,legal,document,identity,authentic,approval,verification,unique",
        contributors = "AnnaSasDev,jguddas"
    ))]
    Signature,
    #[cfg(feature = "signpost_big")]
    #[strum(props(
        svg = "<path d=\"M10 9H4L2 7l2-2h6\"></path><path d=\"M14 5h6l2 2-2 2h-6\"></path><path d=\"M10 22V4a2 2 0 1 1 4 0v18\"></path><path d=\"M8 22h8\"></path>",
        categories = "arrows,navigation,development,gaming",
        tags = "bidirectional,left,right,east,west",
        contributors = "danielbayley"
    ))]
    SignpostBig,
    #[cfg(feature = "signpost")]
    #[strum(props(
        svg = "<path d=\"M12 13v8\"></path><path d=\"M12 3v3\"></path><path d=\"M2.35 10.35a1.20 1.20 0 0 1 0-1.70l2.06-2.06A2 2 0 0 1 5.82 6h12.34a2 2 0 0 1 1.41.586l2.06 2.06a1.20 1.20 0 0 1 0 1.70l-2.06 2.06a2 2 0 0 1-1.41.586H5.82a2 2 0 0 1-1.41-.586z\"></path>",
        categories = "arrows,navigation,development,gaming",
        tags = "navigation,direction,arrow,wayfinding,guide,location,pointer,route,indicator,marker,bidirectional,left,right,east,west",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Signpost,
    #[cfg(feature = "siren")]
    #[strum(props(
        svg = "<path d=\"M7 18v-6a5 5 0 1 1 10 0v6\"></path><path d=\"M5 21a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2z\"></path><path d=\"M21 12h1\"></path><path d=\"M18.5 4.5 18 5\"></path><path d=\"M2 12h1\"></path><path d=\"M12 2v1\"></path><path d=\"m4.92 4.92.707.70\"></path><path d=\"M12 12v6\"></path>",
        categories = "medical",
        tags = "police,ambulance,emergency,security,alert,alarm,light",
        contributors = "karsa-mistmere"
    ))]
    Siren,
    #[cfg(feature = "skip_back")]
    #[strum(props(
        svg = "<path d=\"M17.97 4.28A2 2 0 0 1 21 6v12a2 2 0 0 1-3.02 1.71l-9.99-5.99a2 2 0 0 1-.003-3.43z\"></path><path d=\"M3 20V4\"></path>",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "<path d=\"M21 4v16\"></path><path d=\"M6.02 4.28A2 2 0 0 0 3 6v12a2 2 0 0 0 3.02 1.71l9.99-5.99a2 2 0 0 0 .003-3.43z\"></path>",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "<path d=\"m12.5 17-.5-1-.5 1h1z\"></path><path d=\"M15 22a1 1 0 0 0 1-1v-1a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20v1a1 1 0 0 0 1 1z\"></path><circle cy=\"12\" r=\"1\" cx=\"15\"></circle><circle r=\"1\" cx=\"9\" cy=\"12\"></circle>",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slash")]
    #[strum(props(
        svg = "<path d=\"M22 2 2 22\"></path>",
        categories = "development,math",
        tags = "divide,division,or,/",
        contributors = "danielbayley"
    ))]
    Slash,
    #[cfg(feature = "slice")]
    #[strum(props(
        svg = "<path d=\"M11 16.58V19a1 1 0 0 1-1 1H2L18.37 3.63a1 1 0 1 1 3 3l-9.66 9.66a1 1 0 0 1-1.41 0L8 14\"></path>",
        categories = "design",
        tags = "cutter,scalpel,knife",
        contributors = "karsa-mistmere,danielbayley,jguddas"
    ))]
    Slice,
    #[cfg(feature = "sliders_horizontal")]
    #[strum(props(
        svg = "<path d=\"M10 5H3\"></path><path d=\"M12 19H3\"></path><path d=\"M14 3v4\"></path><path d=\"M16 17v4\"></path><path d=\"M21 12h-9\"></path><path d=\"M21 19h-5\"></path><path d=\"M21 5h-7\"></path><path d=\"M8 10v4\"></path><path d=\"M8 12H3\"></path>",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders_vertical")]
    #[strum(props(
        svg = "<path d=\"M10 8h4\"></path><path d=\"M12 21v-9\"></path><path d=\"M12 8V3\"></path><path d=\"M17 16h4\"></path><path d=\"M19 12V3\"></path><path d=\"M19 21v-5\"></path><path d=\"M3 14h4\"></path><path d=\"M5 10V3\"></path><path d=\"M5 21v-7\"></path>",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    SlidersVertical,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "<rect rx=\"2\" x=\"5\" height=\"20\" ry=\"2\" y=\"2\" width=\"14\"></rect><path d=\"M12.66 8 10 12h4l-2.66 4\"></path>",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "<rect height=\"12\" width=\"7\" x=\"2\" rx=\"1\" y=\"6\"></rect><path d=\"M13 8.32a7.43 7.43 0 0 1 0 7.36\"></path><path d=\"M16.46 6.21a11.76 11.76 0 0 1 0 11.58\"></path><path d=\"M19.91 4.1a15.91 15.91 0 0 1 .01 15.8\"></path>",
        categories = "communication,finance,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "<rect y=\"2\" height=\"20\" ry=\"2\" x=\"5\" width=\"14\" rx=\"2\"></rect><path d=\"M12 18h.01\"></path>",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "<path d=\"M22 11v1a10 10 0 1 1-9-10\"></path><path d=\"M8 14s1.5 2 4 2 4-2 4-2\"></path><line x2=\"9.01\" y2=\"9\" y1=\"9\" x1=\"9\"></line><line x2=\"15.01\" y2=\"9\" y1=\"9\" x1=\"15\"></line><path d=\"M16 5h6\"></path><path d=\"M19 2v6\"></path>",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "<circle cy=\"12\" cx=\"12\" r=\"10\"></circle><path d=\"M8 14s1.5 2 4 2 4-2 4-2\"></path><line x2=\"9.01\" x1=\"9\" y2=\"9\" y1=\"9\"></line><line y2=\"9\" x1=\"15\" x2=\"15.01\" y1=\"9\"></line>",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[cfg(feature = "snail")]
    #[strum(props(
        svg = "<path d=\"M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0\"></path><circle cy=\"13\" r=\"8\" cx=\"10\"></circle><path d=\"M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6\"></path><path d=\"M18 3 19.1 5.2\"></path><path d=\"M22 3 20.9 5.2\"></path>",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Snail,
    #[cfg(feature = "snowflake")]
    #[strum(props(
        svg = "<path d=\"m10 20-1.25-2.5L6 18\"></path><path d=\"M10 4 8.75 6.5 6 6\"></path><path d=\"m14 20 1.25-2.5L18 18\"></path><path d=\"m14 4 1.25 2.5L18 6\"></path><path d=\"m17 21-3-6h-4\"></path><path d=\"m17 3-3 6 1.5 3\"></path><path d=\"M2 12h6.5L10 9\"></path><path d=\"m20 10-1.5 2 1.5 2\"></path><path d=\"M22 12h-6.5L14 15\"></path><path d=\"m4 10 1.5 2L4 14\"></path><path d=\"m7 21 3-6-1.5-3\"></path><path d=\"m7 3 3 6h4\"></path>",
        categories = "weather,seasons",
        tags = "cold,weather,freeze,snow,winter",
        contributors = "karsa-mistmere,lscheibel,ericfennis"
    ))]
    Snowflake,
    #[cfg(feature = "soap_dispenser_droplet")]
    #[strum(props(
        svg = "<path d=\"M10.5 2v4\"></path><path d=\"M14 2H7a2 2 0 0 0-2 2\"></path><path d=\"M19.29 14.76A6.67 6.67 0 0 1 17 11a6.6 6.6 0 0 1-2.29 3.76c-1.15.92-1.71 2.04-1.71 3.19 0 2.22 1.8 4.05 4 4.05s4-1.83 4-4.05c0-1.16-.57-2.26-1.71-3.19\"></path><path d=\"M9.60 21H6a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h7V7a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3\"></path>",
        categories = "home,travel",
        tags = "wash,bath,water,liquid,fluid,wet,moisture,damp,bead,globule",
        contributors = "Andreto,ericfennis,jguddas"
    ))]
    SoapDispenserDroplet,
    #[cfg(feature = "sofa")]
    #[strum(props(
        svg = "<path d=\"M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3\"></path><path d=\"M2 16a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v1.5a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5V11a2 2 0 0 0-4 0z\"></path><path d=\"M4 18v2\"></path><path d=\"M20 18v2\"></path><path d=\"M12 4v9\"></path>",
        categories = "home",
        tags = "armchair,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere"
    ))]
    Sofa,
    #[cfg(feature = "solar_panel")]
    #[strum(props(
        svg = "<path d=\"M11 2h2\"></path><path d=\"m14.28 14-4.56 8\"></path><path d=\"m21 22-1.55-4H4.55\"></path><path d=\"M3 10v2\"></path><path d=\"M6.24 15.04A2 2 0 0 1 8 14h12a1 1 0 0 1 .864 1.50l-3.11 5.45A2 2 0 0 1 16 22H4a1 1 0 0 1-.863-1.50z\"></path><path d=\"M7 2a4 4 0 0 1-4 4\"></path><path d=\"m8.66 7.66 1.41 1.41\"></path>",
        categories = "home,science,sustainability,weather",
        tags = "solar panel,solar,panel,sun,energy,electricity,light",
        contributors = "UsamaKhan,jguddas,karsa-mistmere"
    ))]
    SolarPanel,
    #[cfg(feature = "soup")]
    #[strum(props(
        svg = "<path d=\"M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z\"></path><path d=\"M7 21h10\"></path><path d=\"M19.5 12 22 6\"></path><path d=\"M16.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.73 1.62\"></path><path d=\"M11.25 3c.27.1.8.53.74 1.36-.05.83-.93 1.2-.98 2.02-.06.78.33 1.24.72 1.62\"></path><path d=\"M6.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.74 1.62\"></path>",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,bowl,starter",
        contributors = "kemie"
    ))]
    Soup,
    #[cfg(feature = "space")]
    #[strum(props(
        svg = "<path d=\"M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1\"></path>",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    Space,
    #[cfg(feature = "spade")]
    #[strum(props(
        svg = "<path d=\"M12 18v4\"></path><path d=\"M2 14.49a5.5 5.5 0 0 0 9.59 3.67.6.6 0 0 1 .818.00A5.5 5.5 0 0 0 22 14.5c0-2.29-1.5-4-3-5.5l-5.49-5.31a2 2 0 0 0-3-.02L5 8.99c-1.5 1.5-3 3.2-3 5.5\"></path>",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    Spade,
    #[cfg(feature = "sparkle")]
    #[strum(props(
        svg = "<path d=\"M11.01 2.81a1 1 0 0 1 1.96 0l1.05 5.55a2 2 0 0 0 1.59 1.59l5.55 1.05a1 1 0 0 1 0 1.96l-5.55 1.05a2 2 0 0 0-1.59 1.59l-1.05 5.55a1 1 0 0 1-1.96 0l-1.05-5.55a2 2 0 0 0-1.59-1.59l-5.55-1.05a1 1 0 0 1 0-1.96l5.55-1.05a2 2 0 0 0 1.59-1.59z\"></path>",
        categories = "shapes",
        tags = "star,effect,filter,night,magic,shiny,glitter,twinkle,celebration",
        contributors = "Shiva953,karsa-mistmere"
    ))]
    Sparkle,
    #[cfg(feature = "sparkles")]
    #[strum(props(
        svg = "<path d=\"M11.01 2.81a1 1 0 0 1 1.96 0l1.05 5.55a2 2 0 0 0 1.59 1.59l5.55 1.05a1 1 0 0 1 0 1.96l-5.55 1.05a2 2 0 0 0-1.59 1.59l-1.05 5.55a1 1 0 0 1-1.96 0l-1.05-5.55a2 2 0 0 0-1.59-1.59l-5.55-1.05a1 1 0 0 1 0-1.96l5.55-1.05a2 2 0 0 0 1.59-1.59z\"></path><path d=\"M20 2v4\"></path><path d=\"M22 4h-4\"></path><circle cx=\"4\" r=\"2\" cy=\"20\"></circle>",
        categories = "cursors,multimedia,gaming,weather",
        tags = "stars,effect,filter,night,magic",
        contributors = "karsa-mistmere"
    ))]
    Sparkles,
    #[cfg(feature = "speaker")]
    #[strum(props(
        svg = "<rect height=\"20\" rx=\"2\" width=\"16\" x=\"4\" y=\"2\"></rect><path d=\"M12 6h.01\"></path><circle cy=\"14\" cx=\"12\" r=\"4\"></circle><path d=\"M12 14h.01\"></path>",
        categories = "multimedia,devices",
        tags = "sound,audio,music,tweeter,subwoofer,bass,production,producer,dj",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Speaker,
    #[cfg(feature = "speech")]
    #[strum(props(
        svg = "<path d=\"M8.8 20v-4.1l1.9.2a2.3 2.3 0 0 0 2.16-2.1V8.3A5.37 5.37 0 0 0 2 8.25c0 2.8.65 3.05 1 4.55a5.77 5.77 0 0 1 .029 2.75L2 20\"></path><path d=\"M19.8 17.8a7.5 7.5 0 0 0 .003-10.60\"></path><path d=\"M17 15a3.5 3.5 0 0 0-.025-4.97\"></path>",
        categories = "accessibility,communication",
        tags = "disability,disabled,dda,human,accessibility,people,sound",
        contributors = "doerge,airone01,jguddas,karsa-mistmere"
    ))]
    Speech,
    #[cfg(feature = "spell_check_2")]
    #[strum(props(
        svg = "<path d=\"m6 16 6-12 6 12\"></path><path d=\"M8 12h8\"></path><path d=\"M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1\"></path>",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck2,
    #[cfg(feature = "spell_check")]
    #[strum(props(
        svg = "<path d=\"m6 16 6-12 6 12\"></path><path d=\"M8 12h8\"></path><path d=\"m16 20 2 2 4-4\"></path>",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck,
    #[cfg(feature = "spline_pointer")]
    #[strum(props(
        svg = "<path d=\"M12.03 12.68a.498.49 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.94l-3.44 1.06a1 1 0 0 0-.66.66l-1.06 3.44a.5.5 0 0 1-.943.03z\"></path><path d=\"M5 17A12 12 0 0 1 17 5\"></path><circle cy=\"5\" cx=\"19\" r=\"2\"></circle><circle cx=\"5\" r=\"2\" cy=\"19\"></circle>",
        categories = "arrows,cursors,design,tools",
        tags = "path,tool,curve,node,click,pointer,target,vector",
        contributors = "kaleidosium,mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    SplinePointer,
    #[cfg(feature = "spline")]
    #[strum(props(
        svg = "<circle cx=\"19\" cy=\"5\" r=\"2\"></circle><circle cx=\"5\" cy=\"19\" r=\"2\"></circle><path d=\"M5 17A12 12 0 0 1 17 5\"></path>",
        categories = "design",
        tags = "path,pen,tool,shape,curve,draw",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split")]
    #[strum(props(
        svg = "<path d=\"M16 3h5v5\"></path><path d=\"M8 3H3v5\"></path><path d=\"M12 22v-8.3a4 4 0 0 0-1.17-2.87L3 3\"></path><path d=\"m15 9 6-6\"></path>",
        categories = "development,arrows",
        tags = "break,disband,divide,separate,branch,disunite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Split,
    #[cfg(feature = "spool")]
    #[strum(props(
        svg = "<path d=\"M17 13.44 4.44 17.08A2 2 0 0 0 4.98 21H19a2 2 0 0 0 .558-3.92l-1.11-.32A2 2 0 0 1 17 14.83V7.66\"></path><path d=\"m7 10.56 12.55-3.64A2 2 0 0 0 19.01 3H5a2 2 0 0 0-.558 3.92l1.11.32A2 2 0 0 1 7 9.16v7.17\"></path>",
        categories = "communication,tools,social",
        tags = "bobbin,spindle,yarn,thread,string,sewing,needlework",
        contributors = "karsa-mistmere"
    ))]
    Spool,
    #[cfg(feature = "sport_shoe")]
    #[strum(props(
        svg = "<path d=\"m15 10.42 4.8-5.07\"></path><path d=\"M19 18h3\"></path><path d=\"M9.5 22 21.41 9.41A2 2 0 0 0 21.2 6.4l-5.61-4.20A1 1 0 0 0 14 3v2a2 2 0 0 1-1.39 1.90L8.67 8.05A1 1 0 0 0 8 9c-.155 6.39-2.08 9-4 9a2 2 0 0 0 0 4h14\"></path>",
        categories = "sports",
        tags = "footwear,sports,running,athletic,shoe,sneaker,training,exercise,fitness",
        contributors = "Youya-ui"
    ))]
    SportShoe,
    #[cfg(feature = "spotlight")]
    #[strum(props(
        svg = "<path d=\"M15.29 19.56 16 22\"></path><path d=\"m17 16 3.75 2.09\"></path><path d=\"m19 12.5 3.02-.598\"></path><path d=\"M7.61 6.3a3 3 0 0 0-3.92 1.3l-1.38 2.79a3 3 0 0 0 1.3 3.91l6.89 3.59a1 1 0 0 0 1.34-.447l3.10-6.21a1 1 0 0 0-.447-1.34z\"></path><path d=\"M8 9V2\"></path>",
        categories = "devices,photography,multimedia,communication",
        tags = "winner,soapbox,stage,entertainment,drama,podium,actor,actress,singer,light,beam,play,theatre,show,focus,concert,performance,lens,leaderboard,followspot,best,highlight",
        contributors = "chessurisme,jguddas,karsa-mistmere,ericfennis"
    ))]
    Spotlight,
    #[cfg(feature = "spray_can")]
    #[strum(props(
        svg = "<path d=\"M3 3h.01\"></path><path d=\"M7 5h.01\"></path><path d=\"M11 7h.01\"></path><path d=\"M3 7h.01\"></path><path d=\"M7 9h.01\"></path><path d=\"M3 11h.01\"></path><rect y=\"5\" height=\"4\" width=\"4\" x=\"15\"></rect><path d=\"m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2\"></path><path d=\"m13 14 8-2\"></path><path d=\"m13 19 8-2\"></path>",
        categories = "design,tools",
        tags = "paint,color,graffiti,decoration,aerosol,deodorant,shaving foam,air freshener",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SprayCan,
    #[cfg(feature = "sprout")]
    #[strum(props(
        svg = "<path d=\"M14 9.53V7a4 4 0 0 1 4-4h1.5a.5.5 0 0 1 .5.5V5a4 4 0 0 1-4 4 4 4 0 0 0-4 4c0 2 1 3 1 5a5 5 0 0 1-1 3\"></path><path d=\"M4 9a5 5 0 0 1 8 4 5 5 0 0 1-8-4\"></path><path d=\"M5 21h14\"></path>",
        categories = "nature,gaming,sustainability",
        tags = "eco,green,growth,leaf,nature,plant,seed,spring,sustainability",
        contributors = "ericfennis,mittalyashu,jamiemlaw,karsa-mistmere,jguddas"
    ))]
    Sprout,
    #[cfg(feature = "square_activity")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" y=\"3\" x=\"3\" rx=\"2\"></rect><path d=\"M17 12h-2l-2 5-2-10-2 5H7\"></path>",
        categories = "medical,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "danielbayley"
    ))]
    SquareActivity,
    #[cfg(feature = "square_arrow_down_left")]
    #[strum(props(
        svg = "<rect x=\"3\" width=\"18\" height=\"18\" y=\"3\" rx=\"2\"></rect><path d=\"m16 8-8 8\"></path><path d=\"M16 16H8V8\"></path>",
        categories = "arrows,gaming",
        tags = "direction,south-west,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowDownLeft,
    #[cfg(feature = "square_arrow_down_right")]
    #[strum(props(
        svg = "<rect x=\"3\" rx=\"2\" y=\"3\" width=\"18\" height=\"18\"></rect><path d=\"m8 8 8 8\"></path><path d=\"M16 8v8H8\"></path>",
        categories = "arrows,gaming",
        tags = "direction,south-east,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowDownRight,
    #[cfg(feature = "square_arrow_down")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" y=\"3\" width=\"18\" x=\"3\"></rect><path d=\"M12 8v8\"></path><path d=\"m8 12 4 4 4-4\"></path>",
        categories = "arrows,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowDown,
    #[cfg(feature = "square_arrow_left")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"18\" height=\"18\" x=\"3\" y=\"3\"></rect><path d=\"m12 8-4 4 4 4\"></path><path d=\"M16 12H8\"></path>",
        categories = "arrows",
        tags = "previous,back,direction,west,sign,keyboard,button,<-",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowLeft,
    #[cfg(feature = "square_arrow_out_down_left")]
    #[strum(props(
        svg = "<path d=\"M13 21h6a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6\"></path><path d=\"m3 21 9-9\"></path><path d=\"M9 21H3v-6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutDownLeft,
    #[cfg(feature = "square_arrow_out_down_right")]
    #[strum(props(
        svg = "<path d=\"M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6\"></path><path d=\"m21 21-9-9\"></path><path d=\"M21 15v6h-6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutDownRight,
    #[cfg(feature = "square_arrow_out_up_left")]
    #[strum(props(
        svg = "<path d=\"M13 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-6\"></path><path d=\"m3 3 9 9\"></path><path d=\"M3 9V3h6\"></path>",
        categories = "arrows",
        tags = "outwards,direction,north-west,diagonal",
        contributors = "danielbayley"
    ))]
    SquareArrowOutUpLeft,
    #[cfg(feature = "square_arrow_out_up_right")]
    #[strum(props(
        svg = "<path d=\"M21 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6\"></path><path d=\"m21 3-9 9\"></path><path d=\"M15 3h6v6\"></path>",
        categories = "arrows,social",
        tags = "outwards,direction,north-east,diagonal,share,open,external,link",
        contributors = "danielbayley"
    ))]
    SquareArrowOutUpRight,
    #[cfg(feature = "square_arrow_right_enter")]
    #[strum(props(
        svg = "<path d=\"m10 16 4-4-4-4\"></path><path d=\"M3 12h11\"></path><path d=\"M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3\"></path>",
        categories = "arrows,shapes,layout,multimedia",
        tags = "left,in,inside,input,insert,source,import,place,->",
        contributors = "ethanhazel,karsa-mistmere,ericfennis"
    ))]
    SquareArrowRightEnter,
    #[cfg(feature = "square_arrow_right_exit")]
    #[strum(props(
        svg = "<path d=\"M10 12h11\"></path><path d=\"m17 16 4-4-4-4\"></path><path d=\"M21 6.34V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-1.34\"></path>",
        categories = "arrows,shapes,layout,multimedia",
        tags = "out,outside,output,export,->",
        contributors = "ethanhazel,karsa-mistmere,ericfennis"
    ))]
    SquareArrowRightExit,
    #[cfg(feature = "square_arrow_right")]
    #[strum(props(
        svg = "<rect x=\"3\" height=\"18\" y=\"3\" width=\"18\" rx=\"2\"></rect><path d=\"M8 12h8\"></path><path d=\"m12 16 4-4-4-4\"></path>",
        categories = "arrows",
        tags = "next,forward,direction,west,sign,keyboard,button,->",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowRight,
    #[cfg(feature = "square_arrow_up_left")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" width=\"18\" y=\"3\" x=\"3\"></rect><path d=\"M8 16V8h8\"></path><path d=\"M16 16 8 8\"></path>",
        categories = "arrows",
        tags = "direction,north-west,diagonal,sign,keyboard,button",
        contributors = "danielbayley"
    ))]
    SquareArrowUpLeft,
    #[cfg(feature = "square_arrow_up_right")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"18\" height=\"18\" x=\"3\" rx=\"2\"></rect><path d=\"M8 8h8v8\"></path><path d=\"m8 16 8-8\"></path>",
        categories = "arrows,social",
        tags = "direction,north-east,diagonal,sign,keyboard,button,share",
        contributors = "danielbayley"
    ))]
    SquareArrowUpRight,
    #[cfg(feature = "square_arrow_up")]
    #[strum(props(
        svg = "<rect height=\"18\" rx=\"2\" x=\"3\" y=\"3\" width=\"18\"></rect><path d=\"m16 12-4-4-4 4\"></path><path d=\"M12 16V8\"></path>",
        categories = "arrows",
        tags = "forward,direction,north,sign,keyboard,button",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareArrowUp,
    #[cfg(feature = "square_asterisk")]
    #[strum(props(
        svg = "<rect width=\"18\" y=\"3\" x=\"3\" height=\"18\" rx=\"2\"></rect><path d=\"M12 8v8\"></path><path d=\"m8.5 14 7-4\"></path><path d=\"m8.5 10 7 4\"></path>",
        categories = "text,security,math,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[cfg(feature = "square_bottom_dashed_scissors")]
    #[strum(props(
        svg = "<line y2=\"3\" y1=\"3\" x2=\"19\" x1=\"5\"></line><line x2=\"3\" y2=\"19\" y1=\"5\" x1=\"3\"></line><line x1=\"21\" x2=\"21\" y1=\"5\" y2=\"19\"></line><line y2=\"21\" y1=\"21\" x2=\"10\" x1=\"9\"></line><line y2=\"21\" x2=\"15\" x1=\"14\" y1=\"21\"></line><path d=\"M 3 5 A2 2 0 0 1 5 3\"></path><path d=\"M 19 3 A2 2 0 0 1 21 5\"></path><path d=\"M 5 21 A2 2 0 0 1 3 19\"></path><path d=\"M 21 19 A2 2 0 0 1 19 21\"></path><circle cx=\"8.5\" cy=\"8.5\" r=\"1.5\"></circle><line x2=\"12\" y2=\"12\" y1=\"9.56\" x1=\"9.56\"></line><line y1=\"17\" x1=\"17\" x2=\"14.82\" y2=\"14.82\"></line><circle cy=\"15.5\" r=\"1.5\" cx=\"8.5\"></circle><line x2=\"17\" y2=\"7\" y1=\"14.43\" x1=\"9.56\"></line>",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley,eden881"
    ))]
    SquareBottomDashedScissors,
    #[cfg(feature = "square_centerline_dashed_horizontal")]
    #[strum(props(
        svg = "<path d=\"M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3\"></path><path d=\"M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3\"></path><path d=\"M12 20v2\"></path><path d=\"M12 14v2\"></path><path d=\"M12 8v2\"></path><path d=\"M12 2v2\"></path>",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    SquareCenterlineDashedHorizontal,
    #[cfg(feature = "square_centerline_dashed_vertical")]
    #[strum(props(
        svg = "<path d=\"M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3\"></path><path d=\"M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3\"></path><path d=\"M4 12H2\"></path><path d=\"M10 12H8\"></path><path d=\"M16 12h-2\"></path><path d=\"M22 12h-2\"></path>",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    SquareCenterlineDashedVertical,
    #[cfg(feature = "square_chart_gantt")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" width=\"18\" y=\"3\" x=\"3\"></rect><path d=\"M9 8h7\"></path><path d=\"M8 12h6\"></path><path d=\"M11 16h5\"></path>",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    SquareChartGantt,
    #[cfg(feature = "square_check_big")]
    #[strum(props(
        svg = "<path d=\"M21 10.65V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h12.34\"></path><path d=\"m9 11 3 3L22 4\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    SquareCheckBig,
    #[cfg(feature = "square_check")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\"></rect><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "danielbayley"
    ))]
    SquareCheck,
    #[cfg(feature = "square_chevron_down")]
    #[strum(props(
        svg = "<rect height=\"18\" y=\"3\" width=\"18\" x=\"3\" rx=\"2\"></rect><path d=\"m16 10-4 4-4-4\"></path>",
        categories = "arrows,navigation",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronDown,
    #[cfg(feature = "square_chevron_left")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" y=\"3\" rx=\"2\" height=\"18\"></rect><path d=\"m14 16-4-4 4-4\"></path>",
        categories = "arrows,navigation",
        tags = "back,previous,less than,fewer,menu,panel,button,keyboard,<",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronLeft,
    #[cfg(feature = "square_chevron_right")]
    #[strum(props(
        svg = "<rect width=\"18\" rx=\"2\" height=\"18\" x=\"3\" y=\"3\"></rect><path d=\"m10 8 4 4-4 4\"></path>",
        categories = "arrows,navigation,development",
        tags = "forward,next,more than,greater,menu,panel,code,coding,command line,terminal,prompt,shell,console,>",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronRight,
    #[cfg(feature = "square_chevron_up")]
    #[strum(props(
        svg = "<rect x=\"3\" width=\"18\" y=\"3\" rx=\"2\" height=\"18\"></rect><path d=\"m8 14 4-4 4 4\"></path>",
        categories = "arrows,navigation,math",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareChevronUp,
    #[cfg(feature = "square_code")]
    #[strum(props(
        svg = "<path d=\"m10 9-3 3 3 3\"></path><path d=\"m14 15 3-3-3-3\"></path><rect width=\"18\" y=\"3\" rx=\"2\" height=\"18\" x=\"3\"></rect>",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = "danielbayley,jguddas,karsa-mistmere,ericfennis"
    ))]
    SquareCode,
    #[cfg(feature = "square_dashed_bottom_code")]
    #[strum(props(
        svg = "<path d=\"M10 9.5 8 12l2 2.5\"></path><path d=\"M14 21h1\"></path><path d=\"m14 9.5 2 2.5-2 2.5\"></path><path d=\"M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2\"></path><path d=\"M9 21h1\"></path>",
        categories = "development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottomCode,
    #[cfg(feature = "square_dashed_bottom")]
    #[strum(props(
        svg = "<path d=\"M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2\"></path><path d=\"M9 21h1\"></path><path d=\"M14 21h1\"></path>",
        categories = "development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedBottom,
    #[cfg(feature = "square_dashed_kanban")]
    #[strum(props(
        svg = "<path d=\"M8 7v7\"></path><path d=\"M12 7v4\"></path><path d=\"M16 7v9\"></path><path d=\"M5 3a2 2 0 0 0-2 2\"></path><path d=\"M9 3h1\"></path><path d=\"M14 3h1\"></path><path d=\"M19 3a2 2 0 0 1 2 2\"></path><path d=\"M21 9v1\"></path><path d=\"M21 14v1\"></path><path d=\"M21 19a2 2 0 0 1-2 2\"></path><path d=\"M14 21h1\"></path><path d=\"M9 21h1\"></path><path d=\"M5 21a2 2 0 0 1-2-2\"></path><path d=\"M3 14v1\"></path><path d=\"M3 9v1\"></path>",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,draft,template,boilerplate,code,coding",
        contributors = "danielbayley"
    ))]
    SquareDashedKanban,
    #[cfg(feature = "square_dashed_mouse_pointer")]
    #[strum(props(
        svg = "<path d=\"M12.03 12.68a.498.49 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.94l-3.44 1.06a1 1 0 0 0-.66.66l-1.06 3.44a.5.5 0 0 1-.943.03z\"></path><path d=\"M5 3a2 2 0 0 0-2 2\"></path><path d=\"M19 3a2 2 0 0 1 2 2\"></path><path d=\"M5 21a2 2 0 0 1-2-2\"></path><path d=\"M9 3h1\"></path><path d=\"M9 21h2\"></path><path d=\"M14 3h1\"></path><path d=\"M3 9v1\"></path><path d=\"M21 9v2\"></path><path d=\"M3 14v1\"></path>",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "danielbayley"
    ))]
    SquareDashedMousePointer,
    #[cfg(feature = "square_dashed_text")]
    #[strum(props(
        svg = "<path d=\"M14 21h1\"></path><path d=\"M14 3h1\"></path><path d=\"M19 3a2 2 0 0 1 2 2\"></path><path d=\"M21 14v1\"></path><path d=\"M21 19a2 2 0 0 1-2 2\"></path><path d=\"M21 9v1\"></path><path d=\"M3 14v1\"></path><path d=\"M3 9v1\"></path><path d=\"M5 21a2 2 0 0 1-2-2\"></path><path d=\"M5 3a2 2 0 0 0-2 2\"></path><path d=\"M7 12h10\"></path><path d=\"M7 16h6\"></path><path d=\"M7 8h8\"></path><path d=\"M9 21h1\"></path><path d=\"M9 3h1\"></path>",
        categories = "text,cursors",
        tags = "find,search,selection,dashed",
        contributors = "danielbayley"
    ))]
    SquareDashedText,
    #[cfg(feature = "square_dashed_top_solid")]
    #[strum(props(
        svg = "<path d=\"M14 21h1\"></path><path d=\"M21 14v1\"></path><path d=\"M21 19a2 2 0 0 1-2 2\"></path><path d=\"M21 9v1\"></path><path d=\"M3 14v1\"></path><path d=\"M3 5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2\"></path><path d=\"M3 9v1\"></path><path d=\"M5 21a2 2 0 0 1-2-2\"></path><path d=\"M9 21h1\"></path>",
        categories = "design,development,layout",
        tags = "square,border,width,layout,style,design,rectangular,marquee,dashed,box,rectangle,aspect ratio,1:1",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,chessurisme,danielbayley,colebemis,juanpablofernandez"
    ))]
    SquareDashedTopSolid,
    #[cfg(feature = "square_dashed")]
    #[strum(props(
        svg = "<path d=\"M5 3a2 2 0 0 0-2 2\"></path><path d=\"M19 3a2 2 0 0 1 2 2\"></path><path d=\"M21 19a2 2 0 0 1-2 2\"></path><path d=\"M5 21a2 2 0 0 1-2-2\"></path><path d=\"M9 3h1\"></path><path d=\"M9 21h1\"></path><path d=\"M14 3h1\"></path><path d=\"M14 21h1\"></path><path d=\"M3 9v1\"></path><path d=\"M21 9v1\"></path><path d=\"M3 14v1\"></path><path d=\"M21 14v1\"></path>",
        categories = "text,design",
        tags = "selection,square,rectangular,marquee,tool,dashed,box",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere,chessurisme"
    ))]
    SquareDashed,
    #[cfg(feature = "square_divide")]
    #[strum(props(
        svg = "<rect width=\"18\" y=\"3\" x=\"3\" rx=\"2\" height=\"18\" ry=\"2\"></rect><line x2=\"16\" y2=\"12\" x1=\"8\" y1=\"12\"></line><line x2=\"12\" y2=\"16\" y1=\"16\" x1=\"12\"></line><line y2=\"8\" x1=\"12\" x2=\"12\" y1=\"8\"></line>",
        categories = "math",
        tags = "calculate,math,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    SquareDivide,
    #[cfg(feature = "square_dot")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" y=\"3\" width=\"18\" x=\"3\"></rect><circle cy=\"12\" r=\"1\" cx=\"12\"></circle>",
        categories = "development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareDot,
    #[cfg(feature = "square_equal")]
    #[strum(props(
        svg = "<rect x=\"3\" rx=\"2\" width=\"18\" y=\"3\" height=\"18\"></rect><path d=\"M7 10h10\"></path><path d=\"M7 14h10\"></path>",
        categories = "math",
        tags = "calculate,=",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareEqual,
    #[cfg(feature = "square_function")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" rx=\"2\" height=\"18\" ry=\"2\" width=\"18\"></rect><path d=\"M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3\"></path><path d=\"M9 11.2h5.7\"></path>",
        categories = "development,math",
        tags = "programming,code,automation,math",
        contributors = "mittalyashu,ericfennis"
    ))]
    SquareFunction,
    #[cfg(feature = "square_kanban")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\"></rect><path d=\"M8 7v7\"></path><path d=\"M12 7v4\"></path><path d=\"M16 7v9\"></path>",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    SquareKanban,
    #[cfg(feature = "square_library")]
    #[strum(props(
        svg = "<rect height=\"18\" y=\"3\" x=\"3\" rx=\"2\" width=\"18\"></rect><path d=\"M7 7v10\"></path><path d=\"M11 7v10\"></path><path d=\"m15 7 2 10\"></path>",
        categories = "text,photography,multimedia,navigation,development",
        tags = "books,reading,written,authors,stories,fiction,novels,information,knowledge,education,high school,university,college,academy,learning,study,research,collection,vinyl,records,albums,music,package",
        contributors = "danielbayley"
    ))]
    SquareLibrary,
    #[cfg(feature = "square_m")]
    #[strum(props(
        svg = "<path d=\"M8 16V8.5a.5.5 0 0 1 .9-.3l2.7 3.59a.5.5 0 0 0 .8 0l2.7-3.6a.5.5 0 0 1 .9.3V16\"></path><rect y=\"3\" height=\"18\" rx=\"2\" x=\"3\" width=\"18\"></rect>",
        categories = "transportation,navigation",
        tags = "metro,subway,underground,track,line",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    SquareM,
    #[cfg(feature = "square_menu")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" x=\"3\" y=\"3\" width=\"18\"></rect><path d=\"M7 8h10\"></path><path d=\"M7 12h10\"></path><path d=\"M7 16h10\"></path>",
        categories = "layout",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    SquareMenu,
    #[cfg(feature = "square_minus")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"18\" x=\"3\" y=\"3\" height=\"18\"></rect><path d=\"M8 12h8\"></path>",
        categories = "math,development,text,tools,devices",
        tags = "subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    SquareMinus,
    #[cfg(feature = "square_mouse_pointer")]
    #[strum(props(
        svg = "<path d=\"M12.03 12.68a.498.49 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.94l-3.44 1.06a1 1 0 0 0-.66.66l-1.06 3.44a.5.5 0 0 1-.943.03z\"></path><path d=\"M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6\"></path>",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    SquareMousePointer,
    #[cfg(feature = "square_parking_off")]
    #[strum(props(
        svg = "<path d=\"M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41\"></path><path d=\"M3 8.7V19a2 2 0 0 0 2 2h10.3\"></path><path d=\"m2 2 20 20\"></path><path d=\"M13 13a3 3 0 1 0 0-6H9v2\"></path><path d=\"M9 17v-2.3\"></path>",
        categories = "transportation,navigation",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    SquareParkingOff,
    #[cfg(feature = "square_parking")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" width=\"18\" height=\"18\" x=\"3\"></rect><path d=\"M9 17V7h4a3 3 0 0 1 0 6H9\"></path>",
        categories = "transportation,navigation",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    SquareParking,
    #[cfg(feature = "square_pause")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" width=\"18\" rx=\"2\" height=\"18\"></rect><line x2=\"10\" y1=\"15\" y2=\"9\" x1=\"10\"></line><line y1=\"15\" x1=\"14\" x2=\"14\" y2=\"9\"></line>",
        categories = "multimedia",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SquarePause,
    #[cfg(feature = "square_pen")]
    #[strum(props(
        svg = "<path d=\"M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7\"></path><path d=\"M18.37 2.62a1 1 0 0 1 3 3l-9.01 9.01a2 2 0 0 1-.853.50l-2.87.84a.5.5 0 0 1-.62-.62l.84-2.87a2 2 0 0 1 .506-.852z\"></path>",
        categories = "text",
        tags = "pencil,edit,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    SquarePen,
    #[cfg(feature = "square_percent")]
    #[strum(props(
        svg = "<rect x=\"3\" height=\"18\" width=\"18\" rx=\"2\" y=\"3\"></rect><path d=\"m15 9-6 6\"></path><path d=\"M9 9h.01\"></path><path d=\"M15 15h.01\"></path>",
        categories = "social,finance,shopping,math",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    SquarePercent,
    #[cfg(feature = "square_pi")]
    #[strum(props(
        svg = "<rect rx=\"2\" width=\"18\" y=\"3\" height=\"18\" x=\"3\"></rect><path d=\"M7 7h10\"></path><path d=\"M10 7v10\"></path><path d=\"M16 17a2 2 0 0 1-2-2V7\"></path>",
        categories = "development,math",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    SquarePi,
    #[cfg(feature = "square_pilcrow")]
    #[strum(props(
        svg = "<rect x=\"3\" height=\"18\" y=\"3\" rx=\"2\" width=\"18\"></rect><path d=\"M12 12H9.5a2.5 2.5 0 0 1 0-5H17\"></path><path d=\"M12 7v10\"></path><path d=\"M16 7v10\"></path>",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "danielbayley"
    ))]
    SquarePilcrow,
    #[cfg(feature = "square_play")]
    #[strum(props(
        svg = "<rect height=\"18\" x=\"3\" rx=\"2\" width=\"18\" y=\"3\"></rect><path d=\"M9 9.00a1 1 0 0 1 1.51-.859l4.99 2.99a1 1 0 0 1 0 1.71l-4.99 2.99A1 1 0 0 1 9 14.99z\"></path>",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    SquarePlay,
    #[cfg(feature = "square_plus")]
    #[strum(props(
        svg = "<rect height=\"18\" y=\"3\" rx=\"2\" width=\"18\" x=\"3\"></rect><path d=\"M8 12h8\"></path><path d=\"M12 8v8\"></path>",
        categories = "math,tools,development,text",
        tags = "add,new,increase,increment,positive,calculate,calculator,button,keyboard,toolbar,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    SquarePlus,
    #[cfg(feature = "square_power")]
    #[strum(props(
        svg = "<path d=\"M12 7v4\"></path><path d=\"M7.99 9.00a5 5 0 1 0 8-.005\"></path><rect height=\"18\" width=\"18\" rx=\"2\" x=\"3\" y=\"3\"></rect>",
        categories = "connectivity",
        tags = "on,off,device,switch,toggle,binary,boolean,reboot,restart,button,keyboard,troubleshoot",
        contributors = "danielbayley,jguddas"
    ))]
    SquarePower,
    #[cfg(feature = "square_radical")]
    #[strum(props(
        svg = "<path d=\"M7 12h2l2 5 2-10h4\"></path><rect height=\"18\" x=\"3\" width=\"18\" y=\"3\" rx=\"2\"></rect>",
        categories = "development,math",
        tags = "calculate,formula,math,operator,root,square,symbol",
        contributors = "smnandre"
    ))]
    SquareRadical,
    #[cfg(feature = "square_round_corner")]
    #[strum(props(
        svg = "<path d=\"M21 11a8 8 0 0 0-8-8\"></path><path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4\"></path>",
        categories = "design,development,layout",
        tags = "border,radius,style,design,corner,layout,round,rounded",
        contributors = "liamb13,jguddas"
    ))]
    SquareRoundCorner,
    #[cfg(feature = "square_scissors")]
    #[strum(props(
        svg = "<rect height=\"18\" width=\"18\" y=\"3\" rx=\"2\" x=\"3\"></rect><circle r=\"1.5\" cx=\"8.5\" cy=\"8.5\"></circle><line y1=\"9.56\" x2=\"12\" x1=\"9.56\" y2=\"12\"></line><line y1=\"17\" x2=\"14.82\" y2=\"14.82\" x1=\"17\"></line><circle cx=\"8.5\" r=\"1.5\" cy=\"15.5\"></circle><line y1=\"14.43\" x2=\"17\" y2=\"7\" x1=\"9.56\"></line>",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley,eden881"
    ))]
    SquareScissors,
    #[cfg(feature = "square_sigma")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\"></rect><path d=\"M16 8.9V7H8l4 5-4 5h8v-1.9\"></path>",
        categories = "text,math",
        tags = "sum,calculate,formula,math,enumeration,enumerate",
        contributors = "danielbayley"
    ))]
    SquareSigma,
    #[cfg(feature = "square_slash")]
    #[strum(props(
        svg = "<rect x=\"3\" height=\"18\" y=\"3\" rx=\"2\" width=\"18\"></rect><line y2=\"9\" x1=\"9\" x2=\"15\" y1=\"15\"></line>",
        categories = "development,math",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,ericfennis"
    ))]
    SquareSlash,
    #[cfg(feature = "square_split_horizontal")]
    #[strum(props(
        svg = "<path d=\"M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3\"></path><path d=\"M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3\"></path><line y1=\"4\" y2=\"20\" x1=\"12\" x2=\"12\"></line>",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SquareSplitHorizontal,
    #[cfg(feature = "square_split_vertical")]
    #[strum(props(
        svg = "<path d=\"M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3\"></path><path d=\"M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3\"></path><line x1=\"4\" y1=\"12\" x2=\"20\" y2=\"12\"></line>",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SquareSplitVertical,
    #[cfg(feature = "square_square")]
    #[strum(props(
        svg = "<rect y=\"3\" width=\"18\" height=\"18\" rx=\"2\" x=\"3\"></rect><rect x=\"8\" height=\"8\" width=\"8\" rx=\"1\" y=\"8\"></rect>",
        categories = "layout",
        tags = "float,center,rectangle",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    SquareSquare,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "<path d=\"M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2\"></path><path d=\"M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2\"></path><rect x=\"14\" height=\"8\" rx=\"2\" width=\"8\" y=\"14\"></rect>",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = "danielbayley"
    ))]
    SquareStack,
    #[cfg(feature = "square_star")]
    #[strum(props(
        svg = "<path d=\"M11.03 7.69a1 1 0 0 1 1.90.024l.737 1.45a1 1 0 0 0 .737.53l1.63.256a1 1 0 0 1 .588 1.80l-1.17 1.16a1 1 0 0 0-.282.86l.259 1.61a1 1 0 0 1-1.54 1.13l-1.46-.75a1 1 0 0 0-.912 0l-1.46.75a1 1 0 0 1-1.53-1.13l.258-1.61a1 1 0 0 0-.282-.866l-1.15-1.15a1 1 0 0 1 .572-1.82l1.63-.256a1 1 0 0 0 .737-.535z\"></path><rect rx=\"2\" y=\"3\" width=\"18\" height=\"18\" x=\"3\"></rect>",
        categories = "sports,gaming",
        tags = "badge,medal,honour,decoration,order,pin,laurel,trophy,medallion,insignia,bronze,silver,gold",
        contributors = "karsa-mistmere"
    ))]
    SquareStar,
    #[cfg(feature = "square_stop")]
    #[strum(props(
        svg = "<rect x=\"3\" y=\"3\" rx=\"2\" height=\"18\" width=\"18\"></rect><rect x=\"9\" y=\"9\" width=\"6\" height=\"6\" rx=\"1\"></rect>",
        categories = "multimedia",
        tags = "media,music",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    SquareStop,
    #[cfg(feature = "square_terminal")]
    #[strum(props(
        svg = "<path d=\"m7 11 2-2-2-2\"></path><path d=\"M11 13h4\"></path><rect ry=\"2\" height=\"18\" x=\"3\" rx=\"2\" width=\"18\" y=\"3\"></rect>",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    SquareTerminal,
    #[cfg(feature = "square_user_round")]
    #[strum(props(
        svg = "<path d=\"M18 21a6 6 0 0 0-12 0\"></path><circle cx=\"12\" r=\"4\" cy=\"11\"></circle><rect rx=\"2\" height=\"18\" y=\"3\" width=\"18\" x=\"3\"></rect>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUserRound,
    #[cfg(feature = "square_user")]
    #[strum(props(
        svg = "<rect height=\"18\" y=\"3\" x=\"3\" width=\"18\" rx=\"2\"></rect><circle cy=\"10\" cx=\"12\" r=\"3\"></circle><path d=\"M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2\"></path>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    SquareUser,
    #[cfg(feature = "square_x")]
    #[strum(props(
        svg = "<rect width=\"18\" y=\"3\" ry=\"2\" x=\"3\" height=\"18\" rx=\"2\"></rect><path d=\"m15 9-6 6\"></path><path d=\"m9 9 6 6\"></path>",
        categories = "math,notifications",
        tags = "cancel,close,delete,remove,times,clear,math,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    SquareX,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "<rect height=\"18\" x=\"3\" rx=\"2\" width=\"18\" y=\"3\"></rect>",
        categories = "shapes,multimedia",
        tags = "stop,playback,music,audio,video,rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
    #[cfg(feature = "squares_exclude")]
    #[strum(props(
        svg = "<path d=\"M16 12v2a2 2 0 0 1-2 2H9a1 1 0 0 0-1 1v3a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V10a2 2 0 0 0-2-2h0\"></path><path d=\"M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v3a1 1 0 0 1-1 1h-5a2 2 0 0 0-2 2v2\"></path>",
        categories = "design",
        tags = "square,pathfinder,path,exclude,invert,xor,shape,vector",
        contributors = "EthanHazel,jguddas,jamiemlaw,karsa-mistmere"
    ))]
    SquaresExclude,
    #[cfg(feature = "squares_intersect")]
    #[strum(props(
        svg = "<path d=\"M10 22a2 2 0 0 1-2-2\"></path><path d=\"M14 2a2 2 0 0 1 2 2\"></path><path d=\"M16 22h-2\"></path><path d=\"M2 10V8\"></path><path d=\"M2 4a2 2 0 0 1 2-2\"></path><path d=\"M20 8a2 2 0 0 1 2 2\"></path><path d=\"M22 14v2\"></path><path d=\"M22 20a2 2 0 0 1-2 2\"></path><path d=\"M4 16a2 2 0 0 1-2-2\"></path><path d=\"M8 10a2 2 0 0 1 2-2h5a1 1 0 0 1 1 1v5a2 2 0 0 1-2 2H9a1 1 0 0 1-1-1z\"></path><path d=\"M8 2h2\"></path>",
        categories = "design",
        tags = "square,pathfinder,path,intersect,shape,include,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresIntersect,
    #[cfg(feature = "squares_subtract")]
    #[strum(props(
        svg = "<path d=\"M10 22a2 2 0 0 1-2-2\"></path><path d=\"M16 22h-2\"></path><path d=\"M16 4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h3a1 1 0 0 0 1-1v-5a2 2 0 0 1 2-2h5a1 1 0 0 0 1-1z\"></path><path d=\"M20 8a2 2 0 0 1 2 2\"></path><path d=\"M22 14v2\"></path><path d=\"M22 20a2 2 0 0 1-2 2\"></path>",
        categories = "design",
        tags = "square,pathfinder,path,minus,subtract,subtraction,shape,front,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresSubtract,
    #[cfg(feature = "squares_unite")]
    #[strum(props(
        svg = "<path d=\"M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v3a1 1 0 0 0 1 1h3a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2v-3a1 1 0 0 0-1-1z\"></path>",
        categories = "design",
        tags = "square,pathfinder,path,unite,union,shape,merge,vector",
        contributors = "EthanHazel,jguddas,karsa-mistmere"
    ))]
    SquaresUnite,
    #[cfg(feature = "squircle_dashed")]
    #[strum(props(
        svg = "<path d=\"M13.77 3.04a34 34 0 0 0-3.54 0\"></path><path d=\"M13.77 20.95a33 33 0 0 1-3.54.001\"></path><path d=\"M20.18 17.74c-.51 1.15-1.29 1.93-2.43 2.44\"></path><path d=\"M20.18 6.25c-.51-1.14-1.29-1.92-2.44-2.43\"></path><path d=\"M20.95 10.23a33 33 0 0 1 0 3.54\"></path><path d=\"M3.04 10.23a34 34 0 0 0 .001 3.54\"></path><path d=\"M6.26 20.17c-1.15-.508-1.93-1.29-2.44-2.43\"></path><path d=\"M6.26 3.82c-1.14.51-1.93 1.29-2.44 2.44\"></path>",
        categories = "development,shapes,design",
        tags = "shape,pending,progress,issue,draft,code,coding,version control",
        contributors = "jguddas,aramsoneson"
    ))]
    SquircleDashed,
    #[cfg(feature = "squircle")]
    #[strum(props(
        svg = "<path d=\"M12 3c7.2 0 9 1.8 9 9s-1.8 9-9 9-9-1.8-9-9 1.8-9 9-9\"></path>",
        categories = "shapes",
        tags = "shape",
        contributors = "jguddas"
    ))]
    Squircle,
    #[cfg(feature = "squirrel")]
    #[strum(props(
        svg = "<path d=\"M15.23 22a3 3 0 0 0-2.2-5\"></path><path d=\"M16 20a3 3 0 0 1 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4\"></path><path d=\"M18 13h.01\"></path><path d=\"M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10\"></path>",
        categories = "animals",
        tags = "animal,rodent,pet,pest,nuts,retrieve,updates,storage,stash",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Squirrel,
    #[cfg(feature = "stamp")]
    #[strum(props(
        svg = "<path d=\"M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-6 0c0 2 1 2 1 3.5V13\"></path><path d=\"M20 15.5a2.5 2.5 0 0 0-2.5-2.5h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1z\"></path><path d=\"M5 22h14\"></path>",
        categories = "design,cursors,tools",
        tags = "mark,print,clone,loyalty,library",
        contributors = "karsa-mistmere"
    ))]
    Stamp,
    #[cfg(feature = "star_half")]
    #[strum(props(
        svg = "<path d=\"M12 18.33a2.1 2.1 0 0 0-.987.24L6.39 21.01a.53.53 0 0 1-.77-.56l.881-5.13a2.12 2.12 0 0 0-.611-1.87L2.16 9.79a.53.53 0 0 1 .294-.906l5.16-.755a2.12 2.12 0 0 0 1.59-1.16l2.30-4.67A.53.53 0 0 1 12 2\"></path>",
        categories = "social,multimedia",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "mittalyashu,ericfennis,jguddas"
    ))]
    StarHalf,
    #[cfg(feature = "star_off")]
    #[strum(props(
        svg = "<path d=\"m10.34 4.68 1.18-2.39a.53.53 0 0 1 .95 0l2.31 4.67a2.12 2.12 0 0 0 1.59 1.16l5.16.756a.53.53 0 0 1 .294.90l-3.23 3.15\"></path><path d=\"m17.94 17.94.43 2.50a.53.53 0 0 1-.771.56l-4.61-2.42a2.12 2.12 0 0 0-1.97 0L6.39 21.01a.53.53 0 0 1-.77-.56l.881-5.13a2.12 2.12 0 0 0-.611-1.87L2.16 9.79a.53.53 0 0 1 .294-.906l5.16-.755a8 8 0 0 0 .4-.099\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "multimedia,social",
        tags = "dislike,unlike,remove,unrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StarOff,
    #[cfg(feature = "star")]
    #[strum(props(
        svg = "<path d=\"M11.52 2.29a.53.53 0 0 1 .95 0l2.31 4.67a2.12 2.12 0 0 0 1.59 1.16l5.16.756a.53.53 0 0 1 .294.90l-3.73 3.63a2.12 2.12 0 0 0-.611 1.87l.882 5.14a.53.53 0 0 1-.771.56l-4.61-2.42a2.12 2.12 0 0 0-1.97 0L6.39 21.01a.53.53 0 0 1-.77-.56l.881-5.13a2.12 2.12 0 0 0-.611-1.87L2.16 9.79a.53.53 0 0 1 .294-.906l5.16-.755a2.12 2.12 0 0 0 1.59-1.16z\"></path>",
        categories = "account,social,shapes,multimedia,weather,emoji,gaming",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "colebemis,jguddas"
    ))]
    Star,
    #[cfg(feature = "step_back")]
    #[strum(props(
        svg = "<path d=\"M13.97 4.28A2 2 0 0 1 17 6v12a2 2 0 0 1-3.02 1.71l-9.99-5.99a2 2 0 0 1-.003-3.43z\"></path><path d=\"M21 20V4\"></path>",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "<path d=\"M10.02 4.28A2 2 0 0 0 7 6v12a2 2 0 0 0 3.02 1.71l9.99-5.99a2 2 0 0 0 .003-3.43z\"></path><path d=\"M3 4v16\"></path>",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "<path d=\"M11 2v2\"></path><path d=\"M5 2v2\"></path><path d=\"M5 3H4a2 2 0 0 0-2 2v4a6 6 0 0 0 12 0V5a2 2 0 0 0-2-2h-1\"></path><path d=\"M8 15a6 6 0 0 0 12 0v-3\"></path><circle r=\"2\" cx=\"20\" cy=\"10\"></circle>",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[cfg(feature = "sticker")]
    #[strum(props(
        svg = "<path d=\"M21 9a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z\"></path><path d=\"M15 3v5a1 1 0 0 0 1 1h5\"></path><path d=\"M8 13h.01\"></path><path d=\"M16 13h.01\"></path><path d=\"M10 16s.8 1 2 1c1.3 0 2-1 2-1\"></path>",
        categories = "social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Sticker,
    #[cfg(feature = "sticky_note")]
    #[strum(props(
        svg = "<path d=\"M21 9a2.4 2.4 0 0 0-.706-1.70l-3.58-3.58A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z\"></path><path d=\"M15 3v5a1 1 0 0 0 1 1h5\"></path>",
        categories = "text,social",
        tags = "post-it,comment,annotation,reaction,memo,reminder,todo,task,idea,brainstorm,document,page,paper,sheet,stationary,office",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    StickyNote,
    #[cfg(feature = "stone")]
    #[strum(props(
        svg = "<path d=\"M11.26 2.20A4 4 0 0 0 6.42 4.21l-4 8a4 4 0 0 0 1.35 5.11l6 4a4 4 0 0 0 4.43 0l6-4a4 4 0 0 0 1.57-4.59l-2-6a4 4 0 0 0-2.53-2.53z\"></path><path d=\"M11.99 22 14 12l7.82 3.18\"></path><path d=\"M14 12 8.47 2.30\"></path>",
        categories = "nature",
        tags = "mineral,geology,nature,solid,pebble,crystal,ore,hard,coal,stone,rock,boulder",
        contributors = "Alportan,karsa-mistmere"
    ))]
    Stone,
    #[cfg(feature = "store")]
    #[strum(props(
        svg = "<path d=\"M15 21v-5a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v5\"></path><path d=\"M17.77 10.31a1.12 1.12 0 0 0-1.54 0 2.5 2.5 0 0 1-3.45 0 1.12 1.12 0 0 0-1.54 0 2.5 2.5 0 0 1-3.45 0 1.12 1.12 0 0 0-1.54 0 2.5 2.5 0 0 1-3.77-3.24l2.88-4.18A2 2 0 0 1 7 2h10a2 2 0 0 1 1.65.873l2.89 4.19a2.5 2.5 0 0 1-3.77 3.24\"></path><path d=\"M4 10.95V19a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8.05\"></path>",
        categories = "buildings,navigation,shopping",
        tags = "shop,supermarket,stand,boutique,building",
        contributors = "karsa-mistmere"
    ))]
    Store,
    #[cfg(feature = "stretch_horizontal")]
    #[strum(props(
        svg = "<rect y=\"4\" width=\"20\" height=\"6\" rx=\"2\" x=\"2\"></rect><rect height=\"6\" width=\"20\" y=\"14\" x=\"2\" rx=\"2\"></rect>",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "<rect y=\"2\" width=\"6\" x=\"4\" rx=\"2\" height=\"20\"></rect><rect width=\"6\" x=\"14\" y=\"2\" height=\"20\" rx=\"2\"></rect>",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "<path d=\"M16 4H9a3 3 0 0 0-2.83 4\"></path><path d=\"M14 12a4 4 0 0 1 0 8H6\"></path><line y1=\"12\" x1=\"4\" y2=\"12\" x2=\"20\"></line>",
        categories = "text",
        tags = "cross out,delete,remove,format",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Strikethrough,
    #[cfg(feature = "subscript")]
    #[strum(props(
        svg = "<path d=\"m4 5 8 8\"></path><path d=\"m12 5-8 8\"></path><path d=\"M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07\"></path>",
        categories = "text",
        tags = "text",
        contributors = "nabanita-sarkar,ericfennis,mittalyashu"
    ))]
    Subscript,
    #[cfg(feature = "sun_dim")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"4\" cx=\"12\"></circle><path d=\"M12 4h.01\"></path><path d=\"M20 12h.01\"></path><path d=\"M12 20h.01\"></path><path d=\"M4 12h.01\"></path><path d=\"M17.65 6.34h.01\"></path><path d=\"M17.65 17.65h.01\"></path><path d=\"M6.34 17.65h.01\"></path><path d=\"M6.34 6.34h.01\"></path>",
        categories = "accessibility,weather",
        tags = "brightness,dim,low,brightness low",
        contributors = "mittalyashu,bduffany,karsa-mistmere"
    ))]
    SunDim,
    #[cfg(feature = "sun_medium")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"4\"></circle><path d=\"M12 3v1\"></path><path d=\"M12 20v1\"></path><path d=\"M3 12h1\"></path><path d=\"M20 12h1\"></path><path d=\"m18.36 5.63-.707.70\"></path><path d=\"m6.34 17.65-.707.70\"></path><path d=\"m5.63 5.63.707.70\"></path><path d=\"m17.65 17.65.707.70\"></path>",
        categories = "accessibility,weather",
        tags = "brightness,medium",
        contributors = "mittalyashu,karsa-mistmere"
    ))]
    SunMedium,
    #[cfg(feature = "sun_moon")]
    #[strum(props(
        svg = "<path d=\"M12 2v2\"></path><path d=\"M14.83 16.38a6 6 0 1 1-7.22-7.22c.624-.147.97.66.71 1.24a4 4 0 0 0 5.26 5.25c.589-.255 1.39.09 1.24.715\"></path><path d=\"M16 12a4 4 0 0 0-4-4\"></path><path d=\"m19 5-1.25 1.25\"></path><path d=\"M20 12h2\"></path>",
        categories = "accessibility",
        tags = "dark,light,moon,sun,brightness,theme,auto theme,system theme,appearance",
        contributors = "zishankadri,jamiemlaw,jguddas"
    ))]
    SunMoon,
    #[cfg(feature = "sun_snow")]
    #[strum(props(
        svg = "<path d=\"M10 21v-1\"></path><path d=\"M10 4V3\"></path><path d=\"M10 9a3 3 0 0 0 0 6\"></path><path d=\"m14 20 1.25-2.5L18 18\"></path><path d=\"m14 4 1.25 2.5L18 6\"></path><path d=\"m17 21-3-6 1.5-3H22\"></path><path d=\"m17 3-3 6 1.5 3\"></path><path d=\"M2 12h1\"></path><path d=\"m20 10-1.5 2 1.5 2\"></path><path d=\"m3.64 18.36.7-.7\"></path><path d=\"m4.34 6.34-.7-.7\"></path>",
        categories = "weather",
        tags = "weather,air conditioning,temperature,hot,cold,seasons",
        contributors = "karsa-mistmere"
    ))]
    SunSnow,
    #[cfg(feature = "sun")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"12\" r=\"4\"></circle><path d=\"M12 2v2\"></path><path d=\"M12 20v2\"></path><path d=\"m4.93 4.93 1.41 1.41\"></path><path d=\"m17.66 17.66 1.41 1.41\"></path><path d=\"M2 12h2\"></path><path d=\"M20 12h2\"></path><path d=\"m6.34 17.66-1.41 1.41\"></path><path d=\"m19.07 4.93-1.41 1.41\"></path>",
        categories = "accessibility,weather,seasons,sustainability",
        tags = "brightness,weather,light,summer",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Sun,
    #[cfg(feature = "sunrise")]
    #[strum(props(
        svg = "<path d=\"M12 2v8\"></path><path d=\"m4.93 10.93 1.41 1.41\"></path><path d=\"M2 18h2\"></path><path d=\"M20 18h2\"></path><path d=\"m19.07 10.93-1.41 1.41\"></path><path d=\"M22 22H2\"></path><path d=\"m8 6 4-4 4 4\"></path><path d=\"M16 18a4 4 0 0 0-8 0\"></path>",
        categories = "arrows,weather,time",
        tags = "weather,time,morning,day",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunrise,
    #[cfg(feature = "sunset")]
    #[strum(props(
        svg = "<path d=\"M12 10V2\"></path><path d=\"m4.93 10.93 1.41 1.41\"></path><path d=\"M2 18h2\"></path><path d=\"M20 18h2\"></path><path d=\"m19.07 10.93-1.41 1.41\"></path><path d=\"M22 22H2\"></path><path d=\"m16 6-4 4-4-4\"></path><path d=\"M16 18a4 4 0 0 0-8 0\"></path>",
        categories = "arrows,weather",
        tags = "weather,time,evening,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunset,
    #[cfg(feature = "superscript")]
    #[strum(props(
        svg = "<path d=\"m4 19 8-8\"></path><path d=\"m12 19-8-8\"></path><path d=\"M20 12h-4c0-1.5.44-2 1.5-2.5S20 8.33 20 7.00c0-.472-.17-.93-.484-1.29a2.10 2.10 0 0 0-2.61-.436c-.42.23-.738.61-.899 1.06\"></path>",
        categories = "text",
        tags = "text,exponent",
        contributors = "nabanita-sarkar,ericfennis"
    ))]
    Superscript,
    #[cfg(feature = "swatch_book")]
    #[strum(props(
        svg = "<path d=\"M11 17a4 4 0 0 1-8 0V5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2Z\"></path><path d=\"M16.7 13H19a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H7\"></path><path d=\"M 7 17h.01\"></path><path d=\"m11 8 2.3-2.3a2.4 2.4 0 0 1 3.40.004L18.6 7.6a2.4 2.4 0 0 1 .026 3.43L9.9 19.8\"></path>",
        categories = "design,home,photography",
        tags = "colors,colours,swatches,pantone,shades,tint,hue,saturation,brightness,theme,scheme,palette,samples,textile,carpet",
        contributors = "danielbayley"
    ))]
    SwatchBook,
    #[cfg(feature = "swiss_franc")]
    #[strum(props(
        svg = "<path d=\"M10 21V3h8\"></path><path d=\"M6 16h9\"></path><path d=\"M10 9.5h7\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    SwissFranc,
    #[cfg(feature = "switch_camera")]
    #[strum(props(
        svg = "<path d=\"M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5\"></path><path d=\"M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5\"></path><circle cy=\"12\" r=\"3\" cx=\"12\"></circle><path d=\"m18 22-3-3 3-3\"></path><path d=\"m6 2 3 3-3 3\"></path>",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[cfg(feature = "sword")]
    #[strum(props(
        svg = "<path d=\"m11 19-6-6\"></path><path d=\"m5 21-2-2\"></path><path d=\"m8 16-4 4\"></path><path d=\"M9.5 17.5 21 6V3h-3L6.5 14.5\"></path>",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "<polyline points=\"14.5 17.5 3 6 3 3 6 3 17.5 14.5\"></polyline><line y2=\"13\" x1=\"13\" x2=\"19\" y1=\"19\"></line><line y2=\"20\" x1=\"16\" y1=\"16\" x2=\"20\"></line><line x1=\"19\" y2=\"19\" x2=\"21\" y1=\"21\"></line><polyline points=\"14.5 6.5 18 3 21 3 21 6 17.5 9.5\"></polyline><line x1=\"5\" x2=\"9\" y1=\"14\" y2=\"18\"></line><line y2=\"20\" x2=\"4\" x1=\"7\" y1=\"17\"></line><line y2=\"21\" x1=\"3\" x2=\"5\" y1=\"19\"></line>",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Swords,
    #[cfg(feature = "syringe")]
    #[strum(props(
        svg = "<path d=\"m18 2 4 4\"></path><path d=\"m17 7 3-3\"></path><path d=\"M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5\"></path><path d=\"m9 11 4 4\"></path><path d=\"m5 19-3 3\"></path><path d=\"m14 4 6 6\"></path>",
        categories = "science,medical",
        tags = "medicine,medical,needle,pump,plunger,nozzle,blood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Syringe,
    #[cfg(feature = "table_2")]
    #[strum(props(
        svg = "<path d=\"M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18\"></path>",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "abejenaru,karsa-mistmere,ericfennis"
    ))]
    Table2,
    #[cfg(feature = "table_cells_merge")]
    #[strum(props(
        svg = "<path d=\"M12 21v-6\"></path><path d=\"M12 9V3\"></path><path d=\"M3 15h18\"></path><path d=\"M3 9h18\"></path><rect rx=\"2\" width=\"18\" x=\"3\" height=\"18\" y=\"3\"></rect>",
        categories = "text,files",
        tags = "spreadsheet,grid,row",
        contributors = "chessurisme"
    ))]
    TableCellsMerge,
    #[cfg(feature = "table_cells_split")]
    #[strum(props(
        svg = "<path d=\"M12 15V9\"></path><path d=\"M3 15h18\"></path><path d=\"M3 9h18\"></path><rect height=\"18\" x=\"3\" y=\"3\" rx=\"2\" width=\"18\"></rect>",
        categories = "text,files",
        tags = "spreadsheet,grid,row",
        contributors = "chessurisme"
    ))]
    TableCellsSplit,
    #[cfg(feature = "table_columns_split")]
    #[strum(props(
        svg = "<path d=\"M14 14v2\"></path><path d=\"M14 20v2\"></path><path d=\"M14 2v2\"></path><path d=\"M14 8v2\"></path><path d=\"M2 15h8\"></path><path d=\"M2 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2\"></path><path d=\"M2 9h8\"></path><path d=\"M22 15h-4\"></path><path d=\"M22 3h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2\"></path><path d=\"M22 9h-4\"></path><path d=\"M5 3v18\"></path>",
        categories = "text,files",
        tags = "spreadsheet,grid,cut,break,divide,separate,segment",
        contributors = "chessurisme"
    ))]
    TableColumnsSplit,
    #[cfg(feature = "table_of_contents")]
    #[strum(props(
        svg = "<path d=\"M16 5H3\"></path><path d=\"M16 12H3\"></path><path d=\"M16 19H3\"></path><path d=\"M21 5h.01\"></path><path d=\"M21 12h.01\"></path><path d=\"M21 19h.01\"></path>",
        categories = "text",
        tags = "toc,outline,navigation,document structure,index,overview,sections,chapters,content,documentation,manual,knowledge base,faq",
        contributors = "karsa-mistmere"
    ))]
    TableOfContents,
    #[cfg(feature = "table_properties")]
    #[strum(props(
        svg = "<path d=\"M15 3v18\"></path><rect height=\"18\" y=\"3\" rx=\"2\" width=\"18\" x=\"3\"></rect><path d=\"M21 9H3\"></path><path d=\"M21 15H3\"></path>",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table_rows_split")]
    #[strum(props(
        svg = "<path d=\"M14 10h2\"></path><path d=\"M15 22v-8\"></path><path d=\"M15 2v4\"></path><path d=\"M2 10h2\"></path><path d=\"M20 10h2\"></path><path d=\"M3 19h18\"></path><path d=\"M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6\"></path><path d=\"M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2\"></path><path d=\"M8 10h2\"></path><path d=\"M9 22v-8\"></path><path d=\"M9 2v4\"></path>",
        categories = "text,files",
        tags = "spreadsheet,grid,cut,break,divide,separate,segment",
        contributors = "chessurisme"
    ))]
    TableRowsSplit,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "<path d=\"M12 3v18\"></path><rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\"></rect><path d=\"M3 9h18\"></path><path d=\"M3 15h18\"></path>",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "<rect width=\"10\" height=\"14\" x=\"3\" y=\"8\" rx=\"2\"></rect><path d=\"M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4\"></path><path d=\"M8 18h.01\"></path>",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = "danielbayley"
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "<rect y=\"2\" ry=\"2\" rx=\"2\" x=\"4\" width=\"16\" height=\"20\"></rect><line y1=\"18\" x2=\"12.01\" y2=\"18\" x1=\"12\"></line>",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "<circle cx=\"7\" r=\"5\" cy=\"7\"></circle><circle cy=\"17\" r=\"5\" cx=\"17\"></circle><path d=\"M12 17h10\"></path><path d=\"m3.46 10.54 7.08-7.08\"></path>",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,pills,pharmacy",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Tablets,
    #[cfg(feature = "tag")]
    #[strum(props(
        svg = "<path d=\"M12.58 2.58A2 2 0 0 0 11.17 2H4a2 2 0 0 0-2 2v7.17a2 2 0 0 0 .586 1.41l8.70 8.70a2.42 2.42 0 0 0 3.42 0l6.58-6.58a2.42 2.42 0 0 0 0-3.42z\"></path><circle r=\".5\" cy=\"7.5\" cx=\"7.5\" fill=\"currentColor\"></circle>",
        categories = "account",
        tags = "label,badge,ticket,mark",
        contributors = "colebemis,csandman,aaofyi,ericfennis,karsa-mistmere"
    ))]
    Tag,
    #[cfg(feature = "tags")]
    #[strum(props(
        svg = "<path d=\"M13.17 2a2 2 0 0 1 1.41.586l6.71 6.71a2.4 2.4 0 0 1 0 3.40l-4.59 4.59a2.4 2.4 0 0 1-3.40 0l-6.71-6.71A2 2 0 0 1 6 9.17V3a1 1 0 0 1 1-1z\"></path><path d=\"M2 7v6.17a2 2 0 0 0 .586 1.41l6.71 6.71a2.4 2.4 0 0 0 3.19.193\"></path><circle fill=\"currentColor\" cx=\"10.5\" r=\".5\" cy=\"6.5\"></circle>",
        categories = "account",
        tags = "labels,badges,tickets,marks,copy,multiple",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Tags,
    #[cfg(feature = "tally_1")]
    #[strum(props(
        svg = "<path d=\"M4 4v16\"></path>",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,one,1,first,bar,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally1,
    #[cfg(feature = "tally_2")]
    #[strum(props(
        svg = "<path d=\"M4 4v16\"></path><path d=\"M9 4v16\"></path>",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally2,
    #[cfg(feature = "tally_3")]
    #[strum(props(
        svg = "<path d=\"M4 4v16\"></path><path d=\"M9 4v16\"></path><path d=\"M14 4v16\"></path>",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally3,
    #[cfg(feature = "tally_4")]
    #[strum(props(
        svg = "<path d=\"M4 4v16\"></path><path d=\"M9 4v16\"></path><path d=\"M14 4v16\"></path><path d=\"M19 4v16\"></path>",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = "danielbayley"
    ))]
    Tally4,
    #[cfg(feature = "tally_5")]
    #[strum(props(
        svg = "<path d=\"M4 4v16\"></path><path d=\"M9 4v16\"></path><path d=\"M14 4v16\"></path><path d=\"M19 4v16\"></path><path d=\"M22 6 2 18\"></path>",
        categories = "math,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = "danielbayley"
    ))]
    Tally5,
    #[cfg(feature = "tangent")]
    #[strum(props(
        svg = "<circle cy=\"4\" r=\"2\" cx=\"17\"></circle><path d=\"M15.59 5.41 5.41 15.59\"></path><circle cy=\"17\" r=\"2\" cx=\"4\"></circle><path d=\"M12 22s-4-9-1.5-11.5S22 12 22 12\"></path>",
        categories = "shapes,math,design,tools",
        tags = "tangential,shape,circle,geometry,trigonometry,bezier curve",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Tangent,
    #[cfg(feature = "target")]
    #[strum(props(
        svg = "<circle cy=\"12\" r=\"10\" cx=\"12\"></circle><circle cx=\"12\" r=\"6\" cy=\"12\"></circle><circle cx=\"12\" cy=\"12\" r=\"2\"></circle>",
        categories = "gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[cfg(feature = "telescope")]
    #[strum(props(
        svg = "<path d=\"m10.06 12.49-6.18 1.31a.934.93 0 0 1-1.10-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.26l13.50-4.44\"></path><path d=\"m13.56 11.74 4.33-.924\"></path><path d=\"m16 21-3.10-6.21\"></path><path d=\"M16.48 5.94a2 2 0 0 1 1.45-2.42l1.09-.272a1 1 0 0 1 1.21.727l1.51 6.06a1 1 0 0 1-.727 1.21l-1.09.27a2 2 0 0 1-2.42-1.45z\"></path><path d=\"m6.15 8.63 1.11 4.45\"></path><path d=\"m8 21 3.10-6.21\"></path><circle cx=\"12\" cy=\"13\" r=\"2\"></circle>",
        categories = "science,development,tools",
        tags = "astronomy,space,discovery,exploration,explore,vision,perspective,focus,stargazing,observe,view",
        contributors = "karsa-mistmere"
    ))]
    Telescope,
    #[cfg(feature = "tent_tree")]
    #[strum(props(
        svg = "<circle r=\"2\" cx=\"4\" cy=\"4\"></circle><path d=\"m14 5 3-3 3 3\"></path><path d=\"m14 10 3-3 3 3\"></path><path d=\"M17 14V2\"></path><path d=\"M17 14H7l-5 8h20Z\"></path><path d=\"M8 14v8\"></path><path d=\"m9 14 5 8\"></path>",
        categories = "travel,nature",
        tags = "camping,campsite,holiday,retreat,nomadic,wilderness,outdoors",
        contributors = "danielbayley"
    ))]
    TentTree,
    #[cfg(feature = "tent")]
    #[strum(props(
        svg = "<path d=\"M3.5 21 14 3\"></path><path d=\"M20.5 21 10 3\"></path><path d=\"M15.5 21 12 15l-3.5 6\"></path><path d=\"M2 21h20\"></path>",
        categories = "travel,nature,sustainability",
        tags = "tipi,teepee,wigwam,lodge,camping,campsite,holiday,retreat,nomadic,native american,indian,wilderness,outdoors",
        contributors = "MoltenCoffee,ericfennis,danielbayley"
    ))]
    Tent,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "<path d=\"M12 19h8\"></path><path d=\"m4 17 6-6-6-6\"></path>",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "colebemis,ericfennis"
    ))]
    Terminal,
    #[cfg(feature = "test_tube_diagonal")]
    #[strum(props(
        svg = "<path d=\"M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01a2.83 2.83 0 0 1 0-4L17 3\"></path><path d=\"m16 2 6 6\"></path><path d=\"M12 16H4\"></path>",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TestTubeDiagonal,
    #[cfg(feature = "test_tube")]
    #[strum(props(
        svg = "<path d=\"M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5c-1.4 0-2.5-1.1-2.5-2.5V2\"></path><path d=\"M8.5 2h7\"></path><path d=\"M14.5 16h-5\"></path>",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube,
    #[cfg(feature = "test_tubes")]
    #[strum(props(
        svg = "<path d=\"M9 2v17.5A2.5 2.5 0 0 1 6.5 22A2.5 2.5 0 0 1 4 19.5V2\"></path><path d=\"M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5a2.5 2.5 0 0 1-2.5-2.5V2\"></path><path d=\"M3 2h7\"></path><path d=\"M14 2h7\"></path><path d=\"M9 16H4\"></path><path d=\"M20 16h-5\"></path>",
        categories = "science",
        tags = "tubes,vials,phials,flasks,ampoules,ampules,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTubes,
    #[cfg(feature = "text_align_center")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M17 12H7\"></path><path d=\"M19 19H5\"></path>",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignCenter,
    #[cfg(feature = "text_align_end")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M21 12H9\"></path><path d=\"M21 19H7\"></path>",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignEnd,
    #[cfg(feature = "text_align_justify")]
    #[strum(props(
        svg = "<path d=\"M3 5h18\"></path><path d=\"M3 12h18\"></path><path d=\"M3 19h18\"></path>",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignJustify,
    #[cfg(feature = "text_align_start")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M15 12H3\"></path><path d=\"M17 19H3\"></path>",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    TextAlignStart,
    #[cfg(feature = "text_cursor_input")]
    #[strum(props(
        svg = "<path d=\"M12 20h-1a2 2 0 0 1-2-2 2 2 0 0 1-2 2H6\"></path><path d=\"M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7\"></path><path d=\"M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1\"></path><path d=\"M6 4h1a2 2 0 0 1 2 2 2 2 0 0 1 2-2h1\"></path><path d=\"M9 6v12\"></path>",
        categories = "text,layout",
        tags = "select",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    TextCursorInput,
    #[cfg(feature = "text_cursor")]
    #[strum(props(
        svg = "<path d=\"M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1\"></path><path d=\"M7 22h1a4 4 0 0 0 4-4v-1\"></path><path d=\"M7 2h1a4 4 0 0 1 4 4v1\"></path>",
        categories = "text,cursors",
        tags = "select",
        contributors = "ericfennis"
    ))]
    TextCursor,
    #[cfg(feature = "text_initial")]
    #[strum(props(
        svg = "<path d=\"M15 5h6\"></path><path d=\"M15 12h6\"></path><path d=\"M3 19h18\"></path><path d=\"m3 12 3.55-7.72a.5.5 0 0 1 .894 0L11 12\"></path><path d=\"M3.92 10h6.16\"></path>",
        categories = "text",
        tags = "drop cap,text,format,typography,letter,font size",
        contributors = "jguddas,GRA0007,karsa-mistmere"
    ))]
    TextInitial,
    #[cfg(feature = "text_quote")]
    #[strum(props(
        svg = "<path d=\"M17 5H3\"></path><path d=\"M21 12H8\"></path><path d=\"M21 19H8\"></path><path d=\"M3 12v7\"></path>",
        categories = "text",
        tags = "blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextQuote,
    #[cfg(feature = "text_search")]
    #[strum(props(
        svg = "<path d=\"M21 5H3\"></path><path d=\"M10 12H3\"></path><path d=\"M10 19H3\"></path><circle cy=\"15\" cx=\"17\" r=\"3\"></circle><path d=\"m21 19-1.9-1.9\"></path>",
        categories = "text",
        tags = "find,data,copy,txt,pdf,document,scan,magnifier,magnifying glass,lens",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextSearch,
    #[cfg(feature = "text_wrap")]
    #[strum(props(
        svg = "<path d=\"m16 16-3 3 3 3\"></path><path d=\"M3 12h14.5a1 1 0 0 1 0 7H13\"></path><path d=\"M3 19h6\"></path><path d=\"M3 5h18\"></path>",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis,karsa-mistmere"
    ))]
    TextWrap,
    #[cfg(feature = "theater")]
    #[strum(props(
        svg = "<path d=\"M2 10s3-3 3-8\"></path><path d=\"M22 10s-3-3-3-8\"></path><path d=\"M10 2c0 4.4-3.6 8-8 8\"></path><path d=\"M14 2c0 4.4 3.6 8 8 8\"></path><path d=\"M2 10s2 2 2 5\"></path><path d=\"M22 10s-2 2-2 5\"></path><path d=\"M8 15h8\"></path><path d=\"M2 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1\"></path><path d=\"M14 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1\"></path>",
        categories = "buildings,social",
        tags = "theater,theatre,entertainment,podium,stage,musical",
        contributors = "danielbayley"
    ))]
    Theater,
    #[cfg(feature = "thermometer_snowflake")]
    #[strum(props(
        svg = "<path d=\"m10 20-1.25-2.5L6 18\"></path><path d=\"M10 4 8.75 6.5 6 6\"></path><path d=\"M10.58 15H10\"></path><path d=\"M2 12h6.5L10 9\"></path><path d=\"M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z\"></path><path d=\"m4 10 1.5 2L4 14\"></path><path d=\"m7 21 3-6-1.5-3\"></path><path d=\"m7 3 3 6h2\"></path>",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,cold,freeze,freezing",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ThermometerSnowflake,
    #[cfg(feature = "thermometer_sun")]
    #[strum(props(
        svg = "<path d=\"M12 2v2\"></path><path d=\"M12 8a4 4 0 0 0-1.64 7.64\"></path><path d=\"M2 12h2\"></path><path d=\"M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z\"></path><path d=\"m4.93 4.93 1.41 1.41\"></path><path d=\"m6.34 17.66-1.41 1.41\"></path>",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,warm,hot",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    ThermometerSun,
    #[cfg(feature = "thermometer")]
    #[strum(props(
        svg = "<path d=\"M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z\"></path>",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Thermometer,
    #[cfg(feature = "thumbs_down")]
    #[strum(props(
        svg = "<path d=\"M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z\"></path><path d=\"M17 14V2\"></path>",
        categories = "account,social,emoji",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[cfg(feature = "thumbs_up")]
    #[strum(props(
        svg = "<path d=\"M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z\"></path><path d=\"M7 10v12\"></path>",
        categories = "account,social,emoji",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[cfg(feature = "ticket_check")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"m9 12 2 2 4-4\"></path>",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,booked,purchased,receipt,redeemed,validated,verified,certified,checked,used",
        contributors = "danielbayley"
    ))]
    TicketCheck,
    #[cfg(feature = "ticket_minus")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"M9 12h6\"></path>",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,remove,cancel,unbook,subtract,decrease,-",
        contributors = "danielbayley"
    ))]
    TicketMinus,
    #[cfg(feature = "ticket_percent")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 1 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 1 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"M9 9h.01\"></path><path d=\"m15 9-6 6\"></path><path d=\"M15 15h.01\"></path>",
        categories = "transportation,shopping",
        tags = "discount,reduced,offer,voucher,entry,pass,event,concert,show,book,purchase,%",
        contributors = "danielbayley"
    ))]
    TicketPercent,
    #[cfg(feature = "ticket_plus")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"M9 12h6\"></path><path d=\"M12 9v6\"></path>",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,book,purchase,add,+",
        contributors = "danielbayley"
    ))]
    TicketPlus,
    #[cfg(feature = "ticket_slash")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"m9.5 14.5 5-5\"></path>",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,redeemed,used,marked,checked,verified,spoiled,invalidated,void,denied,refused,banned,barred,forbidden,prohibited,cancelled,cancellation,refunded,delete,remove,clear,error",
        contributors = "danielbayley"
    ))]
    TicketSlash,
    #[cfg(feature = "ticket_x")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"m9.5 14.5 5-5\"></path><path d=\"m9.5 9.5 5 5\"></path>",
        categories = "transportation",
        tags = "entry,pass,voucher,event,concert,show,cancelled,cancellation,refunded,used,void,invalidated,spoiled,denied,refused,banned,barred,forbidden,prohibited,delete,remove,clear,error,x",
        contributors = "danielbayley"
    ))]
    TicketX,
    #[cfg(feature = "ticket")]
    #[strum(props(
        svg = "<path d=\"M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z\"></path><path d=\"M13 5v2\"></path><path d=\"M13 17v2\"></path><path d=\"M13 11v2\"></path>",
        categories = "account,transportation",
        tags = "entry,pass,voucher,event,concert,show,perforated,dashed",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Ticket,
    #[cfg(feature = "tickets_plane")]
    #[strum(props(
        svg = "<path d=\"M10.5 17h1.22a2 2 0 0 0 1.34-.52L18 12\"></path><path d=\"m12 13.5 3.79.506\"></path><path d=\"m3.17 8.18 11-5a2 2 0 0 1 2.64.993L18.56 8\"></path><path d=\"M6 10V8\"></path><path d=\"M6 14v1\"></path><path d=\"M6 19v2\"></path><rect y=\"8\" height=\"13\" rx=\"2\" x=\"2\" width=\"20\"></rect>",
        categories = "transportation,travel",
        tags = "plane,trip,airplane,flight,travel,fly,takeoff,vacation,passenger,pass,check-in,airport",
        contributors = "jguddas,karsa-mistmere"
    ))]
    TicketsPlane,
    #[cfg(feature = "tickets")]
    #[strum(props(
        svg = "<path d=\"m3.17 8.18 11-5a2 2 0 0 1 2.64.993L18.56 8\"></path><path d=\"M6 10V8\"></path><path d=\"M6 14v1\"></path><path d=\"M6 19v2\"></path><rect height=\"13\" x=\"2\" rx=\"2\" y=\"8\" width=\"20\"></rect>",
        categories = "travel,account,transportation",
        tags = "trip,travel,pass,entry,voucher,event,concert,show,perforated,dashed",
        contributors = "jguddas,karsa-mistmere"
    ))]
    Tickets,
    #[cfg(feature = "timer_off")]
    #[strum(props(
        svg = "<path d=\"M10 2h4\"></path><path d=\"M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7\"></path><path d=\"M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2\"></path><path d=\"m2 2 20 20\"></path><path d=\"M12 12v-2\"></path>",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    TimerOff,
    #[cfg(feature = "timer_reset")]
    #[strum(props(
        svg = "<path d=\"M10 2h4\"></path><path d=\"M12 14v-4\"></path><path d=\"M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6\"></path><path d=\"M9 17H4v5\"></path>",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis"
    ))]
    TimerReset,
    #[cfg(feature = "timer")]
    #[strum(props(
        svg = "<line x1=\"10\" x2=\"14\" y1=\"2\" y2=\"2\"></line><line y1=\"14\" y2=\"11\" x2=\"15\" x1=\"12\"></line><circle cy=\"14\" r=\"8\" cx=\"12\"></circle>",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"9\" cy=\"12\"></circle><rect rx=\"7\" width=\"20\" height=\"14\" x=\"2\" y=\"5\"></rect>",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"15\" cy=\"12\"></circle><rect x=\"2\" y=\"5\" rx=\"7\" width=\"20\" height=\"14\"></rect>",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis,jamiemlaw"
    ))]
    ToggleRight,
    #[cfg(feature = "toilet")]
    #[strum(props(
        svg = "<path d=\"M7 12h13a1 1 0 0 1 1 1 5 5 0 0 1-5 5h-.598a.5.5 0 0 0-.424.76l1.54 2.47a.5.5 0 0 1-.424.76H5.40a.5.5 0 0 1-.424-.765L7 18\"></path><path d=\"M8 18a5 5 0 0 1-5-5V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8\"></path>",
        categories = "devices,home",
        tags = "toilet,potty,bathroom,washroom",
        contributors = "EthanHazel,staffanmowitz,karsa-mistmere,jguddas"
    ))]
    Toilet,
    #[cfg(feature = "tool_case")]
    #[strum(props(
        svg = "<path d=\"M10 15h4\"></path><path d=\"m14.81 10.99-.971-1.45 1.03-1.23a2 2 0 0 0-2.02-3.23l-1.82.36L9.91 3.88a2 2 0 0 0-3.62.748L6.14 6.55l-1.72.426a2 2 0 0 0-.19 3.75l.657.27\"></path><path d=\"m18.82 10.99 2.26-5.38a1 1 0 0 0-.557-1.31L16.95 2.9a1 1 0 0 0-1.28.533l-.924 2.12\"></path><path d=\"M4 12.00A1 1 0 0 1 4.99 11H19a1 1 0 0 1 1 1v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z\"></path>",
        categories = "tools,development,home",
        tags = "tools,maintenance,repair",
        contributors = "AlexNaskida,karsa-mistmere,jguddas"
    ))]
    ToolCase,
    #[cfg(feature = "toolbox")]
    #[strum(props(
        svg = "<path d=\"M16 12v4\"></path><path d=\"M16 6a2 2 0 0 1 1.41.586l4 4A2 2 0 0 1 22 12v7a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 .586-1.41l4-4A2 2 0 0 1 8 6z\"></path><path d=\"M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2\"></path><path d=\"M2 14h20\"></path><path d=\"M8 12v4\"></path>",
        categories = "tools,home",
        tags = "toolkit,tools,trunk,chest,box,storage,utility,utilities,container,kit,set,repair,fix,service,maintenance,mechanic,workshop,construction,hardware,equipment,gear,handyman,engineering,craft,diy",
        contributors = "karsa-mistmere"
    ))]
    Toolbox,
    #[cfg(feature = "tornado")]
    #[strum(props(
        svg = "<path d=\"M21 4H3\"></path><path d=\"M18 8H6\"></path><path d=\"M19 12H9\"></path><path d=\"M16 16h-6\"></path><path d=\"M11 20H9\"></path>",
        categories = "weather",
        tags = "weather,wind,storm,hurricane",
        contributors = "ericfennis"
    ))]
    Tornado,
    #[cfg(feature = "torus")]
    #[strum(props(
        svg = "<ellipse rx=\"3\" ry=\"2\" cy=\"11\" cx=\"12\"></ellipse><ellipse cx=\"12\" ry=\"8.5\" cy=\"12.5\" rx=\"10\"></ellipse>",
        categories = "shapes,design,tools,food-beverage",
        tags = "donut,doughnut,ring,hollow,3d,fast food,junk food,snack,treat,sweet,sugar,dessert",
        contributors = "danielbayley,jguddas"
    ))]
    Torus,
    #[cfg(feature = "touchpad_off")]
    #[strum(props(
        svg = "<path d=\"M12 20v-6\"></path><path d=\"M19.65 14H22\"></path><path d=\"M2 14h12\"></path><path d=\"m2 2 20 20\"></path><path d=\"M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2\"></path><path d=\"M9.65 4H20a2 2 0 0 1 2 2v10.34\"></path>",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    TouchpadOff,
    #[cfg(feature = "touchpad")]
    #[strum(props(
        svg = "<rect width=\"20\" rx=\"2\" x=\"2\" height=\"16\" y=\"4\"></rect><path d=\"M2 14h20\"></path><path d=\"M12 20v-6\"></path>",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Touchpad,
    #[cfg(feature = "towel_rack")]
    #[strum(props(
        svg = "<path d=\"M22 7h-2\"></path><path d=\"M6.5 3h11A2.5 2.5 0 0 1 20 5.5V20a1 1 0 0 1-1 1h-9a1 1 0 0 1-1-1V5.5a1 1 0 0 0-5 0V17a1 1 0 0 0 1 1h4\"></path><path d=\"M9 7H2\"></path>",
        categories = "home,travel",
        tags = "flannel,bathroom,toiletries,sanitation,clean,fresh,dry,laundry,laundrette,hospitality,housekeeping,room service,spa break,health club,amenities,hanging",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    TowelRack,
    #[cfg(feature = "tower_control")]
    #[strum(props(
        svg = "<path d=\"M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z\"></path><path d=\"M8 13v9\"></path><path d=\"M16 22v-9\"></path><path d=\"m9 6 1 7\"></path><path d=\"m15 6-1 7\"></path><path d=\"M12 6V2\"></path><path d=\"M13 2h-2\"></path>",
        categories = "travel,transportation",
        tags = "airport,travel,tower,transportation,lighthouse",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    TowerControl,
    #[cfg(feature = "toy_brick")]
    #[strum(props(
        svg = "<rect width=\"18\" height=\"12\" y=\"8\" rx=\"1\" x=\"3\"></rect><path d=\"M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3\"></path><path d=\"M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3\"></path>",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "<path d=\"m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.15a1 1 0 0 1-.988.84H20\"></path><path d=\"M16 18h-5\"></path><path d=\"M18 5a1 1 0 0 0-1 1v5.57\"></path><path d=\"M3 4h8.12a1 1 0 0 1 .99.86L13 11.24\"></path><path d=\"M4 11V4\"></path><path d=\"M7 15h.01\"></path><path d=\"M8 10.1V4\"></path><circle cy=\"18\" cx=\"18\" r=\"2\"></circle><circle cy=\"15\" cx=\"7\" r=\"5\"></circle>",
        categories = "transportation,sustainability,food-beverage",
        tags = "farming,farmer,ranch,harvest,equipment,vehicle",
        contributors = "danielbayley,jguddas"
    ))]
    Tractor,
    #[cfg(feature = "traffic_cone")]
    #[strum(props(
        svg = "<path d=\"M16.05 10.96a5 2.5 0 0 1-8.1 0\"></path><path d=\"m16.92 14.04 4.48 2.04a1 1 0 0 1 .001 1.83l-8.57 3.9a2 2 0 0 1-1.66 0l-8.57-3.91a1 1 0 0 1 0-1.83l4.48-2.04\"></path><path d=\"M16.94 14.14a5 2.5 0 1 1-9.9 0L10.06 3.5a2 2 0 0 1 3.87 0z\"></path><path d=\"M9.19 6.57a5 2.5 0 0 0 5.61 0\"></path>",
        categories = "transportation",
        tags = "roadworks,tarmac,safety,block",
        contributors = "danielbayley,karsa-mistmere,jamiemlaw"
    ))]
    TrafficCone,
    #[cfg(feature = "train_front_tunnel")]
    #[strum(props(
        svg = "<path d=\"M2 22V12a10 10 0 1 1 20 0v10\"></path><path d=\"M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8\"></path><path d=\"M10 15h.01\"></path><path d=\"M14 15h.01\"></path><path d=\"M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z\"></path><path d=\"m9 19-2 3\"></path><path d=\"m15 19 2 3\"></path>",
        categories = "transportation,navigation",
        tags = "railway,metro,subway,underground,speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFrontTunnel,
    #[cfg(feature = "train_front")]
    #[strum(props(
        svg = "<path d=\"M8 3.1V7a4 4 0 0 0 8 0V3.1\"></path><path d=\"m9 15-1-1\"></path><path d=\"m15 15 1-1\"></path><path d=\"M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z\"></path><path d=\"m8 19-2 3\"></path><path d=\"m16 19 2 3\"></path>",
        categories = "transportation",
        tags = "railway,metro,subway,underground,high-speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFront,
    #[cfg(feature = "train_track")]
    #[strum(props(
        svg = "<path d=\"M2 17 17 2\"></path><path d=\"m2 14 8 8\"></path><path d=\"m5 11 8 8\"></path><path d=\"m8 8 8 8\"></path><path d=\"m11 5 8 8\"></path><path d=\"m14 2 8 8\"></path><path d=\"M7 22 22 7\"></path>",
        categories = "transportation,navigation",
        tags = "railway,line",
        contributors = "danielbayley"
    ))]
    TrainTrack,
    #[cfg(feature = "tram_front")]
    #[strum(props(
        svg = "<rect y=\"3\" rx=\"2\" width=\"16\" height=\"16\" x=\"4\"></rect><path d=\"M4 11h16\"></path><path d=\"M12 3v8\"></path><path d=\"m8 19-2 3\"></path><path d=\"m18 22-2-3\"></path><path d=\"M8 15h.01\"></path><path d=\"M16 15h.01\"></path>",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "transgender")]
    #[strum(props(
        svg = "<path d=\"M12 16v6\"></path><path d=\"M14 20h-4\"></path><path d=\"M18 2h4v4\"></path><path d=\"m2 2 7.17 7.17\"></path><path d=\"M2 5.35V2h3.35\"></path><path d=\"m22 2-7.17 7.17\"></path><path d=\"M8 5 5 8\"></path><circle cy=\"12\" r=\"4\" cx=\"12\"></circle>",
        categories = "medical,accessibility",
        tags = "gender,inclusive",
        contributors = "jamiemlaw"
    ))]
    Transgender,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "<path d=\"M10 11v6\"></path><path d=\"M14 11v6\"></path><path d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6\"></path><path d=\"M3 6h18\"></path><path d=\"M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\"></path>",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash2,
    #[cfg(feature = "trash")]
    #[strum(props(
        svg = "<path d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6\"></path><path d=\"M3 6h18\"></path><path d=\"M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\"></path>",
        categories = "files,mail",
        tags = "empty,deletion,cleanup,junk,clear,garbage,delete,remove,bin,waste,recycle,discard,binoculars,rubbish",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash,
    #[cfg(feature = "tree_deciduous")]
    #[strum(props(
        svg = "<path d=\"M8 19a4 4 0 0 1-2.24-7.32A3.5 3.5 0 0 1 9 6.03V6a3 3 0 1 1 6 0v.04a3.5 3.5 0 0 1 3.24 5.65A4 4 0 0 1 16 19Z\"></path><path d=\"M12 19v3\"></path>",
        categories = "nature,sustainability",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TreeDeciduous,
    #[cfg(feature = "tree_palm")]
    #[strum(props(
        svg = "<path d=\"M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4\"></path><path d=\"M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3\"></path><path d=\"M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35\"></path><path d=\"M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14\"></path>",
        categories = "nature,sustainability",
        tags = "vacation,leisure,island",
        contributors = "ericfennis"
    ))]
    TreePalm,
    #[cfg(feature = "tree_pine")]
    #[strum(props(
        svg = "<path d=\"m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z\"></path><path d=\"M12 22v-3\"></path>",
        categories = "nature,sustainability",
        tags = "tree,pine,forest,park,nature",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    TreePine,
    #[cfg(feature = "trees")]
    #[strum(props(
        svg = "<path d=\"M10 10v.2A3 3 0 0 1 8.9 16H5a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z\"></path><path d=\"M7 16v6\"></path><path d=\"M13 19v3\"></path><path d=\"M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5\"></path>",
        categories = "nature,sustainability",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere"
    ))]
    Trees,
    #[cfg(feature = "trending_down")]
    #[strum(props(
        svg = "<path d=\"M16 17h6v-6\"></path><path d=\"m22 17-8.5-8.5-5 5L2 7\"></path>",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingDown,
    #[cfg(feature = "trending_up_down")]
    #[strum(props(
        svg = "<path d=\"M14.82 14.82 21 21\"></path><path d=\"M21 16v5h-5\"></path><path d=\"m21 3-9 9-4-4-6 6\"></path><path d=\"M21 8V3h-5\"></path>",
        categories = "charts,arrows",
        tags = "arrows,estimated,indeterminate,data fluctuation,uncertain,forecast,variable,prediction,dynamic,volatile",
        contributors = "Alportan"
    ))]
    TrendingUpDown,
    #[cfg(feature = "trending_up")]
    #[strum(props(
        svg = "<path d=\"M16 7h6v6\"></path><path d=\"m22 7-8.5 8.5-5-5L2 17\"></path>",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingUp,
    #[cfg(feature = "triangle_alert")]
    #[strum(props(
        svg = "<path d=\"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3\"></path><path d=\"M12 9v4\"></path><path d=\"M12 17h.01\"></path>",
        categories = "notifications,shapes,development",
        tags = "warning,alert,danger,exclamation mark,linter",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    TriangleAlert,
    #[cfg(feature = "triangle_dashed")]
    #[strum(props(
        svg = "<path d=\"M10.17 4.19a2 2 0 0 1 3.66.013\"></path><path d=\"M14 21h2\"></path><path d=\"m15.87 7.74 1 1.73\"></path><path d=\"m18.84 12.95 1 1.73\"></path><path d=\"M21.82 18.18a2 2 0 0 1-1.83 2.82\"></path><path d=\"M4.02 21a2 2 0 0 1-1.83-2.83\"></path><path d=\"m5.13 12.95-1 1.73\"></path><path d=\"M8 21h2\"></path><path d=\"m8.10 7.74-1 1.73\"></path>",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,Yohh"
    ))]
    TriangleDashed,
    #[cfg(feature = "triangle_right")]
    #[strum(props(
        svg = "<path d=\"M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z\"></path>",
        categories = "shapes,math",
        tags = "volume,controls,controller,tv remote,geometry,delta,ramp,slope,incline,increase",
        contributors = "danielbayley"
    ))]
    TriangleRight,
    #[cfg(feature = "triangle")]
    #[strum(props(
        svg = "<path d=\"M13.73 4a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z\"></path>",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Triangle,
    #[cfg(feature = "trophy")]
    #[strum(props(
        svg = "<path d=\"M10 14.66v1.62a2 2 0 0 1-.976 1.69A5 5 0 0 0 7 21.97\"></path><path d=\"M14 14.66v1.62a2 2 0 0 0 .976 1.69A5 5 0 0 1 17 21.97\"></path><path d=\"M18 9h1.5a1 1 0 0 0 0-5H18\"></path><path d=\"M4 22h16\"></path><path d=\"M6 9a6 6 0 0 0 12 0V3a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1z\"></path><path d=\"M6 9H4.5a1 1 0 0 1 0-5H6\"></path>",
        categories = "sports,gaming",
        tags = "prize,sports,winner,achievement,award,champion,celebration,victory",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Trophy,
    #[cfg(feature = "truck_electric")]
    #[strum(props(
        svg = "<path d=\"M14 19V7a2 2 0 0 0-2-2H9\"></path><path d=\"M15 19H9\"></path><path d=\"M19 19h2a1 1 0 0 0 1-1v-3.65a1 1 0 0 0-.22-.62L18.3 9.38a1 1 0 0 0-.78-.38H14\"></path><path d=\"M2 13v5a1 1 0 0 0 1 1h2\"></path><path d=\"M4 3 2.15 5.15a.495.49 0 0 0 .35.86h2.15a.47.47 0 0 1 .35.86L3 9.02\"></path><circle cx=\"17\" r=\"2\" cy=\"19\"></circle><circle r=\"2\" cx=\"7\" cy=\"19\"></circle>",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry,electric",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley,jordan808,LienMaas,jguddas,AnnaSasDev"
    ))]
    TruckElectric,
    #[cfg(feature = "truck")]
    #[strum(props(
        svg = "<path d=\"M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2\"></path><path d=\"M15 18H9\"></path><path d=\"M19 18h2a1 1 0 0 0 1-1v-3.65a1 1 0 0 0-.22-.624l-3.48-4.35A1 1 0 0 0 17.52 8H14\"></path><circle r=\"2\" cy=\"18\" cx=\"17\"></circle><circle cx=\"7\" r=\"2\" cy=\"18\"></circle>",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley,jordan808"
    ))]
    Truck,
    #[cfg(feature = "turkish_lira")]
    #[strum(props(
        svg = "<path d=\"M15 4 5 9\"></path><path d=\"m15 8.5-10 5\"></path><path d=\"M18 12a9 9 0 0 1-9 9V3\"></path>",
        categories = "finance",
        tags = "currency,money,payment",
        contributors = "jamiemlaw"
    ))]
    TurkishLira,
    #[cfg(feature = "turntable")]
    #[strum(props(
        svg = "<path d=\"M10 12.01h.01\"></path><path d=\"M18 8v4a8 8 0 0 1-1.07 4\"></path><circle r=\"4\" cx=\"10\" cy=\"12\"></circle><rect y=\"4\" rx=\"2\" x=\"2\" width=\"20\" height=\"16\"></rect>",
        categories = "multimedia,home",
        tags = "record player,gramophone,stereo,phonograph,vinyl,lp,disc,platter,cut,music,analog,retro,dj deck,disc jockey,scratch,spinning",
        contributors = "karsa-mistmere"
    ))]
    Turntable,
    #[cfg(feature = "turtle")]
    #[strum(props(
        svg = "<path d=\"m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z\"></path><path d=\"M4.82 7.9 8 10\"></path><path d=\"M15.18 7.9 12 10\"></path><path d=\"M16.93 10H20a2 2 0 0 1 0 4H2\"></path>",
        categories = "animals",
        tags = "animal,pet,tortoise,slow,speed",
        contributors = "danielbayley"
    ))]
    Turtle,
    #[cfg(feature = "tv_minimal_play")]
    #[strum(props(
        svg = "<path d=\"M15.03 9.44a.647.64 0 0 1 0 1.12l-4.06 2.35a.645.64 0 0 1-.968-.56V7.64a.645.64 0 0 1 .967-.56z\"></path><path d=\"M7 21h10\"></path><rect y=\"3\" width=\"20\" x=\"2\" height=\"14\" rx=\"2\"></rect>",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,movie,live,ott,running,start,film,home cinema,entertainment,showtime,channels,catchup",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    TvMinimalPlay,
    #[cfg(feature = "tv_minimal")]
    #[strum(props(
        svg = "<path d=\"M7 21h10\"></path><rect rx=\"2\" x=\"2\" y=\"3\" width=\"20\" height=\"14\"></rect>",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    TvMinimal,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "<path d=\"m17 2-5 5-5-5\"></path><rect y=\"7\" rx=\"2\" width=\"20\" height=\"15\" x=\"2\"></rect>",
        categories = "devices,multimedia,communication",
        tags = "television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,entertainment,showtime,channels,terrestrial,satellite,cable,broadcast,live,frequency,tune,scan,aerial,receiver,transmission,signal,connection,connectivity",
        contributors = "colebemis,ericfennis"
    ))]
    Tv,
    #[cfg(feature = "type_outline")]
    #[strum(props(
        svg = "<path d=\"M14 16.5a.5.5 0 0 0 .5.5h.5a2 2 0 0 1 0 4H9a2 2 0 0 1 0-4h.5a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5V8a2 2 0 0 1-4 0V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v3a2 2 0 0 1-4 0v-.5a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5Z\"></path>",
        categories = "text",
        tags = "text,font,typography,silhouette,profile,contour,stroke,line",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    TypeOutline,
    #[cfg(feature = "type")]
    #[strum(props(
        svg = "<path d=\"M12 4v16\"></path><path d=\"M4 7V5a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2\"></path><path d=\"M9 20h6\"></path>",
        categories = "text",
        tags = "text,font,typography",
        contributors = "colebemis,ericfennis"
    ))]
    Type,
    #[cfg(feature = "umbrella_off")]
    #[strum(props(
        svg = "<path d=\"M12 13v7a2 2 0 0 0 4 0\"></path><path d=\"M12 2v2\"></path><path d=\"M18.65 13h2.33a1 1 0 0 0 .97-1.27 10.28 10.28 0 0 0-12.07-7.51\"></path><path d=\"m2 2 20 20\"></path><path d=\"M5.96 5.95a10.28 10.28 0 0 0-3.92 5.76A1 1 0 0 0 3 13h10\"></path>",
        categories = "weather",
        tags = "rain,weather,uncovered,uninsured,antivirus,unprotected,risky",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    UmbrellaOff,
    #[cfg(feature = "umbrella")]
    #[strum(props(
        svg = "<path d=\"M12 13v7a2 2 0 0 0 4 0\"></path><path d=\"M12 2v2\"></path><path d=\"M20.99 13a1 1 0 0 0 .97-1.27 10.28 10.28 0 0 0-19.92 0A1 1 0 0 0 3 13z\"></path>",
        categories = "weather",
        tags = "rain,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Umbrella,
    #[cfg(feature = "underline")]
    #[strum(props(
        svg = "<path d=\"M6 4v6a6 6 0 0 0 12 0V4\"></path><line x1=\"4\" y2=\"20\" x2=\"20\" y1=\"20\"></line>",
        categories = "text",
        tags = "text,format",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Underline,
    #[cfg(feature = "undo_2")]
    #[strum(props(
        svg = "<path d=\"M9 14 4 9l5-5\"></path><path d=\"M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5a5.5 5.5 0 0 1-5.5 5.5H11\"></path>",
        categories = "text,arrows",
        tags = "redo,rerun,history,back,return,reverse,revert,direction,u-turn",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Undo2,
    #[cfg(feature = "undo_dot")]
    #[strum(props(
        svg = "<path d=\"M21 17a9 9 0 0 0-15-6.7L3 13\"></path><path d=\"M3 7v6h6\"></path><circle cx=\"12\" cy=\"17\" r=\"1\"></circle>",
        categories = "text,arrows",
        tags = "redo,history,step,back",
        contributors = "danielbayley"
    ))]
    UndoDot,
    #[cfg(feature = "undo")]
    #[strum(props(
        svg = "<path d=\"M3 7v6h6\"></path><path d=\"M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13\"></path>",
        categories = "text,arrows",
        tags = "redo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Undo,
    #[cfg(feature = "unfold_horizontal")]
    #[strum(props(
        svg = "<path d=\"M16 12h6\"></path><path d=\"M8 12H2\"></path><path d=\"M12 2v2\"></path><path d=\"M12 8v2\"></path><path d=\"M12 14v2\"></path><path d=\"M12 20v2\"></path><path d=\"m19 15 3-3-3-3\"></path><path d=\"m5 9-3 3 3 3\"></path>",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley"
    ))]
    UnfoldHorizontal,
    #[cfg(feature = "unfold_vertical")]
    #[strum(props(
        svg = "<path d=\"M12 22v-6\"></path><path d=\"M12 8V2\"></path><path d=\"M4 12H2\"></path><path d=\"M10 12H8\"></path><path d=\"M16 12h-2\"></path><path d=\"M22 12h-2\"></path><path d=\"m15 19-3 3-3-3\"></path><path d=\"m15 5-3-3-3 3\"></path>",
        categories = "arrows,layout",
        tags = "arrow,expand,vertical,dashed",
        contributors = "danielbayley"
    ))]
    UnfoldVertical,
    #[cfg(feature = "ungroup")]
    #[strum(props(
        svg = "<rect height=\"6\" y=\"4\" rx=\"1\" x=\"5\" width=\"8\"></rect><rect width=\"8\" x=\"11\" height=\"6\" y=\"14\" rx=\"1\"></rect>",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,separate",
        contributors = "danielbayley"
    ))]
    Ungroup,
    #[cfg(feature = "university")]
    #[strum(props(
        svg = "<path d=\"M14 21v-3a2 2 0 0 0-4 0v3\"></path><path d=\"M18 12h.01\"></path><path d=\"M18 16h.01\"></path><path d=\"M22 7a1 1 0 0 0-1-1h-2a2 2 0 0 1-1.14-.359L13.14 2.36a2 2 0 0 0-2.28-.001L6.14 5.64A2 2 0 0 1 5 6H3a1 1 0 0 0-1 1v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2z\"></path><path d=\"M6 12h.01\"></path><path d=\"M6 16h.01\"></path><circle cy=\"10\" r=\"2\" cx=\"12\"></circle>",
        categories = "buildings,navigation",
        tags = "building,education,childhood,school,college,academy,institute",
        contributors = "karsa-mistmere"
    ))]
    University,
    #[cfg(feature = "unlink_2")]
    #[strum(props(
        svg = "<path d=\"M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2\"></path>",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink2,
    #[cfg(feature = "unlink")]
    #[strum(props(
        svg = "<path d=\"m18.84 12.25 1.72-1.71h-.02a5.00 5.00 0 0 0-.12-7.07 5.00 5.00 0 0 0-6.95 0l-1.72 1.71\"></path><path d=\"m5.17 11.75-1.71 1.71a5.00 5.00 0 0 0 .12 7.07 5.00 5.00 0 0 0 6.95 0l1.71-1.71\"></path><line x2=\"8\" y1=\"2\" x1=\"8\" y2=\"5\"></line><line x1=\"2\" x2=\"5\" y1=\"8\" y2=\"8\"></line><line x1=\"16\" x2=\"16\" y1=\"19\" y2=\"22\"></line><line x1=\"19\" y1=\"16\" x2=\"22\" y2=\"16\"></line>",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unplug")]
    #[strum(props(
        svg = "<path d=\"m19 5 3-3\"></path><path d=\"m2 22 3-3\"></path><path d=\"M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z\"></path><path d=\"M7.5 13.5 10 11\"></path><path d=\"M10.5 16.5 13 14\"></path><path d=\"m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z\"></path>",
        categories = "devices,development",
        tags = "electricity,energy,electronics,socket,outlet,disconnect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Unplug,
    #[cfg(feature = "upload")]
    #[strum(props(
        svg = "<path d=\"M12 3v12\"></path><path d=\"m17 8-5-5-5 5\"></path><path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"></path>",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[cfg(feature = "usb")]
    #[strum(props(
        svg = "<circle cx=\"10\" cy=\"7\" r=\"1\"></circle><circle cy=\"20\" r=\"1\" cx=\"4\"></circle><path d=\"M4.7 19.3 19 5\"></path><path d=\"m21 3-3 1 2 2Z\"></path><path d=\"M9.26 7.68 5 12l2 5\"></path><path d=\"m10 14 5 2 3.5-3.5\"></path><path d=\"m18 12 1-1 1 1-1 1Z\"></path>",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_check")]
    #[strum(props(
        svg = "<path d=\"m16 11 2 2 4-4\"></path><path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"></path><circle cx=\"9\" cy=\"7\" r=\"4\"></circle>",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "<path d=\"M10 15H6a4 4 0 0 0-4 4v2\"></path><path d=\"m14.30 16.53.92-.382\"></path><path d=\"m15.22 13.85-.923-.383\"></path><path d=\"m16.85 12.22-.383-.923\"></path><path d=\"m16.85 17.77-.383.92\"></path><path d=\"m19.14 12.22.383-.923\"></path><path d=\"m19.53 18.69-.382-.924\"></path><path d=\"m20.77 13.85.924-.383\"></path><path d=\"m20.77 16.14.924.38\"></path><circle cy=\"15\" cx=\"18\" r=\"3\"></circle><circle cx=\"9\" cy=\"7\" r=\"4\"></circle>",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis,UsamaKhan"
    ))]
    UserCog,
    #[cfg(feature = "user_key")]
    #[strum(props(
        svg = "<path d=\"M20 11v6\"></path><path d=\"M20 13h2\"></path><path d=\"M3 21v-2a4 4 0 0 1 4-4h6a4 4 0 0 1 2.07.578\"></path><circle cx=\"10\" r=\"4\" cy=\"7\"></circle><circle cy=\"19\" r=\"2\" cx=\"20\"></circle>",
        categories = "account",
        tags = "passkey,password,login,authentication,authorization,roles,permissions,private,public,security,person,account,contact",
        contributors = "colebemis,csandman,ericfennis,mittalyashu,karsa-mistmere"
    ))]
    UserKey,
    #[cfg(feature = "user_lock")]
    #[strum(props(
        svg = "<path d=\"M19 16v-2a2 2 0 0 0-4 0v2\"></path><path d=\"M9.5 15H7a4 4 0 0 0-4 4v2\"></path><circle cy=\"7\" r=\"4\" cx=\"10\"></circle><rect rx=\".899\" height=\"5\" y=\"16\" x=\"13\" width=\"8\"></rect>",
        categories = "account,security",
        tags = "person,lock,locked,account,secure",
        contributors = "anthony-mariotti,jguddas"
    ))]
    UserLock,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "<path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"></path><circle cy=\"7\" r=\"4\" cx=\"9\"></circle><line y1=\"11\" y2=\"11\" x1=\"22\" x2=\"16\"></line>",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_pen")]
    #[strum(props(
        svg = "<path d=\"M11.5 15H7a4 4 0 0 0-4 4v2\"></path><path d=\"M21.37 16.62a1 1 0 0 0-3.00-3.00l-4.01 4.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path><circle r=\"4\" cx=\"10\" cy=\"7\"></circle>",
        categories = "account",
        tags = "person,account,contact,profile,edit,change",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    UserPen,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "<path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"></path><circle cx=\"9\" r=\"4\" cy=\"7\"></circle><line y2=\"14\" y1=\"8\" x2=\"19\" x1=\"19\"></line><line x2=\"16\" y1=\"11\" y2=\"11\" x1=\"22\"></line>",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[cfg(feature = "user_round_check")]
    #[strum(props(
        svg = "<path d=\"M2 21a8 8 0 0 1 13.29-6\"></path><circle r=\"5\" cx=\"10\" cy=\"8\"></circle><path d=\"m16 19 2 2 4-4\"></path>",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserRoundCheck,
    #[cfg(feature = "user_round_cog")]
    #[strum(props(
        svg = "<path d=\"m14.30 19.53.92-.382\"></path><path d=\"m15.22 16.85-.923-.383\"></path><path d=\"m16.85 15.22-.383-.923\"></path><path d=\"m16.85 20.77-.383.92\"></path><path d=\"m19.14 15.22.383-.923\"></path><path d=\"m19.53 21.69-.382-.924\"></path><path d=\"M2 21a8 8 0 0 1 10.43-7.62\"></path><path d=\"m20.77 16.85.924-.383\"></path><path d=\"m20.77 19.14.924.38\"></path><circle r=\"5\" cx=\"10\" cy=\"8\"></circle><circle cx=\"18\" r=\"3\" cy=\"18\"></circle>",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,UsamaKhan"
    ))]
    UserRoundCog,
    #[cfg(feature = "user_round_key")]
    #[strum(props(
        svg = "<path d=\"M19 11v6\"></path><path d=\"M19 13h2\"></path><path d=\"M2 21a8 8 0 0 1 12.86-6.34\"></path><circle r=\"5\" cx=\"10\" cy=\"8\"></circle><circle r=\"2\" cx=\"19\" cy=\"19\"></circle>",
        categories = "account",
        tags = "passkey,password,login,authentication,authorization,roles,permissions,private,public,security,person,account,contact",
        contributors = "colebemis,csandman,ericfennis,mittalyashu,karsa-mistmere"
    ))]
    UserRoundKey,
    #[cfg(feature = "user_round_minus")]
    #[strum(props(
        svg = "<path d=\"M2 21a8 8 0 0 1 13.29-6\"></path><circle r=\"5\" cx=\"10\" cy=\"8\"></circle><path d=\"M22 19h-6\"></path>",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundMinus,
    #[cfg(feature = "user_round_pen")]
    #[strum(props(
        svg = "<path d=\"M2 21a8 8 0 0 1 10.82-7.48\"></path><path d=\"M21.37 16.62a1 1 0 0 0-3.00-3.00l-4.01 4.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path><circle cx=\"10\" r=\"5\" cy=\"8\"></circle>",
        categories = "account",
        tags = "person,account,contact,profile,edit,change",
        contributors = "karsa-mistmere,colebemis,csandman,ericfennis"
    ))]
    UserRoundPen,
    #[cfg(feature = "user_round_plus")]
    #[strum(props(
        svg = "<path d=\"M2 21a8 8 0 0 1 13.29-6\"></path><circle cy=\"8\" r=\"5\" cx=\"10\"></circle><path d=\"M19 16v6\"></path><path d=\"M22 19h-6\"></path>",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserRoundPlus,
    #[cfg(feature = "user_round_search")]
    #[strum(props(
        svg = "<circle r=\"5\" cx=\"10\" cy=\"8\"></circle><path d=\"M2 21a8 8 0 0 1 10.43-7.62\"></path><circle r=\"3\" cy=\"18\" cx=\"18\"></circle><path d=\"m22 22-1.9-1.9\"></path>",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass,lens",
        contributors = "jmsv,karsa-mistmere"
    ))]
    UserRoundSearch,
    #[cfg(feature = "user_round_x")]
    #[strum(props(
        svg = "<path d=\"M2 21a8 8 0 0 1 11.87-7\"></path><circle cy=\"8\" r=\"5\" cx=\"10\"></circle><path d=\"m17 17 5 5\"></path><path d=\"m22 17-5 5\"></path>",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserRoundX,
    #[cfg(feature = "user_round")]
    #[strum(props(
        svg = "<circle cy=\"8\" r=\"5\" cx=\"12\"></circle><path d=\"M20 21a8 8 0 0 0-16 0\"></path>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserRound,
    #[cfg(feature = "user_search")]
    #[strum(props(
        svg = "<circle r=\"4\" cy=\"7\" cx=\"10\"></circle><path d=\"M10.3 15H7a4 4 0 0 0-4 4v2\"></path><circle cx=\"17\" cy=\"17\" r=\"3\"></circle><path d=\"m21 21-1.9-1.9\"></path>",
        categories = "account,social",
        tags = "person,account,contact,find,scan,magnifier,magnifying glass,lens",
        contributors = "jmsv,jguddas,colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserSearch,
    #[cfg(feature = "user_star")]
    #[strum(props(
        svg = "<path d=\"M16.05 12.61a1 1 0 0 1 1.90.024l.737 1.45a1 1 0 0 0 .737.53l1.63.256a1 1 0 0 1 .588 1.80l-1.17 1.16a1 1 0 0 0-.282.86l.259 1.61a1 1 0 0 1-1.54 1.13l-1.46-.75a1 1 0 0 0-.912 0l-1.46.75a1 1 0 0 1-1.53-1.13l.258-1.61a1 1 0 0 0-.282-.866l-1.15-1.15a1 1 0 0 1 .572-1.82l1.63-.256a1 1 0 0 0 .737-.535z\"></path><path d=\"M8 15H7a4 4 0 0 0-4 4v2\"></path><circle cy=\"7\" r=\"4\" cx=\"10\"></circle>",
        categories = "account",
        tags = "person,account,favorite,contact,like,review,rating,admin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas,MArtytraM99"
    ))]
    UserStar,
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "<path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"></path><circle cx=\"9\" cy=\"7\" r=\"4\"></circle><line y2=\"13\" y1=\"8\" x1=\"17\" x2=\"22\"></line><line y2=\"13\" x1=\"22\" x2=\"17\" y1=\"8\"></line>",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "<path d=\"M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2\"></path><circle r=\"4\" cx=\"12\" cy=\"7\"></circle>",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_round")]
    #[strum(props(
        svg = "<path d=\"M18 21a8 8 0 0 0-16 0\"></path><circle cx=\"10\" r=\"5\" cy=\"8\"></circle><path d=\"M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3\"></path>",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    UsersRound,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "<path d=\"M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2\"></path><path d=\"M16 3.12a4 4 0 0 1 0 7.74\"></path><path d=\"M22 21v-2a4 4 0 0 0-3-3.87\"></path><circle r=\"4\" cy=\"7\" cx=\"9\"></circle>",
        categories = "account",
        tags = "group,people",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Users,
    #[cfg(feature = "utensils_crossed")]
    #[strum(props(
        svg = "<path d=\"m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8\"></path><path d=\"M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7\"></path><path d=\"m2.1 21.8 6.4-6.3\"></path><path d=\"m19 5-7 7\"></path>",
        categories = "food-beverage,travel,navigation",
        tags = "fork,knife,cutlery,flatware,tableware,silverware,food,restaurant,meal,breakfast,dinner,supper",
        contributors = "karsa-mistmere"
    ))]
    UtensilsCrossed,
    #[cfg(feature = "utensils")]
    #[strum(props(
        svg = "<path d=\"M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2\"></path><path d=\"M7 2v20\"></path><path d=\"M21 15V2a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7\"></path>",
        categories = "food-beverage,travel,navigation",
        tags = "fork,knife,cutlery,flatware,tableware,silverware,food,restaurant,meal,breakfast,dinner,supper",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Utensils,
    #[cfg(feature = "utility_pole")]
    #[strum(props(
        svg = "<path d=\"M12 2v20\"></path><path d=\"M2 5h20\"></path><path d=\"M3 3v2\"></path><path d=\"M7 3v2\"></path><path d=\"M17 3v2\"></path><path d=\"M21 3v2\"></path><path d=\"m19 5-7 7-7-7\"></path>",
        categories = "buildings,home,sustainability",
        tags = "electricity,energy,transmission line,telegraph pole,power lines,phone",
        contributors = "karsa-mistmere"
    ))]
    UtilityPole,
    #[cfg(feature = "van")]
    #[strum(props(
        svg = "<path d=\"M13 6v5a1 1 0 0 0 1 1h6.10a1 1 0 0 1 .712.29l.898.91a1 1 0 0 1 .288.70V17a1 1 0 0 1-1 1h-3\"></path><path d=\"M5 18H3a1 1 0 0 1-1-1V8a2 2 0 0 1 2-2h12c1.1 0 2.1.8 2.4 1.8l1.17 4.2\"></path><path d=\"M9 18h5\"></path><circle cy=\"18\" cx=\"16\" r=\"2\"></circle><circle cy=\"18\" cx=\"7\" r=\"2\"></circle>",
        categories = "transportation",
        tags = "minivan,cart,wagon,truck,lorry,trailer,camper,vehicle,drive,trip,journey,van,transport,carriage,delivery,travel",
        contributors = "Ahmed-Dghaies,karsa-mistmere"
    ))]
    Van,
    #[cfg(feature = "variable")]
    #[strum(props(
        svg = "<path d=\"M8 21s-4-3-4-9 4-9 4-9\"></path><path d=\"M16 3s4 3 4 9-4 9-4 9\"></path><line x2=\"9\" y2=\"15\" y1=\"9\" x1=\"15\"></line><line y1=\"9\" y2=\"15\" x1=\"9\" x2=\"15\"></line>",
        categories = "development,math",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[cfg(feature = "vault")]
    #[strum(props(
        svg = "<rect width=\"18\" x=\"3\" y=\"3\" height=\"18\" rx=\"2\"></rect><circle cy=\"7.5\" fill=\"currentColor\" r=\".5\" cx=\"7.5\"></circle><path d=\"m7.9 7.9 2.7 2.7\"></path><circle r=\".5\" cx=\"16.5\" cy=\"7.5\" fill=\"currentColor\"></circle><path d=\"m13.4 10.6 2.7-2.7\"></path><circle cx=\"7.5\" cy=\"16.5\" fill=\"currentColor\" r=\".5\"></circle><path d=\"m7.9 16.1 2.7-2.7\"></path><circle r=\".5\" cx=\"16.5\" fill=\"currentColor\" cy=\"16.5\"></circle><path d=\"m13.4 13.4 2.7 2.7\"></path><circle cx=\"12\" cy=\"12\" r=\"2\"></circle>",
        categories = "security,travel,home",
        tags = "safe,lockbox,deposit,locker,coffer,strongbox,safety,secure,storage,valuables,bank",
        contributors = "danielbayley"
    ))]
    Vault,
    #[cfg(feature = "vector_square")]
    #[strum(props(
        svg = "<path d=\"M19.5 7a24 24 0 0 1 0 10\"></path><path d=\"M4.5 7a24 24 0 0 0 0 10\"></path><path d=\"M7 19.5a24 24 0 0 0 10 0\"></path><path d=\"M7 4.5a24 24 0 0 1 10 0\"></path><rect height=\"5\" x=\"17\" rx=\"1\" width=\"5\" y=\"17\"></rect><rect y=\"2\" width=\"5\" x=\"17\" rx=\"1\" height=\"5\"></rect><rect height=\"5\" rx=\"1\" x=\"2\" width=\"5\" y=\"17\"></rect><rect height=\"5\" rx=\"1\" width=\"5\" y=\"2\" x=\"2\"></rect>",
        categories = "shapes,math,design,tools",
        tags = "shape,geometry,art,width,height,size,calculate,measure,select,graphics,box",
        contributors = "chessurisme,jguddas"
    ))]
    VectorSquare,
    #[cfg(feature = "vegan")]
    #[strum(props(
        svg = "<path d=\"M16 8q6 0 6-6-6 0-6 6\"></path><path d=\"M17.41 3.59a10 10 0 1 0 3 3\"></path><path d=\"M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14\"></path>",
        categories = "food-beverage,sustainability",
        tags = "vegetarian,fruitarian,herbivorous,animal rights,diet",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Vegan,
    #[cfg(feature = "venetian_mask")]
    #[strum(props(
        svg = "<path d=\"M18 11c-1.5 0-2.5.5-3 2\"></path><path d=\"M4 6a2 2 0 0 0-2 2v4a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V8a2 2 0 0 0-2-2h-3a8 8 0 0 0-5 2 8 8 0 0 0-5-2z\"></path><path d=\"M6 11c1.5 0 2.5.5 3 2\"></path>",
        categories = "account,gaming",
        tags = "mask,masquerade,impersonate,secret,incognito",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    VenetianMask,
    #[cfg(feature = "venus_and_mars")]
    #[strum(props(
        svg = "<path d=\"M10 20h4\"></path><path d=\"M12 16v6\"></path><path d=\"M17 2h4v4\"></path><path d=\"m21 2-5.46 5.46\"></path><circle cx=\"12\" cy=\"11\" r=\"5\"></circle>",
        categories = "medical",
        tags = "gender,sex,intersex,androgynous,hermaphrodite",
        contributors = "jamiemlaw"
    ))]
    VenusAndMars,
    #[cfg(feature = "venus")]
    #[strum(props(
        svg = "<path d=\"M12 15v7\"></path><path d=\"M9 19h6\"></path><circle r=\"6\" cx=\"12\" cy=\"9\"></circle>",
        categories = "medical",
        tags = "gender,sex,female,feminine,woman,girl",
        contributors = "jguddas,jamiemlaw"
    ))]
    Venus,
    #[cfg(feature = "vibrate_off")]
    #[strum(props(
        svg = "<path d=\"m2 8 2 2-2 2 2 2-2 2\"></path><path d=\"m22 8-2 2 2 2-2 2 2 2\"></path><path d=\"M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2\"></path><path d=\"M16 10.34V6c0-.55-.45-1-1-1h-4.34\"></path><line y1=\"2\" y2=\"22\" x2=\"22\" x1=\"2\"></line>",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "<path d=\"m2 8 2 2-2 2 2 2-2 2\"></path><path d=\"m22 8-2 2 2 2-2 2 2 2\"></path><rect rx=\"1\" width=\"8\" height=\"14\" x=\"8\" y=\"5\"></rect>",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "<path d=\"M10.66 6H14a2 2 0 0 1 2 2v2.5l5.24-3.06A.5.5 0 0 1 22 7.87v8.19\"></path><path d=\"M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "<path d=\"m16 13 5.22 3.48a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5\"></path><rect height=\"12\" width=\"14\" y=\"6\" x=\"2\" rx=\"2\"></rect>",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "<rect x=\"2\" width=\"20\" height=\"16\" y=\"4\" rx=\"2\"></rect><path d=\"M2 8h20\"></path><circle cx=\"8\" cy=\"14\" r=\"2\"></circle><path d=\"M8 12h8\"></path><circle cy=\"14\" r=\"2\" cx=\"16\"></circle>",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = "danielbayley"
    ))]
    Videotape,
    #[cfg(feature = "view")]
    #[strum(props(
        svg = "<path d=\"M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2\"></path><path d=\"M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2\"></path><circle cx=\"12\" cy=\"12\" r=\"1\"></circle><path d=\"M18.94 12.33a1 1 0 0 0 0-.66 7.5 7.5 0 0 0-13.88 0 1 1 0 0 0 0 .66 7.5 7.5 0 0 0 13.88 0\"></path>",
        categories = "design,photography",
        tags = "eye,look",
        contributors = "zenoamaro,ericfennis,csandman,karsa-mistmere"
    ))]
    View,
    #[cfg(feature = "voicemail")]
    #[strum(props(
        svg = "<circle r=\"4\" cx=\"6\" cy=\"12\"></circle><circle cy=\"12\" r=\"4\" cx=\"18\"></circle><line y2=\"16\" x2=\"18\" y1=\"16\" x1=\"6\"></line>",
        categories = "connectivity,devices,social",
        tags = "phone,cassette,tape,reel,recording,audio",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Voicemail,
    #[cfg(feature = "volleyball")]
    #[strum(props(
        svg = "<path d=\"M11.1 7.1a16.55 16.55 0 0 1 10.9 4\"></path><path d=\"M12 12a12.6 12.6 0 0 1-8.7 5\"></path><path d=\"M16.8 13.6a16.55 16.55 0 0 1-9 7.5\"></path><path d=\"M20.7 17a12.8 12.8 0 0 0-8.7-5 13.3 13.3 0 0 1 0-10\"></path><path d=\"M6.3 3.8a16.55 16.55 0 0 0 1.9 11.5\"></path><circle r=\"10\" cx=\"12\" cy=\"12\"></circle>",
        categories = "sports,gaming,travel",
        tags = "beach,sand,net,holiday,vacation,summer,soccer,football,futbol,kick,pitch,goal,score,bounce,leather,wool,yarn,knitting,sewing,thread,embroidery,textile",
        contributors = "danielbayley,jguddas"
    ))]
    Volleyball,
    #[cfg(feature = "volume_1")]
    #[strum(props(
        svg = "<path d=\"M11 4.70a.705.70 0 0 0-1.20-.498L6.41 7.58A1.4 1.4 0 0 1 5.41 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.41a1.4 1.4 0 0 1 .997.41l3.38 3.38A.705.70 0 0 0 11 19.29z\"></path><path d=\"M16 9a5 5 0 0 1 0 6\"></path>",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Volume1,
    #[cfg(feature = "volume_2")]
    #[strum(props(
        svg = "<path d=\"M11 4.70a.705.70 0 0 0-1.20-.498L6.41 7.58A1.4 1.4 0 0 1 5.41 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.41a1.4 1.4 0 0 1 .997.41l3.38 3.38A.705.70 0 0 0 11 19.29z\"></path><path d=\"M16 9a5 5 0 0 1 0 6\"></path><path d=\"M19.36 18.36a9 9 0 0 0 0-12.72\"></path>",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Volume2,
    #[cfg(feature = "volume_off")]
    #[strum(props(
        svg = "<path d=\"M16 9a5 5 0 0 1 .95 2.29\"></path><path d=\"M19.36 5.63a9 9 0 0 1 1.88 9.96\"></path><path d=\"m2 2 20 20\"></path><path d=\"m7 7-.587.58A1.4 1.4 0 0 1 5.41 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.41a1.4 1.4 0 0 1 .997.41l3.38 3.38A.705.70 0 0 0 11 19.29V11\"></path><path d=\"M9.82 4.17A.686.68 0 0 1 11 4.65v.686\"></path>",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "karsa-mistmere,colebemis,ericfennis"
    ))]
    VolumeOff,
    #[cfg(feature = "volume_x")]
    #[strum(props(
        svg = "<path d=\"M11 4.70a.705.70 0 0 0-1.20-.498L6.41 7.58A1.4 1.4 0 0 1 5.41 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.41a1.4 1.4 0 0 1 .997.41l3.38 3.38A.705.70 0 0 0 11 19.29z\"></path><line x1=\"22\" x2=\"16\" y2=\"15\" y1=\"9\"></line><line y1=\"9\" x1=\"16\" x2=\"22\" y2=\"15\"></line>",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    VolumeX,
    #[cfg(feature = "volume")]
    #[strum(props(
        svg = "<path d=\"M11 4.70a.705.70 0 0 0-1.20-.498L6.41 7.58A1.4 1.4 0 0 1 5.41 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.41a1.4 1.4 0 0 1 .997.41l3.38 3.38A.705.70 0 0 0 11 19.29z\"></path>",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis"
    ))]
    Volume,
    #[cfg(feature = "vote")]
    #[strum(props(
        svg = "<path d=\"m9 12 2 2 4-4\"></path><path d=\"M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z\"></path><path d=\"M22 19H2\"></path>",
        categories = "social",
        tags = "vote,poll,ballot,political,social,check,tick",
        contributors = "ptrgast,karsa-mistmere"
    ))]
    Vote,
    #[cfg(feature = "wallet_cards")]
    #[strum(props(
        svg = "<rect rx=\"2\" height=\"18\" width=\"18\" x=\"3\" y=\"3\"></rect><path d=\"M3 9a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2\"></path><path d=\"M3 11h3c.8 0 1.6.3 2.1.9l1.1.9c1.6 1.6 4.1 1.6 5.7 0l1.1-.9c.5-.5 1.3-.9 2.1-.9H21\"></path>",
        categories = "account,finance",
        tags = "money,finance,pocket,credit,purchase,payment,shopping,retail,consumer,cc",
        contributors = "danielbayley"
    ))]
    WalletCards,
    #[cfg(feature = "wallet_minimal")]
    #[strum(props(
        svg = "<path d=\"M17 14h.01\"></path><path d=\"M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14\"></path>",
        categories = "account,finance",
        tags = "finance,pocket",
        contributors = "danielbayley"
    ))]
    WalletMinimal,
    #[cfg(feature = "wallet")]
    #[strum(props(
        svg = "<path d=\"M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1\"></path><path d=\"M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4\"></path>",
        categories = "account,finance",
        tags = "money,finance,pocket",
        contributors = "mittalyashu,ahtohbi4,ericfennis"
    ))]
    Wallet,
    #[cfg(feature = "wallpaper")]
    #[strum(props(
        svg = "<path d=\"M12 17v4\"></path><path d=\"M8 21h8\"></path><path d=\"m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15\"></path><circle cx=\"8\" cy=\"9\" r=\"2\"></circle><rect width=\"20\" x=\"2\" height=\"14\" y=\"3\" rx=\"2\"></rect>",
        categories = "account,devices",
        tags = "background,texture,image,art,design,visual,decor,pattern,screen,cover,lock screen",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Wallpaper,
    #[cfg(feature = "wand_sparkles")]
    #[strum(props(
        svg = "<path d=\"m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72\"></path><path d=\"m14 7 3 3\"></path><path d=\"M5 6v4\"></path><path d=\"M19 14v4\"></path><path d=\"M10 2v2\"></path><path d=\"M7 8H3\"></path><path d=\"M21 16h-4\"></path><path d=\"M11 3H9\"></path>",
        categories = "design,gaming,cursors,photography",
        tags = "magic,wizard,magician",
        contributors = "karsa-mistmere"
    ))]
    WandSparkles,
    #[cfg(feature = "wand")]
    #[strum(props(
        svg = "<path d=\"M15 4V2\"></path><path d=\"M15 16v-2\"></path><path d=\"M8 9h2\"></path><path d=\"M20 9h2\"></path><path d=\"M17.8 11.8 19 13\"></path><path d=\"M15 9h.01\"></path><path d=\"M17.8 6.2 19 5\"></path><path d=\"m3 21 9-9\"></path><path d=\"M12.2 6.2 11 5\"></path>",
        categories = "design,gaming,cursors,photography",
        tags = "magic,selection",
        contributors = "mittalyashu,ericfennis"
    ))]
    Wand,
    #[cfg(feature = "warehouse")]
    #[strum(props(
        svg = "<path d=\"M18 21V10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1v11\"></path><path d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 1.13-1.80l7.95-3.97a2 2 0 0 1 1.83 0l7.94 3.97A2 2 0 0 1 22 8z\"></path><path d=\"M6 13h12\"></path><path d=\"M6 17h12\"></path>",
        categories = "buildings,navigation",
        tags = "storage,storehouse,depot,depository,repository,stockroom,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "washing_machine")]
    #[strum(props(
        svg = "<path d=\"M3 6h3\"></path><path d=\"M17 6h.01\"></path><rect height=\"20\" x=\"3\" width=\"18\" rx=\"2\" y=\"2\"></rect><circle r=\"5\" cx=\"12\" cy=\"13\"></circle><path d=\"M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5\"></path>",
        categories = "home,devices,travel",
        tags = "tumble dryer,amenities,electronics,cycle,clothes,rinse,spin,drum",
        contributors = "danielbayley"
    ))]
    WashingMachine,
    #[cfg(feature = "watch")]
    #[strum(props(
        svg = "<path d=\"M12 10v2.2l1.6 1\"></path><path d=\"m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05\"></path><path d=\"m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05\"></path><circle cx=\"12\" cy=\"12\" r=\"6\"></circle>",
        categories = "time",
        tags = "clock,time",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jamiemlaw"
    ))]
    Watch,
    #[cfg(feature = "waves_arrow_down")]
    #[strum(props(
        svg = "<path d=\"M12 10L12 2\"></path><path d=\"M16 6L12 10L8 6\"></path><path d=\"M2 15C2.6 15.5 3.2 16 4.5 16C7 16 7 14 9.5 14C12.1 14 11.9 16 14.5 16C17 16 17 14 19.5 14C20.8 14 21.4 14.5 22 15\"></path><path d=\"M2 21C2.6 21.5 3.2 22 4.5 22C7 22 7 20 9.5 20C12.1 20 11.9 22 14.5 22C17 22 17 20 19.5 20C20.8 20 21.4 20.5 22 21\"></path>",
        categories = "weather,sustainability",
        tags = "water,sea,level,sound,hertz,wavelength,vibrate,low,tide,ocean,rising,down,falling",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    WavesArrowDown,
    #[cfg(feature = "waves_arrow_up")]
    #[strum(props(
        svg = "<path d=\"M12 2v8\"></path><path d=\"M2 15c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"m8 6 4-4 4 4\"></path>",
        categories = "weather,sustainability",
        tags = "water,sea,level,sound,hertz,wavelength,vibrate,high,tide,ocean,rising",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    WavesArrowUp,
    #[cfg(feature = "waves_ladder")]
    #[strum(props(
        svg = "<path d=\"M19 5a2 2 0 0 0-2 2v11\"></path><path d=\"M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M7 13h10\"></path><path d=\"M7 9h10\"></path><path d=\"M9 5a2 2 0 0 0-2 2v11\"></path>",
        categories = "sports,home",
        tags = "swimming,water,pool,lifeguard,ocean,🌊,🏊‍♂️,🏊‍♀️,🏊,🥽",
        contributors = "karsa-mistmere"
    ))]
    WavesLadder,
    #[cfg(feature = "waves")]
    #[strum(props(
        svg = "<path d=\"M2 6c.6.5 1.2 1 2.5 1C7 7 7 5 9.5 5c2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M2 12c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path><path d=\"M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1\"></path>",
        categories = "weather,navigation,multimedia,sustainability",
        tags = "water,sea,sound,hertz,wavelength,vibrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Waves,
    #[cfg(feature = "waypoints")]
    #[strum(props(
        svg = "<path d=\"m10.58 5.41-5.17 5.17\"></path><path d=\"m18.58 13.41-5.17 5.17\"></path><path d=\"M6 12h12\"></path><circle cx=\"12\" r=\"2\" cy=\"20\"></circle><circle r=\"2\" cy=\"4\" cx=\"12\"></circle><circle cy=\"12\" cx=\"20\" r=\"2\"></circle><circle cy=\"12\" cx=\"4\" r=\"2\"></circle>",
        categories = "security,account,navigation,development,social",
        tags = "indirection,vpn,virtual private network,proxy,connections,bounce,reroute,path,journey,planner,stops,stations,shared,spread,viral",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Waypoints,
    #[cfg(feature = "webcam")]
    #[strum(props(
        svg = "<circle r=\"8\" cx=\"12\" cy=\"10\"></circle><circle cx=\"12\" r=\"3\" cy=\"10\"></circle><path d=\"M7 22h10\"></path><path d=\"M12 22v-4\"></path>",
        categories = "connectivity,devices,communication",
        tags = "camera,security",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Webcam,
    #[cfg(feature = "webhook_off")]
    #[strum(props(
        svg = "<path d=\"M17 17h-5c-1.09-.02-1.94.92-2.5 1.9A3 3 0 1 1 2.57 15\"></path><path d=\"M9 3.4a4 4 0 0 1 6.52.66\"></path><path d=\"m6 17 3.1-5.8a2.5 2.5 0 0 0 .057-2.05\"></path><path d=\"M20.3 20.3a4 4 0 0 1-2.3.7\"></path><path d=\"M18.6 13a4 4 0 0 1 3.35 3.41\"></path><path d=\"m12 6 .6 1\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere,jguddas"
    ))]
    WebhookOff,
    #[cfg(feature = "webhook")]
    #[strum(props(
        svg = "<path d=\"M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2\"></path><path d=\"m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06\"></path><path d=\"m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8\"></path>",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere"
    ))]
    Webhook,
    #[cfg(feature = "weight_tilde")]
    #[strum(props(
        svg = "<path d=\"M6.5 8a2 2 0 0 0-1.90 1.46L2.1 18.5A2 2 0 0 0 4 21h16a2 2 0 0 0 1.92-2.54L19.4 9.5A2 2 0 0 0 17.48 8z\"></path><path d=\"M7.99 15a2.5 2.5 0 0 1 4 0 2.5 2.5 0 0 0 4 0\"></path><circle r=\"3\" cx=\"12\" cy=\"5\"></circle>",
        categories = "math",
        tags = "measure,scale,estimate,load,balance,size,measurement,quantity,mass",
        contributors = "nathan-de-pachtere"
    ))]
    WeightTilde,
    #[cfg(feature = "weight")]
    #[strum(props(
        svg = "<circle r=\"3\" cx=\"12\" cy=\"5\"></circle><path d=\"M6.5 8a2 2 0 0 0-1.90 1.46L2.1 18.5A2 2 0 0 0 4 21h16a2 2 0 0 0 1.92-2.54L19.4 9.5A2 2 0 0 0 17.48 8Z\"></path>",
        categories = "math",
        tags = "mass,heavy,lead,metal,measure,geometry,scales,balance",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Weight,
    #[cfg(feature = "wheat_off")]
    #[strum(props(
        svg = "<path d=\"m2 22 10-10\"></path><path d=\"m16 8-1.17 1.17\"></path><path d=\"M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z\"></path><path d=\"m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97\"></path><path d=\"M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62\"></path><path d=\"M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z\"></path><path d=\"M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z\"></path><path d=\"m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98\"></path><path d=\"M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28\"></path><line x2=\"22\" y2=\"22\" x1=\"2\" y1=\"2\"></line>",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WheatOff,
    #[cfg(feature = "wheat")]
    #[strum(props(
        svg = "<path d=\"M2 22 16 8\"></path><path d=\"M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z\"></path><path d=\"M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z\"></path><path d=\"M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z\"></path><path d=\"M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z\"></path><path d=\"M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z\"></path><path d=\"M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z\"></path><path d=\"M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z\"></path>",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten",
        contributors = "karsa-mistmere"
    ))]
    Wheat,
    #[cfg(feature = "whole_word")]
    #[strum(props(
        svg = "<circle cx=\"7\" cy=\"12\" r=\"3\"></circle><path d=\"M10 9v6\"></path><circle cx=\"17\" r=\"3\" cy=\"12\"></circle><path d=\"M14 7v8\"></path><path d=\"M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1\"></path>",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_cog")]
    #[strum(props(
        svg = "<path d=\"m14.30 19.53.92-.382\"></path><path d=\"m15.22 16.85-.923-.383\"></path><path d=\"m16.85 15.22-.383-.923\"></path><path d=\"m16.85 20.77-.383.92\"></path><path d=\"m19.14 15.22.383-.923\"></path><path d=\"m19.53 21.69-.382-.924\"></path><path d=\"M2 7.82a15 15 0 0 1 20 0\"></path><path d=\"m20.77 16.85.924-.383\"></path><path d=\"m20.77 19.14.924.38\"></path><path d=\"M5 11.85a10 10 0 0 1 11.5-1.78\"></path><path d=\"M8.5 15.42a5 5 0 0 1 2.41-1.31\"></path><circle cx=\"18\" r=\"3\" cy=\"18\"></circle>",
        categories = "connectivity,devices,files",
        tags = "connection,signal,wireless,directory,settings,control,preferences,cog,edit,gear",
        contributors = "colebemis,ericfennis,jguddas,karsa-mistmere,luisdlopera"
    ))]
    WifiCog,
    #[cfg(feature = "wifi_high")]
    #[strum(props(
        svg = "<path d=\"M12 20h.01\"></path><path d=\"M5 12.85a10 10 0 0 1 14 0\"></path><path d=\"M8.5 16.42a5 5 0 0 1 7 0\"></path>",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiHigh,
    #[cfg(feature = "wifi_low")]
    #[strum(props(
        svg = "<path d=\"M12 20h.01\"></path><path d=\"M8.5 16.42a5 5 0 0 1 7 0\"></path>",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiLow,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "<path d=\"M12 20h.01\"></path><path d=\"M8.5 16.42a5 5 0 0 1 7 0\"></path><path d=\"M5 12.85a10 10 0 0 1 5.17-2.69\"></path><path d=\"M19 12.85a10 10 0 0 0-2.00-1.52\"></path><path d=\"M2 8.82a15 15 0 0 1 4.17-2.64\"></path><path d=\"M22 8.82a15 15 0 0 0-11.28-3.76\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[cfg(feature = "wifi_pen")]
    #[strum(props(
        svg = "<path d=\"M2 8.82a15 15 0 0 1 20 0\"></path><path d=\"M21.37 16.62a1 1 0 0 0-3.00-3.00l-4.01 4.01a2 2 0 0 0-.506.85l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z\"></path><path d=\"M5 12.85a10 10 0 0 1 10.5-2.22\"></path><path d=\"M8.5 16.42a5 5 0 0 1 3-1.40\"></path>",
        categories = "connectivity,devices",
        tags = "edit,wifi,pen,change,network",
        contributors = "karsa-mistmere,jguddas,danielbayley,luisdlopera"
    ))]
    WifiPen,
    #[cfg(feature = "wifi_sync")]
    #[strum(props(
        svg = "<path d=\"M11.96 10.10v4L13.5 12.5a5 5 0 0 1 8 1.5\"></path><path d=\"M11.96 14.10h4\"></path><path d=\"M17.96 18.10h4L20.43 19.71a5 5 0 0 1-8-1.5\"></path><path d=\"M2 8.82a15 15 0 0 1 20 0\"></path><path d=\"M21.96 22.10v-4\"></path><path d=\"M5 12.86a10 10 0 0 1 3-2.03\"></path><path d=\"M8.5 16.42h.01\"></path>",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless,synchronize,reconnect,reset,restart",
        contributors = "colebemis,ericfennis,jguddas,danielbayley,luisdlopera"
    ))]
    WifiSync,
    #[cfg(feature = "wifi_zero")]
    #[strum(props(
        svg = "<path d=\"M12 20h.01\"></path>",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas,VirtCode"
    ))]
    WifiZero,
    #[cfg(feature = "wifi")]
    #[strum(props(
        svg = "<path d=\"M12 20h.01\"></path><path d=\"M2 8.82a15 15 0 0 1 20 0\"></path><path d=\"M5 12.85a10 10 0 0 1 14 0\"></path><path d=\"M8.5 16.42a5 5 0 0 1 7 0\"></path>",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    Wifi,
    #[cfg(feature = "wind_arrow_down")]
    #[strum(props(
        svg = "<path d=\"M10 2v8\"></path><path d=\"M12.8 21.6A2 2 0 1 0 14 18H2\"></path><path d=\"M17.5 10a2.5 2.5 0 1 1 2 4H2\"></path><path d=\"m6 6 4 4 4-4\"></path>",
        categories = "weather,sustainability",
        tags = "weather,air,pressure,blow",
        contributors = "colebemis,csandman,ericfennis,jamiemlaw"
    ))]
    WindArrowDown,
    #[cfg(feature = "wind")]
    #[strum(props(
        svg = "<path d=\"M12.8 19.6A2 2 0 1 0 14 16H2\"></path><path d=\"M17.5 8a2.5 2.5 0 1 1 2 4H2\"></path><path d=\"M9.8 4.4A2 2 0 1 1 11 8H2\"></path>",
        categories = "weather,sustainability",
        tags = "weather,air,blow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Wind,
    #[cfg(feature = "wine_off")]
    #[strum(props(
        svg = "<path d=\"M8 22h8\"></path><path d=\"M7 10h3m7 0h-1.34\"></path><path d=\"M12 15v7\"></path><path d=\"M7.30 7.30A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.39 4.39M8.63 2.98C8.75 2.66 8.87 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.80-.145 1.19\"></path><line y1=\"2\" y2=\"22\" x1=\"2\" x2=\"22\"></line>",
        categories = "food-beverage",
        tags = "alcohol,beverage,drink,glass,alcohol free,abstinence,abstaining,teetotalism,allergy,intolerance",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WineOff,
    #[cfg(feature = "wine")]
    #[strum(props(
        svg = "<path d=\"M8 22h8\"></path><path d=\"M7 10h10\"></path><path d=\"M12 15v7\"></path><path d=\"M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z\"></path>",
        categories = "food-beverage",
        tags = "alcohol,beverage,bar,drink,glass,sommelier,vineyard,winery",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wine,
    #[cfg(feature = "workflow")]
    #[strum(props(
        svg = "<rect y=\"3\" height=\"8\" width=\"8\" x=\"3\" rx=\"2\"></rect><path d=\"M7 11v4a2 2 0 0 0 2 2h4\"></path><rect x=\"13\" y=\"13\" rx=\"2\" width=\"8\" height=\"8\"></rect>",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "worm")]
    #[strum(props(
        svg = "<path d=\"m19 12-1.5 3\"></path><path d=\"M19.63 18.81 22 20\"></path><path d=\"M6.47 8.23a1.68 1.68 0 0 1 2.44 1.93l-.64 2.08a6.76 6.76 0 0 0 10.16 7.67l.42-.27a1 1 0 1 0-2.73-4.21l-.42.27a1.76 1.76 0 0 1-2.63-1.99l.64-2.08A6.66 6.66 0 0 0 3.94 3.9l-.7.4a1 1 0 1 0 2.55 4.34z\"></path>",
        categories = "animals,security",
        tags = "invertebrate,grub,larva,snake,crawl,wiggle,slither,pest control,computer virus,malware",
        contributors = "karsa-mistmere"
    ))]
    Worm,
    #[cfg(feature = "wrench")]
    #[strum(props(
        svg = "<path d=\"M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.10-3.10c.32-.322.86-.22.98.218a6 6 0 0 1-8.25 7.05l-7.91 7.91a1 1 0 0 1-2.99-3l7.91-7.91a6 6 0 0 1 7.05-8.25c.438.12.54.66.219.98z\"></path>",
        categories = "account,development,tools",
        tags = "account,settings,spanner,diy,toolbox,build,construction",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Wrench,
    #[cfg(feature = "x_line_top")]
    #[strum(props(
        svg = "<path d=\"M18 4H6\"></path><path d=\"M18 8 6 20\"></path><path d=\"m6 8 12 12\"></path>",
        categories = "notifications,math",
        tags = "line,top,arrow,navigation,up,pointer,direction,vector,symbol,cancel,close,delete,remove,times,clear,math,multiply,multiplication,mean,median,average,x̄",
        contributors = "colebemis,ericfennis,jguddas"
    ))]
    XLineTop,
    #[cfg(feature = "x")]
    #[strum(props(
        svg = "<path d=\"M18 6 6 18\"></path><path d=\"m6 6 12 12\"></path>",
        categories = "notifications,math",
        tags = "cancel,close,cross,delete,ex,remove,times,clear,math,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    X,
    #[cfg(feature = "zap_off")]
    #[strum(props(
        svg = "<path d=\"M10.51 4.85 13.12 2.17a.5.5 0 0 1 .86.46l-1.37 4.31\"></path><path d=\"M15.65 10H20a1 1 0 0 1 .78 1.63l-1.72 1.77\"></path><path d=\"M16.27 16.27 10.88 21.83a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14H4a1 1 0 0 1-.78-1.63l4.50-4.64\"></path><path d=\"m2 2 20 20\"></path>",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[cfg(feature = "zap")]
    #[strum(props(
        svg = "<path d=\"M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z\"></path>",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning,electricity,energy",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Zap,
    #[cfg(feature = "zodiac_aquarius")]
    #[strum(props(
        svg = "<path d=\"m2 10 2.45-3.68a.7.7 0 0 1 1.10-.013l2.39 3.41a.7.7 0 0 0 1.09-.001l2.40-3.43a.7.7 0 0 1 1.09 0l2.40 3.43a.7.7 0 0 0 1.09 0l2.38-3.41a.7.7 0 0 1 1.10.013L22 10\"></path><path d=\"m2 18.00 2.45-3.68a.7.7 0 0 1 1.10-.013l2.39 3.41a.7.7 0 0 0 1.09 0l2.40-3.43a.7.7 0 0 1 1.09 0l2.40 3.43a.7.7 0 0 0 1.09 0l2.38-3.41a.7.7 0 0 1 1.10.013L22 18.00\"></path>",
        categories = "social,emoji",
        tags = "water bearer,waves,innovation,air,future,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacAquarius,
    #[cfg(feature = "zodiac_aries")]
    #[strum(props(
        svg = "<path d=\"M12 7.5a4.5 4.5 0 1 1 5 4.5\"></path><path d=\"M7 12a4.5 4.5 0 1 1 5-4.5V21\"></path>",
        categories = "social,emoji",
        tags = "ram,horns,fire,energy,initiative,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacAries,
    #[cfg(feature = "zodiac_cancer")]
    #[strum(props(
        svg = "<path d=\"M21 14.5A9 6.5 0 0 1 5.5 19\"></path><path d=\"M3 9.5A9 6.5 0 0 1 18.5 5\"></path><circle cy=\"14.5\" cx=\"17.5\" r=\"3.5\"></circle><circle cx=\"6.5\" cy=\"9.5\" r=\"3.5\"></circle>",
        categories = "social,emoji",
        tags = "crab,shell,protection,water,intuition,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacCancer,
    #[cfg(feature = "zodiac_capricorn")]
    #[strum(props(
        svg = "<path d=\"M11 21a3 3 0 0 0 3-3V6.5a1 1 0 0 0-7 0\"></path><path d=\"M7 19V6a3 3 0 0 0-3-3h0\"></path><circle r=\"3\" cx=\"17\" cy=\"17\"></circle>",
        categories = "social,emoji",
        tags = "goat,mountain,ambition,earth,discipline,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacCapricorn,
    #[cfg(feature = "zodiac_gemini")]
    #[strum(props(
        svg = "<path d=\"M16 4.52v14.94\"></path><path d=\"M20 3A17 17 0 0 1 4 3\"></path><path d=\"M4 21a17 17 0 0 1 16 0\"></path><path d=\"M8 4.52v14.94\"></path>",
        categories = "social,emoji",
        tags = "twins,duality,communication,air,adaptability,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacGemini,
    #[cfg(feature = "zodiac_leo")]
    #[strum(props(
        svg = "<path d=\"M10 16c0-4-3-4.5-3-8a5 5 0 0 1 10 0c0 3.46-3 6.19-3 10a3 3 0 0 0 6 0\"></path><circle r=\"3\" cx=\"7\" cy=\"16\"></circle>",
        categories = "social,emoji",
        tags = "lion,crown,leadership,fire,confidence,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacLeo,
    #[cfg(feature = "zodiac_libra")]
    #[strum(props(
        svg = "<path d=\"M3 16h6.85c.162-.012.19-.323.03-.38a6 6 0 1 1 4.21 0c-.153.05-.125.36.038.38H21\"></path><path d=\"M3 20h18\"></path>",
        categories = "social,emoji",
        tags = "scales,balance,justice,air,harmony,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacLibra,
    #[cfg(feature = "zodiac_ophiuchus")]
    #[strum(props(
        svg = "<path d=\"M3 10A6.06 6.06 0 0 1 12 10 A6.06 6.06 0 0 0 21 10\"></path><path d=\"M6 3v12a6 6 0 0 0 12 0V3\"></path>",
        categories = "social,emoji",
        tags = "serpent,snake holder,healing,knowledge,astronomy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacOphiuchus,
    #[cfg(feature = "zodiac_pisces")]
    #[strum(props(
        svg = "<path d=\"M19 21a15 15 0 0 1 0-18\"></path><path d=\"M20 12H4\"></path><path d=\"M5 3a15 15 0 0 1 0 18\"></path>",
        categories = "social,emoji",
        tags = "fish,duality,water,dreams,empathy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacPisces,
    #[cfg(feature = "zodiac_sagittarius")]
    #[strum(props(
        svg = "<path d=\"M15 3h6v6\"></path><path d=\"M21 3 3 21\"></path><path d=\"m9 9 6 6\"></path>",
        categories = "social,emoji",
        tags = "archer,arrow,exploration,fire,philosophy,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacSagittarius,
    #[cfg(feature = "zodiac_scorpio")]
    #[strum(props(
        svg = "<path d=\"M10 19V5.5a1 1 0 0 1 5 0V17a2 2 0 0 0 2 2h5l-3-3\"></path><path d=\"m22 19-3 3\"></path><path d=\"M5 19V5.5a1 1 0 0 1 5 0\"></path><path d=\"M5 5.5A2.5 2.5 0 0 0 2.5 3\"></path>",
        categories = "social,emoji",
        tags = "scorpion,stinger,intensity,water,transformation,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacScorpio,
    #[cfg(feature = "zodiac_taurus")]
    #[strum(props(
        svg = "<circle cx=\"12\" cy=\"15\" r=\"6\"></circle><path d=\"M18 3A6 6 0 0 1 6 3\"></path>",
        categories = "social,emoji",
        tags = "bull,strength,stability,earth,endurance,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere"
    ))]
    ZodiacTaurus,
    #[cfg(feature = "zodiac_virgo")]
    #[strum(props(
        svg = "<path d=\"M11 5.5a1 1 0 0 1 5 0V16a5 5 0 0 0 5 5\"></path><path d=\"M16 11.5a1 1 0 0 1 5 0V16a5 5 0 0 1-5 5\"></path><path d=\"M6 19V6a3 3 0 0 0-3-3h0\"></path><path d=\"M6 5.5a1 1 0 0 1 5 0V19\"></path>",
        categories = "social,emoji",
        tags = "virgin,maiden,harvest,precision,earth,analysis,astrology,star sign,horoscope,constellation,celestial",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ZodiacVirgo,
    #[cfg(feature = "zoom_in")]
    #[strum(props(
        svg = "<circle cx=\"11\" cy=\"11\" r=\"8\"></circle><line x2=\"16.65\" y2=\"16.65\" x1=\"21\" y1=\"21\"></line><line x1=\"11\" x2=\"11\" y1=\"8\" y2=\"14\"></line><line x1=\"8\" y1=\"11\" x2=\"14\" y2=\"11\"></line>",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "<circle cy=\"11\" cx=\"11\" r=\"8\"></circle><line x2=\"16.65\" y1=\"21\" y2=\"16.65\" x1=\"21\"></line><line x2=\"14\" y2=\"11\" y1=\"11\" x1=\"8\"></line>",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
