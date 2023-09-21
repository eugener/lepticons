
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideIcon {
    #[cfg(feature = "accessibility")]
    #[strum(props(
        svg = "eJxtzUEKwjAQheGrPGaf0WkykwhJb+AhShQUFKS40Nub6MJCyyze4v9gcr3O9XZGfRUSI8xtCPVdKNCYd7865sf0vOBU6C4JcoC46AzSRS+LrkjwzkNZ2w7sDZ51DY+BhwAJrJNCsf+ecUqwDSyeY/sXl9p17f76A9/tMq0=",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[cfg(feature = "activity_square")]
    #[strum(props(
        svg = "eJwdizEKgDAMRa8SshdNRHRoO7t4CLHFFBykBNTbazr99+E9X/Ou8AQcEOo/jPC2I7kcogFpRrhLUmkYfWdB9NemAingShMQi+PTMYyOHfUGy2SqSfEDOWsaRA==",
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
        svg = "eJxtzUEKgzAQheGrPGaftDOTJikk3sBDlLTQQgURF3p7DYIudDXwvh8mld9Q/h+UORMroUzrFcKQKVKTbhs3qX+NX7wztQ8oBL5a3Q7pZJ2NGj1T661GcLQBDsIXAQfrXS18gNwvm+4Jrq8FzridF6nOMts=",
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
        svg = "eJxti1sKgzAQRbdymX/TzkyapJC4gy6ipAUFBRER3b2vH0E/7znnxlz3ufkjz4lYCX2iQMjTuoTK+Dh0GbvvUOGX6MOC92gbwa43fJIvKATualpZcaGF3pyc0QAOxsNC+CZgb5zdCuchz3OzAJRRMvs=",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[cfg(feature = "alarm_minus")]
    #[strum(props(
        svg = "eJxtjUEKhDAQBL/SzN3szoybZCHxBz5CoqCgIOJBf6/Bgwc9NVQVdEjDksYOaYvEQkj7uUpYInmqwufSVZibtUcbqf5BIbDZZXabSU5caKFPVVujHuyNQwnhl4CdsWUurIN835s/WPv7+ACDmDIA",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmMinus,
    #[cfg(feature = "alarm_plus")]
    #[strum(props(
        svg = "eJx1zUEKgzAQheGrPGZv6szYJIXEG+QQJS1YaKGIiN5egwsX6mrgfT9MyJ8+f9/IcyRWQp7WK4Q+kqc23DZuw/85dHhFSncoBLZY2Xb5yTpXWumRkjXqwd44NBA+CdgZ25TCOkh90Qi4Hk8+pwdYux0W+ZE5Xw==",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmPlus,
    #[cfg(feature = "album")]
    #[strum(props(
        svg = "eJw1i1EKgCAYg68y/guEGeSDepmSFETFhPT2qRGMMbZ9MpujwBp32aKICUJuilbC485iv6Yq4oQ2PdexarkMTssUffMuGKToQrn7n4Gj29AGAbbPuIMP6L/rF+0KIPU=",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[cfg(feature = "alert_circle")]
    #[strum(props(
        svg = "eJxly7ENwCAMBMBVLA+QYAqUwrCMRYGEUlDh7UMwVFSv19+zlCY1g/SI5BHaCIcgOmvi2/bEtbwZ1JvqK5UiPqPS1r9a1szlyBgFtHs4/Acd6SH9",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[cfg(feature = "alert_octagon")]
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yzAGsGcJaqLeJEESFWujts9Rs9eD9N6NicPkIHmKw/jo1rmKTwEBS0FLIDNV01YahR/uaqtGoqX02ylm/QyaNG0IqIEbIXJkqS/5Uv5ZkH8VMrZfffe9vN0st5Q==",
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
        svg = "eJxdzEEKgDAMBMCvhHxANqKtkPY3HgTxXH/v1hQPnsIms/HzuHZpVnRWuVF0VWkcBkbrsfrUTfWQvCHp20jRgAXl/Fkul2FyvMU2bP7sA2kWILY=",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[cfg(feature = "align_end_horizontal")]
    #[strum(props(
        svg = "eJxljEEKgDAMBL8S8gExFKHQ9OzFR4gtpjcpAfX3tgHx4GnZZWdCzZuC5LKLMo4TwsXoEGoLQjhLUmFs8917DEP/x2DU7/NqvFlGZ5T/qGNVgcS4EAHRbL6+xQcikCXW",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[cfg(feature = "align_end_vertical")]
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrhHcBMYhQaHoDV3exxXaTElBvr80gDq5/8j1f06p0lKhZ0I+gSzCA6ilgUE5lyyp4uoXgu/Yf/Fc5Ozqj/Z991b5opiiYmIl5tr3Wwg0WBSXk",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[cfg(feature = "align_horizontal_distribute_center")]
    #[strum(props(
        svg = "eJxtjMsOQDAURH/lZvaCUt30+gNbe0HUTqTx+HstIZV0M4s5c0avY2/JjPNkLCMvQQfD5T4P1jAq0MmQoNXVArVO/b7Wt+WICpffS/bu/Zn7/LSls4YGRpMrEmJLpEe+/CPVigh4lCJKZGBc0gc7uA==",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[cfg(feature = "align_horizontal_distribute_end")]
    #[strum(props(
        svg = "eJx1jMsNgCAQRFvZTAMC8XNh6cAijBDhZgjx072uHjQmHmfezLM5jIU2Rg3aGQ0on8GA1uRLZLSgGNIUC0PXcLaSvbPX63+pIEp9O7vnNg8lkmf0WpFZjBIi3YuYDzkAdVgtgQ==",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[cfg(feature = "align_horizontal_distribute_start")]
    #[strum(props(
        svg = "eJxdjMsNgCAQRFvZTAMKQb2wdGARRohwM4T46V7gIuE48+aNjm5PdAebPGMGPQwFehkTKOYgQd6FwyeGUDB6KHujq5VXSzUyaS56bfy1c0ueLGNVJC9ZQakaIDryAUPGLVc=",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[cfg(feature = "align_horizontal_justify_center")]
    #[strum(props(
        svg = "eJxNjEkOgCAQBL8y6Q8oRPHC8AMfYYQIN0MmLr9XSFyu1VVtc5iFYkhLFIbqQCejBx0MDdqTl8gwoFyBs03xna3VjZSpwfAI31P7q99snSSSZ4xKk950W5bC3AUosiYo",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[cfg(feature = "align_horizontal_justify_end")]
    #[strum(props(
        svg = "eJxFjEkOgCAQBL8y6Q8oE5cLww98hBEi3AwhLr83QyJeu7rK5rAVegQjKN8CBsWQ9lgEZgBdyZcomECVOdvp39lq/bApPbQ117vhL9m0Yy2RvGBhJj65V6KbewE/wiYl",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[cfg(feature = "align_horizontal_justify_start")]
    #[strum(props(
        svg = "eJx1jEsOgCAMRK/S9AICUdxQbuAhjBDLzpDGz+21JMaVy5l5b0LNi8BRkjChR+BcVhZC2yOcrbkIB4T6BIcxdMrH8GeZxo8vrxfWf9o2C0MinBy43RkdtIo3Lssl+w==",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[cfg(feature = "align_horizontal_space_around")]
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkM0NbO3FBDedhAX19prOVAPzvi9pN7oFM6h8wyBN+VAT9A505WgqGEGPYELwXe2DPzdTioJlIOaVK9TrB+xaeQEtvh2e",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[cfg(feature = "align_horizontal_space_between")]
    #[strum(props(
        svg = "eJxVjMkNgCAURFv5mQYU3C58OrAII0S4GUJcuhdMVLzMYd68UcHOkcLBkCBn/eIiQ7Sg1DSgk9GBdm+iY/TQqsp7rW7rndd4HlKKUsgHw6etU3RkGGNDcpN1BrkqgBR/cgFVQC1U",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[cfg(feature = "align_justify")]
    #[strum(props(
        svg = "eJxtzDEKACAIBdCriBcIDaLBuk1DEM11+7KQlib538eXVnuBQQk9wuSEAWHsw7Qjaczi1GQ5UjviS/WavQsfG+33ZvnUZhdlfiCk",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[cfg(feature = "align_left")]
    #[strum(props(
        svg = "eJxVy0EKwCAMBMCvSD5QktLWQ+pvPAjiWX9vjBrwtLA7yzmV6Br98IJrqFElCCWlvSHwNUxgk0h7U4rPfEp9Wh0/s3r1y3qzHWczIKw=",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[cfg(feature = "align_right")]
    #[strum(props(
        svg = "eJxVzEEKgDAMBMCvhP2AJIJWSPsbD4J4tr83iW2hp7BhdvW+npMqZ2ygVzJWO5aEQVX8W3RxUzSkkwNRYJmp5dkGSv/s3kwanW4/aIcgrg==",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[cfg(feature = "align_start_horizontal")]
    #[strum(props(
        svg = "eJxNjEEKgDAMBL8S8gFpKILQ9OzFR4gtpjcpAfX3tqWKlyVkdtbluClITLsooxkRbsaSZwoq7coXIyGUtOjdUPveNauTV57+VkHG9rHPOlYVCIwLEdBMFdSXfwADBCWl",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[cfg(feature = "align_start_vertical")]
    #[strum(props(
        svg = "eJx1i0EKgCAQRa8y/AtUIkLgeIMOESmNu5Ch7PZhkLs2f/Hfe76kTelmTBZUKsOAJOVdlOFA9d0rRxXGjOCH5gf/VbbDyf32vTpWFYqMxZA5zdhAu8ID6D0l0Q==",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[cfg(feature = "align_vertical_distribute_center")]
    #[strum(props(
        svg = "eJxtjEEOQDAQRa8y+XuhpbpprW0cQhBjJ9IEtze1UIuuJnlv3nfHMgU6Lg8N4mVbOXi0IAEGdG5zYA/VgO73dK6MQefeTFiTnirkd2yq9jEwzR6D1mS5MNFE9jOWbK8yXAolSZ1RRkyvP/EAwk87fw==",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[cfg(feature = "align_vertical_distribute_end")]
    #[strum(props(
        svg = "eJxVjEsOgCAMBa/S9AIC8bMBbuAhjBDLzpAm6u21GInsms6bsTmuDKfDAeFyqHuEIwWm98wPMAgU00bscERvOxG8Ldq3VP+JxKYSq4Fq7QsTBIezAaPIKCHya4huyA1vwi1l",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[cfg(feature = "align_vertical_distribute_start")]
    #[strum(props(
        svg = "eJxdjFEKgCAQRK+y7AVKsfpxvUGHiJTWv5CF6vZpEWKfM2/e2BRWAQ5xYyEcEY7ohQmVQTgJB4TrDSknjc52RXD2r2U6faOimPrUV2tfhMETzhqUYf2Q0jWkATdYHS07",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[cfg(feature = "align_vertical_justify_center")]
    #[strum(props(
        svg = "eJxVzFEKgCAQBNCrLHOBUsp+1Bt0iEhp/QtZqG6fGgR+LezMG5vjLnSlIOygJtDtMINyORrEMR0sDgb0lNjA26ECbzs2Nra0Uq++nV+dmzAFh1WT0qzHmtSffwE18CYa",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[cfg(feature = "align_vertical_justify_end")]
    #[strum(props(
        svg = "eJxVy0sKgDAMBNCrhFxADX42TW/gIUSL6U5KQHt70y78rAZm5rkUVoXM2BGChLiLMo4IF+OAkCysP+OmYpcevWsK8K4yW6fn9MH5p9pXHYsKbIwzAZFQXUrnbyNCJhc=",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[cfg(feature = "align_vertical_justify_start")]
    #[strum(props(
        svg = "eJxNjMsNgCAQRFvZTAMqUbwAHViEEeJyM2QTtXsXLniaX964kg6hx2MBFRUDej0mC7pzFFY7gzjlk8XDIrihAsE1rPc/VoP6tR+Mnbp2YYoemyHDpg21Ch8CFSXt",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[cfg(feature = "align_vertical_space_around")]
    #[strum(props(
        svg = "eJxVyjEKgDAMRuGrhP8C1iCK0HR28RBii+kmJaDeXrvZ9X3Pl7QblVvAoCtHU0HvQJryoSYYQY9gBn3HhOC7+gd/bqYUBSszsVu4Sm2tDD94ATLSHYI=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[cfg(feature = "align_vertical_space_between")]
    #[strum(props(
        svg = "eJxVy8sNgCAURNFWXqYBFUQ3QAcWYYT42BlConYvmPhhO2eujn5JdBgo0B5cYoOuB7EPKyeDARQzCtCZQcHqpgRW31ne5Ht46raqs41ftc2JyRlMgkTHoi1StkrkHy5GXS04",
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
        svg = "eJwlijEKgDAUQ68Ssov+L4JDf2cXDyFVUBARcWhvb2tJSAh5LhxPODeEZByIx9gTIRpF6V1bX+/O49qQxKhKRP1vRKmd8h4LXSjv7uXdsRrnAaKTLtIhu0pz7E1f4IL5D4KyIK4=",
        categories = "transportation,text,maps",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Anchor,
    #[cfg(feature = "angry")]
    #[strum(props(
        svg = "eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vZpbopcDLN4b8bY2dllhL0aRpLBnrGdL8Fak0femq0/JgwN66gElTunTHHJixA8CXKQXmqVKdQgAf2HKxXQkP6FEkvtV5NIAFJfcgNEIjN7",
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
        svg = "eJxtj0EOhCAMRa9CeoAOBa2QoJchLkzMLFzh7YfSSXAcVz/59IX3U96OvK/mmIEsmFxquppnyyW99H1J+/ZeTaFaD+gJTHEzOIt2BHPWNtSoDU0YB8Hk/AJF5PhlCGn6YcLtXjqHlqGRE/qgJHkUNSFV7cZwY9ozK+ExjtD//9e6bsHgO9228BPByF1CXUk2q6LF0Od8AK76Vgw=",
        categories = "photography",
        tags = "camera,photo",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[cfg(feature = "app_window")]
    #[strum(props(
        svg = "eJxljMsNgCAQRFvZTAMCIcQD0IFFGCEuN0M2froXPGk8zrx542tehM4AA7oCLOgoSbhlBeJcVpYA7UD12UQ/dCH6bRamFDBpRXa3HfTqBQyN3E7+wH2EG0l3JM4=",
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
        svg = "eJxtzEEKgzAQBdCrfP4+NDOpUCHxBj2EVOm4KBQJtr19ExeiIH8xw8zjx3l8ZNg4PS0nNsQ3UYm5DCE+05CtHDzxSwzs4qX6Lr77bBgS71fcFpFeofBrymZaYSU7qP4oxVXpTuirhTQILqDkpEkUoku7ff4EHDOm",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ArchiveRestore,
    #[cfg(feature = "archive_x")]
    #[strum(props(
        svg = "eJx1i70OgkAQhF9lMj3Krm7E5I43sLUnQlwKE0Mu/rw9dzTQkGnm7wvT8Ej4RSrxjzwRUw5C+DA+PUUa8R375PlRsw3H8m/Du0uOPvJ2RvMR6RSKelF2LrotKr03hSzMSr6uB4NcYJXtjQrDOs4ByixT",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[cfg(feature = "archive")]
    #[strum(props(
        svg = "eJxNizsKhTAURLcyTP94uVcFi8Qd2NqLitdOJPjZvYmFyDTzO36bhogzUIkrsCC2FIQ4ljFaqh1h0zJbDKzY+H/+N37to2EMbEvUu0ivULhHyZnot/hpV2cyMx9SHEStfJcb+dgkIA==",
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
        svg = "eJxtjbsKgDAMRX8lZC/aWuvS+geu7qJiOghSio+/txHEDpIhcM992G2IBJPDtQJpQPMJja0tGLT2xV0DquwzEOYxwuGnSA41As1+oejQIFyPENJTCKdDWXOK/Vmd5L5dGBLqZ0zWidK3dgP4OS4U",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[cfg(feature = "arrow_down_10")]
    #[strum(props(
        svg = "eJxtzDEKgDAUA9CrhL+LVqsubW/g6i4q/g6ClIJ6e38FwaFkyPBIzDFFxmJpb6A66JRCkzNlAmc+HnrU1ZgD1UOJcFHnsBXk3yysc8Tpl8iWNOG2pKQuqZbAq984WuoIQeQ9TAP3ANk5LfQ=",
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
        svg = "eJxFy7EKgDAMRdFfCdlFqlAytJ1dXN3FFtNBkBJQ/95Ghy5vuIfnStoEOOWdxaMhhNvjiPB8e+Uo/OdS+4DB9XoI7lyFIXo8jAXqCEhJY6O5krETLc1evjUeuQ==",
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
        svg = "eJxFizEKgDAMRa8ScgGpgmRoOrt4CLHFdBCkBK23l9ih/O29/3xJu8LLOCFIyocooyOEUhlHhCdHlUaqfYIfLAj+2lQgMp4E/8wY62Z1M9BNS1cfg2Eebg==",
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
        svg = "eJxFyz0KgDAMhuGrhOwirR0ytL2BhxBbTAdBSvDn9jYIlgyB9+HzNa8CnMvGEtAQwh1wQqjtWYSrJOGvP9qjH3UQ/bEIQwo4Gwt0koKmDjtBI6c3uJ9f5soewg==",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[cfg(feature = "arrow_down_to_dot")]
    #[strum(props(
        svg = "eJxFylEKgDAMA9CrhP4PaRHGYOsNPIRUQUFBhoje3g2RkY9AXuIxngumRAML5OKeNHZ10/jLzgHBeXhX0tjWbNsMexIJE+xOxELIperpY30BDjsanw==",
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
        svg = "eJxFybEKgDAMBNBfObKLJoh0SDu7+BESBQUFKQ7699oKlhuOu6e2Rttm2O2JhRDfagh25Rm0/jzoMZ4LJk8Dd2DpXbL0FdlZ4KoWOT8/Lgcaew==",
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
        svg = "eJxFyzEKgDAQRNGrDNuLbBRJkVjbeAgxwU0hSFhQb69JoQz85jEux1UhMW2intgS8uXJEN52hLv2TEGl6ujachjdsaggeNrZwDY96goX+HnmAWwm+8kD6Eceww==",
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
        svg = "eJxFyUEKgDAMRNGrDNmLJoh0kfYGHkKiUEFBigu9vdaCZRYf5qmtybYFdnliIdhdmt50FLQtHvSYzojZ0+jAEl2mfFXYWcAD+ubb7w84fxq1",
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
        svg = "eJxFi0EKgDAMBL8SchdJFemh6Q98hNhiehCkBKy/l+agLOxhhgk17wq1MTqEuyQVRvIID+OE0Owll0PUeAxjD2K4NhVIjKsHcmKio1+c5IAWmAfb51/6kB79",
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
        svg = "eJxtjksKhDAQRK9S1D7MJBPjLBJv4CFExXYhiAQ/tzdZiC6kFw39qqvKz00UdIHTD39YZZGGlf/ke+UvWpewq3Y3WPo24gi0xJKWIfZAXRDSj4PEQEdsYxclS9JX1j/sdAnzXZUTZV7CdJGo3DVOzPkuCg==",
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
        svg = "eJxFyjEKgDAQRNGrDNuLbgjBYpMb2NrLKigoSLDQ2+sSMEzxizeiW9Z9gd6R2BH0Kc1fOkrSFk9yTteKOdLBAewab4O3h0n1gR04jP0vLzwtGtI=",
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
        svg = "eJxFyrEKgDAMBNBfObKLJKBFaPIHru4SBQUFKQ7697aIyA0H9y4e47lgUtobdAhVQA5ZrMtu8dOeBdwO8ouvybcZfisJE/xSYiGkXOX0sj3cARoy",
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
        svg = "eJxNyzEKgDAQRNGrDHsBiULYIskNbO3FBDedhAXj7XWx0GaK/5jQyqY4a1aJ5JjQeqSRIKXuom+6Ik2EbpvCYIcUjlUFOdLMcH5hYSOLP3L+MTA+uwHNVB6s",
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
        svg = "eJxFi0sKgCAURbdyefMIC+IN1B20iEjpOQhCHn12nzpxdD+HY3PcFa+jmZBLTISvjScFFUeGCRLTIdq6t2MVvL02FQRHK4OF70bq18nJMAt46OgHg+8emw==",
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
        svg = "eJxFyzEKgDAMBdCrhOwiqVI6tL2Bq7vYYjoIUgLq7TUdlAz5n8f3Na8CV8ABob7PINytcC4bS0ByCGdJwi1G3+sg+mMRhhRwJwtkulEPRnWV3ycyQHZ2nzz4ah8a",
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
        svg = "eJwli0EKgCAQRa/ymX00kyYu1Bt0CJmCghYhIXX70Hirx+MFPYqeG/SNJBOhRLIEfbqlMP45hSvfO9ZIizj4OmcDA+44cB0kC0OaCniw8O1tT/oA+PoYhQ==",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[cfg(feature = "atom")]
    #[strum(props(
        svg = "eJxNj0sOwjAMRK9iZW/TuGlDpbQ34ALsUEACiQWqWMDtybhK6GbysV7mJeXHmp83yp/ZeXWUv9u6lsUt6bCNl/S6vO90nd1JO1FCZJUucIleOuUo/chBBvZeJmwCQnmSY+QRxzoAEggcF5AAUrkmzLEJCCWABLANzvCBx87GDxIJka3NcGszvPlYm/maJ4Trq6ZpvaZpvdx8AJIJA4Rv/cbf5gfnI0Wl",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[cfg(feature = "award")]
    #[strum(props(
        svg = "eJwly0EKgCAQQNGrDLPXmLFJBccbdIiwoMBFSIu6fUmrv/i8VI5W6gZNcUIotyLx10cxYE7Df3M6l2uHVXEmsaP3QGxDBPLAXI0YZwQckBV2JlqiTjvJL8RUGAo=",
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
        svg = "eJxtzksOAiEMBuCrNN1TKTLAAuYGXsDdJJpoYowLF+PtbUNFJzEkPH6+FupjeV7g1PCwpzJBoRSWCBG8DIZIuTiZMnyzRFm3W2TTyHxnI7AmG+SETL+d3L/nvLJ0xLnu9KtzvV3vZ1hDQw4IL25YEFa2Y4/FqjKrhpNdytpryfOo+/g3Hww4gw==",
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
        svg = "eJxtjksKAjEMhq8Sum9s0+kLOr2BF3BXVFAQceFivL0JUzoOSMjr58ujvNr7BpdZHR0mDwkDtQkmMGwWJoxJc4iwaQGjlHuoh6GZFRtCX7KDNCP+d5P+d84IFk6qloO8Wst4OKPJkJsD11GPyYE9GyDNovg29bg/r7DQrCyhsQo+lsvImda8SE/CC1m/D8Y53w==",
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
        svg = "eJxtzsEKAiEQBuBX+Zm7k2OuenD3DXqBbgsFBREdOmxvn4O2rbAIo/P7MZpf8/uGy0inI6cBiYObPTxsWQLPMZlSIv5Z4KjHHrWyZrayNWhDOmQKGbaTzN5zVlk405QP+tUpP+7PKxYZSRzh49qufSAstS9YWcOK0u+OrVSuiXT6C+QzOFQ=",
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
        svg = "eJxtjksKwzAMRK8yaG/Xdh3bC9s36AW6C7TQQilZZJHcPlIS8oEg0EjDY6Tctf0Hr0KPu04Nkg6u9fAwXBZex6S4Rexe0FHGM7S2zTMLthlryAlSjDTHJHV1zggWnlTzTV6t+ff9vzG4QjYQBlsoEUZZHaudlVmh6gSXgCzF",
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
        svg = "eJxtjk0KwyAQha/ymL1WrVEXmhv0At0FWmihlC66qLfvTBLzA0GYJ2++eTP5M3wfuBW6nHXqkHRwg4eH4WfhdUyKS8TqBR3lu4fmsnhmwhZjDtlBipFum6SO1hnBwpX6fJJT+/x6vu+otlAiVFfIBsJP1LHaUZkVasNKszaoDY0Zjf0DQ8Q3yQ==",
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
        svg = "eJxtzksKwyAQBuCr/Mxeq6mvgskNeoHuAi20UEoXWZjbZ4zmBUGYccbP0fjvhzeeLd2vMlgE6ZrewEDx0jDSB8HBY+s56fP2iGpYe6qwtVGHHJBgYveTxNlzKjP3oC5e8le7+P38XkhNSzfCqEviSltC0nNmmlGl+0O2qZbl6kIn3O03ew==",
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
        svg = "eJxtjE0LwjAMhv9KyL3YxLFNaHv24nX30RVb8CCl+PHvbaZUhZFDPt7nibnOJcJi8cQMNB77mYFB1yLFiqeh7Vp2dGYnhjPNowGom7p/Lir6PQDfSH/dHHyBe1pKtEh7hBjSORaLI8LTYo/wWOdcG4klvDM+ZX8J4OUqqUVG8FXg9fU7btgnrvBhg3oBpIJALg==",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[cfg(feature = "ban")]
    #[strum(props(
        svg = "eJwlikEKwCAMBL8S8gBbxUsh5i8lLbSgIOJBf6/Rw+4cZkj+IvEFaQGtQ5C+WSZOZDq2Z8p3/eAJmLy5QGe9ceu0UssDKWUUSg==",
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
        svg = "eJwti8sNgCAMhldpOoAKB+IB2IAhTCGWxIMhJML2UvH0Pz9bElV4cqzsUG8I3aFBaCMglCmc8snVodLo7SqAt5QLXQmofTVQn1oEGKc5e3sflSE6DAaU5mVTQe2/k5vM/gWCASQy",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[cfg(feature = "bar_chart_2")]
    #[strum(props(
        svg = "eJxdjDEOACEIBL9i+MCdG2Ou4PyNhYmx1t+LEC2sJsAMXEvLrvuf/EduCPEKIbOwQ/eJn6UlVnlLegRZDIvC5S4nXn+DJfGoE5TZIQI=",
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
        svg = "eJxVjF0KgCAQhK+y7AVqs8gH9QYdIlJa30KWfm6fSiA9DTN835hjFQZvcVGgTtJMGp3pyupMCpvAFb2wxRGBQ9xZLM4Ij0XqEe5aUg4qVuE/KwNTM2ioLOWp3f21F5pfJs0=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartBig,
    #[cfg(feature = "bar_chart_horizontal_big")]
    #[strum(props(
        svg = "eJxVzEsKgDAMRdGthLcBiVV00LgDFyG2mM6kFD+7twULOk3ueXZfkpITzIbMwaPyiMk25TrZ6NdEl2AAncElFXALugU9SH3YNAk6UMwJF1X6V+WITX39N4YvruoBWE4moQ==",
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
        svg = "eJxdjEEKACEMA79S+oFdy1I8dP2NB0E86+/VKqKeQpJJJIbkoZgf6UXITQ0hFGraPal38nTMicIa2gF9eG7txe4lz18ev7zQCpMbIQQ=",
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
        svg = "eJxtjs0KwjAMgF8l5N66pD/boN3Fs1fvBYUJIh5E3Nub2AoVpJQvJF9+0r08VjhlPMwQIdoAzoZCQv2DPkPGhr23cXLgwIugkQdvw5HGwsDVE/JK3CcMP03AJe10y5Kul9sZNs44IrwEk4Ay0oCwCT+mOp1JXFXm6nJVJf3raq1NFWgnU1PnP2O7Yj2h9Qq/8htc5EKE",
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
        svg = "eJxtjtEKgCAMRX9l7AdqFvVi/UxJCtGDCOXft7kihJ4unnvmZqNbEsQ8oUE4w5r8hDQgMBgR4lW4d2HziYsWoZDZNjI32z0cDrLhquOK07B+kWbmJBJZtFdWWORBXVmnX9SqdGWl0XxH/2WBPX7H0POuj7gBswQ9jg==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[cfg(feature = "battery_low")]
    #[strum(props(
        svg = "eJxdy1EKgCAMxvGrjF2gZmAv6mVKUogeZJDevg2Dwsf99/tciRtDivlI7JFmhFI9GoQ775ykWIQemsdVvk2P4CbdBXfmK0I10lSQeBJP7y2dFsXKfth2Y7/JQB/3Syc4",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[cfg(feature = "battery_medium")]
    #[strum(props(
        svg = "eJx1zkEKgCAQBdCryFygNLCNepmSFKKFCOnt+6MGbVoNM77vjEl+y+KOew6WpCaRiiWFUlvpTfDxCBnvMwnMV3Jm4pwzZ7y8KBIKrCqQBSE1esylZMxs4C/COs7qP9qHjfJqtu2E/sWLH9gFMl8=",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[cfg(feature = "battery_warning")]
    #[strum(props(
        svg = "eJxljEEKgzAQRa8yzD5tZipmE11300NIW4gg4kJEb++PIUGRLN4keW/81M2Bfg1/pCIXtFNSsjgC6lJ/MRnMBrdglFv/jEXrS1eTe1cls6ZkaQWyUzX04582bVheTCuoCkriBopEOWpZxqPjSyM2NeDVzZ/Hohy5h5WbvwPorDwQ",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[cfg(feature = "battery")]
    #[strum(props(
        svg = "eJwli0EKgCAQRa8yzAVqDGqjXqYkhWghQs7t+6Ob/+Dxvq/pbNQDO6avXC0Hlp2pTpNTuXODWqF0KOzB0S/2i/4pbyIVFMLUQYeku0kFZbPYsvgDNd8cZw==",
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
        svg = "eJxtjt0KwjAMhV/lsPvFJk2zFraBD7CHGFNQEPHCi/n2Zj/MG2lJSHv4vrSv8X3DpauGgjLVZOq3ZkrCVJLWQhoETGzpbDCE9WSIjBypGLa6PRcqtRJXfXtasH17wDlQk5AolHhg2CHIU4A7mF3NcHl2ueUII7M/oERRGY4zGRW6i400N76lhDysf1hzo1LghK3uSgqxLFV/8Mf9ecUsXSVS4cPeve3jvI4eXUL9F7FZQmE=",
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
        svg = "eJxVUMtqAzEM/JXBd6sr+REt7AZCr+0P9La4hRYaCCGH5O8j2YQQjIfxeOQZvLS/c/v/wXkNQiWg3dagnVzXwC7tl7fh2S+n7fKL7zV8+gVkq4a+J1+xkggy1RaZGIk4RdqpkTnaQc056SEhDTsKWFs2opRtQsFsJFM6vDyLkfXlPTz/2eLI6kYI8Wy5xepkTzHoc6DEEBfmAV2NQhWF5J0LZchEYsGw5juD7VGv29Rq1foh5uM6fqI3uAN7eEHY",
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
        svg = "eJxFjcEKAjEMRH9l6L0xaZbWQtuLFy9+xFIFBQ+yetC/N9s9LCETwkxeymv+3HGt7iKZJshE+RSYAiTaGgSSNj3rW32A+tzZKykCJR8RrSmBIaRkOcqkrpXDim1lh/N6IbP5E4bwKLUnvOf7Y+nPG/q3Ojk69F91NpbqBnNz2x/83yn0",
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
        svg = "eJyNzD0KgDAMBeCrhOwq/UOHtjdwdZcoVOggxUFvb0IddHMIeTw+nqetUF6BzoBqaB1CCWjk08VNzyn6rqLoX9j9tEwU1vkHftQ+HwmWgKPSIAuTsrkxjQHLp8EkLVpUvAG/4S4j",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[cfg(feature = "binary")]
    #[strum(props(
        svg = "eJxtjE0KgCAQRq8yzAVKEVfaDTpEpDTuQqT09vkDZeBueO97o7zdAySNTCDczgTSmK/YAFl3UNAoEXxGHBc1lWBRNYvVdKNU4zb9vr3RuQUCo3GVwGeqoqBOMAFsbCQwQfyS4+ivHkg5PK8=",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[cfg(feature = "biohazard")]
    #[strum(props(
        svg = "eJx1kM1qwzAQhF9l0V1Tr34tsHPp2Q9R1EILLZRQSvv2mXUCCUQ5WF7tz3w7WvrHsX++Sf9fnSqak+PqgpP+x3twh+Xp3HBYvl9+3uV1dVtBlYjUPZoEZJkkIzAKwspzYaaxXqWhSBCKFpOx8avI18xhnaCiSJjvGzatiDvmluIvFGaSV8aWI6aYGvKAo9kYBPmHoF1vwtypXjyFE38WJlSfEf3uzlIqlLC0cUdStBvRfoeWN5YyB/lw6s2KHVGmoXfhVrRePVn0yeWLGeQOA5NbprCBO3hgZqd9WW67TwrCdIo=",
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
        svg = "eJx1y0sKgCAURuGtXO4G6lcjBXXcpEWEBQYNQiJq972gJjY9H8fO3RKpd9xKkhGavS2u5O0LoqS6+QEgKzjFRJQZ0oQqQmREkVyhPghjCtNAYXOsmMLuGIYpOb7nR/0Bb582UQ==",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[cfg(feature = "blocks")]
    #[strum(props(
        svg = "eJxdTTkOg0AQ+4o1/SoYiJJid+s0aekRIIYOoRXH72EqDrnwIcv2U9ckrEFYCrYghWAyJ1iGNmmQj0C7oddkMvqX9aMf66Rog/yZIWf1rQkiMzg6/sqrB2fmZ3Aw9R44zu79mCjszG7iDi4YJps=",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[cfg(feature = "bluetooth_connected")]
    #[strum(props(
        svg = "eJw9izEKgDAMRa8SsosmUuvQ9gTO7oKCQhUHh3p7E7WFkAf/v+/O6Vph9rhbsECNXGXAjBzlD5JYDK5WKbi4HQsk8kg9wq1kIX9MQiaVVfvlLGnZ4bttyySrD3VNIL8=",
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
        svg = "eJxNjUsKgDAMRK8yZG819YNC612kCgoKIi709nYUioQXCI/MuLAcYZ0QLi+qgsNLJwh3vErpXf7p3u3DOWP0smlnahA1bRYZrKlAim+y0lRrFA2hVLwrSRTULf9ZwOBfvLWwUTO+TvoBN1wnrw==",
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
        svg = "eJxljcEKgzAQRH9l2HtsdpsUhUToLZd+hKSFFioUKaJ/7wZBEVnmsMzMm/Dr/m88Iz0cuKn8aNjfpfIosnqMW/kSO2rDpaTbsHXEoh7ZJY10x5I1PondK/kz5O8LeY5UE4ZIQshTpBW7uju4V7AYpzTVebfnBlcd4c1aADruMrk=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[cfg(feature = "book_lock")]
    #[strum(props(
        svg = "eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCDkk+b4//pVKhingzQJdeqkdyZV7AW3TisDpFMng6E9Kj35zuN2lnmND0lEynUTeKev8KPAN6BDyvDxzCSgIn4DECO9lKjnggLDqRi3ld49oAHe3iYH/6RZM5S3+BwMjMf0=",
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
        svg = "eJxNzMEKhDAMBNBfGXIv28zqFqH6GXsXFBRUPIioX29LVSSHIeFl/FwvHZpSRi1ANc5kxiH75zVB2DAKGnZq3wdw1d8hlf/E98oP/dRiZyn6FWwakiGZcg+7izaq20ZjL5snW0iqsI89AYnEJgY=",
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
        svg = "eJx1y8EKwjAQBNBfGfYudpe2pJDkD7x6L2kwAQ8SQtW/N7GlVLDsZZl5ox9jDpgMXVoM124UCJpyDDlJYNkHkLklq891YvU2VFAz/8lZjor+t0jeZbwMCeEZpxzK1xDehgZCWvLg4y1kQyx1VgdWu5jc3cMVyF2RX+iKV9Us7abWdrVVcb9jHzYSSDk=",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = ""
    ))]
    BoomBox,
    #[cfg(feature = "bot")]
    #[strum(props(
        svg = "eJx1zDsKgDAQBNCrLHsB3RglQpLaxtZeVNx0EoKf25s0BkG7YR4zehsDw2ywJwFqkJ1Cq4tUWu2XKcBlUCH406BA4MWtHAxSzIebA8fYIESUaZYGVj+PAkiyyH8Zyj+hGqjav6R9ww3K2DQa",
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
        svg = "eJx1kFFqAzEMRK8i9l9T25K3a9gN9AA9RNgWWmghhHwkt4/kfARn1xgjoxlGz5rX3/P6903nZZCB1tsyxGT1WuthfnvIh/l0vPzQ1zJ8xkSKfEzI5Df4YUUZGTpS244oE8mmKR6R1BR59AiilJGn1mqvMpIgTB9t3xhieYUgh4DqTgZ7RjuOt+MqGVey7TdYtgy2B1+Rr+a5oP+Y8U4xQBkFumMoMHxB6ummjeQpXDP2HAETFU/oGSqDIJvMfYjgasdgYq4U3RmC6BCtfgd+lIV2",
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
        svg = "eJxNizsKgDAQRK8ybB80i58myQ1s7cUEN52E4Of2JtrIwDx48EwKa8YZfRZL3BLSVUiQEDfJlnRH+Ey6X5QfyZmmds7sSxZ4S5MewHruFwajrVOsWFT3F+BDD7WtlXsA7okfPA==",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[cfg(feature = "bring_to_front")]
    #[strum(props(
        svg = "eJxVjb0KgDAMhF/lyF5sQweH1jdwdRcV4yZS/Hl7jYOtBMJduPsStmlI2M5ITLgi1YTz3TIts6RXHsuYRFUTKo03Ye2TYIzUejjbMxj2GWfYcOezx+Pl58EK0XoBcR6cKVZTRcsqZTfFQd98mBt2iC4l",
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
        svg = "eJx9zcsJgDAMgOFVQu5qW0qo0LqBQ4iK8SZSfGyvxYuI6SkkHz/x69hHOAMahPV4xrPt8xA5oCaE+24ReJwnjrcpbHyVusYvXWQYArY1GLMVlmmzSdP9pQ6IS6V/RJNMJkdaZczK3+SO5M7JmftWF3YcXfM=",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[cfg(feature = "bus_front")]
    #[strum(props(
        svg = "eJx1jkEKg0AMRa/yyX5ak06nFma8QQ9RqjQuCkUGW2/vqKCCugt57yfff59RUQZ6WDgIblT487Ar/Ew4g1O7BR9JvhHDC2qqV0TzDySEX11GDcSOoFX91jjNXaALISnjxSGwemXBrMnalsjBVz1lvNfPHbOE7q3shXIIt2ZBPbnsSl0=",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[cfg(feature = "bus")]
    #[strum(props(
        svg = "eJx1jsEKwjAMhl8l9J64ZGuXQbuLFy8+xKhChR1kytC3t91Ad3AEQvIn35/4+/BMcAnmrOBmZ3p/KErvvzrbnYEAS+KO/kIKrKl+kEWmlhSFNBIjNSRImhOTQJV7LKriqow5NWiPUhGDo2zSlSJ7gTs1g0BmSmCuZq5S/Tsdb1McrxDfwbAaiK9gWgNTMFJ21unmwa78Z3fxhVtM2G34D8jiTc4=",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[cfg(feature = "cable_car")]
    #[strum(props(
        svg = "eJx1jbsOgzAMRX/lyntpbAIVUsIfdO1eFVQzVKpQ1Mff4wwIhrDYlo/OveF9T4oh0pUdaq0cUx/O+dmHDXlIGb0EHcSdmpIlYLm11Y7N4yPhOw1JI3FL0HF6arLbEf62hDD/ItUEmz572dhldhb5KZY1R8SDL2plK1kA9xVDlw==",
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
        svg = "eJx9jEsKwzAMRK8idIDWMiZ4YeUGPURpTGUopQRDndtXSv8LZzVo5j2lOZ8qNMaAILmcpTJ6h7BoIMxtjXuZqjDSgGPamzCmS7lmaH4tYSFGjaYR9fJ2KWnMm6QvSeHJkLKvDx/4dqwCE+OBBiAnO0e2Wfu7+f4Wt7XQ18KGFvva//QAGSNaoA==",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[cfg(feature = "calendar_check_2")]
    #[strum(props(
        svg = "eJxtjz0LwkAMhv/KS/bD5vU8brh2dnF1LyhUUHEQqf++l16/hpLhIeFJeJM+7bfDrZYLFeqvoSWIysrR8Xza9uBP/TrIZBelSQc70qTn431Hz1o0CP6ZGb1ObSZNNWlSy2xrRhn3446oVTGNph6LSl3d+ZWXBnBMB+/8km8AxI45Qg==",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[cfg(feature = "calendar_check")]
    #[strum(props(
        svg = "eJxVjlsKwyAQRbcyzH9pxwRJQd1LaaQKbSlBaNx95kEIQfCo91yZsORngzXigFByfZUWkSaEf51bsePCqWN0Be8jpnCVXgrv+s3QSZPVse+ZZOx896KKtKv6pupkJsP6Z1EMR5bRzT4TSmU43N+jFZgjfu5AHhyv8aLjSZA2gPk5jQ==",
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
        svg = "eJx9jWEKwjAMha8ScgBdulH6o90NPIS4YgoiMgputzdpBd1g/ZOQ99734ud4y8Ax3TkHJIcwLwENgsxejrUcMgeEd5oyl9Doz8qN/pGeERYjohWE6l6pQiJbjWroG91YBZCPyrtdUDVDtYu6SuhWpP9lX9fMMAW8OKCBTx2ppeKfRabh2WNPKl2j0jUqt94HEt9fVw==",
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
        svg = "eJxtzbEKg0AMBuBXCdml5rc9Tjidu3TtLrRgoYiDiL69iXeoiNzwk+S7JPTN0NKn4heEpHi7BgTK7WXI8Hwca8Io972hidZzHW62pA7/X/elCRWLY5ok5qwJDW07o4YSTSOTnteP/hqaKOJI8ighcbXWp6WGyv3+ipFwueEFvTY8Tg==",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarMinus,
    #[cfg(feature = "calendar_off")]
    #[strum(props(
        svg = "eJx1jMEKwjAQRH9lyD0xO01ihTTgzYtX7wWFCCIepNi/N7FgFVoWdnZnZ1989M+Mc6eOzkiL2vYEYT/VIAzi+tkoU/4zxLTUUr5UipvKSvFLpEC88acw5zU1DzvjF9ISwMEtHBqIzdsVvs3a/+Ju1/sFo3SKCq9JRhap66QlWkPpDQzxPdU=",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[cfg(feature = "calendar_plus")]
    #[strum(props(
        svg = "eJx1jsEKgzAQRH9lyV3qjm2IED330mvvQgsWSumhlPj37roBFZUchsy+md347X49PRp3AxNXd9+BQKW+AgWul+Wf8OfzbIiiD66NJy1p4/v1eVJC49g7Smw6iEJEbK+oQhnNI00EC4QDUDtLQyoLgK1a7G3ptFch5DtqC4nuwPV8bMqhYQ2PXANHjA==",
        categories = "time",
        tags = "date,time,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[cfg(feature = "calendar_range")]
    #[strum(props(
        svg = "eJxtjUEOwiAQRa8ymb3agcaygK7deAhjiZAYYxoS4fbOgEabdvUD7z2ws78meMUpBYdkELJDjRB8vIXUborDHmFmoHiKzGgP0o32Hh8einJ4ZI+qkXmIj1nVZVWkjypQnmyFKOYbrkXdEHUtkJVC0c99XlKAyeGZBqA+7Op/cvmPNJA5DRukNvuOtiJmZsHeTt5PgA==",
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
        svg = "eJxtjr0KwkAQhF9l2T6YnWi84i61ja19QCGCiIVI8vbuZi/kh7DFMMM3w8ZP++3onvgKIaludQsClXYFClxOS0/4yXEOVNEFbuLBRpr4er4f1EtiqZkGVaggsboeY6qoQRnNiDXCigwb0DKIk5VPS+kN1c2osXlVzr5nfnC/M5y/NHj8aC5P8B9Z00d+",
        categories = "time",
        tags = "date,time,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[cfg(feature = "calendar_x")]
    #[strum(props(
        svg = "eJxtzlEKhSAQBdCtyGzgvTEJP7TNlKQQfYRQ7r4ZxyKir6vjuYNuC2NWe5py9IAWVAxpjlnOxYMBtR0eNEWpQZcOBvfj3uCWtAZ1aOI9PaFkwSoLjXumjBqVWW3YG3LPviALjULwL8s42XYfFk2zpv3j6nwtvhY9Sy98AnFxR+s=",
        categories = "time",
        tags = "date,time,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[cfg(feature = "calendar")]
    #[strum(props(
        svg = "eJxVzVEKgCAMBuCryC5Qswgf1MuUpBA9iJDevrlF1NPY/u9nNoe1qCttJTpAAypXB5pG4xFD2mORhC4zKIon8HboPW+PdAZVkcBCALnTtAPaquYr0Y4+1EhmXim9P+wRjiI1iuEdn/9sbwPgMYU=",
        categories = "time",
        tags = "date,birthdate,birthday,time,event",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[cfg(feature = "camera_off")]
    #[strum(props(
        svg = "eJxtjEEKgzAQRa/yyb6pM0kaA1Horov2EEILKZTSRRG9vTMK6kIG5s2H/yZ/3t8XRmoMG4wsEA5LHJbY5rOW2vzr/gXPxjwi4s13DEalc5KrT1sWcqGLiqrsxGQDfAl3kg/FrQap0UcbDhTylphAQXF1cHOfkGxdgyrZqzQB2u8wuw==",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[cfg(feature = "camera")]
    #[strum(props(
        svg = "eJxNTTsKgDAMvUrIHrVpqwhtZwdd3aUKFRxERNTT2+KgBJK8H8+swx5gtNgJlWlQgXRbQdWogYGhSEPxO+oPx8tBlH+CuK//AeJAciHONMkbnclTizN+3vwygb8sComwWYzbnxFxMr2yewCjdyMO",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[cfg(feature = "candlestick_chart")]
    #[strum(props(
        svg = "eJxtzcENgzAMBdBVrL9A5aYpQYqzQYeoCqq5IRQF2J6EAwSJiw/+9n9+/EalTvBpyaYXgn+UTfBT/4uk/fDXKHiDVkELmhYBg/JsQPPQRRXsT+U8+KqMbXqebUfADZk6uDCuBthWQuHtjZPr2CRzA5nssFN2R7YBOlBDTA==",
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
        svg = "eJxtjs0KwjAQhF9l2HtjNj9NCknBmwd9iGKL6UGQEvx5exNBUSx72Z1vZ3bDZcgJY6SzYvhGQTUsbKOF25Ye8lVsRWta2J0XZvjIZbGTGiyUdXsLltDw1IdNjezDO/jgwCYJySuI/9kyHTNu85hTJPaEeyRNeJRBEtI0n1KOVPSlAFVt1fAVWR7xV7V2q/slT4kNQDw=",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[cfg(feature = "car_taxi_front")]
    #[strum(props(
        svg = "eJxtjs0KwjAQhF9l2HtjNj9tCk3Bmwf7EMUW04MgJfjz9m4FRWnZyzLfzOw21z4nDJE61jDJUdvsFqltPuBiGKEwMAUrX1hV7WWHfg97VboS/hCU67+yGGttwcr46ughxRZhXdxVYJeU5g3EazaPp4z5EckQnpFYE9I4nVOOFAj3achJVFnFYpfYEviplEfCzWzdqv/JC9b+R1o=",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[cfg(feature = "car")]
    #[strum(props(
        svg = "eJxlT80KwjAMfpXQe2PT1nWFdiBevPgQEoUJO8iUoW9v0sE8SPrlo8mXv/K4vEa4VnOmDJRGz9iBA7IYxdFiAzuL2WKy1LAX5CP1mICcaEmeW/1TUkEQrUcvCGxFLl9CWhv0Qqe9hGWGRlsuYp6UwGM+BOmrcM08kF8iO8BO91EbvRnKTrceCt9nnm7A72qSAf5UQ8JzNU2zZoeyXdgO7P7Km36r1mbCv/ovTQJEkQ==",
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
        svg = "eJx1zMEJgDAQRNFWlmlAo2IUdtOBRcgqKHiQIEG7N0HiLdf5vGHdvR4r6SMwDUhvgQV5QQvH1Vcdn/O10SKYTE1j6FNKk+PMIzPR5Zuy78iG4fcvHsQkYA==",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[cfg(feature = "case_sensitive")]
    #[strum(props(
        svg = "eJxNy0EKgCAQheGrPGYfMWphoN6gQ8QUGBSERNTtSyJs9Rbf+9027BGjp1WDG5jKwsBScHWG4D7uDVjHtoDMSZYJcnpiS5DrWUVInnQ+vfzrFaM7Sn8Dyl0hfA==",
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
        svg = "eJxtjWEKwjAMha/yyH9rU8dWoS14AA8xumILE6QU1NvbdjIQJASSvO/lmRx8QQzpFoslHgn5ZUkR3pYGwjY/01JinSQ5c2wGZ3zKfg3wFdAEX2mW1dvwymyqM4+5RCyWrhqsom5SO+32zvcnLfn75Y//PkJJiOmgxPnCYkBrWYuhBYPHOAk9/wp9W8WE0x77AYCCPfw=",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = ""
    ))]
    CassetteTape,
    #[cfg(feature = "cast")]
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l2Hs0O7QhhTR/4NV7QCGCiAcR+/duSqEtlD3sY/btpHf5VNxGuRDxGgpBeBsFHavuAvCrXANnVF2QnM6tJKdNlXkDhuUxIh5LofToF6lDt0rPx+uOiaPQC35tn7waqZFg0vlgdvPyH3GoLls=",
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
        svg = "eJxljrEKAjEQRH9lSO+aObObCLnAdTa2FnYHCgoiFhbn35sYxEIG9sHy2J38mJ8XnEa3N3AjKU4BAb6GiBIIm1VIfEZfU7yuKBqgNV/VJ/jpTzXRLQy/m0z9zWHgzo6u5HUrUPLtej9jGUbH5PBqjJXsXCqtuc0qb8P7JgU=",
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
        svg = "eJwlirEKwCAMBX/lkV1qinSK+ZeSFlpoQcRB/15FbrjhTuzN9t2wFol3gtXlPORJZVtdJZ3lwRXp5wPsXUBwg3nMoh3c+xOJ",
        categories = "arrows,navigation,shapes",
        tags = "back,menu",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownCircle,
    #[cfg(feature = "chevron_down_square")]
    #[strum(props(
        svg = "eJwly8EKgDAMA9BfCb0PnQ7xsPVfxA27gyCjoP69K5JL4CWxlV3xJJoJd80qifxKkFIP0b+3rhPhtQ3HwQ4cr00FOdHpF/jRBQTXY27CHzopF9E=",
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
        svg = "eJwliksKgDAMRK8yZF80ElyluYtEQUFBigt7+/4Ww/B4T/1Kfh/wPxIvhFRvJnjuaDoNb/pu34k90sMCXoMEQV0rmrEC288TjQ==",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[cfg(feature = "chevron_left_square")]
    #[strum(props(
        svg = "eJwly0EKgCAQRuGrDP9ewpJo4XiXSGlcBCED2e1rcPFWHy+2cig9Nasw/AZ6GQuodcYMklJP0QHdIMXJhhTvXYUy4/KB/OqCC/RnbpI+O50X1Q==",
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
        svg = "eJwdy7EOgCAMBNBfuXQ3ijIwQP/FCLEMJoY0Uf9eynB3w8vFVg7FU7NKIhcIX6KN8I5ufVaClHqKDuY424HjvasgJ7rcggAPP/UYG/APHCsXiA==",
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
        svg = "eJwlijsKgDAQRK8ybC8a2cJis3eRVVBQCCFFcvv8imF4vCf2RvtuWPbkdkJstxGsDFRZp1cJZ3pwefoPOAYvbeAedKEVx+kTRA==",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[cfg(feature = "chevron_up_square")]
    #[strum(props(
        svg = "eJwdizEKgDAQBL+ybC+ipkiRy1/EBC+FIOHA+HtNimlmmFDzYXiFG1GbcCWekkyFiyfa8JrLqTZMDHMfYrh3UyTh5bE4uOkHruce4gcWOReM",
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
        svg = "eJxtjUsOgCAMRK/S9ACVIiAm6GWICxPjwpXc3mr9sHA1bea9NuV5y8sEeR+QLcImYRByudYxNdqP6eG0qHlXY8u8TlB4wIiwS1gm7mS0ihd7NsKfXEUHMkGFlnqvfCTv1GD3o8jl/usvlw3FeD/z5MJrHa8MOx4=",
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
        svg = "eJx9zbEKgCAQgOFXEXetO5Qc1Lmlh4gKDCIaQvLt88ihwVruH747zm7rvrAEjiNnVwnm5Kan3ja05O0xnoHNjg+ADLDHqAIoYoIXI3GsCZgsQUhdse7nyEytQKkFlqnrX7/XbgOkPUE=",
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
        svg = "eJxNizELgCAUhP/K4R75HiIO5tzS2i6vwKAhpKH+fYogDcdx9/F5ObKcO+SZFLGCvK1zKa2CHxsP/op3wjaphSxcGmxkMDSoRMMk03d9zOyqW52/ySC32k4+7Rof7A==",
        categories = "shapes,money,currency",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[cfg(feature = "circle_dot_dashed")]
    #[strum(props(
        svg = "eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgI3D8nl5GZ8LM8rzpM5sSOGJx6WRKlDESfF6GiAM/N40N15/DoiBRlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrXdSQ3KE1dPYpU116gQD0KrMEJye4EkpdBYds8tp1HXwZhNd4W7W+afFvz/YL8mgx7g/yufZWmS3U8fwCjBmtN",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDotDashed,
    #[cfg(feature = "circle_dot")]
    #[strum(props(
        svg = "eJxtyrkNACAIQNFVCAt41MgyxMLEikq3F6+O6hf/kTSVXkELpoggw5qt85Qp3M/03AfbO2wBfJwWRQ==",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[cfg(feature = "circle_ellipsis")]
    #[strum(props(
        svg = "eJx1yz0KgDAMhuGrhBxAmywuaW/gISQKFRykONjb928plE4vfA+f6B30uSBYJIOgMZdz/1ona3Mn7/F5OC3utAGxXwwVLWtvPLfhlgBL+iIY",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[cfg(feature = "circle_equal")]
    #[strum(props(
        svg = "eJxVyksKwCAMRdGthGygphQ6idlBF1HSgoUORBzo7v1NdHThncf+jg4ei9cJZBwZFN7aJjzLsYh+Qf8XNFmkHUHzaKjpr+FSALkbGd8=",
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
        svg = "eJwlycEJACEMRNFWQhrY3ZxjOtgiJAoKHkQ8aPdqZA4P5rPmpiVCc/i9CDq3tB2m8HO7cPU9QXD4E4HN6nllAY2uEpY=",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[cfg(feature = "circle_slash")]
    #[strum(props(
        svg = "eJwlijkKwDAMBL8i9gOJBCkCsj4jXBhMClfS7+OjGmZ3tLevUnDBCwop4AeUsjV5q+m1IlNvw3sljzkLyPNwTNyrOr/9jncWGA==",
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
        svg = "eJxlzU0KwjAQBeCrPGZfdMaKFpLcwK37kgYn4EJC0Hp78yNUkCxCXt58Y1LwGWm1JIRXXLJa4jOhBAeChnjT3JN3TZzZ1QFnHnNWLJYuzJh0nAWCfTsyyLUVa8UZH5O/B/gyPhFS2+PX+iiV/vmjnSD8HDaOK6fjH/d1CsrH7pV7Az9+oDoD",
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
        svg = "eJxNjLEKwzAMRH/l0G5aCWFasDN36do9NKHKUCjBNMnfR86QBA13uuNeGvt3wZJJCHOmG2EaumKbGz1gl2UT64ePlUxKTbrUVZN+bTF0mZ4coSatQHD1Y1f5sx5BcPeI5z/IK54HQUwquTIP8vcO1kqDBt3rFWrVLKI=",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[cfg(feature = "clipboard_copy")]
    #[strum(props(
        svg = "eJxtTMsKg0AM/JUh96UmpOLB9eylHyFVGg+FIsu2/n03HqpQGUiGebXLdE+waX5YiqSE5ROJCeU2hPc8JtvYGkmKubrZtRdvde1rSIYx0q2B9vUgEFSOUFhm3YXyxViOQpAcxKd85DDFNdT2JHs160lQGKw987/15Cu4CooNP/8Lz3s7kg==",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[cfg(feature = "clipboard_edit")]
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/llD0Bu05LpKZfACsDW0UrUokBVRHQvyfuAB0Y7Dudfa+dx2vGEg0bvKM5GLymIafVpXG6pRyNGMzlRkUWla7daatrH31OGKI50d4Jg9jV1LMj6OxBUBeadR0bFzyYIGC+uxBspYF3Utmy5KJYBW6xNSRxz+BCU15xT5JfYItLpe7/lAVUOX+ut3XLib+vH6daPLA=",
        categories = "text",
        tags = "edit,paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardEdit,
    #[cfg(feature = "clipboard_list")]
    #[strum(props(
        svg = "eJx1jL0KwzAMhF9FaHcbCSMy2Jm7dO0emlBlK8bk5+1je4kDCQKdpE93LozfCGH1SAhhK6Lj9NPo0SIs0xDVY4uwlp4eGDv3zK7O/fuoMHh8k4BV7hkYmlSUlGeyx8Gk6SX1bvgjtcGwluScWSczEKm9IXJJ2mx5NHSN5IR272FCRg==",
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
        svg = "eJxtjcEOgjAMhl+l6X3Vljq2hHH2oC/gjQhxJB4MWRDf3o0LHMiyrvn292szDc8Ev4CCsAR0CHEYXzEFVIQpc0b4jn2K69+0FNA2pzLVNp8uRegD3h3o1XYCAudyTO5m1g3kVyLLHhiZDV2Kq1h2LragcYsyMNUV8FHSkQp4stwJMZS75nPn67Xc2JK/ANfAWeHe5L2pClHSyuSijwOvy9G4LfwDH91GmQ==",
        categories = "text,account",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardSignature,
    #[cfg(feature = "clipboard_type")]
    #[strum(props(
        svg = "eJxtjLEOwjAQQ3/Fuj0Cn6oAUtKZhZW9ohXXDVVRoH9PwkAzVDfYZ8svLNMjYY2iApvmp6UoneAT5Sx4z2Oyn1tKwCJrlT4c6qoPryEZxig3enSmg0JxLMeimtltgSvu6tvf6d23A6emlVyZDfkCanY0n7nTkuBpd0cFmf2/+QINtTvz",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[cfg(feature = "clipboard_x")]
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/l5D0CW8ECKenMwspe0Qp3QEJRBOXvcViaAXk4353upTLfKsonExPK+pP3MlXLdCR4LASbl7vVTJGwtnhIu7Ya0nOshinThRXRZBQI9n7sKi+OWxD8O2vvg1y1HwQxaeTG3MgPPoA5KPRPd/IKffUFCBwzgg==",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[cfg(feature = "clipboard")]
    #[strum(props(
        svg = "eJxNjLEOgCAQQ3/lcjtRLoQ4ALOLq7tR47EZQhT/3sNF0qHtS1qX9jUD7/Hg7NEgpMejRrjjltnjgCCdBJcPl4qC6+oquHPJDJvHSVswTAsBQS/S4nRp8wMlabRtVzTbdqCIqT7Xz/ACllUknA==",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[cfg(feature = "clock_1")]
    #[strum(props(
        svg = "eJw1iksKgDAMBa8SegB/qLiIuUzoIhDaUl3Y25sohcebxQyyVNYI9QzzFIAf42JsHwnH3xOWrE0lRShZ0n25hh3sfOuwweF1r+gFbiAZcw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[cfg(feature = "clock_10")]
    #[strum(props(
        svg = "eJw1ijkKwDAMBL+y+AM5ipBC0WeEC4GwjZMi/n0UhGHZKWZItItlyLjStifIG+yONTEt4ZlatWFaMlrV8tx/hQN+vhPRzoY/QgMZDA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[cfg(feature = "clock_11")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+QFGouUzoIhDaUl3Y25sqhWGG4T3PUlgD+DndvDgUm8mB63fJjz8nn5NWlRiQk8T7ahgbrCzHsGJvcpfoBVhyGUc=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[cfg(feature = "clock_12")]
    #[strum(props(
        svg = "eJw1yrsNwCAMhOFVTiyQR5HK8TIWhSULEEkRto8B0dxffEeiVSyi3uHYA+Tznt42yrRNZyrZmmmKKFnT+3TGBZ95W8w/zsYYMw==",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[cfg(feature = "clock_2")]
    #[strum(props(
        svg = "eJw1issJwDAMQ1cRXqCfQ0+ulzE5GEwS0h6a7es0FISeQI/VmnqCPidtO0H7ZAusJLzMX7gW7245oRbL9zUsHIgaifHJvyQvWSkZOw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[cfg(feature = "clock_3")]
    #[strum(props(
        svg = "eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbUiDDMM75FoEQuQern9cChtNgd5/8u0Ds6Uk1XTGJCTxufuGB6tevxyYuhT4w+HHxmg",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock3,
    #[cfg(feature = "clock_4")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGOYNzCPRLpbRj7TMCTKcq/N+yTR9P1OrNkxLRqtarjNu7PCK+NhC/iV+AFZwGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj"
    ))]
    Clock4,
    #[cfg(feature = "clock_5")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL8S+gC1oj3FfCb0EAhtqR7s700UYdk5zCBLZ83QjxCXAHwbV+N4STh/nrBVHSolQ6tSrtM1JLDzbdMOMXn+Z/QAhakZog==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock5,
    #[cfg(feature = "clock_6")]
    #[strum(props(
        svg = "eJw1iksKwCAMBa8ScoD+oK7SXCa4CAQV20W9vYoKjzeLGRLNYh6kPHheCPIP5oYDmfbhmVK0Yho8pKjhe3sFDtrNue3u+cq4AohpGaA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[cfg(feature = "clock_7")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9AFqBQtCzWdCD4HQlurB/t5UEZadw0xkaawJ7XB+ceBuXI33S4rz5ynWol0lJ9Qi+TqHRoCdbZ82+DDqv6IHbl4Zdg==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[cfg(feature = "clock_8")]
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzdVhGXnMEOiTSyhHSHOAXI7F2d/yTR9nqkW66Y5oRbN1zk0Nvj5dsR1tH/DDz86GRA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[cfg(feature = "clock_9")]
    #[strum(props(
        svg = "eJw1iksKwCAMBa8ScoD+oO0mzWWCi0BQsV3U2xsR4fFmMUOiRSyA/A/uB4LUweLYkGkdniknq6YxQE4av7dXcIGf715OR69nxQ1w8Rlw",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock9,
    #[cfg(feature = "clock")]
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGObDPBLtYhkyjrSsCd1jTpD7nUzT9zO1asO0ZLSq5Trjxg63kJct4B/iB1fwGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[cfg(feature = "cloud_cog")]
    #[strum(props(
        svg = "eJx9j00KwjAQha8yZN9nJj8mgbbgATxEiUIFBSku9PbO2IUuUgkhgfnmmzd9vSz1eqb6GgwnQ/UprzO0DMabsd+t5bG/T4+ZToM5BjjiCD4kSmSJ9UQkpjwzUpkCIum1n6KTX4ZTkxq+nps2EWeEDqWDbxBlnVQ2AbbYk7NI8MK0AA9JJ4n/AKuhkxzcHpHV4LeADK9b7ImFaAmCrhnV8gu8AW1sXzs=",
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
        svg = "eJx1jksKwCAMBa8SPEBLAgUX6m1KEUSFdlFvX+OvdlFcPBIzyagYXDqChxisv04tkCA/ArlsHLjlxNJpRf3iljBqbbhRzvodEmpBJOAmXpQTayauM8wEDxrFXEH6XRp73yv9wB8y7CbZL1Pmu0W3KpZVSk5OD6FQRos=",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[cfg(feature = "codesandbox")]
    #[strum(props(
        svg = "eJyFkEELgzAMhf9K8N7OxK6toIP9gF13FzaYICpMhu7XL4kyusOQHhIe78t7tBqb6QG3OrsQAvprbAgIcnkGDdpQdCYYl6g8WQJ3/kpQQHwlIPAlAdmUiMLxpQTTxHd2qg5S4lSNQ7d0bX+HcWj76VlnwR7BWbEReBvFvikKbfY/IJY2lKCbs547Iu1Qa5BmKLJucmYHLCwFLlh65cnmCJTzD6j2yyo3Y51xG1honfM2F9aJ8SiM+j+Y0Wjb",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[cfg(feature = "coffee")]
    #[strum(props(
        svg = "eJxty70KhTAMBeBXCdnLbXqDP9A6u/gCboKCgoiDiL69KW3VQbLkJN+xa7eN0DtsKIdipI6BQQPJaMmKsLI/byp7y7+HvJfRilay1fk7K26f5jwtAxzGYYZwkkODcFBIcmQPPUlQXqQfaWL8oOEWGhwpp2aiFwD2OFM=",
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
        svg = "eJxli+EKgzAMhF/lyP9ljUqaQSvsAfYQoxs42GCIiL69rYIIEsjdcfeF9OnT9400RzJCmlbpIym14bqVbfg/hw6vSA8xdjeI49rfFQoHKZdzA7GClOkB8NBOxubc/ETZZ7RmM/bZXiq2CuXt4wXTQiiJ",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[cfg(feature = "columns")]
    #[strum(props(
        svg = "eJwly0EKwCAMBMCvhHygxF56UD/TShVKDxKo+X03esnCZjb2cirV0u6qieVg6pY4IMaMr11a1wPFzmR+c9x8l+PT3kJDAGBNlghYCnyYNayr/AMobBw8",
        categories = "layout,design,text",
        tags = "split,parallel,vertical,horizontal",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns,
    #[cfg(feature = "combine")]
    #[strum(props(
        svg = "eJydkMEOgjAQRH9l0nuRLRgwaTl78SNMIbYJAYINyt+7rRLx6mU3M3lzmNFzZwPmpxFKwHX+5oIRtcDbePg2uKTXqBt9iHijp2twaI24UAllKSPkUMhOfNRS2hxsySglGzEWA7uYyv+KERP1+WglZRVyWfAv+BaLpB099mvvhw7T6IdwN6KCIqQoKtAxkR+m0ak+l6NSbCt8S//uwcg2wAuktlFt",
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
        svg = "eJxFjFEKgCAQRK+yeIAtJdTAvMwSIYiK9ZG3z1qhnxmYNzxHoVLcge5NSCWAGnftNQvvJubelRzbkROUHNJ1dqpRLWDQaJALSjWSh4+taC3H/32Nw+QfFUYfzQ==",
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
        svg = "eJxNjFEKgCAQRK+yzAVKqejD7QYdIkpa/0KE7PaxIuLPLsybNy76M1FmzKCPYUExl/eGKwnDTCDx4ZbEWLG5QfubK1at2LGv6IpKdabcpj1HEroY+0JmlQI06oCxSpZGfkzOLUs=",
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
        svg = "eJx1zE0OhCAMBeCrvLwDMCITYAHeYA4xGc3gwsQY4s/tLRvjAhdtmn59DcvwyzgiLbHskZqQ3hLb2OckU0OkYfynHOnZhVe578L8zQl95Ec76PfqipTdTR5BImY11UR9rxt5hVYZWFXxSdDCKVeqwl7Uw19yAqyZQ5A=",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[cfg(feature = "contact_2")]
    #[strum(props(
        svg = "eJxNjEEKwyAQRa/ymX1ptBJcqDfoIYKRKoRSRKjevjMJoWEWw5/3/rjP0jJWT081Q9nFwGCSuVlMFNxdeHCx1LglxOFJKUL19CDEzkmLdODgaooNOZVXbowsoe/it6wtH4fKF03gP0aKUghuK++EoXbSebE3tBjoWhKLopziH6mZrj2Op/oDe7Y5JA==",
        categories = "account",
        tags = "person,user",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[cfg(feature = "contact")]
    #[strum(props(
        svg = "eJxNjdEKwyAMRX8l5L2sycbWgfq8l31EsTKFMoYIs3+/RFcoPlw159yYz1wiLBafdAOaZgaGUc/AAz/uxzcwOnNS3pkcfIFq8YyQJRhhs3hB+KalRIs0IcSQXrG0u2gqOONT9muA3AwvCo2SUkCtu4+dWdM7QCWLUlO5xUZ9C+saQRX5g/1vJ1Sja/ckd/QHIVI8Lw==",
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
        svg = "eJwlijEKgDAMRa/yyS42GYpDmxt4iBIFBQcpIurpbQlveMN7yfZqxwp7M7EQ7HHXpkCaRu+aznJtWDLNLOCpREQEZ2C5Wb4+90l/uHQWBg==",
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
        svg = "eJw9TkEKwzAM+4rxPVkcvLFCkh/suvtIy9zDYITA2t/PDrQYBJIlofR9dYE544ci0BWiHjvGki72KakttcNvnbtkJEZoe8aIsGW8I+wD2zYkWda39OHStOVKOtofDHSrjjxBcNH5SSE+uQaTjIFyoVCHQzf4yaacM/5ejin5",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[cfg(feature = "copy_minus")]
    #[strum(props(
        svg = "eJwljEEKQyEMRK8S3GtNsOV/UG/QbffFL41QShGh9fZN7GaGmcxLfLZXhUnJ4NnAxL9/1Ulc+83keNJZjr2WAZ92DJY+yD2ZTailXYIgfS7j2h481kpo5XJ83wfDkcw1AF6KRYfgLVm3i9AtFK+VJpDM6MtaAIHbRUgf6Yv8A28jLP8=",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[cfg(feature = "copy_plus")]
    #[strum(props(
        svg = "eJxNjkEKAyEMRa8SsteqOKUD6g267b44UoVSigjV2zdxKJ1NwkveD3HP8krQjUe9IHS998F8oc5sMLgTa8FNeRxlg3v4Jy9/uabYoHaP7Hgko44JOZVHbiRbCs3Fp2wtzwGlORfc+94ybB6vFvQ5Ci01KGGEXKmYm42KR0xAnLWK0wADcqUyf+YT4QtRnzg2",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[cfg(feature = "copy_slash")]
    #[strum(props(
        svg = "eJwljsEKAyEMRH8l5L7WBCldUP+g196LK1UopYhQ/fsm7iVh3syE+Hf9ZBgUkBhh6r4hDD735MWjv2gs+pZThzYCSvZXj17EdghtLiJcOiXXV+mnMZVIW3vRf5+9wBHw7oCuaSNDYDfezC6DHy5ZRapAdCGbVgIYzC5jvaEn4h92JCz/",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[cfg(feature = "copy_x")]
    #[strum(props(
        svg = "eJx9zM0KwjAMAOBXCblvLqGIg7Zv4NW7dMUWRKQUbN/eZNXrLvn9EvvMrwidHdIFoZNkRmj/PObenpR5OzD9MB/jEkOF5lBo6Q5FfPJWkyCDkGJ+pDrq0vZtVyrXeuft+14TbA6vBugcJpoJlomneZXANxMWHWkH0idawi6AYV4lsD7SF/4LWsA4Ng==",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[cfg(feature = "copy")]
    #[strum(props(
        svg = "eJwljMEKxCAQQ38lzF3XEVm2oP7BXve+2NLprYjQ9u/r2MsjCUliXUpDvRJ5wrHNTRJxIMiyrdIefSb6EK7Bemozx5fuctz/TTAn+gbwuxi2DGe8sVOH/4XiNFKH7oVdGQ142KljHOlFvgGN4SHI",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[cfg(feature = "copyleft")]
    #[strum(props(
        svg = "eJwlykEKgCAQheGrPDyAaepCGOcGHUKmoKBFSIu6fWPyFt/i/SRHk3NDK8Y7A3nUWX1/mabxM1313rEWs2RkG1KNiHDwOodkQ097wh+G3xUT",
        categories = "",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[cfg(feature = "copyright")]
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYmi6EcW7QIWQKClqEtKjbZ7r4fB7vsR5Vzw36ZnKeUNtZgj4dhafhha9y71gzLS4imTmGEhBg4dosovGp538mH9sVFd0=",
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
        svg = "eJx1jsEJgDAMRVcJWcC2aCHQuoFDiIrxJlK0bi8NqCjtJYS8/8J32zQE2KJHg3B6rBGiTJ6WmYNHbRGOZQwsa+uqJLROtDtjRaInaOUVvem1Dwyjx043YHaTQDr9gMoSA7rhPKD8XRUNVVCo0Iq+pS7QElY4",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[cfg(feature = "creative_commons")]
    #[strum(props(
        svg = "eJytjEsKgDAMRK8Sujf2o1Sh7Q08RImCggspLvT2NlbBA8jwGEiG52hJtE5AhxdKC6CzdMolRXB1+Qe3xX2G0YtBSejRRI0dMJJTGWxBgcGCfGKwyRt7817yjrWs+0rtb9ILHjYt/A==",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[cfg(feature = "credit_card")]
    #[strum(props(
        svg = "eJwdjEEKwCAMBL8S9gOt0t7Uz7RShdKDBBp/b5LTMsvsplEvpr/f3DLiDmq1P40zwgESrUAz4wQNh5I2G5T09q+SBBckapioGPRiRk91zSoLaA4acA==",
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
        svg = "eJxljjEOwCAMA78S5QMtGSoGymcQAxLqwBR+X0iAtupkOT5bcSGVkCOEeqIhhNJkRwgs1rtNc+9yuiIwtbNFqEbpSqrcPAnesQFPqIcHSvepfFF+7ZGiYqX5X6Wxul6wqzPhG+DMNzk=",
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
        svg = "eJx1jTEKwCAMRa8ScoESC8XB9DLBQZAOTub2VaNgh06fJO+/BElFcgSpjOQQRC0Lo8c7HHa+Q05PBHWMF0K1UGI82zSioR2ZYN85wr1AfjW+qE50Ss02iuR/rLtufdngFwvWNmI=",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[cfg(feature = "database_backup")]
    #[strum(props(
        svg = "eJxtjs0OgjAMgF+l2X2V/iFLgCfQq3eCJJhwIGqIvr0dGsOBLN3W9svX1sM03ebHAP27CRbg/mpCCtD7Q+ypVyW09eGHtfXcPUe4NuEsQNwlECjWY8BYpoxmYsMxQUK52E5LwBbSjaREPbqnqnZgYh+46Kh7PQEuOvMlvp4UBRRtjbUS/Zejj4Ti0yKjeapRUAnIr1P2l3/3B1mJRYg=",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[cfg(feature = "database_zap")]
    #[strum(props(
        svg = "eJxtzUEKg0AMBdCrfHIA22QcnMCM0F0X7bb70gotuBB1obc3IyIuJJAfyIMfm7b9d0ODfk7kCB8LbzElYiH0lkp1vGysjt17/OGb6OngX6w3hcN1HfYQLkKZeVYHK2w4nD9YHhzA1V3sUIicMGfq2FQWXvMO1Y4XUSAz/Q==",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[cfg(feature = "database")]
    #[strum(props(
        svg = "eJxdykEKgCAQheGrPOYCNYqLAQ06QFv3UUKBC7EWdfs0Iije4vHDZ0OMa9oC8ulIE/LhSAhTKVOuFCvqbPOwzqZxXzA7GjSMZ+kFGu09xWDxpuqKPpTVD6qXXRxmIcg=",
        categories = "devices,development",
        tags = "storage,memory,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[cfg(feature = "delete")]
    #[strum(props(
        svg = "eJxdi7EKgDAMBX8lZC+agGih7ezi6uBWUKhQxMHB/r1NLQjyhnDkzpz+CrBanLiFbtRR9VAWiDwDQ1vGiuf+Y5V5QWcaqZ2J+7FBIosa4WaLxPlmpAEhCXfiilXdIsmzNvQ2P/cBYswluw==",
        categories = "text,arrows",
        tags = "backspace,remove",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Delete,
    #[cfg(feature = "dessert")]
    #[strum(props(
        svg = "eJxdjkEKAjEMRa8Ssk9s0lRbaGfj2kOUKii4kEFEb29nlEGHQAI/78PL7TK26wnas6AoQnsVNISxoOKQN5/vkG/1foZjwYM4VvCs+8ABDBQiS9/iq/bj5jFwD5IlkDmwNWArQP6B5sg4kedAiSIrJY6T0WTy49NVQIxjTZC+ZdnxFtzCvgFHDjMK",
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
        svg = "eJwdy8ENgCAMheFVmg6gFi8eoBs4hBFiuRnSRNke2tNL/i8vtnIr/Al3hDYnIHw1qySkY5bupTtLqY+oA8fVfhzfSwVywpMCUJBlIzOrPABzTxjt",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[cfg(feature = "dice_2")]
    #[strum(props(
        svg = "eJxVi0EKgDAMBL8S8gA1SsFC2x/4CLHF9CYloP29Ri/2MiwzrCtpEzhzFPZIMwKnvLN8u1weR4SHE0J9Waqq4Hr9BXeswhA9LmTAcjeQJpW/ZIFMk259VSBL",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[cfg(feature = "dice_3")]
    #[strum(props(
        svg = "eJx1i0EKgDAMBL8S8gA1FaSHtj/wEWKL6U1KQPt7G73owdMsM6wraRU4chT2SBaBU95Ynn16HBFKg2moN6q64Hr9BbcvwhA9zjSB5W4gTSrfyQCZn2aBpk+6AHJIJ/c=",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[cfg(feature = "dice_4")]
    #[strum(props(
        svg = "eJx9zEEKgDAMBMCvhDxArYL00PQHPkJsMb1JCdj+XqMXEfS6s7sux0WgEg4IewrChMYi5ELYI3BMK8sdlauTq4J3re6822ZhCISTGcFy0xklDR9kf8SMH3T+vewA4qIvVQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[cfg(feature = "dice_5")]
    #[strum(props(
        svg = "eJx9zDEOgCAQBMCvXO4B6mFiKIAf+AgjxKMz5BLl9wKVFthssZNdk8IukG6LCqHkjHBFL2yRNEJuRcpNOcSDpYEzY905c27C4C2utIDmYaJKtXyR/hFaOlT++qaA1McedUE3AQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[cfg(feature = "dice_6")]
    #[strum(props(
        svg = "eJx9jMENgCAMRVdpOoCKJqQHYAOHMEIsN0NIlO2letIEL/1tX/4zKawZOMSNs0VFCEf0mZ/1tDghlHumeow1ioQzvfSc2ZfM4C3OSgNxNyhB8nwjNf4w3WDUNFJbSF/fBbQ3PoY=",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[cfg(feature = "dices")]
    #[strum(props(
        svg = "eJx1jF0KwjAQhK8y5D0xmz9bSHoCPUSxxRQUJATU25ukD6K0LLsD8+2MT/Ml47lMOQZGiiG9AysS5+Ua82oVhyTDq4HUZPCHGhz8Y8wRU2B3OopegQy0sLzsqIQyaEeuw/WNW27K2z/jGvJEEq721sZv79mBuigkbaCSILPHLNwe6tD/oA+kYkKw",
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
        svg = "eJx1ijEKwCAMAL8S8oDWhI4xP+gjSlqw0KGIg/5excXF6eDuxN5o3wNWPBIjWB6MDQ5V9tFV1t8xb/+VAtweT2IgDpujXrvVCmDjHfQ=",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[cfg(feature = "disc_3")]
    #[strum(props(
        svg = "eJxtzEEKgCAQheGrDO6dHIt0od6gQ4QFBS1CWtTtG02iRat/8R6fi2uK2wzJC1IC4snV3Ks0uObZg9vHY4HJi6EH0lFJQoNGtqiB0MoOyzmfgqtkNTKtX/lPJFtIhoxEA2yybuFr3qChK4I=",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[cfg(feature = "disc")]
    #[strum(props(
        svg = "eJx9yqENACAMBdFVmi4AVJcu84MgQVXB9jQgcKgn7hTdMRp55ZKZsEIJ59E03W76PvlsG3uuFkY=",
        categories = "devices,multimedia",
        tags = "album,music,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[cfg(feature = "divide_circle")]
    #[strum(props(
        svg = "eJxdzLsNwCAMBNBVLC+QQIEogGUsCiSUggq2j43JR1Qn69451HJl6CaiRxg2orEIXdLxbeadwiEqhdcKGn/k1mjHgvjxs1lmbvxHqTSqGahrS0OzcZyitE83rzssRw==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[cfg(feature = "divide_square")]
    #[strum(props(
        svg = "eJx1zFEKgCAMBuCrjF0gNAgf1MuUpBA9iNC8fc6lbz1tP/v+2Rz2ApkcaoQY0hmLQ2UQnnSUKGt1uCLk2g1x8HbhnrdXugNU1Vw7VS2TWm414rixZfVZGlb1o6DR/cMTyWO2ZtIXtCUyTQ==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[cfg(feature = "divide")]
    #[strum(props(
        svg = "eJxNjEsKwCAMRK8S5gIlQksLMZcJLgTpwpW5fbXSz+rBzOOJ5WolkXnEBqoRDLLWEaCyzFel5DORh3umNnh0csQKcn7sYan8iry/sZn+khflmiDy",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[cfg(feature = "dna_off")]
    #[strum(props(
        svg = "eJx1kEEKgzAQRa/yyT5pZsZEBfUGHqLYQgu1dNGFvX1nVEoXBjEf8n5ehnSv8/uGS+9GSuDJU5AECslziC1DNBOqkMYWzJNRrWjTqBfN5JW6oTuZaOh+OgalSYKIaC2HnGtNil5mW6bVsupkl1Wr9kA1U41sc/gCrtCYzv4DXIMaExfkYnPo3fYdYI5oUWDZjpqZQEdzrTyWeQRlbO/wp3/cn1cs3Dtmh4U0HT57bLtatdLwBazwbd0=",
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
        svg = "eJxVTEEKgDAM+0rpA6atzCGsPe/iIwYKE0Q8eHC/dxUvEkJISBL37VihsiAzwk2C1LQ2Ncuv1dhZS+OZrwKL4EwBfJqcz4PzYOw/hPLPyLI02oWN9QHjwRoQ",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[cfg(feature = "donut")]
    #[strum(props(
        svg = "eJxFTUEOAjEI/ArhzligrrtJ27MXH7GpJpp4MBsP+nupHgwMhBkmUx7r80rnyidL2JOm1WINpCgVQxY/6hz0MvjlL0ySkcMxOkZggtMB2gUOE8xwURicW9mNnFb6bev3C/VXZTWmrbIz9ff3iqef3D5d+yHo",
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
        svg = "eJyzSc4sSs5JVUiutFUyNNIzVFIoAjKUFJIroHw7G32IEjsABpwLyQ==",
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
        svg = "eJxNzMEKhDAMBNBfGXIvu5O17KXteS/7EYKCgmhBEfv3xuJBcsgMvCTkdhvQRfkrQb83rULxtqGz9PPP7nR3jaTwuo5SyMtUpnHukZdx3tYoX9Cc2h/wypXeKIVKi0b5CAqj0AsOq1TbrNt8tSdPHibT",
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
        svg = "eJxtjL0KAkEMhF8lbL/jJZv9g71rrH0IWQUFCzks9O3NcoUWR0ImyfBN6/e1P67U37NjcdQ/m64mk1vaYfOX9jy/bnSZ3YkrOFDEVI8cIUJ2K/FkDVUSSBxr1UEP6o8VRjZXULT7hCSeoeztMSI8J4RCNmSHLYjJ0nPsipApWVWoUCHOyD/iC1arMj8=",
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
        svg = "eJxtjk0KwkAMha8SZj9xkvkt1EIP0EMUFRREXLiot/elCnXRRSaT8N730j/n15XORzcVksZ5jpzJKpCgKoVT8MK5cqdeOSsph+Qjp+KG/mDuof9jAAED18wS4UuVY4FP0o66cUOU5LEgr6yZlirdl0LCJXpOCZFNAAsd5HUvV9QIo726cbJxJsE/zrh7XRu1KWDSNs799rjQoken6ugt6Gi/cVlHSE00fAC2y0TW",
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
        svg = "eJwtTjEOAjEM+4rVvUd7JW2R2i7MfIANFSSQGNAdA/wep8cQy0kcx6U/lv68YakmTGLQP9V4P9iXbCZrZbeJWnld3ndcqzkF5O4sL0CFjYgDBQ77KQcEJK5k5ZrD2HUsltIhTAO1cwM3C+Jqk51ZMom6gzn+KEf95R2dD6x81lSapv0AE+QqvA==",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[cfg(feature = "egg_off")]
    #[strum(props(
        svg = "eJxFjs1KBEEMhF8lzL3L/HTSE5gdEM8+hKigIOLBw/r2pkd3l4ZUkVTny/b19P1GL6flMWCZdNQHh4XSCvFBHeEkDFmzfFl9brDZ70YCHZRIpzFHTAb1aGAN8rLZBN7HXDu8GUJ92be7Cd23K1oSbkpiWIffSwcP+qt8vDn/50brsF6b0ltxoxW3CReG64NoU/CqcNbKcI95YY4b8+P985XOclp0obOWlP5c9GhXdIb2X+afPOM=",
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
        svg = "eJxtjcEJACEQA1uRbeDIgQ9htRsfgvjW7l1xVQRfQ8iEcE4lmvZ7cmSqAMKGGQWWAn/DCbxN2NVdCzxdLXUCfTlqB1EsIJg=",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[cfg(feature = "equal")]
    #[strum(props(
        svg = "eJw1i0EKACAIBL8SfiAUOgTqbzoE0dl+nwqelp2d5bPvakYCOKE9Tw9DgeENoyn3cJTTDIa+1aNUSlzuB7jqFbs=",
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
        svg = "eJxNjMEKhDAMRH9lyF3WpKu40Pa8l73uXVBQEC0oon9vWgUlkMmDl7GhXjo0jn5cgc1a1gJBrsOZXt/iyZn8q5uh3JXk7St2eBumYR/6sUWY+nGZHXEBA+FzfZJ4Kd4mcRNHwoRd0xA21p9cMeY7+sk9AOKPKVM=",
        categories = "text,arrows",
        tags = "outbound",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[cfg(feature = "eye_off")]
    #[strum(props(
        svg = "eJxtjV0KwjAQhK8y5N01m/9CLfQAHqKooCAi6EN7e3e3/rxIyDdLMjPb36fnGced23fUGhRTRIQHy00UksEN/VadQ//1s6cakcm3UcYUsdJblAPyoeroUQ0TR+KClWbaMJWKQKX9aS9UGIpREjlYUMXbCbLgET/NHdUEw/qbKXZazr/e6+V2wsw7FxyWICI6v3WxZ7GqaXgBLqs8lA==",
        categories = "accessibility,photography",
        tags = "view,watch,hide,hidden",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    EyeOff,
    #[cfg(feature = "eye")]
    #[strum(props(
        svg = "eJwliz0OgCAMha/y0p0oMLjQ3sALuJlqoomDQQe9PVCGfun7S/f6HtiY5gAfnugm+NGADhfr2dN8w0KShraTpGfWa4f+TD4QMlMk6GeqlnosBb2nGQo=",
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
        svg = "eJxNi80Kg0AMhF8l5N50E8x2F3Z9gvbau9CCgogHD+rTu/EHJDBhZr5JYzO18Mv4EUdSAUvRxoMHZ/cIVMVd3grsSL8c20C6Yp2eNq1T3w1/mCWjICycMSDM5bEv1lIx1KAbGo+O9WRfpMe4JBe9ATpwJZ8=",
        categories = "gaming",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Feather,
    #[cfg(feature = "ferris_wheel")]
    #[strum(props(
        svg = "eJx1j9EKwjAMRX/l0vfWJnW2hXZ/sI8YVZigIENE/950g+2lewkkJ/cmN5X7XB43lG9WxApzVlLLb+n6dFpxn17je8I1q4EY/DlXVEc7eF5MAHXamQ7cwGyNhz/CQ1VHOOPgj8S0qHXLPIIZTgc4hIZ5EDy1AMnLwfgxym0LgtUSz26Lf0OKS9U=",
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
        svg = "eJx1jkELwjAMhf9K6D2xCW2t0O3iVa+7SxU2GNuYY7h/b+YUdvGSEN6X914ablMN98JcHYhULlskT4IMFJDJnT0ICXjSDQGkjuQvYuFIvhKbLZBH1RkpgOJIbh2n7ZlCjWLKdFhDyjT07dI23QOGvummZ2FYM0FHBHWMH/CLlCk3Y24fkJfCiDWQX4rrHvVcwU1Wz1991k5V2IXtBJYZ+Y8U513FNxWgRhk=",
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
        svg = "eJw9js0OgkAMhF+l2ftWW0qFhOXixYNeuZuVBBICBImRt7fr36VNOl9nppqvawe34C6kwNxxxBz2QB4ZCMWjpiGoHksbdmoOmJ9JDOOTRp9wT4CcNNSj2GMGApkJAvLIXF3tUkhdzdOwDf3Ywjz143oPjgQsRaAA3kPxBr9IXcV+iUMLS3DqID6DK2xt9qOJ+6hm+WtfWhxZM/MiRc4bkn/wC3CDOUw=",
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
        svg = "eJx1z80KwjAQBOBXGXLP2t38tdD07EGv3ksUKihIEdG3N1HRHiK5JHyTyaZPxzmdDkj3qLxCekTFRmGOyqihX7116C/jdcI+qnNHAWzJauo0vSKFFgFDAuaifwKWPNhTIJMjFQ8f1/kRrt5vwU25XXchUybMJTlR8bb8gEtJ3QNZdKXe1MbbsoXc/OQrZOHI7ewoEDR5MUTL1JLbSIOQSZqf6bxb++VZM7lv6xNedWi3",
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
        svg = "eJx1jTEPgkAMhf/KS3eQNuV0uDI76OpOhFg2Qy6o/947FhwkXZrva9+L83hPeE1DciMlzG8jIfg4PTwZBcLHiDNZeRcP5b6Lzz45BqMrK2QJHooq8EdlI87aCwTNOlLJ7Vi3F9a6hZzDpqq8LfonhBuweK7YcSffvr4MZDZG",
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
        svg = "eJxNTtEKgzAM/JXQd7sm0+mg9nkP7nXv0gkWOi1OxtzXL1WREkiOy91xOrRzD89a3DGXBdDt0hIQqDgZow8mBF/qkVIio0cpi2Yz/4TRpxhodBj94t3QQRjdML9rgTmbeFVACqpVuEuMtm6yvgO7sOwswH75KgFTLSgKtzdn7lVfnIBlhlJd15U2lhXDBrkZHWX+QQs5Jg==",
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
        svg = "eJxNjs0OgkAMhF9l0ju4bRbFZJezB71yJysJJCtskIO8vUXxJ02bJvNNOy41c4erp4sFm9o2AoHRYkgmXZkXZzE45EUt5qdlup0sVW632iuXxrjEfmiRxn6Y757YKqqjhLrLF7ghlQv9FGKL8PBkCZMnIYRFPfuVe6t6cst1Y/1nMpsX0P57+dGPYNa0/JWeiIk32w==",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey2,
    #[cfg(feature = "file_key")]
    #[strum(props(
        svg = "eJxNTcsKwzAM+xXhe7rYLNklyXmH7bp7yQYdbFBKKW2/vg6lD4SRkWwptHXf4B3pydfKQe6+FghsgdFt4JOgLA3LWTDyulXusT7PlMKlBKaQv13+fZCnSOwJeVS2hC6SlKPVTmFr/7MHW1NSdI6Yw3dghmL3Fi8kLj0=",
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
        svg = "eJxFjsEOgjAQRH9lsne0XdrYQ8vZg165EyG2CQGCDcrfuxjUbLK7ybzJjJ+aHNEGuhrY2jQMhpLR4IKjO9gLK5wOtmb11wr5zoYqf9zclZ/Gfu3T0GEa05AfgbQRVJaDuN0H3JHKz90tYxWmJMwvuYRnanMM5AixS/eYA1mCSLw5N14ivjUddLkUvJfRUIWBWvhX5g0qQTcS",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock2,
    #[cfg(feature = "file_lock")]
    #[strum(props(
        svg = "eJxNTrkOwjAM/ZUn7ym1SwJDkpkBVvaIVqQbqqJwfD0OSFBZ8vHkd/hbKhljoBNvOws5uCQQ9K2MbpVXgE7JLGvAyHnX2eOX/KLoN00w+mW6FNznseRAe8Lj05+BWAiLXkzI03zNJZBrrPYf/T+MBUs1kgYMasMaxqGv8jN4A9olLXc=",
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
        svg = "eJxNjL0KgDAMhF8ldPcnwfoDtbODru6CgoJoQRHr0xtbhxLI5Y7vosxwzjDWosMslkBNPhAQpN9EfF0YBKw0I4VBRH0Ry9aXH6FV8j3UyuyrXZdtArMv23nUAjMu8SqBUigd+CNaOdAiQ1KAJa83+4rFW+Yd+wKtRywW",
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
        svg = "eJxNjMEKwjAMhl8l5L65hEUrdD170Kv3gcIGYys4xPr0pl0dJZA//HxfrO/XAR4d3qitBfhy7BkYmjiVXm8qCk0eiMui4vuplusmf9HZQ3zorF+mMI3zE/wyzuurQ2pV0mWAGzAJzIizCQykkEEIrMkIn39SShUKOJbnzAhurmRXdvYHbdc3Rw==",
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
        svg = "eJx1jEELgzAMhf9KyF3XZGortD3vsF13FzZQEC1Mhu7XL1YRESSQl7x8LzZUQw0vhw/K0hz4VlQMDGquRKYv7QxRron3RsJPneb3JfxDby/zQ29D305t070h9E03fBxSJiFpBliBieCKeBvBkR0ahEmErggjiRayU9yFP2H1gdUnrNzKFVVLtNzIPy+wQco=",
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
        svg = "eJxFjUELgzAMhf/KI/d2NrRbD9bzDtvVu0xZC07FlW3++0URJJA8ku+9lFOTI9pAdwtf24bBKKQMWHH02t24wEW7movjpkRdLVXlaXVX5TT2S5+GDtOYhvwOZKyg0jzE7TdwRwTeH76MJDntIKx2H3Xu1aaO2Ll7ZPwCMWGRTBnf1OYYyBNm2RtC7NIz5kDn1bXy1R/eyzcj",
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
        svg = "eJx1jMEKwjAQRH9lyb2xOzS2hTRnD3r1XlBoobQBixi/3t2oUBAJZIfZ99bHfh3o0pkTV9YRDvseBCr1FZLuvClkYmBsiwLn2rrjW36a4Hd6MPi4TGka5yvFZZzXW2e4Ekm+hlBSk8EPEnwGEwsE6ww9oLimpKnOnWxbCeL9ddpfJZ/5Oi+gKDo0",
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
        svg = "eJx1jc0NgCAMRldpOoAGUfEAbOAQRojlZgjxZ3tBL8bYS9v09fXT0c8JyIeFkkExIOzBJXrG06BEOO4ac2vQ6roIVq9TInAGRwVyy7cZlNULSFBVR+0vEQ0xjugZSbBBmShe+jy8AA0WQ6Q=",
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
        svg = "eJxtjDsKgDAQRK8ypBfdNYJFkhvY2osKCiJCRMzt3RA/jcXyFubNmK3bJwxWNTW4L1GgAkN+rzOC3EGknMmj5swrazC3+jegylOqPlOftszriEBWscJ5gwXCkChqlNwFlK4mig==",
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
        svg = "eJwljDEKgDAMRa8SsgdJTMCh7Q1c3QUFBRGhDtbTm9rh8fPgkXDN9wZLxFGBLTMxqGMgMDjafOqzH0zekJHQ4GjzF1Po6pMUjv1coUhENoSHI6qP/FPcRGpao/QBOqwbhA==",
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
        svg = "eJxVjMEKhDAMRH8l5F62E0p3D61/sNe9iysoiHiQYv/etKIiIUwyeZmwtOtA/8hfeIJPrhUSslowOg3mYRj5wXa2DLUcN+FVEppw5bz1C0iusyQlQtsl3OA0zj1lRPZMm0TGhynLsaqLihboRPUmcrBVFVLJVU50B70tMNA=",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[cfg(feature = "flashlight")]
    #[strum(props(
        svg = "eJxNjMEKhkAIhF9FvC+/yhIdtt7gv3aPCjaI6BBRPX1qBTHINw6jaWnXDH2Ffy6h6AgkiE/cmFq1QCq2MIf4DYI0TB2ZccVGMsuJdfrZzzpN4zzAzhUWCIc4dgWXunqqTes8TctY0C+ccvO4+ZYvXxIqXA==",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[cfg(feature = "flask_conical_off")]
    #[strum(props(
        svg = "eJx1jFEKwjAQRK+y5L9rdppkLaS5gYcoKEQo4odIvb2bgqjQsssMzBsm36dHpfPoTuLJPrCC4DnGSUjIr8cDCYdYBazpL+9aPptC0YFjgCv50DZL/l3GE9yHfgsGg2kHHjkSqm4QJUl1+IL5ervQIqODowVm5q+Pr7FVW6m8AYJXPKk=",
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
        svg = "eJx1jtEOgjAMRX/lpu/FtXOAycYf8AO+kWmCiSSGGKJ/74YgvJg9dDk9va1/dM8el0CtKFxnYWEg6aXfwJbtDxnOKJFJ2hPqjWc1k14GB7NFrH7PMrDCTizU+ENe2Ph4G+P9ivgKJEqI70A1YQyk2fl2G78/TswkugXsW6rxWGjaWrEUZVml6ngmXGEmqbrz3+HV5cVlhyUPS95++gOXBkjI",
        categories = "nature,sustainability,seasons",
        tags = "sustainability,nature,plant",
        contributors = "karsa-mistmere"
    ))]
    Flower2,
    #[cfg(feature = "flower")]
    #[strum(props(
        svg = "eJx9T00LwjAM/Suh981ltbWFtuBtF6+7lyooWJDhQf+9ie2cQ5CS5uO95CXuFu9nOHpxwB52rYrbVgFbB0ivZhXcf4MdVwD7io02NwzJuCbNEwp5sJmDbi3T1CyTcTygyo3kyohKBLfhJYNLlyldTzB5IQWkhxfYk3++PZEKHNx8UTaAGpBGki1TloNZ1fJaGswvTu2Gu+FPd/lIRH8YL7G5T5c=",
        categories = "nature,gaming,sustainability",
        tags = "sustainability,nature,plant,spring",
        contributors = "karsa-mistmere"
    ))]
    Flower,
    #[cfg(feature = "focus")]
    #[strum(props(
        svg = "eJxtyrEKhDAYA+BXCd3L3Z8iXVrnW251lypUcBCRom9v1eUfJEMC+UKa1jSPWKNxBmmPRlj7uLsNn+duw9JvGUM0fwffNT1BfGsEtMy3vIRy4uEyNQTLG6RAfFHS1pXtG/Wg/BotLYuSJ5zlNHc=",
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
        svg = "eJxVjkELwjAMhf9KyD1xSaVboS148+LV+6hChR1kSNF/b4tQN3IIL9/LS/xzfmW4Bbyogg7Xaa4NhlakpJlGdqbPhIStJXYL8aRV6KnboTnBnI/bBNAiJtVFFnbQgGaL0R/a2ejTY03LHdI7oFiENaAipE9Vrpl+OPr+o1gQKST/hB0aC2lHXxwnNNY=",
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
        svg = "eJxNjkELwjAMhf9K6D0xyWa10Pa8i1fvowoTepAhov/eTKUbObxH3kde4n18THBJ7nQA5aEfFRTYRlBRz/vCKCQU0PaoU0ehWxEQ8p5CpaOa1RbwPxiUN7A5l+NuKcyx3OZSrzAn5x2UV3Ky6PurBv3iHNt34kH6p1brbEc+ijQssA==",
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
        svg = "eJx9j8EKwjAQRH9lyT1jdtPGBtqee/HqvUShQgUpIvr3Ji3YHlLJIdnM25ndOtymMF4pfBrFlaLwXu6pUVa19WGR2/rRPwe6NOrEBiWJ6YpeSMjEw1q0nMu1plgPFn7zw3AefkSVnvITTBKO8J2YbTvJy6JM8Sl2Db8L40jsUWh4DZshuIQQuyTvEQ6OkhFsZHKAR5wy+vwBFgcdB+F8RJUc7C5QwNJswxHJLWrSorPNFvgC22xqZQ==",
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
        svg = "eJxtTsEKwjAM/ZXQe2v6ZrWBtuddvHofVZiwgwwZ+ve2TGYPI5CXl5e8JDyH10i3qC5C4P44gEBcwmpoXN2fU+FjZ6TpWHMSI5PxtcQmcBXORnpwu05YnErhUC+mkB9znu6U31HZTlH+FISiOSrUoVVOYXvPerKSNYwn1q4gSnaL33UE/5xWX2kcvwFcOm8=",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[cfg(feature = "folder_git")]
    #[strum(props(
        svg = "eJxVTrsKAjEQ/JUlfdbb3ZC7QBKws7G1sDuiEOEKOUT0700UEo8thtl5MP4+PzJcgjoa4CGTnRkYhu+x5tPUuS486xGdtB9pQms1ukXjxIXwvserE+Rg/huAnySpBJHQQRX4rKLf1RXRp9ualiusQbGC9AqKKr4LSjX95OjbZDJAkqUXNGXcCh8b8TcG",
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
        svg = "eJxNjkELwjAMhf9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEEjyPfJeuk33GU7ZHJhAaHSTgABpsRUrR1/JMjJGq3cr84Bx6BJgDAHjgnvRUTZAXzAK/Yl1eogpadc8S6qXtS5nqK9shAzUZzYcDKy6NtEHl/QLeBX1cNahB+3+pnON41umjb0BySM09Q==",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[cfg(feature = "folder_lock")]
    #[strum(props(
        svg = "eJxNTssKwkAM/JWQe2KSvmG35168ei9tcQsepCxV/95d0Co5ZMjMZMbdxxhg9nhWAZOhHA0MJI2SkV2qSUhZuaN0JwsFd8VPAsp1zd2NW0vQDkI+xGDyJ05oN66wd6ec2rttmSK8PGqD8EyrRHiscwweW4SwrNcQPVYIWyazLRt6d1S29LXZ6ZurIFSC7HYkvAHmlDS9",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[cfg(feature = "folder_minus")]
    #[strum(props(
        svg = "eJxNjjELwkAMhf9KyJ7Y5OrZg7sDNxdXB7eiwglFHETqvzfpYCXD40veCy8/x1eDa8FjD9o1iaOCQreMkp6Glcm40Y5T+O2EhGMkThPxoAa6X+PuhHDo/z+AviVcLMjCCfygZ6x54y1qnu6PG8xaULYIsxRMCB/HYCqLmtdd9Qs/gyiI",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[cfg(feature = "folder_open_dot")]
    #[strum(props(
        svg = "eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA+5nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171sg/4b6iUL8pa7qzyWm3vJ9TPc+1naA+BkPeQH3qFgOzrqX00fkFjJkzVg==",
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
        svg = "eJxNjjELwkAMhf9KuD3xkqtnD64FNxdXB7eiwglFHETsvzepRysZHu/leyH5ObwKXDt3bEB84TgICPh5BOXUrh7VF9xRCkvGyBQjUhqRWlEj+7VuJIRD838B5M3hokViSmALObs+b+yLPo/3xw0m7hx7B5OoRgcfU1HlWRU2rMIWpsps3a8bajcs7BdfMDO1",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[cfg(feature = "folder_root")]
    #[strum(props(
        svg = "eJxNjj0LwjAQhv/KcfudvUsbG0gCbi6uDm4lChE6SJGi/95EIZWbnns/eP1jema4Bjz1oF0WOykodN9T0vO4MRXOtGdn2k9I2FpiNxOPWkAPW7w6wRz7/wbQVUwqQRZ2UAW9YPS7uiL6dF/SfIP0DigGYQmoCOlVSKvpJ0ffJouCDOvQCj4TWi/x",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = ""
    ))]
    FolderRoot,
    #[cfg(feature = "folder_search_2")]
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXQe+KSdt0KbcGbF68evI0qVNhBhoj+ve2EbgSSvOS9x/PP6ZXhFtTZgHSZ7SQg0K0lKJdxw1hwxoGcbjdGJmuR3Iw0SgFy3OSVCfpk9g4gb9apCInJQX3IVUV/qCmiT48lzXdIn6CYqVeQvmWTui1B1Vmof1L0LThrkgHYrL0Hts3wBwVbMqg=",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[cfg(feature = "folder_search")]
    #[strum(props(
        svg = "eJxNTssKwkAM/JWw98Qk21YXdvfci1fvZRUqVJAion9vqvZBCDNkhsnEe/fo4ZzcUQSU26pTUGAbQUU91YVRSCig3VF7T8GvFhBqGgoDHdSoLgL/hVZ5Yzb2rFyOu+lnjuU6luECY3LeQXklJ3vD9xfN9JNznAve1BLEytTTLjEfYfAtig==",
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
        svg = "eJxNjk0LgzAMhv9KyL3OpMSu0Hrexevukg06cDBkiP57W8QPckl4n/A+4df/E7widsRA/JCegaEuY/I2CbbhVpg26GfU4Q06RySLMEZkBF3y5Qu0xSe2BTuWn+RKHbV3IJ/sVwy5iVxquiZrPJ3WhiqqfJEwnOwgcKqsLW8wJQ==",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[cfg(feature = "form_input")]
    #[strum(props(
        svg = "eJx1y8EJgDAQBMBWji1AvTz0k0sHFiEmePlJOFC7N/EliL9lZ9eXtBqVU+BAmvKmJuCaL8EIOnI0rTaAnknwfTsEvy+mFAUzO2Kn3cDNWvu26d8+dAOCRCZV",
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
        svg = "eJxtzbENgDAMBMBVIi+A8gWicLINBRKiNtsTO49SJNXL8vmt9/WcyVAEkt5cZG+BCGsTIFU3N1VD+i4fnXrycmmt9zglYfks2cZlXP5fhv0AsdgrUQ==",
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
        svg = "eJxNzFEKgCAMgOGrjL1bTTII1Bt0iLCgICKsB719Tg16GP9gH9Nu9+5YwRukDsGFVJkac61uy93qa342WAxONAANt6BGCSl6HsjDmJHVx36uEMngiBBqZErTEULkjS2rahmRKorUj9Uvn34BXPksLw==",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[cfg(feature = "fuel")]
    #[strum(props(
        svg = "eJxVjTELwjAQRv/KkT019yWRBJLODrp2LyhEEHEQqf/enoEm5YbjwXt36XF/3mhBVuwVLZyVVfRdEVg3//eYDmKNqbmuSrE6sZauma/5Xeia1YUdAZObQSAjo6FxOvZM+HCQVKJ9yrZgU5lEbWyEi+lZY4pDsN31wUfNg8OZA/ntyQ+p9zkw",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[cfg(feature = "function_square")]
    #[strum(props(
        svg = "eJxVi8EKwjAQRH9l2HvW7C7SCkn/wKt3SYvbm4SA7d9rKggeZhh481JdSkPdMilhz2SE1zo3zyQjwZf14e27twPWvT+ndOrelJ735pgzXS+QoSgilMcgR39yk1hiUEgwNhhrsO52698VVj/z8INv3b8nAw==",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[cfg(feature = "gallery_horizontal_end")]
    #[strum(props(
        svg = "eJxFi0sOgCAMBa/S9AICfhfADTyEEWLZGdKg3t52o6t5eZPx58YEKeDqYG7WYPSdXtF/YoKx2eEXNe8MV0lMAa1DoFwOYtkLwi0wCFUo5gnYa6dFfAFrcB4P",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[cfg(feature = "gallery_horizontal")]
    #[strum(props(
        svg = "eJxdjEsKgDAMRK8S5gJiC+Ii6Q08hNhiupMS/NzedCW4mYEZ3uNjNaUsWALFc5yReOhT4lY2Iy11VxP4QY8ggq6aTX0IoHYLvDynjnUg8ef7CV9g4R4Y",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[cfg(feature = "gallery_thumbnails")]
    #[strum(props(
        svg = "eJx1i0EKgCAQRa8y/AvEmIsC9QYdIlIadyED1e3TVhG4eXz+47mSNqVyeRhQ5Qi6X545qnjwBJKUd9G6LYIbWhDcsapQ9FgsGRZuol0fMfcEdxP+NQ+yJCxn",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[cfg(feature = "gallery_vertical_end")]
    #[strum(props(
        svg = "eJxFi8sNgCAUBFvZbAMCfg9ABxZhhPi4GUKidi9c9Lazk7HnVgTBcZ1hRCt627XL20+MmEQPv8hxL7hSKOKoF0JiOqTUbYjbsSeeCorIlUzrWuFfYdgd8w==",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[cfg(feature = "gallery_vertical")]
    #[strum(props(
        svg = "eJxVjEEKgCAQRa8y/AuECuFixht0iEhp3IUMVLdPd7n7PP57fO2mlAVbIK8uIvEyUOJWDqP2CDxISz3VBK7vV7CC7ppNO4igfglDG0Lif28KflWgHfw=",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[cfg(feature = "gamepad_2")]
    #[strum(props(
        svg = "eJxtkMFKxEAMhl8l9D5xkklmUugWFi8e9CFKFVZYxIMH16c36RQrIoX508yXP5NM19e3F7jRaSAa4NO1urD/5gFuvKXn6S6oeepsJEtnrbMu4TD+JcOVdzvFTLsj92LS//i883bweeft4N+Xjws8n4YnalgY9KFitUVAIMeXCo7NoKCOa8KcK2ZlDwgpU2hDUr5nrFlgRKEKDCQougV1KVC6k2tZyZVQEypwoit5gaTtPLPzgZHbGJvXXgSLyHJcbCCq1UdqQLaiuo+no9PvRqms2V1VvJG/K1WvEQ9NU0NWi0GaHxoD+BR9EFel88/ksC/kK3YVW5q/AYGvaO0=",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[cfg(feature = "gamepad")]
    #[strum(props(
        svg = "eJxtzk0KgCAQBeCryBygnP5woV6mJIVoEUJ5+0YdcOPGp/LNY/QVbifSZAAnEF9OSYkGNhAJy7fVY1ZWV4tsyKpKKUrD0qNzNbhy/TpIZD/3PLJTzSFXqOYft0fhXTh95NUN0JnK3k99vOGInm4yj+UB+wPkUjzP",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[cfg(feature = "gantt_chart_square")]
    #[strum(props(
        svg = "eJxty8EJgDAQRNFWhm1AVjFGSNKBRYgJbm4SAmr3ugjiwcsc3mdcSUvFnmMVT2wJ5fDUEiTlVepDt3SEUze4Rg/BbXMVRE/TCCuDusrHLbgV8xOYwUb6t1yJLyT/",
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
        svg = "eJxFjEsKgDAMRK8ScoBoqrUIaW/gIUoVFFxIcaG3N1WwMJ/FDE+OeK4we5zY0gCGXOQWVBpqS64YgzTlGCRtOe0LZI8GId0eufT1tp6+OUildtQrTtE8gv05D/y/Hl0=",
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
        svg = "eJxtjrEKwzAMRH9FaG9qqTFe7CxduvQjQhNqQ0hCa2jcr69s0yUE4xNI946z6zKlKcwjrEuY49shKyAGUWZoqxBjZ89/Z2df4yNCcmgQNgEQ/BiePjrUCJ8wRJ9TMpKNnS3xGzuUHEgkxzy58lTW4q3Rax89DA7v0sHcTKN7bjTkr+QRqJO+EgHnjqbIt3QTbAf7dgerCl+O4R/wK0M7",
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
        svg = "eJxVi0EKgDAMBL+y5C4aRbHQ9gc+QqqgICLiof29SQVRctgsM2u3dZ+RakfcEhI7aghRaifBGt6W6ngb1jNsM0LKMESZ9IRTF+I89Ge9OMvd1zrGa8HkaOAeZjQwqOS4kE81xf4GvwYpow==",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[cfg(feature = "git_commit")]
    #[strum(props(
        svg = "eJxFyzEKwDAIBdCriBcoWjoUTC4jGQKhQya9fdNImkm+/33R2rUVUE9IjNATnghqM2U5os7S6lPAOZDRVDbijeC08IcWjeeejMsUU7p+/AKGpSEM",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommit,
    #[cfg(feature = "git_compare")]
    #[strum(props(
        svg = "eJxNy7sKgDAMheFXCdlFk0Kt0Dq7uLpLFRQcpIjo29taLyXDGf4v2s7OLiPYwyApBHvGdQYF1jqPudYv81k+9f6RKVr7bYLBYEsC5CR6BobCH/nlvQw0kBQSkGrULzPOuKs+eQHGnipz",
        categories = "development",
        tags = "code,version control",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[cfg(feature = "git_fork")]
    #[strum(props(
        svg = "eJxljLEKgDAMRH8lZBdNBKnQOru4upcqKDhIkWL/3paqVCTD5bh3J81qzTaDVVgjmFMhcVAfVGAnyxR38sYi0OBD+2j+UJ6myc/Uro8FJoUDCWgdaQaGKhwV4etF7gse21iNlbzIQOzqN7kAOPg1ag==",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis"
    ))]
    GitFork,
    #[cfg(feature = "git_merge")]
    #[strum(props(
        svg = "eJxNyjEKgDAQBMCvLPcBjUIwkNwPbO3DKShYSLDQ35ucCmGbXXa8bEn2BXIFMgMhBeoJcuti37w3+5/lw5Jq++EKHfFcMQcaLTozuejg0GpyK7AAfgBhoh93",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[cfg(feature = "git_pull_request_closed")]
    #[strum(props(
        svg = "eJxtjT0KgDAMRq8SsldJxaDQ9gau7lIFBRURh3p7+yPYwemR7z2Isstp1wms00gNgr0TT40VGlUmbdSbxTnGnFrOo2O4Zhg1dtQAUVH3VAcd5k9ukqASDPyvWsEiU+uyT+Ao/rs9WgQn0+UhKZShMQ9DuDca",
        categories = "development",
        tags = "code,version control,rejected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[cfg(feature = "git_pull_request_draft")]
    #[strum(props(
        svg = "eJxVjEEKgCAQRa8yzD5iisRAvUHb9mFCgkRIhN4+tSBbPf68xwhtvXYGvMQeQUeJxBNDoRLto5V4sywYVjWro2M5N1glTsSBzUNW+fQXRFdDn3J2NxCovA1dQUxrTEirK2Vu1A0d3S8Z",
        categories = "development",
        tags = "code,version control,draft",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[cfg(feature = "git_pull_request")]
    #[strum(props(
        svg = "eJxVzFEKgCAMgOGrjF2gNsEK1Bt0iLCgQCIkQm/f1JdiDz9jHzP+iD5sEC0qBJ8s0ijNtc507ezMn8lZN62/6FruHVaLMynQu1oYGHoZkvIzFFqIM+E4N8hkcUJIXJ8lqsmyMRVZjHsBybYqvQ==",
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
        svg = "eJxtzVEKwjAMBuCr/OS91aSNU2h3Aw8xoqDggwwf9Pa2K4wyRggk5PtJsudsrzvsl4mVYN9MJ8KcKdKYDu06po1azovlc8/e0+eBW6YrR7BOAsGxlhNX53WD1FTVXUa8ggMUg/nBsQ9gH4sNbldz43wp3rVAaS2v+sAfwH43+A==",
        categories = "accessibility",
        tags = "glasses,spectacles",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Glasses,
    #[cfg(feature = "globe_2")]
    #[strum(props(
        svg = "eJxtjkELwyAMhf9K8K4zUXED67mXXXsXN+hgh1FG2f79EgttB8VD8njfezG9ynuEW6euhCZ4wNBjLAQEVp7mbfZsqJxOgua0BiI44/wQigO30DzdbNc0gqRt5c2guYgSvbUD6ebr5sstTaMzGA+uIbcxFAY877+n94XYtMZ/oCdjw9ZYH1N93qF+O4WkYOJhFdRPk0wtfv4BXCBBdw==",
        categories = "maps,navigation",
        tags = "world,browser,language,translate",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    Globe2,
    #[cfg(feature = "globe")]
    #[strum(props(
        svg = "eJxtjUEKgDAMBL+y9K42UW9tf+AjpAoKIiIeWl9vavGiEsjA7MIaP+9+GbFbRVrBByEL401nqpw7s8zriEBWSRpYIIyc25Gedmo5s/XHhMGqjhjcU1vWuJ+WIzQgjZcs/mXxlUjyTFtpxV3VwSv5",
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
        svg = "eJx1j0EKwjAQRa8yzN6xSZyYQNIbuHUvUVDoQooLvb0zNLUVmk0g5OW//9Pz8rrDNePJWuCzHXZMIYIe2Ke9vvapPMYy3KC8MxpPHmHM6BDKR+4zOTF/bCBjJupIh/ppg9QUQ55/saqx5HgDXsTi5co6iqbNKiFNuPbtqIutFo7suoQn9o1pEhNqoGxrj1vCZv9a/gWGjGh/",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[cfg(feature = "grid_2_x_2")]
    #[strum(props(
        svg = "eJxNyzsOgCAQhOGrTPYCBmgsdrmBhzBCXDpDNj5ur2BDM8X/ZbjmzXALBcLT9yrJVMjNhPp1T9BcdrWeIk/tEPlYTZGElgDn9ZfWBnEe4RzkBVxbHj0=",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[cfg(feature = "grid_3_x_3")]
    #[strum(props(
        svg = "eJxty00KgCAUBOCrDO8CYRIkqDfoEJHScxfy6Of2pa0ENzMwH2Nz3AQc087iSM2Ex5Em3DXzVyPhSkG4qrdDOXh7rMIIjhYNwz+UqQE19cVAn11QUysvySYsqQ==",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[cfg(feature = "grip_horizontal")]
    #[strum(props(
        svg = "eJx1zksKwCAMBNCrhLlAsZCFEL1M6KLQlav29hU/GEGXYV4mEb2TPhfpG+BOkH4BHpTyhChHTaNY1eNmF8p0lBVeoJH2s443XTmw3G8eY8x6qB+FGUF6",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[cfg(feature = "grip_vertical")]
    #[strum(props(
        svg = "eJyFjTEKwCAMRa8S/gWKBQchepnQodDJqb19g0YQVJwy/Jf3WO4sz0XyRgSQfBHuBGU9SHzUNXGjdPa22ssE6lRhrVLK+X3RMEuW/kY2Nn+PO0F6",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[cfg(feature = "grip")]
    #[strum(props(
        svg = "eJyNzz0KwCAMhuGrhO8CpQUHIXqZ0KHQyUlv359U0GJoJ0Ef8xKWLcm+kuSAeQGl8wBJCXCIPOlr5Kru+4quL/6PskY1Qc0PmEZ6PmbuU3Utxd5m5Smba+Z2zfewA647YnE=",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[cfg(feature = "group")]
    #[strum(props(
        svg = "eJxtzjELg0AMBeC/ErLHXp7IIahzl67dyymNW5HDtv++F2mlg8sjw/ce6R63bDT2fKkpXpsURCutWgFBYOChO7kYut1ppNqQCqNAoKotgfVIQknjihSoYHHosyZHNhL03CTZZgXiLwjWf7tMKdPy6lmZSkam95bPecy2XTbNd8s9N15y/i15J/Cve+R9SrHXPgXGSW0=",
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
        svg = "eJxljUsKgDAQQ68SZq92ipQuWm/QQ4iK40IQKX5u7w8pKLOYkEcSN9VR0HoKrKEXS5UrLqtyLxjZwGQlyuy8hOeuidg8acL8vHVoo5xKEaQbeomeLGH3xHfsCqTWYMBWcsX/wcDqyw4Pvy4G",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[cfg(feature = "hard_drive_upload")]
    #[strum(props(
        svg = "eJxtzUsKgDAMBNCrDNmrTSmli9YbeAhRMS4EkeLn9lpEFJFsAo+Z8VMdBW2gkS1sZtLBUOmLBKW/uWINvbgH5q6J2AJpwh6IDUG6oZcYyBHmC9ahjXJ+KsVS4NVnwU5yxX9T6msH9GouBg==",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[cfg(feature = "hard_drive")]
    #[strum(props(
        svg = "eJxVjs8KwjAMxl8l9N7YxDZ10Ba8efEhBgoThniQ4Xz6td1YGTnk3/clvzC+3k+YKSpiBT+OqqTccs4z13EKp6JK4dN/B3hEdXdoHTgkAgbiSXrOhamRq4EOA82TllGfs0cLXrpr25GgF7A3j2ybRRP6Diif/5ff5WsKjVNWTkFDK6pspNJID+ptWV1kdhuZXb8A4LU8mQ==",
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
        svg = "eJx9jEsKgDAMRK9ScgFNrKIQexsXgrhub998IKvS1YOZN8Pf+z+p0Q0XpIaGKsgCCWmFwos6hcPE3VXlzNXwdHUDWxL6Ake3Vup9jo25R7gd2bkreA==",
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
        svg = "eJxtjDEKgDAQBL9yXB9NLioRkvzA1l5OIYKFBAv9vVELg9gs7M4ydh22AKPDrgJFwaC35TV5mwPTNz9A0YfwHHmZgHeHqkWIDgmBj9Tu04MzAUlQkgUBCQ26qFO+thMkACou",
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
        svg = "eJxdTUEKwkAM/EroPbGZ3XQrtAUf4CNKFRSKePDQ/t6EBZESJmGSycywPl932nVs0NAOHz63SrdKp+EUoml4z58H3cbmqp0Y1QaCrly4LKxijmycOAm8m9jsoEDrpQTRzFmShWnY/Zn2UjpKoouKmgAhTlJ63yGTyplqgPmhZAZlp7gcAkC92NLGM1ycKCwY0hVXafnlfgFoXTYz",
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
        svg = "eJxNi0EKgCAURK8yuM/+V6IE8wYdQn6BQYuQFnX7FCFiGAZm5nnZsxwb5J4VGwV5WuYSpILv2x78Ga+EdVaL0+TgooUFFTEGPVmwEExXyurK1f+PYgMekyb+thenCB9G",
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
        svg = "eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUX09v3yWFmNoZYcvCnp2d5/Xn9/aC3y3LVKpmy5F07VbKAv06mpHqwSrQoVjKblFQkdk6i2sSsLNv64iLb+pCCCrncXgli/RRThdohLoFzzqzcJE+qhxEPh6MUOGEV1glqAbIA4RKkURwxzWUHEg5P+iQR4UZHo8jYzqvNyAyGnALkFzjLJmyS2iJ5tF0l1EhNUjd0VW+I9u1Qxji8lzvncmYQCSv7gK4+XLyD5Lq79dpAxAgmS8oRB6F1KBHGJK3hgzHMW2pReKtWsC/anv6+Pr/f6WaXxWyhv3tW5IVuIwF1aPsHpnB3eQ==",
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
        svg = "eJxNjkELgzAMhf/KI/e6JkNXoe15h+26u3SCghtDxrD/fimiSCAvL3mEz3+674BnoLsw2h/bTiCwWmx0utZHb+Rx8FA/XCj6U/kR/TS+e2QJVBMWFRFVDsQNIXNZa7SEok/jnKYeKQdqCbNmCWkpRiPrMfqN7KVkXJtzZV2z9p3BGqmcONhbA+Ed5Q9qOzPA",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[cfg(feature = "image_off")]
    #[strum(props(
        svg = "eJxtj0ELwjAMhf9K6L2xyRrZoBt48+LV+1BhgogHke3fm3RlvYzCSxpev7ym1/P9gJl6xw4W1qJ1LnXJ4yEdzDSkz/id4N67CwWMBFlHBoYAevOMbZPFXph3SCtcaUeXd1CDsmJLZ4uobtj8OlxtJRVJAbQ7cRTVgcmJsOsgS8inAflRLBntaDdRvKGIhcYg7JFZu0heATX5Bmf9mlylMjx7PlfnH2xXS9E=",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[cfg(feature = "image_plus")]
    #[strum(props(
        svg = "eJxNj8EKwyAMhl8leLczGVoH6nmH7bp7cYUWujHKGPXtF+smIvj5m09J3Gt4T3D34koISJ9+ICBQvFDy6azbLOnWZOA89SK4Q/4juGV+jpDICy0g4Y6NgYbJt0RZzVKj2qLSTz0VlVnVOK9xGSFuXnA1ph1rfsJKKQb3H+KRh9Dy2Clryl7bVZI6SxbUxQBh7foLDu8+nQ==",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[cfg(feature = "image")]
    #[strum(props(
        svg = "eJwli1EKwyAQRK8y7H9S3dBgQT1BLxGMVKGFIkKT23fXsDBvYeb5llPHEWghnCObgAXHwK/uvQSyjlByfZU+/uhv6kWfakvvjDa2SZSH4FTI5Cqj/269YA/0YQt7n5bZuPXKjcEwehPPjh3McwVblVWKf3ASKYE=",
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
        svg = "eJxdjTEOgCAMRa/S9AKmJUEG4DYOJARIdIDbS1GJOr3hv9fakmOLIW1QckjH7lCBgRWIQQFp9HZ5DG+HV9khEUITciddrJ1MErxkGTXO5nZGq3/q3OSewe8fM+UT8KAuqQ==",
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
        svg = "eJxVyUEKgCAQQNGrDLOvHIloMXqDDhFTYNAiJERvryIIrj78x/J4eW+QZJA0gsRWX6LQ8tLc8nf+Di6DB2mgLUxrxTpH2t2sqFMG6SQaTA==",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[cfg(feature = "instagram")]
    #[strum(props(
        svg = "eJwtzVEKwjAMBuCr/OQAc+lm50Nb8AAeQlyxBREZBVtPb7KOvCQf+RO3xUdBivmZiiczErbq6Uz45rWkLgJGvO3edAjupLngPveSsHq6sQXzMC3XGTNGsJYZ7IQLuux2LP30gEaDe+V3RGVPvAxyvZresTwStGrN9EYyuh3+HPgsAA==",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[cfg(feature = "italic")]
    #[strum(props(
        svg = "eJxtzLEJACEMheFVJAuckVwh5NzmCkGsdXuTKGJh9RPeR7jk+ruGH2AE16UkCZYmQQ+JH0WJjSoJfo7vpHbqB7pYWttppXHTAWEMIKQ=",
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
        svg = "eJxti00KgCAQha8yzAVCizRQb9AhIqVxFzL0c/scF9GizeO97+O5klaGy2OPcLcsdWiEM0cmj8oiUMobcevBdXIIbl+YIHqcLZjDCBfy4UpXMfyJsYrpFQ9Q0yT9",
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
        svg = "eJwljEEKgCAQRa8yzN5Jc7IW6g26QLuwwMBFSEjdPi0+//MWj2/P9YqwOZx7UFPRQQIZYlAtkYvQUbfpkyIWtauhAVplNaRgwQt627Ubb8ORQ9ohO6QBIdwOlfnocThWqOKv+BcLHB52",
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
        svg = "eJxNjEEKgCAURK8y/L2Fikng9wRdIiwoMAhpUbfPXxAtBoaZNxPSWlKeUZhc4wjpZPKPuZi0RDG0LxPDPh4LJqbNaBjVNx2qBJDiV8sO9QUWdjAGPiur7AfeIz8ehQ==",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[cfg(feature = "keyboard")]
    #[strum(props(
        svg = "eJx9y8EJgDAMheFVHhlAkyK1h9YNHEJUjDcpBXV7rQfxYm/h//J8nMeEeAYyBJ3XRVMgsYTjKfs6Jb0vJtwvDSE+vfN13nV+G5JiCtRbOK2YJVOOHxIuWFMw928OYn53poS2gC3EqvArFxN4Tpg=",
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
        svg = "eJyNzVEKgCAMBuCrjF2gZlE+mLeJCEKFesjbtzUj8il8+NH/23TbGmbINKExCCdnh5BNuUoSeteI8k4tPxJpOeA9SlZHh4oKoVY7yceWFdXeb6lDvf7D+QPbgu2LU9zyEgOkuIZj584AnxZG6GEUVoC/AG3vRBo=",
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
        svg = "eJwli0EOgDAIBL9C+IC2evDQ9jPaWBLjoSFRfu9SLywws6nXXckyr0yPHNoyh42pVTmbYo9MHdTjHYG5cEmT90q65K5kEWQGCr/hJ9LCeMN1q3wfiBw1",
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
        svg = "eJxNjEEKgzAURK8y/L1tJiWhhSQ36AW6k1RQkFKkC3P7+o2K/MUw899MyMOUxw65RKEVTIsYQZ5Xm8K1/lP4tr8e7yhP3sFb6+FhlmPj4XByjetpX9rURgrj8OlQGOUhmKsUW53KxVBZpTZWYzrNV57uKNSVnf4DSfYukg==",
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
        svg = "eJyFjl0KgCAQhK+yzAXCfhFcL1OSvoqQ3r4UjKyHHped75tR3qyBrHG7DQwJSowB5CNDgGI5DrcFy1igVZfjWhUotc9bMhVOjNXSUt+8fNZl7BKL/heb6rwcn99lJ0Q9PT0=",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[cfg(feature = "layout_grid")]
    #[strum(props(
        svg = "eJxtzkEKgDAMBMCvhP2AiIKXtp/RYnotAdvfS4IK0lxCDjubhJp3odoiZtBVDuGIDcS5nCy29ogF1HSmMGk8BUNqVrx2EF+Zq9xTT1W3yJ+5eXvKVzcwRD09",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[cfg(feature = "layout_list")]
    #[strum(props(
        svg = "eJx1i1EKgCAQBa+y7AXKMiJwu0GHiJTWv5CF7PahUBjk3+PNjAluE7gIe4QQCRXC6a0w4YjAzu8secZkzKZJ+myeSOmfKqtF+1bHKgyWcFEaNGeQri+YakANNdK1BbkBa1A7bg==",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[cfg(feature = "layout_panel_left")]
    #[strum(props(
        svg = "eJxtzEsKgDAMBNCrhLmAFIW6aHsZLabbEtDe3v5EwW4mi8wbE/0mdIZd2EKD2IeDxUKtoHjlC8o5g1JJZ6bSd6aq/v9jXZFahupTSq30DnTVhh92AwYhLiA=",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[cfg(feature = "layout_panel_top")]
    #[strum(props(
        svg = "eJxljEEKgDAMBL8S9gNSFOqh6We0mF5LwPp7G6Qi9hSyzEwoaVM6867CcCvoYsygUtsHkpQPUYYHVdtjmIyP4Wv5ket+i7nlZ43phzLTzlvt2g32NC4g",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[cfg(feature = "layout_template")]
    #[strum(props(
        svg = "eJxdjV0KgCAQhK+yzAVC+gf1MiWtr7JQ3T41qexlF4bvm9HBLUKnQQs68g3xKRA7v7EYjKDdr8IxnGB1k3irs/VBinQ3FH5G6lXdz3pRNTxsXw3W2gXdIi4i",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[cfg(feature = "layout")]
    #[strum(props(
        svg = "eJw1i8EJwCAMAFcJWaDEflpQl2mlCqUPEarbm2h8HRx3NoerQAzpicUhHQjV4Y6QGQbhT3eJ07fpm3hvN/m8fdMXoBp2xAU5PBlmoJIMXEqzSi1IBy0HVtkBaT8mlQ==",
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
        svg = "eJx9j0kKgDAUQ6/y+ftWO4AW2t5FqqCgIMWF3t5OoIh0kyzyCIl2i3frBO40yDiCu7L7YC1a3eTc6n04ZhgNbpIqAUW4TBK5mL8oJmkvQFHWJYJUsay1ulSUMPKDlQ9hs8Tvk+fCDTXlPRA=",
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
        svg = "eJxNyzEKgDAQBMCvLPcBvcNwColgZ+MjBAUFEQsL/b2JRiNbLAuzduv3CYOjrgJrq42BQe7DUCjVNgugth9jA52kT6xAmdgyryNOccRCON5mRyXh5Ht6G9TPSjTymPiURC8/Qijz",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[cfg(feature = "link_2")]
    #[strum(props(
        svg = "eJxNyrEKgDAMBNBfObKLplCj0BbcXPyIgkIFEQeH+vemCiIHOY48d8QzYfY09WAZZbCwaDQMgSRDwdWFBPdBtuURX8gavU2qfnRb9wWZPXWES4sNIRvtVrd5ttqiwg38yx+z",
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
        svg = "eJxNjFEKwyAQRK8y7L80LsG0oN6ghxAjVehHEUmTnr5uAiHsx+zOvhn7CS1jdvTUBvdgYDD00V3NMmU1LmoKDN7dQbGS/bzAJ3NJKvMjb2/S7G1NseFb5pYdjYScyis3R5oJm6MHYXXEggvobSw1vhOqmIjrnombSEeOp/8DS54t3g==",
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
        svg = "eJxtjMEKgzAQRH9lyX3bzDYNOUTPvfQjSltIoYigiP69GxVF8bA8mHmz8f+rvjSgMN7QIBN6hUCpKawp4zVLZZzUJVylPIXMW+VB3n+apbCMwibXrzbRpzBPRz6hc7nJ2a6BTXJSeEJ4uLdlkLDo3RoG43JnYaz+CPfBO5U=",
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
        svg = "eJx1ykEKgCAUhOGrDG8fYZa5UG/QISKl5yIIEarbp6taFAOz+X6TwpJxRJ/ZkiJclgZCOi0JQnlJ4BBXzlWdaWvuzD5nhre0SYgRXVnf9JUrPDwJCcX6G0T3K/olN0KYLPw=",
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
        svg = "eJyNj0EOgCAMBL9C+gC1JTEcgN94MDGe8fdiW0qIHjxtyO4O23js5+YKJSAEd2GCtQqxlPoKkOP8ZHIckuJxAUkaVccsm0HNYB1FvLl+WpTsbYMs+uD2T1vDED/I2Lb3+27b50Jz",
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
        svg = "eJx1j0EKgCAQRa8SHsCaSTTBvE2LIFrr7VNHTTJXHz7vOV9znfcxedyZZJMLARgSKH1IZNbMEbKmotgwxYksbF84lILrlSVNcSVJpM6VrpdAchRkgeaLyveodbXtp5VJ+VAZhiOUVicFhzC8K36+kd5Ky/obmYDWGv7iAaNmYOM=",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[cfg(feature = "locate_fixed")]
    #[strum(props(
        svg = "eJxtzDEKwCAMBdCrSC5QtJQiqJcJDoJ0cNLbN5omQ3H6JHk/oZYnm24jODCDws50nJ3yghSOiVJgSju3s3P2PzzWg9WxX0eo21D96xVJVzGWhjWbFuEGg50RDkF8ViaA+LlhLxpqQhs=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[cfg(feature = "locate_off")]
    #[strum(props(
        svg = "eJxlkN0KwjAMhV8l9L5xSV1/YNuN1z6ETEFBxAsvtrc32do6GYEcmn45gdM9H68bTNyb1sBMvSEW5VUnebMZuoNCQ7eg9U+UM0Op7u7h7JtXiu0Oncrt7Ffgeb1T4fflc4drb84BiUDbqcXoIKJL0AI1SCo8NuAwBmnkICxFmECnnqxuHDEmy2Kg5mq7MaeIgYAcJj8iJYveISdL0lZNHhqrB6wesGEp9EGmhE5mCyPsz7wmwpv4SjR/SX8BxDVelg==",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[cfg(feature = "locate")]
    #[strum(props(
        svg = "eJxtjksKwCAMRK8ScoFiQEpBvUxwIUgXruLt6y8g0tUwzJtJXE5vBDEeCaGSR9NUmtpmzbDBXR0KbqDKrHCw1LX754Blg7S07tifXdpQrdZjl1PhHKF4vBG4TohFP51x+ADWMzcP",
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
        svg = "eJxNzMEJgDAMBdBVPrmLJipeajdwCEGhgtiCRez2pqUHyeEn4SUmrNFhm2nhEb0bVoGg02JNefi3aLRzzUDWtPnImuDPdB7XjuCPK94zsaoJ+ogF2k+FVmRNoUmUCeHV7AmJ65hzzL7YD0wcJss=",
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
        svg = "eJxFjF0KgCAQhK+y7Hs/YxI+qDfoEGJBQUFID3X71IVimF2GbxgbtxT3heLtGGCKj/zk2LC3nWBvz3CtNDs+FEih0e1QXBqF/HwCCAiKFPVVulpLaky+Y5Yw5NI38QLGhiFR",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = ""
    ))]
    Lollipop,
    #[cfg(feature = "luggage")]
    #[strum(props(
        svg = "eJx9jsEKwjAQRH9l2Htxdwglh6R/4LX3EoUIHqSI6N+bINi0lLKHZXZmHhse0zPjEuXcg5p1IggtYx07jn7RKDob2wP4slUDzCpDOFXoEP5oD/Oj26DcluR2mqb1q8ZJtzndr0ifKFTBXJYgvaNYX0M/+yDmm9QXNEA+0Q==",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[cfg(feature = "m_square")]
    #[strum(props(
        svg = "eJwli0sKgDAMRK8SshepFsmizQ3cuhdbTMGFlODn9jbIwDDw5oWaN4X6RBwQ7pJUIjpCkFx20X83OCK81hx6EzicqwqkiDOBmxY6PLR0/iK7GOQPjNUY9g==",
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
        svg = "eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb++64j9GSELIe7+kuYX7gGOrehGwkN27IBCYFFq0bMvfGTKxRAMmphppIYMnq7pmk0Bd88ZdZ5zXFdUelnyY1SWWkkCshYyD2c2atbXnCsKhQJGky12HPOVwkzl8bfE8xssJ8dEqrhTiM/exVUUS5fX/YyKayab8YF6EkD8y",
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
        svg = "eJwdi0EKgCAURK8y/H2mUlmgnqBLSEq6CEKE6vapDAwD857O4Sj4DE2E/BqShCf5EuvihBjSGYshsRD6afXYBKtvVyK8oUtKqGFlm8LMlBNsm9CL14hBMr6A75VpapPsD+b4HQM=",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[cfg(feature = "mailbox")]
    #[strum(props(
        svg = "eJxdjsEKgzAQRH9lyLmm7pJoBOPFi5dePfQmbaGCaKAi+vdd05aWEpjJwmPflqGb77h6dWIG5R2DkcqjRH6N+Z0Tbgtta0YOA4tMW9iG3IX1jhiQdpJmcWdVlcd9b1WGadiGfrwhTP04P7wieyhA7hVEkXwzQn9uibvrQixEopMQc/t/3pJ+RVGykleZwspe5QqbFKXSFFvQaHkCW4464A==",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[cfg(feature = "mails")]
    #[strum(props(
        svg = "eJxFy0sKwzAMBNCrDNpbjeTUdsHOCdpDFCfUWRRKMP3cvnEWCVoIZubFZcoV30SOUKb5UWoisYRfop6wrIUSPvNYy5o7GuKpgSG+7rVgTPRUhTeeBZZ9yIbPnq0RVru9gO7q4Jtr4nA3RXiL5A7CwhdouyL9vvwDMlYnRQ==",
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
        svg = "eJwli0EKgDAQA78S9l7crSB7aPsDP+CtrIKCB6ke9Pe2SiAhJBOOfK2YI42eIWyMwSnEf3Y6dbU68Vmh4CqBDOCJUugamYJtxfYFdkcST7CnJhNKpL6d/jm9ABkZuQ==",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[cfg(feature = "map")]
    #[strum(props(
        svg = "eJwtjEsKwCAMRK8ScoGSiqKg3qYUQVRoF/X2TaKbzIc3iaPXefcGo5f2PgkNOAhggCybk9jxIS+ZTRArFeZ47GmOtbQLJvEY4TsTBhZSmZzICyzMJgUhi7pwC9XI9Xqs7A/pSCbX",
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
        svg = "eJxlzE0KgCAQBeCrDHOBGn8WgnqbFoKoUAu9fVpZhpsZHnzv6RR98S5skKILx26QJHBgdB+FVi+dWD1h9UoOJP/4gpkMMkLIrC4LhFIzr6/FtfnBlm4HI57qZ08skTE/",
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
        svg = "eJxtjlEKwkAMRK8y7P9GE9vdFNqC//UQZRVWqCBFRG9vasEWlUACmZfM1Nf+lnFs3CGSMLiEUAiIxEUvEGytGMQ7LyRdQYXpul+UAMksK5QpkA7WDfz6Md1Kx4FiZUaurTeTeVt/IrCdC0qyZuQvcOHdDKj6ivTPB0XMq306j2k4IT0bx9EhPWyKw9i4t/0srwMIWO9esqcl3wsvmkZh",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[cfg(feature = "megaphone_off")]
    #[strum(props(
        svg = "eJxtzEEKwkAMheGrPGY/sS+ZjhWmcwO37gsKCoO4ELG3N1XRTTf5IXxJuU33M45j2O9EM97DQD6sMQkTzEeoZbO4Wn5aCfZi6ZBb3IoRKp2tOFIymGWYzP928LPYyxB9/dftcj1h5hg04PmNerzzp04XVF/hfisk",
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
        svg = "eJxtzTEKwCAMBdCrhBygrYKlAetlgoMgHZz09o0m3Tp9krxPIpfGNQP3G51H4KHZJA5Mcdd7irU8GYaTdUDoXvJEGN5m2V9TT2V2GtoOp4pwlUkt/VgXFu72wrR2P/0CrLEsww==",
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
        svg = "eJx1y7EJgDAQRuFVjn8BjYKmyGUDhxATvHQSDozby2Gjhe37eKHmTak2xgA6S1JhOA+6GCNIctlFn9KsxNDZEMOxqlBiLDN5cb2BpQ+44Vemt9y/TiVS",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[cfg(feature = "menu")]
    #[strum(props(
        svg = "eJxlzUEKACEIBdCriBeYUYahhXWbFkG0rtuXRkG0+iiP/yWnEqGyR34RKnn8ENoI4pFsGeRRFcSsPv+D6jULbklu0zVi7W7bDl/NIKQ=",
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
        svg = "eJxNjLEKg0AQRH9l2P6IOzmRwJ11mrTphQQURCws1K93V1Rki9l9vJ00NlOLX5YPFVo2BFHYaLDtXfUhIn5vGAxsNd4BuEqdHl5Up74b/pg1y0swM4uWgsVOLSy5p7luna5Dyv7j6XJ1uM/L3QC1Byez",
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
        svg = "eJwli0EKgCAQRa8yzD5rrEzB8QRdQqagoCCkRd0+tcV/f/H/81e8N1gYT9JgG6e6ATJM1MpqqOiAcnrlbMVMBkhj8G1Rg5c9ybGCPIw0IcjLmCsxjuXzr+ED8pMZlw==",
        categories = "devices,multimedia",
        tags = "lyrics,voice,listen,sound,music,radio,podcast,karaoke,singing,microphone",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Mic2,
    #[cfg(feature = "mic_off")]
    #[strum(props(
        svg = "eJxtj8EKwjAMhl/lp/fVJbG2hW7gA3j1XlBQEPEgZXt727rRHUYgIeT7IV94Pd93TDwoZoWJ8lSYl3Wu6xgOBRrDJ34fuA3qQk47DxLNcraaGLX1tSgfOHU1VgKbmAH1iaOFXVmG2eHIwGs5Xk0UyB/tjD65jrTIDu/hkzQWpnzDuTW2WhYd8qra0mJLTXf1/AH1m0Kz",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[cfg(feature = "mic")]
    #[strum(props(
        svg = "eJxNjDEKhTAQRK8ybB9+dvM1CEluYGthF1BQELGQoLc3sYlsMTvMm3FHPBdMnnoWSDQw0OVU/pKtHi300HxzZUYK7lf6wdWVDqyTRAv7gqz4D52UVHZb9xm3eBIh3OyJO8JVNPtLXs1wwcIDk98l4Q==",
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
        svg = "eJxtzFEKgCAMgOGrjF2gjEoD9QYdIlLStxCxun1qoAW9jf37xp1ePbhTYIdgtN2MF0gGhGdzWOVNnFqES2CPkjcJSF4YKUesPqDZjxmxivbFG1ACZ8KABZpCWr3CCGQK3U+I4lNu/9E0fQ==",
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
        svg = "eJxtTssKg0AM/JWwd1OTfYMK/QCvvS+0YEFKD0Xq3zfrytqDLMlkJslsunf6THDv1RiAp6CG7pKVoat6BF4ItdEjWSkZfYjJgIF2e+g8AyNTnNFZhzGY2qXazRakafQYQGJukDggRb4eTh6oFR++cZtkaVelmtw/b3hp9Mmdsm6TQ+NhS+V/KzlT3nPx0Gi0zXHYzM/XA1bqFSv4FlhZINOCMpqHhh+UmUTC",
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
        svg = "eJxly0sKgDAMBNCrhFzAJHbZ9jYuCqUt6MLe3qT4QYWQgcyLbzX3nMoCraayrQEdsA6dWwijny4U/Y8LPVi3e/NBOwdkQugScEbYNYQ17Tz8x1ppVsuB7dfwfNsDwewx7Q==",
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
        svg = "eJwlybsJQCEMQNFVQhZ4TyuLmA0cQqKgYCFiodv7qw7cS5KblAjNovoRZGz1dl6ZvveZqu8JgkVnQOlkzjqJF12HEmU=",
        categories = "maths,shapes",
        tags = "subtract,remove,decrease,reduce,calculate,line,operator,code,coding,minimum,downgrade,-",
        contributors = "colebemis,ericfennis"
    ))]
    MinusCircle,
    #[cfg(feature = "minus_square")]
    #[strum(props(
        svg = "eJwli8sJgDAMhlcJ/wLSeukhyQYOIbaY3qQErNtL6PV78GiX0yfYQdb6bS5IBTSmIIPeXt0WmdEobzEoP6cbVcFRKGUrIQLpD511Fq0=",
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
        svg = "eJxNjEsKhTAUQ7cSMn88by2o0HYHLkJUvA4EkeJn91ZBkQxCOOS4uYmKznOqIBlMiv1ZBve/QHBL30ZoPw4aPcUS29hF9TQZsaciDs+cWO6RbtchuMdai4EU60f4khJGtHzBCSjmJgc=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[cfg(feature = "monitor_dot")]
    #[strum(props(
        svg = "eJxtjLsKgDAQBH9luV70zjck1ja29hKFCBYiIvr3JgiaQrbZYWCUmTezTDCXpoJgTk1cEzZNKTUqfmyj1mG3GDV1ImA50kEgSNw4cq/NQo6kzz+GY1v7lm8EJXal8sh+TAVhW73iBm83Kh4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[cfg(feature = "monitor_down")]
    #[strum(props(
        svg = "eJxljT0KhEAUg68S0sv6Zly0mPEGttsvKj4LQWTw5/aOglpIIEU+krjxHxSNZyUGYn85S/c5stJdZJAvJE0sbBL18KmtA5a+CeppUkLbvtPgKRmxeVpiWiMhTo+1o/DMnof5nL0PqwJGtLjBDpxvLZE=",
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
        svg = "eJxtjT0OQEAUhK8ymQvw1iYUu26g1Qvi6UQ2fm7PKhDRTebLN+OmJig6z0pSSFbnLF0Su9I9xH7I3LcBu2dGzJunIdaxC3qmlNB+HDR4iiUueGpReA8aSL7Yn6sCRrS4wQHLQiyL",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,suspend,hibernate,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPause,
    #[cfg(feature = "monitor_play")]
    #[strum(props(
        svg = "eJxFjDsKhTAURLcyTP943qhokWQHbsBOVLwWgkjws3ujoBbDFIdz7NwERec4SYICOdJfXE1v/xfxdunbgG3sgjqahNjjEYdjSmg/DhocJSOWG0TtErx9spUYSLFmX/AlJYxo+YITTFkmYw==",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,running,start,boot,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorPlay,
    #[cfg(feature = "monitor_smartphone")]
    #[strum(props(
        svg = "eJxtTLsOgzAM/JWT99DEtCFISeYuXbujgmq2CkW0/XucCQbk4XzP+BmKYEz0cAHh6QcGw9YzbPh+PXLw2u1ckSVQjpc6keM+ZOH61bRN79E27nYS6TQhB2OZXgX/RI4JPwVP+M5jkUT6LaqoLtP8lqKmrb3ayBsNDjBk",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[cfg(feature = "monitor_speaker")]
    #[strum(props(
        svg = "eJxNjU0KAyEMha8Ssu9URZ0pqOvZ9BCDI43QRRHpz+2bUKhDFi95L8kXHlsn2CNe3eTAqHXBFM5ipvCP9AwXmpQeUSu5wzuiNgiNleVV907sKIRPRItApd6os+PlTi4OLxfwq90MGFBSJ+6efsyshuzg5dryvUAW5IyQmaAdo1lk6RenL1PvNsw=",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[cfg(feature = "monitor_stop")]
    #[strum(props(
        svg = "eJxNi1sKgCAQRbcyzAZKkzRQd9AiIqXxL0R67D60h/1d7jlHRz8nOAwOCOTDQslgj7AHl6is06BEq5vsWf3avNpMFKlDiDd4Wt7WbJ0SgTM4Mg5MbiKT/P2IAs5IfeACFForbg==",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[cfg(feature = "monitor_up")]
    #[strum(props(
        svg = "eJxtzc0KgzAQBOBXGeZe6iaCFhLfoNfei4rrQRAJ/ry9UVBBZI/fzozr/0FReXYfSAL7sojHwr03KNzBXzEQ+8suGeoyYPY0xOJpCa3bRoOnpMTUVkGjJcSw/8TYFrgVZmP6MJXDiOYnrEnjLUo=",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[cfg(feature = "monitor_x")]
    #[strum(props(
        svg = "eJxlzV0KgzAQBOCrDPNe200jtZB4Aw9Rqrg+CCLBn9sbRRSRfVn4dnZc9wuK0rMVm6QQk6SPOMzdc5XcHf7dGRfuq39AP3kaYmzKoHF7EbPnm9CqqTV4iiW2kxhbA+fXQgzkM9h7X5HBiGYHLDnzLo4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[cfg(feature = "monitor")]
    #[strum(props(
        svg = "eJw9zFEKgCAQBNCrLHOBWovqQ7tMSQrRhwjp7Vu1+hp2eLM62C1SMlCgbDCAQjuc9YeLBjyCbr9HJ22PVXdlsOrTX5aykpJBSZInSTZY5A/XWmxRr61GNVPy2xbM848f+hMlqw==",
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
        svg = "eJx1zLEJACAMBMBVQhYQBQshukywEKxS6fZKFBTB6nn+eOIiXDNwj2gdgsxA4KYtkVlzoocpCJv/2XnzlxqNICD4",
        categories = "layout,development",
        tags = "ellipsis,menu,options,operator,code,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[cfg(feature = "more_vertical")]
    #[strum(props(
        svg = "eJyNyrEJACAMBdFVJAtIBAshZpmPhWCVSrdXSKmF1RX3BN0wWsCsxIkCltdOSCX6Vnmw/KW4XGwDkmog+A==",
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
        svg = "eJwdy8sNgCAQRdFWXl4DKvGzYejAIowQh50hEz/dC27PzfUl7YZX6IhHOBGa8qFWoSdKpYW4czQVDiOD79oQ/LmZIgrXwWG+/tAofJFcFrA=",
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
        svg = "eJxlzEEKgDAMBMCvLPmAROlBiP2Nh0JpC3pofm8b60F6yYZlWCk5agzpRMkh3ddBDsxweIO8LJ/wMlnewRv6Hc/fm63cNgm6dk6oI9Xqxo0+yBommA==",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[cfg(feature = "move_diagonal")]
    #[strum(props(
        svg = "eJxlykEKgCAQheGrDHOBeIoLYfI2LQRRoRZ6+yalINy8xw+f1JJ6ivmgWmK+zp1hyRH8HICDbK8Jsmp8Vsf+9ZANqjxT13d6ZmYzTyof9Aa/syaY",
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
        svg = "eJxlzEEKwCAMBMCvhHygJAfJIfqbHgRRoT3o75tYPAkLy8Kw2luZJdcbesv1fSKSgAAzkEWAAia9Nkp68ODacTjscoMiMsJke/amv4dtZvfLfr0zJoY=",
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
        svg = "eJxtykEKwCAMBMCvhHygZA/Sg/qbHgRRoT3o75uk9FAoLOwuTBy9rlraQaOXdp2JdxINCCAJujnH7UU5/vDg2nD4WndTEguYJp5e+q30Aubd3rzkJoY=",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[cfg(feature = "move")]
    #[strum(props(
        svg = "eJyFzj0KwCAMBeCrBC9QEnAIWG/TQRAV2kFv38Ra6M/gFB58SZ4rObYY0gYlh3Tsq7HAQIAEFtAa75ZbePezrIiUW5hZIchdk3xAnnHWHr2Irn2ud1pxNWSgyUCZlSRqpp7FP2y7bB22DVvf9gTKAk0L",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[cfg(feature = "music_2")]
    #[strum(props(
        svg = "eJwBRAC7/zxjaXJjbGUgY3k9IjE4IiBjeD0iOCIgcj0iNCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+n54TIw==",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[cfg(feature = "music_3")]
    #[strum(props(
        svg = "eJwBQQC+/zxjaXJjbGUgY3g9IjEyIiBjeT0iMTgiIHI9IjQiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+Y9YSWw==",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[cfg(feature = "music_4")]
    #[strum(props(
        svg = "eJxlyz0KgDAMQOGrhOwiUZQWmt7A1V2ioFBBiki9va0/dHB+3zPbsM8wMnYaSPWNo6qoDqrRmjIlaz6waogk1pxk8eImkJORFIIExhbBM977U7MKr0r6zy5XGyaD",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[cfg(feature = "music")]
    #[strum(props(
        svg = "eJxFyzEKgDAQBdGrLL8XWcUQIZsb2NrLKiikkCCit9dEIfW8cft0rDQLhp7Yjl3gpmpObuFdnZJ3ukUNC+klMCC9BWxBUZDNV4tK2fw5Py8u7AG9Xh8g",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[cfg(feature = "navigation_2_off")]
    #[strum(props(
        svg = "eJxVy00KgCAUBOCrDO413tPQwLxBhwgKCiRatNDb5w8EbWYY+Mbf63Ngm8UyKU1oMYIpWmlgYSQpslKXEMEPFQf/XcioUcMp50AMjg3jj+N57Ug0CxZIXKp07jP3WWhF4QUW4SGg",
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
        svg = "eJxFi1sKgCAQRbcy+N/E+EgFdQctIigokOijD919o0QxcM9wOTdcy73DGsXsUCvooYAoO5B8nGjsYLgWKYxNTuGbkEXlWUarQLKeB99ejRP9dj7ODSpFIQUUyWDWl6XXrDYpPS0FIcE=",
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
        svg = "eJxljc0Kg0AMhF8l5C7trFRa2PUNeu1dqjTeiiz+vL2Jf7hIDhkm801813wjTYFRMEnT/iQGVjmuTmebaWjrKOaX/mZA6RfssPcCTbukZuVTauuclqzqV0JcX/2rKFQHfj8IRZ/lFQh019GdQeDOBqHPDTXohMIR3Od5XGZNdEMD",
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
        svg = "eJxtUctOAzEM/JVo7zGxY8ex1PYL4Mq9WpBAKogDQvTvGe9KFYcqWj9nxxPn8HX+fisvx+WJpeizLKfDQ5ZOh1vDCrcfPTu1pmW3bTtG4l4G+fSVlIW4Kc0mJCGFkQlZyCOIRS40hGualXqlTiMq6Zw7riYwJ3DZzM6vxGJVKHrckcWRujp17fe6UCBrZbQxyLwjjGawrSUnPpR1oChZ1FTSrWerbBWekKuKUEfmE39sDZnkjGAEBUC9cMk232jcHDStg6cX4tBK7oNmcpkl2gMA4Q8GiykWGYEVzunYiTkAguEzEmqW07HiHNKxDB6Ixuy5OWDh0ZfwHN83rM1tPNRi03lRxZk1ZeDptgsBOxLFBn3/Hv3y/vlarnxcZCm/u7sKXKa7BzRBpz+yiXmT",
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
        svg = "eJxtjEEKwzAMBL8idI+adexSgZ0f9BHBLbTQQwk9NL9PbOVgQpBgQZrZmN9z/jxpTjww5X9iuC2XmmO82HuMO1YBNSBwsc6oerYStdLQUt/p96JH4jt68eQgOqGnbW1U1KNDEI9r0QreSoMEcoLW6VRuHnRwVvFYOP8=",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[cfg(feature = "outdent")]
    #[strum(props(
        svg = "eJxdzEEKgCAQBdCrfOYCMQbqQr1NC0FUqEXevjSVavWZ4f1vcgol+LghJx+P3ZKCxgoWUGBJzixDONNcEZZYEM6aTCjc7zsF18IL9+cHS2ob8k8Hqann3rOvJ74A8o4urQ==",
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
        svg = "eJxtjc0KwkAMhF9l2PuuTbp1LWwLPoBX7wUFC/0Di7Rvb1Kr7EESCJl8M4lTMz9wq0xPR0izlLfe1PGglzp+7xcmUHY9NUpkWpYsuZB3NlifqDJFgj//JOQ4vRIjJEmNAiWi+lgS6c/zPrgC3nFAicJRkRBjt3btcMc0tsP8rEzuuEQAsTZnLhDChu9gHTd84coQGyz0meu+r7Izq2GD37uNRUc=",
        categories = "development",
        tags = "confirm,verified,done,todo,tick,complete,task",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageCheck,
    #[cfg(feature = "package_minus")]
    #[strum(props(
        svg = "eJxljUEKgzAQRa/yyT6pM0ajEIUeoNvuhRYUrAqVorfvJBYJlAQ+eXl/xi/d2uPRqBuVoLIvVesvgbX+/GECZfeqYzCycDRpMi4ftdM2oZKCYK8nQo7qkxQhk0JRpASGHstEsv/LX84UsIYdahSGisSYx30cpieWeZjWd6NywzUciMPlzDiCi/pPbH3UN2oUscIuySH5eG9HSiHKX7sCRGw=",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[cfg(feature = "package_open")]
    #[strum(props(
        svg = "eJxtkE9LBDEMxb9K6L2xSaZ/BmYWvK9XD94GFBRG8SCy++196bgrjEshpX15yS+ZPpevV3qew4MmHoUatwEhF1JWW4RHox6SnyjchNLRWGhgsUVZlHrYdE6ZjMu4CuxKBc4FCZvqhQa4VQhylhvuMcJdn8JhunOww3TBezeIGx28lmPhIjs8lO14K2pk/941EAK20x0bV/gGrvlKJ/GXzm7Cwfsfbn37eKGzzEE10FnnIBbohLfgfdJ+I9nT/ibBokmM87exVXRJvmqEC4T0MWyNBayp7WdERm2YMZbo8v2uAGE/hVt99BZX0h9JLmZn",
        categories = "development",
        tags = "box,container,unpack,open",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[cfg(feature = "package_plus")]
    #[strum(props(
        svg = "eJxtjUEKgzAQRa/yyT6pM4mmQhR6gG67F1pQsCpUpN6+Ey1toCWBD2/enwlTM7e4VupMBahoC1WHQ2R1+E5KkF3+TZhA2eXYMBhZfJo0GW977bVLqKQguNMHweK4JEXIplgUKYGxx7KR3O/xuzc5nGGPErmhPDHGfu274YZp7Ib5USlruIQHcfycGU/wm/4W67DpK1WKWeEpSTF5z3VPKWzyC0CoS9U=",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[cfg(feature = "package_search")]
    #[strum(props(
        svg = "eJxNjlEKgzAQRK8y5N/UXQ1RUKEH6G//xQoVrIqVorfvJlobEjJkeLM7xVQvTzxKdWMCxfesZjBidyKKSNukj2yUBq6oWEivp4UE2ScIQia5oECB6XIsEylVVXFxa6vit/xltUGq2SKH0WQCYuy3vhtaTGM3LO9SJZpzWBC7y7G2BOvxA6wKj29cKmKFjUrFouvxX8mrBHa46eamb9FsYhttFGbhnTarOJn2VXboX/cme6UrWfeyNMnPwl/UGlJD",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[cfg(feature = "package_x")]
    #[strum(props(
        svg = "eJxNjl0KwjAQhK8y5D2xu02IhaTgAXz1vaBgoX9gkfb2bqqtIYFlh29mJ0zN/MQ9qisTqLidGwajSE+TJuPLTnttM1WmSLCXQ0KJ8zszQpKSUaBMTD6WRLKqDqd0tg778d4bB2vYo4Iz5DJi7NauHR6YxnaYX1GVhit4EKfPhfEEv+E/sA4bvnBUxArrPikqlrnQtovhCx8NSDJLOLheO2nr9L/EB4wYRmw=",
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
        svg = "eJx9kMFqwzAMhl9F+B7Nki3FhiSXnvcQxRt0sMPodtj29JXsFFoohcT6Yn35Y2VpH+f2+Q7tdw2UUAK0vzWow3kNVrblZSjbsqtjv3sUO/m780PXpfnqlGepLtx8dsTzvfp1/DnB2xpeiYEPZoNVGJX4OxtQHFfDygpGqLlMOGfd0dZSIE6Y0zwhWa8kGU+ExIZc7S7V96yrwoN69+ghPcmyLd1Re6iWE2Gt2hJGIesJishkE8S0s6/5wKap2KmjTWG/LSv14wP/+6w+43YBHWJe7g==",
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
        svg = "eJwli9EKgCAMRX9l7D1iiuCD+i+RkkJFiJD+fZuxh7vLOdfVtDcYHjVC5VAIb4kteySLkFM5cvv/2iftoga3yi64s9wJOs354CDDhmKRuKvZ2RUruGdrGaLHiwzYRYNe+AQLCB8LOSRi",
        categories = "layout,arrows",
        tags = "drawer,dock,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[cfg(feature = "panel_bottom_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KQN1eoxcFvYW8/7+veRLYAlqEtSThgNQjcC4zy33vF9YzYzD6RgvRL6MwpIADtUAdk4r+nuJUzIfYP3DvrQO7QCx4",
        categories = "layout",
        tags = "drawer,dock,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomInactive,
    #[cfg(feature = "panel_bottom_open")]
    #[strum(props(
        svg = "eJwti0EKhTAMBa/yyP7zTYug0PYuosUWVEQK1tubqGQxgZnnjjgWXJ4sIcU8p+KJO8KZp5Le9xBrBPVB1TS4v+6CW/IWUY0YJlwsg1ZoXlb+Wq2C24eSMHlae3AD+7OQU60i3BCXJEQ=",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[cfg(feature = "panel_bottom")]
    #[strum(props(
        svg = "eJwdjEEKwCAMBL8S8oESS6EH9TOtVKH0IIGa37vmNGR2N7GXS8kS70wdCEzDj1raUzWxnAiGB3+7tbrJcVu7HN/2FbIAeWAIBgHFP5i4Rne18gQM1BxC",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[cfg(feature = "panel_left_close")]
    #[strum(props(
        svg = "eJxFi00KgCAQha8yzF7CpChQb9AhIqVxEYQMlbdvLKjFzPv5eDbHhYFiWokd6gHhTIHptZdDg1CenyW0IqWKt03debvPTBAcTiOYQyYCavWDTfegO2WUAbmP3+EUIMo=",
        categories = "layout,arrows",
        tags = "primary,drawer,hide",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[cfg(feature = "panel_left_inactive")]
    #[strum(props(
        svg = "eJx1zEEKgCAQheGrDHOBUFskqDfoEJHSuAsZrG5fU5uC3P4f77mSZoYtRyaPakDYPRoESnkhfspxl3KBxuA6GQS3TkwQPY4WVF+VgKQv2Kp/wTS6fR2dL6kr/A==",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftInactive,
    #[cfg(feature = "panel_left_open")]
    #[strum(props(
        svg = "eJxFi0EKgCAURK8y/H1E2aJAvUGHiJS+iyDkU3n71KAWMwzvMTr6VRBvQz0htyKk2jFVdAUnbKgbCezDxlK31W35WX0swnCG5gnqfEVBv9i7AdlBNTmffgCgDiCB",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[cfg(feature = "panel_left")]
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S9gOSelFo+hkttiAeSsHm9zbxsssuM7Hlo1MbggAquV6lC3gDqWAFDc+3nr38d1MjU1zMS/GuT6bBgn2ywUvZHZ0rsJHGpA/nRxvo",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[cfg(feature = "panel_right_close")]
    #[strum(props(
        svg = "eJwli9EKgCAUQ39l3PcIlaBA/ZcoSaEiREj/vnvtYYxtZzaHraA5MoRcHWlCDOmIxZGaCbUPb9pL/IvchPF2lJ+3Z7oDmupU1YxMhMauFWfVM7NCefusJWJ3dM1YYGAGlqzS+w/hHiQb",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[cfg(feature = "panel_right_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtAoW2m7gEGKL6U1KqLq9Vi8iekr4j/9tjpPA7lAjcEwzi0PqEdYUhO93uzCfR6G3TS14u4zCEBwO1AG1harU7CWmqG/Rf2AeWwfGuyyw",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightInactive,
    #[cfg(feature = "panel_right_open")]
    #[strum(props(
        svg = "eJwli1EKgCAMQK8y9h81RehDvUukpFARIqS3b7OP7THemy1xr5BiPlJ1SCtCaQ4Vow/woRH62G8ONY3I21n+vD3zHaHT0I1Bhql+dqYiaaXy9tlqguDwogXITHrSwCNejP8AMyMkiw==",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[cfg(feature = "panel_right")]
    #[strum(props(
        svg = "eJwti8EJgDAMAFcJWUBSEXy0XUaLDYiPErDZ3sT2dY+7i60cArXwVSUh7QiacEVoPWEw6I+XT6lDd9c5Lv7lePNTQINFZIos2YxhUGm2XuUPIUscQg==",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[cfg(feature = "panel_top_close")]
    #[strum(props(
        svg = "eJwli1EKgCAQRK8y7H/EKkSBepcoSaEiQshu324yHwNv3rg7LgUp5i0VTzwSqidLuKUM4clrSY2/jb/Kg+v1F9yez4hqhLEY7GmSMn9V1oOY6gR3zSVh9XRM4AG2s5DorEP4ANG9I/A=",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[cfg(feature = "panel_top_inactive")]
    #[strum(props(
        svg = "eJx1zMEJgDAMheFVQhaQth4sNN3AIcQW05uUgLq9jV4U9JLD/5EXap4FtpKECc2AsBM6hOO6nMvCcvfawGIMnT7EsE7CkAhH04Nno6DpCb6B/QD30/1r6AQf2SvE",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopInactive,
    #[cfg(feature = "panel_top_open")]
    #[strum(props(
        svg = "eJwljNEKgCAMRX9l7D1iWlCg/kukpFARIqR/32YMBjv33Jkc9gK5WlQIvDXCm3yJFmlBaB3EkI5YfpKbmM6M0nPmTHeAqpgR22Rx5S/UW03JxaY4zjxbieAtXjQDTYMGPfBILon7AOe2JDU=",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[cfg(feature = "panel_top")]
    #[strum(props(
        svg = "eJwlTEkKgDAM/ErIByT1otD2M1psQDyEgPH3Ju1lhlmztEOhN766FqQN4Su4IthAcZEQXj61z1QsnJqX2NV889PAknvkSyq4O6VBRnHhzejUH+eXG+g=",
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
        svg = "eJxtjEsKgDAQQ68Sulc7HT8I1bUbD1GqoKAgRYre3lYX3QgzhOSFaLs6u82wdydICbggUsBer+118fFeH+ZcMHVir1CBynCRxjSxkRjEhsGQoPAyq4fWq59mC2p8pnJOMw9LKCUj",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    ParkingCircleOff,
    #[cfg(feature = "parking_circle")]
    #[strum(props(
        svg = "eJwlijEKgDAQBL+y3AdMjBgCydU2tvbhFCJYSLDQ33tRlmVZZqLsVY4NcieyPaHqGII83+XY/Zzjma+CNdEcYP3iy5AdHIzGascpNLlJ/AK8+xXz",
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
        svg = "eJwli0EKgCAURK8y/AuUGZWgrtu0bS8pfXchQnX7/MQww8C8sSUdFeVxNBBaasKdY2VHaiFwyifXv78yetvJwdsrVEZ0tBmoeZ95DBoafZNqnlYjqED+A0VjGjs=",
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
        svg = "eJw1yzsKACEMRdGthGxgxuAUA9HNBAtBLKzi7v2mupCcx5KblASiAR0hSD9tMy9Gfs4/csk1gbp9hr76zVLAH0HJ9FLXmtkbf5G3jdkBnnchQw==",
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
        svg = "eJxtyzEKACEMBMCvhP3AIRxXJX7mFGMrAfX3apHOZlnYWW75NxqCD6S5FjVB2L3XZCp4QfNk5OfAyM6DL/eX+wUl3BtU",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[cfg(feature = "paw_print")]
    #[strum(props(
        svg = "eJxtjUEKQjEMRK8Ssjc2NakV2oIHcOHWXamCggv5iOjtTa0LP0gYJmRemNQuU7ueYMroEdozI7P5K6NgScuRljSnLI1fOP6hes5hAN7h+PrBbvV+hmPG3QbYVQUFZ8Pm+liRVhN0fa6LQFGAyYnuA6kHXpNEEJIA3LPtDAe1jd2h9/We8gaopDf6",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[cfg(feature = "pc_case")]
    #[strum(props(
        svg = "eJxti0EKgCAQRa8yzAHKCRUC9QYdIlIadyED1e3TVi5c/c97PFfSIXDnKOyRNMLrcUEozz+c8slSr0KoxGBwcwuCu3ZhiB43MkCaJ0XNNdq5FSzbISfViQ+yzyVN",
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
        svg = "eJxty0EKwjAQBdCrfGY/kck4JkKSE3gJiYJCBCkubE/flCy66eYPw38/fe+/Fx6ZPuIhVwQOUGjPwMq6UEmnjZS0wwhRFmccnN08fFNnkHN/RCGxGdvRrkv0RbyM3EV9T7U9UedMIoT6H3fK5Dc06rICRC0r0w==",
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
        svg = "eJxty0EKgCAUBNCrDH9f+QMFQb1Bh4hfYFAQ0qJun9YiAlfDzGOcLEnWGXJ54p6QciiCnE8Nrns9uH08IiZPG2vYxsAULONHg4WNreKK5BPrn90RqCGs",
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
        svg = "eJxty00KgCAQQOGrDLPvxyJJUG/QISKlcRGEDGS3r6FFBG3fx7M5LgwU00rsUI0Ip8MeIReHHcKRAtPTi3RvGxm83WcmCA43NYCpNGghiS9NBgzVrfqRe1LDxy5FUyX0",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[cfg(feature = "percent")]
    #[strum(props(
        svg = "eJxVykEKgDAMBMCvlDxAaUFFSPqZ0EOheOgp+b0JEYqnZXcHR39akkyQb0hqeViUqFK8VtwdVeQ+ebTESnBuxiZB8WSJwWCQRe3J18/qtyz8AueuIvY=",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[cfg(feature = "person_standing")]
    #[strum(props(
        svg = "eJxlykEKgCAQRuGr/Mxe0rGGAvUGHSIsKCgIiajblwQRtHqL77k4pTgPiIcnw4R0hxBPTxUFVzwa3NptI3pPSwPWsEpgIXnI8GFBDQFDFP+1NQyj9/KVC3d/Ih0=",
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
        svg = "eJxtUNEKgzAM/JXgu1kTXWpBhb1vHyFsoCAqTIbu65e2OoSNwKVN7nIh5TT2a98ND5jGbpifVUIFMDCDgL7IJHV52jl1GZgLKStPYNUsmjikRROzp0fq1Mwt3KvkprNI0PEra3QwGA1KGYMPObRuw9goULI0Q2N99RwhdiSVP3xP9SJ7+Q6HHImA24Oft0KrwFjkG5oQaEF/nrCvhvlZS0TXAo0Dh44aUmfZBALSE7JNPRwdVBK0vx7eAI8L7id5h+vqoeoPp7JTEg==",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneForwarded,
    #[cfg(feature = "phone_incoming")]
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaaEFUqBTt6ZvsahFaskzYZJIJU41Dt3SP/gbj8OinZ23IAoOAB2bwpqkOG6WpInFmJRlYqDZsYNYkeZFypCfq2E53uNbmwroNA7+KlmVzLkEZI3mVCejCiqnh0RZZgbnT6jFB6tjM/uErVYfc6bscSiQCvu/0VAqdAKMvV8xjoAP5KWE7DcujlIjOHvMAAQO14oa8NGDBdoTsMoW9gozE2V8NFcD9gZsl7+iuGNV8AGG7Uuc=",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[cfg(feature = "phone_missed")]
    #[strum(props(
        svg = "eJxtkN0KwjAMhV8l9L6xyWZ/YBt4rw8xUJgwxAsR59ObrN2YKIVTkn6nJ6QZr7cLTNyaaOBFrWE2MOktpXTJm67ZKdQ1XyhnVC3CFMuK3vvHAOfWnJiBPCZ+Vj0Dg5NDlpGiFJQwpKL5IaKvbIUuaHefJb946//wiqopHNbPoUYi4GGTp1EYRBhjXdTNBwNIpcAyGtZ7aREdI7oECRP1JMm+GDz4kZCDVdkmiGX2/mZoAG4HXFby1nXporoPMYNQDg==",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[cfg(feature = "phone_off")]
    #[strum(props(
        svg = "eJxlUO0KwjAMfJWw/4tNWvsBc+B/fYiCwoQp/hDZ3t6kHVQYLddekwt3Hd75M8Ht1F3JoI9AFi1l8iDblGXRETD6mZBDr5AZuJT0nahHdwRijG7DqpMrYYDWK0zI1zZ5L/IohBKGtGEtRPS2t2iCvjresNbEoRWlD8+Cyl3ej1B1mXNuFpzYBZ7sf4Lqa+9fvUuEFqDk1MCXiCZBwkTdOBz0A8dhfrzusNKp4w5WlkPOhQtdqFBp1abxByugSc4=",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[cfg(feature = "phone_outgoing")]
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaUBAVKkV7+iauFqFllwlJJpkwxTh0S9f2DxiHtp+epWEGDwIMZIFNVZx2SlWsxJmVZGDRaGCm0pCVVKJXeqSO9dTAvTQ31kUY+JXVujSVRwkjeVUI6MKGseHRZkmGqdPqOULs2MT+4StVh9zluxxyJAJuDnoqhU6A0ecbputDB5IpYT8N87OUiK4e0wABA9VihPw4YMF2hOwShaOCjKyzvxoqgMcDd0veq7tiVPUBVOFS3g==",
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
        svg = "eJxtizsKgDAQRK8ybC9mV3AtEm9gay8qrp1I8HN7jYVYyBQD7834dewjzkAFYZ+HaIG4IhwPWO8Sgo3zZPERtc/TofZLFw1DoEahxi6JhD6CHXT7NyVYO4HA3eFMMmn13V3D3Cfm",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[cfg(feature = "pi")]
    #[strum(props(
        svg = "eJxNykEKgCAQheGrDLOfchzBAvUGbduHBQYRLVrk7VMX0eLx+OFzx35ukLVHrRAe9jgi5HKmlK4VXF9NcNdyJ1g9TgZsVMSdBe6EBMoSS4WV/CAPoFVsUpFQ0ySz+egL4vIfgg==",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[cfg(feature = "picture_in_picture_2")]
    #[strum(props(
        svg = "eJxNjLEOgzAQQ3/Fuh2ac1ErpCRzl67dEaAeW4WiAn9PbkMebMt6jr+hGKYkbyr6z2MgiOBq2PDVXTv41zAGaKttDx9oneR4848c13kssHn5WknyFOxJlIJtmYrVGARHtbtgrQudcyKf8zwgaQ==",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[cfg(feature = "picture_in_picture")]
    #[strum(props(
        svg = "eJwVjEsKAyEQRK/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR3lQFBSvwvtZFfaIjxUm9s3fXSGhGTpFHJhGLhsSlpl4ncgOlJZCG2zNZgPCxvPmwQ7UYQq38ZnCdeQK34hiET7nXrVXg6DH+dIacUG4fhH72FMc+2EOJ/0BnL0lcQ==",
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
        svg = "eJxti80KgCAQhF9l2XvlGvYD2rlLDxEp6S1Eyt6+lk5CDAMD8306ui3BbbBFiNmgRPAu7D4ZpAHhCjb5b2ZmJt2wMOljTR6swYUkkJzHWq2yVsAVbwhEpWbqWWC0FPqTxN/Tlc8D4KQp3g==",
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
        svg = "eJxNjMEKwjAQRH9l6b0xsyZNF9KCNy9eew8oRCjiQaT+vZtaNexhdnbfTJyvtwstPDTMDb2gqrLZZbVj3BVojCtaEISKUcXGooLv6ZHpPDQnIXnChC4xMVkdtDAA6U1mXUNv5PD9WfIEb9hNCPlTV4qqOngSs3dTl5H+Kdu6YzC9/AJvQLUzug==",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[cfg(feature = "pin")]
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6H21L2TrBl3BmxevHrwNFCoM8SBj/nsTcawE8nhJ3kfS/HjeaeXRgR19VFl1xd+bRpfTwc5yek3vQrfRnVtCLJClgY/dxMQUrNQCNhtm633jh+O2BEFTQe8vXcGe0ZSc+tqTFCxSg/ED0w6uuIEU23qWq/1pH+YvM/0ujQ==",
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
        svg = "eJwti0sOgCAMRK8y4QIKC+Oi9jKNMSSEEnQht7dVV/N7Q5K7lB1ybyGmgG4yB8h4I9P07UxNyzi0ommu1+kUVsQFMcGsGyv88IP8AJPDGVA=",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[cfg(feature = "play_square")]
    #[strum(props(
        svg = "eJwly0sKgDAMhOGrDNmL+EAqND2IO7HFdCFICVhvr6GLgYGP35d0KEplGgkv00R4clRhGhxBUj5F26+GwfcWBH/vKohM1wqHBXP3bzM2CB8fAxe+",
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
        svg = "eJxNyUEKgDAMRNGrDLmAJqss0t7AQ0gUKriQIqK319pFuxrmP/Mt+77C70AshPzNSPDnv9GG6tGO+UxYAk0KlqSFSuqABXo1eAGI8Rmd",
        categories = "maths,development,shapes,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusCircle,
    #[cfg(feature = "plus_square")]
    #[strum(props(
        svg = "eJxNy8sNgCAQhOFWJtuAAS8cWDqwCCPE5WbIxkf3yl7kNMn/ZWIrm+JhmglXzSpMLhBuC+0bT5BSd1GDFKd+SPFYVZCZlgDnxaCnAZxHOH94ASpGHeU=",
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
        svg = "eJw1ykEKgCAQRuGr/Mw+c8awBPUGHUIoKIho0cJuX0O4evD44lXuDUuimSfjPLzxQwkIsGBwx2JGB0s59gpzPPZzReVELIQqf5/W74taVfkF8VcWFQ==",
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
        svg = "eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv794sZQAiGcnFzi8zyuY5oG5DlN5RnIoYUDwza/1lL0p78Vfe6KoA90dbq8VB2rddayhg0vpt4BFIh1RwBe6uMFWAx/8zU1+sdwK1gD2YrwSn0RHZkgQ7pLCdQQ3vrf5m9m/AAFujAw",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[cfg(feature = "projector")]
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htqd7dNLSQ99+LVe4lCBAUpUvTvjQqmpWUPuzPDW8bdh0fEydOhRgNFTZ3bfbzO/ZMW9qhr/8aaEDGSo3AZw/WM8PTUEsLLEyth9PTFf+nsMXOxV7D0Ug4CQZmG05apytqkq19oI5Op5oCRKAU3G9XZgm3MDd+BdDzA",
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
        svg = "eJx1j7EOgzAMRH/F8p4WO2pYEuYu/QhUUMNWoSiFvy/OQEpJlMXRvbuz7Tw+A6wONcK8OCSEJX0+0xC8wxuCH6eXDzJ29ip4Z5NJaLO7MvVjTblH1xn/71tT7u5698HD4PDBBGS80j0DQyNPbVNMBQIdUaZ4aaigEUMbcwhJyL0tgBqIfTVD1yUytWojkVRemLi+8HaMytIXta93oQ==",
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
        svg = "eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBQqKEjxQW9vYrWCVPYHltn5mPa6v4106MyOE2wkj+QGtlR6LkVK5OC86dttfe7bxeJJR1heUQSSKEFlYXEZYTimBi6sWFghniKiDkr6djQQR0XRuOYQ4uZPAI41NzP0g6tADojxl5dPUz4fKT86w2Jo6kzZ+f66ytMsf9EXdvAFZREShcLf1LXgntDzUsU=",
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
        svg = "eJxdjF0KgCAQhK+y7AVSoTBQb9AhIqX1LWTp5/a1BRK+Dd/MfG6bmSB6nHrQw24wuE5QcLXQY9OUtDCU06NBuDxahC9TyivxC44cmR6o5CX7v8+CNqSq7wah4SU9",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[cfg(feature = "radio_tower")]
    #[strum(props(
        svg = "eJxtjk0KwjAQha/yyL6jb0iaBJpuXHuIEoUKClJc6O1NIljBMmQW837yDffpMeOUzNFKBHvhgaCKgnAS0K4SzTjsqnMcvn7fVD+ViEVb+zpdOXtxayBflnw9I7+SiQZLMmqQn8lQq+ej/tQWBC29ISvKiPYFg2yl1m9gMAor4RQl9mircrA+K9xIRHFgmN2/dAvQ8nlHwoJr9g0UNkhu",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[cfg(feature = "radio")]
    #[strum(props(
        svg = "eJxVj7EOwjAMRH/llL0Gu7SkUtKlMx+BDBJIDKhigL/HDiJtB+ek+O7pnJ7n1w2XHE4HGsAD8cTgjgSMSBH+axPGtHPnmKr/aFvuSbQRauv0xNg3kbolofdZH1fMOUiAvnNg109RM/3WK7BDYXQ1Hv5TuNhwl4C19pKTtKWziV3A5Rp7auILj4M5Ew==",
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
        svg = "eJxVjFEOgCAMQ6+y9AKKH34NLqPE8UuWKLd3izHAT9s0feWaDyXJ5RKN2FZQfcxBdzlVIoJFK3ZQ8zrx4kDiCQt9Pzx82hz+sRfh7x7v",
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
        svg = "eJwBNwDI/zxyZWN0IHk9IjYiIGhlaWdodD0iMTIiIHdpZHRoPSIyMCIgcng9IjIiIHg9IjIiPjwvcmVjdD6+SQ94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[cfg(feature = "rectangle_vertical")]
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHg9IjYiIGhlaWdodD0iMjAiIHk9IjIiIHdpZHRoPSIxMiIgcng9IjIiPjwvcmVjdD64LA94",
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
        svg = "eJxtjEsKhDAQRK9SZJ/MpBtbhRhwr4eQKERwISKit/cHupFaFFQ9nhubOaItVM2gRaIo737n5t3zkIWlMkeO/xVBYrhiZN8s0aIl6i8TH6LmFdkEYtKBNZn0pUM/haFD2AplSSGsd09HndB9+x2ROjBq",
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
        svg = "eJyVkN0KwjAMhV8l5D6zyapusO4NvPVeumF3Icgo/ry9yQQVKYgUGki+nJOkOx9ygiHgjj346IgrrloSEBLsu5WV++4FiQOJioBGqFr9ipRAEx0oR8ZQmeINsIu0qJGQuZY8TzVsobZHdUHFRPbrZfCtOipkYOI3Oo8xwy2gIFynIaeADcL8TNwDskdI43RM2QraZfznmB7Yf+188b/X01P90fcAr2Jhrw==",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[cfg(feature = "replace")]
    #[strum(props(
        svg = "eJxtzk0KwjAQBeCrPGY/NZlW20KTG7h1L2kxXQhSAurtnamgmxDID/nyXqbHtWTMgc6+Q5cc+8Y3IwuEheJ0sOs4/ZA4SFICXdGMOlWVYEgO6tgM15U/wbvEexoLW2ut896iR2uD20qKhVyO+8d7bVRkMPs/3ZZU8AokhHcg3xG27+m5ziUHGgh5WW+52FZfmY8ffHVHuA==",
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
        svg = "eJxVjmEKhEAIRq/y4QHaxmi3hZm5wR4i2KAgaqB+NLdPjaJQFPQ90ad5zOMwdUjzMK1LoKqowXhLdaylsU7Rv040ehM2DlTLBtkFMgZZRlwSNpl8D+kuKPfg9YRrbuYlpHbt8Q/04+KjT7SugmRpoQoMVizu/zgxvQ==",
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
        svg = "eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PQR06RF4sf88vbMut4B2pI7zWueTfJoRPJDVCXtZ7LpE8YftnpnCuD1N4pJIxR7o6FoX6i6tWrwdT2VGlpQNUntaSkb1hZB2TwSB1TgM7h1ZPJ1WMe0sefk+rcq+H+BeYOTxR",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[cfg(feature = "rows")]
    #[strum(props(
        svg = "eJwtjEEOgDAIBL9C+IChXjy0/Yw2lsR4aEiE3wvV00B2dvNou4AWXBF647NLQdoQHj6kf+ewgsmhExZqzUv0ar74bmDkokdKc8bS/zoThRtWfQEhgxw8",
        categories = "layout,design,text",
        tags = "split,lines,queue,series,list,vertical,horizontal",
        contributors = "danielbayley"
    ))]
    Rows,
    #[cfg(feature = "rss")]
    #[strum(props(
        svg = "eJxNilEKgDAMQ68SegEtTKGw7QYeolRBwQ8Zfujt7dyHkkAS8uKh54o50RTArAJB72JPoRy7euf4g4LyCHej3v5xthXbF9idiIVgV6KBUHxVpr35AYbpHaQ=",
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
        svg = "eJxlykEKgDAMRNGrhOxFUkil0PQGHkKiUEFBiojeXmtBBVcD877XMek0QBI0CLoLkrv2uDf4unDwv4xLxd9o6dYIvWDL0GxkIpms+X1tZiAHtrIPnanNJS8=",
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
        svg = "eJx1zk0KgCAQBeCryBygGMpyod6gQ8QUGLQIiajbl6P9LGylvPlmeJomT/MoaDfQFhIEHenjDVyP1WUUVicZc15AdW/ILGWD7zGmHGRsGDVfmxr90PopkW+79KsTg4GuEtWGyqEK05DaE0NeQ20=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[cfg(feature = "school_2")]
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRB9v3dKeL6bak+QrpNTgmB+lpWxwsttwYt188xvv0yHAZ3FEEhE59Rj9jQI8B/EGUZTId6DN2ZQ61gHIuYSWkigrAe90Qr6NdA3HfthXW9HkQUez+vdB6KdU/iHY/6xv0s0kX",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[cfg(feature = "school")]
    #[strum(props(
        svg = "eJxtjFEKwkAMRK8y5H9xE2KNsLvf/niIsgoKFaTIorc3VdCiJRAyeTOTrv3thEOmi6KDBYVBqaTV9C/pQ9nAEQpp1gsE0YeDXzud6yAt2KBB/iv27GnHX398+eNce78uRQ3rxpsF0v2Aeh7rcES9Z2IhjJl810em7eR50/IENrk++Q==",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[cfg(feature = "scissors_line_dashed")]
    #[strum(props(
        svg = "eJxtjkEKgzAURK8y/L1p/yeGFJLcwEOUtKCgIOJCb6+JiKJuZvNmHuP671jj56kqlRZ8UliwUHCvhIKLzRDbP+LsyRLi5EkTBk+5ssHgdkvHGqawqrRIcbec5lnJ5klT8VutL3QO8MlzNMz6si7kAYlc0ALW+j4b",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[cfg(feature = "scissors_square_dashed_bottom")]
    #[strum(props(
        svg = "eJxtTdEKAiEQ/JXBd6/bxcRA77mXXns/LDAoiCOi/r5V4xISl8HZ2Znx9/mRcArqYMA8MxijPNKs+Wh+HMIT2XYBfrYLLT81+U0OnPwaS6Pk7l1PsaIk3ZjiZYnXM+I7KKewBMUK8ZWJnFSx8e8GQwYViWV6HWZwqJDnv6qkl0ay38pO103cVrth6ywKrkEfjKpRwQ==",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[cfg(feature = "scissors_square")]
    #[strum(props(
        svg = "eJxtjWEKwjAMha8S8t9qRy0Vut7AQ0hWbEFBSkF3e5MW3QaD8Erf+5LnS6QKKeZ7qiMOJ4QPPwjvPNXUjbkZpfnBH2UheMqFHhGIXcdhQ2iWDyM9DP51qwmmEa8XZbSBrnrgEUrSFaONctBFZiF+VXxdW9xU7nQ9edsenDo7C03/h77ULj+l",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[cfg(feature = "scissors")]
    #[strum(props(
        svg = "eJx1jUEKgCAQRa8yzAFMzWIW6g06RFhQ0CKkRd0+nbLaBMOfxX/8Z8McwzJC2B22COHgFx3W6G11ld6u/TbB4LAjoTRw8GUmdx9CSzA30QiilyiiZFCERfhrUkYQcKRFLZ+dE6pmLzo=",
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
        svg = "eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf29qoSx7WGYZvuZ7g0XwzEAE0ZNCQuWxAmXbix0r2CtIhFAEM4I9//JTw8rdEt1BIQ1Tbdd8iMcbaw==",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick",
        contributors = ""
    ))]
    SearchCheck,
    #[cfg(feature = "search_code")]
    #[strum(props(
        svg = "eJxNjMEKgDAMQ38l9D6l1YODbf8iVVBQkOFB/95twpS0h+SFuGM8F0yedgtrBEUUXJvz4CrlDunEFH1c16jbDL08MROip4Ggd3Gp9OLfjDCETd90+evMA0dFI4U=",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = ""
    ))]
    SearchCode,
    #[cfg(feature = "search_slash")]
    #[strum(props(
        svg = "eJw9jEEKgDAMBL8Scm8lrYEe0vxFoqCgIMWD/t5WoYdlDrOMnNO1wpzxoOgZkmfHwKgyNKFiW7F9gZIxIdidkajy+VhPv1bplUAQyI0+tvXMC5TrG5E=",
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
        svg = "eJwlijEKgEAMBL+ybK+S08IiyV8kCgoKcljo7yVaTDHMaGw19gVxG0WIahyJeD5z7f7sek7Xitl4FEGRZmj7JI8s/gLNoBNp",
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
        svg = "eJxdzMEJgDAMBdBVPllAUwp6aN3AIUTFeBMpVbe3zcGqBJIQ3o/b5zHgWKcgnlrCfnoyBJnXRYJe0oEt4dLRuSoHOqexrzKKUv9/e0LbEASTp74B28iDgUGtlTbhDDN5QbZopEDOMBZ4A83aMyw=",
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
        svg = "eJxty0EKgCAUBNCrDP8C9X8hLtTbtBBEhVro7fMXLQJhYBh441LMBxp72ghdPLGM5rfb2MIU3KIquFpSf3wtMV+nJwsLFuxgA6vuExM7iEYgq77Mj9/4SiaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[cfg(feature = "separator_vertical")]
    #[strum(props(
        svg = "eJxtykEKgCAQRuGr/MwFaqYQF+ptWgiiQi3y9s0ILYJW7y2+UHI9MCSSMGFwpI1wa1i0MpvCYiqF3sqYvrdcrzOSh8cOFg07c6/4sezUQFbjOv7DH/glJoY=",
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
        svg = "eJx1jEEKgCAQRa8ic4ByJMKFepmSFKKFCNntG0cSIlr9P5/3xiS/ZFEsKBCpRfBxC9mCBnHxcMY1B2qSEF6cGavmDMvpQ9GAE/x97fYeDy+KsjAPEslCahSKo/BFbKUethFvBXVzUHf6BmTrOLI=",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[cfg(feature = "settings_2")]
    #[strum(props(
        svg = "eJxVyjsKgEAMRdGthPSi8UMQkqltXIREYQQLGSx0934GHSzfu0fWYfMwKvZlAeyzFp3k9+fkK1QDcdekYnOwZQLbFYkRgmKFYMezLhTzj3HsL07oBCCgJFo=",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[cfg(feature = "settings")]
    #[strum(props(
        svg = "eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGdmiXpP+X/G13PdzPcOzdjoVEQM5IIRx0A038UHcP4s0vwsjAtPaT1nmStkjoqlFukZpNqaDV+YAI+SIDGoWYnfQQ8aIECaHcli03JAI12wKwyFjAjLZNcVwnO0DG+r00pYRaUjkCgvLAypLYXMIEj7ViwsR+bVf47RfJm1GSX/8+KiJCgqJbSw7fAbe1t7NSotgbwZlSQ/Id2jky2++rh4HydkO3ig9o6MbLbZxOML56x+Lg1jvvYHymPy3K6eEDOjV/Ow==",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[cfg(feature = "shapes")]
    #[strum(props(
        svg = "eJxVjU0OwiAQRq8ymX1HprQFEuAE9QLuGkqExIUhJNbbC+pCM4v5+d7L2PtWE+wOz5oksNhIkQLRigdaxmVgEsqszDSB/AmBiY0eSExy5aWpmsyfO6uxMxf09tR/eFtiqPDIe00OFUKK+ZrqezwcSoSnQ54QStu4W533NuQSbhFCTxXNLW9w7+H4Xhr6gfwLE1cyPQ==",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = ""
    ))]
    Shapes,
    #[cfg(feature = "share_2")]
    #[strum(props(
        svg = "eJx1jEsKwCAMRK8iOYBtbO0H1MuIC0G6cKW3b6LSQqGrhJn3xviYfQoiW1hA+GIBD7rVggZnpt46MyjOUXVugy79UOe79sVSvIIoikotV1qrSO8iNdLL6S5Xsgulh9Qnm2wMrxFzJ9Qg2sLWBtjiWXy0G5SxO3c=",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[cfg(feature = "share")]
    #[strum(props(
        svg = "eJxNzEsKgDAMBNCrDNmLdtDSRe0NPISgoCBaUERvb1q6kCzy4U18HK8FUy9DC8PbjQTR5NJpMfwfKt6Vk+DrFAo+Htu7rfuMeKz7dfZiLKx+0YCDza6I4LN7jBoKXmrvBA/Lrncmn+0H7lsmUA==",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[cfg(feature = "sheet")]
    #[strum(props(
        svg = "eJxtjmEKgCAMha8ydoGYERSolylJIfohQnr7piYo9Gdje997m/RmD+CjQoHgU2nWuNMGhbQisDAjpFIfdwRb1lpO2afl5W4DiRRujFKhouAQYo/IWyYz05G0VC330TKysaa2cFGnX/JL6/5oRzr2BXSZPJ0=",
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
        svg = "eJx9kMsKwjAQRX9lyD7RuXWIgaRrN926lyhUUJAion9vHooF224GZs7ceVwfz0O8nCi+gmIoGoLaKorPkrV+VXHrb4d7T8egOgbhYY1kmIs/dGVHosWgoRz+GzqAGL2eVbMrcj0jT6t5Y2QPTMF1XsyNsZYkTZpocSYB7BbUJQqNzvvaUw0Z24T6xcegN/j1V9A=",
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
        svg = "eJxNjc0KAjEQg19lmPuOO3H7B23PXnyIpQoVepDFg769toosgRySjySW21balcozsWcqr8RQpi2xco6Hb5vjjxrxYDX84R12Xx+VLonPkNlQt4oGsZYUsmAFgeYhkIrxNYjzu1QlmOlTuKZizeRkOZ6MKPpH385v/lsrmw==",
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
        svg = "eJwli8sKgCAURH/lcvca9lRQ123ato+UrtAiRHr8fVqLGQbmHB39muAx2CDcX1/BJTIoJAL5sFH6d8xvjVZXRbD6WBKBMziJHiRX8zDKvYWO5ZA8meCqsIWyL2GRGws=",
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
        svg = "eJwti0EKwCAMBL+y5ANtpD0I6m9KESQR2oP+vjH0NDswm7q2eauga5X3ycQRYUcEB9g84E4lbX9YUqtyYXCmkzCCY/I6Gl0tXk35ALQNGMw=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[cfg(feature = "skip_forward")]
    #[strum(props(
        svg = "eJwtykEKwCAMRNGrDLlAG9FFQb1NKYIkQrtobt8U3cxn4OWh3S4VDG3y3IUSIjiBAxLC7hOp5m2pmnuTE8buCG8oxIeXZ21+57+qH6LnGJY=",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[cfg(feature = "skull")]
    #[strum(props(
        svg = "eJxljeEKwjAMhF8l5H/qLtJZoR34AHuIUYUKCjJkqE9vquiUcT+S3H1cYj6O+XSgfE8MZRptMOVb4i13cfVOu/ihzIfnf/oHuwzXQvvEfSBtJi1hEq159ef0DHWesBHnBVLXgscS69Fay6Ck1LwE51tZO/UUTHZTI4CD5buZqq+/ZU+mLDi/",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[cfg(feature = "slack")]
    #[strum(props(
        svg = "eJxtkEELgzAMhf9KyH3OrBYqtMJuXnb1LlNWb0PK1v37NR1TaXsIOSTvfXnR63x3YOflYZ1BhfBeJmcNCoSPwQvC6g1SJRG4C+z0mRWdfo7OwmTwRi2oSg5U27B2DQVcNRBXnLGI1zsdYcGXGjwyfUL+M3NYMJcMa3qRwX6zAkxs3irJczijGE0yoO2peQXJmGSr5Ikyno9P2035AFXi57TwKZA91UMeLY420hca7nAt",
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
        svg = "eJxtkEEOgCAMBL9i+gHpaogH9DceTIxn+b3UFhHl1FCG6Zawb8faRZ5ppO7ETJxqhB5TF0xL6AVawo1Kk11hUhnIBDUpPYaSUk2nU/DVmufFTpQVfy+ceXzxytvUb8N2qfGRczdWG3Uq6t/wrd1cyWmoPJlaVm+7GAs88TN8Ad2YYmw=",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[cfg(feature = "sliders")]
    #[strum(props(
        svg = "eJx1kEsOgCAMRK9CegFtNYQFcBsXJsY13N5SfpHAagJ5HR61z/1eKqIDQlCB8wQVyQFyBkpHb7cEedtQ3BuaEZk4BjIRSNDKpZXyCOcaNrVvgfIlFYOU0qvbO4NuL/rN4MKhKOCwBT1xEKbrmtzOu5n8zHRNUanP9N4P3ipibA==",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[cfg(feature = "smartphone_charging")]
    #[strum(props(
        svg = "eJwli0EKgCAURK8y/H2lYtZCvUGHiJS+0CJEqG5f2mYGHu/ZHLeCK4XCjqQm3I5GAse0c3GkBCF/SBGetrmdt0PtvD3XwgiOFql6YybMkAJSsT66H+jqVsu/Hn8cIg==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[cfg(feature = "smartphone_nfc")]
    #[strum(props(
        svg = "eJxti9EJwzAMRFcRGkCNbEdOwM4GHcI0oc5fCYa221eOKclHEDrujnthWx4FPhENwnudS47oEb4RBWHTmhHysj5zUWtwCre6n8IrlQxzxDtbGMia5MlZ2KXTY31PVipQp2dAyAkIGU7M5AWa/ilN/XCFjTQyOFKqr65po6jb84H9ALFbM80=",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[cfg(feature = "smartphone")]
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S9gFqioKHpj/wEWKL6U1KwPp7DZ4WZmdiK4dRewQBdNdsKuAZpKWeah+dQF2wgH6ldZ8UR+9SvHZTyoKNA/Gqw8T+OU0vhMIY6Q==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[cfg(feature = "smile_plus")]
    #[strum(props(
        svg = "eJxtjlEKgzAQRK8y5D82sxhpIHqDHkJowYKIoIje3mwE8yPs7DDsW5g49+uAb2s+IiA39nRIkxZog6UzXXwp1MUbfYP1wspDUKtsVgHH//TDwdYEg0Oy7VfaNVWOyir1zCajVyq/0Rf6rsAGfmgeujFAtnI4AYAxOVU=",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[cfg(feature = "smile")]
    #[strum(props(
        svg = "eJxVi0EKhTAMRK8yZP/9plj4H9rewENIFRRERF20tzdNceFimEzmjYvLEdcJMXtiQzjEWkJMGoP71j64fbhmjJ76H7g7ubEw6Io+qoIWJLh12SYk4+nftEzILJeYUUuahC3Uw8qPLemGbR1VOr/oGwJIK6o=",
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
        svg = "eJxtjEsKgCAURbdycS7lQ8SBupegoKCiQQPdfc8P4cCBPC7nHN153Bui8oIEEh/FNxLPvKns4KZsBVfcVN1IzW1Nqs3vPsu7Y/XiohnKSC01+GWeScc1LArDiCqDlg5jC/6+CF39AWeqNkU=",
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
        svg = "eJwljUEKwCAMBL8S8oC2kdKT+plUqiA9iND4+yZ6yUIyO/EtcQcJeCK0EdAhrNlkxlfungPShZBTeXLX7YHR79aLnkvjmoAVJqVZu2Qm8ym0ztHX8iYQZ9B2kL6ggGocbobQbCtvXPwBuRInpw==",
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
        svg = "eJxNy9EJgDAMBNBVwi2gKRQRmoADOIREQcEPKX7o9qaKUJKf494l27LtC9kl4B5ktyCCsiBAU/O1mir1ty/1SaWO6VxpFoyRuBs4kH/rxx4pFlmEPnqGH0o=",
        categories = "design",
        tags = "",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[cfg(feature = "split_square_horizontal")]
    #[strum(props(
        svg = "eJxNiz0LAjEQRP/KsH0ws+f5AUlqG1t7iUIEEQsR7987ueK4Yth98F56Xz8Nt2znA3g8jTUQMXig5pd9jWL9ELXBStp0v6Sl4g5jG6oiOKj5l7FGNewNvIVV9ny87pg8m0fDT5dumJhtK+SMcrtV/i7uJUU=",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[cfg(feature = "split_square_vertical")]
    #[strum(props(
        svg = "eJxNjLsKAkEQBH+l2Xxwus/zAXsbm5iayyqcIGIgy/n3ziKIwVATVHV+nl8zLlM6jtidxupG0ASZZnolHALj1IZU8qrrJf8i7sFNG6pHxB5Bh22N100WO6Zmf9399rhi4ZTWCe8AFdSXS1De3W6VDzggJWE=",
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
        svg = "eJx1jssKwjAQRX/lMvvE3PRloOkfuHUvbTEFCyLBx9+bbJRSshmYc+4Mt79fYsDk5VShCtpQhv6Q2dD/TIemYEh0BVUVTQdXvCG36jGPEW8vbARhXq4heqkFHy8JvJYphryneA7+P610cLCwT5rRQLdK16AiGFQ7Kt3CKCaWBs/kzSq7r7My9alxLDq3cV/nc1ky",
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
        svg = "eJx1y0sKgCAYBOCrDP++h2XoQr1Bh4iUdBGESI/bl25ctRmY+RgV3ZrgXdh80sQk4dY0Ep6S8SsD4Qo2+aJGdflg1LEkD6tpZgPkWSBPFXbZTmAcouF/2EOg4guSBiY4",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[cfg(feature = "square_code")]
    #[strum(props(
        svg = "eJxNi8EKgCAQRH9l2Huk1qGD+i+R0noIQhaqv0/3EDHDHN5jfM2boN6BHIFz2VkC2YXQyER4dK+ShBVHP/ZD9OcqjBTosAbWDA6a7rv5+xmtbtB8/gW0QB/x",
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
        svg = "eJwli0EOgDAIBL9C+IBpvXho+QwSIfFESLS/F9vLTrKz21w44LEztGM5EN6OO8KYqWKXxuo9RUVq23+gxuZ8C3hKBE5XanJM5mhp+gD3iRqN",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareDot,
    #[cfg(feature = "square_equal")]
    #[strum(props(
        svg = "eJxVy8EJgDAMQNFVQhawUUEPSTdwCLHF9CYloG6v0Yte3+dzzYvBKdghaC6rmiCNCMcje0mmL9RbWozc+BB5m00hCU4DUFAKXtz+pf+WC2TUHic=",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareEqual,
    #[cfg(feature = "square_slash")]
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFpRVBo+xkNJiAeSsD4+yb0tCwzUzqdCp9cyhXTjvBXXBG6VcwITHKzTmABWlkiaOWRl8Cyo80b38MnzWsprqshtQFLnxpg",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareSlash,
    #[cfg(feature = "square_stack")]
    #[strum(props(
        svg = "eJyNjMEKwjAQRH9l2HtidgliIckfePUuaXF7kxK0/r1ZhR566mVgHjMvPe9NMWa6RnCojj0jOHF+6CG3WIMha+hdY/0NIPBDD6GSTmYoafNwAJ/3opc7blqm2vDJxJGwrJmE8J7HppkuhPXPdZof2oz0mx3KF2ZlMKk=",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = ""
    ))]
    SquareStack,
    #[cfg(feature = "square")]
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHk9IjMiIHg9IjMiIGhlaWdodD0iMTgiIHJ4PSIyIiB3aWR0aD0iMTgiPjwvcmVjdD6wmg+D",
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
        svg = "eJxFjVEKAjEMRK8S+t9Ipu0mhW5PsB5CUFAo4ocf6+3NLov+ZMgw86a9Lu87XedwNk6Z9gOqDB2FMpsuhQ0EIQGJsiqJsZg7I3KpMXFOobfTxuntR/NQ9gJYJ8IBjBNXibI4CSMKZ59y598ej+eNVswBCPQ5dBVXf3fx6BbqX/9VKqc=",
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
        svg = "eJwtyjEKwCAMheGrhMxCTXDooLlNKYIkQjvo7ZuK23s/X25VLxhckE6ESQU5Igza33tCycevJHdr8zaFblXfx0kKHCEFYvC54CbyARXVGDQ=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[cfg(feature = "step_forward")]
    #[strum(props(
        svg = "eJwti0EKgDAMBL8Sci7YhuKpyW9ECiUp6MH83ig9zS7MtNH1ACdGyghPYdwD9MPjVZS2fY60acNPU5jW9b4YS04VKKdCEDPyMJcjLwXLF/Q=",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[cfg(feature = "stethoscope")]
    #[strum(props(
        svg = "eJx9TcsKAjEM/JXQe2KTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjNntPEwi5LTlyYIF1A8jOVwFRroPKeqgR4qoVY7ffGuPhx48yI1cSklfiCDclbUZrSe/uCTh0/p/c0X8+23lplxO0ezZiDbRHNqy4qBym9Vye0zIy2Q==",
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
        svg = "eJwliksKwDAIBa8iXqCfRaGQeBkrVehKhKa3r0k2b+DNFDbnR4C/ituOwG3SEytSWaan4sIBKnZrVDwQsj8R2tjXrtD+Zt87+gEyQxim",
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
        svg = "eJxljEkKgDAQBL/S9AdcEE8z+YwGJ9cwoP5ek0MI5NQ0VJXkeDgspstcuRN3Ot2U60w8/xCvciNyPUGmwgepVod2gWYtg/YByRAewg==",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[cfg(feature = "stretch_vertical")]
    #[strum(props(
        svg = "eJxVjEkOgCAQBL8y6Q+4hHga+IwShyuZRP29bCFw662aoz+V4muxg55wqVgcoOQN6Cup+HCLJrnC8ZL3jgtV68YOq/lnMx37AasZHsI=",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[cfg(feature = "strikethrough")]
    #[strum(props(
        svg = "eJxFyjsKgDAQhOGrDNv72M0iCknqNB4ioKAgYmERb2+SJgzD33z2ie+BzdHKEzQs0cBgLOuknw2UvB2K8bZJBUtUaIWcP4epueu8dyRxJCMhsSMlfDksuVKbbVH+B6WdHuE=",
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
        svg = "eJx9jEEKwjAQRa8yZO+YmUknLpLewENIFCooSBGxt2/TdFOadjF8mPd4IT379HpAH40zkP7REE87zNuGc8Ft+Ny+HdyjuRKD/Cij/FoDtlUiQNzVANsd8qYLijpoUEVP6K3PV/EUxU15j9r4I28OldyBVTpLbuONN3ZP1g==",
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
        svg = "eJx9jkEKhTAMRK8Ssv/5TSwVofUGHkKqoKAgIqK3F6MrqW4GkjcMz8d+jkMLcQ/IgjAHtAhx06v0/wuXfqqXDpqAFQvIquh8PYBJkdFSkYEGk2WNRItzcg7ufO9VAixdSkDMCxkdZfZa/n0aFGRyFU3UDuLgTrI=",
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
        svg = "eJxdjMsNgCAMhldpOoBaVIwJso0HEgIkeoDtLVQPcmib//HVpOiLd+GEFF24rwNpGVagjdcMmkd2M2qE1owfY00jMzE1I2TFd0coSnShphnoy/oNtZTVJDDfrlxNQoF2gaou6vf5AfENMl0=",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[cfg(feature = "swords")]
    #[strum(props(
        svg = "eJx1kNEOgyAMRX+F8AHbWrBIwvybPZgYNdke9O9X2k4Xow+UUO6h91LmaViHfny5eerHz/vpId4aB4lLcMRLqzTqle/K/cd0RcgFmcreLcB78G5F20H6DPyLa5O8QPgwESnE5xNxVhGCvZwNhl18HoKq5ZbNI2ghjZEvU9goiBqmsYntwZZ5FU1SN9HIdEyA2zMSAPavCpv0C8HMY8w=",
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
        svg = "eJxNjE0KgCAQha8yzAVCRSgYXbfpEJHSuAsZ+rl96qJcPfjeex8dqzAEh4uyYE41oqehMk85bgK3Q4OQS2iEKwVhh2UET+Mc087SSLnVg6dPqBVMs/l9faFs17yy/SUj",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[cfg(feature = "table")]
    #[strum(props(
        svg = "eJxNjF0KgCAQBq/ysRcIk6Bg9QYdIlJa30KWfm6f+hC+zjDD56aC4Gg1I+xlZvI8VOY5x13xOLIEiekQdVQ0ckEj4W3iTkGl8ZLVwPM/tFik/3XCTL35AL40JWw=",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[cfg(feature = "tablet_smartphone")]
    #[strum(props(
        svg = "eJxNjDEOgzAUQ69ieQ/N/02rDAk36CFQQf1sFYpauD3JAsiDLVt+aZneBVtmJNbMO7FUU8Km+WMlUwLxn8diNXr26dYOffoOxTBmvh4Ig0LhqwTq1ESvBfQnz7NwNZnTLjRUg1xQERKt83JMO4OeJ7Q=",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = ""
    ))]
    TabletSmartphone,
    #[cfg(feature = "tablet")]
    #[strum(props(
        svg = "eJwljFEOgCAMQ6+y7AAKizF+AJdRIiTGD7JEub0d/mxd99rQ8q701ENLZL8ytTeyMPUxoRdY/1FyPYtCOk5htlwKV70zdY/oBlqwZXIe0iyrkfECb2T6AISvHP4=",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[cfg(feature = "tablets")]
    #[strum(props(
        svg = "eJxNikEKgCAQRa8yzD5zKrOFeoMOEVNgUBDSom5fmlSb/+G9Z3gOvEzAh0WNwGe6YFGhM+UjnclRwiml3N7/y7Zh9zBa7KkC0p5klBF+aq1F0wJJoRrQQnZFnDe7AEPiJws=",
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
        svg = "eJyNzCkOACAMRNGrNL0Ai0CVXqZBkKCq4PY0LA6B+mJehqSqtAKaMXgEGdZo7atMbu9Mx11gPv2x19sEweQhVQ==",
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
        svg = "eJxFy0EKgDAMBMCvhNxFYgU9tP2BjxBbTA+ClID196YVkRyW7LD2XIUhODwmIIKha4fe9hW8/XhRJMPjLzluAlcKwg5pRsi3w0GjtNDHIHBMO8vrpTa6rjv/AKSmIKk=",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[cfg(feature = "terminal")]
    #[strum(props(
        svg = "eJw1y0EKwCAMRNGrDLlATbCUQvQ2XQiiQrtobl+1uPowvNFWs+VULrSaynMH8uAD7MAMj52ibotEndA4EJ8Ek79vr7jescs4TPwBscoY2g==",
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
        svg = "eJx1j0EKwjAQRa8yZF/0T5rWQNIbeIiCQgURFyL19s7QUmOZkMVP8t6fkPQcXxNdsjsH8iMT01FXIzs3pIPSIW0OYiGBbIlBiD+rMioQo5QaS5LnJpj3DBOgrTQEVCqe4hv2N2zgCW2t8U/ut8eVZmTXO5o5OwRHHzmeJFhDTHVWUxF4YZpls9+5JfPLVHRrtdvcL2TufkI=",
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
        svg = "eJw9jDEOwCAIAL9C+EALaZMO6meIg4np4IS/L0jjdAHuSL29FZQy0omgbLwQps1sYEdJh0slLdV3RBgJ/8kdiaVbljakV5AZH0fGB0F0RSbFuXwRbCEM",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[cfg(feature = "toggle_left")]
    #[strum(props(
        svg = "eJwly0EOgCAMBMCvNPsBlYPxAHymEiHx1DQRfi+lp012Z6MUVpKRcIK+dmtNCDuolvZUTTgCqM8K5ES6RY6b/XLkJvwW4uFSluRpLjO+5h+YQxwe",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[cfg(feature = "toggle_right")]
    #[strum(props(
        svg = "eJwli0EKwDAIBL8ifqBNDj0l+YwNVehJhCa/r+JlF3Z2mk4y4CkPW8dSEXR3vBAyP7mNO9YTYXk5XbGPdoQ3GonSO4F2uuS0uKbx9VPi8QO0BxxN",
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
        svg = "eJxNizsOgCAQBa+yeb1RCKFiuYGHMEJcOkM2fm4vWBibKWYyoeZV6WY40FmSCsNOIMllE2UYD6pXc6CXMYx9iGFfVCgxZkvGSVta6e5XjCU7HYP/0gOEwB5X",
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
        svg = "eJx1y70KgDAMBOBXCdmjjVVRaJ1dXN1FxbiJFH/e3mYTRA6O4+Bz+zwGkHldJHjkDOHyaBHOdQoSjwrh9hh7jz9j41IFjduGIDB57NhA1RejoaSkJCfWtOXAwGA0FNdhFSp5w/oLheyvfAD3zyrm",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[cfg(feature = "tractor")]
    #[strum(props(
        svg = "eJxtjkEKwjAQRa8yZJ+xXzNpCklv4Na9RCFCBCkiensTldbSQsiH/Dcv42/He6JTUPsdmdRlUKt6v6mvvR87Q8DBrBSO0KwWcCRRs6VGg9iUCw9huySvaIq8Hu5i5UuycBEzsi4pSWMai5ch5jMNQbEois+g2hKvoCAV+tYjNu/r1IyatrUEl7Qs/qkCuJ/BfQzbP8MbOOtWIA==",
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
        svg = "eJxtzU0Kg0AMBeCrPLIf2kQrU5jxBh6iVGlcFEQGf25vxI3irBLy5fHC2H0T5r5NGokrgnb9T9Oxr5EKwhKpJIw2hOrw2AN1GD5J0UZqSjCrPZvst5OwoJj8Hf4e/HaGGWIPEScuY43FXvrMFVVX2QDkXzt+",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[cfg(feature = "trash_2")]
    #[strum(props(
        svg = "eJxtjDELwkAMhf/KI3vwEo9rhWtnF9fucgoniDhIqf/exJZzcQgvvHxf8vP8qrgMdNojVelpzDuvxtwOckCaJZYAYYGyQo9dsTWwstjolP5oPdJkknHiCmuNxRwo7Al01p9zvz2uWGQgCYRF13x7dpbei8OObfBWNugrRVqfxAZ/AHwiN3Y=",
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
        svg = "eJxdjFEKgCAQBa8ie4EwiwpcL1PS+isL6e3bFqzwc5g3z+e4s6GYTmIEu4LJFWEEUxCcQFGoClc6mHQU/PB0wWstdnmt01S4fW7dWqydPj3/S31q8xtvXyw5",
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
        svg = "eJxljsEKwjAQRH9l2Htitq5NC0nPXrx6l1WI0IMUKfr3Zi20gpeB2Zl5bHpcngXXTKcDuDvu1fkWwbHzUoXPUUO9mEG1hYNaDoaXKjwz05B2BhnSimJBX2QUyCwa4FtnbUMU12x9vU863qDvTNwRpkwNQV+ZonWW9Jf6/bD/39cFx3W/0DbAB9hgOrE=",
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
        svg = "eJwly8EJgDAMRuFVwr+AbRW8NNnAIcQW05uUgLq9ht4eD7587aZUGNtKKWoMkDz5k9zrYaS1nWqMuIAeRgLdrZj+FUB9nJcxO3MgH7ldFsk=",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[cfg(feature = "tv")]
    #[strum(props(
        svg = "eJw1i0EKgCAQRa/ymQtUQrhRL1OSA6JiA+Xt0yL+4i/ee6b6TXBbUoRmSROC5yOIpWUlXLxL6Gwm1M+pbZwz0+icKTm2yMmjZE5y9kpDYVHQfa/3G+4BVV4esA==",
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
        svg = "eJxNzNEJwCAMBNBVQhaoBosU1G36IYgK7UfdvmlSQQI5Dh4Xeiuj5HpCb7neV0QHHhwfGX0eU9imSkHsoIhkEB5OuyMM+3fO4/OLFUNqnBKpOjHtCxrmJBk=",
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
        svg = "eJwtykEKgDAMRNGrDLmAbQlZJbmBW/cFBQURFyL19ralzGL48PTOz47VaBbwK1kgCH0xISxMrlMjrudxbSjJKAXCN75EI64Ze1bblP+X9xU5",
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
        svg = "eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0uVJGEI6q0tCqUbBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtHNx/X",
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
        svg = "eJxNzEEKgDAMBMCvhP2ABKp4SPoZLabXElB/byul9BICu7NS0uFkKV/mig1059NNsYPKo2DQqwig+q+IsrR6lB/VgMMMppVu2+XBPq4cHp8=",
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
        svg = "eJxtjt0KwjAMRl8l9L6xCevPoNu7DBQmTPHCi/n2NmndUEchgeR8Oc2P6TnDeTA3Spg6IEb2QBjZlkKzRceTR+c6qNXJs0hsI7qow9BqXQXsPbhF4iyHyIz5JJYxby6PFIHK0qtFsX8LFAscW6BZJGi/Jcv1foGVBpMMrKztVZovrQxZQEE+4L4SXgnl0w8oM+Z6kkJFpUuS+gN4g/oGhRrm/Qtvo29XdQ==",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[cfg(feature = "unlock")]
    #[strum(props(
        svg = "eJwli80KgCAQhF9l2Xs/W4QJ6ht07S4p6S1koXz71mIOM8N8Y0o8GKpFIoQU85n4z+WxOInVz6TMCHcOnGRe0ZmhHZ25PCcIFjcFRLvyCywwigh0rztqYEPcC9M9G3g=",
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
        svg = "eJxNyzEKgDAMBdCrfLoXTbToUDu7eAhBQUG0oEh7e9PqIBn+T3ixfrwWTJ0amEDmrkcGo5QhLa03/13zrWvlbJGenPXHFrd1n+GPdb/OTlGDFsSoICW7TzibXSAxrBD4zZjSSMq9Sj7bB+tMJk4=",
        categories = "arrows,files",
        tags = "file",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Upload,
    #[cfg(feature = "usb")]
    #[strum(props(
        svg = "eJxtjUEKwjAQRa/yyT5jZ5I0LSS9QS/gTqKgUEGKC729k1aoYBczs3jvz0/lNpfpgjkbNigvPY3edzbRDOmw0iF9rcr9ikW1JfRjPU7PK87ZjJ4iuCenC6EalWz8LgxnlUIgx38+9iQtIrUdAlgm2X3CDdirIHAUrM6e02kebFm7lr21fQBDWkHD",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[cfg(feature = "user_2")]
    #[strum(props(
        svg = "eJwlyjEKgDAMRuGr/GQXk4LSIckNPESJgoKDFAe9vZZOb3ifxlHj3FCNJkK8RvnPYySJXMd+Xa9y71iNlsRIUjIyGAIeZAY32IB/P6sUXg==",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    User2,
    #[cfg(feature = "user_check_2")]
    #[strum(props(
        svg = "eJw1ykEKgCAQRuGr/Mw+aiykwPEGHSIsSJASc5G3D4t4y/eZuOQdq9DMA3haNDS6WsMKHVnTVmCN88mFDe4WGglJaCC4IjRV8k1r4hlK8MeGePojX0KswQwewT2Uwot/ZB8D+yJA",
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
        svg = "eJxVyjEKgDAQRNGrDNuLmyDBIpsbeIiwCgoWEiz09rqk0DDFFP/FI58rZqHJjfCcAwLY1jkPphR7AynqVnRfoLeQY4Je73tCERoM1dyymn/6cw8wPR+9",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle2,
    #[cfg(feature = "user_circle")]
    #[strum(props(
        svg = "eJxdjMEKgCAQRH9l8V65ExiB+gddu4sFBh1CIurv0zoksYfHMm9G+yX6daZoBEtB/kxE4vXQ6ubNrf689qfJUtvcHmgyYugIslYKI/cOBJLpmFAhqPInHJy1vJG79gY57iVr",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle,
    #[cfg(feature = "user_cog_2")]
    #[strum(props(
        svg = "eJx9z1EKwjAMBuCrhLzvd2nWroVt4AE8hFRBQUGGD3p72xVxD53kKeTjTzLE6xxvZ4rvkcUyzSMrU3ylzvM07Mp4Gr4sDTwvOiy4W6PH8Xmh08gHaWFJFHbvyFG7lCEJGWf0o3cj6EkcugahgVaEWJgcFraFgyPx6KHJ1ECAkBiYP6AkNOkQqa/wOUE3QQelckgitUfb/KjmmDX4APRAZLc=",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserCog2,
    #[cfg(feature = "user_cog")]
    #[strum(props(
        svg = "eJx9z8EKAiEQBuBXGbw77Tiuq6B77tJDLBYUFMQSUW+fk4c8uOFB4f/4nYn5subrCfI7KRoV5Fe5vYI1KVZz3NV4jg2bvqmtNrTovjzOcEzqQAPQuHeLBQuDHF1eTyNWzE/eDOEE5NBqDBq5I2hEA8QSbwmHDsjjhFxMDwQkIIPmD6gNugxC/S+8NPAmsMhQBymkt+ggi7LUtOADNyplDQ==",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[cfg(feature = "user_minus_2")]
    #[strum(props(
        svg = "eJwlisEJgDAQBFtZ9i96QYJCLh1YhERBQUTER9K9OWUfw7ATrvnZsCgn6SHj7OHR2Rpx6BhDa0EMab/TseJW9kTKyqGiKEdL/jOGYz9XZKcUTxSjEFmUzlWXz2tuWXwBfgYfYg==",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserMinus2,
    #[cfg(feature = "user_minus")]
    #[strum(props(
        svg = "eJxNi7EKwzAQQ39FaDftHYdLwee5Sz8iuIEGQimhhPjvaydL0CAkPaXv8Hvj5XxKhMoadDAYrl3Bgj3iOcNWZU6XfsqpTEuZR5TNeSdKdd6IxWkdOcac5ukzoopThNiaqzbXliNRde8b3rH8B8rnI8M=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[cfg(feature = "user_plus_2")]
    #[strum(props(
        svg = "eJxNy1EKgzAQBNCrDPtf6iwhVMjmBj2EpEIFKUX8SG5voiKyH8swb8J/WL/4mLzpwH7w8OjaPajoJIZnAzGkaUnziJRNXoJUTHrBYuIaOcoY5uk3onAnRU3oBLlGVpt1/1U3dbPkiXkif4xUL7wB5qAqcg==",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserPlus2,
    #[cfg(feature = "user_plus")]
    #[strum(props(
        svg = "eJxNy9EKwjAMBdBfCXkvmhCqQtNnX/yIUQcOhsiQ0f69idUx8nAJ99z0Gt4PuCveKALTGngQEDj6BQlyjfsfZGXM6eCjnMq0lHmERVEQSlW8WDTFk5Ne5jRPzxEaK5KhRopnhGpBhit/07Srv/WSfmXEvqU+Yt7wB+9FLtM=",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[cfg(feature = "user_square_2")]
    #[strum(props(
        svg = "eJwli0EKgDAQA78S9i66VcRD2x/4CKnFFjxIKai/d9eSQ2AysddWE3ZHKy8wvM2YMWg6NhjI214Fb0Mu4YwojiZCeByxkX6lWaU2e1tiqEgxH6nKtBDEHAl33mtq4P1BEW70qAf/AT0eJAc=",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare2,
    #[cfg(feature = "user_square")]
    #[strum(props(
        svg = "eJxNy0EKwzAMBMCvLLqHRiq0Pdj+QR8RFFMZeijGpMnvIyeXoIuW3Qk1a8MW6U6oayQhrEewXD7WIvGL8C9zs+NN4dZBClqqfjPUJY8EdcRua7c+OusUflMzzJHeTwgvg0wCwejHkEHscc2QRbrtJu2H6Cgy",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare,
    #[cfg(feature = "user_x_2")]
    #[strum(props(
        svg = "eJxNy1EKgzAQBNCrDPNf6qZiK2Rzgx5CoqAgIuJHcnuziiD7NTtv/NrtI3rlX2pI2zVoUNm9xKFi8G8Dwcdpi/OAmJQ/ImZlS2zK2shVBj9Py4DklM4RWU6aS5QPkUqUr2lTD1uetzVzTq/NbQ+8MSpC",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserX2,
    #[cfg(feature = "user_x")]
    #[strum(props(
        svg = "eJxVy8EKwkAMBNBfCXNfNDFUhc2evfgRZS1YKCJFyvbv3bSHtuQQhnkTv+3vTS/DkxsSnoK0Skpnv6BBH80+k06CFE8+SjH3Yx46yrPhChoNCsrFcHeylikO/aejwgauZq7/BipiEKmxfr64drWzS7nZZXq0f8HILqM=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[cfg(feature = "user")]
    #[strum(props(
        svg = "eJxNizEKgDAQBL+yXB80x4EEcqltfEQ4BQULCRL09xJswlbLzMQr3ztWpcUHsK+Os0AwtjlxMof+QypTikOLUrSj2LnBHiXPBHuVJkJRkub8NH2NBRjF",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[cfg(feature = "users_2")]
    #[strum(props(
        svg = "eJxVjEEKgCAURK8y/H2kIqLwvzfoEGJBQYuQFnX7tECIWQzDPB4f6VwxC03aQofk4KBaBm2gKPLYgMh5K3lfUIQsIV9CvtYtFBrynZG7y5i/yw0OtkZBv9t38QNEPx/i",
        categories = "account",
        tags = "group,people",
        contributors = "karsa-mistmere"
    ))]
    Users2,
    #[cfg(feature = "users")]
    #[strum(props(
        svg = "eJxljsEKgCAMhl9l7K7lXFmgnrv0EGFBQYeIiHr7ksCCGGOMffv47dJtI/QOW1UCqV1Qx8CQxxIsuCm/O/BO6G0Wn7wN0xrmAVaHjBBOh+Yeh8M6Is/R2+Qn+vm10LIyr/CbRUulE6ruNtIUCb0Adv8tjA==",
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
        svg = "eJxly0EKgCAQheGrPNxLjFokqDfoEEFBQURgi7x9KpqLFo+fgW/MNd8bFsumEYI8V1zGaZQxZ7oknPkcDZBeQUJB87LGjv1cEcgyzfCInBBDfTwpN9KE/rSa9lLpC2d2KVg=",
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
        svg = "eJxti7EOwjAMRH/llN0lNkmUIc0fsLKjgBQkQAyoKn+PTYXKUHl4uvO78jy9Os6juwsyBEIC+dHVsrN/Laul2r+CTe2QkSf2zWOIcQgRbNdT0wgPpm9HPNHWmBPYD/twTM2TLswmtusUtF8nt+vjgplHJw7vBbMoLC5U1aT6AXadOD0=",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[cfg(feature = "vibrate")]
    #[strum(props(
        svg = "eJxVy8EKgCAQBNBfGfYe4VLgQf2XSGk9BCEL1d+nQVSnZWfeuG1SQfS0MiwY3DH4uRRc3/rgXlXZl+DHSpoVkvIi6skMhMOTJZyeRsKeo8r9lxqbtmo+XG92Icw=",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[cfg(feature = "video_off")]
    #[strum(props(
        svg = "eJxNjLEKwzAMRH/l8O7UuhrhwfGcoV07dAu0kIIpHUpI/74KCThoeCe4d/kzfic8eneV0KlCB4kjQQQ7MXJmd45VIBcSaU6u5NMqldxUhWizvKUhHn/PWzquek6sYjHc21x9vZ9YpHd0+O2gwbhstOpaKn8KmCqf",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[cfg(feature = "video")]
    #[strum(props(
        svg = "eJwdi8EKgCAQRH9l2HtEIuJB/YwO3SKl9RCELFR/3+phmIF5L9y7MHKkyxj4ycFCs/qNUpj7l0Irh+CNZAjtG8WlniyRFt1PzcI6LUFPp8xA1e5e+gHwCRot",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[cfg(feature = "videotape")]
    #[strum(props(
        svg = "eJxljssJwzAQRFsZpoFEIgQdJHWQIsJaZAU5BCGI3b3XH7DB13n7HhtbkY4p8UGMiZ7QUj/aE92T+Nehq613oq00x9si5Ph7d8WQ+PIIatzAMuUotcm3QOw8EGJlZ+m2yxs96QHOa7jqZ2+N2TuHPwN7jDO6",
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
        svg = "eJxNjDEKwCAMRa8ScoESKaVDkssEB0E6OMXbN2qFTm9473+20qxmsC5ICcFc8EJogicqH8sq7yos3Z/em19Wy5PBaV70AA2mRU9zG/Wo9AUg4SEA",
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
        svg = "eJxti0EKgDAMBL+y5AOSQIVC29+IFEpb0EP7exPRg+AhO2SZDb2VubeK3nI9j0jMcFjhIfexPRpas7d0lMLyjFIouW4YorOVMI1OyZE8YShETDfrdfnflY97AXn6Jiw=",
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
        svg = "eJxVjc0KwjAQhF9l2HvX7qbRFpKevfgQJS2m4EFKwPr2ZhVEWdifmfnYsC2p4BnJEbY9khLysl5ziSQ94bHOJX/W3TJjOBgwhvtUMuZIF4dhUijaWgJtNEv3K0ANsvgfJJJd4t4yfGQHZeHhJtZTVUxFx/Kenk9ozWuqyb5hX2VXD6PqOKt8f7wAdVExsA==",
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
        svg = "eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2pghtD2VZhpnv8IV0H9PjhvSJ4gVjFArSN0onfTj8YR9ewzvjGuXZQVs0Vk35gSBcOQWtV+v0RELrS70QZ2h4rLYdnNStQ0lmbbaD4ayelav47EHNfgdoUbZTtZAfsKAytQ==",
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
        svg = "eJxtjjELwjAQhf/KI3tick1DhiTg5uLawa3YYgodRALqvzeHUlsoN9zde9/xLtz7kjFEcSaCV03bke4JBF3LyDqd7HqX1DF1XCQ0ihycamcv6/hnYZT10LNnYsX/ci4ihQOHp7C84GB8NrTv2I3zGK8Fz2koOYqq4xWFE8jjdMvlq7xr03zAaPoAlcQ30w==",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[cfg(feature = "watch")]
    #[strum(props(
        svg = "eJxNjtsOgyAMhl+l4Z5O6la4UN/FMBNImBrnxXz7FYwboaHJf/ig83HzaYKtV6zAf3plSPZR9tDdTnvo1iUdKc4TrEuc93e2wTSQb5lWJqevlOTHPcCzVy/DKLZFZo3O6Ds2j5GAoMlHkzbIJmhCdrUMWU4arYPcKGwhVlyLzoHAW0bJ/LtwdgOhpVotD6XrCz/gF203Png=",
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
        svg = "eJxNykEKgDAMRNGrhOxFGwVdJL2Bh5AoVHAhRURvb6RYuxqY91nXqNsCUXBA0FvQNbaXLaHnOrHnP2szf3mR7dMRYBYceyAKyd6vEEdGZ9VlegCTmST6",
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
        svg = "eJx1kstOwzAQRX9llL0Hz/iRWGr7BemWRXdRQApSQCwQav+eOwmElyPL49f19fjYh9fhbaKHY/OspErinfjmdLiz6dNhW5RMnROWliz8F5wDR6wpp0CJRGaxnoUhcCKrHkVQI5fYQ1NmZ+uuJnImulTy6JAG9H+3+C/fQpLmzXbklCBP3FnymrhYW2r5i+cilFjzyIJNmjlE7IVNdtyGXgKF3Vuxdjg/a8VYPemk7zpEiqvaoTc5vc/fUxRd7bpnkQVru8SwMauksBAjv4Ld47pqLvX3lVwjuxn/JgtNLLSE1dqwOvCtXaLjNiJ99mUEVwe+4OrAF1xxZu5V7MvsfAf7DFnxIj+856eXR7rpsVFt6CZoG7p+Dq/LEFITnT4AxBCl9A==",
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
        svg = "eJx1zM0KwjAQBOBXGfaemkmtUUhy9uJDyCooeJAiwb59E/p36mVn2eHboO9eP0/0UVqBDlHoSv6jeEnhMLUpfO+/Fx5RbrS45FOt6imFmS9wflM4d/wRPp83vxbOgT5TLZrONB1oCF5brbs1LFEGs+FqR1nNMqQ=",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[cfg(feature = "wifi_off")]
    #[strum(props(
        svg = "eJxtkEEKwjAQRa8ydO83M8lMI7S9gYcIKiiIuHBhb++ktlKhZPEhvM8fXne/PS40ct9IQ6N4eL6XnL6Hbl+hoXuW15XOfXPMUGKDFiWl4I+ppVDBiqxAoYwshR1fwARudwLTDZwDzEhPCYF3iOZtTjgQM2KiiNa2SoasFREt7AthHhKIEMPyRsf5+AcrJPlV64FJTDXA1Uw1Eb5mWPy8WVb42fkA4OpOWA==",
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
        svg = "eJxtT8sKwkAM/JWh98ZNso8s1IJ49iNEhRVUPIjYv3fbqngoITN5TELS3fePguO62RlEijV9txpLffdrJLArek1wpWVSrwsaFnB4pqVpUpcw4YaFVDGjm2zcvQ8I35Q0M/yIO6Oo9SbKxlujFGoYo8HIktRYPTKkxANTgIcgVre6g7xLLblA5nJL7AOYOP89djnfThh43UiD14ekUuVh5iodRf0b0YNCoQ==",
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
        svg = "eJxNjUsKgDAMRK8ScgFtLeii7Q08RLHFdCel+Lm9TUSULDIwjze2pKXC6XBAoJRXqg4nhCPHSpJK6zTCxYS3HePebqESRIfzCErtJmjQ0Mu1RIZBRrwV+yf7LTStGsTL71l5/TcqkCjq",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[cfg(feature = "wrap_text")]
    #[strum(props(
        svg = "eJw1jtEKgzAMRX/lkvex3tYVH9r+wT5C2KCCqDAf9O+XWCWBm8AhOWka5y92n8VTsDNLEBy6Rg1alPQ0pqR12Co+Wd4B9JWvISDAgVoOsT46Qw1SdJmO8/C6jPP2y8II6w7sbfDuZC+qpCbRnpsLXfvOvslo3hp/F/0ugA==",
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
        svg = "eJxNyUEKgCAQRuGr/Mw+yiBhYPQuMQUFBSIu9PYqblw9eJ/oG/W7odmR2QlaRmPLRl7W4V7CmR5cjn5zgBcL27HPiRiMWSrJlRnP",
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
        svg = "eJxNy0EKgCAQheGrPGYfYZEkqHeJlMZFEDKQ3T515eqH9/Fsjqfgc7QScs1SU3o4povFkdoJpfubgnAfvJ3bz9vnEEZwdKsNZtLQjdo4kIHBKD89Zh/V",
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
        svg = "eJyFyzEOgzAMheGrPPkAbm2SEKTAbTogIUBqh+b2fY3EgBhY/Dz8X9m3pS7z+sK+zevnPYq5BkPSPsI6OOypsUfQwWUqjyOfyhXmf0g+GNzoYFFT4nMDMzI6WKBtl5wPYTrDhqqP4i74HmtcQW3DvKU/S946gg==",
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
        svg = "eJxdjDEKwCAMRa8ScoCWSCsO6mXEQZAOTnr7xgRK6PST/PcSSxulVxgJA0JZCYk4p2SOp9Y59vZUmI7P/vA3wjIjJXQsTU2WNmwVUigoI+s+Xz922XI/VEik8LEvQAAtdQ==",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[cfg(feature = "zoom_out")]
    #[strum(props(
        svg = "eJw9jDEKwCAQBL9y7AMSThKxUD9zWAiSwkp/H+XUaouZHS+5SkkkPYAZJE23BjhEfyuOvuQvUTcD2su+oMYBZnh9bTtonKa8L6y96bulPdASH/cHXFIidQ==",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomOut,
}
