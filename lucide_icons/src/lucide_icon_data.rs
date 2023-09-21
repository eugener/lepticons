
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideIcon {
    #[cfg(feature = "accessibility")]
    #[strum(props(
        svg = "eJxtzUEKwjAQheGrPLLP6HQykwhJb+AhShQUFKS40Nub6MJCyyze4v9gcr3O9XbGXBw71Fcba/suLrgx7351zI/pecGpuDsn8AHsozdwF70suiJBvEBJ2w4kBiFdw2OgIYAD6aRQ7L9nlBJsA7NQbP/iUvuu/V9/AN67Mq0=",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[cfg(feature = "activity_square")]
    #[strum(props(
        svg = "eJwlyzEKgDAMheGrhOxFExEd2s4uHkJsMQUHKQH19hrdHh//8zWvCpLLJhqQRoQrYIdwlqTyw/1BfZ0x+sYO0R+LCqSAMw1ALI53x9A7dtTamAZLLYoPTn0aRA==",
        categories = "medical,account,social,science,multimedia,shapes",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "danielbayley"
    ))]
    ActivitySquare,
    #[cfg(feature = "activity")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJSMDTK0DXJ0TVWsPSxVDAGMzyMlOxs9EGK7AD2QAsc",
        categories = "medical,account,social,science,multimedia",
        tags = "pulse,action,motion,movement,exercise,fitness,healthcare,heart rate monitor,vital signs,vitals,emergency room,er,intensive care,hospital,defibrillator,earthquake,siesmic,magnitude,richter scale,aftershock,tremor,shockwave,audio,waveform,synthesizer,synthesiser,music",
        contributors = "colebemis"
    ))]
    Activity,
    #[cfg(feature = "air_vent")]
    #[strum(props(
        svg = "eJxtT7sOwjAM/BWrewzn1E6GUImNhZU9EkNGBtTvx1EQZKj8kHxnn87lVd+NnpflbgS5rVVI6OyBIEEe+p/J5wabAZJd5wOSFmTZyqmLbmWSzg1HBDJHQuJUhZV6DaXIMIqcY8e+bTBgrO7rUM3Yf1C267CEXp5pD/rb/gCvNjbM",
        categories = "home",
        tags = "air conditioner,ac,central air,cooling,climate-control",
        contributors = "karsa-mistmere"
    ))]
    AirVent,
    #[cfg(feature = "airplay")]
    #[strum(props(
        svg = "eJxNTb0KgCAYfJXDXer76Gcx55bWdiHIIFRIgt4+tSE5OLjjflQw0WKbxNKDxrkzDEabQJIlr/2vkbSloTbAN7V1A2wlCa2avKpV8Oeze4fgDxevSRCD8g2YUKgYJf8l9Qs3UyJk",
        categories = "multimedia,connectivity,devices,brands",
        tags = "stream,cast,mirroring,screen,monitor",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Airplay,
    #[cfg(feature = "alarm_check")]
    #[strum(props(
        svg = "eJxtjUEKwjAQRa/ymX2iMxOTCElv0ENIFBQsSHHR3r4JhXbRrj689+Cn8hnL94UxUySUKRNL3bmuUpcuq+7S7/F/45mpv0Eh8M01tptBKjZq9Kh6bzWCow1wED4JOFjvWuED5HraDHdwuxY44za9AKfuMts=",
        categories = "devices,notifications,time",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmCheck,
    #[cfg(feature = "alarm_clock_off")]
    #[strum(props(
        svg = "eJxtjjEOgCAMRa/y4061FSsk6A08BImDi4mD94+ARgfJb9/U1zYc8dywTs2i5EZkRAeHDpybSfRmM4c2z87hNdiTB1uS4VFSjCceCv7CLgI1vekru7QccqQjLIRrMlK6VBXbZgnfjxfqOjWs",
        categories = "devices,notifications,time",
        tags = "morning,turn-off",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,jguddas"
    ))]
    AlarmClockOff,
    #[cfg(feature = "alarm_clock")]
    #[strum(props(
        svg = "eJxti0EKgzAQRa/ymb1pZyZNUki8QQ9R0oKCgoiI3l6DG0GX/733Y27H3P2Rl0QshDFRIOR1X0p1fBy6jsN3avBL9GHBe7adQIou+CRfUAjc1fSy40orvTk5owEcjIeF8E3A3jhbCuchz3OzAZQ1Mvs=",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_minus")]
    #[strum(props(
        svg = "eJxtjUEKhDAQBL/SzN3szoybZCHxBz5CoqCgIOJBf6/Bgwc9dlVBhzQsaeyQ9kishCWSJ6TtXEJV+Fy6CnOz9mgj1T8oBDa7zG4zyYkLLfSpamvUg71xKCH8ErAztsyFdZDve/MHa38fH4LGMgA=",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmMinus,
    #[cfg(feature = "alarm_plus")]
    #[strum(props(
        svg = "eJx1zUEKgzAQheGrPGZv6szYJIXEG+QQJS1YaKGIiN5egwsX6nLe98OE/Onz9408R2Il9JE8IU/rJdSG28Zt+D+HDq9I6Q6FwBYr2y4/WedKKz1SskY92BuHBsInATtjm1JYB6kvGgHX48nn9ABrt8MC+K85Xw==",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmPlus,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "eJw1i1EKgCAYg68y/guEGeSDepmSFETFhPT2qRGMMbZvMpujoCrihNwUrQRr3GWLIiYIbQ6PO4v9ilwHo+Uyflqm6Jt3wSBFF8rdGQaObkMbBNg+4w4+Tj+uX9kWIPU=",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "alert_circle")]
    #[strum(props(
        svg = "eJxNyzEOgCAMheGrND2AUgbiULhMw0BCHJja24tWidM/vO+xtCG9glhGigii3jETsPDue+HezgpGGQ8EjY7srdLTqW/1s5TW6Dh95y3Q8hch7CH9",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[cfg(feature = "alert_octagon")]
    #[strum(props(
        svg = "eJxFjEEOgCAMBL/S9AFoG4MewN8YQ0KARA/we5FCODWdnV2Toi93DJCiC+9jcVeHBgbSirZ6mUHIQD2YeLqNCMbTLH35NN6FCwpbJEYoZPFAyCRvFlz13+ruCFtHD0mtJPWKhv8BN04t5Q==",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertOctagon,
    #[cfg(feature = "alert_triangle")]
    #[strum(props(
        svg = "eJxtjDEKgDAUQ68S3H9tfj9aoQoewAu4CQ4dFBzE80sHdZFseclLx3JmrH21K10bwChRaItC4UskOIvwm0TQxreGQZnZfEOUv4S5GlJdpEN61BMV3WX/gG12ni+7AUBZIpk=",
        categories = "notifications,shapes,development",
        tags = "warning,alert,danger,exclamation mark,linter",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    AlertTriangle,
    #[cfg(feature = "align_center_horizontal")]
    #[strum(props(
        svg = "eJxtjysOgDAMQK/S4Atts/BJxjQGi19ATCLIzs+KYIMsNf29vtSe/gpwzM0qwBKEGmc77Tn7TpiA+2i8gAClYEzZ0pc1SkRTR8ctk6SbBUl6KdZAeZz8dQaUv5RrUpOkw07ILbeTgpg+yySoNIM3FO0+Iw==",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterHorizontal,
    #[cfg(feature = "align_center_vertical")]
    #[strum(props(
        svg = "eJx1j7sOgDAIRX+FuKNAGh9JdXZxdTc6MDoYv1/awaqpYYKbc0/w+3IobH0xsYCcQsXgq3Ab/J20wDS6xXIgG0ZBmeuVkEsuO7Q7iroMyLWRmkiCSKY9NCm6rFNobN7OE+Uj5R+pU04oxMeeVWDShF5EPj45",
        categories = "layout",
        tags = "items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignCenterVertical,
    #[cfg(feature = "align_center")]
    #[strum(props(
        svg = "eJxVjEEKgDAMBL8S9gOSiLZC2t94EMSz/b1tVgSPO8ysn8e1S9OCFXJbwQxpxtWhKapPw6ke5oCaEIUak8Skz78bMNNZ3iTzV7fPfQBpNSC2",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "eJxljE0KgCAQha8yvAtEgwiCum7TISIl3YUMVLcvB2rT5j14P59veRU6AwyoPcagS/WoSUqABZVctyIBo0X0Q99Hr6//xKGjRqMQ9xK/175IoRQwMxPzpE3P4g0UYCXW",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "eJxNjEEKgCAUBa/yeRcIPxIIfm/Qtn2kpLsQobp9Khhth5mxOeyFHoEGXcmXKFAz6BYwKIZ0xCKoIHfi7NR8Z3tVkUFrlR7CeJh//FXnViJ5wcJMzGv/NeZeAHwl5A==",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "eJxtjEsOQDAURbfycueCUp302YGpuSBqJtL47J4STSWdnnPP1evYWzoZEnQwStA+D9YwKtB6AwEy4zwZy8hL1Dp1+1o/1fGwIPDLDO5SfRc+WzpraGA0uSIhtkQ65eBfqVZExJsUUSOD4gK7LDu4",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "eJxdjMsNgCAQRFvZTAMC8XNh7cAijBLhZgjx072AEY2XTXbevNHeTIGscYsNDFmD/MFQoHjjczIa0O7mYBktel2lfq+zVeLXF1mUt9k9Y0Vbx2BpZgxSkNqUSCRlH6J+5AKJgy2B",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "eJxdjMsNgCAQRFvZTAMqQb2wdGARRohwM4T46V7gIMTjzHszKtgtkrN+d5ExSFC4GQJ0eRMdYwKlnOqHMUKrLvtalVVVEpyL2Bx8n32dHWt0ZBiLJHGKAnLVgOFHXlsXLVc=",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "eJxNjMsNgCAQRFvZTAMC8XNh6cAijBDhZgjx072uJuhxZt4bm8Nc6GAY0MnoQPkNMaQlFoZuQXvyJTJ6ONsI7+xj3fxQ+cp8poIc65+2TiWSZ4zakNmMkkU6dwElqCYo",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "eJxFzMsNgCAQBNBWNtOAsPFzYenAIowQ4WYI8dO9ASMed+fNmOTXTGd0OQhGUPBxC1mge9AlYNAtGECpHtZ0xVtTW82qavWLpw//q622LzmQE8zMxAerkpSffQBA4CYl",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "eJxFjMsNgDAMQ1eJsgC04nNpugFDIFqR3lAV8dmeNkj0ZMv2s8txE+CYdhZCMyBcKQgTTggP4Yhwq89FLHrX1b13SrXl11ZgVsCU7P/sG3aswhAIFwv2tFrUyL8gcCX7",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkM0NbO3FBDedhAX19prONFPM+76k3egRTCBN+VAT9A505WgqGEHlFjDo2xnBd7UP/txMKQqWgZhXrlCvH7Br5QU0ex2e",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "eJxVi8kNgCAQRVuZ/AYUFL0wdGARRohwM4S4dC94ULy+RUe3JDoZHci7sPrEED3oCDZ5xgCKWUrQxVAwuim90c/19m3d51yoehu/bZuTJ8uYOpK7bIsoqBJS/M0NYaQtVA==",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_justify")]
    #[strum(props(
        svg = "eJxVzLEJACAMBMBVJAtIIohFdBsLQax1e2MgQqrn+eN5jtXDoQoZwkGNLY1QUmqCxvGZxiptexTJjB5I9VZN8ab8e7MXZiwgpA==",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[cfg(feature = "align_left")]
    #[strum(props(
        svg = "eJxtizsKACEMBa8iucCSLH6K6G0sBLHW2xuNWFk95jHDtbRsBkZwYAbt6UKEsoI/JP6Wk/iaSOqg1WTzw9VPk3DUcFJ/3QlnACCs",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[cfg(feature = "align_right")]
    #[strum(props(
        svg = "eJxdi0sKwCAMBa8iuUBJCv1A6m26KBTXentfVFRcPWYy0f8Lr0v80EEuSZkIEsYCd/K6WeO1lyz1do/UPqGX1uQ1RdwY/uxtBmfDIK4=",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "eJxFjEsKgDAMRK8ScgExlILQdO3GQ4gtpjspAfX22oB1N783oeZN4SxJhdEjXIwO4TZdX0MIkssuyjh6jGFo+xiM+orJsNF14r+zo04dqwokxoUIaKZWtCg+9QElpQ==",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "eJxNjEsKgDAMRK8ScgG1SEFoegMPIbaY7qQEP7e3aQXdDTPvjctxFThTECacEC5Ci8AxbSw13oTDiJDLYNC7TnnvqvWjmtYgVYrxfg72s/ZFGALhbMAcptdBK/8A/ggl0Q==",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "eJxtzEEOQDAQheGrTN5eaKluWmsbhxDE2Ik0we1NSbDoapL35xu3TUMgnpaZg0cNOjwMaF/GwB6qAp3P2SRoNC6PoHE3e7ZI7EcK/N+Jrj619oFp9Oi0JsuZiSVuv2LJtiqxi1BCykQyUlr9hgvFAjt/",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "eJxVjMsNgCAQRFvZbAMC8XNh6cAijBCXmyGbqN0LRKMcZ968sSmsAukkNAgc4sZCOCIc0QsT6h4hswHhqsHZrgjOVu23zzhPn59XVlWePmtfhMETzgaMYqMKKV1DdENug+4tZQ==",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "eJxVzEEKgCAQBdCrDHOBVKw26g06RKQ07kKE6vY5ComrD//PG5PCkYFCPClbXBDu6DNZlBrhbfFYnBFSCYXOTAycqax1fLB2KCosrj/91bVnAm9xUyA1KcELd8MyDB9MpC07",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "eJxVjEsOgCAMBa/S9AIC8bOh3MBDGCGWnSFN1NtbMJq47LzO+JJWgXISOoSL0I4IegwInPLGQqjgyFFYtx6D76oQfNNebn7fqk+tpcUn/Fn7IgyRcHZgHTtTl8rCDTBhJho=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "eJxNjEsOgCAQQ68y6QXUiZ8NcAMPYYQ47AwhUW4v4wLdtEnbV5PCnklCPCRbzKB0WzCo6gS6os9iMYygUo3hTKeAMy9WR8s36hv8uysaNOrcspC3WJmYhXttNHMPOFsmFw==",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "eJxdjEkKgDAMRa8ScgFrcNg0vYGHEFtMd1IC2tvbFkVw96f3bQqbQroYCUFC3EUZJ4QSjAiZsS/mjF6lyAGd7SrgbMNymz7wOzL/n/mjjlUFPONCQEKmFjVyNwj9Je0=",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkE1t4yHEBDedhAX19prOlDPv+5J2I035UBOMoFswgR7BDCrfYNCVo6mgdwi+q33w52ZKUbAyE7uFq9SvleEHLy3NHYI=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "eJxVjMsNgCAQRFvZTAMq/i5ABxZhhLjcDCFRu5c1UfQyhzd5T0e/JDoNmh50GOTdg0ucQQeKmSgQ+7ByMhhgdSWC1bdWuKgjpNOWQP0EXmubE5MzmBSphlUtj7Df036PCz0oLTg=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceBetween,
    #[cfg(feature = "ampersand")]
    #[strum(props(
        svg = "eJxFjT0OgCAMha/y4k4FCsiAJB7AQxAcGB28f2xZSNPlfe+nvO0beM7tdgdFON8tAgXDlJBNRr6CyPpWzkEskWK3RnFQC8XG4AlVtVLA8BLIsw+Ot1p2nalljSVBY5EfpI0eCQ==",
        categories = "text,development",
        tags = "and,typography,operator,join,concatenate,code,&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersand,
    #[cfg(feature = "ampersands")]
    #[strum(props(
        svg = "eJy1jbENgDAMBFex6B2cDyYghWzAEFEoKCnYXziA2ACd/osv/tJRzp22pVu9kI+VlQPHm7mAQGJ4GkiqEJyyvj223UVLoAe4yTZlOFhrl1PfznP6FMBPigsqKSkM",
        categories = "text,development",
        tags = "and,operator,then,code,&&",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Ampersands,
    #[cfg(feature = "anchor")]
    #[strum(props(
        svg = "eJwlikEKwCAMBL8Sci+tEaEH47mXPkKsoCClSA/6+8ZKwg7Ljg25hhKhMmqE0BgVCTujQWfXuTpb8h2hE+OO0NSUupCEjf4u9rCcffyb4GI8DSg6yKsN5OeRRFr0kIfmPoH7IK4=",
        categories = "transportation,text,maps",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Anchor,
    #[cfg(feature = "angry")]
    #[strum(props(
        svg = "eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vapbYpcDJ/PezPGzs4uI+zVMJIMzh/BYM/4tiZ/eGu2/pgwNKyjElTunDLFJS9CEBPkIL3UKlOoQQL6D1cqoCH9CiWa2rcmkQCkvuQGQqkzew==",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[cfg(feature = "annoyed")]
    #[strum(props(
        svg = "eJxtyc0JgDAMQOFVQhbQFIUKaTdwCIlCBA9SPNjt+3cq9PTgfSx3kOeC4JBmBIm5Jvev9Tw19/wen8LpcLdAq9pCZXWwqRl8WjpIRyAgiQ==",
        categories = "emoji",
        tags = "emoji,nuisance,face,emotion",
        contributors = "karsa-mistmere"
    ))]
    Annoyed,
    #[cfg(feature = "antenna")]
    #[strum(props(
        svg = "eJx9zLsNwCAMhOFVTvR52Ipx47ABQ0RKQRMpRZT5kQuoEO19ut/e6yu4z5AZxFBwSLb5mKzRo06y0D4w4hlOnvlYBVpIBuTR+McuFZG1LR8=",
        categories = "devices,multimedia,communication",
        tags = "signal,connection,connectivity,tv,television,broadcast,live,frequency,tune,scan,channels,aerial,receiver,transmission,transducer,terrestrial,satellite,cable",
        contributors = "danielbayley"
    ))]
    Antenna,
    #[cfg(feature = "aperture")]
    #[strum(props(
        svg = "eJx1j0sOwyAMRK+CfAAXQ+qABLkMyiJS1EVW5PbFMelHbVYj2TPj51SWrayz2TKQBVP2pq5pPXRKN91PaV0es6kug7No72B2yhDA1CY0oKc2aUsaMQ4SE3sPybw7I3IEbSGk8Wz59suaPAqGZEb0QcsdWtaIon1mXuVi5MMoPR5jZyX+c8Zh8L8/8EWiX1ZAi+HEIvlZWxj5jfYEsdhWDA==",
        categories = "photography",
        tags = "camera,photo",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "eJxdi0sKgCAURbfyuBtIRaSBuoMWESk9ZyGPPrsvcVKN7uUcjq95EeJcVpYA7UBXgAWdAQZU+xwlCT9PIfqhBdFvszClgEkrsrttoqGXMDRyL37CfYIbUnokzg==",
        categories = "design,files,layout",
        tags = "application,executable",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    AppWindow,
    #[cfg(feature = "apple")]
    #[strum(props(
        svg = "eJxFjksKAkEMRK9SzD6xUybzgXFAXHsBd8O4cOnC+2O6G5SQBOoDb33vnxeel+FuBIsufpgGCqhTwLSM8P7OKY4y5xqVvLouhnZKG5sQh1QLRWrHXQIUE40x9WkWSgj37NRer2XE07oR5sg4MedJBnaIkM7Q3mPY1lMF3tY/dqYOQzKzTfwyX+slLWo=",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Apple,
    #[cfg(feature = "archive_restore")]
    #[strum(props(
        svg = "eJxtzEEKgzAUBNCrDLMPzf+pUCHxBj2EVOl3USgSbHv7Ji5EQWYzMI+J8/jI+CUGYv4mCvGZhmyJ6gkbp6flxIYom7KLl+q7+O6zYUi8X3FbRHqFwq8pzVZYyQ6qP0pxVboT+mohDYILKDl5EoXo0m7LH/i5M6Y=",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "eJx1i80OgkAQg1+l6R1lRidisssbePVOhDgcTAzZ+PP27HKBC+mhTdsvTMMj4Tv2ySO1Jn7ZiCmbEP/IE+HD+PQUaWzDsfzb8O6So4+8ndF8RDqFol6Uk4tui0rvTSELs5Kv68EgF1hle6PCsI4zDTAsUw==",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "eJxNiz0KhUAQg68S0j/ezqhgsesNbO1FxbETWfy5vbsWIikSknx+m4YIm5bZYmBFXIEFcSxjtEB1xJmM2JIJG//P/8avfTSMgW2JehfpFQr3KCUT/RY/7epMZuZDioOole9yAxKSJCA=",
        categories = "files,mail",
        tags = "index,backup,box,storage,records",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Archive,
    #[cfg(feature = "area_chart")]
    #[strum(props(
        svg = "eJw9yiEOwCAMQNGrNPhmKdCsouMGswgcCaICgSCcn2Cw/z8ddRq0z/0BwiIxEpf0OTXpfS+QX2zks3RkYIwYy3UblaES0g==",
        categories = "charts",
        tags = "statistics,diagram,graph,area",
        contributors = "nstokoe"
    ))]
    AreaChart,
    #[cfg(feature = "armchair")]
    #[strum(props(
        svg = "eJxtTrsKgDAM/JXgHjTns1A7u7g6uBUcOjpIvt920RZKhnBPzt7+CXStzS6GzDF5EKhLx2Bsc44J2jfOtini7BfsSUTH3xg/ggw5wVDOHDxQp7FcGSV5VupHkkVREeLgQnkB4E0xNA==",
        categories = "furniture",
        tags = "sofa,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Armchair,
    #[cfg(feature = "arrow_big_down_dash")]
    #[strum(props(
        svg = "eJxNiiEKACEQAL+y2OU4Tk8WVrPFahcMGwwGMfh6tYhMmxmqqTFkK8KrQXsUjp6tHN0B+8eqSANGLlhF5H+cdQKREBMR",
        categories = "arrows,navigation,gaming,files",
        tags = "backwards,reverse,slow,direction,south,download",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigDownDash,
    #[cfg(feature = "arrow_big_down")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTRVMCszyzDJ0TVXMNcFwgyTMLMMsyolOxt9kCI7AAXWDAM=",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,direction,south",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigDown,
    #[cfg(feature = "arrow_big_left_dash")]
    #[strum(props(
        svg = "eJxFiiEKACEQAL+y2JdD1BNhzx9ctQuGDQaDbPD1atEwZWao5c5QPvXrANqloCI920W6xa3CaMRW9OhhIZaNvOPME8ayE9Y=",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigLeftDash,
    #[cfg(feature = "arrow_big_left")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNbRQMDTN0DUrM8nRNdc1VwDiMpMMszKzKiU7G32QMjsAG7YMkQ==",
        categories = "arrows,navigation,gaming",
        tags = "previous,back,direction,west,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigLeft,
    #[cfg(feature = "arrow_big_right_dash")]
    #[strum(props(
        svg = "eJw9yisKQCEURdGpXOzywvPDgavZYrULBoPBIAZHrxbDLovNPY9KxYmoCdMIz98Vz89BqH/SzZKVpylVQMJ65wZ4SRKn",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,turn,corner",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigRightDash,
    #[cfg(feature = "arrow_big_right")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVOwzDALM80xVzDXBeIyXRMPszDLKiU7G32QGjsA+EYLmw==",
        categories = "arrows,navigation,gaming",
        tags = "next,forward,direction,east,indicate turn",
        contributors = "Andreto,mittalyashu,ericfennis,danielbayley"
    ))]
    ArrowBigRight,
    #[cfg(feature = "arrow_big_up_dash")]
    #[strum(props(
        svg = "eJxNiiEKACEQAL+y2Jfj8DxZWM0WHyEYNhgMYvD1ahGZNjNcUxPITkWCl+RXnp+tPN/BdNTBFIsWFoJf14HGeSelxRM0",
        categories = "arrows,navigation,text,development,gaming",
        tags = "caps lock,capitals,keyboard,button,mac,forward,direction,north,faster,speed,boost",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowBigUpDash,
    #[cfg(feature = "arrow_big_up")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXytVQwtCjTNfMwzTHXNVcAwgxdkzIzD8sqJTsbfZAqOwAMiAwN",
        categories = "arrows,navigation,text,development,gaming",
        tags = "shift,keyboard,button,mac,capitalize,capitalise,forward,direction,north",
        contributors = "Andreto,mittalyashu,danielbayley"
    ))]
    ArrowBigUp,
    #[cfg(feature = "arrow_down_01")]
    #[strum(props(
        svg = "eJxtjbsKgDAMRX8lZC/aWuvS+geu7qJiOghSio+/txHEDpIll3MfdhsiweRwrUAa0HxCY2sLBq19cdeAKvsMhHmMEC6HCuF0KGuEJDTC4adIz0ezXyg6NJxif1YnuW8XhoT6GZN1ovSt3dtmLhQ=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "eJxtzLEKgDAMBNBfObKLVqsubf/A1V1UTAdBSkH9e1NBcCjZ8u7OHFNkLJb2BqqDTldocqZM4MzHQ4+6GnOgeigRLuoctoL8q4V1jrgtKU3g1W8cLXWESz4t4fRLZEtiQTLvYCq4B9UTLfQ=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown10,
    #[cfg(feature = "arrow_down_az")]
    #[strum(props(
        svg = "eJx1jDEOgCAQBL+ysQfvkEML5Ae29CQWV2hi4f8jFMYGM9lsMdmNV7kV+zqcEzjAN4wfUhybSPHV2wxHuSccYVEjHcMCphysFGcFLVRh1M5MPwOvchhB0O/xAQRkKyE=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownAZ,
    #[cfg(feature = "arrow_down_circle")]
    #[strum(props(
        svg = "eJxFyUEKgDAMRNGrDNmLtnSRRdobeAiJgoKCFBG9vQ0Fyyw+zBPdsu4L9InkPEHf2lwyUJK+epJzulbMkUbnwTcb2dXgYBQKti78/AEfvxp6",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownCircle,
    #[cfg(feature = "arrow_down_from_line")]
    #[strum(props(
        svg = "eJxlyaEKACEQRdFfediXZVycRRjNFqtdMFgEg/+PWDTIbfdIz6OiOBXJ4gtGeXnX8nJAQ1P6b2kMMuDVw5sns+EWXA==",
        categories = "arrows,navigation,files",
        tags = "backwards,reverse,direction,south,download,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownFromLine,
    #[cfg(feature = "arrow_down_left_from_circle")]
    #[strum(props(
        svg = "eJxtijEKACEQA7+y2MtpiuOKPWsbHyFY2AgW4vvdLbSShBCY4Z5HpfKbBPLI3pFURqPfBH5UCbzFBoK4zt5g+gRGTPsetgC6/RlO",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-west,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftFromCircle,
    #[cfg(feature = "arrow_down_left_square")]
    #[strum(props(
        svg = "eJxFyzEKgDAMQNGrhOwiVSgZ2s4uru5ii+kgSAmot7fRoet/fFfSJsAp7yweDSHcHkeEK0fhPzxfKLUPGFyvQ3DnKgzR42EsUEdAShobzZWMnWhp9gLC4h65",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-west,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownLeftSquare,
    #[cfg(feature = "arrow_down_left")]
    #[strum(props(
        svg = "eJxNySEOACAMBMGvNP0AqTpzVGOweBIEEsH/Q6po1u3wzLtlVe0GiQzqLHGd2QwN49sDricPOQ==",
        categories = "arrows,navigation",
        tags = "direction,south-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownLeft,
    #[cfg(feature = "arrow_down_narrow_wide")]
    #[strum(props(
        svg = "eJx1yTEOgCAQBMCvbOiNLF7E4uQHtvYmFteYWPD/EAqoIFOO/k82vKf7NnCHVIu4pGuNpK2viODvUZAQm8RhcRwMRt+rAIoRJN8=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownNarrowWide,
    #[cfg(feature = "arrow_down_right_from_circle")]
    #[strum(props(
        svg = "eJxtiqENACAMBFdp8IR+RVVhA4YgQSARhPmhBkX+8ubOZluDeg4VQiINTJd7Po7gUCx5U+yV4iXB+Vvo1hH1yQP8WBnp",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,south-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowDownRightFromCircle,
    #[cfg(feature = "arrow_down_right_square")]
    #[strum(props(
        svg = "eJxFi0EKgDAMBL8S8gGpguTQ9OzFR4gtpgdBStD6e4k9lL0sOzu+pF3hZZwQSmUcESTlQ5TREcKTo0qr1T7BDyYEf20qEBlPgj9GbOtkdTPQTUtHH3+oHm4=",
        categories = "arrows,navigation,shapes,gaming",
        tags = "direction,south-east,diagonal,sign,turn,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowDownRightSquare,
    #[cfg(feature = "arrow_down_right")]
    #[strum(props(
        svg = "eJw9ySEOACAIQNGrMC4gJAqSLR7CzWBxMzjPLwbZfvpPV9sDesYpIMDkoWl61/RbZcfDVCTsArN7D2Q=",
        categories = "arrows,navigation",
        tags = "direction,south-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDownRight,
    #[cfg(feature = "arrow_down_square")]
    #[strum(props(
        svg = "eJxFyzEKgDAMBdCrhOwi1g4Zmt7AQ4gtpoMgJWi9vQZByRL++z/UvChcjCNCbYwO4SxJhXEgBMllFX3/Zp0YehvEsM8qkBinwQEdZGDRDxvBQ96u8x/f3Qoewg==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "eJxFikEKgDAMBL8Sci+SIJRCkx/4CImCgoIUEf29KSJlDwM7k4/xXGASHIiBL+pRc1c/zb/ZKUEKEWLwNW1rsW2GIkgIdjvY+Qgy1ejT+gIL+Rqf",
        categories = "arrows,navigation,maps",
        tags = "direction,south,waypoint,location,step,into",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowDownToDot,
    #[cfg(feature = "arrow_down_to_line")]
    #[strum(props(
        svg = "eJxlyaEKwCAUheFXOdjHOI7dMbgzr1jtgsEiGHx/xKBF/vZ/WmPLSJ/xtOATLuP0HM/plCIgIaNDdvZ8YfnfSzq25haJ",
        categories = "arrows,navigation,files,development",
        tags = "behind,direction,south,download,save,git,version control,pull,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowDownToLine,
    #[cfg(feature = "arrow_down_up")]
    #[strum(props(
        svg = "eJxtyzEKwCAQBdGrfOwlWbNoio03SJs+YGEjWIjnly20kikfI/VvGekx5QJ5sGbZRDkUokx+A9z5baA4wm1Zw+6jAO7klwzAeR6i",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,traffic,flow,mobile data,internet,sort,reorder,move",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownUp,
    #[cfg(feature = "arrow_down_wide_narrow")]
    #[strum(props(
        svg = "eJxtzDEKgDAQRNGrDOnFTFyMxZob2NoLFtsIFrk/IUVIs/zywdf/qYb3DN8G7pDeIqHo2qHo4CsjxdsDEmKMvhyWfWCyOWuEkCTf",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowDownWideNarrow,
    #[cfg(feature = "arrow_down_za")]
    #[strum(props(
        svg = "eJxtzDEOgCAMBdCrNOwglBYdkBu4upM4dNDEwXB+y2BcyE/TJi/9+a6PwLGaK0JIQD2WTMlTh5I/3magFtIAAgMJn5YhCY8dfbPRcUWnt47XBNC9ox98oOoi9i97AUmrK5I=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowDownZA,
    #[cfg(feature = "arrow_down")]
    #[strum(props(
        svg = "eJw9yUEKABAQQNGrTPbSTDSpMTdwCGVhoyzk/FjQ3/0no8wGNZmMBGGhNyruPpUnHSMgWQa2p+8b3MgPwg==",
        categories = "arrows,navigation",
        tags = "backwards,reverse,direction,south",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowDown,
    #[cfg(feature = "arrow_left_circle")]
    #[strum(props(
        svg = "eJxFyTEKgDAQRNGrDNuL7iKSYpPaxkPIKigoSLDQ22sMGKb4ME9tjbbNsMsTC8Hu3PimoaB19qDHeC6YPA3cgaV3ydJXZGeBq1p8+/kBL3gaew==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "previous,back,direction,west,sign,turn,button,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeftCircle,
    #[cfg(feature = "arrow_left_from_line")]
    #[strum(props(
        svg = "eJxtySsKQCEURdGpHOyPx/UHwtUZWO2CwSIYnD+ioEl224t7HhXFi+ZgP4udCPyvH/hoVCBZST9EEsglc2UCmzUWfQ==",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,expand,fold,horizontal,<-|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftFromLine,
    #[cfg(feature = "arrow_left_right")]
    #[strum(props(
        svg = "eJxtyzsOgCAQhOGrTOiNLm7AYqW28RAmFhSaUHD/8CiggEw3X34JT/R4T3Uf2MGwH4OVk7X8Tppm8WRG+MlAE3ipm4R6A9mrSwLUyx6o",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "wojtekmaj,ericfennis,karsa-mistmere"
    ))]
    ArrowLeftRight,
    #[cfg(feature = "arrow_left_square")]
    #[strum(props(
        svg = "eJxFy7EKgDAMBNBfCdlFUkU6tJ1d/AixxXQQpATUv9dUUG45eHeupEXg9NghcMori0eyCEeOwm+9KpZnYzC4Vg/B7bMwRI8bGbBNDzXKCj9PNACZ0X5yA+fMHsM=",
        categories = "arrows,navigation,shapes",
        tags = "previous,back,direction,west,sign,keyboard,button,<-",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowLeftSquare,
    #[cfg(feature = "arrow_left_to_line")]
    #[strum(props(
        svg = "eJxlycEKABEURuFX+bOfpjsGqcsb2NorCxtl4f0TxUZndz5uqRdkJ4IE2aiE53cuzxsqSehHY3VzMKCv0H9kAKk7Fnw=",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,collapse,fold,horizontal,|<-",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowLeftToLine,
    #[cfg(feature = "arrow_left")]
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvDLvIBjIGc9niIwSDRTD4f9Sg4dLprKtDS24gAYpnz3A403DH9H1BAaQc/2zIPw+Z",
        categories = "arrows,navigation",
        tags = "previous,back,direction,west,<-",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowLeft,
    #[cfg(feature = "arrow_right_circle")]
    #[strum(props(
        svg = "eJxFyUEKgDAMRNGrDNmLJoh0kfYGHkKiUEFBigu9vW0FZRYf5qmtybYFyRN3BLtzJfeqDdq+HvSYzojZ0+jAEl2hcv2ws4AH9E3d5w81jhq1",
        categories = "arrows,navigation,shapes,gaming",
        tags = "next,forward,direction,east,sign,turn,button,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRightCircle,
    #[cfg(feature = "arrow_right_from_line")]
    #[strum(props(
        svg = "eJxlyaEKwCAYReFXudjHuG5uC//MFh9CMFgEg/j8osEip51PSqgJ8Vf+gmm8lZVzLCsLNEHt3l0yDfjhOWbLO9QHFsQ=",
        categories = "arrows,navigation",
        tags = "next,forward,direction,east,export,expand,fold,horizontal,|->",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightFromLine,
    #[cfg(feature = "arrow_right_left")]
    #[strum(props(
        svg = "eJxtyzsKwCAQRdGtPOyHZMygKSbWabKIQAobIYX7xw9oZXGrw9X/zRHfZRI7HBAI1UzQrUHQwY/d4e8FpBOWqV+0+gTsI7spBbwbHoY=",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,transaction,reorder,move,<-,->",
        contributors = "karsa-mistmere"
    ))]
    ArrowRightLeft,
    #[cfg(feature = "arrow_right_square")]
    #[strum(props(
        svg = "eJxFy0EKgDAMBMCvLLmLtIr00PYHPkJsMT0IUgLq77U5KIEs7LC+5lXAuWwsgYwj3IEGwqW/vmEJZ0nCqtH3bRD9sQgjBZodjGWFVv2wGwszYez0Pn8A/dge/Q==",
        categories = "arrows,navigation,shapes",
        tags = "next,forward,direction,west,sign,keyboard,button,->",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowRightSquare,
    #[cfg(feature = "arrow_right_to_line")]
    #[strum(props(
        svg = "eJxlyTsKwCAURNGtDPYhjPkWL9ZpXIRgYSNYiOsXBW3kdvdIcjnAf8ryAfV/KCN7e0aGRBJ8cW+91a0mrsJzSgXNKRbx",
        categories = "arrows,navigation,development",
        tags = "next,forward,direction,east,tab,keyboard,mac,indent,collapse,fold,horizontal,->|",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowRightToLine,
    #[cfg(feature = "arrow_right")]
    #[strum(props(
        svg = "eJw9ybEJACAMBdFVPvYiESVNzAYOIVikESzcH7XQ4pp7Mtsy9OJqBkWj5FTCfSpPBkVkMNifPm/J3A9l",
        categories = "arrows,navigation",
        tags = "forward,next,direction,east,->",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowRight,
    #[cfg(feature = "arrow_up_01")]
    #[strum(props(
        svg = "eJxtjk0KgzAQha/yePvQJo2xi4w36CFKlY4LQST4c3uThehCBmYx37yfOH6TohUOL7zhjUceNvFR7k086KeGn204wdT9EqZN6Ajt+r8mYSBWoa2IpW+TCj2xlZ1V5f9iZ2u452yCGncTZqtM9ayxA98WLgo=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp01,
    #[cfg(feature = "arrow_up_10")]
    #[strum(props(
        svg = "eJxtjLEKg0AQRH9lmF6SNaumuL0/SJs+RMlaBIIcUf/eUxAsZGCKebwJv1dytMbvDXdoochhDJd1j2Gnjwb6l/oESAO5PtWL8gxWGfrhb+jeCZNRKsK7/uPJWBPDbCyJ3KLE2LfJjZu2CnEBxakt6g==",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp10,
    #[cfg(feature = "arrow_up_az")]
    #[strum(props(
        svg = "eJx1jDEOgCAUQ6/SuIN85CMDcgNXdhKHP2jiYDy/MBgXTNM06Usbz3IJtmU4JgQ45VA1pDi2PsWXrjPcTb4DrEEQxR1CDDLZay5WM5pNFaFmJvMzcMK7Ynj5Hh/2jSsX",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,alphabetical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpAZ,
    #[cfg(feature = "arrow_up_circle")]
    #[strum(props(
        svg = "eJxFyjEKgDAQRNGrDOlFdwnBYpMb2NrLKigoSLDQ25tVUKb4xRvRJes6IUdHjYNepVx6Pk1Sv55kH44ZY3QbBRBX3gZvD5PfO2JQ6NtPbjk8GtI=",
        categories = "arrows,navigation,shapes,gaming",
        tags = "forward,direction,north,sign,button",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpCircle,
    #[cfg(feature = "arrow_up_down")]
    #[strum(props(
        svg = "eJxtyyEOwCAQBdGr/OBJu3QDFVtuUFvfBIEhQRDODwhQZOTLSP5LRHhUMgSymsG6p7wcQ7xMf8nBnN9G0oV7XOhtPgeuZBc0vRUeog==",
        categories = "arrows,navigation",
        tags = "bidirectional,two-way,2-way,swap,switch,network,mobile data,internet,sort,reorder,move",
        contributors = "it-is-not,karsa-mistmere,ericfennis"
    ))]
    ArrowUpDown,
    #[cfg(feature = "arrow_up_from_dot")]
    #[strum(props(
        svg = "eJxFyrEKgDAMBNBfObKLJKBFaPIHru4SBQUFKQ7697aIyA0H9y4e47lgUtobdAhVQA5ZrMtu8dOeBdwO8ouvybcZfimxEFIugt9KwuX0sj3a2Roy",
        categories = "arrows,navigation,maps",
        tags = "direction,north,step,out",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ArrowUpFromDot,
    #[cfg(feature = "arrow_up_from_line")]
    #[strum(props(
        svg = "eJxtybsJACEQBcBWHubHsf5QWO3AIgQDE8FArF8MBAOZcLjnUVGCaOTgP7vBisj/jsinE0moSfoxBpLqNQu2hxbl",
        categories = "arrows,navigation,files,development",
        tags = "forward,direction,north,upload,git,version control,push,expand,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpFromLine,
    #[cfg(feature = "arrow_up_left_from_circle")]
    #[strum(props(
        svg = "eJxlijEKwCAUQ68SvEBrhtLhV+gBXLsXOrgUHLw/RkEXSciQ9yy/JeG7XCTOh+lwwbb2BRvkJwi/qyuLXvDuEBpFKqdXAcBbGGM=",
        categories = "arrows,navigation,maps,development",
        tags = "outwards,direction,north-west,diagonal,keyboard,button,escape",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftFromCircle,
    #[cfg(feature = "arrow_up_left_square")]
    #[strum(props(
        svg = "eJxNy0EKgCAUBNCrDP8CYYH8hXqDtu0jpe8u5EN1+9RNroaZx7iSDsWdo4onw4TH00IoNWbC24ukfIp2Dm5qh+CuXQXR08owdmPp1MaBjK0Gxm8fwPwerA==",
        categories = "arrows,navigation,shapes",
        tags = "direction,north-west,diagonal,sign,keyboard,button",
        contributors = "danielbayley"
    ))]
    ArrowUpLeftSquare,
    #[cfg(feature = "arrow_up_left")]
    #[strum(props(
        svg = "eJw9ySEOACAIRuGrMC6gJMovN7Da3QxEg/efUNhr78Pdz+kMnkqiS106G1peQ5kkUlT2AbzUD1I=",
        categories = "arrows,navigation",
        tags = "direction,north-west,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpLeft,
    #[cfg(feature = "arrow_up_narrow_wide")]
    #[strum(props(
        svg = "eJx1yTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lQIqyO/+0/t4DOfmrgUrZBL8uaxz+Vmr7gnyMnaABIPJQKKlvgRv9I0+vWIlMg==",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending",
        contributors = "lukesmurray,ericfennis,csandman,karsa-mistmere"
    ))]
    ArrowUpNarrowWide,
    #[cfg(feature = "arrow_up_right_from_circle")]
    #[strum(props(
        svg = "eJxtirENwCAMBFd5sUDiL1w5SBkgQ0SioKRAzI/dIAr0p2/urP29ojzpIyF85YbjFyOYsl2RZNvDSJ2DFAWrDl1qApTBGP4=",
        categories = "arrows,navigation,maps",
        tags = "outwards,direction,north-east,diagonal",
        contributors = "danielbayley"
    ))]
    ArrowUpRightFromCircle,
    #[cfg(feature = "arrow_up_right_square")]
    #[strum(props(
        svg = "eJxFizkKgDAQRa/ymV4kCjJFkht4CDHBSSFIGFxur4NFqr88nq95VVwlqQRyTJBcNtG/34FGQv1iIDw2ou9NiP5YVJACzQwWPtmIfY3sDDeBu4Zenwwemw==",
        categories = "arrows,navigation,shapes,social",
        tags = "direction,north-east,diagonal,sign,keyboard,button,share",
        contributors = "danielbayley"
    ))]
    ArrowUpRightSquare,
    #[cfg(feature = "arrow_up_right")]
    #[strum(props(
        svg = "eJxNybENACAIBdFVCAsIFc2XDRzCxILSwji/0cKQXHUPs6+gUbkZWahsFXaUex3J9GXfDsEAD2s=",
        categories = "arrows,navigation",
        tags = "direction,north-east,diagonal",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUpRight,
    #[cfg(feature = "arrow_up_square")]
    #[strum(props(
        svg = "eJxFyzEKgDAMheGrhOwiqaV0aHsDV3exxXQQpASst5cgKG/7P15oZRNoPaJB4FJ3lojkEe6IE8JVs/AbuoYURj2kcK7CkCMe5IDMYHVg1VV+n8kAucV/8gAJiR8a",
        categories = "arrows,navigation,shapes",
        tags = "forward,direction,north,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowUpSquare,
    #[cfg(feature = "arrow_up_to_line")]
    #[strum(props(
        svg = "eJxli7sJACEQBVt5mB/H+g9WO7AIwcBEMBDrFwNBkMlmGO55VJQgkoGqpEXkf6vIJzTyIPXZDezbE0m4eZ0LxYoW5A==",
        categories = "arrows,navigation,files",
        tags = "forward,direction,north,upload,collapse,fold,vertical",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowUpToLine,
    #[cfg(feature = "arrow_up_wide_narrow")]
    #[strum(props(
        svg = "eJx1zTEOgCAQRNGrTOiNDG7AYuUGHsLEYhsTC+P5lYLQQKabV3y9j8dwbu5asEImwT+XdS5/1qp7gryMHSDBYPQDipb6Ery1zge34CUy",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpWideNarrow,
    #[cfg(feature = "arrow_up_za")]
    #[strum(props(
        svg = "eJxtjLEOgCAMRH+lYQehtMiA/IGrO4lDB00cDN9vHYwLuTS93MtdudotsC/mjJCBLIHK1DK9eS0fXWegHtIABAYSPixDEh5z9N1Gxw2dej2vCqB/Qz9ooNIs9h97ACo5K2M=",
        categories = "text,layout,arrows",
        tags = "filter,sort,descending,alphabetical,reverse",
        contributors = "karsa-mistmere"
    ))]
    ArrowUpZA,
    #[cfg(feature = "arrow_up")]
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvDLvIhDGEuR9Y7YLBIhj8P86gXDxZbQ/o2U0CjMCewTiVcEPldbHEVOnPAbejD1g=",
        categories = "arrows,navigation",
        tags = "forward,direction,north",
        contributors = "colebemis,ericfennis"
    ))]
    ArrowUp,
    #[cfg(feature = "arrows_up_from_line")]
    #[strum(props(
        svg = "eJx9yzEKwCAUg+GrBPdSUkWXV2/g2r3QwUXo4P1RB51UsuXjl//NEd+tkoGFPjTqlJez/V66Bge6ZwKJ+46rMBhcjLRDChiqJYo=",
        categories = "arrows,transportation,mail",
        tags = "direction,orientation,this way up,vertical,package,box,fragile,postage,shipping",
        contributors = "danielbayley,ericfennis"
    ))]
    ArrowsUpFromLine,
    #[cfg(feature = "asterisk")]
    #[strum(props(
        svg = "eJxljMEJgDAQBFtZUsCZjXp6cKYDixB8+BF8iPWrCOaR78wwfiznhnUKMxP0YgrZm5dlL2YQmsKgMsYO7Otm/5SBUVp7Tn9yA/iVGVE=",
        categories = "text,maths,development",
        tags = "reference,times,multiply,multiplication,operator,code,glob pattern,wildcard,*",
        contributors = "mittalyashu,ericfennis"
    ))]
    Asterisk,
    #[cfg(feature = "at_sign")]
    #[strum(props(
        svg = "eJwli0EKgCAQRa/ymX00kyYu1Bt0CJmCghYhIXX70Hirx+MFPYqeG/SJJBOhRLIEfbulMP45hSvfO9ZIizj4OmcDA+44cB0kC0OaCniw8O1tT/oA+OwYhQ==",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "eJxNjzsOwjAQRK+ycr9LvHESIiW5ARegQwYJJAoUUcDt8QyKoRl/Vs/zPOXbmu8Xye85RA+yliVIfvG0TLvveJkep+dVznM4eGMuiOzWJC3RWuM6WNtrsk5jtBGbhHAdbT9oj+M2AJIEnBZQAEq5FsyxSQgXgAKwDo7wgcefTexsEERmG3G2Ea8+bKMvPSG8vUpN9lKTvVp9AAqFAcJ3+8bP5gPmEUWl",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[cfg(feature = "award")]
    #[strum(props(
        svg = "eJwly0EKgCAQQNGrDLPXmLFJBccbdIiwoMBFSIu6fUmrv/i8VI5W6gZNcUIoj2L4cisSY07Df3M6l2uHVXEmsaP3QGxDBPLAXI0YZwQckBV2JlqiTjvJL8TjGAo=",
        categories = "account,sports,gaming",
        tags = "achievement,badge,rosette,prize,winner",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Award,
    #[cfg(feature = "axe")]
    #[strum(props(
        svg = "eJw9yzsKgDAURNGtDPZPnfwDMSvQRQQsUihYWLl6o6DFvd1JRzkr1qnbaUAlobdoFdVT4d0IgqJFzyRil9PwkJw+uNCCGhF+M2Lg4KouAaHBRsXDXz+6AZMwGeM=",
        categories = "tools,gaming",
        tags = "hatchet",
        contributors = "Andreto,ericfennis,csandman,jguddas,karsa-mistmere"
    ))]
    Axe,
    #[cfg(feature = "axis_3_d")]
    #[strum(props(
        svg = "eJw9ybkJACAMAMBVgr34EJImZgOHECzSCBbi/GKh7Z3Mtgx6cRUBdyJL5FTCVZV3AyFHYM+/DrXGD2Q=",
        categories = "design",
        tags = "gizmo,coordinates",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Axis3D,
    #[cfg(feature = "baby")]
    #[strum(props(
        svg = "eJx9T0EKw0AI/MqQu1bdmnZhG+gD+oiwPeRYSP9P3aWkOZQgKOrMOJbX/F7wvA2PDLWFRYepnNpwKttK/WAn0LGyc4KyscPAvio7scGI/R8nY+Q0Z2RIhAbzisQ5uPadSPQjNgTpJVrZAygA952EIVXrvHDL2mv4Wanphh1ruVJcEur2zr38fvoA9VM6ug==",
        categories = "accessibility,people",
        tags = "child,childproof,children",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Baby,
    #[cfg(feature = "backpack")]
    #[strum(props(
        svg = "eJxtjL0KgCAUhV/l4C7p5SoG5tzS6tAmNDg2hM+fEZSI3OWen+/4M10ZxyI2BqmoVWIwVD0Nlpxdq8GlFgj0GrJ+q221pF0EPz2bwX/LM2zkv4Zay9RqUKEB50C6SNOR3JFmSGqV3ThwTXAD6no85Q==",
        categories = "gaming,photography,travel",
        tags = "bag,hiking,travel,camping,school,childhood",
        contributors = "karsa-mistmere"
    ))]
    Backpack,
    #[cfg(feature = "badge_alert")]
    #[strum(props(
        svg = "eJxtzsEKwjAMBuBXCbk3NrVre2j3Br6At4GCgogHD+7tTWjcHIxC2vx8pKmv6X2DS8PTkcoAhVKYIkTwchgi5eKkZFizRFmfW2RlyXxnS2BDNsgJGf4nub3vvLJ0xrEedNWxPu7PK8zcsCDMoSEHhA/b3Xuxqsz2kDybT6vXMdL//BcdyziD",
        categories = "account,social,shapes",
        tags = "check,verified,unverified,security,safety,issue",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeAlert,
    #[cfg(feature = "badge_cent")]
    #[strum(props(
        svg = "eJxtjz0KgDAMha8S3BsTTX+G6g28gFvBwdFBPL+tlmqhBEJ4+Xh58Uc4d9imbhnRaXBohiAgQLEYBK1TsVn4NIM2jTWUW9HoxYqQTSpIRUT/nVTrHCXMrN3s+xR19iUwD2AvptZGowBTfoQfJyncDdeeMrc=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,cents,dollar,usd,$,¢",
        contributors = "danielbayley"
    ))]
    BadgeCent,
    #[cfg(feature = "badge_check")]
    #[strum(props(
        svg = "eJxtjksKgDAMRK8S3De2Nf0ItTfwAu4KLtwILrw/NlirBQkMYfKYTDjSucE6dfOA3oBHqxMBgcyjgNB5kcXB61l0vLZQkerJG6tGCWkgkRHzTRJ/7yRjduli6LlqDE/hfQSlgYcE1fMFaJYpxQ==",
        categories = "account,social,shapes",
        tags = "verified,check",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeCheck,
    #[cfg(feature = "badge_dollar_sign")]
    #[strum(props(
        svg = "eJxtjrEOgzAMRH/lxJ40SY3jIWXu0rUDWySGjAz8v0hEZEBCliz7/Hx2WvNWsHyG39vKCLEcMoHganiQjWJqijg1trGVd6gn1dyBqdBNbpCpyHh1Mk/nXMN4Hqb0aq9OSR/2DCmGc0DoN6tXIe2bQl952gzw8med7DbPNnI=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,usd,$",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeDollarSign,
    #[cfg(feature = "badge_euro")]
    #[strum(props(
        svg = "eJxtj8EKgCAMhl9leHepOTUw36AX6CZ08Nih9yctsQQZjPHv498/f8YrwbGybUZH4NCoqEGDyCVBo3U8NwufZtCWsYdqa5p4sSZUkw7iGaG/Ex+dEwUzOwt+KlGDb4EtSJVosJAEC+r6hnx8CFUDb9hNMrY=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,€",
        contributors = "danielbayley"
    ))]
    BadgeEuro,
    #[cfg(feature = "badge_help")]
    #[strum(props(
        svg = "eJxtjkkKAjEQRa/yyT5lhs4E6dzAC7gLKiiIuHDR3t6UhrQNUlT94vNqyI/6vOA0i72l6BDJmzphgmqhMVGIspWA1fMUuN1CvQxPfbFh9CUbSDbE/W6S/84pxvxBlLzjV0seDydSCala2I46ihb6qGBkMznXqdv1fsaiZ6GNwGJYSWmBF7ehqf5o45ksbw/tOd8=",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeHelp,
    #[cfg(feature = "badge_indian_rupee")]
    #[strum(props(
        svg = "eJxtjz0KgDAMha8S3FMb7U+G2ht4AbeCQxfBwftji6W1UAIhvHy8vLg7PBHObdpXwRpYmCUoUCBTEShhGVOz0DQjbB57qLSqyQ+rQjHpIEyI/jvh6JzMmDkm7+Yc1bsamIEjD3VaRouLViCLGilS/TEVNvYF3tE6rA==",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,inr,₹",
        contributors = "danielbayley"
    ))]
    BadgeIndianRupee,
    #[cfg(feature = "badge_info")]
    #[strum(props(
        svg = "eJxtzsEKwyAMBuBXCd7N1FnNwfYN9gK7FTbYYIweemjfvgkR20IR8kv8jJZpnD/w6s3jjtQBYQpjhAiOl4eImSyXDHsvYZbtGdXSek5Za9QhJ2SZdMdJ9uo5Jyw9zVBu8tWh/L7/N6y+Nz4ZWCQDZ9BcNRkLq1gP0fndy31STk1v4qE4VA==",
        categories = "account,accessibility,social,shapes",
        tags = "verified,unverified,help",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeInfo,
    #[cfg(feature = "badge_japanese_yen")]
    #[strum(props(
        svg = "eJxtjk0KhTAMhK8yuG9f09S0Qp838AJvJ7yFG8GFeH5b/McSCGHmYzJx6ucB/2/VsQ41ghbbOziYNASnfVBpeVyaaJ/PJ7SvUzMbdgp7yANSCanvSar0zmRMflUbP7lqG4/CY4MABi++4JEFEVjx2+sakB2kbMjNWAHz+T9x",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,jpy,¥",
        contributors = "danielbayley"
    ))]
    BadgeJapaneseYen,
    #[cfg(feature = "badge_minus")]
    #[strum(props(
        svg = "eJxtjkEKgCAQRa8yuNfUTF2YN+gC7YKCgogWLer2zZBYQgz8P3wefybswzHD2LKuFr4BL6weDBiQOAqMcJ6jOHgzKxytJZQkZ/LBcpBKCogj0nyb+N85SZjtWQwVvRrDumwTXLplSjO41OMnukej2BJLVLwBluUsxQ==",
        categories = "account,social,shapes",
        tags = "verified,unverified,delete,remove,erase",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeMinus,
    #[cfg(feature = "badge_percent")]
    #[strum(props(
        svg = "eJxtjksKgDAMRK8yuG9stU1bqN7AC7gTXLgRXHh/bPFbLIEQZl6SCdu0L5i7amjJGTjiZtLQkLEUNFknYrN4NSabxhy62qPJE3uE60gGiYiY7yVReicTxmPVhzpF7cMdeFUGXjD4bw0efiGpCk5cUibzDhktODI=",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePercent,
    #[cfg(feature = "badge_plus")]
    #[strum(props(
        svg = "eJxtzkEKhSAQBuCrDO71qc9GF9YN3gXeLigoiGjRom7fTIoVhDAjP9+MxqVdB+hq8fuqUEFQaFsHDjQdA075IKl4uDJUnq9PlEvJdGIlyEseSBKp7pvk23OaGf5FEz/81SZO49zDZmphrIDNpr5zR+qUB7asst3NzfDMadIoFnsAQrA3yQ==",
        categories = "account,social,shapes",
        tags = "verified,unverified,add,create,new",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgePlus,
    #[cfg(feature = "badge_pound_sterling")]
    #[strum(props(
        svg = "eJxtjjEOgCAMRa/SuFMBoWCC3sDVwY3EgdHB+8cSCUpCCG3z+/rbcMU7wbkM24TegkfS0YAByU+BQecFBwefRuhy2UIlVE2+WBWKSQMJRuzfSfTWyYzRMaxhzKeuoR7sQelkOg3Fc7TPaKNGC/m/Vpz7NpRcbTxHfDr+",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,british,gbp,£",
        contributors = "danielbayley"
    ))]
    BadgePoundSterling,
    #[cfg(feature = "badge_russian_ruble")]
    #[strum(props(
        svg = "eJxtjksKwCAMRK8S3Mf6/4D1Br1Ad0IXLrsoPX+Viq0ggSFMHpMJZ7oyHCvZJHUaHDUiKVDAynBQ1DosYuHzDLV1HaEm3WMv1o0WMkBYEP1Pwtk7VjGzkxiWWjWGXtgDN1nPDyLrJEC0LgxVRnn7zj59MTQT",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,rub,₽",
        contributors = "danielbayley"
    ))]
    BadgeRussianRuble,
    #[cfg(feature = "badge_swiss_franc")]
    #[strum(props(
        svg = "eJx1jj0KwCAMha8SumuN/wXrDbp26CZ0cOzQ+1OlYhUsgRBevrw8d4U7wrlOm6BWgaWaBwkSWCoESY0lqRn4NE1NHnuotKqxF6tCMekgkhDVOpHRO5YxfUzezTmqdzUwIqDZbZQ/Ox7FYLMA6ubkAbZuOB4=",
        categories = "shopping,money,currency,account,shapes",
        tags = "discount,offer,sale,voucher,tag,monetization,marketing,finance,financial,exchange,transaction,payment,chf,₣",
        contributors = "danielbayley"
    ))]
    BadgeSwissFranc,
    #[cfg(feature = "badge_x")]
    #[strum(props(
        svg = "eJxtzk0KwyAQBeCrDLPXqvWvoLlBL9BdoIUWSukiC3P7OIwxCQRh8D0+RtN/nN7wzHi/yuggSm9GCxZUPRqsDFHUEWDrvAx0PaI2eqeY9aItOSBRidtvEmfPKWL+gUO60FeH9P38XlB0Ru0Qisl4Q5gNx1lTrJRQo9x1UjgWjitdANyNN3s=",
        categories = "account,social,shapes",
        tags = "verified,unverified,lost,delete,remove",
        contributors = "karsa-mistmere,jguddas"
    ))]
    BadgeX,
    #[cfg(feature = "badge")]
    #[strum(props(
        svg = "eJxtjjEKgDAMRa8SujdWbZsMtTfwAm4FB0cH748JlmpBAp/wePwkneU6YF/MOiMHYIxT8eDByYzgkdhKELwsIunaSzUac4/WQC3pJCtK+DbZv3NOtbiZnAZ9Nd/PhiHB",
        categories = "account,social,shapes",
        tags = "check,verified,unverified",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Badge,
    #[cfg(feature = "baggage_claim")]
    #[strum(props(
        svg = "eJxlyz0LwjAQBuC/ctwezJ2ljZBkdnHtXtJgAg4Sgh//3pxKq5Qb7ut57XWqCWaHJ2Ygc+wnBgbdihQrHodl17KjtztJeLvkaADqxu7fJUW/B+Ab6TVbYqhwz3NNDmmPkGI+p+rQIDwd9giP91xaI0mJ9zbkEi4RQiOs27c1hCDGCPq8N0zA4atX9QKmDEAu",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "eJwlikEKwCAMBL8S8gBbxUsh5i8lLbSgIOJBf6/Rw+4cZkj+IvGFEtCeCNIm3WRfZDq2Z8p3/eAJmLy5QGe9ceu0UssDJmwUSg==",
        categories = "account",
        tags = "cancel,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,circle,slash,null,void",
        contributors = "danielbayley"
    ))]
    Ban,
    #[cfg(feature = "banana")]
    #[strum(props(
        svg = "eJw9jrEOwjAMRH/l1D3GduI2lUKXzvwAWxUGRgb+X1wAVZEdyz6/c3sd7yce1+lWYLlnieSoDFP4ERIYoXyGipi2dhkbWzv3Qixgi9S1U+rJmFBljjSzhyWZ78ZmgA78QXaG08qd2J9BEK6Y6V6Ec1JUyspiJ95Apf+Tdh0WMLGSvtjMA5JTdj+v+wBeFCzP",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Banana,
    #[cfg(feature = "banknote")]
    #[strum(props(
        svg = "eJwti8sJwCAQBVtZtoBEPUgOagcWEVbJCjkEEaLdx09u7zEzJkcqkKtFhdAsaoQ3hcL9CwSO6eJiUXY4FWf2EThDKdMdgdqCefZU5+vSws48Z2EIFr0GqXgT0svjX0Mb2H2ByCQy",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "bar_chart_2")]
    #[strum(props(
        svg = "eJxljEEOABAMBL8i/QBtRBzKbxwk4szvKSIRp007s8sl16QaBUAPqmMAMjPlntlw/SNr0SIvWaA9jGB36XZfV6Dbrjuz9lMHlkchAg==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart2,
    #[cfg(feature = "bar_chart_3")]
    #[strum(props(
        svg = "eJxtyyEKACAMQNGrDLvIGOKE6Q2sdsFgNIjnlxXT6n982eMsmMU1ArrIC9lVCVqrfEMGTD1bQirREF2upy8PSfIeAA==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart3,
    #[cfg(feature = "bar_chart_4")]
    #[strum(props(
        svg = "eJxtySEKACAMAMCvDLvIGOKE6Q+sdsFgNIjvlxXT6p3scRbM4hoBXeSF7KoE1Sr/kABTz9awTjRG43r68wBJcB4A",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChart4,
    #[cfg(feature = "bar_chart_big")]
    #[strum(props(
        svg = "eJxFzMsNgCAQBNBWNtOArp/oAejAIowQl5shxE/3ghG97GFm3qptjkJWY2qp3XkUHmFUlVOjglsihVODQekOoMPbKBod6EppDRLnV4m5SyrvX5XqHsX+qsy5eT5y/7EbhjgmzQ==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartBig,
    #[cfg(feature = "bar_chart_horizontal_big")]
    #[strum(props(
        svg = "eJxljEsKgCAURbfyuBsIs8iBuoMWESk9ZyGPPrtPwWjQ8B7OuXZfhCk4zJr0oQwrA2+7Sr3NcRXimDYWhwGULwcFuh1G0JmCcNk9qOCpVtX/V0VXuknvRYu/6gFzwyah",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartHorizontalBig,
    #[cfg(feature = "bar_chart_horizontal")]
    #[strum(props(
        svg = "eJx1ybEJACEQBMBWDht49g/U4LQDixAMNjQQ6xcTI01nrNdBackVFZ2IRHTZvq3ZzgWB5yNA/Nfx1OML/ZsdfQ==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    BarChartHorizontal,
    #[cfg(feature = "bar_chart")]
    #[strum(props(
        svg = "eJx9izEKACEMBL8i+cCZcASLnL+5QhBr/b0xsROshmVnpJb2h0EfYITQUUlKcg7dFCHLs7QsJtuZwKJ3N+nisjvsBfKhTpQ/IQQ=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "colebemis,ericfennis"
    ))]
    BarChart,
    #[cfg(feature = "baseline")]
    #[strum(props(
        svg = "eJxlijEKACEMBL+y2B93CUewiP7ARwgWaQQL/49aaGM3zIy23A0luPSDPyNxUd/lou5SBSSQhxiT+B6Sn9r8CQPOWxbq",
        categories = "text",
        tags = "text,format,color",
        contributors = "ericfennis"
    ))]
    Baseline,
    #[cfg(feature = "bath")]
    #[strum(props(
        svg = "eJxtjUELwjAMhf9K6L11SZt2g3YXz169FxQmiHgQcf/eZFUYY4TwkvC9l/ysrwkuxZwGiBAdg3dcUVS707JoHR+Di70HD0EAnQIEx2dMlYAaJ0oT0vpg6W3ZjPmgX8Z8vz2uMGMxbGCmYpKBj2zYicraK6nMj9QbUWNE1IjUnKIbFpe4hRlaXGoo4U4s/l+vPBv4C10MQoQ=",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bath,
    #[cfg(feature = "battery_charging")]
    #[strum(props(
        svg = "eJxljL0KwkAQhF9l2P7wZvNzzd3VNj5EQGGFKBYi5u3ds0gIYYvZge+b/JrehmuRCwck46RQRD966mfcevDPgkrNp+bUvJoj0rlfwRj2YmxDxqP3IJFCh8H6ucVGzPfnDQuLkIKvFlEVLJ7svPPfHW5Y/QGHeS79",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    BatteryCharging,
    #[cfg(feature = "battery_full")]
    #[strum(props(
        svg = "eJx9jlEKgCAQRK+y7AVKi/pRL1OSQvQhQnr7drUif/paZubNMirYJULSKBFCLsdZv7moUfQI5MwU1Pz0a3TkT2hUxz2jdn9YyJLMASEJwohLst5MWgiGGbthDqc3Kx2W9UWLfhkew9Uy6hfmcMRmFOkHvgCnsj2O",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "eJxFi0sOgCAMRK/S9AJaTHCDXEaJkBgXhES8vVMhumnn88blsBa60lbiwmKZYkh7LNAjU74XNkz1vbk9RDN7N+jOuyOdgaqg0cpgNoE33SMXUVixDvfwh+Ft29gPfQD6RCc4",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "eJxdjkEOgCAMBL/S9ANaTPSCfkaJkBgPhET8vVsCHDwttDNtbXR7oviubJiecCS/ssxM3oXTJ7xHplyaQBamWD6bHdTb7BVuR9mgpgRSJqQgBZ6UOmDFKtwgbWKPunNXfmid0xVT75GSDf4A264yXw==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "eJxtjTEOgzAMRa9ieU8bu4gsgblLD4HaSkFCiAEhuD0/BAIDyvAc5b3YD80Y6FfxRwpyQRslJYsjoE7lF5PBbHALRrn2z1jUPncluXeRM2tylr5Adqm6tv/TIhWLMC0KvphmUBWUjZCjdsi7FCOXHLGpAW9c97CS39Ouszv8FedPPBA=",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "eJwti1sKgCAQRbcyzAZqDOpH3UxJCtGHDKS77075dR+c42valWoL7Jhq/+IfTzk0B5aVKadyZkWfmYBsHP1kXvRXuRM1gQCjOyALfDc2fhGDDYsvL4McZw==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    Battery,
    #[cfg(feature = "beaker")]
    #[strum(props(
        svg = "eJxtizsOgCAQRK8yofezu0AsVm5ga09isaWF8fwKBaEg08zMy9M7P4Zrd4efA8QouKRLOZM2FCEvxcxgrDV/s63fE58yFMkbcSMfJSwcOg==",
        categories = "science,gaming",
        tags = "cup,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis"
    ))]
    Beaker,
    #[cfg(feature = "bean_off")]
    #[strum(props(
        svg = "eJxtjkEKwjAQRa/y6b5jZjIZE6gFD9BDlCooiLhwYW/vtCl1IwnzSfJ5L91rfN9wOTVDQZlaMvXdMiVhKklbIQ0CJrZ0NhjCujJERo5UDHXW60KlVeKm7w4Ltu92OAc6JiQKJe4YdgjyFOAOZlczXJ5dbjnCyOwPKFFUhuNMRoVuYiPNR/+lhDysb1h7o1LghDo3JYVYlqk/+OP+vGLmUyMNPjVm8ViONb26lPovsadCYQ==",
        categories = "food-beverage",
        tags = "soy free,legume,soy,food,seed,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BeanOff,
    #[cfg(feature = "bean")]
    #[strum(props(
        svg = "eJw9TrsKAzEM+xVze9yzYzs2pAelc3+gW7gOHTv0/6nTIfiBBJJQ/4zvG17X7UE7kikYavg9MFSgoTSHQBNwrAYBcZZkuYVQmaaqMMrOQNN9MzDY/+PAfDZslZOQFEM2n4BkLFUhQq+aYRnx3I5+mXWOvkopViHIasZDQGZUniK3KPl9WX6EayrH",
        categories = "food-beverage",
        tags = "legume,soy,food,seed",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bean,
    #[cfg(feature = "bed_double")]
    #[strum(props(
        svg = "eJxtyzEKwCAUA9CrBHfp/0HEwXqDrt2FDo4dyj9/7SJFJEMg4eW7Pg3X7g6CYj7V3pAeBT2bxv8AWnIlbx8qedAAlTPOkpMMC6lEsLg4CE2NMp4XTUop6Q==",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedDouble,
    #[cfg(feature = "bed_single")]
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrPLoXk7RKhtgbuLoXHDI6SM+vLipF3vbg+22vh2Obw5Ig1KJWgYCuMSSKc/4ekKah2HCjYg8dwbROvaRO5h+ZwOr8Nk+HfSK9",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BedSingle,
    #[cfg(feature = "bed")]
    #[strum(props(
        svg = "eJxtijEKgDAQBL9y5ANmF4knnPmBjwhYXGkh935jIynCFgszY3d7XK4jnZQ1UFK15UPVBqEObRRK7kN/BvK0xOacmSIa+89fUJMgDQ==",
        categories = "furniture",
        tags = "sleep,hotel,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bed,
    #[cfg(feature = "beef")]
    #[strum(props(
        svg = "eJxVkM2KAjEQhF+lyD29050fe2BGkL3uvsDehqygoCDiQd/e7gkiElJUmkrqI1M7Xttpj3afAwuVgPaYg7q5zsEH2+mrZ7bTZbkd8D+HX09Clmrqe/AVK4kgU22RiZGIU6SNmhmjHdSSg+4SUo+jgLVlM0rZbiiYzWRKu49n0bv+nMP73xRnVg9CiEfrLYaTvcVkvQdKDPHB2GWdRqGKQvLNhTJkILFiGPnGZHnhrTE1rFp/xHJc+0+sBE9+MkHY",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,meat,bbq,steak",
        contributors = "kemie,ericfennis"
    ))]
    Beef,
    #[cfg(feature = "beer")]
    #[strum(props(
        svg = "eJx1TjEKwzAM/IrILjWSo9iBND/oVOjQzaSDx0JK3l8pLqHQFCH5zJ1ONz7zq8Dj3Fw4AnPhHCBAa8XWfUFupvHkomncpQOwrP0BweEv00EkndFckanrSDEA6YKCDn0wRbHHmjTbBO9PEtSZYnJMGitFeh3MCMRigyzilEX36U5YVfjrBRZjM7OLzmOV3Q9SK6SVJYsdabcyVNL3H+WW9sU3RR5Lvg==",
        categories = "food-beverage",
        tags = "alcohol,bar,beverage,brewery,drink",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Beer,
    #[cfg(feature = "bell_dot")]
    #[strum(props(
        svg = "eJxFjTEOwjAMRa9ieY+x4yhppCQLCwuHqAISSAyoMMDt67RDZflb1n/+Lu/5+4BbxatkCiCB8tkzeZBoqxeQtOtFP+o8qMudnZKCp+QiRGtKwCCkZBxlUmzlNGJbOcJ5XMhsfoBNeCu1J3zw/bn01x2WiorQfxVlsvmvOA1md9sK+l0p9A==",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder,unread",
        contributors = "danielbayley"
    ))]
    BellDot,
    #[cfg(feature = "bell_minus")]
    #[strum(props(
        svg = "eJxtjbEKgDAMRH8luDcmTa0VqrOLHyE6OAr6/3hV0EVCLpC8u+R9Pjda+2rSxIHUL5zI0J4jNY+OdpjzZK5bxBkbtq2LFNEKUsAHzMDgqyHXJXLIX7AUi87KHR4UkbvgIvnjG0pbfA8XfEgljg==",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder,delete,remove,erase",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellMinus,
    #[cfg(feature = "bell_off")]
    #[strum(props(
        svg = "eJxtjcEKhDAMRH9l8N5s07hNha6wNy9+RMGDF0HQ/8cawYsyYQgkbyavZZ8x/ZoxkUL+ERG+isEJqQQmgZk3UcS36fPnpPp8s6xgHWQTFyCuKy1FhdmVReKY9A30ll+YuhZmV49Q3Z7/S0CVr3PfDrAyLRY=",
        categories = "notifications",
        tags = "alarm,notification,silent,reminder",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    BellOff,
    #[cfg(feature = "bell_plus")]
    #[strum(props(
        svg = "eJxtjTsOg0AMRK8yol/Hn2U/0oYmTZocAiUFZSQQ58dAQYMsjyXP87j9x2XC79l9pJJBIpWXMgkkUYT6zKe+bbagsFC/HMxRpRwSkreAIVRJfVcod0N77KlDu7J5P5DRqYhD+CjzH3zH9yhTujMK+vUyNuQDLUU=",
        categories = "notifications",
        tags = "notification,silent,reminder,add,create,new",
        contributors = "mittalyashu,ericfennis,danielbayley"
    ))]
    BellPlus,
    #[cfg(feature = "bell_ring")]
    #[strum(props(
        svg = "eJxtizsOwzAMQ69CZJeqj+vYgJulS5cewkiHjgWa+yOKh0wBIUoQH9uvb198HtM7o/SMDAkp1CCrYIajHvPyv5PBqU5Lux2lpZ1VFXaYduWaMEyGnOO64BPsaVwin2G4Dy8XnMV7FTJ24kIpllE+uR0G8yw/",
        categories = "notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "ericfennis,danielbayley"
    ))]
    BellRing,
    #[cfg(feature = "bell")]
    #[strum(props(
        svg = "eJw9izEOgCAQBL+yoQfvOIOSnNQ2PuKihaWJ/j8CBZnsZosdfey7cW3uSFgtIYEqDI6gk7BAkFt2ecVHiM+u6NSkokNlCoLIxiHP6EUdCXWN/w84fxis",
        categories = "account,notifications",
        tags = "alarm,notification,sound,reminder",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    Bell,
    #[cfg(feature = "bike")]
    #[strum(props(
        svg = "eJxlzD0KgDAMBeCrhO5V+ocObW/g6i5RqNBBioPe3qZVUBxCHuHLs7gmjAvg6ZjoGsMgOaZo45EvfU7ethV5e+M3MSU8339bamqtqPCjtmkPMDs2CAnUMAodueIKdB4JKkjSpPwFv14uIw==",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "eJx1jEsOgCAMRK/S9AIKIV0BN/AQRohlZwjxc3sFP9EEV03nzRsd/ZBgNSgUQjyuRNjOj30YORkkhCW4xAYVWt1kwepbo1L/aV+DjzT1icEZ7Ahky2UtRy8gFIg6IRCK5Ux16Yt2Ox88rw==",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "eJx1kM1qw0AMhF9F7H2n1v56wc6lZz9E2RZaaKGEUtq3z8gJJJDNwbKsn/k0XvrHsX++Sf9bnQYnx9Ux9n9+KZo7LE/ngcPy/fLzLq+r2wqqRKTu0SQgyyQZgVkQdp4LK439Kg1FglCmmIytX0W+Zi7rBBVFwnw/sGlF3DG3FH+hsJK8MrcaMcXUkAcczcYgyD8E7XoT5k714imc+LI0ofqM6Hd3VlKhhJWNO5Ki3Yj2O7S8sZW5yB+n3qxYiDINvQuvovXqyaJPHl/MIG8YmNwyhQ3cwYCZk/ZkuZ0+AQtodIo=",
        categories = "science",
        tags = "fallout,waste,biology,chemistry,chemical,element",
        contributors = "danielbayley,ericfennis"
    ))]
    Biohazard,
    #[cfg(feature = "bird")]
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/l1D3GdtLYQ+jMAGv3SAxdkBgQ308KqB0aWbJkn33vyrO+FtzPw00ybCGWYSqndTmVTYqUIH4RrQ4Hf8uDz1YT0m8ORupBKV4VykePhzIMChrD2joQ4cZ4x57S6EY2ztoLZ+2tZuR/rkiegjDl/fYDj1o5KQ==",
        categories = "animals",
        tags = "peace,freedom,wing,avian,tweet",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Bird,
    #[cfg(feature = "bitcoin")]
    #[strum(props(
        svg = "eJxdj00KwjAQRq8Sus9nZjKZH6g9QXsI0YULCy68P6YRRNzNB4/3mPl5ed3T7TxtRDC1RIHicRUEC1w9KUiyonBLBCbtt4fseYw0xto6mchRxPaGKN7RIpxRpRsRthOa1uydk687/t1tuOvH3Ya75oqQDO3JBmoHwquDIwlYf2s95vmIbQbxxAVV5VFBrJkMxjYt8+n4d3kDLn40Kw==",
        categories = "brands,currency,development,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Bitcoin,
    #[cfg(feature = "blinds")]
    #[strum(props(
        svg = "eJx1yksKgCAURuGtXO4G6lcjBXXcpEWEBQYNQiJq972gJjY937Fzt0TqHbeSZIRmb4srefuCKKlufgDICk4xEWWGNKGKEBlRJFeoD8KYwjRQ2B3DMCXHgilsju/nUX8AbjM2UQ==",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[cfg(feature = "blocks")]
    #[strum(props(
        svg = "eJxdTbsKg0AQ/JVh+yOOGpLi7uo0ae1FxbUTOXz8vW7lg4Ud5sGMn7omQbuh1xTkI5jWIBTYLwVbkEKwDG1Sc6N/WT76sU6KNsifGXJW35ogMjtHx1955eDM/BQOpN4Fx9m9HxWFjdlM3AFCICab",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "eJw9i7EKgDAMRH8lZBdNpNYh7Rc4uwsKClUcHOrfm9YqhBx3907O6VphdrhbsECNXmXAjBz0D5pY9FInyEvYjgUiOaQe4WZVRoiqTOope4UTVuAPKmUet++m+9EHdOMgvw==",
        categories = "connectivity,devices",
        tags = "paired",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothConnected,
    #[cfg(feature = "bluetooth_off")]
    #[strum(props(
        svg = "eJxlTDEKgDAQ+0roXvWKxyGc/YGru+DgoOAgvr/p0qUkIZCE+Ht8F841PGIQiwrdJd3VQ/axttnbJoGYyL7bZB4UC8Uj40HUn0kbFrcHG1k=",
        categories = "connectivity,devices",
        tags = "lost",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothOff,
    #[cfg(feature = "bluetooth_searching")]
    #[strum(props(
        svg = "eJxtijsOgCAQRK8yoQd3kV+BnEBbexILCk0svH9cY7QiM3nFzMtnvRq2SR0REUxS7eFXuwtnWaIqeXikkj91sWTSCHbC6uBAb7Q3IXR0TmDbDPH/3cd/HX8=",
        categories = "connectivity,devices",
        tags = "pairing",
        contributors = "ericfennis,csandman"
    ))]
    BluetoothSearching,
    #[cfg(feature = "bluetooth")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNVcwVzA0ACJdUwXTMKMcIOkDFDFXsrPRBymyAwDtywrA",
        categories = "connectivity,devices",
        tags = "wireless",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    Bluetooth,
    #[cfg(feature = "bold")]
    #[strum(props(
        svg = "eJxlyiEKACEQBdCrfOzLqswuhtFs8QI2wWA0iOcXETTIq49ragXZiqAISicCQS6P8X83wvE7j+MzP2h5z7jrAIv2FT0=",
        categories = "text",
        tags = "text,strong,format",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    Bold,
    #[cfg(feature = "bomb")]
    #[strum(props(
        svg = "eJxNjUsKgDAMRK8yZG819YNC612kCgoKIi709nYUioSXLB6ZcWE5wjoh3F60FIQrXhUcXjrpXf7p3u3DOWP0smlnahA1bRYZrKlAim+y0lRrFA2hVLwrSRTULf9ZwOBfvLWwUTO+TvoBOFonrw==",
        categories = "",
        tags = "fatal,error,crash,blockbuster,mine,explosion,explode,explosive",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bomb,
    #[cfg(feature = "bone")]
    #[strum(props(
        svg = "eJxdj70KhTAMhV/l4J7ctJDbCuob+AJuUgcHBwffH9NUHKQ05Pc7yXCu145t7OaQEKRwIjaH/z0EkRWyVusegn0hBauHliBWespvizVIEXAOTsoNdFBCKgY3hYanL749LRzrjInEWJU+CzidjE5Oyg20dNPwq7dMNyD0KbE=",
        categories = "animals,medical,gaming",
        tags = "health,skeleton,skull,death,pets,dog",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Bone,
    #[cfg(feature = "book_copy")]
    #[strum(props(
        svg = "eJxtzLEKgCAYBOBX+XGXusOkwZxdWt2FBseG6Pn7CzIIueU4Pi7s5aiyLWalwGdXKJRRA6FlBUwMw21iaHISuPRKPNrVHqR+zgn4U2T/rRqr7QTbwwWrsiO7",
        categories = "development",
        tags = "read,dictionary,booklet,library,code,version control,git,repository,clone",
        contributors = "danielbayley"
    ))]
    BookCopy,
    #[cfg(feature = "book_down")]
    #[strum(props(
        svg = "eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v81SEDkO7njw4l2eE47FbCNQcFItycpOQIstBJO+zFgZc9vlS9FKYybFQU0pdh8xkN/nP7kCEILXWN/xC535ISE=",
        categories = "development",
        tags = "code,version control,git,repository,pull",
        contributors = "danielbayley"
    ))]
    BookDown,
    #[cfg(feature = "book_key")]
    #[strum(props(
        svg = "eJxljcEKgzAQRH9l2HtsdpsUhUToLZd+hKSFFioUKaJ/7wZBEVnmsMzMm/Dr/m88Iz0cuKn8aNjfpfIosnqMW/kSO2rDpaTbsHXEoh7ZJY10x5I1PondK/kz5O8LeYqkJOQ5Uk0YIknJrO4O7hUsxilNdd7tucFVR3izFjuLMrk=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCDkk+b4//pVKhingzQJdeqkdyZV7AW3TisDpFMng6E9Kj35zuN2lnmND0lEynUTeKev8KJDn5ZlLQEH4BCRGeC9TyQEHhG9Ah7DqXi3ld49oAHe3iYH/6RZM5S3+Bwz7Mf0=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley"
    ))]
    BookLock,
    #[cfg(feature = "book_marked")]
    #[strum(props(
        svg = "eJxVTDkKgDAQ/MqQXt1dTUSIATsbHyEoKIgGFMHfmwQb2YMZ5rB+vBZMrRoqcJPrO2PdSa4Rj8IwTGS90C3UBzz+Vcp00JSzRWxy1h/bs637DH+s+3W2igmC8OKWqMEmQQNJoc/uXrRlIYY=",
        categories = "development",
        tags = "read,dictionary,booklet,library,code,version control,git,repository",
        contributors = "danielbayley"
    ))]
    BookMarked,
    #[cfg(feature = "book_minus")]
    #[strum(props(
        svg = "eJxVjLEKwCAMRH8luGuT0AhCKnRz6UcIHRw7FL+/ySKU4+COB0+f/g64j3DtQCXJjCQnJwEvWgiyv8Y4GZvt/qcYxViourmp6vIVIBx5gQ/5rhkD",
        categories = "development",
        tags = "code,version control,git,repository,remove,delete",
        contributors = "danielbayley"
    ))]
    BookMinus,
    #[cfg(feature = "book_open_check")]
    #[strum(props(
        svg = "eJxljaEOwzAMRH/lVO4strOkICseKS0YmzIQMmlgyvfPntSS6iQT371XP89vx+s2rTP0LoOvvTQOBREKDmpXt9IiSRDiMFPyPKalXny51H3/5gwWeKxw/q8iyJt2ys1ZiJTw5yENTiZwqQnJhKS9DGvpgfkBY3kofQ==",
        categories = "gaming,development",
        tags = "read,library,plain language,done,todo,tick,complete,task",
        contributors = "schmidt-oliver,karsa-mistmere,ericfennis"
    ))]
    BookOpenCheck,
    #[cfg(feature = "book_open")]
    #[strum(props(
        svg = "eJxdyzEKgDAMheGrhO7Bpgm6xM4uHiLgkNFBHDy9qUtA3vDgh09PuxyOtewN2GcTEKgxipebxBj4CxUZeWtP6ToN0zXloJi24s8ShPUl7QuPERw5",
        categories = "gaming,development",
        tags = "read,library",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    BookOpen,
    #[cfg(feature = "book_plus")]
    #[strum(props(
        svg = "eJxtjLEKgDAMRH8ldG9NgqkUasGtix9RcOjoIP1+k0UR5Di448HLZ7s6HKvbZ6AUZHiSjYOAFTUE0V5lHIxVd/tS9KLMlTyZqeTHl4Cwxx9ADMt4wQ3I/CA4",
        categories = "development",
        tags = "code,version control,git,repository,add",
        contributors = "danielbayley"
    ))]
    BookPlus,
    #[cfg(feature = "book_template")]
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8luLc2p9EKVXBzcXUXHDo6SL9fu1VICRmSd5dcuM8n0jU3OxwB0aBZQpuXSygRS6rAnniycrDoRp86BbAnRKTKPT4mzYPsqYAq4VEl339sg5UTVii3+4rJGdm8GkqSsbL+1UOeCv0LdedZwg==",
        categories = "development",
        tags = "read,code,version control,git,repository,dashed",
        contributors = "danielbayley,jguddas"
    ))]
    BookTemplate,
    #[cfg(feature = "book_up_2")]
    #[strum(props(
        svg = "eJxtizsKgDAQRK8y2Ed3N64fiIKdja29YJFGsJCc36QRJGGYYnjz3H08HudUbS14rDUY1kVqRSrFMLq0qtk16Tq7T+AB4iUIrfFx/B0yugqVJAHbvc/JNYIJ1ljEFLFm9AVAAS9x",
        categories = "development",
        tags = "code,version control,git,repository,push,force",
        contributors = ""
    ))]
    BookUp2,
    #[cfg(feature = "book_up")]
    #[strum(props(
        svg = "eJxljLEKgDAMRH8ldG9NUqMUasGti6t7wcFFcJB+v+lSEDkO7njw4l2eE47FbCNQcFItycpOoBU1BFN7mbEyZt3lS9GKMpPi0Ewpdh8xkN/nP7kCEIK3HjQdv54tISE=",
        categories = "development",
        tags = "code,version control,git,repository,push",
        contributors = "danielbayley"
    ))]
    BookUp,
    #[cfg(feature = "book_x")]
    #[strum(props(
        svg = "eJxtzDEKgDAMBdCrhO6tSWmUQiy4dfEQBQcXwUF6fpNBUJAQSHifL2e7dthmtyagHLh74iUGBlvUIRjtqxF7xKp3+yp6VnNFBmsq8vQdlDQzeQb+wWwGb7sBl4ghLQ==",
        categories = "development",
        tags = "code,version control,git,repository,remove,delete",
        contributors = "danielbayley"
    ))]
    BookX,
    #[cfg(feature = "book")]
    #[strum(props(
        svg = "eJxVjLEKwCAMRH8luGuT0BQKVujm4kcIDo4O4vebjHIc3PHgxVFnh/a5cgO9QZYn+TkIWFFD8NjLjIsx664nRS/KXIqXmdIG3dMR3A==",
        categories = "gaming,development",
        tags = "read,dictionary,booklet,magazine,library",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Book,
    #[cfg(feature = "bookmark_check")]
    #[strum(props(
        svg = "eJxNjDEKgDAQBL+ypD+8W05C4MwzLOwCFmkEC/+Ppgmy3Q4zcben49zSZQU0yeKS4fvaCEK/GSjspv8DPFKNZbg1ZqHAdDC4+MQvVPgXCw==",
        categories = "account",
        tags = "read,finished,complete,clip,marker,tag,task,todo",
        contributors = ""
    ))]
    BookmarkCheck,
    #[cfg(feature = "bookmark_minus")]
    #[strum(props(
        svg = "eJxNi0EKgCAURK8y/L3k/2QiqMdoLxQYVLSIsE6ftopZPGZ44490ZkyBNnYQVlb1yqIfTRIIdA1DlGTW/wFy8fBQ9F27R78u+4zCgdgQigRyhLtVXSkfq9us+ALrkxsG",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkMinus,
    #[cfg(feature = "bookmark_plus")]
    #[strum(props(
        svg = "eJxNy0sKgDAMBNCrDNkXm/gpQvUY7gUFBRUXIurpbdSCZDFMeOPXdhvQVTRzCWHjTGYcsiZvBQIbjiFGBrb/B2Tn4qLaJzqv/TQuPQ6piIVw8Jun9jRk6E6tqmjV5J+x9GzLl4Ya7Q2J0yYG",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkPlus,
    #[cfg(feature = "bookmark_x")]
    #[strum(props(
        svg = "eJxtzLEKgDAMBNBfObqnNqEhFGI/w8Gt4NBFcPD/UXGwg9x2jzs/2tmxzWHnAmEyymTIizaBIN1hCEnnNBaQNVSfnm317yFHhUUlhf5weRWjXoysHyA=",
        categories = "account",
        tags = "read,clip,marker,tag,cancel,close,delete,remove,clear",
        contributors = ""
    ))]
    BookmarkX,
    #[cfg(feature = "bookmark")]
    #[strum(props(
        svg = "eJxNy7EKgCAYReFXubj/5L1YEpiP0S40uAQN0dDTq5uc7cCXnvJWXIe7uUO0aMEiwrkWQfA9QqZKPw/o4/a7nJbBcwMwSBAG",
        categories = "account",
        tags = "read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis"
    ))]
    Bookmark,
    #[cfg(feature = "boom_box")]
    #[strum(props(
        svg = "eJx1i0EKwjAQRa/ymb3YGdqSQpIbuHVf0mACLiSEqrd3qiKKLdlk/nvPXsaaMDk6tBiO3SgQNPoYspPE8j1A5pa83S+Jt5/QwMy8srNsgf4XlBgqrnmqyZE0hLujgVBuehFSzKdUHbH+n4tmS+BtyCWcI4Lq3BGCUqPZ23nRVYv7P+0BRU9IOQ==",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = ""
    ))]
    BoomBox,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "eJx1jL0KgDAQg1/luBfQq1UU2s4uru6i4nWTUvx5e1sHi6BLEvKRqHXwDJPGjgTUvWxrNCqLpVFuHj3sdvKskSoEnu3CPmSB4A6NwYJKhFPjPYsDo55HASRZpL8E8j9CJVCxfZHmDS7bczQa",
        categories = "development,social",
        tags = "robot,ai,chat,assistant",
        contributors = "ericfennis"
    ))]
    Bot,
    #[cfg(feature = "box_select")]
    #[strum(props(
        svg = "eJx1jzEOgCAMRa/yw070FxhIkBt4CBIHRgfj+YVFMSnp0vS936bpLFfFsZk9wBWBYO1lW2dyWjrN6XUYB4nQJSEYP2uyKkA4SlaT2rlKdS5UAf0k0cAk4hBv6m/owIF+lviRB3Y+XYo=",
        categories = "text,design",
        tags = "selection,square,rectangular,marquee,tool,dashed",
        contributors = "llaenowyd,mishkaio,ericfennis,karsa-mistmere"
    ))]
    BoxSelect,
    #[cfg(feature = "box")]
    #[strum(props(
        svg = "eJxljrEKgDAMRH/lcE81qdoOVfADXB3cBAcHBQfx+00FS0ECCVzyLhfO5dqwdsUoDL8IBFUsYmLj7E6O6kzVqRLqIUmw8HcGgvGCepSJkVOnDNN33M5FH8qYoA9fjsMaCwdvHJrYqfnfjKw+MrGk1QOWvyyv",
        categories = "shapes,gaming,development",
        tags = "cube,package",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Box,
    #[cfg(feature = "boxes")]
    #[strum(props(
        svg = "eJx9kbtuwzAMRX+FyE5WJPWwATdA93jNkC1Ahw4q0KHQ95dS7TwVQ4AE6F6J95DTz/n3Cz7fd7PQmIBtlw8BAdeWAHuKWpTEn6/XzUqJs9ox3AhCLoI7sL0bCwYKGQMqenJqmpfTbj+91ZL7aS38bV9FCuZJHoWG8NIC9tWzOP+LJRCnjlqjKIUjj1lrbCF9DpwV7znMiRWw4D05rsKBraxrfKA9Kl6wuplXFTagZ94GS5XGW09vxpUgEmvxpClbrtqwY73pAGR8YMalFdhmeuq3coBEEsEi9cbUDJtIyzCGi/YHZ/qXkw==",
        categories = "shapes,gaming,development",
        tags = "cubes,packages,parts,group,units,collection,cluster",
        contributors = "karsa-mistmere"
    ))]
    Boxes,
    #[cfg(feature = "braces")]
    #[strum(props(
        svg = "eJxNjr0OgCAMhF+lcS/aGvxJkNnF1Z3o0NHB8PxSTIB0ucvlvp57witwb92xwLjPgYFh0MOkoi2e1ENxkNMrKUNm/ROhzrteed4VKk3AJFS5wMgRUxVzVbHI7Rvk07YzkAUr+QNcHSae",
        categories = "development,files",
        tags = "json,code,token,curly brackets,data,{,}",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Braces,
    #[cfg(feature = "brackets")]
    #[strum(props(
        svg = "eJw9yqEOABAQANBfuelm58aEIyuqbhMuCub7US6/x2tsgZlNwwgkdDCJJVPYfSisnMBjDf0VxQsGrRD3",
        categories = "development,files",
        tags = "code,token,array,list,square,[,]",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Brackets,
    #[cfg(feature = "brain_circuit")]
    #[strum(props(
        svg = "eJydj0EKAjEMRa8SZt/YpE3JwDgn0O0s3BVddOlCPL9pFWS0gkopLT8/+S/TOV8KnLbDnhgiSmYUqNfX4yKOyWFMsJYJR4XwJoY6gqNVwl0DDBEERddW+40JAvpX3eIIveyMhf1iOIdhnjYVcZ6eoAl0kWMNNA5nVsc9GwOFEj9UtKRsnS2V7OUr9Z1atFNgb8yaURo6ATmq28pjExPA99nNQeH3xhbI9EeimuPrwBsYdnYr",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not,ericfennis"
    ))]
    BrainCircuit,
    #[cfg(feature = "brain_cog")]
    #[strum(props(
        svg = "eJx1kFFqwzAQRK+y6H+nknblWmAHeoAeIriFFhIIIR/J7aOVP4JiC2MkdobZp5mW/+ty+qXlMbsQHS339bzOTtxh+ljlw3Q53v7oZ3bfIZIiHSMS2e/tY0UeGDpQOw7II8lmKBYRtSiyzgiilJDG1lpueSCBH7/aeWEI+R2CDAKqOxlsGe063q6rZFzJts9g2TKUHqwiq+ZV0DkkfFLwUEaG7hgyCr4g9vSiDWQpXDP2HB4jZUvoGSqDIBWZ+xDe1I6hiKlSdHcIgkG0+hOAroV2",
        categories = "science,development",
        tags = "mind,intellect,artificial intelligence,ai,deep learning,machine learning,computing",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    BrainCog,
    #[cfg(feature = "brain")]
    #[strum(props(
        svg = "eJx1zksKAjEMgOGrhNknpnlIC3XAA3gBdwMuXLoQz2+qIHRaCcniX4SvPrbnHW6n5VLIQc7SbizHJEgCRv5KvnUdjcqRzKCvEhWVOIPGfBupoZNn2D0mlfgiNvSSUXeKj+y6rPXQsGv9kZMNZv5jZpiZGUYzw9TMODe3PphDYT36DbcxRg8=",
        categories = "medical,science",
        tags = "medical,mind,intellect,cerebral,consciousness,genius,artificial intelligence,ai",
        contributors = "karsa-mistmere,jguddas,it-is-not"
    ))]
    Brain,
    #[cfg(feature = "briefcase")]
    #[strum(props(
        svg = "eJxNizsKgDAQRK8ybB80i58myQ1s7UWDm05CUHN7XWxkioHHey7HteD2xASJaZfiyXaEK21FXtoSqqeRkD8nV73gGu2CO5Yi2DxNdgDbuV8YjFZn2LCY7g/Apx201So85RMfPA==",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "eJxVjb0KgDAMhF/lyF5sQweH1jdwdRcV4yZS/Hl7jYOtBMJduPsStmlI2M5ITLgi1QSZllnSK893H8uYRFUTKo03Ye2TYIzUejjbMxj2GWfYcOezx+Pl58EK0XoBcR6cKVZTRcsqZTfFQd98mBt7+S4l",
        categories = "design,layout",
        tags = "bring,send,move,over,forward,front,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    BringToFront,
    #[cfg(feature = "brush")]
    #[strum(props(
        svg = "eJw9jkEOAjEIRa9CZg9CoZ02GecGHqLRhQtNXLjy9P52oiF5tLQ82F79fafbeXk20UJm0qiKrgyUnqRmmlAyRIj6xGO+j5912bfTkOzbT3VZISALaXFlk1JI2cnEM5KLpmETd06ScciJE4agbNABhlu0oxRHSjKaArYKYoPoEBlN6Ah2HurPf50v+pwtgA==",
        categories = "text,design,tools",
        tags = "draw,paint,color",
        contributors = "ericfennis"
    ))]
    Brush,
    #[cfg(feature = "bug_off")]
    #[strum(props(
        svg = "eJxtkL0OwjAMhF/lxB4Tx85PpVKJjQFW9qgMXZAYUJ8fBxAdiLxEyfm7u4yP+lxwO+wuHJGJ5ZqqQODbuEisLhCfC8JuGvdNPI3bihIHCJUCTl1FCGBZnK4uVIV+sep0cUzSW/A0ZMTZw2xNkxreCUXov/oeYGNS3yFlylYoH3+2SGBepSY7fC6YrbAZcC8JN/TqSufJQHLq1RUEnn37MTBliz2842/RX2yzWlg=",
        categories = "development,animals",
        tags = "debug,code,insect,kill,exterminate,pest control",
        contributors = "danielbayley"
    ))]
    BugOff,
    #[cfg(feature = "bug_play")]
    #[strum(props(
        svg = "eJxtkDEOwjAMRa/y1d0mjp00lUoXFhYuwFaVoUslBtTz4xSBGCLHHuyXn++Mz/m14nHutoII4VKO0k3jqU6m8Tu/ibFE6EFkxAYxoGfRnWRWDkHxqQHikRF2aam6mswGc86DjGwl+2/Adp0zu4Tn0XOjxqkhljkphotxRuECrX68NlGIXltLKKIsgaLfFO5Jeag7kzXQGHjokRY3xELir1ZSOaFBb/55fgoSed5/wBtv/12L",
        categories = "development,animals",
        tags = "debug,code,insect",
        contributors = "danielbayley"
    ))]
    BugPlay,
    #[cfg(feature = "bug")]
    #[strum(props(
        svg = "eJx1UbsKAjEQ/JXBftdsdi8POG1sbPyI4yyuESwk3+/mRLCIBAbCPDKbnZ/La8P9dHgURAiXssPhPB87c56//E2MJUJ3RUIcKCoyizaSRTkExQcDxE9CaDJKjYhhJWUXUqLI2TE10sVg7u1uI9vs9w5rugbP126AG5D+ZTeqAyrxpKgX44TCBdqbO04jKUSvo3EVUdbgDXw+r61c+++QDaQxcM2YvLOrSfzVrlSeMFT7InQbBklmJ/PqMb0w923tz/4EvQE+O3JX",
        categories = "development,animals",
        tags = "issue,report,debug,code,insect",
        contributors = "danielbayley"
    ))]
    Bug,
    #[cfg(feature = "building_2")]
    #[strum(props(
        svg = "eJx1jbEKgDAMBX8luBebRykt1M4urg5uBYeMDuL3a6YWacl0XC5JV7mFzmXaPAG7KyCQ/YYJBhJaJjwcjimnWaOcmpSx1tQaXfWVNRV0Qg4UBb8fsbIeEtMtLXlxfcF2aNzQhMa8YhVF7g==",
        categories = "account,buildings",
        tags = "business,company,enterprise,skyscraper,organisation,organization",
        contributors = "maxim-s-barabash,ericfennis,karsa-mistmere,jguddas"
    ))]
    Building2,
    #[cfg(feature = "building")]
    #[strum(props(
        svg = "eJx9zcsJgDAMgOFVQu5qW0qo0LqBQ4iK8Sal+NheixcR4ykkHz/xcewT8DhPnAIahbDNQ+KAmhCO64KwB7QI8V7inkfjq9w1fukSwxCwrcGYtbBMq82a7w91QFwq/SGaZDJ/pNWPWfmb3JHcOTlz7+oEkCZd8w==",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "eJx1zlEKgzAMBuCr/OS9m8m6zkHrDXaIMWXxYTCkuHl7WwQV1KeEfPlJ/PcZFXWgh4WD4EaVP+dZ5WfhAk7tFj6S9o0YXqhrXhH/QJbQpSKEX1tHDcSOoE371jj1Q6BLjuXA6pQFsybfPlGCr3oqeO8/d2yJ7r3shUoI92ahEa6OSl0=",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "eJx1js8KwjAMxl8l9J64ZFuXwbqLFy8+xKhChR1kytC3N2X+GagU0uRLfl/SnYdrgkNwewU/e9d3m6z03Vvn+k9DgCVxSz8hBdZUXqhGpoYUhTQSI1UkSGqBSaCwGrOquCijhQrrrRTE4MlM2pyYF/hdNQgYkx9aNnORys/qeJrieIQpOHEQ78Gx2n8LrskzS3d1YJvvq7/wF/i0MZz9in8AxldNzg==",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "eJx1jb0OgzAMhF/l5L00NoEKKeENunavCqoZKlUo6s/b4wwIhjDZuk/fXXjfk2KIdGWHWivH1IdzDvuwIQ8po5egg7hTU7IELLe22rF5fCR8pyFpJG4Jv0ieMNupCX/LhKDj9NRkv8teNnadnVV+imPNEfHgi9rYShbodEOX",
        categories = "transportation,travel",
        tags = "ski lift,winter holiday,alpine,resort,mountains",
        contributors = "danielbayley"
    ))]
    CableCar,
    #[cfg(feature = "cable")]
    #[strum(props(
        svg = "eJx1j7EOwyAQQ3/Fyk7KmR4IieYPujJ0Q+rA2KHi+wtLk0hEpxv8LFty+pRvxfuxPO+IhSBsPzE0zFp945GBr2VLtxHZ0j/ooNlNeLjgEiGa/arFrYrxo96aANtETlAGzHFWEkBphhOLcm0REqvx3d13oW+tJ33c+QNgjUiK",
        categories = "connectivity,devices,multimedia",
        tags = "cord,wire,connector,connection,link,signal,console,computer,equipment,electricity,electronics,recharging,charger,power,supply,disconnected,unplugged,plugs,interface,input,output,audio video,av,rca,scart,tv,television,optical",
        contributors = "danielbayley"
    ))]
    Cable,
    #[cfg(feature = "cake_slice")]
    #[strum(props(
        svg = "eJx1zEEKwkAMheGrPLJPbGYkcWCmazceQqJQoQspUvT2dnDTjWSX7/HXeCwx3xGfRk6Id6NCWBolGuvhh2N9Xl8Tbo0uLgkuBRmqa4kBYnKE9pvUQgwDlPuLdeVtwIkzGzufZs4bJ7Ee7sFdVg2az/mP+E6+jeMs7Q==",
        categories = "food-beverage,social",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,candles,wish,fondant,icing sugar,sweet,baking",
        contributors = ""
    ))]
    CakeSlice,
    #[cfg(feature = "cake")]
    #[strum(props(
        svg = "eJx1jMEKhDAMRH9l8J5uM1Ttoet5L/sRwh56XFD6/aYIItISBpKZvEn/dc/4vYcvPahF4koQvo5Q+JnuN1jisKRXhZZ0oQE6bW4UBavcaEQw2WZMEN6986vRYplm+kYyIxY2fGUv6BEzQnZe213d6Ekd9QpP0Q==",
        categories = "food-beverage,social,account",
        tags = "birthday,birthdate,celebration,party,surprise,gateaux,dessert,fondant,icing sugar,sweet,baking",
        contributors = "karsa-mistmere"
    ))]
    Cake,
    #[cfg(feature = "calculator")]
    #[strum(props(
        svg = "eJx9jEEKwzAMBL8i9IDWMib4YOUHfURpTGUopQRDnd/HIglJDs5pWe2MwhhfGf5pyMJIHUJhdAgS01syozUIUw2EsWj04a5CHz7pG6HYxZlqqkqMvjbSVkllNpLW75uh3S0m+R3+PbPAwPigDsjIzZBuej1utr35a821NXeh+bZ2nmYk+1qg",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[cfg(feature = "calendar_check_2")]
    #[strum(props(
        svg = "eJxdj7EKAjEQRH9l2D54O8ZwRXK1ja39gcIJKhYi3t+7e+GMSIrHJG+WTX6MzwmnIgcqNB7TSBCdn8DA/e43gy+N7cLIqZchb3zIkK+X+xkziyTBW4uocTbSIpdoqkur2t762jDU/p/o7a4Oc7q6rUVqc9ev3DSBy3aIIX73+wDE2DlC",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[cfg(feature = "calendar_check")]
    #[strum(props(
        svg = "eJxljFEKwjAQRK+y7L/opqFUSHIXscEEVKQETG/vbGNLoezHMDNv1k3xXijF/EjFswxM1XPHNHu2TN88ltTiCYmBVJXgzroL7pnfkWZZqmpA9phCIVUWC1ShP6ohnu0Wwzo4gl0D5dJIIw2F39jPrSQaPb+uJD0ZnD1ZrbUIP3+lOY0=",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,schedule,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck,
    #[cfg(feature = "calendar_clock")]
    #[strum(props(
        svg = "eJx1TjEOgzAM/MqJPSk+iNshZe7SlaFbpA4ZGRDvxwEJGIJs2bo7+3RxSnPG/918KXj6MGoiiLaUo+MnXDG4SH8Stpk7H5ohPorPEA83UbvtK8Lrhu8gba46WSrsQ609wyg1A9LUpFDLJRAnJeQON8Lg73hbAYTYPNw=",
        categories = "time",
        tags = "date,time,event,clock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarClock,
    #[cfg(feature = "calendar_days")]
    #[strum(props(
        svg = "eJx9jFEKwjAQRK+y7AG0m5ayH0lv4CHEBhMQkRIwvb27SRErNF/DzLwZu/hbgnecU3BIjLA6HBCywx5hETEia5Hg4z2kQk32rLvJPuLTQzYSjjKlwmXarMSjogptaM3KgncD/gM16ytBXR0Yqp/iv+zrmgLMDi8MNIRTR1pp+FORaXTjcSeX3LjkxuW++wAMMV9X",
        categories = "time",
        tags = "date,time,event",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarDays,
    #[cfg(feature = "calendar_heart")]
    #[strum(props(
        svg = "eJx1ULsOwjAM/BWL3Udtx0kjFWYGWBnYqjJ0YGBAfD8OQsDQKi/nzndxPNzHx0zX3eakQtKd86ik1LXBynrw/zvpU9LUkUBQqRE6l81+2DaT/fC1khyZaYHoV3CLp2fpFxgVaCVJKKMiGb23TznIznCdGAbRCAuM0cPtFlCKGYe3cE1rP21u2lIDY1RHTSwhcIOSoaSjFDip3gyZDe5TFMaxpHUjhWEPqdxSL99vvAC6R0/J",
        categories = "time",
        tags = "date,time,event,heart,favourite,subscribe",
        contributors = "karsa-mistmere"
    ))]
    CalendarHeart,
    #[cfg(feature = "calendar_minus")]
    #[strum(props(
        svg = "eJxtzjELgzAQBeC/ctwu9Z6tREicu3TtLrRgoYiDiP57L4lREcnweMeX42zfDC19HL8gJMW7bECg3L8MGZ6PYyeMct8HmmgN1/bml9T2/+u+NMNxyTSLYzBNGqJ1QkilHiW6G8Ppo5fmAkoeZRFlqJqQkw1DxAOkWnG1XZLwAr1UPE4=",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarMinus,
    #[cfg(feature = "calendar_off")]
    #[strum(props(
        svg = "eJx1jE8LwjAMxb/Ko/fWJmvrhK7gzYtX7wOFCiIeZGzf3tTB/sBGIMl7efnFT/vNuDfq6gzVKO3MYNh/VQgduXY2ZMsrg0zNmuRLpXgorBQnIhPIG38Lc16z5svJ+I00BXDnNg4VyObjDt9m7Ze41/P9QE+NYoWeZcgcRjmMUqIllH4Mwz3V",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "eJxlzsEKwjAMBuBXCbkPl39aKrQ7e/HqfaAwQcSDSPf2NktQ5+ihJPn6N+kxPEc6Zz5CSLpTGECgVk+DBofdb014yfbbqDfGyH3aaEifbtf7hQoyS2CaJDOYinhZ20GpIqdObDTLyPP7+A81s7UZxGhnX9T22gJmZe/I42u9xMXx5Piz7RK/AVvAR4w=",
        categories = "time",
        tags = "date,time,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "eJxtjc0KAjEMhF8l5K5uuovbQ7tnLz6EuMUURKQU7L69SYv4t6ckM99kXArnDGnxaBA4xAtnj2QRHnHO3NbisUcQZEBIRcnJ7TQ3uWu8BSgk3F4Iql8W41GuYqoqqEIvVET7R5KKPyC1Un3dNVKn5g292fspM8wejzQCDbypfSp+Wj2QPYwrTs1sO1oLiWe/vCdQdE+A",
        categories = "time",
        tags = "date,time,event,range,period",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarRange,
    #[cfg(feature = "calendar_search")]
    #[strum(props(
        svg = "eJx1Tr0KwjAYfJWje2Lu0sQKsbOLq4Nb0CGL4CB9fhMLRWjL8Q333Q+X3vlT8Dx3VxHULWZBcA1GRpfwz6GJ/cOBlvaEJqgcbejGdGg1Y1rKGKu33xCGnb8HXeGwVVUjzB6+buBvR8TMZsTJ3dexl+o4GdrQbtG/o5I7uQ==",
        categories = "time",
        tags = "date,time,search,events",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarSearch,
    #[cfg(feature = "calendar_x_2")]
    #[strum(props(
        svg = "eJxtjr0KwzAMhF9FaA+NLm3swc7cpWv3QAsplNKhlOTtI0fODyFoOHT3nVD4tr+OHpFvEJLqXrcgUJmmQIHrZbsT/nJeDVV0nptwSkea8H59njQgcs3US2RRHVShK6ZV0QTNqGVzQ8Vb0R+AUlpWGQmxotoHrLMQyI+4XMIOzqZ9soGny26BR1ogR34=",
        categories = "time",
        tags = "date,time,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[cfg(feature = "calendar_x")]
    #[strum(props(
        svg = "eJx9jkEKhDAMRa8ScoGZVBEX1cvMFFsQF6Vge3uTpoKIuHokef+3NrpfglgmNAgxV3gXFp8mpBGBNx3CHv7J64LNHmf7kdxs17A5KFRj2bAxMElZeB5EFampchtVlS4NPolyoq+WdarUkWno7pr2V2GvtcKsHS9ye+ASOuUDZChH6w==",
        categories = "time",
        tags = "date,time,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "eJxtzEEKgCAQBdCrDHOBGotwoV6mJIVoIUJ6+2YYadXqw/z3x5W4VyjNo0F48lGTR7II3eOKkGI+U9ULkwWhdJHBTbIL7sp3hEYsNhZGs3NK0KCCBh2VLKwO7D+UypD+olmNpCyXz778QzGF",
        categories = "time",
        tags = "date,birthdate,birthday,time,event",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "eJxtjEEKwjAQRa/yyd7YP0lMA2nBnQs9REEhgogLKe3tzVioLsrAvJnhv8mP+/OGSTojYjCz0mBaMC/XPu811OfX8C64duYSEU9+EAgarV2dxvTbK6XwoKIqf2KyAb6EM+uH4laDaozRhg2F3lIIBsXRwX3zRLJtCza1r9IH2sEwuw==",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "eJxNTTsKgDAMvUrIHrVpqwhtZwdd3aUKFRxERNTT2+KgZMj78sw67AFGi51QmQYVSLcVVI0aGBiKdBTRUX88fg6i/AvEff0vEAeSC3GmSd7oTJ5WnPHz5pcJ/GlRMMJmUSL4KzKZQq/tHqNbIw4=",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[cfg(feature = "candlestick_chart")]
    #[strum(props(
        svg = "eJxtzVsOgyAQheGtTM4GmilSMWHYQRfRVNPxrTGE1t0LxgsPvnKY//PfV1TqBc+ObGoQ/K28BD8N70jTX8Cg39hHFTQgHcaPRsEDlKcWNAu6clS+B1/F2Kb7WTsGbsnUw8rsVVdTm50Bu2JsL5ycY5PMBWSyw07ZHdsCSRFDTA==",
        categories = "charts,money",
        tags = "trading,trader,financial,markets,portfolio,assets,prices,value,valuation,commodities,currencies,currency,exchange,hedge fund,statistics,diagram,graph",
        contributors = ""
    ))]
    CandlestickChart,
    #[cfg(feature = "candy_cane")]
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/l1D0mdp04kUL/gB9gi8TAABID/y9cqrZDK0++e75z+/TvE4/rcEtkEO4CQfThMFIK8iqUA2vPyIsMjqS+LBz/Oc0aZFPiqtyHqV3m/KltLWxkCQZOEOIzIFKFUgGPqEf/bW7X+VWH9ORe3Wd1wDsK7Qk/1Os2rA==",
        categories = "food-beverage",
        tags = "sugar,food,sweet,christmas,xmas",
        contributors = ""
    ))]
    CandyCane,
    #[cfg(feature = "candy_off")]
    #[strum(props(
        svg = "eJxtkM1qwzAQhF9l0V2qd/WzMjiG3HJor70bWmjBKT0Uk759Zh2HhEQISaDZ+Ua7w+/090UfO3esIRO2Z+IphT7TenTrUtKZPbtxeLH6cbi63phDTZFK4Kp7OJLS5TQbUAVUDXlqSCFyxEvJ2uKm1fvOqS1CW9bolgx8tyD1WTtyQTPss8UXDaXGfaS4tcl90K5SPAgDLr1MNw2/VTMlfhXBOAzR4FdixQgxRuP7h4BkUBI+xMU/BHgL8FsAIHO+n/f8/fNJJ9k5EUcnxu3of7suryi1ovEMtS1sDQ==",
        categories = "food-beverage",
        tags = "sugar free,food,sweet,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CandyOff,
    #[cfg(feature = "candy")]
    #[strum(props(
        svg = "eJxtjrEKwkAMhl8ldE+8JHeXHtSCWwd9Abeig0MLDtLnN6dYkJaQDPnzf3+65/h6wP3YzIUSGCUUkDFSSfAZAdjbwCbBzR4N7dr03aFC+u6HunCETGnhsKeFmrKrzZw9iTEBkxrl9qSgnlSLCxnoILy4dgtAMRNTEVKp13oW8R+rdYfbAhsyJKxc/AdHUhAedKniuO7RwfgFu3ly68p9A63CRy0=",
        categories = "food-beverage",
        tags = "sugar,food,sweet",
        contributors = "karsa-mistmere"
    ))]
    Candy,
    #[cfg(feature = "car_front")]
    #[strum(props(
        svg = "eJxtjs0KwjAQhF9l2HtjNj9NCknBmwd9iGKL6UGQEvx5e9OCorQsLMvO7Dcbbl1O6CNdFcNXCqpiYSst3L7MkEuxFbWpYQ9emO67LsZGarBQ1h0tWELDUxt2M7INH/DJgU0SkjckXmvTcM54RWJJSMN4STmSJ0zPSIpQuiY8xj6nYlnS5oMfZHnE39VWVvOvvAGD+UA8",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "eJxtjs0KwkAMhF9lyL3rZn/aLXQL3jzYhyi2uD0IUhZ/3t5UUJSWXJL5MpM01z4nDJE61jDJUdvsFqltPuBiGKEwMAUrX1hV7aWHfhd7VboS/hCU67+yLNbagpXx1dFDgi3COrirwC4pzRuI12weTxlpnM4pRwqE+RHJEO7TkFMkFkUES3jKoBfbYviJlEfCzWzdqv/JC+ghR1o=",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "eJxlT8EKwjAM/ZXQe2PTdesK7UC8ePEjJAoTdpApQ//epBN3kPQlNMl7SfL9/BzhUsyJElAcPWMHDshiEEeLbdhZTBajpYpWkA7UYwRy0kvy3OofUmoEwXr0goattMuXkFaBXsKxlbTM0GytBUyTBvCY9o3oKlw1D+SXwA6w033URm+GvNOth8y3macr8LsYigbmYrwBfhUTtWetDvl3YT2w+6NvNBX5im38D0tWRJE=",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "ahtohbi4,ericfennis,Andreto"
    ))]
    Car,
    #[cfg(feature = "carrot")]
    #[strum(props(
        svg = "eJxtjz0OwyAMha9iZbcbmx8TKc3CzJQTROrQoUMlOuX0xUQqS4V4Apv3+bG+j88THvepCImCMGldKCk6CsBC6jCSi4dvV9uzrV4y0RxIFZgpefgBxuksiaIH9i8UmoOJLxzItVqwmo9dpm29WZBtHXEElopMzqFYGJTMkVKENi7A0qVaGwQsq+zmAJPzD649l4oCF9Ac+8AIXh+wOZm1dRvWHF0G7guQjUIT",
        categories = "food-beverage",
        tags = "vegetable,food,eat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Carrot,
    #[cfg(feature = "case_lower")]
    #[strum(props(
        svg = "eJxtzMsJgDAQhOFWlmlAo2IUdrcDi5BVUPAgQYJ2L8EHHrzOzzdsc7BlJNsFHmSHwBWgICihnF1Vee23iQZB53JqY51SmpQ/3PnbPS+/viIfm9efHiwkYA==",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "eJxNy1EKgCAQhOGrDPsesWphoN6gQ8QWGBSERNTtSyLscfjmd9uwR4yeVg1uYCoLA0vB1RmC+7g3YB3bAjInWSbI6YktIXnSBLmepfLp5V+vGN1R+hvJTSF8",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseSensitive,
    #[cfg(feature = "case_upper")]
    #[strum(props(
        svg = "eJxti7sKgDAQBH9lSe/jkjtNcaa2sbUPWFwjWPj/eBYKgizbzDB61NOwTWFPIAE3GYwcina3KProhUHJhh/hFZFxKzUioveRn2eSdTT+wje/AEDkHbQ=",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    CaseUpper,
    #[cfg(feature = "cassette_tape")]
    #[strum(props(
        svg = "eJxtjWEKwjAMha/yyH/rUsdWoR14AA8xumILE6QU1NubdiCII4SQ996X2Bx8wTMtJTrSHSGGdIvFEQ+El0iEvI23o54me6zAZH3Kfg3worJQXjJGojUpmc2d7GMuEYujqwHraKpVpT+8ce2IvN3h7wN0BzUetDpfWPWo3UkxjGLwEEdl5l+jbasacfq+/QCKZT38",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = ""
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l2Hs0O7QhhTR/4NV7QCGCiAeR+vduSqEtlD3szPBm0rt8Km6jXIh4DYUgvJ2CjlV3AfhVroEzVV2QnM5tJKfNlHEDhqUYEY+hUHr0C9ShW6Hn43XHxFF48ir4NeUFk9o3q7M1unH5D3E8Lls=",
        categories = "devices,connectivity",
        tags = "chromecast,airplay,screen",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cast,
    #[cfg(feature = "castle")]
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8luAfbJFYL1dnF1cGt4ODoIPl+q4IotGS53OOOC3s8Nlj7aiICMop+JPUxaTD3JbVZ9zWQlmoI9RUcwhu3HVg7y+i0zdEGiBQlMvDTg4ys5vsDq2SiaVcq9jlSAg5kpvzGAjAlID9wApOkT7o=",
        categories = "buildings,gaming,maps",
        tags = "fortress,stronghold,palace,chateau,building",
        contributors = "karsa-mistmere"
    ))]
    Castle,
    #[cfg(feature = "cat")]
    #[strum(props(
        svg = "eJx1T7FOREEI/BVyPePCwrKbPC8xtvoDduYsrrAw0fj9wjO5a+4KZoFZmGH7ev8508fj4VWU/IQR1EjQHW2REnRkFZOTROusmEYDppntlMEnZ03xjwSPbLcEyi+WT4fZswpJYJEMrKCqNPGbF/eMgI9TY4E6PFfnGNdYeqnuXLxrJlfL+8pskiFSoacdL6n+tNCMdmh1REn42+G4PdSNx+1y6SSxX/gNRsZ9qsyVffWzwF9yucQnI7ziKvMHnzZJmQ==",
        categories = "animals",
        tags = "animal,pet,kitten,feline",
        contributors = "kemie,karsa-mistmere"
    ))]
    Cat,
    #[cfg(feature = "check_check")]
    #[strum(props(
        svg = "eJw9yrENgDAMRNFVTukNXJBjCpMJYAgkCgqQKNhfJEVS/Oo/f4/vwrmGnQsSDLRbVDRkH+vK3sATIziJDYrSxhlMXf2XeBIq",
        categories = "notifications",
        tags = "done,received,double,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    CheckCheck,
    #[cfg(feature = "check_circle_2")]
    #[strum(props(
        svg = "eJw9TDsKgDAMvUro3trE1CLE3sDJE4gOLoKgk6fXVCLJ8P5yzNcG6+BGJCBaUkjUQgSMngPnrADjhLnqBJp6r6ue0tNi39+uSKObRWx5763Fnn/7AV3LG5c=",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis"
    ))]
    CheckCircle2,
    #[cfg(feature = "check_circle")]
    #[strum(props(
        svg = "eJwtjEEKgCAURK/yca85ZpCg3qBte6EgQVTITbfPJBhm9d6zNbSLDsc2pQgQct2hAiT19SPwRZiZGwHNvJ0+2tta0pNiPqmWmNvtWHc1oQe0kCAzQhj8T/oXX2kbIQ==",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckCircle,
    #[cfg(feature = "check_square")]
    #[strum(props(
        svg = "eJxNjMEKgCAUBH9l8R71HkYE6rlL1+5CgYKokAT9fRYEspcdGEblFO7g44GcfCynFjOIQAySYIYURvW/Y1S2xWHXYuXXuSbLYAx11NW3jC13vDWMyo7oy9WIeQDpTB6r",
        categories = "notifications,shapes",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CheckSquare,
    #[cfg(feature = "check")]
    #[strum(props(
        svg = "eJyzKcjPqczJzEtVKMjPzCsptlUyMlAwU7BUMDRXMFEwNFKys9GHKbEDAEfUDao=",
        categories = "notifications",
        tags = "done,todo,tick,complete,task",
        contributors = "colebemis"
    ))]
    Check,
    #[cfg(feature = "chef_hat")]
    #[strum(props(
        svg = "eJxljrEKAjEQRH9l2N41cyZ7EZLAdTa2FnYHCgoiFhbn35vcIRbHwA67PHYmvcb3DZcsRwN3GvvBw8NVEb16wsagJOaxnKkubKjBI1T9UBfhhhVqGvYw/H8yLjGnjgc7S0nbVqCkx/15xdRlYRRMzGKCT1v76py9so0qX8QqJgU=",
        categories = "food-beverage",
        tags = "cooking,food,kitchen,restaurant",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ChefHat,
    #[cfg(feature = "cherry")]
    #[strum(props(
        svg = "eJytjkEOAjEIRa/yM3uwUFqmSZ0beAF3k7pw6cL7RzomunFpSD4Q4PH7Y3/ecTsvF4X4XlCQjpCQkUjZa0ihGfmolApYLTRfl62fJmHrH478C+QQG5lVY7kJjLXRyh5kmj+4VijnNQbN0OKf6A+KKtqgeYxEzmJBy5kkGhR2eTs0rj6zf428AJKEPnE=",
        categories = "food-beverage",
        tags = "fruit,food",
        contributors = "karsa-mistmere"
    ))]
    Cherry,
    #[cfg(feature = "chevron_down_circle")]
    #[strum(props(
        svg = "eJwlykEKgDAMRNGrDNkXGymu0txFoqCgIMWF3r5pyyze4o/YWezaUTJxJNjnzu7fVZlGV3nW98CW6eYFHENCCr72aEUr2foTiQ==",
        categories = "arrows,navigation,shapes",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownCircle,
    #[cfg(feature = "chevron_down_square")]
    #[strum(props(
        svg = "eJwdi0EKgCAQRa8y/L2UJdHC8S6R0rgIQgay26fy4S/e4/mSTqVSGQvozVGFYXfQx1hBdbykfIkOHvzUg+CfQ4Ui47Yb2dk4cqat+27CDzQLF9E=",
        categories = "arrows,navigation,shapes",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownSquare,
    #[cfg(feature = "chevron_down")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNVOwVDADQV0zJTsbfZC4HQB8NQfk",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slow,dropdown",
        contributors = "colebemis"
    ))]
    ChevronDown,
    #[cfg(feature = "chevron_first")]
    #[strum(props(
        svg = "eJw9yTEOABAMAMCvNPZGamgN1R94hMRgkRjE+zEw3HQ6ymxQk+skQBEZGQ5n6u+Yvs8CvCj82LwRD5U=",
        categories = "arrows,multimedia",
        tags = "previous,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronFirst,
    #[cfg(feature = "chevron_last")]
    #[strum(props(
        svg = "eJw9ySEKACAMAMCvjHWRGTbD3A98hGCwCAbx/aKgXDwdZTaoCbsARWB3oak/Yfo6kwAvCn82vfwPlQ==",
        categories = "arrows,multimedia",
        tags = "skip,next,music",
        contributors = "dperezcabrera,ericfennis"
    ))]
    ChevronLast,
    #[cfg(feature = "chevron_left_circle")]
    #[strum(props(
        svg = "eJwlykEKgDAMRNGrDNkXjQRXae4iUVBQkOJCb2/aLoa3+KN+FD83lEw8EvwNp/Brmg69m97Ls2PNdLGA5yRJEKuPWuwH2lYTjQ==",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[cfg(feature = "chevron_left_square")]
    #[strum(props(
        svg = "eJwly0EKgDAMBMCvhNyLVIt4SPIXscX0IEgJqL+30cPCwuxSK5vBzTghXDWbMsYFQUvd1f7+fNj6ZkShwQ9C52oKmfGICeIcUkjQ4+4iLzkyF9U=",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,panel,button,keyboard,<",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronLeftSquare,
    #[cfg(feature = "chevron_left")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNTRVMLTQNdM1UwBiJTsbfZCMHQCMUQhe",
        categories = "arrows,navigation",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "colebemis"
    ))]
    ChevronLeft,
    #[cfg(feature = "chevron_right_circle")]
    #[strum(props(
        svg = "eJwlijsKwCAQRK8ybB+iwSLFuncJm0ACEUQs9Pb+imF4vMf6Jf0faPVkD0LqZwhaJgrvywvHK7+4PQVrcMLBbX0jGEIax5ATQA==",
        categories = "arrows,navigation,shapes",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    ChevronRightCircle,
    #[cfg(feature = "chevron_right_square")]
    #[strum(props(
        svg = "eJwli7EKgDAMBX/lkV202qFD038RW0wHQUrA+vcaOtxyx8VWDoWUeooyuUB4mTbCU7PKEK0zrYRuPsXZhhTvXQWZ6XILAjz89GPZQvoAKDIXiA==",
        categories = "arrows,navigation,shapes,development",
        tags = "forward,next,more than,greater,menu,panel,code,coding,command line,terminal,prompt,shell,console,>",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronRightSquare,
    #[cfg(feature = "chevron_right")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKtVQwtFAw0wVDJTsbfZCEHQCFAQgx",
        categories = "arrows,navigation,maths,development",
        tags = "forward,next,more than,greater,menu,code,coding,command line,terminal,prompt,shell,>",
        contributors = "colebemis"
    ))]
    ChevronRight,
    #[cfg(feature = "chevron_up_circle")]
    #[strum(props(
        svg = "eJwlijsKgDAQRK8ybC8a2cJis3eRVVBQCCFFcvv8imF4vCf2RvtuWPHkdkJstxEsD1RZp1cJZ3pwefoPOAYvbeAedKEVx/gTRA==",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[cfg(feature = "chevron_up_square")]
    #[strum(props(
        svg = "eJwdyzEKgDAQRNGrDNOLqClSZHMXMcFNIUhYMN5ek2KaefxQ82F4hRtRm3AlNJdTTbh4og14SjIdRwxzD2K4d1Mk4eWxOLjpH1znDvEDF6wXjA==",
        categories = "arrows,navigation,maths,shapes",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronUpSquare,
    #[cfg(feature = "chevron_up")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNbRQMDTVNQNBBTMlOxt9kIwdAIx0CF4=",
        categories = "arrows,navigation,maths,gaming",
        tags = "caret,keyboard,mac,control,ctrl,superscript,exponential,power,ahead,fast,^,dropdown",
        contributors = "colebemis"
    ))]
    ChevronUp,
    #[cfg(feature = "chevrons_down_up")]
    #[strum(props(
        svg = "eJxNybsJACAMBcBVHumDIgabmF0ECxvBwv3x0yhXno4yG2qmnhA8hAUbmboTpl9H3OO3C+/GD+U=",
        categories = "arrows",
        tags = "collapse,fold,vertical",
        contributors = "PeterlitsZo,mittalyashu,ericfennis"
    ))]
    ChevronsDownUp,
    #[cfg(feature = "chevrons_down")]
    #[strum(props(
        svg = "eJxlyTEKACAMBMGvHOlFRKLNmb8IFjaChf9H0gmy3Q53PxOjyaooUC+oGKN/46Mpf3wB8o4P6Q==",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slower",
        contributors = "colebemis"
    ))]
    ChevronsDown,
    #[cfg(feature = "chevrons_left_right")]
    #[strum(props(
        svg = "eJxNybEJACAMBdFVPumDWIQgxOwiWNgIFu6PxELkunu22h7olWaBsuBGbim+29Ms0CD++QDzuA/u",
        categories = "arrows",
        tags = "expand,horizontal,unfold",
        contributors = "karsa-mistmere"
    ))]
    ChevronsLeftRight,
    #[cfg(feature = "chevrons_left")]
    #[strum(props(
        svg = "eJxtySEKACAMBdCrjPUhC0PDd3cRDBbB4P1Rk2XhpYfV9qBeeaqSZjExutiR3jj+l+gPIp8QsA==",
        categories = "arrows,navigation,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsLeft,
    #[cfg(feature = "chevrons_right_left")]
    #[strum(props(
        svg = "eJxNySsKACAMANCrjPXhB4dl7i6CwSIYvD9uRQwvPdn9TBgNV46QKjExGFQJPirvi7UX/X0BElQQew==",
        categories = "arrows",
        tags = "collapse,fold,horizontal",
        contributors = "karsa-mistmere"
    ))]
    ChevronsRightLeft,
    #[cfg(feature = "chevrons_right")]
    #[strum(props(
        svg = "eJxtySEKACAQBMCvLNdFDlHLen8RDBbB4P8RDSaZOJx1dbQiI0EzorvE6E8YX2v4/QYVRxB/",
        categories = "arrows,navigation,gaming",
        tags = "turn,corner",
        contributors = "colebemis"
    ))]
    ChevronsRight,
    #[cfg(feature = "chevrons_up_down")]
    #[strum(props(
        svg = "eJxNybEJACAMBMBVQnoRiyDCm10ECxvBwv2RtxC58rDaHtKrzizJ5AqmjshwfF04/LcH8N8P7g==",
        categories = "arrows",
        tags = "expand,unfold,vertical",
        contributors = "mittalyashu,ericfennis"
    ))]
    ChevronsUpDown,
    #[cfg(feature = "chevrons_up")]
    #[strum(props(
        svg = "eJxlySEKACAMBdCrjHWRH4aG7+4iGCyCwfsjiyIvPu5+poymC0WAZEFMnTnG+Xz9/gIi4hCw",
        categories = "arrows,navigation,gaming",
        tags = "forward,ahead,faster,speed,boost",
        contributors = "colebemis"
    ))]
    ChevronsUp,
    #[cfg(feature = "chrome")]
    #[strum(props(
        svg = "eJxljUEOgCAMBL9C+oBKERQT9DPEg4nxwAl+b0klEj1tk53phnikeO4q5hXIgIpFMnFo2MIg/RYa9/YWmtVh53HtqtAKHlTmMIQ082lEK6Y2zFeuoyfUkwgjLk54j86KQfajVJA0ev98dmhZL7K3/K0bruQ7Hg==",
        categories = "brands",
        tags = "browser,logo",
        contributors = "colebemis,ericfennis"
    ))]
    Chrome,
    #[cfg(feature = "church")]
    #[strum(props(
        svg = "eJxtjLEKgDAMRH/lcA82sVqF6uzi6i44OCg4SL7fOEg7lCw57t6L9/Yc2Mfq4h4BHqLMm0Dg7Jjsm32eSdbh9CTVFOuPneJvWNhoUUpz983V5dn8voT2hq7tSR011KFRDqWVIGhbKhyGI3lfcQ01rg==",
        categories = "buildings,maps",
        tags = "temple,building",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Church,
    #[cfg(feature = "cigarette_off")]
    #[strum(props(
        svg = "eJx9zbEKgCAQBuBXEXerO5Qc1Lmlh4gKDCIaQvTt87ChwVru+Pn+48y+HSuLYDlyFjGvvFOJqURnWio5c06XZ4vlIyADHDBID5KY4MVIHGoCOosXjapY/3Ok505gowQ+U9W/ftduA349QQ==",
        categories = "travel,transportation,medical",
        tags = "smoking,no-smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CigaretteOff,
    #[cfg(feature = "cigarette")]
    #[strum(props(
        svg = "eJx1zCEOwCAQRNGrEDxtdwItYouu6SGaIpAIwvmBhKDAjHmTz/FLQfhbvmQF4UHWgU7peG/geDBQOeuJXCuoRfsfCptR6Gvm5fWtAOAqKss=",
        categories = "travel,transportation,medical",
        tags = "smoking",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Cigarette,
    #[cfg(feature = "circle_dashed")]
    #[strum(props(
        svg = "eJx1kEsKwzAMRK8yZG/VslN/wM0NeohAF1120ftTSVmUJAqG2ViPp9H4rN83Xo/pyZEYibitnXqGRZTHyNQQp2XcdHYZf6JSkc/KStxhsRGJSpeoDpSYWoLKTp6oJg+JlDrUdhQFcZjM2y6T7KA2pWZYbNRVoUIzTOZ4Sg8XheRkUNm5T/D76MkgLudsNeza/AAXpmBD",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDashed,
    #[cfg(feature = "circle_dollar_sign")]
    #[strum(props(
        svg = "eJxNizELgCAUhP/K4R75HiIO5tzS2i6vwKAhpKH+fYogDcdx9/F5ObKcO+SdFLGCPK1zKa2CHxsP/op3wjaphSxcGmxkMDSoRMMk03d9zOyqW52/ySC32k4+7SIf7A==",
        categories = "shapes,money,currency",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[cfg(feature = "circle_dot_dashed")]
    #[strum(props(
        svg = "eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgI3D8nl5GZ8LM8rzpM5sSOGJx6WRKlDESfF6GiAM/N40N15/DoiBRlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrXdSQ3KE1dPYpU116gQD0KrMEJye4EkpdBYds8tp1HXwZhNd4W7W+afFvz/YJ1MmyQX9K89HfpslTH8wegzGtN",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "eJxFi7kNACAMA1eJsgBPHVjGokCiSgXb80NlWXcnyIqSCC2w80w6xjKhrhvFbB7leJM/fKuvdX0KFkU=",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "eJx1yz0KgDAMhuGrhBxAmywuaW/gISQKFRykONjb928plE4vfA+f6B30uSBYJIOgfy7nxlona3Mn7/F5OC3utAGxXwwVLWtvPLfhlgBL8iIY",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "eJxVyksKwCAMRdGthGygphQ6idlBF1HSgoUORBzo7v1NdHThncf+jg4ei9cJZBwZFN7aJjzLsYh+Qf8XNFukHUHTaKjpr+FSALkjGd8=",
        categories = "maths,shapes",
        tags = "calculate,shape,=",
        contributors = "danielbayley"
    ))]
    CircleEqual,
    #[cfg(feature = "circle_off")]
    #[strum(props(
        svg = "eJxti8EJgDAQBFtZUsCZu5iQwBmwAIsQfPgRfNg/XgLmJbPsY5fRe39OHIu7BIa3uKpTW6t+35YpRAilsrKHpcEQpgCOlOKPwoV8Ru8hsTFTkV7DeQE/Qx75",
        categories = "shapes",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure",
        contributors = "danielbayley"
    ))]
    CircleOff,
    #[cfg(feature = "circle_slash_2")]
    #[strum(props(
        svg = "eJwlibsJQCEMAFcJWeA9U8ds4BASBQULEQvd3h9XHNyx5qYlgg6LhhB0PretH4W/94Wr7wmCRUcEFzr3VFmQnxKW",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "eJwlyjsKACAIBuCriBeohIbAvIw0BNHQlLfPavr4Hzz6bGBUsSDs9DAnZY/0FA73JKx96Wig5jUh6P4uJ97X3+UAjt4WGA==",
        categories = "shapes,development,maths",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[cfg(feature = "circle")]
    #[strum(props(
        svg = "eJyzSc4sSs5JVSiyVTI0UFJIrgDSRkC6Ekzb2ehD5O0A50gLOw==",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[cfg(feature = "circuit_board")]
    #[strum(props(
        svg = "eJxlzMEKwjAMBuBXCbkPTTbRQds38Op9dMUUPEgpOt/eJjtsMAr9CfnzuZJiBUn5KdUj3RAWjz3Cz/7SBkb45rmKbYM76UFw76kKzB7vRDDKMDEwnO1xx49ei1oJLuYSXwmKQbGxY4tFo1XW5U67AtOn2zhSToYDpxBdVknT+A38A31fOgM=",
        categories = "science,development",
        tags = "computing,electricity,electronics",
        contributors = "danielbayley,jguddas"
    ))]
    CircuitBoard,
    #[cfg(feature = "citrus")]
    #[strum(props(
        svg = "eJxtTrsOwjAM/JVT95pcEztBCpW6sbCyV2JgQWJg4utxAoKByg/Jp3u43tfHFZfDcJooZmAWyyslFPQVvDhKSH7awgneHUOSHDFJLM4m2rxx53XBc5jrrrnP9ZvBvZiC6klLwccdRaK6XUz/ihs9OIwqCp8tx9ReLnpmOJrwR3kBSy0yAQ==",
        categories = "food-beverage",
        tags = "lemon,orange,grapefruit,fruit",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Citrus,
    #[cfg(feature = "clapperboard")]
    #[strum(props(
        svg = "eJxtjssKwkAMRX8ldJ/Ym5mUFsau3fQH3JW66KKCC/H7vVMRBSXkxbl5lNt8X+VybCZvzaWTJMCmNqhbXtSSwsDohK8cG5KF5oWAXEj2GJWfm7Ec6sqxvBdfO44GJclAH/4owFtEeZfkX8FUn1rRP/rZxaWlQVmd4rtX/1x/Ag40MrQ=",
        categories = "multimedia",
        tags = "movie,film,video,camera,cinema,cut,action,television,tv,show,entertainment",
        contributors = "it-is-not,ericfennis,danielbayley"
    ))]
    Clapperboard,
    #[cfg(feature = "clipboard_check")]
    #[strum(props(
        svg = "eJxNjLEKwzAMRH/l0G5aCWFasDN36do9NKHKUCjGNMnfx/aQBMFJuuNeSOM7w8bpYzmSEtISiQnzNGSLdCMsTdPa7KJCXbjUVhd+fTYMkZ7soSa9QHAtw2XLn/UwXLke/vw7eflzwYk1cmUe5O8drJUGdbrHG4g4LKI=",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "eJxtTMsKg0AM/JUh96UmpOLB9eylHyFVGg+Fsixb/fsaD1VoGUiGebVpumekJRITbJofliMp4T2P2SI1hGW/ayQhpNVzXXvxVte+hmwYI90aaF8PAkHlCBsrrIewfTGWsxCkBPEpHzlNcQ21I8leLfonKAzWnvnXevIVXAXFjq//AdAZO5I=",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_edit")]
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/llD0Bu05LpKZfACsDW0UrUokBVRHQvyfuAB0YbJ/ufK+dx2vGEg0bvKM5GMzlkMFrGnJajTROt5SjkZItmnXtTltd++hzwhDNifZOGMSupp4dQWcPgqrQrOvYuODBBAHz3YVgKzW8k8qWJRfFKnCLrSGJewYXmvKKepL8DFtUKnX/pyygyvlzva1bTvx9/QCf1Tyw",
        categories = "text",
        tags = "edit,paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardEdit,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "eJx1jL0OwjAMhF/l5D1QW1HUIenMwspe0Qp3Q1EE5e2JWUilVpb8991dzPO9IH8SMeG9TEUT9YT113VeHloSeUIVCCGvphvi2VxDfI5FMSW6coBXGQWCrhbXKS/2/4er2yW0t5NbaA1OVCzZMttkAbP6AxJ2SW+WU8f7KGzQF+62QkY=",
        categories = "text",
        tags = "copy,paste,tasks",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardList,
    #[cfg(feature = "clipboard_paste")]
    #[strum(props(
        svg = "eJxljjELwkAMhf9K6J7zXoxRoXbu0tXB7TiHWwQHud9vAtIKJYQk3wsvGd/l0+h5GxacSOZrAYFyBHvXpWZKlpQCo1lN5ho4CON+rJmT+cSIeAzTeAjDaVxtL6SzFSH5uXrXoRvwKg3yD1gWGGnbIGKrO/YntCHv77xwJmRSUvZc9S/UAjKN",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "xnousnow,ericfennis"
    ))]
    ClipboardPaste,
    #[cfg(feature = "clipboard_signature")]
    #[strum(props(
        svg = "eJxtjcEOgjAMhl+l2X3Vlgpbwjh70BfwRoQ4Eg+GLIhvb7cLHMiyrvn672s7j88E32lIMRhnYP4FQwbiOL1iCkaUrIWsZaxTNl17yr+69tOnCEMwdwdyrXsGhnM+VruFZAP6ciTeA8uLxUt2ZcvORTVI3KIEhE0FdJR0KAwea+oZCfItee18U8qNavQXoAZIFe6N3tsqE0GprBZ5HHidRuO28A84lUaZ",
        categories = "text,account",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardSignature,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/l5D2Cs6oAUtKZhZW9ohXuhqoolL8nYaAZKks++073wjI9Et7zmCzKWbD+9ieKCpYiFNg0Py1F6YqzVqcPh9rqw2tIhjHKjR6d6aBQHMuwqGZ2m+HKdfXt7/Tu24JT00quzIZ8ATU7ms/cSUnwtNujgsz+n3wBDa878w==",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "eJxtjMEKwkAMRH9lyH3RhDUo7Pbsxav3YovpQZCyaP17s710DyWQZOYxk+bxUWDj9LSSKRLmXyYmfKehWKYzYVm3u+JwqbBLh5rq0rsvhiHTjRXRpBcIjj7sVz4cNyP4d9VWB7lrGwhiUptr59b84hOYg0J32MURWvQHFyYzgg==",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "eJxNTDsKhTAQvMqwfXgvSwgWSWobW3tRce0kBD+3d2OjTDE/ZkKex4J8RrKEfD2kpiGoZoLM6yIlkiMc61SkVin86iqFbSiCKVJnPZzwwGD8FVaZd+vewKhq/dcb7v13YFi4PtfPdAN3xSSc",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+UHFRc5nQRSC0pbqwt7dRCsMMw3uepbAGcD3dvDiUNpMDP98lP/6cfE5aVWJAThLvyzB2tLKsw4bD7G7RC2+oGXM=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "eJw1i10KgDAMg68SegF/HsSH2suUPRTKNqYP7vZWZRASQr6wWlNP0PugZSW0iJmg/avC078L1+LdLSfUYvk63xkbwkI74hPsYOQBQHsZDA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "eJw1iksKgDAMBa8ScgB/oCjEXCZ0EQhtqS7s7U0phcebxQyJFrEA8t24bghSO4tjQaa5e6acrJrGADlpfJ9WwQF+vmva4WzxiPgHWfIZRw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "eJw1irsNwCAMBVd58gL5FKkcL2NRWLIAkRRh+xghmrvijtWaeoJ+Nx0nQft0C+0kvM0uXIt3t5xQi+X3GRcuBOKObWX5AdG/GDM=",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJWF65iLhO6CIS2VBf29iaKMMyHeSQ6xArkPtK6JQyPnCDznUzL9zP1ZtO0FvSm9Trjxg63kJcc8A/xA1epGTs=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "eJw1iksKwCAMRK8yeID+oK5sLhNcBIKK7aLevrEiDPMW7wWWyhpRL7dvDvwaD2P7SWEdnkLJ2lRSRMmSnrtreNj1+eXEyGdGH4WXGaA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock3,
    #[cfg(feature = "clock_4")]
    #[strum(props(
        svg = "eJw1iksKwCAMRK8ScoF+KF2luUxwEQgqtot6exNEGOYNzCPRJpZA+oPHiSD/ZHPsyLTNn6kW66Y5QS2avzcsuMEr4uMKeUk8AFlpGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[cfg(feature = "clock_5")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL8S8gC1oj3FfCb0EAhtqR7s720owrJzmCHRJpZA+oVhR5B3sg1syLROz1SLddOcoBbNz+0VRBjnO5YTQvT8z/gDiKoZog==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "eJw1iksKwCAMBa8SPEB/UFdpLhNcBIKK7aLevrEqPN4sZpClsAYol9s3B1yNh/H9Sbh2T5iTVpUYICeJz900eLAb88vZ8pnRB4VwGaA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9AFqBQtCzWdCD4HQlurB/t5UEZadw0xkaawJ7XB+ceDbuBr7S4rz5ynWol0lJ9Qi+TqHRoCdbZ82+DDqv6IHblYZdg==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzdVhGWXZYZEm1iC9CPEJaD5zAFyv5dp+jhTLdZNc0Itmq9zYGzw8uyI63B/hx9AwhkQ",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[cfg(feature = "clock_9")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+QN3EXCZ0EQhtqS7s7Y0WYZhheI9Eq1iE3EeYl4DqMwVI+y7T2DlTydZMU0TJmq7zxdjg5dmHFd3+LX4Ab3EZcA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock9,
    #[cfg(feature = "clock")]
    #[strum(props(
        svg = "eJw1iksKwCAMRK8ScoF+KF2luUxwEQgqtot6exNEGOYNzCPRJpZA+oPHiSD/ZHPsyLTNn6kW66Y5QS2avzcsuMEr4uMKeUk8AFlpGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "eJx9j00KwjAQha8yZN9nJj8mgbbgATxEiUIFBSku9PbO2IUuUgkhgfnmmzd9vSz1eqb6HAw7Q/UlbzK0DMabsd+t5bG/T4+ZToM5BjjiCD4kSmSJ9UQkpjwzUpkCIum1n6KTX4ZTkxq+nps2EWeEDqWDbxBlnVQ2AbbYk7NI8MK0AA9JJ4n/AKuhkxzcHpHV4LeADK9b7ImFaAmCrhnV8gu8AW08Xzs=",
        categories = "development",
        tags = "computing,ai,cluster,network",
        contributors = "karsa-mistmere"
    ))]
    CloudCog,
    #[cfg(feature = "cloud_drizzle")]
    #[strum(props(
        svg = "eJx1zMEJgDAQBMBWFgs43eM0CUTBAiwi4CNPH2L9Gh/iI+E4WBh245HOjH3uNgNNfAirg8MAlhvFET5TXEgmI8oPL+qTvKhpt8S+bCzxW/JguFgHqwKnVqVIo6NQtoTTT26Mzjww",
        categories = "weather",
        tags = "weather,shower",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudDrizzle,
    #[cfg(feature = "cloud_fog")]
    #[strum(props(
        svg = "eJxtjDEKgDAQBL+y5AFnLly8HMSAXRofEbBIaeH/0VikkmVhYGDy1e6Oc3OHgIWS2a5QePBYJGWkzqTWhCLG/SfDS4mCBFfyMholzxKvYK36ZxSBq03zADdVHrg=",
        categories = "weather",
        tags = "weather,mist",
        contributors = "ericfennis,karsa-mistmere,mittalyashu"
    ))]
    CloudFog,
    #[cfg(feature = "cloud_hail")]
    #[strum(props(
        svg = "eJx1jMsJgDAQBVt5pIA1u+QLMWABFhHwkKMHsX4TD4KQsCw8GGbSWa6KY1W7ARsKMW4eHhrcz5JnhMrkYzFk0V+/UNoKJEZUTktv5PSV2LXUPSJhBpoiupLmoTRFLGA3DgpEftYDwxw9FA==",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudHail,
    #[cfg(feature = "cloud_lightning")]
    #[strum(props(
        svg = "eJw9jEsKgDAQQ68S3Dt2+psWasEDeIiCiy4UXHh/dAQlhAQSXjnb1bHNwxrBkZyNi0BgwKpAwkidSXLzFKA27/iURFncUMukiFo+0MEObEeH0P2u8V9uMssZVg==",
        categories = "weather",
        tags = "weather,bolt",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudLightning,
    #[cfg(feature = "cloud_moon_rain")]
    #[strum(props(
        svg = "eJxtjTsOwjAQRK8ySu9lf97EkolER8MhLCgoKRDnx2CEUkQjTfPmUx/tecftOF2EiRdDOQUxK4Zzl0AC3pzUDcP5q0BcGUqqSUi5h0giGTJJmae1Hj7ba/0/GJRbRh6bWKgkP4s1g/2OtHfVd6rSGb90h8yQsgFvgzYwWg==",
        categories = "weather",
        tags = "weather,partly,night,rainfall",
        contributors = "it-is-not,karsa-mistmere,jguddas"
    ))]
    CloudMoonRain,
    #[cfg(feature = "cloud_moon")]
    #[strum(props(
        svg = "eJxFjDEKgDAQBL+y2CfmTHIhEAN2Nn7A7sDC0sL/YzyCMrCwMLvlkvvEMQ8beRCLh4cDNRx4TRIRew82G96HWsZ3Uss/dJaQFwY3U11GkGCnAA2ncEcV4xEt5fS9PQTGHV0=",
        categories = "weather",
        tags = "weather,night",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    CloudMoon,
    #[cfg(feature = "cloud_off")]
    #[strum(props(
        svg = "eJxtTkEKwzAM+4rovZqdzHMCWSEP2CMKO+Qy2GH/Z2m79VCKbGEkLFTe86fheR9eAR3SZ5jKZVGn8vceRk8BK1eHQ1ZkaG6JNl9pWHaTlVF8pOZ4khSUFgP0RquHN++nShuVnqtTJGHjny+9gPie+QXScymt",
        categories = "connectivity,weather",
        tags = "disconnect",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudOff,
    #[cfg(feature = "cloud_rain_wind")]
    #[strum(props(
        svg = "eJxtzEEKgCAUBNCrDO39+b/KVzChA3QIoYWboEX3pwzahAwDAw8mn/Vq2Jdp82BPMaVVobDgnkDKiI1JU/UU0GtflGdFEi9TyXP/KPl7OhIJROCMjhDsjMOIWP92A4dsJvg=",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudRainWind,
    #[cfg(feature = "cloud_rain")]
    #[strum(props(
        svg = "eJx1zMEJgDAMheFVQgeITUibFGrBARyi4KFHD+L8th7EgxICDz74816PBtvsVgEStJQWBQUPNC6gElgj1FQFA4z3N3JfhizsSp5Go+SnRLGnzvgh9gfEQPElF6hNJkQ=",
        categories = "weather",
        tags = "weather,rainfall",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudRain,
    #[cfg(feature = "cloud_snow")]
    #[strum(props(
        svg = "eJx1zcEJgDAMBdBVggPEJrSmgVpwAIcoeOjRg/tj40EQDCEQeOT/crarw7FOewSKmFU3AYEAZJNQCHInFG0RE9iGB3lcGTnyVMtsGbW8SXk8dgz0T+oQMZD4xuTZ4teZfftu9RA+Ew==",
        categories = "weather",
        tags = "weather,blizzard",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    CloudSnow,
    #[cfg(feature = "cloud_sun_rain")]
    #[strum(props(
        svg = "eJx9TzsKwzAMvcoju1VLlpoY3MxdfAhDhyyFDqXnr2IoFGKC4A3vJ6m82nvD4zZVFshHprVcdmotP+GplBM6MCl3OLqqRLBsozxninMvCGcFbJR19hK6WlMo4j7BWbGgxLIMQgkSm8Hc6sVYKAe9c2oJqecZAiPR0T7X4ujh6kfkP+ELYhNJyA==",
        categories = "weather",
        tags = "weather,partly,rainfall",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    CloudSunRain,
    #[cfg(feature = "cloud_sun")]
    #[strum(props(
        svg = "eJx9jb0KwzAMhF9FZLdq/dk1uJmz5AW6GTpkKXQoff7KhnZJCEI3nO4+1Vd7b/C4TSsx8IenuV66Ndff4alYBIYQKg3Zp1aOQLwd9algzAMQzgBkWDQ7BJM1BYXYJ7jLFhSJr0clAeYlNwPztLP7n5AWkiYgA0G+6f7vfgFeCzuq",
        categories = "weather",
        tags = "weather,partly",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    CloudSun,
    #[cfg(feature = "cloud")]
    #[strum(props(
        svg = "eJwlizEKgDAQBL+ypPfMgbosxNQ2fsDuwCKlhf/HBBmmGqY88TbcezqdtsJ1KAgiwzub0Sc1NyqWnod/ydCVapnHXj8tUw+0",
        categories = "weather",
        tags = "weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Cloud,
    #[cfg(feature = "cloudy")]
    #[strum(props(
        svg = "eJw9jLEKgDAQQ38luPe8u1qPQnV28QfcDhw6Ovj/WCmUkCwvSXn8rbi36RSjBJUju8HAkKaVTEKuQpZ9afh3J4x8TXuZ//lexokqhD0itkJTiCHWoKRsniixomeHwmSslMbPB+OFH6M=",
        categories = "weather",
        tags = "weather,clouds",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Cloudy,
    #[cfg(feature = "clover")]
    #[strum(props(
        svg = "eJxtkrGOgzAMhl/F6m5f7IQkJ3FdmO8hKm64gUodOvXp+9ugSi3IECGw489fGG+X+z/9/Zx+tYpRln4xaeR38mC8UUoLS5HcseLaZxilqUrLVKQPVKU2Xwp16kshBJdZJWfGUknFGqp0EOMi9jidxy/HOI8vGBTOkcxR5skcZchnsXcChFMuAAy+T8L1u00Bpw3brHR4zBmMWg8IFJx13niR5ghBjhltTxCeYCkcHVjaPE367fuEK29fV1lo1g8YmnSytGu2HYl38Yl3rdYDAXtyfckHUGPD9FCnrhCkxYNKWM60qcZ0oRp/wvHBXMHaeKDh9ekJMXV6CA==",
        categories = "gaming",
        tags = "leaf,luck,plant",
        contributors = "ericfennis,csandman"
    ))]
    Clover,
    #[cfg(feature = "club")]
    #[strum(props(
        svg = "eJxVjbEKgDAQQ38ldO95PbizQi246w+4FRwcHfx/bEGUDoEQ8pJ0lfvEMbstjCQRE7EWJUUTI4B9YFIDL12KIKiEWV+uTqKPZLvLaWjbOf0PL7JWI/IVHkOeHTU=",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Club,
    #[cfg(feature = "code_2")]
    #[strum(props(
        svg = "eJxtzCsKACAQhOGrDHaVhVUMq3cRDBbB4P3xETbJHz9mZNbV0bIZlEARbF+miL9SRD0iWcbro8QunHE4H6obZTgYJQ==",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "ericfennis,mittalyashu"
    ))]
    Code2,
    #[cfg(feature = "code")]
    #[strum(props(
        svg = "eJxly0EKABAQRuGr/LmAxmKaxXAbCyUUG7cnZSH1lt/TVvPMqUS0msro3hCDBM6Bdgw2Qe1FQT8uYBwre3vsAgb7G4s=",
        categories = "text,development",
        tags = "source,programming,html,xml",
        contributors = "colebemis"
    ))]
    Code,
    #[cfg(feature = "codepen")]
    #[strum(props(
        svg = "eJx1jksKwCAMRK8SPECLgUIX1tuUIogK7UJv38RfdVFcDE4ymaeCt+nyDoI37rkPIRHoIezLxiI3Upmd+ikjtoRWa41rZY07IUo+ICAhKS0LiFgdmmBO8KJWnMuR1ov97tfSCv4inW6AnTMzFTMUuH1ga0wvoR5Giw==",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[cfg(feature = "codesandbox")]
    #[strum(props(
        svg = "eJyFkEELgzAMhf9K8N7MxK6toMJ+wK67CxtMEBUmY+7XL40yusOQHhIe78t7tJra+Q7XOjszAblLaBkY8vgMGUJf9MYbm6gyRQJ7+kpQQHgmIMilCIopESMnlxJME99ZUx1iiaaaxn7pu+EG09gN86POPB7BYrQxOAzRvikKbfY/IJXoS9DNopOOxDvUGqQZiqxbPLMDFsheCpZOecacgHP5AdV+WeUWrjNpA69tLiThggWRSCVh1P8BmKJo2w==",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "eJxtyzsKgDAMgOGrhO7FRoMPqM4uXsBNUFAQcRDR25tIaStIh/QnX+w+HDOMteqwgHLGgYDAAPIz3BpVYxMxjfUyE0hn5Sxrzb+2iFtTHy7XZZvgwlrlCq70HTdXyoOLBAqJIBpP5ODNPyo7ChRdfugDAK84Uw==",
        categories = "food-beverage",
        tags = "drink,cup,mug,tea,cafe,hot,beverage",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Coffee,
    #[cfg(feature = "cog")]
    #[strum(props(
        svg = "eJx1j7EKwzAMRH/lyG7XJxs7hTR/0B/oZuiQJdCh5PtrR9B2kBEaxOnpdMurvjc8b9OdAgl1xowAtg6OGTpqMT+mdbl0YF3+Maba2C+WoJNWGkByyECQwxnSztL+8zk7OvoSLZhg8FJQEH1Mxo3zAFjOC47GRgcb33XQ8kigbLOhSBdkaKqeGHpSTc2nqIEcYSffe/LY6YTsr7+ND6UAefk=",
        categories = "account",
        tags = "computing,settings,cog,edit,gear,preferences",
        contributors = "colebemis,karsa-mistmere"
    ))]
    Cog,
    #[cfg(feature = "coins")]
    #[strum(props(
        svg = "eJxli1EKgzAQRK8y5L/brMpmC4nQA/QQJS0oKIiI6O1NFESQhZ0ZZp6P7Ri7P8ZgxCAuwWiSNUvtn0dZ++E7NfgF82El+wJbKt1bILDgfClX4B3J0wvgIA3P1b3pWcgltCRVcsk+CtIC+Z3jDdDDKIk=",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[cfg(feature = "columns")]
    #[strum(props(
        svg = "eJwly0EKwDAIBMCviB8oppceknymDU2g9CBC9ffV5CLuMpu5nQLfuKQXpAOht3F3WT9bwYSgBXcEm5c1qpq32NX8jLeBJufujBb0mMhnNGu3oeoPKzAcPA==",
        categories = "layout,design,text",
        tags = "split,parallel,vertical,horizontal",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "eJydkMEOgjAQRH9l0nuxWzBg0nL24keYQmwTAgQblL93WzUhHr3sZDbzDjNm6V2E78PNRysagc0KLfAIXfTZP7NfsrTmkOKtma/Ro7PiQhW0o4KgoFGc+Oi1cgr8kslKfiQsATtMq78w4kRzPjpJRQ0lS9aSb7lK2qWnYRvC2GOewhjvVtTQhIyiBh1z8pNpzW/9d9Fdf96DqjwDy3eAF7yLUW0=",
        categories = "shapes,development,files",
        tags = "cubes,packages,parts,units,collection,cluster,combine,gather,merge",
        contributors = "danielbayley,ericfennis"
    ))]
    Combine,
    #[cfg(feature = "command")]
    #[strum(props(
        svg = "eJxNjCEOwCAQBL+ywV/a66Yoiq6prSdBIBGE93O4E5vsiJnUy2ioT/j0Rpx6FYI4oTYK3+gZ/B2LveYFMSHkdOxkXnlLE2Y=",
        categories = "development",
        tags = "keyboard,key,mac,cmd,button",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Command,
    #[cfg(feature = "compass")]
    #[strum(props(
        svg = "eJxFjFEKgCAQRK+yeIAtRdTAvMwSIYiK9ZG3z1ToZ4aZN4wlXygcQHVnXDAozVYG9PTo7DK4szmFeqYIOfl4X40qFBI0agVcIhdTR9HZhsYM+bff43xyLxPVH80=",
        categories = "navigation,maps,travel",
        tags = "direction,north,east,south,west,safari,browser",
        contributors = "colebemis"
    ))]
    Compass,
    #[cfg(feature = "component")]
    #[strum(props(
        svg = "eJyFTTsKgDAMvUroXiWRQAu1J7AXcBMcHBQcvD8m2kIFocOD95L3CedybbCOJnHH4AQekHY7CBNMpEqIHmYTQ6/+GErqQAKCYhbhnw7t0s9PIqHLQ0SfJeT2FHK9RW/aZolV6gajODNM",
        categories = "design,development",
        tags = "design,element,group,module,part,symbol",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Component,
    #[cfg(feature = "computer")]
    #[strum(props(
        svg = "eJxdjEEKgCAURK/ymQuUUtHC3w06RJT03YUI6e1DK4k2s5g3b4y3a6DI6EGJoUGn24IwVAcS63YJjBHkY4aTafJ+MsV6lrp9Md2Z/nbVjiUIbYx5IDVK+cvVByidyVDJBUUCLUs=",
        categories = "devices,development,gaming",
        tags = "pc,chassis,codespaces,github",
        contributors = ""
    ))]
    Computer,
    #[cfg(feature = "concierge_bell")]
    #[strum(props(
        svg = "eJxtizEOgCAQBL+yoSfebQihQGobP2BHYkFpYe79og0WZLudmXzVu+Fc3U5oqgQhfQp6No3/AzRuNM/Dlby8Xcmj7kqsCelzxWuETDQlgoUZEIQ2wAPrtiZ6",
        categories = "travel",
        tags = "reception,bell,porter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ConciergeBell,
    #[cfg(feature = "construction")]
    #[strum(props(
        svg = "eJx1zEEOwiAUBNCrTOYAWMAAC35v4CGMbaQLE9OQqrf3s2m6oIu/mfdn8jo/Kn7CQHyWqRahG4j1K7REmZdnqcJEaOA45kv7H/P7Xgsm4c1G2OsWm7TsIKegFb/5bqOf20Gn4IxHMB1/KQZEE9t1OKkmpF3+ucZDkA==",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_2")]
    #[strum(props(
        svg = "eJxVzEEKAyEMBdCrhOxLJ1YGF+oNeojBkSoMpYhQvX2TDIUWF8Gf9+NfWy+wB7zTCuQ2CxYWeRcHC0Z/lX30qbZ0ZGgBbwhpBCTDc/IkQec6+pZTB04twlBacn2UzswhNI649a57L5pwUQrRH/WZYVBAVpNUTXNeMRIyFPIDaf0j+tXil34AZrg5JA==",
        categories = "account",
        tags = "person,user",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "eJxNjNEKwyAMRX8l5L2sycbWgfq8l31EsTKFMoYI079fYlcogldzz4n5zCXCYvFJN6BpZmAY9Qw88ON+/AOjMyflncnBF4ghvWKxSBPCNy0lbs9m8YKQq0VGkPusmgrO+JT9GiD3yktHmiLQqNBWO7Omd4DGfU+VkKWVejRSVVBF/qBWdN27oyfTHf0BPM08Lw==",
        categories = "account",
        tags = "person,user",
        contributors = "lscheibel,karsa-mistmere,FPDK,ericfennis"
    ))]
    Contact,
    #[cfg(feature = "container")]
    #[strum(props(
        svg = "eJxtj70OwjAQg1/lxH4ml+ankUqfgK4d2KowMBSJAfX5cUDqQhXposT2F2d4Le+H3C+nyXvJyNUpkiKowSt6bnHVhE47lMWQvXyHa4tiFreqOXSSqiK2TGEKhbawJaTqhNeBR49eGo0wOYDJD9ZYmiqigiY1aQPxdhqHc6s6Dnthc+INZbZw9TApsH/Tkybj6/TxF+WIQpWVN+2P8pOxdEacC8KufgD6tEfM",
        categories = "development,transportation,mail",
        tags = "storage,shipping,freight,supply chain,docker,environment,devops,code,coding",
        contributors = ""
    ))]
    Container,
    #[cfg(feature = "contrast")]
    #[strum(props(
        svg = "eJwlijEKgDAMRa/yyS42GYpDmxt4iBIFBQcpIurpbQlveMN7yfZqxwp7MrEQ7HXXpkCaRu+aznJtWDLNLOCpREQEZ2C5Wb4+90l/uGwWBg==",
        categories = "photography,accessibility,design",
        tags = "display,accessibility",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Contrast,
    #[cfg(feature = "cookie")]
    #[strum(props(
        svg = "eJx1jLEOgCAMRH/lwg5yhIoD8gd+BImDo4Px+60MJiSSXpu2L3f5rNeBfTUbA0Klh0oH2qKKWu1jxUp/mZKn117yF7I4gfbtPH8oZ1DGNIBhxAimAUtg7NADGX01yw==",
        categories = "account,food-beverage",
        tags = "biscuit,privacy,legal,food",
        contributors = "it-is-not,ericfennis"
    ))]
    Cookie,
    #[cfg(feature = "copy_check")]
    #[strum(props(
        svg = "eJw9TkEKwzAM+4rxPVkcvLFCkh/suvtIy9zDYITA2t/PbmkxCCRLQun76gJjxg9FoCtEPXaMJV3sU1Kbaoe2ZowIv3nskpEYYcl4R1g3lGl+S9/1tphT05Yr6Wh/MNCtOvIEwUXnB4X45BpMMgbKhULdHLrBDzblnPEHX0sp+Q==",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "eJwlTkEKAyEM/ErIXavBli6oP+i19+JKFUopIlR/v4l7mckkM0P8p34zDApo7whT+MpsTx7ChNFfxBZ9y6lDmwEJoeT6Lp3vDuFf917OcQTkojaWZ4rgtOSi/716gT3gw4G9JWW1BaNI6Y2Bni4ZWYkC1sWatBxAoDeG9YZUxAN+5yz/",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "eJxNjMEKwyAQRH9l2XusiikJqH/Qa+/FSBVKKUGo/n13jdBcdnZ234x95XeEqhyqGaGxaoSqh2dd0NsLY952uI1nPcPLCM9/eI+hQIr5mQo9DMI3byUda3NIiZ2ECw5T2VCac95+HiXB5vBmQF3DpIQCOelJrDT03QTJJ3ZAPikZOgEaxEqjF3GF/wFp4jg2",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "eJwljMEKAyEMRH8leNeaIKUL6h/02ntxpRFKKSJ0/fsm7iXDTN5MfLdPhUnJIBk4VG+iePqJy+d4USzHXssAru3FQx5BgGQE/7V98BkcK+giUu9TRdray/H7HAx7MvcAeC0WHYK3ZN0mhx6heI3UgXhGXxYBBG6Ts4Z0Iv8BfN4s/w==",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "eJxdjMEKAyEMRH8l5L7WiJQuqH/Qa+/FlSqUUkSo/n0Td+mhlxknvhn3LK8EnTySQeiG/YIwjjz2HNxJsOAmPI//MPsx8oNrig1yKo/c+MMifMrW8v7sHqU5tXLguTrEuC294N73lmHzeLVA57iQItCLWdTKYm42ajlJAs6ZdJwEGFAryxySifAFaMA4Ng==",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "eJwljEEKwzAMBL+y6G7XMqY0YPsHvfZenFDlVoIhye8jOZcddpA2b0vrkGX9SS/EiXAWehH2de5yi2OITREVp6Hmh/3V/P92wVzoncDP5tgzgovOTxrxk1owZQ3ahUMbF4jwk8YYsol6AY4tIcg=",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[cfg(feature = "copyleft")]
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYpi6EcW7QIWQKClqEtKjbp8lffB7vsR5Vzw36ZHIzobazBH1/FJ6GF77KvWPNtCQk42MJCLBwbRbR+J72RD6IWBUT",
        categories = "",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[cfg(feature = "copyright")]
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYmi6EcW7QIWQKClqEtKjbq7h4vMV7rFfV+4B+mZwn6D9duywJL7MLP+U9sWfaXEQyawwlIMDCdSyi8cmPfWzSANyGFd0=",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyright,
    #[cfg(feature = "corner_down_left")]
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiPVIwQdNZtOoRQkCA6kATdviGQt3mLL3Arb8n1BLdc+x2Vh9FwMAs8rFYU5kEocOoXjqh2K+RZkxOoJTPJbe7HQugDUxcX5g==",
        categories = "arrows",
        tags = "arrow,return",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownLeft,
    #[cfg(feature = "corner_down_right")]
    #[strum(props(
        svg = "eJw1ylEKgCAQhOGrDF4gFaUX3Rt0CKFAQXQhCbp9myADw//wBe71raVd4F7auKMyHkbDavzhJRSFbSkKnEbGGdXh4J49yUPPSWVjJxZCH42kGIE=",
        categories = "arrows,text,development",
        tags = "arrow,indent,tab",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerDownRight,
    #[cfg(feature = "corner_left_down")]
    #[strum(props(
        svg = "eJw1ylEKgCAQRdGtDP5LKkYEOjtoEUKBguhAErT7ZgJ5PxfeCdTrW0u7gHpp447KerAr7OAMSCkMyzQYKI0MZ1SHvFlvyTMyMs31WPdzRvgBlccYkg==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftDown,
    #[cfg(feature = "corner_left_up")]
    #[strum(props(
        svg = "eJw1ysEKgCAMANBfGd4llUEIc3/QtbtQoCA6yEt/3wriXR/JaHer/QQZtc8rGY8QFapomJY/MEmeBY5ktuAguGLX/CanvEWLO35dEz9bDxf3",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerLeftUp,
    #[cfg(feature = "corner_right_down")]
    #[strum(props(
        svg = "eJw1itEJwCAMBVcJLlAjSn9iNugQQgsKooFKods3FuQd736OpNe3lnaB9NLGHQ1awDBxdoLBMG2rYpI0MpzRHB583pM+WB2q/YPujzXhD4zGGII=",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightDown,
    #[cfg(feature = "corner_right_up")]
    #[strum(props(
        svg = "eJw1ykEKgCAQheGrDO4jjYkIxrlB2/ZCgYLoQG66fZMgb/Pg/0hqfnMqN0hNpT3eOAs7uBUQFn2GaR6ESUKLcHlz/DFuAVXZPpzwxI6V8AdSBRfl",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerRightUp,
    #[cfg(feature = "corner_up_left")]
    #[strum(props(
        svg = "eJw1ysEKgCAMANBfGd6lJYMQ5s5d+gihIEF0kAT9fUrEuz7Wmp+cygFaU2lXMB5mAgLfkRGe/iCssZ2wB7M5BIe3XeKIOFiytH69J3kBW+4X9g==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpLeft,
    #[cfg(feature = "corner_up_right")]
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiXHJmIQL1BhxAKFESFJOj2TUG8zVt8rrdyl1wP9JbrOL2iGcSwBitkWQU3/Sa4HkfC7tX2iksvkcEwEoE1J7IfFxQelrAYkw==",
        categories = "arrows",
        tags = "arrow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    CornerUpRight,
    #[cfg(feature = "cpu")]
    #[strum(props(
        svg = "eJx1jlsKgCAQRbcyzAZSqWBA20GLiIrGvxDpsfvUj7DQn2G49xy42q2zB3cZVAjhtgh3uqddPBuUPQKvdmOf/kE3URh00gJJyaKXz/CM3ifPsBgcZQfqULGI0a8QxUaB7LhcUDkXVUNUFKqsou+oB7+vVjg=",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "eJytjFEKgCAMhq8yfG9pFhaoN+gQsoKCHkJ6qNvntKADxM/H2PbzWVojbTPQ5YRqBMQ0pAA68+ptXf7e7uFYYHJiVBIG1KHBHhjJqTR2oEBjQT7R2KaOybyX1GMt675S85v0BhzFLfw=",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "eJwly0EOgCAMRNGrNHMBhegOuIwSaWJckCaW21Ng9RfzJtR8CWmEB7WIE1QyP0Ui3AH6+ZZi2w6q06SwjUMKL3+Z1K2fxRlpflWtftqhUgdU9Rpw",
        categories = "account,money",
        tags = "bank,purchase,payment,cc",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    CreditCard,
    #[cfg(feature = "croissant")]
    #[strum(props(
        svg = "eJxtkLFOBDEMRH/Fut4mduwkKy3XbEPDD9CdQkEBEgXi+xlHaAs4reJdbSYzb7J/3r7e6PXx8uHSSKuoUkjfuIrpVBkbq5Qgxz9S6YOqdF3jPTUG9dBjyCDD4bZkIVbp9Hu5XPeHjLnuZ5gWCdokWNnEtmMToyY+KI0aYT25xGFp1yj3AnNI3HrGr1HWYwga9X/EMzxmAXyAxtnZclFBHqxE+/rwe2xITvKUGdpM6VUqyhv6rxdGfEMwS16J8pLGr/NfROUEZLuDqIO0zUSEzhmc5CiEUpyEy9Rznmd/ANAiXZE=",
        categories = "food-beverage",
        tags = "bakery,cooking,food,pastry",
        contributors = "karsa-mistmere"
    ))]
    Croissant,
    #[cfg(feature = "crop")]
    #[strum(props(
        svg = "eJxFyisOwCAQRdGtvOBJOy+kQUzRNbX1JBVIBGH9fATkmiuO5lgS/tu8F1jFRYI4Z/2SOBP0GCbokuJBfn5TS8uHSzZ+uRTQ",
        categories = "photography,design",
        tags = "photo,image",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Crop,
    #[cfg(feature = "cross")]
    #[strum(props(
        svg = "eJxdzqEOgDAMBNBfafAd62UVJGMag8UvICoRZIKvhzloapon7i6f9TI65mEVIVQQKPbj92u6JCfYI0mQMFF3mDZ18skgMBqrqadfD8NYt+QJ91Dy2NeVB2qWI5I=",
        categories = "shapes",
        tags = "healthcare,first aid",
        contributors = "lscheibel,ericfennis"
    ))]
    Cross,
    #[cfg(feature = "crosshair")]
    #[strum(props(
        svg = "eJxljjEOwCAMA78S5QNtPSAGymcQAxLqwAS/bwqhrcRkJT5bdiGVkCOVk4+dKVRRiLau3m3D9y6nK1I7hl0hapka9JY/Ov5gE8YHK2N4Vqwohme0XRMLWv8TNAPoFPvCN9+VNzk=",
        categories = "photography",
        tags = "aim,target",
        contributors = "colebemis,ericfennis"
    ))]
    Crosshair,
    #[cfg(feature = "crown")]
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Im0yIDQgMyAxMmgxNGwzLTEyLTYgNy00LTctNCA3LTYtN3ptMyAxNmgxNCI+PC9wYXRoPr6hDrQ=",
        categories = "gaming",
        tags = "king,winner,favourite",
        contributors = "ahtohbi4,ericfennis,csandman"
    ))]
    Crown,
    #[cfg(feature = "cup_soda")]
    #[strum(props(
        svg = "eJxtjkEKgDAMBL+yeLea0LQ5VF+gjyh46EXw4P/RFBRFCexhlgybtrwXLEOzBijIRQGxY80MRl+PDXPxTvyTtkYnUmgzps40Y7pks0AL+Z8igiQH5yNqmIkgZ75Zb+z7vhLbyjYUvssDEdktxA==",
        categories = "food-beverage",
        tags = "beverage,cup,drink,soda,straw,water",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    CupSoda,
    #[cfg(feature = "currency")]
    #[strum(props(
        svg = "eJxtjEEKwCAQA7+y5ANFC6UH188sHgTpwZP7+2rVQqWnkGQSJzFLCiSFYSxItGtmnPBu67V3KV6B1DB2kFrGASrdlcdVtCEDbJU1WAdVzLminzOdwz90hO/7+JujCd8MXTZi",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "eJxtjk0OgjAQRq8y6b6j84c0AU6gW/cESTBhQdAQvb1DNYYFaabt13l506ofx/v06GF+10ECdK86EPvp0QLMHlNoqsMPa6qpfQ5wq8NFgLhNIHDMy4CxyOhKbDgmSChX22kJ2EK6kRSoJ/eU5Q5M7AMXHXSvJ8DH1vwTX0+KAoqWK79Ev63VRULxaZHRPGoUVALy7bz6i7/7A1kiRYg=",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "eJxtzbEKg0AMBuBX+ckD2Cbn4QXuhG4d2rV7sUILDlId9O3NiYiDBJL85IPEtut+/dCimRKxEJo5kSf8LaoNS47qeNlYHfv3+MUn0dPBv1hvCofrWuwhXIQy86wOVthwOD+wPDiAq7vYohA5Yc7U8VNZeM09VDteAE+SM/0=",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "eJxdykEKgCAQRuGr/MwFahQXAxp0gLbuo4QCF2It6vZpRFC8xdt8NsS4pi0gn440YSozZYcjVoRcLtTZ5mGdTeO+YHY0aBjP0gs02jvFYPGm6oo+lNUPqpddGuEhyA==",
        categories = "devices,development",
        tags = "storage,memory,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "eJxlizELgCAQRv/KcbvkHYgJ6tzS2tAmFBhINDTYv08lKIhvON7xnj3CGWFxOLIENZgkNLRFosDAINtY8KRfFoVn9Lartbdp21fI7JAY4SKHBiGXQ33B+lbVrdbjfp2StPTv3mJqJbs=",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "eJxdjkEKAjEMRa8Ssk9s0lRbaGfj2kOUKii4kEFEb29nlEGHQAI/78PL7TK26wnaq6AhjAUVoT0LiuKQN5/vkG/1foZjwYM4VvCs+8ABDBQiS9/iq/bj5jFwD5IlkDmwNWArQP6B5sg4kedAiSIrJY6T0WTy49NVQIxjTZC+ZdnxFtzCvgFGazMK",
        categories = "food-beverage",
        tags = "pudding,christmas,xmas,custard,iced bun,icing,fondant,cake,ice cream,gelato,sundae,scoop,dollop,sugar,food,sweet",
        contributors = ""
    ))]
    Dessert,
    #[cfg(feature = "diamond")]
    #[strum(props(
        svg = "eJx1jrEKgDAMRH8ldE9MG6UItX/gD7gVHBwcHPx/tGm3thwcd/A4LjzpveDczO7Ig2WS5Gi2oMZV8pfb07JCtgYQjQpgF2DUBRwSWCYOE8OUD8UPV9AfXg==",
        categories = "shapes,gaming",
        tags = "square,rectangle,oblique,rhombus,shape,suit,playing,cards",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Diamond,
    #[cfg(feature = "dice_1")]
    #[strum(props(
        svg = "eJwli7sNgDAMRFexPADg0FDE3oAhEIlwOhRZgmxPTJp799HFmk+DpyRTRtoQNJdLbfiXcUVov9YeQkdzSJz9J/E+TCEx7hSAgk4L+eatfIrAGO0=",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "eJxVi80JgDAMhVcJGUCNUrDQdgOHEFtMb1IC2u1t9KKX9/fxXEmbAKe8s3ikGeHMUfiNl8cJoT5aWhmbVbXgev0Fd6zCED0uZMByN5AiHT/IApkfugF7USBL",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "eJx1i00KgCAQha8yzAGqMRAX6g06RKQ07kIGytunrmrh5v19PJvjIfA4XBFK11yLqla6cUwni0MyCHcKwj16O7eft9cuDMHhRhoMTws11MYvUkBqwAyQ/qEXTn8n9w==",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "eJx9zEEKgCAQBdCrDHOAagrEhXqDDhEpjbuQgfL2qasIcvMX/818k8IuwCEeLBZJI6Tb4oxQckHILVNu1RW9cDtyZqx/zpybMHiLKynQPExUqZYv0h0h9UNl72MP4wwvVQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "eJx9zMENgCAMBdBVmg6gFhPDAdjAIYwQy82QJsr2Aic94KU/6Wu/SWEXuKIXtkgaIVucEe42UwlVIrfgEA+WduXMWP+cOTdh8BZXWkDzMFGlunyR/hFaOlT6+qaA1Mceelk3AQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "eJx9jEEKgCAQRa8yzAEqC2QW6g06RKQ07kKE9PY1tSqw1f/8x38mhTXDEX1mi4oQOMSN89OrxQkhFYvjFfWOIpszvfyc2ZfM4C3OSgNxNyhBMr6RGn+YbjBqGqktpK/vBLZOPoY=",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "eJx1jF0KwjAQhK8y5D0xmz9bSHoCPUSxxRQUJAS0tzcxD6K0LPszfDvj03zJiPNyjTkwUgzPZcqxna/AylyLkAypqbTWNfhDNQ7+MeaIKbA7HUWvQAZaWF56VEIZfIZsxfWNW27K2z/jGvJEEq7m1sRv7tmBuigkbaDiILPHLNwe6tD/oDelfEKw",
        categories = "gaming",
        tags = "dice,random,tabletop,board,game",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dices,
    #[cfg(feature = "diff")]
    #[strum(props(
        svg = "eJx1ybENABAQBdBVLhbgH7pzGxhCorhSIeYXjWi078lo06gXV8EUF5JT8cdU7mRCsM8w3tlo4hYH",
        categories = "development,files",
        tags = "patch,difference,plus,minus,plus-minus,maths",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Diff,
    #[cfg(feature = "disc_2")]
    #[strum(props(
        svg = "eJxdysEJgEAMBMBWlhSgl+Azlw4sQqJwgg85fGj3GvUhvpbdHfW5+jLBj0wshHpFIvh+V9P2+U1/7gXhuy9bh61gzNSzgKU0ieON1U5fah30",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrhO6NJortUHsDDyFRUHAQcdDbG60FB6c3vJ8vyLzJMoKcrSE2IEfaTac0MRTpj2Ht9wmG1nQNEEtpCR06WyEDobc18h3fUQyZfKlMK8m/IvmHVMhZdKCm6h6+5gWkrCuC",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "eJxNizkKACAMBL8i+YBHHfOZxUKwSqW/N5CAVgO7M4ypWCPhdKqNErZTDYWEs//C4dn+WVE97QJ+nxZG",
        categories = "devices,multimedia",
        tags = "album,music,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide_circle")]
    #[strum(props(
        svg = "eJxdyzEOgCAMheGrEC6gMJgOtZdpGEiIA1O5vS2NYpz+pO8rtnqVIOmMEMPQpKzNXrEekXAzRfjaidIcHT1Pf2xHWOb7C4ty7dxKYPGVh7drdlO+0w2u6yxH",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[cfg(feature = "divide_square")]
    #[strum(props(
        svg = "eJxljE0KgCAQRq8yzAUiA3GhXqYkhWghQs3tm9GMqM38fe+NzWEukMmhQshnbVwnhBjSGovD0SAcaSmxjSSht4N43m5pD0AjR+Ip7poRde98N8IK9WY7oxvzuOoD95Dqo+qYH3oBpbYyTQ==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "eJxVjEsKgDAMRK8ScgFpQFFIc5nQRaG46Kq5vaMFxdVjPjz12r0V6pkTkw9AwMi8sekyV9NWz0KR5jrAFRDEgynkqfG+X6Z/I0xpf82f8gLhVSDy",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "eJx1kEEOgjAQRa/yw761M0MLJMANOIRBE03EuHAht3cGiHHRpmn/4v2+Ttq/zu8bLkM1UQTPjrxEkI+OfegYohlR+zh1YJ6NakWbRp1oRqe0GvuTicb+p2NQnMWLiNaST6nRpOBksWPeLJtODlm9aTOqhRokm8MVcI3WdLYzuAG1Ji7IxebQt21lMAd0KLBkV81MoNxcGw9lHkAJ+z/86R/35xUrDRVX+BzBGprrnlq10vgFrS1t3Q==",
        categories = "medical,food-beverage",
        tags = "gene,gmo free,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    DnaOff,
    #[cfg(feature = "dna")]
    #[strum(props(
        svg = "eJx1kDEKwzAMRa/yyS7XsmLZhjQ36CFKOnQpdOj9qaShdHAwhg/vP1l4e98/Tzyuy62A66FJtZGCJYkIMkomXfbt4rV9+5UHSjk4tdGJ0xgdJVXuJJar5Z4bVcsyUdkKB4WLcClchEvh4sR9cYN63e8Mr7B9iCeogWNJzE0xwmuqfia4ZAycMHXVJzNmD3PwfM4z2P7b+d/4LwfzZpk=",
        categories = "medical",
        tags = "gene,gmo,helix,heredity,chromosome,nucleic acid",
        contributors = "karsa-mistmere"
    ))]
    Dna,
    #[cfg(feature = "dog")]
    #[strum(props(
        svg = "eJx1kk1OAzEMha9idR8TO/5JpFKp6hYuwA6VRRcgIYE4Py8zhXbRajQeZxLbn5+z/Xz9PtHb4+ZZKjlL6gFO4+xKnU0bKUcOCnZqx6LctbFlMRZp+FtrFKMkrp2zNhJO9cUqsoQHCVYaUhClcEcUYXOkbWbI55vd9mFC7LYXFENErjjHivNtINIzcd5GQ2KHh03646ErHgJPOfOUK55y5ikXHrjdfQFCQm3jDlAnsZ/bqHF/C3VQOmBPoH8StJ/vhdPn+3IjwiA4zTDLvTQWx2r91OUx1GL3OMBBe9op2TEohcbTfvXZBWTpJdjMFu3q7FVC51aZ8kGHVsdHGajTcLB23XfuVWi1s9KS0I+4BnPBmETtEFsCs6jxj/4Lg1t75w==",
        categories = "animals",
        tags = "animal,pet,puppy,hound,canine",
        contributors = "kemie,karsa-mistmere"
    ))]
    Dog,
    #[cfg(feature = "dollar_sign")]
    #[strum(props(
        svg = "eJxVjEEKgDAMBL8S+oBqKrEIac69+IiCQgURDx7s703Ei4SwzLK7vG/HCg2TCw5uFVRtQdE4vCzcWUr4LFeFJbkZI1CePJXBE9j338X699C8PNqEleUB4+4aEA==",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "eJxFTUEOwjAM+0rVe0yTlLFJbc9ceMQUkEDigCYO8Pul22FKHEt2LJfP/H2Ge403STgHTrM4dSQfJkEmvfLo8tT16TAGysie6OvHMUDDBWwEhRBGKDEEGls59Z5W7LXY+xHsXyNLDPbbealxe9rttgJfEyHo",
        categories = "food-beverage",
        tags = "doughnut,sprinkles,topping,fast food,junk food,snack,treat,sweet,sugar,dessert,hollow,ring",
        contributors = ""
    ))]
    Donut,
    #[cfg(feature = "door_closed")]
    #[strum(props(
        svg = "eJxty6EOgCAUheFXOaOj954xR7iSLVY7m4FocDy/UhiBnXT27bcnvwX37k6NoFxbJghp8/Q84vjBqsElW1uUrKc/SKFMRAOUdRHt9gElqxzE",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorClosed,
    #[cfg(feature = "door_open")]
    #[strum(props(
        svg = "eJxtjrEOwjAMRH8l6l6Tc+xWkUIldlgZ2CIxZGBgQP1+riBVHSIPHt75ncu7flp4nocbUrCWqgYNkQNuXWHDUk5bZil7kiC21AFUkOQeoVBXiehfmfjEskngcwW7fx+MEDWVPF+d3rtvocN/EIczk+1lIy4H8BfisZd9AaQ/OCY=",
        categories = "home,travel,furniture,security",
        tags = "entrance,entry,exit,ingress,egress,gate,gateway,emergency exit",
        contributors = "karsa-mistmere"
    ))]
    DoorOpen,
    #[cfg(feature = "dot")]
    #[strum(props(
        svg = "eJyzSc4sSs5JVUiusFUyNNIzVFIoAjKUFJIroXw7G32IEjsABowLyQ==",
        categories = "shapes,text",
        tags = "interpunct,interpoint,middot,step,punctuation,period,full stop,end,finish,final,characters,font,typography,type,center,.",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Dot,
    #[cfg(feature = "download_cloud")]
    #[strum(props(
        svg = "eJxljDEKgDAQBL+ypPfMHRcugRjwAT4iYJFGsBDfr7FII8PCwsDks14N++I2BSvFlFaDwYM7gYwRG5OlqhTQ5z8p74skKq7kuTdKHiUWsNzpb44INmhn0qEf09kfsA==",
        categories = "arrows,files",
        tags = "import",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DownloadCloud,
    #[cfg(feature = "download")]
    #[strum(props(
        svg = "eJxNzDEKgDAMBdCrfLIXTbS4tJ1dPISgoCBaUMTe3rQ4SIb8Dy9xcbwWTJ4GYbC921EgqHXYaOrtvxu5TUvBVfkouHhsaVv3GfFY9+v01IHVif4B51zoh4IrNLEntoQknhrCk6volrLVF/sCTvUm0w==",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[cfg(feature = "drama")]
    #[strum(props(
        svg = "eJx1ULsOwjAM/JUTu03spHlIpQszH4HC0BGp/L+4dEAM7WCfcuc7J5nfz8+K1+3ygAWYrRrssszXQS/zn5iQT7V6rmWdYFHPct0x9YAmCeaS2TbJElnmPYjDJYLERgkZ8SDCiiY0bV20aqVlNK0IR7NBDUXt3tidVfbKqJq7RN7VJZGI2ngqmDQjkS3Ma8TKVWa7M3FkvI3gDB1I58HONlR+Q9cotk8mYoHvmH6OL0h8XSk=",
        categories = "multimedia",
        tags = "drama,masks,theater,theatre,entertainment,show",
        contributors = "danielbayley"
    ))]
    Drama,
    #[cfg(feature = "dribbble")]
    #[strum(props(
        svg = "eJxtjL0KAkEMhF8lbL/jJZv9g71rrH0IWQUFCzks9O3NcoUWR0ImyfBN6/e1P67UP7NjcdTfm64mk1vaYfOX9jy/bnSZ3YkrOFDEVI8cIUJ2K/FkDVUSSBxr1UEP6o8VRjZXULT7hCSeoeztMSI8J4RCNmSHLYjJ0nPsipApWVWoUCHOyD/iC1azMj8=",
        categories = "brands,social,design",
        tags = "design,social",
        contributors = "ahtohbi4,karsa-mistmere"
    ))]
    Dribbble,
    #[cfg(feature = "droplet")]
    #[strum(props(
        svg = "eJxNTTEKgEAM+0pwb7W9q0VQF2cfIefgKOjk6+05SSAkISTjud0H9qlZRaG6ORzdBycvHSkJJR4okbFdIY1yoGcrxAYNr8hRyIhs6SHCAoOkSvbfgz/NPLb1cH4B+JcZPA==",
        categories = "weather,gaming",
        tags = "water,weather",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Droplet,
    #[cfg(feature = "droplets")]
    #[strum(props(
        svg = "eJw9TjsOwjAMvYrFnkecxE4ilUocgKknqMrAiAQTp+clQxXFlu33W9779yXP2+VRRR35SEgSpQRFy2wF0SRyUg+wGnh2TlVDhvatInVxVJMqRnYYs6LYYBRJaGVQumRU37Io18LGnR2R95QIbzJtymy/y7pcR6p1ObMRb04fv2tEZ9JZ43y0yYjpgFGOnyKdUg77ZF6MdgbbHb3JLJMUmKTrwNbT7w85Azev",
        categories = "weather",
        tags = "water,humidity,weather",
        contributors = "Andreto,ericfennis"
    ))]
    Droplets,
    #[cfg(feature = "drumstick")]
    #[strum(props(
        svg = "eJxFTztuQzEMu4qQXerTx/J7wGtu0EME7dChBTJkyulDJkgymLBNkSL38+nyKz+fhy8fVkOI3xrmaT20LC1Dh011c3zHRuBrtrZh3nzqZhUStqUSZJoPTasWEEnlirnyJ8yStDGpFVg7JkptODTRaktgbxfobFBJca64rRO6oGG2YCSvh+P+wQLH/Vnj393iXqPZooVwCiTlWQT74T6aMRJ/tT1gIccgpKPkraAmzZEw449ud9/X5hvxnkEj",
        categories = "food-beverage",
        tags = "food,chicken,meat",
        contributors = "Andreto,mittalyashu,ericfennis"
    ))]
    Drumstick,
    #[cfg(feature = "dumbbell")]
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrfLpHTRpbh9i7CA4ugoP3x3QRhEiS6ZHPt2u7D+xrOsswox+zb2o2dmn2ujCEySewjAx/C4QXiEBJo0SUH/G0CZVqFKhe42MPCnM0mw==",
        categories = "maps,sports",
        tags = "barbell,weight,workout,gym",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Dumbbell,
    #[cfg(feature = "ear_off")]
    #[strum(props(
        svg = "eJxtjksKwkAQRK/SzD7tdM83kARygBwiqKAg4sJFcnuroxAXWcynh6r3pnvN7xtdejdlksppDpzIlifBKuTPvhFOhVttlJOSso9N4Jjd0J2sPXR/DCBQ4JJYAnqxcMjoSTxIV65QSRozfHlzmlXaL4WEc2g4RiirAOZbxMuRV9QIo+26c5JxJsE9zPj39mzUqoBJ3TmP+/NKi/ZO1dEiOB2tv3HdRkQtNHwAtrxE1g==",
        categories = "medical,accessibility",
        tags = "hearing,hard of hearing,hearing loss,deafness,noise,silence,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EarOff,
    #[cfg(feature = "ear")]
    #[strum(props(
        svg = "eJw9jDEOgDAMA79isbekKQ0MpT/gEREMjAyI95NWUFmWB9uXL71PHOuwCRafVHxCNSFURdBOECfNgTRaF//ezaCh5LEySu6kkBqKbcZtanKWT1AGf2jC1K8vtAodcg==",
        categories = "medical,accessibility",
        tags = "hearing,noise,audio,accessibility",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Ear,
    #[cfg(feature = "egg_fried")]
    #[strum(props(
        svg = "eJwtTkEOAiEM/ErDHYTFAibAxbMf8GaqiSYezK4H/f1O2T10KJ3pdKq8Znk/SP7NhMmxIfmhC9rNzUS8vR42Ua+f2/dJ92YukYp4C5awYxOlgUyejq5EipRB8QIawyQ6ZgvpEOaB+vMDNwvgYrOdUOxY3QkpduSz3goezidUuWoqTdNXFuQqvA==",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "eJxFjs1KBEEMhF8lzL3L/HTSE5gdEM8+hKigIOLBw+7bmx6XXRpSRVKdL9vPy+8HvZ2W54Bl0lGfHBZKK8QHdYSTMGTN8mX1tcFmvxsJdFAincYcMRnUo4E1yMtmE3gfc+3wZgj1Zd8eJnTfbmhJuCmJYR3+KB086L/y8eb8yo3WYb02pbfiRituEy4M1wfRpuBV4ayV4R7zwhx35tfn9ztd5LToQmctKb1c9Xy0KzpD+x/mtjzj",
        categories = "food-beverage",
        tags = "egg free,vegan,hatched,bad egg",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    EggOff,
    #[cfg(feature = "egg")]
    #[strum(props(
        svg = "eJwtjTsOhDAQQ68yoh9vMp/MRgJuwCEQFJQUVJyeBNHZenr2eK7XQfs0LFlIZCsQZSSnwD/Y4dGSc04MLWxQY0V1rqiFP6JwR7JWs5GjvIBeJchgShkS1JQ+28k9zOOvP88P0ZEa4Q==",
        categories = "food-beverage,animals",
        tags = "bird,chicken,nest,hatch,shell,incubate,soft boiled,hard,breakfast,brunch,morning,easter",
        contributors = "mittalyashu,Andreto,ericfennis"
    ))]
    Egg,
    #[cfg(feature = "equal_not")]
    #[strum(props(
        svg = "eJxtzDkKQCEMBNCrSC7wGSGFEL3NLwSx1tu7xKWxChnejKSYf1PhyZEp1hP6rVbfnjIF+YYJciT4UjWzgZdlJVjrm7pDG1D9IJg=",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "eJxVi0EKACAIBL8ifiAUOgTmbzoE0Tl/nwodOg3Dzsqae4Bxx4ZglDhuFHStqFKiUcny3/JBQU6+9gK5sBW7",
        categories = "maths,development",
        tags = "calculate,maths,operator,assignment,code,=",
        contributors = "ericfennis"
    ))]
    Equal,
    #[cfg(feature = "eraser")]
    #[strum(props(
        svg = "eJxljbEKgDAMRH/lcG80rVUK1dlBP0Lq4NCCg/+PSUEXOS7Du3AXr/0+cUxNGWHZ9OTUyXCVJY/OOOpzoMGIk1AIlSsUXfY0wCuHqv5rsrKTumaOrfbP8V3ZrBW+jP+keDAjIHzRA7T1JMo=",
        categories = "text",
        tags = "pencil,drawing,undo,delete,clear",
        contributors = "maxwellito,karsa-mistmere"
    ))]
    Eraser,
    #[cfg(feature = "euro")]
    #[strum(props(
        svg = "eJxtjTsKgDAQRK8ypM+abP4QAx7AQwQtUlp4f0xSiIUsszDzGCZf9W44V7FbaNU0i5KXkZX8JbalH6ATfA0UMKTGSUcseQuUMDQzeGg+FCxZGHKIHcX+uSNDUXYzW+/AA+FjIqY=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Euro,
    #[cfg(feature = "expand")]
    #[strum(props(
        svg = "eJxtzTEOgCAMQNGrNOwgLaYyIDdwZTdxYIDEwXh+i8bowNI0zctv2NcjwzarSgiEmjVXBj71aHy1IDO3VcUwNBnD6xcHyIYSoTCbxciFsEiggyU+GZ/cbVuwXVzR8qqb/vATduUvL5SdLq8=",
        categories = "text",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Expand,
    #[cfg(feature = "external_link")]
    #[strum(props(
        svg = "eJxNjMEKhDAMRH9lyF3WpKu40Pa8l73uXVBQEC0oon9vWgUlkMmDl7GhXjo0jn5cgc1a1gJBrsOZXt/iyZn8q5uh3JXk7St2eBumYR/6sUWY+nGZHXEBA+FzfZJ4Kd4mcRNHwoRd02iw/rwJW8w8+sk9AOK3KVM=",
        categories = "text,arrows",
        tags = "outbound",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "eJxtjV0KwjAQhK8y5N01u/kttIUewEMEFRREBH1ob2+yQX2RkG+WZGZ2fJTXBafJHAbKGQ3FwcGC6/UkXmHmcd+c8/j1s6XkEMjmpY7eodNqlAXhmNpokRSFHXFEp5p2TDFBKOY/7ZEio2GpiSAabGL1SF3wdJ/mgZKHov8GckMr51/v7Xo/Y5XJiBhsXNVg7bL112ptpvkNLqs8lA==",
        categories = "accessibility,photography",
        tags = "view,watch,hide,hidden",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "eJwliz0OgDAIha9C2BttO7gAN/ACbgZNNHEw1UFvr8DAx8v7oXO+N1gYxwK5XDUNkHsHBFL9z4X5jgmFOtsJ6d70WEEfxlwQ9I3fGKuVIpYPvqkZCg==",
        categories = "accessibility,photography",
        tags = "view,watch",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Eye,
    #[cfg(feature = "facebook")]
    #[strum(props(
        svg = "eJwdjKEOgDAMRH+lmW+gtMsmxjQGi1+COIFAkAq+no2cuCdeXrnbAzrXsEumBawtUqR5jDu5bskN6hnmnKGXsIHtSE1Ifq8/C/QNtUwjVj9wLRT+",
        categories = "account,social,brands",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Facebook,
    #[cfg(feature = "factory")]
    #[strum(props(
        svg = "eJx1jbsKgDAMRX8ldA82QW2H2NnF1cGt4JDBwcH/xyj4RAnkdbj3ypwXhbFxHQP7bA38XrYp1fcHch8nDFAdo7woGm0fN/DgkhSbfZIzhAJQVPoi/EdekhVwBCwG",
        categories = "buildings",
        tags = "building,business,energy,industry,manufacture,sector",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Factory,
    #[cfg(feature = "fan")]
    #[strum(props(
        svg = "eJxtzzEOgDAIBdCrEHcQUNuaVG/gBdxMHBw6OBjPry2JDhoSFl6AH/fl2GAdqkmYgnoQR43vF0ccFKzzXYKBnAT0xKypo1YUhNruC6EQ4wmzgcJ/ZDHmExaE/zvtrPmUiem5GmOdA4zxjXH/pSexPLMLEfkw9A==",
        categories = "home",
        tags = "air,cooler,ventilation,ventilator,blower",
        contributors = "karsa-mistmere"
    ))]
    Fan,
    #[cfg(feature = "fast_forward")]
    #[strum(props(
        svg = "eJxdy0sKgDAUQ9GthG5AkuJAeHY7IkhfQSfuvh86KmQU7rHiz395RvE7f+8ZGMEDEtgWsWMcIdk2y2QrURdkF2pAS18BljccAw==",
        categories = "multimedia,arrows",
        tags = "music",
        contributors = "colebemis"
    ))]
    FastForward,
    #[cfg(feature = "feather")]
    #[strum(props(
        svg = "eJw9jEEKhDAMRa8Ssp9ME0ynhdYTzGxnLygoiLhwoZ7eRlEC/0F4/6e5WXpoM/7EkVTAUrLx4MHZvQJV8YyvAjvSP8c+kO5Yp7dV6zQOUwcrZ2SPsBWGAskogrAaTTXpVssvXibrpRrPhQ/pYx86aiWf",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[cfg(feature = "ferris_wheel")]
    #[strum(props(
        svg = "eJx1j1EKwjAQRK8y5D8xu7FmA0lv0EOUKFRQkCKitzdpof1Jvwb27czuxHyf8+OGOSlWyL+kqOp30T6eVtzH1/iecE1qIAZ/zhXV0Q6eFyOgTjvTgRuYrfHwR3io7gBnHPyRmRa3boUHMMNpgYM0wqXgqQWovCzGj6HctiBYXerZbfEPQoBL1Q==",
        categories = "maps",
        tags = "big wheel,daisy wheel,observation,attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    FerrisWheel,
    #[cfg(feature = "figma")]
    #[strum(props(
        svg = "eJx9kLsKgDAMRX8luPtIJFahCm5dXN0LDh0dxMGvN3WwSFsJpSXNOVyid3s42MZiYeCK57Zi8KeRQujlRQbpVKaPPh/gKiZde8ekXxMSkJM5G+ZRqgElqpUyCFJEKLk/nVI6KZwBh2R47Hz6KMuPh5KeIb8ETwTXDVYDTAY=",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Figma,
    #[cfg(feature = "file_archive")]
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym4TW0kIUtqFFdbuKCC1UtVWparo3+NSkLqw+IZ7vrs03KYa7oW5OhCpXLZIngQZKCCTO3sQEvCkCgGkjuQvYuFIvhKbLZBH9RkpgOJIbj2n7ZlCjWLKdFhLyjT07dI23QOGvummZ2FYO0FPBE2MH/CLlCk3Y24fkF+KWQN5KYyojioruNma+ZvPuqkKu7KdwTIj/7HivJv4BhWQRhk=",
        categories = "files",
        tags = "zip,package,archive",
        contributors = "karsa-mistmere"
    ))]
    FileArchive,
    #[cfg(feature = "file_audio_2")]
    #[strum(props(
        svg = "eJxNzdEKgyAUBuBXOXhfy6PNBtr1Lrbb3YsLEqSkhay338nVGoJH+D//o6Ode3gadpeA2HNpERCqfLDAhyrrG5dlDXg9H1FBr4Ss1af1e6vjGJbghw7i6If5ZRinNqCrAaygyXAjhPeNJFQqhJUgcy0nXiVx1Do/udDBRH0M3EJD0XwbdlnNN/2pPd80KfGnPi2NPdI=",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio2,
    #[cfg(feature = "file_audio")]
    #[strum(props(
        svg = "eJx1j8EKwjAQRH9l6X3XJF1jC20vXjzo1YO3UoUWSluwBPx7J0WlChJYspk3zKSY6rmla5mc7E625Fwr2wYXQ5bFkRVl8XGoeJYcA09noEerkT/4hiPOlsRFTfxeYUxJKYWgpCFNqmITc6piGvtH3w03msZumO9lYpWQopSRM5Qt4AsB/KlmoAa2tQOMZjhKJqx2xn5Zxbyd/sdo+Nto6I8RpfLAae3JA1oy4Fh95QljR0sW",
        categories = "files",
        tags = "music,audio,sound,headphones",
        contributors = "karsa-mistmere"
    ))]
    FileAudio,
    #[cfg(feature = "file_axis_3_d")]
    #[strum(props(
        svg = "eJxNTk0LwjAM/SuP3jub0GkO3c4e9Op9oNDB3AqOgf56s09KIAnvixdSM0Y8K3MnX5Tg67lhMNw8Vr+JMkAvR+IcsPy4FOVtNf9MHU5zYB3S0H27tn8hDW0/fipDXk26BOwgi3CTqHjvICA3SZQsZ6PeSgm89Qf1B6d4MEc=",
        categories = "design,files",
        tags = "model,3d,axis,coordinates",
        contributors = "karsa-mistmere"
    ))]
    FileAxis3D,
    #[cfg(feature = "file_badge_2")]
    #[strum(props(
        svg = "eJxljLEOgzAQQ3/FYr+0viPpkjJ3aNcO3SJ1YKnUATHw9RxEAiR0w9mW/fK/DD2+9+bFNkToIxWF4rqcuBp5CPxrTz0Gou9biM86npouXxZgl3esglYM5m2u1ITq6qXPefRjC6oTiRiimFDMNcXd1p4BOzQqBA==",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge2,
    #[cfg(feature = "file_badge")]
    #[strum(props(
        svg = "eJxNjb0KwzAMhF9FeJdry7+Dkydo1wzdDC04YBJDs/TtqyRNGw6EhL67Sy0vBR6duFkIg80EBIqlgZBKlO5KCoJ0A6n/D3kr6EWfLqu9T22u7zpOT2jzOC2vTmjLLI8IbI8b+EUYPhod6JANmK2Og9HDfu3y91PB4QmgvXRrLlU0qNGwV6OT7gd/ALbOM/I=",
        categories = "files",
        tags = "award,achievement,badge,rosette,prize,winner",
        contributors = "karsa-mistmere"
    ))]
    FileBadge,
    #[cfg(feature = "file_bar_chart_2")]
    #[strum(props(
        svg = "eJxtjr0KhTAMhV8ldK/X5GrtUJ0ddHUXFBREC4qgT2+sP3SQQBLynXOIsfXSQZOKEqMgBspVTUAQniV5W9E78KQOyT9IqpIgLi7zLjLzOwMzY6dhG/qxBTv14zKnAiM2cdNAIWgnvCUsfn9giV6l8nIepB3BD4LKof+LDus9N5k=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart2,
    #[cfg(feature = "file_bar_chart")]
    #[strum(props(
        svg = "eJxtjr0KhTAMhV8ldK/XhNrboTo76OouKCiIFhRBn95Yf+gggSTkO+cQ6+qlgyYVJaooAcp1TUAQnyV5WzE48KQOKTxIqv5RUlzmXWT2dwZm1k3DNvRjC27qx2VOBSo2cTNAMRgvvCUsfn9giVmlCnIeZDyhD4LaI/2iA+sCN5s=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileBarChart,
    #[cfg(feature = "file_box")]
    #[strum(props(
        svg = "eJx9j8GKAjEMhl8lzD3ZNum0FUbPHtbrHvY2jIJC1QFlwLc3nXbQixJIQv4vf0g39vcj7NfNzjpqgXlrY8/AYOZg5L9A7W8Rt/4loXaTazbdT3bYdOM1PdLpcoDxerrcb+vGOkU1RWADcQYrovBylGkVwApZHpA8icd5QIZrE9wkxHEwQIFJ8kjiIiXREmUgLyQr7d1cmYwHkwSriOqrK5htS1VXLLaotqrnoURc1IRlubfllKb6dLHGcvj/7f360VnxgE49lI3tBwC+ALsMTC/hCXGWYyA=",
        categories = "files",
        tags = "box,package,model",
        contributors = "karsa-mistmere"
    ))]
    FileBox,
    #[cfg(feature = "file_check_2")]
    #[strum(props(
        svg = "eJxFjbsKgDAMRX/l0r1qY3wM1dlBV3dBQcFHQRH8e6MUyoUQyMm51g3XjLFSHYNoNjwQCMkf0tQXUdYajjJQk4eTlu1mVdv4e6+tO9ZnXfYJ7lj266yUERtklKAE5Q96RGDfuKUw4pWwDq4XV/slhw==",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileCheck2,
    #[cfg(feature = "file_check")]
    #[strum(props(
        svg = "eJxNjs0KhDAMhF9l6L2uCe2uC9XzHtard0FBwZ+CIujTm6qIBJIwfDOJ8+XcoEpVTiay4N+7ZDDiUFq2hR6CTG6In4Lm4hPZ/2neVOZeITBzfuzWrh1q+LEd5ilVZMQkLQHHSA7wQgS+fui/IBuOwGhzZ+2HsSkT",
        categories = "files",
        tags = "done,document,todo,tick,complete,task",
        contributors = "ericfennis"
    ))]
    FileCheck,
    #[cfg(feature = "file_clock")]
    #[strum(props(
        svg = "eJw9jr0OgkAQhF9lcv2t7rGckHA0Nhba0puTBBICBImRt3fPv2aKnW9nppqva4dbMBf2cK5zkXLswZYcmMSSTyLkLZUqemoOlJ9ZFHMnH23CLYNc8sgfRR8zCDI1BPLITF3tUkldzdOwDf3YYp76cb0HwwJtERRwexRv8IvUVeyXOLSIm2LeID6DKQyWYHziPq5G/taXWse6TLPYk8sbln/xC3JoOUw=",
        categories = "files,time",
        tags = "history,log,clock",
        contributors = "karsa-mistmere"
    ))]
    FileClock,
    #[cfg(feature = "file_code_2")]
    #[strum(props(
        svg = "eJxNjc0KgzAQhF9lyD3WHZM2hei5h/bau9CCgtVApdC3Nwb/GFgWvm9nfajHBq9SPQzIRkxNEHkKNZ+XzN7FZBa8nXek4/YzqvKn+bzyYej+Xdu/EYa2H7+lktiGOByYwyVxUaK8fPxcIQ6FTjl0rdhCqAukbHgCbGwtqg==",
        categories = "files,development",
        tags = "script,document,html,xml,property list,plist",
        contributors = "danielbayley,ericfennis,karsa-mistmere"
    ))]
    FileCode2,
    #[cfg(feature = "file_code")]
    #[strum(props(
        svg = "eJxNTssKg0AM/JVh79pNulYPq+ce2mvvQgsKVhcUQb/eREWWgSQk84gP9dTgW5o3uTQDPx81g2EViUwzRQvp3BDHi4Q/eZq9DvFqKn9Tw8qHoVu6tv8hDG0/jaUhJyIpBdii2IknRcjnD3+yoLvGKiKz6+5AuYYqrvsGYW0xgA==",
        categories = "files,development",
        tags = "script,document,gist,html,xml,property list,plist",
        contributors = "ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileCode,
    #[cfg(feature = "file_cog")]
    #[strum(props(
        svg = "eJx1z80KwjAQBOBXGXLP2t38tdD07EGv3ksUKihIEdG3N1HRHiK5JHyTyaZPxzmdDkj3qLzCHJVRSI+o2KihX7116C/jdcI+qnNHAWzJauo0vSKFFgFDAuaifwKWPNhTIJMjFQ8f1/kRrt5vwU25XXchUybMJTlR8bb8gEtJ3QNZdKXe1MbbsoXc/OQrZOHI7ewoEDR5MUTL1JLbSIOQSZqf6bxb++VZM7lv6xNdX2i3",
        categories = "files",
        tags = "executable,settings,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FileCog,
    #[cfg(feature = "file_diff")]
    #[strum(props(
        svg = "eJxtjDEOgCAUQ6/yww7yq0BMkNlBV3YSB0YH4+DphTjIQDq06Uvrz3RlOhax86QMYbUJBNJVsqSbm6I4MqMtJKJTZvvGjwh+qIfB/7cgHqPrkJlYZ9sHrgEvUZUo/Q==",
        categories = "files,development",
        tags = "diff,patch",
        contributors = "karsa-mistmere"
    ))]
    FileDiff,
    #[cfg(feature = "file_digit")]
    #[strum(props(
        svg = "eJx1zT0PgkAMBuC/0nQH6ZtyOlyZHXR1J0IsmyEX1H/vnQ44SLo079OPOI/XRE9jMPk43TwZB6aXseRk/sJjGpIbK3dxV+a7eO+T02B8FiUswUOhEv5QFrhoDwI1n0KFy75uT6J1SziGlarcLfrniDQk8Pxiww6+br0B+mQ2Rg==",
        categories = "files,development",
        tags = "number,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileDigit,
    #[cfg(feature = "file_down")]
    #[strum(props(
        svg = "eJxNTk0LgzAM/SuP3utMtK6D6nmH7bq7sIGC04Ii6K83VZHyIAnvizhfTw2+pXpTnhjws6gZjDRAyzVTRMjmhjgmNH/uiXkd4VVV7hYKK+eHbuna/gc/tP00lopyCcmw4BR2N54WMV8/iMXOuoh6Tun/ABlkATq75A1ZmTCn",
        categories = "files,arrows",
        tags = "download,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileDown,
    #[cfg(feature = "file_edit")]
    #[strum(props(
        svg = "eJxFjt0KwyAMhV8l9N5MY5wKtk/Q3e5id8IGFqQV1pu9/WLZDyEhJB/nnNTyXuA+DhcGY9FdORMQaCkDpKgEdDNp8PIi/f8p2Ypy6IYpnbrGlNpWX3VZH9C2Zd2f42BYaBkBRCAc4AcR+GtrNLJQhGeTCcVTunv3LfpjzB6jAzIgelQxRmX7wSFbScB8+2V4A1iIMYk=",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere"
    ))]
    FileEdit,
    #[cfg(feature = "file_heart")]
    #[strum(props(
        svg = "eJxtj7tuwzAMRX+F8M5b8SFZApzMHdI1QzcjLZAARmKgWfr3oYO8hkDgFSWeS4nDPJ739LPqvpzK1kclpRRLSFn3FXmjiXrkraZnjSP79G49fCzu9TCfpv/pcPyl+XQ4nv9WnXigIZXCXa/gDQn4/qAkaKPQflS40VWW/okVpTCy7jgQiEZeYIw+PmQTwzLMly3C37mjkC3c9mpuccdoGc055osTlAy9bwoySZ0MhQ057wTKEUIC8ehXIY0X8vsx9AVCNUbF",
        categories = "files",
        tags = "heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FileHeart,
    #[cfg(feature = "file_image")]
    #[strum(props(
        svg = "eJxNTtEKgzAM/JXQd7sm0+mg9nkP7nXv0gkWOi1OxtzXL1WREsiFy91xOrRzD89a3DGXBdDt0hIQqDgZXx9MCEbqkVIio0cpi2Yz/4TRpxhodBj94t3QQRjdML9rgTmbeFVACqpVuEuMtm6yvoOpFiTAflmsGBfGcxRub87cq744AcsMpbquK20sKz4b5GZ0lPkDPsc5Jg==",
        categories = "files",
        tags = "image,graphics,photo,picture",
        contributors = "karsa-mistmere"
    ))]
    FileImage,
    #[cfg(feature = "file_input")]
    #[strum(props(
        svg = "eJxNjcsKwjAQRX/lkn1qZkw0Qtq1C926LyikUNuARejfdxr64sIwcM7cCakeIt6lelowR7I1g2FyWPPrWrgH2cKB75cdadn+VlXhNJ9XIfXt2DbdB6lvuuFXKpI2yPBgA5/FRRF5/SiGi2QONQv53kAeZ52z4Qmf1yz6",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileInput,
    #[cfg(feature = "file_json_2")]
    #[strum(props(
        svg = "eJxVjUELgzAMhf/Kw3tdE+rWQ/W8w3bdvbCBgmhhUti/X1tqrQRCXt7LF+PsNuLdN08F5pGUZTBkKhb8urXdg1Tbge/XwxJh8qoZzCWeD8at82+elg/cOi3bt28o0BCaBkvoFMyRED4+ElsCZWaYPBVNUaMonN20qf7vSA3SdUiQFxWzULN5tmVcFOgfdmdBJA==",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson2,
    #[cfg(feature = "file_json")]
    #[strum(props(
        svg = "eJxVT8sKgzAQ/JUh99jsom0O0XMP7bV3oQUF0UAl0H59NzbESCDZeewwcb5fBzxbdae6asDXc89gmHi0TIEKQl4eiEtC8+NSNbf/8ld17hQDO+eX6TON8wt+Gef13SqqZUkuCzawmzFZxJw7GEg8gVIBmQJlTBEjIxzVjSkK7P8C2dKlKegiNMcm8SibSOTUHwvTRQQ=",
        categories = "files,development",
        tags = "code,json,curly braces,curly brackets",
        contributors = "karsa-mistmere"
    ))]
    FileJson,
    #[cfg(feature = "file_key_2")]
    #[strum(props(
        svg = "eJxNTs0OgjAMfpUvvYNrMxSTjbMHvXInkwSSCQtykLe3Iv6kadPk+3WpmTtcPV0s2NS2EQiMDkMy6cq8OIvBIS9qMT8s0+9kqXK7l7xyaYxL7IcWaeyH+e6JrVL1lFB1uRI3SuVCP4XYIixK2xMmT0IID0+r4RtVy63XjTXPZDYvoPsX+cGPYNa2/IWeiDg32w==",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey2,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "eJxNTcsKwzAM+xXhe7rYLNklyXmH7bp7yQYdbFBKKW2/vg6lD4SRkWwptHXf4B3pydfKQe6+FghsgdFt4JOgLA3LWTDyulXusT7PlMKlBKaQv13+fdBFEkKeIrFXHpVtOVrtFLb2P3uwNSVF54g5fAdmKHZvASzuLj0=",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey,
    #[cfg(feature = "file_line_chart")]
    #[strum(props(
        svg = "eJxNjrkKhkAMhF9l2H79Tby2WK3/Qlt7QUHBY0ER9OmNByIhB8PMR6yrlhZ1qgoKvQj8jysGwz9Ly7XSR5DNLfFX0FwmXpTf4V1l9ncCM+umfuu7sYGbunGZU0WhhGQYsA9zGR+LmJ8fBopBgQ6EJa0FnhtQ8lIPJsUrKQ==",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FileLineChart,
    #[cfg(feature = "file_lock_2")]
    #[strum(props(
        svg = "eJxFjsEOgjAMhl/lT+/oVra4w8bZg165EyFuCQGCC8rbWwxqmrRN+n3p76cmR7SBrga2Ng2DoaQ0uODoDvbCCqeDrVn9b4VsZ0OVP2525aexX/s0dJjGNORHIG0EleYgtvuAO1L5ubtlPFObYyBHiF26xxzIElYRS8L8kkmQzpu58fLiG9NBl0vBexgNVRiohX9h3j5vNxI=",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock2,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "eJxNTrsOwjAM/JWT95TaJYEhycwAK3tEK9INVVEofD0JldrKks86+R72FVJE7+jGx0ZDLiYIBG0dVa7MO6KgRJY9oeR+avR1EX/J20M19HYaHgkfRyyEaS5IeI99io7OhPm/4zA+Y3Jkqqr+e7uV0WDJSkKHrsRwKWPQZlkDftGpLXc=",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock,
    #[cfg(feature = "file_minus_2")]
    #[strum(props(
        svg = "eJxFjbsKgDAUQ38ldPfRa6sO1dlBV3dBoYK0BYvg31uLDy5cAjlJlJu8xtywQYBIczERCHk8SmisUtlzkUpQV/5WEtQhWKuyO94qZ7dzW80CZ1fj94bx0IbwalCOOoIPEuB3sQCXuvxaLoHNJKw=",
        categories = "files",
        tags = "document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileMinus2,
    #[cfg(feature = "file_minus")]
    #[strum(props(
        svg = "eJxNjMsKgzAQRX/lMnutM5jWQpJ1F+22e6EFBdFApRi/3jFxIQPzOJw7NrRzh4+jF9elgTyurUBQ7VXo9ucT0CkdyxkU8r6V5pnDK3l72R96G6YhDv34RZj6cf454lpD2hpIhSaJh+JtEhdRyRAWdnQnRM5nzFj95G6teSwW",
        categories = "files",
        tags = "delete,remove,erase,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileMinus,
    #[cfg(feature = "file_output")]
    #[strum(props(
        svg = "eJxNjcEKwjAQRH9lyD01OybaQ9qzB716LyikUNuARfDvXUO1ZWBZeG9nY+7mhFtjLh5kEt8RhCuh5fVYhbP4KoCnw4qsbi9v2rj7nrcxT8N76Mc78tSP87Mxom3QUYMOdREXReXfRzVCErepWcgjQGj3KPnjD535LNY=",
        categories = "files,arrows",
        tags = "document",
        contributors = "Andreto,ericfennis,karsa-mistmere,danielbayley"
    ))]
    FileOutput,
    #[cfg(feature = "file_pie_chart")]
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXH7otN1nYVtoE3D3rdwdtAoYOxFRyCf29bdUrIS0je470mDKvHtS3ObCHiZRAIVC4ppa/JnFiTgRzt71XG7VEVXbNL8q4Jy/ScxvmGsIzzem8L1pEawUEUXCZ+KJH8ddSkNJip5sGQ08igwLEdSer9n8WWsyJXge3BpJnhHdeBFXHds/Xpetm0L1GHN+c=",
        categories = "files",
        tags = "statistics,diagram,graph,presentation",
        contributors = "karsa-mistmere"
    ))]
    FilePieChart,
    #[cfg(feature = "file_plus_2")]
    #[strum(props(
        svg = "eJxNjbEKhDAQRH9lSK/nziU5i2h9xV1rLyhEEA0ogn9vFEVZWBbe2xkX6tmjKdRfg/Sia4LIjmHC6pOan+jUgF97oyRei1ale+3vpQtjv/bd0CKM3TBPhZKYhrhyMEN+iKcS5avxDTHePlIuYCFcbrABdJsr4A==",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FilePlus2,
    #[cfg(feature = "file_plus")]
    #[strum(props(
        svg = "eJxNjEELgzAMhf9KyF1ngt06qD3vsF13FxQURAuKqL/etBYpgbzk5Xsxrl46aCr8UZkr4M+zZmAofGUyrZQYotwRp0bG/1euvlf4QGse/qE1bhr2oR9bcFM/LnOFVEpImgYuQAcwItYEcCOBGGHnqH7XCNu1SyCBw1FFWNSH35FVN3sCbOw3Rw==",
        categories = "files",
        tags = "add,create,new,document",
        contributors = "colebemis,ericfennis"
    ))]
    FilePlus,
    #[cfg(feature = "file_question")]
    #[strum(props(
        svg = "eJxtTjEOwkAM+0rUPeaS67UgHZ0ZYGU/leFGBsTA6zmrUtuhiqxYtmMlv8unyuvaPaxHEr8NxcUlcLSxr+2Etr2a7wX154h0X45/3ZRPLJzyVhvEAuIMV7SQ4oyLWnGYECwhG9DPiEy0QBJDpIHIH4ijZhcbK4Kt3h9K1C5J",
        categories = "files",
        tags = "readme,help,question",
        contributors = "karsa-mistmere"
    ))]
    FileQuestion,
    #[cfg(feature = "file_scan")]
    #[strum(props(
        svg = "eJx1z8EKwjAQBNBfWXJPzC5prJD27EGv3osKLZQ2YBH8eydF2xxaAiEhj5lNiM3U0qNSV7HE9nY0xYWdKUjOvhESsmlpnN7s7yCGzYnSg7Rgqg6HlFCHOPafvhueFMdumF6VYgeErSQklzP8EeB/KXsSWXpYi5YsMhstU7jsKXZrVhpxQ6ExU/PPFvUFnStDKg==",
        categories = "files",
        tags = "scan,code,qr-code",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileScan,
    #[cfg(feature = "file_search_2")]
    #[strum(props(
        svg = "eJxNjsEKgzAMhl8l9G5nMut6aHveYbvuLp1goWhxMuae3lRFJCEJP1/+xKRm6uBtxRMrqYDudUNAUOYoePriSeBOHdJZKOh1k+qxLf+FM5ds6Ewa4hxD30IaQj99rMCKl7hooBL0Cu6IMz6MPrbg54xJJWC0gnL3P1aQJ8Y3iJ2Ph6+SFGC9Vk59XF8A9SU2NA==",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "karsa-mistmere"
    ))]
    FileSearch2,
    #[cfg(feature = "file_search")]
    #[strum(props(
        svg = "eJxNTckKwjAQ/ZVH7omdydIIac8e9Oo9oNBCbQMWQb/eNK0LjxlmeFtIce5wacTJgLkjExmMqoAln2tlj2SUBR/cj5L5emjRht1ib0OahufQj1ekqR/neyMopyEvD67gi3CTZPGn0YLqqKFzIpVUh/Vb4V5/BZvntgd5Scou86XfA0AxhQ==",
        categories = "files",
        tags = "lost,document,find,browser",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FileSearch,
    #[cfg(feature = "file_signature")]
    #[strum(props(
        svg = "eJxtjLEOAjEIhl+F3F48KPQgqTc7nKuDWxOHDg4OxueXOpwOhvDn/yD56qM9O9yO05lnIEd9oTYGhjmGUrRT+eXEF/kyBHdD3chAUae1HoZvrbvVgKzTnwcZCoNjocYYotghHM2XT2xU0BVoAcphuaN7yuMiKDlFyHX3vgHedC2Q",
        categories = "files",
        tags = "edit,signature",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FileSignature,
    #[cfg(feature = "file_spreadsheet")]
    #[strum(props(
        svg = "eJx1jr0KhEAMhF8lbK9ncnpusVpfoa29oLCC6IIi6NOb9QdSKIEkZOYbYlw9W2gyVWIcJkD/X01AEPkKeFtQHHiSRZKHgKo0TIoT3lRuPj4wN27s174bWnBjN8xTpjBmiJsGikAfxsvC5vsHDfi1JFKEkD4KnPiCeEUyOwoFPgI=",
        categories = "files",
        tags = "spreadsheet,sheet,table",
        contributors = "karsa-mistmere"
    ))]
    FileSpreadsheet,
    #[cfg(feature = "file_stack")]
    #[strum(props(
        svg = "eJyFjj0OwzAIRq+CskMNMTaV3MxZeojKHTx06FB56OmL+5MpUoUQ6Ht6gnK/PBpcT9OZE0jXptNSDiNcyoaEIfVEWgOQIWVgUvQes2GuSAZhBM7wyzoea3DwSX7GyvkWIT53jmSwbmTjxkxCiaLL4qXvLa6899oMLH803rQX7z85Gw==",
        categories = "files,development",
        tags = "versions,multiple,copy,documents,revisions,version control,history",
        contributors = ""
    ))]
    FileStack,
    #[cfg(feature = "file_symlink")]
    #[strum(props(
        svg = "eJxNjr0KhDAQhF9lSB8vuxc1RbS+4mztBYUI/gQUwbc3BlEZWBb225mxvlkd2kJUGsyOdMNgqCiWXOdJ+iedpOBf9pxk2LZclPZzvpfWz8M+9FMHP/fTuhSCghvCMGAFE8ELCfCVOJICGXxl1MvsaURmk3TH0tnIZTd5AJnKMFo=",
        categories = "files",
        tags = "symlink,symbolic,link",
        contributors = "karsa-mistmere"
    ))]
    FileSymlink,
    #[cfg(feature = "file_terminal")]
    #[strum(props(
        svg = "eJxNTrsKgDAM/JXQ3UdC1Q7V2UFXd0Ghgo+CIujXm6qUcpCEy91x2vaHgaEULco4A6rznoAgdYj4OjEgeJNBComIuiLOms98i0onLrDSdpuveVpHsNu0HnspULKJhwJKQb3CX8Liv8OiAHOX6RBk+YqcoIz0nwcasDCA",
        categories = "files,development",
        tags = "terminal,bash,script,executable",
        contributors = "karsa-mistmere"
    ))]
    FileTerminal,
    #[cfg(feature = "file_text")]
    #[strum(props(
        svg = "eJxVjMsKwjAQRX9lmH1rZ+wjhSRrF7p1X1BoobQBixi/3rwIkYF53HvuSDMdMzwU3qitO+BLPzEwNL4qt72pENzkmbgUKr4PdXeN4S9qefIPtTT7atdle4LZl+14KaTWhVwTwA2IACZEywBactAZ4cMKBYLldHq593zJenPIZs5QkP/Z5AW0icwY34+Z/AEuaUHK",
        categories = "files,text",
        tags = "data,txt,pdf,document",
        contributors = "colebemis,ericfennis"
    ))]
    FileText,
    #[cfg(feature = "file_type_2")]
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9lyD21OyYxh7RnD3r1XlBIobQBS8C/Nxa1PZSFZeG9nd2Qujni3qirARnFdARRL0XN26myFzGVBc9uRbpM2ag2HD7rbUjT8Br68YE09eP8bJSUNJTmwRp+Eb9KkX8Xi3HMWqLLsola/xEfuQMshNn9wRsecTR9",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType2,
    #[cfg(feature = "file_type")]
    #[strum(props(
        svg = "eJxtTssKwjAQ/JUl99Tu2MYIac8e9Oq9oJBCaQOWgH692yc5lIXdZV6MC83o6VWpBxdZSbiZBgTKp9HyRU4AufCMFNB4XrLyvph/qnanKbB2Yei+Xdu/KQxtP34qxYWYZFlCTnYWrhIRbx2uxOeo2ZvISdTekImtxxEj0YhmZ/5ndDhm",
        categories = "files,text",
        tags = "font,text,typography,type",
        contributors = "karsa-mistmere"
    ))]
    FileType,
    #[cfg(feature = "file_up")]
    #[strum(props(
        svg = "eJxNTssKhDAM/JWh97omWtdD9byH3eveBQUFHwVF0K83VZEykIR5EeuqpUVdqB+lkQF/sorBiD20XCsFhGxuiUNC8/8dme8V3lVpX76wtG7qt74bG7ipG5e5UJRKSEYOjpGfxtsi5ucHsfCaBTW3MpABGZ14IHn0A1zwMLs=",
        categories = "files,arrows",
        tags = "upload,import,export",
        contributors = "karsa-mistmere"
    ))]
    FileUp,
    #[cfg(feature = "file_video_2")]
    #[strum(props(
        svg = "eJxFjcEKgzAQRH9l2HtSsyRtDsZzD+3Vu1RpAlbFhrb+fVcRZGEYdt/MllOTI9pAdwtf24bBKGQMWHH02t24wEW7movjpsRdLVXlaU1X5TT2S5+GDtOYhvwOZKygIh6S9hu4IwLvD19Gmpx2EFa7jzr3anNH7dw9MmKXnjEHOhMWKWbCL5DoN7U5BvKEWRZmTa189QfvrTcj",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVideo2,
    #[cfg(feature = "file_video")]
    #[strum(props(
        svg = "eJxNjrkKhEAQRH+lmHzc6V7HNRiNN9hNDcwEBQWPAUXQr7c9EGn6oHhVtPPFVKNM1J/CwIK/UcFgmL20XDM9BNlcEz8FzdknsL/TvKrUvfbA1PmhXdqmr+CHpp/GRFEoJhkx2CA+wAsR+PqhIwMiWLy19Kyj/A7cAFeUKm4=",
        categories = "files",
        tags = "movie,video,film",
        contributors = "karsa-mistmere"
    ))]
    FileVideo,
    #[cfg(feature = "file_volume_2")]
    #[strum(props(
        svg = "eJxtjkELgzAMhf9K8N7MpFY7UM87bNfdxQ0URAVlsP36PecoPUihSV6+99pybtaOHlVyk4wd6SVvlJTS7Rh0L4kEVO1EY8HovWB33c2fpC5PW2BdztPwHvrxSfPUj+tSJZLBhMuTpuR/4B8BHP4giBHLrmWrjEw+b8FQF8Pi0YjZ5+ilYMZCW84zBgjGFygWRpuTshoMBy6wruNUwuoLppFDOQ==",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileVolume2,
    #[cfg(feature = "file_volume")]
    #[strum(props(
        svg = "eJxNjsEKg0AMRH9l8L6pya66B/XcQ3vtoTdpCwpbXVAW+vdNxbYSCBMy85I6dkuPe5OdHUR6dp1AkK8lRi4VFSd2VECO5X9lVCWbtfXhE2/rOIVXGMYH4jSMy9xkrDRo85AcfjVuFjVvF58VODdWyZJcL0FVMv66o35fYwbzjUpHHirJaxJ2NmRLCInR4Zd6A985NVM=",
        categories = "files",
        tags = "audio,music,volume",
        contributors = "karsa-mistmere"
    ))]
    FileVolume,
    #[cfg(feature = "file_warning")]
    #[strum(props(
        svg = "eJxtjLEKgCAYhF/lx13zPzQJzLmh1nahwbEhHHr6lKAc5IY77rjPn/FKdMxiY6MsYRkjCKSrZEmZm6I4EqMtJHan7PqebxH8UIHB/1jQlE1/YJeU5m97AO/7Iks=",
        categories = "files,notifications",
        tags = "hidden,warning,alert,danger,protected,exclamation mark",
        contributors = "karsa-mistmere"
    ))]
    FileWarning,
    #[cfg(feature = "file_x_2")]
    #[strum(props(
        svg = "eJxtzLEKgDAMRdFfCe6t9plWh9rZQVf3gkMXwUH6/cYOKihvCRxy/R6PROtQzUxAMhxBoKYMCkun7WRYW8LoHlJyZa6Cr6/34O+IkUp2yX1pa8lAOrIf7AuqN55zTScl",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX2,
    #[cfg(feature = "file_x")]
    #[strum(props(
        svg = "eJxNjcEKgzAMhl8l9G5ngt0Uas87bNfdhQ0URAuTsfr0JtVKKTQ/P9+XWN8tPbxb9cRKG6D7tSMgKOUVnH6YFTypR8qLgl43bR67vCpnL7LQWT+PYRymD/h5mJZvq7Biib8aqIQ6ggfibAQDMkTaKPiT4DFx10gIUvEh8XIn1dFpkrLL58LkbKDROjQ=",
        categories = "files",
        tags = "lost,delete,remove,document",
        contributors = "ericfennis"
    ))]
    FileX,
    #[cfg(feature = "file")]
    #[strum(props(
        svg = "eJxNTM0KgCAYe5UP7/340Y8H9dyhrt2FAgVRIQnq6dPqIINtjG08qKhhE2ShXd0DToNCQGgzquROWgRJUVMsgwrXse7nb3wTyZt8KHnw9rLG7RC8cfEQhHZplIgBtsDe4l+RDwRlIQw=",
        categories = "files",
        tags = "document",
        contributors = "colebemis,ericfennis"
    ))]
    File,
    #[cfg(feature = "files")]
    #[strum(props(
        svg = "eJyNjqEOwzAMRH/FKvetTuokk7LigY2OTx0IHJgK9vW11aqooMSyzud3V7/vX6PPrXuKQincC9LEGKhnFAQWCJQRERmKbINMmiWgTD1hgC2r5h7kbdd2NYNjCIURXHRAdJTxzOfoV4I+1uB/N9aLlxnrXilSRjqXdfAtRp216X5aALdbNbI=",
        categories = "files",
        tags = "multiple,copy,documents",
        contributors = "ericfennis"
    ))]
    Files,
    #[cfg(feature = "film")]
    #[strum(props(
        svg = "eJx1jUsOgCAMRK/S9AAaRMUFcAMPYYRYdoYQP7cXdGOM3TV982Z09HOCw6BE2INLZFAMCOTDQum5zxvGnGnQ6roIVq9TInAGRwVyy6kMyusFJKiqo/aXiIYYR/SMJNihTBQvfQovAi1DpA==",
        categories = "photography,multimedia",
        tags = "movie,video,reel,camera,cinema,entertainment",
        contributors = "colebemis,ericfennis,danielbayley"
    ))]
    Film,
    #[cfg(feature = "filter_x")]
    #[strum(props(
        svg = "eJxty6EKgDAQBuBX+bHv3N3tdIO5bLHaBYNhgkF8fjEIBvPHl4/l3LAOzcRKnhU6So1IFLqZUw2Qy0WyUCk5Jm/WlNw+p+R37iJQZ/gj7qH40g1QqR0m",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "gubser,ericfennis"
    ))]
    FilterX,
    #[cfg(feature = "filter")]
    #[strum(props(
        svg = "eJwti0EKwCAQA78SfEDb3UqhoH5HBHEFvfh7WfWQEMKMq5JHlIIqqfTmDTNeaOgB8WW/NX6QBZP2PhUzwd1HDxOwCBH0",
        categories = "layout",
        tags = "funnel,hopper",
        contributors = "colebemis"
    ))]
    Filter,
    #[cfg(feature = "fingerprint")]
    #[strum(props(
        svg = "eJxtkDtuAzEMRK9CuOdEpERKC2zcuM4hFpvCZYrcHx7ZgN0I+9FvRvPI/e/4v8vv9+XHxfzmkojnN5fihxXhOx+TIe1y3b+m47q/fSG2IW5Bjw1JsZg/P4uiw1wNtaM29YXXOnwTNxQ/pxaJVtVREVq5ubIQrBxEe0IV5ewkHMUKE0eYwnNJOpCsi0lOTTIqJhwtfQ3XxCpmJby2DqYlB2UfMMZCT7C8o9jqyDB4ymTywqppoEYjfi7UG4PGkWzjq++bBCYFWp9rWFeWKx/mBwdGaeE=",
        categories = "account,security,medical,devices",
        tags = "2fa,authentication,biometric,identity,security",
        contributors = "karsa-mistmere"
    ))]
    Fingerprint,
    #[cfg(feature = "fish_off")]
    #[strum(props(
        svg = "eJxtUUFqAzEM/IrI3aplWZYMaWDJpZc8YkkPOWShh5L3d7y7LKWUhVlbYkYz8vlr/n7Q5/vpJkFSuPqLsy45sb1wWRLAyDhbmxp7rbRhxickRhL3pGyNcnL8UmHTFGypEWuNJFwgzCIVrapKhaUIKBqLcq2jnCOm4MiFNjzE232TxtDcwLRKTi2x5j40a2KPURd3DKqipHAap8v5beS6nI90TpK5+dUpkMYGdAfTdVzvCePgWciIi2KcDeN1K46Dpb1+XdkS2BK8SBuAMLo4I21l9TJ1dqcV9iRtBPAFZ+Fo858+JngJrKTJP9YXARcazl1XEwg6FTq2pBxU5KOzzcjUaIW8dbl2yPa44UGwm4dx8fkXFy1A8+eue0PvWTKVfPj4AWW/cBc=",
        categories = "food-beverage,animals",
        tags = "food,dish,restaurant,course,meal,seafood,animal,pet,sea,marine,allergy,intolerance,diet",
        contributors = "jguddas,kemie,ericfennis"
    ))]
    FishOff,
    #[cfg(feature = "fish_symbol")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVIwNCu21DU0VTAy0DVxNjRUMDJWMFKwAGElOxt9kEI7ABKqCyk=",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "danielbayley"
    ))]
    FishSymbol,
    #[cfg(feature = "fish")]
    #[strum(props(
        svg = "eJxtUUFqAzEM/IrI3VPLsiUb0kDJpZd+oLclPeTQQqGl7+94s4RClwUhS6OZkfb4uXxf5e3x8OJoouWCUZOhutSZuXQ0RkNzyeLILgWtSognAtioMfGVb9a+UhCZCLG0Tr4eTseHqXE63pW0U+gHba/looFhy0CErCHzU8lJFd13RkI0w+Mc0qWhzTCCJsPm85JUlEZUmqAYN2COUm/FmbS01c/rtHZk43p0wmAw27OZ54UCxc9MC4V6p1sNXo2UXezK0lKkbPYVg9S0+T7FFPU/6YfyunpbP22opz8Mhi5Fnwfawg1d1pBvXdTBnzD6nfUXOYFnOA==",
        categories = "food-beverage,animals",
        tags = "dish,restaurant,course,meal,seafood,pet,sea,marine",
        contributors = "kemie"
    ))]
    Fish,
    #[cfg(feature = "flag_off")]
    #[strum(props(
        svg = "eJxtzD0KgDAMBeCrPLKLJlZwqL2Bq7uooCAiVKS9va2/i0NIHvkSvbbbiL6iuoR0OTIUEITZqoQRamcmo9PIjH6xgkijfhdcWL5On1cfm6dlgJOKRAiOQyf4O/ozBhqROQCUWSaK",
        categories = "account,social",
        tags = "unflag",
        contributors = "karsa-mistmere,cyberalien,ericfennis"
    ))]
    FlagOff,
    #[cfg(feature = "flag_triangle_left")]
    #[strum(props(
        svg = "eJwBIwDc/zxwYXRoIGQ9Ik0xNyAyMlYyTDcgN2wxMCA1Ij48L3BhdGg+p+oJQA==",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleLeft,
    #[cfg(feature = "flag_triangle_right")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVcwMgozyjE0UDDVBRFKdjb6IEk7AKZ7CRg=",
        categories = "development",
        tags = "report,timeline",
        contributors = "tidoni,ericfennis"
    ))]
    FlagTriangleRight,
    #[cfg(feature = "flag")]
    #[strum(props(
        svg = "eJwljDEKgDAMRa8SsgdJTMCh7Q1c3QUFBRGhDtbTm9rh8fPgkXDN9wZLxFGBLTMxqGMgMDjafOqzH0zekJHQ4GjzF1Po6pMUjv1coXBEEYRHIqoP/1Pc2Gpao/QBOnEbhA==",
        categories = "account,social",
        tags = "report",
        contributors = "colebemis,ericfennis"
    ))]
    Flag,
    #[cfg(feature = "flame")]
    #[strum(props(
        svg = "eJxVjjsKw0AMRK8yuJey+u3a4BhygBxiSYqUKVLl9NEuboIQvEHoMfu7f154Xpf7ygFxjpsmjC1zRCD6KCRsK3GQkpBlKk1JWdyIVZ2cSziUKs5vhfMGR50sXGGwZENw9IY23BASR5l6CWM3S6lungfr/0XO9F2O/TJKHz+k7yUw",
        categories = "weather,social,gaming",
        tags = "fire,lit",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Flame,
    #[cfg(feature = "flashlight_off")]
    #[strum(props(
        svg = "eJxVjM0KwkAMhF8l5L64Myyrh23fwKv3UoUKIh5kqW9v4mJ/CGFmwpcpr+E9ybXTM7Ig1zRQKNEGwdwUdofAC+IY3fwmaV8O3tCXpedoX0BNYxR6hW2qWMHH/XmTGZ0CKjNNTyofy9mELoY6tEHZSLKRbKTFP/oFvFgw0A==",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "eJxNjMEKhDAMRH8l5F42GYp4aP2DvXoXd6ELi3gQUb/eplWQEGYyeUmYhyXRJ/JbW2pGITiU9qvKkC1JLrUwOf8MHHqVUcyU8j2S4uAuvOxnF/6/6UsbImvLtGnkhmm/BCaZNOYibaXgeoHKlLnmN3wCXusqXA==",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "eJx1jF0KwjAQhK8y5L1rdpofC2lu4CEKChGK+CBib29SEBVadpmB+YZJ9+lRcB7NSS3qO4kErXg/KRR2PRmg4nxRSgx/edfyuSojO4p3NDkd2mZOv8t8UnrXb0FXYdiBR/FgiRskQkMZvmC+3i5YOBrS4PVxrW6wrFarrZTfgjg8qQ==",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,non toxic,lab,chemistry,experiment,test",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FlaskConicalOff,
    #[cfg(feature = "flask_conical")]
    #[strum(props(
        svg = "eJxtjrEKwzAMRH9FZLeqE5YVgxvo3q7dDR08dOhQ+v21MxgCQYt4D+6ufOq30eu6PCCkP2dTr0pK0g+BFeA1p3tk71TYrIKwWyHOBI7WoOzpwMPg72Asqf/CUL/NVEKkPIqeumzlMhZsZe5Y2UibnxgnpAaZ5g8VAStI",
        categories = "science,gaming",
        tags = "beaker,erlenmeyer,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    FlaskConical,
    #[cfg(feature = "flask_round")]
    #[strum(props(
        svg = "eJx1iysOgDAQRK8ywbN0tz82Kb0BFt8EUYkgnB9qwBQxZt576Shnxb4MKxvIFcnykNPU3pw+5qBkNybVDp3JQ2r87Up4hDYDBo8OpqN68gIOlYU0vPwGYNMp/g==",
        categories = "science,gaming",
        tags = "beaker,lab,chemistry,experiment,test",
        contributors = "Andreto,ericfennis,danielbayley"
    ))]
    FlaskRound,
    #[cfg(feature = "flip_horizontal_2")]
    #[strum(props(
        svg = "eJx1yiEOgDAQRNGrTOoJ7ECziKU3wOJJEBgSBOH8UNNUdMVX79u9PyeOJVwjFBGx+9s0JOuzJCtOgWaEM6xCcHjZFpk8mT1gBR/RPC4Y",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal2,
    #[cfg(feature = "flip_horizontal")]
    #[strum(props(
        svg = "eJx1zDEOgCAMQNGrNO4gLWgwQWYXD0F06OhgOL80JugA6dL05Tdc6WY412H3YLcpERAYGVW2jO4wgBr1AgLEdohhlCSGGuIMlm0tEd7yO8grVs20iMnUFnQ98T2gHzz3SDhV",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipHorizontal,
    #[cfg(feature = "flip_vertical_2")]
    #[strum(props(
        svg = "eJxtyzEKgDAMheGrhO7B5mnVIXZ28RCCQxbBwftj61RKeOP3P33O1+jawi0LjZwocZlJDFmHalnbAsJ/QH5yTCTY4YDEKqsncxFj7wR09AHzYC89",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere,jguddas"
    ))]
    FlipVertical2,
    #[cfg(feature = "flip_vertical")]
    #[strum(props(
        svg = "eJxtjLEOgCAQQ3/lwk7kihIGZGZxdSdxYHQwfL/eYtBcOjWvr+msV6NjNRuY4r5UEMhJLCzKpxO6NzlNouQ0ihy6f5csy8FkeepWU2diFCiAnZCokfCQZjUJ+KEbbso4VQ==",
        categories = "design,photography",
        tags = "reflect,mirror,alignment,dashed",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FlipVertical,
    #[cfg(feature = "flower_2")]
    #[strum(props(
        svg = "eJx1jk0KwjAQha/yyH5qJjFthaQ36AXclShUsCBFit7eSe1PNiWLCd9882b8q3v3uAXVsoHrLCw0WJ78BrJkN6QpISETtxfUO09qIj0PDnqPWP2eeCADOxGrxp/SwsbHxxifd4xBGYX4CYpT/QZVJ+ffbXx+HOtJnC0gbxkTz4WRrRVxUZaVVEczoQozkequh8OrS4tLDkselrx8+geUmUjI",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "eJx9T00LwyAM/SvBezuj0ymosFsvu/YubrDBhFF22P79DNp1ZVAk5uO95CXuEZ9XOHt2QgGHXsV9r4CMA5bXsgYef0FOFUDRsNHmjiAZ16R5QiUPNlPA1zJdy3IxigdUuZNUGVGx4Ha0ZHDpNqX7BdLLMxQM0rv6yTNJpAoHN1+UDaAGLCOLLVOWg0nV0loazD9e2g11w0Z3/YqI/jI+s9NPlw==",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "eJxtyrEKhDAQBNBfGbYPdztB0iTW11xrL1GIYCEiQf9exWYLmWIG5sU8rXkekY8kSkHen16TeGnj57nbuPRbwZDk7xG6pieI7xUFHQtveQvjNMAXWgjWN0iFhmqku1ZxbzSA+musdKxGnp8HNHc=",
        categories = "photography",
        tags = "camera,lens,photo,dashed",
        contributors = "karsa-mistmere,danielbayley,jguddas,ericfennis"
    ))]
    Focus,
    #[cfg(feature = "fold_horizontal")]
    #[strum(props(
        svg = "eJx1yTEKgDAMRuGr/HQPmtSKhdgbeAjBoYvgIJ5fzGCXlLe9T6/9rjjWsAlY6hyKDt8q2sCEPGKBPOLD0gOeeiKjJydnZIqwHE7ghEjWzy/WfDvl",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldHorizontal,
    #[cfg(feature = "fold_vertical")]
    #[strum(props(
        svg = "eJx1y7EKgDAMBNBfCd2L5mpLhdjZxdVdcOgiOIjfr1nEIeW2e3dyblelfXILg4DbJ1ek07LIn/IKAwZizBZwr5ItSa9Ub52AFh0ciUcfNBRsj0o6+PgBbzg8QQ==",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    FoldVertical,
    #[cfg(feature = "folder_archive")]
    #[strum(props(
        svg = "eJxVjsEKAjEMRH8l9J64mZXuFtqCNy9evS9VqLAHWaTo39siVCWHIXmTSfx9eWS6BHMCCMN5XqrQ0IrByDyJG/tMWcVaFreyzKgNDt1OzUnjcf+bQCg6prooKo4aQLYm+l07G326bWm90hYMDKVnMGqrvqq6Zvrg6PuPakm1sH4T/tBUGB29ARr9NNY=",
        categories = "files",
        tags = "archive,zip,package",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderArchive,
    #[cfg(feature = "folder_check")]
    #[strum(props(
        svg = "eJxNjr0KhTAMhV8luCfXnJZeC73C3VxcHdxEhw4KDuLz2wr+kOnLlxNOWIct0vQrWksoo7oBBCrPAaOrHubEkb/izb1TVnGOxc8sFRLg/8TzJZnGvj8QdjVjCoqKpyzQF3X45BZ1uLosntSc1rK99QFHySWH",
        categories = "files",
        tags = "done,directory,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderCheck,
    #[cfg(feature = "folder_clock")]
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6L0xyWa10Pa8i1fvIwoTepAhov/eqKMbgfCS95GXdB8fE1yyOx1AaOhHAQGyYi9eznslz8gYve29TB3GbkWAMQSMFY9iUppBizEIbWBTrqTdN7Akvc1ar6Dv7Dg4mLOzrq/fZNDfLql9xwG4f0q1zHbkA4tMLLA=",
        categories = "files,time",
        tags = "history,directory,clock",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderClock,
    #[cfg(feature = "folder_closed")]
    #[strum(props(
        svg = "eJxNjq0OhTAMhV+lwbd3PSO7LBkkOAwWgSMgJhAIwvOzIQap+np+csKxnJG2thprgonqFhDIPAfG1LzMiSP/xdvyU1ZxjsXvLA0SoH/j2Ul2qL8NhEvtmoKi4ikLmKsu/PKKLpQtIDURpig3lK4kzg==",
        categories = "files",
        tags = "directory,closed",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderClosed,
    #[cfg(feature = "folder_cog")]
    #[strum(props(
        svg = "eJx9j0ELwjAMhf9K6L2xSde6wrbzLl69jylMmCBDRP+9yQZuh056SNP39b2k6m9TP15hqo030L9rQ6XUz1yb6rDITfXongNcanMihwHYtUXHwODkkGXL57D2IP3gMW1eCGPCNGKpV/4JToUjppbd9jvwy2PQeI1dw+9MeARKWFhMFn2GoIAMFFXeIyJGUCP0wuSAhDKl+PwBFgcrg1A+olQHvwsU6GG2IUFyizpddLbZAl/ZAmpl",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FolderCog,
    #[cfg(feature = "folder_dot")]
    #[strum(props(
        svg = "eJxNjjELwkAMhf9KyJ7Y5MrZg+uBm4urg1uJwgkdpIjovzenYCXD48t7LyTfpnuF84iHHrSrEicFhe4zSnocVibnSltO4bcTEo6ROM3Egzrobq23JIR9/38B9CHBvMjCCZqhJyx5074o2a6LzRew14gSEBYXBHu6aAt97fIGkMIojQ==",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
    ))]
    FolderDot,
    #[cfg(feature = "folder_down")]
    #[strum(props(
        svg = "eJxljrEOgzAMRH/FYrcbX2hKpBSpW5euHbqhdshQJAbE9+MwBCR00/n5TpemYc70uzevluCyhgEEcpvAeHe7Z/OZbxJ9vSmrhMAS/ywdzOCxx8sn+Wd7bCAs6r8WFJVIBeDT9OlSVvSpblGQuiWcyahXUs/Wy6bKV7zfLS0=",
        categories = "files,arrows",
        tags = "directory,download,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderDown,
    #[cfg(feature = "folder_edit")]
    #[strum(props(
        svg = "eJxNTbsOwjAM/BWre47YeVBLoTMDrAxsEQwZOjAgvh+nqqIOtk73LJ/6bfS+TPcZUYg9MlcBUz9PTB3peXu3BE3ESkLiV6i60ImEGJy9+JyWcup9SxmtVslIj/TyjsFQZ1knzYKhGrSJbQQ5Q1fMZocMwe/CVfzBbOjHB6JXNqdIY/4Pc4suCg==",
        categories = "files",
        tags = "directory,edit,rename",
        contributors = "karsa-mistmere"
    ))]
    FolderEdit,
    #[cfg(feature = "folder_git_2")]
    #[strum(props(
        svg = "eJxljk0LwjAMhv9KyL21fWe1gbbnXbx6H1WYsIMMGfrvbTeYBQnk602eJDyH10i3yBchmP44gECmmFVQuLpfTaUeOy1Nx+qTaJm0ryl2wVThrKWHadcJi+MUDvViCvkx5+lO+RPZgim/S+yY5sioQ5ucwv6e9WQlK2hPRrkSUbxb/B9xJaw8GN740hC/AFA6bw==",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "eJxVjjELAjEMhf9K6N54SUrvCm3BzcXVwe2oQoUb5BDRf2+r0HpkCC9f3uP5+/zIcAnqaICHTHZmYBi+w5pPU9e66KxHdNJupAmt1egWjRMXwftur58gB/OfAPwkScWIhA4q4LOKfldbRJ9ua1qukF5BEStI77JFwRoU16cfjr5VJgMkWXpAI+MWfAAeITcG",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit,
    #[cfg(feature = "folder_heart")]
    #[strum(props(
        svg = "eJx1T7tuwzAM/BUiO6/iUZQtwM2coV07dDPcIYOHDkW/v1RQOFkCCdLxcTze8r3+XOXr9fRuJiyXulIoJY8plR+xFTUYumZeeXV0v7eIoTX0HTMT8iiU/8KF5aE50a8hTuflZaiel0ObSe5ijmklqsvtuQ1SooUiuCkcxoQTXDEjfM9UzZtfDPiM63duG9ypZ07RA71qrpURKCle32xCCG13NHVEbLlY+udwajUHzrCuo/XzsPEHPFZDyw==",
        categories = "files",
        tags = "directory,heart,favourite,bookmark,quick link",
        contributors = "karsa-mistmere"
    ))]
    FolderHeart,
    #[cfg(feature = "folder_input")]
    #[strum(props(
        svg = "eJxljjELg0AMhf9KcE+al2uPBq7OLl27Sx1uUHAQf7+niBzIW77kC+GluV8yDZ/ma+S/118ZAnE2MrYcxENfkLQEBIlRfJS3FbRL6Ck60+q40Ipqsb/snvXMtjKaNj32Dm2qmiBk6N1MTogU+MilN2B/LLw=",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderInput,
    #[cfg(feature = "folder_kanban")]
    #[strum(props(
        svg = "eJxtjrEOwjAMRH/F6m4TXyKTSKESGwsrA1sFQwYGBtTvJ2ZIGSpPz89nXX0vn0bP03RNhNDUFhAo/AaMW96YOzc+Soljp6xixlJeLBkdcN7ifknxkv4/EFaNjx4UlUIucJ/mevAWcx1dMmlY045QuMGeMTc2zBdtjTNy",
        categories = "charts,development,design,files",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,directory,project,root",
        contributors = "danielbayley"
    ))]
    FolderKanban,
    #[cfg(feature = "folder_key")]
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEBLyPfJeuk33GU7ZHJhAaHSTgABpsRUrR1/JMjJGq3cr84Bx6BJgDAHjgnvRVTZAXzAK/Yl1e4gpadc8S6qXtS5nqM9sOBhYsxED9aWDmuiDS/oFvIp6OOvQg3Z/07nG8S3Txt7IMTT1",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "eJxNTssKwkAM/JWQe2KSvmG35168ei9tcQsepCxV/95d0CqBMMwjE3cfY4DZ41kFTIZyNDCQNEpGdqkmIWXljhJPFgruip8FlOuauxu3lqAdgnyEweTPnNBuXGHvTrm1d9syRdieHhUh7xIhLOs1RI8VwisxDcJjnWPw2OZYDvTueNnS1Wanb6+CUAmy29HwBukqNL0=",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[cfg(feature = "folder_minus")]
    #[strum(props(
        svg = "eJxNjjELAjEMhf9K6J54eT3rFdqCm4urg9uhQoVDHETOf296gycZHi/5XnjpOb4qXbM79oSuahhBoG4ZME7D6tl85Z1E/9spq4TAEieWAWawX+ONJH/o/z8Q3uovFhSVSO2Asytp01qUNN0fN5qRnW4dzZpddPQxUW+KRY1tVPkCP3soiA==",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[cfg(feature = "folder_open_dot")]
    #[strum(props(
        svg = "eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA83nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171pV/wn1FoX5T1u3OJqfd8n5O9TzXdoJ5MGSgPjVE86Hpl9JH5xeKUTNW",
        categories = "files,development",
        tags = "directory,root,project,active,current,pinned",
        contributors = ""
    ))]
    FolderOpenDot,
    #[cfg(feature = "folder_open")]
    #[strum(props(
        svg = "eJxVjkEKwlAMRK8Suk9Mpj+fBmrB3T+B+6KLLiq4EM9vFLGVbIb3wjDjfX4sdD12t0pWyKQ4Q+IEAmmeUQiSa4POGzSJQhBf2cSd6qb4o5K2soNgnP2iKVNzcsbSS/R/lbVKrDIgI35Cv6LZsHvO9EQ3jYf3/OkFiKsoRw==",
        categories = "files",
        tags = "directory",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    FolderOpen,
    #[cfg(feature = "folder_output")]
    #[strum(props(
        svg = "eJxljbEKgDAMRH8luDcmKVUL1bmLq3vRoYOCg/j9tiKlILc88sKdO8MVYRubWaBHs5iVFCOjVQKiJGq0OiQESmFg7Dq0Ow6SUIqgT3ih6jnRzdUhV3ppJtfm0clV06wj098cBpiUhjdFPxuHKY8=",
        categories = "files,arrows",
        tags = "directory,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderOutput,
    #[cfg(feature = "folder_plus")]
    #[strum(props(
        svg = "eJxNjrEKwkAMhl8l3J54ydWzB9eCm4urg1tR4YQiDiL17U1sbUuG8CXfH5Kf3avAtXHHCsQXjp2AgP+VoJzqhVG54I5SmGeMTDEipR6pFgXZL3EzIRyq9QWQN4eLBokpgS3k7Nq8sS/a3N8fNxikcSwOBh77xzhqN/Ymm/aXdZjcmNlOTpgyYXa/XygztQ==",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[cfg(feature = "folder_root")]
    #[strum(props(
        svg = "eJxNTr0KwjAQfpXj9jt7lzY2kATcXFwd3EoUInSQIkXf3kQhlRuO75fPP6ZnhmvAUw/aZbGTgkL3PSU9jxumgjPt2ZnGCQlbS+xm4lEL0MMWr04wx/6/AXQVk0qQhR1UQS8Y/a6uiD7dlzTfIL0CiiKkd/kGYQmo1fSTo2+TRUGGdWgFHxRcL/E=",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = ""
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXQe+OSdt0KbcGbF68evI0qVNhBhoj+vekGnQTCe8l7jxee06vALaqzBe4KuYmBoVuHNV/GnWvhRQ/oTbuRJnROo581jiyEj7u9KsGc7H8C8JtMFiMSeqgPvqoUDrVFCvmx5PkOS1SMvYL8jYo29BFEgkS6iVJoxckgD0B23T2Qa4E/Am8yqA==",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "eJxNTssKwkAM/JWQ+8ZNtq0u7O65F6/eSxQqVJAion9vqvZBCEwyw8yke/fo4ZzxyAzi26oTEPA27MTJqVbvmJiis7+TPlAMqwSYmobiQAcxKAvh/0QrfiM29KywpN2UWZJeRx0uoK+MvEcYMwYEfX8vE/3okuaCNzEHtjL1tIvNB2MYLYo=",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch,
    #[cfg(feature = "folder_symlink")]
    #[strum(props(
        svg = "eJxlTjEKgEAM+0pxb732sHhwOru4uosODgoO4vutoseBZAlJSBL38VhgbopeIAzV5JCJKaCAoCyegh+NgjMwMKlSWKkWo5IM9xqduCxs7ORMuCs7KdpY3pNt/Ia3GljB44O/bb9YT+S82Z5pSl5pryyE",
        categories = "files",
        tags = "directory,symlink,symbolic,link",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderSymlink,
    #[cfg(feature = "folder_sync")]
    #[strum(props(
        svg = "eJxtTjEOgzAM/IrFbjcxISUSZWbp2h21Q4ZWqkTF+3tOEUUC2ZFyPvvuuvf4yfS4VNdE6oYwKik5lGdlvTV/TMC5lrSZeIlJ0lNa++pKOCPOkgZ123PS2Vd9dzLHvlt9vZJ3c8hhz72MC1BrGO8uiQUpRQ1R/ZtOKlGiISo7AdVaNoMHdoocOnPIfGQI1rdF3sRgB0+VosWLxYRBZPQSAFuhNO6sNq5f4sFPkw==",
        categories = "files,arrows",
        tags = "directory,synchronize,synchronise,refresh,reconnect,transfer,backup",
        contributors = ""
    ))]
    FolderSync,
    #[cfg(feature = "folder_tree")]
    #[strum(props(
        svg = "eJyVjrEOwjAMRH/F6m7TixNIpFCJjYWVga2CIQMDA8r3k4IEFlWHyovtu7NffozPQrd9d4IS+rIbQaD+XWCct7+Z21zYSfiuwBJZ/J0lMcQdTDSQNq8NE2qwxwmXbsib6f+QLYXDP0VlnWPEaDkSSwgNxPvWG/NHOELXoyhpddcmCySRm6rogg+6aHwBcFlOJQ==",
        categories = "files",
        tags = "directory,tree,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderTree,
    #[cfg(feature = "folder_up")]
    #[strum(props(
        svg = "eJxljrEOwjAMRH/l1N0mvlShkUIlNhZWBrYKhgwgMaB+Pw5DioQsD+fnO115Le+K+2E4j2ColhaCCN+h8DJtWlxX2WuO/WZimpJofohOdMHjZm+fiKfxNwFcLd7cqKYZDfA6zGXXWsyldzHCwpr+yTPDPFR8ETv+AI+RLOY=",
        categories = "files,arrows",
        tags = "directory,upload,import,export",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderUp,
    #[cfg(feature = "folder_x")]
    #[strum(props(
        svg = "eJxtjr0OwjAMhF/F6m4TX0JopFCJjYWVga2CIQNIDIjnr1P1b6hOsnT+fNblb/8r9Do3t0BwRWMPArlRYNzb1bP5widJftkpq8TIkt4sLczgssbrJflr2H4g/NU/LSgqiSrAo+nyobbo8tzlk+RI6myYdrCGifOWD4i/Le0=",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    FolderX,
    #[cfg(feature = "folder")]
    #[strum(props(
        svg = "eJxNjq0OgDAMhF+lwbfQGxksARIcBovALSAmEAjC87MhGKn6ej+57vRXoL0v5ppQBbUeBKreA2NpM3PkwI048/2UVaxlcQdLiwgYczw5yUz1v4Fwq9liUFQcJQFrMXRlWjE8Mlcdgg==",
        categories = "files",
        tags = "directory",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Folder,
    #[cfg(feature = "folders")]
    #[strum(props(
        svg = "eJxVjrEORkAQhF9lo9/9/x3CbXLUGq1CJxRXKBTi+R2JQ7b6Zmcm49dxCzTXWedIq6AYQaD/dWD09jBHDpyL5UlTVilLFltYHCK84verde8Kwm5TDIqK0aljyBr/O1c0Pm0BuV316wtaJOcBC7woZA==",
        categories = "files",
        tags = "multiple,copy,directories",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Folders,
    #[cfg(feature = "footprints")]
    #[strum(props(
        svg = "eJx1kLEOwjAMRH/lxG5jO6mTSoWlMysDW1WGjgyI78ehUDHAdOdc5HvycJvuC66H3SlD/UHGqY5hlTsY9wUq4RLqzJIiLQbl3JMjc0c+9pwK4k2QuDbpuJvboEoWg3tIZa9n9cnip0ChlCGX3XHYt/bjsDGYwGSFmCVEW1v0ppejWGf04aCVg1YOzewJDs0oDSSj30CwguANYrKBCP6AaGwqS/6RxG3SV/AEmOBGyw==",
        categories = "maps",
        tags = "steps,walking,foot,feet,trail,shoe",
        contributors = "karsa-mistmere"
    ))]
    Footprints,
    #[cfg(feature = "forklift")]
    #[strum(props(
        svg = "eJxNjsEKgzAMhl8l5F5nUmJXaD3v4nV36QYdOBgyRN/eBqFKDn/g/0K+8Bv/GV4RB2IgfsjIwNDqmLItgn24KdOH9JnT9IY5IiOkNSLZkltJr9BRV0wBqf1xdaHq2zuQz/YrhtxCLndDVzSeLrWGGmq8ShjOdhI4VXYt0TAl",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[cfg(feature = "form_input")]
    #[strum(props(
        svg = "eJx1y0EKgDAMBMCvhH2Amh700vQHPkJsMb1JCai/t8WLIF6SZYf1Ja1Gl2AEacqbmoAdqJyC+p575Gha04Dg+zYIfl9MKQpmdsROu4GbtfZt07996AZ6ziZV",
        categories = "text",
        tags = "2fa,authenticate,login,field,text",
        contributors = "mittalyashu,ericfennis"
    ))]
    FormInput,
    #[cfg(feature = "forward")]
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/LiXnGHCFuoNOoRQoCAqJEG3T4J4m7f4XG/lKbme6C3XcXlFK8iCDYgx36rglh8F1+NIOLzaBbTdmqNAYGYE0ZKIPz5ReAGrIBjF",
        categories = "mail",
        tags = "send,share,email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Forward,
    #[cfg(feature = "frame")]
    #[strum(props(
        svg = "eJxdjLsNgEAMQ1eJsgCKC0QRbhsKJEQdticfQNxVlq3np8d+bmSyMsBk8GS6vM4eiGg6BdM0ydhk+cj3GA+fe9ZKYyiblBv5GMjH2jMp+FlvsSMrUQ==",
        categories = "design,photography",
        tags = "logo,design,tool",
        contributors = "Bowero,karsa-mistmere,ericfennis"
    ))]
    Frame,
    #[cfg(feature = "framer")]
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik01IDE2VjloMTRWMkg1bDE0IDE0aC03bS03IDAgNyA3di03bS03IDBoNyI+PC9wYXRoPtteD3g=",
        categories = "brands,design",
        tags = "logo,design,tool",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Framer,
    #[cfg(feature = "frown")]
    #[strum(props(
        svg = "eJxNi0EKgDAQA78S9l51ixWEtj/wEVIFBRFRD+3vdStCDyEkmdiwnmGbEZIj1oTztYYQYo7e1t/u7THeCyZHA3fg7lJcGaVVK0KWwAJ5u637jMiOekLUr1UNE9JXJCmEFapg2fxb/rApTj/9AF7OLC8=",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "eJxVjcEKwjAQRH9lyT0xO0kkgaRnD3rtvaAQQcSDSP17uw20kT0sb3kzmx/3541mFMVB0XfZgKKZi3IL8opDPog15NWVY2pqaqZXrcDv5mt6V7oWdWFPwOgnEMjKaGicjj0TPhwlKqH/KLuKTWUSdWcrXG3PGmMy0XXtJiTNxuPMkcL25AeqoDkw",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[cfg(feature = "function_square")]
    #[strum(props(
        svg = "eJxVizEKwzAQBL+yXK+L7kSwA5J/kDZ9kE3OXTCCxL+PpIDBxe4Uw8RtyQW2rC8riWQkfBMFwlahFXvHZ52L/fXe9BQvrZvi+1kMc6L7DTJkhYfy6KR/3UN89k4hLnBAYHW9bdW5FVa78nDIH+SJJwM=",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "eJxFjFsKgCAURLdymQ2U9vxQd9AiIqXrX8jFavcpQf3MwDnMmGMVJm+xaJqyauFMU5EznxhpyKr/RQqbEIe4s1ioGXRbdKAzeuECNChdFqVKvod14R5omR4P",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "eJxdjEsKgDAMRK8S5gJiC+Ii6Q08hNhiupMS/NzedCW4G97wHh+rKWXBEiie44zEQ0eJW9mMtNRdTeAHPYIIarcggK6aTZ37dDB1rQuJv94v+AJeBB4Y",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "eJx1y8EJgDAMheFVwltAoj0otN3AIcQW05uUgLq9zU2EXkJ4H7+veVeqd8AIklwO0QB2oKsklfbOoIYT6LEb/WBB9OemQilgdTSysIFNH1h6wN2Ef80Lxs4sZw==",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "eJxFi80NgCAYQ1dpuoCAvwdgA4cwQvy4GUKibi9c9NSmr8+eWxEEx3WGEa3obdcmbz8wYhI9/CDHveB27AmJ6ZDiqA1xpVCk1oV4aigi15NpXjP8C1gpHfM=",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "eJxVjMEJgDAQBFtZtgExgvi4SwcWISZ4+Uk4ULs3+elrYGBGzs0NSblOCDYujDJ0FaXm3XGV5KZsHo9yJiyXw7yZQNzKiagNoWc9iPL9/YYvWJUd/A==",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "eJxdkMFqwzAMhl9F5G7NkiVbgTRQdtlhe4iQDTooY4cd1j39pMS0pQT8K8mnX/41nT+/PuBCh4FogF/XOsCF+2toHubpKah5urJjZ8rO2N5pj2Qg3G0Usztug3jHSR/4jbMblztnfdrdTb6XnxO8H4Y3algY9KVitUVAIMeTCo7NoKCOa8KcK2ZlLwgpU2hDUn5mrFlgRKEKDCQouhV1KVB2J9eykiuhJlTgRGfyBknbeWTnAyO3MTbvPQkWkeX2YwNRrb5SA7IV1X38c0y6H5TKmt1VxQf5vVL1HvHSNDVktQjS/NAI4Cn2IK5Kx2ty6Av5i13FluZ/giJo7Q==",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "eJxtj1sKwyAQRbcis4DWMU3xQ91MK1Uo+QhCzO4zuRoCIV+XYc6Zh/vnKarKnt6kqvHEmtQqJRtJgwzuuVPBgUVT9+arSZYwwl5QzBsfmjs9NIzHvmK44y14cLZzh88nP8dPUSnmXyrt1iV/S/Jk5LIqIQZemlGItgthA+8IPM8=",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gantt_chart_square")]
    #[strum(props(
        svg = "eJxty8EJgDAMheFVQhaQVKwV2m7gEGKL6U1KQN1eoyAevH7/e77mWWAP2CJwLgtLQHIIxy1bScIP1GtjMPpGD9GvkzCkgOMAjnt1lY87IMP2JxABWe7ecgJ7/ST/",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    GanttChartSquare,
    #[cfg(feature = "gantt_chart")]
    #[strum(props(
        svg = "eJxtyTEOABAMBdCrNC7AN1BJ9QYOITF0NLh/xNLJ+p7seYxWD4OpGFJQiY9UPAohW/sEQGCrPhdFVRWp",
        categories = "charts,time,development,design",
        tags = "projects,manage,overview,roadmap,plan,intentions,timeline,deadline,date,event,range,period,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    GanttChart,
    #[cfg(feature = "gauge_circle")]
    #[strum(props(
        svg = "eJxFjFEKgDAMQ69SeoBq1TmEbjfwEKMKCn7I8ENvb6egkARCwpM9HQtMAUd21ENDPnENJguzI1+MUapyjKJr1m0GvQJyg5ADWur5NDu9c5Sf2lJnOEPzAO7j3P3XHl0=",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = ""
    ))]
    GaugeCircle,
    #[cfg(feature = "gauge")]
    #[strum(props(
        svg = "eJw9ijEKACEQA78S7M9z3YXjYPUHPkKwsBEs/D9qoSSkyIz2PCpKMI08SCCPmKjvfqMeltiygP5MDqtrdj7LHu7aE8xdEk4=",
        categories = "account,transportation,sports,science",
        tags = "dashboard,dial,meter,speed,pressure,measure,level",
        contributors = "ericfennis,Andreto,danielbayley,karsa-mistmere"
    ))]
    Gauge,
    #[cfg(feature = "gavel")]
    #[strum(props(
        svg = "eJx1jTEOgCAMRa/y416koFAT9ARegujgYuLg/WNxwQHT3zbtS/5PV74P7HN38gD2FM0I7Y2MeBU5w7FsD/utrH+Hd5SbYcmvzGDbLakvnkuqzgGqQKHBBPJDJkRl0iBOc5iEKnsA1mwwrA==",
        categories = "maps,tools",
        tags = "hammer,mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Gavel,
    #[cfg(feature = "gem")]
    #[strum(props(
        svg = "eJxtyyEOgDAQRNGrTOo3dLpNQ5OlJ4AL4EgQKxAI7h+KqUL/9+0+Hse5hK1AnenKKMII6ppQ99Bs+kSz4UgoZtQuqchCFZXyA/vvKY7wAq57G4I=",
        categories = "gaming,money,development",
        tags = "diamond,crystal,ruby,jewellery,price,special,present,gift,ring,wedding,proposal,marriage,rubygems",
        contributors = "connium,ericfennis"
    ))]
    Gem,
    #[cfg(feature = "ghost")]
    #[strum(props(
        svg = "eJx9TL0KgCAQfpXDXbsfJAXzCWptFxocHBqioafPW6QpjoPvP53lqnAsZotAWB2SyWlSMadhkf/xGLgECIB6tqObuIkVYOf1156g2AablQnITvjt2fCM9RczACSN",
        categories = "gaming",
        tags = "pac-man,spooky",
        contributors = "mittalyashu,ericfennis"
    ))]
    Ghost,
    #[cfg(feature = "gift")]
    #[strum(props(
        svg = "eJxtjrEKwzAMRH9FaG9qqTFe7CxduvQjQhNqQ0hCa2jcr69s0yUE4xMS946z6zKlKcwjrEuY49shKyAGUWZoqxBjZ89/Z2df4yOCH8PTR4caITk0CJ8wRJ95hE1GRrKxsyV+Y4eSA4mLOZFYZN2onMVbo9c+ehgc3qWDuZlG99xoyF/JI1AnfSUCzh1NkW/pJtgO9u0OVhW+HMM/ACJDOw==",
        categories = "gaming,account",
        tags = "present,box,birthday,party",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Gift,
    #[cfg(feature = "git_branch_plus")]
    #[strum(props(
        svg = "eJyVjsEKgCAQRH9l6C45iouC+Qd9hNDBY4fw0NdnCXXxEsvC7rwZmLjno2BbplVgK82U4nxLKb6AHiFbWGiwrVbN+Xx95BxkBIZ/M3SQHBC6R7VrXIauyoAYgr6oD10qxzbq",
        categories = "development",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis"
    ))]
    GitBranchPlus,
    #[cfg(feature = "git_branch")]
    #[strum(props(
        svg = "eJxVy1EKgCAMBuCrjL1HWRQK6g06hJiQIBHSg96+zaCIPWzj/36d4hGgjgbFjFCEwQWh0proG/mzumdjtY/ZpwC+NpMb8YWKks2TvuqLyXLlr0537bAZXIUE5RQoGGhERxczju0NvPEpow==",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit")]
    #[strum(props(
        svg = "eJxNyzEKwCAMheGrhFygJKVDwXiZ4CBIB6fk9m0jitMb3vcnrV1bgS54IqgJEn/rsTkd486p1aeAUShjwRvBaWDniX+0UbrWGQ3TiiZ+AYYrIQw=",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommit,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "eJxVyrEKgCAUheFXubhH3SuYgTq3tLaLBQYNIRH19mlCGWf4h/MptwS3zhA04wzcpRnK2POpUXW+jfqzeIusRYk2u3uYNBuQg/DcEhA0cRhLR5toIiVEQNnLT1ZU0di98gbC9Spz",
        categories = "development",
        tags = "code,version control",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "eJxtjLEKgDAMRH8lZBdNBKnQOru4upcqKDhIkVL/3pYWKSJZctx7J81uzbGCuRWSQLAKWwTjQ2IcZJ3qQRZYl/ouwz+QT1sZ/lKnvjZYFE4koHekGRiacFSFbxRlrnjuoxqVUmQgdu3bPD3UNWo=",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis"
    ))]
    GitFork,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "eJxNyjEKgDAQBMCvLPcBjUIwkNwPbO3DKShYSLCIvzcXUWSbXXa8bEn2BZIDmYGQAvUEuepi3zw3+5eVw34q6/ihI54r5kCjRWcmFx0c2prSFCrgG2BgH3c=",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "eJxtjTEKgDAMRa8SsldJxaDQ9gau7lIFBRURh3p721qwg9Pn5T2Isstp1wms00gNwqmxQrB3JKPKVxuVsqR9zG/FeXQM1wyjxo4aICrqnuqgw/mTmySoBAP/q1awyNS67BM4iv+cjHN7av14khTK0JgHQm83Gg==",
        categories = "development",
        tags = "code,version control,rejected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "eJxljMEKgCAQRH9l2XvEGkmB6x907R4WJEiEROjfp3mo6DTMvMcoY71xC5jASB2CiSU9Y4Na1QVr9dJkseRf2qdjhZlxoA7k2GaUpy8gOit6kLPbApEYe4RA920UjIJSFbkmMzv6AiFxLxk=",
        categories = "development",
        tags = "code,version control,draft",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "eJxNzG0KgCAMBuCrjF2gNsEK1Bt0iLBAQSIkQm+fH/2I/Rh792zK+mjDATZrpBnBpt6jRoFGDX1t1Mda3JDEdiP/6NpuB7vGlQRIJzYGhrEUlc7PVGklRgV/HpC4fcmkcUFI1KcSMlVZjXkBzAQqvQ==",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitPullRequest,
    #[cfg(feature = "github")]
    #[strum(props(
        svg = "eJw9UMuOAkEI/BXindqBhrYnmTXZ7NmPMOPBowfj91tMJ6bDq4CCZnveXg+5/56uluL+1rgFhpQs9dS0IfdGv6tTEollqMFT4Wd1xJhF8ILtsI6WbCY6ScoXQzLRQ2kTrZflmP8uLjllV1SdZdlJIiT5S8TSZOpjLwlZ9yMnhaf00uxaEauiD7IsjEfS6XTsDA50Z+iNYeEj33G6bD91g8v2vcQqNnYNpIkrV1b+8lv2AUS9PiU=",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Github,
    #[cfg(feature = "gitlab")]
    #[strum(props(
        svg = "eJyNkDEOwjAMRa9idc8ntiMnkUpnhpyALRIDQ5EYuL9wBGWoGKq8LD92nuz52V93up2nhwixQmpQqAaOHUkciuMEcPJboIMtE/GModX5ZYqY963euQaBGBksX/wPaQYm99ne80djAbEc07TvFB05OZ83gmQUbSwkvFZYDYZSvIadrUZdXfQ6LfNpLGV5AxTnODA=",
        categories = "brands,development",
        tags = "logo,version control",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Gitlab,
    #[cfg(feature = "glass_water")]
    #[strum(props(
        svg = "eJw9jLEKgDAQQ3/l6N7aO722Qi24OegPuBUcOjg4+P94hVoCIQ+SxCe/Ba5FHciGgGgLJmQJYEWoSaPx884wFpxugYCA3hCuvQNteaoUh3qXYj91gJQZuDWd+E+2Up98HKUeEA==",
        categories = "food-beverage",
        tags = "beverage,drink,glass,water",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GlassWater,
    #[cfg(feature = "glasses")]
    #[strum(props(
        svg = "eJxtjdEKwjAMRX/lkvdUkzZOod0f+BGjCgo+yPBB/96UwhhshJCEnMPN9TnX1x1zoUSo30InH79CYjTmQ/+OeUPJeQ97T58HboWukiA2KRTHVqzc9uWCNqvRK0eDQSIMQw0DS4iQkJyNvEtLx+XiPHfB2zxqLfwBvXo3+A==",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_2")]
    #[strum(props(
        svg = "eJxtTk0LwjAM/Suh99YmXalC1/MuXncvVZjgQYYM/fcmLWwTRg4vj/eRxFd+T3Dr1ZXQ+A7QDxgyAYGV0bwtHQsqxZNYU1wDAZxx3eizA9fcjG6xaxpB0rbwZtBchAnf2oF01XXV5ZamyRkMB9eQ29jkRzzv39P7Qqxc479hIGP91lgec3neoXx7haSgfBrODFZcTU8/XZlBdw==",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Globe2,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "eJxtjUEKhTAMRK8yZO//TdRd2xt4CKmCgoiIi9bTmypuVAIZ8jLD2DCuYeoRoiMWwqpiCCGdp7f/6+/tNM49IjtSUxQV1SRXKPHtzi5vl3Yb0DlqWCAt178S5zI6jAps8IDFNyzeEBnuuSu3+APXOiv5",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Globe,
    #[cfg(feature = "goal")]
    #[strum(props(
        svg = "eJxtyjEKgDAMheGrPLob09RAA7XgAVzdCw4ODg7eH6NDJ3m86f/K1e4D+xzWKIhpkzNjGvyhlvFttXQhTKqITJIWg4ERfW7J/nh22hT6OUYmgxBLlw9pCB3e",
        categories = "gaming",
        tags = "flag,bullseye",
        contributors = ""
    ))]
    Goal,
    #[cfg(feature = "grab")]
    #[strum(props(
        svg = "eJyFj70KgDAMhF/lcG9NYpQWqm/g6l5wcHSQPr+tgz9QkFuS474cCXs8NqxjM7MDs+0XHwUCKjJiJNF7hyS22kyhLdwUHlrBtLgfVmokwVv/W9tX0AGs32AFpFpn+fVOcpaCUhcd3EWyydNm9G1k6xO/z57yqkpv",
        categories = "cursors,design,layout",
        tags = "hand",
        contributors = "ericfennis"
    ))]
    Grab,
    #[cfg(feature = "graduation_cap")]
    #[strum(props(
        svg = "eJw9yrENgDAMRNFVTukjbEeOhGSyQYZAUFBQUCAXTE/cpLj7zbNnfy+cW+oiYPLaIzdT1lFojvtSsyVgs8krWFyPgoJ1jAXkWSf8Ad17FjY=",
        categories = "buildings,maps",
        tags = "school,university,learn,study,mortarboard,education,ceremony,academic,hat,diploma,bachlor's,master's,doctorate",
        contributors = "Tummerhore,ericfennis"
    ))]
    GraduationCap,
    #[cfg(feature = "grape")]
    #[strum(props(
        svg = "eJx9j0EOAiEMRa/SsLdSmCIkMDdw696giSazMBMXenupQ1QSmQ0L+vr7frwd7xc4JbU3Bvhgpg2jDyCPGuNWpmPM1zlPZ8jPpOg9gTkpqyA/yodDJ+TCNOwOh4XxSFR3/pASYtDyJ1XOEDruxBYDbgwsBuqwpFGHihUL7lp84xyyqxIWTU9Yo69QaTmslmuLhR/qBYpAaH8=",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "eJxNi8sNgCAQRFuZbAMGuHjYpQOLMEJcboZs/HSv4IXLZGZeHte8GW6hQHh61m94guayqwm5mXCVZNpr5KkJkY/VFEloCXBef9K+gTiPcA7kBVn9Hj0=",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "eJxty80JwCAMBeBVQhYoVoQK6gYdolRpvBUJ/dm+sZ4ELyHJ954raWe4c2TyqBYESvkgbvvjUSO8/yxyzBjcVAvBnRsTRI+rBkuSFaivDpQZiwV9DUGZXj7U4yyp",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "eJyFjbsJACAMBVeRLCAKFkJ0mWAhWFnp9gY/iIhYpbjLO6SYKQWRHSgQVPhovtWBBY9yUI/T6twuPp5ua2+Z99QdVOZTZOGVPPBMb6sBgjNBeg==",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "eJx1jjEKwDAIRa8iXqBYyFAwuYx0KHTK1N6+NRpwMJPDe/7/Wa4u9wnyVqQdof8HQZ6KBzbejDZ2K9LxUhLJsbuam0XNQrWpuJ6HKbaypWUtYR/FZR+LCEF6",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "eJyNzzEOgCAMBdCrNP8CRhMGksJlGgcTJya9vQLVdIDA1OG/9qcsR5JzJ7kDHCgFrCC53rEh8lLTyKpM7KE7XVUvZtxC2lFYnmWpwWz8Nc8w11N++KX97+cz12znA63HYnE=",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "eJxtzr0OgzAMBOBXOXk3jY1QhESYu3TtXgXUsFUoovTtG6P+MLDc4u9O7h63nDAEutTw1yY6lkqqlhXKmpT67mSi735OPOqksTA4KKq2hC5HUgXiF40OBbNBm018ZD1Uzk3kbZaV7QXWZW/nMWakcbqnHKghzGsgITynIadAnrBu+bIsJeOfkkG3XUQPev/Nb+0ND/9JbQ==",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,gather,dashed",
        contributors = "danielbayley"
    ))]
    Group,
    #[cfg(feature = "hammer")]
    #[strum(props(
        svg = "eJxlj8EKwjAMhl8leG9s0jZrYQ68eZgPMfSwwwYeZM/vv4giSNM/TWh+vvSP6TnT/XRYpZBoqFwI9xa4JkRQlm7PieLvmdBXctlroRjSiKIdhv64Ww79x/gKA8sEe1WSiPf/n1UjNyER7oKwFhcwmEdLqLNnRX8Dm41iHIUy21S4GLk4WkjcMgYsX9rCTbnq2VgqubxpQVo5b4KhBWvMyrlb4G0k4PjyvQD1SD5Z",
        categories = "tools",
        tags = "mallet",
        contributors = "Andreto,ericfennis"
    ))]
    Hammer,
    #[cfg(feature = "hand_metal")]
    #[strum(props(
        svg = "eJxtj70OwjAMhF/l1D0mdpvGkUIXZtbskRg6gMSA8vy4gPgpleWTLH+ns/O13mac9t2RFSwUCvsqEPilnDhpPzOkMQ3dlHeLccof+wDmkl4sGzvAN9kibe0tKKzY9M9eIjg4pjg+5PsQ0h4mcu5phPUhUoAw2R2JjBP7xnSWqtCnCeq0xFVqeKfeAdW9PSQ=",
        categories = "emoji,multimedia",
        tags = "rock",
        contributors = "mittalyashu,ericfennis"
    ))]
    HandMetal,
    #[cfg(feature = "hand")]
    #[strum(props(
        svg = "eJyFjbEKwzAMRH9FZJdqybKrgJsvaNfsph0ydOhQ8v2VS0gyBMIh0B33uPKp3wlet+7BBsxjrgICoQkFZQ57D+67oVwaM5SNVOAw6gkpR2RwktLpqh2xBrbU2KUQ5lw9+2OM/k0oTxTyBJUSkmVM1PceRX1jpNxuXWLwamx9uV+B0zr5A510Qiw=",
        categories = "cursors,accessibility",
        tags = "wave,move,mouse,grab",
        contributors = "ericfennis"
    ))]
    Hand,
    #[cfg(feature = "hard_drive_download")]
    #[strum(props(
        svg = "eJxljU0KgzAQha/yeHvbTAghi8Qb5BClSsdFoUioenujIIIym+F97yf+XkXRJWaxsP/ANj43qY0H+IqHbxxcU+/EY/8umBMtMQ1d0foZQvvhoyUxEEuiOGLcPTW2Bc7W7CFBH0bug1nMla0VAi4G",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "eJxtzUsKgDAMBNCrDNmrTSnFResNPISoGBeCSPFze60iikg2gcfMuLEKgsbTwBY2MfFgqHBZhMLdXLKGnvMHprYOWD1pwtI3QY5PETZPbAjTBdL2nQRPZywGXn0WnEuq+G9KfW0H9pwuBg==",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "eJxVj88KwjAMxl8l9N7YxDV10Ba8efEhBgoThniQ4Xx6+0/K6CFp8n3JL355PO+wUVDECj4cVA7pyyluXMrRH7Iq+tf0nuEW1NXiYMEiETAQrzJxSkx5KZtpV9C8aln0MXm04Gk89x4JOoHh4pCHbtGEbgRK4795d94afeHMfIKGVCWWhigVWTppVWeNqVeRqbYmb/a//gfekDyZ",
        categories = "development,devices",
        tags = "computer,server,memory,data,ssd,disk,hard disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    HardDrive,
    #[cfg(feature = "hard_hat")]
    #[strum(props(
        svg = "eJxtjaEOgDAMRH/lgm9YN9YgBhqDxS9BTCLIBF9PIWSMhNT0ru+uYYt7wjo0swX3kcEw9+iWvgZxJvsapMbkag3O9mjG0F6dYyjNrGGz+ILy1ZVsrTXqf5Id2GdyUSAPKSTJ/P3ooIcahGRXwBPOnDYz",
        categories = "tools",
        tags = "helmet,construction,safety,savety",
        contributors = "Andreto,ericfennis"
    ))]
    HardHat,
    #[cfg(feature = "hash")]
    #[strum(props(
        svg = "eJxtjEEKgDAMBL9S8gG7sYpC2t94EMRz/b1NQgXR05LM7Mqxn1uonIkjhYpMicLVzrUFNIoM6hQxUxEm+jRg77erDNHh6JXFxxl/6vxAc5F6tbs31ykreA==",
        categories = "text,social",
        tags = "hashtag,number,pound",
        contributors = "colebemis,ericfennis"
    ))]
    Hash,
    #[cfg(feature = "haze")]
    #[strum(props(
        svg = "eJx1zKsOgDAMheFXaeY3tu6GGNMYLH4JYmYJgvcPrQDVpan6Tv5yt6fDtakRDUKidybwq1oWtlq+xUHmO0pgJzJcplo2iYt6UqVs3lEWdKI4CvoWIIDl0ytYaYUQTzTxpxd0gj8V",
        categories = "weather",
        tags = "mist,fog",
        contributors = "ericfennis"
    ))]
    Haze,
    #[cfg(feature = "hdmi_port")]
    #[strum(props(
        svg = "eJxNjTEKgDAUQ6/ycf9qoiKF2tnFC7gVHP7QwUE8v+1SS4bweJD4Oz4m19YdpLgIgYwlCsU+tSx4559zw5AoNDBRc7VScXbBD2U8+Hqx9ouA5qr6AP+hHYs=",
        categories = "devices,multimedia,gaming",
        tags = "socket,plug,slot,controller,connector,interface,console,signal,audio,video,visual,av,data,input,output",
        contributors = "danielbayley"
    ))]
    HdmiPort,
    #[cfg(feature = "heading_1")]
    #[strum(props(
        svg = "eJx1zDEOABEQQNGrTPSbzcxuUAw30OolCo1EIc6PioL2v+RzCTVBNML9gJS0sPzOZHkH7eUBkC6SUY0ZfA+1NexHth3A",
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading1,
    #[cfg(feature = "heading_2")]
    #[strum(props(
        svg = "eJx1y6EOgDAMBNBfaeYP1lKWibI/wOIJiEoE/x+KIQgQd+Jezo71dNqnNCuxeE3N+ntq9oa6lA9g+RPhEIduGUqKIVIog7sRAolW8HO7AFDBIlo=",
        categories = "text,development",
        tags = "h2,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading2,
    #[cfg(feature = "heading_3")]
    #[strum(props(
        svg = "eJx1jDESgCAMBL9yY59IIqgF8gNbewYLSgv/P4KNFNhkctns+SveGec27BaieR2CH+sp+Basx9wBor9kYQcx7JLwQoKpRPNOYRcVWpKBUNm6NmpB0voNC55gqRUNKX3iA4+XMDo=",
        categories = "text,development",
        tags = "h3,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading3,
    #[cfg(feature = "heading_4")]
    #[strum(props(
        svg = "eJx1zaENACEMQNFVGha4a9MAorABFk+CqEQQ5icoEGD/E19a6Qo1mMSApN5E+VaKcoLP9gJIT3GA/2DlixEu26MJc4wk5Q==",
        categories = "text,development",
        tags = "h4,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading4,
    #[cfg(feature = "heading_5")]
    #[strum(props(
        svg = "eJx1ja0OgDAQg1/lgqdwd4MbyUBjUCR4AmISQXh+fgRBMNMm/Zo2bPMeaW2zwRFL9FkXijvqwhf4qf4BLElixHrkGl0CGmyBg8BDiaGPVlSSwHIGPy6oRm7gr63r6K5oz/YunpvIL4w=",
        categories = "text,development",
        tags = "h5,html,markup,markdown",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Heading5,
    #[cfg(feature = "heading_6")]
    #[strum(props(
        svg = "eJxtjMEKgCAQRH9l2Xula4mB+gddu8cWGHQI6VB/n9IhiS4zMG94dp+OALPDoQVJwaC3TZ68LYEZ9Q+Q9CG8Rt4WiA4JgS+HUqc+U/f59OBCQAKk4IqAKgWq7lK+thsitCou",
        categories = "text,development",
        tags = "h6,html,markup,markdown",
        contributors = "ericfennis"
    ))]
    Heading6,
    #[cfg(feature = "heading")]
    #[strum(props(
        svg = "eJxtySEKACAMAMCvjH1AN0QMcz+w2gXDosH/IzaF1TtZYxvMii0DsRGjSrim8gzHnpyg8s8BT+sVqQ==",
        categories = "text,development",
        tags = "h1,html,markup,markdown",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heading,
    #[cfg(feature = "headphones")]
    #[strum(props(
        svg = "eJxdjrEKgDAMRH8ldA82qVILtbOLHxFwyOgg+X4jOLRyw90Nj7t6ya1wbuFIQLMmYWCILnJn6zp62pe+IxtmKVA+gFaIlkdCkf7IsIGsKbQ6vTfaA11OHfs=",
        categories = "multimedia,connectivity,devices,files,gaming",
        tags = "music,audio,sound",
        contributors = "colebemis,csandman,ericfennis,jguddas"
    ))]
    Headphones,
    #[cfg(feature = "heart_crack")]
    #[strum(props(
        svg = "eJxdTksKAlEMu0qYfeu0fR8HngMewAu4k+fChYIL7495A7oYSpoS0jbtffs8cD9NF1tgqZumRdgKQkLdSFnzmcDAvJUVjtHpqwWzBDRLouRU8gbXmsSHKL5bdhw1d7IGaEXSmceG4VlRr9PaDiPT2n7JXuawEBPjAxfmGvz3fQE/mSkh",
        categories = "emoji",
        tags = "heartbreak,sadness,emotion",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartCrack,
    #[cfg(feature = "heart_handshake")]
    #[strum(props(
        svg = "eJxtT0uqAkEMvErhvvOS9B/mCR7AC7gbxoULBRfi+a0eBPFDOpWmKt/pOt9OOP5v9tZhaTFJPRAKYojixpAl7+gYrqtZ4TcuzKsFGiIkh0TKyeTVXWoKPsjgH8WOJnlhlAimIomy2Ug4V9TDZjv9jZ2202szR0YXTajSy+xiFSvo06Jou+sizfmGFKWNnqL17ASu1OenRlhPYFHtUOq9kC7le/DFGowHBP+lZVh70x7IP0hr",
        categories = "emoji,account,security",
        tags = "agreement,charity,help,deal,terms,emotion,together,handshake",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartHandshake,
    #[cfg(feature = "heart_off")]
    #[strum(props(
        svg = "eJxdTUEKwkAM/ErYe2Izu+lWaAs+wEeUKigU8eCh/t6EikgJk5DJZKZf7o8rrRgSkGhVn4ne37GxY38I0dg/p9eNLkM6aytGWwNBF65cZ1YxRzHOnAXeTWxyUKDxUoJo4SLZwjTs/kw7qS1l0VlFTYAQZ6mdcyikcqQtwPxQC4OKrzjtAkCd2NzEM1ycKSwY0lZXaf3lfgBoKDYz",
        categories = "social,multimedia",
        tags = "unlike,dislike,hate,emotion",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    HeartOff,
    #[cfg(feature = "heart_pulse")]
    #[strum(props(
        svg = "eJxdTjEOAjEM+0p0e0ObNA2VyklsLHyADZXhBgYG/i/cIt1wipxEluO4fZ7fjV6X5Z4qpdwT5xrQCmlQloRhbFeABuKsVLBqh84LxaDEFjIoAWMTwp6DDDLI4VjozNYxWQlSyhxhNgRvJ38sazuNTGvbkyGHUJJbhWT4w+L/zaeBsm3G4vvlDzz7LPI=",
        categories = "medical",
        tags = "heartbeat,pulse,health,medical,blood pressure,cardiac,systole,diastole",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    HeartPulse,
    #[cfg(feature = "heart")]
    #[strum(props(
        svg = "eJxdjjsOAjEMRK8y2t4m8SdhpWUlDsAF6FAoKCgouL+YbIms8VijZ8vb5/F94XlZbnVFjVE1VmFrcHG1SkvNK4WpclRtHH2Q6w1FHJoSjIxJHjLtITZDsb9lw1lz0NVBFKGFxybw7uj3Zd9O86f9B4uwH20=",
        categories = "medical,social,multimedia,emoji,gaming,shapes",
        tags = "like,love,emotion,suit,playing,cards",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Heart,
    #[cfg(feature = "help_circle")]
    #[strum(props(
        svg = "eJxNS0sKgCAUvMrgPntPiRLMG3QIeQUGLUJa1O3TgohhGObnZc2yLZBzVGwUchFSkOuxwbdvH/wej4R5VJPT5OCihQUVMDo9WLAQTFPCyvqr+9+LDbhPmvjrbqWIH0Y=",
        categories = "accessibility,text,shapes,notifications",
        tags = "question mark",
        contributors = "danbovey,colebemis,csandman,ericfennis,danielbayley"
    ))]
    HelpCircle,
    #[cfg(feature = "helping_hand")]
    #[strum(props(
        svg = "eJxNjrEKw0AMQ39FZD819jmXM6SBbhn6EwcdMjTQof9PnYaGItBgoWdNr/Ze8bh2W4YMGCiadrtlZPQhgfRUgy+Sm0KPU7itSTlsllgqjDImY/UmrBVfO9pKjxZzeUau8cDtn230AhkXZ/3hQ0loAff7CPFuni77ynk6t2oUUVDO6AMQlyqM",
        categories = "emoji",
        tags = "agreement,help,proposal,charity,begging,terms",
        contributors = "karsa-mistmere,jguddas"
    ))]
    HelpingHand,
    #[cfg(feature = "hexagon")]
    #[strum(props(
        svg = "eJxNjjEKgDAMRa/y6R41bbEdasEDuLoXHBwcHMTB05s4SMgQeOT9/HK2a8c2ucUzeFxz8/AYdIiJuxQOShQNlS0Icf4RAvJtREiSinJkoHqSZLTv4+Nq6bVEfQEGuB0S",
        categories = "shapes,brands,development",
        tags = "shape,node.js,logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Hexagon,
    #[cfg(feature = "highlighter")]
    #[strum(props(
        svg = "eJxFjDEOgCAQBL+yoT/kDiSSIC/wEyQWFJhYGN8v12A2U81m8l2fhnM3VwIzRcTXt9Q9eVPyorLkeREBCwUbMagCgRtjErvBdVqtKL+AU3VwQJi1D07EGss=",
        categories = "text,design",
        tags = "mark,text",
        contributors = "lscheibel,Andreto,ericfennis"
    ))]
    Highlighter,
    #[cfg(feature = "history")]
    #[strum(props(
        svg = "eJxtjD0KgDAUg68Surf2x2ctPHsCPUTBoYODg/T8viKIg4SEQPjCZ7kq9kVtAc6XhAQLJ05auon0hO3Sk4kjvMQaMKvMQ4czfy5Co0o/i/OIjQ6h3/EG4sUegA==",
        categories = "arrows,time",
        tags = "time,redo,undo,rewind,timeline,version,time machine,backup,rotate,ccw",
        contributors = "ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    History,
    #[cfg(feature = "home")]
    #[strum(props(
        svg = "eJxNjFEKgCAQRK+y+C+1GyKC+t01hIIEUyEJ6vRt4UcMDDPwZmwNbYPFiX0CA0Zqdn0iBgKCkYWS06z+XdItvB3epbe1pCvFvEItMbfDCQNEfIIEqLoTfXwn/QOGnh24",
        categories = "account",
        tags = "house,living",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Home,
    #[cfg(feature = "hop_off")]
    #[strum(props(
        svg = "eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUXndv3yWFmNoZYcvCnp2d5/Xn9/aC3y3LVKpmy5F07VbKAv06mpHqwSrQoVjKblFQkdk6i2sSsLNv64iLb+pCCCrncXgli/RRThdohLoFzzqzcJE+qhxEPh6MUOGEV1glqAbIA4RKkURwxzWUHEg5P+iQR4UZHo8jYzqvNyAyGnALkFzjLJmyS2iJ5tF0l1EhNUjd0VW+I9u1Qxji8lzvncmYQCSv7gK4+XLyD5Lq79dpAxAgmS8oRB6F1KBHGJK3hgzHMW2pReKtWsC/anv6+Pr/f6WaXxWyhv3tW5IVuIwF1aPsHpnF3eQ==",
        categories = "food-beverage",
        tags = "beer,brewery,drink,hop free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    HopOff,
    #[cfg(feature = "hop")]
    #[strum(props(
        svg = "eJxtkb1uBCEMhF/Fut4EA+ZH2lyzdV4gXUSKlCny/srY7N01FJi11t/MAMfv198Pfb/fPqQFJQ16yqBGKaIblIREJqeAH6yE0rHkdj/eDLwfTxwDZBJnIwiMJSAChQniEmAxfkNL9emgEwQJZWupGkeRC1bduqb4oFBBZq/FMrpjNhr73tSZOMXNnijq4sFmkCawtTasPiKj8dDrEjkZ7MasS20jUULrmayms4cyMvDYIFY8kXTSydjZU3G99OWqhS6zdVZr3Ww9gqWpeBLCaJ9cPE4OIzUeQWPnkKQxxpL0z2049dvIlCcH6cWKXl9dKg74epF/cDF/lg==",
        categories = "food-beverage",
        tags = "beer,brewery,drink",
        contributors = "karsa-mistmere"
    ))]
    Hop,
    #[cfg(feature = "hotel")]
    #[strum(props(
        svg = "eJx9kDELwkAMhf9K6J54yeXOE87ODnbt4FZ0uKXgIP39pqByYK9kSMh774MkP6dXgce5GziBXOIkIODWQpsWrhbWpbDUC5RR6wTKrevzYWX2+UueT8ARyGtCEr0zaQzI5NiDp6SfMZB3Cu7Kwdz/kCHBsZhxQzF4U5I9iblNbGppJ2YfkQUjhdkuGUV+njcAW1oJ",
        categories = "buildings,maps,travel",
        tags = "building,hostel,motel,inn",
        contributors = "karsa-mistmere"
    ))]
    Hotel,
    #[cfg(feature = "hourglass")]
    #[strum(props(
        svg = "eJxtzrsKgDAMBdBfubi3mku1Hargrqu74ODg4CB+v6mDipRAIK9D4j4fK5a2GGuQq7iii2XqdfE7yQ/E681pnBXPmSCqFMbWoTFinbhBCOGmG1rgzv2zBw/xNjBMZAZX+/zJSDK+8k2aP6x/Nelyet0L4p82Fg==",
        categories = "time,gaming",
        tags = "timer,time,sandglass",
        contributors = "karsa-mistmere"
    ))]
    Hourglass,
    #[cfg(feature = "ice_cream_2")]
    #[strum(props(
        svg = "eJyVjTEOhDAMBL+yoo/PxrGTk3LU1/ABOgQFDRIF4v2EDkq0Wk0z2i3buC+Yf00vLSRNBkYOLfm3wv9xYiipQOHI8GENEXHJFUEP7Y0kQmRUMlxlSI1TEnDTlc+13pXbx0vfquNkD59Dusknob8sJw==",
        categories = "food-beverage",
        tags = "gelato,food,dessert,dish,restaurant,course,meal",
        contributors = "kemie,jguddas,karsa-mistmere,ericfennis"
    ))]
    IceCream2,
    #[cfg(feature = "ice_cream")]
    #[strum(props(
        svg = "eJx1jDsKgDAQRK8ypDfuxg1JEQN2FnqIgIWNYOH9MfEHFmHZYob3JuzpWLH0anNghmjyYNKdTQwGXcfaC2jiQqgY2qLE8Ipz7t1gYR86pwqUDMw9mF/GX27kk07c3SHz",
        categories = "food-beverage",
        tags = "gelato,food",
        contributors = "karsa-mistmere"
    ))]
    IceCream,
    #[cfg(feature = "image_minus")]
    #[strum(props(
        svg = "eJxNjsEKgzAQRH9l2Htsdos2QpJzD+21d0kFBVuKlBL/vhtEkUDeDjssz3+674BnoLsw2h/bTiCw+tjodK2P2cjjkKF5uFD0p3Ij+ml898gSSISQORA3hEVZK6RAq6UUfRrnNPWYtUtIOVCrWAq0si6j38xeasa1OVfWNeu/O1gjlRMHe2sgvKv8AWhcM8A=",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "eJxtj0ELwjAMhf9K6L21yRrZoBt48+LV+1BhgogHke3fm6RzvUjgNaGvX17z4/68wUK9I3KwoJwO5nWcbRzyTk1Dfo3vCa69O2EMCcF0JCCIIJOn0DYm+kK9Qza4UrAJ7MoaLPx92WYX24bqb4sbebX9UuGfOILoQOWAoevAJFo1wB9Ma0Yt6SZMl8CsoUNk8oFIuoReADX5Bif5Gp+5Mjx5OlbnF2xiS9E=",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "eJxNj0ELwyAMhf9KyN1OM2odqOcdtuvuxRVa6MYoY9R/v6itFEF9L98Lif303xGeDu+kQNGv6wkIJB8l+Hdtj1rQ46CB9diht6fUw9t5eg8QlcMWYSWHRAiRimRX6YQmaEOzecEc2VBTkuxWNExLmAdYMhNWh5wIMT2MlKK3+xKvtEQrzo00utx1XCmoMWRA3jSQqlP/AQt8Pp0=",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "eJwli1EKwyAQRK8y7H9S3dBgQT1BLxGMVKGFIkLi7eMaFmZY5j1bYqhIMX9SdaQN4XS0ENrI0h/u1UYdea9pQN4+xPM25BK+EaFzL0Joo4rQHblHb/9bTdgd/VhDP6dlVma9c2MwlNzEs2ED9V7BWmSR/AWFzimB",
        categories = "photography,text,multimedia,files",
        tags = "picture,photo",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Image,
    #[cfg(feature = "import")]
    #[strum(props(
        svg = "eJxljLEKgDAQQ3/l6H7Yi1U71M4uXd0LDl0EB+n32w7igWRIeCEJV74LHatJAhqrwMQwdBbD25yeRMh1sfvXydO0uQwC2S5uqYr9QHMUmTVg7IteMIq6fgDgbyNR",
        categories = "arrows,files",
        tags = "save",
        contributors = "mittalyashu,ericfennis"
    ))]
    Import,
    #[cfg(feature = "inbox")]
    #[strum(props(
        svg = "eJxVTdsKgzAM/ZXQ92ama1OFWtjbXvYRwgYKRQsTYft604KIJBzCueSEvKRfmuYP5GWa12+vjAGS5YoWyAE1BdtCFE3FcDtSMeRhHeHdq5dD68AhUTVtPBR3U0eukS6ENpvmpO+S0Yxt9zg1YvQM9unR2DOiCX0HJO//tV9a4w70CCtk",
        categories = "account,mail",
        tags = "email",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Inbox,
    #[cfg(feature = "indent")]
    #[strum(props(
        svg = "eJxlzMEJwCAMheFVQhYoiWA9qNv0IIgK7UG3r1otpZ4ege+PTtEX78IBKbpwnQYFKNiBGASQRKu3KazuLrNBIoRCdbkuP5vrzdSCD25I4tsM0xv5p82o8VctzcQ38VguqQ==",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Indent,
    #[cfg(feature = "indian_rupee")]
    #[strum(props(
        svg = "eJxtyzEKgDAMBdCrfLpbG4OxQuwNPITokEVw8P5YHaRDlgT++1+v7TYcS1gFbDSEov0bFW0gu3AKiJHjiOyuiI0dmCvsEkUmJHy/o4RUz19+ACWGKAU=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    IndianRupee,
    #[cfg(feature = "infinity")]
    #[strum(props(
        svg = "eJw1jbEKgDAMRH/lcI+2aagdav/AH+hW6uAiOPj/eBXkSAJ3eUm+23Pi2KbdK7x2UdE5rmJUFGsGg4NnOaSu7CZ+DgEM60WX3gAwFiN+4JMknuMwDEKY1qnkZXwsLxFqGW4=",
        categories = "multimedia",
        tags = "unlimited,forever,loop,maths",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Infinity,
    #[cfg(feature = "info")]
    #[strum(props(
        svg = "eJxVyU0KgCAQQOGrDLOvHIloMXqDDhFTYNAiJCJv7x8Irh68j+Xycp/gDZJCkD9Vp4ZSy1N1y8/+OjgMbqSBlm+YM+bZ0+pGRY0i5iMaTA==",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[cfg(feature = "instagram")]
    #[strum(props(
        svg = "eJwtjG0KAiEQhq/yMgfYdtzN7YcKHaBDRCspRIQIaafP0WV+zMP7ZZJ/ZKRi6Uz4xj0HS2om1PYIwcdnyENJtWeKGM6cpOfM554Ddks31mCelu26YsUMllOTXnDBULp2hH4yIFVnXvHtUZUlPbXxygeUBrx1UoNYOpJ2fxSnLAA=",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "eJxdzLEJQCEMBNBVQhb4JnwLIbqNhSDWur1RUdDq4Hh3klOJ0Njjj1A1yGiSpkNoNOog30BBDmWzrF2E9+S1s7T755q6QztgiiCk",
        categories = "text",
        tags = "oblique,text,format",
        contributors = "colebemis,ericfennis"
    ))]
    Italic,
    #[cfg(feature = "iteration_ccw")]
    #[strum(props(
        svg = "eJwtjFEKgCAYg68yfLfURP4H9QYdIipQEBX0pduXEnvYBt9m69EDLsd2JSDFKbheNN8Ww2moccIsmI7hFIh5u46ht7WkJ8V8o5aYe3NMGkiNcUb4slKT/Sn/AgR/HOI=",
        categories = "arrows,design",
        tags = "arrow,right",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCcw,
    #[cfg(feature = "iteration_cw")]
    #[strum(props(
        svg = "eJwtjEEKgCAURK8yuLfUJP5CXbfpEFFBgqigm26fVqt5DG/G5K1eOCxbNaTYBdeDxjTMnECcCnVG5zd7u2jmzNhnzuQU7uDjiZx8rMUyglJoT82D/MRfcQ99UxvK",
        categories = "arrows,design",
        tags = "arrow,left",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    IterationCw,
    #[cfg(feature = "japanese_yen")]
    #[strum(props(
        svg = "eJxtizEKgDAQBL+ypDe6F+5QOPMCbe0FizQBC/+P2ohFuoGZ8XO/Co45rBRMUTdhHToy6mJI1WAPcUQK2fu3zf4dBmqhtA3/5gZNNhsY",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis"
    ))]
    JapaneseYen,
    #[cfg(feature = "joystick")]
    #[strum(props(
        svg = "eJxtjL0KgDAQg1/luL3oxT+EtrOLq4ObVEHBQURE394WQR0kQ0i+EL1020i94RpCUnQgUBykoFBl30zY8WbvGCX9Fgq7QstWR+HU6uc6J8k8+iECj5ryJW5a3TzQajhhcodhgffTcB42N7UXHxAteg==",
        categories = "gaming,devices",
        tags = "game,console,control stick",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Joystick,
    #[cfg(feature = "kanban_square_dashed")]
    #[strum(props(
        svg = "eJx1jzEKgDAMRa8S3EWTVmug9gYeouDQ0UF6ftNFKyRkCf+Fx0+88l3g3Idjg1DDkOLUkhTfHEmA18AqgBWwgMsEBHObUTblhsEV1KTeAtxZEXQrIXBFHaA3CX9io67UIlR7sQUWAb131LzOquV+jzyUMHME",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,draft,template,boilerplate,code,coding",
        contributors = "danielbayley"
    ))]
    KanbanSquareDashed,
    #[cfg(feature = "kanban_square")]
    #[strum(props(
        svg = "eJxty00KgCAQhuGrDHOB0CIN1Bt0iEhp3IUM/dw+x0W0aDMD78PnSloZzhyZPCqLcHvsEa52S30agVLeiBsH18kguH1hguhxtmAOI13KpytdYfiDscL0wgNdRST9",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding,toolbar,button",
        contributors = "danielbayley"
    ))]
    KanbanSquare,
    #[cfg(feature = "kanban")]
    #[strum(props(
        svg = "eJxty7ENACAIAMFVCAsYjBILZAOHMLGwtDDOrzRUtn95WX1PGBUbQz5EqBIsqThQfMI/KLYklwtKwhXO",
        categories = "charts,development,design",
        tags = "projects,manage,overview,board,tickets,issues,roadmap,plan,intentions,productivity,work,agile,code,coding",
        contributors = "danielbayley"
    ))]
    Kanban,
    #[cfg(feature = "key_round")]
    #[strum(props(
        svg = "eJwljEsKgDAMRK8Ssm9sbawu2t7AC7iTKFRwISKit7dBhvksHhOP+SqwJBxbcMPtxQIFYnCqwrfxxWu0uyM21XOgDtS2Etaw4QlzbPQmR9lO2VeQN2FPHYI8CV3QdSasVcEfyR8OVR52",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock",
        contributors = ""
    ))]
    KeyRound,
    #[cfg(feature = "key_square")]
    #[strum(props(
        svg = "eJxljT0OwjAMha9idbdJ4rRJpNAT0AuwoTBkCBID6vnxq4AF+Vf+5Pfq8/bqdD9Pmw8SKUhqUliKbTOG2tGNWWayMoQAIwc0WCXZkhoDsAHrx88lGymSQT55vIFep7WeYLzWr/3DRzIl0n+0FdPzThYK5POuzZEsOCF63Fm7ooXhJbLVT+INd94zSQ==",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,car key",
        contributors = ""
    ))]
    KeySquare,
    #[cfg(feature = "key")]
    #[strum(props(
        svg = "eJxNjEEKgCAURK8y/L2GikmgnqBLxC8oMAhpUbfPTxAthhmYNxN5q1wW8JUoaE/gO5Hxkmoi8Ry7l8nxmM4Vc6LdGlg16B5NAkjxq2WP9gYHN1qLUJRT7gMfJm4ehQ==",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "eJx9y0EKgCAQBdCrDHOAmpEwF9oNOkRUNO5ChPL2ZUG00d3///FtWOcIp0OFkBx2CCE9Jbzb4ZcodyIEWf0m0SFrHGybf4PdpyiwOBw1GGmIOFMef8RUsa5ipmwGWBV/qoa6gj2wFqZPLvnwTpg=",
        categories = "text,devices,development",
        tags = "layout,spell,settings,mouse",
        contributors = "it-is-not,ericfennis"
    ))]
    Keyboard,
    #[cfg(feature = "lamp_ceiling")]
    #[strum(props(
        svg = "eJxtijEOgCAQBL+yoRc5kCOXnNQ2fsCOxILCwsL4frGhotgtZkbv8lScq9nJw7/RZJ1/lLULRqrkrwWytZ/kGDRiKYG4BAQ4UFu0zHA9/QDhERtm",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampCeiling,
    #[cfg(feature = "lamp_desk")]
    #[strum(props(
        svg = "eJxtjTsOwkAQQ69ipfeS8Wx+0pKahgvQRVCkAIkC5fxMlCZSotHYzbNdvtNvxutafSyjocMhdOjZs6Me1VguKzGWA0ffNBJ+wt2H1KCNzzC9He0JEltaqGdNS5YGCqLmPIWjjrNwLbrt+v8SQytU",
        categories = "furniture",
        tags = "lighting,household,office,desk,home,furniture",
        contributors = "karsa-mistmere,jguddas"
    ))]
    LampDesk,
    #[cfg(feature = "lamp_floor")]
    #[strum(props(
        svg = "eJxty6EOABAUheFXudPNsDHbJSteQLMJgiCY53eVm5RTvvPjantAj6IEMMNNCz7TSl9FQvUwIV+0gXC0/QjFVDNcHpwYWQ==",
        categories = "furniture",
        tags = "lighting,household,floor,home,furniture",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    LampFloor,
    #[cfg(feature = "lamp_wall_down")]
    #[strum(props(
        svg = "eJxtzaEOgCAYBOBXudGZ8oOAG5IpVoKNzUAgGJzPL6R/c+7ifbcLV7krzk3sSkHpapuGS75p6Q4RwzTqGBiZjrIvBMI8IklS8j/SYK3EDt3l5bMzj+WTF6xiIpw=",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallDown,
    #[cfg(feature = "lamp_wall_up")]
    #[strum(props(
        svg = "eJxtjb0KgDAMhF8ldC+a9Heonbv4Am4Fhw4dHCTPb5wCIjfcwX3HlavfA87N7IjgR5wOUsvT2XSYWpa3rUUZD4gcOgHBKkIrqeUfUsAwSEFxYvoMPduoNw/Z/SLa",
        categories = "furniture",
        tags = "lighting,household,wall,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    LampWallUp,
    #[cfg(feature = "lamp")]
    #[strum(props(
        svg = "eJxtjLEOgCAQQ3/lwn7INcRggswM+gNuRAcGBwfD93suupAOTfPaxqvclY7ZrIFQw+lJXPaLhs2kOLw0xa8jIEEbO0QHaIzdsVixE4PAqL6ok1OJOhpy+G8f+w0f5w==",
        categories = "furniture",
        tags = "lighting,household,home,furniture",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Lamp,
    #[cfg(feature = "landmark")]
    #[strum(props(
        svg = "eJx1zUEKgCAQBdCrDF6gZpJqYd4mIggNapG3bxzNKAoXw+e/j2aZ3QgHDYpQQUC+pODg23AkidZUUVmTLHetkkmbCOYl9h8U61LKRnIaPXFB8eqM9Q+W8sL9/cMbr34Jk3ew+tntG3cE/GroQEMXWQb2BG7qRBo=",
        categories = "money,maps,buildings",
        tags = "bank,building,capitol,finance,money",
        contributors = "connium,ericfennis"
    ))]
    Landmark,
    #[cfg(feature = "languages")]
    #[strum(props(
        svg = "eJxtjrENgDAQA1ex0r/Im+RJEbIBQyBRfINEwf4CGihI65Ptq8d6OrY57BkFBgutDk/W6ksSNMHEQBn/eCGyKztgAl07eyRIyaJRMjR2mvefFv9cLstPLVM=",
        categories = "text",
        tags = "translate",
        contributors = "ericfennis,mittalyashu,johnletey"
    ))]
    Languages,
    #[cfg(feature = "laptop_2")]
    #[strum(props(
        svg = "eJwdy0EOgCAMRNGrNHMBBV24AC6jREiMC0Iivb1TVtO8/IaWzy4jYoNoxA756tVLhDsgje4hJde7dBLvpkYpLPaXwlPfLOpoK2S4mQ/P4aqfzNaq9AMO/xw1",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis"
    ))]
    Laptop2,
    #[cfg(feature = "laptop")]
    #[strum(props(
        svg = "eJxVjbEKgCAYhF/laNf8L7WCcnaotV1oaAkaoudPnYobjvs4+KYr3Qf2uVlpIH7rE0GYEkXF6L8bfMZTPEy0tSCaA6idSwKpL1F6zNi62GnPH1YFLzZbmjC1xRtexQka6A==",
        categories = "devices",
        tags = "computer,screen,remote",
        contributors = "ericfennis,csandman"
    ))]
    Laptop,
    #[cfg(feature = "lasso_select")]
    #[strum(props(
        svg = "eJxtT8tqAzEM/BWRu6fWw2sbtoF+QD/CbA97yKHQkkO+PtIuISEssmVkjWZG8+/4X+nn8/RdSWQUKpQ9OEmy03n+iPZ5fgHxhK4L+gRT4nirkaB3dD4YUKgR29eERnE3chLivORkMCFPLTX/SO2PMylKFNRGBXfa0u4IqiQHEoW4DXHOYHZocj9btYfdDmbYNjZZE3If0OLnISPmUnJNnL1h4X837RPesAU51gAXZIWwl5caqMnhGrQPHkbpayzZL0lQqu/WyrtYA9s1Pz3eAUYLWOM=",
        categories = "arrows,design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    LassoSelect,
    #[cfg(feature = "lasso")]
    #[strum(props(
        svg = "eJxtTTEOwkAM+4rVPeGSS0qGoxIP4BEnGBiRYOL15FrUqbIS2bITt1f/PPG4TLczVLvDURJCSjYt7TTspe2hyhVi15kDY9YoFFLuhYwNxk6RkuItBZXn5Ii0PPegXTKt/w4nOehwSHSFrr8zSIZNbbDvfvMDzDwp6A==",
        categories = "design,cursors",
        tags = "select,cursor",
        contributors = "mishkaio,ericfennis,csandman"
    ))]
    Lasso,
    #[cfg(feature = "laugh")]
    #[strum(props(
        svg = "eJxNi0EKgzAQRa/ymb1tJiWhhSQ36AW6k1RQEBFxEW/vTEQIs3gzzHshT1ueB2yR2BDyIbTCUpnC8/qnsPb7iH+kL7/Br97Dw8hw5+HQXJ0b2f601CKFeVoGFBvp8zBMOHQTcEWpEFetxmXXymqxu6PbPgFG+i6S",
        categories = "emoji",
        tags = "emoji,face,happy,good,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Laugh,
    #[cfg(feature = "layers")]
    #[strum(props(
        svg = "eJxtyzEKgDAMheGrhF5AzJIl9jYihZIUdPH29mkUhJIM//A9bV7PzY2aFzv2Jc1MOKEe6EhOWaewWRG12PqtupXgjH410Jg/LzfnH78AQ4cqjA==",
        categories = "design,layout",
        tags = "stack,pages",
        contributors = "colebemis"
    ))]
    Layers,
    #[cfg(feature = "layout_dashboard")]
    #[strum(props(
        svg = "eJxljlsKgCAURLdymQ2E2YNA3UxJ+itCuvtS0LR+7z1nZoTTu6cowUFG29N4iQ3kggQDXfbwRmIFhUQoMSRciSy9zyLOmWNT9ePfek5s/HJNc03ttb6Ct9NS4lKiinUDJoc9PQ==",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "eJyFjkEKgDAMBL8S8gEpFbyk/YwW02sJaH+vxlZbPHhJYLMzhFKYBTjElcXhhJAdWoRd5xYXYU3TGRj0NFx1Two9TTN+q42yp8r57RdBvtcP1jzaGWwVVOoAG789PQ==",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "eJx1jNEJgDAMRFcJWUCjFRFaN3AIscX0T0pA3V5bUCzYn+S4d3c6uEXgNNgiHOmG+xHC7q2wwR6BnV9Zohx1FeOjfkqk/oPZ1NvaZmGwBidSoDjNRSsHQwlQVyJN/SEXa2M7bg==",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "eJxljVEKgCAQRK+yzAVCCuxDvUxJ668sVLevBZcMP4d5bybUvAldETPoLLtwhAdxLgdLhFtB9S0d6FYkhUn5FMxyS2sMM9N3c3+rI9rA96Bp0B7ZMS4g",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "eJxVjEEKgDAMBL8S9gNSFOqh7We0mF5LwPp7NUaslz0sMxNqXoRaxAg6dPeyCke4GVSv34E4l40lwiOF4eZTUMtIr6abetCKT+BvdZD1zf5yTY9XOwHnSy4g",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "eJxtjEkKwCAQBL8y9AeCZAf1M4lkvMpAzO+jEiGIlz40VaWDO4TY+YvFYAWFaKBAj8EIimVvfwqnd4PVQ+atLtb375X7VZKuphrrWnOPz/jSai//FC4i",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[cfg(feature = "layout")]
    #[strum(props(
        svg = "eJw9i9EKgCAMRX9l7Adi9pKg/kxJCtGDCM2/bxvZ07g754SW9w5tRHQIJdez9Ii0IXDEFaGxgWHjqUcvRlNYtEvhqncGJsPDRfQSynEkk3SKqc5n6k8Z06/6GU7zBWB2JpU=",
        categories = "design,layout",
        tags = "window,webpage",
        contributors = "colebemis,ericfennis"
    ))]
    Layout,
    #[cfg(feature = "leaf")]
    #[strum(props(
        svg = "eJw9jbEOAjEMQ3/Fur0hbps2lcpJ6GZ+gA0dAyMD/y/CDSdbyhA/e36e3zde1+VOIuuto0NDxBBHE240MRjYUaU6OJD3iIaq0ON4xCOTqvR4a2LQ+ljWefl3r/NcCIa7pgKKWzIpLTj11LYRE6xiGQyXw/ls+AEmyyMJ",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,energy,plant,autumn",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Leaf,
    #[cfg(feature = "leafy_green")]
    #[strum(props(
        svg = "eJxdUEEOAjEI/ArxvmOhpZRk3cR49hEbPXj04P8j7SbqGkJTOnQYZn6urwfdT4erkMiNITrBm5FAbGK4KWX4FOWq0ErjSBEc740YSiUijajgZtGbNQeqIzcko/qkYK7nPSAJrtR+2rlnjSLpuuuN/y1zjMz2R78xjyFfNV1HaV2T6iWjuBMrilBBSXGvMO6FEDf0bVwqscNKZ09cujbL4Us4c1jmY7dqmXeGERvZB3sD0LZBkw==",
        categories = "food-beverage,emoji",
        tags = "salad,lettuce,vegetable,chard,cabbage,bok choy",
        contributors = "karsa-mistmere"
    ))]
    LeafyGreen,
    #[cfg(feature = "library")]
    #[strum(props(
        svg = "eJxtybEJACEQBMBWFht4Vo7D4N4OvogHAxPBQK5+MdDIdMb6PyrKGxoVCgElZHuWZtv3MUL9OgnJGS8hEKeemPErHVU=",
        categories = "photography,multimedia",
        tags = "book,music,album",
        contributors = "johnletey,csandman,ericfennis"
    ))]
    Library,
    #[cfg(feature = "life_buoy")]
    #[strum(props(
        svg = "eJx9j8sKgCAQRX9lcJ/lA0pQ/yUsKCgIaVF/n44KLsLNncU9XOZot3t3rOAeQxgn4N50fTgDsbpPvdXXfG+wGHJKqgTk4BIjcrGvKCbpJEBRNiLRNbGUrTkcQqz7wYpD/r24BAdZKXw39z0Q",
        categories = "accessibility,medical",
        tags = "preserver,life belt,lifesaver,help,rescue,ship,ring,raft,inflatable,wheel,donut",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    LifeBuoy,
    #[cfg(feature = "ligature")]
    #[strum(props(
        svg = "eJx1yzEKgDAMheGrBPfG5BklQ+0NXN1Fh44O4vltF0Gow4MHP188tyvTMXeLE2T1XQIYpOzByMqURxICOzsN5aNLsa8oxZdOpMjWCGq14PYmgvyhT3kAImAq+Q==",
        categories = "text",
        tags = "text,font,typography,alternates,alternatives",
        contributors = "danielbayley"
    ))]
    Ligature,
    #[cfg(feature = "lightbulb_off")]
    #[strum(props(
        svg = "eJxtTkEKwzAM+4roPVrjNE4KWX+QR4TtkMtgh/2fOYHt0mLL2AhJLu/26Xjel+qVGd5THsyOO2xzMmegNIViHeV2BheXo9yG8ig//UtgtVqfuaoMMLSNmjDH9BruiCMxMdkRDHFCeBFR7anc9YLwlit9+zNfFTMz5w==",
        categories = "photography",
        tags = "lights",
        contributors = "mittalyashu,ericfennis,karsa-mistmere,danielbayley"
    ))]
    LightbulbOff,
    #[cfg(feature = "lightbulb")]
    #[strum(props(
        svg = "eJxtTTsKgDAMvcrDvbGJtlWoggfwEFKHjg7eH9NSOkkI4eX94nO9Gfc2nOzAcyIxDAqGKYDJGSF9G1obkHoncoeHh63jsSQLdQmaACqgUBOmistq0rDHsfTtsbdq8JL9D8EWInnuzAfmLiZs",
        categories = "photography",
        tags = "idea,bright,lights",
        contributors = "ericfennis,danielbayley"
    ))]
    Lightbulb,
    #[cfg(feature = "line_chart")]
    #[strum(props(
        svg = "eJw9irEJACAMwF4p7iJFBYXaDzxCcOgiOIj3WwclWxKabQn0YqoHvzEJJsPkrmV6bWCGbCNEGxQd/3IAMF0Q5A==",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "karsa-mistmere"
    ))]
    LineChart,
    #[cfg(feature = "link_2_off")]
    #[strum(props(
        svg = "eJxFyjEKgDAQBMCvLPcBvcNwColgZ+MjBAUFEQuL5PcmRA1b7e7Ya743LI6mDqyjDgYGdQxDodTbKoHe/owNdJO5sAZtYcd+rvDiiIXg2VFLCJxryHO0SX02nonGTyTbl0qhDz96KPM=",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "eJxNyjEKgDAMBdCrfLKLplCj0BbcXDxEQaGCiIODvb2pgsiHfJI8d8QzYfY09WAZZbCwaDQMgSRDwdWFBPdBtuURX8ganU2qfnRb9wWZPbEhXNqdltG1JWTznNUWFW78fR+z",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    Link2,
    #[cfg(feature = "link")]
    #[strum(props(
        svg = "eJxVjbENwCAMBFd50ZvYAcsNYYMMgZQiBUWK7K8ATUC2rnjp79NT3hvX4U5hSCgKBY8zr7F9DTSlZJ5toJJ429EgLqeta3L6ZREiS00jdRsFrCNsA7WLaLF9iJQicg==",
        categories = "text,account",
        tags = "chain,url",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Link,
    #[cfg(feature = "linkedin")]
    #[strum(props(
        svg = "eJxNjFEKwyAQRK8y7L80LsG0oN6ghxAjXaEfRSRNevpGW0LYj9ndeTP2FapgdnTXBtdgYDDso3c1yyRqXNQUGNy/g2LV9uMCH8wpqcyHvL20Zm9LihWboxvhnecqjkaCpPyQ6kgzYXXEDW+gtzGX+EyIa+fi1qX8kZ/pv0SULd4=",
        categories = "account,social,brands",
        tags = "logo,social media,social",
        contributors = "okcoker,csandman,ericfennis"
    ))]
    Linkedin,
    #[cfg(feature = "list_checks")]
    #[strum(props(
        svg = "eJx9ybENABAQAMBVPnqRR1A8GxhCotBIFPYPui+QK49GmQ1qFN0AetCblVYkUicSsf5sRgOuhXugfk5gswAJAiWx",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = "ericfennis"
    ))]
    ListChecks,
    #[cfg(feature = "list_end")]
    #[strum(props(
        svg = "eJxtzbEKgDAMBNBfOboXexGDQ+zs4kcUHLoIDuL32ywiWG4JeQdnZ7kq9iVsVFDWMWQb/JftK9qHBM5dEUJvpiIQpBbGdtU4/auH76qz5/UHXPgoUg==",
        categories = "multimedia,text",
        tags = "queue,bottom,end,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListEnd,
    #[cfg(feature = "list_filter")]
    #[strum(props(
        svg = "eJxtybENABAQBdBVLhbgIyjObWAIieJKhf0jmqu07/GeR2l1NxIVRXPC/pGwRSVERfgMAqFptrlXJRXR",
        categories = "text",
        tags = "options",
        contributors = "danielbayley"
    ))]
    ListFilter,
    #[cfg(feature = "list_minus")]
    #[strum(props(
        svg = "eJx1zCEKACAMQNGrDLvIFIZhLls8hGBYNHh/ZEWL1v/g8+xLYRTXEAFjTU44WBO+QkA/wPyUaDP1dGgD8Zsc8Q==",
        categories = "multimedia,text",
        tags = "playlist,remove,song,subtract,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListMinus,
    #[cfg(feature = "list_music")]
    #[strum(props(
        svg = "eJxti7EKgDAMRH8ldFebSEOG2NnF1cGt4ODo4P9jSkAcwnEcd4/Tuz0XnEvaCAHLzqnq1LeqH0EZC6A0sujOgOY8ePPFVY7oToC0zhFh4BjYRX7kBYkpKn0=",
        categories = "multimedia",
        tags = "playlist,queue,music,audio,playback",
        contributors = "karsa-mistmere"
    ))]
    ListMusic,
    #[cfg(feature = "list_ordered")]
    #[strum(props(
        svg = "eJxtjc0KAjEMhF8l5B7txFp6aPfsxYcQFSqICIqsb2/iT2FlD2GYmW9IOZ8uRxpRGYFp1MoKpqf5ZKIuQ1k6NJQP+kNModynPjE/hd9h/sKZ//90+Lq7NzpU3kZKDY/ojWeTBqHpTJEIeRP3QUAqare6CQSLtaig8y/3dzuV",
        categories = "text",
        tags = "number,order,queue",
        contributors = "ericfennis,csandman"
    ))]
    ListOrdered,
    #[cfg(feature = "list_plus")]
    #[strum(props(
        svg = "eJx1zSEKACAMQNGrDLvIFIbCNFs8hGAwGsTzi2ll1v/C59X3hJFNQwT0NZjC7rXCIgT0A4y6REiHFPDvMq3QBSuRJC4=",
        categories = "multimedia,text",
        tags = "playlist,add,song,track,new",
        contributors = "ericfennis"
    ))]
    ListPlus,
    #[cfg(feature = "list_restart")]
    #[strum(props(
        svg = "eJx1jL0OgCAMhF+lYQcpLf4kyOygD0F0YHQwPr+FQRdMc23vvuTCma4Mx6w2h9AvpGLoShTDCwZA9wfGJkAnJHnwYOtMmoCNr6qJlq9o12iIxDvjxbImwwgoa0W53OqW3N6cP/YAIHQwew==",
        categories = "multimedia,text",
        tags = "reset,refresh,reload,playlist,replay",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListRestart,
    #[cfg(feature = "list_start")]
    #[strum(props(
        svg = "eJxtzLEKgDAMBNBfCd2LzYklQ+zs4upecOgiOPj/mCyCtIQsebnTuz6NzjXsnImxzaHo5LeiP5GxJMpDAFvkkAoCJZ+IiBaX/vWydnEl249fMZwoMw==",
        categories = "multimedia,text",
        tags = "queue,top,start,next,playlist",
        contributors = "karsa-mistmere"
    ))]
    ListStart,
    #[cfg(feature = "list_todo")]
    #[strum(props(
        svg = "eJx1ykEKgCAQBdCrDLOPMM1cqDfoEJHSuAhCBqrbp21qUXz4i/++zXFm2FNgcqgR8uFQIJSWCBTTQnzvp8MevW3r3dttYoLgcJUgBuhKVKMqV3h4FBI0mW8Q3a+Yl1xJQCz8",
        categories = "text",
        tags = "todo,done,check,tick,complete,tasks,items,pending",
        contributors = ""
    ))]
    ListTodo,
    #[cfg(feature = "list_tree")]
    #[strum(props(
        svg = "eJx1izEOgCAQBL9yoRfZw1wwOaltfITR4koLw/sJHQVku50Z/e7f6D3cxSCwLcllXduZtUdyTgDSuIkkZXsCwcPvxG0Whx5CkalYAccUKro=",
        categories = "files,text,layout",
        tags = "tree,browser",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    ListTree,
    #[cfg(feature = "list_video")]
    #[strum(props(
        svg = "eJxtyzEOQBEQBNCrbPTyg/yNYqk1LqCTKDQShTi/1dAoppk3Qz2PCsWJqDQoHYzw9O3O0xUEfANf7EsaXxh/MJIzJaazWfatHtc=",
        categories = "multimedia",
        tags = "playlist,video,playback",
        contributors = "karsa-mistmere"
    ))]
    ListVideo,
    #[cfg(feature = "list_x")]
    #[strum(props(
        svg = "eJx1ybEKgCAUBdBfubhH3TIpeDm39BGBg4vg4P+jTjroeo7EP3m4R30kuL+HsrJWs9LGwMyC12gCb3BbNPToznLoLwOnqSR+",
        categories = "multimedia,text",
        tags = "playlist,subtract,remove,delete,unqueue",
        contributors = "ericfennis"
    ))]
    ListX,
    #[cfg(feature = "list")]
    #[strum(props(
        svg = "eJxtzsENgCAMBdBVSAdQWxLjAdjGg4nxLNvbFggInH6AR3/dfT2niehhB/OSB0JOPh5gIsltcKuY4FT2hAMpUc65VZTnSaa/o7XLhqVVkYW8WWfrU9NdR0x1U/Bfp+gP3F5Ccw==",
        categories = "text",
        tags = "options",
        contributors = "colebemis,ericfennis"
    ))]
    List,
    #[cfg(feature = "loader_2")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJUMDRKtFSwVDBQADJ1zfSMDC11LfRMzZTsbPRByuwA+mYKxg==",
        categories = "multimedia,layout",
        tags = "load",
        contributors = "mittalyashu,danielbayley,karsa-mistmere,ericfennis"
    ))]
    Loader2,
    #[cfg(feature = "loader")]
    #[strum(props(
        svg = "eJxtUEkOgCAM/ArpA1CrAU2Q33gwMZ7x91paliAnCrN0Bned96HCtMOEoB7cwYAKKNfvGcG7gUjeMVWwLKFzZSm2ZAIXvc1sabU1TOQpJLQR0YZNj1aCyEh7jMZFVsbxH80ILGVS0l6wlkMtQq9FzsmyEq0U6sapEmP9D+JAZtEhyV6iP2Dj",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "eJxtzMEJwCAMheFVQhYopkgpqMuIB0F68KTbV42Rgj0Fed+vSfEJUMiiRqjtKEIoymI7VY2nM0dHzgwqZo4jpdmo+wdrNtII3f7lXsaREq12YR+zTwF8YZYtXgi+CuJ5YxN0fn7YCxz2Qhs=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "eJxlkMEKwjAMhl8l9N64pK5tYPPi2YeQKSiIePCwvb1NM91kBBIavv8Pf7vH/XmFkXvXOpiod8Rlss2xvNkdup1Ch85QZcRVCfNG8w9P1cA0M9OadIPOS/MTY3nR/uDX+X2DS+9OCYlA27HFHCBjEGiBGiQdPDQQMKfSKECqRSig20heFXvM4rkYqLnarswpYyKggBIHJPEYA7J4Ks2mRGi8HvB6wKdaGFPZEoayq0xhF3P7kW+sVbw59RLyA8QtXpY=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "eJx1zUEKwCAMBMCvSD5QDEgpqJ8JHgTpwVP8fTUREaGnJey4+pLfZNgGQDCth+3JGMD1E+WM/hooeqG7QVxIJuxzYN7LiZ3+gz+0zd315NilXKkkQ6w1Nc0a4B5I6/gB2AI3Dw==",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[cfg(feature = "lock")]
    #[strum(props(
        svg = "eJwli0sOgCAQQ6/SzAV0/AQXwA3cuidChJ0hROX2DpombdK+6hz2glwNDYQY0hGLIWZC/SM/3yI+Eu7kS5R+Iau7drT6dCXCG1oVmDflZszoRQyWuKZGNsa+9G4b1Q==",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[cfg(feature = "log_in")]
    #[strum(props(
        svg = "eJxNzEEKgDAMBMCvLLmLJipe2v7ARwgKFUQLiujvXYsHyWGXMIlLwxExeum1RR2bwWCoOMq0U3+Lgi0WjQRXvkfBpW25l3mdkLZ5PXYvStWBj9TA3mX6oeAyvZSsFVzmpRbcDDWm5qTP9gFMOybL",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[cfg(feature = "log_out")]
    #[strum(props(
        svg = "eJxNjLEKhDAQRH9l2F7udrhThCS1ja29oKAgGtBC/941WsgWbwfejIvtNqDzUpegVv+WIL52mjFj88qwPPwkuM9VCS4u0zGNc4+4jPO2etEcWtgIlLC/SOojBZfUXb1QBYdRaeTN3VhefnJPArQmWA==",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[cfg(feature = "lollipop")]
    #[strum(props(
        svg = "eJxFjEsKgDAQQ68yzN5PapEu2t7AQ5RRUFCQ4kJvbz+ghATCC7GyRdkXkscxwBQdGya5S/O2q9jbM1wrzY4PBVJodDtk50UmP58AAoIiRX2RLta1NSblmFQZ0ui7eAHFsiFR",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = ""
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htxdwglh6R/4LX3EoUIHqSI6N+bKLRpKXtYZmfmseExPTMuUc49qFkngtAy1rHj6FeNorOxPYAv2zTArDKEU4UOYUF7mB/dDuX2JHfQNK1fNU66zel+RXpHsV6QPlGogrmsGvrba6y1fyXfpL41ij7R",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "m_square")]
    #[strum(props(
        svg = "eJwdizsKgDAQRK8ybC8SDZIiyQ1s7cUEN2AhYfFz+3yYYuDNPJvjIfgczYRcayK8KQg7Uobwd84xnSydeDs2wdt7F0ZwtBqoZTOXRs2gn35poy97yxj2",
        categories = "transportation,maps,navigation",
        tags = "metro,subway,underground,track,line",
        contributors = "danielbayley"
    ))]
    MSquare,
    #[cfg(feature = "magnet")]
    #[strum(props(
        svg = "eJxtjUEKgDAMBL+yeE81TdoqVF/QTxQ8eFDw4P+xLSIIkjCXnWTjma8N69wdHuxISeFNcFQQcjBhQsNQhsF1Ewus3R9VpqKKz9awoqGqAwlJqh+7Jfa1YolvkcMIhf4kbMvFJ7sBFZIlMA==",
        categories = "design",
        tags = "horseshoe,lock,science,snap",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Magnet,
    #[cfg(feature = "mail_check")]
    #[strum(props(
        svg = "eJxtzLEKwzAMBNBfObJLta6unYCbuUO7djftkKXQIeT7YyckZAgCgbjTS/88DvjemxcJu75DJghXRyh8+OMNTsaPg6lphxpwaJs+XSrSp436FSpKq13ETWMuXY9lVcaE6gLcs3ROXi3AVtqL3/MZZ4cmtA==",
        categories = "mail",
        tags = "email,message,letter,subscribe,delivered,success,read,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailCheck,
    #[cfg(feature = "mail_minus")]
    #[strum(props(
        svg = "eJxljLEKhDAYg18luPe//kFbCz3nG8719nIOXQQH8fltBUWQQCAk+eKS1ozp3YwktPu5RBC2ytDw094zuCn/FioqAbVg7pshvipkiCdqLihvegkenfhUti0Oqxg1FOtgv2XzvI7qoCG7q9kBi08l2w==",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MailMinus,
    #[cfg(feature = "mail_open")]
    #[strum(props(
        svg = "eJxNjbEKAjEQRH9luD7j7prL7UG82kJbC7ughcUJFuL3m4hKWFhmhhlefpTnDdfdcDSlwRkvHLl1OueJDmV6qRSDQeppqGofex/s1BdAD3W0ekhdaIyQ1ZHOw5I3jbnkH/luBpXQgBg5FeUc8XlfAiVBDq30H78BVKQoIA==",
        categories = "mail",
        tags = "email,message,letter,read",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailOpen,
    #[cfg(feature = "mail_plus")]
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l6D1rdqybBmLPHuzVe9BDLoIH6febCKWFloWF4c289Mnfgte1m0jo+WGZIHw7R8dbv83grHx6qKhENMAydGM6NcmYFtW7qoIbJAZcJOTa7fF/TaOO4g3+Xjv76aQRarMdEYPGspIfyCktRQ==",
        categories = "mail",
        tags = "email,message,letter,add,create,new,compose",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailPlus,
    #[cfg(feature = "mail_question")]
    #[strum(props(
        svg = "eJxtjs0KwkAMhF8leN8xme72B2rPHuzV+1IPvQgepM9vUlAEJUxImMlHxkd9rnI7HWZSTFGubaVQNCox8Zy/d+FmXFQMhkHC4GpEOUzjMUjT+ObdndelHkMnBV31eJa9BckSoa3oxTO/p7P1YgXsFzAhoyQ4J1klTEI7wqcWeUETCQ8UxzdhoIk/Q3/QVCE3qH28F1Y8Opo=",
        categories = "mail",
        tags = "email,message,letter,delivery,undelivered",
        contributors = "karsa-mistmere"
    ))]
    MailQuestion,
    #[cfg(feature = "mail_search")]
    #[strum(props(
        svg = "eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb2+74T9GSELIe7+kuYX7gGOrehGwkN27IBCYHFq0bMvfGTKxRAMmphp5IYMnq7pmk0Fd88ZdE87rimoPSz4kdYm5ZBBrIeNgdkmztvZcQTgUKLJ0vuuwTEu4yRy+tnge4+WEsVWFQny2iqvUH3NPomX9/5iIZrI5P5gXgjY/Mg==",
        categories = "mail",
        tags = "email,message,letter,search",
        contributors = "karsa-mistmere"
    ))]
    MailSearch,
    #[cfg(feature = "mail_warning")]
    #[strum(props(
        svg = "eJxtjLEKwzAMBX/lkd2q9LDjBtzMHZq1u2mHLIUOxd9fO5AQSBAIxJ0uffNvxvvWTSRMJTz7TBDaxtHx7vc3WIwvhYnJgAY4GyV0Y7q00pjW3qf2orvKEBEk5qp7LKuVzFG0hz6qc3ydWA1f/Dkhi6ht7A/ZAy6D",
        categories = "mail",
        tags = "email,message,letter,delivery error,exclamation mark",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailWarning,
    #[cfg(feature = "mail_x")]
    #[strum(props(
        svg = "eJxtjLEKwzAMRH/lyC7VurpxDW7mDu3a3bRDlkKGkO+PHAhkCBIC8e5emeo84vfo3iTs+ukrQYQ2QuEzHn9wMX4DTE0zGuCYu6FcmmQou+rvqiR3zQk3TdWzEdtpGhNq6BFenjmpWoJvRDzTmjM5shVquS2S",
        categories = "mail",
        tags = "email,message,letter,remove,delete",
        contributors = "karsa-mistmere,jguddas"
    ))]
    MailX,
    #[cfg(feature = "mail")]
    #[strum(props(
        svg = "eJwli1EKgCAUBK+yvP9MpbJAPUGXkJT0IwgRqtunxcLAwozOYSuIIe2xGBITId+GJOHnY2ggXMmXWD8nq/sWWH26EuENHVJCdTNbFEamnGDLgA+8TnSS8Ql8rU5LW2Rf7YkdAw==",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "eJxdjsEKgzAMhl/lp+fZmdBqBfXixcuuHnaTbTBBVJjI+vZLO8fGCPxJ4CNfyqVf77hW6sQMynsGI5WiRKbW/O4Jd4W2DSOHgUWmLWxL7sI6IAaknaTZ3FnV5THcrctlHv04TDcs8zCtj0qRPRQg9w6iSO6M0J9f4u2mEAuR6CTE3P2/t6VfUZR4FkGq8JSeK3jaV+lZQKPlBVq2OuA=",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "eJxFi0sKwzAQQ68iZu9pZpzGLtg5QXuI4oQ6i0IJpp/b184iQQsJSS+scypYv5GU8FmmkiPJQMjz8silZkuoY21+kXoaw6kBY3jdS8YU6akKZxwLLDufDJ8dWyOsdjOP7jrANa4RB3dT+LdI6iAsfIE2Zen35x8/KCdF",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Mails,
    #[cfg(feature = "map_pin_off")]
    #[strum(props(
        svg = "eJxtjt0KwjAMRl8l9L6xTX+WwhzsAXyIoYKCiBdeuLc3aTcHMgonTfhySP+a3je4HM0pYQygGBldhgpXXwTvzg5kBp4qJipYIjS2ULLJDP1BdUP/k/qCXhYCJhJtVm1eteTEO7EIa2/ZMnTIC9pMF5FpxyziAIqRsGOoaN6i2gBhafX3H0HurKzuHRyxiJbSZrCEPinyln/cn1eY/dGQgZmkSP2stY4lqqHhCx3oTS8=",
        categories = "maps,navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[cfg(feature = "map_pin")]
    #[strum(props(
        svg = "eJwli0EKgDAQA78S9l7crSA9tP2BH/BWVkHBg1QP+nu3SiBDSBKPcq2YE42eIayMwQWI/+x0wVl04ktAAJsEMoAnyrFrzxx1q7ovqIl6gt6JxBsfI7fRX+cX/fwZuQ==",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[cfg(feature = "map")]
    #[strum(props(
        svg = "eJw1jEsKwCAMRK8ScoESRVFQb1OKICq0C719E203mQ9vEnor82oVesv1uSNqsOBBAxk2itjxISeZjRcrFaZwfNMUSq4nDIroEYZaMjlpFk7kBBbmJ6U0uBaiwtrN7seLfQHpVibX",
        categories = "text,maps",
        tags = "location,navigation,travel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Map,
    #[cfg(feature = "martini")]
    #[strum(props(
        svg = "eJxlyTEKwCAQBMCvLPZH2EvhBS7+IB9IF0hhI1iI7xcbG9sZr1/L+O/wGFSzheTHpOQrqCA7uVfhhVMiTKLYu34A7k8XQw==",
        categories = "food-beverage",
        tags = "cocktail,alcohol,beverage,bar,drink,glass",
        contributors = "karsa-mistmere,ericfennis,danielbayley"
    ))]
    Martini,
    #[cfg(feature = "maximize_2")]
    #[strum(props(
        svg = "eJxlzV0KgCAMB/CrjF2gpvkgqLfpQRAV6sHdPj8yil42xn7/zeQUOPi4Q04+nodFUiBB0CganVkmceaH9SMlkPriDpksSoRSmyAEFvXBWufWt+ZfttxLHsuebaHSb0x7ASxpMT8=",
        categories = "arrows,layout,design",
        tags = "fullscreen,arrows,expand",
        contributors = "colebemis,ericfennis"
    ))]
    Maximize2,
    #[cfg(feature = "maximize")]
    #[strum(props(
        svg = "eJxtzKEOgDAMBNBfaeYb6DUsE2Mag8UvQVQiyL6fzQBLlhN34uXilW+jc3V7IN2WDALNLVxXUZfi1ESKr4NQODrIMB5JJfFFP1kbNoLiCWKdZJTf5wORMylt",
        categories = "layout,design",
        tags = "fullscreen,expand,dashed",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Maximize,
    #[cfg(feature = "medal")]
    #[strum(props(
        svg = "eJxtjlEKwkAMRK8S9n+jmba7KbQF/+shShVWqCBFRG9vSsEuKoEEMi+ZaW7DPdGpdcfIEJKKwCFQZCkHEGhvJcRSeDD6kkvT9bApgZAEGSocWCfrBn79WG7RS+BYm5Hrmt1i3jWfCGLnoIqtGfkLXKVYAVVfs/75oBRTth8v8zidaW5d5Wh8tk5g82UzLtAq5wFAog+P5HnL9wYtUkZh",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[cfg(feature = "megaphone_off")]
    #[strum(props(
        svg = "eJxtzMEKwkAMBNBfGfa+sZNs1wrb/QOv3gsKCot4EKl/b1pEL73MkPCS8pieV5zHcDyIZqxhIF/WmIQJ5hFq2S2ulp9Wgr1YOuUW92KESmcbjpQMZhkm878d/Cz2MkRf/3W73S+YdQyqAe9vz/T2cS2nC6of4RkrJA==",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification,disable,silent",
        contributors = "karsa-mistmere"
    ))]
    MegaphoneOff,
    #[cfg(feature = "megaphone")]
    #[strum(props(
        svg = "eJw9yzEOgCAQRNGrTOgXHVcIBXICPQSJhY2JhaHw9K6FlDN5P1/1PrAv7lSQYJLQOK025ib6uJKHT5T8u430EYw+VYVihEUSfBK7u34By7AVog==",
        categories = "multimedia,notifications",
        tags = "advertisement,attention,alert,notification",
        contributors = "mittalyashu,ericfennis"
    ))]
    Megaphone,
    #[cfg(feature = "meh")]
    #[strum(props(
        svg = "eJx1jMEKwCAMQ3+l9AO2VXCsoP5M8SDIDp7072dVdtpOIclLnKQiOYI0j2QQSpcDQeqwwe2zDy6nO0IzPbYIlTxeCI2W1fhUWqnFKsOz4+0gHGOeI/5g3yP7Tz+szizD",
        categories = "emoji",
        tags = "emoji,face,neutral,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Meh,
    #[cfg(feature = "memory_stick")]
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrBPdok2pVqM4urg5uBYeODtLz2zqUCg1Zkv/4EHu7x8O1NLsBmgPqZrVdylabhZRMvUyTSFHomGsdIwpLEmHwrKoyOgYGFYeAkT2ZMgAO1FJO0uh20uNBRQ/jtvXljRxw+Pfw6535hxcV3lbG",
        categories = "devices,development,gaming",
        tags = "ram,random access,technology,computer,chip,circuit,specs,capacity,gigabytes,gb",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MemoryStick,
    #[cfg(feature = "menu_square")]
    #[strum(props(
        svg = "eJx1y0EKgDAMBMCvhHxAW0F7SPoDHyG2mN6kBNTfa/Cih16WZYelmlcFyWUTZXQB4ShJ5a31ZPQITw4Il2Wkzg6R9kUFEuM8QRDXG9j0A+ebMn7lBswnJVI=",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "eJxljD0KQCEMg69SeoH3WkQcqrdxEMRZb+9PUQenkORLJKcSoZJHg1DZI/8IbVjiobw0yDepIIudoVXG3ok+vCS53emrO5PNdmAaIKQ=",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options",
        contributors = "colebemis,ericfennis"
    ))]
    Menu,
    #[cfg(feature = "merge")]
    #[strum(props(
        svg = "eJxlizEOgCAMRa/SsBdpLcJQOYEegsTBhcTBeH7LoAt523v/61XvE47VtQwLCAoYrujUfdGv7sTADwU/174IBiF5SqZ9TrwJMI+3xsE8Rox/ewFPZhxV",
        categories = "development,arrows",
        tags = "combine,join,unite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Merge,
    #[cfg(feature = "message_circle")]
    #[strum(props(
        svg = "eJwBNgDJ/zxwYXRoIGQ9Im0zIDIxIDEuOS01LjdhOC41IDguNSAwIDEgMSAzLjggMy44eiI+PC9wYXRoPmBODLo=",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis,jguddas,karsa-mistmere"
    ))]
    MessageCircle,
    #[cfg(feature = "message_square_dashed")]
    #[strum(props(
        svg = "eJx1jjsOwjAQRK8ySr8ms3YIlkxuQEuPQrENEgXy+Vk30Gyaad782vvxMTyv0y3jfF/2WZiYqihU1HTa2mk4tvbzkciWI3BxwN3zmKFI1SUqUKL2A0B/AG+QkR4nTBhNFXA1CV68VidSULosQTCD2uU//gWc+UPs",
        categories = "account",
        tags = "comment,chat,conversation,draft",
        contributors = "danielbayley"
    ))]
    MessageSquareDashed,
    #[cfg(feature = "message_square_plus")]
    #[strum(props(
        svg = "eJxNzLEKgDAMBNBfObIXzVkRoXV2cXUXFBREHBzUr9dikZIlOV7O7cMxY/TSUaHlQBD5O2rera1WY2H7JAYNZ7VpAN7SuCwUNW5dtgkXvWguONVLLbg0niEugw0qtcVnlRHxe6p++wC0Siez",
        categories = "account",
        tags = "comment,chat,conversation,add,feedback",
        contributors = "danielbayley"
    ))]
    MessageSquarePlus,
    #[cfg(feature = "message_square")]
    #[strum(props(
        svg = "eJxNjCEOwCAQBL+ywV/a21xTQ9E1tfUkCAQCgeL1gCNjJiPG19gy0uM+KvSKBHFOVKa9dxGD/VsGhVltD2B3wR9rFAbHDxGz",
        categories = "account",
        tags = "comment,chat,conversation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    MessageSquare,
    #[cfg(feature = "messages_square")]
    #[strum(props(
        svg = "eJxdjbEOgCAQQ3/lwn7IXU4CCTK7uDq4ER0YHBwM3y8sYkyHNk3zGq50ZzgmtZCATwwMpoqwptmeKCCr7AZJk/atRM6uz6pzGTcVw9BAMXScA5/5tySqRJSM9vuEXJBexAOz9SIC",
        categories = "account",
        tags = "comment,chat,conversation,copy,multiple",
        contributors = "danielbayley"
    ))]
    MessagesSquare,
    #[cfg(feature = "mic_2")]
    #[strum(props(
        svg = "eJwly00KgCAUBOCrDG+f9exHBfUEXUIsKCgIaVG3T20x32Zm7BXuDYujkyV0Y0Q3IDMFKbREpQPn9MLoyswTWJK3bbl6G/cUjxXxdaQIydFIiI8jVmXzt/4D8d8Zlw==",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Mic2,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "eJxtj8EKAjEMRH8l7L1xkxjbQnfBD/DqvaCgIOJBivv3trW6wi6BDEPeQCbcrvczTDx0zB28KGuWZqdqx7Ap0Bge8XmB09AdyKHzQIIse4vEUFdfh/KBk6mxEviLKVCfOFqwX5ZBVzhS8Cjbo0YB+aBGcecMocgK78EnmVnQ8g3nNbOLltRqUqtJ/tfzDfUqQrM=",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "eJxNjDEKgDAMRa/y6V5sUrUItTdwdXArKCiIOEjR29vWoZLhJeT9b09/rZh7MRCDvYaGSiPjFky50UKNzf8v9SScrVLe2dLSgVRgb2CySJJqqCC5uPt2LHioF9QJ3IkcyR+fSM5y0twLk98l4Q==",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,listen,radio,podcast,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Mic,
    #[cfg(feature = "microscope")]
    #[strum(props(
        svg = "eJxtjbEKwzAQQ39FZD8aycZJwM3cpWuGboYON3Yo+f7YBEIGDzecnsTLv/J3fJ/DO4Gzz8OaHy1a8wUCJGePMFZUJkwYwXqjMbqx01xQifpARVBbgybTljzt8Z5Bn55cSFsoPM3NbXwt9x/cwzU8ABDWOw0=",
        categories = "science,medical",
        tags = "medical,education,science,imaging,research",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Microscope,
    #[cfg(feature = "microwave")]
    #[strum(props(
        svg = "eJxtzFsKgCAQheGtDGcDZXQx0HbQIqIkfQsRq92nhlTQ08D8842wana0m8VpiaoEHWGAtDKrdhKsAZ0SNcimMIgigkEklq+6xNr7iCG/48nyB22T07RIjIwT910McfUKLbHeVz8hiE+5AACJNH0=",
        categories = "food-beverage,home",
        tags = "oven,cooker,toaster oven,bake",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Microwave,
    #[cfg(feature = "milestone")]
    #[strum(props(
        svg = "eJxtTKsOgDAM/JVmfkBbRibKNAJ+ALcEMYFAkH4/q2FmOXF3uYc8+S1wre7ACMsWMgHBZPBVKTdfmQryPXsewm7t0yUZbZ+kvRAga+wnrPwHH/jPHto=",
        categories = "arrows,navigation,development,gaming",
        tags = "sign,signpost,direction,version control",
        contributors = "karsa-mistmere"
    ))]
    Milestone,
    #[cfg(feature = "milk_off")]
    #[strum(props(
        svg = "eJxtTssKwkAM/JWw98Zm9g214Af06r2gUKGIBxH792bb2nqQJTPJJJlN8+ifA12OpkuEIZm2ORSlbTY9E17C1tlOvKbgmHLvyFE9Pw4RBIbkkYMPnJPburJ1i4VY6SIn0hgrFiSWjNPuFElq9cEZda9Lq6rZEH7rCq/K/rlT130f2EWaYfnfK5YSKy4elp31JXab8Xa/0oSjAQy9vyzKhqaZdLQMtR+UNETC",
        categories = "food-beverage",
        tags = "lactose free,bottle,beverage,drink,water,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MilkOff,
    #[cfg(feature = "milk")]
    #[strum(props(
        svg = "eJxtjsEKwzAIhl9Fek8WxSQKWWEPsGvvgR1y2GGH0eefXdrSQhF+9Fc/LZ/6bfC6D08BajKM5bY4Y9l9BZrJZ9HKwBAs0PmUCcgT6tvymLwKP7Z2gAwYrEkThWpzq2tZS8fa0ez0RA5/sjuQ3QmNgBHYVmSii1ftbqzJsz3Xte9E06Xu0o+btwN+z7Y4xQ==",
        categories = "food-beverage",
        tags = "lactose,bottle,beverage,drink,water,diet",
        contributors = "karsa-mistmere"
    ))]
    Milk,
    #[cfg(feature = "minimize_2")]
    #[strum(props(
        svg = "eJxly0sKwCAMBNCrhFygSepSvU0XgqjQLurtmwb7oxBmCHnxreaeU1mg1VS2NaAD1qGRQhj9dKHof1zowZruy412CTgj7FrCCJ0DMul+tvm35YHEjobGL9NtD8J0Me0=",
        categories = "arrows,layout,design",
        tags = "exit fullscreen,arrows,close,shrink",
        contributors = "colebemis,ericfennis"
    ))]
    Minimize2,
    #[cfg(feature = "minimize")]
    #[strum(props(
        svg = "eJxtyqEOgDAMhOFXaeYb6DVZJsY0BotfgqhEkD0/xQBLlhP3iy+f9TI6lrAl0qYVBJp9wl6rhpKnR5T8Oggl404y9pFUkmg/6I82ghIJ0rijDPvoDZPFKW0=",
        categories = "layout,design",
        tags = "exit fullscreen,close,shrink",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Minimize,
    #[cfg(feature = "minus_circle")]
    #[strum(props(
        svg = "eJwlybsJACAMANFVQhbwU1nEbOAQEgUFCxEL3V7F6uAeSR3SMsj2aCyCrN9xo5FJfWfqcRZIHoMDY4t79BYfYIgSZQ==",
        categories = "maths,shapes",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    MinusCircle,
    #[cfg(feature = "minus_square")]
    #[strum(props(
        svg = "eJwly0EKgDAMRNGrhLmAtG66SHIDDyG2mO6kBNTba3Q18IbPo21O1vpuLkgFdAtm0Nmr2w/XB+OdDOUpAuVjdaMqWAqlbCWOIH0ApJgWrQ==",
        categories = "maths,development,text,tools,devices,shapes",
        tags = "subtract,remove,decrease,reduce,calculator,button,keyboard,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    MinusSquare,
    #[cfg(feature = "minus")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVUwNMowNFGys9EHidkBAGN3B1U=",
        categories = "maths,development,text,tools,shapes",
        tags = "subtract,remove,decrease,decrement,reduce,negative,calculate,line,divider,separator,horizontal rule,hr,html,markup,markdown,---,toolbar,operator,code,coding,minimum,downgrade",
        contributors = "colebemis,ericfennis"
    ))]
    Minus,
    #[cfg(feature = "monitor_check")]
    #[strum(props(
        svg = "eJxNjEsKgDAQQ68SZi86taBC2xt4CFFxXAhSip/b2yIWCQTCI8/sQxBMlrYOXEHF6EKTM2UCzvh5DJB5XSRYYk3wlyVFePu2VBPOdQoSd5Vu6eDMZ+1ZgZvjJ8ykhWJpM3gAH6gmBw==",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "eJxtjL0KgDAMBl8lZBdN/IfW2cXVXapQwUGKFH17UwTtIBnyHQenzOrMtoDTmCOYUyO18i+NFXYqfWyn9umwMGscmIHY5xMDQyZHiay+iDnhsfwYhG0bWqERlUhKtS9+TANMtnnFDWxtKh4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "eJxljb0OQEAQhF9lsr2wd4Tizhto9YJYhUTk4uftOcIVss3OfJkZMzdO0FmqWIF1nVNpYu+V5iUTZ+Ak0tDRdYEvfesg/TiIs8QpYbekCIclTVgesY2dk+tLfMwHQu09mK/pf7AqoFiKD5yP4S2R",
        categories = "connectivity,devices",
        tags = "tv,screen,display,desktop,download",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorDown,
    #[cfg(feature = "monitor_off")]
    #[strum(props(
        svg = "eJxtjDEOgCAMRa/SuKO0oQETZGZxZSc6sJg4GM9vu7BI2g7N++/Huz4Nzm3a0QP67CoBgZVBQ4YKH9bgzPrpTSkuaqTYPRLAhbtn1cvrIBmAsIUBQKnwr/uTS0vJynb2ATplLWw=",
        categories = "connectivity,devices",
        tags = "share",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    MonitorOff,
    #[cfg(feature = "monitor_pause")]
    #[strum(props(
        svg = "eJxtjU0OQEAUg6/S9AK8MQmLGTewtRfEsxOZ+Lk9Y4GIVdN+aeumJig6z0pSSFbnLF0Ss9I9xH7I3LcB2o+DBk+xxDp2QT1NSmynELtnRsyXOWux8B40kHyxP1cFjGhxgwPa+iyL",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "eJxFjDsKgDAQRK8ybC+6UdEiyQ28gJ2ouBaChODn9iaIsZhi5jFP74MXTIY2LtCgRpmF9GR1HonVbh49LkOKcBsqCe4tMq+LeENcEc518hLWIt7iwepP27ECN0f1CxNpoVjaBB49myZj",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "eJxtjLsOgzAMRX/lyntoYtoQpCRzl67dUUE1W4Ui2v49zgQD8mAf30f8DEUwJnq4gPD0A4Nh6xg2fL8eGbx2O+tmCZTjpVbkuBdZuH41bdN7tI27nVg6dchBWKZXwS+R84TvPBZJpJdM81uKfi1hUZUJfyWuuZrIGx7EMGQ=",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "eJxNTcsKwyAQ/JVh701V1KSgnnPpRwQjVeihiPTx910p1LCH2Z3ZmXGPrWXsnq5mMlBiXSi4cyeD+0tyxiVPQg6pptjwKnvLnqQg1LcnRfh40oScyi03FiyBeam6rzsOkQvsqjcFBdHnxNvTjptRZT36YqnxnhB73EyIXCQN1zL0p58cvmUQNsw=",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "eJxNjMsJgDAQRFtZpgFNFD+Q2IFFiAY3NwnBT/e6itHb7rw3Y4IbIx0WNWi3aEGbnyJbVCB2fuYoZ2cy8Tpz25enIZ0CFJ7nVVWZBnT+1ZYhMk0WvdKk6rUUItmPNKQVNwmcANErbg==",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "eJxtzc0KQEAUBeBXOd29uDMKNeMNbO2FXAslTX7e3kxCSXdz6+ucY+bGCTpLUwFOoCMNf1SaOEBpbq5YgXWdvbL0rcNuSRGkHwdxljglHJY0YblgGzsn/ktCLAQ+hdma/kzlUCz5AydMay1K",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "eJxlzUsKgDAMBNCrDNn7Sa2o0HoDDyEqxoUgUvzcXouiiGQT8piMmWonaC2NrMMUrMI0OIdKE3kpzePFzfjw3DUO0g29OEusCZslRViH1sm5xYT5uuyWEh/zgfdrxQqcLfrfV+VQLPkDB0CPLo4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "eJxFy0EOgCAMBMCvNPsBbTXqAfyMEiExHgyJ8HsLRj213c6a0y2RskUH8i5sPlpwDzqThYCusEavWwuqwWyaUpjNHg5HSRQP+mKLCZT1FNbJdaot6rUasvyoduXBPH74BgmaJas=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,virtual machine,vm",
        contributors = "colebemis,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    Monitor,
    #[cfg(feature = "moon_star")]
    #[strum(props(
        svg = "eJxtizEKwCAQBL+y2B/xNBEOLv4gH0gnpLBMIb5fFMFGppwZ/VPJ+G7zsINPAQF2IBMLBpOQvCbq0euo6xH4em6EY1yZlmkXJhpl",
        categories = "accessibility,weather",
        tags = "dark,night,star",
        contributors = "karsa-mistmere"
    ))]
    MoonStar,
    #[cfg(feature = "moon")]
    #[strum(props(
        svg = "eJwBNADL/zxwYXRoIGQ9Ik0xMiAzYTYgNiAwIDAgMCA5IDkgOSA5IDAgMSAxLTktOVoiPjwvcGF0aD481Avf",
        categories = "accessibility",
        tags = "dark,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Moon,
    #[cfg(feature = "more_horizontal")]
    #[strum(props(
        svg = "eJyNyrEJACAMBdFVQhYQBQshZpmPhWCVSrdXSKmF1RX3BN0wGmFVjokJ02snrBJ8q7xY+WL5UhuPhCD4",
        categories = "layout,development",
        tags = "ellipsis,menu,options,operator,code,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[cfg(feature = "more_vertical")]
    #[strum(props(
        svg = "eJxtjDEOABAMRa8iLiAkBkm5TGOQmEzcXrWNhekN7/0P2Ab2aka23hpchECczAJOdAHNThCvl9WnUs136ck2kK8g+A==",
        categories = "layout",
        tags = "ellipsis,menu,options",
        contributors = "colebemis"
    ))]
    MoreVertical,
    #[cfg(feature = "mountain_snow")]
    #[strum(props(
        svg = "eJw9zLEOgzAQA9Bfsdhxm8sduUgpc4f2I6oysCAxMPH1HANslv3ktv62GdOrWxwZCof1BkOyt3yi2ruxPU4ztkt+lUkD8Ol/4SB9ohUYRSNpRqEPVIGwBGPV2LRC4JTMVO/HAy6pHFg=",
        categories = "nature",
        tags = "alpine,climb,snow",
        contributors = "kerkeslager,ericfennis"
    ))]
    MountainSnow,
    #[cfg(feature = "mountain")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKtVAwVjBRsFAw1TVVMFUwNPUw8gEKVSnZ2eiD1NgBAOC1CoQ=",
        categories = "nature,gaming",
        tags = "climb,hike,rock",
        contributors = "kerkeslager,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Mountain,
    #[cfg(feature = "mouse_pointer_2")]
    #[strum(props(
        svg = "eJwBMgDN/zxwYXRoIGQ9Im00IDQgNy4wNyAxNyAyLjUxLTcuMzlMMjEgMTEuMDd6Ij48L3BhdGg+MmsMEA==",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ericfennis,csandman"
    ))]
    MousePointer2,
    #[cfg(feature = "mouse_pointer_click")]
    #[strum(props(
        svg = "eJxtTjEOwzAI/ArKbhqwASO5eUHziEodOjRSh059fbEtZepyoLvjuPa+f57wuC6Hg4MAMRCalSTIrDcmoBKCf5etXbp3a+cFKa4WhjkKcskT/3gNqVZg5OwRb7FVt12QsoKhq7xSp2rq6k4ZXSJrHTQxw8AjWqkYdNQhTP789wOqoS9m",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "mittalyashu,ericfennis"
    ))]
    MousePointerClick,
    #[cfg(feature = "mouse_pointer_square_dashed")]
    #[strum(props(
        svg = "eJx1TjkOg0AM/MqIfklmdkmEtOEF4QN0SBTbIFEg3o9pgMLILqw5nZdxLZh+Vd8gjoLwPibYVXX5dbBdPjVsbyLCFc0UbBNokvobUh3/MugzOIENxCswKHitVlro4mJx30wPjoh283DRCLkGpptjB6mwUE4=",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = ""
    ))]
    MousePointerSquareDashed,
    #[cfg(feature = "mouse_pointer_square")]
    #[strum(props(
        svg = "eJxVjDEKgDAUQ68Surf2x1qX2tlBVwe3gkMXwUE8v+2gIgmEhPDCkc6MbVAzBSJLlwjCVmlqjr8OXuK+oSSzVzE0FRLDg9qFKHYQCzG9dqadWCa/vt8bV2sbew==",
        categories = "arrows,cursors,development,tools",
        tags = "inspector,element,mouse,click,pointer,box,browser,selector,target,dom,node",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    MousePointerSquare,
    #[cfg(feature = "mouse_pointer")]
    #[strum(props(
        svg = "eJw9i0EKgDAMBL+y9N5oDCYUYl/gJwQPXgQPnvr6NpfCsrDsjH/X/+A+0isQGK0GViqGjXbORlIQlWOeA2mp+hJO9WmyYESh8+t7GRUe",
        categories = "arrows,cursors",
        tags = "arrow,cursor,click",
        contributors = "ashygee,ericfennis"
    ))]
    MousePointer,
    #[cfg(feature = "mouse")]
    #[strum(props(
        svg = "eJwVy8sNgCAQRdFWXl4DKkHdMHRgEUaIw86QiZ/uhe25uaHmw6C5nGpCNxKvcCaekkyFkye+5kRtvjKGoQ8xXLspknCbHJbb99Ap/qNUFrA=",
        categories = "devices",
        tags = "device,scroll,click",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Mouse,
    #[cfg(feature = "move_3_d")]
    #[strum(props(
        svg = "eJxtyTEKgDAQRNGrDOkXGZcMCmtu4CEEizSChXh+TSFYhN/9F+d2VexLWjP8piqVSgztlvjsyOAMWY9GCG6Ot45yAtXM/v4Acckfcw==",
        categories = "design",
        tags = "arrows,axis,gizmo,coordinates,transform,translate",
        contributors = "lscheibel,ericfennis"
    ))]
    Move3D,
    #[cfg(feature = "move_diagonal_2")]
    #[strum(props(
        svg = "eJxlzEEKgDAMBMCvLPmAROlBiP2Nh0JpC3pofm8b60F6SXZhWCk5agzpRMkh3ddBDsxweB95WT7hZbK8gzf0O8Lfm9W1O4Jy2ybUUavVxo0+yCsmmA==",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "eJxlykEKwCAMRNGrhFygjOJCUG/ThSAqtIvm9g0pLRQ3M3x4aY4mrfad5qj9PDLDUyDEZwAuaXtNSavGZ3X8X5sUZA5Mlx6ivrMUZ6nc6A2/zSaY",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal,
    #[cfg(feature = "move_down_left")]
    #[strum(props(
        svg = "eJxFyiEKACAMRuGrDC8gP7IwmMsGrXbBYDR4f9Si9X1PZ1uDenQFIEjiiuBM/c2mH4U48xmebcWDD4w=",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "siarie,ericfennis,karsa-mistmere,jonas-hoebenreich"
    ))]
    MoveDownLeft,
    #[cfg(feature = "move_down_right")]
    #[strum(props(
        svg = "eJw9yiEOACAIQNGrMC7gGCOwIdmg1e5mMBq8/xyF+t+3u96BXXGQAvEkbcToVqK7pQpIj0PTPtMbD8M=",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDownRight,
    #[cfg(feature = "move_down")]
    #[strum(props(
        svg = "eJw9yqENACAMBdFVGhYg/YIgSicoFk+CQCLYP1BTd8k7OfNuWi31SlyNQYBx+Z1UsqNKLK4DCHnwMxA+",
        categories = "arrows",
        tags = "arrow,direction,downwards,south",
        contributors = "jonas-hoebenreich"
    ))]
    MoveDown,
    #[cfg(feature = "move_horizontal")]
    #[strum(props(
        svg = "eJxlzEEKwCAMBMCvhHygJAfJIfqbHgRRoT3o75sEehIWloVhdY62W+03zFH7+2QkAQFmIIsAJSx6/ajowZNrx+mw4RZlZITN9my9rNk3xTYf9gO9OyaG",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis,csandman"
    ))]
    MoveHorizontal,
    #[cfg(feature = "move_left")]
    #[strum(props(
        svg = "eJw9ySEOACAIBdCrMC/g+IEZkGzAQ7gZjAbvP0chP73rHdq9TKHmIIYLsRTTGmKaHjaAlA/Icg+Y",
        categories = "arrows",
        tags = "arrow,direction,back,west",
        contributors = "jonas-hoebenreich"
    ))]
    MoveLeft,
    #[cfg(feature = "move_right")]
    #[strum(props(
        svg = "eJw9yqENACAMBdFVGhYg/YIgSjWiDEGCQCLYP1BTd8k7OfNuWi0NrlQNIIb95JJUsqNKLI4dCHnvjhAw",
        categories = "arrows",
        tags = "arrow,direction,trend flat,east",
        contributors = "jonas-hoebenreich"
    ))]
    MoveRight,
    #[cfg(feature = "move_up_left")]
    #[strum(props(
        svg = "eJxNySEKACAMBdCrjF1AflgYzGWDVrtgMBq8P2KR1fdsj7NoZm5CQJcCsFt66hZOKpSg/y64nA9V",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpLeft,
    #[cfg(feature = "move_up_right")]
    #[strum(props(
        svg = "eJxFySEKACAMBdCrjF1APrIwmMsGrXbBYDR4f8Si9T1bfU8aiSsiSYY2gN3CZbefSlKEoO8OxaYPjA==",
        categories = "arrows",
        tags = "arrow,direction",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUpRight,
    #[cfg(feature = "move_up")]
    #[strum(props(
        svg = "eJw9ySEOACAIBdCrMC/g+IEZkBNgtbsZjAbvP0chP73rHdq9jEbiDIKzkBTTGmKaHjaBlA/JSg+m",
        categories = "arrows",
        tags = "arrow,direction,upwards,north",
        contributors = "jonas-hoebenreich"
    ))]
    MoveUp,
    #[cfg(feature = "move_vertical")]
    #[strum(props(
        svg = "eJxtyjEKwCAMheGrhFyg5A3SIXqbDoKo0A56+8bQDoVC4M+DT3srs+R6UG+5XmfkncQOBJAE+znp9qKkPzy4Xjh8rbuByAKmaYV1yLOtWN7tDbzsJoY=",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "eJyFzjEKgDAMBdCrfHIBSaBDoHobB0FU0KG9vUlpEerQ6VP6fpJ4nXvet2PFdW7Hc88UoBCwIIADLXFqYok/q47EecDIGmEtWmwD64ir31EO8Vo3vdAkM4kQElsSsj3Zk0ua72z7rBWuHfnsC8nNTQs=",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "eJwBRAC7/zxjaXJjbGUgY3g9IjgiIHI9IjQiIGN5PSIxOCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+nsgTIw==",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "eJwBQQC+/zxjaXJjbGUgY3k9IjE4IiByPSI0IiBjeD0iMTIiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+YwwSWw==",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "eJxty0EKgCAQQNGrDLOPGKNQcLxB2/YxBQUGIRF2+7QIN63/+3YfjwUmxt4A6aH1pCp1UoPO1jk5+4HNQCKpliRrED+DRMYOQS5G0giB8dnfWlTOmcVfdgNXWSaD",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "eJxVyzEKgDAQBdGrLL8X2QQlQjY3sLWXVVBIIUEk3l6ChaaeN/6Yz40WwTgQu6mLbBpzsUXwbUnB6540rqS3gB0oCSxIs6Av5q2fypUqz589u+QfIA==",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "eJxVy0EKgCAQheGrPNxrzGhooN6gQwQFBRItWujtU4mgzfth+MZfy71jDWKelCb0GcGUrDSwMJIUWanriOiHhqP/XsioUcMp50AMTh3jj9NxbsgUBAsUrqnNb0s/V9pQfAAW0SGg",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Navigation2Off,
    #[cfg(feature = "navigation_2")]
    #[strum(props(
        svg = "eJwtyrEJACAMBMBVHheQBEQEdR0RJAlo4/ZapLviqum6QwWmU85ugRgMKmDCJ2UkJ4deo+/+ALgSDuw=",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation2,
    #[cfg(feature = "navigation_off")]
    #[strum(props(
        svg = "eJxFi0sKgDAMRK8Sum8k/dgW2t7AQwgKCkVcuLC3N1VRAvPC8Cbu47HAlMTg0Wi4QwNR8aD4ONE6abkWOXZNzvGbkEMdWEanQbFeZGivwZ5+u6zbDJWSUALOF4rBrA9ZbVK+AC0sIcE=",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NavigationOff,
    #[cfg(feature = "navigation")]
    #[strum(props(
        svg = "eJwtisEJACAMxFY5XEBav9p1RJC2oB+314K/JKS6zdNN4TZ0r5YKiMAMBhUwhT2ImqTmf8sFtm4O3w==",
        categories = "navigation,maps",
        tags = "location,travel",
        contributors = "colebemis"
    ))]
    Navigation,
    #[cfg(feature = "network")]
    #[strum(props(
        svg = "eJxljM0Kg0AMhF8l5C7tRCot7PoGvfYuVRpvRRZ/3t6swroogQzJzDdu6L6Bpr4N6rliGmbPYFps2zXvol3/0xD92t0iULsTdiSuDXKisqy5ry0qiUudCfo3Qan1/H4QqrEoGxDobmNaQCH5gzCWEY1QhkII8nkmZwVwQ0MD",
        categories = "development",
        tags = "tree",
        contributors = "ericfennis,johnletey,csandman,karsa-mistmere"
    ))]
    Network,
    #[cfg(feature = "newspaper")]
    #[strum(props(
        svg = "eJxtja0OhTAMhV+lwZe7noylJLtoDBaBIyBqSBBkzw9DwASp6Xd+cuI+H0brvxo8ASZhBoHcfWCM/mW+uNeSCakoSBam7XpLiZG4XRxLLXWbEwxD1cVfHu7iMy9K4o31y2pI1Lj5shwF05SLY5iewAllezQR",
        categories = "multimedia,communication",
        tags = "news,feed,home,magazine,article,headline",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    Newspaper,
    #[cfg(feature = "nfc")]
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8SPMDYpDVVqL2Bhyi4cOnC+2M/tKsS8sjAPBLe9D10n8ultMNK8nCWKkwezuthdYlhLc0YRv+AU1IIJ2Z4pcYu5bTtE4sFB5NDtrZyNTYLpuapprCeJIkBKzX2V2KG8ANXli9x",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Nfc,
    #[cfg(feature = "nut_off")]
    #[strum(props(
        svg = "eJxtUUFOAzEM/Eq095jYseNYavsCuHKvChJIBXFAqPye8S6qeqiidRx7djyZ7L6O32/lZb88sRR9luWwe8jSYXdtWOH2o0en1rRssa3LSNzLIJ9+ImUhbkqzCUlIYZyELOQRxCJnGsI1w4l6pU4jKumcG64mMCdwWcPGr8RiVSh63JHFkbo6de33ulAgp8poY5B5RxrNEFtLTnwo60BRsqippFvPVlkrPCFXFamOPE/8sTZkkjOSERQA9cIl23ylcXPQtA6eXohDK7kPmslllmgPAIQ/GCymMDICFs7p8MQcAMHwGQk1y+mwOId0mMED2Zg9nQMWO/oSnuP7irW5jodaOJ0XVaxZUwaebr0QsCNRbNB38+jn98/XcpH9IrKUC2Nfyu//tlUBTdDhD7JEeZM=",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    NutOff,
    #[cfg(feature = "nut")]
    #[strum(props(
        svg = "eJxtUD9PRDEI/yrkdrBQoCV5XmJcdXVwuzwHBwcHc59f6EvedGlCaeH3B7bf2983fD1f3llAP+Ry3Z7q67qdBQNud70Nak3hiG0dIxkDnMYcOykLcVOaTUhCgPMlZCFvSSzyQy6MFXbqSJ08kHTOow+r8aW4OxzxUOAA1jvqA1dl93WCgpKB5z13JNFOMZCigBQmKNAz6yndZpmKkWIS0JGBTCmEYhW6OaTRzDRh5DoWgyJTiwk5SnKlTaMIK9fG1SujSgvXj/kymy4FsxPmEqlcjOxLQvoJk5abMK1N+EzrnoxYhdbTG/v670tx1DDJLDlj1T3jsImSOQrmXlMNNc9E/TyX9g9MgGML",
        categories = "food-beverage",
        tags = "hazelnut,acorn,food,diet",
        contributors = "karsa-mistmere"
    ))]
    Nut,
    #[cfg(feature = "octagon")]
    #[strum(props(
        svg = "eJxFzE0KABAUBOCrvByAvIQFriMlT7Fxe/KT1dQ307hKeUQqUCmV3jwz3GpAkJpLtRIRjjy6xee/3XKYBSfuc5i5PBcj",
        categories = "shapes",
        tags = "stop,shape",
        contributors = "colebemis"
    ))]
    Octagon,
    #[cfg(feature = "option")]
    #[strum(props(
        svg = "eJw9ySEOACAIAMCvMD7gGA4NyA98hJuBYDD4/6mFfLrHcZgNOwO7LAGqLmiaPpgGU35eAi7D/Q/j",
        categories = "development",
        tags = "keyboard,key,mac,alt,button",
        contributors = "mittalyashu,danielbayley,karsa-mistmere"
    ))]
    Option,
    #[cfg(feature = "orbit")]
    #[strum(props(
        svg = "eJxtjEEKhDAMRa8Ssjfjr+0wgdYbzCGkM6DgQsSF3l5tXRSRZJP8976PwxzHP8UtMAzTHLhhimu6Wv/KcesLzF25JvqJSu9cqRl2JTV1S0+/wF/UYslAtENNx+ZRUYsKTizep3bipdSIIyMonUrlY0E3Zwfytjj/",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "outdent")]
    #[strum(props(
        svg = "eJxtzEEKgCAQBdCrDHOBmAnUhXqbFoKoUAu9fWoaEa4+H97/OkVfvAsHpOjCdRqUoGAHYpBAAq3eprC6u8wGiRAyGeSapXWuST3r4IsHmqOGxLMRf7r6VeNXvfgG8iYurQ==",
        categories = "text,development",
        tags = "text,tab",
        contributors = "Themistoklis,ericfennis"
    ))]
    Outdent,
    #[cfg(feature = "package_2")]
    #[strum(props(
        svg = "eJxljLsKhTAQRH9lsM9ed5NcXYiCnYW2FnaCRRrBQvx+Y+EDZFmYYeZMWKctYq6y3kIjlzvnk0CQp2OTVOvf3sigY1aH34nV4YKXBEPIeeNIm7uPgsTBRiUvzwqYyvTcCUO/Wz0L7P6/gwMIIyZR",
        categories = "files,development",
        tags = "box,container,archive,zip",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Package2,
    #[cfg(feature = "package_check")]
    #[strum(props(
        svg = "eJxtjd0KgzAMhV/l0Pt2JtZVoRX2ALvdvbDBBP9gMvTtlzo3eiEJhH75TuqnZn7iHlRPZ0izlNVW1f4UN7X/7a9MoOxWNtHIYmnSZFzeaadtQmUKgr38EXKU7yQIuRSDIiUw5lgu0sHnvTMFrGGHCoWhIjHGbu3a4YFpbIf5FVRuuIIDcWzOjCO4Td/F2m/6wkERK6wUFMtcaH9/uQQ2+QO7nUVH",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[cfg(feature = "package_minus")]
    #[strum(props(
        svg = "eJxljUEKgzAQRa/yyT6pM0ajkAg9QLfdCy0oWBUqpd6+EysSkAQ+eXl/xs/t0uER1I1KUNmVqvGXyBp//DCBsnvVMhhZPJo0GZcP2mmbUElBsNcDIUf1SYqQSbEoUgJjj2Ui2fPylzMFrGGHGoWhIjGmYR368Yl56sflHVRuuIYDcbycGUdwm76Ljd/0lYJiVlg5KJL80p7/txQ2+Qe7IkRs",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[cfg(feature = "package_open")]
    #[strum(props(
        svg = "eJxtkE9LBDEMxb9K6L2xSaZ/BmYWvK9XD94GFBRG8SCyfntfOu4Ks0shpX15yS+ZPpevV3qew4MmHoUatwEhF1JWW4RHox6SnyjchNLRWGhgsUVZlHrYdE6ZjMu4CuxKBc4FCZvqhQa4VQhylhvuMcJdn8JhunOww3TGezeIGx28lmPhIjs8lO14K2pk/941EAK20x0bV/gGrvlCJ/GPzm7CwXsNt759vNBJ5yAa6Mdvwy1zULxP0v+R7Gn/k2DRJMb529gquiRfNcIZQvoYtsYC1tT2MyKjNswYS3T5fleAsJ/CrT56iwvpL0kmZmc=",
        categories = "development",
        tags = "box,container,unpack,open",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "eJxtjUEKgzAQRa/yyT6pM4lGIRF6gG67F1pQsCpUpN6+iZY2UEngkzfvT9zUzC1uXlyoABVtIWp3iqx2v0kF0svRhAmUXcuGwcjikSRJWd1LK01CQwYEc/4iaJRLUkTYFItBSmDscdhI5v/zh1U5jGKLCrmiPDHGfu274Y5p7Ib56YVWXMGCOF7OlCXYTf+Itdv0lbxgFnixFxST9lz3dyhs8htAsEvV",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "eJxNTtEKgzAM/JWj73YmWqqgwj7A172LEyY4FSdD/36p3VxpadLLXe6KuVkfuJeqZgLFt6xhMGJ3IopI22SIbJQGqFSBkF5PCAmydyCEbHJCIQWg07FspFRVxcXZVsXP/Gm1QarZIofRZALGNOxDP3aYp35cX6VKNOewIHaXY20J9qB/iVVx0HcuFbHCRr7uUtn9PS4CT277pR06LDLVRqHdZWyObpMu00cUT/rHrcVXspJ1L0uS/Az8AdJiUkM=",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "eJxNjl0KwjAQhK8y5D2xu0lIC2nBA/jqe0HBQv/AIvb2bmqtIYFlh29mJ87t8sCtVhcmUHEtWwajSE+TJhNsr4N2mSpTJLjzIcGifGVGSFIyCpSJyceSSE418ZTONvF3fAjGwxkOqOAN+YyY+rXvxjvmqRuXZ62s4QoBxOlzYQIhbPgONnHDV64VscKb9rnvq+zMyfCFjwYkmRYeftBe2nr9L/EBjAhGbA==",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageX,
    #[cfg(feature = "package")]
    #[strum(props(
        svg = "eJxtTjsKwzAMvcoju9RKtisX3EAP0LVDt0CHDg10KDl/5ECCIUEgwfvpld/w/+B960bjhMhquCKxpK4vp8r1ZVU8VJAHheJch4SELXzJKDaoX4cQ7xuEgDw1RggWo4sasPo8qbH5O7m89j3GwAGGzIZUNx11Fc/Rp+hGzffYNWU=",
        categories = "files,development",
        tags = "box,container",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Package,
    #[cfg(feature = "paint_bucket")]
    #[strum(props(
        svg = "eJxtTjEOwkAM+4rV/UKS3sEhHX0BfICtKkMHkBj4v3BQRYdWkTPYjp32Hj8zHpfuZWeYpRojRxCjw6HLuNRnEQcxSZVKKRah19/lvRvaIcKG9o8sdBWUrXJzWD/brsJWXaoNljJ00mR8yeSUXDKcnPQIirYgkNf2Lx1cMMo=",
        categories = "design",
        tags = "fill,paint,bucket,design",
        contributors = "karsa-mistmere,jguddas"
    ))]
    PaintBucket,
    #[cfg(feature = "paintbrush_2")]
    #[strum(props(
        svg = "eJxtjTEKgDAUQ6/ycbc2aRUK1dnF1cFNdOjoIP/8VpFOJWR6IS9e+53kHJsFXhBMWDEkt1Mo9gtbasu5Vx5WYGCCvJDJqTPhHyLXi92aKXbv4RTL7SDgygQqbAVnK9XXQFYrC3gAjIcrKQ==",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Paintbrush2,
    #[cfg(feature = "paintbrush")]
    #[strum(props(
        svg = "eJxtjjEOwzAIRa/ylR0asB1syc0J0gt0s9Khg4cOvb+KoypThPj6Eo8P9dO+b7zu00MyB4PyEiAR1kk4lUOaQjGPIuXsdss+Lyi4QP7QJgaZe/RMGtKUxZkhM8ShQOE5rfU27q/1/KIg76QIFBE4kSH27EG7kmCh5G0XWxI5QczlMOlEfiwvMfM=",
        categories = "text,design,photography",
        tags = "brush,paintbrush,design,color",
        contributors = "karsa-mistmere"
    ))]
    Paintbrush,
    #[cfg(feature = "palette")]
    #[strum(props(
        svg = "eJxtkM1uAyEMhF8FcV8XG+wFaXcvOfchIloplXqo0h6aPH0GiPIjRQI8MB8jzFK/jvX709X/1XMk9a6eVm9NHFePsi1vA9mWR3S+E/0Gh9csrCcUd/NL8gGwW6g8oz/7v4P7WP07i5MdMIfqRmX5TRAcxqhUxBwUWcoTzcmuEmvOLkyU4jwRw8tRx46JBVIKZi7tDK6pDNXdfQvpSchGepPWQy0fmEqxGikow1NS1QkdhHjVbU07AWaKVwd0gX9Mxv35Ts6t19bjdgEcC17u",
        categories = "text,design,photography",
        tags = "color,theme",
        contributors = "ericfennis,csandman"
    ))]
    Palette,
    #[cfg(feature = "palmtree")]
    #[strum(props(
        svg = "eJxtUDEOwjAM/IrFblM7dtNKpRIPYOIFKAwZOjAw8XrOXWCorFwcy3c5e3k93p2el9NNC01tYJM6Anzk4BCcu1GIORlN3TZlJUT307qck7su/wpV1K8hU3IAAwLdowSNrcjgeEOULAUzic5lY+VdFvmBKoRmmqVqgy1NrgayAr5XlhL41MvmkGRACIoV7UDNZkuaNVaZIwGDiVVkE4MIofgczQI/MNh2s8GilRy3Embp3mzfTRYsvf+28QWkH0sY",
        categories = "nature",
        tags = "vacation,leisure,island",
        contributors = "ericfennis"
    ))]
    Palmtree,
    #[cfg(feature = "panel_bottom_close")]
    #[strum(props(
        svg = "eJwti9EKgCAMRX9l7D1iiuCD+i+RkkJFiFD+fZvFHi4791xX09ogp7Ll5pEswuNRI9wltvyBPkBlrji6RHCz7ILby5mgKxYNL2mYnf6XsSJxxQruWlqG6PEgA3bSoCc+qaUILxVNJGI=",
        categories = "layout,arrows",
        tags = "drawer,dock,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KQN1e05OCXgL5j/99zYvAXpJwQBoRjoAW4Wy33o9B4FxWlsbRd1qIfpuFIQWcqAcamFQ0e4pTMR9i/8C9ty62ySx4",
        categories = "layout",
        tags = "drawer,dock,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomInactive,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "eJwljG0KgCAMhq8y9j9qSlCg3iVSUqgIEdLbtxmDPRvvh8lhL1AtaoTWd+ZHMVpHDOmIxSItCG/yJfbTmVFyzpzpDlAVW4nzxOLMVD8rSSF7xeXMs5UI3uK1Ak2gBw08IovgPvYeJEQ=",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJRfDQ9jNabEA8lIDp703MJZNdZvNoh8DLp/SCtCP0xleX+GfBFUH/OywkgzpqXnxX881Pg5lM30ynoFLsrE7krlv1AykFHEI=",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "eJxFi1EKgCAQRK+y7H+ESVHgeoMOESmtH0HIUnr7tKA+BmbmzZjoV4GYCTuEKzhhQjUisA8by+sToUaI6dnkGqxp68+aYxEGRzhPoM+yLaBWP9jVAKpvdKOh6OM33dUgyg==",
        categories = "layout,arrows",
        tags = "primary,drawer,hide",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_inactive")]
    #[strum(props(
        svg = "eJx1jEEKgCAQRa8yzAVCbZHgeIMOESmNu5DB7Pahq4Lc/f8e/7scd4Gb0CDkSqgRai9XCsKEakHgmA6Wnr2b2sC7cxOGQLhaUHNRTTT0FbboX2EG3L6OHhtuK/w=",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftInactive,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "eJxFyzEKgDAMBdCrhOwiWgeFpjfwEGKL6SBICdre3lRBh2T473+bwipwRS9M2I0IqRD2CBzixvJGmdAglOenXNnZtu6cPRZh8ITzBObUrkKNfti7AdTANHof3746IIE=",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "eJwli1EKwCAMQ69SeoGh+9lAvcwmszD2UQqzt7fVn7yEJInrJaAZd4Q+lS1EhFbpaZIxHAg/3dKW5e5tSZv/Snrpq6Bh3Q2nIU6oIQZf+qYM2XMb6A==",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "eJwli1EKgCAQRK8y7H+ESlCg3iVKUqgIEcrbt6sfOyzz3tgctoJcHWnCm/YSHamZEEM6Yul//hrlNIQq6e0oO2/PdAd8ir2JkeqGZl2xr1vNrljePmuJ2B1dMxYYmIFPqPT+B/NEJBs=",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KqLq9Vi8iegt5/3+b4ySwOdQIawrCDqlH4JhmlvveL8xnRqG3TS14u4zCEBwO1AG1harU30tMUd+i/8A8tg7F9Syw",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightInactive,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "eJwli9EKgCAMRX9l7D1qitCD+i+RkkJFiJD+fXM9bAfuPdeWuFdoDjVCl//mUJNDWhEK54rRBSnmI1VpvJ3Hztsz3xE6/XPFGiE0JhkmCdkdlrfPVhMEhxctQGbSkwa+0Y/GfyCpJIs=",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "eJwljEEKgDAMBL8S9gOSiuCh7We02IJ4KAWT35vG0+7A7MZejkGasIK6JASQ/KAObztHTeAdVEu76vCe4zJ3Od7tKaTBVLahJW8gZX8QdjR3WvkDCFQcQg==",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "eJwli9EKgCAMRX/lsveIJUSB+i9RkkJFiJD9fZsx2B0759oc1oJcHQ2EJ20lOuKJEEPaY/lvgYbwtp1fNb3tteftka6AOsiPxZCcJbhFZS2IqY6391IiNkfnDB5hOgMZxQr8B9N0I/A=",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQth4sNN3AIcQW05uUgLq9DV4U7C38l/9DzavASegQjpKECc2EwLlsLM9dm1qES39iGLQQw74IQyKczQiejYJGb/AN7A+4Tu4/Qzch6ivE",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopInactive,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "eJwti9EKgCAMRX9l7D1iWlCg/kuUpFARIqR/32axhwM755rk1wzFokZ44paDRZoQEn8UozYEH/eQP1MldaaXnTNHvDxUsjizUQ2FoYhJfymNM/eSA2wWTxqBhk6D7vjEi3Ev7YEkNQ==",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "eJwdi0EKgDAMBL8S8gFJvSi0/YwWWxAPJWDy+yY5ze6yk2e7GKTgjjANyaABja238XQuSAfCP27uEWve3Kv5HV8DSSYQglAoavU0kMOe/qkL1G8b6A==",
        categories = "layout,design,development",
        tags = "drawer,menu bar,browser,webpage",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTop,
    #[cfg(feature = "paperclip")]
    #[strum(props(
        svg = "eJw1jbEKgDAQQ38luPf06rVeoRb8lIKDQwUH/x+9ogTekJAkX/U+sK/D6ZlEwExTcIk4wVAjIqZX7JQkdTSlsDjDJhDLTAollWZ+goXVw39VTzp3tH9Eh5JHuy4PCMYboQ==",
        categories = "text,design,files,mail",
        tags = "attachment,file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Paperclip,
    #[cfg(feature = "parentheses")]
    #[strum(props(
        svg = "eJw9ykEKQBEQBuCr/NlPL28mUcMNHEJZWCruH0oW3+7TXmZDjSZ7/HaQEG8Bl0n6nZH0PevAQ8AQBLpeWxmZE54=",
        categories = "development,files,maths",
        tags = "code,token,parenthesis,parens,brackets,parameters,arguments,args,input,call,maths,formula,function,(,)",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Parentheses,
    #[cfg(feature = "parking_circle_off")]
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8ypFezWT8I0drGQ4QoKChIkKC3N9EijbDLg3nDaLs6u81wnSApYO9AFXi97HXx+V4f5lwwdWKvUIHKcNHGNLmRGMSGwZCg8DKrh9arn2YLanymck4zD0moJSM=",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    ParkingCircleOff,
    #[cfg(feature = "parking_circle")]
    #[strum(props(
        svg = "eJwlijEKgDAMRa8ScgEbK5ZCmtnF1b1EoYKDFAe9vSnl83nDe6xn1esAfRPSiKBfZzU4FB66F77zU2BPuEagsIUyZQ8enI3s8xJb3CL5Ab57FfM=",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas"
    ))]
    ParkingCircle,
    #[cfg(feature = "parking_meter")]
    #[strum(props(
        svg = "eJx1jrEOwjAQQ3/F6n5HLpdeEil0YeYjKhgyIoH4fi4dUIdWlrz42XJ7rZ+O53W6V9RVoQgQlyFMS7uMdGl/RiIkfvUo8dLc41EyhjPyNhxIlA2R9WacIMmtwJFhvbyFjdIGGGXOD1biwomEZ3evnZyqu1M/5nA0HQ==",
        categories = "transportation,maps",
        tags = "driving,car park,pay,sidewalk,pavement",
        contributors = "danielbayley"
    ))]
    ParkingMeter,
    #[cfg(feature = "parking_square_off")]
    #[strum(props(
        svg = "eJxtjrsKwzAMRX/lkt2qJeVRgxvo1qVrd0MHL4EMwd8fOUMSSBACIY6ObpzTkvF/NV+lHtZvgcBbMTpo5jYdC5vKeeGoC2BquRnjo4rGeOjwpOHHYad9Pc/sSa/0VCExwt+YWMGaFLplMJHrP6HIDWlhhuLk9GEFj5w16w==",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquareOff,
    #[cfg(feature = "parking_square")]
    #[strum(props(
        svg = "eJwli8EKgCAQRH9l2R8oMzJBPXfp2l1SWm8hC9Xf5xbDMPB442reGa6SmDyqGeHxqBEol4P4J/dHapsBg+vkENwZmSB5XC0osxkaowYNfYtqnRYrqkjhBU5AGjs=",
        categories = "transportation,maps",
        tags = "parking lot,car park",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    ParkingSquare,
    #[cfg(feature = "party_popper")]
    #[strum(props(
        svg = "eJx1kb1uwzAMhF+FyK6rSOrPgJulS4d27dAtcAcPLdChyPP3qACZHNiSaR8/Hk2tv5e/Xb6eT+8VQ1ThYmL2rRk9OfpyOq9PkXNe75lFfEfWA8VMxgNJq9hjyvKx9hNaMlhBrxfDIrFyXEmxNHGoXfMGxWgJtYuiOaVSZ7Qn+NgSxQk0NO69USvlTYtoPvZUJ2OEfcIMCsHBRsKWg4IqbRMfnBM6CVhoxWf0qv2gsCqHy5ostLHiLBzkCJJilL7mlwXVpPA/F6kRLtJg/tGPpkrIN3Jhy80wnKh2GlV2TivnMWb2yFXTbN9vW+SmyOXXKvEScCTzFh7X593wH2lof2M=",
        categories = "emoji",
        tags = "emoji,congratulations,celebration,party,tada,🎉,🎊,excitement,exciting,excites,confetti",
        contributors = "karsa-mistmere"
    ))]
    PartyPopper,
    #[cfg(feature = "pause_circle")]
    #[strum(props(
        svg = "eJxdjDEKwCAMRa8ScoFWsUMh9TLBQZAOTvH2TRp0cHrw3+MT186tAMuDISLwcHbFiZkO95lafQtI/GeQ4BzGS6n7bbVVszWX9sYv0mo/nishQw==",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "colebemis,ericfennis"
    ))]
    PauseCircle,
    #[cfg(feature = "pause_octagon")]
    #[strum(props(
        svg = "eJx1jCsOgDAQRK+y6QEKndAPyVKNKLaeBFGJIAhOD3QJKOR7LzO8zluhZVCTacnY3KvIze0if6X7K177K6IEbT0SQFXsgsZpBEfAWG0CicmCz/Z4b0/5JSL5",
        categories = "multimedia,shapes",
        tags = "music,audio,stop",
        contributors = "mittalyashu"
    ))]
    PauseOctagon,
    #[cfg(feature = "pause")]
    #[strum(props(
        svg = "eJxNyzEKACEMBdGrhH+BRRCrxMvsysZWAurtRQu1meoNl/Qaacq/msAFUM2fqcCD+moTBER+Joy8+CH3uLnz2w9G5htU",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "eJxtjUEKQjEMRK8Ssjc236RWaAsewIVbd6UKCi7kI6K3N23diBLChMxLJtbLXK8nmBNOCPWVUEyeCZkxx+Vwc/xQ3QiDM+lHfyhz2Y83k/vFbuV+hmPC3QbYFQUFZ8Wm+liRFmto3bcLT0GAyYnuPekEvCYJICQeuHnbLxzUJnaHltdy8hurRjf6",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "eJxti0EKgCAQRa8yzAHKCRUC9QYdIlIadyED1e3TVi7cfD7v8VxJh8Cdo7BH0gjl8bggvP9yyidLvQqhcoPBzS0I7tqFIXrcyABpnhQ112jnVrBsh5xUJz60KSVN",
        categories = "devices,gaming",
        tags = "computer,chassis",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PcCase,
    #[cfg(feature = "pen_line")]
    #[strum(props(
        svg = "eJxFizsKgDAQRK8ypE/M7iZKYM0J9AJ2AYsUFhbeH7ONMjAwn6d3ezrO1e3E4NiLqzpZV/Vf5pAhITcOdjKLQwSBbAuoXD6NRD4dH/0C6QYVeg==",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenLine,
    #[cfg(feature = "pen_square")]
    #[strum(props(
        svg = "eJxVTKsOgDAM/JVmvoN2XUCMaQT8AG4JYgKBIPt+WsMjTXqP3F06y1Vhn9xKBDJLYWDo7VBZo4+hyPVvIDccXE6dreT0bo0+AvtY2JPG7VmeIEBYVFA8UFQSyvbUb5EQH1Y=",
        categories = "text",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    PenSquare,
    #[cfg(feature = "pen_tool")]
    #[strum(props(
        svg = "eJxty0EKwjAQBdCrfGY/kck4JkKSE3gJiYJCBCkubE/flCy66eYPw38/fe+/Fx6ZPuIhVwQOUGjPwMq6UEmnjZS0wwhRFmccnN08fFNnkHN/RCGxGdvRrkv0RbyM3EV9T7U9Uf+ZRAh1HnfK5Dc06rICRCUr0w==",
        categories = "text,design,cursors",
        tags = "vector,drawing,path",
        contributors = "ashygee,mittalyashu,ericfennis"
    ))]
    PenTool,
    #[cfg(feature = "pen")]
    #[strum(props(
        svg = "eJwBQgC9/zxwYXRoIGQ9Ik0xNyAzYTIuODUgMi44MyAwIDEgMSA0IDRMNy41IDIwLjUgMiAyMmwxLjUtNS41WiI+PC9wYXRoPvZCDv4=",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Pen,
    #[cfg(feature = "pencil_line")]
    #[strum(props(
        svg = "eJxlizEKgDAUQ68Surf2t/1K4dsT6AXcCg4dFBy8P/YvOkggkORFrno37LNZKSD4lk2RQbsi3zI6RnRcg1NIzXcRIuIygfJhU09k0/Z/n8RgJd/pAZUVHMo=",
        categories = "text,design,tools",
        tags = "pencil,change,create,draw,sketch,draft,writer,writing,biro,ink,marker,felt tip,stationery,artist",
        contributors = "colebemis,csandman,mittalyashu,ericfennis,danielbayley"
    ))]
    PencilLine,
    #[cfg(feature = "pencil_ruler")]
    #[strum(props(
        svg = "eJx1jz0OwjAMRq/yiT2mdpwfpNIThAuwVWVgoBID9xc2EQxQ5MSR8uRne7zPjysux93KCQkK3U3j3j+n8Y1OHFFQqUCozELKeKXBI0RSDM0IEsVvamG8FXD89a4VGRJkgwhEzJcCU2rCxAWZquurIU+9uQZtkRI4UzpveLga+tPDnMxQin4XOmg/Pn3uc9+CkFVTXkJnwZm/fWkr5/IxPwFZsEzm",
        categories = "tools,design,layout,text",
        tags = "edit,create,draw,sketch,draft,writer,writing,stationery,artist,measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent",
        contributors = "danielbayley"
    ))]
    PencilRuler,
    #[cfg(feature = "pencil")]
    #[strum(props(
        svg = "eJw9jDEOgDAMA79idSc0baMyhL4APsBWiYEBJAb+L5IBZOkGW2e9+3Ngn8PKFbknmgSGjAi2FJSlklXRgZROJhmEZAtNR3ebfg8XC8SNf3oB80QWUA==",
        categories = "design,cursors,tools,text",
        tags = "rubber,edit,create,draw,sketch,draft,writer,writing,stationery,artist",
        contributors = "ericfennis,wojtekmaj,mittalyashu,danielbayley"
    ))]
    Pencil,
    #[cfg(feature = "percent_circle")]
    #[strum(props(
        svg = "eJxty0EKgCAQheGrDLOvnEBhYPQGHSKmwKAgpEXdPsVFBK5+eB9PdEu6r6C3RxoR9KlNOQaDDNWDnPMVYfF4kAXuHLiCZfxoYuDYG2pIPpH92QsTGSGs",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentCircle,
    #[cfg(feature = "percent_diamond")]
    #[strum(props(
        svg = "eJxtjzsKgDAMhq8S3BubtkGF6gn0Am6CQwcFB++PaRVBW0JCwv/l5Y/lDLD21WSwAdJoF4OOIAX9mJVia5A7iCEDbEoToIqAVhEZycoKWUN/QN0T5mrwdbxn8O9VHRoQD6gpV3dyyCKzYuBCs8jyk8P2034BAHk3rA==",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentDiamond,
    #[cfg(feature = "percent_square")]
    #[strum(props(
        svg = "eJxty00KgCAQhuGrDLPvx0JJUG/QISKlcRGEDFS3T2sRQZtv8T28JoWZYY+eyaIYEA6LPcJ5L4W4ED9/ytChM00JnNkmJvAWVyFBVwpUoXK+NGrQVLfiR3Ik5McuQAYl9A==",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "eJxlykEKgDAMBMCvhDxASUFFSPqZ0EOheOgp/b2x1YP0tLvscMlXAguCG0LzoNOT+jTqM/L6oMiaq5YEaoL74r+2t1TB4OlwkB+l47OjTfgG600i9g==",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "eJxlyuEKQDAUhuFb+Tr/l+2ME7XtDlyERlGUlsTds5SUn2/P6+KU4jwgHp4ME+LpqSKkuyi44tHg1m4b0XtaGrCGVQILyUOGDwtqCBii+K+tYRi9l69ceM8iHQ==",
        categories = "accessibility,people",
        tags = "people,human,accessibility,stick figure",
        contributors = "mittalyashu,ericfennis"
    ))]
    PersonStanding,
    #[cfg(feature = "phone_call")]
    #[strum(props(
        svg = "eJx1UMsKwkAM/JXQe8dNut0H1ELv+hELHnrw4EE8+PUm7bYUVAIDSWYyYYZHec50OzdXEeKALK+uCAk5LW4FnLThjJgrrouE0LUdXLRpv8K6CW34wTeqieK0HycPZpL54GdWiAqC5Cu6pRBJOyNsr8H3OmK+JLhMGZkLq3OogkDhzpDYGhwdVLJovz3MAMcHt0jezTicLKhx2ONiD6cPFLWu7EQR2f+lhqmnmhFpqOx25getik6Q",
        categories = "connectivity,devices,communication",
        tags = "ring",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere"
    ))]
    PhoneCall,
    #[cfg(feature = "phone_forwarded")]
    #[strum(props(
        svg = "eJxtUNEKgzAM/JXQd7MmampBhb1vHyFsoCAqTMbc1y+tOoSNwqVJ7nIh5TT2S98Nd5jGbpgflaECGJhBQH9kTV2edk5dRuZClREDC8fw0owyjZoyB/pKnZq5hVtlrjqLBD0/00YHg9VHCWP0IY/Ob7g2CpQ0SdG6UM1XWDuSyB9+oAaRO3+HQ4ZEwO3BL1ihU2Assg1tfOhAs0DYV8Ms1xLRpUDrwaOnhtRZNoGA9ITskgBHB5VE7a9HMMDjgvtJ3vG6eqj6A6g4UxI=",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneForwarded,
    #[cfg(feature = "phone_incoming")]
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaaEFUqBTt6ZvsahFassySZJIJU41Dt3SP/gbj8OinZ23IAoOAB2bwpqkOG6WpInGm2jAbmFnJBhbN5ZM00hN1bKc7XGtzYd2GgV9Fy7I5l6CMkbzKBHRhxdTwaIuswNxp9ZggdWxm//CVqkPu9F0OJRIB33d6KoVOgNGXK+Yx0IFkSthOw/IoJaKzxzxAwECtuCEvDViwHSG7TGGvICNx9ldDBXB/4GbJO7orRjUfYXVS5w==",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "eJxtUO0KwjAMfJXQ/41NtvUDtoH/9SEGChOG+EPE+fQm7TYGSuFKcne5kHa63a8wc2eigZk6wwbeUpGXX0s2fXtQUd9mqTYzyZlcPWXCJn0MzxEunTkzA3lM/KoGBgYnjywjRSkoYUgLFiKir2yFLmi3KVAYb/0fvUrVFI7bcKiRCHjc5WkUBgHGWC/o8sMAUqlgXQ3rRlpEp4guQcJEA0myXwwe/ETIwSrsE8SSvb8ZGoD7BdeTfPRceqj+CzEIUA4=",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "eJxlUO0KwjAMfJWw/4tNWvsBc+B/fYiCwoQp/hDZ3t6kHVQYLddekwt3Hd75M8Ht1F3JoI9AFi1l8iDblGXRETD6mZBDr5AZuJT0nahHdwRijG7DqpMrYYDWK0zI1zZ5L/IohBKGtGEtRPS2t2iCvjresNbEoRWlD8+Cyl3ej1B1mXNuFpzYBZ7sf4Lqa+9fvUuEFqDk1MCXiCZBwkTdOBz0A8dhfrzusPCp4w4WkkPOlQpduVBp1abxByuCSc4=",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MTdR+gQv/bQwgtKIgKlVJ7+iauFqEly4RNJpkw5TT2S98Nd5jGbpgflWEGDwIMZIFNXZ52Sl2uxIWFZOClSTNVhqyBRbJXeqROzdzCrTJX1kUY+Jk1ujSVoISRvCoEdGHD2PBosyTD1Gm1iBA7NrF/+ErVIXf+LocciYDbg55KoRNg9PmG6RroQH5K2E/DvJAS0cVjGiBgoEaMkBcHLNiekF2icFSQkXX2V0MF8Hjgbsl7dVeMqj9VCFLe",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOutgoing,
    #[cfg(feature = "phone")]
    #[strum(props(
        svg = "eJxtkMsKwkAMRX8ldD/XSdpmZqAW3OtHFFx04cKFuPDrTfqQAUvgQh43J2R4Tq+Z7ufmJkKsKPJuJyGhaMFBwNkSLkhl07WRoW1oEZNX+1XWjgY9mPdRN6XLbzl1YCaZK56jkEwEuds0LoFElvnAfhq63krM14xYqKDwxEbWzaCkD4ak4FITzLJ4/xkOQH3g/pJPMw4nf9T4BdqtOmY=",
        categories = "text,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Phone,
    #[cfg(feature = "pi_square")]
    #[strum(props(
        svg = "eJxtizsKgDAQRK8ybC9mV3AtEm9gay8qrp1I8HN7DYJYyHTvzfPr2EesRyAh2DhPFgNxRTgDFYR9HqI94Eig9nkKar900TAEahRq7JJI6CPYQbd/U4K1EwjcPc4kk1bf3wXPIyfm",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "eJxNyrEKgCAQgOFXOW6/8jzBAvUNWtvDAoOIhoZ8+9QhGn7+5XPHfm6QtUetEDJ7NAhP2Vim64LrqwnuWu4Eq8fJgI2KuLPAnZBAKbFUWMkP8gBaxSYVCTVNMpuPvuLWH4I=",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "eJxNjLsKhUAQQ38lTK93Z64owu7WNrb2ouLYiSw+/t6dTlKEJJz4fUyKOVAvjHaoR4HAmQoppKu+GXKymxy45LKFDaIVRf+zj+iPZUq4tjlpIHYEXbZVU6CGcOdGCEf2bE9Of+OMiC/4iiBp",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "eJwVjEsKAyEQRK/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR3mLKgpehfezKuwRHytM7Ju/u0JCM3SKODCNXDYkLDPxOpEdKC2FNtiazQaEjefNgx2owxRu4zOF68gV9DhfWiMuCL+I4tgjXL1ZhM+5V+2bQfj2sMMcTvoDoAYlcQ==",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture,
    #[cfg(feature = "pie_chart")]
    #[strum(props(
        svg = "eJxFy7EJgEAUA9BVwg1wXiLCF74HDuAQgoWlhVg4vV+Lk4RUL36s545tSouYRXDINs4siMZEDMrWp+rdS6v/B4Fq9OOCLpa74QcRMBYw",
        categories = "charts,files",
        tags = "statistics,diagram,presentation",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    PieChart,
    #[cfg(feature = "piggy_bank")]
    #[strum(props(
        svg = "eJxtTjEKAzEM+4q53W7sxO0F0vtB19tLOnjsUDL09VUo3HQYhJGEpPZ+foJe9+WhlbyzilNik5VUCmcyzuKTZVWWDCQYIK5AyFTEd0tRBlvkYfMpHVaH58aKAAsDF2w98RT+aaDT7t9la5c5YWvHEKM6tM8OlYoGXOiJT6+kGulQflP1MIs=",
        categories = "money",
        tags = "money,savings",
        contributors = "ericfennis"
    ))]
    PiggyBank,
    #[cfg(feature = "pilcrow_square")]
    #[strum(props(
        svg = "eJxti80KgCAQhF9l2XvlGvYD2rlLDxEp6S1E+nn7XDoJMcwwMPPp6LYE3oXdJ4M0IFzBJv/VeBuUCDlbhIdz0g0Dkz7W5MEaXEgCyXms1SprBWyRRSAqNVPPAF9LoD9J/C1dubzy1Cne",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "danielbayley"
    ))]
    PilcrowSquare,
    #[cfg(feature = "pilcrow")]
    #[strum(props(
        svg = "eJx1ijEKACEMBL8SfIB3wXhHIFrb+AjBwtJCfL8IopUs28yM1NQKZKciGqCOn/LyTOblmP9qGCiwtom0hfl3jQOa3Q9sSBoy",
        categories = "text",
        tags = "paragraph,mark,paraph,blind,typography,type,text,prose,symbol",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Pilcrow,
    #[cfg(feature = "pill")]
    #[strum(props(
        svg = "eJxljDEKgEAMBL+yXJ8zEY8oxHuI3YGFhYKF/8fE4hqLnWIHxu72HNjXdAnngjEgTMJtykvBB4aASUlP//FXUOiWqg3RqtaLs7dirrt9AZOcGgo=",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,tablet,pharmacy",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Pill,
    #[cfg(feature = "pin_off")]
    #[strum(props(
        svg = "eJxljUELwjAMhf9K6H2xL7brAt3AmxevuxcUKgzxIOL+vd2oTpAckpd87yVO19uFZumNiKEXSi+tynmVQ9wt0BD/UFQWlUXY4Ht6ZDr35qSkT3Bok5CQLYUGDFDZ6VTG0LEePjdLnuBZ3IiQsf5egn7i4El578Y2I20u27hj4E6/hjdAPTO6",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "eJxNjrEKwzAMRH9FeI/rE0qcgGPo1qVrh26BFlwIpUMJ6d9XCoQYgcRJusel+fV+0sqjAzv6QWfUqZpVr9j2OZ3sLafP9C30GN21JcQCWRr42E1MTMFKJWC7YbbeN34470cQ1BX0/9YVHB51yaWvNUnBIjUYG5gOcMUNpNjWs9wtpyXMfzRFLo0=",
        categories = "maps,account",
        tags = "save,map,lock,fix",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Pin,
    #[cfg(feature = "pipette")]
    #[strum(props(
        svg = "eJx1jTEKgDAMRa/ycU81iYqF6gn0Am6Cg0MLDuL5TQcdChLe9PL44dyuA/tYJYEImPjQ6MlXU6izmsL7sCiEb/qxiTv0UNeSsYljZBqwnUJnHuCja0tH5qIlQ6bMbCsX67f2AMSFKQ4=",
        categories = "text,design,science",
        tags = "eye dropper,color picker,lab,chemistry",
        contributors = "Andreto,ericfennis,karsa-mistmere"
    ))]
    Pipette,
    #[cfg(feature = "pizza")]
    #[strum(props(
        svg = "eJx1jbEKwCAMRH8luGs90bhYoR/QjxA6uBQ69P9pgtAuloQj3ONy5Wp3p2M1OxIB3XmYWhZ1a/kYCOmPMYHn7AyCKHhiyzb4TS7ZMYom35LL0pUd0ER9pKGaEB8uWpU3+ACqBy9e",
        categories = "food-beverage",
        tags = "pie,quiche,food",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Pizza,
    #[cfg(feature = "plane_landing")]
    #[strum(props(
        svg = "eJxNUEsKAjEMvUpwn2eTJraF0RPoBdwN42IWXQh6f0wHGaU05H3IC5me83ulx/lwU1JdNR0u03Fwl2lXMkohSaMqta5scBII3Jf40IqGamhBmr+QN7V88bVShffMp4DJ4XmOKErxZDDD5dpRlBy2a+lPMxgpdIFFVZSYNyKSoJ06trk5xFiiMtLArbKMTiE9OqkQXxhWgmQ0D5sGnwtD7WrQSAqb/1bjCGbket8P8gF1sD+l",
        categories = "transportation,travel",
        tags = "arrival,plane,trip,airplane,landing",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneLanding,
    #[cfg(feature = "plane_takeoff")]
    #[strum(props(
        svg = "eJxNUEkOwjAM/IrVu4fY2aXSF5QPcIvg0EMOHPi/cFhKFSmWZ9HYnh/tudH9PF2UVDd10zKfBrbMO5PgE0lGoGCls7JVCCPGZi5y9sSQQq7DdLKj7oOuhUQpUuqojHB0KVyF9gCn5A8+hbxhqWyS1BRhiO37pWXPFraqUG6CMVH4cijZulo7wxfkdGOoR0iMhBJY4AbvypoRy1hM/wMZq5ZdrvsdXoLvPKc=",
        categories = "transportation,travel",
        tags = "departure,plane,trip,airplane,takeoff",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    PlaneTakeoff,
    #[cfg(feature = "plane")]
    #[strum(props(
        svg = "eJwdj7EOwjAMRH/l1N0G22naSqULC0P5CBQGhgwMiIGv51xFSeR79p28vh+fF56X4W6TzrBFHVZh1kNH4b26ocJNRxR+iCYmiXCWQpFgt8CMwvGq3hIqWxY+psRdlD7U/SAwaHAs9gXmXRxxK11SDzgPodhXokcyRowajRYMOPJCo6eTNy1pXUUn1qb+G7b1lAttf7mWLN0=",
        categories = "transportation,travel",
        tags = "plane,trip,airplane",
        contributors = "ahtohbi4,csandman,ericfennis"
    ))]
    Plane,
    #[cfg(feature = "play_circle")]
    #[strum(props(
        svg = "eJwti0EKgDAMBL+y9APaHsRDzGeCSKE0pXqwvzdRT7PszpLkLmVH30KcA+Q2JuN4yTR9O1PTMg6taJrrdbqNFXFBTLDowQo//CI/kkoZUA==",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[cfg(feature = "play_square")]
    #[strum(props(
        svg = "eJwly0sKgDAMBNCrDNmL+EEqtDmIO7HFdCFICai3t9HFwMCb8SVtijvQQLhyVAnUOYKkvIv+/fmw1E1P7Fs7sD9XFcRAxwyHCWNTsxgb8Asi+Re+",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlaySquare,
    #[cfg(feature = "play")]
    #[strum(props(
        svg = "eJyzKcjPqUzPz1MoyM/MKym2VTJVMFYwtFQwNFIwVTAyBBLGSnY2+lBVdgBVDA2H",
        categories = "arrows,multimedia",
        tags = "music,audio,video,start,run",
        contributors = "colebemis"
    ))]
    Play,
    #[cfg(feature = "plug_2")]
    #[strum(props(
        svg = "eJx1zDEOgCAQRNGrTOiNDMqqCXIDWws7EgtKC7PnFxsqaP+bTHjSm3Hv5tjgVEwM419iqJ2+Bw5c1DfEY82cGyAgz2JOpyQQWBAcypFVe9X9B0vKKRA=",
        categories = "devices,development",
        tags = "electricity,socket,outlet",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    Plug2,
    #[cfg(feature = "plug_zap_2")]
    #[strum(props(
        svg = "eJx1jjEKgDAMRa/ycU81iVKF6gn0Am6iQxfBQXp+00FwqEsC/30eP1zbHXGM1ckKIYG4LurMAl9Noc50Cm9n4QbcJtISav8Rg4ed2Hk0pPbVriaS2CfZTel8DmEhdC0JbJZ83Q8lsy+W",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    PlugZap2,
    #[cfg(feature = "plug_zap")]
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8y2BuzuzFaRE+gF7ATLCwiWHh/zAa0SRj2A/N44d6fE8fUrN4I2BrZ2Tjo2BzRbyEGjbH1KaxcgWVwa+bQqXAOn/ZiMENaKat1MD1IdFkQVQCyWvqMgVzFTmNyO7jTRz0/8QIM0jQN",
        categories = "devices",
        tags = "electricity,electronics,charge,charging,battery,connect",
        contributors = "mittalyashu,ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    PlugZap,
    #[cfg(feature = "plug")]
    #[strum(props(
        svg = "eJx1izEKgDAQBL+ypD80xx2ccOYHtinsAhYpLSTvNzbBQtlmmWH8LFfFsYYtMpgbaUg+PTD5UAss8weP+icM1rQIBHNfpP4qyRuQZNtHegPetSOj",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    Plug,
    #[cfg(feature = "plus_circle")]
    #[strum(props(
        svg = "eJxNycEJgDAMRuFVfrKAJqcc0m7gEBKFCh6kiOj2tnpoTw/eZ75l31fkQDwS/C6V0udrtOH3aMd8JiyBJgVL0kp1dcACvRq8h3gZnQ==",
        categories = "maths,development,shapes,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusCircle,
    #[cfg(feature = "plus_square")]
    #[strum(props(
        svg = "eJxNy7sNgDAMBNBVLC+AEhoXdjZgCEQinA5FFp/twVCQ6k73dNzKYnAJjghHzaaCgRC01FXt6+eL7YmIiQc/JN5mU8iCE0GISg4+dRAi0P7DDTOBHeU=",
        categories = "maths,tools,development,text,shapes",
        tags = "add,new,increase,increment,positive,calculate,calculator,button,keyboard,toolbar,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusSquare,
    #[cfg(feature = "plus")]
    #[strum(props(
        svg = "eJxNybENACAIBMBVCAsYiHQvGziEiQWlhXF+Y2No77DGDpqNu5FoSGVHeeb4I0p20lyGNA63",
        categories = "maths,tools,development,text,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,toolbar,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,+",
        contributors = "colebemis,ericfennis"
    ))]
    Plus,
    #[cfg(feature = "pocket_knife")]
    #[strum(props(
        svg = "eJx1jr0OwjAMhF/l1D0h5wSTIfQNmJA6sFUwdGBAAvH8OFFpl1bRXWR//iuv8TPhce4uEfLlPYCQJrlGKCJO7zl29pvkz7u+HGp3X5YZzNDJB24gBfMOkuBzRDYbExJCfe7oVZs9HW2hzIimmm122z6BNDaIrNOQXBp0qf4B/eU6Pg==",
        categories = "tools",
        tags = "swiss army knife,penknife,multi-tool,multitask,blade,cutter,gadget,corkscrew",
        contributors = ""
    ))]
    PocketKnife,
    #[cfg(feature = "pocket")]
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yuI8aK2lhQgdo214oUBAVkqBO32gtZBbz+bz3ZdTJwD6zdYDeoNAcOHR0SJ9fQiPF7muakpeqIQZxG2up4Q9Tss2zSsbgbmf9ATFYn86ZTVlFsgZAQbmgP6RexLYiBQ==",
        categories = "brands,development",
        tags = "logo,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Pocket,
    #[cfg(feature = "podcast")]
    #[strum(props(
        svg = "eJxtjUEKwkAMRa/ymX3i/HZiW2h7Ay/gbhgFBRdSXOjtzYybLiTwfyCPl7nct/K4onyWQAaUt3cXsHmFdT78zuv8zK8bLks4keCQPRB9iA6xRKiJ9gm9qCGpZTVfGiAOFlEehRUySS3UzlVftTv5CKZssGYmRsQ/EAdw9B8TpsZFYdyBX/DtMew=",
        categories = "multimedia,social",
        tags = "mic,music",
        contributors = "iiaishwarya,ericfennis,karsa-mistmere"
    ))]
    Podcast,
    #[cfg(feature = "pointer")]
    #[strum(props(
        svg = "eJyFj88KwjAMxl8l7J6YpOlsYe4J9Lp70cMOHjxIn99UpChMRgjkz+/jS6ZHea5wOw0XVRArCRKwh6BXwzwd2n6eOiUJRCpKUdA3yKiolb978H5LaiC85B2lbCkZMsXF9qT87+APKB4GXMPvmyvqFZV8gkYRKY0YKWcfBbtjoLFl9xJwNDRez0eQ2D1fm+1L9Q==",
        categories = "cursors",
        tags = "mouse",
        contributors = "ericfennis"
    ))]
    Pointer,
    #[cfg(feature = "popcorn")]
    #[strum(props(
        svg = "eJyNTjsOwjAMvYrV3Y88N20dKXRmgAuwVWVgAImB+wtHSGWAAXnx+9r1sTyvctl3J7r4YmKS3qNZNhR7+g/F5G6uu1Y71095WEyK+Ld2Z24alb+ClsRXDFGLggwX3tQwCm1VEINialFEwWEKaoyHCCpyI5XHPqTSvKGhVwpced4uvQDH6jhT",
        categories = "food-beverage,multimedia",
        tags = "cinema,movies,films,salted,sweet,sugar,candy,snack",
        contributors = ""
    ))]
    Popcorn,
    #[cfg(feature = "popsicle")]
    #[strum(props(
        svg = "eJw9jTEOgzAQBL+yor8Nd7LDWXL4QT6QDpGCAiQK/i/OSFDMNFNM3adjwf/TfdX5hiammS70kKEXo6/i1MaUWDIu9dCIAxXBGg3BzMIBRr1chPrrxvpqh7Hen80MZpKZG08/AR+vHzg=",
        categories = "food-beverage",
        tags = "ice lolly,ice cream,sweet,food",
        contributors = ""
    ))]
    Popsicle,
    #[cfg(feature = "pound_sterling")]
    #[strum(props(
        svg = "eJxtjCEOgDAQBL+yqS/c9oBWHP0BjyAgTiII7ycIUlM1YiZj1347zjVsLMiHxHlQ1Vh+QkK18YuqtVSQH04dsyDRmbqG6my3F42TIJY=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    PoundSterling,
    #[cfg(feature = "power_off")]
    #[strum(props(
        svg = "eJxtjUsKgDAMRK8SeoDYxJq2UAsewEMUXLgRXIjnN6XQjSWfxcwbJt3lOeFYzU4BZwFBcVuECFaHgC16D7SYnKZK5tR5Qao4SWk41WOU0P4gQQz8ur9xqa5Nut37AL33JXI=",
        categories = "connectivity",
        tags = "on,off,device,switch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    PowerOff,
    #[cfg(feature = "power")]
    #[strum(props(
        svg = "eJwtykEKgCAQRuGr/Mw+c8awBPUGHUIoKIho0SJvnwOuHg+++JT3wJZo5cU4D2/8VAICLBg8sJjZwVKOo8Icr/PeUSURC+Hj3v61vahVlX/xbxYV",
        categories = "connectivity",
        tags = "on,off,device,switch,reboot,restart",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Power,
    #[cfg(feature = "presentation")]
    #[strum(props(
        svg = "eJxli7EKgDAMRH/l6B5sEoJL7Ozi6l5w6CI4iN9vXFQot9zd4/lRz4ZtSotAm+RUfHiu4h9g6MVcBYIcYYo223+TrNqb+4hwjQyRF9+Q0Ryn",
        categories = "multimedia,photography,devices,communication,design",
        tags = "screen,whiteboard,marker pens,markers,blackboard,chalk,easel,school,learning,lesson,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Presentation,
    #[cfg(feature = "printer")]
    #[strum(props(
        svg = "eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv794sZQAoGcnFzi8zyuY5oG5DlN5RnIoYUDwza/1lL0p78Vfe6KoA90dbq8VB2rddayhg0vpt4BFIh1RwBe6uMFWAx/8zU1+sdwK3ilvkggy4S3fkNYdagIMqS7lEDN5m9m/AAF6TAw",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htqd7dNLSQ99+LVe4lCBAUpUvTvjQqmpWUPuzPDW8bdh0fEydOhRgNFTZ3bfbzO/ZMW9qhr/8aaEDGSo3AZw/WM8PLESghPTy1h9PTFf+nsMXOxV7D0Ug4CQZmG05apytqkq19oI5Op5oCRKAU3G9XZgm3MDd+A8zzA",
        categories = "multimedia,photography,devices,communication",
        tags = "cinema,film,movie,home video,presentation,slideshow,office,meeting,project,planning",
        contributors = "danielbayley"
    ))]
    Projector,
    #[cfg(feature = "puzzle")]
    #[strum(props(
        svg = "eJxtk8FOJDEMRH8l4h5v4iS2I7FIe9/9gb2NhgMHDkjw/6LKAaERaLprunviLj9X5v7l8vZUHn/f/etb5tjFJda1Sptbhqq0tcVmiMaW8HjusixK6lWm8/BmeNDi88rbfK2iY+Fax6hfj59rF+u9pF5kB47S8OlVYrioG6ynwx5nNMVNVNlG0XVRWa2XoygqrQ7RPksq2pkm3QwAS+Z2lrAWRr55flrBhnanm2wGb55tlqNnFRteOD2f6od+/TaTi+/AKI5eMAXNWUC5sgHCQaKbWHsAC2UxATArl2V74CFTEnUSDT8KIsP6HhIb30uBZJ6FP1khovS76enPt+5RtDH2rlfUWMfUh1UmNQl02P5O3JcQRw8Ka0XLgcmNNWR3EI02ZPUFIKcleuOmaD6o/QapMZ61ayrnEIi1b0Ok3BxrsdPGoaIcyS+8fZg0wzJDUrCqNOXW01VTf6LSA3YlE58kEdjO1suwbjavcsawh8OIhNDNkLj5JkKqTCknXL/HVDKgo2SyiZgqc2JMLPAs/X/3cP+Lf7GHd/XQrjQ=",
        categories = "development,gaming",
        tags = "component,module,part,piece",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Puzzle,
    #[cfg(feature = "qr_code")]
    #[strum(props(
        svg = "eJx1j0EOwiAQRa8yYY92ZiLdQNduPERjG6c70xDU28vUaG0KISGE/95n8PN4jfAMhg28ll3G6SYxmJOBOd+jgcc0RNGLzh8V7/wiffBf+C+q577+ztKopvHOuvdRYAjmQgjoxHJPQNDosvmUWFGFtihhOjRYyJCgTWsJasm5LYAMSFLt4HqErva000osD4xUHzh/xq7RG703d6E=",
        categories = "development,social",
        tags = "barcode,scan",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    QrCode,
    #[cfg(feature = "quote")]
    #[strum(props(
        svg = "eJyNjrEOwjAMRH/F6u7gcxpSpNKZhbU7CkNHJFAHvp5LkMqCEIocW3f26Y23y2OR67E7R3GUKCZZwRrmVEwRPGnIaa8eDFld/dSXJoupCy3+CIfsM1BMqkNRvD0w7l1Ym8toRojfOQSzgQ3BIma3sq0Knt007irZNG58SL8A8wdw0f8JF07UvGo86JlCq1/jd5oX7HxAYQ==",
        categories = "text",
        tags = "quotation",
        contributors = "Billiam"
    ))]
    Quote,
    #[cfg(feature = "rabbit")]
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/lyC7VkhXbgTRfkK7dTTp47FDy/ZVDyBSMD5t7HHfzt/4aPs/hpQGFR3tbVSgCBIEMYY/DMj86tcwXKwUqjUo1ONJhMoezn+OHTLmxrhMnJLYqPEUc0k2BciG/q4xc0MktcveSO9k17XK26NH+aiQbiXuBelB0veuVOXm88hirA+cI8RLGtzM8K51kOGqpYbzfK9o4yOX9AakVRzA=",
        categories = "animals",
        tags = "animal,rodent,pet,pest,bunny,hare,fast,speed,hop",
        contributors = ""
    ))]
    Rabbit,
    #[cfg(feature = "radar")]
    #[strum(props(
        svg = "eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBQqKEjxQW9vQrWCtOzfw+x8THs/PkY6debACTaSR3IDWyo9lyIlcnDe9O2+PvftYvGkIyyvKAJJlKCysLiMMBxTAxdWLKwQTxFRByX9OBqIo6JoXHMIcbMRgGPNzQz94iqQA2L85+XLlK9nys/OsBjKr/lOnZH6NMs/9I0dfEFZhESh8Hd1Lbg30f1SxQ==",
        categories = "navigation,maps,security,communication",
        tags = "scan,sonar,detect,find,locate",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Radar,
    #[cfg(feature = "radiation")]
    #[strum(props(
        svg = "eJx1kLFuwzAMRH+FyM6rSYm2DLhZOvcjCnfw2KFTvr53DpouDQTZoEg+3nH7+vg+7PP18h5pkcd0uW4vertuj8yCso7cHc1RjtWxeCjobw2rFfOJZgNhhAQLA4VuoXPUPrni4Ylh6Q2dcfCurqj4Xt7Rb//MThJyx2yThQvpIRnJ3iCxzs7wBcoqIzJS8iRwPuskT1Nm2mgWSCwUqsPpR92eeI4VQ6blmTJoivMJEIgQ+r7/B+9kqiLYMU5/v6tik+fDo/SpiJvgt5s2IzR3Y+1eKY1/in4ABr5YTg==",
        categories = "science",
        tags = "radioactive,nuclear,fallout,waste,atomic,physics,particle,element,molecule",
        contributors = "danielbayley,ericfennis"
    ))]
    Radiation,
    #[cfg(feature = "radio_receiver")]
    #[strum(props(
        svg = "eJxdjEsKgCAQhq8yzAVSoTBwvEGHiJTGXcjQ4/apC4l2//NzxyoMgXAZQU+nQe+GGnnXCz3/mhw3gSsFYUKjEDimnYXQIuS7RAhPM02XV91/eRa0YdV5L7rPJT0=",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[cfg(feature = "radio_tower")]
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9lyL2rE5ImgaYXz35EiUIFBSke9O/dRLCCZckGdnZm33CfHjNO2RydJLAXHghasSC8RLSpJDMOu7o5Dt/90NQwqcWhtX2tTsdB/Gool6Vcz1iysQblmQ3r/8qmhX7Un1hFsJobi4WW2F4xyBbqwgYGk7ASTklSj9YqB+tzwg1HEg/G2f9LtwirxzsSDly9bxI+SG4=",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[cfg(feature = "radio")]
    #[strum(props(
        svg = "eJxVj7EOwjAMRH/l5L0Gu7SkUtKlMx+BDBJIDKhigL/HCSJtBzuKffd0js/z64ZLotOBB8jAMgmkY4UgcECeetEYd1k5xqo/+lZ6VmuU21o9C/ZN4G5x2H22xxX2TiRKmBN5t0/5uei3XoEzFE435+FfhYsNdzF46hxy0rZk9scvkHKNt+r4ApCVORM=",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,danielbayley"
    ))]
    Radio,
    #[cfg(feature = "rail_symbol")]
    #[strum(props(
        svg = "eJxtirEJACAMwF4p7kUrVhCqH3iE4OAiOPg/0kUcnAJJZLU9oGdTGYgHBVPEqivylPQNkwJ4h4wMEaPyPgcuEReZ",
        categories = "transportation,maps",
        tags = "railway,train,track,line",
        contributors = "danielbayley"
    ))]
    RailSymbol,
    #[cfg(feature = "rainbow")]
    #[strum(props(
        svg = "eJxtzLEKgDAQA9BfCd1FL8h1OfsHfsSBg6OD/4+9IqVDScj0iD3+3riOdJKQ7LKhtmVhnVRsDVKsQw2n0KYEwqmKo+wEf7YP6gOKCxw+",
        categories = "weather",
        tags = "colors,colours,spectrum,light,prism,arc,clear,sunshine",
        contributors = "danielbayley"
    ))]
    Rainbow,
    #[cfg(feature = "rat")]
    #[strum(props(
        svg = "eJxtT7sOwjAM/BWL3Sa22ySVSucufEQVhg4MSCC+n3MGulRRHvZd7nzza/vs9Lhd7lpobIlVCrZzX2926gV5SyRVohylkq2qjV0mSlzIRXGWbwLJxMCpNMS6LPM1DJb5sMkA8bFBVlwyy0jKUkKYJQOCWhDQyKh7OxqOsQKennGb1CYGrkkJR8ajcv9lkQG3UqLIAwtn0KC+amkdTUewsxk9JOsGNDTARioeT5lktg6bkXUmPHnY1U6D07RL0j/0Az8IU9o=",
        categories = "animals",
        tags = "mouse,mice,gerbil,rodent,pet,pest,plague,disease",
        contributors = "henri42,jguddas,karsa-mistmere,danielbayley"
    ))]
    Rat,
    #[cfg(feature = "ratio")]
    #[strum(props(
        svg = "eJw9jEEOgCAMBL/S7AcUDp5aPqPEciVNhN8LKFx2k83OcI6nkcZ0qwn8DiqCA/Sky1TgPKi2HZRLr8BbBwIPrI7rhN160Ze/o0kn9gLLmx7v",
        categories = "connectivity,devices,design,photography",
        tags = "screens,rotate,rotation,aspect ratio,proportions,16:9,widescreen,4:3,responsive,mobile,desktop,monitor,orientation,portrait,landscape",
        contributors = "danielbayley"
    ))]
    Ratio,
    #[cfg(feature = "receipt")]
    #[strum(props(
        svg = "eJx1jKEOwCAMRH+lmW8GTQMIhp6ZRcyRTCAQE8u+f4dBsTTNy7W5F+/yVLq25VCSV0wTtiSEnTNLY4AR/3guKa5dm+KQW0ehsisCh4HNYLTqyP2ie5g1YfXZj88Hf6koeQ==",
        categories = "money,travel",
        tags = "bill,voucher,slip,check,counterfoil",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Receipt,
    #[cfg(feature = "rectangle_horizontal")]
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHk9IjYiIGhlaWdodD0iMTIiIHg9IjIiIHdpZHRoPSIyMCIgcng9IjIiPjwvcmVjdD64Tw94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHJ4PSIyIiBoZWlnaHQ9IjIwIiB4PSI2IiB3aWR0aD0iMTIiIHk9IjIiPjwvcmVjdD6+Lg94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,9:16,vertical,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleVertical,
    #[cfg(feature = "recycle")]
    #[strum(props(
        svg = "eJxtkEFqQzEMRK8isrdqWZItQRrorov2EIEuskihi96fjp3woclHIBsJvRnp+HP+vdDX6+FzkOS7cYifhUNppYqQIuyjcISgOMLv+dbjWq3Mgn0MluyU7IfT8WVyT8eNLpjNS3Cr+oAHhN07+LnAds+3Xp3s4Vc8rfXSWNoz/Vsw1IvSih116CYklR0GN5ukDFcVxdhhJqsZOQ4y166psBDZ3h6OQyKcuMZc67/5OebWcbe4Kqcp4W+6Z19ZR8BSVyWDVl+KsfIoq7KN/QFn4mBy",
        categories = "sustainability",
        tags = "sustainability,salvage,arrows",
        contributors = "karsa-mistmere"
    ))]
    Recycle,
    #[cfg(feature = "redo_2")]
    #[strum(props(
        svg = "eJxdjLEKgDAQQ3/l6O55V5uhcBbcuvgRgoOL4CB+v21BhxKy5CWxa7sP2md3KkgDYWhyycZKkn189UIxR8YCBlVLUygrxiNdXIrkJev0P71daBiS",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Redo2,
    #[cfg(feature = "redo_dot")]
    #[strum(props(
        svg = "eJxNi1sKgCAQRbdy8b+HCg6CuoMWIRYY+BESUbtvJIoYmDNzD9eltaayoHohBdLJUMyLSSK44dHBbXHPmL2YlAQdJnem2Zb+nIakaGEx8kjYrt3vZ6B6XTRv+qo3fI8gPw==",
        categories = "text,arrows",
        tags = "redo,history,step,over,forward",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RedoDot,
    #[cfg(feature = "redo")]
    #[strum(props(
        svg = "eJw9izsKwCAQRK8y2JtkXdhlYeMNcgghhUWKFCHnVwtlYJgPz9/yVdxnuBJBf6lRQvZ9rNnXxyAtBsPRRbA48myCtPHD3XWhDdu4FTA=",
        categories = "text,arrows",
        tags = "undo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Redo,
    #[cfg(feature = "refresh_ccw_dot")]
    #[strum(props(
        svg = "eJxtjEsKhDAQRK9SZJ/MpBtbhRhwr4eQKERwISKit/cHupFaFFQ9nhubOaItVM2gRaIo737n5t3zkIWlMkeO/xVBYrhiZN8s0aIl6i8TH6LmFdkEYtKBNZn0pUM/haFDWAtlSSFsd09HndB9+x2RMjBq",
        categories = "arrows,development,shapes",
        tags = "arrows,rotate,reload,synchronise,synchronize,circular,cycle,issue,code,coding,version control",
        contributors = ""
    ))]
    RefreshCcwDot,
    #[cfg(feature = "refresh_ccw")]
    #[strum(props(
        svg = "eJxtjlEKgCAQRK8y+K+lpiaYJ6hDCH342Ud4/tyEipCFYZhl30440pmxL2xTElIlD4+RhntevXCmScuscBNUlVVjZjEMdB3Dw9DQxWTT3XzhIPeDg+D8hlMV24FIW/NsyvvgArClLag=",
        categories = "arrows",
        tags = "arrows,rotate,reload,rerun,synchronise,synchronize,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCcw,
    #[cfg(feature = "refresh_cw_off")]
    #[strum(props(
        svg = "eJxtTzEOwjAM/IrVPaa2EyeVSiXUhQEegcrQBYkB8X4uGboQJTrZuvPZN78fn52e5+GuQuUmhXOkBLhMnBM1GNsTJVtFyEhGHo2MxcFLrWIelvlUrZb5MCwkfrVv6lDw0NUwnYQiZVbHTo8N/uUvyJ20qir0LlPZlOOEJuL2IORsHrRrV5OKbpgLNYLwlAMSwLvkvhoh9tDLodiMT3pwP/YfT40=",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle,cancel,no,stop,error,disconnect,ignore",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCwOff,
    #[cfg(feature = "refresh_cw")]
    #[strum(props(
        svg = "eJxtjs0KgCAQhF9l8b6Wmn9gnjvUQwgdPHaInr81QUJkYVhm2Pk2XOnOcK7sUCBk8uBhphHgkXZudZXqGW4XkCS7FOBYDFO5jqF1kK0enVGPsz+g1PcALAD8APSNGZQ4sjdCtOgFyMktuA==",
        categories = "arrows",
        tags = "rotate,reload,rerun,synchronise,synchronize,arrows,circular,cycle",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RefreshCw,
    #[cfg(feature = "refrigerator")]
    #[strum(props(
        svg = "eJxtjLEKgCAYhF/lcJf85dTFnFtaG9qEBseG8PkzgjCQW+7ugy+e+So4ZrU6+EwQpkVAzfLbYBVmC/seurUl9Fvbze8qxekxpth5xRThgIhDqP4DN7Z8Ie8=",
        categories = "food-beverage,home",
        tags = "frigerator,fridge,freezer,cooler,icebox,chiller,cold storage",
        contributors = "karsa-mistmere"
    ))]
    Refrigerator,
    #[cfg(feature = "regex")]
    #[strum(props(
        svg = "eJx1jLsKhTAQRH9lsN/c7FySKERrGz9CsLARLCSFX6+CT1C2WM7MYeLYTj26Mms04J/UZlX8bVkVj2ZQGh/gjENuvIf7VNTujrw4TQENLUHY7YTC2t0ZTLx4/ewfLEzC+RxeAEsvLSk=",
        categories = "text,development",
        tags = "search,text,code",
        contributors = "mittalyashu,ericfennis"
    ))]
    Regex,
    #[cfg(feature = "remove_formatting")]
    #[strum(props(
        svg = "eJxtyTEKgDAQRNGrDOnFbLKJFmtuYGsvWGwjWIjnd20kkDBT/SfXfiuOxa2MaWOl/ERXZPxykR8TgtfcAYpgzKatnZRgt3UseLOhthd8SCX1",
        categories = "text",
        tags = "text,font,typography,format,x,remove,delete,times,clear",
        contributors = "ericfennis"
    ))]
    RemoveFormatting,
    #[cfg(feature = "repeat_1")]
    #[strum(props(
        svg = "eJxtjDEKwCAQBL+y2B9xVUhjrNPkEUIKGyFF8P05AyEWsmw1w8Qr3wXnZipXOAQE0ZsUlw5S/PDhQTZhVgqrYzcLJ2rVkJO3I7OSI+jbUOrm7icm1bSF7a88Zf4tOA==",
        categories = "multimedia",
        tags = "replay",
        contributors = "ericfennis"
    ))]
    Repeat1,
    #[cfg(feature = "repeat_2")]
    #[strum(props(
        svg = "eJxtjDEKgDAUQ68S3Iv9KVaF2tnF1b3g0KXgIJ7f30EdlDcEkvDCno6MbWoKMcIZB6WJoa19DPe6iIMMc58IwipiaLj677OQkK56jPJnEvjsX5EmT7HP8wKymSTx",
        categories = "arrows,social,multimedia",
        tags = "arrows,retweet,repost,share,repeat,loop",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Repeat2,
    #[cfg(feature = "repeat")]
    #[strum(props(
        svg = "eJxtjDEKgDAQBL+ypD90k4BNTG3jIwIWaQIWkvd7JygWYdlqhklnuSqO1TUu8IiIonc5TQZyevEeQHZhUYpZRzMrB2rTkJenI6OSJxj6r2TmFj7zBt9bJUM=",
        categories = "arrows,multimedia",
        tags = "loop,arrows",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Repeat,
    #[cfg(feature = "replace_all")]
    #[strum(props(
        svg = "eJyVkN0KwjAMhV/l0PvMJatug3Vv4K330g27C0FG8eftTXuhIgWRQhrIl3OSDJdjDJic2bOF9TVxxVVPAiEx47BJ5XF4QVJDvCLQH1WvoUgJOl9DOUoMlSnegWtPWY2EkmvJ89ygRZMeNQWVJHLY5sFbdVQogYHf6Dr7iNsyxeBMZxDm5RRiTh/OsDVY786IQY7alfjPMS3Yfu18tb/X01P90fcExIFhrw==",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "eJxtzkEKwjAQBdCrfGY/NZlW20KTG7h1L2kxXQhSAurtnamgmxBISPLyf6bHtWTMgc6+Q5cc+8Y3IwuEheJ0sOs4/ZA4SFICXdGMOlWVYEgO6tgM15U/wbvEexoLW2ut896iR2uD20qKhVyO+8d7bVRkMPs/3ZZU8A7kO8L2CiSEvKy3XAINhO/Bc51Ltr2+Mh8/g/dHuA==",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Replace,
    #[cfg(feature = "reply_all")]
    #[strum(props(
        svg = "eJxlyrEKgCAURuFX+XGXUoTboM4tPYRQkCAqJEFv35VoKc54PltLulLMG2qJuR1OEBRBQ2kQSHg7vMLbn2XEmDru/qND27E6sWi+0yl1MDAYe9JIMz+ckb8Bghsl/w==",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    ReplyAll,
    #[cfg(feature = "reply")]
    #[strum(props(
        svg = "eJw1ykEKgCAQBdCrfNxLKgMmqOs2HUIoSBAdSIJunwXxts9zK3fJdQe3XPsZhIO2IGgDByuin/4RPad+YAtiNQp6vqRJNKZ6SZK00NdHig9v0Rgo",
        categories = "mail",
        tags = "email",
        contributors = "Andreto,ericfennis,mittalyashu"
    ))]
    Reply,
    #[cfg(feature = "rewind")]
    #[strum(props(
        svg = "eJxdy0sKwCAMBNCrDF6gTIqLQup1SqEYoW68vR9ciJBFZpinyb7yWESyN+b/diR4QcB2hMfILugxh0F3IdIFz07a7zGKlVTOiRxj",
        categories = "arrows,multimedia",
        tags = "music",
        contributors = "colebemis"
    ))]
    Rewind,
    #[cfg(feature = "rocket")]
    #[strum(props(
        svg = "eJxtUDsOwjAMvYrF7kfsxE0qFSQ2Fg5RlYEFCamIgdNjt8BUJbb8e362h8f4vNH1sLsUGEkHm1jCgnasZCFzRi3sQfcmVGG0gsoKyYzUu9HL6F6jRaV4SzCy791x2AfJcfhR3UVJjDPnUZX8B0JIOaO3kyhao1V/E140JW9elVEbVRh3JM6pyEarXkq5kG4QXnrvdy4zzFmRslOVSdCpr5oaBdhlA7cM+ooDJF/VjIpfIDAcaN/SQv7AD1TxScQ=",
        categories = "gaming,development",
        tags = "release,boost,launch,space,version",
        contributors = "ericfennis"
    ))]
    Rocket,
    #[cfg(feature = "rocking_chair")]
    #[strum(props(
        svg = "eJxVjmEKhTAIgK8iHqCXxl4F227QIYKCgqhB/Wi3TxeLQlHB7xNt2Ja4zOsIYZvXY3dYFQYY/lKJtTSpo7e/jHqbhMgOuUQ4pRsh4CSHrQ5Rhiy9BN2TuQVqPmA+9gihPyYYHHZc1PpETxVIlilUhgQr5i8AMDG9",
        categories = "furniture",
        tags = "chair,furniture,seat",
        contributors = "connium,ericfennis"
    ))]
    RockingChair,
    #[cfg(feature = "roller_coaster")]
    #[strum(props(
        svg = "eJxtz7sOhTAIBuBX+ePenl6Qo0n1DVzdjQ6OJhqfXzBGF9KhhA8ClG06VixdNTBiO9ZVX36a6ssLMaiwbywjsdP9bWxQn2SDdrFBKemw1pIbJgIhyIsgR3OSSFbwOYNBu6L8T1FEcNmx5++qC7M8Pi8=",
        categories = "maps",
        tags = "attraction,entertainment,amusement park,theme park,funfair",
        contributors = "karsa-mistmere"
    ))]
    RollerCoaster,
    #[cfg(feature = "rotate_3_d")]
    #[strum(props(
        svg = "eJxlTzEOwzAI/ArqDjXGGCylWTrnEVU6dKlUKf2/ComqDl04g++4Y3rd3g+4X04Ld2q9g5FeWak3gUZVDFhoaIUKnGXEbAQaBN0skOtW9yEX0CgrSatQgLoZUunxgVRP83ROq3n6Gj7DhUfL/VYMhJyjIe+Y5ej/VQvHPiW1NWmlkbtiBLXUNkE7IBhVoCAX3MPlQ7eWkTFDoq4hFY+cTm4SKmOP6xx+ph8MLT08",
        categories = "design",
        tags = "gizmo,transform,orientation,orbit",
        contributors = "lscheibel"
    ))]
    Rotate3D,
    #[cfg(feature = "rotate_ccw")]
    #[strum(props(
        svg = "eJxNjEsKgDAQQ68Sum/tx7EOjD2BHqLgoksX4vmdIkgJeQRCIle9G87NHAkhVgbDI6jZanaZPvguu7g8Iyr2hNUUmfq4yHCRHmr0Ny/fYhZa",
        categories = "arrows,design,photography",
        tags = "arrow,left,counter-clockwise,restart,reload,rerun,refresh,backup,undo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCcw,
    #[cfg(feature = "rotate_cw")]
    #[strum(props(
        svg = "eJxNTDsOgCAMvUrD3iqFqk0qJ9BDEB0YHYznty6G5eX97ap3g3MNO0eIXBUURnCKinowCbvMpMm9ieYM7LB5dwnFhm9crL9IjzSUP3sBG9YW+w==",
        categories = "arrows,design,photography",
        tags = "arrow,right,clockwise,refresh,reload,rerun,redo",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas,karsa-mistmere"
    ))]
    RotateCw,
    #[cfg(feature = "router")]
    #[strum(props(
        svg = "eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PER06RB4s+Xt+YVtuBe9IHWH7r7ys91wiecInkhrhtc4l/0xoCuf6MIVHKhlzpKtjUai/uGr1ejCVHVVaOkDlaS0Z2RtG1jEZDFLnNLBzaPV0UsW4t+Th97Qq93qIfwGWRDxR",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows")]
    #[strum(props(
        svg = "eJwti0EKgDAMBL8S8gFJvXho+xktNiAeSsDk9ybW07C7s3m0XaA3PrsUpA1hWMGEoAVXD/qFhw/pc7boa17iV/PFdwMjn9yyNKnORE763bDqCyIdHDw=",
        categories = "layout,design,text",
        tags = "split,lines,queue,series,list,vertical,horizontal",
        contributors = "danielbayley"
    ))]
    Rows,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "eJxNikEKgDAMBL+y5AMaqEKg7Q98RIiCggcpHvT3pvag7MIuzMRDzxVzoimAWQWC3sO+Qjl2Fef4k4LyCG+z3v95thXbF9iVaCDYnYiFUHyq02h+AIcgHaQ=",
        categories = "development,social",
        tags = "feed,subscribe",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Rss,
    #[cfg(feature = "ruler")]
    #[strum(props(
        svg = "eJx1jk0KgEAIRq8i7cfS+SuYOkFdoF3QokVBi+5PGrUoGkQRH0++tE/HAnNbDExogTzaidGBdiVF0hbdahiDHMMbGqtbzxihxqiM4Bq3qnwVy9zmi17uWHSp1AxdepJs5NADsQw2/MdJUJPFtZCQl6M+9x9+AsHHPTA=",
        categories = "tools,design,layout",
        tags = "measurements,centimeters,cm,millimeters,mm,metre,foot,feet,inches,units,size,length,width,height,dimensions,depth,breadth,extent,stationery",
        contributors = "Andreto,ericfennis,csandman,karsa-mistmere"
    ))]
    Ruler,
    #[cfg(feature = "russian_ruble")]
    #[strum(props(
        svg = "eJxNyrEKABAQgOFXueziCp06ZouHUIYbDfL8koH+8f94tCnQk6oBEIWaAwf2pqnEhaQym6My/9bLGxttFxJB",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    RussianRuble,
    #[cfg(feature = "sailboat")]
    #[strum(props(
        svg = "eJxti7EKgDAQQ38ldBeb85AOZ2cXf8Ct4HCjg/j9tg6KIFlCXp7t5XBsU1hEwDRLUSjindqcn6HTNWTrm5PtNQkqGCEYanOmv1fjJ8eHXAAuHlc=",
        categories = "transportation,travel",
        tags = "ship,boat,harbor,harbour,dock",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Sailboat,
    #[cfg(feature = "salad")]
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/l6G7VkpyqgjTQrUt/oFtohyyFDv1/KscQGhKMD3xPPk79Z/xOeF0Od4PwxPkw9MfqDf1CWAKNDkeejye/6f8b/tj7xqRnsIxCBfXWWU5UUiEzrFwoSZI9u1iik25stSbrbKbKqttxkzZPWRFsW/LNAQRRaW+BTA4j6a5K7pilbVzA+ZlBFrUjuDB1UWOJ+AFQYUoE",
        categories = "food-beverage,emoji",
        tags = "food,vegetarian,dish,restaurant,course,meal,side,vegetables,health",
        contributors = "kemie"
    ))]
    Salad,
    #[cfg(feature = "sandwich")]
    #[strum(props(
        svg = "eJxtjr0KwzAMhF9FZLdqyT9KwM2coX2BboYOHlroUPz8lUuIPQQNEsfdp0uf/C3wvE53B0TVZQIC+x+9CsVRMFSNm9Z0aaE1HVFioGXzh1WNzcpdaNIIa5sqjwn9ZhzOJ/x3qwaCIiai9ZmBdwijj2BvrGHa3OO0Gi6i7TQMukNRIbz0kQTwGDrLMMqMLB3yA6WWQD4=",
        categories = "food-beverage",
        tags = "food,snack,dish,restaurant,lunch,meal",
        contributors = "kemie"
    ))]
    Sandwich,
    #[cfg(feature = "satellite_dish")]
    #[strum(props(
        svg = "eJxtjTEKgDAQBL+ypI9mjV4QYsAH5AN2AQsbwcL/o4kEC+WObXaY9Uc6N6yTij1okmssUcKUY/5FBd9mLvhK7yM4wGr7rSIdaJNAHocWLT9Ux5uai79uEa/uAj2lJUE=",
        categories = "connectivity,devices,multimedia",
        tags = "antenna,receiver,dish aerial,saucer",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SatelliteDish,
    #[cfg(feature = "satellite")]
    #[strum(props(
        svg = "eJxtzEEKgCAQheGrPNxLjdqoYN7AC7QTWrQoaNH9yQlqkwyz+j9eOuu1YZ1VIQuPCIsJfndwKqdBYk4vOciDCK3p53XPBJARAtYsZOntMAKstv9UIgxVBmOUaxP8oRubdSuF",
        categories = "connectivity,science",
        tags = "space station,orbit,transmitter",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Satellite,
    #[cfg(feature = "save_all")]
    #[strum(props(
        svg = "eJxtjS0LgEAQRP/KYF+9HRbdcJotVoNNMFwwGOR+v1r8AJk2D96L27wnLG0x1LCZIMI5BYVJw2qwrKHkQ4TQ0nt/P8Kp6GJ1qbp4CzWA2VL9hxzqWZoknpt/Tvb2bYyP6QC/jS1m",
        categories = "text,files",
        tags = "floppy disks,copy",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    SaveAll,
    #[cfg(feature = "save")]
    #[strum(props(
        svg = "eJxtzEELQEAUBOC/Mu1dzNOGWs4uru6Koja2SPHrLTms0qupqe+Ncd02oi9VwwLCWncCQeKPkUTSBh2+j6TV0DsZOsipKhPfU5Vxiz3sNA9wyzRva6mY+WH4ZIo3hI9/5c9PhpvloEb+oRcebS6u",
        categories = "text,files",
        tags = "floppy disk",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Save,
    #[cfg(feature = "scale_3_d")]
    #[strum(props(
        svg = "eJxNylEKgCAQhOGrDPsesYKFoN6gQ8QWGBSEROTtyyLrceb7rUxR5hGSHLEhREeKIMe9vK0f9vbNLtClSnn8orXfAgZHnUa7swqssub3s0WDDZqqKXQCrFolLw==",
        categories = "design",
        tags = "gizmo,transform,size",
        contributors = "lscheibel,ericfennis,jguddas"
    ))]
    Scale3D,
    #[cfg(feature = "scale")]
    #[strum(props(
        svg = "eJyljbEKg0AQRH9lsN+Ns8t5F7j4B/mBdMEU1wQExe93bazshGGaYd6r83dt+L26PwdEXAocZRItWYck1KeB4uAipnRRT+LCTzfWx/Ed60mwm4B3hrGxv1ho8I3lYnHkZpOhR7iQxRA6pOigNTsfO3vQOMw=",
        categories = "maps",
        tags = "balance,legal,license,right,rule,law",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Scale,
    #[cfg(feature = "scaling")]
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+yoSfmODFcclLT+AgTiystzL1fbaCh3dkZvc/HcO3hSASGgHKouvxr1c4ogRs7FaPiUWaPDWzZp/L6RZt4HPAFjH8g1Q==",
        categories = "design",
        tags = "scale,resize,design",
        contributors = "karsa-mistmere"
    ))]
    Scaling,
    #[cfg(feature = "scan_face")]
    #[strum(props(
        svg = "eJxtz7EKg0AQBNBfGezXuKPHRbhYp0mbXrDYUlDu+z0t5JBlGViY10xa592wfJpfj/gPM0F05RQUGpspvU4xpdtpRG+sIZg9SIXGXEkpn4lHI6jfUEthduUbOmzahiKHM3LFgSNGazv1BoRHdQBC+EI5",
        categories = "devices,social",
        tags = "face,biometric,authentication,2fa,dashed",
        contributors = "karsa-mistmere"
    ))]
    ScanFace,
    #[cfg(feature = "scan_line")]
    #[strum(props(
        svg = "eJxtyrEKwzAMBNBfObSbVheCFttzl67dAy24UEqGYJK/j5wheAgaTne8OE9LwTvJc4C9xokg7n4KBhZKjrcmcjydGobCHoL1ClKhVjsZ/Cvhihqoj7GXgbWXv+//g41JlIJVk5hHq56bHrPbpvIOrUI0bw==",
        categories = "devices,social",
        tags = "qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    ScanLine,
    #[cfg(feature = "scan")]
    #[strum(props(
        svg = "eJxtyiEOgDAQRNGrTOob2GmaNaUag8U3QaxEkJ6fYsiK5psvXrnbY7i2cCTomRtBrCMBI42hluUTtfxOFMnoIdhnkALR7mQcZ3FGFZQ9exnZnXwBlDYpaw==",
        categories = "devices,social",
        tags = "qr-code,dashed",
        contributors = "wojtekmaj,ericfennis"
    ))]
    Scan,
    #[cfg(feature = "scatter_chart")]
    #[strum(props(
        svg = "eJx9zlEKgCAMBuCrjB2gGGX6oN6gQ4QFBj2ERNTtS4cFIb39bN9+pt0c3DJBMFgJBHcalCkcHKyuWVj9kTcglW9EkSZD2XDi84KNonvFzwepVj61bbF2HTYPo8G+gWYn5UnFbZzaC0G+Q20=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[cfg(feature = "school_2")]
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRh7P9dKeL6bak+QrL4NhBepqI6cuU3Bi3XzzG+/TIcBncUQSETn1GP2NAjwH8QZRlsj/QZ2zLHOoHyrmElZAqKgDvdUO8jnYNxH3bVljT50FEsfv3QuulVN8g2v2sb/KNSRc=",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "eJxtjsEKwkAMRH9lyH1xE2KNsLtnL35EWQWFClJk0b83VtCiJZdM3syQdO1vJxwyXRQdLCgMSiWtXveSPpQNHKGQZr1AEH04+LbTuQ7Sgg0a5L9iz552/PXHyR/n2vt1KWpYN94skO4H1PNYhyPqI9OWMGYSQr1n4umjNy1PN58++Q==",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "eJx9jk0KgzAUhK8yvL1p3yOGFJLcwEOUtGChhSIu9PYm/qCouJnNN/Mx7v9sa7w8VaXSgkcOCxYK7pZRcPHTxO8bsfdkCY0nIcTOk86VCQa3WH6sYQqrSoscB8s8Ty42F5qK7yq90GOAN561YdLLupATJLJDA9Q8Phs=",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors_square_dashed_bottom")]
    #[strum(props(
        svg = "eJxtTUEKAjEM/MqQe9dNqKVCu2cvXr0vVaigIIuI/t7U4tpDaRg6mcxMuM+PjFOkg4XILBCM+tiIkaP9cyjP7NoF5NkujP5oCpsSOIU1lkfN3fue4lTJpjGly5KuZ6R3JE9YIgkhvQrRkyo2/t1g2aIii06vww4eFcp0q9j9Wmplp+umbmf8sPUOX1yDPoxGUcE=",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[cfg(feature = "scissors_square")]
    #[strum(props(
        svg = "eJxtTW0KQiEQvMqw/7N8mBj4vEGHiH2SQkGIUO/26Rp9QLDMwHzs+BK5IsV8TnWmaUd4NCKsgve81DTkInrw214InnPhSwQ31RF4FSqvyDCDv51qwjLT8aCMNhiop3Y91d2vjDbKYUC/T+J3SjZkUNt/W9fWthun9s5C8P3oCdAvP6U=",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "eJxtjUEKgDAMBL8S8oBqa5Uc2v7AR0gUFDxI8aC/17RFPQhhIeyw43iJvE7Ah8cOIXpsEPiUJ7gql8Ftwz7D6LEnpQ2kSCeMdB/C1GAL0SqilyiiYsg6EWn6NWmrCFLci6Z+di6m7i86",
        categories = "text,design,tools",
        tags = "cut,snip,chop,stationery,crafts",
        contributors = "colebemis,ericfennis"
    ))]
    Scissors,
    #[cfg(feature = "screen_share_off")]
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+ysSdydyAUSG3jI0gsaEwsDO8XGiWRbLGbnWTCle6MY512EshmEoOhW1RdhfR31OZMS38oLkqmGOZmieF1eTBlPwDEIFfMn5zMEGVhB4gcBD16ALRLLkM=",
        categories = "connectivity,devices,communication",
        tags = "desktop,disconnect,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShareOff,
    #[cfg(feature = "screen_share")]
    #[strum(props(
        svg = "eJxtjDEOgCAUQ6/SuBP5HxAGZHbxECQOLCYO5p9fWMRE0qFNX9p45bvgWKedDMxmM4Ohm1RNQroX1bnQ8i0UizJTinN7SfH9CmAqYQCIQV7sn5zkEeCUG408THHS0QO6wy7K",
        categories = "connectivity,devices,communication",
        tags = "host,desktop,monitor",
        contributors = "csandman,ericfennis,johnletey"
    ))]
    ScreenShare,
    #[cfg(feature = "scroll_text")]
    #[strum(props(
        svg = "eJxtjLEOgCAMRH+lYW9sK0RMkJnF1Z3EoaOD4fsFE4kDuenuXl648q1wbmb3IKwsWUCA3ghKQUlM5VsZGC3Q4Xqn1sus1sQwNVcM3cgr8NLZGqzGNCQdeEU3flj+1wOvSiwV",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "danielbayley"
    ))]
    ScrollText,
    #[cfg(feature = "scroll")]
    #[strum(props(
        svg = "eJxFjDEKgDAQBL9ypD+8WyMqxNRpbO0DFiktJO83ETzZapZhwpXvQufm9oWgRZFBIHkHRmUklfq9Ssqe5JiMpXMdi3cxDL0VgxV1JZ3NbeNWTL/5AA4dHTw=",
        categories = "gaming,development,text",
        tags = "paper,log,scripture,document,notes,parchment,list,long,script,story,code,coding",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    Scroll,
    #[cfg(feature = "search_check")]
    #[strum(props(
        svg = "eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf29qoSzLHmYZvuZ7g0XwzEAE0ZNCQuWxAmXbix0rFMGMYI8gke/7r58aVu6W6A4KaZhqu+YDh90baw==",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick",
        contributors = ""
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "eJxNjFEKgCAQRK8y7L/Frn0kqHcJCwoKQvqo26cGJrPLwLxh7DldK2ZHh4FRgiLyts+5t5WyRjpRRT8PWwz7guhoJITHEXPyu3gqfbiZEYawGjqdv868RnEjhQ==",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = ""
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "eJw9jEEKgDAMBL8Scm8lrYEe0vxFoqCgIMWD/t5WoYdlDrOMnNO1wpzxoOgZkmfHwKgyNKFiW7F9gZIxIdiTkajy/lhPv1bplUAQyI0+tvXMC5TzG5E=",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort,/",
        contributors = ""
    ))]
    SearchSlash,
    #[cfg(feature = "search_x")]
    #[strum(props(
        svg = "eJxVzMEKgCAQBNBfGfausdqCB/VfYgsKCkI61N+HCFKHYQ5vmHhO14o50cHeCoIVIxDKcaiQY+fQFD/Vrei+QJ9EzAS9W5dEoY4af04cw7EZra/pNy88RCOA",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,stop,clear,cancel,abort",
        contributors = ""
    ))]
    SearchX,
    #[cfg(feature = "search")]
    #[strum(props(
        svg = "eJwlijEKgDAMAL8SsqukOjgk+YtEQUFBSof2923a4bjhju2J9l4QBXcEy4JEzaVbeRlZ+T/SDafgFwgCTdu8On540QrMxBNp",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass",
        contributors = "colebemis,ericfennis"
    ))]
    Search,
    #[cfg(feature = "send_horizontal")]
    #[strum(props(
        svg = "eJw9irEJACAMwF4p7iJVEAq1H/iAm+DQRXDwf7SDEjIlvPpWGMXNBAb5KyB5ak44WBV+T82AUTH/cgD7URB8",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "danielbayley"
    ))]
    SendHorizontal,
    #[cfg(feature = "send_to_back")]
    #[strum(props(
        svg = "eJxNy9EJgDAMRdFVHllAUwr60bqBQ4iK8U+kqN1eU8WWQAjhXLfPY0D0xJZwrlMQTy1B5nWRkM798mQIVyKdqzToXMqy/1CRvY+o+4+2IQgmT30DtgcPBgZ1mucSVqikgGzRSIas8MjwBsuNMyw=",
        categories = "design,layout",
        tags = "bring,send,move,under,back,backwards,overlap,layer,order",
        contributors = "james-yeoman,jguddas"
    ))]
    SendToBack,
    #[cfg(feature = "send")]
    #[strum(props(
        svg = "eJw9yrEJACEQBdFWPubL3e4Jh7DagQ2YCQYmgoH9owbKhPO051FRvGkiEPohL1lyK5tM0GfvoAfFjcAM/u6cOUoQ/w==",
        categories = "mail,communication,connectivity",
        tags = "email,message,mail,paper airplane,paper aeroplane,submit",
        contributors = "colebemis,ericfennis"
    ))]
    Send,
    #[cfg(feature = "separator_horizontal")]
    #[strum(props(
        svg = "eJxty1EKgCAQBNCrDHuB2i3ED/U2fQiiQn3k7dsV+giCgYHhTSi5HhgciYVwa29aEkmYMGTOKSymUuitjOl7y/U6I3l4sGAHO3hzr/ixSiwCWe3lPvwB+BQmhg==",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "eJxtyk0KgCAQBeCrPOYCNVOIC/U2LQRRoRZ6+8aoReDq/fC5FPOBxp5YCF08CROavFv/jYJbhgqultQfX0vM1+nJwmIHiwab4T4xsWzUQNbBtdgfvwH3siaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[cfg(feature = "server_cog")]
    #[strum(props(
        svg = "eJx1kMEKwjAQRH9lyX3XbJLWFFrPXrx6L1GIoCBFiv69u9RDKik5hGRm3mTTp9uU7ldIn8GwM5Deyz4NxptDv1vkQ/8cXxkugzkFaoDtMYwOHFhZjA7duTiDnDO35QW4eRUAl5Ea5Sv3nx4KusV12CqspFutm3GVwC18C20my1WFY116cEN7YE8BqUPyFUdHTv5E1Q2DiFIgHPLiqRn8z4BSw3VChE7jdT2ShwUijlo+6BAK2TLIgBC1wBcv/AKAfocs",
        categories = "development,devices",
        tags = "cloud,storage,computing,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    ServerCog,
    #[cfg(feature = "server_crash")]
    #[strum(props(
        svg = "eJx1jbsKgDAMRX8luEeTWIJDdXZxdS84dFBwkH6/7SIpVDKEPM65/g5PhGPuNgWm1QUBAcrFKCi7mSHPkdUuQFIFgESUbvFDkS7eqp1RE9YkFZNVU8lKWBH469bYE7dTp/bp4hEUXSb1LO37eAFErUAW",
        categories = "development,devices",
        tags = "cloud,storage,problem,error",
        contributors = "mittalyashu,ericfennis"
    ))]
    ServerCrash,
    #[cfg(feature = "server_off")]
    #[strum(props(
        svg = "eJxtT70KAjEMfpVwe2oTm16HeouLiw9x4JBBwUE6+PQmHlwVSijNz/eT1Of6UridpusMrHRcGRiiBdnPLfUaLVOUaakH5yx1Z5JNI3AQf2cn8FaBtC4Yvav5PRBgBpobUscio5nRAJuA0i/ub8nNg3KQO2EQDyxYLmnkmoGKhjgweXwvMLG4zz5i60de",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "yukosgiti,ericfennis,csandman"
    ))]
    ServerOff,
    #[cfg(feature = "server")]
    #[strum(props(
        svg = "eJx1jVEKgCAMQK8iO0A5iehDvUxJCtGHCOntmwuLiL50b+8xHd2cRDGgQHgXVp8MTCAyg3jxIyzJ008S4YXVfc2s5jj+1J+snsHhqbewO5HRwEiJoqeTSJJiUJiTW63mviSsl/CWaWz2CXeMOLI=",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "eJxFizsKgDAQBa+ybC8aPyxCNrWNh5BViGAhwUJv7yaBpBp4M8/e2+NhZ1z7Dsg3Mzrbxs3ZYswIhpapGjmDXAcExgFBXkZDyi9Ro6xLFgXlTpFONfoBH38kWg==",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGdmiXpP+X/G13PdzPcOzdjoVEQM5IIRx0A038UHcP4s0vwsjAtPaT1nmStkjoqlFukZpNqaDV+YAI+SIDGoWYnfQQ8aIECaHcli03JAI12wKwyFjAjLZNcVwnO0DG+r00pYRaUjkCgvLAypLYXMIEj7ViwsR+bVf47RfJm1GSX/8+KiJCgqJbSw7fAbe1t7NSotgbwZlSQ/Id2jky2++rh4HydkO3ig9o6MbLbZxOMD57x+Lg1jvvYHylPy3K6eEDOid/Ow==",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "eJxVjVEKwjAMhq8S8r7YrNu6QdsTzAv4NrpiCz5IKehub6oISiD5Q76P2PtWE+wOzzNpYLWRIQNKijua+qljUmZZmWkA/XMEJl7mjtSgV55EnWn5c0fTN+aC3p7aD29LDBUeea/JoUFIMV9TfcfydMgIh/QBQRbdrMZ7G3IJtwihIYZGhHB8UxFQpqAfyL8AF9AyPQ==",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = ""
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "eJxtjUsOgCAMRK/S9ABoVfwk4GWICxPighXc3lZAXLhr5s2bGncG5w9w0SKtCC5Z1AjB4oi76TLdTW0xpaHgx5l/Wo3WRdq+NX9eB0SyuCq9IaSBC4ua+IxyajXxi8ScRqVJTDGKJ/kscfb618tjMisLTbsBlYQ7dw==",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "eJxNjFEKgCAQRK+y+B/lUuKHeYMOIRQoiAqJ6O1brY/Yj9kZ3oxKJls4d3aswLFIg4CwjKPPcvwHE5ZJMq3mXtIqRd+8Cxek6EK+d8YFCFqhggQxuI/QanCVE4MMKr7ayHfpduv8YB/uRSZQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "eJxtjFEKgCAQRK+y7AVijaBAvUxJCtGHCOntc02Foq9lZ9486c0awEeFAuFyW7AKaUbIwYjgU8mtcbsNT5G40HLgnZaHOw0kUrjkRpQTqUxj/gQxyUwla8ckTZ0pgvx/WNGtjan25cfKuobGqk8v9gZ4wTyd",
        categories = "text,files",
        tags = "spreadsheets,table,excel",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Sheet,
    #[cfg(feature = "shell")]
    #[strum(props(
        svg = "eJwtTkkOgCAM/ErDHXRKo5ggP/ARJB48evD/sS2kzXSbybS+/XvoPsMFIaAzMa0EQhStQoY2F8VNw6cIIxVfOtn28Bxa9gbpyBPHgZNw3BObybShnFStBwmtLvZM+wELIxuz",
        categories = "animals,development,nature,science,travel,food-beverage,home",
        tags = "beach,sand,holiday,sealife,fossil,ammonite,biology,ocean,terminal,command line,session,bash,zsh,roll,wrap,chewing gum,bubble gum,sweet,sugar,hosepipe,carpet,string,spiral,spinner,hypnotise,hypnosis",
        contributors = "danielbayley"
    ))]
    Shell,
    #[cfg(feature = "shield_alert")]
    #[strum(props(
        svg = "eJxtyyEOgDAMRuGr/JkvtNsYFWU3wOIJiAkECWTnJyAIAvPMy2f7fBasgxvFw/tDKUJJeOo2UgqkCLVfGAkK4ScuW3urbF+rNf4PSaVhed8FUzEdSQ==",
        categories = "account,security,development,notifications,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,warning,emergency,attention,urgent,alarm,crest,bravery,strength,tough,attacked,damaged,injured,hit,expired,disabled,inactive,error,exclamation mark,!",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldAlert,
    #[cfg(feature = "shield_ban")]
    #[strum(props(
        svg = "eJw9yzEKgDAMRuGr/HQPNmmqGWJv4OouOjgoCIrnFwW7vOXx+TFdK5Y+DCwQOY0URhzHvJFRIkO6uzmihYHjl1C8eVXx3+6KDFaw1PcAweEV5Q==",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,uncertified,cancel,error,crest,bravery,attacked,damaged,injured,hit,expired,eliminated,disabled,inactive,/",
        contributors = "danielbayley"
    ))]
    ShieldBan,
    #[cfg(feature = "shield_check")]
    #[strum(props(
        svg = "eJw9yzEKgDAQRNGrDOkXk82qK6y5ga29aGGhICieXyMYBqb5PDuma8XSuyEwmE8lgVLwY72RUiRFvNvZo4Ei+O9csiqrZL/dO2T9TkhKfgABKBZp",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secured,safety,protection,protected,guardian,guarded,armored,armoured,defense,defence,defended,blocked,threat,prevention,prevented,antivirus,vigilance,vigilant,active,activated,enabled,detection,scanned,found,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audited,admin,verification,verified,certification,certified,tested,passed,qualified,cleared,cleaned,disinfected,uninfected,task,completed,todo,done,ticked,checked,crest,bravery",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldCheck,
    #[cfg(feature = "shield_ellipsis")]
    #[strum(props(
        svg = "eJx1jCEOgDAQBL+yqT+4a6GcKP0BFk9AVCBIILyfgkAQalbsZCZs05Gw9GYQC2t3pQZKwmO7kpIjhTu7meGhEH7GxFDfVgyvm29JFcsPytky8192ATAQJT8=",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,securing,protecting,guarding,armoring,armouring,defending,blocking,preventing,antivirus,detecting,scanning,finding,auditing,admin,verifying,crest,upgrading,loader,loading,throbber,progress,dots,more,etc,...,…",
        contributors = "danielbayley"
    ))]
    ShieldEllipsis,
    #[cfg(feature = "shield_half")]
    #[strum(props(
        svg = "eJxVy6sOgDAMRuFX+TPf0HZcKsreADtPQCAQJBCenzCxZOaYk8+v9Tmwz2ERhept1MNIOA8nGUUyxHfaGCMMwiUhefer5K3NWs8HmPIVow==",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,logo,sigil,flag,team,faction,fraternity,university,college,academy,school,education,uniform,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,ranking,army,cadet,scout",
        contributors = "danielbayley"
    ))]
    ShieldHalf,
    #[cfg(feature = "shield_minus")]
    #[strum(props(
        svg = "eJw9iysOgDAQBa/yUr+hu+XzxNIbYPEERAWCBML5+YiaETMZP5arYBvDpAazk9KConHudqEkIdI9rBE9CI0/Qvbmu7LX99VaWMMDhacVjg==",
        categories = "account,security,development,gaming",
        tags = "unshield,cybersecurity,unsecure,unguard,unblock,antivirus,clean,clear,disinfect,patch,fix,stop,cancel,remove,relax,admin,crest,bravery,weakened,damaged,hit,unarm,disable,deactivate,decommission,downgraded,minimum,-",
        contributors = "danielbayley"
    ))]
    ShieldMinus,
    #[cfg(feature = "shield_off")]
    #[strum(props(
        svg = "eJxlTbsOgzAQ+xWL/a6XuyQEKeUPWLtH7dChlRgQ308IggX5sVi281yWLz7PbnID93C+RB6wSxrYSF/hR4mMjBWOtRvzY2+N+ez+FRVSec8mX2ebENb+LYhIcNKsqLCh2fEWOCp5tnTtbO3nJUE=",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,interception,threat,prevention,unprevented,antivirus,detection,undetected,exploit,vulnerability,vulnerable,weakness,infected,infection,comprimised,data leak,unaudited,admin,verification,unverified,inactive,cancelled,error,crest,bravery,damaged,injured,hit,expired,eliminated",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShieldOff,
    #[cfg(feature = "shield_plus")]
    #[strum(props(
        svg = "eJxtjDEOgCAQBL+yob/IgcgVJz+wpTdaUFiYaHy/aEFFs8VOZvRc74J9Ngs7OHcJjRBim8NBQp4E/ombxQQB239M0uGzkja33lykA2qUQ46NvBjBHNM=",
        categories = "account,security,development,gaming",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,extra,added,professional,enterprise,full,maximum,upgraded,ultra,activate,enable,audit,admin,verification,crest,medic,+",
        contributors = "danielbayley"
    ))]
    ShieldPlus,
    #[cfg(feature = "shield_question")]
    #[strum(props(
        svg = "eJxtjb0KgDAQg18luF+9u/pzQvUNXN1Fhw4OguLz2zp0kpAMCR8J53pH7GM1i0L1MmpgJLy0Bxl5Mvin3xgdDMJfVFOoMzWFwg5OMKweHpwkaJ0pJGFKqcz+gdKh9NGxlO0FX0MiNw==",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,threat,prevention,unprevented,antivirus,vigilance,vigilant,detection,undetected,scan,find,exploit,vulnerability,vulnerable,weakness,infection,comprimised,data leak,audit,admin,verification,unverified,uncertified,uncertain,unknown,inactive,crest,question mark,?",
        contributors = "danielbayley,jguddas"
    ))]
    ShieldQuestion,
    #[cfg(feature = "shield_x")]
    #[strum(props(
        svg = "eJxtyzEKgDAQBdGrfNKvZpOsbiDmBrb2ooWFgqB4fokgWNhM85i0j+eCuTM9Ozh3KAUosR1kJSVPCn+1k0UDBdsnJqe6XDm978ahEkQSyA/GYvjaDbTxHbo=",
        categories = "account,security,development,gaming",
        tags = "unshielded,cybersecurity,insecure,unsecured,safety,unsafe,protection,unprotected,guardian,unguarded,unarmored,unarmoured,defenseless,defenceless,undefended,defender,blocked,stopped,intercepted,interception,saved,thwarted,threat,prevention,prevented,antivirus,vigilance,vigilant,detection,detected,scanned,found,exploit,vulnerability,vulnerable,weakness,infection,infected,comprimised,data leak,audited,admin,verification,unverified,inactive,cancel,error,wrong,false,crest,bravery,attacked,damaged,injured,hit,dead,deceased,expired,eliminated,exterminated",
        contributors = "mittalyashu,ericfennis"
    ))]
    ShieldX,
    #[cfg(feature = "shield")]
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik0xMiAyMnM4LTQgOC0xMFY1bC04LTMtOCAzdjdjMCA2IDggMTAgOCAxMCI+PC9wYXRoPsBaDmU=",
        categories = "account,security,development,gaming,shapes",
        tags = "cybersecurity,secure,safety,protection,guardian,armored,armoured,defense,defence,defender,block,threat,prevention,antivirus,vigilance,vigilant,detection,scan,find,strength,strong,tough,invincible,invincibility,invulnerable,undamaged,audit,admin,verification,crest,bravery,knight,foot soldier,infantry,trooper,pawn,battle,war,military,army,cadet,scout",
        contributors = "colebemis"
    ))]
    Shield,
    #[cfg(feature = "ship_wheel")]
    #[strum(props(
        svg = "eJx9kMsKwjAQRX9lyD7RuXWIgaRrN926lyhUUJAion9vkhaUvjYzMGfuPK6P1y7eLtQFtVcU30ExUv6UXPtNj2v/OD1bOgfVMAgvayTDXPyhOzsSLQYV5TBtaABitHpRza7I9YI8readkSMwB7d5MVfGWpI0aabFmQRwWFGXKPR33mDPyJhsF/ovBoO++AVX0A==",
        categories = "transportation,travel,maps",
        tags = "steering,rudder,boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "danielbayley"
    ))]
    ShipWheel,
    #[cfg(feature = "ship")]
    #[strum(props(
        svg = "eJyVT7sKwzAM/JUju1VLVuII3EC3Ll27h3To0KFD8ffXdiBQ8FKEDj3vpPReP088zsNNILzRRCOYBAypUUNf0QmqM4WSM1mp7xP/rwxLOlXZJR3ibBRmiL8w04QGvpkwWF/OnDqDbpXXyBQjBS3xzIgUpy4jONzjWt7aqZw4uf7kkNzdLCf7rP2O5HA0vqsaRnA=",
        categories = "transportation,travel,maps",
        tags = "boat,knots,nautical mile,maritime,sailing,yacht,cruise,ocean liner,tanker,vessel,navy,trip",
        contributors = "karsa-mistmere"
    ))]
    Ship,
    #[cfg(feature = "shirt")]
    #[strum(props(
        svg = "eJxVjrEOwjAMRH/l1N3GdtyQSKUzA6zsEQwZGBgQA19PEoaq8ubTe3fLq7wrHqfpasIhIbBHaIQVh0PaKSXIJXC0ERaDjb+QcnAYW3jyPMhjUeg/BOfMyc/xo3JvElbO6KjVtClgZDeVaqzznqUGdy0N7b6Teud3WpdDH7/+AGDDKHk=",
        categories = "shopping",
        tags = "t-shirt,shopping,store,clothing,clothes",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    Shirt,
    #[cfg(feature = "shopping_bag")]
    #[strum(props(
        svg = "eJxtjDsKgDAQRK8ypF/MJsuSIuYGthZ2AYsUFhbi+V1TBASZZpjPy2e9GvbZLYqACL1ZajDru8y1b0Bh1YMiyeZKnt53yYNh/8bpp2AF+yqQDmFK8GP1ACINICo=",
        categories = "shopping",
        tags = "ecommerce,cart,purchase,store",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ShoppingBag,
    #[cfg(feature = "shopping_basket")]
    #[strum(props(
        svg = "eJxtjrEOwzAIRH8FZecaqF0Hyc0f9COidvDQSh3y/wr2EGVATNzjjqv/bW/0eU6/TCKUuExrvXVxrScSc8Yhe6mjpnPgumNECh5UkDYlpXmMdq0ZljfMV8HCKKTs6ldQ2K+DOBthZEGH1D9l5CY5at9r8NV5AFOAPuI=",
        categories = "shopping",
        tags = "cart,e-commerce,store,purchase,products,items,ingredients",
        contributors = "danielbayley"
    ))]
    ShoppingBasket,
    #[cfg(feature = "shopping_cart")]
    #[strum(props(
        svg = "eJxNjUsKAjEQRK/S9H7a6XLygyRrNx5iiEKELGRwobfXqAOhoRf1HlWx3LbSrlSeiT1TeSWGMm2JlXM8/GiOg6Xhj3d50O7ro9Il8RkyG+qvokGsJYUsWEGg+XsgFeNrEOeHVCWY6QNcU7FmcrIcT0YUfaN35zf/nyub",
        categories = "shopping",
        tags = "trolley,cart,basket,e-commerce,store,purchase,products,items,ingredients",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ShoppingCart,
    #[cfg(feature = "shovel")]
    #[strum(props(
        svg = "eJxljDsKgDAQRK+y2GfdbFw/EHMCPUTAwkJBUCw8vdkUKZSBx8B8/BGvFZaxmhmYbyObGIEk5VMFX2sh+FIbUMA2ihb6f77bDjjvUVCiQ3GQQUk2ndJJ2dM3AzIy6bqcvtw6JLk=",
        categories = "nature,tools,gaming",
        tags = "dig,spade,treasure",
        contributors = "Andreto,ericfennis"
    ))]
    Shovel,
    #[cfg(feature = "shower_head")]
    #[strum(props(
        svg = "eJx1zjEKgDAMBdCrfNyt/W3aIlRv4CEEBxfBQTy/reAgNHySIS+E5HO9dmxTdwgEzoRa3ZyHOp/zpwt9oWjCKmYMeJut6RNSaz2ghs1TAqbbWLbMglEzD3rNImg1I5xqCRTFXPmFP3sAkGFQiA==",
        categories = "home,travel",
        tags = "shower,bath,bathroom,amenities,services",
        contributors = "karsa-mistmere"
    ))]
    ShowerHead,
    #[cfg(feature = "shrink")]
    #[strum(props(
        svg = "eJxtzT0LgCAQgOG/crRrmh94YM4tre5Bg4NCQ/T7u4NyEvSG4+G9eB13gXOdmnZAz4Nvwgv/WBmaEjQL/SnFmWGKP98RNMqQtWsK1GblwhtXBQUGmspEMjLmIC+w0p1hudOvi2DAdPkCPSQt8A==",
        categories = "layout,arrows",
        tags = "scale,fullscreen",
        contributors = "mittalyashu,ericfennis"
    ))]
    Shrink,
    #[cfg(feature = "shrub")]
    #[strum(props(
        svg = "eJxljbEKgDAMRH/lcG9totYOVejm4g+4FRw6KDhIv98UUQcJSTjeceePeCasQzUTgzmrflOsuBp9XcjoP97DZe2ChYWRIVCjHdhkM5GcYHWHsjcUdzKxw6NJ3vKP3akFtUq6X3YBbJUjoQ==",
        categories = "nature",
        tags = "forest,undergrowth,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Shrub,
    #[cfg(feature = "shuffle")]
    #[strum(props(
        svg = "eJxtjrEOwzAIRH8FZedasE1syc3cpR9RpUOGVOrQ/1ewo0zJgAQc3L36e/8X+jyGl5LkRRBnQaA7KRLDKCCwYFwNwhk2Y/RRSNvyEJ+qw1RvzWmqh99XMilFiux1lj3OPK14WuppBcX9zDu9uu50nPyBOx8rjB1mR8grN9x8jSHxxLEBS9c7Pw==",
        categories = "multimedia,arrows",
        tags = "music,random,reorder",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Shuffle,
    #[cfg(feature = "sigma_square")]
    #[strum(props(
        svg = "eJwdizsKwCAQRK+ybK/BfBXUOk3a9CFKVkgRRPK5fdRihhl4T0e/J3gNdghf7ZhPi0A+HJQMConwBJeoTqubIlh9bYnAGVzECJKrdZrl2cPAckjeTHBV2ELZH1t0Gws=",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
        contributors = "danielbayley"
    ))]
    SigmaSquare,
    #[cfg(feature = "sigma")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNbRQMA8z8TDLMVOw0AXiDEOjMl1jJTsbfZASOwDmBwru",
        categories = "text,maths",
        tags = "sum,calculate,formula,maths,enumeration,enumerate",
        contributors = "mittalyashu,johnletey,ericfennis"
    ))]
    Sigma,
    #[cfg(feature = "signal_high")]
    #[strum(props(
        svg = "eJxtyzkOABAQQNGriN42kVAMN9DqJQqlQpzfUtBM+18+9jIaq4EnYKCb1IZHVCdGfOQ2TWEJMXDJU3Su/GUBNKMdsA==",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalHigh,
    #[cfg(feature = "signal_low")]
    #[strum(props(
        svg = "eJw9yTEOABAMAMCvNN1RjcRS/YFHSAwdDeL9WLreyRrbYDbsDEwWKaNK+qjiVV+dUHwuj+oO2g==",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis,danielbayley,karsa-mistmere"
    ))]
    SignalLow,
    #[cfg(feature = "signal_medium")]
    #[strum(props(
        svg = "eJxtyakNACAMAMBVCJ6vIQFRugFDkCAqEYT5eUQV9g5Hm6x60RUUeLY+aEJ3kVAqnVomfibAqyy1AYslFmk=",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    SignalMedium,
    #[cfg(feature = "signal_zero")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVIwMsjQMzBUsrPRBwnaAQBpcAd7",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g,lost",
        contributors = "ericfennis,danielbayley,karsa-mistmere,azdle"
    ))]
    SignalZero,
    #[cfg(feature = "signal")]
    #[strum(props(
        svg = "eJxtyysOwCAQRdGtTPBtmRfSVkzZQS2eBIFEENbPR6DG3pMrJdZM6TM/CDaflo2Xa0Yvm55B7XCKMBa9Gs0raAKQa3xv6ZzTJRQ=",
        categories = "connectivity",
        tags = "connection,wireless,gsm,phone,2g,3g,4g,5g",
        contributors = "ericfennis"
    ))]
    Signal,
    #[cfg(feature = "siren")]
    #[strum(props(
        svg = "eJx1jbEKwzAMRH/lyG7XOiI7ATdzlv5AN0MHL4UORd9fu5TQgos4gXT3uPwoz4rbebokCItCEdoI1KmF7xtqcU/m4nXa8qljWz5gBUMh+AnTscrPAzTuao4jmtK6qwwcWbxibpIFOiL/gr1xYNxnv3LFe/sUUtcYF1o8nBebI0wl",
        categories = "medical",
        tags = "police,ambulance,emergency,security,alert,alarm,light",
        contributors = "karsa-mistmere"
    ))]
    Siren,
    #[cfg(feature = "skip_back")]
    #[strum(props(
        svg = "eJwtykEKwCAMRNGrDLlAq7QLwXqbUgRJhHZhbt8YXP0ZeLlL00cYXSp/70UhIe5ICBE2D/inkrcFS26Vb2iYlDCsJ0GjZ3gMT1N+tAIYzA==",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "eJwtykEKgDAMBdGrfHoBTWkXQsxtRAolKejC3N5I3cxsHg/rfppiWNP72lNFAVVQRkVeIyUJL78S7k0POIVLeGK0xfO8zwf/lLyi3xiW",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "eJxljVsKwjAQRbcyzP/E3pHUCEnBBXQRJQoRFKRIUVdvkqoVynzM4xzu+Hge4+VE8RF4zzQGBlN85qbc+c1MO/+1ZlBt2I/+p92Ge6Jj4N6RNpMmN0mNKfeFXqHGEnZirEDKmPBaaz3anDIoKTW1YGwrW6OWXK68UyOAQeaHxSqvf2FvpuY4vw==",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slack")]
    #[strum(props(
        svg = "eJx9kMEKgzAQRH9l2XutWw0oJEJvufTqXao03oqENv59s7FVa8TDEsjszGNWDt3dglNIGcLAbyIQ3n1rjUL/NSq8IJiufxirsMBKntlRyWdjDbQKb1RCkYiaUuOtVz/AkwLxBI1NvF7JAFvCHScygvIV/AjmwwXDcp1FsEnbwH5hUxUu+cUXAb8G71QTDCg15S+/0Gy6JeJEEW8MyQtjbuX+Dpnt0PylQGhK67hakGbSBwYRcC0=",
        categories = "account,social,communication,brands,development",
        tags = "logo",
        contributors = "colebemis,ashygee,wojtekmaj,mittalyashu,ericfennis"
    ))]
    Slack,
    #[cfg(feature = "slash")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNTJSAEMjJTsbfZCoHQBr6Adc",
        categories = "development,maths",
        tags = "divide,division,or,/",
        contributors = ""
    ))]
    Slash,
    #[cfg(feature = "slice")]
    #[strum(props(
        svg = "eJw9jDEKgDAQBL+y2CfmsnJGiHmBfsAuYJFCwUJ8v0bQZouZZeKRz4J1bPYA6YxCy3AZNim21aT4+VmCZQ9aJep1Izh5qUwflr0Vj3ccBM7QcPkrNzZ1GKw=",
        categories = "design",
        tags = "cutter,scalpel,knife",
        contributors = "karsa-mistmere,danielbayley"
    ))]
    Slice,
    #[cfg(feature = "sliders_horizontal")]
    #[strum(props(
        svg = "eJx1j8ENgCAMRVchXUBaieEAbOPBxHiW7aVQCBA8kbavn1d3X8+pInkwoN70IL/ogRBURG4HtzEUXEaZ2euoLaYK9YJEGtO4jqU/wWlooU//RblJeszNNcPHDMuwX8pS+aOFcL3GSHBZXcWixIm2FV2zuCxpFQNqmiXCNvgD3kJibA==",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders")]
    #[strum(props(
        svg = "eJxtkEEKwCAMBL9S8oFqKuJB/U0PhdJz/X2NScBKTotkdt0k39dzbg0L+ADb6wuQ4JDWX+ih5p2gmgcqCM28g+E81LGQFIocSipx/BkusOYIq17yJCMXHbOkv9y45k6BOPVlr8HK+lEY0NsYuwUeJq3J7Z1VV88gbK+51v0A3hRibA==",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "eJwly0EKgCAUhOGrDG9fqZi1UG/QISKlJ7QIEbLbl7gbPv6xOR4F+XWkCE8KhR1JTaiOZkJnjunk8k9ByLWRt1P7eXvvhREcbVKNxixYIQWkYn0NHXRrW+U/FCocIg==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "eJxti9EJwzAMRFcRGkCNbEdOwM4GHcI0oc5fCYa221eOKclHEDrujnthWx4FvhEF4b3OJUf0CJ+IBiEv6zOXiKx+04pxCre6n8IrlQxzxDtbGMia5MlZ2KXTY31PVipQp2dAyAkIGU7M5AWa/ilN/XCFjTQyOFKqr65po6jb84H9ALVGM80=",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "eJwdi0EKgDAMBL8S8gBtioKHNj/wEWKL6U1KQP29SS+77CyTej0VpLZLNGMMCE8rKhlpQfiMIPR3lOVqYzBOs3uc7kMFSsadItAmUyD/nPIPijEY6Q==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "eJxtjlEKgzAQRK8y5D82sxhpIHqDHkJowUIphYro7c0GjSDC7g7MvoGJv34c8GzNQwTkxJ4OadIBbbB0pos3hbpY0DtY/1l5CGpdm/cAP+/vCzNbEwyWTSTLrFI5KqvUziabXv0coz/lCl0qsIEfmotuDJDpeKx9kTlV",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "eJw1i1sKgCAQRbdymf8eIwUF6g5aRFhQEBHWh+4+R/FjOHDmXO1O764d3hD3BBcSVWLMtLorf6uf9TuwGVom8PByO0JhkGvySSqJ1dd57whsaCYEldD2TIhFRBHSSlXb5HgslSozFlNHtf4BAQwrqg==",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[cfg(feature = "snail")]
    #[strum(props(
        svg = "eJxtjdEKwjAMRX/l0vfVJp21g7Z/4KvvpQoTfJAhon9vsoE6GCG5JCfJTff6GHHO5sggXwMCHEiTpfQSc9tFUYbO3Dx3pqSd3pbUrlO7XTBlEw3aKxtyom9Rr0sLLunPiGkkbr3V57HzNkiNp0NdDNROyDP8LL63FOFBgyXsLW9wZuHs7LDiHynPN0s=",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley"
    ))]
    Snail,
    #[cfg(feature = "snowflake")]
    #[strum(props(
        svg = "eJxtjDEKgDAMRa/y6S6aUEqHtncRFBRUHBzs7W1KkA4dwid57ycc+7UiczTEBi9FI1FWLpmpnlMYxUrhd1ldUplUbtx7fjYs0Zw8gdxgB4sywoU03MKjMvQoOWi1W/Yo76vQtD9nWDZF",
        categories = "weather,seasons",
        tags = "cold,weather,freeze,snow,winter",
        contributors = "lscheibel,ericfennis"
    ))]
    Snowflake,
    #[cfg(feature = "sofa")]
    #[strum(props(
        svg = "eJx1TbsOQEAQ/JWJfuN2HCE5ao1WoZMolArZ70fDXXKyxWbe4VjPHVtfTHTo5mYlCPecUDgmGLSqGEL5RIbwBaFq9We8P3dtYkJoEjnEw9ldbsKUXDL1Htoac7vuT1HCW/cKFwOLOFc=",
        categories = "furniture",
        tags = "armchair,furniture,leisure,lounge,loveseat,couch",
        contributors = "karsa-mistmere"
    ))]
    Sofa,
    #[cfg(feature = "soup")]
    #[strum(props(
        svg = "eJylzj0OwjAMBeCrWN3ziO38WSqdWbgAWwVDRwbuL5KKoiLChDLZL++Tx/v8WOh2HM4sJDwbGfn1mbOT7meyyzCNh1aYxnct19bCvpOwIVJThVIvTpBIeoVkMAqiItf/0OTgE4o6mNZZHJPAS91G5AINbRmQW5ikJ/O3HDY57mVY2ey02vqy5Zf979HhE34C3W9WWg==",
        categories = "food-beverage",
        tags = "food,dish,restaurant,course,meal,bowl,starter",
        contributors = "kemie"
    ))]
    Soup,
    #[cfg(feature = "space")]
    #[strum(props(
        svg = "eJwVizEOABEUBa/yov/WsxENao1DCIVSIc6PZmaaCbOugR5VsRb0m81AO9EOFIL5b6+N8OqCW6hS+N6VDrfeDkU=",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Space,
    #[cfg(feature = "spade")]
    #[strum(props(
        svg = "eJxVTssKwkAM/JWh98TdZJetsBb8AH/AW1kPHjx4EL/fiZRCCZP3JNPf6+eJx2W6VZyHZK0gxOFq9FXrlYiI9LfGzNLIOrNwYVW4b9iYMG2w6MHWI5OZECOJqccnKeLReUkLQ7tPSz+FoKXvsjJPz9+yT36hoiZB",
        categories = "shapes,gaming",
        tags = "shape,suit,playing,cards",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Spade,
    #[cfg(feature = "sparkle")]
    #[strum(props(
        svg = "eJxVjjEKgDAMRa8S3FNNirSF2hP0BG4FB4cKDt4fEzNICQSS93lJvttzwrFNFzF4JJdgdbExMCxSJBuOAbTH6oG4C5Yx/RGDXw9VLExdOA4ao2gaJvVoAAeRYbu4TyXP+lt5AdHHIfo=",
        categories = "shapes",
        tags = "star,effect,filter,night,magic,shiny,glitter,twinkle,celebration",
        contributors = "Shiva953,karsa-mistmere"
    ))]
    Sparkle,
    #[cfg(feature = "sparkles")]
    #[strum(props(
        svg = "eJxtjksKwzAMRK8yZO+0I9W4BjcnSC/QXaCLLFLoIuT88YcQE4xAAunpMeE/rTO+r+5HgRr2Pk7bP6mTQHCPxbgVZ5H7qKAsGUCGTww1Fi3CJRPmoitEkY7C5CuIuQhNDaZ4n24ItxR4CEfst4Vuj8aeHnTNi8LOzQ8H+uqyAzJIQrg=",
        categories = "shapes,cursors,multimedia,gaming,weather",
        tags = "stars,effect,filter,night,magic",
        contributors = "karsa-mistmere"
    ))]
    Sparkles,
    #[cfg(feature = "speaker")]
    #[strum(props(
        svg = "eJwljUEKwCAMBL8S8oDWSOlJ/YyVGpAeglD9fZN6GjY7YYOU3OHlq9eIdCLUwnftEb1DmAoEGQsraTgwhd3+UsgsuRUQO0JWg4zqkDdp1Sk0fgpMiqgDg/4WhjdujnTHW6O+eekD2kEnpw==",
        categories = "multimedia,devices",
        tags = "audio,music",
        contributors = "colebemis,ericfennis"
    ))]
    Speaker,
    #[cfg(feature = "speech")]
    #[strum(props(
        svg = "eJxtj7EOwjAMRH/FYs9hx3GcSKUSO6zsEQwMDAyI7ydpoROyfIvf6c7Ts73udDvszgWFIr9DgjwEFbFFKI3lZSIkp9D1UqBHgzot8r1SQbTrwAqyZVKwJRJKMGsGH7SvtBA41k66lVPsobt52o8e87S1kdrriKM0h9HYNQfMGoSRWf+5nMSadlp/jtCzrD9V3TbDBz0FNY0=",
        categories = "accessibility,communication",
        tags = "disability,disabled,dda,human,accessibility,people,sound",
        contributors = "doerge,airone01,jguddas,karsa-mistmere"
    ))]
    Speech,
    #[cfg(feature = "spell_check_2")]
    #[strum(props(
        svg = "eJydjjEKgDAQBL+ypE90TwkpYn7gI0SLNIKg/8eLkFSpbI5lZoqL1/ZkHIs5PejhLQW6xKQ4FJViDdagOIeOmCHc6YgRei0hbrK8C/k2OhaN1Aa/mvbOC4BbK4A=",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck2,
    #[cfg(feature = "spell_check")]
    #[strum(props(
        svg = "eJxlizEKACEQA78S7OXORRaL1R/4CMHCRrDw/7gW2kiakMnIKLOhRtMZjsHWEbSRSfJtlOQcctC5hRd09egHabz1ly8+OBfH",
        categories = "text,development",
        tags = "spelling,error,mistake,oversight,typo,correction,code,linter,a",
        contributors = "danielbayley,jguddas"
    ))]
    SpellCheck,
    #[cfg(feature = "spline")]
    #[strum(props(
        svg = "eJxNzNEJgDAMBNBVwi2gKRQRmoIDOIREQcEPKX7Y7U2tgiQ/xz0u6JZ0X0gvAfegJHAgzQKPGJraxvCpXFXR/sU/dUznSrNg9MTdwI7sWzu2SM9eEfEGekgfSg==",
        categories = "design",
        tags = "",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split_square_horizontal")]
    #[strum(props(
        svg = "eJxNi70KwzAQg19F3G56d2n6A7bnLl27F7fgQikdQkjePnKGkEFIgu+L/+dQ8Upyv8Cut74EgwYPxvjjXJSfG3y1kxwPjc9xs+yEvnaFEhzG+GhalI41B17DTvt+fm/MnsRVMFuSo2BimbN9bbKNygsvHCVF",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[cfg(feature = "split_square_vertical")]
    #[strum(props(
        svg = "eJxNjL0KAkEMhF9lSB9M5jx/YHdrG1t7WYUTRCxkOd/eLMJxRZgMfN+k9/Uz4ZblPOJwGaupw5WgcnKrDgPhcWyDlLTpeEmL5Ef4rg3VQvIugad9jdeUGjvKpivv+XjdMTMLTfD1LM5I/nOOvu1sp8oPN34lYQ==",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareVertical,
    #[cfg(feature = "split")]
    #[strum(props(
        svg = "eJxtyzsOgCAQhOGrTOhBd5eXCVJb6CFILGhMLAznNzQ0munmy5/u8lScqzrIQ6prTuU09TOnQRGyya8Qg7npaKRYWMx9mgwF1mxi4F0g3+wihwVe+0EvvWIjqg==",
        categories = "development,arrows",
        tags = "break,disband,divide,separate,branch,disunite",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Split,
    #[cfg(feature = "spray_can")]
    #[strum(props(
        svg = "eJx1zs0KwjAQBOBXGfaemEn/DDR9A6/epS2mYEEk+PP2Jh6UIrnsYb7dZfrrKQZMXg4VqqANZeh3ORv6r3RoCkKiK1BVlA6ueENu6TaPEY9lisFLLXh6YSMI83IO8ZO8vDR5PS/+Pq10cLCwd5rRQLdK16AiGFQ7Kt3CKKYsDR7Ji1X2v87K1KfGvmhuY2/xU1ky",
        categories = "design,tools",
        tags = "paint,color,graffiti,decoration,aerosol,deodorant,shaving foam,air freshener",
        contributors = "danielbayley"
    ))]
    SprayCan,
    #[cfg(feature = "sprout")]
    #[strum(props(
        svg = "eJxtjjEOwzAMA78iZBdr2bIdA2l+0EcE6ZCxQ6e+vlSHZgkMwYRIHLW8tvchz/v06JLTYWlal1vs1uXvWKK1V1TNqJi1waXoZXSgyoDvBsMsMRmZU6Sgaxa4FjJcnRwQgqxo5IYcWriPOL0qzpokNSo/V1c5TNrWpTPFR4BJNA+lKCSikRFgF1PjNb8i/p01oTtMw3MMyWfHF64iPXg=",
        categories = "nature,gaming",
        tags = "leaf,nature,plant",
        contributors = "ericfennis,mittalyashu"
    ))]
    Sprout,
    #[cfg(feature = "square_asterisk")]
    #[strum(props(
        svg = "eJx1y00KgCAYhOGrDN++H8vQhXqDDhEp6SIIkbLbh25ctZx5eFV0e4J34fBJE5OEV9NMiFnTRMh1PMEmX9WooQRGXVvysJpWNkHeFcrV4JT9AsYhOv6HIwQafpNmJjg=",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[cfg(feature = "square_code")]
    #[strum(props(
        svg = "eJxNy8EKgCAQBNBfGfYuqXXooP5LpLQegpCF6u9Tg4jZw8KbcSWtgsvTSDhzFPZkZgKnvLG8/92x1I6l4IY2CO5YhBE97UbDaGXR07zJ3yfUs6rn8we21R/x",
        categories = "text,development",
        tags = "gist,source,programming,html,xml,coding",
        contributors = ""
    ))]
    SquareCode,
    #[cfg(feature = "square_dashed_bottom_code")]
    #[strum(props(
        svg = "eJxtjjEOwCAIRa9C3E2F6NDEegPX7iYdWJp0aHr+goskms/Cf3wgP+1luA53YwAMnqDLlbwpKXnwCFLku2ZeExA2DQcR6tCZRq9BxmgNoM8afnm27rKWcQH0GUt+dzwxuQ==",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottomCode,
    #[cfg(feature = "square_dashed_bottom")]
    #[strum(props(
        svg = "eJxtjLEKgDAQQ38ldD/0QjsIZ//A1b3gcKOD+P2ei1aQLEkePNvb4djmtBRQG0GMERUK1/JuxHbN/QGe/SHRUrXhFlZ7tFNoXX+A5i+5AIJlIUs=",
        categories = "shapes,development,files",
        tags = "rectangle,aspect ratio,1:1,shape,snippet,code,coding",
        contributors = ""
    ))]
    SquareDashedBottom,
    #[cfg(feature = "square_dot")]
    #[strum(props(
        svg = "eJwli0EKwDAIBL8ifqCkvfSQ5DM2VKEnEZr8PoqnXXZmqw4yWA0vhF8e44blRuAhL1t2nQ1PhBlOr0cceiVR+gaQr8UhrUz1CClx3/v4Go0=",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareDot,
    #[cfg(feature = "square_equal")]
    #[strum(props(
        svg = "eJxVy0EKgDAMBMCvhHzARgU9JP2BjxBbTG9SAurvNXixt2Vnl2veDC7BAeEsyVSQZgTNZVf7cn21R7h9E7nzQ+RjNYUkuExAQSm4eNfK+JcHahkeJw==",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareEqual,
    #[cfg(feature = "square_slash")]
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFpRVBo+xkNJiAeSkDze5N6ybDZ2dJpV2CSk7ViWhGs4ozwjtsdGeGRQ3m0rUwxaOWSm8CSPxeXnZsj/9FyRFdDah9Hkxpg",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareSlash,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "eJyNjDsKwzAQRK8ybC9FK0SIQdIN0qYPssm6C0b4c3tLMrhw5WZgHjPP/79Z0Ad6O7BJijXDKKt0V8J+XDIV1YbSxaU2gIXuSliK/lEN0Z8eNuDnVTSr+6ZpSBlrIHaEZeyzBHoRtgPIMP4kNzKVTbvVQ9wBbCcwqQ==",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = ""
    ))]
    SquareStack,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "eJwlizEOABAQBL9y2Q8IGgU+gzitXILfc1HNFDNxtiK0EzxoPjjQGlU4wQYQt9FZvh9tcjQ65Au3qA+D",
        categories = "shapes",
        tags = "rectangle,aspect ratio,1:1,shape",
        contributors = "colebemis,ericfennis"
    ))]
    Square,
    #[cfg(feature = "squirrel")]
    #[strum(props(
        svg = "eJxtTjEOwjAM/Mqpu4PtJG2RQhdmVvYIhowMiPdzKRUwVKeL7dg+X3nUZ8P9NFxsxlgTErRDejYRn4rZTSUjrTQNuabAlFQYB46MzsrXnw7G8wRTRJj1hwpsz6SzSTTTYSmH7mApPx8jXHnMArdDlAiyWXX4puziL/F/r5KuaU8q85R7pcQ2yduS9yZnWGxB7dt7A0AdPds=",
        categories = "animals",
        tags = "animal,rodent,pet,pest,nuts,retrieve,updates,storage,stash",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Squirrel,
    #[cfg(feature = "stamp")]
    #[strum(props(
        svg = "eJxtTkEKwkAM/EroPbGzaXYVakE8e+3BW6mHHgXF9zu1WETKsruTTDIz7X14TnI7VpeQlCY0Vdfu5l7XrgwOlorArfgpGed4689BIYRPCvwxjSAsepQBgu+0gAa/DcVLYTHWajmrpczKlT78S7luZaGw93uLMxFDxfLE4OKLqrr6yP2cZ0wPMg9IoqvPiXxVfQPcMDdG",
        categories = "design,cursors,tools,maps",
        tags = "mark,print,clone,loyalty,library",
        contributors = "karsa-mistmere"
    ))]
    Stamp,
    #[cfg(feature = "star_half")]
    #[strum(props(
        svg = "eJwBNwDI/zxwYXRoIGQ9Ik0xMiAxNy44IDUuOCAyMSA3IDE0LjEgMiA5LjNsNy0xTDEyIDIiPjwvcGF0aD5iWAyo",
        categories = "social,multimedia",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "mittalyashu,ericfennis,danielbayley,karsa-mistmere"
    ))]
    StarHalf,
    #[cfg(feature = "star_off")]
    #[strum(props(
        svg = "eJxFjVEKwjAQRK+y5D8ju0m6CaQ5QT2EoKAQxA8/7O3dWGl/dthh5k19Xd53us7unBEi/Y5QgWhPFJF1SchCwsRCrFAlzuBsTvdIxQfE4Fo9DU6rO81C0QoCnUj+QD+hsOfFSNI9I9qUOUe7P543Wnl24uizySom493UoiPUvv/CKqc=",
        categories = "multimedia,social",
        tags = "dislike,unlike,remove,unrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StarOff,
    #[cfg(feature = "star")]
    #[strum(props(
        svg = "eJw1jUEKgDAQA78SfEB0F2u3UPsdEaQt6MXfW7FCTmEyibUc91Yyatnzda6DKBTiOAUYdYEqAtVDWmbKDDGKQYVT41o8vYejae9+ru+MQT7Tax5SHPtjegDvlRuE",
        categories = "account,social,shapes,multimedia,weather,emoji,gaming",
        tags = "bookmark,favorite,like,review,rating",
        contributors = "colebemis"
    ))]
    Star,
    #[cfg(feature = "step_back")]
    #[strum(props(
        svg = "eJwtyjEKgDAMheGrhM4Fm5DBIeY2IoWSFHRob2+KTv/j8UmrdsLAI+GeYNDXGeVI3FSSyraUSvc2LzfoXu25g3KmApyRICYv+BN9ARZCGDQ=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "eJwti0sKwCAMBa8SshZqgnRlcptSBEmEdqG3b4Su3oeZ2ptdMFnwRFgkWCJicUaYtF+tx2a0Du/rdoPhzd5HkHIqwDkRQ9QQgvwZ/QAGRxf0",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "eJx9TcsKAjEM/JXQe2KTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjNntPEwi5LTlyYIF1A8jOVwFRroPKeqgR4qoVY7ffGuPhx48yI1cSklfiCDclbUZrSe/uCTh0/p/c0X8+23lplxO0RzZsDbR7NqK4KAzTei5P0zIy2Q==",
        categories = "science,medical",
        tags = "phonendoscope,medical,heart,lungs,sound",
        contributors = "karsa-mistmere"
    ))]
    Stethoscope,
    #[cfg(feature = "sticker")]
    #[strum(props(
        svg = "eJxtjDELgzAQRv/Kh7vXfBcTLKTOHdq1QzfRIaOg+PtNdFAhHAcH790LU79EjK/qSycO9u16hcLkqdO1shkMKJQnMtDI5jSgtf5acZ/j+V914ZGDXbhkYVcffQmlsJ+lBVOPA8XuReYt6Mmz0ZQ6/k42COY04A==",
        categories = "communication,social",
        tags = "reaction,emotion,smile,happy,feedback",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Sticker,
    #[cfg(feature = "sticky_note")]
    #[strum(props(
        svg = "eJxNjLEOgCAQQ3+lcQe5gyOaILODrg5uRAdGB8P3KzFR06XJa1840pmxD81MogV2lMRgmBp1t0JuMyBNukcFnMl9C7DipdMyPee1iaGtwhh+Wtjis3/RBdRlG0I=",
        categories = "communication,social",
        tags = "note,comment,reaction,memo",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    StickyNote,
    #[cfg(feature = "stop_circle")]
    #[strum(props(
        svg = "eJwlikkKwDAIAL8ifqDLoVBI/IyVKvQkQpvf1ySnGZgpbM6PAH8Vtx2B26QnVqSyzE7FhQNyOhFeu0IrHggqdmsMbT3l3z/6ATDkGKY=",
        categories = "multimedia,shapes",
        tags = "media,music",
        contributors = "colebemis,ericfennis"
    ))]
    StopCircle,
    #[cfg(feature = "store")]
    #[strum(props(
        svg = "eJydkb0KwzAMhF9FZJdqXZzYhTSQvV27Bzp4KXQofv7K6Y9TyBSMbTjpk3RoeMzPRLdTcwcF8uKVyzOBQM6OUpDYElKU1s9VVfGQ7niGYc04HEqZcfgWu3hS5PjLd/YjKdYCI3PcQLUjWKg2c2y5ibEWCNlvsDZMgtsKWCS3df6lgpshgcp9a2qGWPpWwkdyLNGA6S+NtKfiZBeK3WjcTS67cGvvjGtd2gsI8GyR",
        categories = "buildings,maps",
        tags = "shop,supermarket,stand,boutique,building",
        contributors = "karsa-mistmere"
    ))]
    Store,
    #[cfg(feature = "stretch_horizontal")]
    #[strum(props(
        svg = "eJxVjEsKgDAQQ68ScgE/FFczvYwWp9syoN7eWhVxkyyS96Sk2XEoA2Epr+bKidjy4qYce6LstYmWUbrrH6VR9/DsVTGEH/jpXuwEpNUewg==",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "eJxdy0sOgCAQBNGrdPoCfkJczXAZJQ5bMol4e2ElYVt5JSWdjle5E1UZCEv5Nm9hJZ58uSkPotROoizdRxmu34xn41uYtw+q4R7C",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "eJxFyr0KgDAMBOBXObL7kzSIQtu5iw9RUFAQcXCwb2/bpYRw3PHZJ74HNkcrT9CwRAODsVwn/Wyg5O1QjLdNKliiQivk/HOYmrvOe0cSRyyEjx1pjlxlJCSuc7ZF+R+lwx7h",
        categories = "text",
        tags = "cross out,delete,remove,format",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Strikethrough,
    #[cfg(feature = "subscript")]
    #[strum(props(
        svg = "eJxtjjEOhEAIRa9C7GEHBnZmkllvsJUnMFrYmFh4/wgWVoZAfv7jE/oxnxusv2FXMKhQh7F/whv7Q1jA8BX9JQG3DXVJyGSkigIuUMimYEY5Q4jYIC1I7N2y6+oJabMQM9wjRXnyK0518SEkilT0tpofTuX54QJsriqC",
        categories = "text",
        tags = "text",
        contributors = "nabanita-sarkar,ericfennis,mittalyashu"
    ))]
    Subscript,
    #[cfg(feature = "subtitles")]
    #[strum(props(
        svg = "eJxtjDELgCAYRP/K0S55n4oE5tzS2tAmNDg0NES/P13EQW453oMXnvRmXOu0e9BkO8UwVxRDE3TVyMB4LENOU8QoJQRdEgh0GVV5m7+VhT06DFGSaXsA+ajPVvwByMYvBA==",
        categories = "",
        tags = "captions,closed captions,accessibility",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Subtitles,
    #[cfg(feature = "sun_dim")]
    #[strum(props(
        svg = "eJyNjUEKgCAQRa8yzAFMR9ONeoMOERYYtAhpUbcvcxOE0urDvPcYG5YU1hnC4VAQQjjLJocKve0K9nYb9wiTw0EQqMi4yDAfX4g4CKqwOyNeYaqRGaZ7A5pJJdtKmYrz9H+U76ML4TVNzg==",
        categories = "accessibility,weather",
        tags = "brightness,dim,low,brightness low",
        contributors = "mittalyashu,bduffany,karsa-mistmere"
    ))]
    SunDim,
    #[cfg(feature = "sun_medium")]
    #[strum(props(
        svg = "eJx9jEEKwjAQRa8yZO+YmUknLpLewENIFCooSBGxt2/TdFOadjF8mPd4IT379HpAH40zkIZoiKf9z9uGc8Ft+Ny+HdyjuRKD/Cij/FoDtlUiQNzVANsd8qYLijpoUEVP6K3PV/EUxU15j9r4I28OldyBVTpLbuONN35P1g==",
        categories = "accessibility,weather",
        tags = "brightness,medium",
        contributors = "mittalyashu,karsa-mistmere"
    ))]
    SunMedium,
    #[cfg(feature = "sun_moon")]
    #[strum(props(
        svg = "eJx9TkEOgCAM+8rincnmopAgP+ARJB64kHgwvt+AHkwE07SXtuvcHo8E2zoEYjCR0UxQRVfIAw0EpETJ4N1YKt69i3xyx9AtJwtaKCSUwkaCFlzgll4mMBCn1jDrjpNnnOpV9bNskcpzn8gFd6xNEA==",
        categories = "accessibility",
        tags = "night,dark,light,moon,sun,brightness,theme,auto theme,system theme,appearance",
        contributors = "itsjavi,mittalyashu,karsa-mistmere"
    ))]
    SunMoon,
    #[cfg(feature = "sun_snow")]
    #[strum(props(
        svg = "eJx1j7EOwjAMRH/l1N0hF5ukSKF/wMpeiaELEgPi+3GWdnEHe3n3rHP/rN8Nr/v0YMZtVSgy6JNRp6VfBl76Hipg2RgAGgqfGpEMOwOFPwmuvTVVA+ekNTVJLUhYUkP1JSeJ0ci7zoHLBhP/NEIuNcSsELyKDlMO/gcsEFYT",
        categories = "weather",
        tags = "weather,air conditioning,temperature,hot,cold,seasons",
        contributors = "karsa-mistmere"
    ))]
    SunSnow,
    #[cfg(feature = "sun")]
    #[strum(props(
        svg = "eJx9jkEKhTAMRK8Ssv/5TSwVofUGHkKqoKAgIqK3F6MrqW4GkjcMz8d+jkMLcQvIgjAHtAhx16v0/wuXfqqXDpqAFQvIquh8PYBJkdFSkYEGk2WNRItzcg7ufO9VAixdSkDMCxkdZfZa/n0aFGRyFU3UDuLSTrI=",
        categories = "accessibility,weather,seasons",
        tags = "brightness,weather,light,summer",
        contributors = "colebemis,ericfennis,karsa-mistmere"
    ))]
    Sun,
    #[cfg(feature = "sunrise")]
    #[strum(props(
        svg = "eJx9jrEOgCAMRH+lcQfbShQSZHbhI0gcWEgcjN8vYHQRzSU39F2vtVvYI6xz54mBD90525eRszdISpoBCKtLRdXeMc9AOnIL4AdJZCROV7X4rc6v8dJq0DCCEgqyGns05suhUCwSGvBJnYlYQYE=",
        categories = "arrows,weather,time",
        tags = "weather,time,morning,day",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunrise,
    #[cfg(feature = "sunset")]
    #[strum(props(
        svg = "eJx9jLEKgDAQQ3/lcG+9O4tWqM4uXd0LDl0KDv4/Xiu6WCWQIS+J28MRYZsaTwyEKzeza3M2u5sko8dOWHFtqNi75mVvI9cAfpBEo8bhula/1wzMS/Whh14ZMEpUGQomG4QDZikL+LRO2T9B0w==",
        categories = "arrows,weather",
        tags = "weather,time,evening,night",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Sunset,
    #[cfg(feature = "superscript")]
    #[strum(props(
        svg = "eJxtjj0KwzAMha9isutVlmVbAjc36NQThHTIUujQ+1OZQqYgENL7nn7GZ/se6XVf3pqyJyNb1nGb4jpOlCUYXbKHcMpykO5MGRWqQuFGJUF9BjSUoimKDmYJF7QLIXeCl2hMY058E2Su6Z95Rixo06Wl7ZEFUpzQi4WsBHOPM9zOl35fri4U",
        categories = "text",
        tags = "text,exponent",
        contributors = "nabanita-sarkar,ericfennis"
    ))]
    Superscript,
    #[cfg(feature = "swiss_franc")]
    #[strum(props(
        svg = "eJxtyzsOABAQANGrbBwAS/yS5QZavUSxpcL9g0alnZeh2RfDyKKiBoPNchSF1K2FnnlAz+kDZ0rScXi0AZPsFpU=",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    SwissFranc,
    #[cfg(feature = "switch_camera")]
    #[strum(props(
        svg = "eJxlTssKwkAQ+5Uw98FO1vqA3Z69ePVeVmEFC1JE9O+7bQ/bUoZhCEkm8e/2k3APcjWDnS/7liCqPKZU3o4FI+NUS+N3o6fxxelQp5UQ/Fq1/AQmXVjjs4+vB+IviFEQ//Ptg7hRNNMlobMTSHXqkHfboDvkpIlDYQehSzZx",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[cfg(feature = "sword")]
    #[strum(props(
        svg = "eJxdjNEKgCAMRX9F/IDKVUZg/k0PgqhQD+3vm5tC9LBd7u49cyVHjCGdquSQ7uvQZhlWZTZas7I0svlQI+3d2BnvmEQgatbqMaJYdScPrAR8yzW0EsLUIPIo/leWDwyBaeX2mXwvv/FlMl0=",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "eJx1kE0KhDAMha9SeoCZaaythY63mYUgKoyL9vbmTxHRRRNI8iXvNS/zWMdh+pllHqb1/7XOv1rjIobGBHwSuUAt2+f3zvSZyeqQStYUyo01FSQX4DoC52EswkealBkOCofLcNHNFfQCQU4gzMfwvYlAkjsUD05CEBvp2YUXIa2e7ORkupEVpeXt4YgXxOso8DYi9Gd2/aef2QDBtGPM",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Swords,
    #[cfg(feature = "syringe")]
    #[strum(props(
        svg = "eJx1jjsOg0AMRK8yoreD8X6wtOEEySGipEgBEgX3F14KKADZcjFPnpkyf5Y/fs9mkh4dAkIzlEcVh3KgjAwlPaO3GAw9Z4ixfkkg1HH0qxzQjsTJ1/VtKmkreUlEvAgyiNx0cE8jhV7V8w8kpB2tcwA2Eg==",
        categories = "science,medical",
        tags = "medicine,medical,needle,pump,plunger,nozzle,blood",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Syringe,
    #[cfg(feature = "table_2")]
    #[strum(props(
        svg = "eJxNjbEKgDAMRH8ldA82UUsLtbOLq3vBoUvBQfr9JhWk3HK8O+7inZ8C12aOAPO+ZgYGq0JxbakOXSH7Y4KOtdzIV0FjaoGRzyAp07BF2PHX9ibFSU/TC8dLG+Y=",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "abejenaru,karsa-mistmere,ericfennis"
    ))]
    Table2,
    #[cfg(feature = "table_properties")]
    #[strum(props(
        svg = "eJxNjEsKgDAMBa8ScgGJpaCQdu3GQ4gtpjspwc/tbRG0m7eY4Q3viwoEhzNZMAcN6LmrzHOOq8KZgorDwiFfDnuEsgZBYtpEX3FXUm714PkL9gTjZP5eK8g25gHBjCUj",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yzAVCJSiY8QYdIlIadyFD2e1TF+HmL/7jPbp2FQiMm7HgbrOgp6l9nnI8FJ4UVBjrD7kwWoSX0SGUvhLTKdpx1Zrg6Q86WGXsDcDMI/kAwiglbA==",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "eJxNjEEKgzAURK8yzD42/zctWSTeoIeQKv3uioRWb2+yURkYhhnmpWV6F/znsVimeMKm+WOl5kAsa6YS1e/ElhnZp1s79Ok7FMOY+XogDAqFrxKoUxO9FtCfPM/C1WROu9BQDXJBRUi0zssx7Z7tJ7Q=",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = ""
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "eJwljEEKgDAMBL8S8gBtg4iHtp/RYgviIQSMvzexl0wYdjdx3QVYMxLC+18eMLUgtNrPJiYCwtMPaRnjiiXN3ivp6ncFJZM0hWgL/m7GOKhO8rwnywd2vBz+",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "eJxtykEKgCAQheGrDLPPnNJsod6gQ8QUGBSEtKjbhxXVws0MvP+zPEWeR+DdoUHg43rRoUZvyzt6+0P0KMqwtd8CDA47qoBMIJliGr+01EI1QFJoBUbItkjnZSdGGCcL",
        categories = "medical",
        tags = "medicine,medication,drug,prescription,pills,pharmacy",
        contributors = "karsa-mistmere,it-is-not,ericfennis"
    ))]
    Tablets,
    #[cfg(feature = "tag")]
    #[strum(props(
        svg = "eJw9jbEKgDAMRH8ldE9MY7Ut1M4O+gNuokOHDg7i92sQCsfdcI+7dO13gXMyqxWQWR7LNZJEUDsouk8g5IJmT06A60hDQDXt8WdQGWBUZNGpzeTU6XhO7cKDL8S2NS8qIx3l",
        categories = "account",
        tags = "label,badge,ticket,mark",
        contributors = "colebemis,csandman,aaofyi,ericfennis,karsa-mistmere"
    ))]
    Tag,
    #[cfg(feature = "tags")]
    #[strum(props(
        svg = "eJxljTsOwkAMRK8ySr9mvZ8klpbUFKGloIugoAgSBeL8eIREisi/YsZv2mt5P3A/dmdDPaXPsPaSDFw3seKNJGXkzVIS4pqljoGLevh5Aj2IgZbZQdduageSp/bn9zCJerG99NSK6pGZsziKkYhe6uPMWQfo9vgFD8kqfg==",
        categories = "account",
        tags = "labels,badges,tickets,marks,copy,multiple",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Tags,
    #[cfg(feature = "tally_1")]
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXyNVEwKTM0U7Kz0QcJ2QEAXiEHNQ==",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,one,1,first,bar,prison,cell,sentence",
        contributors = ""
    ))]
    Tally1,
    #[cfg(feature = "tally_2")]
    #[strum(props(
        svg = "eJxVybEJACAMBMBVQhYQIQjCmw0cQrCwtBDnD2kCae9w1zu0B08h+bWxojgpInoOA3CwDm4=",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,two,2,second,double,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally2,
    #[cfg(feature = "tally_3")]
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnvcVh9TxqVm5IeyWxILxk+lAjELRdMKxXT",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,three,3,third,triple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally3,
    #[cfg(feature = "tally_4")]
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVQhaQQBCEbzZwCMHC0kKcX2wsAmnveFh9TxqVm5IeyWxILxn+KNGQkIgzF+gzHT0=",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,4,fourth,quadruple,bars,prison,cell,sentence",
        contributors = ""
    ))]
    Tally4,
    #[cfg(feature = "tally_5")]
    #[strum(props(
        svg = "eJx1y7EJACAMBdFVPi4gCSIKMRs4hGBhaSHOLzbp0t7jZI+zMFvoCelSDirxJxWD6gG5C7kPMzIYVMweh1QkoQ==",
        categories = "maths,gaming",
        tags = "count,score,enumerate,days,five,5,fifth,bars,prison,cell,sentence,slash,/",
        contributors = ""
    ))]
    Tally5,
    #[cfg(feature = "target")]
    #[strum(props(
        svg = "eJxtjLsJACAMRFeRLOCnsIouEywEq1S6vYEEFLR6cPfukDrTaI5WgZjA0VSyIEBFr31F8yTPx7LVq33ubm0DxO8hVQ==",
        categories = "brands,gaming",
        tags = "logo,bullseye,deadline,projects,overview,work,productivity",
        contributors = "colebemis"
    ))]
    Target,
    #[cfg(feature = "tent")]
    #[strum(props(
        svg = "eJx1y0EKgCAURdGtPJxLPu1Hws8duIiggZOgQfunHASBOr2Hq9d+FxybyYzwDnSYTdKp5qQfnlItWi6t5fBa4dq56EGxATIy/O0BP0Alhg==",
        categories = "nature",
        tags = "campsite,wigwam",
        contributors = "MoltenCoffee,ericfennis"
    ))]
    Tent,
    #[cfg(feature = "terminal_square")]
    #[strum(props(
        svg = "eJxFy0EKgCAQheGrDLOPMINaON6gQ0RK4yIIGUhvnw5EPHibj9/duzAEwmsBY2AadOjd2MG7j7eGxvL8S46HQK6EEwLHdLIQmhWhEFqEqp+L8pOCsGqre+dfnGMgqQ==",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "eJw1i8EJwCAQBFtZroG4YggBtZs8BFEheeS6z3GQ17DLTF6za2/jwpptPHeRBB5gAImEXWrefqVmFzUW4Sl4jTEYaTsKlP5b4PIHsVIY2g==",
        categories = "development",
        tags = "code,command line,prompt,shell",
        contributors = "colebemis,ericfennis"
    ))]
    Terminal,
    #[cfg(feature = "test_tube_2")]
    #[strum(props(
        svg = "eJxlzDsKgDAQhOGrDPZZMxtZI0RrCz1EwMJGsBDP7wO0UAam+eBPa95mTG0xKlHDJCqUwphVYsB9/hxdkKZx4rn7L8G7amCNUHSpvHpdeqoLDQqD/WmkgtZXrxyfJSFF",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube2,
    #[cfg(feature = "test_tube")]
    #[strum(props(
        svg = "eJxtzaEOgDAMBNBfaeZb1mZliLE/wOLJEJUIwvdDCZnC3svdlWM7DfY5LJxIQS7OpC0CU0ImBiFFcSC12J4oQfTAET9cJdQy+FAtfW7ykuUfeY94NNSON2E+ITM=",
        categories = "science",
        tags = "tube,vial,phial,flask,ampoule,ampule,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTube,
    #[cfg(feature = "test_tubes")]
    #[strum(props(
        svg = "eJx1js0KgCAQhF9l8a65mz8IJnTr0rW70MFjh/D5SxChMpY5zAz7Mf6IZ4J9YqsDymiFnkloKJL3IZjiKMtXrACd0Bux4IeCCL6BSFZSfLzw6rL85kVd1giUbCdH9VM4QLOo/io0ietWXcWDPgo=",
        categories = "science",
        tags = "tubes,vials,phials,flasks,ampoules,ampules,lab,chemistry,experiment,test",
        contributors = "danielbayley"
    ))]
    TestTubes,
    #[cfg(feature = "text_cursor_input")]
    #[strum(props(
        svg = "eJx1jbEKhTAMRX/l4h5ek9hXhers4kcUHDI6SL/fCkUqKJnuzTlJ3NNh2KZu9eiNk0LhyjC0Tk2kxt0cfxc/x9tihTijRqSCoknQxb+IHvxf+iSQCgpJpqZAKb5eDhZaEJIflyBG4cUcETK7e3ECuCc7lg==",
        categories = "text,layout",
        tags = "select",
        contributors = "ericfennis,karsa-mistmere,danielbayley,jguddas"
    ))]
    TextCursorInput,
    #[cfg(feature = "text_cursor")]
    #[strum(props(
        svg = "eJx1jCEOgDAMRa/yM9+wLg2Yshtg8UsQlQiy89MhKIZ89ZL3vp7tMhxr2nhBKUbcBILsYxKSfQ6Gs3GqOo2o6ps+ZYR5iJ1+zI/oj5Ae3g1sVSJx",
        categories = "text,cursors",
        tags = "select",
        contributors = "ericfennis"
    ))]
    TextCursor,
    #[cfg(feature = "text_quote")]
    #[strum(props(
        svg = "eJx1yzEOABAMQNGrNC4gJcFQnbs4hMRgNIjzY+nE+l8+jTo7tGwKRgjiDZO9iUnBIaCT9JH0FH+WFRQ2v+Acqg==",
        categories = "text",
        tags = "blockquote,quotation,indent,reply,response",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    TextQuote,
    #[cfg(feature = "text_select")]
    #[strum(props(
        svg = "eJx1j0EKwjAQRa8yzL7onzTVQNIbeIiCQgURFyL29mZI0FomZJHkv/cnJD6m50znxCdPbhIS2uvq8onHuFM6xq+DsJJAtiQghJ/VGOVJsJY6S8rPzTBzgQnQNxoZNCqOwgv2N2zgCH2r8U9u1/uFFiQ+Mr0lMTzTIuWa04Oa6lSzZMXMuxYhpQHZuhq6CocqD9u5H2VqfkI=",
        categories = "text,cursors",
        tags = "find,search,selection,dashed",
        contributors = "danielbayley"
    ))]
    TextSelect,
    #[cfg(feature = "text")]
    #[strum(props(
        svg = "eJx1yTsOABAQBcCrbByAPOJTLLXGISQKpcL9g0Y02hkedXZqURR4chLZiMTqYOJbGgT9OVi5Nzy3ALf9Fo8=",
        categories = "text,files,cursors",
        tags = "find,search,data,txt,pdf,document",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Text,
    #[cfg(feature = "theater")]
    #[strum(props(
        svg = "eJyVjjEKgDAMRa/ycY82SZUO1Rt4CNGho6B4fm1BdIiD/Cm8l5/EddoTlr4aBew2JYVSqIbYZDDEBxdOWmIJ7CCzg689ad0hUICp+VtD0WBr5Zogp/3+5sJkCwHcJrtY5CCecrO7wrki+fcMOfjj81+rJ8rGV2U=",
        categories = "buildings,social",
        tags = "theater,theatre,entertainment,podium,stage,musical",
        contributors = "danielbayley"
    ))]
    Theater,
    #[cfg(feature = "thermometer_snowflake")]
    #[strum(props(
        svg = "eJxtjbEOgCAMRH/l4q7SUokkyB+4OriROLCYOBi/3+LAIkPb3F1fG650ZxxLtzKIM5kuhrF4MdTEQx5y/+C08LCwvVYDI4bTBa+9BTvQ/LE0TKUaF9iUz2aYJAkEBgTqdW6SGKy6OKr3yr52xjTn",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,cold,freeze,freezing",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ThermometerSnowflake,
    #[cfg(feature = "thermometer_sun")]
    #[strum(props(
        svg = "eJxtTrEKQjEM/JXj7Y1Neq0+qG92eauDW8HBRXCQ9/2mIiJYQi6Q3OWuPtrzhutxWtUwN4KIvYJhL3la6q4TlvpLS5v9H+5FCvQgDCqE90BrEdw0SubHSKHB55nNYG9j7fvLQOs/02nguxZJ9KwOlDkhO3xZL3hzN9M=",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather,warm,hot",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ThermometerSun,
    #[cfg(feature = "thermometer")]
    #[strum(props(
        svg = "eJwBPADD/zxwYXRoIGQ9Ik0xNCA0djEwLjU0YTQgNCAwIDEgMS00IDBWNGEyIDIgMCAwIDEgNCAwWiI+PC9wYXRoPrArDeE=",
        categories = "weather",
        tags = "temperature,celsius,fahrenheit,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Thermometer,
    #[cfg(feature = "thumbs_down")]
    #[strum(props(
        svg = "eJxNTrsOwjAM/BWre0zu3DapFCqxdYCVgS0SQwYGBsT3Y0AKyJJt3cPncq+PJtf9cEISjGcOa9m9sbV0ZhFkBQXRFduoSJVCiV4I0IWBOs03qlnIh07JrJNwY/ypffKZ/9y+NXenuWPRLyZPVODomWSL1RQmn/Z1WTDN+dJffQEUJiqI",
        categories = "account,social",
        tags = "dislike,bad,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsDown,
    #[cfg(feature = "thumbs_up")]
    #[strum(props(
        svg = "eJxNjrEKwzAMRH9FZLfqk+JYATfQrUP7A90MHTx06FDy/VVacIJA6N6dhMq7fho9z8M9E+IKGZZy2thSuoNEic0Io0eaj1qFhKIXCDy74DS9grAq2eXgZU4kch33fJAga7DDAQdNOE8dOeQ8BzBwg8MWqzKUfu2/swmzR//1C10SKtg=",
        categories = "account,social",
        tags = "like,good,emotion",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    ThumbsUp,
    #[cfg(feature = "ticket")]
    #[strum(props(
        svg = "eJx1jbEKgDAMRH8luBebq7YItbOLq4NbwaGjg/T7jQ6aQQkhXN4dF/d8FNrGZgYN2ZEjK8OyviKDcGsrF4W9fhhUA50wfgmvwYhh6rQmrE2K7VWY4lPLjvqKb8Dhl7AiJ97sLxs=",
        categories = "account,transportation",
        tags = "entry,pass,voucher,perforated,dashed",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Ticket,
    #[cfg(feature = "timer_off")]
    #[strum(props(
        svg = "eJxtjjEKgDAMRa/ycTc2sdgKtTfwEAWHLoKDeH5Th3awhE/ICy8kXOnOOLZhZwPJdohhKiiGurC0gDl5eJivmBx8SSVlUtqRHVloqs1gQ7O2BmgtTP7yKdDSt0znMAtYnrFpLxSyMK8=",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    TimerOff,
    #[cfg(feature = "timer_reset")]
    #[strum(props(
        svg = "eJxtizEOgCAQBL+ysQc5OAQTpLbQR5BYUFoY3u8ZE23IVruzk85yVRzLsJOBrTzkND5TTj+wIG6qhxjkSkSEkRCiCngbgZTXTsRNPkFPHXkWsHLzH7oBER0j3w==",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "mittalyashu,ericfennis"
    ))]
    TimerReset,
    #[cfg(feature = "timer")]
    #[strum(props(
        svg = "eJxFy0EKwCAMBMCvhHygjbTQQ8xnggdBevAUf9/YgJ6WZWe51beApYx0IRh5ngjDe/KgGcLHRMKb3rGtS4oL0cZau7YCPeODoCOw2o8dxSwfEP8hDA==",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "eJwli8sNgEAIRFshNODnYDzs0gwSIfFESHS7F/E0MPNec+EAfzpuCD4q8lkRbjtC85oRVOzU6LhkXQi16fOosTlfAl4Gp7hnjCKT+Vd6AYpnHB4=",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "eJwtjEEOgCAMBL/S9AMKB0/AZyqxTTw1TZTfWyynTXZmt2gng1HxQND3D+5ysVVM2asgj5zGFfOO4E7GVra5a4VE6e5AI3RymubTkgK3D6jmHE0=",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleRight,
    #[cfg(feature = "tornado")]
    #[strum(props(
        svg = "eJxtyzEKgDAQRNGrDOlFZ5UlgTX1Nh5CsEhp4f2RNCHFtv/x7b2/hudMlxCH76na2lO1AczIrhEUULxEoqC2JZwI2abpBy/qJAk=",
        categories = "weather",
        tags = "weather,wind,storm,hurricane",
        contributors = "ericfennis"
    ))]
    Tornado,
    #[cfg(feature = "touchpad_off")]
    #[strum(props(
        svg = "eJx1jqEOgDAMRH+lmW9YL8vUmMZg8UsQMyQIsu+nQzBESUWbXu9d01muSvvs1kChgEC+F+vUBGOhHVWiy2nqjpxeH0hCFVjKI7ElieJ8Y4N39Ehonv8hxi1+/2QsMk5veig1VQ==",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TouchpadOff,
    #[cfg(feature = "touchpad")]
    #[strum(props(
        svg = "eJxNi0sKgDAQQ68yZC/aUrrq9AYeQmxxupMy+Lm9LYK4S/LyQs2r0sWwoJvhQPUtZ0kqLU0gyWUTZRiPGMYuxLAvKpQYsyXjpL0a6duPGEt2Ogb/oQdysR5X",
        categories = "devices",
        tags = "trackpad,cursor",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Touchpad,
    #[cfg(feature = "tower_control")]
    #[strum(props(
        svg = "eJxtzLEKAjEQhOFXGa7PmtmYTQLxaptrLewOLFIoWMg9v9FCOQhb7Xzw1+f6aridpoVZFFTRBPWw8/FOybD+rwThvyclSgqNXuJuNdfn6zTXw6c31181g2ErA6BBdXMDehRYT6eBMMLckBYq7KIjCNDm/vIGRyE+qw==",
        categories = "travel,transportation",
        tags = "airport,travel,tower,transportation,lighthouse",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    TowerControl,
    #[cfg(feature = "toy_brick")]
    #[strum(props(
        svg = "eJx1y70KwzAMBOBXObQrteo2uGBn7tK1e0hCla0Ek5+3j7UFQjiQjoMvTkOXsYx91kQSCNNaPqFcT9Bh/Gkuw52wJQrUxJuBJv7brOgTfcQhfJ+d46rm6sFiedetQOAsXNrsDRo5wtcZKvtLuQP1pSrm",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "eJxtjuEKwjAMhF8l5H/jTpvZwbo38K//pQoVJsgQ0bc33cANNig9yH13Sfu8vDJdI58O5HPTg47ctbsy7dq/5wk4+w0jEKpNA4E0OampciDx9uGtUq/JByorL0+aVHhTUbFiQe9MNTvMsXQfUn+j9I0MZRoii0n6RB7vnuwVNvmF1iU1X1sTQna63mNBhDG556kuLBp+OG5WIA==",
        categories = "transportation,sustainability,food-beverage",
        tags = "farming,farmer,ranch,harvest,equipment,vehicle",
        contributors = "danielbayley"
    ))]
    Tractor,
    #[cfg(feature = "traffic_cone")]
    #[strum(props(
        svg = "eJxtUEuOwjAMvYrF3m/iOCat1OEGc4HZVWXBAiSkcn/xHCQ2oEQvifU+dpb7+rjI+ffwN8PliLo2RMiAMlagSTmclp9knpY3v2MWK+gbZkxSyTI6NFieu6Mq0seUzy9yc+odsRpmlwEjTx2Twq7KYtl4SyNDSziSYOgMq+IS4nuku4T6VpRFQhKZ/P8ZeetsyKjVNjw63QOumNAJdLYdTH9dr51x/IYcL8g2IlvMso4yNWxRaZUj2q7DKneqM0MrpncbT9lfT0Q=",
        categories = "transportation",
        tags = "roadworks,tarmac,safety,block",
        contributors = "danielbayley"
    ))]
    TrafficCone,
    #[cfg(feature = "train_front_tunnel")]
    #[strum(props(
        svg = "eJx9TzsKwzAMvcoju1xLVkwMbm7QNUM3QYcshQ7F56/6oQRigoQQeh+e6sOeK27n4SIQWViMI7x9eIkvjeMw19ObN9c/m0fkMDUOagkSpq+AMuLi957ACeMaIvcwPcBcV0yh+IQiJW2ULCP/MrJ4xrRlQK97o3txHxKkDuTPcMEWewHVkkWw",
        categories = "transportation,maps",
        tags = "railway,metro,subway,underground,speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFrontTunnel,
    #[cfg(feature = "train_front")]
    #[strum(props(
        svg = "eJxtjrEKhEAMRH9lsI+3WTfeCnv+ga3FdYtXXCNYiN/vrCBYLAkJZF4ySVve//h9mimia3V+54AAd0WEmzlrxvQq1Jhudh2gJio1SQ0la9rEtWER3/KwGLtntUNCjsWLodAe7giLA6lCgATsWzGKPCYeXe2Hnhqe2glE3Dol",
        categories = "transportation",
        tags = "railway,metro,subway,underground,high-speed,bullet,fast,track,line",
        contributors = "danielbayley"
    ))]
    TrainFront,
    #[cfg(feature = "train_track")]
    #[strum(props(
        svg = "eJx1zrEJwDAMBMBVHi8QJGysQvEGHiKQIk0gRfbHUmFXFnz1x8Prd/0P7jN1BlUPp6aHt02nvWYZAtlQAVFAAgnEJiWiDN5Tt2/sqcsGLq4zUA==",
        categories = "transportation,maps",
        tags = "railway,line",
        contributors = "danielbayley"
    ))]
    TrainTrack,
    #[cfg(feature = "tram_front")]
    #[strum(props(
        svg = "eJxtzU0Kg0AMBeCrPLIf2kQrU5jxBh6iVGlcFEQGf25vxI3irBLy5fHC2H0T5r5NGokrgnb9T9OxL5FKwmhDCGukgurw2AN1GD5J0UZqSjCrPZvst5OwoJj8Hf4e/HaGGWIPEScuY43FXvrMFVVX2QDlsjt+",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "eJxtjMsKwjAQRX/lkn0wcw1phbRrN267lyhEEHEhxf69M1oshS6GO49zJj/Pr4pL5057pCqt6/POVn3+H+SANEosAeIF9ASPTdE2eHrR4pA2tBZpUEk5McWzxqIOCH0Cjlyc++1xxcTOSePwFs2gyV9ONovBhs3wF4pukTjPa/gDfNo3dg==",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash2,
    #[cfg(feature = "trash")]
    #[strum(props(
        svg = "eJxtzKEOgDAMBNBfueAb6EHGSMY0BjtPhphEkH0/nZlCNGnaexee6y249+Gc4Yr6IYaxnWLoD93gqi55goqCQvBYs62TUNSGyf0wD5cMWU4bEZYlmwFhJWBlNx/bXyEY",
        categories = "files,mail",
        tags = "garbage,delete,remove,bin",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Trash,
    #[cfg(feature = "tree_deciduous")]
    #[strum(props(
        svg = "eJxVT0EKAjEM/Mqw90SzaW0X6oIP8APeynrYowfx/U5EKlJKkplMJmmP/txxP0/XClv22hMSjp/nWmXWioGI6UmS5s2YGYxs0SRJtBCYWfKLs8m0dId/ZS6Rjwq+sYGqUCyaQwEqOIbBRTMCp23/swatL7/1Yt/btLZDXLC2cYfNJF4+mDcVry7n",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    TreeDeciduous,
    #[cfg(feature = "tree_pine")]
    #[strum(props(
        svg = "eJx1jj0KhUAMhK8y2G/ey2YlCqtgZ6EXsBMsLBQsxPMbBQUFSTdf5icu/TpiKJKZFRwgEJKewfjbsSNTSetA+hCdic1hGN3r/SQ5cgO+ugEyKEnDHjIFhKcnOys4nT7KWbukjL9jaRmvva1leb85udEO0W4r7w==",
        categories = "nature",
        tags = "tree,pine,forest,park,nature",
        contributors = "karsa-mistmere,jguddas,danielbayley"
    ))]
    TreePine,
    #[cfg(feature = "trees")]
    #[strum(props(
        svg = "eJxtzr0KwzAMBOBXObJH1cWJf8ANdOvQrh26GTp46NCh+PnrNGAymFuEPjgpftI343Ue7lRQi0wXAwOtIbwE0Ba9LkWzpgYjx0X8g4cVLPQ5rPG09a2xtbqtwHaABgzF9GSqkr2YxNqr/4gbKe5GD865KxYhy3QEv0N98V2nGZSlXfsBkWg53A==",
        categories = "nature",
        tags = "tree,forest,park,nature",
        contributors = "karsa-mistmere"
    ))]
    Trees,
    #[cfg(feature = "trello")]
    #[strum(props(
        svg = "eJx1jEEOgCAMBL9C+gGDaNCk8BkllitpovxeaFJunJruziyWdLH5AjgwpZ0VTJWHUn6IA9ijFVWKN99MkkRcuhdRbEXPgThZ8dCH/YTepbWbokNV/AdwaCw5",
        categories = "account,brands,development",
        tags = "logo,brand",
        contributors = "bdbch,csandman,mittalyashu,ericfennis"
    ))]
    Trello,
    #[cfg(feature = "trending_down")]
    #[strum(props(
        svg = "eJxlizsKwCAQRK8yeICE3ZCYYuNtUgiiQtJ4e/FXiMUMPHhPYnDJWf8iBuv/71HMIA06thN3XwWGVkb24RtZSrpK2fryNPkZYRMeTQ==",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingDown,
    #[cfg(feature = "trending_up")]
    #[strum(props(
        svg = "eJxlyzEKgDAQRNGrDDmAOhtiLNbcxiIQkoA23t5VsBCbzxTztLdyllw39Jbrsa9OBBH0QwCDZbnHZBEwuqTjC5L+KGejj7fQf94XPbYeGA==",
        categories = "charts,arrows",
        tags = "statistics",
        contributors = "colebemis,karsa-mistmere"
    ))]
    TrendingUp,
    #[cfg(feature = "triangle_right")]
    #[strum(props(
        svg = "eJwdjDEKgDAUQ68S3H/s/y1WoXZ20Au4lTo4Onh/bCWQPAi89JT3xrUOhxl0LgaDa1FptPkqSoVr7YWTMHTazTEg0Fcuwgj9nz6M55DT2J35A37lE6Y=",
        categories = "shapes,maths",
        tags = "volume,controls,controller,tv remote,geometry,delta,ramp,slope,incline,increase",
        contributors = "danielbayley"
    ))]
    TriangleRight,
    #[cfg(feature = "triangle")]
    #[strum(props(
        svg = "eJxFjDsKgDAQRK8ypN+Y/aBbxIDXsAtYpFCw8P7INsp0w3uv3v0ZONZ0CedFwU5ObF0gKDHSbI5ykoNt+24YhAfPP4jwSffU6hTR9gKUFxOz",
        categories = "shapes",
        tags = "equilateral,delta,shape,pyramid,hierarchy",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Triangle,
    #[cfg(feature = "trophy")]
    #[strum(props(
        svg = "eJx1zzEPgjAQhuG/8oWds9+lvWsTZHFhcWVwIzowOhh/v6DCQCBNc8ub53LNc3iNeJyrq6F0UdKgkjD/MD0i1Kmzqm1Oc9c2a82MMnKTh2/OvNNHqI7claY1Ucx6+j1AUqolupRcS3FQlBeXnMAsnuDQIBrnqXtW3Fg/apVowoXiYvEIy9DO3j4Y7H8eFaHX2xp/AGz+Szs=",
        categories = "sports,gaming",
        tags = "prize,sports,winner,achievement,award",
        contributors = "karsa-mistmere"
    ))]
    Trophy,
    #[cfg(feature = "truck")]
    #[strum(props(
        svg = "eJxljs0KwjAQhF9l2HtiVtf+QNKzF6/eZRUi9CBFir692Qi10MvA7Mx8bHxeXxm3ROcjuDsd1PkGwbHzUoQvrYZyMYNiMwe1HAwvRXhmpiHuDDLEBcWCPssokFk0wDfO2obIbv/v62PS8Q59J2oJ+knEHWFKVDu/dE2tH/abfR0sc6NxuwJ8AdjdOrE=",
        categories = "transportation",
        tags = "delivery,van,shipping,haulage,lorry",
        contributors = "colebemis,ahtohbi4,ericfennis,Andreto,csandman,karsa-mistmere,danielbayley"
    ))]
    Truck,
    #[cfg(feature = "turtle")]
    #[strum(props(
        svg = "eJyNj7EOwjAMRH/l1D3B55iSSqFzF36ALRJDBpAYUL+fENSqQ4fKi6179p3TO38KHtfuRQUFCptDJghpVbui29lxdiFHxCaKYw85sPFUZ8Xu3ZhOP8cxLb4381Fx8UO9SNnRefaMDWgJ94jeD6FKk0rW+sE/hcAmXekv8GQ2BA==",
        categories = "animals",
        tags = "animal,pet,tortoise,slow,speed",
        contributors = ""
    ))]
    Turtle,
    #[cfg(feature = "tv_2")]
    #[strum(props(
        svg = "eJwli9EJgDAMBVcJbwHbKvjTZAOHEFtM/6QE1O019Ou4g8vXbkqFsa2UosYAyZM3yb0eRncrpowUQM8P0MuYQX2I1naqMeLimw/yAa1hFsk=",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "eJw1i1EKgCAQRK8y7AVKIfxRL1OSgqjYQnn7NiHmZ3jzxvawM/pwpAl3OjhKWwnPBIINIYZ0RnakNkKfg7fL9/O21TxyKgGtpsKXOAYaSsNIpvcb/gVZuh6w",
        categories = "devices,multimedia,communication",
        tags = "television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,entertainment,showtime,channels,terrestrial,satellite,cable,broadcast,live,frequency,tune,scan,aerial,receiver,transmission,signal,connection,connectivity",
        contributors = "colebemis,ericfennis"
    ))]
    Tv,
    #[cfg(feature = "twitch")]
    #[strum(props(
        svg = "eJwBOwDE/zxwYXRoIGQ9Ik0yMSAySDN2MTZoNXY0bDQtNGg1bDQtNFYyem0tMTAgOVY3bTUgNFY3Ij48L3BhdGg+5+oQKQ==",
        categories = "brands,social,account,gaming",
        tags = "logo,social",
        contributors = "ahtohbi4,johnletey"
    ))]
    Twitch,
    #[cfg(feature = "twitter")]
    #[strum(props(
        svg = "eJwdjsEKwkAMRH9l6D1jk83uulB78exHSD14FPTk13daQgJ5eQxZPs/fG6/b9IhAfo0dQbdAYW7OBp9tMOGdxfwKP1hQDpJpWprFvcArVRgCBXWTIauhqlPugNxhKZzWBLsVKk2nGQoWP+d/WpfL8dK6A3M+Hxk=",
        categories = "brands,social,account",
        tags = "logo,social",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Twitter,
    #[cfg(feature = "type")]
    #[strum(props(
        svg = "eJxly1EKgCAMBuCrjP8C6TAkUG/TgyAq1EPevrUKghhsbPv+0FsZJdeVest13yIceXJSbO7mkcL0qhTUDhvBBnRwhJ1lyr6AButZ/MdeP8vQjHsi/LMnGx0kGQ==",
        categories = "text",
        tags = "text,font,typography",
        contributors = "colebemis,ericfennis"
    ))]
    Type,
    #[cfg(feature = "umbrella")]
    #[strum(props(
        svg = "eJxtyjEKgDAQRNGrDOnV3UHEYs0NvIDdgoWlheT8ZiGkCgO/mWevfw/uI50klK4yy4ZWCGRizZWyLSGzda/hy+4Ew9WtkLFj0X78GnkcUw==",
        categories = "weather",
        tags = "rain,weather",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Umbrella,
    #[cfg(feature = "underline")]
    #[strum(props(
        svg = "eJwtykEKgDAMRNGrDLmAaShZpbmBW/cFBQURFyLt7bVVZjF8eHbma8WcaFTEW7NCwX1BwFMkt6ERt307FlRJJEyo4fvyd3m726b8AZfKFTk=",
        categories = "text",
        tags = "text,format",
        contributors = "colebemis,ericfennis,csandman,johnletey"
    ))]
    Underline,
    #[cfg(feature = "undo_2")]
    #[strum(props(
        svg = "eJxVjDEKwCAUQ68S3LX/gxmEX+cuPYTQwaFDh9LzV0EECRleHsSe8lZcuzsTNCIi3fR02bYusk3dTFUJLAxEr7QoBn2y7n7QoTq/fpJgGUc=",
        categories = "text,arrows",
        tags = "redo,rerun,history,back,return,reverse,revert,direction,u-turn",
        contributors = "lscheibel,danielbayley,karsa-mistmere"
    ))]
    Undo2,
    #[cfg(feature = "undo_dot")]
    #[strum(props(
        svg = "eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0ulJSEKpRkhXNUPBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtFyR/X",
        categories = "text,arrows",
        tags = "redo,history,step,back",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UndoDot,
    #[cfg(feature = "undo")]
    #[strum(props(
        svg = "eJw9izEKwCAQBL+y2JvkPDg5uPiC5BFCCssU4vvlCmWb3R3G/tobvju8jDykSSh2+ldsk0SgXBWKyxM1el9LkA5+GMRbnbn2FMg=",
        categories = "text,arrows",
        tags = "redo,rerun,history",
        contributors = "aelfric,ericfennis,csandman"
    ))]
    Undo,
    #[cfg(feature = "unfold_horizontal")]
    #[strum(props(
        svg = "eJx1yTEOgCAQBdGr/NAT3UUIJEht4yFMLGhMLIznVyiodjPdvHwfT8W5mp0CiGswJU/tlTwk/rCxAMTgV4GoAS2a8CzJRQnk4WxPcI9kHXpDP5hnO50=",
        categories = "arrows,layout",
        tags = "arrow,collapse,fold,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldHorizontal,
    #[cfg(feature = "unfold_vertical")]
    #[strum(props(
        svg = "eJx1yzEKgDAQRNGrLOkX3YkJEWJqG1t7wSKNYCGe38RGiw3TzePHc7sy7ZNZBATc7E2KXT1T/FNYocBAglkD6asETXyRzFoEtOgQRzKyJctlujt+kT5+AG84PEE=",
        categories = "arrows,layout",
        tags = "arrow,expand,vertical,dashed",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    UnfoldVertical,
    #[cfg(feature = "ungroup")]
    #[strum(props(
        svg = "eJxtjDEKgDAQBL+y7AfkQMXiLp/R4KUNB+rvjRaCkGab2RmteQ0cZQs3LkQ9jUK0nYjLOBKey+5hnJl0eO5JX6lR+eFuRuTTbp70Hp8=",
        categories = "shapes,files",
        tags = "cubes,packages,parts,units,collection,cluster,separate",
        contributors = "danielbayley"
    ))]
    Ungroup,
    #[cfg(feature = "unlink_2")]
    #[strum(props(
        svg = "eJxFizEKACEMBL+y2MtFIZfGE66z8RGCRRrBwv+jqWTZambSbEvRP1cDQzQ2BoPOgp3Ux+FfUJH/AjHR5fRYmzcFyA8+",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink2,
    #[cfg(feature = "unlink")]
    #[strum(props(
        svg = "eJxtjd0KwyAMRl8leG9mQqMt1L5LYYMOurGLXdS3n1FX9lOEhCTn84yP+bnAOZob9dh3QIwsQBjY5kKLRcezoHMd1Or0WSS2AV0oS99qPXkcBNyqcdaPyEzjSS3TuLsEKQDloxRLwf4tkC1wbIFm0aD9lqzX+wUSRcMGNo6mN5BykzyRThlU5A02opwK0WJyAHK7ka+odk3S8ANvdVlgDSX+gP0OvwCjKld1",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unlock")]
    #[strum(props(
        svg = "eJwdy0sKgCAUheGtHO68xy3CBHUHTZtLSjoLEcrdp3JmH/9RyV8ZRRMzIX2aFsIbXQ5VdkKFtXrpHny8Q+6pUVM7GvXYHOA0HQLMp7AbNsx1DDnKoYctMT/M2Rt4",
        categories = "security",
        tags = "security",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Unlock,
    #[cfg(feature = "unplug")]
    #[strum(props(
        svg = "eJxtjsEKwkAMRH9l6H3XTdJGhbVfoD/greDBQwUP/j8mOSilYdglZB6Z6e/l88TjMrzojAlSZJj7wZdz/1kM5ty6aRVwq7JwHeGvhcSnKzHotBY1sXM7LMB7cvZYJ5D410CUANTc1MBAY1LawjVksSWJLha9muHddGuWqP/v9QWv+US4",
        categories = "devices,development",
        tags = "electricity,electronics,socket,outlet,disconnect",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Unplug,
    #[cfg(feature = "upload_cloud")]
    #[strum(props(
        svg = "eJxljDsKgDAQRK8ypHfNLpsfxIAH8BABizSChXh+jUUaeQwMPHj5rFfDvphNwUoxpTUgwII7jgIjNqaQqpJDn/2kvC+SqJiS594oeZRYwHKnvznYg/2kHejwD/RqH/g=",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UploadCloud,
    #[cfg(feature = "upload")]
    #[strum(props(
        svg = "eJxNy7EKgDAMBNBfObIXTbToUDu7+BGCgoJoQRH9e9PSQTLcHby4MF4Lpo4GYbC961EgKPXYaOvtfxu5TU3eFfHJu3Bs77buM8Kx7tfZETdowYIKWpLLwrvkHlYjhFezIjySZ0wbfbIf64wmTg==",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[cfg(feature = "usb")]
    #[strum(props(
        svg = "eJxtjUEKwjAQRa/ymX1iZpI0LSS9QS/gTqKgUEGKC729U7tooV3Mh+G9mZ/rY6rjDfVTiB2hfgslwqQb9fm00D5vrLBI4vbW6/K+41poCDaBO+s1EGdjJit/CsMbpRDIec+HzkqDZJsWESyjHD5hBw4qCLyNRufIafUebFi7/rm2/QBFxEHD",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_2")]
    #[strum(props(
        svg = "eJwlyjEKgDAMBdCrfP4uJgWlQ5MbeIgSBQUHKQ56eynO75U4Wpwb4jFqIuI1ZqIZJ3oZf/Vy1XvHalySIGnNyBAoZNAZ0mMP/gFBXBRe",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    User2,
    #[cfg(feature = "user_check_2")]
    #[strum(props(
        svg = "eJwtzEEKgCAUBNCrDH8f9S0kwe8NOkRYkCAl1SJvX1bMYhbzGJvGc8EkNHAHNqOGRlNSsUJDztYFOOvD7uOMXagj+CxknrqE+kK+0dm0xRzDOiNtYT0PIdZgBvfgFkrBvH8/cjcCwiJA",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserCheck2,
    #[cfg(feature = "user_check")]
    #[strum(props(
        svg = "eJxNjcEKwjAQRH9l2HvQXZdqIZuzFz+ixIKB0IZaiv17je2hzGmYNzO+dPMLT6MHNxBenHQKxbnKqdN7c/TQRSj4Uy0FH9MUc4/4MWoJcTW6EiYjrcgWBl/GvOY09ChjGua30e+HGXwDXyCC9r+3Q+ELWK8moQ==",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[cfg(feature = "user_circle_2")]
    #[strum(props(
        svg = "eJxFy0sKgDAMBNCrDNmLbZHioukNPESJgoILCS709vYjSlaZeROOdK6YmSY7wpnk4WHKddbBUAx9ATHIprIvkIvJOoIyDQS581dRqz+mNW991u/qdw8tyR+9",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle2,
    #[cfg(feature = "user_circle")]
    #[strum(props(
        svg = "eJxNzEEKgCAQBdCrDO4r/YERmDdo214sMGgRElG3b7QImcVn+G/G+DX6baE4CCUF+YsTnHdOa5q3t+ZzuWDIvv15wXZ3BJoHMXYEWWuNSfUOBJI8ilAh6HInnCqx9CPd2gc66iVr",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle,
    #[cfg(feature = "user_cog_2")]
    #[strum(props(
        svg = "eJx9z1EKwjAMBuCrhLzvd2nWroVt4AE8hFRBQUGGD3p72xVxD530qeTLn2SI1znezhRfI4tnmkdWpvhOP8vTsCvlafiyVAiL6njp8Wv0OD4vdBr5IC0sicLuHTlql2dIQsYZ/ejdCHoSh65BaKAVIRYmh4Vt4eBIPHpoMjUQICQG5g8oCU1aROojfE7QTdBBqSySSO3QNh+qOWYNPvMjZLc=",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserCog2,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "eJx9z8EKAiEQBuBXGbw77Tiuq6B77tJDLBYUFMQSUW+fk4c8uOFB4f/4nYn5subrCfIrKfIK8rvco4I1KVZz3NV4jg0L39RWO7XovjzOcEzqQAPQuHeLBQuDHF1eTyNWzE/eDOEE5NBqDBq5I2hEA8QSbwmHDsjjhFxMDwQkIIPmD6gNugxC/S+8NPAmsMhQBymkt+ggi7LUtOADN0dlDQ==",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[cfg(feature = "user_minus_2")]
    #[strum(props(
        svg = "eJwlykEKgDAQA8CvhNxFd5Gi0O0PfIRUQUFExEP7e1slhxAy/pqfDYtxkh4yzg4OXU0jio7BtxUEH/c7HitiMg5EzMaRuI19Jf8Z/LGfK5IYVYmkRnFELluktH5deGXhBYB4H2I=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserMinus2,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "eJxNi8EKwjAQRH9lmHvQXZaIkM3Zix9RYsFCESlS0r83aS9lDsNj3qTv8Hvj5XxKhMoadDAYrj3Bgj3imWGrMqdLP+VUpqXMI8rmvBGL04hSnfeuHGNO8/QZUcWpSlR1SiS2xiKtde+mdy3/AcnDI8M=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_plus_2")]
    #[strum(props(
        svg = "eJw9zFEKgzAQBNCrDPNf6oYQKmRzgx5CUqGClCJ+JLfXTUT2Y9mdx8T/tH/xUb7FQ8YpIGCweYjDwBSfBlLMy5bXGbkqRyIX5YvYlN5ID1Ncl9+M4pRymirNVDs9UaS9T23qsi2Ubm0bco69I9z4AOaZKnI=",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserPlus2,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "eJxNi9EKgzAMRX8l5L1sCaGb0PR5L/sI6YQJMoaItH9vQ1UkD5ebe07498sXPopv8sC0Ou4FBO52Tpy8/LWDrIwx3EyKIY1zmgaYFQUhFcVHjazYGdLGGKbxN0BmReoQMrUs1qtUan8abdTB1h9z24iwuX536IQ37xUu0w==",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[cfg(feature = "user_square_2")]
    #[strum(props(
        svg = "eJwli0EKgCAQRa/ymX3UWEQL9QYdIlRSaBEiZLdvNAbmL957+j5KhDe08wbFx4oVU7uBFSayemyC1S5ldwVkQwvBVUOsZF9ZbtKPrc7BFcSQzlgEbQQxZkLtP8tI9SRfYqcStsB+NXAkBw==",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare2,
    #[cfg(feature = "user_square")]
    #[strum(props(
        svg = "eJxNjEEKwzAMBL+y6B4aqdD2YPsHfURQTGXooRiTtr+P7FyCDkLamQ01a8M/0pXwLWuzSPwg/Maj+hKC5fKyNoIULl1IQUvVd4Y6wY7Uwav38NyhI07hszTDGul5h/A2ySIQzD4MmcRu5xuySXe7k3aMqCgy",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare,
    #[cfg(feature = "user_x_2")]
    #[strum(props(
        svg = "eJw1zFEKg0AMBNCrDPNfalKxFTZ7gx5CtkIFKUX8cG/vZlXyESbzSPgP6xcf41taSD906ND43ETRMIa7gxjStKR5RMrGnkib8UUsxtbJUcYwT78RWWq3qVG17BLlSeSS5eHa1WWPY0UV64nrj8vuu9AqQg==",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserX2,
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "eJxNy8EKwkAMBNBfCXNfNDFUhc2evfgRZS1YKCJFSvfvTbBCySEM8ya/+8+THoY7dyS8JOmVlI5xSZPeun0mXQQlH2JUch3nOg00GxRUm+HsbzVcg/zKkqfxNVBjwwW0ikHEv0d23DzzKXSozUYZaCv3W9/87RfAQy6j",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "eJxNizEKgDAQBL+yXB80x4EEcqltfEQ4BQULCRL09xJswlbLzMQr3ztWpcUHsK+Os0AwtjlxMof+QypTikOLUrSj2LnBXqWJYI+SZ0JRkub8NH2NjBjF",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_2")]
    #[strum(props(
        svg = "eJxVjEEKgCAQRa/ymb2kIqLgeIMOIVNQ0CKkRd2+LBDirT7/8dJejgUT02gcTCweHrqhjIWmnIYm5CRrlW2GXEyRICdTIFQm15TvzKm3rP23vPJwDxrm3aGHb0bnH+I=",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    Users2,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "eJxljsEKgCAMhl9l7K7lXFmgnrv0EGFBQYeIiHr7ksCCGGOMffv47dJtI/QOW1UCqV1Qx8CQxxIsuCm/O/BO6G0Wn7wN0xrmAVaHjBAOh/U9TocmIs/R2+Qn+vm10LIyr/CbRUulE6ruNtIUCb0AdwYtjA==",
        categories = "account",
        tags = "group,people",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Users,
    #[cfg(feature = "utensils_crossed")]
    #[strum(props(
        svg = "eJxtTjEKwzAM/MqR3aotJVYKbl6QfqCbaYcOMXTo/6kUUggkSKfhdHdS+dTvG69b11IGByaBoQoEcaueeEk0wrDjjUWcmTF2U7l4ylT+Wfc0wFoszFBdusq3yovaxvAkJQXDhx2I8+p7NBcp9BjcmBLYn8nUh0xyIklXDGHv/gFfZjQ5",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere"
    ))]
    UtensilsCrossed,
    #[cfg(feature = "utensils")]
    #[strum(props(
        svg = "eJxtjbEKgDAQQ3/lcG+9u1qLUPsHrh3cig5dBAe577dV6CCSIYHwEn+mK8M+d4sBFrchkCY9AVflIVXHR6w4chd8X4ngG+cKx/hTMAHZyILJgn1HVEkyfk7MepRKXJu4AbKYJM8=",
        categories = "food-beverage,travel,maps",
        tags = "food,restaurant,meal,cutlery,breakfast,dinner,supper",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Utensils,
    #[cfg(feature = "utility_pole")]
    #[strum(props(
        svg = "eJx1yTEKgDAQRNGrDOmD7oSwCGtu4CEEizSCheT8mibVht/9Z8/5Vlx7OIRg4xqKLf0VG0Lk6kJCanS+Tr7MgOLDLRtyVGj8G/wBPT8zCw==",
        categories = "buildings,home,sustainability",
        tags = "electricity,energy,transmission line,telegraph pole,power lines",
        contributors = ""
    ))]
    UtilityPole,
    #[cfg(feature = "variable")]
    #[strum(props(
        svg = "eJxty0EKgCAQheGrPGYvMWqRoN6gQwQFBRGBLfT2KUm1aPHzYPjGHuO5YHI09JAchBYqZ1Ajb5sivH0cd1BBQ0HDiNrLtnWfEaUjQ0h5uCVEvjdxOWdaUKVfk1+i/KMXZmEpWA==",
        categories = "development,maths",
        tags = "code,coding,programming,symbol,calculate,algebra,x,parentheses,parenthesis,brackets,parameter,(,)",
        contributors = "danielbayley,jguddas"
    ))]
    Variable,
    #[cfg(feature = "vegan")]
    #[strum(props(
        svg = "eJxtTTkOgDAM+4rFntCkJYBU+AGPqMrAyMD/RQCpE7JsefCRz3Id2JduU2hRY8MrwSGQAA2VZzKeFMIDzTwgkaRuzf1TXXMbEMNUk/eM1GnknnzOaX/xkZMgshV/ke8vICK27A3wxCPm",
        categories = "food-beverage,sustainability",
        tags = "vegetarian,fruitarian,herbivorous,animal rights,diet",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Vegan,
    #[cfg(feature = "venetian_mask")]
    #[strum(props(
        svg = "eJxtjaEOgDAMRH+lwR+sXQaIgcZgEbgFxCSC/w/tEsgEuXS7Jveu8Up3pnNqViGWFCiQKzI3qsyzeqk3CFVJhG3ICOkNONRx3SCL7M0cOzs2x+9kT8wHt1bkST9Pos0O3ga/CI/GoEAwSB9RWYNODT2Cby7u",
        categories = "account,gaming",
        tags = "mask,masquerade,impersonate,secret,incognito",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    VenetianMask,
    #[cfg(feature = "vibrate_off")]
    #[strum(props(
        svg = "eJxti7EOwjAQQ3/Fyn4ldyRRhjR/wMqOAlKQADGgiv49d1SoDJUHy/ZzeZ5eHefR3QUZAiGB/NzVsrO9lpVS7B/BJnbIyBP75jHEOIQINvXUNMKD6dsRT7R15gT2wz4cU/OkD6OJTZ2C9uvldn1cMPPoxOEtauZLnJeoqEH1A3aNOD0=",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "eJxVyzEKgDAUA9CrhOwifhQc2t5FbLEdBCkfrLe3HUSdQsiLORaN8Ja7YIZAOoE8SWf6tjvzqsq+BD+Ww6o4k9doORMxpC2q5TASl+VE5FIbUdpcX827G3q9Icw=",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "eJxNjLEKwzAQQ39FeHfqU83hwfGcoV07dAu0kIIpHUpI/76XEHDQ8CSQlD/jd8Kjd1cJnSp0kDgSRDCJkTO7c6wCuZBIc3Iln9ZRyW2qEG0rb26Ix+x5S8dXz4lVzIZ7u6uv9xMLe0c6/HYuYrS4waprqfwBCjIqnw==",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "eJwdi8EKgCAQRH9l2HtEIuJB/YwO3SKl9RCELFR/3+phmGEeL9y7MHKkyxj4ycFCs/qNUpg7S6GVQ/DULBxpsYT2RTJa7ygu9WRRoluBI4xf7e6lH/z8Gi0=",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "eJx1zkEKwjAQBdCrfP4FNEEkiyQ38BAyDU7AhYSAevtO2i66aHfD//Pgx1ak41unron+SmipL+2J7k60n2XEP/FGLHeOlwFy/Dy7Ykp8eAQ1Z8WIcpTa5F0g9h4IMesMtw2v7Y4HOK/hkI8FJ34GknUzug==",
        categories = "devices,communication,connectivity,photography,files",
        tags = "vhs,movie,film,recording,motion picture,showreel,cassette",
        contributors = ""
    ))]
    Videotape,
    #[cfg(feature = "view")]
    #[strum(props(
        svg = "eJxtjLEOwkAMQ3/F6p5yMRduKZ27sLKfysCIVHQDX09SJApSFVmKk2cPj/q843buLgblwt6yiaGIzbnPlpFQYKFF4nmCiZtZ4mtI4mTo1Y3DIbrG4duohB6rQr0klMQPn20d7oXoRGmsBFdIPcTJfr2wCfej5bqRKcjpz4NtC74BR305uQ==",
        categories = "design,photography",
        tags = "eye,look",
        contributors = "zenoamaro,ericfennis,csandman,karsa-mistmere"
    ))]
    View,
    #[cfg(feature = "voicemail")]
    #[strum(props(
        svg = "eJx1zDEKwDAMA8CvGH2gOJTQwfZnTIZA6JAp/n0b2kKXjJIOidfurZCHghOoK3aQD0WGyfasJkvFx5+1ehYKvusMGjxPKNIb06ensgsfPiEA",
        categories = "connectivity,devices,social",
        tags = "phone,cassette,tape,reel,recording,audio",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Voicemail,
    #[cfg(feature = "volume_1")]
    #[strum(props(
        svg = "eJwtTFEKgCAUu8rDA5gvfJqg3qBDCEEFoUL+dPueImODjW2+luc7S4Za7tzeIBCBwICDdRC7YeEYXVcS0S9zFH1N7YIjiB1JkoZNapOIDxQDmVYqOwbciz/+Exru",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Volume1,
    #[cfg(feature = "volume_2")]
    #[strum(props(
        svg = "eJxNjFEKhDAMRK8SeoBsZzfRLVRv4CEKggpiC/rj7Y0iImEGMryZWPK8D3mhkqdlWxsHkFJFgb6XcD5mFiOcrq6Nn7vUxpK2kfrGdVBWoT9LldQGvB1MNfv6Khj3poPlJBx+CQb6h4cw5CkcR2kmRg==",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,speaker",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Volume2,
    #[cfg(feature = "volume_x")]
    #[strum(props(
        svg = "eJxli1EKwCAMQ68SeoFRQUFQbzOGICpsH3r7tbLBYB9NaPISeivzaBW95XqdkZhh4eBh1rE+IhKzV7WUwvaMUii57hhGZo4w1a04R/KEIWaM4kp9WAlX+d+87A15eyYs",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    VolumeX,
    #[cfg(feature = "volume")]
    #[strum(props(
        svg = "eJwti1EKABAQRK8yuYBWUQrXkZJV/Li9XfmYV/Pqpcn9VB6Y3MZe2RDBIyDCvZEegWiKSm9Ksj8qFxX0EBo=",
        categories = "connectivity,communication,multimedia",
        tags = "music,sound,mute,speaker",
        contributors = "colebemis"
    ))]
    Volume,
    #[cfg(feature = "vote")]
    #[strum(props(
        svg = "eJxtiyEPgCAQRv/KzX7Id4MxNiRTrAYb00BxMzh/v1zQ5F5876WzXo32aTgiQUhx7IacRhU5vXr2FDbLMDCRe8XSYKvmtgMdb0jxS1h/ZhFCLPKZB93vHd4=",
        categories = "social",
        tags = "vote,poll,ballot,political,social,check,tick",
        contributors = "ptrgast,karsa-mistmere"
    ))]
    Vote,
    #[cfg(feature = "wallet_2")]
    #[strum(props(
        svg = "eJxNTKsOgDAM/JVmfrBrttSUaQwWvwRRiSB8P8Vsy4l75U7v9hhdWzgghGxLQqi6/mnV3gmJgRsTU3LAmV+kEURXe5l95LPMg8iG3K8/zgUbpA==",
        categories = "account,money",
        tags = "finance,pocket",
        contributors = "danielbayley"
    ))]
    Wallet2,
    #[cfg(feature = "wallet_cards")]
    #[strum(props(
        svg = "eJxVTcsKwkAM/JVh741Ntqst7PbsxY8o2+IWPEhZUP/eREEsgZlkHiRuS67YnsmJg6J3eKxzLclx71CW9Vrqd3+ZOcaDFcZ4n2rBnNzFY5gEglaHIY0U7v4FiJUsvisxF5+ptwwdyUOIabixYVbFVHTEHw50QmteoyaFhoLKXg9rKZ2Ffz/edd4xsA==",
        categories = "account,money",
        tags = "money,finance,pocket,credit,purchase,payment,shopping,retail,consumer,cc",
        contributors = "danielbayley"
    ))]
    WalletCards,
    #[cfg(feature = "wallet")]
    #[strum(props(
        svg = "eJxtzKEOgDAMhOFXafALbbkBYkxjsAjcEkQlgvT5GWZBkBNnvvzpKrfRuXSbConu0xqLkhLXCXGACRxdTv0Lc2p8oOiCZrm+mowe4g+WubY/lgkGDziafQAcGSIR",
        categories = "account,money",
        tags = "money,finance,pocket",
        contributors = "mittalyashu,ahtohbi4,ericfennis"
    ))]
    Wallet,
    #[cfg(feature = "wallpaper")]
    #[strum(props(
        svg = "eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2RoW2h7Isw8x3+EK6jul2QXpF8YL0jtIJxiiUPuz+sA+P4ZlxjnLvoC0aq6b8QBCunILWq3V6IKH1qZ6JMzTcV+sOTuqWoSSzNuvB/NRf5SI+elCz3wBalO1UzeQDsgEytQ==",
        categories = "account,devices",
        tags = "cover,lock screen",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wallpaper,
    #[cfg(feature = "wand_2")]
    #[strum(props(
        svg = "eJxtj7EKwzAMRH/lyB7Xkh3bATdzhvYHugU6ZGihQ8n3V3JJCIkRHEZPvpPyZ/rOeF6bN5MJHk6kJcOpyCRCKGK1pBkZ9sbGBVCS2eOElM689DdWC33soBqUsE5sDvwf8WiGfNHNhrztRx4RDu6M7h3C4it96kG+Tix44QqISGMtQo8Mc1v1Irix38APsIlS9A==",
        categories = "design,gaming,cursors,photography",
        tags = "magic,wizard",
        contributors = "karsa-mistmere"
    ))]
    Wand2,
    #[cfg(feature = "wand")]
    #[strum(props(
        svg = "eJx1zTEOgCAMheGrNOwgLYI0QW7g6m7iwGLiYDy/4kRiWTq8P1+azu0qsM9qQQ/jSiqnoU45tQHDraUUgYu0k+0EnEwExHoY0MnfuNgeDYaq9P9+OCAE1ixRetlHsaEPPOZELw==",
        categories = "design,gaming,cursors,photography",
        tags = "magic,selection",
        contributors = "mittalyashu,ericfennis"
    ))]
    Wand,
    #[cfg(feature = "warehouse")]
    #[strum(props(
        svg = "eJxtjrEKwjAQhl/l5/bE5JqGDEnAzcW1g1uxxRQ6iATUtzdBqS2UW767+47//L3PCUOgMzOcbNqOVc9gqFJaFDqZdS+4q9ZxGaGRbGFlOztR8O9CS+OgZleNlf/LuVD0hxoe/fKChXZJ8/7GbDaP8ZrxDqQV4TkNORVkQhqnW8pffgWy9aCq8QOOXTfT",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "watch")]
    #[strum(props(
        svg = "eJxNjtEOgyAMRX+l4Z1O6lZ4UP/FMBNImBrnw/z7FYkboaHJ7b2n7XzcfJrAf3plSIE/St96xWrobmU8dOuSjhTnCdYlzvs7m8A0kH+pViq7L5f4xz3As1cvwyhji8wandF3bB4jAUGTnyZtkE3QhOxqGbKcNFoHOXGyhVhxLToHAm8ZxfPPQskGQku1ei5K1wk/4BdvIT54",
        categories = "time",
        tags = "clock,time",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Watch,
    #[cfg(feature = "waves")]
    #[strum(props(
        svg = "eJy1jT0KgDAMRq/y0d1ooq0KtYuzh5A6ODp4f0w6CV2VEMjL34vXfp84FrcJQqZAHkwChli1jrDwmJV8FgrodDJAtCdlx9g3yppMvTKTbZd7l2JrghRfGpbKU336yDT9aXoAe60/UA==",
        categories = "weather,maps,multimedia",
        tags = "water,sea,sound,hertz,wavelength,vibrate",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Waves,
    #[cfg(feature = "webcam")]
    #[strum(props(
        svg = "eJxNykEKgDAMRNGrhOxFGwVdJL2Bh5AoVHAhRURvb6QoWc68z7pm3RbQWzA0CFlwQNDLFmHkunDkLyvg89Zn+3QkmAXHHoiSFWbv5ySQ0Vl1Pz2WgyT6",
        categories = "connectivity,devices,communication",
        tags = "camera,security",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Webcam,
    #[cfg(feature = "webhook")]
    #[strum(props(
        svg = "eJxtz0sKwlAMBdCthM5zfXn/QC2IYxdR6qATwYH7x9uKOimBQD4ckvE5v1a5n4ebdbEK76sWuC9qMAnMzjJrROYcfsmSJTBMolhbEEzRELmYUZrGYRpPmzmNX/lRuSgJlii3vqAkhTcYUeuKopzNH9cYFd01I9QDyqLUnRJS6WoFlXpE228X9rc30vy/Mkj/OW9lxjQU",
        categories = "development,social,account",
        tags = "push api,interface,callback",
        contributors = "karsa-mistmere"
    ))]
    Webhook,
    #[cfg(feature = "wheat_off")]
    #[strum(props(
        svg = "eJx1kstuwyAQRX9l5D1TZnjYSEm+wNl2kZ3lVnIlt+qiqpq/7x1bcR/BQswguFyGA4f34WOip2PzqqRK4p345nR4sOnTYVuUTJ0TlpYs3AvOgSPWlFOgRCKz2MjCEDiRdY8m6JFL7KEps7N1VxM5E10qdXQoA/r/W/zNt5CkebMdOSXIE3dWvCYulkutfvFchBJrHlmwSTOHiL2wyY7b0EugsHsr1g7nZ60Yqyed9FOHSHFVO4wmp4/5Z4qiq133LLJgbZcYNmaVEhZi5Fewe1xXzaX+vpJrZDfjv2ShiYWWsFobVge+tUt03EaUz76M4OrAF1wd+IIrzsy9in2Zne9gnyErXuSX9/zy9kxXPTaqDX3dsiA3dF0SpCY6fQPD6qX0",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten free,allergy,intolerance,diet",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WheatOff,
    #[cfg(feature = "wheat")]
    #[strum(props(
        svg = "eJydkDEOgzAMRa/yxZ4UOw4QiXKCdu3QDakDA0OHqudvnFR0SQGhyJZlf/k5v3+OrwmPc3VlMIMadNXQn7Q79MvMWWlBbL2DB9FMWmkanfXQqOOjGGKDXKImzEbnpiQyKroXMK1iOt0d0G5BAsgfgRApRdJyB7eFiZr44QMcrsETv3kUSBabWE2Gb82vBTErR1IyRC/42lm4M/FRZ8//XZk1RZDPiIxbLF0BJd8PgIIiUmJaLF3hZOP3gj5MWZvt",
        categories = "food-beverage",
        tags = "corn,cereal,grain,gluten",
        contributors = "karsa-mistmere"
    ))]
    Wheat,
    #[cfg(feature = "whole_word")]
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l2HtqJrVGIcnZix8hq6DgQYoE+/dt2tJS6GV32Tdvgr5b/TyhXRQ6gf6jeEEbpZYUDhNN4Xv/vfCIcqPFJZ8KKq8UZn3Mb0ro9/0jfD6v/gKcA32mWlSNqRrQELzWWm5rOKxhMBsubg9bFTKk",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "eJxtkE0KwkAMha8SZt+YZCbpCG1v4CEGFRREXLiwtzfTH6lQsgiB7/HC1z3uzyuM0geRAB/27Ws5x+kcukOFhu5V3je49OGUUYENtSgokA9DC1TBimxAgYxZCju+ggm5bQRNd3AmNAM9JyRuMJqnOeERmDEmiNjaXsgwa0VEC3sDLUWCIsBoeSfjfPyDFSX5V9uCSUw1weLvzG54lUJhdkY/O1/ftU5Y",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[cfg(feature = "wifi")]
    #[strum(props(
        svg = "eJxtykEKwjAUBNCrDP8A8c/XaBZJbuAhAgoKIi5ctLfvT2lLF2VmM8PLv/Z/4VHkHsFzo8LbQ/AClZpPXdS8uRRcXkNsEXGRt0NoSCFZo/MVmu7l5/19YmARmmC0IqaCwfoOSr84X+67rBPV4Ski",
        categories = "connectivity,devices",
        tags = "connection,signal,wireless",
        contributors = "colebemis,ericfennis"
    ))]
    Wifi,
    #[cfg(feature = "wind")]
    #[strum(props(
        svg = "eJxtyz0KgDAMhuGrfPQA0cTaH6gFNxcPUXBwdPD+mAwWBwnvlOcrV7tPHIvbOVKE1oRmWCPYjhI8TZu4WgbDtfRJpqC/sArk1Yz0S1nUcibfsebB4aMfD3kgyQ==",
        categories = "weather",
        tags = "weather,air,blow",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Wind,
    #[cfg(feature = "wine_off")]
    #[strum(props(
        svg = "eJxtT8sKwkAM/JVh742bZB9ZqAXx7EeICiuoeBDRv3drVTyUkMlrMiT9dXur2C/dxiBSzQ39YmwN/W+Qwb7qOcPXjkmDznBYwPGe57ZJfcYbVyykign920btbUT8lqSFEUbcGCVtN1ExXhvl2NKUDEaWpeUaUCA17ZgiAgSpuTUNCj535COZLx1xiGDi8vfY6Xg54CFLJ+Lw4BYdnp8wdRt1JA0v0UZCoQ==",
        categories = "food-beverage",
        tags = "alcohol,beverage,drink,glass,alcohol free,abstinence,abstaining,teetotalism,allergy,intolerance",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    WineOff,
    #[cfg(feature = "wine")]
    #[strum(props(
        svg = "eJx1jLEOgCAQQ3+lYT/kiCeYILOLP+BGcGB0MH6/6EAcNJdL076mYU9HwTapxcPa4lUM3R3F0IADm8Lmg7AFy+n+SBIIzHNCkg1Z0kJ9FT+PmVgLqsFQ37+7kLVNXnRpKBY=",
        categories = "food-beverage",
        tags = "alcohol,beverage,bar,drink,glass,sommelier,vineyard,winery",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Wine,
    #[cfg(feature = "workflow")]
    #[strum(props(
        svg = "eJxVjUkKgDAMAL8S8gHtAnpo+wMfUWwxvUkpLr+3yUUlh4RhmLia1wZnSY08zgiXR4NAuWzUBNwCaucagxtYD26PjSB5XCZQ6rBRg4ZRpl9kWWQlOKl/aj2jjER5vW///QciSyjq",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "wrap_text")]
    #[strum(props(
        svg = "eJw1jkEKwzAMBL+y6F7qtVOTg+0f9BGBFhwISaA5JL+vZLdIMDoM2k3LvL5xMksQXIqo8A2nwlNKuptT0j4dFa8szwD6yscUEOBAHYdYb4OpJqm6LVd7vG/zenyyMMJ2AEc7vGvuzyqpl9A8uh7Psbcx9nL/Gl8Yky6A",
        categories = "text,arrows",
        tags = "words,lines,break,paragraph",
        contributors = "bduffany,ericfennis"
    ))]
    WrapText,
    #[cfg(feature = "wrench")]
    #[strum(props(
        svg = "eJxNjrEKgDAMRH/lcE80tqQItX/gRxQcHDo4OPn1Nq2gBF7gchcunvk6sK/DJp4DlF0WCKZ3hH0R1rr1p1cVU3EcAhmyQrtOgRcPQyHlRWDIM8uMhm5y5IodmuULtxz1cH2rMNxDiqN1TA/GsCOZ",
        categories = "account,development,tools",
        tags = "account,tool,settings,spanner",
        contributors = "Andreto,ericfennis,csandman"
    ))]
    Wrench,
    #[cfg(feature = "x_circle")]
    #[strum(props(
        svg = "eJxNyTsKgDAQRdGtPKYXjWBgYJK9yCgoKEiwSHafX5Pqwj2id9DnRHBkFoLG0rU0tXqZu3v59v/C4eg1G3iysBXrHIjBGCUDxpwZzw==",
        categories = "maths,shapes,development",
        tags = "cancel,close,delete,remove,times,clear,error,incorrect,wrong,mistake,failure,linter,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XCircle,
    #[cfg(feature = "x_octagon")]
    #[strum(props(
        svg = "eJxNzEEKgCAQheGrPNxXKDUlqHcJggpKhdx0+0QNXQ18//CUd9e7OwvvThsezeZ+IQhw6vkYrxDI8lMJletvkszMqKEsG+XXcGDT7OYTZEegVCM2SUKiLR8PxSW3",
        categories = "maths,shapes,notifications",
        tags = "delete,stop,alert,warning,times,clear,maths",
        contributors = "colebemis,ericfennis"
    ))]
    XOctagon,
    #[cfg(feature = "x_square")]
    #[strum(props(
        svg = "eJxNy0EKgCAQheGrPGYfUZEkqHeJlMZFEDKQ3j6mNi7f//FcSYeAUz5ZPE0boXpaCE+Own8ozdNMaF8vVUdwo/6Cu3dhRE/XtMIOBkZJY0cWFr28U4sf1Q==",
        categories = "maths,shapes,notifications",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    XSquare,
    #[cfg(feature = "x")]
    #[strum(props(
        svg = "eJw9ybEJACAMRNFVQhaQWEiKMxs4hGBhI1i4P0ZBuav+w6yrU8tcRCn5RNkQTjU8Gxei/9sGq2sO7w==",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,times,clear,maths,multiply,multiplication",
        contributors = "colebemis,ericfennis"
    ))]
    X,
    #[cfg(feature = "youtube")]
    #[strum(props(
        svg = "eJxtjkEKgDAMBL8SvDc2tVGEKvgAH1Hw4EXw4MnXG2uqghKykOwybFjjNsPUFaNDBmqi80gOLrUyBNaQBXnpSeiNLPgWuVZVp0YJDU/UkTB/kfBGJlxGsqo6H2RqWfShPIv3IddfBEgMbCoju9+BAwNZLQw=",
        categories = "account,multimedia,social,text,brands",
        tags = "logo,social,video,play",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere,jguddas"
    ))]
    Youtube,
    #[cfg(feature = "zap_off")]
    #[strum(props(
        svg = "eJyFyzEOgzAQRNGrjHyATXaxjZEMt0mBhAApKeD2GQwUiIJmZ4v/8jwN69CPH8xTP/6+rVMTr4hSB2gFg74l1PDSmOvy68y7fIdpC8kbhSkdNEiMfB5gQkIF9bTlkvMhjFdY0KKtM4f1GONwl32Zl/QPTDw6gg==",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ZapOff,
    #[cfg(feature = "zap")]
    #[strum(props(
        svg = "eJwti7EJACEQBFtZbOB/1w992xFBPEETu1fEZIJhJjQrM1lFs1xH/x09BA9+oA4JCSL4HrO5CxfDc8+4AF5nEOw=",
        categories = "connectivity,devices,photography,weather",
        tags = "flash,camera,lightning",
        contributors = "colebemis"
    ))]
    Zap,
    #[cfg(feature = "zoom_in")]
    #[strum(props(
        svg = "eJxdjrEKwCAMRH8l5ANaIq04qD8THATp4KR/X2NCh07H3b0jiVw7twI8ExIh9IQBgcd2OZ5a59jqU2C6FfvD3wiDErrFT9PxVWsksE2EI+OCYWJFrz+roZakN4JO9ZnNvkGULXU=",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "eJxFjEEKwCAMBL8S8oCWSCs9qJ8RD4L04Cn5fWNC29PC7sym2mcdDWbGC6FKRiJNtixp97mk0e8GHLSOWzwRhDIGJeWv2CuVFvwqZMeGkWsr7en42AdavyJ1",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
