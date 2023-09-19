
use base64::*;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideIcon {
    #[strum(props(
        svg = "eJxtzUEKwjAQheGrPLLP6HQykwhJb+AhShQUFKS40Nub6MJCyyze4v9gcr3O9XbGXBw71Fcba/suLrgx7351zI/pecGpuDsn8AHsozdwF70suiJBvEBJ2w4kBiFdw2OgIYAD6aRQ7L9nlBJsA7NQbP/iUvuu/V9/AN67Mq0=",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[strum(props(
        svg = "eJwlizEKgDAMRa8SshdNRHRoO7t4CLHFFBykBNTba3T5vA/v+ZpXhbMklYA0Ikgum+jPV8AO4f62vocx+saC6I9FBVLAmQYgFse7Y+gdO2oNpsFUk+IDUowaRA==",
        categories = "medical,account,social,science,multimedia,shapes",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "danielbayley"
    ))]
    ActivitySquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJSMDTK0DXJ0TVWsPSxVDAGMzyMlOxs9EGK7AD2QAsc",
        categories = "medical,account,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "colebemis"
    ))]
    Activity,
    #[strum(props(
        svg = "eJxtT7sOwjAM/BWrewzn1E6GUImNhZU9EkNGBtTvx1EQZKj8kHxnn87lVd+NnpflbgS5rVVI6OyBIEEe+p/J5wabAZJd5wOSFmTZyqmLbmWSzg1HBDJHQuJUhZV6DaXIMIqcY8e+bTBgrO7rUM3Yf1C267CEXp5pD/rb/gCvNjbM",
        categories = "home",
        tags = "air conditioner,ac,central air,cooling,climate-control",
        contributors = "karsa-mistmere"
    ))]
    AirVent,
    #[strum(props(
        svg = "eJxNTb0KgCAYfJXDXer76Gcx55bWdiHIIFRIgt4+tSE5OLjjflQw0WKbxNKDxrkzDEabQJIlr/2vkbSloTbAN7V1A2wlCa2avKpV8Oeze4fgDxevSRCD8g2YUKgYJf8l9Qs3UyJk",
        categories = "multimedia,connectivity,devices,brands",
        tags = "stream,cast,mirroring,screen,monitor",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Airplay,
    #[strum(props(
        svg = "eJxtzUEKgzAQheGrPGaftDOTJikk3sBDlLTQQgURF3p7DYIudDXwvh8mld9Q/h+UKRMLoczrVcKQKVKTbhs3qX+NX7wztQ8oBL5a3Q7pZJ2NGj1T661GcLQBDsIXAQfrXS18gNwvm+4Jrq8FzridF6m+Mts=",
        categories = "devices,notifications,time",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmCheck,
    #[strum(props(
        svg = "eJxtjjEOgCAMRa/y4061FSsk6A08BImDi4mD94+ARgfJb9/U1zYc8dywTs2i5EZkRAeHDpybSfRmM4c2z87hNdiTB1uS4VFSjCceCv7CLgI1vekru7QccqQjLIRrMlK6VBXbZgnfjxfqOjWs",
        categories = "devices,notifications,time",
        tags = "morning,turn-off",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    AlarmClockOff,
    #[strum(props(
        svg = "eJxti0EKgzAQRa/ymb1pZyZNUki8QQ9R0kILCiIienujbgRdfXjv/Zj/fW6+6BMFQp4TsZadygrV8bbrOnbv4YdPohcLnqNtBJte8UE+oBC4s2ml4EorvTg5owEcjIeF8EXA3ji7Fs5D7sdmAZNjMvs=",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[strum(props(
        svg = "eJxtjUEKhDAQBL/SzN3szoybZCHxBz5CoqCgIOJBf6/Bgwc9NVQVdEjDksYOaYvEQkj7uUpYInmqwufSVZibtUcbqf5BIbDZZXabSU5caKFPVVujHuyNQwnhl4CdsWUurIN835s/WPv7+ACDmDIA",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmMinus,
    #[strum(props(
        svg = "eJx1jUEKgzAQRa/ymb2pM2OTFBJvkEOUtGChhSKl6O1NcOFCXX1478EP+TXm9xNjJE/IcyTWslNZoT5cVt2H7/034BEpXaEQ2Ooq28xHCm600b1K1qgHe+PQQfggYGdsVwvrIO1JI+D2f/CcbmAdNrEA98E5Xw==",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmPlus,
    #[strum(props(
        svg = "eJw1i2EKgCAYQ68yvguEGeQP9TIlKYiKCentUyMYY4z3ZDZHQW6KVsLjzmIVMUGoijjBGnfZ8j1tPrkOUstleFqm6Jt3wSBFF8rdSQaOXiMbBNg+5w4+pB/XL+MyIPU=",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[strum(props(
        svg = "eJw9zDEOwCAIQNGrGA7QikPTAb0McTAxHZzw9oVinX4ID4jb4F4DSwZMEHh6hyZCodP3hXp7apDk24kZbs0aBb+qNvVbXNbQ5TdW/3FE3P4FIW4h/Q==",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[strum(props(
        svg = "eJxFjFEKgCAMQK8ydgBrI6wP7TYRgqhQH3r7TCd+Pfb2NpOiL3cMkKIL72NxV4cGBtKKtkpm6GYoWUw922a6xtMs8vk03oULMlskRihk8aiQMVNjzf9qtCTLFqmVpNf9vHL0HzZ+LeU=",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertOctagon,
    #[strum(props(
        svg = "eJxtjDEKgDAUQ68S3H9tfj9aoQoewAu4CQ4dFBzE80sHdZFseclLx3JmrH21K10bwChRaItC4UskOIvwm0TQxreGQZnZfEOUv4S5GlJdpEN61BMV3WX/gG12ni+7AUBZIpk=",
        categories = "notifications,shapes,development",
        tags = "warning,alert,danger,exclamation mark,linter",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    AlertTriangle,
    #[strum(props(
        svg = "eJxtjysOgDAMQK/S4Atts/BJxjQGi19ATCLIzs+KYIMsNf29vtSe/gpwzM0qwBKEGmc77Tn7TpiA+2i8gAClYEzZ0pc1SkRTR8ctk6SbBUl6KdZAeZz8dQaUv5RrUpOkw07ILbeTgpg+yySoNIM3FO0+Iw==",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterHorizontal,
    #[strum(props(
        svg = "eJx1j7sOgDAIRX+FuKNAGh9JdXZxdTc6MDoYv1/awaqpYYKbc0/w+3IobH0xsYCcQsXgq3Ab/J20wDS6xXIgG0ZBmeuVkEsuO7Q7iroMyLWRmkiCSKY9NCm6rFNobN7OE+Uj5R+pU04oxMeeVWDShF5EPj45",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterVertical,
    #[strum(props(
        svg = "eJxdzEEKwCAMBMCvhHygJKW1hehveiiUnvX3ukY8eFrCzsa+938oa+SdqUjkkym3UGmn4ky2wSTrEkTUOySWwScSFovuGPTy78hu72kraIsgtg==",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[strum(props(
        svg = "eJxFjEEKgCAQRa8y/AtEQwiB47pNh4iUdBcipLePEcrte/99m8NZqAkY9CRfosCAqmAB5dpxDOmKRTAbODvp3tlefWLtwTyKcdTU/tV9lEhesDMT88ZqlLkXE70l1g==",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[strum(props(
        svg = "eJxNy0EKgCAQheGrDO8C4SBB4HiDtu0jJd2FCNnt0yJpM4t/3meS3zKd0eUgUCMoFQGD3ht83EMW1H4JNKwZ2t6aR9WkND48dftTpfWujjUHcoKZmZgXbp/W7A0XhCXk",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[strum(props(
        svg = "eJxtjEsOQDAURbfycudCS3XSZwem5oKomUjjs3tK/JJO77nnmKlrHK2MDLQxFMh2Q28dQxzLdAAJWobWWUaOwsT+X5jTeubXSeBb4orpO/BoY+0stYxSaJJyjpRHfvwjXckAuJQ0SNTH2AHHCju4",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[strum(props(
        svg = "eJxdjMsNgCAQRFvZTAMi8XNh6cAijBDhZgjx072LB0I8zrx5Y5LfMqWboUHBxz1kRj+AruhyYEwgYZIfxghrurK3prWEzO2+vqjPlbOqHWsO5BhLr0ifWhVSuoboH3kBgf8tgQ==",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[strum(props(
        svg = "eJxdjMsNgCAUBFt52QYEgnrh0YFFGCHCzRDip3vFgx+uOzNrkp8yBR/nkBlSg9LOUKCD0YK26HJgdKBr1bCmKb41/0rcel/p79mTLWMO5BiDJrUqUUCZPkBW5ARgFS1X",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[strum(props(
        svg = "eJxNzMsNgCAQRdFWJq8BhfjZMHRgEUaIsDNk4qd7hYTgdubcZ5LfhG6GBl3RSWBMoODjHoShBtDDGEGpEGu67K2plZr+2UfnSttG37JjlUCOsShN+tTlk2/2BTE7Jig=",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[strum(props(
        svg = "eJxdjMsNgCAQRFvZTAPCxs+FpQOLMEKEmyHET/cKHjSeJnkzb0zyc6Y9uhwEPegQMCg9EXxcQhboFnQKOljTlL011brRgI/7t1S90/xq65QDOcHITLyxKk1h9gI8nyYl",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[strum(props(
        svg = "eJxtjLsNgDAQQ1c5eQEg4tPksgFDIBJx6VB04rM9SRooqCzZ79mmsCpdjBGUchiQhLiJMroedEavUsebMcDZpvDOVitX0xfJevdz077avqiQZ8yGzGHqUCr3AAk9Jfs=",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkM0NbO3FBDedhAX19prONFPM+76k3UhTPtQEvQOVW8Cgb2fQI5hAV46mghHBd7UP/txMKQqWgZhXrlCvH7Br5QUy2R2e",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[strum(props(
        svg = "eJxVjMsNgCAUBFt52QYUFL3w6MAijBDhZgjx071gjMHrzuzo6JZE8WRI0BFs8owB5F1YfWKIHpRZB7oYCkY3xTf6eWUgVOW2dSH7I97wd9vm5Mkypo7kLtsCylQBKf7kBllxLVQ=",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[strum(props(
        svg = "eJxty7sJwDAMBNBVxC0QpEBwIXubFIGQOt7ekn9gcHXo9E7f57spc8QF+iVC2NLOE5TF26SHm6RV9teQPmRp1HK1tQzYbsK0BWXfIKQ=",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[strum(props(
        svg = "eJxVzUEKwCAMBMCvhP1ASUpbD6m/6aFQetbfG6OCnpYlQ1a/938oy40TlCx2UObWLIQRdasmqsuJsDTDB/yB9dU6Cv0YOr7GzLAFZ4kgrA==",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[strum(props(
        svg = "eJxty0EKwCAMRNGrSC5QkkJrIfU2LgRxrbc3juLK1YfhjeZUomv800OuCVItt8VGYQp6DRMUco0wH+HIMp/WswXy8/Mu67ftZ74grg==",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[strum(props(
        svg = "eJxNjFEKgCAMQK8ydoFwRBA4v/vpEJHS/AsZVLdPLcGf8djem01hV3gYJ4QrepVKEuIhymgyp5uREPIc0dmh+M7Wqllzn/769zGz6apzUwHPuBIBLVQOZeVeBgklpQ==",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[strum(props(
        svg = "eJxljEsKgDAMRK8ScgE1lILQ9AYeQmwx3UkJfm5vW0EElzPz3rgcFwWJaRVltAhHCiqMI8LZ8sU4GIRcEqF3XeW9a9YDfNzCmvdhsD9rm1UgME4EtFNfh1r5GwAmJdE=",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[strum(props(
        svg = "eJxtzDsOgCAMgOGrNN2NgiILOLt4CCPEshlCot5ewPgYmJr071fl7RLg0CgQdmcCaWQdwnkPsm6loLFH8PGG46DqBAaV2QOaDEr36bP81DYHAqNx4hwkVSKVtPsVCXJkhX0ULJK2kEQsI3/DBcDBO38=",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[strum(props(
        svg = "eJxVjEsOgCAMBa/S9AICQd0AN/AQRohlZ0gT9fYCxg/LvpmpSWFh2KNnsig1AoW4ElscEA6LPcJ57ylfCp3pSuBMzX5utnQtxkd9n4qv2mYm8BYnBUqQqqRsDZENuQCAhC1l",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[strum(props(
        svg = "eJxVjMENgCAUQ1f56QIKQb0AGziEEeLnZgiJur2gkcixfX3V0a+JjuASGwgFOg0G0PWGmJMEsQ8bJ4MRVndFsPrR8kr9aZEn1Lf+O6jWviQmZzBLEoplX0jpGtKAG0HuLTs=",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[strum(props(
        svg = "eJxtjEsKgDAMRK8ScgFt8LNpegMPIbaY7qQE1NubFsSNqxne8MaXtClcjCNCsSAESXkXZZwQbkZnceaoYnXA4LsqBN+0l/d/roG5PdBnHasKRMaFwJFQX5fKwgM0XSYa",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[strum(props(
        svg = "eJxNjMsNgCAQRFvZTAPqxs8F6MAijBCXmyEkaveyRKPXN++NSWHNdFl0DJIQN8kWIyidFoUc0WcpYw8qYIAzjQbO1OyRftnrt9WfoM/8VfuShbzFzMQs3OqizN04qiYX",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[strum(props(
        svg = "eJxNjEkOgCAMRa/S9AJi47Ch3MBDGCGWnSFNlNsLRKO7P71vU9gU0sVICBLiLso4IZRgRMiMfTFn9CpFDuhsVwFnG/bbPw+5mXdv2s/8UceqAp5xISAhU4sauRsVtiXt",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkE1t4yHEBDedhAX19prOdMO870vajTTlQ00wgq4cTQW9A5VbwKBHMIO+PSH4rvbBn5spRcHKTOwWrlK/VoYfvDusHYI=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[strum(props(
        svg = "eJxVjFsKgCAURLdymQ1U2uNH3UGLiJSufyFCtfvUQOxvmDNnVHB7pFtjArHzB0eNGfRoDKm5vI2c4ggKaSNgVJcFo4r2dXXVIx8txZbtXbXOLTJZjVWQGFj0meTuR2QLXkFoLTg=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceBetween,
    #[strum(props(
        svg = "eJxFjT0OgCAMha/y4k4FCsiAJB7AQxAcGB28f2xZSNPlfe+nvO0beM7tdgdFON8tAgXDlJBNRr6CyPpWzkEskWK3RnFQC8XG4AlVtVLA8BLIsw+Ot1p2nalljSVBY5EfpI0eCQ==",
        categories = "text,development",
        tags = "and,typography,operator,join,concatenate,code,&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersand,
    #[strum(props(
        svg = "eJy1jbENgDAMBFex6B2cDyYghWzAEFEoKCnYXziA2ACd/osv/tJRzp22pVu9kI+VlQPHm7mAQGJ4GkiqEJyyvj223UVLoAe4yTZlOFhrl1PfznP6FMBPigsqKSkM",
        categories = "text,development",
        tags = "and,operator,then,code,&&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersands,
    #[strum(props(
        svg = "eJwljLEKgDAMRH8lZBdtRHBoO7v4ESUWWigixaH9e1NDwh3HPc5yrlwicHNoCIG7ww2hOlzR21lbb0u+IzRSphn1LnkXk0g06EF5+4Q3weXw3MDQQcEsIK9HImn6pwfmP4OEIK4=",
        categories = "transportation,text,maps",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Anchor,
    #[strum(props(
        svg = "eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vZpbopcDLN4b8bY2dllhD0bRpLBXrGdL8Fak0femq0/JgwN66gElTunTHHJixA8CXKQXmqVKdQgAf2HKxXQkP6FEkvtV5NIAFJfcgNEGjN7",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[strum(props(
        svg = "eJxtyUEKgCAQQNGrDHOBGikwGL1Bh4gpmKBFiAu9vYorwdWH/1jeIN8DkhySQZDcG2pW9Lx09/xfUeF2eFqgXW2jtgY41Ew+bQMUShEgiQ==",
        categories = "emoji",
        tags = "emoji,nuisance,face,emotion",
        contributors = "karsa-mistmere"
    ))]
    Annoyed,
    #[strum(props(
        svg = "eJx9zLsNwCAMhOFVTvR52Ipx47ABQ0RKQRMpRZT5kQuoEO19ut/e6yu4z5AZxFBwSLb5mKzRo06y0D4w4hlOnvlYBVpIBuTR+McuFZG1LR8=",
        categories = "devices,multimedia,communication",
        tags = "signal,connection,connectivity,tv,television,broadcast,live,frequency,tune,scan,channels,aerial,receiver,transmission,transducer,terrestrial,satellite,cable",
        contributors = "danielbayley"
    ))]
    Antenna,
    #[strum(props(
        svg = "eJxtkEEOhCAMRa9CeoAOBUVIwMsQFyZmFq7w9tNadcaJq0/a/5r/yXVe6zKZtQBZMHVjdaxt1zG/dD/mZX5PprkCzqLtwWxUILLwhAZMHZjGE+rQk2Bi/4UIaVBLwpBuePz382pAH9UkaeQCeZSnAA5teGC+hyko4zH1ioRnRONqiYNxGP115U5ccfYlhjOZxXhU4Zr8Eyf1AbIjVgw=",
        categories = "photography",
        tags = "camera,photo",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[strum(props(
        svg = "eJxljE0KgCAUhK/ymAukItJCvUGHiJSeu5BHP7dPaVO0mYH5+MbXvAgdJQkHGAWqZ2vQk1eABXEuK0uAdoh+6EL02yxMKWDSiuxuO+jTCxgauR3+gfsIN1GKJM4=",
        categories = "design,files,layout",
        tags = "application,executable",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    AppWindow,
    #[strum(props(
        svg = "eJxFjksKAkEMRK9SzD6xUybzgXFAXHsBd8O4cOnC+2O6G5SQBOoDb33vnxeel+FuBIsufpgGCqhTwLSM8P7OKY4y5xqVvLouhnZKG5sQh1QLRWrHXQIUE40x9WkWSgj37NRer2XE07oR5sg4MedJBnaIkM7Q3mPY1lMF3tY/dqYOQzKzTfwyX+slLWo=",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Apple,
    #[strum(props(
        svg = "eJxtzEEKgzAQBdCrfP4+NDOpUCHxBj2EVOm4KBQJtr19ExeiIH8xM8zjx3l8ZHymIVuieuJbBvFLDMRcDiFsnJ6WExt28VJ9F999NgyJ9ytui0ivUPg1ZTOtsJIdVH+U4qp0J/TVQhoEF1By0iQK0aXdPn/5FTOm",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ArchiveRestore,
    #[strum(props(
        svg = "eJx1i7sOglAQRH9lMj3Krm7E5C5/YGtPhLgUJobc+Ph7uTbQkGkmc+akabhlTB+nEO+xz+HUmpgHJWIY75GdRnydB7ZpX/5tenY50DsvRzQvkU6hqP+ZW4iuh0qvTTGLs5iP884gJ1hlW1BhWOAPETUsUw==",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[strum(props(
        svg = "eJxNizsKhTAURLcyTP94uVcFi8Qd2NqLitdOJPjZvYmFyDTzO36bhogzUIkrsCCOZYyWsiO21Ath0zJbDKzY+H/+N37to2EMbEvUu0ivULhHyZnot/hpV2cyMx9SHEStfJcb/LkkIA==",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Archive,
    #[strum(props(
        svg = "eJw9yiEOwCAMQNGrNPhmKdCsouMGswgcCaICgSCcn2Cw/z8ddRq0z/0BwiIxEpf0OTXpfS+QX2zks3RkYIwYy3UblaES0g==",
        categories = "charts",
        tags = "statistics,diagram,graph,area",
        contributors = "nstokoe"
    ))]
    AreaChart,
    #[strum(props(
        svg = "eJxtTrsKgDAM/JXgHjTns1A7u7g6uBUcOjpIvt920RZKhnBPzt7+CXStzS6GzDF5EKhLx2Bsc44J2jfOtini7BfsSUTH3xg/ggw5wVDOHDxQp7FcGSV5VupHkkVREeLgQnkB4E0xNA==",
        categories = "furniture",
        tags = "sofa,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Armchair,
    #[strum(props(
        svg = "eJxNiiEKACEQAL+y2OU4Tk8WVrPFahcMGwwGMfh6tYhMmxmqqTFkK8KrQXsUjp6tHN0B+8eqSANGLlhF5H+cdQKREBMR",
        categories = "arrows,navigation,gaming,files",
        tags = "backwards,reverse,slow,direction,south,download",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigDownDash,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTRVMCszyzDJ0TVXMNcFwgyTMLMMsyolOxt9kCI7AAXWDAM=",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,direction,south",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigDown,
    #[strum(props(
        svg = "eJxFiiEKACEQAL+y2JdD1BNhzx9ctQuGDQaDbPD1atEwZWao5c5QPvXrANqloCI920W6xa3CaMRW9OhhIZaNvOPME8ayE9Y=",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigLeftDash,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNbRQMDTN0DUrM8nRNdc1VwDiMpMMszKzKiU7G32QMjsAG7YMkQ==",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigLeft,
    #[strum(props(
        svg = "eJw9yisKQCEURdGpXOzywvPDgavZYrULBoPBIAZHrxbDLovNPY9KxYmoCdMIz98Vz89BqH/SzZKVpylVQMJ65wZ4SRKn",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigRightDash,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVOwzDALM80xVzDXBeIyXRMPszDLKiU7G32QGjsA+EYLmw==",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigRight,
    #[strum(props(
        svg = "eJxNiiEKACEQAL+y2Jfj8DxZWM0WHyEYNhgMYvD1ahGZNjNcUxPITkWCl+RXnp+tPN/BdNTBFIsWFoJf14HGeSelxRM0",
        categories = "arrows,navigation,text,development,gaming",
        tags = "caps lock,capitals,keyboard,button,mac,forward,direction,north,faster,speed,boost",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigUpDash,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXytVQwtCjTNfMwzTHXNVcAwgxdkzIzD8sqJTsbfZAqOwAMiAwN",
        categories = "arrows,navigation,text,development,gaming",
        tags = "shift,keyboard,button,mac,capitalize,capitalise,forward,direction,north",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigUp,
    #[strum(props(
        svg = "eJxtjbsKgDAMRX8lZC/aWuvS+geu7qJiOghSio+/txHEDhLIcM992G2IBJPDtQJpQPMJja0tGLT2xV0DquwzEOYxQrgcKoT0NcLpUNYIh58iPQLNfqHo0HCK/Vmd5L5dGBLqZ0zWidK3dgPbji4U",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[strum(props(
        svg = "eJxtyzEKgDAMheGrPLKLtlZd2t7A1V1UTAdBSkG9vekgOJQMgffx23NOjNXR0UL1MPkqQ97WGbz9eBygm6kEaoAS4UqXsBPkXxa3JeFxpAwhyteEK6yJHclwy94ReAs7J0d9znLgX83oLfQ=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown10,
    #[strum(props(
        svg = "eJx1jDEOgCAQBL+ysQfvkEML5Ae29CQWV2hi4f8jFMYGM9lsMdmNV7kV+zqcEzjAN4wfUhybSPHV2wxHuSccYVEjHcMCphysFGcFLVRh1M5MPwOvchhB0O/xAQRkKyE=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownAZ,
    #[strum(props(
        svg = "eJxFyUEKgDAMRNGrhOxFW7rIIu0NPIREQUFBioje3sRCyyw+zGPZsuwL5IhuQJBH67Xv38R98cTndK0wRxydB7rJyK4GB4FSsHWh8gccxhp6",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownCircle,
    #[strum(props(
        svg = "eJxlyaEKACEQRdFfediXZVycRRjNFqtdMFgEg/+PWDTIbfdIz6OiOBXJ4gtGeXnX8nJAQ1P6b2kMMuDVw5sns+EWXA==",
        categories = "arrows,navigation,files",
        tags = "backwards,reverse,direction,south,download,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownFromLine,
    #[strum(props(
        svg = "eJxtijEKACEQA7+y2MtpiuOKPWsbHyFY2AgW4vvdLbSShBCY4Z5HpfKbBPLI3pFURqPfBH5UCbzFBoK4zt5g+gRGTPsetgC6/RlO",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftFromCircle,
    #[strum(props(
        svg = "eJxFy7EKgDAMBNBfCdlFqlAytJ1dXN3FFtNBkBJQ/96GDp2Su8e5kg4BTvlk8WgI4clRuL2vxxmh1DMhfBqCG3UQ3L0LQ/R4GQs0EJCSlp3WSsYutHX7AcjtHrk=",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-west,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftSquare,
    #[strum(props(
        svg = "eJxNySEOACAMBMGvNP0AqTpzVGOweBIEEsH/Q6po1u3wzLtlVe0GiQzqLHGd2QwN49sDricPOQ==",
        categories = "arrows,navigation",
        tags = "direction,south-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownLeft,
    #[strum(props(
        svg = "eJx1yTEOgCAQBMCvbOiNLF7E4uQHtvYmFteYWPD/EAqoIFOO/k82vKf7NnCHVIu4pGuNpK2viODvUZAQm8RhcRwMRt+rAIoRJN8=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownNarrowWide,
    #[strum(props(
        svg = "eJxtiqENACAMBFdp8IR+RVVhA4YgQSARhPmhBkX+8ubOZluDeg4VQiINTJd7Po7gUCx5U+yV4iXB+Vvo1hH1yQP8WBnp",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownRightFromCircle,
    #[strum(props(
        svg = "eJxFizEKgDAMRa8ScgGpgmRoOrt4CLHFdBCkBK23l9ih/O29/3xJu8LLOCE8OaowOkIolXFEkJQP0YaqfYIfLAj+2lQgMp4E/8wY62Z1M9BNS1cfggYebg==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-east,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownRightSquare,
    #[strum(props(
        svg = "eJw9ySEOACAIQNGrMC4gJAqSLR7CzWBxMzjPLwbZfvpPV9sDesYpIMDkoWl61/RbZcfDVCTsArN7D2Q=",
        categories = "arrows,navigation",
        tags = "direction,south-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownRight,
    #[strum(props(
        svg = "eJxFyz0KgDAMhuGrhOwirR0ytL2BhxBbTAdBSvDn9jYKSobA+/D5mmcBzmVhCWgI4Qo4IBwlCb/hfEJtz2L0vQ6i3yZhSAFHY4F2UtD0w0rQyOl17uMb6I8ewg==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[strum(props(
        svg = "eJxFikEKgDAMBL8Sci+SIJRCkx/4CImCgoIUEf29KSJlDwM7k4/xXGASHIiBL+pRc1c/zb/ZKUEKEWLwNW1rsW2GIkgIdjvY+Qgy1ejT+gIL+Rqf",
        categories = "arrows,navigation,maps",
        tags = "direction,south,waypoint,location,step,into",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowDownToDot,
    #[strum(props(
        svg = "eJxlyaEKwCAUheFXOdjHOI7dMbgzr1jtgsEiGHx/xKBF/vZ/WmPLSJ/xtOATLuP0HM/plCIgIaNDdvZ8YfnfSzq25haJ",
        categories = "arrows,navigation,files,development",
        tags = "behind,direction,south,download,save,git,version control,pull,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownToLine,
    #[strum(props(
        svg = "eJxtyzEKwCAQBdGrfOwlWbNoio03SJs+YGEjWIjnly20kikfI/VvGekx5QJ5sGbZRDkUokx+A9z5baA4wm1Zw+6jAO7klwzAeR6i",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,traffic,flow,mobile data,internet,sort,reorder,move",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownUp,
    #[strum(props(
        svg = "eJxtzDEKgDAQRNGrDOnFTFyMxZob2NoLFtsIFrk/IUVIs/zywdf/qYb3DN8G7pDeIqHo2qHo4CsjxdsDEmKMvhyWfWCyOWuEkCTf",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowDownWideNarrow,
    #[strum(props(
        svg = "eJxtzDEOgCAMBdCrNOwglBYdkBu4upM4dNDEwXB+y2BcyE/TJi/9+a6PwLGaK0JIQD2WTMlTh5I/3magFtIAAgMJn5YhCY8dfbPRcUWnt47XBNC9ox98oOoi9i97AUmrK5I=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownZA,
    #[strum(props(
        svg = "eJw9yUEKABAQQNGrTPbSTDSpMTdwCGVhoyzk/FjQ3/0no8wGNZmMBGGhNyruPpUnHSMgWQa2p+8b3MgPwg==",
        categories = "arrows,navigation",
        tags = "backwards,reverse,direction,south",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDown,
    #[strum(props(
        svg = "eJxFyTEKgDAQRNGrDNuL7iKSYpPaxkPIKigoSLDQ25tEUKb4ME9tjbbNiJ64IdidKqlXadD69aDHeC6YPA3cgaV32fL3y84CV7Uo+/gBLIcaew==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "previous,back,direction,west,sign,turn,button,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeftCircle,
    #[strum(props(
        svg = "eJxtySsKQCEURdGpHOyPx/UHwtUZWO2CwSIYnD+ioEl224t7HhXFi+ZgP4udCPyvH/hoVCBZST9EEsglc2UCmzUWfQ==",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,expand,fold,horizontal,<-|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftFromLine,
    #[strum(props(
        svg = "eJxtyzsOgCAQhOGrTOiNLm7AYqW28RAmFhSaUHD/8CiggEw3X34JT/R4T3Uf2MGwH4OVk7X8Tppm8WRG+MlAE3ipm4R6A9mrSwLUyx6o",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "wojtekmaj,ericfennis,karsa-mistmere"
    ))]
    ArrowLeftRight,
    #[strum(props(
        svg = "eJxFyzEKgDAMBdCrhOwiqSId2s4uHkJsMR0EKQH19poKSiD/w+O7khaBI0dhj2QROOWV5e2Xxw7hrL88YTC4VgfB7bMwRI8bGbBND/WUFX6eaAAyo/3kBvFjHsM=",
        categories = "arrows,navigation,shapes",
        tags = "previous,back,direction,west,sign,keyboard,button,<-",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowLeftSquare,
    #[strum(props(
        svg = "eJxlycEKABEURuFX+bOfpjsGqcsb2NorCxtl4f0TxUZndz5uqRdkJ4IE2aiE53cuzxsqSehHY3VzMKCv0H9kAKk7Fnw=",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,collapse,fold,horizontal,|<-",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftToLine,
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvDLvIBjIGc9niIwSDRTD4f9Sg4dLprKtDS24gAYpnz3A403DH9H1BAaQc/2zIPw+Z",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeft,
    #[strum(props(
        svg = "eJxFyUEKgDAMRNGrDNmLJoh0kfYGHkKiUEFBigu9vW0FZRYf5qmtybYFyRN3BLtzJfeqDdq+HvSYzojZ0+jAEl2hcv2ws4AH9E3d5w81jhq1",
        categories = "arrows,navigation,shapes,gaming",
        tags = "next,forward,direction,east,sign,turn,button,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRightCircle,
    #[strum(props(
        svg = "eJxlyaEKwCAYReFXudjHuG5uC//MFh9CMFgEg/j8osEip51PSqgJ8Vf+gmm8lZVzLCsLNEHt3l0yDfjhOWbLO9QHFsQ=",
        categories = "arrows,navigation",
        tags = "next,forward,direction,east,export,expand,fold,horizontal,|->",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightFromLine,
    #[strum(props(
        svg = "eJxtyzsKwCAQRdGtPOyHZMygKSbWabKIQAobIYX7xw9oZXGrw9X/zRHfZRI7HBAI1UzQrUHQwY/d4e8FpBOWqV+0+gTsI7spBbwbHoY=",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "karsa-mistmere"
    ))]
    ArrowRightLeft,
    #[strum(props(
        svg = "eJxFy0EKgCAQheGrPGYfoUW4UG/QISKlcRGEDFS3ryko3u77eb7mWVCPQJbAuSwsgYwj3NIR9pKEXzgVom/1EP02CSMFGh2MZadB6Q+rsTAD+ubZ1y8Bgh79",
        categories = "arrows,navigation,shapes",
        tags = "next,forward,direction,west,sign,keyboard,button,->",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowRightSquare,
    #[strum(props(
        svg = "eJxlyTsKwCAURNGtDPYhjPkWL9ZpXIRgYSNYiOsXBW3kdvdIcjnAf8ryAfV/KCN7e0aGRBJ8cW+91a0mrsJzSgXNKRbx",
        categories = "arrows,navigation,development",
        tags = "next,forward,direction,east,tab,keyboard,mac,indent,collapse,fold,horizontal,->|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightToLine,
    #[strum(props(
        svg = "eJw9ybEJACAMBdFVPvYiESVNzAYOIVikESzcH7XQ4pp7Mtsy9OJqBkWj5FTCfSpPBkVkMNifPm/J3A9l",
        categories = "arrows,navigation",
        tags = "forward,next,direction,east,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRight,
    #[strum(props(
        svg = "eJxtjksKgDAMRK8yZF+0tX4WrTfwEKJiXAgixc/tbQtSF5JNmJfMjNl6xxgtrQUaaKHhh1qTBb01L+1q6ENWCezT4HBb0oRzGR3HbfeCIlyWZEngaZnZWYpf4f5jJ2uo/BAVC/UTJktPOdV4ANVkLgo=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp01,
    #[strum(props(
        svg = "eJxtzDsKgEAMBNCrDOlFV+On2N0b2NqLirEQRBY/tzcKgoWkGMhjxi5tEPSO5gwVOGLokbfx/ff21boEb6b4AVPCJA1LlP5hriifvXXoAg5HJifIMI0SHBWEfeqDOGLCqaaxaj6Dd8FfzPQt6g==",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp10,
    #[strum(props(
        svg = "eJx1jDEOgCAUQ6/SuIN85CMDcgNXdhKHP2jiYDy/MBgXTNM06Usbz3IJtmU4JgQ45VA1pDi2PsWXrjPcTb4DrEEQxR1CDDLZay5WM5pNFaFmJvMzcMK7Ynj5Hh/2jSsX",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpAZ,
    #[strum(props(
        svg = "eJxFyjEKgDAQRNGrDNuLbgjBYpMb2NrLKigoSLDQ2+sSMEzxizeiW9Z9gT6R2BH0Ls1fOkrSFk9yTteKOdLBAewab4O3h0n1gR04jP0vLzw1GtI=",
        categories = "arrows,navigation,shapes,gaming",
        tags = "forward,direction,north,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpCircle,
    #[strum(props(
        svg = "eJxtyyEOwCAQBdGr/OBJu3QDFVtuUFvfBIEhQRDODwhQZOTLSP5LRHhUMgSymsG6p7wcQ7xMf8nBnN9G0oV7XOhtPgeuZBc0vRUeog==",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,mobile data,internet,sort,reorder,move",
        contributors = "it-is-not,karsa-mistmere,ericfennis"
    ))]
    ArrowUpDown,
    #[strum(props(
        svg = "eJxFijEKgDAQBL+yXC+SgAbh7n5gay+noKAgwUJ/b6JI2GLZneFjPBdMQnuDDqEKSCHlOv/KP+2dh2sHX4it0bYZUcgR7Bbyua80X+nD+gDZxxoy",
        categories = "arrows,navigation,maps",
        tags = "direction,north,step,out",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowUpFromDot,
    #[strum(props(
        svg = "eJxtybsJACEQBcBWHubHsf5QWO3AIgQDE8FArF8MBAOZcLjnUVGCaOTgP7vBisj/jsinE0moSfoxBpLqNQu2hxbl",
        categories = "arrows,navigation,files,development",
        tags = "forward,direction,north,upload,git,version control,push,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpFromLine,
    #[strum(props(
        svg = "eJxlijEKwCAUQ68SvEBrhtLhV+gBXLsXOrgUHLw/RkEXSciQ9yy/JeG7XCTOh+lwwbb2BRvkJwi/qyuLXvDuEBpFKqdXAcBbGGM=",
        categories = "arrows,navigation,maps,development",
        tags = "outwards,direction,north-west,diagonal,keyboard,button,escape",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftFromCircle,
    #[strum(props(
        svg = "eJxNyzEKgDAMBdCrfHIBqULJ0PYGru5ii+kmJaDe3gYHnZL/Hz+0simk1F00kmPCWbPK+16RJkLrZyTcFlIYbJDCsaogR5oZzi8sbGTlj5zvBsZnD9BWHqw=",
        categories = "arrows,navigation,shapes",
        tags = "direction,north-west,diagonal,sign,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftSquare,
    #[strum(props(
        svg = "eJw9ySEOACAIRuGrMC6gJMovN7Da3QxEg/efUNhr78Pdz+kMnkqiS106G1peQ5kkUlT2AbzUD1I=",
        categories = "arrows,navigation",
        tags = "direction,north-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpLeft,
    #[strum(props(
        svg = "eJx1yTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lQIqyO/+0/t4DOfmrgUrZBL8uaxz+Vmr7gnyMnaABIPJQKKlvgRv9I0+vWIlMg==",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "lukesmurray,ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowUpNarrowWide,
    #[strum(props(
        svg = "eJxtirENwCAMBFd5sUDiL1w5SBkgQ0SioKRAzI/dIAr0p2/urP29ojzpIyF85YbjFyOYsl2RZNvDSJ2DFAWrDl1qApTBGP4=",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,north-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowUpRightFromCircle,
    #[strum(props(
        svg = "eJxFi0sKgCAURbdyefMIC+IN1B20iEjpOQhCHn12nzpxdD+HY3PcFa+jmZBLTISvjScFFUeGCRLTIdq6t2MVvL02FQRHK4OF70bq18nJMAt46OgHg+8emw==",
        categories = "arrows,navigation,shapes,social",
        tags = "direction,north-east,diagonal,sign,keyboard,button,share",
        contributors = "danielbayley"
    ))]
    ArrowUpRightSquare,
    #[strum(props(
        svg = "eJxNybENACAIBdFVCAsIFc2XDRzCxILSwji/0cKQXHUPs6+gUbkZWahsFXaUex3J9GXfDsEAD2s=",
        categories = "arrows,navigation",
        tags = "direction,north-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpRight,
    #[strum(props(
        svg = "eJxFy7EKgDAMBNBfCdlFUqV0aPsHru5ii+kgSAlY/17SQbnljsf5mneBJ+CEUFtAg9D6uEsSDkgOgXM5WHqPftRD9NcmDCngSRbIDLMGZnWV3xcyQHZ1n7z3zh8a",
        categories = "arrows,navigation,shapes",
        tags = "forward,direction,north,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowUpSquare,
    #[strum(props(
        svg = "eJxli7sJACEQBVt5mB/H+g9WO7AIwcBEMBDrFwNBkMlmGO55VJQgkoGqpEXkf6vIJzTyIPXZDezbE0m4eZ0LxYoW5A==",
        categories = "arrows,navigation,files",
        tags = "forward,direction,north,upload,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpToLine,
    #[strum(props(
        svg = "eJx1zTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lYLQQKabV3y9j8dwbu5asEImwT+XdS5/1qp7gryMHSDBYPQDipb6Ery1zge34CUy",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpWideNarrow,
    #[strum(props(
        svg = "eJxtjLEOgCAMRH+lYQehtMiA/IGrO4lDB00cDN9vHYwLuTS93MtdudotsC/mjJCBLIHK1DK9eS0fXWegHtIABAYSPixDEh5z9N1Gxw2dej2vCqB/Qz9ooNIs9h97ACo5K2M=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpZA,
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvDLvIhDGEuR9Y7YLBIhj8P86gXDxZbQ/o2U0CjMCewTiVcEPldbHEVOnPAbejD1g=",
        categories = "arrows,navigation",
        tags = "forward,direction,north",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUp,
    #[strum(props(
        svg = "eJx9yzEKwCAUg+GrBPdSUkWXV2/g2r3QwUXo4P1RB51UsuXjl//NEd+tkoGFPjTqlJez/V66Bge6ZwKJ+46rMBhcjLRDChiqJYo=",
        categories = "arrows,transportation,mail",
        tags = "direction,orientation,this way up,vertical,package,box,fragile,postage,shipping",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowsUpFromLine,
    #[strum(props(
        svg = "eJxljMEJgDAQBFtZUsCZjXp6cKYDixB8+BF8iPWrCOaR78wwfiznhnUKMxP0YgrZm5dlL2YQmsKgMsYO7Otm/5SBUVp7Tn9yA/iVGVE=",
        categories = "text,maths,development",
        tags = "reference,times,multiply,multiplication,operator,code,glob pattern,wildcard,*",
        contributors = "mittalyashu,ericfennis"
    ))]
    Asterisk,
    #[strum(props(
        svg = "eJwli0EKgCAQRa/ymb00kyYu1Bt0CLGgoEVISN2+NP7i8Xk8n/eSjxUlkCHkJ5CMH+/O6IdfR3+ma8MSaBYLV6ekocF9FlyVJGFIuwJWBq61rYkv9/IYhQ==",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[strum(props(
        svg = "eJxNj0sOwjAMRK9iZW/TuGlDpbQ34ALsUEACiQWqWMDtybhK6GbysV7mJeXHmp83yp/ZeXWUv9u6lsUt6bCNl/S6vO90nd1JO1FCZJUucIleOuUo/chBBvZeJmwCQnmSY+QRxzoAEggcF5AAUrkmzLEJCCWABLANzvCBx87GDxIJka3NcGszvPlYm/maJ4Trq6ZpvaZpvdx8AJIJA4Rv/cbf5gfnI0Wl",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[strum(props(
        svg = "eJwly0EKgCAQQNGrDLPXmLFJBccbdIiwoMBFSIu6fUmrv/i8VI5W6gZNcUIotyLx10cxYE7Df3M6l2uHVXEmsaP3QGxDBPLAXI0YZwQckBV2JlqiTjvJL8RUGAo=",
        categories = "account,sports,gaming",
        tags = "achievement,badge,rosette,prize,winner",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Award,
    #[strum(props(
        svg = "eJw9yzsKgDAURNGtDPZPnfwDMSvQRQQsUihYWLl6o6DFvd1JRzkr1qnbaUAlobdoFdVT4d0IgqJFzyRil9PwkJw+uNCCGhF+M2Lg4KouAaHBRsXDXz+6AZMwGeM=",
        categories = "tools,gaming",
        tags = "hatchet",
        contributors = "Andreto,ericfennis,csandman,jguddas,karsa-mistmere"
    ))]
    Axe,
    #[strum(props(
        svg = "eJw9ybkJACAMAMBVgr34EJImZgOHECzSCBbi/GKh7Z3Mtgx6cRUBdyJL5FTCVZV3AyFHYM+/DrXGD2Q=",
        categories = "design",
        tags = "gizmo,coordinates",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Axis3D,
    #[strum(props(
        svg = "eJx9T0EKw0AI/MqQu1bdmnZhG+gD+oiwPeRYSP9P3aWkOZQgKOrMOJbX/F7wvA2PDLWFRYepnNpwKttK/WAn0LGyc4KyscPAvio7scGI/R8nY+Q0Z2RIhAbzisQ5uPadSPQjNgTpJVrZAygA952EIVXrvHDL2mv4Wanphh1ruVJcEur2zr38fvoA9VM6ug==",
        categories = "accessibility,people",
        tags = "child,childproof,children",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Baby,
    #[strum(props(
        svg = "eJxtjL0KgCAUhV/l4C7p5SoG5tzS6tAmNDg2hM+fEZSI3OWen+/4M10ZxyI2BqmoVWIwVD0Nlpxdq8GlFgj0GrJ+q221pF0EPz2bwX/LM2zkv4Zay9RqUKEB50C6SNOR3JFmSGqV3ThwTXAD6no85Q==",
        categories = "gaming,photography,travel",
        tags = "bag,hiking,travel,camping,school,childhood",
        contributors = "karsa-mistmere"
    ))]
    Backpack,
    #[strum(props(
        svg = "eJxtzkEKAjEMBdCrhOwbm9pJu2jnBl7A3YCCgogLF+PtTTSMDkohaT+vadttup/g0HG3pTpAJUlThgxRF0OmUoOWAp9MqNh2jbwsWXyzJfAhKxSUDN+Twr/nojHZ49g29tWxXc7XI8ypIyeEB3es2vw486urNeXWDAv6HYrsXn78Ex5TOIM=",
        categories = "account,social,shapes",
        tags = "check,verified,unverified,security,safety,issue",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeAlert,
    #[strum(props(
        svg = "eJxtjz0KgDAMha8S3BsTTX+G6g28gFvBwdFBPL+tlmqhBEJ4+Xh58Uc4d9imbhnRaXBohiAgQLEYBK1TsVn4NIM2jTWUW9HoxYqQTSpIRUT/nVTrHCXMrN3s+xR19iUwD2AvptZGowBTfoQfJyncDdeeMrc=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    BadgeCent,
    #[strum(props(
        svg = "eJxtjksKgDAMRK8S3De2Nf0ItTfwAu4KLtwILrw/NlirBQkMYfKYTDjSucE6dfOA3oBHqxMBgcyjgNB5kcXB61l0vLZQkerJG6tGCWkgkRHzTRJ/7yRjduli6LlqDE/hfQSlgYcE1fMFaJYpxQ==",
        categories = "account,social,shapes",
        tags = "verified,check",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeCheck,
    #[strum(props(
        svg = "eJxtjrEOgzAMRH/lxJ40SY3jIWXu0rUDWySGjAz8v0hEZEBCliz7/Hx2WvNWsHyG39vKCLEcMoHganiQjWJqijg1trGVd6gn1dyBqdBNbpCpyHh1Mk/nXMN4Hqb0aq9OSR/2DCmGc0DoN6tXIe2bQl952gzw8med7DbPNnI=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,usd,$",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeDollarSign,
    #[strum(props(
        svg = "eJxtj8EKgCAMhl9leHepOTUw36AX6CZ08Nih9yctsQQZjPHv498/f8YrwbGybUZH4NCoqEGDyCVBo3U8NwufZtCWsYdqa5p4sSZUkw7iGaG/Ex+dEwUzOwt+KlGDb4EtSJVosJAEC+r6hnx8CFUDb9hNMrY=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,€",
        contributors = "danielbayley"
    ))]
    BadgeEuro,
    #[strum(props(
        svg = "eJxtjkkKAjEQRa/yyT5lhs4E6dzAC7gLKiiIuHDR3t6KhrQNUtT0eTXkR31ecJrF3lJ0iORNnTBBsWlMFKLkELBqnkIrt1APQ1NfbAh9yQaSjLjfTfLfOdUwfxAl79qrJY+HE6mEVC1sRx1FC31UMJLF5uvU7Xo/YzGz0IaUFnhpLoPA0rLh3nx65htZ3g+OOd8=",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeHelp,
    #[strum(props(
        svg = "eJxtjz0KgDAMha8S3FMb7U+G2ht4AbeCQxfBwftji6W1UAIhvHy8vLg7PBHObdpXwRpYmCUoUCBTEShhGVOz0DQjbB57qLSqyQ+rQjHpIEyI/jvh6JzMmDkm7+Yc1bsamIEjD3VaRouLViCLGilS/TEVNvYF3tE6rA==",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,inr,₹",
        contributors = "danielbayley"
    ))]
    BadgeIndianRupee,
    #[strum(props(
        svg = "eJxtjkEKAjEMRa8Ssm9saqfNojM38ALuBhQURFy4mLm9aY3VAQnkh8/LT8pjfl7gNOJhTzKAUApzhAheiyFSFqctw9dLlOu4hax1z7+xbljIBnKKDL9J7t85X7F0xKns6qtTuV3vZ1h5RE4IS1ANCKvpwk0VrpjBZjZIPjvkGVuMdPoF5Eg4VA==",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeInfo,
    #[strum(props(
        svg = "eJxtjk0KhTAMhK8yuG9f09S0Qp838AJvJ7yFG8GFeH5b/McSCGHmYzJx6ucB/2/VsQ41ghbbOziYNASnfVBpeVyaaJ/PJ7SvUzMbdgp7yANSCanvSar0zmRMflUbP7lqG4/CY4MABi++4JEFEVjx2+sakB2kbMjNWAHz+T9x",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,jpy,¥",
        contributors = "danielbayley"
    ))]
    BadgeJapaneseYen,
    #[strum(props(
        svg = "eJxtjk0KhDAMha/yyL6dtlPbLmpvMBeYnaCgIOLChd7eVMUfkECS9/h4SRyrqUVd0u8rQ4EgnaksLBSXhpU+CG4el+ekz+sTOtrpqR07jSPkAQlGinuSeDunMub+lOInv5pi3w0NZlOSdoRF8zSEmWdgaTbJbKbSCpcqLMU=",
        categories = "account,social,shapes",
        tags = "verified,unverified,delete,remove,erase",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeMinus,
    #[strum(props(
        svg = "eJxtjksKgDAMRK8yuG9stU1bqN7AC7gTXLgRXHh/bPFbLIEQZl6SCdu0L5i7amjJGTjiZtLQkLEUNFknYrN4NSabxhy62qPJE3uE60gGiYiY7yVReicTxmPVhzpF7cMdeFUGXjD4bw0efiGpCk5cUibzDhktODI=",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePercent,
    #[strum(props(
        svg = "eJxtzkEKwyAQBdCrfGavVWvUheYGvUB3gRZaKKWLLszt46AxCQRhcD6PmYm/6f/CI9HtKsOAIJ2ZLCxUeRpW+iBK8dgyJz1/j6iVnqnKetCGHJAoZNhPEmfrFDN3pzFe+NQxft7fJ7JJpA1h1okCIevWcuzYslptNXMzuZpqTbcLQ/U3yQ==",
        categories = "account,social,shapes",
        tags = "verified,unverified,add,create,new",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePlus,
    #[strum(props(
        svg = "eJxtjjEOgCAMRa/SuFMBoWCC3sDVwY3EgdHB+8cSCUpCCG3z+/rbcMU7wbkM24TegkfS0YAByU+BQecFBwefRuhy2UIlVE2+WBWKSQMJRuzfSfTWyYzRMaxhzKeuoR7sQelkOg3Fc7TPaKNGC/m/Vpz7NpRcbTxHfDr+",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,british,gbp,£",
        contributors = "danielbayley"
    ))]
    BadgePoundSterling,
    #[strum(props(
        svg = "eJxtjksKwCAMRK8S3Mf6/4D1Br1Ad0IXLrsoPX+Viq0ggSFMHpMJZ7oyHCvZJHUaHDUiKVDAynBQ1DosYuHzDLV1HaEm3WMv1o0WMkBYEP1Pwtk7VjGzkxiWWjWGXtgDN1nPDyLrJEC0LgxVRnn7zj59MTQT",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,rub,₽",
        contributors = "danielbayley"
    ))]
    BadgeRussianRuble,
    #[strum(props(
        svg = "eJx1jj0KwCAMha8SumuN/wXrDbp26CZ0cOzQ+1OlYhUsgRBevrw8d4U7wrlOm6BWgaWaBwkSWCoESY0lqRn4NE1NHnuotKqxF6tCMekgkhDVOpHRO5YxfUzezTmqdzUwIqDZbZQ/Ox7FYLMA6ubkAbZuOB4=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,chf,₣",
        contributors = "danielbayley"
    ))]
    BadgeSwissFranc,
    #[strum(props(
        svg = "eJxtzk0KwjAQBeCrPGafmMT8CUlv4AXcFRQURFy4aG9vho61LSUwEx5fJlPe/eeOa6XzUeeArKPrPTxMOxZep6xaSfhnUSe+rpGUOTMTmwMZskKqkbCcpPa+M8zihbpy4FW78ny8bhhtpRNhdJVsIAxWuuO4UUZCB6GbF1P/0S/dIDd7",
        categories = "account,social,shapes",
        tags = "verified,unverified,lost,delete,remove",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeX,
    #[strum(props(
        svg = "eJxtjjEKgDAMRa8SujdWbZsMtTfwAm4FB0cH748JlmpBAp/wePwkneU6YF/MOiMHYIxT8eDByYzgkdhKELwsIunaSzUac4/WQC3pJCtK+DbZv3NOtbiZnAZ9Nd/PhiHB",
        categories = "account,social,shapes",
        tags = "check,verified,unverified",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Badge,
    #[strum(props(
        svg = "eJxVi7sKwzAMRX9FaA+11JCkYHvu0jV7cExt6FCM6ePvK6XgpggkdO859r7UBKvDCzPQdB4WBgYjQx13PI/tN/qjtwc1vG0ejUD93P9zqaN9APwg83NLDBWeea3JIR0RUszXVB1OCK9tFzmE8HY4qKW8tyGXcIsQtBMmSMtGWDkKfeuGbXGDVDrtqA+mQ0Au",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[strum(props(
        svg = "eJwlikEKwCAMBL8S8gBbxUsh5i8lLbSgIOJBf6/Rw+4cZkj+IvGFEtCeCNIm3WRfZDq2Z8p3/eAJmLy5QGe9ceu0UssDJmwUSg==",
        categories = "account",
        tags = "cancel,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,circle,slash,null,void",
        contributors = "danielbayley"
    ))]
    Ban,
    #[strum(props(
        svg = "eJw9jrEOwjAMRH/l1D3GduI2lUKXzvwAWxUGRgb+X1wAVZEdyz6/c3sd7yce1+lWYLlnieSoDFP4ERIYoXyGipi2dhkbWzv3Qixgi9S1U+rJmFBljjSzhyWZ78ZmgA78QXaG08qd2J9BEK6Y6V6Ec1JUyspiJ95Apf+Tdh0WMLGSvtjMA5JTdj+v+wBeFCzP",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Banana,
    #[strum(props(
        svg = "eJwti8ENgCAQBFu5XAEqPIgPoAOKMAfxSHwYQiJ0Lye+ZrO7Y0uiCt2hQXhyrOxQbwhtAKFMcMonV4dKo7erCN5SLnQloP7VQG2yiDBOc/b2PipDdBgMKM3LpoLa/yQ3mf0LfackMg==",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[strum(props(
        svg = "eJxdjLEJACAMBFeRLKAGEYvoNhaCWOv2agJCrK74+6PeRjXLZ0AHZh76dIjCdemgkL1aIZZ5RBkD6C9+LgeCBKN241M3lWIhAg==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart2,
    #[strum(props(
        svg = "eJxtyyEKACAMQNGrDLvIGOKE6Q2sdsFgNIjnlxXT6n982eMsmMU1ArrIC9lVCVqrfEMGTD1bQirREF2upy8PSfIeAA==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart3,
    #[strum(props(
        svg = "eJxtySEKACAMAMCvDLvIGOKE6Q+sdsFgNIjvlxXT6p3scRbM4hoBXeSF7KoE1Sr/kABTz9awTjRG43r68wBJcB4A",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart4,
    #[strum(props(
        svg = "eJxtjMsJgDAQRFtZpgFdP+ghSQcWISa4uUkIfrrXlYAXLzMwvDdmm7OQt5haancehUc4U+nqTApLpsuCa9BpMYDSUwySEFfJ73JEn8WiU0v5YhVQs/94bqB//Y92A3clJs0=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartBig,
    #[strum(props(
        svg = "eJxNjFsKgCAURLdymQ2EWeSHuoMWESld/0Kkx+67kkRfMwznjN2XwhQcZk36UIaVgbddXb3NcS3EMW1cHAbQ5TCBsoQCnSkUltaDboexWpVvVoNeo6HSfmciKf1ZD3HIJqE=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartHorizontalBig,
    #[strum(props(
        svg = "eJx1ybEJACEQBMBWDht49g/U4LQDixAMNjQQ6xcTI01nrNdBackVFZ2IRHTZvq3ZzgWB5yNA/Nfx1OML/ZsdfQ==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChartHorizontal,
    #[strum(props(
        svg = "eJxti0EKwCAMBL8S8oE2oQQPqb/poVB61t9roiiCp2V3Z/R7/wcy30gnQqKajJBrsnX2HvUwLKrDPoYJmXx1N2xYaZ+sBslAC5OSIQQ=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart,
    #[strum(props(
        svg = "eJxlijEKACEMBL+y2B93CUewiP7ARwgWaQQL/49aaGM3zIy23A0luPSDPyNxUd/lou5SBSSQhxiT+B6Sn9r8CQPOWxbq",
        categories = "text",
        tags = "text,format,color",
        contributors = "ericfennis"
    ))]
    Baseline,
    #[strum(props(
        svg = "eJxtzk0LwjAMBuC/EnJvXbJ+bNDu4tmr94LCBBEPIvPfmywKMqSUt02ftC339pjhVPEwQoLkI/Q+NpLU2elw5HzcB5+GHnoIAnQVIPh4pNwY2Jwkz8S/BcdPF3EqO31lKtfL7QwLVaQO4SUZERauOMhOIqtU85EqiHHtYJPMRqX8x452yGQ4W+v22vUD2YjmpvWL31ujQoQ=",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bath,
    #[strum(props(
        svg = "eJxljM0KgzAQhF9lyD00s/7kEnPupQ8htLAFW3ooom/vRkER2cO3A/NN+vV/xbNzDzaIyl4gCHY0ytge2dunXlxOt+LktJst4r3ei8GfxVCGlFfvQyL6Co3WQ8HRGN7fF2bpHCuHyShi5MbZyHWu1PICh7Eu/Q==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    BatteryCharging,
    #[strum(props(
        svg = "eJxljksKgDAMRK8ScgFtFd20XkaLLYgLKWhvb8b6gboaMnmPxGxujORdmH20rGqmw7Jm2nLsYYpe+o4pWe6lT+gHU8EbzBJWR0kL0YipZCfSoXMmmZUCDOyGAXXv7nM16j+Kjx4WzDVnp4BRtvkJZHHggU+3pD2O",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[strum(props(
        svg = "eJw9i0EOgCAMBL/S9ANaTPACfEaJkBgPhET4va1FT5tOZ1yJW4XucUW4816TR7IIzaNBSDEfqTKZEYqi0mWCm6QL7sxXhEbM+NeMbuebiJdvWkQWbcgDvpHVxv7Jpz7oJyc4",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[strum(props(
        svg = "eJxtjUEKgCAQRa8ic4GaAtuYlylJIVqIkN6+P4kuotVn/nszY6LbksorTaTusCe/EmtSsbyNd+HwCdWIqkoAC1kzyJ41Z7icygwClKeaBTMzEjPPIovWZJS6s+7KDf2jymdhkp+dJj/T4TJf",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[strum(props(
        svg = "eJxljcEKgzAMhl8l5N6tfyb20nreZQ8h26CCiAcRfXtTLUWRHpo/+b7Ej+0U6Rf4g4pclFZIyOqD/jLXX62M1kZTNMKNfyaj8cWryb2rollTtGOFaier74Y/LRJYhGlFYIBpQc7axyvBCctwbu4Q7CE53ndovLKFSUP3sMg37vwG5xU8EA==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[strum(props(
        svg = "eJwli1EKgCAQRK+y7AVqDepHvUxJCtGHCK23b1a/HjPzxtd0NtLAjqn2gZzKnVtgWVHNBcPB9JWrZfQ7R7/YL/qnvIlUIMFSN9mRRUBk2Uw2Lf4sFxxn",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    Battery,
    #[strum(props(
        svg = "eJxtizsOgCAQRK8yofezu0AsVm5ga09isaWF8fwKBaEg08zMy9M7P4Zrd4efA8QouKRLOZM2FCEvxcxgrDV/s63fE58yFMkbcSMfJSwcOg==",
        categories = "science,gaming",
        tags = "cup,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis"
    ))]
    Beaker,
    #[strum(props(
        svg = "eJxtjtEKwjAMRX/lsvfFJk1jC3PgB+wjxhQURHzwwf292Trmi7QkpL2ck+41vm+4nJqhoEwtmfptmZIwlaStkAYBE1s6GwxhPRkiI0cqhlrrc6HSKnHTd4cF23c7nAMdExKFEncMOwR5CnAHs6sZLs8utxxhZPYHlCgqw3Emo0I3sZHmo28pIQ/rH9bcqBQ4odZNSSGWpeoP/rg/r5j51EiDj3hbeh3nOnp0CfVfsYhCYQ==",
        categories = "food-beverage",
        tags = "soy free,legume,soy,food,seed,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BeanOff,
    #[strum(props(
        svg = "eJw9TrsKAzEM+xVze9yzYzs2pAelc3+gW7gOHTv0/6nTIfiBBJJQ/4zvG17X7UE7kikYavg9MFSgoTSHQBNwrAYBcZZkuYVQmaaqMMrOQNN9MzDY/+PAfDZslZOQFEM2n4BkLFUhQq+aYRnx3I5+mXWOvkopViHIasZDQGZUniK3KPl9WX6EayrH",
        categories = "food-beverage",
        tags = "legume,soy,food,seed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bean,
    #[strum(props(
        svg = "eJxtyzEKwCAUA9CrBHfp/0HEwXqDrt2FDo4dyj9/7SJFJEMg4eW7Pg3X7g6CYj7V3pAeBT2bxv8AWnIlbx8qedAAlTPOkpMMC6lEsLg4CE2NMp4XTUop6Q==",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedDouble,
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrPLoXk7RKhtgbuLoXHDI6SM+vLipF3vbg+22vh2Obw5Ig1KJWgYCuMSSKc/4ekKah2HCjYg8dwbROvaRO5h+ZwOr8Nk+HfSK9",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedSingle,
    #[strum(props(
        svg = "eJxtijEKgDAQBL9y5ANmF4knnPmBjwhYXGkh935jIynCFgszY3d7XK4jnZQ1UFK15UPVBqEObRRK7kN/BvK0xOacmSIa+89fUJMgDQ==",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bed,
    #[strum(props(
        svg = "eJxVkM2KAjEQhF+lyD29050fe2BGkL3uvsDehqygoCDiQd/e7gkiElJUmkrqI1M7Xttpj3afAwuVgPaYg7q5zsEH2+mrZ7bTZbkd8D+HX09Clmrqe/AVK4kgU22RiZGIU6SNmhmjHdSSg+4SUo+jgLVlM0rZbiiYzWRKu49n0bv+nMP73xRnVg9CiEfrLYaTvcVkvQdKDPHB2GWdRqGKQvLNhTJkILFiGPnGZHnhrTE1rFp/xHJc+0+sBE9+MkHY",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak",
        contributors = "kemie,ericfennis"
    ))]
    Beef,
    #[strum(props(
        svg = "eJx1TjEKwzAM/IrILjWSo9iBND/oVOjQzaSDx0JK3l8pLqHQFCH5zJ1ONz7zq8Dj3Fw4AnPhHCBAa8XWfUFupvHkomncpQOwrP0BweEv00EkndFckanrSDEA6YKCDn0wRbHHmjTbBO9PEtSZYnJMGitFeh3MCMRigyzilEX36U5YVfjrBRZjM7OLzmOV3Q9SK6SVJYsdabcyVNL3H+WW9sU3RR5Lvg==",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Beer,
    #[strum(props(
        svg = "eJxFjbEKwzAMRH/l0G7Vsowdg+2lS5d+RHALLXQoaYf272MnQxA6EPd0l9/z94Fboask9hDP6ewsO0jopxNI3PWiHzUOalKzRlnhOJqA0JcjLISVO8eJlWo+jdiaj3A7PmTuvscmdhvtJfbg23Nprzvar5BMhKWQEtq/0DSY3a0r+4sp9A==",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley"
    ))]
    BellDot,
    #[strum(props(
        svg = "eJxtjbEKgDAMRH8luDcmTa0VqrOLHyE6OAr6/3hV0EVCLpC8u+R9Pjda+2rSxIHUL5zI0J4jNY+OdpjzZK5bxBkbtq2LFNEKUsAHzMDgqyHXJXLIX7AUi87KHR4UkbvgIvnjG0pbfA8XfEgljg==",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder,delete,remove,erase",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellMinus,
    #[strum(props(
        svg = "eJxtjcEKhDAMRH9l8N5s07hNha6wNy9+RMGDF0HQ/8cawYsyYQgkbyavZZ8x/ZoxkUL+ERG+isEJqQQmgZk3UcS36fPnpPp8s6xgHWQTFyCuKy1FhdmVReKY9A30ll+YuhZmV49Q3Z7/S0CVr3PfDrAyLRY=",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    BellOff,
    #[strum(props(
        svg = "eJxtjTsOg0AMRK8yol/Hn2U/0oYmTZocAiUFZSQQ58dAQYMsjyXP87j9x2XC79l9pJJBIpWXMgkkUYT6zKe+bbagsFC/HMxRpRwSkreAIVRJfVcod0N77KlDu7J5P5DRqYhD+CjzH3zH9yhTujMK+vUyNuQDLUU=",
        categories = "notifications",
        tags = "notification,silent,reminder,add,create,new",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellPlus,
    #[strum(props(
        svg = "eJxtizsOwzAMQ69CZJeqj+vYgJulS5cewkiHjgWa+yOKh0wBIUoQH9uvb198HtM7o/SMDAkp1CCrYIajHvPyv5PBqU5Lux2lpZ1VFXaYduWaMEyGnOO64BPsaVwin2G4Dy8XnMV7FTJ24kIpllE+uR0G8yw/",
        categories = "notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "ericfennis,danielbayley"
    ))]
    BellRing,
    #[strum(props(
        svg = "eJw9izEOgCAQBL+yoQfvOIOSnNQ2PuKihaWJ/j8CBZnsZosdfey7cW3uSFgtIYEqDI6gk7BAkFt2ecVHiM+u6NSkokNlCoLIxiHP6EUdCXWN/w84fxis",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    Bell,
    #[strum(props(
        svg = "eJx9zDEKgDAMBdCrhO4qaVp0aHsDV3eJQoUOUhz09jbUxUGHkM/n8R1vmdMKfHmFfWsVZK9IPp+lGUoKrqsouF9sP22F+Gy+1D4fERavRtQgkxOa1FBDYMppoKhFiwo3wUouIw==",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[strum(props(
        svg = "eJxtjEEKgCAQRa8yzAVKEVfaDTpElDTuQqT09ukQYuBumPfeN8HtEZJFoRAef0SyWC5y/qRoUSOEAiVCZmUxUw0Ww1lm9xMS213Y1lp0bZHgsLhqkDMxqK8OCAViTDQIRfLW4+iPXkg9PK8=",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[strum(props(
        svg = "eJx1kM1qw0AMhF9F7H2n1v56wc6lZz9E2RZaaKGEUtq3z8gJJJDNwbKsn/k0XvrHsX++Sf9bnQYnx9Ux9n9+KZo7LE/ngcPy/fLzLq+r2wqqRKTu0SQgyyQZgVkQdp4LK439Kg1FglCmmIytX0W+Zi7rBBVFwnw/sGlF3DG3FH+hsJK8MrcaMcXUkAcczcYgyD8E7XoT5k714imc+LI0ofqM6Hd3VlKhhJWNO5Ki3Yj2O7S8sZW5yB+n3qxYiDINvQuvovXqyaJPHl/MIG8YmNwyhQ3cwYCZk/ZkuZ0+AQtodIo=",
        categories = "science",
        tags = "fallout,waste,biology,chemistry,chemical,element",
        contributors = "danielbayley,ericfennis"
    ))]
    Biohazard,
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/l1D3GdtLYQ+jMAGv3SAxdkBgQ308KqB0aWbJkn33vyrO+FtzPw00ybCGWYSqndTmVTYqUIH4RrQ4Hf8uDz1YT0m8ORupBKV4VykePhzIMChrD2joQ4cZ4x57S6EY2ztoLZ+2tZuR/rkiegjDl/fYDj1o5KQ==",
        categories = "animals",
        tags = "peace,freedom,wing,avian,tweet",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Bird,
    #[strum(props(
        svg = "eJxdj00KwjAQRq8Sus9nZjKZH6g9QXsI0YULCy68P6YRRNzNB4/3mPl5ed3T7TxtRDC1RIHicRUEC1w9KUiyonBLBCbtt4fseYw0xto6mchRxPaGKN7RIpxRpRsRthOa1uydk687/t1tuOvH3Ya75oqQDO3JBmoHwquDIwlYf2s95vmIbQbxxAVV5VFBrJkMxjYt8+n4d3kDLn40Kw==",
        categories = "brands,currency,development,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Bitcoin,
    #[strum(props(
        svg = "eJx1y0sKgCAURuGtXO4G6lcjBXXcpEWEBQYNQiJq972gJjY9H8fO3RKpd9xKkhGavS2u5O0LoqS6+QEgKzjFRJQZ0oQqQmREkVyhPghjCtNAYXOsmMLuGIYpOb7nR/0Bb582UQ==",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[strum(props(
        svg = "eJxdjbsKhEAUQ38l3H5Y47qsxczUNrb2ouK1Exl8/L2OjQ9ShBxCYqeuCdBu6DU4+QtWJ8wEy9AGPcEUiWBz8hVvP7Hv7VgHReukZIKUVV4TRBJlaFhk9wzOTC9wOPUJDGfze02cZ/HG70isJps=",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[strum(props(
        svg = "eJw9zDEKgDAMBdCrhOyiiWgd2p7A2V1QUKji4GBv72+xQkjIzyP2mu+NFseHIUPSoKqOukkD+ojEsLd1Qt6G/VwpqmNRpgdTBVOwD0xRcg6c2IcT6v9bti2XF4W+dVMgvw==",
        categories = "connectivity,devices",
        tags = "paired",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothConnected,
    #[strum(props(
        svg = "eJxlTDEKgDAQ+0roXvWKxyGc/YGru+DgoOAgvr/p0qUkIZCE+Ht8F841PGIQiwrdJd3VQ/axttnbJoGYyL7bZB4UC8Uj40HUn0kbFrcHG1k=",
        categories = "connectivity,devices",
        tags = "lost",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothOff,
    #[strum(props(
        svg = "eJxtijsOgCAQRK8yoQd3kV+BnEBbexILCk0svH9cY7QiM3nFzMtnvRq2SR0REUxS7eFXuwtnWaIqeXikkj91sWTSCHbC6uBAb7Q3IXR0TmDbDPH/3cd/HX8=",
        categories = "connectivity,devices",
        tags = "pairing",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothSearching,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNVcwVzA0ACJdUwXTMKMcIOkDFDFXsrPRBymyAwDtywrA",
        categories = "connectivity,devices",
        tags = "wireless",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    Bluetooth,
    #[strum(props(
        svg = "eJxlyiEKACEQBdCrfOzLqswuhtFs8QI2wWA0iOcXETTIq49ragXZiqAISicCQS6P8X83wvE7j+MzP2h5z7jrAIv2FT0=",
        categories = "text",
        tags = "text,strong,format",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Bold,
    #[strum(props(
        svg = "eJxNjUsKgDAMRK8Ssrea+kGh9S5SBQUFKS709mYsFAkvs3hk4sIWw75Q9Dwwhcez1Jq3pvDoyqRHd07XSrPnQwbTEhDTF8pkTUOgSlPUptlVdABS6FtZUgXd4x4PUPyrt5asatS3Wb82oCev",
        categories = "",
        tags = "fatal,error,crash,blockbuster,mine,explosion,explode,explosive",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bomb,
    #[strum(props(
        svg = "eJxdj70KhTAMhV/l4J7ctJDbCuob+AJuUgcHBwffH9NUHKQ05Pc7yXCu145t7OaQEKRwIjaH/z0EkRWyVusegn0hBauHliBWespvizVIEXAOTsoNdFBCKgY3hYanL749LRzrjInEWJU+CzidjE5Oyg20dNPwq7dMNyD0KbE=",
        categories = "animals,medical,gaming",
        tags = "health,skeleton,skull,death,pets,dog",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bone,
    #[strum(props(
        svg = "eJxtzLEKgCAYBOBX+XGXusOkwZxdWt2FBseG6Pn7CzIIueU4Pi7s5aiyLWalwGdXKJRRA6FlBUwMw21iaHISuPRKPNrVHqR+zgn4U2T/rRqr7QTbwwWrsiO7",
        categories = "development",
        tags = "read,dictionary,booklet,library,code,version control,git,repository,clone",
        contributors = "danielbayley"
    ))]
    BookCopy,
    #[strum(props(
        svg = "eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v81SEDkO7njw4l2eE47FbCNQcFItycpOQIstBJO+zFgZc9vlS9FKYybFQU0pdh8xkN/nP7kCEILXWN/xC535ISE=",
        categories = "development",
        tags = "code,version control,git,repository,pull",
        contributors = "danielbayley"
    ))]
    BookDown,
    #[strum(props(
        svg = "eJxljcEKgzAQRH9l2HtsdpsUhUToLZd+hKSFFioUKaJ/7wZBEVnmsMzMm/Dr/m88Iz0cuKn8aNjfpfIosnqMW/kSO2rDpaTbsHXEoh7ZJY10x5I1PondK/kz5O8LeYqkJOQ5Uk0YIknJrO4O7hUsxilNdd7tucFVR3izFjuLMrk=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[strum(props(
        svg = "eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCDmEfN//+1cqGaaANwt06aV2JFfuBXRNGwKnVySDoz8pPfrN4faXeo4NSUfJdBJ5p6zzo8AnIDFCnpdnLgEF4RvQIbyXqeSAA8KqiFrK74poAHe3iYH/6RZM5S3+BwFwMf0=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley"
    ))]
    BookLock,
    #[strum(props(
        svg = "eJxVTDkKgDAQ/MqQXt1dTUSIATsbHyEoKIgGFMHfmwQb2YMZ5rB+vBZMrRoqcJPrO2PdSa4Rj8IwTGS90C3UBzz+Vcp00JSzRWxy1h/bs637DH+s+3W2igmC8OKWqMEmQQNJoc/uXrRlIYY=",
        categories = "development",
        tags = "read,dictionary,booklet,library,code,version control,git,repository",
        contributors = "danielbayley"
    ))]
    BookMarked,
    #[strum(props(
        svg = "eJxVjLEKwCAMRH8luGuT0AhCKnRz6UcIHRw7FL+/ySKU4+COB0+f/g64j3DtQCXJjCQnJwEvWgiyv8Y4GZvt/qcYxViourmp6vIVIBx5gQ/5rhkD",
        categories = "development",
        tags = "code,version control,git,repository,remove,delete",
        contributors = "danielbayley"
    ))]
    BookMinus,
    #[strum(props(
        svg = "eJxljaEOwzAMRH/lVO4strOkICseKS0YmzIQMmlgyvfPntSS6iQT371XP89vx+s2rTP0LoOvvTQOBREKDmpXt9IiSRDiMFPyPKalXny51H3/5gwWeKxw/q8iyJt2ys1ZiJTw5yENTiZwqQnJhKS9DGvpgfkBY3kofQ==",
        categories = "gaming,development",
        tags = "read,library,plain language,done,todo,tick,complete,task",
        contributors = "schmidt-oliver,karsa-mistmere,ericfennis"
    ))]
    BookOpenCheck,
    #[strum(props(
        svg = "eJxdyzEKgDAMheGrhO7Bpgm6xM4uHiLgkNFBHDy9qUtA3vDgh09PuxyOtewN2GcTEKgxipebxBj4CxUZeWtP6ToN0zXloJi24s8ShPUl7QuPERw5",
        categories = "gaming,development",
        tags = "read,library",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    BookOpen,
    #[strum(props(
        svg = "eJxtjLEKgDAMRH8ldG9NgqkUasGtix9RcOjoIP1+k0UR5Di448HLZ7s6HKvbZ6AUZHiSjYOAFTUE0V5lHIxVd/tS9KLMlTyZqeTHl4Cwxx9ADMt4wQ3I/CA4",
        categories = "development",
        tags = "code,version control,git,repository,add",
        contributors = "danielbayley"
    ))]
    BookPlus,
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8luLc2p9EKVXBzcXUXHDo6SL9fu1VICRmSd5dcuM8n0jU3OxwB0aBZQpuXSygRS6rAnniycrDoRp86BbAnRKTKPT4mzYPsqYAq4VEl339sg5UTVii3+4rJGdm8GkqSsbL+1UOeCv0LdedZwg==",
        categories = "development",
        tags = "read,code,version control,git,repository,dashed",
        contributors = "danielbayley,jguddas"
    ))]
    BookTemplate,
    #[strum(props(
        svg = "eJxtizsKgDAQRK8y2Ed3N64fiIKdja29YJFGsJCc36QRJGGYYnjz3H08HudUbS14rDUY1kVqRSrFMLq0qtk16Tq7T+AB4iUIrfFx/B0yugqVJAHbvc/JNYIJ1ljEFLFm9AVAAS9x",
        categories = "development",
        tags = "code,version control,git,repository,push,force",
        contributors = ""
    ))]
    BookUp2,
    #[strum(props(
        svg = "eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v+lSEDkO7njw4l2eE47FbCNQcFItycpOoBU1BFN7mbEyZt3lS9GKMpPi0Ewpdh8xkN/nP7kCEIK3HjQdv54tISE=",
        categories = "development",
        tags = "code,version control,git,repository,push",
        contributors = "danielbayley"
    ))]
    BookUp,
    #[strum(props(
        svg = "eJxtzDEKgDAMBdCrhO6tSWmUQiy4dfEQBQcXwUF6fpNBUJAQSHifL2e7dthmtyagHLh74iUGBlvUIRjtqxF7xKp3+yp6VnNFBmsq8vQdlDQzeQb+wWwGb7sBl4ghLQ==",
        categories = "development",
        tags = "code,version control,git,repository,remove,delete",
        contributors = "danielbayley"
    ))]
    BookX,
    #[strum(props(
        svg = "eJxVjLEKwCAMRH8luGuT0BQKVujm4kcIDo4O4vebjHIc3PHgxVFnh/a5cgO9QZYn+TkIWFFD8NjLjIsx664nRS/KXIqXmdIG3dMR3A==",
        categories = "gaming,development",
        tags = "read,dictionary,booklet,magazine,library",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Book,
    #[strum(props(
        svg = "eJxNjDEKgDAQBL+ypD+8W05C4MwzLOwCFmkEC/+Ppgmy3Q4zcben49zSZQU0yeKS4fvaCEK/GSjspv8DPFKNZbg1ZqHAdDC4+MQvVPgXCw==",
        categories = "account",
        tags = "read,finished,complete,clip,marker,tag,task,todo",
        contributors = ""
    ))]
    BookmarkCheck,
    #[strum(props(
        svg = "eJxNy0EKgCAUBNCrDO4l/ycTQT1Ge6HAoKJFhHX6/LSJWTwYZsKRz4Ipqo08mLTTvXboR5sZDNNCYM2FzL8AXzQ8KoVO7imsyz6jUlRkFW7RNPmzNr1sZZVe6sIbBg==",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkMinus,
    #[strum(props(
        svg = "eJxNy8EKhDAMBNBfGXIv28zqFqH1M/YuKCioeBBRv962iEgOQ5I3fmnWHm2QSStQjTOFcSj+ZUMQNo6Chr3a9wHc9HdK7T+pXvtxmDscGsQJDgbRr2CPqzImc0ab1G3zs7yxldxNmXD12AuKaiYG",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkPlus,
    #[strum(props(
        svg = "eJxtzLEKgDAMBNBfObqnNqEhFGI/w8Gt4NBFcPD/UXGwg9x2jzs/2tmxzWHnAmEyymTIizaBIN1hCEnnNBaQNVSfnm317yFHhUUlhf5weRWjXoysHyA=",
        categories = "account",
        tags = "read,clip,marker,tag,cancel,close,delete,remove,clear",
        contributors = ""
    ))]
    BookmarkX,
    #[strum(props(
        svg = "eJxNy7EKgCAYReFXubj/5L1YEpiP0S40uAQN0dDTq5uc7cCXnvJWXIe7uUO0aMEiwrkWQfA9QqZKPw/o4/a7nJbBcwMwSBAG",
        categories = "account",
        tags = "read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis"
    ))]
    Bookmark,
    #[strum(props(
        svg = "eJx1i0EKwjAQRa/ymb3YGdqSQpIbuHVf0mACLiSEqrc3sVIiKFlk+O89fZtzwGLo1GM6D7NA0JXHkIMElnaArD1ZfayJ1XuooFb+sbP8A+M3SN5lPAwJ4R6XHMrVEZ6GJkLa9uDjJWRDLDWrgdUuJnf1SG/BFU+Vr1Q8VGeju/UBjc1jo70ANn1IOQ==",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = ""
    ))]
    BoomBox,
    #[strum(props(
        svg = "eJx1jDsKgDAQBa+y7AV0YxSFJLWNrb2ouOkkBD+3N7EwCNo9Znij1sEzTBo7ElD3sq3RqCxCo9w8ejg0SoTdTp41UoXgAhEIPNuFfUBhnxrvWzwY9RQFkGSReknkf4ZKoGL7Ms1bXMyeNBo=",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "ericfennis"
    ))]
    Bot,
    #[strum(props(
        svg = "eJx1jzEOgCAMRa/yw070FxhIkBt4CBIHRgfj+YVFMSnp0vS936bpLFfFsZk9wBWBYO1lW2dyWjrN6XUYB4nQJSEYP2uyKkA4SlaT2rlKdS5UAf0k0cAk4hBv6m/owIF+lviRB3Y+XYo=",
        categories = "text,design",
        tags = "selection,square,rectangular,marquee,tool,dashed",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere"
    ))]
    BoxSelect,
    #[strum(props(
        svg = "eJxljrEKgDAMRH/lcE81qdoOVfADXB3cBAcHBQfx+00FS0ECCVzyLhfO5dqwdsUoDL8IBFUsYmLj7E6O6kzVqRLqIUmw8HcGgvGCepSJkVOnDNN33M5FH8qYoA9fjsMaCwdvHJrYqfnfjKw+MrGk1QOWvyyv",
        categories = "shapes,gaming,development",
        tags = "cube,package",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Box,
    #[strum(props(
        svg = "eJx9kbtuwzAMRX+FyE5WJPWwATdA93jNkC1Ahw4q0KHQ95dS7TwVQ4AE6F6J95DTz/n3Cz7fd7PQmIBtlw8BAdeWAHuKWpTEn6/XzUqJs9ox3AhCLoI7sL0bCwYKGQMqenJqmpfTbj+91ZL7aS38bV9FCuZJHoWG8NIC9tWzOP+LJRCnjlqjKIUjj1lrbCF9DpwV7znMiRWw4D05rsKBraxrfKA9Kl6wuplXFTagZ94GS5XGW09vxpUgEmvxpClbrtqwY73pAGR8YMalFdhmeuq3coBEEsEi9cbUDJtIyzCGi/YHZ/qXkw==",
        categories = "shapes,gaming,development",
        tags = "cubes,packages,parts,group,units,collection,cluster",
        contributors = "karsa-mistmere"
    ))]
    Boxes,
    #[strum(props(
        svg = "eJxNjr0OgCAMhF+lcS/aGvxJkNnF1Z3o0NHB8PxSTIB0ucvlvp57witwb92xwLjPgYFh0MOkoi2e1ENxkNMrKUNm/ROhzrteed4VKk3AJFS5wMgRUxVzVbHI7Rvk07YzkAUr+QNcHSae",
        categories = "development,files",
        tags = "json,code,token,curly brackets,data,{,}",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Braces,
    #[strum(props(
        svg = "eJw9yqEOABAQANBfuelm58aEIyuqbhMuCub7US6/x2tsgZlNwwgkdDCJJVPYfSisnMBjDf0VxQsGrRD3",
        categories = "development,files",
        tags = "code,token,array,list,square,[,]",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Brackets,
    #[strum(props(
        svg = "eJydj0EKAjEMRa8SZt/YpE3JwDgn0O0s3BVddOlCPL9pFWS0gkopLT8/+S/TOV8KnLbDnhgiSmYUqNfX4yKOyWFMsJYJR4XwJoY6gqNVwl0DDBEERddW+40JAvpX3eIIveyMhf1iOIdhnjYVcZ6eoAl0kWMNNA5nVsc9GwOFEj9UtKRsnS2V7OUr9Z1atFNgb8yaURo6ATmq28pjExPA99nNQeH3xhbI9EeimuPrwBsYdnYr",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,ericfennis"
    ))]
    BrainCircuit,
    #[strum(props(
        svg = "eJx1kFFqAzEMRK8i9l9T25K3a9gN9AA9RNgWWmghhHwkt4/kfARn1xgjoxlGz5rX3/P6903nZZCB1usyxGT1VuthfnvIh/l0vPzQ1zJ8xkSKfEzI5Df4YUUZGTpS244oE8mmKR6R1BR59AiilJGn1mqvMpIgTB9t3xhieYUgh4DqTgZ7RjuOt+MqGVey7TdYtgy2B1+Rr+a5oP+Y8U4xQBkFumMoMHxB6ummjeQpXDP2HAETFU/oGSqDIJvMfYjgasdgYq4U3RmC6BCtfgd+jIV2",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    BrainCog,
    #[strum(props(
        svg = "eJx1zksKAjEMgOGrhNknpnlIC3XAA3gBdwMuXLoQz2+qIHRaCcniX4SvPrbnHW6n5VLIQc7SbizHJEgCRv5KvnUdjcqRzKCvEhWVOIPGfBupoZNn2D0mlfgiNvSSUXeKj+y6rPXQsGv9kZMNZv5jZpiZGUYzw9TMODe3PphDYT36DbcxRg8=",
        categories = "medical,science",
        tags = "medical,mind,intellect,cerebral,consciousness,genius,artificial intelligence,ai",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    Brain,
    #[strum(props(
        svg = "eJxNizsKgDAQRK8ybB80i58myQ1s7UWDm05CUHN7XWxkYIqZ91yOa0G+PTHha4lpl+LJdoTqaSRcaSvyfi0hV2WCa9QL7liKYPM02QFs535hMFqNYcNiuv8APu2grlrhAdzhHzw=",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[strum(props(
        svg = "eJxVjb0OgCAMhF/lwk6EhsEBfQNXd6PGuhlC/Hl7KYNgmrS95u6rD+scweu+cexUq3DtS+S83bmHNEjhEdH7Ruy9P6bIWDo1OFgzEQgmldWkaXRFI2n+aZBAJF5BrAMVihFXlTJCOXV1kDcf5gWQ/C4l",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[strum(props(
        svg = "eJw9jkEOAjEIRa9CZg9CoZ02GecGHqLRhQtNXLjy9P52oiF5tLQ82F79fafbeXk20UJm0qiKrgyUnqRmmlAyRIj6xGO+j5912bfTkOzbT3VZISALaXFlk1JI2cnEM5KLpmETd06ScciJE4agbNABhlu0oxRHSjKaArYKYoPoEBlN6Ah2HurPf50v+pwtgA==",
        categories = "text,design,tools",
        tags = "draw,paint,color",
        contributors = "ericfennis"
    ))]
    Brush,
    #[strum(props(
        svg = "eJxtkL0OwjAMhF/lxB4Tx85PpVKJjQFW9qgMXZAYUJ8fBxAdiLxEyfm7u4yP+lxwO+wuHJGJ5ZqqQODbuEisLhCfC8JuGvdNPI3bihIHCJUCTl1FCGBZnK4uVIV+sep0cUzSW/A0ZMTZw2xNkxreCUXov/oeYGNS3yFlylYoH3+2SGBepSY7fC6YrbAZcC8JN/TqSufJQHLq1RUEnn37MTBliz2842/RX2yzWlg=",
        categories = "development,animals",
        tags = "debug,code,insect,kill,exterminate,pest control",
        contributors = "danielbayley"
    ))]
    BugOff,
    #[strum(props(
        svg = "eJxtkDEOwjAMRa/y1d0mjp00lUoXFhYuwFaVoUslBtTz4xSBGCLHHuyXn++Mz/m14nHutoII4VKO0k3jqU6m8Tu/ibFE6EFkxAYxoGfRnWRWDkHxqQHikRF2aam6mswGc86DjGwl+2/Adp0zu4Tn0XOjxqkhljkphotxRuECrX68NlGIXltLKKIsgaLfFO5Jeag7kzXQGHjokRY3xELir1ZSOaFBb/55fgoSed5/wBtv/12L",
        categories = "development,animals",
        tags = "debug,code,insect",
        contributors = "danielbayley"
    ))]
    BugPlay,
    #[strum(props(
        svg = "eJx1UbsKAjEQ/JXBftdsdi8POG1sbPyI4yyuESwk3+/mRLCIBAbCPDKbnZ/La8P9dHgURAiXssPhPB87c56//E2MJUJ3RUIcKCoyizaSRTkExQcDxE9CaDJKjYhhJWUXUqLI2TE10sVg7u1uI9vs9w5rugbP126AG5D+ZTeqAyrxpKgX44TCBdqbO04jKUSvo3EVUdbgDXw+r61c+++QDaQxcM2YvLOrSfzVrlSeMFT7InQbBklmJ/PqMb0w923tz/4EvQE+O3JX",
        categories = "development,animals",
        tags = "issue,report,debug,code,insect",
        contributors = "danielbayley"
    ))]
    Bug,
    #[strum(props(
        svg = "eJx1jbEKgDAMBX8luBebRykt1M4urg5uBYeMDuL3a6YWacl0XC5JV7mFzmXaPAG7KyCQ/YYJBhJaJjwcjimnWaOcmpSx1tQaXfWVNRV0Qg4UBb8fsbIeEtMtLXlxfcF2aNzQhMa8YhVF7g==",
        categories = "account,buildings",
        tags = "business,company,enterprise,skyscraper,organisation,organization",
        contributors = "maxim-s-barabash,ericfennis,karsa-mistmere,jguddas"
    ))]
    Building2,
    #[strum(props(
        svg = "eJx9zV0KgCAMwPGrjL1XKjIMtBt0iKhovYVIH7cv6SWi9aTstz/zcewTxCOgQeBxnjhdX4WwzUPigJoQ9oAW4V6Je34aX+Wu8UuXGIaAbQ3GrIVlWm3WPH+oA+JS6Q/RJJP5I61+zMrX5I7kzsmZe1cnjHtd8w==",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[strum(props(
        svg = "eJx1jlEKgzAQRK8y7H9ad5umFhJv0EOUKl0/CkWCrbc3QVBB/Z03jxn/fUZFHehh4SC4UeXPOav8TLiAU7sFH0l9I4YX1DWviH8gS9CmfWsMxI4wBLoQfm0ddQq61JGsZWE1ZcGsqbA9UYKveip47587Zgnde9mTSgj3ZkEjr+lKXQ==",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[strum(props(
        svg = "eJx1js8KwjAMxl8l9J64ZFuXwbqLFy8+xKhChR1kytC3N2X+GagU0uRLfl/SnYdrgkNwewU/e9d3m6z03Vvn+k9DgCVxSz8hBdZUXqhGpoYUhTQSI1UkSGqBSaCwGrOquCijhQrrrRTE4MlM2pyYF/hdNQgYkx9aNnORys/qeJrieIQpOHEQ78Gx2n8LrskzS3d1YJvvq7/wF/i0MZz9in8AxldNzg==",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[strum(props(
        svg = "eJx1jTEOgzAQBL+yuj7EdxgiJJsfpE0fBZSjiBQhK4HfY0MBhWlntLPu+wyKztOdDUotDFPrrgm2blcWklcfQQMxlyq3ErA86uLgxv4VMHtiIYyTp5Kg/fDWEJEhRGIJ/6ELGkGddmlxaDYx+cueVWfGgm+6xVazAOBAQ5c=",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[strum(props(
        svg = "eJx1j7EOwyAQQ3/Fyk7KmR4IieYPujJ0Q+rA2KHi+wtLk0hEpxv8LFty+pRvxfuxPO+IhSBsPzE0zFp945GBr2VLtxHZ0j/ooNlNeLjgEiGa/arFrYrxo96aANtETlAGzHFWEkBphhOLcm0REqvx3d13oW+tJ33c+QNgjUiK",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley"
    ))]
    Cable,
    #[strum(props(
        svg = "eJx1zEEKwkAMheGrPLJPbGYkcWCmazceQqJQoQspUvT2dnDTjWSX7/HXeCwx3xHvRoWwNEqE+DRyGuvhh2N9Xl8Tbo0uLgkuBRmqa4kBYnKE9pvUQgwDlPuLdeVtwIkzGzufZs4bJ7Ee7sFdVg2az/mP+E6+jJUs7Q==",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = ""
    ))]
    CakeSlice,
    #[strum(props(
        svg = "eJx1jMEKhDAMRH9l8J5uM1Ttoet5L/sRwh56XFD6/aYIItISBpKZvEn/dc/4vYcvPahF4koQvo5Q+JnuN1jisKRXhZZ0oQE6bW4UBavcaEQw2WZMEN6986vRYplm+kYyIxY2fGUv6BEzQnZe213d6Ekd9QpP0Q==",
        categories = "food-beverage,social,account",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,fondant,icing sugar,sweet,baking",
        contributors = "karsa-mistmere"
    ))]
    Cake,
    #[strum(props(
        svg = "eJx9i2EKgzAMRq8ScoCtKUX6o/UGO8SYZSmMMaSwensTRVCh/ulX8t4LY3oV+OehcETqEGpEhzDKWIRpeTnlNxf5GuzDXYM+fPI3QbVrM8nq0DJVxqupztkURm51tpJ28u9ZGIaID+qADN8MKdPrntk289eZa2fuIvPt7IhmHW5aoA==",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[strum(props(
        svg = "eJxdj8EKwjAQRH9lyD3YHWPIIe3Zi1fvBYUKKh5E6t+babSVksMjw9thNz/654BT6w40WDjGniAaPU/P/e7/D74sLEEhh+S6vFFJl6+X+xmjtc6iw8jKd6FQYkqV9FVrNplpFjWf1qK6mloiytnWQdri/k65WQSn7RB8mPf7AMRjOUI=",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[strum(props(
        svg = "eJxtjlEKAjEMRK8S8i+aWIpC27uIW2xBRUrB7u03aXf3a8lHyMybIa7EZ4Uye2SEFPMrVY90QxDFIPzzVNMQmscrQmlKBnfWXHDv/I3QWAgrEeotjdZTZKuoQhsqnnZxX2vgCFSLLoNkGky/Sd/Y2d+jJpg8fu5AFljGnIzaaoQFf9I5jQ==",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[strum(props(
        svg = "eJx1TjEOgzAM/MqJPSk+iNshZe7SlaFbpA4ZGRDvxwEJGIJs2bo7+3RxSnPG/918KXj6MGoiiLaUo+MnXDG4SH8Stpk7H5ohPorPEA83UbvtK8Lrhu8gba46WSrsQ609wyg1A9LUpFDLJRAnJeQON8Lg73hbAYTYPNw=",
        categories = "time",
        tags = "date,time,event,clock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarClock,
    #[strum(props(
        svg = "eJx9jFEOAiEMRK/S9AC6ZTcbPmBv4CGMS4TEGENIhNvb1o+VD/lqZ+bNuBxuBXL1aBCaxwWB/xnhnfYSPZJFyE3TGNI9FrU2d5be5h7pGaAZjyu3SbHKilhW0suoQD0qjD0aJKoHxZu/BE2oPblSNHSwr2uJsHu8WKAlniaNxPyJyAyy9X/Gk3YwaQeTffYBAwZfVw==",
        categories = "time",
        tags = "date,time,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[strum(props(
        svg = "eJx1ULsOwjAM/BWL3Udtx0kjFWYGWBnYqjJ0YGBAfD8OQsDQKi/nzndxPNzHx0zX3eakQtKd86ik1LXBynrw/zvpU9LUkUBQqRE6l81+2DaT/fC1khyZaYHoV3CLp2fpFxgVaCVJKKMiGb23TznIznCdGAbRCAuM0cPtFlCKGYe3cE1rP21u2lIDY1RHTSwhcIOSoaSjFDip3gyZDe5TFMaxpHUjhWEPqdxSL99vvAC6R0/J",
        categories = "time",
        tags = "date,time,event,heart,favourite,subscribe",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[strum(props(
        svg = "eJxtjrEKg0AQRH9l2V7ijomccGedJm16IQEDQSxE9O+9dZUTkS2GHd4M4/tmaOkT+AUhKd5lAwLlehkyPB/HnzDKPRlR0Tqu/U1Lav//dV+aEbhkmiSwqMJ0jj8UVWhHV++YcBZwZ1BLckMKQyDWGe0LtkoDVqjaQmnBAr24PE4=",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarMinus,
    #[strum(props(
        svg = "eJx1jMEKwjAQRH9lyD0xO01ihTTgzYtX7wWFCCIepNi/N7FgFVoWdndmZ1989M+Mc6eOzkiL2vYEYT/VIAzi+tkoW/4zxLTUUr5UipvKSvFLpEC88acw5zU1DzvjF9ISwMEtHBqIzdsVvs3a/+Ju1/sFo3SKCi+WUeckx0mWaA2lNwzSPdU=",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[strum(props(
        svg = "eJxtjs0KwjAQhF9l2XuxO9XQQNKzF6/eCwoVRDyI2Lc322x/aEsOw2S+2d3wbj8d3SJfICTV1bUgUKmvQIHzaekJXznOH0nR1dyEgw5pwvPxulMvkcH0Q2RxSSVrn7xTVCFDlanHaCDVDf0tCMmZlLmhqpVqxWoG2GJvh3jr7l0wwc4gbwv8BP8BW+xHjA==",
        categories = "time",
        tags = "date,time,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[strum(props(
        svg = "eJxtjcEOwiAMhl+l6V1dYXEcYGcvPoRxREiMMYREfHtbWNyW7PRDv+9vbfL3DKk4VAjpWyP4+AjZIRmET5xyaE92NAIrPY72JL3RPuPLQyEWzoyUQwmqW4qqU1ZFmtVZEWYWk+S3FcVQ1JhuJnXtBOfffd9ygMnhlQagPhzqPRmukQYyl2GH1M6xo70SM7NhP0yET4A=",
        categories = "time",
        tags = "date,time,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[strum(props(
        svg = "eJx1Tr0KwjAYfJWje2Lu0sQKsbOLq4Nb0CGL4CB9fhMLRWjL8Q333Q+X3vlT8Dx3VxHULWZBcA1GRpfwz6GJ/cOBlvaEJqgcbejGdGg1Y1rKGKu33xCGnb8HXeGwVVUjzB6+buBvR8TMZsTJ3dexl+o4GdrQbtG/o5I7uQ==",
        categories = "time",
        tags = "date,time,search,events",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarSearch,
    #[strum(props(
        svg = "eJxtjjELwkAMhf9KyF5sXvV6w7Wzi6t7QaGCiIOI/fcmlx5WLRkeeXz5SLoPj5FOHR8gJM0xDCBQbVOhwn633AlP2X4KTYyR+7QxSZ+ul9uZJukYTC8NCZrwnDSDoQYVNHcZibw8jL+gyWonIc407tT6XwrMLFwr7fxQuyLOEL6gclzgN1nKR34=",
        categories = "time",
        tags = "date,time,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[strum(props(
        svg = "eJxtjl0KwjAQhK+y7AXspqHkIellNJiA+BACxtu7P7VK6dOwM98MG1u+dmjvhA5hJJwRXvXWS0IKCGx7hJLrvXRz2hByjRfprfFRnxkGcbYwTbbitpN1EVSgPzTspBFaCAdQCJpQC7ORerI6Ohn9huRtXdXpzyfDv9DKfitPO/wBW4tH6w==",
        categories = "time",
        tags = "date,time,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[strum(props(
        svg = "eJxljEEOgCAMBL/S9ANaMMYD8BklQmI8EBLh97agJ07T7M7WJL9nqBYXhMRQjNLwxCMHi7QhBB/PkPvNpUZnJtk5c8XbQ1FcrQiVKaC2L9RSVkX61FGRl0owijT3TlFXdR9y/Lsv8UQxhQ==",
        categories = "time",
        tags = "date,birthdate,birthday,time,event",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l6N3Y2SSmgbTgzYN+REEhgogHKfXv3S1SPcjCvh2Yt+V2vV8wS9+INHhRqfjEeYlD2VppKI/xWXHum1NCOoRRIGhtNnpN+ZuVUrkz0ZQfMbuIUOOR+qH61aAZU3Lxj8LgKASjYe/hlz6RXdeBre5VegPasTC7",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[strum(props(
        svg = "eJxNTbsKgDAM/JWQPWrTVhHazg66uksVKjiIiKhfb4uDkuFyL86swx5gtNgJlWlQgXRbQdWogYGhSEfxO+qPR+Qgyr9A3Nf/AnEguRBnmuSNzuRpxRk/b36ZwF8WhUTwZ0RG2CzKFHpt9wCkgSMO",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[strum(props(
        svg = "eJxtjUEOwjAMBL9i7QeQSUNTKc4PeASiFe4NVVGA3xO3UptDj971zsT3IyuNgvtAvnRI8WJJisv0zPSZx6yCDqTT/NIsuIG+gh70EwygpR5sI3tPsYGxL9eDthfck2uLVbNRDOnROC31hzqceCqOXXEnIlc9HJTD3v0BNQNDTA==",
        categories = "charts,money",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,exchange,hedge fund,statistics,diagram,graph",
        contributors = ""
    ))]
    CandlestickChart,
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/l1D0mdp04kUL/gB9gi8TAABID/y9cqrZDK0++e75z+/TvE4/rcEtkEO4CQfThMFIK8iqUA2vPyIsMjqS+LBz/Oc0aZFPiqtyHqV3m/KltLWxkCQZOEOIzIFKFUgGPqEf/bW7X+VWH9ORe3Wd1wDsK7Qk/1Os2rA==",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = ""
    ))]
    CandyCane,
    #[strum(props(
        svg = "eJxtkM2qwjAQhV9l6D65nfxNCrXgzsW927svKChUcSFF394zsaJoCMlAzpzvJNOfx8uetqvmmG0kbMPEY7BdpHK0ZQnJxIabof/R/qF/uv6YbQ6ekuUsaziC0ONUG1AJVLFxrEjWs8dNilLjhuL951AXoc0luiYD385I/daOnPAZNlHjk9iU/dqTX77JnZU2k984Btx1bnxpeK2oKfCvcxiHIir8TCwYIcaofPMREBRKjjd+Nh8BRgPMEgDIFN/nPR1OO7ryqnEN3RwK6nWpt3KNVm0a7rU0bA0=",
        categories = "food-beverage",
        tags = "sugar free,food,sweet,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CandyOff,
    #[strum(props(
        svg = "eJxtjrEKwkAMhl8ldE+8JHeXHtSCWwd9Abeig0MLDtLnN6dYkJaQDPnzf3+65/h6wP3YzIUSGCUUkDFSSfAZAdjbwCbBzR4N7dr03aFC+u6HunCETGnhsKeFmrKrzZw9iTEBkxrl9qSgnlSLCxnoILy4dgtAMRNTEVKp13oW8R+rdYfbAhsyJKxc/AdHUhAedKniuO7RwfgFu3ly68p9A63CRy0=",
        categories = "food-beverage",
        tags = "sugar,food,sweet",
        contributors = "karsa-mistmere"
    ))]
    Candy,
    #[strum(props(
        svg = "eJxtTssKwjAQ/JVh743ZPJoUkoI3D/oRxRbTgyAl+Ph7E0FRLHuZndmZnXAZcsIY6awYvlFQDQvbaOG2BUO+hq1oTQu788IMH7ocdlKDhbJub8ESGp76sKmRfXgHHxzYJCF5ReJ/bZmOGbd5zCkSe0Ka5lPKkQq8R9KER+ElYSmbqrZq+IosRfxVrf3qfpUnlapAPA==",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[strum(props(
        svg = "eJxtjs0KwkAMhF9lyL3rZn/aLXQL3jzYhyi2uD0IUhZ/3t5UUJSWXJL5MpM01z4nDJE61jDJUdvsFqltPuBiGKEwMAUrX1hV7aWHfhd7VboS/hCU67+yLNbagpXx1dFDgi3COrirwC4pzRuI12weTxlpnM4pRwqE+RHJEO7TkFMkFkUES3jKoBfbYviJlEfCzWzdqv/JC+ghR1o=",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[strum(props(
        svg = "eJxlT8EKwjAM/ZXQe2PTdesK7UC8ePEjJAoTdpApQ//epBN3kPQlNMl7SfL9/BzhUsyJElAcPWMHDshiEEeLbdhZTBajpYpWkA7UYwRy0kvy3OofUmoEwXr0goattMuXkFaBXsKxlbTM0GytBUyTBvCY9o3oKlw1D+SXwA6w033URm+GvNOth8y3macr8LsYigbmYrwBfhUTtWetDvl3YT2w+6NvNBX5im38D0tWRJE=",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[strum(props(
        svg = "eJxtjz0OwyAMha9iZbcbmx8TKc3CzJQTROrQoUMlOuX0xUQqS4V4Apv3+bG+j88THvepCImCMGldKCk6CsBC6jCSi4dvV9uzrV4y0RxIFZgpefgBxuksiaIH9i8UmoOJLxzItVqwmo9dpm29WZBtHXEElopMzqFYGJTMkVKENi7A0qVaGwQsq+zmAJPzD649l4oCF9Ac+8AIXh+wOZm1dRvWHF0G7guQjUIT",
        categories = "food-beverage",
        tags = "vegetable,food,eat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Carrot,
    #[strum(props(
        svg = "eJxtzGEKQEAQxfGrTO8CLLLUzNzAITQU5YM2bdyeTZTy9f37PbY52DKSHQJXgIKgBNku8FDO7qq89ttEg6BzObWxTilNyg+/gPP43vz6inxsXn8CHXwkYA==",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[strum(props(
        svg = "eJxNy0EKgCAUhOGrDG8f8dTCQL1BhwgLDApCIur2KRG6msU3vzmmM2C2tEtwB9VoKGhyps3gzM+jAsvQF/Br9NuCaEkS/G2Jddonrcinj6teMIar9C/IHyF8",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseSensitive,
    #[strum(props(
        svg = "eJxti7sKgDAQBH9lSe/jkjtNcaa2sbUPWFwjWPj/eBYKgizbzDB61NOwTWFPIAE3GYwcina3KProhUHJhh/hFZFxKzUioveRn2eSdTT+wje/AEDkHbQ=",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    CaseUpper,
    #[strum(props(
        svg = "eJx9jW8KAiEQxa/ymO+ZY8uugQodoEMsrqSwQYhQ3T51IQhiGYb583uPZ3LwBS9LihBDusViiUfC29JAyBt4pqXEukly5tgMzviU/RqQO/dVzbLOKtdNs1FnHnOJWCxdNVjFjtprx16z//jvI5SEmA5KnC8sBrSWtRhaMHiMk9DzL+jXKiacvrEfdSo9/A==",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = ""
    ))]
    CassetteTape,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l2Hs0O7QhhTR/4NV7QCGCiAeR+vduSqEtlD3szPBm0rt8Km6jXIh4DYUgvJ2CjlV3AfhVroEzVV2QnM5tJKfNlHEDhqUYEY+hUHr0C9ShW6Hn43XHxFF48ir4NeUFk9o3q7M1unH5D3E8Lls=",
        categories = "devices,connectivity",
        tags = "chromecast,airplay,screen",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cast,
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8luAfbJFYL1dnF1cGt4ODoIPl+q4IotGS53OOOC3s8Nlj7aiICMop+JPUxaTD3JbVZ9zWQlmoI9RUcwhu3HVg7y+i0zdEGiBQlMvDTg4ys5vsDq2SiaVcq9jlSAg5kpvzGAjAlID9wApOkT7o=",
        categories = "buildings,gaming,maps",
        tags = "fortress,stronghold,palace,chateau,building",
        contributors = "karsa-mistmere"
    ))]
    Castle,
    #[strum(props(
        svg = "eJx1T7FOREEI/BVyPePCwrKbPC8xtvoDduYsrrAw0fj9wjO5a+4KZoFZmGH7ev8508fj4VWU/IQR1EjQHW2REnRkFZOTROusmEYDppntlMEnZ03xjwSPbLcEyi+WT4fZswpJYJEMrKCqNPGbF/eMgI9TY4E6PFfnGNdYeqnuXLxrJlfL+8pskiFSoacdL6n+tNCMdmh1REn42+G4PdSNx+1y6SSxX/gNRsZ9qsyVffWzwF9yucQnI7ziKvMHnzZJmQ==",
        categories = "animals",
        tags = "animal,pet,kitten,feline",
        contributors = "kemie,karsa-mistmere"
    ))]
    Cat,
    #[strum(props(
        svg = "eJw9yrENgDAMRNFVTukNXJBjCpMJYAgkCgqQKNhfJEVS/Oo/f4/vwrmGnQsSDLRbVDRkH+vK3sATIziJDYrSxhlMXf2XeBIq",
        categories = "notifications",
        tags = "done,received,double,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    CheckCheck,
    #[strum(props(
        svg = "eJw9TDsKgDAMvUro3trE1CLE3sDJE4gOLoKgk6fXVCLJ8P5yzNcG6+BGJCBaUkjUQgSMngPnrADjhLnqBJp6r6ue0tNi39+uSKObRWx5763Fnn/7AV3LG5c=",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis"
    ))]
    CheckCircle2,
    #[strum(props(
        svg = "eJwtjEEKgCAURK/yca85ZpCg3qBte6EgQVTITbfPJBhm9d6zNbSLDsc2pQgQct2hAiT19SPwRZiZGwHNvJ0+2tta0pNiPqmWmNvtWHc1oQe0kCAzQhj8T/oXX2kbIQ==",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckCircle,
    #[strum(props(
        svg = "eJxNjMEKgCAUBH9l8R71HkYE6rlL1+5CgYKokAT9fRYEspcdGEblFO7g44GcfCynFjOIQAySYIYURvW/Y1S2xWHXYuXXuSbLYAx11NW3jC13vDWMyo7oy9WIeQDpTB6r",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckSquare,
    #[strum(props(
        svg = "eJyzKcjPqczJzEtVKMjPzCsptlUyMlAwU7BUMDRXMFEwNFKys9GHKbEDAEfUDao=",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis"
    ))]
    Check,
    #[strum(props(
        svg = "eJxljrEKAjEQRH9lSO+aickmQi5wnY2thd2BgoKIhYX39+7dIRbHwA47PHanvob3DZfOHRXcScl9RIQ3EVkioUMSEvNYYopPG0qKSKYf6gt8v0JV0h6K/02W5c0p8KBn1+p2KtDq4/68YmTnmB0+5moWbC0OY5hjYyeqfQHEEyYF",
        categories = "food-beverage",
        tags = "cooking,food,kitchen,restaurant",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ChefHat,
    #[strum(props(
        svg = "eJytjkEOAjEIRa/yM3uwUFqmSZ0beAF3k7pw6cL7RzomunFpSD4Q4PH7Y3/ecTsvF4X4XlCQjpCQkUjZa0ihGfmolApYLTRfl62fJmHrH478C+QQG5lVY7kJjLXRyh5kmj+4VijnNQbN0OKf6A+KKtqgeYxEzmJBy5kkGhR2eTs0rj6zf428AJKEPnE=",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Cherry,
    #[strum(props(
        svg = "eJwlirEKwCAMBX/lkV1qinSK+ZeSFlpoQcRB/15FbrjhTuzN9t2wFol3gtXlPORJZVtdJZ3lwRXp5wPsXUBwg3nMoh3c+xOJ",
        categories = "arrows,navigation,shapes",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownCircle,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8Sci9aLeKhyV/EFtODICWg/t7GsrAszE6seVd4CGeE9++7JBVCvyJILodo37V9JuQ4mMDx2lQgEZ5+AT+6AMG1GDfCHy6zF9E=",
        categories = "arrows,navigation,shapes",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNVOwVDADQV0zJTsbfZC4HQB8NQfk",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slow,dropdown",
        contributors = "colebemis"
    ))]
    ChevronDown,
    #[strum(props(
        svg = "eJw9yTEOABAMAMCvNPZGamgN1R94hMRgkRjE+zEw3HQ6ymxQk+skQBEZGQ5n6u+Yvs8CvCj82LwRD5U=",
        categories = "arrows,multimedia",
        tags = "previous,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronFirst,
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvjHWRGTbD3A98hGCwCAbx/aKgXDwdZTaoCbsARWB3oak/Yfo6kwAvCn82vfwPlQ==",
        categories = "arrows,multimedia",
        tags = "skip,next,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronLast,
    #[strum(props(
        svg = "eJwlykEKgDAMRNGrDNkXjQRXae4iUVBQkOJCb2/aLoa3+KN+FD83lEw8EvwNp/Brmg69m97Ls2PNdLGA5yRJEKuPWuwH2lYTjQ==",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[strum(props(
        svg = "eJwdy8EKgCAQBNBfGfYuYUl00P2XSGk9BCEL1d/nepiZw2NiK4fiqVklkd8IX6KF8I5ufWaClHqKDuY42YHjvasgJ7p8gF9dcAE95ib8AzTBF9U=",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,panel,button,keyboard,<",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronLeftSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNTRVMLTQNdM1UwBiJTsbfZCMHQCMUQhe",
        categories = "arrows,navigation",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "colebemis"
    ))]
    ChevronLeft,
    #[strum(props(
        svg = "eJwlijEKwCAQBL+yXB+iwSLFeX8Jl0AEBREL/b2KxTDFDGsoGj9o82QvgvbtMmVI+NxdOD/1x+spWYMbDu6YrGEFGckBE0A=",
        categories = "arrows,navigation,shapes",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    ChevronRightCircle,
    #[strum(props(
        svg = "eJwlizEKgDAQBL+yXC8aTZEiub+ICV4KQcKB+ntzWuyyMDuxlU1xJ1oIz9dS6i6ayAXCVbPKP1v/zMRxNIHjuaogJzrchAAPP/QYNsAvFyIXiA==",
        categories = "arrows,navigation,shapes,development",
        tags = "forward,next,more than,greater,menu,panel,code,coding,command line,terminal,prompt,shell,console,>",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronRightSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKtVQwtFAw0wVDJTsbfZCEHQCFAQgx",
        categories = "arrows,navigation,maths,development",
        tags = "forward,next,more than,greater,menu,code,coding,command line,terminal,prompt,shell,>",
        contributors = "colebemis"
    ))]
    ChevronRight,
    #[strum(props(
        svg = "eJwlijsKgDAQRK8ybC8a2cJis3eRVVBQCCFFcvv8imF4vCf2RvtuWPHkdkJstxEsD1RZp1cJZ3pwefoPOAYvbeAedKEVx/gTRA==",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[strum(props(
        svg = "eJwdy0EKgCAQheGrPGYfYblwoXOXSGlcBCED1e11XLzN//FiK6fiS7QTpNRLNJELhH+WNmAjvDWrzM5xtQPH51BBTnQHOA+/jMEbG3AHGfkXjA==",
        categories = "arrows,navigation,maths,shapes",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronUpSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNbRQMDTVNQNBBTMlOxt9kIwdAIx0CF4=",
        categories = "arrows,navigation,maths,gaming",
        tags = "caret,keyboard,mac,control,ctrl,superscript,exponential,power,ahead,fast,^,dropdown",
        contributors = "colebemis"
    ))]
    ChevronUp,
    #[strum(props(
        svg = "eJxNybsJACAMBcBVHumDIgabmF0ECxvBwv3x0yhXno4yG2qmnhA8hAUbmboTpl9H3OO3C+/GD+U=",
        categories = "arrows",
        tags = "collapse,fold,vertical",
        contributors = "PeterlitsZo,mittalyashu,ericfennis"
    ))]
    ChevronsDownUp,
    #[strum(props(
        svg = "eJxlyTEKACAMBMGvHOlFRKLNmb8IFjaChf9H0gmy3Q53PxOjyaooUC+oGKN/46Mpf3wB8o4P6Q==",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slower",
        contributors = "colebemis"
    ))]
    ChevronsDown,
    #[strum(props(
        svg = "eJxNybEJACAMBdFVPumDWIQgxOwiWNgIFu6PxELkunu22h7olWaBsuBGbim+29Ms0CD++QDzuA/u",
        categories = "arrows",
        tags = "expand,horizontal,unfold",
        contributors = "karsa-mistmere"
    ))]
    ChevronsLeftRight,
    #[strum(props(
        svg = "eJxtySEKACAMBdCrjPUhC0PDd3cRDBbB4P1Rk2XhpYfV9qBeeaqSZjExutiR3jj+l+gPIp8QsA==",
        categories = "arrows,navigation,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsLeft,
    #[strum(props(
        svg = "eJxNySsKACAMANCrjPXhB4dl7i6CwSIYvD9uRQwvPdn9TBgNV46QKjExGFQJPirvi7UX/X0BElQQew==",
        categories = "arrows",
        tags = "collapse,fold,horizontal",
        contributors = "karsa-mistmere"
    ))]
    ChevronsRightLeft,
    #[strum(props(
        svg = "eJxtySEKACAQBMCvLNdFDlHLen8RDBbB4P8RDSaZOJx1dbQiI0EzorvE6E8YX2v4/QYVRxB/",
        categories = "arrows,navigation,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsRight,
    #[strum(props(
        svg = "eJxNybEJACAMBMBVQnoRiyDCm10ECxvBwv2RtxC58rDaHtKrzizJ5AqmjshwfF04/LcH8N8P7g==",
        categories = "arrows",
        tags = "expand,unfold,vertical",
        contributors = "mittalyashu,ericfennis"
    ))]
    ChevronsUpDown,
    #[strum(props(
        svg = "eJxlySEKACAMBdCrjHWRH4aG7+4iGCyCwfsjiyIvPu5+poymC0WAZEFMnTnG+Xz9/gIi4hCw",
        categories = "arrows,navigation,gaming",
        tags = "forward,ahead,faster,speed,boost",
        contributors = "colebemis"
    ))]
    ChevronsUp,
    #[strum(props(
        svg = "eJx9zUEKgCAQBdCriAeYHFMzsC4jLYJo0Upv34wTBi1affi++aa8X/nY1LVoNFrlQmkpa8s1DfK+pte5P3bs56YqLjpqVW2LYgUXai0CTuzZPZr7EWYvMoKnD3ghgAkygu57wqUHF0TS6uw6bYNoIMZ+dQOtHzse",
        categories = "brands",
        tags = "browser,logo",
        contributors = "colebemis,ericfennis"
    ))]
    Chrome,
    #[strum(props(
        svg = "eJxtjLEKgDAMRH/lcA82sVqF6uzi6i44OCg4SL7fOEg7lCw57t6L9/Yc2Mfq4h4BHqLMm0Dg7Jjsm32eSdbh9CTVFOuPneJvWNhoUUpz983V5dn8voT2hq7tSR011KFRDqWVIGhbKhyGI3lfcQ01rg==",
        categories = "buildings,maps",
        tags = "temple,building",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Church,
    #[strum(props(
        svg = "eJx9zTEKgDAMBdCrhO5V87HoUDu7eAhRoYKIg5R6e1vs4FBd8iHvh+ht3Rfy6AQgyHNIQVeKZ2t0GUtGH+Npae7EwCBGD1dbriNHeDEiu5xwG8TKQmWs+Tlqp0qiUBJpqvzX79oNA2c9QQ==",
        categories = "travel,transportation,medical",
        tags = "smoking,no-smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CigaretteOff,
    #[strum(props(
        svg = "eJx1zCEOwCAQRNGrEDxtdwItYouu6SGaIpAIwvmBhKDAjHmTz/FLQfhbvmQF4UHWgU7peG/geDBQOeuJXCuoRfsfCptR6Gvm5fWtAOAqKss=",
        categories = "travel,transportation,medical",
        tags = "smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Cigarette,
    #[strum(props(
        svg = "eJx1kEsKwzAMRK8yZG/VslN/wM0NeohAF1120ftTSVmUJAqG2ViPp9H4rN83Xo/pyZEYibitnXqGRZTHyNQQp2XcdHYZf6JSkc/KStxhsRGJSpeoDpSYWoLKTp6oJg+JlDrUdhQFcZjM2y6T7KA2pWZYbNRVoUIzTOZ4Sg8XheRkUNm5T/D76MkgLudsNeza/AAXpmBD",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDashed,
    #[strum(props(
        svg = "eJxNizELgCAUhP/K4R75HiIO5tzS2i6vwKAhpKH+fVogDcdx9/F52bLsK+QaFLFCLqUV5H5n8P3Hgz/imbAMaiILlzobGQwNKtEwybRdHzO66lbnbzLIzbaRB+uaH+w=",
        categories = "shapes,money,currency",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[strum(props(
        svg = "eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgq3hORycjM+lucV58mc2BHDEw9LotShiJPH6GiAM/N40Nl5/DoiBWlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrbdSQ7KE1dPYpU116gQD0KrMEJye4EkpNBYds8tp1HTwZhNc4W7W+afFvz/YL8mgx7g1U+g/wulQzV9vwBoeZrTQ==",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDotDashed,
    #[strum(props(
        svg = "eJxdyrsJACAMhOFVQhbwUccsc1gIVql0e5UIgtUPd5+gGXoljMIpM2F6bSeySvBf5bqz868fW357FkU=",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[strum(props(
        svg = "eJx1yz0KgDAMhuGrhBxAmywuaW/gISQKFRykONjb928plE4vfA+f6B30uSBYJIOgfy7nxlona3Mn7/F5OC3utAGxXwwVLWtvPLfhlgBL8iIY",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[strum(props(
        svg = "eJxVysEJgEAQA8BWQhpwVwQ/e9uBRcgqnOBDDh/avad+9BWSiW3jnjElDj1Usgrdmntz+0r3k1hKrDPiSNSWKDWEiPOp9fW6X7ebGd8=",
        categories = "maths,shapes",
        tags = "calculate,shape,=",
        contributors = "danielbayley"
    ))]
    CircleEqual,
    #[strum(props(
        svg = "eJxti8EJgDAQBFtZUsCZu5iQwBmwAIsQfPgRfNg/XgLmJbPsY5fRe39OHIu7BIa3uKpTW6t+35YpRAilsrKHpcEQpgCOlOKPwoV8Ru8hsTFTkV7DeQE/Qx75",
        categories = "shapes",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure",
        contributors = "danielbayley"
    ))]
    CircleOff,
    #[strum(props(
        svg = "eJwlycEJACEMRNFWQhrY3ZxjOtgiJAoKHkQ8aPdqZA4P5rPmpiVCc/i9CDq2tJ2m8HO7cPU9QXD4E4HN6nllAY2mEpY=",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[strum(props(
        svg = "eJwlyjsKACAIBuCriBeohIbAvIw0BNHQlLfPavr4Hzz6bGBUsSDs9DAnZY/0FA73JKx96Wig5jUh6P4uJ97X3+UAjt4WGA==",
        categories = "shapes,development,maths",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[strum(props(
        svg = "eJyzSc4sSs5JVUiusFUyNFJSKAJSBkoKyZVgrp2NPkTeDgDowQs7",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[strum(props(
        svg = "eJxljcEKAjEMRH8l5L5o4ooutP0Dr96XbjEFD1KKrn9v0j1UWHKYhMy8cSXFCqvHE0JRYYRPXqp4pCuCpPyQuu1f8wR3sEBwr7kKLB5vRDDJODMwHNvwwPdmNEtwMZf4TBA1PmlFa4irHWrZnn+0CzC9h44jw8m4w3UOnbHBVTvwB3iGOgM=",
        categories = "science,development",
        tags = "computing,electricity,electronics",
        contributors = "danielbayley,jguddas"
    ))]
    CircuitBoard,
    #[strum(props(
        svg = "eJxtTrsOwjAM/JVT95pcEztBCpW6sbCyV2JgQWJg4utxAoKByg/Jp3u43tfHFZfDcJooZmAWyyslFPQVvDhKSH7awgneHUOSHDFJLM4m2rxx53XBc5jrrrnP9ZvBvZiC6klLwccdRaK6XUz/ihs9OIwqCp8tx9ReLnpmOJrwR3kBSy0yAQ==",
        categories = "food-beverage",
        tags = "lemon,orange,grapefruit,fruit",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Citrus,
    #[strum(props(
        svg = "eJxtjssKwkAMRX8ldJ/Ym5mUFsau3fQH3JW66KKCC/H7vVMRBSXkxbl5lNt8X+VybCZvzaWTJMCmNqhbXtSSwsDohK8cG5KF5oWAXEj2GJWfm7Ec6sqxvBdfO44GJclAH/4owFtEeZfkX8FUn1rRP/rZxaWlQVmd4rtX/1x/Ag40MrQ=",
        categories = "multimedia",
        tags = "movie,film,video,camera,cinema,cut,action,television,tv,show,entertainment",
        contributors = "it-is-not,ericfennis,danielbayley"
    ))]
    Clapperboard,
    #[strum(props(
        svg = "eJxNjMEKwkAMRH9lyH3RhLAo7Pbsxav3YovpQZCyaP17Ew9tCUwyGeaVebw32Dg9rFVSwvytxARXcbP8zWcamlU6EZbQrhyi1ZVX3wxDpStnqEkvEBx92Le8WbdH8uuS9z7JLe8LSUyCHMyN/DyDNWjQpGv8A3+5LKI=",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[strum(props(
        svg = "eJxtTMsKgzAQ/JVl76HOshUPiWcv/Qip0vVQKBJS+/dNcqhCZWBnmZdf53ukT2Bhei9TtMAd01bvmmVk2irZvDwsBlbu/aW0ev8ao9EU+NaRDu0oJNQUuPwl6C5kFoMcBSfJSZkqI4cptKS2J1GqSU+CAoIOwL/1xJXQOKWKn/8FvNE7kg==",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[strum(props(
        svg = "eJxtjDEOwjAQBL+ycm/DXc4JluK8AFoKuohEOBIFiiwgv8eXAlJQeL1a3Uw7j9eM1zTkFM3B4L3mXD4yWKJhgzROt5SjkbIvunftTqmuffQ5YYjmRHsnDGJXU8+OoG8PgrbQrHFsXPBggoD57kKwlQ7eSWVLyEW1Ktxqa0jinsHFpr7SniS/wZaWCu7/wAKqnD/XW9xy4u/pB65IPLA=",
        categories = "text",
        tags = "edit,paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardEdit,
    #[strum(props(
        svg = "eJx1jDELg0AMhf9KyH6tCUdwuHPu0rW7VGncihyt/fdNXDxBCeQl7+O9NI/PAjpOLy0ZI8J3GopmbBHmJSOZ/FaxzQiLky5dPdWld18Uhox3EojKPQNDY0Om/KG4GcGum9R/4IfUgcDK3uyddTMDkcYTIoek9ciloWMkO/QH/jhCRg==",
        categories = "text",
        tags = "copy,paste,tasks",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardList,
    #[strum(props(
        svg = "eJxljjELwkAMhf9K6J7zXoxRoXbu0tXB7TiHWwQHud9vAtIKJYQk3wsvGd/l0+h5GxacSOZrAYFyBHvXpWZKlpQCo1lN5ho4CON+rJmT+cSIeAzTeAjDaVxtL6SzFSH5uXrXoRvwKg3yD1gWGGnbIGKrO/YntCHv77xwJmRSUvZc9S/UAjKN",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "xnousnow,ericfennis"
    ))]
    ClipboardPaste,
    #[strum(props(
        svg = "eJxtjTEOgzAMRa9iZY9bGzckEmHu0F6gGyqoQepQoYiW29dhgQFF+baefl6aaXhm+I59TtF4A781Jx1kIA3jK+VoxMASDStfCm+bU3nVNp8uJ+ijuXuQq+sYGM7lWN1mkg3o5ES8B5Zni5fiKpadixxI2qoEhHUFdNT0KAwBHXWMBOWufd1CvcaNHIYLUA2kCv/GEGxViKBUVkMeB16v1bR9+AcvaEaZ",
        categories = "text,account",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardSignature,
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/l5D2Cs6oAUtKZhZW9ohXuhqoo0L8nYaAZKg939vleWKZHwifKWfCex2Q/Z9P8tBSlE6xRVLCUDxZZq/ThUFt9eA3JMEa50aMzHRSKYxkW1cxuO7jirr7dnd59W3BqWsmV2ZAvoGZH85k7KQmedntUkNn/ky8Sjzvz",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/l5D0CW8ECKenMwspe0Qp3QEJRBOXvcViaAXmwfe/uUplvFeWTiQlrpiPB5uVuNVMkuCyEsv7oe5mqNceQdi01pOdYDVOmCyuiySgQ7H3Yt7w4bkLw66z9H+SqfSCISWtunVvzgw9gDgr9w06O0KMvAMkzgg==",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[strum(props(
        svg = "eJxNjL0KgDAMhF8lZC/aUIpD29nF1V1UTDcpxZ+3N+1iCeRL7rhzaV8zPB4HBN7jwdmjQbjjlrmKSTwteCtkEwbXlVRw55IZNo+TtmCYFgKCXkYL6dLmF5Rco21/RbNtA4q4NpfO8AGOwCSc",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+UHFRc5nQRSC0pbqwt7dRCsMMw3uepbAG8HO6eXEobSYHrt8lP/6cfE5aVWJAThLvyzB2tLKsw4bD7G7RC2+ZGXM=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[strum(props(
        svg = "eJw1i10KgDAMg68SegF/HsSH2suUPRTKNqYP7vZWZRASQr6wWlNP0PugZSW0iJmg/avC078L1+LdLSfUYvk63xkbwkI74hPsYOQBQHsZDA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[strum(props(
        svg = "eJw1iksKgDAMBa8ScgB/oCjEXCZ0EQhtqS7s7U0phcebxQyJFrEAUm9cNwT5OotjQaa5e6acrJrGADlpfJ9WwQF+vmva4WzxiPgHWfoZRw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[strum(props(
        svg = "eJw1yjsKwDAMA9CrCF+gn6GT68uYDAaThLRDc/vmQxYJ8cRqRT1A603HSSitdoJ+Ywpv04Vz8uoWA3Ky+D6dcaHFvC2WH9BOGDM=",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJWF65iLhO6CIS2VBf29iaKMMyHeSQ6xApkHmndEoZHTpD7nUzL9zP1ZtO0FvSm9Trjxg63kJcc8A/xA1e4GTs=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[strum(props(
        svg = "eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbUiDDMM75FoEQuQern9cChtNgd5/8u0Ds6Uk1XTGJCTxufuGB6tevxyYuhT4w+HHxmg",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock3,
    #[strum(props(
        svg = "eJw1iksKwCAMRK8ScoF+KF2luUxwEQgqtot6exNEGOYNzCPRJpZA/gePE0H6ZHPsyLTNn6kW66Y5QS2avzcsuMEr4uMKeUk8AFlhGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[strum(props(
        svg = "eJw1ikEKgDAMBL8S+gC1oj3FfCb0EAhtqR7s700UYdk5zCBLZ83QjxCXAHwbV+N4STh/nrBVHSolQ6tSrtM1JLDzbdMOMXn+Z/QAhakZog==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock5,
    #[strum(props(
        svg = "eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbWtMMwwvEeiRSxA6uHWzaG0WRzkfi/T/HGmnKyaxoCcNF5nx/Bo9cdPe9eHxg+G8Bmg",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[strum(props(
        svg = "eJw1ikEKgDAMBL8S8gC1ggUh5jOhh0BoS/Vgf29KEZadwwyJNrEE8l4YdgTpk82xIdM6PVMt1k1zglo0P/eoIIKf71wOCHHUf8UfcU8Zdg==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzelCMvOYYZEm1iC9CvELUDeyeZYA9MyPVMt1k1zQi2an3tUOODnOxH30f4Nf0I7GRA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+QN3EXCZ0EQhtqS7s7Y0WYZhheI9Eq1iE3EeYl4DqMwVI+y7T2DlTydZMU0TJmq7zxdjg5dmHFd3+LX4Ab3EZcA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock9,
    #[strum(props(
        svg = "eJw1iksKwCAMRK8ScoF+KF2luUxwEQgqtot6exNEGOYNzCPRJpZA/gePE0H6ZHPsyLTNn6kW66Y5QS2avzcsuMEr4uMKeUk8AFlhGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[strum(props(
        svg = "eJx9j00KwjAQha8yZN9nJj8mgbbgATxEiUIFBSku9PbO2IUuUgkhgfnmmzd9vSz1eqb6HAw7Q/UlbzK0DMabsd+t5bG/T4+ZToM5BjjiCD4kSmSJ9UQkpjwzUpkCIum1n6KTX4ZTkxq+nps2EWeEDqWDbxBlnVQ2AbbYk7NI8MK0AA9JJ4n/AKuhkxzcHpHV4LeADK9b7ImFaAmCrhnV8gu8AW08Xzs=",
        categories = "development",
        tags = "computing,ai,cluster,network",
        contributors = "karsa-mistmere"
    ))]
    CloudCog,
    #[strum(props(
        svg = "eJx1zMEJgDAQBMBWFgs43eM0CUTBAiwi4CNPH2L9Gh/iI+E4WBh245HOjH3uNgNNfAirg8MAlhvFET5TXEgmI8oPL+qTvKhpt8S+bCzxW/JguFgHqwKnVqVIo6NQtoTTT26Mzjww",
        categories = "weather",
        tags = "weather,shower",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDrizzle,
    #[strum(props(
        svg = "eJxtjDEKgDAQBL+y5AFnLly8HMSAXRofEbBIaeH/0VikkmVhYGDy1e6Oc3OHgIWS2a5QePBYJGWkzqTWhCLG/SfDS4mCBFfyMholzxKvYK36ZxSBq03zADdVHrg=",
        categories = "weather",
        tags = "weather,mist",
        contributors = "ericfennis,karsa-mistmere,mittalyashu"
    ))]
    CloudFog,
    #[strum(props(
        svg = "eJx1jMsJgDAQBVt5pIA1u+QLMWABFhHwkKMHsX4TD4KQsCw8GGbSWa6KY1W7ARsKMW4eHhrcz5JnhMrkYzFk0V+/UNoKJEZUTktv5PSV2LXUPSJhBpoiupLmoTRFLGA3DgpEftYDwxw9FA==",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudHail,
    #[strum(props(
        svg = "eJw9jEsKgDAQQ68S3Dt2+psWasEDeIiCiy4UXHh/dAQlhAQSXjnb1bHNwxrBkZyNi0BgwKpAwkidSXLzFKA27/iURFncUMukiFo+0MEObEeH0P2u8V9uMssZVg==",
        categories = "weather",
        tags = "weather,bolt",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudLightning,
    #[strum(props(
        svg = "eJxtjTsOwjAQRK8ySu9lf97EkolER8MhLCgoKRDnx2CEUkQjTfPmUx/tecftOF2EiRdDOQUxK4Zzl0AC3pzUDcP5q0BcGUqqSUi5h0giGTJJmae1Hj7ba/0/GJRbRh6bWKgkP4s1g/2OtHfVd6rSGb90h8yQsgFvgzYwWg==",
        categories = "weather",
        tags = "weather,partly,night,rainfall",
        contributors = "it-is-not,karsa-mistmere,jguddas"
    ))]
    CloudMoonRain,
    #[strum(props(
        svg = "eJxFjDEKgDAQBL+y2CfmTHIhEAN2Nn7A7sDC0sL/YzyCMrCwMLvlkvvEMQ8beRCLh4cDNRx4TRIRew82G96HWsZ3Uss/dJaQFwY3U11GkGCnAA2ncEcV4xEt5fS9PQTGHV0=",
        categories = "weather",
        tags = "weather,night",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudMoon,
    #[strum(props(
        svg = "eJxtTkEKwzAM+4rovZqdzHMCWSEP2CMKO+Qy2GH/Z2m79VCKbGEkLFTe86fheR9eAR3SZ5jKZVGn8vceRk8BK1eHQ1ZkaG6JNl9pWHaTlVF8pOZ4khSUFgP0RquHN++nShuVnqtTJGHjny+9gPie+QXScymt",
        categories = "connectivity,weather",
        tags = "disconnect",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudOff,
    #[strum(props(
        svg = "eJxtzEEKgCAUBNCrDO39+b/KVzChA3QIoYWboEX3pwzahAwDAw8mn/Vq2Jdp82BPMaVVobDgnkDKiI1JU/UU0GtflGdFEi9TyXP/KPl7OhIJROCMjhDsjMOIWP92A4dsJvg=",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudRainWind,
    #[strum(props(
        svg = "eJx1zMEJgDAMheFVQgeITUibFGrBARyi4KFHD+L8th7EgxICDz74816PBtvsVgEStJQWBQUPNC6gElgj1FQFA4z3N3JfhizsSp5Go+SnRLGnzvgh9gfEQPElF6hNJkQ=",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudRain,
    #[strum(props(
        svg = "eJx1zcEJgDAMBdBVggPEJrSmgVpwAIcoeOjRg/tj40EQDCEQeOT/crarw7FOewSKmFU3AYEAZJNQCHInFG0RE9iGB3lcGTnyVMtsGbW8SXk8dgz0T+oQMZD4xuTZ4teZfftu9RA+Ew==",
        categories = "weather",
        tags = "weather,blizzard",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSnow,
    #[strum(props(
        svg = "eJx9TzsKwzAMvcoju1VLlpoY3MxdfAhDhyyFDqXnr2IoFGKC4A3vJ6m82nvD4zZVFshHprVcdmotP+GplBM6MCl3OLqqRLBsozxninMvCGcFbJR19hK6WlMo4j7BWbGgxLIMQgkSm8Hc6sVYKAe9c2oJqecZAiPR0T7X4ujh6kfkP+ELYhNJyA==",
        categories = "weather",
        tags = "weather,partly,rainfall",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    CloudSunRain,
    #[strum(props(
        svg = "eJx9jb0KwzAMhF9FZLdq/dk1uJmz5AW6GTpkKXQoff7KhnZJCEI3nO4+1Vd7b/C4TSsx8IenuV66Ndff4alYBIYQKg3Zp1aOQLwd9algzAMQzgBkWDQ7BJM1BYXYJ7jLFhSJr0clAeYlNwPztLP7n5AWkiYgA0G+6f7vfgFeCzuq",
        categories = "weather",
        tags = "weather,partly",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudSun,
    #[strum(props(
        svg = "eJwlizEKgDAQBL+ypPfMgbosxNQ2fsDuwCKlhf/HBBmmGqY88TbcezqdtsJ1KAgiwzub0Sc1NyqWnod/ydCVapnHXj8tUw+0",
        categories = "weather",
        tags = "weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cloud,
    #[strum(props(
        svg = "eJw9jLEKgDAQQ38luPe8u1qPQnV28QfcDhw6Ovj/WCmUkCwvSXn8rbi36RSjBJUju8HAkKaVTEKuQpZ9afh3J4x8TXuZ//lexokqhD0itkJTiCHWoKRsniixomeHwmSslMbPB+OFH6M=",
        categories = "weather",
        tags = "weather,clouds",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Cloudy,
    #[strum(props(
        svg = "eJxtkrGOgzAMhl/F6m5f7IQkJ3FdmO8hKm64gUodOvXp+9ugSi3IECGw489fGG+X+z/9/Zx+tYpRln4xaeR38mC8UUoLS5HcseLaZxilqUrLVKQPVKU2Xwp16kshBJdZJWfGUknFGqp0EOMi9jidxy/HOI8vGBTOkcxR5skcZchnsXcChFMuAAy+T8L1u00Bpw3brHR4zBmMWg8IFJx13niR5ghBjhltTxCeYCkcHVjaPE367fuEK29fV1lo1g8YmnSytGu2HYl38Yl3rdYDAXtyfckHUGPD9FCnrhCkxYNKWM60qcZ0oRp/wvHBXMHaeKDh9ekJMXV6CA==",
        categories = "gaming",
        tags = "leaf,luck,plant",
        contributors = "ericfennis,csandman"
    ))]
    Clover,
    #[strum(props(
        svg = "eJxVjbEKgDAQQ38ldO95PbizQi246w+4FRwcHfx/bEGUDoEQ8pJ0lfvEMbstjCQRE7EWJUUTI4B9YFIDL12KIKiEWV+uTqKPZLvLaWjbOf0PL7JWI/IVHkOeHTU=",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Club,
    #[strum(props(
        svg = "eJxtzCsKACAQhOGrDHaVhVUMq3cRDBbB4P3xETbJHz9mZNbV0bIZlEARbF+miL9SRD0iWcbro8QunHE4H6obZTgYJQ==",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "ericfennis,mittalyashu"
    ))]
    Code2,
    #[strum(props(
        svg = "eJxly0EKABAQRuGr/LmAxmKaxXAbCyUUG7cnZSH1lt/TVvPMqUS0msro3hCDBM6Bdgw2Qe1FQT8uYBwre3vsAgb7G4s=",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "colebemis"
    ))]
    Code,
    #[strum(props(
        svg = "eJx1jksKgDAMRK8ScgClAcFF7W1EBGkLumhvb5J+1IV0MSSdyTwbw5G34CGG3V/ngoaAH8E8TCJmYjW6qUP5khU6O9a4s8fuV8jEB9iEkIycQsisxJpIZ06I0VnJaaT1Ur/7tLSCv0ine8F+M+pvLJWhMKHC1hI136DMRos=",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[strum(props(
        svg = "eJyFkMEKwjAMhl8l9N66ZLXtYBN8AK/eBwoOxjZwyObTm6ZD6kFGD0l//i/5ST218wNujboQArpraAkIivg0ajS+7LXXNlO5sgT2/JWghPDKQOBJEWRTJkaOJ2WYbHyrU32IIU71NPZr3w13mMZumJ+N8uYI1kQbgTMh2jdFoM3+B8TK+Aqks8ZxRqQdKi2SHYKkLo7ZAUtDngNWTngyBQIVfAHRflnhFmwUp4GFUl35T4wFbpPEjPg/mGxo2w==",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[strum(props(
        svg = "eJxtzLsKgDAMBdBfCdmLjQYfUJ1d/AE3QUFBxEFE/97U1gciHdJLzo2Zm6WHNseKEkh7ahgYNJA8LVkRFiawpjC3jCzkNfNWtJJfmbyz4vppjsPUwUY5xgi7jBBhC12SwRZa4qEXbnXWSLuCzF967i7L36sHAYU4Uw==",
        categories = "food-beverage",
        tags = "drink,cup,mug,tea,cafe,hot,beverage",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Coffee,
    #[strum(props(
        svg = "eJx1j7EKwzAMRH/lyG7XJxs7hTR/0B/oZuiQJdCh5PtrR9B2kBEaxOnpdMurvjc8b9OdAgl1xowAtg6OGTpqMT+mdbl0YF3+Maba2C+WoJNWGkByyECQwxnSztL+8zk7OvoSLZhg8FJQEH1Mxo3zAFjOC47GRgcb33XQ8kigbLOhSBdkaKqeGHpSTc2nqIEcYSffe/LY6YTsr7+ND6UAefk=",
        categories = "account",
        tags = "computing,settings,cog,edit,gear,preferences",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Cog,
    #[strum(props(
        svg = "eJxli1EKgzAQRK8y5L/brMpmC4nQA/QQJS0oKIiI6O1NFESQhZ0ZZp6P7Ri7P8ZgxCAuwWiSNUvtn0dZ++E7NfgF82El+wJbKt1bILDgfClX4B3J0wvgIA3P1b3pWcgltCRVcsk+CtIC+Z3jDdDDKIk=",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[strum(props(
        svg = "eJwli0EOgDAIBL9C+IChXjy0/Yw2lsR4ICSW3wvtBXazM1naqSBWMCHImO/jS3tBOhB647vrys7sCCNuzVt4NT/8NrDkGvlEDrpvtMg0q7NB1R8gTBw8",
        categories = "layout,design,text",
        tags = "split,parallel,vertical,horizontal",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns,
    #[strum(props(
        svg = "eJydkEEOgyAURK8yYY/lo0abgOtueogGTSExaiyx9fYFbBvW3cyEz7zFjFoH42EHd7des5Zh10wyrK9khz5d72387NQpxju13LxFr9mVKkhDBUFAojgHkVtlBMKJxycPh4hFIMOk+AujkGgvteFUNBC8DF4GLTdOWXoe99FNA5bZTf6hWQNJSCgaUJ2Sn0ynUv1fwXyI0J2qtEe0Y5DvAG+1EVFt",
        categories = "shapes,development,files",
        tags = "cubes,packages,parts,units,collection,cluster,combine,gather,merge",
        contributors = "danielbayley,ericfennis"
    ))]
    Combine,
    #[strum(props(
        svg = "eJxNjCEOwCAQBL+ywV/a66Yoiq6prSdBIBGE93O4E5vsiJnUy2ioT/j0Rpx6FYI4oTYK3+gZ/B2LveYFMSHkdOxkXnlLE2Y=",
        categories = "development",
        tags = "keyboard,key,mac,cmd,button",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Command,
    #[strum(props(
        svg = "eJxFjFEKgCAQRK+yeIAtRdTAvMwSIYiK9ZG3z1ToZ4aZN4wlXygcQHVnXDAozVYG9PTo7DK4szmFeqYIOfl4X40qFBI0agVcIhdTR9HZhsYM+bff43xyLxPVH80=",
        categories = "navigation,maps,travel",
        tags = "direction,north,east,south,west,safari,browser",
        contributors = "colebemis"
    ))]
    Compass,
    #[strum(props(
        svg = "eJyFTTsKgDAMvUroXiWRQAu1J7AXcBMcHBQcvD8m2kIFocOD95L3CedybbCOJnHH4AQekHY7CBNMpEqIHmYTQ6/+GErqQAKCYhbhnw7t0s9PIqHLQ0SfJeT2FHK9RW/aZolV6gajODNM",
        categories = "design,development",
        tags = "design,element,group,module,part,symbol",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Component,
    #[strum(props(
        svg = "eJxVjMEJgDAQBFs5tgFNUPGRswOLEA1efhICxu7lokj87uyMi35NlBk96GJY0Bm2JAzTgcSHXRJjBMWscHKN/idXrGerX69r2xLTxF87liS0MeaBzCgF6FQBY5UMH7kBUJotSw==",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = ""
    ))]
    Computer,
    #[strum(props(
        svg = "eJxtizEOgCAQBL+yoSfebQihQGobP2BHYkFpYe79og0WZLudmXzVu+Fc3U5oqgQhfQp6No3/AzRuNM/Dlby8Xcmj7kqsCelzxWuETDQlgoUZEIQ2wAPrtiZ6",
        categories = "travel",
        tags = "reception,bell,porter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ConciergeBell,
    #[strum(props(
        svg = "eJx1zE0OhCAMBeCrvLwDMCITYAHeYA4xGc3gwsQY4s/tLRvjAhdtmn59Dcvwy1j2SE1Ib4lt7HOSqSHSMP5TjvTEEWnZhVe578L8zQl95Ec76PfqipTdTR5BImY11UR9rxt5hVYZWFXxSdDCKVeqwl7Uw19yAreRQ5A=",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[strum(props(
        svg = "eJw1i9EKwyAMRX8l5H2scVJ8UP9gH1GsTKGMIcL075toSx5uknuO/W01we7wTSuQ2TRoWGQeBhb09im9tyGXcEQoDl8IoTkkxdk5SaBZe1tiqMBfjVAYYqYN45/3mhg2CCnmT6pjZ1EEb4/8jdBpCF0NvfHFdFMSDApygfKjdSKSlzH9Gz0BY2U5JA==",
        categories = "account",
        tags = "person,user",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[strum(props(
        svg = "eJxVjMEKwjAQRH9l2Huxu4pWSHL24keUNJhAEQkB2783m/ZQ2cOwM2/GfMYSMVl68g08jAJBr9dJJ4/78YeQMyflncnBF+TFkhBiSK9YLPFAqM6Z8E1TiZuxWrpoTQvO+JT9HJBbz1eYVSvDvUJb7Myc3gErN2qposPSZJV9T5Ed1IivdCy09w/9ATI8PC8=",
        categories = "account",
        tags = "person,user",
        contributors = "lscheibel,karsa-mistmere,FPDK,ericfennis"
    ))]
    Contact,
    #[strum(props(
        svg = "eJxtj70OwjAQg1/lxH4ml+ankUqfgK4d2KowMBSJAfX5cUDqQhXposT2F2d4Le+H3C+nyXvJyNUpkiKowSt6bnHVhE47lMWQvXyHa4tiFreqOXSSqiK2TGEKhbawJaTqhNeBR49eGo0wOYDJD9ZYmiqigiY1aQPxdhqHc6s6Dnthc+INZbZw9TApsH/Tkybj6/TxF+WIQpWVN+2P8pOxdEacC8KufgD6tEfM",
        categories = "development,transportation,mail",
        tags = "storage,shipping,freight,supply chain,docker,environment,devops,code,coding",
        contributors = ""
    ))]
    Container,
    #[strum(props(
        svg = "eJwlijEKgDAMRa/yyS42GYpDmxt4iBIFBQcpIurpbQlveMN7yfZqxwp7MrEQ7HXXpkCaRu+aznJtWDLNLOCpREQEZ2C5Wb4+90l/uGwWBg==",
        categories = "photography,accessibility,design",
        tags = "display,accessibility",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Contrast,
    #[strum(props(
        svg = "eJx1jLEOgCAMRH/lwg5yhIoD8gd+BImDo4Px+60MJiSSXpu2L3f5rNeBfTUbA0Klh0oH2qKKWu1jxUp/mZKn117yF7I4gfbtPH8oZ1DGNIBhxAimAUtg7NADGX01yw==",
        categories = "account,food-beverage",
        tags = "biscuit,privacy,legal,food",
        contributors = "it-is-not,ericfennis"
    ))]
    Cookie,
    #[strum(props(
        svg = "eJw9TtEKwjAM/JWQ99YmRHHQ9g989V26YfYgyCjo/t5kshG4yyW5cPn96ApjwRcx0BnYSoJgzSff1LxMrcNnHrsWJEHQaX5q//ffgleEdcPFBButTuZ2X83795sAXVqgSJAChzgY8F1a8pErMK2U2nZhGeLgUY4YP22SKfk=",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[strum(props(
        svg = "eJwljMEKwyAQRH9l2bvWFVsSUP+g196LkSqUUkRI/PvsmsvOzvBm/Lf+MgwbkO4Igy49RC2r5AtGfxMs+pZThyPggtBYmBjT7HXrhVGHUHL9lH79bQjDbelF/3/3AlvApwN6JEWawCir9MrHvlwyEokD9oVMmgRY0CufOSQT8QRoMiz/",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[strum(props(
        svg = "eJw9jMEKAyEMRH8leF9rgi1dUP+g196LK1UopSxC9e+bKN1Lhsy8Gfcq7wSNvMKzgi56ZUVWUtBw+MGdBAtuwP0Pz3BCAs/yAe8pVmhe8d7evWIip/LMlSnL7Rm0EXzLVvPwuS294D6PmmHz6mYBL3FBjWAWWvTKh+42GrHkA/4zmjgIINArH5IhmQg/Uvw4Ng==",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[strum(props(
        svg = "eJwljsEKAyEMRH8l5K41QUoX1D/otffiShVKKYtQ/fsm7mXCTN6EhHf7FBgckW4Ig2QywqTTT14+hYtiKRwld5gRZXWMiELW0l61C+Ulmiv6tb3XMxmKSlt7KXyfvcIe8e6BrtmQJXCGjd1E+OGz00gdiK/k8iKAwW4i6w09kf5xCCz/",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[strum(props(
        svg = "eJw9jsEKQiEQRX9lcK85ItED9Q/atg+fpBARIqR/3zjW28xlrufIuGd5JejGC7wIGEhpBPRfjtUHd5pYcAv+P+KSDtlwf8A1xQa1e0FsHRyfsrdMlCXLi+nwzKk8cuOe7OkF9763DLsXVwt4jhIVgpZGqo2GudmoZzU3oD2jjkyAAbXR4DPmF+ELVbQ4Ng==",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[strum(props(
        svg = "eJwljMEKwzAMQ39F+J4sNmGskOQPdt19pGXubZTA1r+v3V6eLCOpbEsf+K3z0EqcCbqsHx3Xve2VxOR/ivFB2J2t3LzXyvc9FHOlZwbfe+DISEFCnAzyyj35yx3MK6d+JiCIk0F8yCfaAZa5Icg=",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYpi6EcW7QIWQKClqEtKjbp8lffB7vsR5Vzw36ZnIzobazBH1+FJ6GF77KvWPNtCQk42MJCLBwbRbR+J72RD6IZxUT",
        categories = "",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[strum(props(
        svg = "eJwlykEKgCAQheGrPDyAqelCGOcGHUKmoKBFSIu6fZMuHt/i/SRHk3NDK8Y7A3nUoL5dpmn8TFe9d6zFLD4h2znFGhHh4HUOyYbc8z/jD9mNFd0=",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyright,
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiPVIwQdNZtOoRQkCA6kATdviGQt3mLL3Arb8n1BLdc+x2Vh9FwMAs8rFYU5kEocOoXjqh2K+RZkxOoJTPJbe7HQugDUxcX5g==",
        categories = "arrows",
        tags = "arrow,return",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownLeft,
    #[strum(props(
        svg = "eJw1ylEKgCAQhOGrDF4gFaUX3Rt0CKFAQXQhCbp9myADw//wBe71raVd4F7auKMyHkbDavzhJRSFbSkKnEbGGdXh4J49yUPPSWVjJxZCH42kGIE=",
        categories = "arrows,text,development",
        tags = "arrow,indent,tab",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownRight,
    #[strum(props(
        svg = "eJw1ylEKgCAQRdGtDP5LKkYEOjtoEUKBguhAErT7ZgJ5PxfeCdTrW0u7gHpp447KerAr7OAMSCkMyzQYKI0MZ1SHvFlvyTMyMs31WPdzRvgBlccYkg==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftDown,
    #[strum(props(
        svg = "eJw1ysEKgCAMANBfGd4llUEIc3/QtbtQoCA6yEt/3wriXR/JaHer/QQZtc8rGY8QFapomJY/MEmeBY5ktuAguGLX/CanvEWLO35dEz9bDxf3",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftUp,
    #[strum(props(
        svg = "eJw1itEJwCAMBVcJLlAjSn9iNugQQgsKooFKods3FuQd736OpNe3lnaB9NLGHQ1awDBxdoLBMG2rYpI0MpzRHB583pM+WB2q/YPujzXhD4zGGII=",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightDown,
    #[strum(props(
        svg = "eJw1ykEKgCAQheGrDO4jjYkIxrlB2/ZCgYLoQG66fZMgb/Pg/0hqfnMqN0hNpT3eOAs7uBUQFn2GaR6ESUKLcHlz/DFuAVXZPpzwxI6V8AdSBRfl",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightUp,
    #[strum(props(
        svg = "eJw1ysEKgCAMANBfGd6lJYMQ5s5d+gihIEF0kAT9fUrEuz7Wmp+cygFaU2lXMB5mAgLfkRGe/iCssZ2wB7M5BIe3XeKIOFiytH69J3kBW+4X9g==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpLeft,
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiXHJmIQL1BhxAKFESFJOj2TUG8zVt8rrdyl1wP9JbrOL2iGcSwBitkWQU3/Sa4HkfC7tX2iksvkcEwEoE1J7IfFxQelrAYkw==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpRight,
    #[strum(props(
        svg = "eJx1jksKgDAMRK8ScgHbokKg9QYeQlSMO5Hi5/b2IxSl3YRkJm8Yvc+jhXOdLBuULQLP68I27pfBGuEOc3eHwk5XHuh0wJxDiXDAmxNRSt/bYBkmg71sQB0hxks/Q2QdBbLhvEF5XRQJUUCo0Iq+pR7iJlY4",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[strum(props(
        svg = "eJytjFEKgCAMhq8yfG9pFhaoN+gQsoKCHkJ6qNvntKADxM/H2PbzWVojbTPQ5YRqBMQ0pAA68+ptXf7e7uFYYHJiVBIG1KHBHhjJqTR2oEBjQT7R2KaOybyX1GMt675S85v0BhzFLfw=",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[strum(props(
        svg = "eJwli0EOgCAMBL/S7AcEojfgM0qkifFAmlh/b4un2c3u5NF2IS1IoLdgA/XGZ5eCuILGPzx8SLcUUPPiQs0X3400zlmTwX2rMRjTpH39VT9SRxpw",
        categories = "account,money",
        tags = "bank,purchase,payment,cc",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    CreditCard,
    #[strum(props(
        svg = "eJxtkLFOBDEMRH/Fut4mduwkKy3XbEPDD9CdQkEBEgXi+xlHaAs4reJdbSYzb7J/3r7e6PXx8uHSSKuoUkjfuIrpVBkbq5Qgxz9S6YOqdF3jPTUG9dBjyCDD4bZkIVbp9Hu5XPeHjLnuZ5gWCdokWNnEtmMToyY+KI0aYT25xGFp1yj3AnNI3HrGr1HWYwga9X/EMzxmAXyAxtnZclFBHqxE+/rwe2xITvKUGdpM6VUqyhv6rxdGfEMwS16J8pLGr/NfROUEZLuDqIO0zUSEzhmc5CiEUpyEy9Rznmd/ANAiXZE=",
        categories = "food-beverage",
        tags = "bakery,cooking,food,pastry",
        contributors = "karsa-mistmere"
    ))]
    Croissant,
    #[strum(props(
        svg = "eJxFyisOwCAQRdGtvOBJOy+kQUzRNbX1JBVIBGH9fATkmiuO5lgS/tu8F1jFRYI4Z/2SOBP0GCbokuJBfn5TS8uHSzZ+uRTQ",
        categories = "photography,design",
        tags = "photo,image",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Crop,
    #[strum(props(
        svg = "eJxdzqEOgDAMBNBfafAd62UVJGMag8UvICoRZIKvhzloapon7i6f9TI65mEVIVQQKPbj92u6JCfYI0mQMFF3mDZ18skgMBqrqadfD8NYt+QJ91Dy2NeVB2qWI5I=",
        categories = "shapes",
        tags = "healthcare,first aid",
        contributors = "lscheibel,ericfennis"
    ))]
    Cross,
    #[strum(props(
        svg = "eJxljTEOwCAIRa9CuEArQ+OgXoY4mJgOTnr7KmLT2InweY/vOBXOEbh5NIRQ+jgRuMoa3DHvweV0R6jUY4tQjUfqdDPTarTwgSmsoUgfdrjXjkomhhJL/H/V6g2abfaFH+EsNzk=",
        categories = "photography",
        tags = "aim,target",
        contributors = "colebemis,ericfennis"
    ))]
    Crosshair,
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Im0yIDQgMyAxMmgxNGwzLTEyLTYgNy00LTctNCA3LTYtN3ptMyAxNmgxNCI+PC9wYXRoPr6hDrQ=",
        categories = "gaming",
        tags = "king,winner,favourite",
        contributors = "ahtohbi4,ericfennis,csandman"
    ))]
    Crown,
    #[strum(props(
        svg = "eJxtjkEKgDAMBL+yeLea0LQ5VF+gjyh46EXw4P/RFBRFCexhlgybtrwXLEOzBijIRQGxY80MRl+PDXPxTvyTtkYnUmgzps40Y7pks0AL+Z8igiQH5yNqmIkgZ75Zb+z7vhLbyjYUvssDEdktxA==",
        categories = "food-beverage",
        tags = "beverage,cup,drink,soda,straw,water",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CupSoda,
    #[strum(props(
        svg = "eJyFjTEKwCAMRa8ScoESC8XBeBlxEKSDU7x9o1bqUHB6/PyXxIVUQo5QGC1CEEYyytrp3TFq73K6Iwgxngh1QAzjpalD1aZMUWdkP1Nh6F+tS9dWZD2+V1t+v035AQm1NmI=",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[strum(props(
        svg = "eJxtjs0OgjAMgF+l2X2V/iFLgCfQq3eCJJhwIGqIvr1lGsOBLF239svX1sM03ebHAP2rCcQB+ncTLMDdk3jyagptffhhbT13zxGuTTgLEHcJBIp8DBjLjK7EhmOChHKxnZaALaQbSYl6dE9V7cDEPnDRUfd6Alx05kt8PSkKKFqOXIn+WqOPhOLTIqP5V6OgEpBfp9Vf/t0fV+hFiA==",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[strum(props(
        svg = "eJxtzb0Kg0AMB/BX+ZMHsE3OwwvcCd06tGv3YoUWHKQ66NubExEHCeTzB4lt1/36oUUzJ/KE/5RICY0VFhtt66iOl43VsX+PX3wSPR38i/WmcLiuwR7CRSgzz+pghQ2H8wPLgwO4uos1CpET5kwdP5WF15xDteMFURAz/Q==",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[strum(props(
        svg = "eJxdykEKgCAQheGrPOYCNYqLAQ06QFv3UUKBC7EWdfs0Iije4ofHZ0OMa9oC8uFICPl0pAlTiSkpJyvqbPOwzqZxXzA7GjSMZ+kFGu09xWDxpuqKPpTVD6qXXRyJIcg=",
        categories = "devices,development",
        tags = "storage,memory,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[strum(props(
        svg = "eJxVi70KgCAUhV/lcndJL4gK6tzS2tAmFBhINDTk26fSD3GGj3P4jt3DEWF2OBAH2ZvEFLREIQIBAW8hRqP6Oit9Qm+7+vY2rdsCmRwKiXBWUqEo1Ai50FS3Wo/btv9F3xd63QtiVSW7",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Delete,
    #[strum(props(
        svg = "eJxdjkEKAjEMRa8Ssk9s0lRbaGfj2kOUKii4kEFEb29nlEGHwA/8vMDL7TK26wnas6AowliwZ3sVNBzy5nMd8q3ez3AseBDHCp51HziAgUJk6Sm+al9uHgP3IFkKmQtbA7YC5B9ojowTeQ6UKLJS4jgZTSY/Pl0FxDjWBOn7LDveglvYN0XLMwo=",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = ""
    ))]
    Dessert,
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8ldE9MG6UItX/gD7gVHBwcHPx/tGm3thwcd/A4LjzpveDczO7Ig2WS5Gi2oMZV8pfb07JCtgYQjQpgF2DUBRwSWCYOE8OUD8UPV9AfXg==",
        categories = "shapes,gaming",
        tags = "square,rectangle,oblique,rhombus,shape,suit,playing,cards",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Diamond,
    #[strum(props(
        svg = "eJwdy7ENgDAMRNFVLA8AODQUiTdgCEQinA5Zlgjbg91c8b4uazsN9C2YEP5dEaT1S6wgbQgjREfkp1eTcM6z/zjfhwnUgjsloCTTQt5c+QN1pBjt",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[strum(props(
        svg = "eJxVy0EKgCAQheGrDHOAagohQb1Bh4iUxl3IQHn7dFa1+uF9PFfSIVA9Lgh3jsIeaUV4dCgtc0vVcMoni3pwY/8Fd+3CED1uZMDyMFGnPn7IApkfvWOjIEs=",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[strum(props(
        svg = "eJx1yzEOgCAMheGrND2AWkwIA3ADD2GEWDZDmgi3N9VFB5c3/F+er3kTOEsSDkgOofaABoFz2Vme1ALOCP3e2pSjH/UX/bEKQwq4kAXHw0RKGt9kgMyPOSD7oQturyf3",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[strum(props(
        svg = "eJx9zMEJgDAMBdBVQgZQoyA9tN3AIcQW05uUgHV7G72IoJck5PG/zXERyMVhj5CPa3FMK4tDMgh7CsL3WXFAKDq9bTXn7TYLQ3A40QiGm46U9Pkg8yM0flDte9kJ58ovVQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[strum(props(
        svg = "eJx9zMENgCAMBdBVmg6gFhPDAdjAIYwQy80QEnF7qV70gJf+pi/9JoU1wxF9ZoukETjEjfOzF4sjwnnPVEPVKBLO9PLnzL5kBm9xpgk0dwMJyfFF+kdoalDta5sCUh+7AIy4NwE=",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[strum(props(
        svg = "eJx9jEEKgCAQRa8yzAEqDWQW6g06RKQ07kIGytuX7gps8z/8x382x00gXw41Qi6tOKadxaEihAfMCKXlmYJwm70d68/bYxWG4HBRBoiHSVVUxzdS+oeZDqOukfpC+vpuos8+hg==",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[strum(props(
        svg = "eJx1jF0KwjAQhK8y5D0xmz9bSHoCPUSxxRQUpAS0t3djBVEsy87AfjsT5/FUkMfpnEsSZAQWNi1wn4aS18sjCdb5bUu1Lu5qsIu3vmQMSVxpr1oDcrDKS97eKOPwEr2OtBfppeO3XyYt9IE0Qu2tjZ/eYwA1WWn6gzhBbot5hC3UoP1CT5+eQrA=",
        categories = "gaming",
        tags = "dice,random,tabletop,board,game",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dices,
    #[strum(props(
        svg = "eJx1ybENABAQBdBVLhbgH7pzGxhCorhSIeYXjWi078lo06gXV8EUF5JT8cdU7mRCsM8w3tlo4hYH",
        categories = "development,files",
        tags = "patch,difference,plus,minus,plus-minus,maths",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Diff,
    #[strum(props(
        svg = "eJxdikEKgDAMBL8S9gHaBI9pfuAjJAoVPEjxoL+3pR6kp4GZUd+zHxv5E8EC8rsxFwSYjq2bfl/xE/r7t53LlWiNmFmIJQ2Ba63WXl7ZHfQ=",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrhO6NJortUHsDDyFRUHAQcdDbG60FB6c3vJ8vyLzJMoKcrSE2IEfaTac0MRTpj2Ht9wmG1nQNEEtpCR06WyEDobc18h3fUQyZfKlMK8m/IvmHVMhZdKCm6h6+5gWkrCuC",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[strum(props(
        svg = "eJxFi7kNACAMA1eJsgBPHbJMRIFElSpsT8Tb2LJ8R9JUegUZBVNGUK+IILYmU9g/0+XscZ7H+tgEfjgWRg==",
        categories = "devices,multimedia",
        tags = "album,music,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[strum(props(
        svg = "eJxdzD0OwCAIBeCrEC7QymA6WC9DHExMBye5fSn0J3UiwPdeavUoIGHHQAhD56aDdI0IQnbOablUTp+Nbi3jyHGcMVmh3L3/yEu5dm4Ful5XBBZXPB7l/3wCrJcsRw==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[strum(props(
        svg = "eJxtzUEKgCAQBdCrDHOBsEBcqJcpSSFaiJDevpnRXLUa+P99xuawF6gON4RMZ0WIIZ2xOFQGofWiSfGko0TJvV145+2V7gBVOSRbV+o0bfgSb0ouWVbDSvghPUaE6x8e4UStP+KtmfQFoN8yTQ==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[strum(props(
        svg = "eJx1jLEJwDAMBFcRv0CQISEB2csIFwaTwpW0fZS4SePqi7s/0Ta0V1LL4AQaMSD1jANFtkmL9HZXshTwAhln7CDnefH0bdivVWRZ5POXfADjuiDy",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[strum(props(
        svg = "eJx1kE0OwiAQha/ywh5kZgq1Sdsb9BCmmmhijQsX9vbO4E9clBB4i+/xMaG/Hx5nHAc3UQLPnoIkUEieQ+wYopnQhDR1YJ6NakWbRr1oJq/Ujf3ORGP/0zEozRJERGs55NxqUvSy2DEXS9HJR9YU7YZqoRbZ5vAV3GBvOtsbuAXtTVyRi82hb9vawBzRocKyXTUzgbbmKjzWeQRlvP/hT3+93E5YeXDMDs9vkqbDWkKrVhpfrNBt3Q==",
        categories = "medical,food-beverage",
        tags = "gene,gmo free,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    DnaOff,
    #[strum(props(
        svg = "eJx1kDEKwzAMRa/yyS7XsmLZhjQ36CFKOnQpdOj9qaShdHAwhg/vP1l4e98/Tzyuy62A66FJtZGCJYkIMkomXfbt4rV9+5UHSjk4tdGJ0xgdJVXuJJar5Z4bVcsyUdkKB4WLcClchEvh4sR9cYN63e8Mr7B9iCeogWNJzE0xwmuqfia4ZAycMHXVJzNmD3PwfM4z2P7b+d/4LwfzZpk=",
        categories = "medical",
        tags = "gene,gmo,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere"
    ))]
    Dna,
    #[strum(props(
        svg = "eJx1kk1OAzEMha9idR8TO/5JpFKp6hYuwA6VRRcgIYE4Py8zhXbRajQeZxLbn5+z/Xz9PtHb4+ZZKjlL6gFO4+xKnU0bKUcOCnZqx6LctbFlMRZp+FtrFKMkrp2zNhJO9cUqsoQHCVYaUhClcEcUYXOkbWbI55vd9mFC7LYXFENErjjHivNtINIzcd5GQ2KHh03646ErHgJPOfOUK55y5ikXHrjdfQFCQm3jDlAnsZ/bqHF/C3VQOmBPoH8StJ/vhdPn+3IjwiA4zTDLvTQWx2r91OUx1GL3OMBBe9op2TEohcbTfvXZBWTpJdjMFu3q7FVC51aZ8kGHVsdHGajTcLB23XfuVWi1s9KS0I+4BnPBmETtEFsCs6jxj/4Lg1t75w==",
        categories = "animals",
        tags = "animal,pet,puppy,hound,canine",
        contributors = "kemie,karsa-mistmere"
    ))]
    Dog,
    #[strum(props(
        svg = "eJxVjEEKgDAMBL8S8oBqIrUIac69+IiCQgURDx7q723Qi4QwMOyu7NuxQqWIxAiVX96NbGyeUaWzlMqZrwJLxJkC+DQ5nwfnwb7/LpS/I3NptAkr6wPjcxoQ",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[strum(props(
        svg = "eJxFTUEOAjEI/ArhzligrrtJ27MXH7GpJpp4MBsP+nupHgwMhBkmUx7r80rnyidL2JOm1WINpCgVQxY/6hz0MvjlL0ySkcMxOkZggtMB2gUOE8xwURicW9mNnFb6bev3C/V3ZTWmrbIz9df3iqef3D5eCSHo",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = ""
    ))]
    Donut,
    #[strum(props(
        svg = "eJxty6EOgCAUheFXOaOj954xR7iSLVY7m4FocDy/UhiBnXT27bcnvwX37k6NoFxbJghp8/Q84vjBqsElW1uUrKc/SKFMRAOUdRHt9gElqxzE",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorClosed,
    #[strum(props(
        svg = "eJxtjrEOwjAMRH8l6l6Tc+xWkUIldlgZ2CIxZGBgQP1+riBVHSIPHt75ncu7flp4nocbUrCWqgYNkQNuXWHDUk5bZil7kiC21AFUkOQeoVBXiehfmfjEskngcwW7fx+MEDWVPF+d3rtvocN/EIczk+1lIy4H8BfisZd9AaQ/OCY=",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorOpen,
    #[strum(props(
        svg = "eJyzSc4sSs5JVUiusFUyNNIzVFJIroSxioAMJTsbfYgSOwAHXgvJ",
        categories = "shapes,text",
        tags = "interpunct,interpoint,middot,step,punctuation,period,full stop,end,finish,final,characters,font,typography,type,center,.",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Dot,
    #[strum(props(
        svg = "eJxljDEKgDAQBL+ypPfMHRcugRjwAT4iYJFGsBDfr7FII8PCwsDks14N++I2BSvFlFaDwYM7gYwRG5OlqhTQ5z8p74skKq7kuTdKHiUWsNzpb44INmhn0qEf09kfsA==",
        categories = "arrows,files",
        tags = "import",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DownloadCloud,
    #[strum(props(
        svg = "eJxNjDEKgDAMRa8SshdNtLi0nV08hKCgIFqwlPb2ptVBMryf8PKNn8MGi8WJCUjHfmZgaGVISRr1f1ccVY/ONOXJGX8d+djPFfy1n+G2OACJx9IDVHJVP8mZqiaySIyQ+GUWdoJy1sWv7gNOaybT",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[strum(props(
        svg = "eJx1ULsOwjAM/JUTu03spHlIpQszH4HC0BGp/L+4dEAM7WCfcuc7J5nfz8+K1+3ygAWYrRrssszXQS/zn5iQT7V6rmWdYFHPct0x9YAmCeaS2TbJElnmPYjDJYLERgkZ8SDCiiY0bV20aqVlNK0IR7NBDUXt3tidVfbKqJq7RN7VJZGI2ngqmDQjkS3Ma8TKVWa7M3FkvI3gDB1I58HONlR+Q9cotk8mYoHvmH6OL0h8XSk=",
        categories = "multimedia",
        tags = "drama,masks,theater,theatre,entertainment,show",
        contributors = "danielbayley"
    ))]
    Drama,
    #[strum(props(
        svg = "eJxtjL0KAkEMhF8lbL/jJZv9g71rrH0IWQUFCzks9O3NaqHFkZBkMnzT+nXttzP15+xYHK22Jkf99ZFL2339pd2PjwudZnfgCg4UMdU9R4iQaSWerKFKAonjrDroQf2xwsjmCop2n5DEM5S9PUaE54RQyIZssAUxWXqOXREyJasKFSrEGflHvAFVKzI/",
        categories = "brands,social,design",
        tags = "design,social",
        contributors = "ahtohbi4,karsa-mistmere"
    ))]
    Dribbble,
    #[strum(props(
        svg = "eJxNTTEKgEAM+0pwb7W9q0VQF2cfIefgKOjk6+05SSAkISTjud0H9qlZRaG6ORzdBycvHSkJJR4okbFdIY1yoGcrxAYNr8hRyIhs6SHCAoOkSvbfgz/NPLb1cH4B+JcZPA==",
        categories = "weather,gaming",
        tags = "water,weather",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Droplet,
    #[strum(props(
        svg = "eJw9TjsOwjAMvYrFnkecxE4ilUocgKknqMrAiAQTp+clQxXFlu33W9779yXP2+VRRR35SEgSpQRFy2wF0SRyUg+wGnh2TlVDhvatInVxVJMqRnYYs6LYYBRJaGVQumRU37Io18LGnR2R95QIbzJtymy/y7pcR6p1ObMRb04fv2tEZ9JZ43y0yYjpgFGOnyKdUg77ZF6MdgbbHb3JLJMUmKTrwNbT7w85Azev",
        categories = "weather",
        tags = "water,humidity,weather",
        contributors = "Andreto,ericfennis"
    ))]
    Droplets,
    #[strum(props(
        svg = "eJxFTztuQzEMu4qQXerTx/J7wGtu0EME7dChBTJkyulDJkgymLBNkSL38+nyKz+fhy8fVkOI3xrmaT20LC1Dh011c3zHRuBrtrZh3nzqZhUStqUSZJoPTasWEEnlirnyJ8yStDGpFVg7JkptODTRaktgbxfobFBJca64rRO6oGG2YCSvh+P+wQLH/Vnj393iXqPZooVwCiTlWQT74T6aMRJ/tT1gIccgpKPkraAmzZEw449ud9/X5hvxnkEj",
        categories = "food-beverage",
        tags = "food,chicken,meat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Drumstick,
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrfLpHTRpbh9i7CA4ugoP3x3QRhEiS6ZHPt2u7D+xrOsswox+zb2o2dmn2ujCEySewjAx/C4QXiEBJo0SUH/G0CZVqFKhe42MPCnM0mw==",
        categories = "maps,sports",
        tags = "barbell,weight,workout,gym",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dumbbell,
    #[strum(props(
        svg = "eJxtjk0Kg0AMha8S3E86yfwKKngADyFtoYVSuuhCb98XXdiFi0wm4b3vpfvM3wfd+mbKJJXTHDiRlSdBFfJX74RT4VadclJS9tEFjrkZuou5h+6PAQQMXBJLgC8WDhk+iSfqyhVRksaMvLxlWqq0O4WEc3AcIyKrAOZbyMtZrqgRRnv14CTjTIJ/mHH3tjZqVcCkHpzX832nRfpGG1oUDX3dx3UfITXR8AO260TW",
        categories = "medical,accessibility",
        tags = "hearing,hard of hearing,hearing loss,deafness,noise,silence,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EarOff,
    #[strum(props(
        svg = "eJw9jDEOgDAMA79isbekKQ0MpT/gEREMjAyI95NWUFmWB9uXL71PHOuwCRafVHxCNSFURdBOECfNgTRaF//ezaCh5LEySu6kkBqKbcZtanKWT1AGf2jC1K8vtAodcg==",
        categories = "medical,accessibility",
        tags = "hearing,noise,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Ear,
    #[strum(props(
        svg = "eJwtjrEOAiEQRH9lQs8JhwuYAI21P2Bn0EQTC3NnoX/vLGexL8PuMLulP5b+vGGpJkxi0L/V+HmoD5WnamW3mVp5Xd53XKs5BeTuLH+AXhsRBwUO+ykHBCSOZOWYzdi1LZbWYUyD+nKDWwS52mRnlkyi6eD2P+Wou7xj8oGVz3qVXtN+E/gqvA==",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[strum(props(
        svg = "eJxFjs1KBEEMhF8lzL3L/HTSE5gdEM8+hKigIOLBg/v2pneWXRpS1Z3qfNl+Xn4/6O20PAcsky71yWGhtEJ8UEc4CUPWLF9WXxtsvncjgQ5KpNOYLSaDejSwBnnZbALvY44d3gyhvuzbw4Tu2w0tCTclMazDH6WDBx2VL2f2r9xoHdZrUnorbrTiNuHCcH0QbQpeFc5aGe4xN8xxZ359fr/TWU6LLvR3yFlL5vXQis7Q/g/m5Tzj",
        categories = "food-beverage",
        tags = "egg free,vegan,hatched,bad egg",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EggOff,
    #[strum(props(
        svg = "eJwtjTsOhDAQQ68yoh9vMp/MRgJuwCEQFJQUVJyeBNHZenr2eK7XQfs0LFlIZCsQZSSnwD/Y4dGSc04MLWxQY0V1rqiFP6JwR7JWs5GjvIBeJchgShkS1JQ+28k9zOOvP88P0ZEa4Q==",
        categories = "food-beverage,animals",
        tags = "bird,chicken,nest,hatch,shell,incubate,soft boiled,hard,breakfast,brunch,morning,easter",
        contributors = "mittalyashu,Andreto,ericfennis"
    ))]
    Egg,
    #[strum(props(
        svg = "eJxtzEEKABAQheGraC6gmbJQw20slKy5PY8Fyupv6nujJddkmgRiT6ZzIERW2rwcRbUwUV+Jur1Av5Yvi38CsyaHDlC+IJg=",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[strum(props(
        svg = "eJw9y0EKACAIRNGrhBcIgxaBeZsWQbTO26eJrT4Mb2jNPZJghwZJysvRoFXXCkzZDNOXWJ1a3cQl7AW5OxW7",
        categories = "maths,development",
        tags = "calculate,maths,operator,assignment,code,=",
        contributors = "ericfennis"
    ))]
    Equal,
    #[strum(props(
        svg = "eJxljbEKgDAMRH/lcG80rVUK1dlBP0Lq4NCCg/+PSUEXOS7Du3AXr/0+cUxNGWHZ9OTUyXCVJY/OOOpzoMGIk1AIlSsUXfY0wCuHqv5rsrKTumaOrfbP8V3ZrBW+jP+keDAjIHzRA7T1JMo=",
        categories = "text",
        tags = "pencil,drawing,undo,delete,clear",
        contributors = "maxwellito,karsa-mistmere"
    ))]
    Eraser,
    #[strum(props(
        svg = "eJxtjTsKgDAQRK8ypM+abP4QAx7AQwQtUlp4f0xSiIUsszDzGCZf9W44V7FbaNU0i5KXkZX8JbalH6ATfA0UMKTGSUcseQuUMDQzeGg+FCxZGHKIHcX+uSNDUXYzW+/AA+FjIqY=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Euro,
    #[strum(props(
        svg = "eJxtzTEOgCAMQNGrNOwgLaYyIDdwZTdxYIDEwXh+i8bowNI0zctv2NcjwzarSgiEmjVXBj71aHy1IDO3VcUwNBnD6xcHyIYSoTCbxciFsEiggyU+GZ/cbVuwXVzR8qqb/vATduUvL5SdLq8=",
        categories = "text",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Expand,
    #[strum(props(
        svg = "eJxNjMEKgzAQRH9l2LvU3VSxkOTcS6+9Cy0oiAYUMX/vJgrKws48eIwN7dLh5+jDDdisdSsQlHpcaHtXdy7k21wM5a4mbx9pw9swDXHoxz/C1I/L7IgrGAgf75XFU/E2i5FVehK2lCUhiiOjqCGc/Ozu4nIpUw==",
        categories = "text,arrows",
        tags = "outbound",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[strum(props(
        svg = "eJxtjV0KwjAQhK8y5N01m/9CW+gBPERRQUFE0Ad7e3e3VF8k5JslmZntH/PrgtPgDh21BsUcEeHBchOFZHBjv1fn2H/97KlGZPJtkjFFrPQW5YB8rDp6VMPMkbhgpZl2TKUiUGl/2gsVhmKSRA4WVPF2gix4xq25o5pgWH8zxU7L+dd7u97PWHhwwWEJIqLvTe1ZrGoaPy7CPJQ=",
        categories = "accessibility,photography",
        tags = "view,watch,hide,hidden",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[strum(props(
        svg = "eJwliz0OgDAIha9C2BttO7gAN/ACbgZNNHEw1UFvr8DAx8v7oXO+N1gYxwK5XDUNkHsHBFL9z4X5jgmFOtsJ6d70WEEfxlwQ9I3fGKuVIpYPvqkZCg==",
        categories = "accessibility,photography",
        tags = "view,watch",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Eye,
    #[strum(props(
        svg = "eJwdjKEOgDAMRH+lmW+gtMsmxjQGi1+COIFAkAq+no2cuCdeXrnbAzrXsEumBawtUqR5jDu5bskN6hnmnKGXsIHtSE1Ifq8/C/QNtUwjVj9wLRT+",
        categories = "account,social,brands",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Facebook,
    #[strum(props(
        svg = "eJx1jbsKgDAMRX8ldA82QW2H2NnF1cGt4JDBwcH/xyj4RAnkdbj3ypwXhbFxHQP7bA38XrYp1fcHch8nDFAdo7woGm0fN/DgkhSbfZIzhAJQVPoi/EdekhVwBCwG",
        categories = "buildings",
        tags = "building,business,energy,industry,manufacture,sector",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Factory,
    #[strum(props(
        svg = "eJxtzzEOgDAIBdCrEHcQUNuaVG/gBdxMHBw6OBjPry2JDhoSFl6AH/fl2GAdqkmYgnoQR43vF0ccFKzzXYKBnAT0xKypo1YUhNruC6EQ4wmzgcJ/ZDHmExaE/zvtrPmUiem5GmOdA4zxjXH/pSexPLMLEfkw9A==",
        categories = "home",
        tags = "air,cooler,ventilation,ventilator,blower",
        contributors = "karsa-mistmere"
    ))]
    Fan,
    #[strum(props(
        svg = "eJxdy0sKgDAUQ9GthG5AkuJAeHY7IkhfQSfuvh86KmQU7rHiz395RvE7f+8ZGMEDEtgWsWMcIdk2y2QrURdkF2pAS18BljccAw==",
        categories = "multimedia,arrows",
        tags = "music",
        contributors = "colebemis"
    ))]
    FastForward,
    #[strum(props(
        svg = "eJxNy8EKgzAMBuBXCbkva4LpWmh9gu26u7CBgogHD+rT21gECfw/P3xJc7f08Mv4EUfSAEvJzoMHZ/cI1MQz3grsSL8c+0C6Y5ue9tqmcZj+sHJG9girZBSErcxQypYYNXSnL9KKY8WsVZe+9AE5NSWf",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[strum(props(
        svg = "eJx1j9EKwjAMRX/l0vfWJnW2hXZ/sI8YVZigIENE/950g+2lewkkJ/cmN5X7XB43lG9WxApzVlLLb+n6dFpxn17je8I1q4EY/DlXVEc7eF5MAHXamQ7cwGyNhz/CQ1VHOOPgj8S0qHXLPIIZTgc4hIZ5EDy1AMnLwfgxym0LgtUSz26Lf0OKS9U=",
        categories = "maps",
        tags = "big wheel,daisy wheel,observation,attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    FerrisWheel,
    #[strum(props(
        svg = "eJx9kLsKgDAMRX8luPtIJFahCm5dXN0LDh0dxMGvN3WwSFsJpSXNOVyid3s42MZiYeCK57Zi8KeRQujlRQbpVKaPPh/gKiZde8ekXxMSkJM5G+ZRqgElqpUyCFJEKLk/nVI6KZwBh2R47Hz6KMuPh5KeIb8ETwTXDVYDTAY=",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Figma,
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym4TW0kIUtqFFdbuKCC1UtVWparo3+NSkLqw+IZ7vrs03KYa7oW5OhCpXLZIngQZKCCTO3sQEvCkCgGkjuQvYuFIvhKbLZBH9RkpgOJIbj2n7ZlCjWLKdFhLyjT07dI23QOGvummZ2FYO0FPBE2MH/CLlCk3Y24fkF+KWQN5KYyojioruNma+ZvPuqkKu7KdwTIj/7HivJv4BhWQRhk=",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere"
    ))]
    FileArchive,
    #[strum(props(
        svg = "eJxNjcEKgzAMhl/lp3edjXU6UM87bNfdSydYKFqclPn2i91GJZAE/i9fWq/XEc9O3BWIRqk0gVDEoowedV7dpMor0PWcooy3QKJvT/t53/rZbc5OA/xsp/XVCck2cGtABZoI/hCG/x+ZqENWagUVtZLxIpRJa+xi3ADz7sRFwGzsrQUWHjvzTRN1jONNeaA+MJI90g==",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio2,
    #[strum(props(
        svg = "eJx1j8EKwjAQRH9l6X3XJF1jC20vXjzo1YO3UoUWSluwBPx7J0WlChJYspk3zKSY6rmla5mc7E625Fwr2wYXQ5bFkRVl8XGoeJYcA09noEerkT/4hiPOlsRFTfxeYUxJKYWgpCFNqmITc6piGvtH3w03msZumO9lYpWQopSRM5Qt4AsB/KlmoAa2tQOMZjhKJqx2xn5Zxbyd/sdo+Nto6I8RpfLAae3JA1oy4Fh95QljR0sW",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio,
    #[strum(props(
        svg = "eJxNTk0LwjAM/SuP3jub0GkO3c4e9Op9oNDB3AqOgf56s09KIAnvixdSM0Y8K3MnX5Tg67lhMNw8Vr+JMkAvR+IcsPy4FOVtNf9MHU5zYB3S0H27tn8hDW0/fipDXk26BOwgi3CTqHjvICA3SZQsZ6PeSgm89Qf1B6d4MEc=",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere"
    ))]
    FileAxis3D,
    #[strum(props(
        svg = "eJxljLEOgzAQQ3/FYr+0viPpkjJ3aNcO3SJ1YKnUATHw9RxEAiR0w9mW/fK/DD2+9+bFNkToIxWF4rqcuBp5CPxrTz0Gou9biM86npouXxZgl3esglYM5m2u1ITq6qXPefRjC6oTiRiimFDMNcXd1p4BOzQqBA==",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge2,
    #[strum(props(
        svg = "eJxNjb0KwzAMhF9FeJdry7+Dkydo1wzdDC04YBJDs/TtqyRNGw6EhL67Sy0vBR6duFkIg80EBIqlgZBKlO5KCoJ0A6n/D3kr6EWfLqu9T22u7zpOT2jzOC2vTmjLLI8IbI8b+EUYPhod6JANmK2Og9HDfu3y91PB4QmgvXRrLlU0qNGwV6OT7gd/ALbOM/I=",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge,
    #[strum(props(
        svg = "eJxtjr0KhTAMhV8ldK/X5GrtUJ0ddHUXFBREC4qgT2+sP3SQQBLynXOIsfXSQZOKEqMgBspVTUAQniV5W9E78KQOyT9IqpIgLi7zLjLzOwMzY6dhG/qxBTv14zKnAiM2cdNAIWgnvCUsfn9giV6l8nIepB3BD4LKof+LDus9N5k=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart2,
    #[strum(props(
        svg = "eJxtjr0KhTAMhV8ldK/XhNrboTo76OouKCiIFhRBn95Yf+gggSTkO+cQ6+qlgyYVJaooAcp1TUAQnyV5WzE48KQOKTxIqv5RUlzmXWT2dwZm1k3DNvRjC27qx2VOBSo2cTNAMRgvvCUsfn9giVmlCnIeZDyhD4LaI/2iA+sCN5s=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart,
    #[strum(props(
        svg = "eJx9j8GKAjEMhl8lzD3ZNum0FUbPHtbrHvY2jIJC1QFlwLc3nXbQixJIQv4vf0g39vcj7NfNzjpqgXlrY8/AYOZg5L9A7W8Rt/4loXaTazbdT3bYdOM1PdLpcoDxerrcb+vGOkU1RWADcQYrovBylGkVwApZHpA8icd5QIZrE9wkxHEwQIFJ8kjiIiXREmUgLyQr7d1cmYwHkwSriOqrK5htS1VXLLaotqrnoURc1IRlubfllKb6dLHGcvj/7f360VnxgE49lI3tBwC+ALsMTC/hCXGWYyA=",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere"
    ))]
    FileBox,
    #[strum(props(
        svg = "eJxFjbsKgDAMRX/l0r1qY3wM1dlBV3dBQcFHQRH8e6MUyoUQyMm51g3XjLFSHYNoNjwQCMkf0tQXUdYajjJQk4eTlu1mVdv4e6+tO9ZnXfYJ7lj266yUERtklKAE5Q96RGDfuKUw4pWwDq4XV/slhw==",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileCheck2,
    #[strum(props(
        svg = "eJxNjs0KhDAMhF9l6L2uCe2uC9XzHtard0FBwZ+CIujTm6qIBJIwfDOJ8+XcoEpVTiay4N+7ZDDiUFq2hR6CTG6In4Lm4hPZ/2neVOZeITBzfuzWrh1q+LEd5ilVZMQkLQHHSA7wQgS+fui/IBuOwGhzZ+2HsSkT",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis"
    ))]
    FileCheck,
    #[strum(props(
        svg = "eJw9jr0OgkAQhF9lcv2t7rGckHA0Nhba0puTBBICBImRt3fPv2aKnW9nppqva4dbMBf2cK5zkXLswZYcmMSSTyLkLZUqemoOlJ9ZFHMnH23CLYNc8sgfRR8zCDI1BPLITF3tUkldzdOwDf3YYp76cb0HwwJtERRwexRv8IvUVeyXOLSIz2AKg7gp7Q2WYHziPq5G/taXWse6TLPYk8sbln/xC3LMOUw=",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere"
    ))]
    FileClock,
    #[strum(props(
        svg = "eJxNjc0KgzAQhF9lyD3WHZM2hei5h/bau9CCgtVApdC3Nwb/GFgWvm9nfajHBq9SPQzIRkxNEHkKNZ+XzN7FZBa8nXek4/YzqvKn+bzyYej+Xdu/EYa2H7+lktiGOByYwyVxUaK8fPxcIQ6FTjl0rdhCqAukbHgCbGwtqg==",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCode2,
    #[strum(props(
        svg = "eJxNTssKg0AM/JVh79pNulYPq+ce2mvvQgsKVhcUQb/eREWWgSQk84gP9dTgW5o3uTQDPx81g2EViUwzRQvp3BDHi4Q/eZq9DvFqKn9Tw8qHoVu6tv8hDG0/jaUhJyIpBdii2IknRcjnD3+yoLvGKiKz6+5AuYYqrvsGYW0xgA==",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[strum(props(
        svg = "eJx1j00LwjAMhv9K6L2xSb82WHf2oFfvYwoTFGSI6L83VdEdIj205XnyJunG4zyeDjAX4w2M92KSXI9iyJu+W71p312G6wT7Ys4tZqCAwWJr8aVUtBA8MhBV+kcImIASZvSiKDx/uJUmpNY3QK5W65zR1wklRAyFN3UDqiE6zxigrfFeG29LAfiWpqSgABHjLgwMDE4OAVueGowbdpAFsfsxK691Wv4tYfymPgFcHWi3",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FileCog,
    #[strum(props(
        svg = "eJxtjDEOgCAUQ6/yww7yq0BMkNlBV3YSB0YH4+DphTjIQDq06Uvrz3RlOhax86QMYbUJBNJVsqSbm6I4MqMtJKJTZvvGjwh+qIfB/7cgHqPrkJlYZ9sHrgEvUZUo/Q==",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere"
    ))]
    FileDiff,
    #[strum(props(
        svg = "eJx1zT0PgkAMBuC/0nQH6ZtyOlyZHXR1J0IsmyEX1H/vnQ44SDq06dOPOI/XRE9jMM3f5ON082QcmB7TkNxYmV7GAu7irsx38d4np8H4LEpYgodCpflDWeCiPQjUfAIVLvu6PYnWLeEYVqpyteifI9KQwPOLDTv4uvUG/eQ2Rg==",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileDigit,
    #[strum(props(
        svg = "eJxNTk0LgzAM/SuP3utMtK6D6nmH7bq7sIGC04Ii6K83VZHyIAnvizhfTw2+pXpTnhjws6gZjDRAyzVTRMjmhjgmNH/uiXkd4VVV7hYKK+eHbuna/gc/tP00lopyCcmw4BR2N54WMV8/iMXOuoh6Tun/ABlkATq75A1ZmTCn",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileDown,
    #[strum(props(
        svg = "eJxFjt0KwyAMhV8l9N5MY5wKtk/Q3e5id8IGFqQV1pu9/WLZDyEhJB/nnNTyXuA+DhcGY9FdORMQaCkDpKgEdDNp8PIi/f8p2Ypy6IYpnbrGlNpWX3VZH9C2Zd2f42BYaBkBRCAc4AcR+GtrNLJQhGeTCcVTunv3LfpjzB6jAzIgelQxRmX7wSFbScB8+2V4A1iIMYk=",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere"
    ))]
    FileEdit,
    #[strum(props(
        svg = "eJxtj7tuwzAMRX+F8M5b8SFZApzMHdI1QzcjLZAARmKgWfr3oYO8hkDgFSWeS4nDPJ739LPqvpzK1kclpRRLSFn3FXmjiXrkraZnjSP79G49fCzu9TCfpv/pcPyl+XQ4nv9WnXigIZXCXa/gDQn4/qAkaKPQflS40VWW/okVpTCy7jgQiEZeYIw+PmQTwzLMly3C37mjkC3c9mpuccdoGc055osTlAy9bwoySZ0MhQ057wTKEUIC8ehXIY0X8vsx9AVCNUbF",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FileHeart,
    #[strum(props(
        svg = "eJxNTtEKgzAM/JXQd7sm0+mg9nkP7nXv0gkWOi1OxtzXL1WREkiOy91xOrRzD89a3DGXBdDt0hIQqDgZow8mBF/qkVIio0cpi2Yz/4TRpxhodBj94t3QQRjdML9rgTmbeFVACqpVuEuMtm6yvgO7sOwswH75KgFTLSgKtzdn7lVfnIBlhlJd15U2lhXDBrkZHWX+QQs5Jg==",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere"
    ))]
    FileImage,
    #[strum(props(
        svg = "eJxNjcsKwjAQRX/lkn1qZkw0Qtq1C926LyikUNuARejfdxr64sIwcM7cCakeIt6lelowR7I1g2FyWPPrWrgH2cKB75cdadn+VlXhNJ9XIfXt2DbdB6lvuuFXKpI2yPBgA5/FRRF5/SiGi2QONQv53kAeZ52z4Qmf1yz6",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[strum(props(
        svg = "eJxVjUELgzAMhf/Kw3tdE+rWQ/W8w3bdvbCBgmhhUti/X1tqrQRCXt7LF+PsNuLdN08F5pGUZTBkKhb8urXdg1Tbge/XwxJh8qoZzCWeD8at82+elg/cOi3bt28o0BCaBkvoFMyRED4+ElsCZWaYPBVNUaMonN20qf7vSA3SdUiQFxWzULN5tmVcFOgfdmdBJA==",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson2,
    #[strum(props(
        svg = "eJxVT8sKgzAQ/JUh99jsom0O0XMP7bV3oQUF0UAl0H59NzbESCDZeewwcb5fBzxbdae6asDXc89gmHi0TIEKQl4eiEtC8+NSNbf/8ld17hQDO+eX6TON8wt+Gef13SqqZUkuCzawmzFZxJw7GEg8gVIBmQJlTBEjIxzVjSkK7P8C2dKlKegiNMcm8SibSOTUHwvTRQQ=",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson,
    #[strum(props(
        svg = "eJxNjs0OgkAMhF9l0ju4bRbFZJezB71yJysJJCtskIO8vUXxJ02bJvO1My41c4erp4sFm9o2AoHRYkgmXZkXZzE45EUt5qdlup0sVW63nlcujXGJ/dAijf0w3z2xVVRHCb0uX+CGVC70U4gtwsOTJYRF6T1h8iQr91b15ZbrxupnMpsX0P6z/OhHMGta/kpPibk32w==",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey2,
    #[strum(props(
        svg = "eJxNTcsKwzAM+xXhe7rYLNklyXmH7bp7yQYdbFBKKW2/vg6lD4SRkWwptHXf4B3pydfKQe6+FghsgdFt4JOgLA3LWTDyulXusT7PlMKlBKaQv13+fZDHSGwJeVL2hC6SlKPVTmFr/7MHW1NSdI6Yw3dghmL3Fi7sLj0=",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey,
    #[strum(props(
        svg = "eJxNjrkKhkAMhF9l2H79Tby2WK3/Qlt7QUHBY0ER9OmNByIhB8PMR6yrlhZ1qgoKvQj8jysGwz9Ly7XSR5DNLfFX0FwmXpTf4V1l9ncCM+umfuu7sYGbunGZU0WhhGQYsA9zGR+LmJ8fBopBgQ6EJa0FnhtQ8lIPJsUrKQ==",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileLineChart,
    #[strum(props(
        svg = "eJxFjsEOgjAQRH9lsne0XdrYQ8vZg165EyG2CQGCDcrfuxjUbLK7ybzJjJ+aHNEGuhrY2jQMhpLR4IKjO9gLK5wOtmb11wr5zoYqf9zclZ/Gfu3T0GEa05AfgbQRVJaDuN0H3JHKz90tYxWmJMwvuYRnanMM5AixS/eYA1mCSLw5N14ivjUddLkUvJfRUIWBWvhX5g0qQTcS",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock2,
    #[strum(props(
        svg = "eJxNjrEOwjAMRH/l5D2ldklgSDIzwMoe0Yp0Q1UUCl9PQqW28uDTk8939hVSRO/oxsdGQy4mCARtHVVU5h0oWyLLHii5nxp9Xcxf8vZQH3o7DY+E99in6OhMiMP4jMmRIUyzIybMf/4pWqqr3nu7ldFgyUpCh67EcClj0GZZA37tYS13",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock,
    #[strum(props(
        svg = "eJxFjbsKgDAUQ38ldPfRa6sO1dlBV3dBoYK0BYvg31uLDy5cAjlJlJu8xtywQYBIczERCHk8SmisUtlzkUpQV/5WEtQhWKuyO94qZ7dzW80CZ1fj94bx0IbwalCOOoIPEuB3sQCXuvxaLoHNJKw=",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileMinus2,
    #[strum(props(
        svg = "eJxNjL0KgDAMhF8ldPcnwfoDtbODru6CgoJoQRHr05tWhxLI5Y7vosxwzjDWosMslkBNPhAQpG4ivi4MAlaakcIgor6IZfuVH6FV4h5qZfbVrss2gdmX7TxqgRmXeJVAKZQe/BGtPGiJISnA4qf372/2leM9+wKtDiwW",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileMinus,
    #[strum(props(
        svg = "eJxNjcEKwjAQRH9lyD01OybaQ9qzB716LyikUNuARfDvXUO1ZWBZeG9nY+7mhFtjLh5kEt8RhCuh5fVYhbP4KoCnw4qsbi9v2rj7nrcxT8N76Mc78tSP87Mxom3QUYMOdREXReXfRzVCErepWcgjQGj3KPnjD535LNY=",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileOutput,
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXH7otN1nYVtoE3D3rdwdtAoYOxFRyCf29bdUrIS0je470mDKvHtS3ObCHiZRAIVC4ppa/JnFiTgRzt71XG7VEVXbNL8q4Jy/ScxvmGsIzzem8L1pEawUEUXCZ+KJH8ddSkNJip5sGQ08igwLEdSer9n8WWsyJXge3BpJnhHdeBFXHds/Xpetm0L1GHN+c=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FilePieChart,
    #[strum(props(
        svg = "eJxNjbEKhDAQRH9lSK/nziU5i2h9xV1rLyhEEA0ogn9vFEVZWBbe2xkX6tmjKdRfg/Sia4LIjmHC6pOan+jUgF97oyRei1ale+3vpQtjv/bd0CKM3TBPhZKYhrhyMEN+iKcS5avxDTHePlIuYCFcbrABdJsr4A==",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FilePlus2,
    #[strum(props(
        svg = "eJxdjMEKgzAMQH8l9K4zwWwd1J532K67CxsoiBYUsX69qS0iUugr6Xsxrp4a+FXqg2XOQK97TUBQhJPJa8bTQEgN0nmQ0feR8zvGq7LmFhZa44bOd23/Bze0/TRWCkuJ5NJABehdTIo1u+hRJK1gCSQhRfpICa4yp0+O0TM1fLgbbUQ3Rw==",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis"
    ))]
    FilePlus,
    #[strum(props(
        svg = "eJxtTjEOwkAM+0rUPeaS67UgHZ0ZYGU/leFGBsTA6zmrUtuhiqxYtmMlv8unyuvaPaxHEr8NxcUlcLSxr+2Etr2a7wX154h0X45/3ZRPLJzyVhvEAuIMV7SQ4oyLWnGYECwhG9DPiEy0QBJDpIHIH4ijZhcbK4Kt3h9K1C5J",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere"
    ))]
    FileQuestion,
    #[strum(props(
        svg = "eJx1z8EKwjAQBNBfWXJPzC5prJD27EGv3osKLZQ2YBH8eydF2xxaAiEhj5lNiM3U0qNSV7HE9nY0xYWdKUjOvhESsmlpnN7s7yCGzYnSg7Rgqg6HlFCHOPafvhueFMdumF6VYgeErSQklzP8EeB/KXsSWXpYi5YsMhstU7jsKXZrVhpxQ6ExU/PPFvUFnStDKg==",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileScan,
    #[strum(props(
        svg = "eJxNjsEKgzAMhl8l9G5nMut6aHveYbvuLp1goWhxMuae3lRFJCEJ4fv/xKRm6uBtxRMrqYDudUNAUOYoePriacGdOqTzoqDXTarHJv4LZy7Z0Jk0xDmGvoU0hH76WIEVi7hooBL0Cu6IMz6MPrbgf4yhVAL8nAV5Gq0g7oxvEDsfD18lKcB6rZz6uL4A9nU2NA==",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "karsa-mistmere"
    ))]
    FileSearch2,
    #[strum(props(
        svg = "eJxNTckKwjAQ/ZVH7omdydIIac8e9Oo9oNBCbQMWQb/eNK0LjxlmeFtIce5wacTJgLkjExmMqoAln2tlj2SUBR/cj5L5emjRht1ib0OahufQj1ekqR/neyMopyEvD67gi3CTZPGn0YLqqKFzIpVUh/Vb4V5/BZvntgd5Scou86XfA0AxhQ==",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileSearch,
    #[strum(props(
        svg = "eJxtjLEOAjEIhl+F3F48KPQgqTc7nKuDWxOHDg4OxueXOpwOhvDn/yD56qM9O9yO05lnIEd9oTYGhjmGUrRT+eXEF/kyBHdD3chAUae1HoZvrbvVgKzTnwcZCoNjocYYotghHM2XT2xU0BVoAcphuaN7yuMiKDlFyHX3vgHedC2Q",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FileSignature,
    #[strum(props(
        svg = "eJx1jr0KhEAMhF8lbK9ncnpusVpfoa29oLCC6IIi6NOb9QdSKIEkZOYbYlw9W2gyVWIcJkD/X01AEPkKeFtQHHiSRZKHgKo0TIoT3lRuPj4wN27s174bWnBjN8xTpjBmiJsGikAfxsvC5vsHDfi1JFKEkD4KnPiCeEUyOwoFPgI=",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere"
    ))]
    FileSpreadsheet,
    #[strum(props(
        svg = "eJyFjj0OwzAIRq+CskMNMTaV3MxZeojKHTx06FB56OmL+5MpUoUQ6Ht6gnK/PBpcT9OZE0jXptNSDiNcyoaEIfVEWgOQIWVgUvQes2GuSAZhBM7wyzoea3DwSX7GyvkWIT53jmSwbmTjxkxCiaLL4qXvLa6899oMLH803rQX7z85Gw==",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = ""
    ))]
    FileStack,
    #[strum(props(
        svg = "eJxNjr0KhDAQhF9lSB8vuxc1RbS+4mztBYUI/gQUwbc3BlEZWBb225mxvlkd2kJUGsyOdMNgqCiWXOdJ+iedpOBf9pxk2LZclPZzvpfWz8M+9FMHP/fTuhSCghvCMGAFE8ELCfCVOJICGXxl1MvsaURmk3TH0tnIZTd5AJnKMFo=",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere"
    ))]
    FileSymlink,
    #[strum(props(
        svg = "eJxNTrsKgDAM/JXQ3UdC1Q7V2UFXd0Ghgo+CIujXm6qUcpCEy91x2vaHgaEULco4A6rznoAgdYj4OjEgeJNBComIuiLOms98i0onLrDSdpuveVpHsNu0HnspULKJhwJKQb3CX8Liv8OiAHOX6RBk+YqcoIz0nwcasDCA",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere"
    ))]
    FileTerminal,
    #[strum(props(
        svg = "eJxdjEELwjAMhf9K6H2zjdvaQdezB716HyhsMLaCQ6y/3qSdUkcgL3l871nfrwPcOnFRVVkDnpoeAUHyFHQ9VWaQ4qAwNwq86rI+p/BbOHvgQmf9MoVpnO/gl3FeH51QFYVoGUAJJoIb4mwEAxJ0FBBU0hf9hoTfhvmM3cwvEyNapAq9Y9ls/+tkirQ/8gMuxUHK",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileText,
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9lyD21OyYxh7RnD3r1XlBIobQBS8C/Nxa1PZSFZeG9nd2Qujni3qirARnFdARRL0XN26myFzGVBc9uRbpM2ag2HD7rbUjT8Br68YE09eP8bJSUNJTmwRp+Eb9KkX8Xi3HMWqLLsola/xEfuQMshNn9wRsecTR9",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType2,
    #[strum(props(
        svg = "eJxtTssKwjAQ/JUl99Tu2MYIac8e9Oq9oJBCaQOWgH692yc5lIXdZV6MC83o6VWpBxdZSbiZBgTKp9HyRU4AufCMFNB4XrLyvph/qnanKbB2Yei+Xdu/KQxtP34qxYWYZFlCTnYWrhIRbx2uxOeo2ZvISdTekImtxxEj0YhmZ/5ndDhm",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType,
    #[strum(props(
        svg = "eJxNTssKhDAM/JWh97omWtdD9byH3eveBQUFHwVF0K83VZEykIR5EeuqpUVdqB+lkQF/sorBiD20XCsFhGxuiUNC8/8dme8V3lVpX76wtG7qt74bG7ipG5e5UJRKSEYOjpGfxtsi5ucHsfCaBTW3MpABGZ14IHn0A1zwMLs=",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileUp,
    #[strum(props(
        svg = "eJxFjcEKgzAQRH9l2HtSsyRtDsZzD+3Vu1RpAlbFhrb+fVcRZGEYmDez5dTkiDbQ3cLXtmEwCjkDVhy9djcucNGu5uLIlLirpao8re2qnMZ+6dPQYRrTkN+BjBVUxEPafgN3ROD94cvIktMOwmr3Uedebe6YnbtHxiJrTPgFEv2mNsdAnhC79Iw50JkwS2bW1spXf+LINyM=",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVideo2,
    #[strum(props(
        svg = "eJxNjrkKhEAQRH+lmHzc6V7HNRiNN9hNDcwEBQWPAUXQr7c9EGn6oHhVtPPFVKNM1J/CwIK/UcFgmL20XDM9BNlcEz8FzdknsL/TvKrUvfbA1PmhXdqmr+CHpp/GRFEoJhkx2CA+wAsR+PqhIwMiWLy19Kyj/A7cAFeUKm4=",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere"
    ))]
    FileVideo,
    #[strum(props(
        svg = "eJxtjkELgzAMhf9K8N7MpFY7UM87bNfdxQ0URAVlsP36PecoPUihSV6+99pybtaOHlVyk4wd6SVvlJTS7Rh0L4kEVO1EY8HovWB33c2fpC5PW2BdztPwHvrxSfPUj+tSJZLBhMuTpuR/4B8BHP4giBHLrmWrjEw+b8FQF8Pi0YjZ5+ilYMZCW84zBgjGFygWRpuTshoMBy6wruNUwuoLppFDOQ==",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVolume2,
    #[strum(props(
        svg = "eJxNjsEKg0AMRH9l8L6pya66B/XcQ3vtoTdpCwpbXVAW+vdNxbYSCBMy85I6dkuPe5OdHUR6dp1AkK8lRi4VFSd2VECO5X9lVCWbtfXhE2/rOIVXGMYH4jSMy9xkrDRo85AcfjVuFjVvF58VODdWyZJcL0FVMv66o35fYwbzjUpHHirJaxJ2NmRLCInR4Zd6A985NVM=",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere"
    ))]
    FileVolume,
    #[strum(props(
        svg = "eJxtjLEKgCAYhF/lx13zPzQJzLmh1nahwbEhHHr6lKAc5IY77rjPn/FKdMxiY6MsYRkjCKSrZEmZm6I4EqMtJHan7PqebxH8UIHB/1jQlE1/YJeU5m97AO/7Iks=",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere"
    ))]
    FileWarning,
    #[strum(props(
        svg = "eJxtzLEKgDAMRdFfCe6t9plWh9rZQVf3gkMXwUH6/cYOKihvCRxy/R6PROtQzUxAMhxBoKYMCkun7WRYW8LoHlJyZa6Cr6/34O+IkUp2yX1pa8lAOrIf7AuqN55zTScl",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX2,
    #[strum(props(
        svg = "eJxVjMEKgzAMhl8l9G5ngp0KtecdtuvuwgYKooXJsD69SVWoFJqfP98X69u5g0+jXlhoA/S4twQEubyM0x+Tgid1SGmR0bvU5rnLq3L2Jged9dMQhn78gp/6cf41CguW+KuAcqgieCDORjAgQ6SNgoUElxQklbHjbc2BvdS57HdH7DrK58HT2QCgQDo0",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX,
    #[strum(props(
        svg = "eJxNTM0KgCAYe5UP7/340Y8H9dyhrt2FAgVRIQnq6dPqIINtjG08qKhhE2ShXd0DToNCQGgzquROWgRJUVMsgwrXse7nb3wTyZt8KHnw9rLG7RC8cfEQhHZplIgBtsDe4l+RDwRlIQw=",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis"
    ))]
    File,
    #[strum(props(
        svg = "eJyNjqEOwzAMRH/FKvetTuokk7LigY2OTx0IHJgK9vW11aqooMSyzud3V7/vX6PPrXuKQincC9LEGKhnFAQWCJQRERmKbINMmiWgTD1hgC2r5h7kbdd2NYNjCIURXHRAdJTxzOfoV4I+1uB/N9aLlxnrXilSRjqXdfAtRp216X5aALdbNbI=",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis"
    ))]
    Files,
    #[strum(props(
        svg = "eJx1zcENgCAMheFVmg6gQVQ8ABs4hBFiuRlCFLcX4sUYe+2Xv09HvyY4g0tkUEwIl0GJELPBDoF82Cg9kCtY3dbA6n1JBM7grEAehQvU0wskqGag/ldER0wjRiYS7FARxUefhzcEmEOk",
        categories = "photography,multimedia",
        tags = "movie,video,reel,camera,cinema,entertainment",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Film,
    #[strum(props(
        svg = "eJxty6EKgDAQBuBX+bHv3N3tdIO5bLHaBYNhgkF8fjEIBvPHl4/l3LAOzcRKnhU6So1IFLqZUw2Qy0WyUCk5Jm/WlNw+p+R37iJQZ/gj7qH40g1QqR0m",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "gubser,ericfennis"
    ))]
    FilterX,
    #[strum(props(
        svg = "eJwti0EKwCAQA78SfEDb3UqhoH5HBHEFvfh7WfWQEMKMq5JHlIIqqfTmDTNeaOgB8WW/NX6QBZP2PhUzwd1HDxOwCBH0",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "colebemis"
    ))]
    Filter,
    #[strum(props(
        svg = "eJxtkDtuAzEMRK9CuOdEpERKC2zcuM4hFpvCZYrcHx7ZgN0I+9FvRvPI/e/4v8vv9+XHxfzmkojnN5fihxXhOx+TIe1y3b+m47q/fSG2IW5Bjw1JsZg/P4uiw1wNtaM29YXXOnwTNxQ/pxaJVtVREVq5ubIQrBxEe0IV5ewkHMUKE0eYwnNJOpCsi0lOTTIqJhwtfQ3XxCpmJby2DqYlB2UfMMZCT7C8o9jqyDB4ymTywqppoEYjfi7UG4PGkWzjq++bBCYFWp9rWFeWKx/mBwdGaeE=",
        categories = "account,security,medical,devices",
        tags = "2fa,authentication,biometric,identity,security",
        contributors = "karsa-mistmere"
    ))]
    Fingerprint,
    #[strum(props(
        svg = "eJxtUUFqAzEM/IrI3aplWZYMaWDJpZc8YkkPOWShh5L3d7y7LKWUhVlbYkYz8vlr/n7Q5/vpJkFSuPqLsy45sb1wWRLAyDhbmxp7rbRhxickRhL3pGyNcnL8UmHTFGypEWuNJFwgzCIVrapKhaUIKBqLcq2jnCOm4MiFNjzE232TxtDcwLRKTi2x5j40a2KPURd3DKqipHAap8v5beS6nI90TpK5+dUpkMYGdAfTdVzvCePgWciIi2KcDeN1K46Dpb1+XdkS2BK8SBuAMLo4I21l9TJ1dqcV9iRtBPAFZ+Fo858+JngJrKTJP9YXARcazl1XEwg6FTq2pBxU5KOzzcjUaIW8dbl2yPa44UGwm4dx8fkXFy1A8+eue0PvWTKVfPj4AWW/cBc=",
        categories = "food-beverage,animals",
        tags = "food,dish,restaurant,course,meal,seafood,animal,pet,sea,marine,allergy,intolerance,diet",
        contributors = "jguddas,kemie,ericfennis"
    ))]
    FishOff,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVIwNCu21DU0VTAy0DVxNjRUMDJWMFKwAGElOxt9kEI7ABKqCyk=",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "danielbayley"
    ))]
    FishSymbol,
    #[strum(props(
        svg = "eJxtUUFqAzEM/IrI3VPLsiUb0kDJpZd+oLclPeTQQqGl7+94s4RClwUhS6OZkfb4uXxf5e3x8OJoouWCUZOhutSZuXQ0RkNzyeLILgWtSognAtioMfGVb9a+UhCZCLG0Tr4eTseHqXE63pW0U+gHba/looFhy0CErCHzU8lJFd13RkI0w+Mc0qWhzTCCJsPm85JUlEZUmqAYN2COUm/FmbS01c/rtHZk43p0wmAw27OZ54UCxc9MC4V6p1sNXo2UXezK0lKkbPYVg9S0+T7FFPU/6YfyunpbP22opz8Mhi5Fnwfawg1d1pBvXdTBnzD6nfUXOYFnOA==",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "kemie"
    ))]
    Fish,
    #[strum(props(
        svg = "eJxtjEsKgDAMRK8ydC+aWMFF7Q3cuhcVFESEirS3N/W7cRFeSN6MWdttRF+pugR3OTIUYMjudEKQ2YmUNWnUrHllDeZG/z6ocHRFn6pPm6dlQKBKsYJngTDc9OdZ1CjZA5SHJoo=",
        categories = "account,social",
        tags = "unflag",
        contributors = "karsa-mistmere,cyberalien,ericfennis"
    ))]
    FlagOff,
    #[strum(props(
        svg = "eJwBIwDc/zxwYXRoIGQ9Ik0xNyAyMlYyTDcgN2wxMCA1Ij48L3BhdGg+p+oJQA==",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleLeft,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVcwMgozyjE0UDDVBRFKdjb6IEk7AKZ7CRg=",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleRight,
    #[strum(props(
        svg = "eJwljDEKgDAMRa8SsgdJTKBD2xu4ugsKCiJCHaqnN6XD4+eHx4/38uywJpwU2AoTgzoGAsHR3uex+MHkDhkJBUd7/zDHoY3keB7XBlUSKsLrwebJCUUQKre3q03KPzqmG4Q=",
        categories = "account,social",
        tags = "report",
        contributors = "colebemis,ericfennis"
    ))]
    Flag,
    #[strum(props(
        svg = "eJxVjjsKw0AMRK8yuJey+u3a4BhygBxiSYqUKVLl9NEuboIQvEHoMfu7f154Xpf7ygFxjpsmjC1zRCD6KCRsK3GQkpBlKk1JWdyIVZ2cSziUKs5vhfMGR50sXGGwZENw9IY23BASR5l6CWM3S6lungfr/0XO9F2O/TJKHz+k7yUw",
        categories = "weather,social,gaming",
        tags = "fire,lit",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Flame,
    #[strum(props(
        svg = "eJxVjM0KwkAMhF8l5L64E5bVw7Zv4NV7qUIFEQ+y2Ld30l9KCBMy30z5dN9B7o1ekQW5ps7EJHIQeA3h8Ah2Q+yjH9MkbcvJG9qy9ZyZAmrqo5hXcFPFDr6e74eM1mhW+VFwoYIKlRH+JurQgrpns+XChNmcpK7oH71GMNA=",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[strum(props(
        svg = "eJxNjEkKgDAMRa8Ssi8moYiL6g3cui9VqCDiQsR6ejsolE/4A4+Yw54e5h5H7qB1BKIkn76YbIxAUZxGr3Q9KJmYHKWQpSfxLA8Opkk/B7Ot+wJBemwRbi4WG3cIIddIJqYiWT5GCpN78R9+AV9IKlw=",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[strum(props(
        svg = "eJx1jFEKwjAQRK+y5L9rdppkLaS5gYcoKEQo4odIvb2bgqjQsssMzBsm36dHpfPoTuLJPrCC4DnGSUjIr8cDCYdYBazpL+9aPptC0YFjgCv50DZL/l3GE9yHfgsGg2kHHjkSqm4QJUl1+IL5ervQIqODowVm5q+Pr7FVW6m8AYJXPKk=",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,non toxic,lab,chemistry,experiment,test",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FlaskConicalOff,
    #[strum(props(
        svg = "eJxtjrEKwzAMRH9FZLeqE5YVgxvo3q7dDR08dOhQ+v21MxgCQYt4D+6ufOq30eu6PCCkP2dTr0pK0g+BFeA1p3tk71TYrIKwWyHOBI7WoOzpwMPg72Asqf/CUL/NVEKkPIqeumzlMhZsZe5Y2UibnxgnpAaZ5g8VAStI",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    FlaskConical,
    #[strum(props(
        svg = "eJx1iysOgDAQRK8ywbN0tz82Kb0BFt8EUYkgnB9qwBQxZt576Shnxb4MKxvIFcnykNPU3pw+5qBkNybVDp3JQ2r87Up4hDYDBo8OpqN68gIOlYU0vPwGYNMp/g==",
        categories = "science,gaming",
        tags = "beaker,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,danielbayley"
    ))]
    FlaskRound,
    #[strum(props(
        svg = "eJx1yiEOgDAQRNGrTOoJ7ECziKU3wOJJEBgSBOH8UNNUdMVX79u9PyeOJVwjFBGx+9s0JOuzJCtOgWaEM6xCcHjZFpk8mT1gBR/RPC4Y",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal2,
    #[strum(props(
        svg = "eJx1zDEOgCAMQNGrNO4gLWgwQWYXD0F06OhgOL80JugA6dL05Tdc6WY412H3YLcpERAYGVW2jO4wgBr1AgLEdohhlCSGGuIMlm0tEd7yO8grVs20iMnUFnQ98T2gHzz3SDhV",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal,
    #[strum(props(
        svg = "eJxtyzEKgDAMheGrhO7B5mnVIXZ28RCCQxbBwftj61RKeOP3P33O1+jawi0LjZwocZlJDFmHalnbAsJ/QH5yTCTY4YDEKqsncxFj7wR09AHzYC89",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    FlipVertical2,
    #[strum(props(
        svg = "eJxtjLEOgCAQQ3/lwk7kihIGZGZxdSdxYHQwfL/eYtBcOjWvr+msV6NjNRuY4r5UEMhJLCzKpxO6NzlNouQ0ihy6f5csy8FkeepWU2diFCiAnZCokfCQZjUJ+KEbbso4VQ==",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipVertical,
    #[strum(props(
        svg = "eJx1jlsKwjAQRbdyyf/UTGLaCkl30A34V6JQwYIUKbp7J7WP/JRAJpw5cyf+1b173IJq2cB1FhYaLEdeA1myG9KUkJCJ2wvqnSc1kZ4HB71HrH5PPJCBnYhV409pYePjY4zPO+InKDYKY1Byx29QdXL+3cbnn2M9ibkF5C1j4rkwsrUiLsqykupoJlRhJlLd9XB4dWlxyWHJw5KXT/8AlatIyA==",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[strum(props(
        svg = "eJx9T00LwyAM/SvBezuj0ymosFsvu/YubrDBhFF22P79DNp1ZVAk5uO95CXuEZ9XOHt2QgGHXsV9r4CMA5bXsgYef0FOFUDRsNHmjiAZ16R5QiUPNlPA1zJdy3IxigdUuZNUGVGx4Ha0ZHDpNqX7BdLLMxQM0rv6yTNJpAoHN1+UDaAGLCOLLVOWg0nV0loazD9e2g11w0Z3/YqI/jI+s9NPlw==",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere"
    ))]
    Flower,
    #[strum(props(
        svg = "eJxtyr0KhDAQBOBXGbYPdztB0iTW11x7veSECBYiEvTt/Wu2kGlmmC/mYc5jj7wlUQrmJF6Q12u18XXfbZy6peCf5OsRfk1HEO8jCjqWS57COA3whRaC9QlSoaEa6Y5W3BMNoH4aKx2rkTud/TR3",
        categories = "photography",
        tags = "camera,lens,photo,dashed",
        contributors = "karsa-mistmere,danielbayley,jguddas,ericfennis"
    ))]
    Focus,
    #[strum(props(
        svg = "eJx1yTEKgDAMRuGr/HQPmtSKhdgbeAjBoYvgIJ5fzGCXlLe9T6/9rjjWsAlY6hyKDt8q2sCEPGKBPOLD0gOeeiKjJydnZIqwHE7ghEjWzy/WfDvl",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldHorizontal,
    #[strum(props(
        svg = "eJx1y7EKgDAMBNBfCd2L5mpLhdjZxdVdcOgiOIjfr1nEIeW2e3dyblelfXILg4DbJ1ek07LIn/IKAwZizBZwr5ItSa9Ub52AFh0ciUcfNBRsj0o6+PgBbzg8QQ==",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldVertical,
    #[strum(props(
        svg = "eJxVjsEKAjEMRH8l9J64mZXuFtqCNy9evS9VqLAHWaTo39siVCWHIXmTSfx9eWS6BHMCCMN5XqrQ0IrByDyJG/tMWcVaFreyzKgNDt1OzUnjcf+bQCg6prooKo4aQLYm+l07G326bWm90hYMDKVnMGqrvqq6Zvrg6PuPakm1sH4T/tBUGB29ARr9NNY=",
        categories = "files",
        tags = "archive,zip,package",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderArchive,
    #[strum(props(
        svg = "eJxNjr0KhTAMhV8luCfXnJZeC73C3VxcHdxEhw4KDuLz2wr+kOnLlxNOWIct0vQrWksoo7oBBCrPAaOrHubEkb/izb1TVnGOxc8sFRLg/8TzJZnGvj8QdjVjCoqKpyzQF3X45BZ1uLosntSc1rK99QFHySWH",
        categories = "files",
        tags = "done,directory,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderCheck,
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6L0xyWa10Pa8i1fvIwoTepAhov/eqKMbgfCS95GXdB8fE1yyOx1AaOhHAQGyYi9eznslz8gYve29TB3GbkWAMQSMFY9iUppBizEIbWBTrqTdN7Akvc1ar6Dv7Dg4mLOzrq/fZNDfLql9xwG4f0q1zHbkA4tMLLA=",
        categories = "files,time",
        tags = "history,directory,clock",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderClock,
    #[strum(props(
        svg = "eJxNjq0OhTAMhV+lwbd3PSO7LBkkOAwWgSMgJhAIwvOzIQap+np+csKxnJG2thprgonqFhDIPAfG1LzMiSP/xdvyU1ZxjsXvLA0SoH/j2Ul2qL8NhEvtmoKi4ikLmKsu/PKKLpQtIDURpig3lK4kzg==",
        categories = "files",
        tags = "directory,closed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderClosed,
    #[strum(props(
        svg = "eJx9j8EKwjAQRH9lyT1jdtPGBtqee/HqvUShQgUpIvr3Ji3YHlLJIdnM25ndOtymMF4pfBrFlaLwXu6pUVa19WGR2/rRPwe6NOrEBiWJ6YpeSMjEw1q0nMu1plgPFn7zw3AefkSVnvITTBKO8J2YbTvJy6JM8Sl2Db8L40jsUWh4DZshuIQQuyTvEQ6OkhFsZHKAR5wy+vwBFgcdB+F8RJUc7C5QwNJswxHJLWrSorPNFvgC22xqZQ==",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FolderCog,
    #[strum(props(
        svg = "eJxNjjELwkAMhf9KyJ7Y5MrZg+uBm4urg1uJwgkdpIjovzenYCXD48t7LyTfpnuF84iHHrSrEicFhe4zSnocVibnSltO4bcTEo6ROM3Egzrobq23JIR9/38B9CHBvMjCCZqhJyx5074o2a6LzRew14gSEBYXBHu6aAt97fIGkMIojQ==",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
    ))]
    FolderDot,
    #[strum(props(
        svg = "eJxljrEOgzAMRH/FYrcbX2hKpBSpW5euHbqhdshQJAbE9+MwBCR00/n5TpemYc70uzevluCyhgEEcpvAeHe7Z/OZbxJ9vSmrhMAS/ywdzOCxx8sn+Wd7bCAs6r8WFJVIBeDT9OlSVvSpblGQuiWcyahXUs/Wy6bKV7zfLS0=",
        categories = "files,arrows",
        tags = "directory,download,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderDown,
    #[strum(props(
        svg = "eJxNTbsOwjAM/BWre47YeVBLoTMDrAxsEQwZOjAgvh+nqqIOtk73LJ/6bfS+TPcZUYg9MlcBUz9PTB3peXu3BE3ESkLiV6i60ImEGJy9+JyWcup9SxmtVslIj/TyjsFQZ1knzYKhGrSJbQQ5Q1fMZocMwe/CVfzBbOjHB6JXNqdIY/4Pc4suCg==",
        categories = "files",
        tags = "directory,edit,rename",
        contributors = "karsa-mistmere"
    ))]
    FolderEdit,
    #[strum(props(
        svg = "eJxljk0LwjAMhv9K6L01fWe1gbbnXbx6H1WYsIMMGfrvbafMggTy9SZPEu7DY6RLVCchcL8fQCAuZjU0zu5XU6nHzkjTseYgRibja4pN4CocjfTgdp2wOJXCrl5MId/mPF0pP6OynaL8KhGK5qhQhz5yCtt71pOVrGE8sXYloni3+D/iSli54C9XGuIbAFA6bw==",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[strum(props(
        svg = "eJxVjjELAjEMhf9K6N54SUrvCm3BzcXVwe2oQoUb5BDRf2+r0HpkCC9f3uP5+/zIcAnqaICHTHZmYBi+w5pPU9e66KxHdNJupAmt1egWjRMXwftur58gB/OfAPwkScWIhA4q4LOKfldbRJ9ua1qukN5BkShIr7JZwRoU16cfjr5VJgMkWXpAI+MWfAAeMTcG",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit,
    #[strum(props(
        svg = "eJx1T7tuwzAM/BUiO6/iUZQtwM2coV07dDPcIYOHDkW/v1RQOFkCCdLxcTze8r3+XOXr9fRuJiyXulIoJY8plR+xFTUYumZeeXV0v7eIoTX0HTMT8iiU/8KF5aE50a8hTuflZaiel0ObSe5ijmklqsvtuQ1SooUiuCkcxoQTXDEjfM9UzZtfDPiM63duG9ypZ07RA71qrpURKCle32xCCG13NHVEbLlY+udwajUHzrCuo/XzsPEHPFZDyw==",
        categories = "files",
        tags = "directory,heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FolderHeart,
    #[strum(props(
        svg = "eJxljjELg0AMhf9KcE+al2uPBq7OLl27Sx1uUHAQf7+niBzIW77kC+GluV8yDZ/ma+S/118ZAnE2MrYcxENfkLQEBIlRfJS3FbRL6Ck60+q40Ipqsb/snvXMtjKaNj32Dm2qmiBk6N1MTogU+MilN2B/LLw=",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderInput,
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/F6m4TXyKTSKESGwsrA1sFQwYGBtTvJ2ZIGSpPz89nXX0vn0bP03RNhNDUFhAo/AaMW96YOzc+Soljp6xixlJeLBkdcN7ifknxkv4/EFaNjx4UlUIucJ/mevAWcx1dMmlY045QuMGeMTc2zBdtjTNy",
        categories = "charts,development,design,files",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,directory,project,root",
        contributors = "danielbayley"
    ))]
    FolderKanban,
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEBLyPfJeuk33GU7ZHJhAaHSTgABpsRUrR1/JMjJGq3cr84Bx6BJgDAHjgnvRVTZAXzAK/Yl1e4gpadc8S6qXtS5nqM9sOBior2yEDKw6muiDS/oFvIp6OOvQg3Z/07nG8S3Txt7JQzT1",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[strum(props(
        svg = "eJxNTssKwkAM/JWQe2KSvmG35168ei9tcQsepCxV/95d0Co5zJCZycTdxxhg9nhWAZOhHA0MJI2SkV2qSUhZuaO0JwsFd8XPAsp1zd2NW0vUDkE+wmDyZ05sN66wd6fc2rttmSK8PGqD8ExQImwZEcKyXkP0WCE81jkGj22O5UDvjpctXW12+vYqCJUgux0Nb99rNL0=",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[strum(props(
        svg = "eJxNjjELAjEMhf9K6J54eT3rFdqCm4urg9uhQoVDHETOf296gycZHi/5XnjpOb4qXbM79oSuahhBoG4ZME7D6tl85Z1E/9spq4TAEieWAWawX+ONJH/o/z8Q3uovFhSVSO2Asytp01qUNN0fN5qRnW4dzZpddPQxUW+KRY1tVPkCP3soiA==",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[strum(props(
        svg = "eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA+5nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171sg/4b6iUL8pa7qzyWm3vJ9TPc+1naA+BkPeQH3qFgOzrqX00fkFjJkzVg==",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
    ))]
    FolderOpenDot,
    #[strum(props(
        svg = "eJxVjkEKwlAMRK8Suk9Mpj+fBmrB3T+B+6KLLiq4EM9vFLGVbIb3wjDjfX4sdD12t0pWyKQ4Q+IEAmmeUQiSa4POGzSJQhBf2cSd6qb4o5K2soNgnP2iKVNzcsbSS/R/lbVKrDIgI35Cv6LZsHvO9EQ3jYf3/OkFiKsoRw==",
        categories = "files",
        tags = "directory",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FolderOpen,
    #[strum(props(
        svg = "eJxljbEKgDAMRH8luDcmKVUL1bmLq3vRoYOCg/j9tiKlILc88sKdO8MVYRubWaBHs5iVFCOjVQKiJGq0OiQESmFg7Dq0Ow6SUIqgT3ih6jnRzdUhV3ppJtfm0clV06wj098cBpiUhjdFPxuHKY8=",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
    #[strum(props(
        svg = "eJxNjsEKwjAMhl8l9N7Y/J11hW7gzYtXD96GChWGeBCZb28zyjZyCF/y/SHpPXwy3TtzbgguSxhAIDcXLC7tyrZwtgeOfpmJFQ7BchwttyiA4xpXk/yp2V4gfMXfSpCFI+kCV9OnnX7Rp/H5etAknREY+qH0YGhCZZ07lVWr8iz5Ku2rpFx6XNw/XnAztQ==",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[strum(props(
        svg = "eJxNjj0LwjAQhv/KcfudvUsbG0gCbi6uDm4lChE6SJGi/95EIZWbnns/eP1jema4Bjz1oF0WOykodN9T0vO4MRXOtGdn2k9I2FpiNxOPWkAPW7w6wRz7/wbQVUwqQRZ2UAW9YPS7uiL6dF/SfIP0DigGYQmoCOlVSKvpJ0ffJouCDOvQCj4TWi/x",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = ""
    ))]
    FolderRoot,
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXQe+KSbt0KbcGbF68evI0qVNhBhoj+vW2FbgRC3st7j+ee8yvBzatzD9IlNrOAQFdHUC7ThjHjhCNZ3ThGJmOQ7II0SQZy3OxFCfrU7xNA3qxjNhKThfKQqwruUFoEFx9rXO6weiU0KIgfr5jr9c1X4bL0LwquFWdNMgL3dQ/ApgX+AAJbMqg=",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[strum(props(
        svg = "eJxNTtEKwjAM/JXQ98Yk3aaFts978dX3EYUJE2SI6N+bMp0jhBy54+7SfXiMcM7uyAxCfTMICJANe/FyapU8I2P09vcyBozhLwHGrsM44UEMykrQl+iFNmJDz8aVtKuZJel11ukC+sqO9w70vdw5u1BFC13Sr+BNzIGtTFt3tfkAZEYtig==",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch,
    #[strum(props(
        svg = "eJxlTjEKgEAM+0pxb732sHhwOru4uosODgoO4vutoseBZAlJSBL38VhgbopeIAzV5JCJKaCAoCyegh+NgjMwMKlSWKkWo5IM9xqduCxs7ORMuCs7KdpY3pNt/Ia3GljB44O/bb9YT+S82Z5pSl5pryyE",
        categories = "files",
        tags = "directory,symlink,symbolic,link",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSymlink,
    #[strum(props(
        svg = "eJxtTjEOgzAM/IrFbjcxISUSZWbp2h21Q4ZWqkTF+3tOEUUC2ZFyPvvuuvf4yfS4VNdE6oYwKik5lGdlvTV/TMC5lrSZeIlJ0lNa++pKOCPOkgZ123PS2Vd9dzLHvlt9vZJ3c8hhz72MC1BrGO8uiQUpRQ1R/ZtOKlGiISo7AdVaNoMHdoocOnPIfGQI1rdF3sRgB0+VosWLxYRBZPQSAFuhNO6sNq5f4sFPkw==",
        categories = "files,arrows",
        tags = "directory,synchronize,synchronise,refresh,reconnect,transfer,backup",
        contributors = ""
    ))]
    FolderSync,
    #[strum(props(
        svg = "eJyVjrEOwjAMRH/F6m7TixNIpFCJjYWVga2CIQMDA8r3k4IEFlWHyovtu7NffozPQrd9d4IS+rIbQaD+XWCct7+Z21zYSfiuwBJZ/J0lMcQdTDSQNq8NE2qwxwmXbsib6f+QLYXDP0VlnWPEaDkSSwgNxPvWG/NHOELXoyhpddcmCySRm6rogg+6aHwBcFlOJQ==",
        categories = "files",
        tags = "directory,tree,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderTree,
    #[strum(props(
        svg = "eJxljrEOwjAMRH/l1N0mvlShkUIlNhZWBrYKhgwgMaB+Pw5DioQsD+fnO115Le+K+2E4j2ColhaCCN+h8DJtWlxX2WuO/WZimpJofohOdMHjZm+fiKfxNwFcLd7cqKYZDfA6zGXXWsyldzHCwpr+yTPDPFR8ETv+AI+RLOY=",
        categories = "files,arrows",
        tags = "directory,upload,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderUp,
    #[strum(props(
        svg = "eJxtjr0OwjAMhF/F6m4TX0JopFCJjYWVga2CIQNIDIjnr1P1b6hOsnT+fNblb/8r9Do3t0BwRWMPArlRYNzb1bP5widJftkpq8TIkt4sLczgssbrJflr2H4g/NU/LSgqiSrAo+nyobbo8tzlk+RI6myYdrCGifOWD4i/Le0=",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderX,
    #[strum(props(
        svg = "eJxNjq0OgDAMhF+lwbfQGxksARIcBovALSAmEAjC87MhGKn6ej+57vRXoL0v5ppQBbUeBKreA2NpM3PkwI048/2UVaxlcQdLiwgYczw5yUz1v4Fwq9liUFQcJQFrMXRlWjE8Mlcdgg==",
        categories = "files",
        tags = "directory",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Folder,
    #[strum(props(
        svg = "eJxVjrEORkAQhF9lo9/9/x3CbXLUGq1CJxRXKBTi+R2JQ7b6Zmcm49dxCzTXWedIq6AYQaD/dWD09jBHDpyL5UlTVilLFltYHCK84verde8Kwm5TDIqK0aljyBr/O1c0Pm0BuV316wtaJOcBC7woZA==",
        categories = "files",
        tags = "multiple,copy,directories",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Folders,
    #[strum(props(
        svg = "eJx1kLEOwjAMRH/lxG5jO6mTSoWlMysDW1WGjgyI78ehUDHAdOdc5HvycJvuC66H3SlD/UHGqY5hlTsY9wUq4RLqzJIiLQbl3JMjc0c+9pwK4k2QuDbpuJvboEoWg3tIZa9n9cnip0ChlCGX3XHYt/bjsDGYwGSFmCVEW1v0ppejWGf04aCVg1YOzewJDs0oDSSj30CwguANYrKBCP6AaGwqS/6RxG3SV/AEmOBGyw==",
        categories = "maps",
        tags = "steps,walking,foot,feet,trail,shoe",
        contributors = "karsa-mistmere"
    ))]
    Footprints,
    #[strum(props(
        svg = "eJxNzrsKwzAMBdBfEdqdRjKKa7Azd8naPbgFF1IooYT27yOTJxquQAd0w6f/ZnhE7IiB+CY9A0Ndxug2CbbhUkwb0mtMwxPGiIyQ/hHJa/40bUHLeWcbWLkyOav97RXIZ/sWQ24il5uu0Rp3l2pDFVW+lDCc7SBwVJkBLI8wJQ==",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[strum(props(
        svg = "eJx1y8EJgDAQRNFWlilAkz3oJZsOLEJMcHOTsKB2r8GLIN6G/5hQ82J0CgZQPQQM2ksyvZcDPUFzWdUEnhFD3w4xbLMpJcHkmTxr53yzVt82/tuHLnWzJlU=",
        categories = "text",
        tags = "2fa,authenticate,login,field,text",
        contributors = "mittalyashu,ericfennis"
    ))]
    FormInput,
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiXnGHCFuoNOoRQoCAqJEG3T4J4m7f4XG/lKbme6C3XcXlFK8iCDYgx36rglh8F1+NIOLzaBbTdmqNAYGYE0ZKIPz5ReAGrIBjF",
        categories = "mail",
        tags = "send,share,email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Forward,
    #[strum(props(
        svg = "eJxtjDEKgDAQBL8S8gHJFmJx5jcWglifv3fXmIRAqoG72bHrvI/g2CNi8ESQD7kSELItcrJ9pm5p6+q/1ILn0fWScbRajWNiqlp+TXUM1Rew9CtR",
        categories = "design,photography",
        tags = "logo,design,tool",
        contributors = "Bowero,karsa-mistmere,ericfennis"
    ))]
    Frame,
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik01IDE2VjloMTRWMkg1bDE0IDE0aC03bS03IDAgNyA3di03bS03IDBoNyI+PC9wYXRoPtteD3g=",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Framer,
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yzN5qJINAvUGHCAsKIsJa6O1zzKLF5y3em9Fu9W6bwRukBsGFRJkYM62uH2/1MV4LTAYH6oC6U1ClhBQtD/I45sjqbd1nCGSwR4gFMiMwqoa45aq0P0uK9XvGT0h99Q1eriwv",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[strum(props(
        svg = "eJxVjcEKwjAQRH9lyT0xO0kkgbRnD3rtvaAQQcSDiP17u6m0KXtY3vJmNj/uzxtN3ClA0XfeTtGEP86bg+rzQaw+V7ce/eL6xU2qNqTNfI3vQtdOXdgTMPgRBLIyGhqnY8uED0eJSmgfZVewqkyibmyFi21ZY0gmuqbdhKTZeJw5Ulif/ACpbjkw",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[strum(props(
        svg = "eJxVy0EKgzAQheGrPGafaWZC0ULiDbrtvkTpuCsSqN6+Jojg4r3NzxeXKRf85rFYIukJy5ZICWuiQLBp/lg5wtrCVsMQb9UN8fsuhjHR8wHpssJDuXfSft9LfPZOIS5wQGB1zVZ1tcJqd+7O+Afm0ycD",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[strum(props(
        svg = "eJxFjMsNgCAQRFvZTAMKfg9ABxZhhLjcDNmgdi/ERG+T92bGHKsweYtF05RVC2eaipz5xEhDVv0vUtiE0mWhQWf0whaqxNuiA3GIO0shM6hU3sO6cA9heB4P",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[strum(props(
        svg = "eJxdjEsKgDAMRK8S5gJiC+Ii6Q08hNhiupMS/NzedCW4G2bmPT5WU8qCJVA8xxmJh14lbmUzegQR1G5BAGmpu5rAX3TVbOrRax+njnUg8ef7CV9U1R4Y",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[strum(props(
        svg = "eJx1i0EKgCAQRa8y/AvEmIsC9QYdIlIadyED2e3LVhG4eYv3eK6kTalUDwOSlHdRD7agy2ME1ZdnjiqPnhDc0IbgjlWFosdiybBwC019wtwL3F3499y9byxn",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[strum(props(
        svg = "eJxFi00KgCAYBa/yeBdI7Xeh3qBDREqfuxChun26qdXADGPPrQiC4zrDiFb0tmvK2y+MmEQPf8hxL5CYDimO2hC3Y09cKRSpYiGeCkXkGkz72uFfXYcd8w==",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[strum(props(
        svg = "eJxVjMEJgDAQBFtZtgExgvi4SwcWISZ4+Uk4ULs3+elrYZYZOTc3JOU6Idi4MMrQUZSad8dVkpuycTzKmbiVE1HbBMJyOczbHbrWhSjf3i/4Ak+oHfw=",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[strum(props(
        svg = "eJxtUEFqxDAM/IrI3aolS7YCSWDppYf2ESEtbGEpPfSw7esrO2ZTyhLwyNHMyKPp8v7xBleahzzAN88D0QDXitHv1O7L9FBZy3Tj2t4buyTtEvvHbD0emoK02ypG6s58x5ms8+zgxT4mHvzP9esMr/PwQgUTgz5lzLYKCMT6hYRjMUio4xYwxoxR2QtCilSxICk/MuYoMKJQBgYSFG1FXhOk3ckxbeRIqAEVONCFXCChnSd2fqWR2xiba8+CSWQ9Go2IavmZCpBtqO7jv+ukv4NC2qK7qvggf1fIrhEvTUNBVqtBih9aA3iKPYij0umWHPpCfuqu6paWX4QDaO0=",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[strum(props(
        svg = "eJxdj00KhDAMha9ScoAZU8ehi7aXmSm2IC6koN7e/OHCVcjj+x5JXNpa3OkToAd38BxoYoIvuBMlzvHNVI7KojLifJQNqoYHKnXTa0CrGpXGyezxWc0hGhRMQjsrcM/Nb+XX3d7+vSbwdE0tba7dvqCIZHlhk4U0FvIF7wU8zw==",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[strum(props(
        svg = "eJxti0EKgDAMBL8S8gFJxVqh7Q98hNhiepMSUH+vURAPXpbdGdbXPAtsJQkHJIfAuSwsT98DtgjHnfUaBqNv9BD9OglDCjgO4LhXruTDHZBh+yOIgCx3rzkBijsk/w==",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    GanttChartSquare,
    #[strum(props(
        svg = "eJxtyTEOABAMBdCrNC7AN1BJ9QYOITF0NLh/xNLJ+p7seYxWD4OpGFJQiY9UPAohW/sEQGCrPhdFVRWp",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    GanttChart,
    #[strum(props(
        svg = "eJxFjFEKgDAMQ69SeoBq1TmEbjfwEKMKCn7I8ENvb6egkARCwpM9HQtMAUd21ENDPnENJguzI1+MUapyjKJr1m0GvQJyg5ADWur5NDu9c5Sf2lJnOEPzAO7j3P3XHl0=",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = ""
    ))]
    GaugeCircle,
    #[strum(props(
        svg = "eJw9ijEKACEQA78S7M9z3YXjYPUHPkKwsBEs/D9qoSSkyIz2PCpKMI08SCCPmKjvfqMeltiygP5MDqtrdj7LHu7aE8xdEk4=",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "ericfennis,Andreto,danielbayley,karsa-mistmere"
    ))]
    Gauge,
    #[strum(props(
        svg = "eJx1jTEOgCAMRa/y416koFAT9ARegujgYuLg/WNxwQHT3zbtS/5PV74P7HN38gD2FM0I7Y2MeBU5w7FsD/utrH+Hd5SbYcmvzGDbLakvnkuqzgGqQKHBBPJDJkRl0iBOc5iEKnsA1mwwrA==",
        categories = "maps,tools",
        tags = "hammer,mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Gavel,
    #[strum(props(
        svg = "eJxtyyEOgDAQRNGrTOo3dLpNQ5OlJ4AL4EgQKxAI7h+KqUL/9+0+Hse5hK1AnenKKMII6ppQ99Bs+kSz4UgoZtQuqchCFZXyA/vvKY7wAq57G4I=",
        categories = "gaming,money,development",
        tags = "diamond,crystal,ruby,jewellery,price,special,present,gift,ring,wedding,proposal,marriage,rubygems",
        contributors = "connium,ericfennis"
    ))]
    Gem,
    #[strum(props(
        svg = "eJx9TL0KgCAQfpXDXbsfJAXzCWptFxocHBqioafPW6QpjoPvP53lqnAsZotAWB2SyWlSMadhkf/xGLgECIB6tqObuIkVYOf1156g2AablQnITvjt2fCM9RczACSN",
        categories = "gaming",
        tags = "pac-man,spooky",
        contributors = "mittalyashu,ericfennis"
    ))]
    Ghost,
    #[strum(props(
        svg = "eJxtjkEKgzAQRa8yzL42MzVkk7jpppseQqo0AVFpA9WevpPEbkRCfgi89/l2noZ1CGMP8xTG+HbICohBkhnqEsTY2POfbOyrf0RYBEX4hC76JCH4Pjx9dKgRVocmKQlsbK5f2KH0wEpCy7vQ9ueNLdVzGz10Du+ywdxMpVuuNKSr5BCok74SAaeNJsc3bxNtJ/t6J6siX47lH/d8Qzs=",
        categories = "gaming,account",
        tags = "present,box,birthday,party",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Gift,
    #[strum(props(
        svg = "eJyVjsEKgCAQRH9l6C45iouC+Qd9hNDBY4fw0NdnCXXxEsvC7rwZmLjno2BbplVgK82U4nxLKb6AHiFbWGiwrVbN+Xx95BxkBIZ/M3SQHBC6R7VrXIauyoAYgr6oD10qxzbq",
        categories = "development",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis"
    ))]
    GitBranchPlus,
    #[strum(props(
        svg = "eJxNjEEKgDAMBL+y5C5aRbFQ+wMfIVWwUESKh/b3JlVEckiWmY0J/tiQ2okGQlJlZU6q582xI2tqcaxxPrqwwSWmIyEKhMtSYeehn/XSVB6KxJWfdS7XjnWiWY3Qi4ZGw6MqvkQTbG+86imj",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[strum(props(
        svg = "eJxtyjEOwCAIheGrEC7QQNOhCXoZ4mBiOjjB7asy6kR4/ydau7YCagmJEdTj9oQ3ZrkiZ2n1K+Ac0WhWsPG+CE5rHXiiA6UnLNOGf4fPIQw=",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommit,
    #[strum(props(
        svg = "eJxNyjsKgDAQBNCrLNuL7gZihMTaxtZeoqBgIUFEb28+omGKgZmn7ersNoO9DJJCsHdqZ1Bgq8t0t/plcY5YJitztI/HApPBngTIRYwMDJUP+eazDjSQHBKQ6tQvCy54aD75AMVPKnM=",
        categories = "development",
        tags = "code,version control",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[strum(props(
        svg = "eJxtjLEKgDAMRH8lZBdNBGmh7ezi6l6qoOAgRYr+vS0FqSgZLsndO+VW77YZvMYWwZ0aiaNeUQUaVWfbqE+sy6nuJ5T/ZeWrarfHApPGgQTIQJaBoYlDVdx6Ud4VjzKhCSlBBuLQPs4NOP41ag==",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis"
    ))]
    GitFork,
    #[strum(props(
        svg = "eJxNyjsKgDAQBNCrDHsBjUIwkN0b2NqHVVCwkGCht9d8ijDNDPO8HlHPDfoymYkQmUaCPnmJ78otvmG2/LbiBl3h3rEyzRaDWVxwcOhz/pZgAvIBYbAfdw==",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[strum(props(
        svg = "eJxtjTsKgDAQRK+ybB9lIy4KSW5gay9RUFARsTC313zAFFbDznvLKLucdp3AOo3UINg75qmxQqPKiI1KWqiDxBh+OJeO4Zph1NhRA0RF3VPtsa8/uEmCSjDwP2oFiwytyz6BkxolIdwUdt0b7XvJNO8d8wBC8zca",
        categories = "development",
        tags = "code,version control,rejected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[strum(props(
        svg = "eJxVjEEKgCAQRa8yzD5ijKJAvUHb9mFBgkRIhN4+R4Nq9Zn/3nxprDduBa+wQTBRIfUpQ04t64K1fDQGXfFS5KePdMznBovCkXroppYRV39AdFX0Imf3FQLlvSgUCkIIopypHdhkR98ejy8Z",
        categories = "development",
        tags = "code,version control,draft",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[strum(props(
        svg = "eJxlzFEKgCAMgOGrjF2gNsEK1Bt0iLCgQCIkQm/f1Jcg9vDD9jHjj+jDBj5ZpBHB59ZoUaEzXTs782G6Kf1H13LvsFqcSYHe1cLA0MuQlJ+h0EKcCce5QeL6JFFNlkwSWTIVWYx7Ac5qKr0=",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitPullRequest,
    #[strum(props(
        svg = "eJw9UMuOAkEI/BXindqBhrYnmTXZ7NmPMOPBowfj91tMJ6bDq4CCZnveXg+5/56uluL+1rgFhpQs9dS0IfdGv6tTEollqMFT4Wd1xJhF8ILtsI6WbCY6ScoXQzLRQ2kTrZflmP8uLjllV1SdZdlJIiT5S8TSZOpjLwlZ9yMnhaf00uxaEauiD7IsjEfS6XTsDA50Z+iNYeEj33G6bD91g8v2vcQqNnYNpIkrV1b+8lv2AUS9PiU=",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Github,
    #[strum(props(
        svg = "eJyNkDEOwjAMRa9idc8ntiMnkUpnhpyALRIDQ5EYuL9wBGWoGKq8LD92nuz52V93up2nhwixQmpQqAaOHUkciuMEcPJboIMtE/GModX5ZYqY963euQaBGBksX/wPaQYm99ne80djAbEc07TvFB05OZ83gmQUbSwkvFZYDYZSvIadrUZdXfQ6LfNpLGV5AxTnODA=",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Gitlab,
    #[strum(props(
        svg = "eJw9jLEKgDAQQ3/l6N7aO722Qi24OegPuBUcOjg4+P94hVoCIQ+SxCe/Ba5FHciGgGgLJmQJYEWoSaPx884wFpxugYCA3hCuvQNteaoUh3qXYj91gJQZuDWd+E+2Up98HKUeEA==",
        categories = "food-beverage",
        tags = "beverage,drink,glass,water",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GlassWater,
    #[strum(props(
        svg = "eJxtjUEKAjEMRa/yyb7VpI2j0M4NPMRQBQUXMrjQ25tQGbqQEJLw3yOl3df2uGKtlAntXelg41OJleay6+lcfpTnfNyAbg3Yc3ndcKl05gzWRSDYewUJvm8XxC2nB0eighMUU4tT4JjAMRubwl+aO84n40MXrNVejcIXv7o3+A==",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[strum(props(
        svg = "eJxtTk0LwjAM/Suh99YmbalC1/MuXncvVZjgQYYM/fcmHWwTRg4vj/eRpFd5j3Dr1JXQBA8YeoyFgMDKaN5mz4LK6STWnNZABGecH0Jx4BY3o5vtmkaQtK28GTQXYcK3diDddN10uaVpdAbjwTXkNjaFAc/79/S+EBvX+G/oydiwNdbHVJ93mDqFVkH9MhLjpyG7Fj3/AFqgQXc=",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Globe2,
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8yZK82UXdtb+AhpAoKIiIuWk9vPzuVQIa8PBjt1tNtM1wwxEI4YyiC8/m0uil/q7d1n+HZUJQCFzlISR9Tsp0sq4/xWjAZGlggI/d1i7xUHEYHVnjB6h9WX4gE79SVWuwD1zkr+Q==",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Globe,
    #[strum(props(
        svg = "eJxtyjEKgDAMheGrPLob09RAA7XgAVzdCw4ODg7eH6NDJ3m86f/K1e4D+xzWKIhpkzNjGvyhlvFttXQhTKqITJIWg4ERfW7J/nh22hT6OUYmgxBLlw9pCB3e",
        categories = "gaming",
        tags = "flag,bullseye",
        contributors = ""
    ))]
    Goal,
    #[strum(props(
        svg = "eJyFj70KgDAMhF/lcG9NYpQWqm/g6l5wcHSQPr+tgz9QkFuS474cCXs8NqxjM7MDs+0XHwUCKjJiJNF7hyS22kyhLdwUHlrBtLgfVmokwVv/W9tX0AGs32AFpFpn+fVOcpaCUhcd3EWyydNm9G1k6xO/z57yqkpv",
        categories = "cursors,design,layout",
        tags = "hand",
        contributors = "ericfennis"
    ))]
    Grab,
    #[strum(props(
        svg = "eJw9yrENgDAMRNFVTukjbEeOhGSyQYZAUFBQUCAXTE/cpLj7zbNnfy+cW+oiYPLaIzdT1lFojvtSsyVgs8krWFyPgoJ1jAXkWSf8Ad17FjY=",
        categories = "buildings,maps",
        tags = "school,university,learn,study,mortarboard,education,ceremony,academic,hat,diploma,bachlor's,master's,doctorate",
        contributors = "Tummerhore,ericfennis"
    ))]
    GraduationCap,
    #[strum(props(
        svg = "eJx1j8EKwjAMhl+l5G5s2qW20O4NvHqXKijsIMODe3tTt0pBewmBfPn+JD7Oz5u6JDgao/hkph2jD6oUGOO+TMeY73OermpOYEHlVwJy6KRbpKvkynzZQnkkgrol7AGHP2RjNWh50xI67mjJYqAVk3DeEjqs3FCVGnXow8tHbKC5xyG7nlejb18j+W3osDX/N/wNiAhofw==",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[strum(props(
        svg = "eJxNy8ENgCAQBMBWLtuAAT4+ODqwCCPE42fIRbF7ufjht9nZja0cSi8jgFpneNBTswrDrSAp9RT9c7dNiosdUrx2FcqMLZDzMnyIdZM4T+Ge5ANk7h49",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[strum(props(
        svg = "eJxty80JwCAMBeBVQhYoVoQK6gYdolRpvBUJ/dm+sZ4ELyHJ954raWeglA9ij2pBuHNkauvjUSO8/yxyzBjcVAvBnRsTRI+rBkuSFaivDpQZiwV9DUGZXj7VjCyp",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[strum(props(
        svg = "eJyNzTEKgDAMBdCrlH8BiZChkPYywUFw6qS3N7QBAyp0CuG//IjuTY8ttQJC0qsg2zhtW1FlGWkVVz0wEPWHelrY7Rt5xfjUT4h/uogReZ5isewGgttBeg==",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[strum(props(
        svg = "eJx9jj0KwCAMRq8iuUBJwaEQvUzoUOjkpLc3RAXFn+kN74UvxF/g/zXBAYLhJLiF0cEDnq5iPY2VWo3tIuo0CvVmUXVTaI9Zs63d/lUXpywDiR5Beg==",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[strum(props(
        svg = "eJyNjTEKgDAMRa9S/gXEgkMg7WWCg+DUqd7e2hBUiNQpkLy8x7IV2dcgNWGOCKUNBDkSFmSe9JrZqL436HqhP5SnejiiBtt0XBoxQN/8ZPfUO/1BEd7tQZOGNr95ArHOYnE=",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[strum(props(
        svg = "eJx1zjELgzAQhuG/8nH72dyJBEGdu3TtXqL03IqEtP33TaSWDrpcSHjeI93jFg1jT5ca/toEx1JJ1bJCWU1p6E5FDN3PiUdtGjKDg6Jq89C0J1UgPmlwyJgLLGuN96yHyrkJvK5l5fIF1vRvlylE2DTfLfbUEJ7zGK0nT3itc8mHEN7lkqPCv1F+EqXDVtwWb9kHJy5JbQ==",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[strum(props(
        svg = "eJxlj8EKwjAMhl8leG9s0jZrYQ68eZgPMfSwwwYeZM/vv4giSNM/TWh+vvSP6TnT/XRYpZBoqFwI9xa4JkRQlm7PieLvmdBXctlroRjSiKIdhv64Ww79x/gKA8sEe1WSiPf/n1UjNyER7oKwFhcwmEdLqLNnRX8Dm41iHIUy21S4GLk4WkjcMgYsX9rCTbnq2VgqubxpQVo5b4KhBWvMyrlb4G0k4PjyvQD1SD5Z",
        categories = "tools",
        tags = "mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Hammer,
    #[strum(props(
        svg = "eJxtj70OwjAMhF/l1D0mdpvGkUIXZtbskRg6gMSA8vy4gPgpleWTLH+ns/O13mac9t2RFSwUCvsqEPilnDhpPzOkMQ3dlHeLccof+wDmkl4sGzvAN9kibe0tKKzY9M9eIjg4pjg+5PsQ0h4mcu5phPUhUoAw2R2JjBP7xnSWqtCnCeq0xFVqeKfeAdW9PSQ=",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[strum(props(
        svg = "eJyFjbEKwzAMRH9FZJdqybKrgJsvaNfsph0ydOhQ8v2VS0gyBMIh0B33uPKp3wlet+7BBsxjrgICoQkFZQ57D+67oVwaM5SNVOAw6gkpR2RwktLpqh2xBrbU2KUQ5lw9+2OM/k0oTxTyBJUSkmVM1PceRX1jpNxuXWLwamx9uV+B0zr5A510Qiw=",
        categories = "cursors,accessibility",
        tags = "wave,move,mouse,grab",
        contributors = "ericfennis"
    ))]
    Hand,
    #[strum(props(
        svg = "eJxljUsKgDAQQ68SZq92ipQuWm/QQ4iK40IQKX5u7w8pKAMTyCOJm+ooaD0F1tCLpcoVl1W5F4xsYLISZXZewnPXREg39BI9WcI6tFE8aUWYt1MJz9898R27Aqk1GLCVXPF/MLD6sgMkZi4G",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[strum(props(
        svg = "eJxtzUsKgDAMBNCrDNmrTZHSResNPISoGBeCSPFze60iikh2ecyMG6sgaDwNbGCSPB5yKlwWoXA3l6yhZ/vA1NYB0+pJE6TtOwmeLGHpmyDHUxEu2zzx2RcDrz4DtpIq/ptSX9sBCgIuBg==",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[strum(props(
        svg = "eJxdj0ELwjAMhf9K2L2xL26Zg67gzYs/YqAwYYgHGdNfb1qHxdHDy2vymq9hut2vtEhfSUUvE5guMJs8so9hl6ZieAzPkS59dW64bqhhgIQgsw5ihc/HqhF/F05mp5PbW8YpH7pj6UG5VapPLUtdIg7cdgR7/p12p60xfDkNSFdO04St7LGSaiH9/Qo+91MQviQ38x/fmDyZ",
        categories = "development,devices",
        tags = "computer,server,memory,data,ssd,disk,hard disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    HardDrive,
    #[strum(props(
        svg = "eJxtjaEOgDAMRH/lgm9YN9YgBhqDxS9BTCLIBF9PIWSMhNT0ru+uYYt7wjo0swX3kcEw9+iWvgZxJvsapMbkag3O9mjG0F6dYyjNrGGz+ILy1ZVsrTXqf5Id2GdyUSAPKSTJ/P3ooIcahGRXwBPOnDYz",
        categories = "tools",
        tags = "helmet,construction,safety,savety",
        contributors = "Andreto,ericfennis"
    ))]
    HardHat,
    #[strum(props(
        svg = "eJxlzEEKgDAMBMCvhHxAE6sopP2NB0E819/bmFBKewpLZlfu6zkhU8SA8JZzIGSOyHOJrDHJpCbJL5XQirXBHq0y2sUo+RyT2b2n/msrm1EK1X7YHyt4",
        categories = "text,social",
        tags = "hashtag,number,pound",
        contributors = "colebemis,ericfennis"
    ))]
    Hash,
    #[strum(props(
        svg = "eJx1zKsOgDAMheFXaeY3tu6GGNMYLH4JYmYJgvcPrQDVpan6Tv5yt6fDtakRDUKidybwq1oWtlq+xUHmO0pgJzJcplo2iYt6UqVs3lEWdKI4CvoWIIDl0ytYaYUQTzTxpxd0gj8V",
        categories = "weather",
        tags = "mist,fog",
        contributors = "ericfennis"
    ))]
    Haze,
    #[strum(props(
        svg = "eJxNjTEKgDAUQ6/ycf9qoiKF2tnFC7gVHP7QwUE8v+1SS4bweJD4Oz4m19YdpLgIgYwlCsU+tSx4559zw5AoNDBRc7VScXbBD2U8+Hqx9ouA5qr6AP+hHYs=",
        categories = "devices,multimedia,gaming",
        tags = "socket,plug,slot,controller,connector,interface,console,signal,audio,video,visual,av,data,input,output",
        contributors = "danielbayley"
    ))]
    HdmiPort,
    #[strum(props(
        svg = "eJx1zDEOABEQQNGrTPSbzcxuUAw30OolCo1EIc6PioL2v+RzCTVBNML9gJS0sPzOZHkH7eUBkC6SUY0ZfA+1NexHth3A",
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading1,
    #[strum(props(
        svg = "eJx1y6EOgDAMBNBfaeYP1lKWibI/wOIJiEoE/x+KIQgQd+Jezo71dNqnNCuxeE3N+ntq9oa6lA9g+RPhEIduGUqKIVIog7sRAolW8HO7AFDBIlo=",
        categories = "text,development",
        tags = "h2,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading2,
    #[strum(props(
        svg = "eJx1jDESgCAMBL9yY59IIqgF8gNbewYLSgv/P4KNFNhkctns+SveGec27BaieR2CH+sp+Basx9wBor9kYQcx7JLwQoKpRPNOYRcVWpKBUNm6NmpB0voNC55gqRUNKX3iA4+XMDo=",
        categories = "text,development",
        tags = "h3,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading3,
    #[strum(props(
        svg = "eJx1zaENACEMQNFVGha4a9MAorABFk+CqEQQ5icoEGD/E19a6Qo1mMSApN5E+VaKcoLP9gJIT3GA/2DlixEu26MJc4wk5Q==",
        categories = "text,development",
        tags = "h4,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading4,
    #[strum(props(
        svg = "eJx1ja0OgDAQg1/lgqdwd4MbyUBjUCR4AmISQXh+fgRBMNMm/Zo2bPMeaW2zwRFL9FkXijvqwhf4qf4BLElixHrkGl0CGmyBg8BDiaGPVlSSwHIGPy6oRm7gr63r6K5oz/YunpvIL4w=",
        categories = "text,development",
        tags = "h5,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading5,
    #[strum(props(
        svg = "eJxtjDEKgDAQBL9yXB9NLioRkvzA1l5OIYKFBAv9vVELg9gs7M4ydh22AKPDrgJFwaC35TV5mwPTNz9A0YfwHHmZgHeHqkWIDgmBj9Tu04MzAUlQkgUBCQ26qFO+thMkACou",
        categories = "text,development",
        tags = "h6,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading6,
    #[strum(props(
        svg = "eJxtySEKACAMAMCvjH1AN0QMcz+w2gXDosH/IzaF1TtZYxvMii0DsRGjSrim8gzHnpyg8s8BT+sVqQ==",
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading,
    #[strum(props(
        svg = "eJxdjrEKgDAMRH8ldA82qVILtbOLHxFwyOgg+X4jOLRyw90Nj7t6ya1wbuFIQLMmYWCILnJn6zp62pe+IxtmKVA+gFaIlkdCkf7IsIGsKbQ6vTfaA11OHfs=",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Headphones,
    #[strum(props(
        svg = "eJxdTksKAlEMu0qYfeu0fR8HngMewAu4k+fChYIL7495A7oYSpoS0jbtffs8cD9NF1tgqZumRdgKQkLdSFnzmcDAvJUVjtHpqwWzBDRLouRU8gbXmsSHKL5bdhw1d7IGaEXSmceG4VlRr9PaDiPT2n7JXuawEBPjAxfmGvz3fQE/mSkh",
        categories = "emoji",
        tags = "heartbreak,sadness,emotion",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartCrack,
    #[strum(props(
        svg = "eJxtT0uqAkEMvErhvvOS9B/mCR7AC7gbxoULBRfi+a0eBPFDOpWmKt/pOt9OOP5v9tZhaTFJPRAKYojixpAl7+gYrqtZ4TcuzKsFGiIkh0TKyeTVXWoKPsjgH8WOJnlhlAimIomy2Ug4V9TDZjv9jZ2202szR0YXTajSy+xiFSvo06Jou+sizfmGFKWNnqL17ASu1OenRlhPYFHtUOq9kC7le/DFGowHBP+lZVh70x7IP0hr",
        categories = "emoji,account,security",
        tags = "agreement,charity,help,deal,terms,emotion,together,handshake",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartHandshake,
    #[strum(props(
        svg = "eJxdTUEKwkAM/ErYe2Izu+lWaAs+wEeUKigU8eCh/t7EooiEyZBkMtMv19uZVgwJSPT8sDonWt809rsQjf19elzoNKSjtmK0NRB04cp1ZhVzFOPMWeDdxCYHBRovJYgWLpItTMPux7ST2lIWnVXUBAhxltr5DoVU9rQFmB9qYVDxEYe/AFAnNjfxDBdnCguGtNVVWr+5L2gHNjM=",
        categories = "social,multimedia",
        tags = "unlike,dislike,hate,emotion",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    HeartOff,
    #[strum(props(
        svg = "eJxdTjEOAjEM+0p0e0ObNA2VyklsLHyADZXhBgYG/i/cIt1wipxEluO4fZ7fjV6X5Z4qpdwT5xrQCmlQloRhbFeABuKsVLBqh84LxaDEFjIoAWMTwp6DDDLI4VjozNYxWQlSyhxhNgRvJ38sazuNTGvbkyGHUJJbhWT4w+L/zaeBsm3G4vvlDzz7LPI=",
        categories = "medical",
        tags = "heartbeat,pulse,health,medical,blood pressure,cardiac,systole,diastole",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartPulse,
    #[strum(props(
        svg = "eJxdjjsOAjEMRK8y2t4m8SdhpWUlDsAF6FAoKCgouL+YbIms8VijZ8vb5/F94XlZbnVFjVE1VmFrcHG1SkvNK4WpclRtHH2Q6w1FHJoSjIxJHjLtITZDsb9lw1lz0NVBFKGFxybw7uj3Zd9O86f9B4uwH20=",
        categories = "medical,social,multimedia,emoji,gaming,shapes",
        tags = "like,love,emotion,suit,playing,cards",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heart,
    #[strum(props(
        svg = "eJxNS0sKgCAUvMrgPntPiRLMG3QIeQUGLUJa1O3TgohhGObnZc2yLZBrVGwUchFSkPOxwbdvH/wej4R5VJPT5OCihQUVMDo9WLAQTFPCyvqr+9+LDbhPmvjrbqWXH0Y=",
        categories = "accessibility,text,shapes,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    HelpCircle,
    #[strum(props(
        svg = "eJxNjrEKw0AMQ39FZD819jmXM6SBbhn6EwcdMjTQof9PnYaGItBgoWdNr/Ze8bh2W4YMGCiadrtlZPQhgfRUgy+Sm0KPU7itSTlsllgqjDImY/UmrBVfO9pKjxZzeUau8cDtn230AhkXZ/3hQ0loAff7CPFuni77ynk6t2oUUVDO6AMQlyqM",
        categories = "emoji",
        tags = "agreement,help,proposal,charity,begging,terms",
        contributors = "karsa-mistmere,jguddas"
    ))]
    HelpingHand,
    #[strum(props(
        svg = "eJxNjjEKgDAMRa/y6R41bbEdasEDuLoXHBwcHMTB05s4SMgQeOT9/HK2a8c2ucUzeFxz8/AYdIiJuxQOShQNlS0Icf4RAvJtREiSinJkoHqSZLTv4+Nq6bVEfQEGuB0S",
        categories = "shapes,brands,development",
        tags = "shape,node.js,logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Hexagon,
    #[strum(props(
        svg = "eJxFjDEOgCAQBL+yoT/kDiSSIC/wEyQWFJhYGN8v12A2U81m8l2fhnM3VwIzRcTXt9Q9eVPyorLkeREBCwUbMagCgRtjErvBdVqtKL+AU3VwQJi1D07EGss=",
        categories = "text,design",
        tags = "mark,text",
        contributors = "lscheibel,Andreto,ericfennis"
    ))]
    Highlighter,
    #[strum(props(
        svg = "eJxtjD0KgDAUg68Surf2x2ctPHsCPUTBoYODg/T8viKIg4SEQPjCZ7kq9kVtAc6XhAQLJ05auon0hO3Sk4kjvMQaMKvMQ4czfy5Co0o/i/OIjQ6h3/EG4sUegA==",
        categories = "arrows,time",
        tags = "time,redo,undo,rewind,timeline,version,time machine,backup,rotate,ccw",
        contributors = "ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    History,
    #[strum(props(
        svg = "eJxNjFEKgCAQRK+y+C+1GyKC+t01hIIEUyEJ6vRt4UcMDDPwZmwNbYPFiX0CA0Zqdn0iBgKCkYWS06z+XdItvB3epbe1pCvFvEItMbfDCQNEfIIEqLoTfXwn/QOGnh24",
        categories = "account",
        tags = "house,living",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Home,
    #[strum(props(
        svg = "eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUXM7efpxhmNoZYcuJPTy/y+vf+/0Ufl+WqVTJlybt2qmQBb51MSfVglWhRrGQ2KalI7JxEtYlZWbb1zUW29SkFFXK5vRLE+hBThdohLoFzzqzcJE+qTyMeDkcpcMIqrBPUAmQBwiVIo3jGNJc9kXB40heJCDd6NoqM7bzajMxgyClA/gOjbMImqS2SR9tVQo3UJHVDV/WGaN8OZYzDe7lzLiODSFjZB3T14eIeJNfdrdcGIkYwWVKOOAitQ4kwJmkND4xh3lKLwlu1gn3R9vL38/37SXe7LGYL3RR5oftIt/EVqEPbA6and3k=",
        categories = "food-beverage",
        tags = "beer,brewery,drink,hop free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    HopOff,
    #[strum(props(
        svg = "eJxtkb1uBCEMhF/Fut4EA+ZH2lyzdV4gXUSKlCny/srY7N01FJi11t/MAMfv198Pfb/fPqQFJQ16yqBGKaIblIREJqeAH6yE0rHkdj/eDLwfTxwDZBJnIwiMJSAChQniEmAxfkNL9emgEwQJZWupGkeRC1bduqb4oFBBZq/FMrpjNhr73tSZOMXNnijq4sFmkCawtTasPiKj8dDrEjkZ7MasS20jUULrmayms4cyMvDYIFY8kXTSydjZU3G99OWqhS6zdVZr3Ww9gqWpeBLCaJ9cPE4OIzUeQWPnkKQxxpL0z2049dvIlCcH6cWKXl9dKg74epF/cDF/lg==",
        categories = "food-beverage",
        tags = "beer,brewery,drink",
        contributors = "karsa-mistmere"
    ))]
    Hop,
    #[strum(props(
        svg = "eJx9kDELwkAMhf9K6J54yeXOE87ODnbt4FZ0uKXgIP39pqByYK9kSMh774MkP6dXgce5GziBXOIkIODWQpsWrhbWpbDUC5RR6wTKrevzYWX2+UueT8ARyGtCEr0zaQzI5NiDp6SfMZB3Cu7Kwdz/kCHBsZhxQzF4U5I9iblNbGppJ2YfkQUjhdkuGUV+njcAW1oJ",
        categories = "buildings,maps,travel",
        tags = "building,hostel,motel,inn",
        contributors = "karsa-mistmere"
    ))]
    Hotel,
    #[strum(props(
        svg = "eJxtzrsKgDAMBdBfubi3mku1Hargrqu74ODg4CB+v6mDipRAIK9D4j4fK5a2GGuQq7iii2XqdfE7yQ/E681pnBXPmSCqFMbWoTFinbhBCOGmG1rgzv2zBw/xNjBMZAZX+/zJSDK+8k2aP6x/Nelyet0L4p82Fg==",
        categories = "time,gaming",
        tags = "timer,time,sandglass",
        contributors = "karsa-mistmere"
    ))]
    Hourglass,
    #[strum(props(
        svg = "eJyVjTEOhDAMBL+yoo/PxrGTk3LU1/ABOgQFDRIF4v2EDkq0Wk0z2i3buC+Yf00vLSRNBkYOLfm3wv9xYiipQOHI8GENEXHJFUEP7Y0kQmRUMlxlSI1TEnDTlc+13pXbx0vfquNkD59Dusknob8sJw==",
        categories = "food-beverage",
        tags = "gelato,food,dessert,dish,restaurant,course,meal",
        contributors = "kemie,jguddas,karsa-mistmere,ericfennis"
    ))]
    IceCream2,
    #[strum(props(
        svg = "eJx1jDsKgDAQRK8ypDfuxg1JEQN2FnqIgIWNYOH9MfEHFmHZYob3JuzpWLH0anNghmjyYNKdTQwGXcfaC2jiQqgY2qLE8Ipz7t1gYR86pwqUDMw9mF/GX27kk07c3SHz",
        categories = "food-beverage",
        tags = "gelato,food",
        contributors = "karsa-mistmere"
    ))]
    IceCream,
    #[strum(props(
        svg = "eJxNjkELwjAMhf9KyL2zibR2kPbsQa/eRx1sMEWGyPrvTR2OEcjLlzzCk1f3HuAe8coE7Ydsx8BgtcjodHZ7NnzbMSgPJ0xyqD+STOOzh0IRHULhnyxK5FUVmau1mpLkcc5TD7NuEfISsVUpVdSyHpP8kz00GTlzbGzwa98yWMNN4AD24oFpi/IFaTwzwA==",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[strum(props(
        svg = "eJxlT8EKwjAM/ZWwe2OTNbJBN/Dmxav3ocIEEQ8i29+btGU9jMBL0r68vMTX8/2AhYaGG1hYk+Y1t2tux3gw0hg/03eG+9BcyGMgSDgxMHjQzjF2bQKbMO4YN3FqUYogZf38YhuPdUPip19zY7nLU1S4Or23o0o9GJwI+x4S+BQtyI9C8Wih1UzhhiJmGr2wQ2atAjkVqM43cdbT5CpVw7Hjc2X+AWufS9E=",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[strum(props(
        svg = "eJxNj0ELwjAMhf9K6L2zibSr0PbsQa/eRx1sMEWGyPrvTVvdRqEvL+8jJO7VvQe4e3ElBKRP2xEQKH4ouTrrvZd023lgP7QiuEOeEdw0PntI5IUWkLDIwoKGlbtEGc3QDy3hqbJUmWJZ7YbGcY5TD3HxgsO5oDFlw0gNg/sf8chHaHlslDX1X9dVkhpLFtTFAOG69RcNAD6d",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[strum(props(
        svg = "eJwli2EKwyAMha/y8H87TVlxoJ5glyipTGGDIcLa2y+pBPJI3veFlrnjV/deonHe4IhmMWgSJHFeUXJ9lT76U/sUbuqlwLXxO4Pl+xD8ovnQQ5BRpvDdesEezYcc3H1aZuvXsTcCwepMNHvysM8V5FRWKf0BiP0pgQ==",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Image,
    #[strum(props(
        svg = "eJxljLEKgDAQQ3/l6H7Yi1U71M4uXd0LDl0EB+n32w7igWRIeCEJV74LHatJAhqrwMQwdBbD25yeRMh1sfvXydO0uQwC2S5uqYr9QHMUmTVg7IteMIq6fgDgbyNR",
        categories = "arrows,files",
        tags = "save",
        contributors = "mittalyashu,ericfennis"
    ))]
    Import,
    #[strum(props(
        svg = "eJxVTdsKgzAM/ZXQ92ama1OFWtjbXvYRwgYKRQsTYft604KIJBzCueSEvKRfmuYP5GWa12+vjAGS5YoWyAE1BdtCFE3FcDtSMeRhHeHdq5dD68AhUTVtPBR3U0eukS6ENpvmpO+S0Yxt9zg1YvQM9unR2DOiCX0HJO//tV9a4w70CCtk",
        categories = "account,mail",
        tags = "email",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Inbox,
    #[strum(props(
        svg = "eJxdzEEKwCAMBMCvLPlASQTrQf1ND4Ko0B7q74u2CvWY7OzakmONIR0oOaTrdKRgsIMFCqzJ220Ib7ur4oiFcLMjYULl725/boUF69foSUb1T0fY982ybyZ+APFxLqk=",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Indent,
    #[strum(props(
        svg = "eJxtyzEKgDAMBdCrfLpbG4OxQuwNPITokEVw8P5YHaRDlgT++1+v7TYcS1gFbDSEov0bFW0gu3AKiJHjiOyuiI0dmCvsEkUmJHy/o4RUz19+ACWGKAU=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    IndianRupee,
    #[strum(props(
        svg = "eJw1jbEKgDAMRH/lcI+2aagdav/AH+hW6uAiOPj/eBXkSAJ3eUm+23Pi2KbdK7x2UdE5rmJUFGsGg4NnOaSu7CZ+DgEM60WX3gAwFiN+4JMknuMwDEKY1qnkZXwsLxFqGW4=",
        categories = "multimedia",
        tags = "unlimited,forever,loop,maths",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Infinity,
    #[strum(props(
        svg = "eJxVyUEKgCAQheGrPGZfORLRYvQGHSKmwKBFSETdvkwQXD3e/4luUfcVejtiS4jfGII+//XSZfdyzGfA4mhiCx6upk+YYk1jaA0XegHnnBpM",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[strum(props(
        svg = "eJw1jmEKAiEQha/ymAPYjru5/VChA3SIaCWFiBAh7fQ5azEwPL55b3g2h1tBbo6OhBjSPRZHeiK801bikP2oCXXfuYrT24PkvH1dS8Tm6MIGzGpezwsWTGAZrcyMEwbZ2c/0kQcS9faRngGNHRnVC9QueBXV9B/pgVgy4vZfGSIsAA==",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[strum(props(
        svg = "eJxVjEsKwCAMRK8ScoFq0IWQepsuCqVrvb0Tf+BqSOa90e/9H6r+5sBUED4hBemYqtg762VQ1o5aF7kb4qYSBor7ZNcsurSRocSNNmG3IKQ=",
        categories = "text",
        tags = "oblique,text,format",
        contributors = "colebemis,ericfennis"
    ))]
    Italic,
    #[strum(props(
        svg = "eJwtjFEKgCAYg68yfLfURP4H9QYdIipQEBX0pduXEnvYBt9m69EDLsd2JSDFKbheNN8Ww2moccIsmI7hFIh5u46ht7WkJ8V8o5aYe3NMGkiNcUb4slKT/Sn/AgR/HOI=",
        categories = "arrows,design",
        tags = "arrow,right",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCcw,
    #[strum(props(
        svg = "eJwtjEEKgCAURK8yuLfUJP5CXbfpEFFBgqigm26fVqt5DG/G5K1eOCxbNaTYBdeDxjTMnECcCnVG5zd7u2jmzNhnzuQU7uDjiZx8rMUyglJoT82D/MRfcQ99UxvK",
        categories = "arrows,design",
        tags = "arrow,left",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCw,
    #[strum(props(
        svg = "eJxtizEKgDAQBL+ypDe6F+5QOPMCbe0FizQBC/+P2ohFuoGZ8XO/Co45rBRMUTdhHToy6mJI1WAPcUQK2fu3zf4dBmqhtA3/5gZNNhsY",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis"
    ))]
    JapaneseYen,
    #[strum(props(
        svg = "eJxtzLEKg0AQBNBfGbY/kp1EQ+Du6jRpLezkFBQsRET0771DUAvZYpl9y9ihmlrUTv5U6KciiGcaQ8Nfds3gzDPHzVbf14PhbFiKt49U6u1RnUOzSDeijFR8TwndGPoGYXGiFIxOXoKwOsnTz65+AyAaLXo=",
        categories = "gaming,devices",
        tags = "game,console,control stick",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Joystick,
    #[strum(props(
        svg = "eJx1jzEKgDAMRa8S3EWTVmug9gYeouDQ0UF6ftNFKyRkCf+Fx0+88l3g3Idjg1DDkOLUkhTfHEmA18AqgBWwgMsEBHObUTblhsEV1KTeAtxZEXQrIXBFHaA3CX9io67UIlR7sQUWAb131LzOquV+jzyUMHME",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,draft,template,boilerplate,code,coding",
        contributors = "danielbayley"
    ))]
    KanbanSquareDashed,
    #[strum(props(
        svg = "eJxty80JgDAMhuFVQhaQVrEV2m7gEGKL6U1K8Gd7GwXx4CWE9+FzJc0Mp8cW4bgvpbwQe1QWYc+R6XlLVY3BNTIIbp2YIHocLZjNSJfy6UpX6P6grzC8cAFYQiT9",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    KanbanSquare,
    #[strum(props(
        svg = "eJxty7ENACAIAMFVCAsYjBILZAOHMLGwtDDOrzRUtn95WX1PGBUbQz5EqBIsqThQfMI/KLYklwtKwhXO",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    Kanban,
    #[strum(props(
        svg = "eJwljEsKgDAMRK8Ssm9sbawu2t7AC7iTKFRwISKit7dBhvksHhOP+SqwJBxbcMPtxQIFYnCqwrfxxWu0uyM21XOgDtS2Etaw4QlzbPQmR9lO2VeQN2FPHYI8CV3QdSasVcEfyR8OVR52",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = ""
    ))]
    KeyRound,
    #[strum(props(
        svg = "eJxljT0OwjAMha9idbdJ4rRJpNAT0AuwoTBkCBID6vnxq4AF+Vf+5Pfq8/bqdD9Pmw8SKUhqUliKbTOG2tGNWWayMoQAIwc0WCXZkhoDsAHrx88lGymSQT55vIFep7WeYLzWr/3DRzIl0n+0FdPzThYK5POuzZEsOCF63Fm7ooXhJbLVT+INd94zSQ==",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = ""
    ))]
    KeySquare,
    #[strum(props(
        svg = "eJxNjEEKgCAURK8y/L2Fikng9wRdIn5BgUFIi7p9WhAthhmYNxNkzZJmyMWkXeMImelxOZl8CTG0LxPDPh4LJqbNaBjVNx2KKlCLX12PULawsIMx8ElZZT/wBiRbHoU=",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[strum(props(
        svg = "eJx9y8EJgDAMheFVQgbQpEjtoXUDhxAV05uUgrq9rYJ4aU8J/8ezYZ0jhNOhQgjXcw6/REkfIbw95Q5BVr9JdMgaB9vm3WD3KQosDkcNRhoizpTjj5gq1lXMlM0Aq+JO1VBXsAfWwvTJDQhvTpg=",
        categories = "text,devices,development",
        tags = "layout,spell,settings,mouse",
        contributors = "it-is-not,ericfennis"
    ))]
    Keyboard,
    #[strum(props(
        svg = "eJxtijEOgCAQBL+yoRc5kCOXnNQ2fsCOxILCwsL4frGhotgtZkbv8lScq9nJw7/RZJ1/lLULRqrkrwWytZ/kGDRiKYG4BAQ4UFu0zHA9/QDhERtm",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampCeiling,
    #[strum(props(
        svg = "eJxtjTsOwkAQQ69ipfeS8Wx+0pKahgvQRVCkAIkC5fxMlCZSotHYzbNdvtNvxutafSyjocMhdOjZs6Me1VguKzGWA0ffNBJ+wt2H1KCNzzC9He0JEltaqGdNS5YGCqLmPIWjjrNwLbrt+v8SQytU",
        categories = "furniture",
        tags = "lighting,household,office,desk,home,furniture",
        contributors = "karsa-mistmere,jguddas"
    ))]
    LampDesk,
    #[strum(props(
        svg = "eJxty6EOABAUheFXudPNsDHbJSteQLMJgiCY53eVm5RTvvPjantAj6IEMMNNCz7TSl9FQvUwIV+0gXC0/QjFVDNcHpwYWQ==",
        categories = "furniture",
        tags = "lighting,household,floor,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampFloor,
    #[strum(props(
        svg = "eJxtzaEOgCAYBOBXudGZ8oOAG5IpVoKNzUAgGJzPL6R/c+7ifbcLV7krzk3sSkHpapuGS75p6Q4RwzTqGBiZjrIvBMI8IklS8j/SYK3EDt3l5bMzj+WTF6xiIpw=",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallDown,
    #[strum(props(
        svg = "eJxtjb0KgDAMhF8ldC+a9Heonbv4Am4Fhw4dHCTPb5wCIjfcwX3HlavfA87N7IjgR5wOUsvT2XSYWpa3rUUZD4gcOgHBKkIrqeUfUsAwSEFxYvoMPduoNw/Z/SLa",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallUp,
    #[strum(props(
        svg = "eJxtjLEOgCAQQ3/lwn7INcRggswM+gNuRAcGBwfD93suupAOTfPaxqvclY7ZrIFQw+lJXPaLhs2kOLw0xa8jIEEbO0QHaIzdsVixE4PAqL6ok1OJOhpy+G8f+w0f5w==",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Lamp,
    #[strum(props(
        svg = "eJxdjtEKgCAMRX9l7AdqS6oH828igtCgHvLv2zIFxYdx3TmX2WP3K0RakBkhcpqP5EGGRkJnO6Wc/VjdjfgpNCeFKLFjg2ZGFeprVnLTq58Ga8kUqWnOTRnm/5qUC3yGI27Bwxl2f1+yY5DXwwQGJsV+wL1ujkQa",
        categories = "money,maps,buildings",
        tags = "bank,building,capitol,finance,money",
        contributors = "connium,ericfennis"
    ))]
    Landmark,
    #[strum(props(
        svg = "eJxtjrENgDAQA1ex0r/Im+RJEbIBQyBRfINEwf4CGihI65Ptq8d6OrY57BkFBgutDk/W6ksSNMHEQBn/eCGyKztgAl07eyRIyaJRMjR2mvefFv9cLstPLVM=",
        categories = "text",
        tags = "translate",
        contributors = "ericfennis,mittalyashu,johnletey"
    ))]
    Languages,
    #[strum(props(
        svg = "eJwdi0sOgCAMRK/S9AIKunABXEaJkBgXpInt7Z2yevNNo55CX7+kZQ4H09DMkanVfjdBBI1kY7LMO2rzuqTFfyU9/a2kYV40AqC5XcE4ia2vyg8l2Bw1",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis"
    ))]
    Laptop2,
    #[strum(props(
        svg = "eJxVjbEKgCAYhF/laNf8L7WCcnaotV1oaAkaoudPnYobjvs4+KYr3Qf2uVlpIH7rE0GYEkXF6L8bfMZTPEy0tSCaA6idSwKpL1F6zNi62GnPH1YFLzZbmjC1xRtexQka6A==",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis,csandman"
    ))]
    Laptop,
    #[strum(props(
        svg = "eJxtT8tqAzEM/BWRu6fWw2sbtoF+QD/CbA97yKHQkkO+PtIuISEssmVkjWZG8+/4X+nn8/RdSWQUKpQ9OEmy03n+iPZ5fgHxhK4L+gRT4nirkaB3dD4YUKgR29eERnE3chLivORkMCFPLTX/SO2PMylKFNRGBXfa0u4IqiQHEoW4DXHOYHZocj9btYfdDmbYNjZZE3If0OLnISPmUnJNnL1h4X837RPesAU51gAXZIWwl5caqMnhGrQPHkbpayzZL0lQqu/WyrtYA9s1Pz3eAUYLWOM=",
        categories = "arrows,design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    LassoSelect,
    #[strum(props(
        svg = "eJxtTTEOwkAM+4rVPeGSS0qGoxIP4BEnGBiRYOL15FrUqbIS2bITt1f/PPG4TLczVLvDURJCSjYt7TTspe2hyhVi15kDY9YoFFLuhYwNxk6RkuItBZXn5Ii0PPegXTKt/w4nOehwSHSFrr8zSIZNbbDvfvMDzDwp6A==",
        categories = "design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    Lasso,
    #[strum(props(
        svg = "eJxNjEEKwjAQRa/ymX01P5KgkOQGXsBdiYUWikhx0dzeTFqlzOIxf96fkKclzwOWKDSCvFbaytKYwnm7p/DuPyOeUe68gpfew8PUYefhcNg6N9I+tKmNFObpNaDYKDdBYcO6Q8OTobpq7a7GdJo3ke7f27787C9IfC6S",
        categories = "emoji",
        tags = "emoji,face,happy,good,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Laugh,
    #[strum(props(
        svg = "eJxtyzEKgDAMheGrhF5AzJIl9jYihZIUdPH29mkUhJIM//A9bV7PzY2aFzv2Jc1MOKEe6EhOWaewWRG12PqtupXgjH410Jg/LzfnH78AQ4cqjA==",
        categories = "design,layout",
        tags = "stack,pages",
        contributors = "colebemis"
    ))]
    Layers,
    #[strum(props(
        svg = "eJx1jksOgCAMRK/SzAUMfkMCXEaJsCUk6u21SAwS3XTRvjcdFewcadfoQEeazvrVRQ0J2vwSncYEChchYFTDuFFJuncPP5Q8n/qc+La+GNGiTpOVVnxhYUTu/FfxBCGJPT0=",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[strum(props(
        svg = "eJyFjkEKgDAMBL8S9gMiCl6afkaL6bUErL+XSLGKhd72MDOsS2FVOhkTKGXGCJIQd1HGAjripnKvbIR3g+HevSRz5setQq38rA5fkk2t+a20PhcvCeI9PQ==",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[strum(props(
        svg = "eJx1zFEKgCAMBuCrjF2gVkYE2g06RKQ030IG1e1LH8QIX8bY9//TwW0C4TJICOz8zmJwRDi9FU7bSz3CHeesmxifdSplIFXmf79y61iFwRpcSIHiBPH0hakGNNSkawt5AHTcO24=",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[strum(props(
        svg = "eJxljUEKgDAMBL8S9gMiCnpI+xktptcS0P7etFLBep3MZDmFTelymEBn3FUcFlAyMIJyxRLiIWpgheeh+J5r1Q4WFH/uuvfdt8qP+o/7+Zbd3NQuIA==",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[strum(props(
        svg = "eJx1zEEKgDAMBMCvhP2AFIV6aPoZLabXErD+XlMUpNBLctidDSVtSpLyIcrwoDPvKgy3gkp9PuhizKBqN4bJ+jH0qqUfeCd8o24Zq0Hf1n7sBgsCLiA=",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[strum(props(
        svg = "eJxlzEsKgDAMBNCrhFxAin9IexktptsSUG9vU7VQupksMm8o+k2AfThYLM4IZ9iFLZoFIV7pIqTsEW5NR532HWX1Plq75rYZ/oVafastGwvTxlTYAwN+LiI=",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[strum(props(
        svg = "eJxNjFEKgDAMQ69SegHp/FHYdhkdbiB+jILd7W03EX9akrzE17Qx3GXnHJAWBAk4I7R+qwqHkFM5Mo+4NrOin6wX/VmuBOLUIy3pX3WBxgSZUtKYlzTPSOnZV3B/8gFf0SaV",
        categories = "design,layout",
        tags = "window,webpage",
        contributors = "colebemis,ericfennis"
    ))]
    Layout,
    #[strum(props(
        svg = "eJw9jbEOAjEMQ3/Fur0hbps2lcpJ6GZ+gA0dAyMD/y/CDSdbyhA/e36e3zde1+VOIuuto0NDxBBHE240MRjYUaU6OJD3iIaq0ON4xCOTqvR4a2LQ+ljWefl3r/NcCIa7pgKKWzIpLTj11LYRE6xiGQyXw/ls+AEmyyMJ",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,energy,plant,autumn",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Leaf,
    #[strum(props(
        svg = "eJxdUEEOAjEI/ArxvmOhpZRk3cR49hEbPXj04P8j7SbqGkJTOnQYZn6urwfdT4erkMiNITrBm5FAbGK4KWX4FOWq0ErjSBEc740YSiUijajgZtGbNQeqIzcko/qkYK7nPSAJrtR+2rlnjSLpuuuN/y1zjMz2R78xjyFfNV1HaV2T6iWjuBMrilBBSXGvMO6FEDf0bVwqscNKZ09cujbL4Us4c1jmY7dqmXeGERvZB3sD0LZBkw==",
        categories = "food-beverage,emoji",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[strum(props(
        svg = "eJxtybEJACEQBMBWFht4Vo7D4N4OvogHAxPBQK5+MdDIdMb6PyrKGxoVCgElZHuWZtv3MUL9OgnJGS8hEKeemPErHVU=",
        categories = "photography,multimedia",
        tags = "book,music,album",
        contributors = "johnletey,csandman,ericfennis"
    ))]
    Library,
    #[strum(props(
        svg = "eJx9j0EKgCAQRa8yzD7LEkpQ7xIWFBSEtMjbl2NWi3Dzh+E/PjxlZ2eXEeyhkdcI7joVgvX0GlXG3qit3ycYNK6CyQbuqAVF4EL/obhgXQOS8ZaIIovFzM3REGHFD5Yc/OMgMBm9Cic1dT0Q",
        categories = "accessibility,medical",
        tags = "preserver,life belt,lifesaver,help,rescue,ship,ring,raft,inflatable,wheel,donut",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    LifeBuoy,
    #[strum(props(
        svg = "eJx1yzEKgDAMheGrBPfG5BklQ+0NXN1Fh44O4vltF0Gow4MHP188tyvTMXeLE2T1XQIYpOzByMqURxICOzsN5aNLsa8oxZdOpMjWCGq14PYmgvyhT3kAImAq+Q==",
        categories = "text",
        tags = "text,font,typography,alternates,alternatives",
        contributors = "danielbayley"
    ))]
    Ligature,
    #[strum(props(
        svg = "eJxtTkEKwzAM+4roPVrjNE4KWX+QR4TtkMtgh/2fOYHt0mLL2AhJLu/26Xjel+qVGd5THsyOO2xzMmegNIViHeV2BheXo9yG8ig//UtgtVqfuaoMMLSNmjDH9BruiCMxMdkRDHFCeBFR7anc9YLwlit9+zNfFTMz5w==",
        categories = "photography",
        tags = "lights",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,danielbayley"
    ))]
    LightbulbOff,
    #[strum(props(
        svg = "eJxtTTsKgDAMvcrDvbGJtlWoggfwEFKHjg7eH9NSOkkI4eX94nO9Gfc2nOzAcyIxDAqGKYDJGSF9G1obkHoncoeHh63jsSQLdQmaACqgUBOmistq0rDHsfTtsbdq8JL9D8EWInnuzAfmLiZs",
        categories = "photography",
        tags = "idea,bright,lights",
        contributors = "ericfennis,danielbayley"
    ))]
    Lightbulb,
    #[strum(props(
        svg = "eJw9irEJACAMwF4p7iJFBYXaDzxCcOgiOIj3WwclWxKabQn0YqoHvzEJJsPkrmV6bWCGbCNEGxQd/3IAMF0Q5A==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere"
    ))]
    LineChart,
    #[strum(props(
        svg = "eJxFy0EKgCAQheGrPLxAjSRTYEK7Nh1CKDCIaNFCb5+aJbN48PONvuztsI5iGUA886Sg0MYjMFgY3SRg9M9IgZ20lXXoKzv2c0OgUZAU8PLdUNbHnm1SxaZWiCwvaUPOH30APtUo8w==",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[strum(props(
        svg = "eJxNyjEKgDAMBdCrfLKLplCr0BbcXDxEQaGCiIODvb2JgsiHhOQ/f6QzYw409WA3usHCopEwHFw2FH2tJPoPstUivZAlMptc/ei27guKCcSGcHGgTpaeLaHw8xarKt78lB+z",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    Link2,
    #[strum(props(
        svg = "eJxVjbENwCAMBFd50ZvYAcsNYYMMgZQiBUWK7K8ATUC2rnjp79NT3hvX4U5hSCgKBY8zr7F9DTSlZJ5toJJ429EgLqeta3L6ZREiS00jdRsFrCNsA7WLaLF9iJQicg==",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Link,
    #[strum(props(
        svg = "eJxNjF0KwyAQhK8y7Ls0SjAtqDfoIcRIV+hDEUmTnr5uWkJY2J/Zb8a9YmPMnu7a4hotLIZeuk+7TKzGRU3RwOzqoIyS/bhgDubkVPZDwV0kObiaUwPn8uDmSRvC6qn3zdON8C5zY0+j4AIGl0pNz4y0iYq6s2n9I79n+AJCzi3e",
        categories = "account,social,brands",
        tags = "logo,social media,social",
        contributors = "okcoker,csandman,ericfennis"
    ))]
    Linkedin,
    #[strum(props(
        svg = "eJx9ybENABAQAMBVPnqRR1A8GxhCotBIFPYPui+QK49GmQ1qFN0AetCblVYkUicSsf5sRgOuhXugfk5gswAJAiWx",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "ericfennis"
    ))]
    ListChecks,
    #[strum(props(
        svg = "eJxtzbEKgDAMBNBfOboXexGDQ+zs4kcUHLoIDuL32ywiWG4JeQdnZ7kq9iVsVFDWMWQb/JftK9qHBM5dEUJvpiIQpBbGdtU4/auH76qz5/UHXPgoUg==",
        categories = "multimedia,text",
        tags = "queue,bottom,end,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListEnd,
    #[strum(props(
        svg = "eJxtybENABAQBdBVLhbgIyjObWAIieJKhf0jmqu07/GeR2l1NxIVRXPC/pGwRSVERfgMAqFptrlXJRXR",
        categories = "text",
        tags = "options",
        contributors = "danielbayley"
    ))]
    ListFilter,
    #[strum(props(
        svg = "eJx1zCEKACAMQNGrDLvIFIZhLls8hGBYNHh/ZEWL1v/g8+xLYRTXEAFjTU44WBO+QkA/wPyUaDP1dGgD8Zsc8Q==",
        categories = "multimedia,text",
        tags = "playlist,remove,song,subtract,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListMinus,
    #[strum(props(
        svg = "eJxti7EKgDAMRH8ldFebSEOG2NnF1cGt4ODo4P9jSkAcwnEcd4/Tuz0XnEvaCAHLzqnq1LeqH0EZC6A0sujOgOY8ePPFVY7oToC0zhFh4BjYRX7kBYkpKn0=",
        categories = "multimedia",
        tags = "playlist,queue,music,audio,playback",
        contributors = "karsa-mistmere"
    ))]
    ListMusic,
    #[strum(props(
        svg = "eJxtjdEKwjAMRX8l5D3aG2vpQ7tnX/wIUaGCiKCI+/s1Yyts7CGEe88JSc/H6049Mgemv2ZW1F0jHFOvVndpb1KXRnV27AS6dGteyTO0HScptkdNfl++hW6Zz55Cwc8bsW5B4IpugECIJ391AlLROoePQLA7igqaPwD3iDuV",
        categories = "text",
        tags = "number,order,queue",
        contributors = "ericfennis,csandman"
    ))]
    ListOrdered,
    #[strum(props(
        svg = "eJx1zSEKACAMQNGrDLvIFIbCNFs8hGAwGsTzi2ll1v/C59X3hJFNQwT0NZjC7rXCIgT0A4y6REiHFPDvMq3QBSuRJC4=",
        categories = "multimedia,text",
        tags = "playlist,add,song,track,new",
        contributors = "ericfennis"
    ))]
    ListPlus,
    #[strum(props(
        svg = "eJx1jL0OgCAMhF+lYQcpLf4kyOygD0F0YHQwPr+FQRdMc23vvuTCma4Mx6w2h9AvpGLoShTDCwZA9wfGJkAnJHnwYOtMmoCNr6qJlq9o12iIxDvjxbImwwgoa0W53OqW3N6cP/YAIHQwew==",
        categories = "multimedia,text",
        tags = "reset,refresh,reload,playlist,replay",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListRestart,
    #[strum(props(
        svg = "eJxtzLEKgDAMBNBfCd2LzYklQ+zs4upecOgiOPj/mCyCtIQsebnTuz6NzjXsnImxzaHo5LeiP5GxJMpDAFvkkAoCJZ+IiBaX/vWydnEl249fMZwoMw==",
        categories = "multimedia,text",
        tags = "queue,top,start,next,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListStart,
    #[strum(props(
        svg = "eJx1ykEKgCAQBdCrDLOPMM1cqDfoEJHSuAhCBqrbp21qUXz48Hnf5jgzUEwLsUONcDrsEfYUmO6dD4cCobREb9t693abmCA4XCWIAboS1ajKFR4ehQRN5htE9yvmJRdNmiz8",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = ""
    ))]
    ListTodo,
    #[strum(props(
        svg = "eJx1izEOgCAQBL9yoRfZw1wwOaltfITR4koLw/sJHQVku50Z/e7f6D3cxSCwLcllXduZtUdyTgDSuIkkZXsCwcPvxG0Whx5CkalYAccUKro=",
        categories = "files,text,layout",
        tags = "tree,browser",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListTree,
    #[strum(props(
        svg = "eJxtyzEOQBEQBNCrbPTyg/yNYqk1LqCTKDQShTi/1dAoppk3Qz2PCsWJqDQoHYzw9O3O0xUEfANf7EsaXxh/MJIzJaazWfatHtc=",
        categories = "multimedia",
        tags = "playlist,video,playback",
        contributors = "karsa-mistmere"
    ))]
    ListVideo,
    #[strum(props(
        svg = "eJx1ybEKgCAUBdBfubhH3TIpeDm39BGBg4vg4P+jTjroeo7EP3m4R30kuL+HsrJWs9LGwMyC12gCb3BbNPToznLoLwOnqSR+",
        categories = "multimedia,text",
        tags = "playlist,subtract,remove,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListX,
    #[strum(props(
        svg = "eJx1jkEKgDAMBL9S+gA1KUgOsb/xIIhn+3tjqoZIewrsDpvhfTvWUHCJcwwF9JxySI6ECDHzeDOZKykd4J/RAYk9+zDaUV2H7i5+r5OJSJiGqcWqw1s7q9QyJptGs3DbF+AVQnM=",
        categories = "text",
        tags = "options",
        contributors = "colebemis,ericfennis"
    ))]
    List,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJUMDRKtFSwVDBQADJ1zfSMDC11LfRMzZTsbPRByuwA+mYKxg==",
        categories = "multimedia,layout",
        tags = "load",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,ericfennis"
    ))]
    Loader2,
    #[strum(props(
        svg = "eJyFj0EOgCAMBL9ieABKJSAJ8BsPJsYz/l4KrdFA9ERLd9pdv2/HOiQVhAIxnBCEyU9uc5eg/EY/oij6IkUJ4JARfJcPsZWWVmrp5gpSxdMWUk5OlpYbCZruUXkLOiD5NpVgg42xR2DFaSgF9FLQ6cJVb785uo7Lsorhrhd2AaD0YOM=",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[strum(props(
        svg = "eJxtzMEJwCAMBdBVJAsUU0opqMsED4L04Em3bzRVofUUkrz/TQy3V1lbQFCFh+aZ0cLBK7bVma0iZ4TyEXE8Z6bO64v1RG/mkPpfb5F8M4gD9YqBKSSKXlGRd7JwgqLckbxXbF+wBxtnQhs=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[strum(props(
        svg = "eJxdkN0KwjAMhV8l7L5xp3X9gW03XvsQMgUFES+8cG9v0nZuSiGHpCdfSPr77XGhGUMD29Bsi74lV5G0a8Z+p6axL9bFImq1BWsL0r+5cjb4rrRK+mtdPPUze7/8Dfd5el3pPDTHwABpOHQcHUV2iTpCy1CxU0uOY5AARyE/cCKtehjt2HNMxgpA4YrdwBE5gOA4+YmRDHvHNhlIKJo8tUYHGB1gQn7sg1TBTmrZI94VXtasa9UL1wPVZZclP8K8XpY=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[strum(props(
        svg = "eJxljkEKwDAIBL8S/EAxUEohyWekh0DoISf9fY1CU9LTIDurplbvK3DMsEMQBUYIjBkUgjaWtA2pJFNfRxmnZB08V9kXMc7OoNi5v6p9y+LH5eUJqp3aFYg9JnH2DMeQPC4P2Tw3Dw==",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[strum(props(
        svg = "eJwlizsOgCAQRK8y2Qvo+gkWwA1s7YkQoTOEKNxeNmaKN8V7OoezIDdDE+FNvkRDvBFiSFcs/TOh/aiGZkKuolo9SGj17UqEN7QrMB/KrVgx9jG441nEFMd+AFgb1Q==",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[strum(props(
        svg = "eJxNzMEJgDAMBdBVPrmLJipeajdwCEGhgtiCRez2pqUHyeEn4SUmrNFhm2nhEb0bVoGg02JNefi3aLRzzUDWtPnImuDPdB7XjuCPK94zsaoJ+ogF2k+FVmRNoUmUCeHV7AmJ65hzzL7YD0wcJss=",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[strum(props(
        svg = "eJxNjLEKhDAQRH9l2F7udrhThCS1ja29oKAgGtBC/941WsgWbwfejIvtNqDzUpegVv+WIL52mjFj88qwPPwkuM9VCS4u0zGNc4+4jPO2etEcWtgIlLC/SOojBZfUnV5KwWFQGvXmbqRefnJPA+4mWA==",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[strum(props(
        svg = "eJxFjF0KgCAQhK+y7Hs/YxI+qDfoEGJBQUFID3X71IVimF2GbxgbtxT3heLjGGCKt/zk2LC3nWBvz3CtNDs+FEih0e1QXBqF/HwCCAiKFPVVulpLaky+Y5Yw5NI38QLGjiFR",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = ""
    ))]
    Lollipop,
    #[strum(props(
        svg = "eJxtjr0KwzAMhF/l0B4qHSZ4sPMGXbMHt+BChxJKad4+TgL5I2gQ0t19Uvh034xHlHsNataOILSUVazY+m1GmbNxvwB/dkiAWaUJtwnahBXtYb51J5Q7k9xF0nT6aqekV5/eT6R/FKsFfRQK0lDafHiRDza/6ot7c402VD7R",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[strum(props(
        svg = "eJwlizsKgDAQRK+ybC8SDbJFkhvY2osJbsBCwuLn9mYNA1PMm+dK2gTuHIU9GkJ4PI4I79+c8s7S9lLBgMH1KgR3rsIQPc4EZlrosFDT2Yv0ojB8hfAY9g==",
        categories = "transportation,maps,navigation",
        tags = "metro,subway,underground,track,line",
        contributors = "danielbayley"
    ))]
    MSquare,
    #[strum(props(
        svg = "eJxtjUEKgDAMBL+yeE81TdoqVF/QTxQ8eFDw4P+xLSIIkjCXnWTjma8N69wdHuxISeFNcFQQcjBhQsNQhsF1Ewus3R9VpqKKz9awoqGqAwlJqh+7Jfa1YolvkcMIhf4kbMvFJ7sBFZIlMA==",
        categories = "design",
        tags = "horseshoe,lock,science,snap",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Magnet,
    #[strum(props(
        svg = "eJxtzLEKwzAMBNBfObJLta6unYCbuUO7djftkKXQIeT7YyckZAgCgbjTS/88DvjemxcJu75DJghXRyh8+OMNTsaPg6lphxpwaJs+XSrSp436FSpKq13ETWMuXY9lVcaE6gLcs3ROXi3AVtqL3/MZZ4cmtA==",
        categories = "mail",
        tags = "email,message,letter,subscribe,delivered,success,read,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailCheck,
    #[strum(props(
        svg = "eJxljLEKhDAYg18luPe//kFbCz3nG8719nIOXQQH8fltBUWQQCAk+eKS1ozp3YwktPu5RBC2ytDw094zuCn/FioqAbVg7pshvipkiCdqLihvegkenfhUti0Oqxg1FOtgv2XzvI7qoCG7q9kBi08l2w==",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailMinus,
    #[strum(props(
        svg = "eJxNjbEKAjEQRH9luD7j7prL7UG82kJbC7ughcUJFuL3m4hKWFhmhhlefpTnDdfdcDSlwRkvHLl1OueJDmV6qRSDQeppqGofex/s1BdAD3W0ekhdaIyQ1ZHOw5I3jbnkH/luBpXQgBg5FeUc8XlfAiVBDq30H78BVKQoIA==",
        categories = "mail",
        tags = "email,message,letter,read",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailOpen,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l6D1rdqybBmLPHuzVe9BDLoIH6febCKWFloWF4c289Mnfgte1m0jo+WGZIHw7R8dbv83grHx6qKhENMAydGM6NcmYFtW7qoIbJAZcJOTa7fF/TaOO4g3+Xjv76aQRarMdEYPGspIfyCktRQ==",
        categories = "mail",
        tags = "email,message,letter,add,create,new,compose",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailPlus,
    #[strum(props(
        svg = "eJxtjs0KwkAMhF8leN8xme72B2rPHuzV+1IPvQgepM9vUlAEJUxImMlHxkd9rnI7HWZSTFGubaVQNCox8Zy/d+FmXFQMhkHC4GpEOUzjMUjT+ObdndelHkMnBV31eJa9BckSoa3oxTO/p7P1YgXsFzAhoyQ4J1klTEI7wqcWeUETCQ8UxzdhoIk/Q3/QVCE3qH28F1Y8Opo=",
        categories = "mail",
        tags = "email,message,letter,delivery,undelivered",
        contributors = "karsa-mistmere"
    ))]
    MailQuestion,
    #[strum(props(
        svg = "eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb++64j9GSELIe7+kuYX7gGOrehGwkN27IBCYFFq0bMvfGTKxRAMmphppIYMnq7pmk0Bd88ZdZ5zXFdUelnyY1SWWkkCshYyD2c2atbXnCsKhQJGky12HPOVwkzl8bfE8xssJ8dEqrhTiM/exVUUS5fX/YyKayab8YF6EkD8y",
        categories = "mail",
        tags = "email,message,letter,search",
        contributors = "karsa-mistmere"
    ))]
    MailSearch,
    #[strum(props(
        svg = "eJxtjLEKwzAMBX/lkd2q9LDjBtzMHZq1u2mHLIUOxd9fO5AQSBAIxJ0uffNvxvvWTSRMJTz7TBDaxtHx7vc3WIwvhYnJgAY4GyV0Y7q00pjW3qf2orvKEBEk5qp7LKuVzFG0hz6qc3ydWA1f/Dkhi6ht7A/ZAy6D",
        categories = "mail",
        tags = "email,message,letter,delivery error,exclamation mark",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailWarning,
    #[strum(props(
        svg = "eJxtjLEKwzAMRH/lyC7VurpxDW7mDu3a3bRDlkKGkO+PHAhkCBIC8e5emeo84vfo3iTs+ukrQYQ2QuEzHn9wMX4DTE0zGuCYu6FcmmQou+rvqiR3zQk3TdWzEdtpGhNq6BFenjmpWoJvRDzTmjM5shVquS2S",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailX,
    #[strum(props(
        svg = "eJwdi0EKgCAURK8y/H2mUlmgnqBLSEq6CEKE6vapDMww8J7O4SiIIZ2xGBIL4Um+REOSE946hM/QRMj9WD02werblQhv6JISaljZpjAz5QTbJvTiNWKQjC/ge2Wa2iT7A/bDHQM=",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[strum(props(
        svg = "eJxdjsEKgzAQRH9lyLmm7pJoBPXixUuvHnqTtlBBVKiI/n03saVFAjNZeOzbfGrnJ+6FujCD0pbBiOVRJL/a/M8RN5m2FSOFgUWiLWxN7sbaIwaknaRZ3FWV+dnvLfNp7Le+Gx6Yxm6YX4Uie8pAbg+iQH4Yob+3hN1VJhYi0UmIuTmet8Q/UZCsXKhUYZOiWJr2XqUTjwbLG1sLOuA=",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[strum(props(
        svg = "eJxFy0sKwzAQA9CriNl7mhmntgt2TtAeojihzqJQgunn9o2zSNBCIPTiMuWK5ZtICWWaH6UmEktYF0f4JeoJn3msZZ0dDfHUwBBf91owJnqqwhvPAss+ZMNnz9YIq90qoLs6+OaaONxNEd4iuYOw8AXaUqTfn382kSdF",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Mails,
    #[strum(props(
        svg = "eJxtjt0KwjAMRl8l9L6xTX+WwjbYA/gQQwUFES+8mG9v0k4HMgonTfhySP+cX1c4D+aYMAZQTIwuQ4WrL4J3JwcyA08VMxUsERpbKNlkxv6gurH/SX1BLwsBE4k2qzZ/teTEO7MIa2/ZMnTIK9pMF5FpxyziAIqJsGOoaN6i2gBhbfX3H0HurKzuHRyxiJbSZrCEPinylr/fHhdYaDBEBhYv1cB7LW0qUQ2NHx3KTS8=",
        categories = "maps,navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[strum(props(
        svg = "eJwli0EKgDAQA78S9l7crSA9tP2BH/BWVkHBg1QP+nu3SiBDSBKPcq2YE42eIayMwQWI/+x0wVl04ktAAJsEMoAnyrFrzxx1q7ovqIl6gt6JxBsfI7fRX+cX/fwZuQ==",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[strum(props(
        svg = "eJw1jEEKwCAMBL8S8oGSiqKg/qYUQVRoD/r7JmovyWyYjW81j7sWaDWV9wmowIADBaQZTmLiQVYygxOUE0Z/7Gr0OZULOgV0CIOXQujnSrzIiizONkUxOAukl8MPZ4Xz737pfSbX",
        categories = "text,maps",
        tags = "location,navigation,travel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Map,
    #[strum(props(
        svg = "eJxlyTEKwCAQBMCvLPZH2EvhBS7+IB9IF0hhI1iI7xcbG9sZr1/L+O/wGFSzheTHpOQrqCA7uVfhhVMiTKLYu34A7k8XQw==",
        categories = "food-beverage",
        tags = "cocktail,alcohol,beverage,bar,drink,glass",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    Martini,
    #[strum(props(
        svg = "eJxlzEsKwCAMBNCrhFygxuhCUG/ThSAqtIt6+2ptSz+bDIE3Y0uONYY0Q8khrYtD0sAgaRyD3k4X8faHzS0ZSL/xAatsiwJh66kQKjnk9raQ1P3XqtOKgXhUHnYHLHoxPw==",
        categories = "arrows,layout,design",
        tags = "fullscreen,arrows,expand",
        contributors = "colebemis,ericfennis"
    ))]
    Maximize2,
    #[strum(props(
        svg = "eJxtzKEOgDAMBNBfaeYb6DUsE2Mag8UvQVQiyL6fzQBLlhN34uXilW+jc3V7IN2WDALNLVxXUZfi1ESKr4NQODrIMB5JJfFFP1kbNoLiCWKdZJTf5wORMylt",
        categories = "layout,design",
        tags = "fullscreen,expand,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Maximize,
    #[strum(props(
        svg = "eJxtjuEKglAMhV/lcP/f1abeO0GF/ttDiAU3MAiJqLdvJqSUDDbY+XZ2qlt3TzjV7hhJGFxAKARE4rwTCPZWDOLMC0mbU266HhYlQBLLCmUKpIN1A388pltpOVAs7ZFrqt30vKm+EdjOBQVZM/IfuHI2A6q+JN1wUMS02veXsR/OGGtXOPSv2nG0+bT5cZ/ldQAB68NL8rTkewMtgkZh",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[strum(props(
        svg = "eJxtzMEKwjAQBNBfGXLP2smmsUKaP/DqvaCgEMSDiP17NxbaSy8z7PJ282t633Ed3fkkIeEfCvKjlVEYoRau5ENzJa86EOxF4yVVfxQlgnS640hJYJJhUvvbwc58L4O39abr43nDzNEFh+9Sc7Bq49JGGyo/4YYrJA==",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification,disable,silent",
        contributors = "karsa-mistmere"
    ))]
    MegaphoneOff,
    #[strum(props(
        svg = "eJw9yzEOgCAQRNGrTOgXHVcIBXICPQSJhY2JhaHw9K6FlDN5P1/1PrAv7lSQYJLQOK025ib6uJKHT5T8u430EYw+VYVihEUSfBK7u34By7AVog==",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification",
        contributors = "mittalyashu,ericfennis"
    ))]
    Megaphone,
    #[strum(props(
        svg = "eJxdzDEKwCAMBdCrhBygVcFSQb1McBCkg5PevjFxaadPkpcfqXZqBWgktA6BpmbnMJjjqfccW30KDMfrC2Gu9AjDJrx5tDKyXmrbZQLKSziMVRwUh5+VXi9qbrW/v80vrkgsww==",
        categories = "emoji",
        tags = "emoji,face,neutral,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Meh,
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrBPdok2pVqM4urg5uBYeODtLz2zqUCg1Zkv/4EHu7x8O1NLsBmgPqZrVdylabhZRMvUyTSFHomGsdIwpLEmHwrKoyOgYGFYeAkT2ZMgAO1FJO0uh20uNBRQ/jtvXljRxw+Pfw6535hxcV3lbG",
        categories = "devices,development,gaming",
        tags = "ram,random access,technology,computer,chip,circuit,specs,capacity,gigabytes,gb",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MemoryStick,
    #[strum(props(
        svg = "eJx1y0EKgDAMBMCvhHxAW0F7SPoDHyG2mN6kBNTfa3rSg5dl2WGp5lWhnowe4ckB4WopuWyijC4gHCWptBqps0OkfVGBxDhPEMT1BjZ9wPlfGd9yA7EKJVI=",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[strum(props(
        svg = "eJxtjUEOQBEMRK8ivcD/GhGLchsLiVhze1VlZTWZ5r0O1dKyGTaCRTADd3bujoMr/pDoW1QiYfUojt+KP8ab1G8yEnQkXHYCXnwgpA==",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options",
        contributors = "colebemis,ericfennis"
    ))]
    Menu,
    #[strum(props(
        svg = "eJxlizEOgCAMRa/SsBdpLcJQOYEegsTBhcTBeH7LoAt523v/61XvE47VtQwLCAoYrujUfdGv7sTADwU/174IBiF5SqZ9TrwJMI+3xsE8Rox/ewFPZhxV",
        categories = "development,arrows",
        tags = "combine,join,unite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Merge,
    #[strum(props(
        svg = "eJwBNgDJ/zxwYXRoIGQ9Im0zIDIxIDEuOS01LjdhOC41IDguNSAwIDEgMSAzLjggMy44eiI+PC9wYXRoPmBODLo=",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[strum(props(
        svg = "eJx1jjsOwjAQRK8ySr8ms3YIlkxuQEuPQrENEgXy+Vk30Gyaad782vvxMTyv0y3jfF/2WZiYqihU1HTa2mk4tvbzkciWI3BxwN3zmKFI1SUqUKL2A0B/AG+QkR4nTBhNFXA1CV68VidSULosQTCD2uU//gWc+UPs",
        categories = "account",
        tags = "comment,chat,conversation,draft",
        contributors = "danielbayley"
    ))]
    MessageSquareDashed,
    #[strum(props(
        svg = "eJxNjLEKhEAMRH9lSL+cmVuRg13ra2ztBQUFEQsL9euNroKkmOTxJmFulh5tlIoKzRuCyGzU2fYvRufh6xcGHXv1bwDuUobP+agM4zB1WBlFc8GqUX6CzUIzS15p7mnd7gW/kjpMcpGqdj7uAbUDJ7M=",
        categories = "account",
        tags = "comment,chat,conversation,add,feedback",
        contributors = "danielbayley"
    ))]
    MessageSquarePlus,
    #[strum(props(
        svg = "eJxNjCEOwCAQBL+ywV/a21xTQ9E1tfUkCAQCgeL1gCNjJiPG19gy0uM+KvSKBHFOVKa9dxGD/VsGhVltD2B3wR9rFAbHDxGz",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    MessageSquare,
    #[strum(props(
        svg = "eJxdjbEOgCAQQ3/lwn7IXU4CCTK7uDq4ER0YHBwM3y8sYkyHNk3zGq50ZzgmtZCATwwMpoqwptmeKCCr7AZJk/atRM6uz6pzGTcVw9BAMXScA5/5tySqRJSM9vuEXJBexAOz9SIC",
        categories = "account",
        tags = "comment,chat,conversation,copy,multiple",
        contributors = "danielbayley"
    ))]
    MessagesSquare,
    #[strum(props(
        svg = "eJwli0EKgCAQRa8yzD5rrEzB8QRdQqagoCCkRd0+tcV/f/H/81e8N1gYT9JgG6e6ATJM1MpqqOiAcnrlbMVMBkhj8G1Rg5c9ybGCPIw0IcjLmCsxjuXzr+ED8pMZlw==",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Mic2,
    #[strum(props(
        svg = "eJxtj0EKAjEMRa/y6b51kljbQmfAA7h1X1BQEHEhxbm9bbWOiyGQv8j7kBdv1/sZM4+KWeHVk0oqzC2muKnQFB/pecFpVAfyxgeQGJa9M8Roa2hD5cBZt1ot/NUsaMicHFxnGXaFI4tgZHu0SSAfVFuz85qMyAofELIsLGz9hsta2GZZrehrWbNbV00KP8839OtCsw==",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[strum(props(
        svg = "eJxNjEEKhTAMRK8yZF9+k34tQu0N3LpwV1BQEHEhRW9vK0Ili2Qyb8bt4ZgxttSxQIKBgc6j0hVt0aih++rrKzOQd7+c9660NGAdJVjYB2TFf+iopLDrsk04pSUWwsVpN4STX53+8sAZ8zeTryXh",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,listen,radio,podcast,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Mic,
    #[strum(props(
        svg = "eJxtjbEKwzAQQ39FZD8aycZJwM3cpWuGboYON3Yo+f7YBEIGDzecnsTLv/J3fJ/DO4Gzz8OaHy1a8wUCJGePMFZUJkwYwXqjMbqx01xQifpARVBbgybTljzt8Z5Bn55cSFsoPM3NbXwt9x/cwzU8ABDWOw0=",
        categories = "science,medical",
        tags = "medical,education,science,imaging,research",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Microscope,
    #[strum(props(
        svg = "eJxtjE0KgCAYRK/yMRcoox8D7QYdIkrSXYhY3T41yII2s3iPN8Kq2dEpUYN2szgtUZUgrcyqnQRrQPYIDJR2EEUMBpGygNrbM8QPnsPuueM52ianaZEYGSfuuygieomWWO+rHxGKj7kA6fM0fQ==",
        categories = "food-beverage,home",
        tags = "oven,cooker,toaster oven,bake",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Microwave,
    #[strum(props(
        svg = "eJxtTKsOgDAM/JVmfkBbRibKNAJ+ALcEMYFAkH4/q2FmOXF3uYc8+S1wre7ACMsWMgHBZPBVKTdfmQryPXsewm7t0yUZbZ+kvRAga+wnrPwHH/jPHto=",
        categories = "arrows,navigation,development,gaming",
        tags = "sign,signpost,direction,version control",
        contributors = "karsa-mistmere"
    ))]
    Milestone,
    #[strum(props(
        svg = "eJxtTssKg0AM/JXg3dTMvsEK/QCvvS+0YEFKD0Xq3zeroj2UJZnMJJlN+8rvgW7nqo+EIVZdeypK1+56IkzCxppenJbgEFO2ZKlZHvsAAkPSyN55TtHuXdm7xUKM9IEjaYw1CyJLwuVwCiSN+uCKJuvSpmo1+F9eY6rNnzt13WXPNtCS1v+d5kKx5dXDsDWuxGEzPp53mnGugIo+G86iqHQBHS1D3ReUO0TC",
        categories = "food-beverage",
        tags = "lactose free,bottle,beverage,drink,water,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MilkOff,
    #[strum(props(
        svg = "eJxtjsEKwzAIhl9Fek8WxSQKWWEPsGvvgR1y2GGH0eefXdrSQhF+9Fc/LZ/6bfC6D08BajKM5bY4Y9l9BZrJZ9HKwBAs0PmUCcgT6tvymLwKP7Z2gAwYrEkThWpzq2tZS8fa0ez0RA5/sjuQ3QmNgBHYVmSii1ftbqzJsz3Xte9E06Xu0o+btwN+z7Y4xQ==",
        categories = "food-beverage",
        tags = "lactose,bottle,beverage,drink,water,diet",
        contributors = "karsa-mistmere"
    ))]
    Milk,
    #[strum(props(
        svg = "eJxly0sKwCAMBNCrhFygSepSvU0XgqjQLurtG8X+EMIMIS+25FhjSBuUHNKxOzTAOjRSCL1dbuTtxIVerGn+vNPKDpkQztYGoYrDVVct4ea/Vobhfuw/wzI99gLCETHt",
        categories = "arrows,layout,design",
        tags = "exit fullscreen,arrows,close,shrink",
        contributors = "colebemis,ericfennis"
    ))]
    Minimize2,
    #[strum(props(
        svg = "eJxtyqEOgDAMhOFXaeYb6DVZJsY0BotfgqhEkD0/xQBLlhP3iy+f9TI6lrAl0qYVBJp9wl6rhpKnR5T8Oggl404y9pFUkmg/6I82ghIJ0rijDPvoDZPFKW0=",
        categories = "layout,design",
        tags = "exit fullscreen,close,shrink",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Minimize,
    #[strum(props(
        svg = "eJwlybsJACAMANFVQhbwU1nEbOAQEgUFCxEL3V7F6uAeSR3SMsjyaCyC7N9xo5FJfWfqcRZIHoMDY4t79BYfYIASZQ==",
        categories = "maths,shapes",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    MinusCircle,
    #[strum(props(
        svg = "eJwdy7ENgDAMRNFVrFsAJTQpbG/AEIhEOB2KLAHbQ9xc8U6fRzucXsEKemKt9dNckApo/JRBd69uIcrLDJSv3Y2qYCuUssUxST+R4Rat",
        categories = "maths,development,text,tools,devices,shapes",
        tags = "subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    MinusSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVUwNMowNFGys9EHidkBAGN3B1U=",
        categories = "maths,development,text,tools,shapes",
        tags = "subtract,remove,decrease,decrement,reduce,negative,calculate,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    Minus,
    #[strum(props(
        svg = "eJxNjEsKhTAUQ7cSMn88by2o0HYHLkJUvA4EkeJn91ZBkUxCDjlubqKi85wqSAaTYn+Wwf0vENzStxHaj4NGT7HE4ZkTy+5piG3soqaWEfeQbtchuMdai4EU60f4khJGtHzBCSL6Jgc=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[strum(props(
        svg = "eJxtjL0KgDAMBl8lZBdN/IfW2cXVXapQwUGKFH17o4J2kAz5joNTZnZmmcBpTBHMobGQt2ukGhsVP7ZR67BZGDV2zEDs04GBIZGjSFabhRxxn38MwvZuXY2gRFIqffZjKmCy1StObLsqHg==",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[strum(props(
        svg = "eJxljT0KgDAUg68S3i76WkWH1hu4uouKz0EQKf7cXq1oB8kS8pHEzI0TdJYqVmBd51Sa+M5K85KJM3ASaejoUuBL3zpIPw7iLHFK2C0pwjZ2Ti6XEJYnOSz52l0Is/4wX9P/YVVAsRQfOAGYjS2R",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,download",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorDown,
    #[strum(props(
        svg = "eJxtjDEOgCAMRa/SuKO0oQETZGZxZSc6sJg4GM9vu7BI2g7N++/Huz4Nzm3a0QP67CoBgZVBQ4YKH9bgzPrpTSkuaqTYPRLAhbtn1cvrIBmAsIUBQKnwr/uTS0vJynb2ATplLWw=",
        categories = "connectivity,devices",
        tags = "share",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    MonitorOff,
    #[strum(props(
        svg = "eJxtjT0KgDAUg68ScgF9taBD6w1c3UXF5yZS/Lm91kFFXBLIRxI3NUHReVaSQrI6Z+mSmJXuIfZD5r4N2D0zQvtx0OAplljHLqinSYl5O5249KzFwnvQQPLF/lwVMKLFDQ7ShiyL",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[strum(props(
        svg = "eJxFjDsKgDAQRK8ybC+6UdEiyQ28gJ2ouBaChODn9iaIsRhm4DFP74MXTIY2LtCgRpmF9GR1HonVbh49ZF4X8Ya4ItyGSsJlSBHcW+c6eQmriLd4sPrTdqzAzVH9wkRaKJY2gQdMGCZj",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[strum(props(
        svg = "eJxtTD0PgjAQ/SsvtxfbQ0tJ2s4uru5EiMdmSIP677lOMJA3vLzP+BmKYEz0cAHh6QcGw1YYNny/HjV47XatzBIox0u9yHE/snD9atqm92gbdzupdNqQQ7BMr4LvPBZJ5AnLLxETZJrfUhI5S1DHafJX4rqri7wBKQwwZA==",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[strum(props(
        svg = "eJxNTcsKwyAQ/JVh701V1KSgnnPpRwQjVeihiPTx910p1LCH2Z3ZmXGPrWXsnq5mMlBiXSi4cyeD+0tyxiVPQg6ppthQ354U4VX2lj1JQWBCMvPxpAk5lVtuzNju645D5AK76k1BQfQ58fa042ZUWY++WGq8J0ROloZQGQix18396SeHL1oQNsw=",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[strum(props(
        svg = "eJxNjFsKgCAURLdymQ2UFj1A20GLiJSufyHSY/dxJazPmXNmTPRrostiBJ3BJbboQLdFD2IfNk5STKYSbzLZfj1df4pqISc6TxtQzKHM9iUxOYtZaVL90QqR7kcG0oqHAh4fCStu",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[strum(props(
        svg = "eJxtjbsKg0AURH9lmD7EuytoYNc/SJs+qHgtBJHFx9+7CiqITDHF4cy4/h8UlWf3gSSwL4sYFu69gcId+CsGYn/ZRYa6DNC6bTR4SkosnpaY2iqop0mIORYx7BW1TbgNZmP6cJXDiOYnWAFZXC1K",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[strum(props(
        svg = "eJxlzUsKgDAMBNCrDNn7Sa2o0HoDDyEqxoUgUvzcXouiiGSVPCZjptoJWksj6zAFqzANzqHSRF5K83hxMz48d43DZkkRdksJYb4W6YZenCXWhHVonZzX2Md84P1asQJni/73VTkUS/7AASlyLo4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[strum(props(
        svg = "eJxFi8EJgDAQBFs5rgG9U9RHYjMaTEB8hICxezcG9TWwO2OiWxKdYU3esrZMGWCKFd6FzSfL0jNdljueTVOC2ezhcJQV14ALVAGlMoNTcYv1uthEf/dptTYyfvINC74lqw==",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,virtual machine,vm",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    Monitor,
    #[strum(props(
        svg = "eJxtizEKwCAQBL+y2B/xNBEOLv4gH0gnpLBMIb5fFMFGppwZ/VPJ+G7zsINPAQF2IBMLBpOQvCbq0euo6xH4em6EY1yZlmkXJhpl",
        categories = "accessibility,weather",
        tags = "dark,night,star",
        contributors = "karsa-mistmere"
    ))]
    MoonStar,
    #[strum(props(
        svg = "eJwBNADL/zxwYXRoIGQ9Ik0xMiAzYTYgNiAwIDAgMCA5IDkgOSA5IDAgMSAxLTktOVoiPjwvcGF0aD481Avf",
        categories = "accessibility",
        tags = "dark,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Moon,
    #[strum(props(
        svg = "eJx1jLEJACAMBFcJWUAULISYZYKFYJVKtzcQBAutrrj7J+kqo4HMijEhyHKqAZmCa6aT3dpH5ZWZyN+zDY6gIPg=",
        categories = "layout,development",
        tags = "ellipsis,menu,options,operator,code,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[strum(props(
        svg = "eJxtjLEJACAMBFeRLCAKFkLMMsFCsEql2yvGJmh18H8cchPu1fEoECI4nkrZAEKvN+HVzmzk9LFMLD+xBZAAIPg=",
        categories = "layout",
        tags = "ellipsis,menu,options",
        contributors = "colebemis"
    ))]
    MoreVertical,
    #[strum(props(
        svg = "eJw9zLEOgzAQA9Bfsdhxm8sduUgpc4f2I6oysCAxMPH1HANslv3ktv62GdOrWxwZCof1BkOyt3yi2ruxPU4ztkt+lUkD8Ol/4SB9ohUYRSNpRqEPVIGwBGPV2LRC4JTMVO/HAy6pHFg=",
        categories = "nature",
        tags = "alpine,climb,snow",
        contributors = "kerkeslager,ericfennis"
    ))]
    MountainSnow,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKtVAwVjBRsFAw1TVVMFUwNPUw8gEKVSnZ2eiD1NgBAOC1CoQ=",
        categories = "nature,gaming",
        tags = "climb,hike,rock",
        contributors = "kerkeslager,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Mountain,
    #[strum(props(
        svg = "eJwBMgDN/zxwYXRoIGQ9Im00IDQgNy4wNyAxNyAyLjUxLTcuMzlMMjEgMTEuMDd6Ij48L3BhdGg+MmsMEA==",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ericfennis,csandman"
    ))]
    MousePointer2,
    #[strum(props(
        svg = "eJxtTjEOwzAI/ArKbhqwASO5eUHziEodOjRSh059fbEtZepyoLvjuPa+f57wuC6Hg4MAMRCalSTIrDcmoBKCf5etXbp3a+cFKa4WhjkKcskT/3gNqVZg5OwRb7FVt12QsoKhq7xSp2rq6k4ZXSJrHTQxw8AjWqkYdNQhTP789wOqoS9m",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "mittalyashu,ericfennis"
    ))]
    MousePointerClick,
    #[strum(props(
        svg = "eJx1TjkOg0AM/MqIfklmdkmEtOEF4QN0SBTbIFEg3o9pgMLILqw5nZdxLZh+Vd8gjoLwPibYVXX5dbBdPjVsbyLCFc0UbBNokvobUh3/MugzOIENxCswKHitVlro4mJx30wPjoh283DRCLkGpptjB6mwUE4=",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = ""
    ))]
    MousePointerSquareDashed,
    #[strum(props(
        svg = "eJxVjDEKgDAUQ68Surf2x1qX2tlBVwe3gkMXwUE8v+2gIgmEhPDCkc6MbVAzBSJLlwjCVmlqjr8OXuK+oSSzVzE0FRLDg9qFKHYQCzG9dqadWCa/vt8bV2sbew==",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    MousePointerSquare,
    #[strum(props(
        svg = "eJw9i0EKgDAMBL+y9N5oDCYUYl/gJwQPXgQPnvr6NpfCsrDsjH/X/+A+0isQGK0GViqGjXbORlIQlWOeA2mp+hJO9WmyYESh8+t7GRUe",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ashygee,ericfennis"
    ))]
    MousePointer,
    #[strum(props(
        svg = "eJwVy8sNgCAQRdFWXl4DKkHdMHRgEUaIw86QiZ/uhe09uaHmw1Bf4Up8Qkc8JZkKJ0+0PBOay6nWbGQMQx9iuHZTJOE2OSy379BT/AGU8Baw",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Mouse,
    #[strum(props(
        svg = "eJxtyTEKgDAQRNGrDOkXGZcMCmtu4CEEizSChXh+TSFYhN/9F+d2VexLWjP8piqVSgztlvjsyOAMWY9GCG6Ot45yAtXM/v4Acckfcw==",
        categories = "design",
        tags = "arrows,axis,gizmo,coordinates,transform,translate",
        contributors = "lscheibel,ericfennis"
    ))]
    Move3D,
    #[strum(props(
        svg = "eJxlzEEKgDAMBMCvLPmAROlBiP2Nh0JpC3owvzeN9SBeslkYVlrNmlPZ0Woq57FRADMCnqAo0yui/Cyv4AX9jufr3ercHeFi27YYVb0ad3oDyBwmmA==",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[strum(props(
        svg = "eJxlyksKgDAMhOGrhFxAxtJFIfY2LgqlLejC3N5YHyDdzM/AJ61mzams1Goq+7YwHHlCuAfgKNNroowan7Vxf92lzqYC04Gn9j2T4orxTk+/vSaY",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal,
    #[strum(props(
        svg = "eJxFyiEKACAMRuGrDC8gP7IwmMsGrXbBYDR4f9Si9X1PZ1uDenQFIEjiiuBM/c2mH4U48xmebcWDD4w=",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "siarie,ericfennis,karsa-mistmere,jonas-hoebenreich"
    ))]
    MoveDownLeft,
    #[strum(props(
        svg = "eJw9yiEOACAIQNGrMC7gGCOwIdmg1e5mMBq8/xyF+t+3u96BXXGQAvEkbcToVqK7pQpIj0PTPtMbD8M=",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDownRight,
    #[strum(props(
        svg = "eJw9yqENACAMBdFVGhYg/YIgSicoFk+CQCLYP1BTd8k7OfNuWi31SlyNQYBx+Z1UsqNKLK4DCHnwMxA+",
        categories = "arrows",
        tags = "arrow,direction,downwards,south",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDown,
    #[strum(props(
        svg = "eJxlzDEKwDAIheGrSC5QdBAHm9t0CIQk0A719tWUTAHhB/l4Onq1WtoFo5f23GdCAQEiQD8B5JT1WCjrxjl0YN7sdIY+SQmM/r5eivqfwk/7AbzfJoY=",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis,csandman"
    ))]
    MoveHorizontal,
    #[strum(props(
        svg = "eJw9ySEOACAIBdCrMC/g+IEZkGzAQ7gZjAbvP0chP73rHdq9TKHmIIYLsRTTGmKaHjaAlA/Icg+Y",
        categories = "arrows",
        tags = "arrow,direction,back,west",
        contributors = "jonas-hoebenreich"
    ))]
    MoveLeft,
    #[strum(props(
        svg = "eJw9yqENACAMBdFVGhYg/YIgSjWiDEGCQCLYP1BTd8k7OfNuWi0NrlQNIIb95JJUsqNKLI4dCHnvjhAw",
        categories = "arrows",
        tags = "arrow,direction,trend flat,east",
        contributors = "jonas-hoebenreich"
    ))]
    MoveRight,
    #[strum(props(
        svg = "eJxNySEKACAMBdCrjF1AflgYzGWDVrtgMBq8P2KR1fdsj7NoZm5CQJcCsFt66hZOKpSg/y64nA9V",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpLeft,
    #[strum(props(
        svg = "eJxFySEKACAMBdCrjF1APrIwmMsGrXbBYDR4f8Si9T1bfU8aiSsiSYY2gN3CZbefSlKEoO8OxaYPjA==",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpRight,
    #[strum(props(
        svg = "eJw9ySEOACAIBdCrMC/g+IEZkBNgtbsZjAbvP0chP73rHdq9jEbiDIKzkBTTGmKaHjaBlA/JSg+m",
        categories = "arrows",
        tags = "arrow,direction,upwards,north",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUp,
    #[strum(props(
        svg = "eJxtykEKwCAMBMCvhHygZA/Sg/qbHgRRoT3U3zdJ6aEgLOwuTBy9zlraQaOXdp2JdxINCCAJujnH7UM5LnhwbTj8rbuJxADTLYnFGm9P/TDv9gG89CaG",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[strum(props(
        svg = "eJyFzjsKgDAMBuCrhF5AEugQqN7GoVDagg7t7U2igo+hU/jJl0eoJfUU8wq1xLxvs/PAQIAEHtC7JUy3WMLPsiJS7mFkhSCbJrmAPOKsf9gjOvbZbrTh7MhBl4JSG0nUTJbFP2y/mu1s2pBGfNkDyeRNCw==",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[strum(props(
        svg = "eJwBRAC7/zxjaXJjbGUgY3g9IjgiIGN5PSIxOCIgcj0iNCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+n/QTIw==",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[strum(props(
        svg = "eJwBQQC+/zxjaXJjbGUgY3k9IjE4IiBjeD0iMTIiIHI9IjQiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+ZA4SWw==",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[strum(props(
        svg = "eJxNykEKgCAQheGrDLOPGKVQcLxB2/ZhQYFBSETdPoeoXD14/+e2YZ9hZOwskOmbSKpSB2n0rpbk3QtWC5nk+qewpBAnSIwaIVyMZPKejK2Yp35KfumFLtkNU30mgw==",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[strum(props(
        svg = "eJxtyjEKgDAMQNGrhOwiabFUSHoDV3eJgkIHKSJ6e1s6uHT6w398LtcOq+A0Avl5iGQ6c5PFwH1ZgfVIGjdIghZBX0HyuY+gK6belnJVZf2zD7l6HyA=",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[strum(props(
        svg = "eJxVy0EKgCAUhOGrDO413tPQQL1BhwgKCiRatLDbpxZBm/k33/hjOlfMQYyD0oQ2PZiSlQYWRpIiK3UZEX1XcfTfhYzqNZxyDsTg1DD+OG37gkxBsMD1hktK89NCK4o3FwghoA==",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Navigation2Off,
    #[strum(props(
        svg = "eJwtyrEJACAMBMBVHheQBEQEdR0RJAlo4/ZapLviqum6QwWmU85ugRgMKmDCJ2UkJ4deo+/+ALgSDuw=",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation2,
    #[strum(props(
        svg = "eJxFi10KgCAQhK+y+N7G+pMK6g06RFBQINFDD3r71oRiYWZn+CZcy73DGsXsUCt4RQFRdiD5WNHYwXAtUhgbnMI3IYvKM4xWgWQ8D769Gif66XycG1SKQgqoko299Fh6ZLRB6QEtJCHB",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NavigationOff,
    #[strum(props(
        svg = "eJwtisEJACAMxFY5XEBav9p1RJC2oB+314K/JKS6zdNN4TZ0r5YKiMAMBhUwhT2ImqTmf8sFtm4O3w==",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation,
    #[strum(props(
        svg = "eJxtjcEKwkAMRH9lyL3opFgUdvsHXr0XW0xvUpba/r1ZRW2LBJIwyZsJQ3dNGKYoFMzeK4F1/c1SFF+nt/Lo22RZqMMuA3V4YfPm+vFZG+iGcum0+vml67+oe5MMbZTzAazGomwIYu/ls6BRlwI4lhnN0AKlgno5fi9PUnpDAw==",
        categories = "development",
        tags = "tree",
        contributors = "ericfennis,johnletey,csandman,karsa-mistmere"
    ))]
    Network,
    #[strum(props(
        svg = "eJxtja0OhTAMhV+lwZe7noylJLtoDBaBIyBqSBBkzw9DwASp6Xd+cuI+H0brvxo8ASZhBoHcfWCM/mW+uNeSCakoSBam7XpLiZG4XRxLLXWbEwxD1cVfHu7iMy9K4o31y2pI1Lj5shwF05SLY5iewAllezQR",
        categories = "multimedia,communication",
        tags = "news,feed,home,magazine,article,headline",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Newspaper,
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8SPMDYpDVVqL2Bhyi4cOnC+2M/tKsS8sjAPBLe9D10n8ultMNK8nCWKkwezuthdYlhLc0YRv+AU1IIJ2Z4pcYu5bTtE4sFB5NDtrZyNTYLpuapprCeJIkBKzX2V2KG8ANXli9x",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Nfc,
    #[strum(props(
        svg = "eJxtUUtOQzEMvEr09jGxY8exVHoC2LJHBQmkglgg1N6ecYoqFlX0En/mjSeT3dfz91t5ud8eWYo+ybbf3WVpv7s2rHD70Wen1rRc9raWkbiXQT79QMpC3JRmE5KQwsiELOQBxCJHGsI1twP1Sp1GVNI5L7iawJzAZW0XfiUWq0LR44YsjtTVqWu/1YUCOVRGG4PMO8Johr215MSHsg4UJYuaSrr1bJVV4Qm5qgh1ZD7xx2rIJGcEIygA6oVLtvlK4+agaR08vRCHVnIfNJPLLNEeAAh/MFhMYWQELJzT4Yk5AILhMxJqltNhcQ7pMIMHojF7OgcsTvQlPMf3hbW5xkMtnM6LKtasKQNPty4E7EgUG/T9e/Tj++drOcv9JrKVM+PcyukvPa0U0ATtfwGySnmT",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NutOff,
    #[strum(props(
        svg = "eJxtUD9PRDEI/yrkdrBQoCV5XmJcdXVwuzwHBwcHc59f6EvedGlCaeH3B7bf2983fD1f3llAP+Ry3Z7q67qdBQNud70Nak3hiG0dIxkDnMYcOykLcVOaTUhCgPMlZCFvSSzyQy6MFXbqSJ08kHTOow+r8aW4OxzxUOAA1jvqA1dl93WCgpKB5z13JNFOMZCigBQmKNAz6yndZpmKkWIS0JGBTCmEYhW6OaTRzDRh5DoWgyJTiwk5SnKlTaMIK9fG1SujSgvXj/kymy4FsxPmEqlcjOxLQvoJk5abMK1N+EzrnoxYhdbTG/v670tx1DDJLDlj1T3jsImSOQrmXlMNNc9E/TyX9g9MgGML",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,diet",
        contributors = "karsa-mistmere"
    ))]
    Nut,
    #[strum(props(
        svg = "eJxFzE0KABAUBOCrvByAvIQFriMlT7Fxe/KT1dQ307hKeUQqUCmV3jwz3GpAkJpLtRIRjjy6xee/3XKYBSfuc5i5PBcj",
        categories = "shapes",
        tags = "stop,shape",
        contributors = "colebemis"
    ))]
    Octagon,
    #[strum(props(
        svg = "eJw9ySEOACAIAMCvMD7gGA4NyA98hJuBYDD4/6mFfLrHcZgNOwO7LAGqLmiaPpgGU35eAi7D/Q/j",
        categories = "development",
        tags = "keyboard,key,mac,alt,button",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Option,
    #[strum(props(
        svg = "eJxty0EKgzAQheGrDLN36otJ6UDiDXoISQstuBBxobfXJAhBJIFZvO/38T/H8UtxCwzDFNdy58Ad9/5R5t6fLM3KmbuszJ3aispzblytpmH50SfwG61YMhAd0NLxy1NRiwZOLJ4pS7yOOnFkBHXTqLws6NLs9Lw4/w==",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[strum(props(
        svg = "eJx9jDEKwCAQBL+y3AfCXcBYqL9JIYgKSaG/z2FiY5FqWJhZV0vqKeYTtcR8X54OWOxgwQE2FNw2jeCG19iTMKGJJ1Z23SxKGdTgT1aatzGLOn7s96Nc0ik/8w0urQ==",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Outdent,
    #[strum(props(
        svg = "eJxljLsKhTAQRH9lsM9ed5NcXYiCnYW2FnaCRRrBQvx+Y+EDZFmYYeZMWKctYq6y3kIjlzvnk0CQp2OTVOvf3sigY1aH34nV4YKXBEPIeeNIm7uPgsTBRiUvzwqYyvTcCUO/Wz0L7P6/gwMIIyZR",
        categories = "files,development",
        tags = "box,container,archive,zip",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Package2,
    #[strum(props(
        svg = "eJxtjd0KwjAMhV/l0PvWJevsBu3AB/DW+4GCg/2BQ9zbm9YpvZAEDjn5TuKXbr3jGtRIR0izlNVWtf4QN63/7s9MoOJSd5EoYmnSZFw5aKdt5oqKBXv6WShRP7Mg5FIMCpSZMcdykf48H52pYA07NKgMVRkxD9vQTzcscz+tj6BKww0ciGNzYRzBJXwHW5/wjYJiVtg4KBJ90a6fWQIJfgO7tUVH",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[strum(props(
        svg = "eJxljUEKgzAQRa/yyT6pM0ajkAg9QLfdCy0oWBUqpd6+k9BKoCTw4c37M37ttwG3oC5Ug+qhVp0/Rdb5Y8IEKq5Nz2AU8WnSZFw5aadtRiUFwZ4PhBLNKytCNsWiSBmMPZaNZP+PP5ypYA07tKgMVZmxTPs0znesyzhvz6BKwy0ciOPnwjiCS/pX7HzSdwqKWWHnoEjy/UtKKYUkfwC7KkRs",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[strum(props(
        svg = "eJxtkMtqw0AMRX9FeD/qSPI8DE6g+3TbRXeGFhpwQhYhJH+fKzsPcMKABHN1pSP1h+H4T7+r5ksjd0KVa4uQMimrDcKd0RSivyBcheLGWKhlsUFZlKYw6xwTGeduFNiVMpwDCmbVG7VwqxDkJG/cXYC7/DTr/sPB1v0db2cQZzp4LYXMWRZ4aDvhjeiR/HsxQAjYTrepXOBruaQHnYQbnb2Fg/cVbtzu/+iiq0asobNnRZY5X5BVvdjLnpvg0CTG6WRsBVOinxrhDiHTGjaGDNZYlzuiolTsGHJw+XPRgHCfzLV8+4gH6RVJJmZn",
        categories = "development",
        tags = "box,container,unpack,open",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[strum(props(
        svg = "eJxtjUEKgzAQRa/yyT6pM4mmQhR6gG67F1pQsCpUpN6+Ey1toCWBD2/enwlTM7e4VupMBahoC1WHQ2R1+E5KkF3+TZhA2eXYMBhZfJo0GW977bVLqKQguNMHweK4JEXIplgUKYGxx7KR3O/xuzc5nGGPErmhPDHGfu274YZp7Ib5USlruIQHcfycGU/wm/4W67DpK1WKWeEpSTF5z3VPKWzyC0CoS9U=",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[strum(props(
        svg = "eJxNjlEKgzAQRK8y5N/UXQ1RUKEH6G//xQoVrIqVorfvJlYbEnZgeLM7xVQvTzxKdWMCxfesZjBi9yKKSNukj2yUBq6oWEivp4UE2ScIQja5oECB6XIsGylVVXFxZ6viOP6y2iDVbJHDaDIBMfZb3w0tprEblnepEs05LIjd51hbgvX4D6wKj29UKmaFjUtFouuh5FUCO9x0c9O3aDaxjTYKs+ScNqs4mfZVduhf9yZ3pStZN1ma5GfhL9QaUkM=",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[strum(props(
        svg = "eJxNjl0KwjAQhK8y5D2xu01IC0nBA/jqe0HBQv/AIvb2bqqtIYGB4ZvZCXO7PHCL6sIEKq5Vy2AU6WnSZHzZa69t5oqKBXs+LJSoXlkQ0pSCAmVmyrE0klVNOKWzTdiPD944WMMeNZwhlxFTv/bdeMc8dePyjKo0XMODOH0ujCf4Df+BTdjwlaJiVlg5KhJ970qbSuALHwtIOks4uEE7Wev0f8QHjChGbA==",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageX,
    #[strum(props(
        svg = "eJxtTjsKwzAMvcoju9RKtisX3EAP0LVDt0CHDg10KDl/5ECCIUEgwfvpld/w/+B960bjhMhquCKxpK4vp8r1ZVU8VJAHheJch4SELXzJKDaoX4cQ7xuEgDw1RggWo4sasPo8qbH5O7m89j3GwAGGzIZUNx11Fc/Rp+hGzffYNWU=",
        categories = "files,development",
        tags = "box,container",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Package,
    #[strum(props(
        svg = "eJxtTjEOwkAM+4rV/UKS3sEhHX0BfICtKkMHkBj4v3BQRYdWkTPYjp32Hj8zHpfuZWeYpRojRxCjw6HLuNRnEQcxSZVKKRah19/lvRvaIcKG9o8sdBWUrXJzWD/brsJWXaoNljJ00mR8yeSUXDKcnPQIirYgkNf2Lx1cMMo=",
        categories = "design",
        tags = "fill,paint,bucket,design",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[strum(props(
        svg = "eJxtjTEKgDAUQ6/ycbc2aRUK1dnF1cFNdOjoIP/8VpFOJWR6IS9e+53kHJsFXhBMWDEkt1Mo9gtbasu5Vx5WYGCCvJDJqTPhHyLXi92aKXbv4RTL7SDgygQqbAVnK9XXQFYrC3gAjIcrKQ==",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Paintbrush2,
    #[strum(props(
        svg = "eJxtjjEOwzAIRa/ylR0asB1syc0J0gt0s9Khg4cOvb+KoypThPj6Eo8P9dO+b7zu00MyB4PyEiAR1kk4lUOaQjGPIuXsdss+Lyi4QP7QJgaZe/RMGtKUxZkhM8ShQOE5rfU27q/1/KIg76QIFBE4kSH27EG7kmCh5G0XWxI5QczlMOlEfiwvMfM=",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[strum(props(
        svg = "eJx9kMFuAyEMRH8FcV8XG+wFaXcvOfcjIloplXqo0h7afn0GSJREiiLt4gczHmGW+nGsn++u/q3eSL2rv6vn2Oi4epRteRmWbbmxcrg6Rsv80HvjyB3Q+9zZwuXSY/fWr/3Pwb2t/pXFyQ6iQ3WjsnwnAIfxVSpiDkSW8kRzsjNizdmFiVKcJ2JoOerYMbEApeDPpZ1BNZVBXd23kJ6EbKQ3tB5q+cBUitVIQRmakqpOGCXEM7c17QQ2U9w6YAo8WzLu13fy32ZtM24nGjRe7g==",
        categories = "text,design,photography",
        tags = "color,theme",
        contributors = "ericfennis,csandman"
    ))]
    Palette,
    #[strum(props(
        svg = "eJxtUDEOwjAM/IrFblM7dtNKpRIPYOIFKAwZOjAw8XrOXWCorFwcy3c5e3k93p2el9NNC01tYJM6Anzk4BCcu1GIORlN3TZlJUT307qck7su/wpV1K8hU3IAAwLdowSNrcjgeEOULAUzic5lY+VdFvmBKoRmmqVqgy1NrgayAr5XlhL41MvmkGRACIoV7UDNZkuaNVaZIwGDiVVkE4MIofgczQI/MNh2s8GilRy3Embp3mzfTRYsvf+28QWkH0sY",
        categories = "nature",
        tags = "vacation,leisure,island",
        contributors = "ericfennis"
    ))]
    Palmtree,
    #[strum(props(
        svg = "eJwly9sKgCAMBuBXGbuXmCJ4ob5LpKRQESKkb59TBjvwf7MlHhVKdygRvhxqckgGIcV8prr25lAh9NlLY+ntxn/eXvmJ0Gk4PYRcs9GkbZyS2LLy9t1rguDwJg1GKFBiFMcc+B8VWiRi",
        categories = "layout,arrows",
        tags = "drawer,dock,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KQN1e05OCnkL+439f8yKwlyQckEaEI6BFqPcxCGd7OJeVpXH0nRai32ZhSAEn6oEGJhXNnuJUzIfYP3DvrQu4HCx4",
        categories = "layout",
        tags = "drawer,dock,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomInactive,
    #[strum(props(
        svg = "eJwljG0KgCAMhq8y9j9qSlCg3iVSUqgIEdLbtxmDPWzvh8lhLxBDOmKxSAtCtagRWt+ZD8VoHW/yJXaTM6PknDnTHaARP2fOqJ+VqYhJUsJecTnzbCWCt3itQBPoQQOPyCK4DwjDJEQ=",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[strum(props(
        svg = "eJwlzMEJwCAMheFVQhYosRR6UJdppQqlBwk02d5Er/m/vNjLxfC3m2tCOhE04Y7QJWFAqKU9lVeQFdRDjpv/5fi2r4CSicMETSLBCNlSmGezrvIAIQscQg==",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[strum(props(
        svg = "eJxFi1EKgCAQRK+y7L+ESVGg3qBDRErrRxCyVN6+taA+hoH3ZmyOC0NxaBDOFJgc6gHhegDFtBK/JAtqpUotb5v683afmSA4nEYwh8xEVPSLTfegO2WUAcnnb9DMIMo=",
        categories = "layout,arrows",
        tags = "primary,drawer,hide",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[strum(props(
        svg = "eJx1jEEKgCAQRa8yzAVCbZGg3qBDRErjLmSwun2Nq4Lc/A/v8b8raWU4cmTyqCaEy6NBOFuWpzQCpbwRNx3cIIPg9oUJosfZghqrEiHoK2zVv8J0uH0d3SaAK/w=",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftInactive,
    #[strum(props(
        svg = "eJxFi1EKgDAMQ69S+i+i80Nh3Q08hLhh9yHIKLrd3k5BPwJJXmJTWAUKoUHgEDcWwm5EuKIXfm3KhD1Cfjap1OBsW3/OHosweMJ5AnPqWEGtfrB3AygD06g+fAO3DSCB",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJvSi0/YwWWxAPJWD6ezf1kiHDTuzlUKqlXVUTy8ZkiVemMe/bTq2/7jABMEeOi3c53u0pNAKcoJEZGd4dEAeWvskf6kgb6A==",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[strum(props(
        svg = "eJwli1EKgCAQRK8y7H+ESlCg3iVSUqgIEdLb5+rH7sCbNzr5IyMVQ5IQfDxDNiRWQjWkCKX/L7ocBk6VTatn3ll9xcejyNYtTRYjq+ir2rAU7LJl9bvnAGfoXrFBQU3tuGVuf+e2JBs=",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[strum(props(
        svg = "eJx1zNEJgDAMBNBVQhaQWgULTTdwCLHF9E9KqHV7Cf6I6N9xjztf0ipQGmGPcBJahCNHYUIzIXDKG8udm2LwnQ6C3xdhiISzGcEM1aho9xJX+2+xf+AeXxfB/yyw",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightInactive,
    #[strum(props(
        svg = "eJwljNEKgCAMRX9l7D1qitCD+i+RkkJFiJD+fZs9bIfde3dtiXuF5lAj9LFTzEeqDmlFKCwpRht4c6hpGN7O8uftme8ITbFoEBr97HwrYpIUclZS3j5bTRAcXrQAmUlPGnjEF8d/InQkiw==",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[strum(props(
        svg = "eJwdi0EKgDAMBL8S8gFJRfDQ9jNabEA8lIDJ75vmtDA7k0e7BKzgjjC0YPKxmJ9v6QXpRNC4e+OnS5Cat9XV/PLXQMnhgWAUniXvyasU2N1l1QkOjBxC",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[strum(props(
        svg = "eJwdzNEKgCAMBdBfuew9YgpRoP5LlKRQESGkf98mezhju5t741bQPFnCKxihdmqfpZiPVDzxTPjyXlJvgxv1Lrgz3xGNPS2EZjpVMCyyPpCkZoJ71pKwe7oW8AQ7WEjpWhfhB7nuI/A=",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[strum(props(
        svg = "eJx1zMEJgDAMheFVQhaQth4sNN3AIcQW05uUgLq9jV4U9Pp/yQs1zwI7oUPYShImNAPCcQXOZWG5S203FmPo9CGGdRKGRDiaHjwbBU1P8A3sB7if7l9DJxshK8Q=",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopInactive,
    #[strum(props(
        svg = "eJwli9EKgCAMRX9l7D1iWlCg/kukpFARIqR/32bs4bJ7zjU57AVytagQYkhHLBZpQcitVww0wpt8iX/fpHBmlJ0zZ7oDVOpSVbwgNvhdOZQEm+I482wlgrd40Qw0DRr0wCdciPsA8wkkNQ==",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJvSi0/YwWGxAPIWD8vUl72V2WmSztUHj51F6QNoTe+Oo6t3wFE4LnimAjxeKqeQmv5pufBpb8I+eo4O6VRhmF4GQw9Qf2CBvo",
        categories = "layout,design,development",
        tags = "drawer,menu bar,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[strum(props(
        svg = "eJw1jbEKgDAQQ38luPf06rVeoRb8lIKDQwUH/x+9ogTekJAkX/U+sK/D6ZlEwExTcIk4wVAjIqZX7JQkdTSlsDjDJhDLTAollWZ+goXVw39VTzp3tH9Eh5JHuy4PCMYboQ==",
        categories = "text,design,files,mail",
        tags = "attachment,file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Paperclip,
    #[strum(props(
        svg = "eJw9ykEKQBEQBuCr/NlPL28mUcMNHEJZWCruH0oW3+7TXmZDjSZ7/HaQEG8Bl0n6nZH0PevAQ8AQBLpeWxmZE54=",
        categories = "development,files,maths",
        tags = "code,token,parenthesis,parens,brackets,parameters,arguments,args,input,call,maths,formula,function,(,)",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Parentheses,
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8ypFezWT8I0drGQ4QoKChIkKC3N9EijbDLg3nDaLs6u81wnSApYO9AFXi97HXx+V4f5lwwdWKvUIHKcNHGNLmRGMSGwZCg8DKrh9arn2YLanymck4zD0moJSM=",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    ParkingCircleOff,
    #[strum(props(
        svg = "eJwlijEKgDAMRa8ScgEbK5ZCmtnF1b1EoYKDFAe9vany+bzhPda96rFBTUgOQW9jb3w+Cne/Fz7zVWBNOEegsIQyZA8enI3s4xRb3CJ5AbuCFfM=",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ParkingCircle,
    #[strum(props(
        svg = "eJx1jrEOwjAQQ3/F6n5HLpdeEil0YeYjKhgyIoH4fi4dUIdWlrz42XJ7rZ+O53W6V9RVoQgQlyFMS7uMdGl/RiIkfvUo8dLc41EyhjPyNhxIlA2R9WacIMmtwJFhvbyFjdIGGGXOD1biwomEZ3evnZyqu1M/5nA0HQ==",
        categories = "transportation,maps",
        tags = "driving,car park,pay,sidewalk,pavement",
        contributors = "danielbayley"
    ))]
    ParkingMeter,
    #[strum(props(
        svg = "eJxtjrsKwzAMRX/lkt2qJeVRgxvo1qVrd0MHL4EMwd8fOUMSSBACIY6ObpzTkvF/NV+lHtZvgcBbMTpo5jYdC5vKeeGoC2BquRnjo4rGeOjwpOHHYad9Pc/sSa/0VCExwt+YWMGaFLplMJHrP6HIDWlhhuLk9GEFj5w16w==",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquareOff,
    #[strum(props(
        svg = "eJwdi0EKgCAURK8y/AuUGZmgrtu0bS8pfXchQnX7VIaZxeONyfEs+CxJwtuXY7q4WBIrIVc0EZ4UCnfizNAOzty+MIKlXUOoQ/HsJSTGGlG7bLqpTXI/PPsaOw==",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquare,
    #[strum(props(
        svg = "eJx1kb1uwzAMhF+FyK6rSOrPgJulS4d27dAtcAcPLdChyPP3qACZHNiSaR8/Hk2tv5e/Xb6eT+8VQ1ThYmL2rRk9OfpyOq9PkXNe75lFfEfWA8VMxgNJq9hjyvKx9hNaMlhBrxfDIrFyXEmxNHGoXfMGxWgJtYuiOaVSZ7Qn+NgSxQk0NO69USvlTYtoPvZUJ2OEfcIMCsHBRsKWg4IqbRMfnBM6CVhoxWf0qv2gsCqHy5ostLHiLBzkCJJilL7mlwXVpPA/F6kRLtJg/tGPpkrIN3Jhy80wnKh2GlV2TivnMWb2yFXTbN9vW+SmyOXXKvEScCTzFh7X593wH2lof2M=",
        categories = "emoji",
        tags = "emoji,congratulations,celebration,party,tada,🎉,🎊,excitement,exciting,excites,confetti",
        contributors = "karsa-mistmere"
    ))]
    PartyPopper,
    #[strum(props(
        svg = "eJw1zDEOwCAIQNGrEC7QYtqhCfUyxMHEdHCC2xdFp5/AA5bapRXoL9KJIOpNXpvNfMQ+c6tfASMf3whKwTVFzfsMPdS2cxbkWife9WLbH5wyIUM=",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis"
    ))]
    PauseCircle,
    #[strum(props(
        svg = "eJx1jCsOgDAQRK+y6QEKndAPyVKNKLaeBFGJIAhOD3QJKOR7LzO8zluhZVCTacnY3KvIze0if6X7K177K6IEbT0SQFXsgsZpBEfAWG0CicmCz/Z4b0/5JSL5",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "mittalyashu"
    ))]
    PauseOctagon,
    #[strum(props(
        svg = "eJx9y8sJACEMhOFWwjSwCIunxGZ2xXiVgNq9D/Dq9Z9vuMTPqObfVPCCNOakJnAe1HdpAo/Az4KBN5/JzeXyOn4AOsQbVA==",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[strum(props(
        svg = "eJxtjUEKQjEMRK8Ssjc236RWaAsewIVbd6UKCi7kI6K3N23diBLChLxJJtbLXK8nmBNOCPWZkNn0lVAwx+WgOX5cnYfBTfrRH5dR9uPb5H5tt3I/wzHhbgPsioKCs2JTfaxIizW07tuFpyDA5ET3nnQCXpMEEBIP3Nj2yw5qE7tDy2s5+Q2q0Df6",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[strum(props(
        svg = "eJxty0EKgCAQQNGrDHOAckKFQL1Bh4iUxl3IQHb7mp2Ltv/xQyuHQI/oEJ6IC8Jds3BEsghc6snyVYPQumoKsw4pXLsw5IgbOSDLkyE1rYOt4Nn/djIDvKatJU0=",
        categories = "devices,gaming",
        tags = "computer,chassis",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PcCase,
    #[strum(props(
        svg = "eJxFizsKgDAQRK8ypE/M7iZKYM0J9AJ2AYsUFhbeH7ONMjAwn6d3ezrO1e3E4NiLqzpZV/Vf5pAhITcOdjKLQwSBbAuoXD6NRD4dH/0C6QYVeg==",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenLine,
    #[strum(props(
        svg = "eJxVTKsOgDAM/JVmvoN2XUCMaQT8AG4JYgKBIPt+WsMjTXqP3F06y1Vhn9xKBDJLYWDo7VBZo4+hyPVvIDccXE6dreT0bo0+AvtY2JPG7VmeIEBYVFA8UFQSyvbUb5EQH1Y=",
        categories = "text",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenSquare,
    #[strum(props(
        svg = "eJxty0EKAjEMBdCrfLJPJY2xFdqewEtIFRQqyOBCPb1xZjGb2fxPyPvleX7dcKn0kAg5InGCQj0TK+uXWtn9SSsrzBBlCcYp2CkiDg0G2fshCsnD2LZ2LuGLfFhyFf0+9XHFVCkS+qeSiPd7bkfLu/0AQg8r0w==",
        categories = "text,design,cursors",
        tags = "vector,drawing,path",
        contributors = "ashygee,mittalyashu,ericfennis"
    ))]
    PenTool,
    #[strum(props(
        svg = "eJwBQgC9/zxwYXRoIGQ9Ik0xNyAzYTIuODUgMi44MyAwIDEgMSA0IDRMNy41IDIwLjUgMiAyMmwxLjUtNS41WiI+PC9wYXRoPvZCDv4=",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Pen,
    #[strum(props(
        svg = "eJxlizEKgDAUQ68Surf2t/1K4dsT6AXcCg4dFBy8P/YvOkggkORFrno37LNZKSD4lk2RQbsi3zI6RnRcg1NIzXcRIuIygfJhU09k0/Z/n8RgJd/pAZUVHMo=",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    PencilLine,
    #[strum(props(
        svg = "eJx1jz0OwjAMRq/yiT2mdpwfpNIThAuwVWVgoBID9xc2EQxQ5MSR8uRne7zPjysux93KCQkK3U3j3j+n8Y1OHFFQqUCozELKeKXBI0RSDM0IEsVvamG8FXD89a4VGRJkgwhEzJcCU2rCxAWZquurIU+9uQZtkRI4UzpveLga+tPDnMxQin4XOmg/Pn3uc9+CkFVTXkJnwZm/fWkr5/IxPwFZsEzm",
        categories = "tools,design,layout,text",
        tags = "edit,create,draw,sketch,draft,writer,writing,stationery,artist,measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent",
        contributors = "danielbayley"
    ))]
    PencilRuler,
    #[strum(props(
        svg = "eJw9jDEOgDAMA79idSc0baMyhL4APsBWiYEBJAb+L5IBZOkGW2e9+3Ngn8PKFbknmgSGjAi2FJSlklXRgZROJhmEZAtNR3ebfg8XC8SNf3oB80QWUA==",
        categories = "design,cursors,tools,text",
        tags = "rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley"
    ))]
    Pencil,
    #[strum(props(
        svg = "eJxty0EKgCAUBNCrDH9f+QMFQb1Bh4hfYFAQ0qJun9YiAlfDzGOcLEnWGXJ64p6QciiCXE8Nrns9uH08IiZPG2vYxsAULONHg4WNreKK5BPrn90RmSGs",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentCircle,
    #[strum(props(
        svg = "eJxtjzsKgDAMhq8S3BubtkGF6gn0Am6CQwcFB++PaRVBW0JCwv/l5Y/lDLD21WSwAdJoF4OOIAX9mJVia5A7iCEDbEoToIqAVhEZycoKWUN/QN0T5mrwdbxn8O9VHRoQD6gpV3dyyCKzYuBCs8jyk8P2034BAHk3rA==",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentDiamond,
    #[strum(props(
        svg = "eJxti00KgCAQha8yzL4fCyVBvUGHiJTGRRAyUN0+rUUEbR7vvY/PpDAz7NEzWRQDAoW4ED/9sNgjnHemPDp0pimCM9vEBN7iKiToSoEqqJwvGjVoqlvxQ7Ik5IddSv4l9A==",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[strum(props(
        svg = "eJxFy1EKwCAIgOGrhAfYSNjGQLuM9BDEHnqy28+Mtadf5ZNqeXJQZDgg9OhRS7xtRW+ifaBEUprUHEQZzs1cY8BR6fNgcJJFl7CXeH3Wpx+/5iMi9g==",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[strum(props(
        svg = "eJxlykEKgCAQRuGr/Mxe0rGGAvUGHSIsKCgIiajblwQRtHqL77k4pTgPiKenihAPT4YJ6Q4FVzwa3NptI3pPSwPWsEpgIXnI8GFBDQFDFP+1NQyj9/KVC3lGIh0=",
        categories = "accessibility,people",
        tags = "people,human,accessibility,stick figure",
        contributors = "mittalyashu,ericfennis"
    ))]
    PersonStanding,
    #[strum(props(
        svg = "eJx1UMsKwkAM/JXQe8dNut0H1ELv+hELHnrw4EE8+PUm7bYUVAIDSWYyYYZHec50OzdXEeKALK+uCAk5LW4FnLThjJgrrouE0LUdXLRpv8K6CW34wTeqieK0HycPZpL54GdWiAqC5Cu6pRBJOyNsr8H3OmK+JLhMGZkLq3OogkDhzpDYGhwdVLJovz3MAMcHt0jezTicLKhx2ONiD6cPFLWu7EQR2f+lhqmnmhFpqOx25getik6Q",
        categories = "connectivity,devices,communication",
        tags = "ring",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere"
    ))]
    PhoneCall,
    #[strum(props(
        svg = "eJxtUNEKgzAM/JXQd7MmampBhb1vHyFsoCAqTMbc1y+tOoSNwKVN7nIh5TT2S98Nd5jGbpgflaECGJhBQF9kTV2edk5dRuZClREDL02UaebKMBtYOJSVvlKnZm7hVpmrziJBz8+00cFgNShhjD7k0fkN10aBkiYpWheq+QprRxL5ww/UIHLn73DIkAi4PfgFK3QKjEW2oY2BDvQXCPtqmOVaIroUaD149NSQOssmEJCekF0S4Oigkqj99QgGeFxwP8k7XlcPVX8Ap6pTEg==",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneForwarded,
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaUBAVKkV7+iauFqEly4RNJpkwxTh0S9f2DxiHtp+epSELDAIemMGbqjjtlKpYiTMrycAi2Uui0rCBWRMrPVLHemrgXpob6zYM/Mpqls2pBCWM5FUmoAsbxoZHmyUZpk6r5wixYxP7h69UHXKX73LIkQi4OeipFDoBRp9vmK6BDuSnhP00zM9SIrp6TAMEDFSLG/LigAXbEbJLFI4KMrLO/mqoAB4P3C15r+6KUdUHYkpS5w==",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[strum(props(
        svg = "eJxtUFsKwkAMvErY/8YmbfcBbcF/PURBoUIRP0RaT2+yu60FZWFCJjM7Ie10u19h5s6QNbBQZ9jArEXqIrQ3fXtQUd9GaeKig7P069ykj+E5wqUzZ2Ygi4Ff1cDAUMqjgpG8NBTQhYxp4NFWRYWlU7ZJkCa2sH/0KlWTO26fQ41EwOMuT6PQCTD6OmMZHzqQTgXralg3QhGdPJYBAgYaSJJtNliwEyG7QmGfIJbo/c3QANwvuJ7krefSQ/UfMO5QDg==",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[strum(props(
        svg = "eJxlUO0KwjAMfJWw/4tNWvsB28D/+hAFhQlT/CEy396kHVQYLZdekyt3HV75PcN17C5k0Ecgi5YyeZBtyrLoCBj9QsihV8gMXFp6T9SjOwIxRrdh1cmRMECbFSbkY5u8F3kUQglD2rA2InrbWzRBbx1vWHvi0IrSh0dB5S7vn1B1eefULDixCzzb/wTV196/epcILUDJqYHPEU2ChIm6aTjoB07Dcn/eYKWxY+7gq1UKV7pq1VEdmn4rREnO",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaqCAqVErt6ZvsahFassySZJIJU01jv/TdcINp7Ib5URtm8CDAQBbYNNVhozRVJC4sJAMvqg1ZA4v8XlKtRnqiTu18h2ttLqyLMPCzaHVpLkEZI3lVCOjCiqnh0RZZgbnT6jFB6tjM/uErVYfc6bscSiQCvu/0VAqdAKMvV8xjoAPJlLCdhuVRSkRnj3mAgIFaMUJeGrBge0J2mcJeQUbi7K+GCuD+wM2Sd3RXjGo+VWdS3g==",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOutgoing,
    #[strum(props(
        svg = "eJxtkMsKwkAMRX8ldD/XSdpmZqAW3OtHFFx04cKFuPDrTfqQAUvgQh43J2R4Tq+Z7ufmJkKsKPJuJyGhaMFBwNkSLkhl07WRoW1oEZNX+1XWjgY9mPdRN6XLbzl1YCaZK56jkEwEuds0LoFElvnAfhq63krM14xYqKDwxEbWzaCkD4ak4FITzLJ4/xkOQH3g/pJPMw4nf9T4BdqtOmY=",
        categories = "text,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Phone,
    #[strum(props(
        svg = "eJxtiz0KhUAMhK8ypJe3yQNjsesNbO1FxdiJLP7cXldBLCQQZuab8XPfRqxjFy0QFwTrx8HirbdAf8J+/fk0QqX/pUHppyYaukCVQo1dAil6AXbQ5ZvkYG0EAnceZ5JJrU/vANc0J+Y=",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[strum(props(
        svg = "eJxNykEKgCAQheGrDLOfchzBAvUGbduHBQYRLVrk7VMX0eLx8+Bzx35ukNmjQcjao1YIT7ljia4Jrq8muGu5E6weJwM2KuLOAndCAmWJpcJKfpAH0Co2qUioaZLZfPQF4wwfgg==",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[strum(props(
        svg = "eJxNjLsKhUAQQ38lTK93Z64owu7WNrb2ouLYiSw+/t6dTgIJIZz4fUyKOVAvjHaoR4HAmQoppKu+HXKymxy45LKFDaIVRf+zj+iPZUrQZVs1BWoI1zYnDcSOcNyBhJCdczw5/sYZEV/8ViBp",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[strum(props(
        svg = "eJwVjEsKAyEQBa/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR6nVK6gX3s+qsEd8rDCxb/7uCgnN0CniwDRy2ZCwzMTrRHagtBTaYGs2GxA2njcPdqAOU7iNzxSuI1e4fhEtgh7nS2vEBeEbUbrpXhx7hM+5V+3DjHI06Q+ZGyVx",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture,
    #[strum(props(
        svg = "eJxFy7EJgEAUA9BVwg1wXiLCF74HDuAQgoWlhVg4vV+Lk4RUL36s545tSouYRXDINs4siMZEDMrWp+rdS6v/B4Fq9OOCLpa74QcRMBYw",
        categories = "charts,files",
        tags = "statistics,diagram,presentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    PieChart,
    #[strum(props(
        svg = "eJxtTjEKAzEM+4q53W7sxO0F0vtB19tLOnjsUDL09VUo3HQYhJGEpPZ+foJe9+WhlbyzilNik5VUCmcyzuKTZVWWDCQYIK5AyFTEd0tRBlvkYfMpHVaH58aKAAsDF2w98RT+aaDT7t9la5c5YWvHEKM6tM8OlYoGXOiJT6+kGulQflP1MIs=",
        categories = "money",
        tags = "money,savings",
        contributors = "ericfennis"
    ))]
    PiggyBank,
    #[strum(props(
        svg = "eJxti80KgCAQhF9l2XvlGvYD2rlLDxEp6S1Eyt6+lk5CDHOZbz4d3ZbAu7D7ZJAGhNtgi3AFm/w3xGxQImTeJ92wMOljTR6swYUkkJzHWq2yVsAVbwhEpWbqWeBrKfQniT/SleQB7Nop3g==",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "danielbayley"
    ))]
    PilcrowSquare,
    #[strum(props(
        svg = "eJx1ijEKACEMBL8SfIB3wXhHIFrb+AjBwtJCfL8IopUs28yM1NQKZKciGqCOn/LyTOblmP9qGCiwtom0hfl3jQOa3Q9sSBoy",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Pilcrow,
    #[strum(props(
        svg = "eJxljDEKgEAMBL+yXJ8zEY8oxHuI3YGFhYKF/8fE4hqLnWIHxu72HNjXdAnngjEgTMJtykvBB4aASUlP//FXUOiWqg3RqtaLs7dirrt9AZOcGgo=",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,tablet,pharmacy",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Pill,
    #[strum(props(
        svg = "eJxNjM0KwjAQhF9l6b1rJiZNF9KCNy9eew8oRCjiQaR9e7c/tmUPs8x8M7F/vh40oClsQaNVUR1WHWe7jacJauOMboxmWBmEpYMD/E6fTPemuAnJFxyqZMmS0UMJBkg96fUNNcvlnxnyBM/WdQh5mZuGDnPwJHx2XZWR9pYp3TVwLVvhB0B1M7o=",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6H21L2TrBl3BmxevHrwNFCoM8SBj/nsTZawEEl6S9/HS/HjeacXowI5W/s+P6ahTNbPL6WBvOb2md6Hb6M4tIRbI0sDHbmJiClYqAdsNs/W+8cNxO4KgrqD/l65g96hLTn2tSQoWqcH4gWkHV9xAim09y9VyWsL8BTP1Lo0=",
        categories = "maps,account",
        tags = "save,map,lock,fix",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pin,
    #[strum(props(
        svg = "eJx1jTEKgDAMRa/ycU81iYqF6gn0Am6Cg0MLDuL5TQcdChLe9PL44dyuA/tYJYEImPjQ6MlXU6izmsL7sCiEb/qxiTv0UNeSsYljZBqwnUJnHuCja0tH5qIlQ6bMbCsX67f2AMSFKQ4=",
        categories = "text,design,science",
        tags = "eye dropper,color picker,lab,chemistry",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Pipette,
    #[strum(props(
        svg = "eJx1jbEKwCAMRH8luGs90bhYoR/QjxA6uBQ69P9pgtAuloQj3ONy5Wp3p2M1OxIB3XmYWhZ1a/kYCOmPMYHn7AyCKHhiyzb4TS7ZMYom35LL0pUd0ER9pKGaEB8uWpU3+ACqBy9e",
        categories = "food-beverage",
        tags = "pie,quiche,food",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Pizza,
    #[strum(props(
        svg = "eJxNUEsKAjEMvUpwn2eTJraF0RPoBdwN42IWXQh6f0wHGaU05H3IC5me83ulx/lwU1JdNR0u03Fwl2lXMkohSaMqta5scBII3Jf40IqGamhBmr+QN7V88bVShffMp4DJ4XmOKErxZDDD5dpRlBy2a+lPMxgpdIFFVZSYNyKSoJ06trk5xFiiMtLArbKMTiE9OqkQXxhWgmQ0D5sGnwtD7WrQSAqb/1bjCGbket8P8gF1sD+l",
        categories = "transportation,travel",
        tags = "arrival,plane,trip,airplane,landing",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneLanding,
    #[strum(props(
        svg = "eJxNUEkOwjAM/IrVu4fY2aXSF5QPcIvg0EMOHPi/cFhKFSmWZ9HYnh/tudH9PF2UVDd10zKfBrbMO5PgE0lGoGCls7JVCCPGZi5y9sSQQq7DdLKj7oOuhUQpUuqojHB0KVyF9gCn5A8+hbxhqWyS1BRhiO37pWXPFraqUG6CMVH4cijZulo7wxfkdGOoR0iMhBJY4AbvypoRy1hM/wMZq5ZdrvsdXoLvPKc=",
        categories = "transportation,travel",
        tags = "departure,plane,trip,airplane,takeoff",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneTakeoff,
    #[strum(props(
        svg = "eJwdj7EOwjAMRH/l1N0G22naSqULC0P5CBQGhgwMiIGv51xFSeR79p28vh+fF56X4W6TzrBFHVZh1kNH4b26ocJNRxR+iCYmiXCWQpFgt8CMwvGq3hIqWxY+psRdlD7U/SAwaHAs9gXmXRxxK11SDzgPodhXokcyRowajRYMOPJCo6eTNy1pXUUn1qb+G7b1lAttf7mWLN0=",
        categories = "transportation,travel",
        tags = "plane,trip,airplane",
        contributors = "ahtohbi4,csandman,ericfennis"
    ))]
    Plane,
    #[strum(props(
        svg = "eJwti0EKgDAMBL+y9APaHsRDzGeCSKE0pXqwvzehnnbY2SXJXcoJeY8QU4CMmd1iDUzL9ExNy7i0ommuz+0WO+KGmGDoYIUf/iF/lUMZUA==",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[strum(props(
        svg = "eJwli0sKgDAMRK8yZC/iB6nQ9CDuxBbThSAlYL29hi5mePB4vqRDUSrTSHhyVGEaHEFSPkUbv0wTodoH31sQ/L2rIDJdKxwWzN2/zbSJ8AEpJhe+",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlaySquare,
    #[strum(props(
        svg = "eJyzKcjPqUzPz1MoyM/MKym2VTJVMFYwtFQwNFIwVTAyBBLGSnY2+lBVdgBVDA2H",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "colebemis"
    ))]
    Play,
    #[strum(props(
        svg = "eJx1zDEOgCAQRNGrTOiNDMqqCXIDWws7EgtKC7PnFxsqaP+bTHjSm3Hv5tjgVEwM419iqJ2+Bw5c1DfEY82cGyAgz2JOpyQQWBAcypFVe9X9B0vKKRA=",
        categories = "devices,development",
        tags = "electricity,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[strum(props(
        svg = "eJx1jjEKgDAMRa/ycU81iVKF6gn0Am6iQxfBQXp+00FwqEsC/30eP1zbHXGM1ckKIYG4LurMAl9Noc50Cm9n4QbcJtISav8Rg4ed2Hk0pPbVriaS2CfZTel8DmEhdC0JbJZ83Q8lsy+W",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlugZap2,
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8y2BuzuzFaRE+gF7ATLCwiWHh/zAa0SRj2A/N44d6fE8fUrN4I2BrZ2Tjo2BzRbyEGjbH1KaxcgWVwa+bQqXAOn/ZiMENaKat1MD1IdFkQVQCyWvqMgVzFTmNyO7jTRz0/8QIM0jQN",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "mittalyashu,ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    PlugZap,
    #[strum(props(
        svg = "eJx1izEKgDAQBL+ypD80xx2ccOYHtinsAhYpLSTvNzbBQtlmmWH8LFfFsYYtMpgbaUg+PTD5UAss8weP+icM1rQIBHNfpP4qyRuQZNtHegPetSOj",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Plug,
    #[strum(props(
        svg = "eJxNycEJgDAMRuFVfrKAJqcc0m7gEBKFCh6kiOj2tnpoTw/eZ75l31fkQDwS/C6V0udrtOH3aMd8JiyBJgVL0kp1dcACvRq8h3gZnQ==",
        categories = "maths,development,shapes,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusCircle,
    #[strum(props(
        svg = "eJxNy7ENgDAMBMBVrF8AJTQu4mzAEIhEOB2KLAjbI4sCutfff+p1M7oFM+hqxVQQGKS17Wpv7kMQQcM3OU1+yOlYTakIFqYQlR28+kGIxOcHDzTbHeU=",
        categories = "maths,tools,development,text,shapes",
        tags = "add,new,increase,increment,positive,calculate,calculator,button,keyboard,toolbar,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusSquare,
    #[strum(props(
        svg = "eJxNybENACAIBMBVCAsYiHQvGziEiQWlhXF+Y2No77DGDpqNu5FoSGVHeeb4I0p20lyGNA63",
        categories = "maths,tools,development,text,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis"
    ))]
    Plus,
    #[strum(props(
        svg = "eJx1jr0OwjAMhF/l1D0h5wSTIfQNmJA6sFUwdGBAAvH8OFFpl1bRXWR//iuv8TPhce4uEfLlPYCQJrlGKCJO7zl29pvkz7u+HGp3X5YZzNDJB24gBfMOkuBzRDYbExJCfe7oVZs9HW2hzIimmm122z6BNDaIrNOQXBp0qf4B/eU6Pg==",
        categories = "tools",
        tags = "swiss army knife,penknife,multi-tool,multitask,blade,cutter,gadget,corkscrew",
        contributors = ""
    ))]
    PocketKnife,
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yuI8aK2lhQgdo214oUBAVkqBO32gtZBbz+bz3ZdTJwD6zdYDeoNAcOHR0SJ9fQiPF7muakpeqIQZxG2up4Q9Tss2zSsbgbmf9ATFYn86ZTVlFsgZAQbmgP6RexLYiBQ==",
        categories = "brands,development",
        tags = "logo,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Pocket,
    #[strum(props(
        svg = "eJxtjUEKwkAMRa/ymX3i/HZiW2h7Ay/gbhgFBRdSXOjtzYybLiTwfyCPl7nct/K4YlsCA8rbq/P+eDOs8+F3Xudnft1wWcKJBIfsgehDdIglQk20T+hFDUktq/nSAHGwiPIorJBJaqF2rvqq3clHMGWDNTMxIv6BOICj/5gwNS4K4w78Au65Mew=",
        categories = "multimedia,social",
        tags = "mic,music",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere"
    ))]
    Podcast,
    #[strum(props(
        svg = "eJyFj88KwjAMxl8l7J6YpOlsYe4J9Lp70cMOHjxIn99UpChMRgjkz+/jS6ZHea5wOw0XVRArCRKwh6BXwzwd2n6eOiUJRCpKUdA3yKiolb978H5LaiC85B2lbCkZMsXF9qT87+APKB4GXMPvmyvqFZV8gkYRKY0YKWcfBbtjoLFl9xJwNDRez0eQ2D1fm+1L9Q==",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis"
    ))]
    Pointer,
    #[strum(props(
        svg = "eJyNTjsOwjAMvYrV3Y88N20dKXRmgAuwVWVgAImB+wtHSGWAAXnx+9r1sTyvctl3J7r4YmKS3qNZNhR7+g/F5G6uu1Y71095WEyK+Ld2Z24alb+ClsRXDFGLggwX3tQwCm1VEINialFEwWEKaoyHCCpyI5XHPqTSvKGhVwpced4uvQDH6jhT",
        categories = "food-beverage,multimedia",
        tags = "cinema,movies,films,salted,sweet,sugar,candy,snack",
        contributors = ""
    ))]
    Popcorn,
    #[strum(props(
        svg = "eJw9jTEOgzAQBL+yor8Nd7LDWXL4QT6QDpGCAiQK/i/OSFDMNFNM3adjwf/TfdX5hiammS70kKEXo6/i1MaUWDIu9dCIAxXBGg3BzMIBRr1chPrrxvpqh7Hen80MZpKZG08/AR+vHzg=",
        categories = "food-beverage",
        tags = "ice lolly,ice cream,sweet,food",
        contributors = ""
    ))]
    Popsicle,
    #[strum(props(
        svg = "eJxtjCEOgDAQBL+yqS/c9oBWHP0BjyAgTiII7ycIUlM1YiZj1347zjVsLMiHxHlQ1Vh+QkK18YuqtVSQH04dsyDRmbqG6my3F42TIJY=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8SeoDYxJq2UAsewEMUXLgRXIjnN6XQjSWfxcwbJt3lOeFYzU4BZwFBcVuECFaHgC16D7SYnKZK5tR5Qao4SWk41WOU0P4gQQz8ur9xqa5Nut37AL33JXI=",
        categories = "connectivity",
        tags = "on,off,device,switch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PowerOff,
    #[strum(props(
        svg = "eJw1ykEKgCAUBNCrDH+fORqWoN6gQwgFBREtWuTt06DV27xw5XvDEmXmpKyDU27IHh4aBDsaNVpoSaFvMYVjP1cURjGCYqKw+vzys9620gvx6RYV",
        categories = "connectivity",
        tags = "on,off,device,switch,reboot,restart",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Power,
    #[strum(props(
        svg = "eJxli7EKgDAMRH/l6B5sEoJL7Ozi6l5w6CI4iN9vXFQot9zd4/lRz4ZtSotAm+RUfHiu4h9g6MVcBYIcYYo223+TrNqb+4hwjQyRF9+Q0Ryn",
        categories = "multimedia,photography,devices,communication,design",
        tags = "screen,whiteboard,marker pens,markers,blackboard,chalk,easel,school,learning,lesson,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Presentation,
    #[strum(props(
        svg = "eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv794sZQAgmcnFzi8zyuY5oG5DlN5RnIoYUDwza/1lL0p78Vfe6KoA90dbq8VB2rddayhg0vpt4BFIh1RwBe6uMFWAx/8zU1+sdwK3jrCwQZ0l1KoIbwSn2RQJYJq45q8zczfgAKOTAw",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htqd7dNLSQ99+LVe4lCBAUpUvTvjQqmpWUPuzPDW8bdh0fEydOhRgNFTZ3bfbzO/ZMW9qhr/8aaEDGSo3AZw/WM8PTUEkZPSggvT/zFf+nsMXOxV7D0Ug4CQZmG05apytqkq19oI5Op5oCRKAU3G9XZgm3MDd+AXjzA",
        categories = "multimedia,photography,devices,communication",
        tags = "cinema,film,movie,home video,presentation,slideshow,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Projector,
    #[strum(props(
        svg = "eJxtk8FOJDEMRH8l4h5v4iS2I7FIe9/9gb2NhgMHDkjw/6LKAaERaLprunviLj9X5v7l8vZUHn/f/etb5tjFJda1Sptbhqq0tcVmiMaW8HjusixK6lWm8/BmeNDi88rbfK2iY+Fax6hfj59rF+u9pF5kB47S8OlVYrioG6ynwx5nNMVNVNlG0XVRWa2XoygqrQ7RPksq2pkm3QwAS+Z2lrAWRr55flrBhnanm2wGb55tlqNnFRteOD2f6od+/TaTi+/AKI5eMAXNWUC5sgHCQaKbWHsAC2UxATArl2V74CFTEnUSDT8KIsP6HhIb30uBZJ6FP1khovS76enPt+5RtDH2rlfUWMfUh1UmNQl02P5O3JcQRw8Ka0XLgcmNNWR3EI02ZPUFIKcleuOmaD6o/QapMZ61ayrnEIi1b0Ok3BxrsdPGoaIcyS+8fZg0wzJDUrCqNOXW01VTf6LSA3YlE58kEdjO1suwbjavcsawh8OIhNDNkLj5JkKqTCknXL/HVDKgo2SyiZgqc2JMLPAs/X/3cP+Lf7GHd/XQrjQ=",
        categories = "development,gaming",
        tags = "component,module,part,piece",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Puzzle,
    #[strum(props(
        svg = "eJx1j0sOwjAMRK9ieV+oHZFukq7ZcIiKVrg7VEWB3p44SC2fRFnkMzMvY7dM1wBPjwZhSRshyDTfJHg8Iaz5/TGPQfTeu6Pae5dDfz6N2w1TSf2qn5h3jTVjttR9CAKjxwsTkJXGDAwMra4mnaJRq5q+rUzx0FJBI4Yu7hBSyLkrGA0QS5Vh6hLZ2tdWkVQuTFwvnIZpdukFxMN3oQ==",
        categories = "development,social",
        tags = "barcode,scan",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    QrCode,
    #[strum(props(
        svg = "eJyNjrEOwjAMRH/F6u7gcxpSpNKZhbU7CkNHJFAHvp5LkMqCEIocW3f26Y23y2OR67E7R3GUKCZZwRrmVEwRPGnIaa8eDFld/dSXJoupCy3+CIfsM1BMqkNRvD0w7l1Ym8toRojfOQSzgQ3BIma3sq0Knt007irZNG58SL8A8wdw0f8JF07UvGo86JlCq1/jd5oX7HxAYQ==",
        categories = "text",
        tags = "quotation",
        contributors = "Billiam"
    ))]
    Quote,
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/lyC7VkhXbgTRfkK7dTTp47FDy/ZVDyBSMD5t7HHfzt/4aPs/hpQGFR3tbVSgCBIEMYY/DMj86tcwXKwUqjUo1ONJhMoezn+OHTLmxrhMnJLYqPEUc0k2BciG/q4xc0MktcveSO9k17XK26NH+aiQbiXuBelB0veuVOXm88hirA+cI8RLGtzM8K51kOGqpYbzfK9o4yOX9AakVRzA=",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = ""
    ))]
    Rabbit,
    #[strum(props(
        svg = "eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBUqKEjxQW9vYrWCRPbvYXY+pr3ubxMdOrPjBBvJI7mBLeVeSpESOThv+nZbnvt2tXjSCZYrikASJaisLM4jDMfUwIWKhRXiKSLqoKRvRwNxlBWNNYcQN38CcCy5maEfXAFyQIy/vPE0j+cjzZ0RQ+OjM1zu/XXz0yJ/0Rd28BllERKFzN+UteKez9tSxQ==",
        categories = "navigation,maps,security,communication",
        tags = "scan,sonar,detect,find,locate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radar,
    #[strum(props(
        svg = "eJx1kLFuwzAMRH+FyM6rSYm2DLhZOvcjCnfw2KFTvr53DpouDQTZoEg+3nH7+vg+7PP18h5pkcd0uW4vertuj8yCso7cHc1RjtWxeCjobw2rFfOJZgNhhAQLA4VuoXPUPrni4Ylh6Q2dcfCurqj4Xt7Rb//MThJyx2yThQvpIRnJ3iCxzs7wBcoqIzJS8iRwPuskT1Nm2mgWSCwUqsPpR92eeI4VQ6blmTJoivMJEIgQ+r7/B+9kqiLYMU5/v6tik+fDo/SpiJvgt5s2IzR3Y+1eKY1/in4ABr5YTg==",
        categories = "science",
        tags = "radioactive,nuclear,fallout,waste,atomic,physics,particle,element,molecule",
        contributors = "danielbayley,ericfennis"
    ))]
    Radiation,
    #[strum(props(
        svg = "eJxdjFsKgCAQRbcy3A2UQmGg7qBFRErjX8jQY/cpgURfl/s6dl+EKTjMA6nx0PC2q5G3rVDTr8lxFbocNCi/cjsY0JmCcPE9iGPaWGpaXnX/5RlSmvvGewCeUSU9",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[strum(props(
        svg = "eJxtjk0KwjAQha/yyL6jb0iaBJpuXHuIEoUKClJc6O1NIljBMmQW837yDffpMeOUzNFKBHvhgaCKgnAS0K4SzTjsqnMcvn7fVD+ViEVb+zpdOXtxayBflnw9I7+SiQZLMmqQn8lQq+ej/tQWBC29ISvKiPYFg2yl1m9gMAor4RQl9mircrA+K9xIRHFgmN2/dAvQ8nlHwoJr9g0UNkhu",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[strum(props(
        svg = "eJxVj7EOwjAMRH/llL0Gu7SkUtKlMx+BDBJIDKhigL/HTkVKh4ul+O7pnJ7n1w2XHE4HGsAD8cTgjgSMSBH+awpj2rlzTNV/tC33JNoItVU9MfZNpG5N6H3WxxX6zoElQD/LnHMQNy3rP7BDYXQ1Hn4qXGy4a8Bae8lJ2tLZhl3A5Rp7auILka05Ew==",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Radio,
    #[strum(props(
        svg = "eJxtirEJACAMwF4p7kUrVhCqH3iE4OAiOPg/0kUcnAJJZLU9oGdTGYgHBVPEqivylPQNkwJ4h4wMEaPyPgcuEReZ",
        categories = "transportation,maps",
        tags = "railway,train,track,line",
        contributors = "danielbayley"
    ))]
    RailSymbol,
    #[strum(props(
        svg = "eJxtzLEKgDAQA9BfCd1FL8h1OfsHfsSBg6OD/4+9IqVDScj0iD3+3riOdJKQ7LKhtmVhnVRsDVKsQw2n0KYEwqmKo+wEf7YP6gOKCxw+",
        categories = "weather",
        tags = "colors,colours,spectrum,light,prism,arc,clear,sunshine",
        contributors = "danielbayley"
    ))]
    Rainbow,
    #[strum(props(
        svg = "eJxtT7sOwjAM/BWL3Sa22ySVSucufEQVhg4MSCC+n3MGulRRHvZd7nzza/vs9Lhd7lpobIlVCrZzX2926gV5SyRVohylkq2qjV0mSlzIRXGWbwLJxMCpNMS6LPM1DJb5sMkA8bFBVlwyy0jKUkKYJQOCWhDQyKh7OxqOsQKennGb1CYGrkkJR8ajcv9lkQG3UqLIAwtn0KC+amkdTUewsxk9JOsGNDTARioeT5lktg6bkXUmPHnY1U6D07RL0j/0Az8IU9o=",
        categories = "animals",
        tags = "mouse,mice,gerbil,rodent,pet,pest,plague,disease",
        contributors = "henri42,jguddas,karsa-mistmere,danielbayley"
    ))]
    Rat,
    #[strum(props(
        svg = "eJxVjLsNwCAMRFexboF8ilSGZRIU0yJLCdtjI7mgsnX33nErt1JPOEHtn+erj0rCYa+U+opauoOsvJB5cyHz1Bbeme5M7IRsQ6ENvFEe7w==",
        categories = "connectivity,devices,design,photography",
        tags = "screens,rotate,rotation,aspect ratio,proportions,16:9,widescreen,4:3,responsive,mobile,desktop,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[strum(props(
        svg = "eJx1jKEOwCAMRH+lmW8GTQMIhp6ZRcyRTCAQE8u+f4dBsTTNy7W5F+/yVLq25VCSV0wTtiSEnTNLY4AR/3guKa5dm+KQW0ehsisCh4HNYLTqyP2ie5g1YfXZj88Hf6koeQ==",
        categories = "money,travel",
        tags = "bill,voucher,slip,check,counterfoil",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHg9IjIiIHk9IjYiIHJ4PSIyIiB3aWR0aD0iMjAiIGhlaWdodD0iMTIiPjwvcmVjdD6rpw94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHg9IjYiIGhlaWdodD0iMjAiIHJ4PSIyIiB3aWR0aD0iMTIiIHk9IjIiPjwvcmVjdD67PQ94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,9:16,vertical,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleVertical,
    #[strum(props(
        svg = "eJxtkEFqQzEMRK8isrdqWZItQRrorov2EIEuskihi96fjp3woclHIBsJvRnp+HP+vdDX6+FzkOS7cYifhUNppYqQIuyjcISgOMLv+dbjWq3Mgn0MluyU7IfT8WVyT8eNLpjNS3Cr+oAHhN07+LnAds+3Xp3s4Vc8rfXSWNoz/Vsw1IvSih116CYklR0GN5ukDFcVxdhhJqsZOQ4y166psBDZ3h6OQyKcuMZc67/5OebWcbe4Kqcp4W+6Z19ZR8BSVyWDVl+KsfIoq7KN/QFn4mBy",
        categories = "sustainability",
        tags = "sustainability,salvage,arrows",
        contributors = "karsa-mistmere"
    ))]
    Recycle,
    #[strum(props(
        svg = "eJxdjLEKgDAQQ3/l6O55V5uhcBbcuvgRgoOL4CB+v21BhxKy5CWxa7sP2md3KkgDYWhyycZKkn189UIxR8YCBlVLUygrxiNdXIrkJev0P71daBiS",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Redo2,
    #[strum(props(
        svg = "eJxNi1sKgCAQRbdy8b+HCg6CuoMWIRYY+BESUbtvJIoYmDNzD9eltaayoHohBdLJUMyLSSK44dHBbXHPmL2YlAQdJnem2Zb+nIakaGEx8kjYrt3vZ6B6XTRv+qo3fI8gPw==",
        categories = "text,arrows",
        tags = "redo,history,step,over,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RedoDot,
    #[strum(props(
        svg = "eJw9izsKwCAQRK8y2JtkXdhlYeMNcgghhUWKFCHnVwtlYJgPz9/yVdxnuBJBf6lRQvZ9rNnXxyAtBsPRRbA48myCtPHD3XWhDdu4FTA=",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Redo,
    #[strum(props(
        svg = "eJxtjEEKhDAQBL/S5J7sZgbHFWJg7/oIiUIEDyIi+nujgl5kDgXTRbmxmSPaUtUMWiSK8u5z/Ly7F7Kw9C9Q4HueIDNcMX7vLtGiJeq3EqdQ84RsBjH5wJpM/tihn8LQYSqVVQhrAiVuJ5N0zX4Hjvgwag==",
        categories = "arrows,development,shapes",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = ""
    ))]
    RefreshCcwDot,
    #[strum(props(
        svg = "eJxtjlEKgCAQRK8y+K+lpiaYJ6hDCH342Ud4/tyEipCFYZhl30440pmxL2xTElIlD4+RhntevXCmScuscBNUlVVjZjEMdB3Dw9DQxWTT3XzhIPeDg+D8hlMV24FIW/NsyvvgArClLag=",
        categories = "arrows",
        tags = "arrows,rotate,reload,rerun,synchronise,synchronize,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCcw,
    #[strum(props(
        svg = "eJxtTzEOwjAM/IrVPaa2EyeVSiXUhQEegcrQBYkB8X4uGboQJTrZuvPZN78fn52e5+GuQuUmhXOkBLhMnBM1GNsTJVtFyEhGHo2MxcFLrWIelvlUrZb5MCwkfrVv6lDw0NUwnYQiZVbHTo8N/uUvyJ20qir0LlPZlOOEJuL2IORsHrRrV5OKbpgLNYLwlAMSwLvkvhoh9tDLodiMT3pwP/YfT40=",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle,cancel,no,stop,error,disconnect,ignore",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCwOff,
    #[strum(props(
        svg = "eJxtjs0KgCAQhF9l8b6Wmn9gnjvUQwgdPHaInr81QUJkYVhm2Pk2XOnOcK7sUCBk8uBhphHgkXZudZXqGW4XkCS7FOBYDFO5jqF1kK0enVGPsz+g1PcALAD8APSNGZQ4sjdCtOgFyMktuA==",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCw,
    #[strum(props(
        svg = "eJxtjLEKgCAYhF/lcJf85dTFnFtaG9qEBseG8PkzgjCQW+7ugy+e+So4ZrU6+EwQpkVAzfLbYBVmC/seurUl9Fvbze8qxekxpth5xRThgIhDqP4DN7Z8Ie8=",
        categories = "food-beverage,home",
        tags = "frigerator,fridge,freezer,cooler,icebox,chiller,cold storage",
        contributors = "karsa-mistmere"
    ))]
    Refrigerator,
    #[strum(props(
        svg = "eJx1jLsKhTAQRH9lsN/c7FySKERrGz9CsLARLCSFX6+CT1C2WM7MYeLYTj26Mms04J/UZlX8bVkVj2ZQGh/gjENuvIf7VNTujrw4TQENLUHY7YTC2t0ZTLx4/ewfLEzC+RxeAEsvLSk=",
        categories = "text,development",
        tags = "search,text,code",
        contributors = "mittalyashu,ericfennis"
    ))]
    Regex,
    #[strum(props(
        svg = "eJxtyTEKgDAQRNGrDOnFbLKJFmtuYGsvWGwjWIjnd20kkDBT/SfXfiuOxa2MaWOl/ERXZPxykR8TgtfcAYpgzKatnZRgt3UseLOhthd8SCX1",
        categories = "text",
        tags = "text,font,typography,format,x,remove,delete,times,clear",
        contributors = "ericfennis"
    ))]
    RemoveFormatting,
    #[strum(props(
        svg = "eJxtjDEKwCAQBL+y2B9xVUhjrNPkEUIKGyFF8P05AyEWsmw1w8Qr3wXnZipXOAQE0ZsUlw5S/PDhQTZhVgqrYzcLJ2rVkJO3I7OSI+jbUOrm7icm1bSF7a88Zf4tOA==",
        categories = "multimedia",
        tags = "replay",
        contributors = "ericfennis"
    ))]
    Repeat1,
    #[strum(props(
        svg = "eJxtjDEKgDAUQ68S3Iv9KVaF2tnF1b3g0KXgIJ7f30EdlDcEkvDCno6MbWoKMcIZB6WJoa19DPe6iIMMc58IwipiaLj677OQkK56jPJnEvjsX5EmT7HP8wKymSTx",
        categories = "arrows,social,multimedia",
        tags = "arrows,retweet,repost,share,repeat,loop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Repeat2,
    #[strum(props(
        svg = "eJxtjDEKgDAQBL+ypD90k4BNTG3jIwIWaQIWkvd7JygWYdlqhklnuSqO1TUu8IiIonc5TQZyevEeQHZhUYpZRzMrB2rTkJenI6OSJxj6r2TmFj7zBt9bJUM=",
        categories = "arrows,multimedia",
        tags = "loop,arrows",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Repeat,
    #[strum(props(
        svg = "eJyVkN0KwjAMhV8l5D6zyapusO4NvPVeumF3Icgo/ry9yQQVKYgU2oZ8OSdJdz7kBEPAHXvw0RFXXLUkICTYdytL990LEgcSFQF9oWr1KlICTXSgHBlDZYo3wC7SokZC5lryPNWwhdoO1QUVE9mvl8a36qiQgYnf6DzGDLeAgnAPyB5hfkbXacgpYIOQxumYsn21yvjPNj2w/5r54n+Pp6v6o+4BpGphrw==",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[strum(props(
        svg = "eJxtzkEKwjAQBdCrfGY/NTOttoWmN3DrXtJiuhCkBNTbm4mgmxCYBPLyf6bHNUUsns7SoQuOpZFmZIWy0jwd7HqefkgdNGSCvKMZ86gqxRAcsmMzXFdygrjAJY2VrbXWeW/Ro7XFbSXFQi7H8vE+N2ZkMMqf7mtIeG5Lip4GQly3W0zluL88KeE7356ks1fm5w+eUUe4",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Replace,
    #[strum(props(
        svg = "eJxlyrEKgCAURuFX+XGXUoTboM4tPYRQkCAqJEFv35VoKc54PltLulLMG2qJuR1OEBRBQ2kQSHg7vMLbn2XEmDru/qND27E6sWi+0yl1MDAYe9JIMz+ckb8Bghsl/w==",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    ReplyAll,
    #[strum(props(
        svg = "eJw1ykEKgCAQBdCrfNxLKgMmqOs2HUIoSBAdSIJunwXxts9zK3fJdQe3XPsZhIO2IGgDByuin/4RPad+YAtiNQp6vqRJNKZ6SZK00NdHig9v0Rgo",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Reply,
    #[strum(props(
        svg = "eJxdy0sKwCAMBNCrDF6gTIqLQup1SqEYoW68vR9ciJBFZpinyb7yWESyN+b/diR4QcB2hMfILugxh0F3IdIFz07a7zGKlVTOiRxj",
        categories = "arrows,multimedia",
        tags = "music",
        contributors = "colebemis"
    ))]
    Rewind,
    #[strum(props(
        svg = "eJxtUDsOwjAMvYrF7kfsxE0qFSQ2Fg5RlYEFCamIgdNjt8BUJbb8e362h8f4vNH1sLsUGEkHm1jCgnasZCFzRi3sQfcmVGG0gsoKyYzUu9HL6F6jRaV4SzCy791x2AfJcfhR3UVJjDPnUZX8B0JIOaO3kyhao1V/E140JW9elVEbVRh3JM6pyEarXkq5kG4QXnrvdy4zzFmRslOVSdCpr5oaBdhlA7cM+ooDJF/VjIpfIDAcaN/SQv7AD1TxScQ=",
        categories = "gaming,development",
        tags = "release,boost,launch,space,version",
        contributors = "ericfennis"
    ))]
    Rocket,
    #[strum(props(
        svg = "eJxNjm8KgCAMxa8ydoDSRf9AvUGHCAoKooT8oLdvDgzZ2Mbe+8Ez/rnSdd47+Oe8w2uxa3ogGHhqymOSjc60xeqMAFFbnFmBRBZJIUTevTxYKVAFFN8vC6EnufKvAvwaDtgsLtSMOcSqO+BWUhkBJZHY5j7/2jG9",
        categories = "furniture",
        tags = "chair,furniture,seat",
        contributors = "connium,ericfennis"
    ))]
    RockingChair,
    #[strum(props(
        svg = "eJxtz7sOhTAIBuBX+ePenl6Qo0n1DVzdjQ6OJhqfXzBGF9KhhA8ClG06VixdNTBiO9ZVX36a6ssLMaiwbywjsdP9bWxQn2SDdrFBKemw1pIbJgIhyIsgR3OSSFbwOYNBu6L8T1FEcNmx5++qC7M8Pi8=",
        categories = "maps",
        tags = "attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    RollerCoaster,
    #[strum(props(
        svg = "eJxlTzEOwzAI/ArqDjXGGCylWTrnEVU6dKlUKf2/ComqDl04g++4Y3rd3g+4X04Ld2q9g5FeWak3gUZVDFhoaIUKnGXEbAQaBN0skOtW9yEX0CgrSatQgLoZUunxgVRP83ROq3n6Gj7DhUfL/VYMhJyjIe+Y5ej/VQvHPiW1NWmlkbtiBLXUNkE7IBhVoCAX3MPlQ7eWkTFDoq4hFY+cTm4SKmOP6xx+ph8MLT08",
        categories = "design",
        tags = "gizmo,transform,orientation,orbit",
        contributors = "lscheibel"
    ))]
    Rotate3D,
    #[strum(props(
        svg = "eJxNjEsKgDAQQ68Sum/tx7EOjD2BHqLgoksX4vmdIkgJeQRCIle9G87NHAkhVgbDI6jZanaZPvguu7g8Iyr2hNUUmfq4yHCRHmr0Ny/fYhZa",
        categories = "arrows,design,photography",
        tags = "arrow,left,counter-clockwise,restart,reload,rerun,refresh,backup,undo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCcw,
    #[strum(props(
        svg = "eJxNTDsOgCAMvUrD3iqFqk0qJ9BDEB0YHYznty6G5eX97ap3g3MNO0eIXBUURnCKinowCbvMpMm9ieYM7LB5dwnFhm9crL9IjzSUP3sBG9YW+w==",
        categories = "arrows,design,photography",
        tags = "arrow,right,clockwise,refresh,reload,rerun,redo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCw,
    #[strum(props(
        svg = "eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PQR06RB4s+Xt+YVtuBa91LjlSJ4T3bxE+kdQIeVnvuUTyhO0PUzjXhyk8UsmYI10di0L9xVWr14Op7KjS0gEqT2vJyN4wso7JYJA6p4GdQ6unkyrGvSUPv6dVuddD/Auc0jxR",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[strum(props(
        svg = "eJwdjEEOgDAIBL9C+IChXjy0/Yw2QmI8NCTC76VcWDY7u3WOU8Eb7giWd4YpIZbyyaXckA4EHnKz5t/rtnq9PvIOsBIoIThFGB2jHPKSNthF9R8Gehw8",
        categories = "layout,design,text",
        tags = "split,lines,queue,series,list,vertical,horizontal",
        contributors = "danielbayley"
    ))]
    Rows,
    #[strum(props(
        svg = "eJxNjFEKgDAMQ68SegEtTGGw9QYeolRBwQ8ZfujtrduH0kIS8kg69FwxZ5oCmDUiovdj10iSureW9IOC8gj/RlX/cbYV2xeUTEywK9PgcnuqW62VB4SMHaQ=",
        categories = "development,social",
        tags = "feed,subscribe",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Rss,
    #[strum(props(
        svg = "eJx1jk0KgEAIRq8i7cfS+SuYOkFdoF3QokVBi+5PGrUoGkQRH0++tE/HAnNbDExogTzaidGBdiVF0hbdahiDHMMbGqtbzxihxqiM4Bq3qnwVy9zmi17uWHSp1AxdepJs5NADsQw2/MdJUJPFtZCQl6M+9x9+AsHHPTA=",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Ruler,
    #[strum(props(
        svg = "eJxNyrEKABAQgOFXueziCp06ZouHUIYbDfL8koH+8f94tCnQk6oBEIWaAwf2pqnEhaQym6My/9bLGxttFxJB",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    RussianRuble,
    #[strum(props(
        svg = "eJxti7EKgDAQQ38ldBeb85AOZ2cXf8Ct4HCjg/j9tg6KIFlCXp7t5XBsU1hEwDRLUSjindqcn6HTNWTrm5PtNQkqGCEYanOmv1fjJ8eHXAAuHlc=",
        categories = "transportation,travel",
        tags = "ship,boat,harbor,harbour,dock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Sailboat,
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/l6G7VkpyqgjTQrUt/oFtohyyFDv1/KscQGhKMD3xPPk79Z/xOeF0Od4PwxPkw9MfqDf1CWAKNDkeejye/6f8b/tj7xqRnsIxCBfXWWU5UUiEzrFwoSZI9u1iik25stSbrbKbKqttxkzZPWRFsW/LNAQRRaW+BTA4j6a5K7pilbVzA+ZlBFrUjuDB1UWOJ+AFQYUoE",
        categories = "food-beverage,emoji",
        tags = "food,vegetarian,dish,restaurant,course,meal,side,vegetables,health",
        contributors = "kemie"
    ))]
    Salad,
    #[strum(props(
        svg = "eJxtjr0KwzAMhF9FZLdqyT9KwM2coX2BboYOHlroUPz8lUuIPQQNEsfdp0uf/C3wvE53B0TVZQIC+x+9CsVRMFSNm9Z0aaE1HVFioGXzh1WNzcpdaNIIa5sqjwn9ZhzOJ/x3qwaCIiai9ZmBdwijj2BvrGHa3OO0Gi6i7TQMukNRIbz0kQTwGDrLMMqMLB3yA6WWQD4=",
        categories = "food-beverage",
        tags = "food,snack,dish,restaurant,lunch,meal",
        contributors = "kemie"
    ))]
    Sandwich,
    #[strum(props(
        svg = "eJxtjTEKgDAQBL+ypI9mjV4QYsAH5AN2AQsbwcL/o4kEC+WObXaY9Uc6N6yTij1okmssUcKUY/5FBd9mLvhK7yM4wGr7rSIdaJNAHocWLT9Ux5uai79uEa/uAj2lJUE=",
        categories = "connectivity,devices,multimedia",
        tags = "antenna,receiver,dish aerial,saucer",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SatelliteDish,
    #[strum(props(
        svg = "eJxtzEEKgCAQheGrPNxLjdqoYN7AC7QTWrQoaNH9yQlqkwyz+j9eOuu1YZ1VIQuPCIsJfndwKqdBYk4vOciDCK3p53XPBJARAtYsZOntMAKstv9UIgxVBmOUaxP8oRubdSuF",
        categories = "connectivity,science",
        tags = "space station,orbit,transmitter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Satellite,
    #[strum(props(
        svg = "eJxtjS0LgEAQRP/KYF+9HRbdcJotVoNNMFwwGOR+v1r8AJk2D96L27wnLG0x1LCZIMI5BYVJw2qwrKHkQ4TQ0nt/P8Kp6GJ1qbp4CzWA2VL9hxzqWZoknpt/Tvb2bYyP6QC/jS1m",
        categories = "text,files",
        tags = "floppy disks,copy",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    SaveAll,
    #[strum(props(
        svg = "eJxtzEELQEAUBOC/Mu1dzNOGWs4uru6Koja2SPHrLTms0qupqe+Ncd02oi9VwwLCWncCQeKPkUTSBh2+j6TV0DsZOsipKhPfU5Vxiz3sNA9wyzRva6mY+WH4ZIo3hI9/5c9PhpvloEb+oRcebS6u",
        categories = "text,files",
        tags = "floppy disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Save,
    #[strum(props(
        svg = "eJxNzEsKgDAMBNCrhOxFUkil0PYGHkKiUEFBiojeXm39dBXImxkrY5RpADkckkGQPd/oUKG3dWZv39jF/GjqcBlaujVA77BlaDZSgdLE/f1tZiADutIfna2WJS8=",
        categories = "design",
        tags = "gizmo,transform,size",
        contributors = "lscheibel,ericfennis,jguddas"
    ))]
    Scale3D,
    #[strum(props(
        svg = "eJyljbEKg0AQRH9lsN+Ns8t5F7j4B/mBdMEU1wQExe93bazshGGaYd6r83dt+L26PwdEXAocZRItWYck1KeB4uAipnRRT+LCTzfWx/Ed60mwm4B3hrGxv1ho8I3lYnHkZpOhR7iQxRA6pOigNTsfO3vQOMw=",
        categories = "maps",
        tags = "balance,legal,license,right,rule,law",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Scale,
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+yoSfmODFcclLT+AgTiystzL1fbaCh3dkZvc/HcO3hSASGgHKouvxr1c4ogRs7FaPiUWaPDWzZp/L6RZt4HPAFjH8g1Q==",
        categories = "design",
        tags = "scale,resize,design",
        contributors = "karsa-mistmere"
    ))]
    Scaling,
    #[strum(props(
        svg = "eJxtz7EKg0AQBNBfGezXuKPHRbhYp0mbXrDYUlDu+z0t5JBlGViY10xa592wfJpfj/gPM0F05RQUGpspvU4xpdtpRG+sIZg9SIXGXEkpn4lHI6jfUEthduUbOmzahiKHM3LFgSNGazv1BoRHdQBC+EI5",
        categories = "devices,social",
        tags = "face,biometric,authentication,2fa,dashed",
        contributors = "karsa-mistmere"
    ))]
    ScanFace,
    #[strum(props(
        svg = "eJxtyjEKgDAUA9CrhO5Ff0T+Uju7uLoLChVEHKTo7W0RpINkSALPHdMZMHdmaKBjOxFEnSKgZaDxrsrCu8+JogksIRj/IAWisZA2rWD/qILSt6W0jKXc1n3Bzc4IDW55+8pfU6ev2WblH6yxNG8=",
        categories = "devices,social",
        tags = "qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ScanLine,
    #[strum(props(
        svg = "eJxtyiEOgDAQRNGrTOob2GmaNaUag8U3QaxEkJ6fYsiK5psvXrnbY7i2cCTomRtBrCMBI42hluUTtfxOFMnoIdhnkALR7mQcZ3FGFZQ9exnZnXwBlDYpaw==",
        categories = "devices,social",
        tags = "qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    Scan,
    #[strum(props(
        svg = "eJx9zl0KgCAMB/CrjB2gGGX5oN6gQ4QFBj2ERNTtY1oiEb792X77UHbxdp3Ba6wEgj019iFcMRhVR2HUI7kjmGQjJH9pLujdGlLRpgeoK1IWbTrw+XYbdweTxqGB5iDpSHKXq+YGPWdDbQ==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRh7P9dKeL6bak+QrL4NhBepqI6cuU3Bi3XzzG+/TIcBncUQSETn1GP2NAjwH8QZRlsj/QZ2zLHOoHyrmElZAqKgDvdUO8jnYNxH3bVljT50FEsfv3QuulVN8g2v2sb/KNSRc=",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[strum(props(
        svg = "eJxtjsEKwkAMRH9lyH1xE2KNsLtnL35EWQWFClJk0b83VtCiJZdM3syQdO1vJxwyXRQdLCgMSiWtXveSPpQNHKGQZr1AEH04+LbTuQ7Sgg0a5L9iz552/PXHyR/n2vt1KWpYN94skO4H1PNYhyPqI9OWMGYSQr1n4umjNy1PN58++Q==",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[strum(props(
        svg = "eJxtjkEKgzAURK8y/L1p/yeGFJLcwEOUtKCgIOJCb2+iiKJuZvNmHuP671jj56kqlRZ8cliwUHCvjIKLzRDbP+LsyRIGT0KIkyedKxsMbrd0rGEKq0qLHHfLOtwtScnmSVPxW6UXeg3wyXM0THpZF/KARC5oAdW9Phs=",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[strum(props(
        svg = "eJx1TcEKwjAM/ZWQe+sSaonQ7uzFq/dRhQoKMkT0781anAU3Gl6TvLz3wn14ZDhFPDhgHhgYOn1k2PDR/WbQOZNvF8DPdmG0wz5sJsM+zLbUqe9elhivTDaNKF3GdD1DekUUhPQu3xixnFSy0e+sIwcVibWWMpwVqDDVf5RmkMdv5FrWTdXeiN2Kh4Kz0QeNlFHB",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[strum(props(
        svg = "eJxtTVsKwyAQvMrif20NViwYb9BDFCNVaKGIUHP77LqQBwSWGZjHjisxVPjnqaZRDDcBM5KAFPM7VVZK61JH765U8C7kEj4RSrcClixSI8IIm979XjXBNIrnQ2qlgVENeJQid5dRWlpgoNsSx6nGU7iozNnWF9vmYuXdGui4PloA0Lc/pQ==",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8ScoDa1ipZtL2Bh5AoKLiQ4kJvb1v/IISBMI95lsfAUw+8OqwRgsMSgbf0eFscpbdzuwzQOWxIKA058iUmdS9CSzAnUQmih7hEcVsRfoR/JmUEQY64qOW9swOpHi86",
        categories = "text,design,tools",
        tags = "cut,snip,chop,stationery,crafts",
        contributors = "colebemis,ericfennis"
    ))]
    Scissors,
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+ysSdydyAUSG3jI0gsaEwsDO8XGiWRbLGbnWTCle6MY512EshmEoOhW1RdhfR31OZMS38oLkqmGOZmieF1eTBlPwDEIFfMn5zMEGVhB4gcBD16ALRLLkM=",
        categories = "connectivity,devices,communication",
        tags = "desktop,disconnect,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShareOff,
    #[strum(props(
        svg = "eJxtjDEOgCAUQ6/SuBP5HxAGZHbxECQOLCYO5p9fWMRE0qFNX9p45bvgWKedDMxmM4Ohm1RNQroX1bnQ8i0UizJTinN7SfH9CmAqYQCIQV7sn5zkEeCUG408THHS0QO6wy7K",
        categories = "connectivity,devices,communication",
        tags = "host,desktop,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShare,
    #[strum(props(
        svg = "eJxtjLEOgCAMRH+lYW9sK0RMkJnF1Z3EoaOD4fsFE4kDuenuXl648q1wbmb3IKwsWUCA3ghKQUlM5VsZGC3Q4Xqn1sus1sQwNVcM3cgr8NLZGqzGNCQdeEU3flj+1wOvSiwV",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "danielbayley"
    ))]
    ScrollText,
    #[strum(props(
        svg = "eJxFjDEKgDAQBL9ypD+8WyMqxNRpbO0DFiktJO83ETzZapZhwpXvQufm9oWgRZFBIHkHRmUklfq9Ssqe5JiMpXMdi3cxDL0VgxV1JZ3NbeNWTL/5AA4dHTw=",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Scroll,
    #[strum(props(
        svg = "eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf9+GQlmWPcwy/KzvAZvgnYEIYksKCZVnB8p2Frt2sE+QCMH+vkUw+6lj5WGJzUEhTYt3aCqJmxtr",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick",
        contributors = ""
    ))]
    SearchCheck,
    #[strum(props(
        svg = "eJxNjFEKgCAQRK8y7L/Frn0kqHcJCwoKQvqo26cGJrPLwLxh7DldK2ZHh4FRgiLyts+5t5WyRjpRRT8PWwz7guhoJITbEXPyp3gqfbiZEYawGjqdv868RmkjhQ==",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = ""
    ))]
    SearchCode,
    #[strum(props(
        svg = "eJw9jEEKgDAMBL8Scm8lrYEe0vxFoqCgIMWD/t5WoYdlDrOMnNO1wpzxoOgZkmfHwKgyNKFiW7F9gZIxIdiTkajy/lhPv1bplUAQyI0+tvXMC5TzG5E=",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/",
        contributors = ""
    ))]
    SearchSlash,
    #[strum(props(
        svg = "eJxVzEEKgCAUBNCrDH+v8TXBhXqX+AUFBSEt6vZlgtRiFsMbJuzDMWOMtLHVDl475eAoha5ACo19VfxUlizrBLkiMRNyJE+Q823PqPLnxDAMq17bknZzAztoI4A=",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort",
        contributors = ""
    ))]
    SearchX,
    #[strum(props(
        svg = "eJwlijEKgDAMAL8SsqukOjgk+YtEQUFBSof2923a4bjhju2J9l4QBXcEK4JEzblbeRlZ+T/SDafgFwgCTdu8On540QrMzBNp",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass",
        contributors = "colebemis,ericfennis"
    ))]
    Search,
    #[strum(props(
        svg = "eJw9irEJACAMwF4p7iJVEAq1H/iAm+DQRXDwf7SDEjIlvPpWGMXNBAb5KyB5ak44WBV+T82AUTH/cgD7URB8",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "danielbayley"
    ))]
    SendHorizontal,
    #[strum(props(
        svg = "eJxVjMEJgDAMRVf5ZAFNKeihdQOHEBXjTaRU3d62iq3kEl7ei9nn0UHmdRFnqSWcllgTjnVyksAeiCJciXemikFnUvae/vXjquLFF22DE0yW+gasPQ8KCnWasAlHMSqFyBqNZJGj6LN4A8YNMyw=",
        categories = "design,layout",
        tags = "bring,send,move,under,back,backwards,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    SendToBack,
    #[strum(props(
        svg = "eJw9yrEJACEQBdFWPubL3e4Jh7DagQ2YCQYmgoH9owbKhPO051FRvGkiEPohL1lyK5tM0GfvoAfFjcAM/u6cOUoQ/w==",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "colebemis,ericfennis"
    ))]
    Send,
    #[strum(props(
        svg = "eJxty1EKgCAQBNCrDHuB2i3ED/U2fQiiQn3k7duN+giCgWHgTSi5bhgciYVwai+EIc/UFqYUJlMp9FbG7XvL9dgjeXiwYAU7eHOv+LFKLAKZ7eU+/AL4HCaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[strum(props(
        svg = "eJxtyk0KgCAQBeCrPOYCNVOIC/U2LQRRoRZ6+0ahReDq/fC5FPOFxp5YCF08CWvqPghN5h3cNlRwtaQ+fS0xP7cnC4sTLBpshvvEwrJRA9kH12J//AX34CaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[strum(props(
        svg = "eJx1kEELwjAMhf9K6D2xabu5wbazF6/eRxUqKMiQof/exHnopKOn9L33vaZdvE7xdoH46g07A1NvvIH4/k5Dt1vkoXuMzwTn3hwDVcD2EEYHDqwcRofulM0gc+I6vwA3rwLgElKlfOX+00NGt7gOW4XldKt1M64SuIWvoU5kuahwU5buXNEe2FNAapF8wdGSkz9RdcMgohQIh7x4Sgb/M6DUcJnQQKvxst6QhwUijlI+6BIK2TLIgtBogc9e+AF/Zocs",
        categories = "development,devices",
        tags = "cloud,storage,computing,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    ServerCog,
    #[strum(props(
        svg = "eJx1jbsKgDAMRX8luEeTWIJDdXZxdS84dFBwkH6/7SIpVDKEPM65/g5PhGPuNgWm1QUBAcrFKCi7mSHPkdUuQFIFgESUbvFDkS7eqp1RE9YkFZNVU8lKWBH469bYE7dTp/bp4hEUXSb1LO37eAFErUAW",
        categories = "development,devices",
        tags = "cloud,storage,problem,error",
        contributors = "mittalyashu,ericfennis"
    ))]
    ServerCrash,
    #[strum(props(
        svg = "eJxtT70KAjEMfpVwe2oTm16HeouLiw9x4JBBwUE6+PQmHlwVSijNz/eT1Of6UridpusMrHRcGRiiBdnPLfUaLVOUaakH5yx1Z5JNI3AQf2cn8FaBtC4Yvav5PRBgBpobUscio5nRAJuA0i/ub8nNg3KQO2EQDyxYLmnkmoGKhjgweXwvMLG4zz5i60de",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    ServerOff,
    #[strum(props(
        svg = "eJxdjVsKgCAQRbcis4BSiehD3UxJCtGHCOnumzHt9eMZnHPvqGDnyJz1q4saJmAhaZCIXHC9h1+iw4kDK1ujeooZ9Q+/xNqTWo0YWumd3vxuWRYaRjRkQSJ0XOBU/tElq7rPlnSBBylN/Nonjsw4sg==",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[strum(props(
        svg = "eJxVyjsKgEAMRdGthPSi8UMQkqltXIREYQQLGSx0934GHSzfu0fWYfMwKvZlAeyzFp3k9+fkK1QDcdekYnOwZQLbFYkRgmKFYMezLhTzj3HsL07oBCCgJFo=",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[strum(props(
        svg = "eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGemhJ0v9L/ra7Hu5nOPZux0IiIGekEA56gCYu1NODePOLMDIwrf2kdZ6kLRK6a5RbpGZTKmh1viBCvsiARiFmJ71EvChBQii3ZcsNiUDNtgAsMhYwo21THPfJDpCxfi9NKaGWVI6AoDywsiQ2lzDBY62YMLFf2xV++0XyZpTk17+PioiQoOjWksN3wG3t7ayUKPZFcKbUkPyGdo7M9vvqY6C83dCt4gcauvFyG6cTjM/esTgYX3m/9c7HopwePjs3fzs=",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[strum(props(
        svg = "eJxVjTEKwzAMRa8itEe14iROwPYJ0gt0C46pDR2KMTS9fWXaoUGDkP57fPvcaoLd4XUmDaw2MmRAyXBHUz91TMosKzMNoP9CYOJl7kgNeuVJ1JmWkzuavjE39PbSOrwtMVR45b0mhwbhcKgR3g55QChyMUKK+Z5qi8VqvLchl/CIEBpoaBRUvLbD8fsI+oX8BwLQMj0=",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = ""
    ))]
    Shapes,
    #[strum(props(
        svg = "eJxljE0KgCAQRq8yfAewprIf0C4jLYJo0Upvn6MWgqsZ+N57xp2Puw56LEaQ8xa8xhssNHbT5XU3hZJ9zjMPyFJLtS3eauw674M8W6xKb6AQPx6VZpAf4qvVFNtB3kVNyRSj8gThgvSCpMb8J1L3016ScDt3",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yzD5qhhIX6g06RFCQICokkrdPrUXM4g2f97+KWzph17jOQJzlxsAw9avfSfwPBs6DRKPGVjIqBlec9QfEYH26NJIAUVdqQYLo3mcY1b1CGhnhrqBGflkal+Z39wHumCZQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[strum(props(
        svg = "eJxtjEEKgDAMBL8S8gFJRVBo/YwWWxAPpWD9vUmsKOIlm4TZsclPGVJxaBDSocGzRSg69zjn4JB6hODjErLuo22kN9o1bh4OcjhwzWgUDkOcJAImhank9VOSugcVAd8f9tK95Ld1+LGKrqKl2mvlZk9pMTyd",
        categories = "text,files",
        tags = "spreadsheets,table,excel",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Sheet,
    #[strum(props(
        svg = "eJwtTkkOgCAM/ErDHXRKo5ggP/ARJB48evD/sS2kzXSbybS+/XvoPsMFIaAzMa0EQhStQoY2F8VNw6cIIxVfOtn28Bxa9gbpyBPHgZNw3BObybShnFStBwmtLvZM+wELIxuz",
        categories = "animals,development,nature,science,travel,food-beverage,home",
        tags = "beach,sand,holiday,sealife,fossil,ammonite,biology,ocean,terminal,command line,session,bash,zsh,roll,wrap,chewing gum,bubble gum,sweet,sugar,hosepipe,carpet,string,spiral,spinner,hypnotise,hypnosis",
        contributors = "danielbayley"
    ))]
    Shell,
    #[strum(props(
        svg = "eJxtyyEOgDAMRuGr/JkvtNsYFWU3wOIJiAkECWTnJyAIAvPMy2f7fBasgxvFw/tDKUJJeOo2UgqkCLVfGAkK4ScuW3urbF+rNf4PSaVhed8FUzEdSQ==",
        categories = "account,security,development,notifications,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,warning,emergency,attention,urgent,alarm,crest,bravery,strength,tough,attacked,damaged,injured,hit,expired,disabled,inactive,error,exclamation mark,!",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldAlert,
    #[strum(props(
        svg = "eJw9yzEKgDAMRuGr/HQPNmmqGWJv4OouOjgoCIrnFwW7vOXx+TFdK5Y+DCwQOY0URhzHvJFRIkO6uzmihYHjl1C8eVXx3+6KDFaw1PcAweEV5Q==",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,cancel,error,crest,bravery,attacked,damaged,injured,hit,expired,eliminated,disabled,inactive,/",
        contributors = "danielbayley"
    ))]
    ShieldBan,
    #[strum(props(
        svg = "eJw9yzEKgDAQRNGrDOkXk82qK6y5ga29aGGhICieXyMYBqb5PDuma8XSuyEwmE8lgVLwY72RUiRFvNvZo4Ei+O9csiqrZL/dO2T9TkhKfgABKBZp",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secured,safety,protection,protected,guardian,guarded,armored,armoured,defense,defence,defended,blocked,threat,prevention,prevented,antivirus,vigilance,vigilant,active,activated,enabled,detection,scanned,found,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audited,admin,verification,verified,certification,certified,tested,passed,qualified,cleared,cleaned,disinfected,uninfected,task,completed,todo,done,ticked,checked,crest,bravery",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldCheck,
    #[strum(props(
        svg = "eJx1jCEOgDAQBL+yqT+4a6GcKP0BFk9AVCBIILyfgkAQalbsZCZs05Gw9GYQC2t3pQZKwmO7kpIjhTu7meGhEH7GxFDfVgyvm29JFcsPytky8192ATAQJT8=",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,securing,protecting,guarding,armoring,armouring,defending,blocking,preventing,antivirus,detecting,scanning,finding,auditing,admin,verifying,crest,upgrading,loader,loading,throbber,progress,dots,more,etc,...,…",
        contributors = "danielbayley"
    ))]
    ShieldEllipsis,
    #[strum(props(
        svg = "eJxVy6sOgDAMRuFX+TPf0HZcKsreADtPQCAQJBCenzCxZOaYk8+v9Tmwz2ERhept1MNIOA8nGUUyxHfaGCMMwiUhefer5K3NWs8HmPIVow==",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,logo,sigil,flag,team,faction,fraternity,university,college,academy,school,education,uniform,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,ranking,army,cadet,scout",
        contributors = "danielbayley"
    ))]
    ShieldHalf,
    #[strum(props(
        svg = "eJw9iysOgDAQBa/yUr+hu+XzxNIbYPEERAWCBML5+YiaETMZP5arYBvDpAazk9KConHudqEkIdI9rBE9CI0/Qvbmu7LX99VaWMMDhacVjg==",
        categories = "account,security,development,gaming",
        tags = "unshield,cybersecurity,unsecure,unguard,unblock,antivirus,clean,clear,disinfect,patch,fix,stop,cancel,remove,relax,admin,crest,bravery,weakened,damaged,hit,unarm,disable,deactivate,decommission,downgraded,minimum,-",
        contributors = "danielbayley"
    ))]
    ShieldMinus,
    #[strum(props(
        svg = "eJxlTbsOgzAQ+xWL/a6XuyQEKeUPWLtH7dChlRgQ308IggX5sVi281yWLz7PbnID93C+RB6wSxrYSF/hR4mMjBWOtRvzY2+N+ez+FRVSec8mX2ebENb+LYhIcNKsqLCh2fEWOCp5tnTtbO3nJUE=",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,interception,threat,prevention,unprevented,antivirus,detection,undetected,exploit,vulnerability,vulnerable,weakness,infected,infection,comprimised,data leak,unaudited,admin,verification,unverified,inactive,cancelled,error,crest,bravery,damaged,injured,hit,expired,eliminated",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShieldOff,
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+yob/IgcgVJz+wpTdaUFiYaHy/aEFFs8VOZvRc74J9Ngs7OHcJjRBim8NBQp4E/ombxQQB239M0uGzkja33lykA2qUQ46NvBjBHNM=",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,extra,added,professional,enterprise,full,maximum,upgraded,ultra,activate,enable,audit,admin,verification,crest,medic,+",
        contributors = "danielbayley"
    ))]
    ShieldPlus,
    #[strum(props(
        svg = "eJxtjb0KgDAQg18luF+9u/pzQvUNXN1Fhw4OguLz2zp0kpAMCR8J53pH7GM1i0L1MmpgJLy0Bxl5Mvin3xgdDMJfVFOoMzWFwg5OMKweHpwkaJ0pJGFKqcz+gdKh9NGxlO0FX0MiNw==",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,undetected,scan,find,exploit,vulnerability,vulnerable,weakness,infection,comprimised,data leak,audit,admin,verification,unverified,uncertified,uncertain,unknown,inactive,crest,question mark,?",
        contributors = "danielbayley,jguddas"
    ))]
    ShieldQuestion,
    #[strum(props(
        svg = "eJxtyzEKgDAQBdGrfNKvZpOsbiDmBrb2ooWFgqB4fokgWNhM85i0j+eCuTM9Ozh3KAUosR1kJSVPCn+1k0UDBdsnJqe6XDm978ahEkQSyA/GYvjaDbTxHbo=",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,prevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,inactive,cancel,error,wrong,false,crest,bravery,attacked,damaged,injured,hit,dead,deceased,expired,eliminated,exterminated",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldX,
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik0xMiAyMnM4LTQgOC0xMFY1bC04LTMtOCAzdjdjMCA2IDggMTAgOCAxMCI+PC9wYXRoPsBaDmU=",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis"
    ))]
    Shield,
    #[strum(props(
        svg = "eJx9kEELwjAMhf9K6L3VvBlqod3Zy67epQoTFGSI6L833ZCJbrskkC8vebyYz12+nCi/kmEYys+hd8lsTR1XA67j7XBv6ZhMwyA8vJMCy3BEVw4kVhwqKuV/oQGI0dpZNYdebmfk+po3TvbAFFyXx1w570n00sRKcAqwW1D3VejL3k88GgvU/SemMaA3+CdX0A==",
        categories = "transportation,travel,maps",
        tags = "steering,rudder,boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "danielbayley"
    ))]
    ShipWheel,
    #[strum(props(
        svg = "eJyVT7sKwzAM/JUju1VLVuII3EC3Ll27h3To0KFD8ffXdiBQ8FKEDj3vpPReP088zsNNILzRRCOYBAypUUNf0QmqM4WSM1mp7xP/rwxLOlXZJR3ibBRmiL8w04QGvpkwWF/OnDqDbpXXyBQjBS3xzIgUpy4jONzjWt7aqZw4uf7kkNzdLCf7rP2O5HA0vqsaRnA=",
        categories = "transportation,travel,maps",
        tags = "boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "karsa-mistmere"
    ))]
    Ship,
    #[strum(props(
        svg = "eJxVjrEOwjAMRH/l1N3GdtyQSKUzA6zsEQwZGBgQA19PEoaq8ubTe3fLq7wrHqfpasIhIbBHaIQVh0PaKSXIJXC0ERaDjb+QcnAYW3jyPMhjUeg/BOfMyc/xo3JvElbO6KjVtClgZDeVaqzznqUGdy0N7b6Teud3WpdDH7/+AGDDKHk=",
        categories = "shopping",
        tags = "t-shirt,shopping,store,clothing,clothes",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Shirt,
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8ypF/MJsuSIuYGthZ2AYsUFhbi+V1TBASZZpjPy2e9GvbZLYqACL1ZajDru8y1b0Bh1YMiyeZKnt53yYNh/8bpp2AF+yqQDmFK8GP1ACINICo=",
        categories = "shopping",
        tags = "ecommerce,cart,purchase,store",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ShoppingBag,
    #[strum(props(
        svg = "eJxtjrEOwzAIRH8FZecaqF0Hyc0f9COidvDQSh3y/wr2EGVATNzjjqv/bW/0eU6/TCKUuExrvXVxrScSc8Yhe6mjpnPgumNECh5UkDYlpXmMdq0ZljfMV8HCKKTs6ldQ2K+DOBthZEGH1D9l5CY5at9r8NV5AFOAPuI=",
        categories = "shopping",
        tags = "cart,e-commerce,store,purchase,products,items,ingredients",
        contributors = "danielbayley"
    ))]
    ShoppingBasket,
    #[strum(props(
        svg = "eJxVjc0KwkAMhF8l5N7YxGZ/YHfPXnyIsgor7EGKB317je2hJZCBmY+ZVB9L7XdYMjJC/WQU03fGgCWd1rSkjTKfIx7pHfacXw1uGa9Co4K9Jl3IOWChSWYBgfF/AkwaWiQfdi5T1OEX+M7kdPA0nS9KLLZh3eUL/KMrmw==",
        categories = "shopping",
        tags = "trolley,cart,basket,e-commerce,store,purchase,products,items,ingredients",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShoppingCart,
    #[strum(props(
        svg = "eJxljDsKgDAQRK+y2GfdbFw/EHMCPUTAwkJBUCw8vdkUKZSBx8B8/BGvFZaxmhmYbyObGIEk5VMFX2sh+FIbUMA2ihb6f77bDjjvUVCiQ3GQQUk2ndJJ2dM3AzIy6bqcvtw6JLk=",
        categories = "nature,tools,gaming",
        tags = "dig,spade,treasure",
        contributors = "Andreto,ericfennis"
    ))]
    Shovel,
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrfNyt/W3aIlRv4CEEBxfBQTy/reAgNHySIS+E5HO9dmxTdwgEzoRa3ZyHOp/zpwt9oWjCKmYMeJut6RNSaz2ghs1TAqbbWLbMglEzD3rNImg1I5xqCRTFXPmFP3sAkGFQiA==",
        categories = "home,travel",
        tags = "shower,bath,bathroom,amenities,services",
        contributors = "karsa-mistmere"
    ))]
    ShowerHead,
    #[strum(props(
        svg = "eJxtzT0LgCAQgOG/crRrmh94YM4tre5Bg4NCQ/T7u4NyEvSG4+G9eB13gXOdmnZAz4Nvwgv/WBmaEjQL/SnFmWGKP98RNMqQtWsK1GblwhtXBQUGmspEMjLmIC+w0p1hudOvi2DAdPkCPSQt8A==",
        categories = "layout,arrows",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Shrink,
    #[strum(props(
        svg = "eJxljbEKgDAMRH/lcG9totYOVejm4g+4FRw6KDhIv98UUQcJSTjeceePeCasQzUTgzmrflOsuBp9XcjoP97DZe2ChYWRIVCjHdhkM5GcYHWHsjcUdzKxw6NJ3vKP3akFtUq6X3YBbJUjoQ==",
        categories = "nature",
        tags = "forest,undergrowth,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Shrub,
    #[strum(props(
        svg = "eJxtjrEOwzAIRH8FZedasE1syc3cpR9RpUOGVOrQ/1ewo0zJgAQc3L36e/8X+jyGl5LkRRBnQaA7KRLDKCCwYFwNwhk2Y/RRSNvyEJ+qw1RvzWmqh99XMilFiux1lj3OPK14WuppBcX9zDu9uu50nPyBOx8rjB1mR8grN9x8jSHxxLEBS9c7Pw==",
        categories = "multimedia,arrows",
        tags = "music,random,reorder",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Shuffle,
    #[strum(props(
        svg = "eJwly8sKgCAUBNBfudy9hj0V1HWbtu0jpSu0CJEef5/WYmBgzujo1wS3wQbhCi6RQSERyIeN0t+fb4zZ1Gh1VQ5WH0sicAYn0YPkah5GubfQsRySJxNcFVuUfQFrrhsL",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
        contributors = "danielbayley"
    ))]
    SigmaSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNbRQMA8z8TDLMVOw0AXiDEOjMl1jJTsbfZASOwDmBwru",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
        contributors = "mittalyashu,johnletey,ericfennis"
    ))]
    Sigma,
    #[strum(props(
        svg = "eJxtyzkOABAQQNGriN42kVAMN9DqJQqlQpzfUtBM+18+9jIaq4EnYKCb1IZHVCdGfOQ2TWEJMXDJU3Su/GUBNKMdsA==",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalHigh,
    #[strum(props(
        svg = "eJw9yTEOABAMAMCvNN1RjcRS/YFHSAwdDeL9WLreyRrbYDbsDEwWKaNK+qjiVV+dUHwuj+oO2g==",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    SignalLow,
    #[strum(props(
        svg = "eJxtyakNACAMAMBVCJ6vIQFRugFDkCAqEYT5eUQV9g5Hm6x60RUUeLY+aEJ3kVAqnVomfibAqyy1AYslFmk=",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalMedium,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVIwMsjQMzBUsrPRBwnaAQBpcAd7",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g,lost",
        contributors = "ericfennis,danielbayley,karsa-mistmere,azdle"
    ))]
    SignalZero,
    #[strum(props(
        svg = "eJxtyysOwCAQRdGtTPBtmRfSVkzZQS2eBIFEENbPR6DG3pMrJdZM6TM/CDaflo2Xa0Yvm55B7XCKMBa9Gs0raAKQa3xv6ZzTJRQ=",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    Signal,
    #[strum(props(
        svg = "eJx1jbEKwzAMRH/lyG7XOiI7ATdzlv5AN0MHL4UORd9fu5TQgos4gXT3uPwoz4rbebokCItCEdoI1KmF7xtqcU/m4nXa8qljWz5gBUMh+AnTscrPAzTuao4jmtK6qwwcWbxibpIFOiL/gr1xYNxnv3LFe/sUUtcYF1o8nBebI0wl",
        categories = "medical",
        tags = "police,ambulance,emergency,security,alert,alarm,light",
        contributors = "karsa-mistmere"
    ))]
    Siren,
    #[strum(props(
        svg = "eJwti0EKwCAMBL+y5ANtpD0I6m9KESQR2oP5fTX0trPMpK7NbhV0rfI+mTgi7IjggDkPOFNJ2y+W1KpcsJDpJBivgjDYcfg75eWUD7QlGMw=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[strum(props(
        svg = "eJwtykEKwCAMBdGrfHKBNqKLgnqbUgRJhHZhbt+0uJnZvDy026WCoU2eu1BCBCdwQELYPZFq3paquTc5YeyOMEMhPgi2Pvm/80/VF6L3GJY=",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[strum(props(
        svg = "eJxtjVEKwjAQRK+y5H9jZiU1QlLwAD1EiUIEBSlS1NO7aaH9aJmP3dl5zMZ8H/LjRkMyMJQ/yZx1fNWJaeNhTtu4oeD3sFf/LnRNpgskbpQSRp7yel/TJ8R6womtZ3BdC35brEOjLb2QkJsE6xs+WvEUVOrJMWCh+WWl6uul7A+jSDi/",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[strum(props(
        svg = "eJx9kMEOgjAQRH9ls3eRFZpA0pJ44+KVOxFiuRnSKP17u2200hoPkx6mMy+zcp2vBp7LZLTCCmFTSO7R83LTRmGDYBWeEFY2CoGdPHKik/fRaJgUXqiFphADldr5ZydglUAs73GIv3fSw766d9zAovofzJULhtV9lcGCl8Bs2LPti9/gJi6tfk0TDGh7qh8uNSbbCnGgjBeLbVIf4RtfNKe5S4HoqRzyad76kF4g93At",
        categories = "account,social,communication,brands,development",
        tags = "logo",
        contributors = "colebemis,ashygee,wojtekmaj,mittalyashu,ericfennis"
    ))]
    Slack,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJSAEMjJTsbfZCoHQBr6Adc",
        categories = "development,maths",
        tags = "divide,division,or,/",
        contributors = ""
    ))]
    Slash,
    #[strum(props(
        svg = "eJw9jDEKgDAQBL+y2CfmsnJGiHmBfsAuYJFCwUJ8v0bQZouZZeKRz4J1bPYA6YxCy3AZNim21aT4+VmCZQ9aJep1Izh5qUwflr0Vj3ccBM7QcPkrNzZ1GKw=",
        categories = "design",
        tags = "cutter,scalpel,knife",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Slice,
    #[strum(props(
        svg = "eJxtj8ENgCAMRVchLCCthnAAtvFgYjzD9gKlCMKpKX39fdj7ek4RwEkEKSI6eUgRUoFUI+TW2y1D3haU3soGKEJ3XpxJQJrlykcC9SOch+aLG1dntOVoglERnOos0Q35QIBVcPft6lva9Kz/sRXttE3VVgtd1gRTTbDpM/wC3phibA==",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[strum(props(
        svg = "eJx9kEEOgCAMBL9i+gFpJYYD8BsPJsaz/N4CBRSMJ7LN7HaLPfZzmwI6IITpIgeaH0xPYIUavJ0j5G1GebhUMhpRFcebjAiSxBC0JZh0B+dhM7E2INu6Bo8cUpK/ZivrMTdB2Mosf6yg7fzErh+/YOQUJa11aT3WrfVIPKbWLvAN3aZibA==",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[strum(props(
        svg = "eJwli0sKgCAURbdyefM+ilkDdQctIlJ6QoMQIdt9abPLPeeYFPaMO/rMloQiPJYkgUM8OH9zJBRLEyGVBlLjzgy1c+baMsNbWoXstZ6xQIwQktXZ/YeqbrXcCx0BHCI=",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[strum(props(
        svg = "eJxti20KgzAMhq8ScoDMtDVVaL3BDlGmrP4bUth2+6WWoT8k5P2A9wnb8ijwXueSI3qEb0RB+EQ0CJsaI+Rlfeai0eAUbnU/hVcqGeaId7YwkDXJk7OwS6fH+p6sVKBOz4CQExAynJjJCzT9U9r64QobaWRwpFRfU9NGUbf3A/sBtlszzQ==",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[strum(props(
        svg = "eJwdy1EKgCAQBNCrLHOAcqWgD/UGHSJSWv9CFqrb5/ozA2+Y0Mqp9NSsEsELqL0RHtRzBUmpl2gHB/qGt1EpzPZL4T5UKEfs7Ik3mRzbZpp+gkMY6Q==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[strum(props(
        svg = "eJxtjl0KgzAQhK8y5D02sxhpIHqDHkJowUIpgiJ6e13/4oOwPwzz7TKxrfsG79K8REAOrOmw1DJAGyydqeJDoSqe6BPMO2Yeglzbrp3A3/f/wcjSBINJtrWpUVXmqKxSO3tx6dU+zvQJfaLPCCzgm+ImGwNkSMYMfyw5VQ==",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[strum(props(
        svg = "eJw1i0sKgDAQQ68SZu9nigWFtjfwEFIFBRFRF/b2dlq7CJlJXozfLr8v8K8lVoQrWkvwIb3ONLl35pyeFbOlsQd3N9caCp2oShJUEGf27VgQ2NJAeFW0uuV45SBIIKxQPysVayqbjKQpa9kW+gMEqiuq",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[strum(props(
        svg = "eJxtjdEKwjAMRX/l0vfVJp21g7Z/4KvvpQoTfJAhon9vsoE6GCG5JCfJTff6GHHO5sggXwMCHEiTpfQSc9tFUYbO3Dx3pqSd3pbUrlO7XTBlEw3aKxtyom9Rr0sLLunPiGkkbr3V57HzNkiNp0NdDNROyDP8LL63FOFBgyXsLW9wZuHs7LDiHynPN0s=",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley"
    ))]
    Snail,
    #[strum(props(
        svg = "eJxtjEEKgCAQRa/ycS81g4gL9S5BQUFFixZ5+0axcOFCPvLeG79v54JEQRErPLIyietXlllFP2Qr+t+tDtfkc6lxr+leMQd18Aiy2mgDeZln0nADh8LQo2RR027sIOeL0NQvZ6w2RQ==",
        categories = "weather,seasons",
        tags = "cold,weather,freeze,snow,winter",
        contributors = "lscheibel,ericfennis"
    ))]
    Snowflake,
    #[strum(props(
        svg = "eJx1TbsOQEAQ/JWJfuN2HCE5ao1WoZMolArZ70fDXXKyxWbe4VjPHVtfTHTo5mYlCPecUDgmGLSqGEL5RIbwBaFq9We8P3dtYkJoEjnEw9ldbsKUXDL1Htoac7vuT1HCW/cKFwOLOFc=",
        categories = "furniture",
        tags = "armchair,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere"
    ))]
    Sofa,
    #[strum(props(
        svg = "eJylzj0OwjAMBeCrWN3ziO38WSqdWbgAWwVDRwbuL5KKoiLChDLZL++Tx/v8WOh2HM4sJDwbGfn1mbOT7meyyzCNh1aYxnct19bCvpOwIVJThVIvTpBIeoVkMAqiItf/0OTgE4o6mNZZHJPAS91G5AINbRmQW5ikJ/O3HDY57mVY2ey02vqy5Zf979HhE34C3W9WWg==",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,bowl,starter",
        contributors = "kemie"
    ))]
    Soup,
    #[strum(props(
        svg = "eJwVizEOABEUBa/yov/WsxENao1DCIVSIc6PZmaaCbOugR5VsRb0m81AO9EOFIL5b6+N8OqCW6hS+N6VDrfeDkU=",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Space,
    #[strum(props(
        svg = "eJxVTssKwkAM/JWh98TdZJetsBb8AH/AW1kPHjx4EL/fiZRCCZP3JNPf6+eJx2W6VZyHZK0gxOFq9FXrlYiI9LfGzNLIOrNwYVW4b9iYMG2w6MHWI5OZECOJqccnKeLReUkLQ7tPSz+FoKXvsjJPz9+yT36hoiZB",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Spade,
    #[strum(props(
        svg = "eJxVjjEKgDAMRa8S3FNNirSF2hP0BG4FB4cKDt4fEzNICQSS93lJvttzwrFNFzF4JJdgdbExMCxSJBuOAbTH6oG4C5Yx/RGDXw9VLExdOA4ao2gaJvVoAAeRYbu4TyXP+lt5AdHHIfo=",
        categories = "shapes",
        tags = "star,effect,filter,night,magic,shiny,glitter,twinkle,celebration",
        contributors = "Shiva953,karsa-mistmere"
    ))]
    Sparkle,
    #[strum(props(
        svg = "eJxtjksKwzAMRK8yZO+0I9W4BjcnSC/QXaCLLFLoIuT88YcQE4xAAunpMeE/rTO+r+5HgRr2Pk7bP6mTQHCPxbgVZ5H7qKAsGUCGTww1Fi3CJRPmoitEkY7C5CuIuQhNDaZ4n24ItxR4CEfst4Vuj8aeHnTNi8LOzQ8H+uqyAzJIQrg=",
        categories = "shapes,cursors,multimedia,gaming,weather",
        tags = "stars,effect,filter,night,magic",
        contributors = "karsa-mistmere"
    ))]
    Sparkles,
    #[strum(props(
        svg = "eJwlzMEJwCAMBdBVQgZojUhP6jKpVEF6EKG6fRM9/ZC8fN8Sd2gjoEVoc0VO5cldRoMgB4fwlbvngHQhLBL9qX/Rc2lcE7Aw0oKlWQw5RfscfS1vgkEbTUktsiuG1e1hSL26+APNcSen",
        categories = "multimedia,devices",
        tags = "audio,music",
        contributors = "colebemis,ericfennis"
    ))]
    Speaker,
    #[strum(props(
        svg = "eJxtj7EOwjAMRH/FYs9hx3GcSKUSO6zsEQwMDAyI7ydpoROyfIvf6c7Ts73udDvszgWFIr9DgjwEFbFFKI3lZSIkp9D1UqBHgzot8r1SQbTrwAqyZVKwJRJKMGsGH7SvtBA41k66lVPsobt52o8e87S1kdrriKM0h9HYNQfMGoSRWf+5nMSadlp/jtCzrD9V3TbDBz0FNY0=",
        categories = "accessibility,communication",
        tags = "disability,disabled,dda,human,accessibility,people,sound",
        contributors = "doerge,airone01,jguddas,karsa-mistmere"
    ))]
    Speech,
    #[strum(props(
        svg = "eJydjjEKgDAQBL+ypE90TwkpYn7gI0SLNIKg/8eLkFSpbI5lZoqL1/ZkHIs5PejhLQW6xKQ4FJViDdagOIeOmCHc6YgRei0hbrK8C/k2OhaN1Aa/mvbOC4BbK4A=",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck2,
    #[strum(props(
        svg = "eJxlizEKACEQA78S7OXORRaL1R/4CMHCRrDw/7gW2kiakMnIKLOhRtMZjsHWEbSRSfJtlOQcctC5hRd09egHabz1ly8+OBfH",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck,
    #[strum(props(
        svg = "eJxNjMEJgDAMRVcJWUBTKCI0AQdwCImCggcpHnR7k9ZDSSCE9/5PemQ9N8iMAUEfRhrtvowRJXWVSvot57Fi10qosa7l3mFlnCPQMFEA296G7IXS54Z8eUMfSg==",
        categories = "design",
        tags = "",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[strum(props(
        svg = "eJxNi7sKwzAQBH9luV7kbh3nAZLqNGnTByWgQAgpjLH/3icwxsXCzTET/8+h4pXkfoFdb30JBg0M5uPjXNTZbzjVTnI8ND/HrbIT+toVj0CYj6NpUW+sNWANu+z7+b0xWRKjYGYSqmDiyv4/NrdZeQEuoSVF",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[strum(props(
        svg = "eJxNjL0KAkEMhF9lSB9M5jx/YHdrG1t7WYUTRCxk8d7erMVxRUgyfN+k9/Uz4ZblPOJwGaupw5WgcnKrDgPhMWyDlLTpeEmL5Ef4rg3VQvIugad9jdOUGj3Kpivv+XjdMXsWp+DLLLTY8W8FM/9xsJ0qPzetJWE=",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareVertical,
    #[strum(props(
        svg = "eJxtyzsOgCAQhOGrTOhBd5eXCVJb6CFILGhMLAznNzQ0munmy5/u8lScqzrIQ6prTuU09TOnQRGyya8Qg7npaKRYWMx9mgwF1mxi4F0g3+wihwVe+0EvvWIjqg==",
        categories = "development,arrows",
        tags = "break,disband,divide,separate,branch,disunite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Split,
    #[strum(props(
        svg = "eJx1zssKwjAQBdBfucw+MTd9GWj6B27dS1tMwYJI8PH3Ji6UItkMw5yZ4fbXUwyYvBwqVEEbytDv8mzov9KhKQiJrkBVUTq44g25pds8Rry8NIIwL+cQvdSCxzLF8OmeXtjk9bz4+7TSwcHC3mlGA90qXYOKYFDtqHQLo5hmqfBIXqyy/3FWpjw19kVzG3sD7p1ZMg==",
        categories = "design,tools",
        tags = "paint,color,graffiti,decoration,aerosol,deodorant,shaving foam,air freshener",
        contributors = "danielbayley"
    ))]
    SprayCan,
    #[strum(props(
        svg = "eJxtjjEOwzAMA78iZBdr2bIdA2l+0EcE6ZCxQ6e+vlSHZgkMwYRIHLW8tvchz/v06JLTYWlal1vs1uXvWKK1V1TNqJi1waXoZXSgyoDvBsMsMRmZU6Sgaxa4FjJcnRwQgqxo5IYcWriPOL0qzpokNSo/V1c5TNrWpTPFR4BJNA+lKCSikRFgF1PjNb8i/p01oTtMw3MMyWfHF64iPXg=",
        categories = "nature,gaming",
        tags = "leaf,nature,plant",
        contributors = "ericfennis,mittalyashu"
    ))]
    Sprout,
    #[strum(props(
        svg = "eJx1yzsOgCAYA+CrNP/uA8XAANzAQxghwmBiCPFxe4WFyalpv1RFtyZ4FzafNDFJuDWNhPjFQHhKuYJNvqhRXT4YdSzJw2qa2QB5FshThV22ExiHaPgf9hCo+AKTWSY4",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[strum(props(
        svg = "eJxNi0EKgCAURK8y/L2k1qKFepdI6bsIQj5Ut08NImYYZniMK2kVcMobiyczE84chd96eRoJd89Sh6XghnYI7liEET3tRsNoZdHVeCN/PqHaqq6PP8G4H/E=",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = ""
    ))]
    SquareCode,
    #[strum(props(
        svg = "eJxtjjEOwCAIRa9C3E2F6NDEegPX7iYdWJp0aHr+goskms/Cf3wgP+1luA53YwAMnqDLlbwpKXnwCFLku2ZeExA2DQcR6tCZRq9BxmgNoM8afnm27rKWcQH0GUt+dzwxuQ==",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottomCode,
    #[strum(props(
        svg = "eJxtjLEKgDAQQ38ldD/0QjsIZ//A1b3gcKOD+P2ei1aQLEkePNvb4djmtBRQG0GMERUK1/JuxHbN/QGe/SHRUrXhFlZ7tFNoXX+A5i+5AIJlIUs=",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottom,
    #[strum(props(
        svg = "eJwli0EOgDAIBL9C+ICpXjy0/Qw2QuKJkIi/F8JpdzOzXRcZvHIZD2wnAi+52ap/Aw8E9YE7gueYfcvD7CRKzwIKpQUkr9SIlArPHwYyGo0=",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareDot,
    #[strum(props(
        svg = "eJxVy8EJgDAQRNFWlmnARAU9ZNOBRYgJbm4SFozdy+LFXObwHxNqPpQkl1OU4VdQbYwRdJek8pWHMYGabQyDHWK4dhVKjG0h78Q7E2u9zH95AXNVHic=",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareEqual,
    #[strum(props(
        svg = "eJwli0sKgDAMRK8ScgFpRVBoexkNJiAuSkBze5O6mR/zSqddob8VM4LrjGBDmeRkrZhWhEcO5RFbmQJo5ZKbwJKPi3Pum1v+q+Wofo1T+wA7iRpg",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareSlash,
    #[strum(props(
        svg = "eJyNjLEOwjAMRH/l5D0htiJEpSR/wMqO0gp3Q1UE5e+JQerQqctZ57t76XlvijHTNYJDdewZwYnzQxe5xRrsZQ7da6y/AgR+6CJU0skIJW0cDuDzHvRyx0nLVBve89g004WwrJmE8MnEkbD+j07zQ5vlfWaD8gVxXTCp",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = ""
    ))]
    SquareStack,
    #[strum(props(
        svg = "eJwli8sNABAQBVvZvAYEFwc0g1hX2QTd+90mmRnfSxKaARY0WhYO0A7EpVWWz+vJfhqD6NUd4ga9FQ+D",
        categories = "shapes",
        tags = "rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
    #[strum(props(
        svg = "eJxtTjEOwjAM/Mqpu4PtJG2RQhdmVvYIhowMiPdzKRUwVKeL7dg+X3nUZ8P9NFxsxlgTErRDejYRn4rZTSUjrTQNuabAlFQYB46MzsrXnw7G8wRTRJj1hwpsz6SzSTTTYSmH7mApPx8jXHnMArdDlAiyWXX4puziL/F/r5KuaU8q85R7pcQ2yduS9yZnWGxB7dt7A0AdPds=",
        categories = "animals",
        tags = "animal,rodent,pet,pest,nuts,retrieve,updates,storage,stash",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Squirrel,
    #[strum(props(
        svg = "eJxtTkEKwkAM/EroPbGzaXYVakE8e+3BW6mHHgXF9zu1WETKsruTTDIz7X14TnI7VpeQlCY0Vdfu5l7XrgwOlorArfgpGed4689BIYRPCvwxjSAsepQBgu+0gAa/DcVLYTHWajmrpczKlT78S7luZaGw93uLMxFDxfLE4OKLqrr6yP2cZ0wPMg9IoqvPiXxVfQPcMDdG",
        categories = "design,cursors,tools,maps",
        tags = "mark,print,clone,loyalty,library",
        contributors = "karsa-mistmere"
    ))]
    Stamp,
    #[strum(props(
        svg = "eJwBNwDI/zxwYXRoIGQ9Ik0xMiAxNy44IDUuOCAyMSA3IDE0LjEgMiA5LjNsNy0xTDEyIDIiPjwvcGF0aD5iWAyo",
        categories = "social,multimedia",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "mittalyashu,ericfennis,danielbayley,karsa-mistmere"
    ))]
    StarHalf,
    #[strum(props(
        svg = "eJxFjVEKwjAQRK+y5D8ju0m6CaQ5QT2EoKAQxA8/6u3dWGl/dphh5m19Xd53us7unBEi/Y5QgWhPFJF1SchCwsRCrFAlzuBsSfdIxQfE4Fo9DU6rO81K0QYCnUj+QD+hsOfFSNI9I9orS451fzxvtPLsxNFHTIZudt2sVUepfQH/oyqn",
        categories = "multimedia,social",
        tags = "dislike,unlike,remove,unrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StarOff,
    #[strum(props(
        svg = "eJw1jUEKgDAQA78SfEB0F2u3UPsdEaQt6MXfW7FCTmEyibUc91Yyatnzda6DKBTiOAUYdYEqAtVDWmbKDDGKQYVT41o8vYejae9+ru+MQT7Tax5SHPtjegDvlRuE",
        categories = "account,social,shapes,multimedia,weather,emoji,gaming",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "colebemis"
    ))]
    Star,
    #[strum(props(
        svg = "eJwtyksKgDAMRdGthIwLNqEDB212I1IoSUEHZvfGz+i+B6eOrhs4NeSMcHFDWqP01eMXlLo8Suq04bspTOt6HkFK4gwlEUPMF/5EbhWdGDQ=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[strum(props(
        svg = "eJwtyj0KgDAMQOGrhMwF21Cc0txGpFCSgg7m9saf6XvD49F1Ay8NK8IVrAG9eEAZhZfnEZ42fDeFaV3Po2HJqQLlVAgiv/N/5AYGcBf0",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[strum(props(
        svg = "eJx9TcsKAjEM/JXQe2KTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjNntPEwi5LTlyYIF1A8jOVwFRroPKeqgR4qoVY7ffGuPhx48yI1cSklfiCDclbUZrSe/uCTh0/p/c0X8+23lplxO0ezZiDbRHNqy4qBym9Vye0zIy2Q==",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[strum(props(
        svg = "eJxtjDELgzAQRv/Kh7vXfBcTLKTOHdq1QzfRIaOg+PtNdFAhHAcH790LU79EjK/qSycO9u16hcLkqdO1shkMKJQnMtDI5jSgtf5acZ/j+V914ZGDXbhkYVcffQmlsJ+lBVOPA8XuReYt6Mmz0ZQ6/k42COY04A==",
        categories = "communication,social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Sticker,
    #[strum(props(
        svg = "eJxNjLEOgCAQQ3+lcQe5gyOaILODrg5uRAdGB8P3KzFR06XJa1840pmxD81MogV2lMRgmBp1t0JuMyBNukcFnMl9C7DipdMyPee1iaGtwhh+Wtjis3/RBdRlG0I=",
        categories = "communication,social",
        tags = "note,comment,reaction,memo",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StickyNote,
    #[strum(props(
        svg = "eJwlikkKwDAIRa8iXqDDolBIvIyVKnQlQpvb1ySbP/BeYXN+BLhV3HYEz1oR+BuXyjI5FRcOeO0KrXggqNitMWaqJ0LrmX736Ac5Yxim",
        categories = "multimedia,shapes",
        tags = "media,music",
        contributors = "colebemis,ericfennis"
    ))]
    StopCircle,
    #[strum(props(
        svg = "eJydkb0KwzAMhF9FZJdqXZzYhTSQvV27Bzp4KXQofv7K6Y9TyBSMbTjpk3RoeMzPRLdTcwcF8uKVyzOBQM6OUpDYElKU1s9VVfGQ7niGYc04HEqZcfgWu3hS5PjLd/YjKdYCI3PcQLUjWKg2c2y5ibEWCNlvsDZMgtsKWCS3df6lgpshgcp9a2qGWPpWwkdyLNGA6S+NtKfiZBeK3WjcTS67cGvvjGtd2gsI8GyR",
        categories = "buildings,maps",
        tags = "shop,supermarket,stand,boutique,building",
        contributors = "karsa-mistmere"
    ))]
    Store,
    #[strum(props(
        svg = "eJxljDsKwCAQRK+yzAXyIaTa9TKJZG1lIXp7FQsFm2GK9x5H/xjFJDhBff/wmta3g9SHT01wg7LgguOt8Y5na4KWQLWOoRWyKR7C",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[strum(props(
        svg = "eJxdjEsKwCAMRK8S5gL9ULqKXqaVxq0E2t7eKAriZhaP94ZTuJTS57CDbA+QhPiIGlhBf+VvvFUcTnheiu+5VqZv5rd4UsebnmWaBx7C",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[strum(props(
        svg = "eJxFyjsKgDAQhOGrDNv72M0iCknqNB4ioKAgYmERb2+SJgzD33z2ie+BzdHKEzQs0cBgLOuknw2UvB2K8bZJBUtUaIWcP4epueu8dyR2pIQkjmQkfLksuVybbVH+B6XaHuE=",
        categories = "text",
        tags = "cross out,delete,remove,format",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Strikethrough,
    #[strum(props(
        svg = "eJxtjjEOhEAIRa9C7GEHBnZmkllvsJUnMFrYmFh4/wgWVoZAfv7jE/oxnxusv2FXMKhQh7F/whv7Q1jA8BX9JQG3DXVJyGSkigIuUMimYEY5Q4jYIC1I7N2y6+oJabMQM9wjRXnyK0518SEkilT0tpofTuX54QJsriqC",
        categories = "text",
        tags = "text",
        contributors = "nabanita-sarkar,ericfennis,mittalyashu"
    ))]
    Subscript,
    #[strum(props(
        svg = "eJxtjDELgCAYRP/K0S55n4oE5tzS2tAmNDg0NES/P13EQW453oMXnvRmXOu0e9BkO8UwVxRDE3TVyMB4LENOU8QoJQRdEgh0GVV5m7+VhT06DFGSaXsA+ajPVvwByMYvBA==",
        categories = "",
        tags = "captions,closed captions,accessibility",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Subtitles,
    #[strum(props(
        svg = "eJyNjUEKgCAQRa8yzAFMR9ONeoMOERYYtAhpUbcvcxOE0urDvPcYG5YU1hnC4VAQQjjLJocKve0K9nYb9wiTw0EQqMi4yDAfX4g4CKqwOyNeYaqRGaZ7A5pJJdtKmYrz9H+U76ML4TVNzg==",
        categories = "accessibility,weather",
        tags = "brightness,dim,low,brightness low",
        contributors = "mittalyashu,bduffany,karsa-mistmere"
    ))]
    SunDim,
    #[strum(props(
        svg = "eJx9jEEKwjAQRa8yZO+YmUknLpLewENIFCooSBGxt2/TdFOadjF8mPd4IT379HpAH40zkP7REE87zNuGc8Ft+Ny+HdyjuRKD/Cij/FoDtlUiQNzVANsd8qYLijpoUEVP6K3PV/EUxU15j9r4I28OldyBVTpLbuONN3ZP1g==",
        categories = "accessibility,weather",
        tags = "brightness,medium",
        contributors = "mittalyashu,karsa-mistmere"
    ))]
    SunMedium,
    #[strum(props(
        svg = "eJx9TkEOgCAM+8rincnmopAgP+ARJB64kHgwvt+AHkwE07SXtuvcHo8E2zoEYjCR0UxQRVfIAw0EpETJ4N1YKt69i3xyx9AtJwtaKCSUwkaCFlzgll4mMBCn1jDrjpNnnOpV9bNskcpzn8gFd6xNEA==",
        categories = "accessibility",
        tags = "night,dark,light,moon,sun,brightness,theme,auto theme,system theme,appearance",
        contributors = "itsjavi,mittalyashu,karsa-mistmere"
    ))]
    SunMoon,
    #[strum(props(
        svg = "eJx1j7EOwjAMRH/l1N0hF5ukSKF/wMpeiaELEgPi+3GWdnEHe3n3rHP/rN8Nr/v0YMZtVSgy6JNRp6VfBl76Hipg2RgAGgqfGpEMOwOFPwmuvTVVA+ekNTVJLUhYUkP1JSeJ0ci7zoHLBhP/NEIuNcSsELyKDlMO/gcsEFYT",
        categories = "weather",
        tags = "weather,air conditioning,temperature,hot,cold,seasons",
        contributors = "karsa-mistmere"
    ))]
    SunSnow,
    #[strum(props(
        svg = "eJx9jtEKQEAQRX9l2ndjZ2wrtesPfISWoihJ4u8xPGl5ubfm3KbjQj+HoYXZK6Mg7F4Rn71Jly69cemmeumg8aoiBl4FXacX0DEyGiwykCA0JBFZUY7WwpPfu4qBuIsJsP4go8XM3J+TX4MCdS6ikdkB4dhOsg==",
        categories = "accessibility,weather,seasons",
        tags = "brightness,weather,light,summer",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Sun,
    #[strum(props(
        svg = "eJx9jrEOgCAMRH+lcQfbShQSZHbhI0gcWEgcjN8vYHQRzSU39F2vtVvYI6xz54mBD90525eRszdISpoBCKtLRdXeMc9AOnIL4AdJZCROV7X4rc6v8dJq0DCCEgqyGns05suhUCwSGvBJnYlYQYE=",
        categories = "arrows,weather,time",
        tags = "weather,time,morning,day",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunrise,
    #[strum(props(
        svg = "eJx9jLEKgDAQQ3/lcG+9O4tWqM4uXd0LDl0KDv4/Xiu6WCWQIS+J28MRYZsaTwyEKzeza3M2u5sko8dOWHFtqNi75mVvI9cAfpBEo8bhula/1wzMS/Whh14ZMEpUGQomG4QDZikL+LRO2T9B0w==",
        categories = "arrows,weather",
        tags = "weather,time,evening,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunset,
    #[strum(props(
        svg = "eJxtjj0KwzAMha9isutVlmVbAjc36NQThHTIUujQ+1OZQqYgENL7nn7GZ/se6XVf3pqyJyNb1nGb4jpOlCUYXbKHcMpykO5MGRWqQuFGJUF9BjSUoimKDmYJF7QLIXeCl2hMY058E2Su6Z95Rixo06Wl7ZEFUpzQi4WsBHOPM9zOl35fri4U",
        categories = "text",
        tags = "text,exponent",
        contributors = "nabanita-sarkar,ericfennis"
    ))]
    Superscript,
    #[strum(props(
        svg = "eJxtyzsOABAQANGrbBwAS/yS5QZavUSxpcL9g0alnZeh2RfDyKKiBoPNchSF1K2FnnlAz+kDZ0rScXi0AZPsFpU=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    SwissFranc,
    #[strum(props(
        svg = "eJxlTssKwkAM/JUh92Aza33Abs9evHovq7CCBSki+vfGethKCWEYZiaTeO8fBeckRzPY/rDuCaLxMaXytK0czksrXVx9M12syYC2/BnBpzXzS2DRWTRfx3y7YEwSBPmVxOj4ntBNP7k2DLYDqUEDfJcfDBtvmjRU9QOfMTZx",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[strum(props(
        svg = "eJxtjUsOgCAMRK/S9ABq8RcT5DYuSAiQ6AJubykaWbhom+nMa3UMLjvrD4jB+uvckaZuBlq5jbBw1S6LYqHR/csYLWRWTI0ImXhuCImqTko0A024LNXwhBYUuOhc9U+YqilTNR++yzfwnTJd",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[strum(props(
        svg = "eJx1UEkOgzAM/EqUB7SNyUKklN/0gIQAqT2E39dL0gYEh9iyPRPPOK3LtE3j/FLrMs6f91Mbe3PKBAyd8vgkcoNGekj3yhkSMzMgK2q1GcmZcoc1cEZCA2aQlyE8ChjrLPUezE0jP3OGZkP8g89NeJLco3gwErzYiJcuqglbVvWyyh09NNqDSLeFGY4OsFlOQQYI6n6XqtAvwX9jzA==",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Swords,
    #[strum(props(
        svg = "eJx1jjsOg0AMRK8yoreD8X6wtOEEySGipEgBEgX3F14KKADZcjFPnpkyf5Y/fs9mkh4dAkIzlEcVh3KgjAwlPaO3GAw9Z4ixfkkg1HH0qxzQjsTJ1/VtKmkreUlEvAgyiNx0cE8jhV7V8w8kpB2tcwA2Eg==",
        categories = "science,medical",
        tags = "medicine,medical,needle,pump,plunger,nozzle,blood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Syringe,
    #[strum(props(
        svg = "eJxNjbEKgDAMRH8ldA82UUsLtbOLq3vBoUvBQfr9JhWk3HK8O+7inZ8C12aOAPO+ZgYGq0JxbakOXSH7Y4KOtdzIV0FjaoGRzyAp07BF2PHX9ibFSU/TC8dLG+Y=",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "abejenaru,karsa-mistmere,ericfennis"
    ))]
    Table2,
    #[strum(props(
        svg = "eJxNjEEKgCAQAL+y7AdCRShwPXfpEZHSegtZyn6fEpTXGWbcsQpDIFyUBXOqEb0bGvMux02AY9pZCKuAm9Ag5EKoEa4UhF9eGq9ZC7z7hlrBNJv/1wtlO/MAwm8lIw==",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[strum(props(
        svg = "eJxNjEEKgCAQAL+y7AdCJSjY9Qc9IlJabyFL2e9TAvE6wwxduwoExs1YcLdZ0NPUmKccD4UnBRXGyuFldAgS0yn6k1wYLUJpomYt8NSHDlYZf4Mw82g+yhAlbA==",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[strum(props(
        svg = "eJxNjDEOgzAQBL+y2t7Ed3EiFzY/yCNQQDm6CFkJ/B67AXTNaXZ30jK9C5Y1Uwmb5o+VTAnElhmJ/zwWq8ATtXJnn25t0KfvUAxj5uuBMCgUvp5AnZroFUB/8jyBq5857UJTNclFFSHROi9HtAOVPCe0",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = ""
    ))]
    TabletSmartphone,
    #[strum(props(
        svg = "eJwti0EKgDAMBL8S9gHaBhEPbT+jxRbEQwlof2+iXnbYZSe0vAqVXPciEexAd8QEagoG9Tfbh6tuUiL8jBRG81I46pmpex0XNY0m8d+NPDhvf3umB4ewHP4=",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[strum(props(
        svg = "eJxNikEKgCAQRa8yzD5zKrOFeoMOEVNgUBDSom5fmlSb/+G9Z3gOvEzAh0WNwGe6YFGhM+UjnclRwiml3N7/y7Zh9zBa7KkC0p5klBF+aq1F0wJJoRrQQnZFnDe7AEPiJws=",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,pills,pharmacy",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Tablets,
    #[strum(props(
        svg = "eJw9jbEKgDAMRH8ldE9MY7Ut1M4O+gNuokOHDg7i92sQCsfdcI+7dO13gXMyqxWQWR7LNZJEUDsouk8g5IJmT06A60hDQDXt8WdQGWBUZNGpzeTU6XhO7cKDL8S2NS8qIx3l",
        categories = "account",
        tags = "label,badge,ticket,mark",
        contributors = "colebemis,csandman,aaofyi,ericfennis,karsa-mistmere"
    ))]
    Tag,
    #[strum(props(
        svg = "eJxljTsOwkAMRK8ySr9mvZ8klpbUFKGloIugoAgSBeL8eIREisi/YsZv2mt5P3A/dmdDPaXPsPaSDFw3seKNJGXkzVIS4pqljoGLevh5Aj2IgZbZQdduageSp/bn9zCJerG99NSK6pGZsziKkYhe6uPMWQfo9vgFD8kqfg==",
        categories = "account",
        tags = "labels,badges,tickets,marks,copy,multiple",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Tags,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVEwKTM0U7Kz0QcJ2QEAXiEHNQ==",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,one,1,first,bar,prison,cell,sentence",
        contributors = ""
    ))]
    Tally1,
    #[strum(props(
        svg = "eJxVybEJACAMBMBVQhYQIQjCmw0cQrCwtBDnD2kCae9w1zu0B08h+bWxojgpInoOA3CwDm4=",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally2,
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnvcVh9TxqVm5IeyWxILxk+lAjELRdMKxXT",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally3,
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnveFh9TxqVm5IeyWxILxn+KNGQkIgzF+gzHT0=",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally4,
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVPi4gCSIKMRs4hGBhaSHOLzbp0t7jZI+zMFvoCelSDirxJxWD6gG5C7kPMzIYVMweh1QkoQ==",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = ""
    ))]
    Tally5,
    #[strum(props(
        svg = "eJxty7sJACAMBNBVQhbwU1hFlwkWgpWVbu8voIhVyN074lQ4R+Dm0ViEMo5G4LreQGr3gR4nYHr3YSO+kIwO68M2IVU=",
        categories = "brands,gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[strum(props(
        svg = "eJx1y0EKgCAURdGtPJxLPu1Hws8duIiggZOgQfunHASBOr2Hq9d+FxybyYzwDnSYTdKp5qQfnlItWi6t5fBa4dq56EGxATIy/O0BP0Alhg==",
        categories = "nature",
        tags = "campsite,wigwam",
        contributors = "MoltenCoffee,ericfennis"
    ))]
    Tent,
    #[strum(props(
        svg = "eJxFy0EKgCAQheGrDLOPUINaON6gQ0RK4yIIGUhvnxoRb/c+fnttwuAJzxmUAj30obNjA2c/Xisqw9MvKewCmdAgpEKoETjEg4VQLQjlhdzhjl64/7VunXsAlYEgqQ==",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[strum(props(
        svg = "eJw1y0EKwCAMRNGrDLlATbCUQuJtuhBEhXaht68NdDV8eKO9lVlyvdBbrs9tFMEHOIAZETsl3X6S1OFgIxbC/PYkDDGSsFq818HxC7D6GNo=",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "colebemis,ericfennis"
    ))]
    Terminal,
    #[strum(props(
        svg = "eJxlzDsKgDAQhOGrDPZZMxtZI0RrCz1EwMJGsBDP7wO0UAam+eBPa95mTG0xKlHDJCqUwphVYsB9/hxdkKZx4rn7L8G7amCNUHSpvHpdeqoLDQqD/WmkgtZXrxyfJSFF",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube2,
    #[strum(props(
        svg = "eJxtzaEOgDAMBNBfaeZb1mZliLE/wOLJEJUIwvdDCZnC3svdlWM7DfY5LJxIQS7OpC0CU0ImBiFFcSC12J4oQfTAET9cJdQy+FAtfW7ykuUfeY94NNSON2E+ITM=",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube,
    #[strum(props(
        svg = "eJx1js0KgCAQhF9l8a65mz8IJnTr0rW70MFjh/D5SxChMpY5zAz7Mf6IZ4J9YqsDymiFnkloKJL3IZjiKMtXrACd0Bux4IeCCL6BSFZSfLzw6rL85kVd1giUbCdH9VM4QLOo/io0ietWXcWDPgo=",
        categories = "science",
        tags = "tubes,vials,phials,flasks,ampoules,ampules,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTubes,
    #[strum(props(
        svg = "eJx1jbEKhTAMRX/l4h5ek9hXhers4kcUHDI6SL/fCkUqKJnuzTlJ3NNh2KZu9eiNk0LhyjC0Tk2kxt0cfxc/x9tihTijRqSCoknQxb+IHvxf+iSQCgpJpqZAKb5eDhZaEJIflyBG4cUcETK7e3ECuCc7lg==",
        categories = "text,layout",
        tags = "select",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    TextCursorInput,
    #[strum(props(
        svg = "eJx1jCEOgDAMRa/yM9+wLg2Yshtg8UsQlQiy89MhKIZ89ZL3vp7tMhxr2nhBKUbcBILsYxKSfQ6Gs3GqOo2o6ps+ZYR5iJ1+zI/oj5Ae3g1sVSJx",
        categories = "text,cursors",
        tags = "select",
        contributors = "ericfennis"
    ))]
    TextCursor,
    #[strum(props(
        svg = "eJx1yzEOABAMQNGrNC4gJcFQnbs4hMRgNIjzY+nE+l8+jTo7tGwKRgjiDZO9iUnBIaCT9JH0FH+WFRQ2v+Acqg==",
        categories = "text",
        tags = "blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextQuote,
    #[strum(props(
        svg = "eJx1j1EKwjAQRK+y5L/obJpqIO0NPERBoYKIHyL29u6GUBvYkI/JzrxJ2PSa3wtdR3cJ5GcmpqOeTm5uSgdNp7QxiDsIZEMMQvxTjacCMfZQZ0Hy3QLTZ5gB+kZDgkbFU/zAXsMOPKFvNerkcX/eaOXRnR2tyPIVOYmIiaCkMoUsmZLgwujMea7ZbA6u6gyl4zf2B2ZkfkI=",
        categories = "text,cursors",
        tags = "find,search,selection,dashed",
        contributors = "danielbayley"
    ))]
    TextSelect,
    #[strum(props(
        svg = "eJx1yTsOABAQBcCrbByAPOJTLLXGISQKpcL9g0Y02hkedXZqURR4chLZiMTqYOJbGgT9OVi5Nzy3ALf9Fo8=",
        categories = "text,files,cursors",
        tags = "find,search,data,txt,pdf,document",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Text,
    #[strum(props(
        svg = "eJyVjjEKgDAMRa/ycY82SZUO1Rt4CNGho6B4fm1BdIiD/Cm8l5/EddoTlr4aBew2JYVSqIbYZDDEBxdOWmIJ7CCzg689ad0hUICp+VtD0WBr5Zogp/3+5sJkCwHcJrtY5CCecrO7wrki+fcMOfjj81+rJ8rGV2U=",
        categories = "buildings,social",
        tags = "theater,theatre,entertainment,podium,stage,musical",
        contributors = "danielbayley"
    ))]
    Theater,
    #[strum(props(
        svg = "eJxtjbEOgCAMRH/l4q7SUokkyB+4OriROLCYOBi/3+LAIkPb3F1fG650ZxxLtzKIM5kuhrF4MdTEQx5y/+C08LCwvVYDI4bTBa+9BTvQ/LE0TKUaF9iUz2aYJAkEBgTqdW6SGKy6OKr3yr52xjTn",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,cold,freeze,freezing",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ThermometerSnowflake,
    #[strum(props(
        svg = "eJxtTrEKQjEM/JXj7Y1Neq0+qG92eauDW8HBRXCQ9/2mIiJYQi6Q3OWuPtrzhutxWtUwN4KIvYJhL3la6q4TlvpLS5v9H+5FCvQgDCqE90BrEdw0SubHSKHB55nNYG9j7fvLQOs/02nguxZJ9KwOlDkhO3xZL3hzN9M=",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,warm,hot",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ThermometerSun,
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik0xNCA0djEwLjU0YTQgNCAwIDEgMS00IDBWNGEyIDIgMCAwIDEgNCAwWiI+PC9wYXRoPrArDeE=",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Thermometer,
    #[strum(props(
        svg = "eJxNTrsOwjAM/BWre0zu3DapFCqxdYCVgS0SQwYGBsT3Y0AKyJJt3cPncq+PJtf9cEISjGcOa9m9sbV0ZhFkBQXRFduoSJVCiV4I0IWBOs03qlnIh07JrJNwY/ypffKZ/9y+NXenuWPRLyZPVODomWSL1RQmn/Z1WTDN+dJffQEUJiqI",
        categories = "account,social",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[strum(props(
        svg = "eJxNjrEKwzAMRH9FZLfqk+JYATfQrUP7A90MHTx06FDy/VVacIJA6N6dhMq7fho9z8M9E+IKGZZy2thSuoNEic0Io0eaj1qFhKIXCDy74DS9grAq2eXgZU4kch33fJAga7DDAQdNOE8dOeQ8BzBwg8MWqzKUfu2/swmzR//1C10SKtg=",
        categories = "account,social",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[strum(props(
        svg = "eJx1jbEKgDAMRH8luBebq7YItbOLq4NbwaGjg/T7jQ6aQQkhXN4dF/d8FNrGZgYN2ZEjK8OyviKDcGsrF4W9fhhUA50wfgmvwYhh6rQmrE2K7VWY4lPLjvqKb8Dhl7AiJ97sLxs=",
        categories = "account,transportation",
        tags = "entry,pass,voucher,perforated,dashed",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Ticket,
    #[strum(props(
        svg = "eJxtjjEKgDAMRa/ycTc2sdgKtTfwEAWHLoKDeH5Th3awhE/ICy8kXOnOOLZhZwPJdohhKiiGurC0gDl5eJivmBx8SSVlUtqRHVloqs1gQ7O2BmgtTP7yKdDSt0znMAtYnrFpLxSyMK8=",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    TimerOff,
    #[strum(props(
        svg = "eJxtizEOgCAQBL+ysQc5OAQTpLbQR5BYUFoY3u8ZE23IVruzk85yVRzLsJOBrTzkND5TTj+wIG6qhxjkSkSEkRCiCngbgZTXTsRNPkFPHXkWsHLzH7oBER0j3w==",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis"
    ))]
    TimerReset,
    #[strum(props(
        svg = "eJxFy0sKwCAMBNCrSC7QRlroQnOZ4EKQLlyZ2zefgquBmTdl9LclwQoZkmSPpYGXprZ4ApXDEBWnPt7/aB8MbF/EjblPHi3xCsYSbFZ4DMVMHxNUIQw=",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[strum(props(
        svg = "eJwli8ENgDAMA1eJvADQB+LRdplQ0Ui8oki02xPCy7J9l7Wx0SOn9YK0gnqTq1vBlkCzYAcNP0A6omhsNS+fVzOL8t1IA2FHDo8ZtjP/W1+a2Rwe",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[strum(props(
        svg = "eJwti0sOgDAIBa9CuIDWhau2l0EiJK4Iie3thdTd+8xUY3Kw2fBEePVyaXjsCGsYURCE9RZvWCLbyKPXLb1eSY0eBoq1BE/zp9ILaN39A6nbHE0=",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleRight,
    #[strum(props(
        svg = "eJxtyzEKgDAQRNGrDOlFZ5UlgTX1Nh5CsEhp4f2RNCHFtv/x7b2/hudMlxCH76na2lO1AczIrhEUULxEoqC2JZwI2abpBy/qJAk=",
        categories = "weather",
        tags = "weather,wind,storm,hurricane",
        contributors = "ericfennis"
    ))]
    Tornado,
    #[strum(props(
        svg = "eJx1jqEOgDAMRH+lmW9YL8vUmMZg8UsQMyQIsu+nQzBESUWbXu9d01muSvvs1kChgEC+F+vUBGOhHVWiy2nqjpxeH0hCFVjKI7ElieJ8Y4N39Ehonv8hxi1+/2QsMk5veig1VQ==",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TouchpadOff,
    #[strum(props(
        svg = "eJxNi0sKgDAQQ68yZC/aUrrq9AYeQmxxupMy+Lm9LYK4CSEvL9S8Kt0MB6oXw4LePEtSaW0CSS6bKMN4xDB2IYZ9UaHEmC0ZJ+3VSN9+xFiy0zH4Dz10JB5X",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Touchpad,
    #[strum(props(
        svg = "eJxtzLEKAjEQhOFXGa7PmtmYTQLxaptrLewOLFIoWMg9v9FCOQhb7Xzw1+f6aridpoVZFFTRBPWw8/FOybD+rwThvyclSgqNXuJuNdfn6zTXw6c31181g2ErA6BBdXMDehRYT6eBMMLckBYq7KIjCNDm/vIGRyE+qw==",
        categories = "travel,transportation",
        tags = "airport,travel,tower,transportation,lighthouse",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    TowerControl,
    #[strum(props(
        svg = "eJx1yz0LwkAQhOG/Mky/8dbTcMJdahtb+5AEN52Ew49/720niEzzTvHkbZkqbFlvVgt1T7wKI/EuTMTWjhLPda7WKnHIOwdDvo/VMBdeNCBdj1OQrpfuIOo796NCEXzS6hEdOvmGp19oEv/KD/BHKuY=",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[strum(props(
        svg = "eJx9jtEKAiEQRX9lmHennXJcF3T/oNfewwIDg1gi6u8bW8hgl0C86D1zNNyO9wyniPsd2DwUhh7HsKm3Y/h2FpgPdqXwwN1qwR4kGXLQGQayuvFDyC3JK3cqr4uGVHlNElIxcTGakg23sXSZUjnDFJEEIT0j9hqviCwVmutf7B/VfuuAfTayeOcz47GKtrNIT83wBjVxViA=",
        categories = "transportation,sustainability,food-beverage",
        tags = "farming,farmer,ranch,harvest,equipment,vehicle",
        contributors = "danielbayley"
    ))]
    Tractor,
    #[strum(props(
        svg = "eJxtUEuOwjAMvYrF3m/iOCat1OEGc4HZVWXBAiSkcn/xHCQ2oEQvifU+dpb7+rjI+ffwN8PliLo2RMiAMlagSTmclp9knpY3v2MWK+gbZkxSyTI6NFieu6Mq0seUzy9yc+odsRpmlwEjTx2Twq7KYtl4SyNDSziSYOgMq+IS4nuku4T6VpRFQhKZ/P8ZeetsyKjVNjw63QOumNAJdLYdTH9dr51x/IYcL8g2IlvMso4yNWxRaZUj2q7DKneqM0MrpncbT9lfT0Q=",
        categories = "transportation",
        tags = "roadworks,tarmac,safety,block",
        contributors = "danielbayley"
    ))]
    TrafficCone,
    #[strum(props(
        svg = "eJx9TzsKwzAMvcoju1xLVkwMbm7QNUM3QYcshQ7F56/6oQRigoQQeh+e6sOeK27n4SIQWViMI7x9eIkvjeMw19ObN9c/m0fkMDUOagkSpq+AMuLi957ACeMaIvcwPcBcV0yh+IQiJW2ULCP/MrJ4xrRlQK97o3txHxKkDuTPcMEWewHVkkWw",
        categories = "transportation,maps",
        tags = "railway,metro,subway,underground,speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFrontTunnel,
    #[strum(props(
        svg = "eJxtjrEKhEAMRH9lsI+3WTfeCnv+ga3FdYtXXCNYiN/vrCBYLAkJZF4ySVve//h9mimia3V+54AAd0WEmzlrxvQq1Jhudh2gJio1SQ0la9rEtWER3/KwGLtntUNCjsWLodAe7giLA6lCgATsWzGKPCYeXe2Hnhqe2glE3Dol",
        categories = "transportation",
        tags = "railway,metro,subway,underground,high-speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFront,
    #[strum(props(
        svg = "eJx1zrEJwDAMBMBVHi8QJGysQvEGHiKQIk0gRfbHUmFXFnz1x8Prd/0P7jN1BlUPp6aHt02nvWYZAtlQAVFAAgnEJiWiDN5Tt2/sqcsGLq4zUA==",
        categories = "transportation,maps",
        tags = "railway,line",
        contributors = "danielbayley"
    ))]
    TrainTrack,
    #[strum(props(
        svg = "eJxtzU0Kg0AMhuGrfGQ/tIlWpjDjDTxEqdK4KIgM/tzeiBvF2b5PPhLG7pugXf/TFIkrwrhEEsLct0mPskYqCJZLqsNjH9Rh+CRFG6kpwax2ZbK3k7CgmPwd/h78doYZYg8RJy5jjc1e+sw9qq6yAeS3O34=",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9lyD2YWUNaIe3Zi9feJQoRRDxIsX/vrkpE6CFMdue9zffjo+I0uMMWqbJ3Y97Yasyt4A5pZiwB9IR4gey7ot/gxVOfTGlF65EmlZSjKV5qLOpAoEcgs/yc6+V2xsLBkQ5P0Qya/ORic2ewYV/4XcZW/suxwS98Ijd2",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash2,
    #[strum(props(
        svg = "eJxtzKEOgDAMBNBfueAb6EHGSMY0BjtPhphEkH0/nZlCNGnaexee6y249+Gc4Yr6IYaxnWLoD93gqi55goqCQvBYs62TUNSGyf0wD5cMWU4bEZYlmwFhJWBlNx/bXyEY",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash,
    #[strum(props(
        svg = "eJxVT0EKAjEM/Mqw90SzaW0X6oIP8APeynrYowfx/U5EKlJKkplMJmmP/txxP0/XClv22hMSjp/nWmXWioGI6UmS5s2YGYxs0SRJtBCYWfKLs8m0dId/ZS6Rjwq+sYGqUCyaQwEqOIbBRTMCp23/swatL7/1Yt/btLZDXLC2cYfNJF4+mDcVry7n",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TreeDeciduous,
    #[strum(props(
        svg = "eJx1jj0KhUAMhK8y2G/ey2YlCqtgZ6EXsBMsLBQsxPMbBQUFSTdf5icu/TpiKJKZFRwgEJKewfjbsSNTSetA+hCdic1hGN3r/SQ5cgO+ugEyKEnDHjIFhKcnOys4nT7KWbukjL9jaRmvva1leb85udEO0W4r7w==",
        categories = "nature",
        tags = "tree,pine,forest,park,nature",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    TreePine,
    #[strum(props(
        svg = "eJxtzr0KwzAMBOBXObJH1cWJf8ANdOvQrh26GTp46NCh+PnrNGAymFuEPjgpftI343Ue7lRQi0wXAwOtIbwE0Ba9LkWzpgYjx0X8g4cVLPQ5rPG09a2xtbqtwHaABgzF9GSqkr2YxNqr/4gbKe5GD865KxYhy3QEv0N98V2nGZSlXfsBkWg53A==",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere"
    ))]
    Trees,
    #[strum(props(
        svg = "eJxdjFEKgCAQRK+y7AXCLCxQL1OS/spCevvarRT6GQbmvbE5bAS5OBwRqkONUCTPtFN0qBaEGNIR6em5MuntwJ63Yt+C6dTaXC2P5kf38TPml+MjNTX8AnPyLDk=",
        categories = "account,brands,development",
        tags = "logo,brand",
        contributors = "bdbch,csandman,mittalyashu,ericfennis"
    ))]
    Trello,
    #[strum(props(
        svg = "eJxlizsKwCAQRK8yeICE3ZCYYuNtUgiiQtJ4e/FXiMUMPHhPYnDJWf8iBuv/71HMIA06thN3XwWGVkb24RtZSrpK2fryNPkZYRMeTQ==",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingDown,
    #[strum(props(
        svg = "eJxlyzEKgDAQRNGrDDmAOhtiLNbcxiIQkoA23t5VsBCbzxTztLdyllw39Jbrsa9OBBH0QwCDZbnHZBEwuqTjC5L+KGejj7fQf94XPbYeGA==",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingUp,
    #[strum(props(
        svg = "eJwdjDEKgDAUQ68S3H/s/y1WoXZ20Au4lTo4Onh/bCWQPAi89JT3xrUOhxl0LgaDa1FptPkqSoVr7YWTMHTazTEg0Fcuwgj9nz6M55DT2J35A37lE6Y=",
        categories = "shapes,maths",
        tags = "volume,controls,controller,tv remote,geometry,delta,ramp,slope,incline,increase",
        contributors = "danielbayley"
    ))]
    TriangleRight,
    #[strum(props(
        svg = "eJxFjDsKgDAQRK8ypN+Y/aBbxIDXsAtYpFCw8P7INsp0w3uv3v0ZONZ0CedFwU5ObF0gKDHSbI5ykoNt+24YhAfPP4jwSffU6hTR9gKUFxOz",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Triangle,
    #[strum(props(
        svg = "eJx1zzEPgjAQhuG/8oWds9+lvWsTZHFhcWVwIzowOhh/v6DCQCBNc8ub53LNc3iNeJyrq6F0UdKgkjD/MD0i1Kmzqm1Oc9c2a82MMnKTh2/OvNNHqI7claY1Ucx6+j1AUqolupRcS3FQlBeXnMAsnuDQIBrnqXtW3Fg/apVowoXiYvEIy9DO3j4Y7H8eFaHX2xp/AGz+Szs=",
        categories = "sports,gaming",
        tags = "prize,sports,winner,achievement,award",
        contributors = "karsa-mistmere"
    ))]
    Trophy,
    #[strum(props(
        svg = "eJxljsEKwjAQRH9l2Htitq5NC0nPXrx6l1WI0IMUKfr3ZhWs0svA7Mw8Nt1O94JzpsMO3O236nyL4Nh5qcLHqKFezKDawkEtB8NLFZ6ZaUgbgwzpi2JBX2QUyCwa4FtnbUMU1yx9vU46XqDPTNwRpkwNQR+ZonU+6S/1/WG/3tcFR/rnLIAX2Zw6sQ==",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley"
    ))]
    Truck,
    #[strum(props(
        svg = "eJyNj7EOwjAMRH/l1D3B55iSSqFzF36ALRJDBpAYUL+fENSqQ4fKi6179p3TO38KHtfuRQUFCptDJghpVbui29lxdiFHxCaKYw85sPFUZ8Xu3ZhOP8cxLb4381Fx8UO9SNnRefaMDWgJ94jeD6FKk0rW+sE/hcAmXekv8GQ2BA==",
        categories = "animals",
        tags = "animal,pet,tortoise,slow,speed",
        contributors = ""
    ))]
    Turtle,
    #[strum(props(
        svg = "eJwly8EJgDAMRuFVwr+AbRW8NNnAIcQW05uUgLq9hp4ePPjytZtSYWwrpagxQPLkT3Kvh9HdiikjBZDWdqox4gJ6/gXqIy9jduZAPr0lFsk=",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[strum(props(
        svg = "eJw1i1EKgCAQRK8y7AVKIfxRL1OSgqjYQnn7NiHmY+DNG9vDzhiODOFOB0dHeiX0R5oQQzojO1KboDHRHLxdvp+3reaRUwloNRW+xDTQUBpGMr3f8C9d3x6w",
        categories = "devices,multimedia,communication",
        tags = "television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,entertainment,showtime,channels,terrestrial,satellite,cable,broadcast,live,frequency,tune,scan,aerial,receiver,transmission,signal,connection,connectivity",
        contributors = "colebemis,ericfennis"
    ))]
    Tv,
    #[strum(props(
        svg = "eJwBOwDE/zxwYXRoIGQ9Ik0yMSAySDN2MTZoNXY0bDQtNGg1bDQtNFYyem0tMTAgOVY3bTUgNFY3Ij48L3BhdGg+5+oQKQ==",
        categories = "brands,social,account,gaming",
        tags = "logo,social",
        contributors = "ahtohbi4,johnletey"
    ))]
    Twitch,
    #[strum(props(
        svg = "eJwdjsEKwkAMRH9l6D1jk83uulB78exHSD14FPTk13daQgJ5eQxZPs/fG6/b9IhAfo0dQbdAYW7OBp9tMOGdxfwKP1hQDpJpWprFvcArVRgCBXWTIauhqlPugNxhKZzWBLsVKk2nGQoWP+d/WpfL8dK6A3M+Hxk=",
        categories = "brands,social,account",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Twitter,
    #[strum(props(
        svg = "eJxNzGEKgCAMhuGrjO8C6TAkUG/TD0FUqB96+4ZWyGAvjIe5WlJPMZ9US8z35WHIkpFhNZdFcNunghu2sYfeQU17HKAuYSXlUfGLfY/Dap7YYL7g3z4bbCQZ",
        categories = "text",
        tags = "text,font,typography",
        contributors = "colebemis,ericfennis"
    ))]
    Type,
    #[strum(props(
        svg = "eJxtyjEKgDAQRNGrDOnV3UHEYs0NvIDdgoWlheT8ZiGkCgO/mWevfw/uI50klK4yy4ZWCGRizZWyLSGzda/hy+4Ew9WtkLFj0X78GnkcUw==",
        categories = "weather",
        tags = "rain,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Umbrella,
    #[strum(props(
        svg = "eJwtykEKgDAMRNGrDLmAaShZpbmBW/cFBQURFyL29ralzOIz8OzOz4410ayIr2aFgvuCgJdIblMjbudxbSiSSJjwjZYwfm23TfkPl8oVOQ==",
        categories = "text",
        tags = "text,format",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Underline,
    #[strum(props(
        svg = "eJxVjDEKwCAUQ68S3LX/gxmEX+cuPYTQwaFDh9LzV0EECRleHsSe8lZcuzsTNCIi3fR02bYusk3dTFUJLAxEr7QoBn2y7n7QoTq/fpJgGUc=",
        categories = "text,arrows",
        tags = "redo,rerun,history,back,return,reverse,revert,direction,u-turn",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Undo2,
    #[strum(props(
        svg = "eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0ulJSEKpRkhXNUPBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtFyR/X",
        categories = "text,arrows",
        tags = "redo,history,step,back",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UndoDot,
    #[strum(props(
        svg = "eJw9izEKwCAQBL+y2JvkPDg5uPiC5BFCCssU4vvlCmWb3R3G/tobvju8jDykSSh2+ldsk0SgXBWKyxM1el9LkA5+GMRbnbn2FMg=",
        categories = "text,arrows",
        tags = "redo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Undo,
    #[strum(props(
        svg = "eJx1yTEOgCAQBdGr/NAT3UUIJEht4yFMLGhMLIznVyiodjPdvHwfT8W5mp0CiGswJU/tlTwk/rCxAMTgV4GoAS2a8CzJRQnk4WxPcI9kHXpDP5hnO50=",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldHorizontal,
    #[strum(props(
        svg = "eJx1yzEKgDAQRNGrLOkX3YkJEWJqG1t7wSKNYCGe38RGiw3TzePHc7sy7ZNZBATc7E2KXT1T/FNYocBAglkD6asETXyRzFoEtOgQRzKyJctlujt+kT5+AG84PEE=",
        categories = "arrows,layout",
        tags = "arrow,expand,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldVertical,
    #[strum(props(
        svg = "eJxNzFEKgDAMA9CrlFxACip+tLuMDrvfUVBvb6eI+0zIi9S8Olkuu7liBh1lc1MsoFMxgS7FCKoRGEmGNk/yoLdrM+beheAg/+fHbp59Hp8=",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,separate",
        contributors = "danielbayley"
    ))]
    Ungroup,
    #[strum(props(
        svg = "eJxFizEKACEMBL+y2MtFIZfGE66z8RGCRRrBwv+jqWTZambSbEvRP1cDQzQ2BoPOgp3Ux+FfUJH/AjHR5fRYmzcFyA8+",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink2,
    #[strum(props(
        svg = "eJxtjdEKgzAMRX8l9L1ZE6xVUP9F2MCBG3vYQ/37JU23IUoggeTcnOE1vxe4ju5BHXYNECNHIEzspdHiMfAcMYQGrActj8Q+YUhl2dZupxb7CGHVOOsjctNwUcs0/FwRKQHJMRZLwY4WEAucW6BaNOj3kvX+vEHm0XUOMpWxyWAZsowKKlLBrRJ/XsB8AuqJWvtFvSW4srI/fi2wHS3cGyyhL/wBo89XdQ==",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[strum(props(
        svg = "eJwli0sKgCAURbdyefM+rxAT1B00bS4p6SxEqHafFnd27jk6h70gP4YmwpV8iYZ4IVTATMj3d8SQjlh+VMlMVg8ttPp0JcIbWiWYN+kEBMY6hupVx01sin0B20MbeA==",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Unlock,
    #[strum(props(
        svg = "eJxtjsEKwkAMRH9l6H3XTdJGhbVfoD/greDBQwUP/j8mOSilYdglZB6Z6e/l88TjMrzojAlSZJj7wZdz/1kM5ty6aRVwq7JwHeGvhcSnKzHotBY1sXM7LMB7cvZYJ5D410CUANTc1MBAY1LawjVksSWJLha9muHddGuWqP/v9QWv+US4",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet,disconnect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Unplug,
    #[strum(props(
        svg = "eJxljDsKgDAQRK8ypHfNLpsfxIAH8BABizSChXh+jUUaeQwMPHj5rFfDvphNwUoxpTUgwII7jgIjNqaQqpJDn/2kvC+SqJiS594oeZRYwHKnvznYg/2kHejwD/RqH/g=",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UploadCloud,
    #[strum(props(
        svg = "eJxNy7EKgDAMBNBfObIXTbTo0HZ28SMEBQXRgiL2743FQTLcHby4OJwzRk+9MNhe9SAQlHpstHX2v41cpqbgivcpuLivaV22CXFftvPwxA1asKCCluw+EVx2iT1VhCRKLeHWyaIpOdVn+wDsCSZO",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[strum(props(
        svg = "eJxtjU0KwjAQha/ymH1jZpI0LSS9QS/gTqKgUEGKC729E3+o0C5m3uL7Zl4ql7lMJ8yZmFAeGlbzmSnSkHYfOqSvVbmnn6yS2H/rdrifccw0ehPBvXG6EKpRycKvwnCNUghkv+Zjb6RFNG2HAJZJNp+wBXsVBM6ERmfL6fQe3LB2vffS9gJCQEHD",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[strum(props(
        svg = "eJwlyjEKgDAMBdCrfP4uJgWlQ5MbeIgSBQUHKQ56eynO75U4Wpwb4jFqIppxIuI1ZnoZf/Vy1XvHalySIGnNyBAoZNAZ0mMP/gFAFhRe",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    User2,
    #[strum(props(
        svg = "eJwtzEEKgCAQheGrPGYfNRaS4HiDDhEWJEhJucjbhxVv+T5+m+a8YRGaeACbWUOjq2tYoSNn2wqc9eH0cYW/hUaCL0KGcAoNlXyns+mIJYZ9RTrCni8h1mAGj+AeSsG8vR+5BwVOIkA=",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserCheck2,
    #[strum(props(
        svg = "eJxNjcEKwjAQRH9l2HvQXZdqIZuzFz+ixIKB0IZaiv17je2hzGmYNzO+dPMLT6MHNxBenHQKxbnKqdN7c/TQRSj4Uy0FH9MUc4+4Gl0J8WPUEiYjrcgWBl/GvOY09ChjGua30e+HGXwDXyCC9r+3Q+ELWKgmoQ==",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[strum(props(
        svg = "eJxNyjEKgDAQRNGrDNuLu0GCRZIbeIiwCgoWEiz09rpGUKaY4r+w5X3GGGmQHo6zhwfbGnFgSqE1kIIuRdcJekYSJuhxvyOUSJ2hmj/25odXJvxzFzGuH70=",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle2,
    #[strum(props(
        svg = "eJxNzEEKgCAQBdCrDO4r/YERmDdo214sMGgRElG3b7QImcVn+G/G+DX6baE4CCUF+YsTnHdOa5q3t+ZzuWDIvv15wXZ3BJoHMXYEWWuNSfUOBJI8ilAh6HInnCqx9CPd2gc66iVr",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle,
    #[strum(props(
        svg = "eJx9z8EKwjAMBuBXCb3vd2nWboV14AP4EFIFBQUZHvTtbezAHTrpIYd8/ZOM6Tqn25nmaMRQekXDQ67vXJ2Zxl1pT+PCFCz9YPRTt0aP4/NCp2gO3MIRC9zek6f2+yxxUKzoR++W0RN7dA1CA6kIdrAaFraFhyce0EOyqYEAJrawf0BJaPIiXB8xaIJsgg5CZZFMaoe2eqhozBp8APMuZLc=",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserCog2,
    #[strum(props(
        svg = "eJx9z8EKwjAMBuBXCbk3Lk3XbdDu7MWHGFVQUJAhom9vsyLu0EkvLfn6JwnpMqfrCdIrIvcIc0RBSO/8anEMu1Iew5flQrcoh8ufYY3u0+MMx4gHboDbvZ8cOGj0mHx7WrVqfvJmmTpgT87QYEgqgluywKLlLeHJA/fUkWRTAwMxsCX7B5QEkwfheoteE2QTOBIog2RSW7TRRUVj1uADNhhlDQ==",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[strum(props(
        svg = "eJwlykEKgCAQheGrPGYfNRJS4HiDDhEmJEiEtNDb5yCz+GHe597zu3EJHbyC99PCYtGb2GAh72YF3oVUQo4oQishVKGtpwntSsboXU5PRDNCzITGo7XXmF79W+XK/A99lh9i",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserMinus2,
    #[strum(props(
        svg = "eJxNi8EKwjAQRH9lmHvQXZaIkM3Zix9RYsFCESlS0r83aS9lDo9h3qTv8Hvj5XxKhMoadDAYrj3Bgj3iucNWZU6XfsqpTEuZRyxOI0p13hs2560rx5jTPH1GVHGqElWdEomtdZFG3dn0ruU/yIMjww==",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[strum(props(
        svg = "eJw9y1EKgzAQBNCrDPNf6oYQKmRzgx5CUqGClCL9SG7frFHZj2WYN/E7/d54KZ/iIeMUEDDY3cRhYIp3AynmZcvrjFyUDyJX5UhsSm+klymuy2dGdUrxRJH2G6qyT4rbY9OmTmtGunWOfSsHDhf+A+YTKnI=",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserPlus2,
    #[strum(props(
        svg = "eJxNi0EKwjAQRa8y/H3QGYaokMnajYcosWChiBQpye1NTAtlFp/hvRc+w/dFT8ODPQmvTgYlpXM7p07v/viTroIYTi2KIU1LmkdK2XADpWK4gBaDNqXDGObpPVIRAyuosOEKynW4Jln+W+1mbW6DIhv06C33tu4u/wDx8S7T",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[strum(props(
        svg = "eJwli0EKgDAMBL+y5C6aKuKh7Q98hMRiCx6kFNTfm1YCWdidsddWInZHKy8wvM2YMdTr2GAgb/sKeCspyxkgryNmgjyahpAdTRX6Z29zkAJlRsLT/p32EpVdlNVGlRjSEUurVKyC/wArISQH",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare2,
    #[strum(props(
        svg = "eJxNizsKwzAQRK8ybG/i3UCSQtINcggji6wghRHCn9t7JTdmimF481xJsUJT/mn1xB/C7ulJKFZCOPrY8ly10+AeTQgu5hL/CaXzaDcerU1iaacLB7dMVTF7+r4hvA4yCQSjhSGD6Ou+IWt3mxNOlH4oMg==",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare,
    #[strum(props(
        svg = "eJxdy1EKg0AMBNCrDPNfarZiK2z2Bj2EbIUKIiJ+uLc3UfyRfAxh3sS5W//4Kb9SQ9quQYPK7yEBFVN8OkgxD0see+RN+SFyUbbEoqydnGWK4zD1KEEpL2KzDMFS7H8TRXxq2tXNnt0xceoTm152B7uaKkI=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserX2,
    #[strum(props(
        svg = "eJxNzMEKwjAQBNBfWeYedNelKmRz9uJHlFiwUESKlOTvTdoGyp6GfTP+2//e9DI8uSPhxUmvpHSu59Tpoztm0kUQ/KmWgo/jHKeBYjZcQbNBQTEZ7pVsz+Cn8TNQZsMNlMXAF1AqkUsllSzrYFXNNiQ74hXtG83+AcGfLqM=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[strum(props(
        svg = "eJxNizEKgDAQBL+yXB804UACudQ2PiKcgoKFBAn6e402YYtl2ZlwpHPFLDRZD2eLcYnB6GsMGx59u8HFUQxdlWLQLeu+IAsxQW+h4a1LyH7M/8YHi0MYxQ==",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[strum(props(
        svg = "eJxVjEEKgCAQRa/ymb2kIqLgeIMOIVNQ0CKkRd2+LBDirT7/8dJejgUT02gcTCweHrqhjIWmnIYm5CRrlW2GXEyRUJkcQU6m0JTvzKm3rP23vPJwDxrm3aGHb0WgH+I=",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    Users2,
    #[strum(props(
        svg = "eJxljjEKgDAMRa8SsrfaNFqFtrOLh5AqKDiIiOjttQhVkJAh5P3Ht0u3jdA7bFUJpHZBHQNDHkew4Kb83sA7obdZDHkbpjXMA4TDYY0QTocGYXXIEXme3iY/0c+vhZaVeYXfLloqnVB1r5GmSOgFeZotjA==",
        categories = "account",
        tags = "group,people",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Users,
    #[strum(props(
        svg = "eJxtTjEKwzAM/MqR3aotJVYKbl6QfqCbaYcOMXTo/6kUUggkSKfhdHdS+dTvG69b11IGByaBoQoEcaueeEk0wrDjjUWcmTF2U7l4ylT+Wfc0wFoszFBdusq3yovaxvAkJQXDhx2I8+p7NBcp9BjcmBLYn8nUh0xyIklXDGHv/gFfZjQ5",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere"
    ))]
    UtensilsCrossed,
    #[strum(props(
        svg = "eJxtjbEKgDAQQ3/lcG+9u1qLUPsHrh3cig5dBAe577dV6CCSIYHwEn+mK8M+d4sBFrchkCY9AVflIVXHR6w4chd8X4ngG+cKx/hTMAHZyILJgn1HVEkyfk7MepRKXJu4AbKYJM8=",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Utensils,
    #[strum(props(
        svg = "eJx1yTEKgDAQRNGrDOmD7oSwCGtu4CEEizSCheT8mibVht/9Z8/5Vlx7OIRg4xqKLf0VG0Lk6kJCanS+Tr7MgOLDLRtyVGj8G/wBPT8zCw==",
        categories = "buildings,home,sustainability",
        tags = "electricity,energy,transmission line,telegraph pole,power lines",
        contributors = ""
    ))]
    UtilityPole,
    #[strum(props(
        svg = "eJxdzMsJgDAQRdFWHrMPMkkUA4kdWISgoCAi6EK7Nz8juLgMPA5j9+GcMTrqW0g+hBbKZ5CjzlZBdLY4bqAODQUNI3IfW5dtwsWOuCZc0pEh3JyOjKunAb00jT8TP5hCH2ZDKVg=",
        categories = "development,maths",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[strum(props(
        svg = "eJxtTTkOgDAM+4rFntCkJYBU+AGPqMrAyMD/RQCpE7JsefCRz3Id2JduU2hRY8MrwSGQAA2VZzKeFMIDzTwgkaRuzf1TXXMbEMNUk/eM1GnknnzOaX/xkZMgshV/ke8vICK27A3wxCPm",
        categories = "food-beverage,sustainability",
        tags = "vegetarian,fruitarian,herbivorous,animal rights,diet",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Vegan,
    #[strum(props(
        svg = "eJxtjaEOgDAMRH+lwR+sXQaIgcZgEbgFxCSC/w/tEsgEuXS7Jveu8Up3pnNqViGWFCiQKzI3qsyzeqk3CFVJhG3ICOkNONRx3SCL7M0cOzs2x+9kT8wHt1bkST9Pos0O3ga/CI/GoEAwSB9RWYNODT2Cby7u",
        categories = "account,gaming",
        tags = "mask,masquerade,impersonate,secret,incognito",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    VenetianMask,
    #[strum(props(
        svg = "eJxti8EKwjAQRH9lyH1jdk1CDmn+wKt3iUIEFQ9S2r93Y5H2UOYw7Ox7+X35NFwH8xQkCIQE8m9T8qH/S14pxbYIdrFTQhrZVQcbgvUB3NNi1RMOTL+NeKQ9mSPY2aM/x+pIjU4T9zTyuq/K4/66YZbBiBhMrG0wLzUtq6IdKl92fTg9",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[strum(props(
        svg = "eJxVy0EKgCAQheGrPGYf4VDgQr1LpDQugpCB6vZpENXqLf7vuW1SQfS0MiwY3DH4WQqubz24V1X2JfixkmbF4ckSTk8jQVJeRD2ZgbDnqHK3Uolpr+bDBWUCIcw=",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[strum(props(
        svg = "eJxNjLEKwzAQQ39FeHfqU83hwfGcoV07dAu0kIIpHUpo/r6XEHDQ8CSQlD/jd8Kjd1cJnSp0kDgSRDCJkTO7c6wCuZBIc3Iln9ZRyW2qEG0rb26Ix+x5S8dXz4lVzIZ7u6uv9xMLe0c6/HYuYrS4waprqfwBCkEqnw==",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[strum(props(
        svg = "eJwlizEKgDAQBL9ybC9ikJAil2dY2IkJXgpBwoHm9yZabLEzjL82FYqM0xhyg6WZ2ha3Ivixu+BL2pXKwzCgO0cVxjSDflAZFlTqdyTlQ7Rp0+vehRfsqRot",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[strum(props(
        svg = "eJxljkEKAjEQBL/S9Ac0QSSHJD/wETIbnIAHCQH1907iwi7sZQ7TXUXHVqTjXZeuif5MaKkP7YnuSnwTL0T7WELMm+NpADm+7l2xJN48ghpnwXjlKLXJs0CsHggxhRuOFf6nOzzAeQ0HfPanZOxYLRv/A47bM7o=",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = ""
    ))]
    Videotape,
    #[strum(props(
        svg = "eJxtjLEOwkAMQ3/F6p5yMRduKZ27sLKfysCIVHQDX09SJApSFVmKk2cPj/q843buLgblwt6yiaGIzbnPlpFQYKFF4nmCiZtZ4mtI4mTo1Y3DIbrG4duohB6rQr0klMQPn20d7oXoRGmsBFdIPcTJfr2wCfej5bqRKcjpz4NtC74BR305uQ==",
        categories = "design,photography",
        tags = "eye,look",
        contributors = "zenoamaro,ericfennis,csandman,karsa-mistmere"
    ))]
    View,
    #[strum(props(
        svg = "eJxVjDEKwDAMA79i9IHiUEKHxJ8xGQKhQyb7901xC+0k0J1UtE8djdQrOIFmxQ5Sq8iQsgWV8lqr5wN/+6ONfjbytGAGOUdaipHxc3pbcgEgZSEA",
        categories = "connectivity,devices,social",
        tags = "phone,cassette,tape,reel,recording,audio",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Voicemail,
    #[strum(props(
        svg = "eJwtTFEKgCAUu8rDA5gvfJqg3qBDCEEFoUL+dPueImODjW2+luc7S4Za7tzeIBCBwICDdRC7YeEYXVcS0S9zFH1N7YIjiB1JkoZNapOIDxQDmVYqOwbciz/+Exru",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Volume1,
    #[strum(props(
        svg = "eJxNjFEKhDAMRK8SeoBsZzfRLVRv4CEKggpiC/rj7Y0iImEGMryZWPK8D3mhkqdlWxsHkFJFgb6XcD5mFiOcrq6Nn7vUxpK2kfrGdVBWoT9LldQGvB1MNfv6Khj3poPlJBx+CQb6h4cw5CkcR2kmRg==",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Volume2,
    #[strum(props(
        svg = "eJxNjFEKwCAMQ68SeoFRYYKg3mYMQVTYPvT2a92EfTQl7Ut8q3mctaDVVO4rEDN2WDiYOaxGRM7sVHeKfvtC0edUDnQOZAyhG4lbwhDvZKmduFI/djLvc7EalYrFPnmeJiw=",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    VolumeX,
    #[strum(props(
        svg = "eJwti1EKABAQRK8yuYBWUQrXkZJV/Li9XfmYV/Pqpcn9VB6Y3MZe2RDBIyDCvZEegWiKSm9Ksj8qFxX0EBo=",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis"
    ))]
    Volume,
    #[strum(props(
        svg = "eJxtiyEPgCAQRv/KzX7Id4MxNiRTrAYb00BxMzh/v1zQ5F5876WzXo32aTgiQUhx7IacRhU5vXr2FDbLMDCRe8XSYKvmtgMdb0jxS1h/ZhFCLPKZB93vHd4=",
        categories = "social",
        tags = "vote,poll,ballot,political,social,check,tick",
        contributors = "ptrgast,karsa-mistmere"
    ))]
    Vote,
    #[strum(props(
        svg = "eJxNTKsOgDAM/JVmfrBrttSUaQwWvwRRiSB8P8Vsy4l75U7v9hhdWzgghGxLQqi6/mnV3gmJgRsTU3LAmV+kEURXe5l95LPMg8iG3K8/zgUbpA==",
        categories = "account,money",
        tags = "finance,pocket",
        contributors = "danielbayley"
    ))]
    Wallet2,
    #[strum(props(
        svg = "eJxVjcEKwkAMRH9l2Htjk221hd2evfgRZVvcggcpC+rfm+xBLIEMmZlHwr6mgte2lBwdDw7v6LzDriIOn3rkdbvnUuMpnAyYwnMuGUt0N49xFghaHYY0krn7NyAGWf0AMWefaLAOnclDiGl8sO2kjrnoiKv2dEFrWaMh9Q31ans9jFK5Cv9+fAF2zTGw",
        categories = "account,money",
        tags = "money,finance,pocket,credit,purchase,payment,shopping,retail,consumer,cc",
        contributors = "danielbayley"
    ))]
    WalletCards,
    #[strum(props(
        svg = "eJxtzKEOgDAMhOFXafALbbkBYkxjsAjcEkQlgvT5GWZBkBNnvvzpKrfRuXSbConu0xqLkhLXCXGACRxdTv0Lc2p8oOiCZrm+mowe4g+WubY/lgkGDziafQAcGSIR",
        categories = "account,money",
        tags = "money,finance,pocket",
        contributors = "mittalyashu,ahtohbi4,ericfennis"
    ))]
    Wallet,
    #[strum(props(
        svg = "eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2RoW2h7Isw8x3+EK6jul2QXpF8YL0jtIJxiiUPuz+sA+P4ZlxjnLvoC0aq6b8QBCunILWq3V6IKH1qZ6JMzTcV+sOTuqWoSSzNuvB/NRf5SI+elCz3wBalO1UzeQDsgEytQ==",
        categories = "account,devices",
        tags = "cover,lock screen",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wallpaper,
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/lyB7Xkh3bATdzhvYHugU6ZGihQ8n3V3JJCIkRHEZPvpPyZ/rOeF6bN5MJHk6kJcOpyCRCKGK1pBkZ9sbGBVCS2eOElM689DdWC33soBqUsE5sDvwf8WiGfNHNhrztRx4RDu6M7h3C4it96kG+Tix44QqISGMtQo8Mc1v1Irix38APsIlS9A==",
        categories = "design,gaming,cursors,photography",
        tags = "magic,wizard",
        contributors = "karsa-mistmere"
    ))]
    Wand2,
    #[strum(props(
        svg = "eJx1zTEOgCAMheGrNOwgLYI0QW7g6m7iwGLiYDy/4kRiWTq8P1+azu0qsM9qQQ/jSiqnoU45tQHDraUUgYu0k+0EnEwExHoY0MnfuNgeDYaq9P9+OCAE1ixRetlHsaEPPOZELw==",
        categories = "design,gaming,cursors,photography",
        tags = "magic,selection",
        contributors = "mittalyashu,ericfennis"
    ))]
    Wand,
    #[strum(props(
        svg = "eJxtjjELwjAQhf/KI3tick1DhiTg5uLawa3YYgodRALqvzeHUlsoN9zde9/xLtz7kjFEcSaCV03bke4JBF3LyDqd7HqX1DF1XCQ0ihycamcv6/hnYZT10LNnYsX/ci4ihQOHp7C84GB8NrTv2I3zGK8Fz2koOYqq4xWFE8jjdMvlq7xr03zAaPoAlcQ30w==",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[strum(props(
        svg = "eJxNjk0OgyAQha/ywp6pjO3AQr2LoSaaUDXWRb19B42tYULC+/mYKg5LTB3ipzaODZbaiEHc9ldT3Q67qeYpbWkYO8zTMK7vbMMVyLdOqZPTZ0rz7drjWZuXE1Lbk4il4OydikfLYBT5WLaOxPWWScJVRpaTJR+QGztbiReupxCg8FJIM/8ujm7P5Pmq7h+lc4Uf8AtuKT54",
        categories = "time",
        tags = "clock,time",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Watch,
    #[strum(props(
        svg = "eJy1jT0KgDAMRq/y0d1ooq0KtYuzh5A6ODp4f0w6CV2VEMjL34vXfp84FrcJQqZAHkwChli1jrDwmJV8FgrodDJAtCdlx9g3yppMvTKTbZd7l2JrghRfGpbKU336yDT9aXoAe60/UA==",
        categories = "weather,maps,multimedia",
        tags = "water,sea,sound,hertz,wavelength,vibrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Waves,
    #[strum(props(
        svg = "eJxtyjsKgDAQhOGrLNuLZhW0yOYGHkJWIYKFBBG9vSM+sLAamP/zNiabBkrKDZPtyq7Abljh4PMrB3+zB4CXf2zulki9cluTSIREO79PcYK0ZtWbDpSVJPo=",
        categories = "connectivity,devices,communication",
        tags = "camera,security",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Webcam,
    #[strum(props(
        svg = "eJxtz0sKwlAMBdCthM5zfXn/QC2IYxdR6qATwYH7x9uKOimBQD4ckvE5v1a5n4ebdbEK76sWuC9qMAnMzjJrROYcfsmSJTBMolhbEEzRELmYUZrGYRpPmzmNX/lRuSgJlii3vqAkhTcYUeuKopzNH9cYFd01I9QDyqLUnRJS6WoFlXpE228X9rc30vy/Mkj/OW9lxjQU",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere"
    ))]
    Webhook,
    #[strum(props(
        svg = "eJx1kstOxDAMRX/F6j4mdh5tpJn5gs6WxeyqglSkglggNPw91y2Ux6SqYqfJzY1zksPr8DbRw7F5VlIl8U58czrc2fDpsE1Kps4JS0sWbgXnwBFzyilQIpFZrGdhCJzImscnaJFL7KEps7N5VxM5E10qdXQoA/r/S/y3byFJ82Y7ckqQJ+6seE1cLJda/eK5CCXWPLJgkWYOEWthkx23oZdAYfdUrB32z1oxVk866bsOkeKqduhNTu/zzxBFVzvuWWTB2i4xbMwqJSzEyK9g97iumkv9fiXXyG7Gf8lCEwstYbU2rA58a4fouI0on30ZwdWBL7g68AVX7Jl7FXsyO8/BHkNW3Mgv7/np5ZGuemxUG/r4yldBxu+SIDXR6RPD4qX0",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WheatOff,
    #[strum(props(
        svg = "eJydkDEOgzAMRa/yxZ4UOw4QiXKCdu3QDakDA0OHqudvnFR0SQGhyJZlf/k5v3+OrwmPc3VlMIMadNXQn7Q79MvMWWlBbL2DB9FMWmkanfXQqOOjGGKDXKImzEbnpiQyKroXMK1iOt0d0G5BAsgfgRApRdJyB7eFiZr44QMcrsETv3kUSBabWE2Gb82vBTErR1IyRC/42lm4M/FRZ8//XZk1RZDPiIxbLF0BJd8PgIIiUmJaLF3hZOP3gj5MWZvt",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten",
        contributors = "karsa-mistmere"
    ))]
    Wheat,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l2HtqJrVGIcnZix8hq6DgQYoE+/dNKKQUetlZ5vEm6HvUzxNjlF6gUxS6kv8oXlI4LDSF7/33wiPKjRaXfKqoVils9aLRt5ld/wifz6vfgHOgz1SLbjDdABqC117rbw1LlMNs2NwZWN0ypA==",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[strum(props(
        svg = "eJxtkEEKwkAMRa8SZt+YZCbpCG1v4CEGFRREXLhob2+mtVKhZBEI7/PD6x735xVG6YNIgIl9BxiXNS3XoTtUaOhe5X2DSx9OGRXYUIuCAvkwtEAVrMgGFMiYpbDjK5iQ20bQdAdnQjPQc0LiBqN5mhMegRljgoit7YUMs1ZEtLA30LdIUAQYLe9knI9/sKIk/2pbMIuZFVCYBbH4W4scXl3Rz84H4GdOWA==",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[strum(props(
        svg = "eJxtysEJhTAQBNBWhi0gf3e/0RxiOrCIgIKCiAcP2r1ZUPEgMzAMvLjmbUTfUuch/yyMUotAKjCl+DOR4uOCK7J2Pnv4SzafUBFc0CyF31D5LedpGXBIS8qEvawo4dDrq33HYt5kOgHWYCki",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis"
    ))]
    Wifi,
    #[strum(props(
        svg = "eJxtyz0KgDAMhuGrfPQA0cTaH6gFNxcPUXBwdPD+mAwWBwnvlOcrV7tPHIvbOVKE1oRmWCPYjhI8TZu4WgbDtfRJpqC/sArk1Yz0S1nUcibfsebB4aMfD3kgyQ==",
        categories = "weather",
        tags = "weather,air,blow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Wind,
    #[strum(props(
        svg = "eJxtT9sKAkEI/ZXDvK+NOhcHtoXouY+ICiao6CGi/r7Z7UIPi3j06FG0v25vFful2xhEqrmhX4ylof81MthXPWf42jFp0BkNCzje89w0qc+YcMVCqnijn2zcvY2IX0paGGHEjVHSdhMV47VRji1NyWBkWVquAQVS044pIkCQmlvbQcHnjnwk86UjDhFMXP4eOx0vBzxk6UQcHtyiw/NDnxNt0lE0vADRLkKh",
        categories = "food-beverage",
        tags = "alcohol,beverage,drink,glass,alcohol free,abstinence,abstaining,teetotalism,allergy,intolerance",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WineOff,
    #[strum(props(
        svg = "eJx1jLEOgCAQQ3+lYT/kiCeYILOLP+BGcGB0MH6/6EAcNJdL076mYU9HwTapxcPa4lUM3R3F0IADm8Lmg7AFy+n+SBIIzHNCkg1Z0kJ9FT+PmVgLqsFQ37+7kLVNXnRpKBY=",
        categories = "food-beverage",
        tags = "alcohol,beverage,bar,drink,glass,sommelier,vineyard,winery",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wine,
    #[strum(props(
        svg = "eJxNjUsOgCAMRK/S9AIKkugCuIGHIEIsO0OI4u2l+InpZpqZvKdTWDKkYlAinAYHhCP6TAYnBApxpdxi4crqjudWby4TeIPzCELsykmQ0LeriRQPeWJ1o/8w1SCq4vGV+/uEL/8CFwMo6g==",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[strum(props(
        svg = "eJw9jl0KhDAMhK8y5H3ZJnWLD21vsIcQdqGCqKAPenuT+kMCyYSPzMShH//YJJEwYedEgbDp8KrEVI5vY3Kcu7Xgl+jrwVL403l4OLCWQyivxlCDFJ2GvT6ep35cl0QcYN2AW1vEVfaicqys2XH7uFsmdmcmPd8xDhhtLoA=",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis"
    ))]
    WrapText,
    #[strum(props(
        svg = "eJxNjrEKgDAMRH/lcE80tqQItX/gRxQcHDo4OPn1Nq2gBF7gchcunvk6sK/DJp4DlF0WCKZ3hH0R1rr1p1cVU3EcAhmyQrtOgRcPQyHlRWDIM8uMhm5y5IodmuULtxz1cH2rMNxDiqN1TA/GsCOZ",
        categories = "account,development,tools",
        tags = "account,tool,settings,spanner",
        contributors = "Andreto,ericfennis,csandman"
    ))]
    Wrench,
    #[strum(props(
        svg = "eJxNyTsKgDAQRdGtPKYXjWBgYJK9yCgoKEiwSHafX5Pqwj2id9DnRHBkFoLG0rU0tXqZu3v59v/C4eg1G3iysBXrHIjBGCUDxpwZzw==",
        categories = "maths,shapes,development",
        tags = "cancel,close,delete,remove,times,clear,error,incorrect,wrong,mistake,failure,linter,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XCircle,
    #[strum(props(
        svg = "eJxNzEEKgCAQheGrPNxXKDUlqHcJggpKhdx0+0QNXQ18//CUd9e7OwvvThsezeZ+IQhw6vkYrxDI8lMJletvkszMqKEsG+XXcGDT7OYTZEegVCM2SUKiLR8PxSW3",
        categories = "maths,shapes,notifications",
        tags = "delete,stop,alert,warning,times,clear,maths",
        contributors = "colebemis,ericfennis"
    ))]
    XOctagon,
    #[strum(props(
        svg = "eJxNy0EKgCAQRuGr/Mw+oiJJcLxLpDQugpCB9PahK7fv47kcL0WuTCuhMG0EiekWZVoOQi4daocvBZXevZvb5917qiAwPcsOOxmYRi0OZGExyg9CXx/V",
        categories = "maths,shapes,notifications",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XSquare,
    #[strum(props(
        svg = "eJw9ybEJACAMRNFVQhaQWEiKMxs4hGBhI1i4P0ZBuav+w6yrU8tcRCn5RNkQTjU8Gxei/9sGq2sO7w==",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    X,
    #[strum(props(
        svg = "eJxtjkEKgDAMBL8SvDc2tVGEKvgAH1Hw4EXw4MnXG2uqghKykOwybFjjNsPUFaNDBmqi80gOLrUyBNaQBXnpSeiNLPgWuVZVp0YJDU/UkTB/kfBGJlxGsqo6H2RqWfShPIv3IddfBEgMbCoju9+BAwNZLQw=",
        categories = "account,multimedia,social,text,brands",
        tags = "logo,social,video,play",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Youtube,
    #[strum(props(
        svg = "eJyFy8EKgzAQBNBfGfIB2+6axAjRv+lBEBXaQ/P3jkEP4sHL7gzMy+sylWmcP1iXcf59e6cmXhGlDdAGBn1LaOGlMzfk1zkf8h2mfUjeKUzpoEFiZHiACQkN1NPWS85AGK+womK9M3P4Kz/fUUutnNfpBkv1OoI=",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[strum(props(
        svg = "eJwti7EJACEQBFtZbOB/1w992xFBPEETu1fEZIJhJjQrM1lFs1xH/x09BA9+oA4JCSL4HrO5CxfDc8+4AF5nEOw=",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis"
    ))]
    Zap,
    #[strum(props(
        svg = "eJxdi0EKwCAMBL8S8oCWlFY8qJ8JHgTpwZP5fVODUnpa2JkJXBrXDCwRiRC427aIHlPYDadQy51BDoVucxeCUMRDvT53IY1e+Zuc5tH0yHr/c/v4BlqqpmLJdB9CkC11",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[strum(props(
        svg = "eJxNzTsKwCAMBuCrhBygJdKKg3qZ4CBIB6d4+8YH4pTX9xPPuXJJwBKQCKEGdAjcxhT9Pc/Rl/wlEApoFLVVxSizl311tVsNdXxE3JLPTPY3g9O2P1vzInU=",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
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
        self.decompress(self.get_str("svg").expect("get svg"))
    }

    pub fn categories(&self) -> Vec<&str> {
        self.get_str("categories")
            .expect("get categories")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn tags(&self) -> Vec<&str> {
        self.get_str("tags")
            .expect("get tags")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn contributors(&self) -> Vec<&str> {
        self.get_str("contributors")
            .expect("get contributors")
            .split(',')
            .collect::<Vec<&str>>()
    }
}
