
use base64::*;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};

#[derive(EnumIter, EnumProperty, PartialEq, Eq, Debug, Clone)]
pub enum LucideIcon {
    #[strum(props(
        svg = "eJxtzVEKwjAMxvGrfOS90axNWqHdDTzEqIKCggwf9PZ282GDjTyE8P9Bcr2P9XFF/RQSI9RvoUAY20V9Pvxrn1/D+4ZLoackyAniojPMYiqrrkjwzkNZ2+7YGzzrFp4DdwESWAeF4jiPcUqwHSyeY/sX19pN2i36B+E3Mq0=",
        categories = "accessibility,medical",
        tags = "disability,disabled,dda,wheelchair",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Accessibility,
    #[strum(props(
        svg = "eJwdizEKgDAMRa8SshdNRHRoO7t4CLHFFBykBNTba7p8Po/3fM27whtwQHjaSi6HaECaEeqPGOEuSaWR6DsLor82FUgBV5qAWByfjmF07Ki3s0ymmhQ/O9caRA==",
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
        svg = "eJxtzUEKgzAQheGrPGaftDOTJikk3sBDlLTQQgURF3p7DYIudDXwvh8mld9Q/h+UORMroUzrFcKQKVKTbhs3qX+NX7wztQ8oBL5a3Q7pZJ2NGj1T661GcLQBDsIXAQfrXS18gNwvm+4Jrq8FzridF6nOMts=",
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
        svg = "eJxti0EKgzAQRa/ymb1pZyZNUki8QQ9R0kILCiIiensT3Ai6+vDe+zH/x9x9MSYKhLwkYim7llVq423XbRze0w+fRC8WPGfbCaTqig/yAYXAnU0vBTfa6MXJGQ3gYDwshC8C9sbZWjgPuR+bDZNTMvs=",
        categories = "devices,notifications,time",
        tags = "morning",
        contributors = "lscheibel,johnletey,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmClock,
    #[strum(props(
        svg = "eJxtjUEKhDAQBL/SzN3szoybZCHxBz5iyQoKCiIe9PcmePCgp4aqgg5pWNLYYYnkCWmPxJp3yyvUhNepmzD/1h7/SO0HCoEtrrDLTJJxpZXeVWuNerA3DjWEHwJ2xtalsA7yfm6+YO2v4wOB2DIA",
        categories = "devices,notifications,time",
        tags = "remove",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmMinus,
    #[strum(props(
        svg = "eJx1zUEKgzAQheGrPGZv6szYJIXEG+QQJS1YaKGIiN5egwsX6nLe98OE/Onz9408R2Il9JE8IU/rJdSG28Zt+D+HDq9I6Q6FwBYr2y4/WedKKz1SskY92BuHBsInATtjm1JYB6kvGgHX48nn9ABrt8MC+K85Xw==",
        categories = "devices,notifications,time",
        tags = "add",
        contributors = "mittalyashu,ericfennis,jguddas,karsa-mistmere"
    ))]
    AlarmPlus,
    #[strum(props(
        svg = "eJw1S20KgCAUu8rwAmEG+UO9TEkKomJCefueD4KxD7aZ5o+OJ549WCG1wGuFEhjMjcJKMliCj1fovHJmmT9nakkjxexRS8z9plJCgWhig4bc2e5Q8/TP3QfaPyD1",
        categories = "photography,multimedia",
        tags = "photo,book",
        contributors = "ericfennis,csandman,mittalyashu"
    ))]
    Album,
    #[strum(props(
        svg = "eJxNyzEKwCAMheGrSA7QGofSIfUywUGQDk7m9tU+Ak4/5H0RrV1bCToe4kRBDe0zkbKc2LO0+pZgCevwMmqz99JL7fZye0QG+094c/8BH90h/Q==",
        categories = "notifications,shapes",
        tags = "warning,alert,danger,exclamation mark",
        contributors = "colebemis,ericfennis"
    ))]
    AlertCircle,
    #[strum(props(
        svg = "eJxFjEsKwCAMRK8ScgDbhGJdqLcpRRAV2oXevtYProZ5eRmdoi93DJCiC+9j8BRKAgNJQUdNZuhkonFYeLmNdIxWb2PZau/CBZkMEiOUmgoh86g9q/5b021Q7NR1krjeufXpfzX4LeU=",
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
        svg = "eJxty0EKwCAMRNGryFygJKXVQvQ2XRSKa7290YgrV0PC+/J/+XWFI064ShG3Do8pejEhydFNEpP6JI9ReCuILdHd2MfsNU2YTVi2AWlfILY=",
        categories = "text",
        tags = "text,alignment,center",
        contributors = "colebemis,ericfennis"
    ))]
    AlignCenter,
    #[strum(props(
        svg = "eJxljEEKwCAQA7+y5APFRQqC67mXPqJUqd6KCLW/b9erp4QkE1/T2agLLOgpsWXBCsqpXLkJzO9fAYNqVwl+0X3wg5r3Dnpl7KDcRN1HyxQFOzMxb6PRLHwkWCXW",
        categories = "layout",
        tags = "items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndHorizontal,
    #[strum(props(
        svg = "eJxdjEEKgCAURK/ymQtEHwkEvzdo2z5S0l3Ih+r2pSBBu2HmvXElbkqXgEFnDpoE4wRKMe9JBW+8BQZUGuLdUHnvumX70mX7d0fzWceqiYJgZibmpf3Vzj8UHCXk",
        categories = "layout",
        tags = "items,right,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignEndVertical,
    #[strum(props(
        svg = "eJxtjDsOgCAQRK+ymd6oKNKw3sDW3qgRO2OIn9sLUogJzSY7b97ofR4tnetkDaMBmXldjGWUNehiuHszJGh3j0Crc99v9WtdoRbQZxaI9pytPm0brKGJ0ZWKhDgy6ZEP/0j1IgGCUiWJjIwHzG47uA==",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeCenter,
    #[strum(props(
        svg = "eJxtjE0KgCAUhK/ymAtk0s/G5w06RKSkuxApu33qIiRaDcw336hgt0iJMYBCDgm6vImOMYGc9buLjD7DmzFCq67stapWruZmJH4eUpVf7VijI8NYekHylKKQ0jVEfsgDeKgtgQ==",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeEnd,
    #[strum(props(
        svg = "eJx9i8ENgCAQBFu5bAMqQf1wdGARRojwM+Si2L3CR+PD1yYzOyb5Rehk9KDM0KB0jwId0UlgDKDg4xqE0WlY05S/NbXKlZV4/MvaJ9tmCeQYkya1qyoKeonuYy4zVS1X",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "r4lv,ericfennis"
    ))]
    AlignHorizontalDistributeStart,
    #[strum(props(
        svg = "eJxNjEsOgCAQQ68y6QUU4mfDcAMPYYQIO0Mmfm4vkBhZNe1ra5LfhK7oJDAmUPBxD8JQA+hhjKCboUGpijVd6VtTV5nPH2kuslftU//PjlUCOcaiNOlTV1Iy+wI7SSYo",
        categories = "layout",
        tags = "center,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyCenter,
    #[strum(props(
        svg = "eJxFjEkOgCAQBL8y6Q8oE5cLww98hBEi3AyZuPxeITFcu6ra5rAp3QIGXclrFEygRzCCYkh7VIEZQLkqznbFd7ZWzf+g4V9qXV+f5pYdq0bygoWZ+OS+kLK5FzcfJiU=",
        categories = "layout",
        tags = "right,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyEnd,
    #[strum(props(
        svg = "eJxFjMsNgCAQRFvZTAMCUbwAHViEESLcDCF+upflAJfNzueNyeEo9FksoPxaKFC9GvQkX2L7YkhnLBZyhjMT951pVA8EeGHtC4OtWuqBXXuJ5C02RepWggO23A8MWyX7",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalJustifyStart,
    #[strum(props(
        svg = "eJxVyjEKgDAMRuGrhP8C1iCK0PQGru5ii+kmJaDeXrvZ9X3Pl7QbPYIJdOVoKhhB5RYwSFM+1AS9A31lRvBd/YM/N1OKgmUg5pUr1PQDdq28McAdng==",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceAround,
    #[strum(props(
        svg = "eJxVi8sNgCAUBFt52QYU/F14dmARRohwM4T46V7gIHjZw86M8mYL9DAG0OV0sIwR5G+GBFnjdhsYogfFp8OsmuTPKlfFj/30VXHFUMVtyY41WNKMpSN5ygzSVQEp/uQFUYwtVA==",
        categories = "layout",
        tags = "around,items,bottom,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignHorizontalSpaceBetween,
    #[strum(props(
        svg = "eJxVjUEKwCAQA78i+UDZFEoPq7/poVB6tr+vRkQ9hSGzG3/u9wqZETSEzyKOElTkQjuSb9VJLrNWxt7pQNgerO7snBgjFHf3B2WSIKQ=",
        categories = "text",
        tags = "text,alignment,justified,menu,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignJustify,
    #[strum(props(
        svg = "eJxdjDsOgDAMQ68S5QIoQXyGtLdhQELM5fbE6WfoZNl6z/bc70VFEqswFU28Mn1edw9FZFvAZAsSm+ggIcpWDZ8nFuPZoKP+RQ+3sz9mYCCs",
        categories = "text",
        tags = "text,alignment,left,list",
        contributors = "colebemis,ericfennis"
    ))]
    AlignLeft,
    #[strum(props(
        svg = "eJxty8EJwCAMheFVJAuUpNBaSN3GgyCedXsTFUXw9EPyPY4heZPxB0IwRfpIqCVLbnB8qXHcpL6Q+u+DbSjng7UL6eYd1k5bAWdcIK4=",
        categories = "text",
        tags = "text,alignment,right",
        contributors = "colebemis,ericfennis"
    ))]
    AlignRight,
    #[strum(props(
        svg = "eJxFjEEKgCAQRa8yzAXCQYLAcd2mQ0RK4y5koLp9KonL//5/3+V4KNwpqDDOCPlhJASJ6RRlNAW9rSjcondT3XvXrH875JKN7UK/WIZ17SoQGDcioJVqUZH/APi+JaU=",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartHorizontal,
    #[strum(props(
        svg = "eJx1i0sKgCAQhq8y/BcoRYLA8QYdIlIadyFD2e3DgqBF2+/hS1qUJOVVlDGAjhxVGCOoVIYF1RufDOMQfNf64H8u85TuO7/XNqtQZEyW7G77JhoKFxL4JdE=",
        categories = "layout",
        tags = "left,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignStartVertical,
    #[strum(props(
        svg = "eJxtjDEOgCAQBL9yud4oKNKAtY2PMGo8O0NI1N97kIgUVJvs7Kxx2+LhOlZPFkWHQNuxk7fYI9wWFYLjkAhPxIOpgzCYqGVbxl3afndN/NC/dc6eYLU4SQmaKhVI6DKiQY+i0LMhWGkLSDEZZQIv0307fw==",
        categories = "layout",
        tags = "items,flex,justify,space,evenly,around",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeCenter,
    #[strum(props(
        svg = "eJxljMsNgCAQRFvZTAMi8XMBOrAII8TlZsgmaveCHpR4m8ybNyaFRYhDXFksBtBh0YNOi7YD7dELPzFloOFMUwRnbi1X4ztS+P7kh7+1zcLkLSZNWrFWhZSuIm1FLngWLWU=",
        categories = "layout",
        tags = "bottom,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeEnd,
    #[strum(props(
        svg = "eJxVzMENgCAMheFVmi4gENQLsIFDGCGWmyFN1O0FNEGu7+9Xk8LGcFuUGoFC3IktTgjpsqgQzuiZ3piHEZ0ZCnCmsu/ox/Ij3ZSoam7qWJnAW1wUSE1KlFK2rnThAUuDLTs=",
        categories = "layout",
        tags = "top,items,flex,justify",
        contributors = "ericfennis"
    ))]
    AlignVerticalDistributeStart,
    #[strum(props(
        svg = "eJxVjEsKgDAMRK8S5gLa4mfT9gYeQmwx3UkpqLc3EVHcJDDz3riSlkqnhxlAh0cPKvIsiFNeuXpIvudYWZAOwTUqBHdrDyl3/KAWOvcbeK1trkzRY7JkLNtWG83CBRnBJho=",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyCenter,
    #[strum(props(
        svg = "eJxNjMsNgCAQRFvZTAPqxs8F6MAijBCXmyEkSveyHsTLHGbeG5PCnklCPCRbzKAr+iwWwwhKtwWDak6gUjuGM50KzrxaA5Ym9vjfFSU+69yykLdYmZiFe120cw8+PyYX",
        categories = "layout",
        tags = "bottom,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyEnd,
    #[strum(props(
        svg = "eJxVzEEKgCAQheGrDO8CmZRt1Bt0iEhp3IUMVLdPDYq27+d7NsdVKJ8OGnSkIOzQDyCOaWNxMKDSRtBVdgNvuwq8bayUqRXzWfWzz/Gr9kWYgsOsSbNWNdTJ3w0OJe0=",
        categories = "layout",
        tags = "top,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalJustifyStart,
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrLHMB4yKKkE1t4yHEBDedhAX19prOVAPzvi9pN7oFE6h8w6ArR1NB70Ca8qEmGEGPYEbwXe2DPzdTioKVmdgtXKV+rQw/eAEnth2C",
        categories = "layout",
        tags = "center,items,flex,justify,distribute,between",
        contributors = "ericfennis"
    ))]
    AlignVerticalSpaceAround,
    #[strum(props(
        svg = "eJxVi8sNgCAQBVvZvAYU8HMBOrAII8TlZgiJ2r2gicppszNvdPRLoj24xAaiA7EPKyeDAXQY9KAz83xi/iSsbkpg9Z1lNOKN2yp+9qVWX7XNickZTJKkYNkWU1hl1F9cVgstOA==",
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
        svg = "eJwljLEKwCAMRH8lZC+tKUIH49ylHyFWUJBSpIP+faOSkHB3jzM+FZ8D+MqoCME3Ro1QGHe0Zp2pNTk9AaqaTCPGQ55IEllp2EJ3yprXfRFuxkuDopOc2kB2DsmJy6jumP0Bg+Igrg==",
        categories = "transportation,text,maps",
        tags = "ship",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Anchor,
    #[strum(props(
        svg = "eJxtjEEKgCAURK8yuLf8kpVg3aBDhAUFBWEt6vZpbopcDLN4b8bY2dllhL0aRpLBnrGdL8Fak0femq0/JgwN66gElTunTHHJixA8CXKQXmqVKdQgAf2HKxXQkP6FEkvtV5NIAFJfcgNEIjN7",
        categories = "emoji",
        tags = "emoji,anger,face,emotion",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Angry,
    #[strum(props(
        svg = "eJxtyUEKgCAQQNGrDHOBGikwGL1Bh4gpmKBFiAu9vYorwdWH/1jeIN8Dkh2SQZDUG2pW9Lx09/xfUeF2eFqgXW2jtgY41Ew+bQMUShkgiQ==",
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
        svg = "eJxtj0EOwyAMBL+C/AAXhwSMRPMZlEOkqIecyO9rg0Bt2pPN2rOsU97PfGwmlyfQBOaUYsHkqz7X9GjzNR37azOFRJ7REZhLWpYyiRIwzmCKtJNFuyim6x1SnZDCgNQmoo8N5/u+TAM6bo7kUHMpWauaofU3RmWvcmUcxuXrF4X/ISL2YYuJ7ODjyt9g5NH3GDTOJovMI2Sn3rOiVgw=",
        categories = "photography",
        tags = "camera,photo",
        contributors = "colebemis,ericfennis"
    ))]
    Aperture,
    #[strum(props(
        svg = "eJxljMsNgCAQRFvZTAMCIcQD0IFFGCEuN0M2froXPGm8TebNG1/zInQGGNBRknBLCnQFWBDnsrIEaAeqzyb6oQvRb7MwpYBJK7K77aBXL2Bo5Hb1B+4j3E3vJM4=",
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
        svg = "eJxtzFEKgzAQBNCrDPMfmt1UqJB4gx5CqnT9KBQJtr19Ez9EQeZn2XlMnMdHxmcasiWqJ+ZvohA2Tk/LiQ3xSwxEeSu7eKm+i+8+G4bE+xW3RaRXKPyactkKK9lB9Ucprkp3Ql8tpEFwASUnS6IQXdqt+QMHBDOm",
        categories = "files,mail",
        tags = "unarchive,index,backup,box,storage,records",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ArchiveRestore,
    #[strum(props(
        svg = "eJx1i7sOglAQRH9lMj3Krm7E5C5/YGtPhLgUJobc+Ph7uTbQkGkmM+ekabhlTB+nEF/ngXiPfQ6n1kQM4z2y04iZULZpX/g2Pbsc6J2XI5qXSKdQ1P/MLUTXQ6XXppjFWczHeWeQE6yyrVNhWM4fDNQsUw==",
        categories = "files,mail",
        tags = "index,backup,box,storage,records,junk",
        contributors = "danielbayley"
    ))]
    ArchiveX,
    #[strum(props(
        svg = "eJxNiz0KhUAQg68S0j/ezqhgsesNbO1FxbETWfy5vbsWIoEQknx+m4YIm5bZYmBFbGegEMmVOJYxWkqOuAILNv6f/41f+2gYA9sS9S7SKxTuUUom+i1+2tWZzMyHFAdRK9/lBhI3JCA=",
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
        svg = "eJxtjTsOgzAQRK8ymt5K7Bin8XKDtOmjgLIUkRCy+NwetkBQoOnmzSf3n6JohP8HfEI0ucg63wzUecevJ8L9fQJD+y3QtvtpESZi6pqiwkjMQl8RwyIMxGLe1rL8ac7b3uiSunBx5quN6vG2AvhDLhQ=",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowDown01,
    #[strum(props(
        svg = "eJxtyzEKgDAUg+GrhOyirVWXtjdwdRcVn4MgUlBvbysIDpIhw8dvtz4IRse1hKph0jJDb/ME3r7cNtBF9weqgYoimf7DKqJ8sn0aAmRaZgmONXE6qoq44hlij6+JYxmDOD5ZCvwN2Ict9A==",
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
        svg = "eJxFyUEKgDAMRNGrDNmLtnSRRdobeAiJgoKCFBG9vQ0Fyyw+zBPdsu4L9InkPEHf2lwyUJK+epJzulbMkUbnwTcb2dXgYBQKti78/AEfvxp6",
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
        svg = "eJxFy7EKgDAMRdFfCdlFqlAyNJ1dXN3FFtNBkBKw/r22S8f3DtfleChITKcooyGEl3FGyIVxQihtPCmoNPVurIF3964CgfEyFmggaFTPTutPxi60dfsAvI4euQ==",
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
        svg = "eJxFyzsKgDAQBNCrDHsBiYJskaS28RBigptCkLD4ub1JmjAwxTzG5rgrXkcT4WstMR2ijgwTcoGR8KSg0hZvh3rw9tpUEBydjJYqdeuymhl889LpB3fHHm4=",
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
        svg = "eJxFyzEKgDAMheGrhOwi1g4Zmt7AQ4gtpoMgJWi9vQZBedv/8ULNi4LksooyDoRwMY4IZ0kqb6iN0SE06zH0dohhn1UgMU6DAzrIwNIPG8FD3tb5j2/p6R7C",
        categories = "arrows,navigation,shapes,gaming",
        tags = "backwards,reverse,direction,south,sign,keyboard,button",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    ArrowDownSquare,
    #[strum(props(
        svg = "eJxFylEKgDAMA9CrhP4PaRHGYOsNPIRUQUFBhoje3g2Rka/kJR7juWBKNLBALu5JY1c3jb/sHBCch3cljW3Nts2wOxELwZ5EwoRcaj19rC8OMxqf",
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
        svg = "eJxFyTEKgDAQRNGrDNuL7iKSYpPaxkPIKigoSLDQ22sMGKb4ME9tjbbNsMsTC8Hu3PimoaB19qDHeC6YPA3cgaV3ydJXZGeBq1p8+/kBL3gaew==",
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
        svg = "eJxFy7EKgDAMBNBfCdlFUkU6NM4ufoTYYjoIUgLWv5coKLccvLtQ0qpwMXYIkvImykge4cxR5a2lMjqEapsxtHYYw7GoQGTcyYFvenhibPDzTAOQm/wnN+lDHsM=",
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
        svg = "eJxFyUEKgDAMBMCvLLmLJoj0kPYHPkKiUEFBigf9vbaCZQ/L7qitybYFdnliIaS3OoLdZQZtPw96TGfE7Gl0YIkuU74q7CzgAX1T8vsDNv8atQ==",
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
        svg = "eJxFi0EKgDAQA78S9i6yVaSH1h/4CLHF7UGQsqD+XltBCYQkQ1yOi+JIQcUTW4LEtIq++fTUEa7q+SmGRteWw+j2WQXB02TBRmwBZfrBxgY8oG+qPn4DCaQe/Q==",
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
        svg = "eJxtjjsOgzAQRK8ymt5K7BiTwssNcogooCwFEkIWn9tjFwgKtN28nU8cv0nRCocX3vDGIx+b+Ch6Ew/6qeFnG04wdb+EaRM6YunbpEJPaNf/NQkDsQptRWxFz67yf4mzNdxzNkGNuymzVaZ6ztgB5DouCg==",
        categories = "text,layout,arrows",
        tags = "filter,sort,ascending,numerical",
        contributors = "karsa-mistmere"
    ))]
    ArrowUp01,
    #[strum(props(
        svg = "eJxtzDsKgEAMBNCrDOlFV+On2N0b2NqLirEQRBY/tzcKgoWkCfOYsUsbBL2jOUMFjhh65G18596+WpfgzRQ/YEqYpGGJ0j/MFeWztw5dgAzTKMFRQTgdGSbsUx/EkX6HBjlhVXgG74K/ANOvLeo=",
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
        svg = "eJxFikEKgEAMA78SehfZBV2Etj/w6l2qoKAgiwf9vVURCSSQGd76fcIgtFZokIoEDymX96/80TZEhLqLP7E52zLCTqEYCFnI2w6fR3qxXtrnGjI=",
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
        svg = "eJxNyzEKgDAMRuGr/OQCUoWSoe0NXN3FFtNNSsB6ew0Our6PF1rZFFLqLhrJMaH1SCPhijQRzppV3t4tpDDYkMKxqiBHmhnOLyxsZPFHzj8Gxmc3ygIerA==",
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
        svg = "eJxFyzsKgDAQBNCrDNuLREG2SHIDDyEmuCkECYuf2+tikWYY5jG+5lVR70AD4SpJJZBjguSyif79w5HwWEbf2yH6Y1FBCjQzWPhkE9ua7Aw3gbtGL5mmHps=",
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
        svg = "eJxFyzEKgDAMheGrhOwiqaV0aHsDV3exxXQQpASst5cgKG/7P15oZRPgUneWiOQRWo9oEO6IE8JVs/Dbu4YURj2kcK7CkCMe5IDMYHVg1VV+n8kAucV/8gANQh8a",
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
        svg = "eJwli0EKgCAQRa/ymb00kyYu1Bt0CLGgoEVISN2+NP7i8Xk8n/eSjxUlkCHkO5CMH5/O6IdfR3+ma8MSaBYLV6ekocF9FlyVJGFIuwJWBq61rYkv9+oYhQ==",
        categories = "text,account",
        tags = "mention,at,email,message,@",
        contributors = "colebemis,csandman,ericfennis,danielbayley"
    ))]
    AtSign,
    #[strum(props(
        svg = "eJxNjzsOwjAQRK+ycr9LvHESIiW5ARegQwYJJAoUUcDt8QyKoRl/Vs/zPOXbmu8Xye85RA+yliVIfvG0TLvveJkep+dVznM4eGMuiOzWJC3RWuM6WNtrsk5jtBGbhHAdbT9oj+M2AJIEnBZQAEq5FsyxSQgXgAKwDo7wgcefTexsEERmG3G2Ea8+bKMvPSG8vUpN9lKTvVp9AAqFAcJ3+8bP5gPmEUWl",
        categories = "science",
        tags = "atomic,nuclear,physics,particle,element,molecule",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    Atom,
    #[strum(props(
        svg = "eJwly0EKgCAQBdCrfGavMWNmgnqDDhFTUOAipEXdPqT14yU9m9Yd+mRiIbRME0HfTDOVNPxa0rXeB7ZMC3s7hgAWO0dwgEg13jjj4cDWizPRMvfaS/kAxUYYCg==",
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
        svg = "eJxtzsEKwyAMBuBXCd7NjLPRg+0b7AV2K2ywwRg99NC+fQ0G20IR1Px8ieZpnD/w6s3jjqmDhOzHAAFcWQQBY7Jli7BnjFGuZ6Rby1xlLdAhJ2QL6Y6T7NVzThg/zZBv8tUh/77/Nyy+N+QNLFTPVeu11EmsKLWSEe+29qIjbePmNx3uOIM=",
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
        svg = "eJxtjkkKAjEQRa/yyT5lhs4E6dzAC7gLKiiIuHDR3t6UhrQNElJUPV4N+VGfF5xmsbcUHSJ5UydMUO1pTBSibCFgZZ4Cp1uph8HUVxugD9lIsinud5L8t06x5g+i5B2fWvI4OJFKSNXCdtVRtNBHBSMb5L923a73MxYzC21IaYEXp0Fg0YxarT9189ksbw+eOd8=",
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
        svg = "eJxtzsEKwjAMBuBX+cm9sa1d20O3N/AFvA0UFEQ8eJhvb2K7ucEoJM3PR9ryGt83XHo6HTl3yBz9GBBg5TgETtlISfhnkZNet6iVJbOVLUFbskFGSLfeZPaes8rimYZy0K8O5XF/XjG5npwnfHzrOkfCVGfBymb8C9m66nPl0tqWWX8B4zI4VA==",
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
        svg = "eJxtjksKhDAQRK9S9D6ZJBOTLJLcYC4wO2EGFERcuNDb21HxA9JQ1RSP6o5DPTb4Jfq8ZagQpDO1hYXi0bDSB8HicWZO+rLeoV2OTG3YEewlN0gwUl2bxNM5VTD3pRxf5dUcu7b/YzKJtCNMOlEgzGzasJvVmS1UXgCXeCzF",
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
        svg = "eJxtzsEOgyAMBuBXabjDgGHhgL7BXmA3ky3ZEmM8eJC3txVETQxJC38+CnHq5x98WvF6qtBAUGh7Bw40LQNO+SCpeDgyVJ63V1RKzXRmNShDLkgSac6T5N1zmhm+RRcf/NUuDv/xC8m2wqCAxVC31G3uic6BLavdFpOKWTaTr2C1K0MIN8k=",
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
        svg = "eJxtzk0KwyAQBeCrPGavVetfQXODXqC7QAstlNJFF/b2cRJJIgRhlMfnzKTv+Hvinul6ltEhSm9GCwtVj4aVIYpaArbMy8DPHrWyZmpha9CadEhU4vadxNE4xczfaEgnXnVI79fngWIyXQhFZ9KO8Dft1hxXyqjRJZt/7Gnp6ATclDd7",
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
        svg = "eJxVi8sKAjEMRX8lZD/YxGGmQtu1G7ezHzrFFlxIKT7+3kaxVQIJufccc11LhM3iiRlIH6eVgUHVoYEHXub2K/nRmZ0YzjSPZqBxGf+5ONBvAHwj1d0cfIGnxQkhhnSOxaJGeLz3PW0lWqQ9Qq4JiSW8Mz5lfwngq8iqtvUgeGG0QJ+6Yb0+4Nfp1AuX0EAu",
        categories = "transportation,travel",
        tags = "baggage,luggage,travel,cart,trolley,suitcase",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    BaggageClaim,
    #[strum(props(
        svg = "eJwlilEKgCAQRK8y7AEsxZ9g3bvEFhQUhPSht9fVj5lheI/1zvqc0JLIB0LusxK0jiu8TC787f+FI9Eb3QaLjy6MMsuoNCflFEo=",
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
        svg = "eJwti8sJwCAQRFtZtoBEPUgOagcWEVbJCjkEEaLdx09OM8y8Z3KkAs2iRuCYLi4WpULI1WKPN4XCvQmEOTizD8EZSpnuCHlSVJdEbWaH1u3McxaGYNFrkIo3Ib08/jawcbsPg7QkMg==",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "ericfennis,mittalyashu"
    ))]
    Banknote,
    #[strum(props(
        svg = "eJxdi7ENwCAMwF5BeaAkQqhDyjcdkBAzfA9JgIHJg20uuf6u0wfowTXhO4nGPkkeEj+SJdZYJR2pc1gvXe1u5InWRDswnHQAlPQhAg==",
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
        svg = "eJxdjFEKgCAYg6/yswuUleSDeoMOESn9voVI2e3ToIheNtj2TW9zYnIGU0/9LhQLBaubmlod/ZKIfVg5GYyg00C0oJiLg47gEhsMoFzbQtX9jxLdu68q7xP5gR/sApeUJs0=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = ""
    ))]
    BarChartBig,
    #[strum(props(
        svg = "eJxNjFsKgCAURLdymQ2EWeSHuoMWESld/0IuPXafUkJ/w8ycY/dFmILDrEkfyrAy8Larrbc5rkL5clCg22EElTyBzhSES9uDOKaNxWGoVP1/VLkr/Z8bNDXJ623UA2AUJqE=",
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
        svg = "eJxdzEEOgCAMRNGrkF5A25jGReE2LkyMa7m9A0xYsPqBvE4893ulqllsl1Qti6If3mqo9ZbYGisxMD4PmpMG5cZiOdyQc9/HrU/6A5NmIQQ=",
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
        svg = "eJxdjk0LwjAMhv9KyL11yfqxQbuLZ6/eCwoTRDyIzH9v2jA2RilvPp43SXqXzwy3jJcRAgTrobe+kGj9XX2GjPVnZ8PQQw9OgBo5cNZfKRYGVk6UZ+J9wfDXeJzSqW6Z0vPxusPCGQeEH2X0CIsIdZJKNVayMispPcZmYFaEWJ2iB7b5tTfq2KgWpgO6Z2iD2gjabvgDXU9ChA==",
        categories = "travel",
        tags = "amenities,services,bathroom,shower",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Bath,
    #[strum(props(
        svg = "eJxljLsKwzAMRX/lot3UV3l4cTx36UcEWlAhLR1KSf8+coaEEDwcS5yj/Bm/hvsgN3ZIxlGhiP7o1F+/z8F/FlRKvtSm5K3ska7tJsZwDGM9ZDx3LxIpNOisnSp2Y3q+H5g5iKrg7ySd6mwEs657l6tWFodxLv0=",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "colebemis,ericfennis,csandman,karsa-mistmere,johnletey"
    ))]
    BatteryCharging,
    #[strum(props(
        svg = "eJxlzlEKgCAMBuCrjF2gXFEv2mVKUogeRChv3yZiSU+/k+8f08GuEZz1u4sGVY9w+S06fk4IyeCMEG6DxJFy5GHRnfQWffjTQlLMFXPiHJjwTEIpJ2NhBcunrC4dsVOttvRr5LK6n/L836vGFzXlseIHwWA9jg==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,karsa-mistmere,johnletey"
    ))]
    BatteryFull,
    #[strum(props(
        svg = "eJxFy1EKgCAQBNCrLHOBWgP7US9TkkL0IUJ6+9bK+lp23oxJfslULBQo+LiFbMEjqFrMoPRAqvc545qDsIYzQ9s5s8fDU1HC4pUFGVT4/SXnqZVbrZcF9Wf/RrW4Vy/ixyc4",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryLow,
    #[strum(props(
        svg = "eJx1jkEKgCAQRa8yzAVKA9uolylJIVqIkN6+rxZE0Ooxf94fRke3JIrZsGTyLmw+GRYjU0+K4ZkpljacYU0eW8VWD7Vn9R4OR0UinCALUKAKSvhZNkKu2p+MWfWO+qovpb0kOu8Tj3wBzvAyXw==",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge",
        contributors = "ericfennis,johnletey,karsa-mistmere"
    ))]
    BatteryMedium,
    #[strum(props(
        svg = "eJxljEEKwkAMRa8Ssh+dfEuzmenajYcoKowg4kKk3t7EDmNLySL54b2fnuOr0CXzSTrSghEEijZiG+/+bFewO1gqATykvRtDal5PeuyaFkPT5grTFtb99rjSJJkBpg8yy4FpQs32F3HYsQr7U/nnSJxZ39VdsxvIs1ZYd/Hf/QXoPTwQ",
        categories = "connectivity,devices",
        tags = "power,electricity,accumulator,charge,exclamation mark",
        contributors = "Kaladii,ericfennis"
    ))]
    BatteryWarning,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFNBb20/YwWGxAPIWD7e5P2kgm7O1HKqSAtYUCYV/rAx5fWhLQj1MJ3VftXBCsPzHFxL8eH3wKNTHA9THYjbUbLiXzss/wDLhQcZw==",
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
        svg = "eJxtjt0KwjAMhV/lsPvFJk2zFraBD7CHGFNQEPHCi/n2Zj/MG2lJSHv4vrSv8X3DpauGgjLVZOq3ZkrCVJLWQhoETGzpbDCE9WSIjBypGLa6PRcqtRJXfXtasH17wDlQk5AolHhg2CHIU4A7mF3NcHl2ueUII7M/oERRGY4zGRW6i400N76lhDysf1hzo1LghK3uSgqxLFV/8Mf9ecUsXSVS4cPeve3jvI4eXUL9F7FZQmE=",
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
        svg = "eJxVUF2LAjEM/CtD35vbpB9mYVeQe737A/e29AQFBREf9N+bbBGR0uk0nXSGTO14bac92mMOSiXgOgfxs93nwM6201fXbKfLcjvgfw6//gBZqqHvwVesJIJMtUUmRiJOkTZqZIx2UVMOuktIXY4C1paNKGXrUDAbyZR2H9+ie/15Dvd/pzizuhBCPJpvsTjZXQzWPlBiiBfGDms1ClUUkm8ulCEDiRnDkm8Mlle8VaYWq9YfMR3XPok1wRN9UEHY",
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
        svg = "eJxFjbEKwzAMRH/l0G7Vsowdg+2lS5d+RHALLXQoaYf272MnQxA6EPd0l9/z94Fboask9hDP6ewsO0jopxNI3PWiHzUOalKzRlnhOJqA0JcjLISVO8eJlWo+jdiaj3A7PmTuvscmdhvtJfbg23Nprzvav9BEWAopof0KyTSY3a0r/BYp9A==",
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
        svg = "eJx1zD0KgDAMBeCrhO5V0h90aHsDV3eJQoUOUhz09ja0gw4OIQ/y5TnaM6UN6PICx84KoLukgVP2QpcdXF9RcA23w4vyu/2zWGVj+FHHckZYvZhQAVfNaJLUUoMpo0BHxZpVeAC/+i4j",
        categories = "transportation",
        tags = "bicycle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,csandman,jguddas,karsa-mistmere"
    ))]
    Bike,
    #[strum(props(
        svg = "eJxtzEEKgCAQheGrDHOBUsSVeoMOESmNuxApvX2pUAZuZvF+vlHBbRGSRiYQyPmdokaJkNsSnsIRLm8jaRRo1FSAUZXlMn2x96nexl90rJHAalwk8JnqtzJ1gQlg4yKBCeKnHKN/ugFNvTyv",
        categories = "text,development",
        tags = "code,digits,computer,zero,one,boolean",
        contributors = "mittalyashu,karsa-mistmere,ericfennis"
    ))]
    Binary,
    #[strum(props(
        svg = "eJx1kM1qwzAQhF9l0V1Tr34tsHPp2Q9R1EILLZRQSvv2mXUCCUQ5WLvsz3w7XvrHsX++yXF1wUn/W51a/GdUNHdYns4Dh+X75eddXle3FVSJSN2jSUCWSTICsyDsPBdWGvtVGooEoUwxGVu/inzNXNYJKoqE+X5g04q4Y24p/kJhJXllbjViiqkhDziajUGQfwja9SbMnerFUzgxWJpQfUb0uzsrqVDCysYdSdFuRPsdWt7Yylzkj1NvVuyJMg29C6+i9erJok8eX8wgbxiY3DKFDdzBBzMn7ctyO30CClZ0ig==",
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
        svg = "eJx1ykEKgCAUhOGrPN4FatQoQV236RBhgUGLkIi6fUpQG1sNzPebbdwDTZYHSTKgY2eqfDnzgqip7X8AKAqS6IC6QB2hCRAFUSQPqA/8Ev06U7QsmPxpWaW5LEPn5lF3A20gNlE=",
        categories = "home",
        tags = "shades,screen,curtain,shutter,roller blind,window,lighting,household,home",
        contributors = "karsa-mistmere,it-is-not"
    ))]
    Blinds,
    #[strum(props(
        svg = "eJxdTbsKg0AQ/JVh+yOOGpLi7uo0ae1FxbUTOXz8vW7lg4WdB8OMn7omYVqDUGC/FGxBCsEytEmDfATaDb0mo9G/LB/9WCdFG+TPDDmrb00QmZ2j46+8anBmfhoHUu+G4+zej4rCxmwm7jEXJps=",
        categories = "development,shapes",
        tags = "addon,plugin,integration,extension,package,build,stack,toys,kids,children,learning",
        contributors = "danielbayley"
    ))]
    Blocks,
    #[strum(props(
        svg = "eJx1izEKgDAMRa8SsosmUuuQ9gTO7oKCQhUHB3t7EwpuhZDH/7wv9/LssAY8PXigTq9x4GZO+idtPEZpTYqSjmuDTAGJETIXvkompfWjyabVZM192Qy/+gF0YyC/",
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
        svg = "eJxNjUsKgDAMRK8yZG819YNC612kCgoKIi709nYUioSXLB6ZcWE5wjoh3F60FIQrXhUcXjrpXf7p3u3DOWP0smlnahA1bRYZrKlAim+y0lRrFA2hVLwrSRTULf9ZwOBfvLWwUTO+TvoBOFonrw==",
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
        svg = "eJxljcEKgzAQRH9l2HtsdpsUhUToLZd+hKSFFioUKaJ/7wZBEVnmsMzMm/Dr/m88Iz0cuKn8aNjfpfIosnqMW/kSO2rDpaTbsHXEoh7ZJY10x5I1PondK/kz5O8LeY5UE4ZIQshTpBW7uju4V7AYpzTVebfnBlcd4c1aADruMrk=",
        categories = "development,security",
        tags = "code,version control,git,repository,private",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    BookKey,
    #[strum(props(
        svg = "eJxVjcEKwjAMhl8l5L7ZhKZOaAfeevHqvbhhd5NRqr69DcjYCDn85Pv++FcqGaaANwt06aV2JFfuBXRNGwKnKZLB0Z+UHv3mcLtLPceGpKNkOom8U9b5USDPyzOXgILwXqaSAw4In4DECN+ADmHVpJbyu0c0gLvbxMD/dgum8lb/AxLyMf0=",
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
        svg = "eJxNizsKgDAQRK8ybB/MLn4IJDmGfUAhgoqFiPH0bqxkimEeb/yRzowp0MYOwmYwrRnQjl0SCKyGIUYy2z+AXNw/FH1T79Gvyz6jSCC2hFvbabHOjlD4w+pWK77rbhsG",
        categories = "account",
        tags = "delete,remove",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkMinus,
    #[strum(props(
        svg = "eJxNi0EKhDAQBL/SzD1selY3CInP2LugoKDiQUR9vYmKyByG7q7yUzW3qIMMLKA0zmTGIfvnlUJh4xFqtKV9F9CFv11K/0l66ftubLBqEKpgYxAXX4pfwcqzjmyiXmxxb/ml0N6OfdgDiy0mBg==",
        categories = "account",
        tags = "add",
        contributors = "siarie,ericfennis"
    ))]
    BookmarkPlus,
    #[strum(props(
        svg = "eJxNy7EKgCAYReFXubj/5L1YEpiP0S40uAQN0dDTq5uc7cCXnvJWXIe7uUO0aMEiwrkWQfA9QqZKPw/o4/a7nJbBcwMwSBAG",
        categories = "account",
        tags = "read,clip,marker,tag",
        contributors = "colebemis,csandman,siarie,ericfennis"
    ))]
    Bookmark,
    #[strum(props(
        svg = "eJx1i0EKwjAQRa/ymb3YGdrSQtIbuHVf0mACLiSEqrd3UrVUsMxiZv5/z9zGHDBZOtXoz80oEFQ6DDlIYNkGkLmmwRyLMphV7NDN/Cdn2Sva3yJ5l3GPUw6WpCI8LfWE9NCPEHy8hGyJ9V4S1YowGBeTu3qkBXMqcaNbma4w73alvv2HVorbDfYCQaZIOQ==",
        categories = "devices,multimedia,social",
        tags = "radio,speakers,audio,music,sound,broadcast,live,frequency",
        contributors = ""
    ))]
    BoomBox,
    #[strum(props(
        svg = "eJx1jDsKgDAQRK+y7AV0YxSFJLWNrb2ouOkkBD+3N7EwCFrNMI95ah08w6SxIwF1L9sajcriaJSbRw8824W9RhIIu508h1ohHBolggsR9lPjfYsHox6jAJIski+B/I9QCVRsX6R5gwvanzQa",
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
        svg = "eJxNizsKwCAQRK8ybC+JSz6NeoO06UOUrF0Qyef2UaswMMw8eCaFPeOOPosl7gkS4iHZkh4I6S2M8LQuey6oHWe66jlzblngLS16Aut13BiMvkaxYlHDH4AvPVW3Wu4D8PwfPA==",
        categories = "transportation",
        tags = "work,bag,baggage,folder",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Briefcase,
    #[strum(props(
        svg = "eJxVjb0KgDAMhF/lyF5sQweH1jdwdRcV4yZS/Hl7jYOtBMJduPsStmlI2M5ITLgi1YRjGZO86ny3TMssSWUTKo03Ye2TYIzUejjbMxj2GWfYcOezx+Pl58EK0XoBcR6cKVZTRcsqZTfFQd98mBt6sy4l",
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
        svg = "eJx9jUsKgDAMRK8SslfbUkKF1ht4CFEx7kSKn9trdSNiXA0zj8f4uW8j7AENwnzHFtCeZbvKOnaRA2pC4H4cOJ6rwsoXyav81ESGLmBdgjFLZpkWm2jaH9QBca70B9EkI/OHtPphVn6TPZI9J2vubR1wL13z",
        categories = "account,buildings",
        tags = "organisation,organization",
        contributors = "ericfennis"
    ))]
    Building,
    #[strum(props(
        svg = "eJx1zkEOgkAMBdCr/HQ/Sus4YjLDDTyEEWJZmBgyQbk9AyRAAmz7+tvvv8+oKAM9LBwENyr8eZgVfhbO4NRu4SNp34jhhZrqFfGry6iB2BGafyAhdIEuBK3qt8YJ0ny8OARWryyYNfm2RA6+6injvX7u2BLdW9kL5RBuzUI9t3tKXQ==",
        categories = "transportation",
        tags = "coach,vehicle,trip,road",
        contributors = "danielbayley"
    ))]
    BusFront,
    #[strum(props(
        svg = "eJx1TkEKwkAM/ErYe2KTdrcpdHvx4sVHlFVYoQepUvT3Zimoh5ZASGYyk+nv4zPDJbqzQliCG/pDQYb+i7PfIQRYMne0KVJgzfWDPDK1pCikiRipIUFSa0wCle1YUMUVmaw16I9SEUMgM+nKYF4QTs0oYJpSaNPCVa5/r9NtTtMV0iu61sEcnThI7+hYy83K/gXsSj6/Keewq/8Ax8pNzg==",
        categories = "transportation",
        tags = "bus,vehicle,transport,trip",
        contributors = "ahtohbi4,ericfennis,Andreto,karsa-mistmere"
    ))]
    Bus,
    #[strum(props(
        svg = "eJx1jb0OgzAMhF/l5L00NoEKKeENunavCqoZKlUo6s/b4wwIhrDZ9+m+C+97UgyRruxQa+WY+nDOYR825CFl9BJ0EHdqSi0By62tdmweHwm/SJ7wj8RCmO2rCd9pSGpJS9Bxemqy2+VebuycnSk/xbHmiHjwRU28kgXZZUOX",
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
        svg = "eJx1zEEKwkAMheGrPLJPbGYkcWCmazceQqJQoQspUvT2dnDTjWSX7/HXeCwx3xGfRk6Id6NCWBolGuvhh2N9Xl8Tbo0uLgkuBRmqa4kBYnKE9pvUQgwDlPuLdeVtwIkzGzufZs4bJ7Ee7sFdVg2az/mP+E6+jeMs7Q==",
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
        svg = "eJx9i0EKhDAMRa8ScoCZphTpovUGc4hhLJOCiEhBe3vT6kIXdRXy33tuCb8E2aNGWLZ61jgk9kgdAof45ySrQhBosHfvEvRujFOATB7FyrqeTT4rR9dWzOKc5jnWgsyRkD2aqzx/E8Pg8UMdkOKXosLKemW6zexzZtqZechsO7ujHRuvWqA=",
        categories = "maths,devices",
        tags = "count,calculating machine",
        contributors = "ericfennis,csandman"
    ))]
    Calculator,
    #[strum(props(
        svg = "eJxtjz0LwkAMhv9KyH7YvJ7HDdfOLq7uBYUKKg4i9t83afo1lBseLnnekJRP++3oVvMFQhKvqQWBKnsBAefT9k/4SVwLSnSZm3KwIU15Pt536qVmMP0VkpRw9spkqkmTak7mOeHGGMg74tFbEA9I5Qnl4s6nvCQRxu0ohrjsNwDE4TlC",
        categories = "time",
        tags = "date,time,event,confirm,subscribe,done,todo,tick,complete,task",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarCheck2,
    #[strum(props(
        svg = "eJxdztEKwjAMBdBfCXkXTTdGhbb/Iq7YgoqUgtvfm9syHCMPN5CTEFfivVJZPBumsrZIMT9S9SyW6ZvnmnqrZmBSMnJwZ+wF98zvSIsomBSYnqsmQnBNKdCO2m20QezZA4SQSydIGCPcDgx/+7nVRLPn15VkIqM1ntp7GIQfgBg5jQ==",
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
        svg = "eJx9jGEKwjAMha8ScgBdujHyY90NPIS4YgsiUgqutzdpFaew/gp53/feFN0lQcwWDcJqsUfwLlx9skiMIPmAENeCn2FJvuTzdNTePN3C3UE2FkdpkzC9pt5M2hJVpbeqjD9oW+Q/UQ3qKuurWV65hr7u45w8LBZPDDT4Q1eQhhtEpsHGfSaT3JjkxuQvewEJal9X",
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
        svg = "eJxdjbsKhEAMRX8lpJc1111RmLHeZtvtBQUFEQsR/XsTR3wxxeFOzk3cUI4NVZ5/EJLkn5YgUGwvQoTv55oJk7zPDyWajAv3siWF69q+pkU8g2mGZ0mZFqVili2qatJDVWSnCUt30YwkjCC89SQODeXDNQnHxSDleyk/5BW9EzxO",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarMinus,
    #[strum(props(
        svg = "eJx1jMEKwjAQRH9lyD0xO01ihTTgzYtX7wWFCCIepNS/NyloFVoWdpbdty8++mfGuVNHZ6RFbXuCsFM1CIO4fl6UKf8txLTUUr5UipvqSvFrpEC88acw85qah53xC7QEcHALhwZi83bFb7P2v7rb9X7ByE6RCq9PSkmFcYqCVii9AQyLPdU=",
        categories = "time",
        tags = "date,time,event,delete,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarOff,
    #[strum(props(
        svg = "eJxljs0KwkAMhF8l5F5sprqssO3Zi1fvBYUKIh5E2rc3aVb7xx6GSb7MTnq1746uNZ8hJNUltCBQaa9AgdNh7gkf2U8DVXSRm7SzkCY97s8b9VKzBKYeroN6qKgNhhqU0cVqPIg83scVaCuIk1J6pqmx1TrUWOT/j86YDt5r2+AP5Q7IlXT+g79bwkeM",
        categories = "time",
        tags = "date,time,event,add,subscribe,create,new",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarPlus,
    #[strum(props(
        svg = "eJxtjEEKwjAQRa8yzF7tJMVmkXTtxkOIDSYgIiVgvL0zk4oWuhrm//e+n+O1QA1oEeZ3QIPwylNJAclxUjXhvEdIMd9S0WL0B/FGf8+PCNVweGSML59Ky0siMyrQgrZMEZ4X0X29NSiEbRvUNUSuGIZ+7PNSEkwBzzQA9WmnMxL+VxbInYaNRp19R1sSd27VfQA8UU+A",
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
        svg = "eJxtjsEKg0AMRH8l5C41Y6sedj330mvvQgsWSumhFP17k90VdZEchsm8CXHf/jfQw/MNQlLd6x4EKm0KFLhetp7wl/O6UMXQcudOdqRz79fnSRM810yTeAbTqCJqRwRV1KCEWthy3oAt96BFUsZrVSwEqwo5YpsULh80saQ+g5HgrBR+WeEZWuhHfg==",
        categories = "time",
        tags = "date,time,event,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX2,
    #[strum(props(
        svg = "eJxtj0EKAyEMRa8ScoE2GSkudC7TShVKF4NQ5/ZNjMOAzCrk897XhC09K+RU3rlGJI+wR3QImwxG+JVXzZa3iIvkTfM13NRbw6d8EzQS4iEmdWfniLI17qmgCg10QmTzRvoJ1IzJSLpbt05VlotScuNFZ0x3uP98gnmcw2ehSqPkgP9psUfr",
        categories = "time",
        tags = "date,time,event,remove,busy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CalendarX,
    #[strum(props(
        svg = "eJxdjEEOgCAMBL/S9ANaNIYD8BklQmI8EBLh97ZgOHjadGe2Jvk9Q7W4IiQOhfDEIweLpBGKxYX70vrg4xlyA85MsnPmireHSo1XZXHjDV8kqVqyKtKnCtSd6bHo+5/ISFFXaO6OpDxYhvsC7agxhQ==",
        categories = "time",
        tags = "date,birthdate,birthday,time,event",
        contributors = "colebemis,ericfennis"
    ))]
    Calendar,
    #[strum(props(
        svg = "eJxtjMEKwjAQRH9l6N3Y2SSmgbTgzYN+REEhgogHKfXv3VWpHmRh3y7Mm3I5X0+Y2TfS4PGBKJTzm0NZW2got/FeceybQ0LahVEgaG1Wek35+yulcmOiKT9idhGhxj21ofrFoBlTcvGPwuAoBKNh6+FfeSK7rgNb3Yv0BNr4MLs=",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,Andreto,ericfennis,karsa-mistmere"
    ))]
    CameraOff,
    #[strum(props(
        svg = "eJxNTbsKgDAM/JWQPWrTVhHazg66uksVKjiIiKhfb4uDkuFyL86swx5gtNgJlWlQgXRbQdWogYGhSEfxO+qPR+Qgyr9A3Nf/AnEguRBnmuSNzuRpxRk/b36ZwJ8WBSP4K6JE2CzKFHpt9wCkcSMO",
        categories = "photography,devices,communication",
        tags = "photo,webcam,video",
        contributors = "colebemis,lscheibel,ericfennis"
    ))]
    Camera,
    #[strum(props(
        svg = "eJxtzcENgzAMBdBVrL9A5YaUIMXZoENUBWFuCEUBtifhABxytL/9n59/UakXfDuyqUHwr7IJfhn+kZZNwKB16qMKGlCeW9Au6EA6TKNGwac8lfPgH2Vs0/tuuwJuyTyDOpMBewPudNlWnFzHJpkKZLLDTtld2QEwmUNM",
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
        svg = "eJxtkM1qwzAQhF9l8V2qR38rg2PILYf22ruhhRac0kMxydt31k5pSYWQhDQ730g7fs5fb/Jy6M7VZ+F0EMzJD1m2pd+Gii5w6Kbxweqn8cf1BPiaohSPqkc6ksq+mo2oQqr6PDckHxF5U7K2uGnzPiO1RWrrFt2Sie9Xpv7Xzij8DFy2+KK+1HiMEm/fxOC1rxJPAYSHIcy/Gl+rZkp4DIHtMESDXwXKFrKNxnd3AcmgEnCKq7sLcBbgbgGELPlvv5f3j1e54NCFTi6BG/frfrzuR5Za0fQNtURsDQ==",
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
        svg = "eJxtTssKwjAQ/JVh743ZPJoUkoI3D/oRxRbTgyAl+Ph7E0FRLHuYYWdndsJlyAljpLNi+EZBNSxso4XbFg75GraiNS3szgszfNblsJMaLJR1ewuW0PDUh02N7MM7+ODAJgnJKxL/a8t0zEjTfEo5kifc5jGnSFzoo4Ak3CNpwlJAVVs1fEWWIv6q1n51v8oTlzBAPA==",
        categories = "transportation",
        tags = "vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarFront,
    #[strum(props(
        svg = "eJxtjs0KwjAQhF9l2HtjNj9tCk3Bmwf7EMUW04MgJfjz9m4FRWnZyzDfzuw21z4nDJE61jDJUdvsFqttPuBiGKEwMAUrX1hV7UVDv4e9Kl0JfwjK9V9bFmttwcr46ughxRZhXdxVYJeU5g3EazaPp4xnJNaE+RHJENI4nVOOFAj3achJoEhhdoktgZ9KeSTczNat+p+8ANUwR1o=",
        categories = "transportation",
        tags = "cab,vehicle,drive,trip,journey",
        contributors = "danielbayley"
    ))]
    CarTaxiFront,
    #[strum(props(
        svg = "eJx1T80KwjAMfpXQe2OTdusK7UC8ePEhJAoTdpApQ9/etANPSvolNN+Xv3w/Pye4FHOiBBQnFuzBAVkM6mi1XpzFZDFaaugU6UADRiCnWtLnNv9QyiuCZWSFF6ty/RLS1mDQcOw0rTNqtnEB01wDMKa9174VrhkD8RrEAfZ1n2oTmzHv6tZjltsi8xXkVUw0IO9iSONSTNNs7Ji/F7YD+5/l9L/+A09gRJE=",
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
        svg = "eJxtzMsJgDAQhOFWlmlAo2IUdrcDi5BVUPAgQYJ2L8EHHrzOzzdsc7BlJNsFHmSHwBWgICihnF1Vee23iQZB53JqY51SmpQ/3PnbPS+/viIfm9efHiwkYA==",
        categories = "text",
        tags = "text,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    CaseLower,
    #[strum(props(
        svg = "eJxNy0EKgCAQheGrPGYfMWphoN6gQ8QUGBSERNTtSyJs9Rbf+9027BGjp1WDG5jKwsBScHWG4D7uDVjHtoDMSZYJcnliRZDzWUtInnQ+vfzrFaM7Sn8DyjUhfA==",
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
        svg = "eJx9jmEKwjAMha/yyH/rUsdaoS14AA8xumILE6QU1NvbdiAIMkIgee97JCYHXxBDusViiSdCflmShGdaSqzTQNiEt6WRnDm2gDM+Zb8G+KpyRXJHfCV1YzbXmcdcIhZLVw2WsVtN2onXD/7k7xPkAKEOUpwvLEa0HmoxtGDwFJXQ86/Rt1UonL5nP4iCPfw=",
        categories = "connectivity,devices,multimedia,communication,files",
        tags = "audio,music,recording,play",
        contributors = ""
    ))]
    CassetteTape,
    #[strum(props(
        svg = "eJxtjMEKgzAQRH9lyD1tdtAQIeYPeu090EIKpfQgon9vFgUVZA9vGd5M/Oeh4NWbBxGePhOEqyegZZFTAI7CPbD1K9abFO86kuJhqnoduq0YEK4ln1u0m9Sg2aXv5/fGJL2hwaxwlVw5KW9O1FYvLXKkLls=",
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
        svg = "eJxljrEKAjEQRH9l2N41cyZ7EZLAdTa2FnYHCgoiFhbe35vcIRbHwC47PHYmvcb3DZcsRwN3GvvBw8NVEb16wsagJOax2FQXNtTgEap+qItwwwo1DXsY/j8Zl5hTx4OdpaRtK1DS4/68YuqysBd8mMXqamcUTJztyjaqfAHEKiYF",
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
        svg = "eJwdi1EKgCAQRK8y7L+UJdGHepdIaf0IQhaq2+vKwDDD4/maT8EXaCX8o2s/C+EtSTiQ3Qmcy8UydvSTCtE/hzBSoNtusLNxcKZHuZLYACfzF9E=",
        categories = "arrows,navigation,shapes",
        tags = "back,menu,panel",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronDownSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNVOwVDADQV0zJTsbfZC4HQB8NQfk",
        categories = "arrows,navigation,gaming",
        tags = "backwards,reverse,slow",
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
        svg = "eJwliksKgDAMRK8yZF80ElyluYtEQUFBigt7+/4Ww/B4T/1Kfh/wPxIvhFRvJnjuaDoNb/pu34k90sMCXoMEQV0rmrEC288TjQ==",
        categories = "arrows,navigation,shapes",
        tags = "back,previous,less than,fewer,menu,<",
        contributors = "danielbayley"
    ))]
    ChevronLeftCircle,
    #[strum(props(
        svg = "eJwdy8EKgCAQBNBfGfYuYUl00P2XSGk9BCEL1d/nehiG4TGxlUPx1KySyG+EN9FCaL1mwjeGlHqKDuY42YHjvasgJ7p8gF9dcAE95ib8AzYOF9U=",
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
        svg = "eJwlykEKgCAQheGrPGYfabhoMc5dYgoKCkJc6O0ddfH4Fv9jfZK+F1Ik7whazc0sQ+F1duH/yDfOSJ932BEQFls/9CANxhATQA==",
        categories = "arrows,navigation,shapes",
        tags = "back,more than,greater,menu,>",
        contributors = "danielbayley"
    ))]
    ChevronRightCircle,
    #[strum(props(
        svg = "eJwlyzEKgDAQRNGrDNuLRlOkSPYuYoKbQpCwoN7eLCmmeZ+JrRyK9iZaCVLqKZrIBUKXjfDUrDLgM+A424HjvasgJ7rcggAPP/VZtsA/IrQXiA==",
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
        svg = "eJwlirEKwCAMBX/lkb20lgwdYv6lpIUWFEQc9O9VHI4b7sT+bOGFVU/uJFhbzkMHqeyrq6S7fHg8xQuOwdsAPIcZtAPJaRNE",
        categories = "arrows,navigation,shapes",
        tags = "caret,ahead,menu,^",
        contributors = "danielbayley"
    ))]
    ChevronUpCircle,
    #[strum(props(
        svg = "eJwlyzEKgDAMheGrPLKLVDt0aHIXscV0EKQE1Nvb4PCW/+PlXnfD3YopU0iEh2klaG2H2l/6SAvhdZA8+0HytZmiMJ0JISJOY4jODvIBJw8XjA==",
        categories = "arrows,navigation,maths,shapes",
        tags = "caret,keyboard,button,mac,control,ctrl,superscript,exponential,power,ahead,menu,panel,^",
        contributors = "danielbayley,ericfennis"
    ))]
    ChevronUpSquare,
    #[strum(props(
        svg = "eJyzKUgsyVBIsVXKNbRQMDTVNQNBBTMlOxt9kIwdAIx0CF4=",
        categories = "arrows,navigation,maths,gaming",
        tags = "caret,keyboard,mac,control,ctrl,superscript,exponential,power,ahead,fast,^",
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
        svg = "eJx1zU0KhDAMBeCrhBwg03TaToXqZYoLQWbhqr29CVEUwdXrz/eSUpetrjPUNiJ7hNotNwmHU/nY/1TeXbizdfnP0HhEz8Q/hC7HjNC88e71Kl7doZUkcslUphjMsaRO+tIQHxV9Zkf5nBwpJNsla4erf7Z2rz47Hg==",
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
        svg = "eJx9zTEKgDAQBMCvHOmj3mLQIqa28RGiQgQRCwnm9yaiYBFtbovZY/UyrxN5NAIQdDzJIQX5K4zOY8nord8tjY3oGMRo4UrLZeQIL0ZklxKug1iZqYRVP0/1UEhkSuK+Kr36XTsBA0c9QQ==",
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
        svg = "eJxNizELgCAUhP/K4R75HiIO5tzS2i6vwKAhpKH+fVogDcdx9/F52bLsK+QeFLFCLqUV5Hpn8P3Hgz/imbAMaiILlzobGQwNKtEwybRdHzO66lbnbzLIzbaRB+upH+w=",
        categories = "shapes,money,currency",
        tags = "monetization,marketing,currency,money,payment",
        contributors = "karsa-mistmere,jguddas"
    ))]
    CircleDollarSign,
    #[strum(props(
        svg = "eJx1kNEKwjAMRX/l0vfGppvtCtv+wI8YVVDwQYYP+vcm7YPoOgI3D8nl5GZ8LM8rzpM5sSOGJx6WRKlDESfF6GiAM/N40N15/DoiBRlGVscRRarDU0gisWHyTIOHwjYcp6SWxZFPUNo/yAqjwFrXdSQ3KE1dPYpU116gQD0KrMEJye4EkpdBYds8tp1HXwZhNd4W7W+afFvz/YL8ngx7g/yqfZWmS3U8fwCjDmtN",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,draft,code,coding,version control",
        contributors = ""
    ))]
    CircleDotDashed,
    #[strum(props(
        svg = "eJxtyrkNACAIQNFVCAt41MgyxMLEikq3F6+O6hf/kTSVXkELpogg05qt45Qp3M/03AfbO2wBfLIWRQ==",
        categories = "development,shapes",
        tags = "pending,dot,progress,issue,code,coding,version control,choices,multiple choice,choose",
        contributors = "karsa-mistmere"
    ))]
    CircleDot,
    #[strum(props(
        svg = "eJx1yzEKwCAMheGrhBygNVm6RG/QQ5S0YKFDEQe9vYqLIE4/vI8n+gb9HtBkkRhBc2+oMehk7+7kv6KH2+JJBxD7zVDTto7Ga5tuBU7rIhg=",
        categories = "shapes",
        tags = "pending,ellipsis,progress,…,...",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    CircleEllipsis,
    #[strum(props(
        svg = "eJxVyksKwCAMRdGthGygphQ6idlBF1HSgoUORBzo7v1NdHThncf+jg4ei9cJZBwZFN7aJjzLsYh+Qf8XNFmkHUHzaKjpr+FSALkbGd8=",
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
        svg = "eJwlycEJACEMRNFWQhrY3ZxjOtgiJAoKHkQ8aPdqZA4P5rPmpiVCc/i9CDq3tB2m8HO7cPU9QXD4E4HN6nllAY2uEpY=",
        categories = "shapes,maths,development",
        tags = "diameter,zero,Ø,nothing,null,void,ban,maths,divide,division,half,split,/",
        contributors = "danielbayley"
    ))]
    CircleSlash2,
    #[strum(props(
        svg = "eJwtyrEJwDAMRNFVDi2QWJAioGgZ4cJgUriytreEXT2O+9LbX+Hlo/IQJh9jvwTnROXKSMXasF5hHhETbG5HcGe1f12NrBYY",
        categories = "shapes,development,maths",
        tags = "diameter,zero,Ø,nothing,null,void,cancel,ban,no,stop,forbidden,prohibited,error,incorrect,mistake,wrong,failure,divide,division,or,/",
        contributors = "danielbayley"
    ))]
    CircleSlash,
    #[strum(props(
        svg = "eJyzSc4sSs5JVUiusFUyNFJSSK6E0EVAykDJzkYfIm8HAOpBCzs=",
        categories = "shapes",
        tags = "off,zero,record,shape",
        contributors = "colebemis"
    ))]
    Circle,
    #[strum(props(
        svg = "eJxlzcEKwjAMBuBX+cl9aLKJDtq+gVfvoyum4EFKUff2tlXYYBTahvz5YlLwGR9LPWFpdyqFEDTEu2ZLfCG845y1fZ051AFnnlNWzJauzBh1mASCYzvSya2vwRpxxsfkHwG+qCPBL+1JdUWJ/Job7QzhV7dyXDkd9lxx+PSHGl6qFfwCcD86Aw==",
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
        svg = "eJxNjLEKwzAMRH/l0G5aCWFasDN36do9NKHKUCjGtOnfx8qQBA2S3nEvlfFZUeZMTPhNQ7VMF0L5r8DG6WU1kxIaEMLsaZdO3urSp6+GIdOdI9SkFwjObbht+bLuILTrFo9/kEc8FoKYuNmdu/l9BavboEG3eAGA5yyi",
        categories = "text",
        tags = "copied,pasted,done,todo,tick,complete,task",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCheck,
    #[strum(props(
        svg = "eJxtTMsKgzAQ/JVh76HushUPiWcv/Qip0ngolBBS+/fNeqhCZWB3mJdP8z1jDdQR4rw8Yg6khPQJxIR6hfBephy3RFpN7v3FWr1/jTliCnTroEM7CgSNwVVWWHehfoksR8FJcWJTNnKY4hYa9yRbtehJUBisA/O/9eQruHGKDT//C8QqO5I=",
        categories = "text,arrows",
        tags = "copy,paste",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardCopy,
    #[strum(props(
        svg = "eJxtjL0OwjAMhF/llD2hdp2WSG2fAFYGtopWpBIDqiJ+3p44A3RgsH138n3dOl8S1ldvyGB9l5PN3iBrNnguU4rFx3m5xtQbMUO309bQ3ccUMfXmSJUTBrFraGRH0KlAUBXasg6tCx5MEDDfXAi21sA7qW1eclasArfYBhJ5ZHCmKS+rB8kvsFnFXPd/ygKqnT8127rlyN/XD52ZPLA=",
        categories = "text",
        tags = "edit,paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardEdit,
    #[strum(props(
        svg = "eJx1jLEKg0AMhl8lZL/WhCM43Dl36dpdqjRuRY7Wvn0TF09QMiT5Pv4/zeOzwC8jI8y2COE7DUUztgaWFeg4vbRkjAiL8y5dPdWld18Uhox3EojKPQNDY0O2+UNxA8Gum9R/4IfUgcDK3uyddTMDkcYTI4em9ciloWMlO/UH5UxCRg==",
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
        svg = "eJxtjTEOgzAMRa9ieY9bGxcSiTB3aC/QDRXUIHWoUETp7ZtkgQFF+ba/vp/beXxGmH8eGWFeS0mDIHynIQaPFmEtGsbpFaJHxa495a2u/fQxwODxbkGvdS8gcM7PpG5h3YxUJbDsDSOLoUtmZcqOxTVo2KIMTE0FfJS0pAKOau6FGPIv+dS5psiNa3IX4AY4IeybnDNVdpS0Mkn0ccC1KRq2g38dvkaZ",
        categories = "text,account",
        tags = "paste,signature",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardSignature,
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/F8h7BWVEAKenMwspe0Qp3Q1UUyt/jZmgzVB7O9t29OI+vTEviK9P8SwyTpYqO01tzYs/0nYasNWIJ4S6e1lYXP31WGhI/EMir9EJCZxuYSoHfH862e2hvJ8/QFpxoJa/MhnwjSHHQUHDgAoTLYQ9CQAmb8wcGqDvz",
        categories = "text",
        tags = "paste,format,text",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    ClipboardType,
    #[strum(props(
        svg = "eJxtjLEOwjAMRH/l5D0CW8ECKenMwspe0Qp3QEJRBOXvcViaAXnw+c73UplvFeWTiQk2L3ermSLBDSGsmY6E9zJV+6mytr8h7VprSM+xGqZMF1ZEk1Eg2Puwb3lx3Izg6qz9HeSqfSGISSM35kZ+8AHMQaF/spNH6KMvCXszgg==",
        categories = "text",
        tags = "copy,paste,discard,remove",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ClipboardX,
    #[strum(props(
        svg = "eJxNjLEOgCAQQ3/lcjtRLoQ4ALOLq7tR47EZQhT/3sNF0uG1TVqX9jXDHbfMHgcE3uPB2aNBKF/xeCSEJNCCUhFcV1fBnUtm2DxO2oJhWggIepEW0qXNXyhxo22zotm2A0VM9bl+hheVxSSc",
        categories = "text",
        tags = "copy,paste",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    Clipboard,
    #[strum(props(
        svg = "eJw1iksKgDAMBa8ScgB/qLiIuUzoIhDaUl3Y29tQCo83ixkSLWIB5Ltx3RCkdpaGBZnm7plysmoaA+Sk8X28ghPa+fbpgMvrUfEPcRkZcw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock1,
    #[strum(props(
        svg = "eJw1i10KgDAMg68SegF/HsSH2suUPRTKNqYP7vZWZRASQr6wWlNP0H7QshJaxEzQ+6vC078L1+LdLSfUYvk63xkbwkI74hPsYOQBQIoZDA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock10,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+QFGouUzoIhDaUl3Y25sqhWGG4T3PUlgD+DndvDgUm8mB63fJjz8nn5NWlRiQk8T7ahgbrCzHsGJvcpfoBVhyGUc=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock11,
    #[strum(props(
        svg = "eJw1irsNwCAMBVd58gL5FKkcL2NRWLIAkRRh+xghmrvijtWaeoJ+Nx0nQft0C+0kvM0uXIt3t5xQi+X3GRcuBOKObWX5AdG/GDM=",
        categories = "time",
        tags = "time,watch,alarm,noon,midnight",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock12,
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9ANWD55iPhN6CIS2VA/29yaKsOws7JDoECsYR1pzgtzOzTlfMi3fz9SbTdNa0JvW64wbO7wiPnLIv8QPVjAZOw==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock2,
    #[strum(props(
        svg = "eJw1iksKwDAIBa8iHqA/aFbWy0gWgiQh7aK5fZVQeLxZzJBoF8sg48L9QJB3sjs2ZFqnZ2rVhmnJ0KqW544KEvjF0nI6I/8z/gCImBmg",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock3,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGOYNzCPRLpbRj7TMCXI7V+d4yTR9P1OrNkxLRqtarjNu7PCK+NhC/iV+AFZoGT8=",
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
        svg = "eJw1iksKwCAMRK8yeID+oK7SXCa4CAQV20W9fbWtMMwwvEeiRSxA7sOtm0NpszhIfS/T/HGmnKyaxoCcNF5nx/Bo9cdPe9eHxg+G4Rmg",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock6,
    #[strum(props(
        svg = "eJw1ilEKgCAQRK8yeIDKICGwvczix8KiYn3k7VuLYJhheC+yNNYE7ofzq0OzWRz4fi/F+eMUa9GukhNqkXydAyPAyrJPG3wY9m/RA2/eGXY=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock7,
    #[strum(props(
        svg = "eJw1ikEKgDAMBL+y9ANaEfEQ85nQQyC0pXqwvzelCMvOYYZEm1iCvFeIW4D0yeZYA9MyPVMt1k1zQi2an3tUOODnOxH30f4Nf0IzGRA=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock8,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AD+QN3EXCZ0EQhtqS7s7Y0WYZhheI9Eq1iEtCPMS0D1mQLk/i7T2DlTydZMU0TJmq7zxdjg5dmHFd3+LX4Ab4AZcA==",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "wojtekmaj,ericfennis,danielbayley"
    ))]
    Clock9,
    #[strum(props(
        svg = "eJw1iksKgDAMRK8y9AJ+EFcxlwldBEJbqgt7exNFGOYNzCPRLpbRj7TMCTKcq/N+yTR9P1OrNkxLRqtarjNu7PCK+NhC/iV+AFZwGT8=",
        categories = "time",
        tags = "time,watch,alarm",
        contributors = "colebemis"
    ))]
    Clock,
    #[strum(props(
        svg = "eJx9j00KwjAQha8yZN9nJj9NAm3BA3iIUoUKClJc6O2daRe6SCWECcyXb95003WZbhdaeuMNTe/ecJL6kurM0B229tA9xudM596cAhxxBB8TJbLEeiISU54ZqYwBkfTatenklbGa1PD13PUTcUZoUBr4ClG2SWUXYIuWnEWCF6YGeEg6SfwH2AyN5OD6iKwGvwdkeN2iJRaiJgi6ZlTLL/ABazRfOw==",
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
        svg = "eJx1jksKgDAMRK8ScgAlAcFF7W1EBGkFXbS3N+lXF9LFNJ/JPHP6I27ewel3d18LEoM8hnmYVGgSpdQpRR5pC60Zi92aY3crRJYDsoQQ9McIkRZk0UCpFocuWqO+ZKm53O72lBrwZ2l0L9ivJ1MpQ0PoaKIlJC0/oS9Giw==",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,ericfennis"
    ))]
    Codepen,
    #[strum(props(
        svg = "eJyFkMEKgzAQRH9lyd2tu6ZJBBX6Ab32LrRQQVSolNqv72aVkh6K5JDNMG9nSDW18x2utTkzAblLaBkY8ngyygh90Wc+s4kqt0hgT18JCgjPBATZFEExJWLkZFOCaeLbNNUhlmiqaeyXvhtuMI3dMD9q4/EIFqONwWGI9k1RaLP/AalEX4JOFp10JN6h1iDNUGSd4podsED2UrB0yjPmBJzLD6j2yyq3kISJLRhYuDZSDF603etbGPV/AJfraNs=",
        categories = "brands,development",
        tags = "logo",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Codesandbox,
    #[strum(props(
        svg = "eJxlzEsKhDAMBuCrhOzLNJ0wzkDrejZewJ2goCDiQkRvb+L7QRd/k3yJb7OuhDxgQhF8S8oYGCyQPCu1IYz9S03sd/lWyP1vtaKN/P7RuTacHpt11RQwUMAPwijhJFxARhicNgUqOUGyy0zzsnGjSvhxlObuRicAjzhT",
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
        svg = "eJxli+EKgzAMhF/lyP9ljUqaQSvsAfYQoxs42GCIiL69rYIIEsjdcfeF9OnT9400RzJCmlbpIym14bqVbfg/hw6vSA8xdjeI49rfFQoHKZdzA7GClOkB8NBOxubc/ETZZ7RmM/bZXiq2CuXt4wXTQiiJ",
        categories = "money,gaming",
        tags = "money,cash,finance,gamble",
        contributors = "lscheibel,ericfennis,karsa-mistmere"
    ))]
    Coins,
    #[strum(props(
        svg = "eJwljEEOgDAIBL9C+IChXjy0/Yw2QmI8EBLb3wvtZWE3s5u1nQY6CiYE1x2hT/3kMi5IBwI3udnWrz3Imrfo1fzI22Akz8iLfilmaO3QtM4GVX8QERw8",
        categories = "layout,design,text",
        tags = "split,parallel,vertical,horizontal",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Columns,
    #[strum(props(
        svg = "eJydkEEOgyAQRa/ywx7LoEabgOtueogGSSExaiyxevsCsYmbbroZmJn3Fn/UYk3AsmkmGZz1Txc0axnevg8u//a8ykCnLgnv1PwIDr1md6ogDRUEAYniGotcKyMQRzy1PA6SloSTJsVfGkWivdWGU9FA8DK+ZazlyulET8M++NFinvwYXpo1kISsogHVmTyYTuX4MSJVP+Ifp9ky8j3AB7GxUW0=",
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
        svg = "eJxFjFEKgCAQRK+yeIAtRdTAvIxECKJifeTtM1foZx7MG8b6UH08oO6Mrwz80yk626CzC3lnS47tzAlKDum+ulUoJGjUCrhELmZSMdyGxlD82+9xPrkXEk0fzQ==",
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
        svg = "eJxVjEEKgCAQRa8yzAVKqXCh3qBDRErjLmSgun2OiNTmL/7779scd4bb4YxwpcDkUE0IFNNB7NAg5AI1wiPp7SB7b6vVSNP0+NO6Ve66dm5MEByuCyhD9U+qD1BayNLJC1pKLUs=",
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
        svg = "eJx1zD0OhCAUBOCrTOYArMgGKMAb7CE2q1ksTIwh/tzeR2MssHjNfDMvLMMvY49siTSM/5QjPXFEWmIb+5yEGmKRimYXXqXfhfmbE/rIj3bQ79UVKdlNHkEmZjXVRT3XjbxCqwysqvgkaOGUK1dhL+rhLzkBuaxDkA==",
        categories = "development",
        tags = "roadwork,maintenance,blockade,barricade",
        contributors = "karsa-mistmere,ericfennis,jguddas"
    ))]
    Construction,
    #[strum(props(
        svg = "eJw1zEEKwyAQBdCrfGZfmkkluFBv0EMEI1UIpYhQvX0dTXEx6n9/zGcvEYelJ29gvSsoLHJuGgs5c5fcGZ+yPwN8s8RMyJYeBF/7axU0Y2dy8AV1hF0qwjcdJXalCTGkVyzznrsZRSk4c6Z3QGP5Q10tdVF5jLbKmg6FXFAi3qaUOc3V/9Mfaes5JA==",
        categories = "account",
        tags = "person,user",
        contributors = "karsa-mistmere"
    ))]
    Contact2,
    #[strum(props(
        svg = "eJxNjEEKAjEMRa8Ssh+cRNER2q7deIihU2xhECkF29ubRgpDFp8k733zWUuEzeKTbkDLysAw95l44sf9uAOjM6fOO5ODL1AtnhGyBCPEkF6xWKQFoVm8IHzTVqIeROuCMz5lvwfIanjBaJaUAtLu/9uZPb0DVLIoXZU1GqnTuFcL2pEByo2u46eermoM9AcnMDwv",
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
        svg = "eJwlikEKgDAMBL+y5C42ORQPaX7gI0oUFDxIEVFfr7XsYRlm1Nfi2wy/ErEQyneB4PePpn3zpns+FkyJRhbwkCMiQlvHcrI8Na6RvbbsFgY=",
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
        svg = "eJw9TkEKwzAM+4rxPVkcvLFCkh/suvtIy9zDYITA2t/PbmkxCMuWhNL31QXGjB+KQFeIOuwYS7rYp6Q21Q5tzRgRZJrf0jMSI/zmscu+LhnvCOuGbTGlus1X0pH+YKBbdeQJgovODwrxyTXYyRgoFwp1U2gHP1iVs8Yfakgp+Q==",
        categories = "text,notifications",
        tags = "clone,duplicate,done,multiple",
        contributors = "danielbayley,jguddas"
    ))]
    CopyCheck,
    #[strum(props(
        svg = "eJwdjMEKAyEMRH8leNeaYEsX1D/otffiShVKKYtQ/ftNcskwLzMTP/1bYVEyeDUwkZVYxd8NLFSe40ViOR61DFjJ8Otg4WSr/d0GpwKjqWjq/9/30ZRzW3o5/l6jwZ7MIwDeikWH4C1Zt/GhZyhekDhg39AXTQCB2/iQDMlEPgFsgiz/",
        categories = "text,maths",
        tags = "clone,duplicate,remove,delete,collapse,subtract,multiple,-",
        contributors = "danielbayley,jguddas"
    ))]
    CopyMinus,
    #[strum(props(
        svg = "eJxNjUEKAyEMRa8S3I81wSkzoN6g2+6LI1UopQxCx9s3sUK7yefp+4l7lGeChl4hKWjEuSg4hGdO6hncSbTgfvI8JBrSMsp/8p5ihZzKPVf+sCx6xdrevJJTXzg6vMtWc5e4Lb3gXreaYfPqYgHPcUKNYCaa9MqDrjYaeRIC5owmdgMI9MqDZJGsCB9bYzg2",
        categories = "text,maths",
        tags = "clone,duplicate,add,multiple,expand,+",
        contributors = "danielbayley,jguddas"
    ))]
    CopyPlus,
    #[strum(props(
        svg = "eJwljsEKAyEMRH8l5K41IqUL6h/02ntxpQqlFBFW/34T9zKTCW9C/Lf+MkwKSA+EYS+f4pYzLY/+Jlj0LacOMyAjbQRkos1lR917YdYhlFw/pV/zEJTb0ov+/+4F9oBPB3RPijSBUVbpjcW+XDKykgScC5m0CLCgN5b1hpyIJ20SLP8=",
        categories = "text,development,maths",
        tags = "clone,duplicate,cancel,ban,no,stop,forbidden,prohibited,error,multiple,divide,division,split,or,/",
        contributors = "danielbayley,jguddas"
    ))]
    CopySlash,
    #[strum(props(
        svg = "eJx1jkEKAyEMRa8S3GtNkNIB9Qbddl8cqUIpRYSOt2/MdNPFbPLzk/dD/LO+MgwKCi8KNmQl1p8fu4/+NLHodxgPYPqHW04dtqB41ViYbENkyKzk+iidI07Bp669SMvpmYv+fe8F1qCuDvCcNBoEq0mbhQvdXLJzNB2wL2iTEEBgFi7yxjwRv0ueODY=",
        categories = "notifications,maths",
        tags = "cancel,close,delete,remove,clear,multiple,multiply,multiplication,times",
        contributors = "danielbayley,jguddas"
    ))]
    CopyX,
    #[strum(props(
        svg = "eJwlTLsKwzAM/JVDu13LmNKA7T/o2r04ocpWgiHJ30dyljud7pG3pXUchV4EWdaf9EKcCPs6d7nPc5ibZqLSaVTzw3o1/79dMBd6J/CzOfaM4KLzk0L8pBbsZQqqhUMbCUT4SWEM2US9AIjuIcg=",
        categories = "text",
        tags = "clone,duplicate,multiple",
        contributors = "colebemis,csandman,ericfennis,danielbayley,jguddas"
    ))]
    Copy,
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYpi6EcW7QIWQKClqEtKjbq8hbvMV7rFfV+4B+mdxK0H+6dlkSXmYXfsp7Ys+0JSTjYwkIsHAdi2j8WMciDYnYFRM=",
        categories = "",
        tags = "licence",
        contributors = "ericfennis,johnletey,csandman"
    ))]
    Copyleft,
    #[strum(props(
        svg = "eJwlikEKgCAQRa/ymQOYmi6EcW7QIWQKClqEtKjbZ7r4fB7vsR5Vzw36ZHKeUNtZgr4dhafhha9y71gzLS4imTmGEhBg4dosovGp538mH9sGFd0=",
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
        svg = "eJx1jlsKgCAQRbcyzAZSqWBA20GLiIqmvxDpsfvUAin0b7jnnuFqO48OToM1wrFOjg3KFoHndWH33FeE1ncUdroKQqej9gq+4yHFJiW3Te1tcAyTwV42oPb4JkQ/ILJEgWw4Dyifi6IhCgoVVtF31A3WxFY4",
        categories = "devices",
        tags = "processor,cores,technology,computer,chip,circuit,memory,ram,specs,gigahertz,ghz",
        contributors = "colebemis,karsa-mistmere,ericfennis,jguddas"
    ))]
    Cpu,
    #[strum(props(
        svg = "eJytjEsKgDAMRK8Sujf2o1Sh7Q08RImCggspLvT2NlbBA8jwGEiG52hJtE5AhxdKC6CzdMolRXB1+Qe3xX2G0YtBSejRRI0dMJJTGWxBgcGCfGKwyRt7817yjrWs+0rtb9ILHjYt/A==",
        categories = "",
        tags = "licence,license",
        contributors = "ericfennis"
    ))]
    CreativeCommons,
    #[strum(props(
        svg = "eJwdy0EKwCAMRNGrhLlAG2l36mVaqULpQoQmtzdm9UN4E3u5BtXSnjoS+ABpwgmShAD62z2qXTuo+yfHbQ1yfNtXSNiVBItVrWxU2Wt2qTwBX/kacA==",
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
        svg = "eJxtzjEKwCAMBdCriBdozVA6qJcRB0E6OJnb91ubSKHTR/9LiE+lpZpNC9bt1iRGErI/Gf02++hrubLpLlhCzW6yTsgTbxI+mGB8DgN7KJHRL+Vl3gnd/kdplYp5niL4Bt/oNzk=",
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
        svg = "eJxdzTEKwCAMBdCrhFygxELpYLxMcBCkg5PevmpMaTt98L8fvaQiOYI0RnIIUjUL44nBb1oHn9MVoRLjjtBWOMYDoc7odBCD/Y3OR46dI1t8aVudnplkLPWrH7XuZeZZTcM3DFc2Yg==",
        categories = "currency,money",
        tags = "finance,money",
        contributors = "connium,ericfennis"
    ))]
    Currency,
    #[strum(props(
        svg = "eJxtjk0OgjAQRq8y6b4j84c0AU6gW/cESTBhQdQQvb1TNIYFaaZf23l503qYptv8GKB/NYE4wN0zebybIAF6DwttffhhbT13zxGuTTgLEHcJBIp1GTCWKaOZ2HBMkFAuttMSsIV0IylRj+6pqh2Y2AcuOupeT4CLzvwTX0+KAoq21voS/ZSrj4Ti0yKj+VWjoBKQb6fsL//uD1jkRYg=",
        categories = "devices,arrows,design,development,photography",
        tags = "storage,memory,bytes,servers,backup,timemachine,rotate,arrow,left",
        contributors = "ericfennis,jguddas,danielbayley,karsa-mistmere"
    ))]
    DatabaseBackup,
    #[strum(props(
        svg = "eJxtzbEKg0AMBuBX+ckD2Cbn4QXuhG4d2rV7sUILDlId9O3NiYiDBPIn5IPEtut+/dCimRKxEP6WSmjmRN42C0d1vGysjv17/OKT6OngX6w3hcN1LfYQLkKZeVYHK2w4nB9YHhzA1V1sUIicMGfq+KksvOYeqh0vUBAz/Q==",
        categories = "devices,development",
        tags = "cache busting,storage,memory,bytes,servers,power,crash",
        contributors = "danielbayley"
    ))]
    DatabaseZap,
    #[strum(props(
        svg = "eJxdykEKgCAQheGrPOYCNYqLAQ06QFv3YUKBC6kWdfs0Iije4ufBZ2NKS94iwunIENYSTQiHI1bllgp1tnlYZ/O4z5gcDRrGs/QCjfaeYrB4U3VFH8rqB9XLLhqGIcg=",
        categories = "devices,development",
        tags = "storage,memory,bytes,servers",
        contributors = "colebemis,jguddas"
    ))]
    Database,
    #[strum(props(
        svg = "eJxFi7EKgCAURX/l8Xap90BMUOeW1oY2ocBAoqHB/j6VRO5wudxzzO2fALvFhUeQs45CQU0g8gwMYw0LXlXfIu8NnRmK7Uw8rwMSWyRGeEtLhES5p7xz68IWqrH0s/XrClelsR9hcCW7",
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
        svg = "eJwli7ENgDAMBFexPADg0FDE3oAhEIlwOmRZImwPhub1+vvLVncHuxkTwpszwtWKKyMtCFrbof73/kHr8ZQ8hif53FyhMK6UgJIOEwWLVR58uhjt",
        categories = "gaming",
        tags = "dice,random,tabletop,1,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice1,
    #[strum(props(
        svg = "eJxVi0EKgDAMBL8S8gA1SsFC2x/4CLHF9CYlYP29Ri/2soeZHVfSJlAujyMCp7yzeKQZ4SETQn33zFH4w6XqM7heu+COVRiix4UMWO4GUqXwpyyQadQNbgkgSw==",
        categories = "gaming",
        tags = "dice,random,tabletop,2,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice2,
    #[strum(props(
        svg = "eJx1yzEOgCAMheGrND2AWkwIA3ADD2GEWDZDmii31+qig0uH/+vzNS8C9QhoEPaShAOSQ6jtLpzLyvKk62dEaHqj73UX/TYLQwo4kQXH3UBKGt9kgMyPOSD7oRNqlSf3",
        categories = "gaming",
        tags = "dice,random,tabletop,3,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice3,
    #[strum(props(
        svg = "eJx9zMEJgDAMBdBVQgZQoyA9tN3AIcQW05uUgHV7G72IoLckL//bHBeBfDjsETimlcUhGYQ9BeF7rDgg5HL9FF28bTXn7TYLQ3A40QiGm46U9Pgg8yM0flDte9kJ7T4vVQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,4,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice4,
    #[strum(props(
        svg = "eJx9zMENgCAMBdBVmg6gFhPDAdjAIYwQy82QJsr2Aic94Okn/7XfpLALcIgHi0XSCLfFGSGVUCVyi9y6K3rhduTMWP+cOTdh8BZXWkDzMFGlWr5I/wgtHSp7fVNA6mMPgJ43AQ==",
        categories = "gaming",
        tags = "dice,random,tabletop,5,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice5,
    #[strum(props(
        svg = "eJx9jMENgCAMRVdpOoCKJqQHYAOHMEIsN0OaKNsL3DTBS3/a1/9MCrvAFb2wRUUIt8UFIbeZyjKXyC04xIOlfTkz1p4z5yYM3uKqNBAPk6qoHt9IzT9Mdxh1jdQX0tf3AKEwPoY=",
        categories = "gaming",
        tags = "dice,random,tabletop,6,board,game",
        contributors = "mittalyashu,ericfennis"
    ))]
    Dice6,
    #[strum(props(
        svg = "eJx1jE0KwjAUhK8yZJ+Ylz9bSHoCPUSxxRQUJATU25s0C1FaHm9m8c2MT/MlI87LNebASDGkd2DFXqumZs9lyrHhQkmywR9qcfCPMUdMgd3pKHoFMtDC8vKjEspgFdmO6xu33JTYP+Ma8kQSru7Wxe/u2YG6KCRtoNIgs8cs3B7q0P+gD6A/QrA=",
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
        svg = "eJxNi0sKgDAMRK8y5ADaBJdpbuAhJAoVXEhxobe39YerYea9UZ+zLxP8iMRCyCUCwfermrY3N328wrsPv6+ftg5bwhipZwFLagJXWlc7AV1YHfQ=",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley,jguddas,ericfennis"
    ))]
    Disc2,
    #[strum(props(
        svg = "eJxtzDEKgDAMheGrhO6Nport0PYGHkKioOAgxUFvb2oHRZze8H4+z0vidQI+giKjgM+ySaZW0Vflj34b9hnGoPoOyHCtCS1a3aABQqdbNDnOUfQfUqgH/hPJ3aRAVqMFMUV38DYvo4wrgg==",
        categories = "devices,multimedia",
        tags = "album,music,vinyl,record,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "danielbayley"
    ))]
    Disc3,
    #[strum(props(
        svg = "eJxNi7kNACAIAFchLOBTI8sQCxMrKt1eosRQXXF3JENldtCGJSPINlbjumRKzzN55+KH9sXsAH3QFkY=",
        categories = "devices,multimedia",
        tags = "album,music,cd,dvd,format,dj,spin,rotate,rpm",
        contributors = "colebemis,danielbayley,jguddas,ericfennis"
    ))]
    Disc,
    #[strum(props(
        svg = "eJxVzDEOwCAIBdCrEC7Q6mA6WC9DHExMBye4famEVKcf4PFzb08FjjeGhCBBM2pGS9b5wpKPT5X8Wzf+k8zqfscyC6bV4KV2pdQG9QpDtycCsSkSV3YvL6xXLEc=",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideCircle,
    #[strum(props(
        svg = "eJxtzF0KgCAMB/CrjF0gNIgetMuUpBA9iNB2+zbN6KGXffH7z+WwFrjSVqJHMyPEkPZY2pzJo0VgjyMC1ZpZT4sbNLe4I50ByAqfRBiPkmJdNWZqF6vqsfyxL9Ld/uH2sFvqj62eO70BupIyTQ==",
        categories = "maths,shapes",
        tags = "calculate,maths,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    DivideSquare,
    #[strum(props(
        svg = "eJxVjDEKgDAMAL8S8gFpQVFI85nQoVAcOiW/NyGCON1wx5GMJbPDalgQRB3VaQ0PZNrSMs1xd9Dq9kKwmpWVpDr3qKNieo/xKCf+z9/yAeKlIPI=",
        categories = "maths,development",
        tags = "calculate,maths,division,operator,code,÷,/",
        contributors = "csandman,ericfennis"
    ))]
    Divide,
    #[strum(props(
        svg = "eJx1kEEKgzAQRa/yyT5pZsZEBfUGHqLYQgu1dNGF3r4zWkoXBjEf8n5ehnSv8/uGS+9GSuDJU5AECslziC1DNBOqkMYWzJNRrWjTqBfN5JW6oTuZaOh+OgalSYKIaC2HnGtNil5mW6bNsunkK6s27YFqphrZ5vAFXKExnf0HuAY1Ji7IxebQu+07wBzRosCyHTUzgY7m2ngs8wjK2N/hT/+4P69YuXfMDgtpOqx7LPuuVq00fACtBm3d",
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
        svg = "eJxVjMEKgDAMQ3+l9AOmrcwhdD3v4kcMFCaIePDg/t5VvEgID0IS2bdjhcoRmREqNSLcDWTklyqdtVTOfBVYIs4UwKfJ+Tw4D+b+Uyj/jCxLo13YWB/j7hoQ",
        categories = "currency,money",
        tags = "currency,money,payment",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    DollarSign,
    #[strum(props(
        svg = "eJxFjcEKAjEMRH8l5J6xSeq6C23PXvyIpQoKHmTxoH9vqgdJJg9mElIe6/NK58onS9iTptUCQylKxZDFjzqHvQx/+QeTZOS4GB0jNMHpAO0ChwlmuCgMzq3sxp9W+m3r9wttlZ2pvyqrBd9fxtIvbh9c8SHo",
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
        svg = "eJyzSc4sSs5JVSiyVTJUUkiuAFJGeiBWJZRlZ6MPUWIHAAXAC8k=",
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
        svg = "eJxNjLEKgDAMRH/lyF400eLSdnbxIwQFBdGCIvbvTYuDZLi78O5cHK8Fk6dBGGzvdhQIaj026nr7z0Zu01JwVS4FF48tbes+Ix7rfp2eOrByojvg7Av6QcEV9BFPLISk2qiwRkt4uLyVL+wLTtAm0w==",
        categories = "arrows,files",
        tags = "import,export,save",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Download,
    #[strum(props(
        svg = "eJxtjL0KAkEMhF8lbL/jJZv9g71rrH0IWQUFCzks9O3NcoUWR0ImyfBN6/e1P67UP7NjcdTfm64mk1vaYfOX9jy/bnSZ3YkrOFDEVI8cIUJ2K/FkDVUSSBxr1UEP6o8VRjZXULT7hCSeoeztMSI8J4RCNmSHLYjJ0nPsipApWVWoUCHOyD/iC1azMj8=",
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
        svg = "eJxtjs0KwjAQhF9lyT1rdvNbqIU+QB+iqKAg4sFDfXtnq1APPWwmCTPfbP+cX1c6H91USBrnOXImm0CCqRROwQvnyp165aykHJKPnIob+oOlh/6PAQQCXDNLRC5VjgU5STvuxg1VkseCvrJ2Wqt0XwoJl+g5JVQ2ASx0sNe9XlEjjHbqxsnGmQT3OGPv9duoTQGTtnHut8eFFj06VUfvny4CxXMVWM00fAC2rETW",
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
        svg = "eJwtjsEOAiEMRH+l4Q7CYgET4OLZH/BmqokmHsyuB/37nbJ76AOmw7RVXrO8HyT/ZsLk2NDcTNRTflACbr0eNlOvn9v3SfdmLpGKeAsf4Y9NlAaZPB1diRQpo8UL2hCTqMwW1mHMg/ryg1sEuNhsJxQ71nTC9J181lnBI/mEKlfdSrfpKxV2Krw=",
        categories = "food-beverage",
        tags = "food,breakfast",
        contributors = "karsa-mistmere,jguddas"
    ))]
    EggFried,
    #[strum(props(
        svg = "eJxFjk1KBEEMha8Sel/P/FRSHehpENceQlRQEHHhwrm9qZ5hhoLkpfKSL9vPy+8HvZ2W54Bl0hGfHBZKK8QHdYSTMGTN0iX1tcHmfzcS6KBEOo3ZYjKoRwNrkJfMJvA+5trhzRDqy749TOi+3dCScFMSwzr8UTp40CXy8Wb/yo3WYb02pbfiRituEy4M14BoU/CqcNbycI95YY478+vz+53OelpUF/qTypWu5fkoyzpN+z/mlzzj",
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
        svg = "eJxVjEEKACAIBL8SfiAMOgTWbzoE0bl+n2mBnQZlZqm3Ud3EDBHcYiRGEEwGJijkj1NITX3aAqMmzN9d13lJkPOUZnYDUUkgmA==",
        categories = "maths,development",
        tags = "calculate,off,maths,operator,code,≠",
        contributors = "ericfennis"
    ))]
    EqualNot,
    #[strum(props(
        svg = "eJxNjEEKACAIBL8SfiAMPATqbzoE0Tl/nwZFJxl3dnn02ZIVgQppoQD5cUJHw/gq53CUj/lnSK8SA47X3blEFbs=",
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
        svg = "eJxNzEELgzAMhuG/8pG7zKRTHLQ977Lr7sIGCqIFRey/N62CUmjywkNsaJcOP0cfbsBmrVuBoNTHhW7v6t6FfJurod3V5O0j3fA2TEMc+vGPMPXjMjviCgbCx/fK8CTeZhjFkSFsrLbUqSlMiKmfyWe7A+KtKVM=",
        categories = "text,arrows",
        tags = "outbound",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    ExternalLink,
    #[strum(props(
        svg = "eJxtjUsKAjEQRK9SZG+b7nwHZgbmAB4iqKAgIuhivL1Jx89GQl41SVX1eCuPEw6T2Q2UMxqKg4MF1+tJvMLM47Y55/HrZ0vJIZDNSx29Q6fVKAvCPrXRIikKO+KITjVtmGKCUMx/2iNFRsNSE0E02MTqkbrg7j7NAyUPRf8N5IZWzr/ey/l6xCqTETFYuarB8y39tVqbaX4BLqQ8lA==",
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
        svg = "eJw1i0EKg0AMRa8Ssm86CWY6AzOeoN12L7SgIOLChXp6J6MS+OF/3ktzt/Twy/gRR9IAS8nOgwdn9wjUxBpvBXakX459IN2xTU9T2zQO0x82ySiCsHHGgLBaLa809oYadKF1fJFilVhPOp5uqTd9ADkPJZ8=",
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
        svg = "eJxNzcEKgzAQBNBfGXLXmjVWC+q5h/bau6SCQtBgJdS/7yZNqwSSwLydrW23Dng24q5ANEjVEQhZOJTQo0yLm1RpAbqe9yjhnyPR1ic/3tZ2NpsZpx52Hqf11QjJbeCrAmWoAoyE8W8ji9IleaegQq1knrl8r9Xjok0PvXFhKbDwI6Dfjbh4803/KqbRepUf1ActjT3S",
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
        svg = "eJw9jr0OgkAQhF9lcv2t7rGckHA0Nhba0puTBBICBImRt3fPv2aKnW9nppqva4dbMBf2cK5zkXLswZYcmMSSTyLkLZUqemoOlJ9ZFHMnH23CLYNc8sgfRR8zCDI1BPLITF3tUkldzdOwDf3YYp76cb0HwwJtERRwexRv8IvUVeyXOLSIm2LeID6DKQyWYHziPq5G/taXWse6TLPYk8sbln/xC3JoOUw=",
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
        svg = "eJx1jTEPgkAMhf/KS3eQNuV0uDI76OpOhFg2Qy6o/947FhwkXZrva9+L83hP+BixEHycHp6MAuFtlMFrGpIbKWFeQRcP5b6Lzz45BqMrK2QJHooq8EdlI87aCwTNOlLJ7Vi3F9a6hZzDpqq8LfonhBuweK7YcSffvr78bDZG",
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
        svg = "eJxNTs0OgjAMfpUvvYNrMxSTjbMHvXInkwSSCQtykLe3Iv6kadPk+3WpmTtcPV0s2NS2EQiMDkMy6cq8OIvBIS9qMT8s0+9kqXK7l7xyaYxL7IcWaeyH+e6JrVL1lFB1uRI3SuVCP4XYIixK2xMmT0IID0+r4RtVy63XjTXPZDYvoPsX+cGPYNa2/IWeiDg32w==",
        categories = "files,security",
        tags = "key,private,public,security",
        contributors = "karsa-mistmere"
    ))]
    FileKey2,
    #[strum(props(
        svg = "eJxNTcsKwzAM+xXhe7rYLNklyXmH7bp7yQYdbFBKKW2/vg6lD4SRkWwptHXf4B3pydfKQe6+FghsgdFt4JOgLA3LWTDyulXusT7PlMKlBKaQv13+fZCnSOwJeVS2hC6SlKPVTmFr/7MHW1NSdI6Yw3dghmL3Fi8kLj0=",
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
        svg = "eJxFjsEKwjAQRH9l2Hs12SaYQ9KzB732XmwxgdKWGqr9e7dSlYVlYd5jx09NjmgDXQ1sbRoGQ8locMHRHeyFFU4HW7P6Z4VcZ0OVP2525aexX/s0dJjGNORHIG0EleUgtvuAO1L5ubtlPFObYyBHeAViQuzSPeZAlrCKXhJmCfRmbry8+NZ00OVS8F5GQxUGauFfmTc3jDcS",
        categories = "files,security",
        tags = "lock,password,security",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FileLock2,
    #[strum(props(
        svg = "eJxNjr0OwjAMhF/l5D2ldklgSDIzwMoe0Yp0Q1UUfp4eBySoPPj0yec7f0slYwx04m1nIQeXBIK+jVFVeQV0S2ZZAyPnXWePX/OLot+0h9Ev06XgGYiFkKf5mksgR3gE2hPu81jyRy0KuLnaffT/MhYs1UgaMGgMaxmHvsov4A3ZMS13",
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
        svg = "eJxNjMEKg0AMRH9lyF1rgtta2N1zD+21d6EFBdEFRVy/3rh6kEBeGN7Ehnpq8HP04TI3kNe9FgiKfTK9Zr4ESmlYrkEm30du3kd5JW9v+0Nvw9DFru3/CEPbT6MjLrWkq4IUqJJ4Kt4mcWFHT0IUdQ1hORk5Uf3kbq3KLBY=",
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
        svg = "eJxNjMEKgzAQRH9lyV3rDqZNIebcQ3vtXWhBQTRQKU2/3k2MEBZ2doY3a32/DvTq1IPbWhNu5x4EauJUcn25CEQxMMqgwvNS6/te/itnT/Ghs36ZwjTOb/LLOK+fTnErJVmG0JBJYEacTWCAQFD0yxpY1Ijn5KVQwDG8ZlZnNuruD3YDbX83Rw==",
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
        svg = "eJxNjsEKgzAMhl8l9G5nMut6aHveYbvuLp1goWhxMuae3lRFJCH5CV/+xKRm6uBtxRMrqYDudUNAUOYoWH3xNOBOHdJ5UNDrJtVjW/4LZy7Z0Jk0xDmGvoU0hH76WIEVL3HRQCXoFdwRZ3wYfWzBzxmTSoD/scKsRiuIO+MbxM7Hw1dJCrBeK6c+ri/2nTY0",
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
        svg = "eJxdjMEKgzAMhl8l5K5rMrUVas87bNfdhQ0URAuTse7pFzspKoX+yc/3xfp27uDR4I2KvAS+VC0Dg1peJtObNoUkd8TbIuO7zsvrX/6is6floLN+GsLQj0/wUz/OrwapEEk+A6zARHBFnI1gYIHOCB+SrCRlNwiBYi38kdV7Rif1wEpZY1TqZERSJfIHL5tByg==",
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
        svg = "eJxFjc0KgzAQhF9l2HtSsyRtDsZzD+3Vu1RpAlbFhra+fVcRZGF/2G9myqnJEW2gu4WvbcNgFFIGrDh67W5c4KJdzcXxU7JdLVXlaVVX5TT2S5+GDtOYhvwOZKyg0jxE7TdwRwTeA19GnJx2EFa7jzr3atsO27l7ZMQuPWMOdCb8AjFhEXsZs1yG8E1tjoG2jJWv/uxVNyM=",
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
        svg = "eJxVjcEKwjAQRH9lyb2xOzRaIc3Zg169FxRaKG3AIsavdzdWiASyw+ybWR/7daBbZy7cWEc47XsQqNZXiXpyYcjEwCiNCteDdedv+G2C32lh8HGZ0jTOd4rLOK+PznAjIflaQk1tBjck+AwmFgjWGXpB8azEO6pIaskhzRUZJf/3ZQ1vNb/MB6D+OjQ=",
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
        svg = "eJx1jUsOgCAMRK/S9AAaQMUFcAMPYYRYdoYQP7cX4sYYu2mavr4Zk8KSIZ0WJcIRfSaLYkSgEFfKz16gQrjqdKatgjPbnAm8xUmD2stTAfX0Agp001P3S4QkxhEDIwm2qBDNS5/AGwhUQ6Q=",
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
        svg = "eJxtjDEKgDAMRa/y6S6a2IJD2xu4uosKCiJCReztTSnq4hBeQt7/du+PGaNTbQMealQwYMgedEGQOYmUt2XSvH1lDeZO/z7IBMrRp+rT1mWbEMkpVrgyIgvSmSlqkvwNlLYmig==",
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
        svg = "eJwljDEKgDAMRa8SsgdJTMCh7Q1c3QUFBRGhDtbTm9rh8fPgkXDN9wZLxFGBLTMxqGMgMDjafOqzH0zekJHQ4GjzF1Po6pMUjv1coUhENoSHI6qP/FPcRGpao/QBOqwbhA==",
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
        svg = "eJxVjMEKAjEMRH8l5F7sDKV6aPcPvHpfVmEFEQ9S1r83sai7hJCZ4WXKY3zOcq56RBbklkYKJdogmJrDJgg8IU7RxWeSDmXnDUP59eztC2hpikKvsE0Nf/B2vV9kYVUcVF6omlUWO4BZujXUoRVKdpQdZSct/aJvvNww0A==",
        categories = "photography,devices",
        tags = "torch",
        contributors = "Andreto,ericfennis,karsa-mistmere,csandman"
    ))]
    FlashlightOff,
    #[strum(props(
        svg = "eJxNjDEKgDAMRa8SuheTUMShegNXd1GhgoiDSPX0Jq1CCeH/PB7xx3gGmFvTUwP1hMCW07qLcJQKKEMKg3UlsDwQTqgljRs4ED+m85X+7Py27gtEak1t4OYUUYIaORMVU53CJM6qZvzyzvyXX18aKlw=",
        categories = "photography,devices",
        tags = "torch",
        contributors = "csandman,ericfennis"
    ))]
    Flashlight,
    #[strum(props(
        svg = "eJx1jF0KwjAQhK8y5L1rdpofC2lu4CEKChGK+CBSb29SEBVadpmB+YZJ9+lRcB7NSS3qO4kErXg/KRR2PRmg4nxRSgx/edfyuSojO4p3NDkd2mZOv8t8UnrXb0FXYdiBR/FgiRskQkMZvmC+3i5YOBrS4PVxrW6wrFarrZTfgjc8qQ==",
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
        svg = "eJx1jtEOgjAMRX/lpu/FtXOAycYf8AO+kWmCiSSGGKJ/74YgvJg9dDk9va1/dM8el0CtKFxnYWEg6aXfwJbtDxnOKJFJ2hPqjWc1k14GB7NFrH7PMrDCTizU+ENe2Ph4G+P9ivgOVBPiK5AoYQyk2fl2G78/TswkugXsW6rxWGjaWrEUZVml6ngmXGEmqbrz3+HV5cVlhyUPS95++gOXlUjI",
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
        svg = "eJxVjkELwjAMhf9KyD1xSaVboS148+LV+6hChR1kSNF/b4tQN3J4JN/LS/xzfmW4Bbyogg7Xaa4CQytS0kwjO9NnQsLWEruFeNLa6KnboTnBnI/bBNAiJtVFFnbQgGaL0R/a2ejTY03LHdI7oFiE9KnqENaA2kw/HH3/USyIFJJ/wg6NhbSjLx1pNNY=",
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
        svg = "eJxNjkELwjAMhf9K6L0xyWa10Pa8i1fvIwoTepAhov/eqKMbgfCS95GXdB8fE1yyOx1AaOhHAQGyYi9eznslz8gYve29TB3GbkWAMQSMFY9iUppBizEIbWBTrqTdN7Akvc1ar6Cv7Dg4mLOzru/fZNDfLql9xwG4f0q1zHbkA4s+LLA=",
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
        svg = "eJx9j8EKwjAQRH9lyT1jdtPGBtqee/HqvUShQgUpIvr3Ji3YHlLJJZN5mdmtw20K45XCp1FcKZoaZRWF96za+rDYbf3onwNdGnVig5LEdEUvJGTiYS1azuWqKerBwm9eGM7Dj6jSVX6GScYRvhOz/U7ysihTfapdy+/COBJ7FBpew2YILiHELtl7hIOjFAQbmRzgEaeMOX+AJUHHQThfUaUEuwsUsDTHcERyi5q06ByzBb7aPmpl",
        categories = "files",
        tags = "directory,settings,control,preferences,cog,edit,gear",
        contributors = "karsa-mistmere"
    ))]
    FolderCog,
    #[strum(props(
        svg = "eJxNjjELwkAMhf9KyJ7Y5MrZg+uBm4urg1uJwgkdpIjovzenYCXD48t7LyTfpnuF84iHHrSrEicFhe4zSnocVibnSltO4bcTEo6ROM3Egzrobq23JIR9/38B9CHBvMjCCZqhJyx5074o2a6LzRew54iiCIsLgr1cQgt97fIGkKYojQ==",
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
        svg = "eJxljsEKwjAMhl8l9N6YZlYbaHvexav3UYUJO8iQoW9vqjgLo9A/yZ98SbwPjxEuyZwEmPr9wMBA+pxly2f/z0HzsUNpKg4PgjJhqCGvBlXjiNIztePAizc57urGHMttLtMV5mTYQHkl46o+Vbva9LVzXM9zAZwUyxiArFdl/f0SNsQPS8yPrESmhvgG/5s6bw==",
        categories = "files",
        tags = "directory,root,project,git,repo",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    FolderGit2,
    #[strum(props(
        svg = "eJxVjrEKAjEQRH9lSZ/1djfkLpAE7GxsLeyOKES4Qg4R/XsThcRjq9k3M4y/z48Ml6COBnjIZGcGhuF7rPk0da2LznpEJ+1HmtBajW7ROHERvO/x6gQ5mP8G4CdJKkEkdFABn1X0u7oi+nRb03KF9A6KRMEaFCtIr6K4mn44+jaZDJBk6QWNjFvwAR0fNwY=",
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
        svg = "eJxNjkELwjAMhf9K6L0xydpioe15F6/eRxUmTJAhov/eFHWVEBLyPfJeuk33GU7ZHJhAaHSTgABpsRUrR1/JMjJGq3cr84Bx6BJgDAHjgnvRVTZAXzAK/Yl1e4gpadc8S6qXtS5nqM9sOBhYsxED9aWDmuiDS/oFvIp6OOvQg3Z/07nG8S3Txt7IMTT1",
        categories = "files,security",
        tags = "directory,key,private,security,protected",
        contributors = "karsa-mistmere"
    ))]
    FolderKey,
    #[strum(props(
        svg = "eJxNTssKwkAM/JWQe2KSvmG35168ei9tcQsepCxV/95d0CqBMMwjE3cfY4DZ41kFTIZyNDCQNEpGdqkmIWXljhJPFgruip8FlOuauxu3lqAdgnyEweTPnNBuXGHvTrm1d9syRdieHhUh7xIhLOs1RI8VwisxDcJjnWPw2OZYDvTueNnS1Wanb6+CUAmy29HwBukqNL0=",
        categories = "files,security",
        tags = "directory,lock,private,security,protected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    FolderLock,
    #[strum(props(
        svg = "eJxNjsEKwkAMRH8l7D2xma1rF7YL3rx49eCtqLBCEQ8i9e/NKrQlh+ElM2HSc3gVuvbu2BKaomEAgZrfgHHqFmbjwjuJft4pq4TAEkeWDgbYL/HqJH9o1x8Ib/UXC4pKpHrA2eW0qS1yGu+PG03onW4dfdTUm+Kvk3Gs3urKXz7kKIg=",
        categories = "files",
        tags = "directory,remove,delete",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderMinus,
    #[strum(props(
        svg = "eJxVjtEKwjAMRX8l9L2xyZqyQlfwrV/g+6jChAoyRPTvzVTcJA83nHMJSdfxNsFxMJcA5IHQi2WMewYGp0MQkZW7wm5cIWH0wCjNEopAWJV9K6XFbyBbPkh1KlVb5ZanDmP3dzIEjA171pV/wn1FoX5T1u3OJqfd8n5O9TzXdoJ5MGSgPjS85lNTltJH5xeKQTNW",
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
        svg = "eJxNjsEKwjAMhl8l9J7YpLOu0A28efHqwdtQocIQDyLu7U22sY0ePv7m+0Pyu/sUuDfuXIH4wrETEPDjE5RLvWbUXPBAKSx/jEwxIqUeqRYNclzrZkI4VdsNIF8ONy0SUwIbyNW1eWdXtLl/vh4wcOPYOxhEGR38jKLkkSqbtpXDLIdJSnNnv7h/XoEztQ==",
        categories = "files",
        tags = "directory,add,create,new",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    FolderPlus,
    #[strum(props(
        svg = "eJxNjkELwjAMhf9K6D1xSbe6Qlvw5sWrB2+jChV2kCFD/72pQieBwJe893jhMT0LXKM59SBdYTcJCHTfEZTzuDEqF9yTt+3GyOQckp+RRlGQw2avSrDH/j8BZGWb1UhMHupDLiaFXW2RQr4veb5BfkXDYmCJRnd+K9kq+r1TaJVZgId1aAEfEz4v8Q==",
        categories = "files,development",
        tags = "directory,root,project,git,repo",
        contributors = ""
    ))]
    FolderRoot,
    #[strum(props(
        svg = "eJxNTsEKwjAM/ZXQe+OSbt0KbcGbF68evI0qVNhBhoj+vWmFbgSSvOS9x/PP+ZXhFtS5B+4y2ZmBoavFmi/ThrXgrEd0pt1IE1qr0S0aJxbAx01emGBO/d4B+E0miRAJHZQHX1X0h5Ii+vRY03KH9A2KGAcFa1B1po9cSDah/knRt+BkkEegvvYByDbDHwP3Mqg=",
        categories = "files",
        tags = "directory,search,find,lost,browser",
        contributors = "karsa-mistmere"
    ))]
    FolderSearch2,
    #[strum(props(
        svg = "eJxNTssKwkAM/JWw98Qk21YXdvfci1fvZRUqVJAion9vqvZBCDNkhsnEe/fo4ZzcUQSU26pTUGAbQUU91YVRSCig3VF7T8GvFhBqGgoDHdSoLgL/hVZ5Yzb2rFyOu+lnjuU6luECY3LeQXklJ3vD9xfN9JNznAve1BLEytTTLjEfYfAtig==",
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
        svg = "eJxtjsEKwjAQRH9l2Htqd8M2BpKevfTqvaxChApSpOjfmyDoQdnDDsyDeek23wtOmSYWsBx0Fgj6dq6mTWlMu8aMyS6rLWfYMxNHgj3q94Q1kzToXf/F9Jf6zO7BsfirOg4bhzJMQ9U4Busdd9zFJuGk+EXxVXkBMCwwJQ==",
        categories = "transportation",
        tags = "vehicle,transport,logistics",
        contributors = "ericfennis"
    ))]
    Forklift,
    #[strum(props(
        svg = "eJx1y8EJgDAQBMBWji1Ak3voJ5cOLEJM8PKTcKB2r/EliJ9d2GFDzYuR5rKqCTyDTsEA2ksyFbAD1eNu0JMx9O0QwzabUhJMnsmzds43a+vbxn/70AWHayZV",
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
        svg = "eJxljTEOgDAMA78S5QMoGRBD2t8wICHm8Htwkg5VJ8vy2bb7ek5yaazK9Grjncl/gRO4bhuYbkGCkCMzaKE1MLMe/ahozWk2ltWMBul14nky2A+w8StR",
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
        svg = "eJxdy1EKgCAMgOGrjL1bTTII1Bt0iLCgICKqB719m0YPPYxf2Tcb1jNsM4TkkDRCiKUnp0Fv67L39hjvBSaHA3VA3aWoMkqrVgbyCBbk7bbuM0TtsK8aQkjEL47OifnHVtRrf4JDJp8KJvPpB2DALC8=",
        categories = "emoji,account",
        tags = "emoji,face,bad,sad,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Frown,
    #[strum(props(
        svg = "eJxVjcEKwjAMhl8l9N7Z/G2lhXZnD3rdfaBQQcSDiHt7lw5qRw7hS74/SY/780YLZwUo+iIr9mtf2SpaUMdjOog1pupWx23LuKlCLNTM1/wudM3qwo6Ayc0gkJHS0DgdeyZ8OEhUQvso24KmMon6ZyNcTM8aUxyC7a4PPmoeHM4cyLcnP6mWOTA=",
        categories = "transportation,maps",
        tags = "filling-station,gas,petrol,tank",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Fuel,
    #[strum(props(
        svg = "eJxVi8sKwjAQRX/lMvuMmRmkFZL+gVv3khanOykB69+bVBBc3AccTtqWUrFnMsL7aF/Wh9dMMhJe61z9e7dGtc3eZ0qn7k3pea+OOdP1AhmKIkJ5DHJ0y01iiUEhwdhgrMG6261/V1j9zMMPfgDYQicD",
        categories = "development,shapes,maths",
        tags = "programming,code,automation,maths",
        contributors = "mittalyashu,ericfennis"
    ))]
    FunctionSquare,
    #[strum(props(
        svg = "eJxFjMsNgCAQRFvZTAMKfg9ABxZhhLjcDNmgdi/ERG+T9zLPHKsweYtF05RVC2eaipz5xEhDVv0vUtiEbosOxCHuLBZqBqXLQoPO6IULKbOAN1gf7gFgRh4P",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryHorizontalEnd,
    #[strum(props(
        svg = "eJxdjMsJgDAQBVtZXgOSBCSH3XRgEWKCm5uExU/3JifB08DADB+rKWXB4imcLiLxNFTiVjajq2ZTgfOgRxBAWuqu1k0E3YIZ1Dr8yEaQ+Pv9hi9dvh4Y",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryHorizontal,
    #[strum(props(
        svg = "eJx1i0EKgCAQRa8y/AvEmIsC9QYdIlIadyED1e3TVhG4evAez5W0KZ05qnjwBLo9RpCkvItWY0HXa0qFQXBDG4I7VhWKHoslw8ItNPUJcy9wd+Hf8wDE8Cxn",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,album,portfolio,preview",
        contributors = "danielbayley"
    ))]
    GalleryThumbnails,
    #[strum(props(
        svg = "eJxFi8sNgCAQBVt52QYU/B6ADizCCHG5GbKJ2r3LRW6TmYy7dmFET9sCy6an4LqqgvvDhJnN2EJJh+DxNBBeT7rgzlFYcSUUDZbAKZ8sqmz96hE+R68d8w==",
        categories = "layout,design,development,photography,multimedia,files",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio,history,versions,backup,time machine",
        contributors = "danielbayley"
    ))]
    GalleryVerticalEnd,
    #[strum(props(
        svg = "eJxVjEEKgDAMBL8S9gNiC+Ih6Q98hNhiepMSUH9verOnXXaZ4Ws3pSzYIgWdVySe+pS4lcPortlU4Ds9gghqHgGkpZ5qfnh/BUvHOpD47xuEH1X8Hfw=",
        categories = "layout,design,development,photography,multimedia",
        tags = "carousel,pictures,images,scroll,swipe,album,portfolio",
        contributors = "danielbayley"
    ))]
    GalleryVertical,
    #[strum(props(
        svg = "eJxtkMFqwzAMhl9F5G7NkiVbgTRQdtlhe4iQDTooY4cd1j39pDi0PZSAf0X+9MvSdP78+oALHQaiAX5dqwv7bx7gwlt6np6CmqfORrJ0yHqJSziMj0jud6GBku72ivmhc9452zkLrlvkG/+9/Jzg/TC8UcPCoC8Vqy0CAjm+VHBsBgV1XBPmXDEre0BImUIbkvIzY80CIwpVYCBB0S2oS4HSnVzLSq6EmlCBE53JCyRt55GdD4zcxti89iRYRJbbxQaiWn2lBmQrqvt4OjrdN0plze6q4o38Xal6jXhomhqyWgzS/NAYwKfog7gqHa+Tw76Qv9hVbGn+B4O3aO0=",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,csandman,johnletey"
    ))]
    Gamepad2,
    #[strum(props(
        svg = "eJxtzksOhSAMBdCtkC7gPeovDIDNKBES48CQiLu3hRonjqDNuc21W9qDKp0D1KAufjtQBR1MNGIdvf2z8rZZWhqhgxiK8gnzQXEU0zeD40+jxPsvb8SZ5nj1eHz9EeasrtryKA6ocwxpjVn6182ZlhzppznGAX8D3do8zw==",
        categories = "gaming,devices",
        tags = "console",
        contributors = "ericfennis,johnletey"
    ))]
    Gamepad,
    #[strum(props(
        svg = "eJxty8EJgDAMheFVQhaQVKwV2m7gEGKL6U1KQN1eoyAevH7/e77mWWAP2CJsJQkHJIdw3MC5LCyP1GtjMPpGD9GvkzCkgOMAjnt1lY87IMP2JxABWe7ecgJ6iyT/",
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
        svg = "eJxFjEsKgDAMRK8ScoBoqrUIaW/gIUoVFFxIcaG3N1WwMJ/FDE+OeK4we5zY0gCGXOQWVBpqS64YgzTlGCRtOe0LZI8GIV0eufT9tp6+OUildtQrTtE8gv05D/y3Hl0=",
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
        svg = "eJxtjkELgzAMhf9KyH2uySy9tF522WU/QqasBVHZCtP9+iWVXURKXyB53+P5eRrWIY09zFMa8zsgGyAGUWaoNyHGxp//zsa/+keG2KdnzAEtwiIUwhrQIXxSl6OmKKLGxpf4hQOSukiOOrnYFypr8W7Rc5sjdAHv0sHdXGVbrizoN/IIzMleiYC1oyvyLd0E28Gx3sFmgy/H8A/7B0M7",
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
        svg = "eJxdykEKgCAQBdCrDO6jLAoF9QYdQkxIkAhpMd6+Gd3JLP7w/zM5PRFwteIQgLJFpdgoqJS7cGZm40xIJeQIpY2hNhqQjGLT11FhV3VUr/9uuKw4pQLtNWhY6OREHzOe3Q+8Iymj",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitBranch,
    #[strum(props(
        svg = "eJxlyzEOwCAIheGrEC7QQNOhiXgZ4mBiOjjB7VuxTk5v+L+XtHZtBbrgiaAuSPytxeZ0zJxTq08Bo1DGgjeC08TOCw/00xWHZcK40rXhF4YDIQw=",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,ericfennis,johnletey"
    ))]
    GitCommit,
    #[strum(props(
        svg = "eJxNyrEKgCAUheFXudw96l7BDLS5pbVdLDBoCImot08rSs7wD+fTbg5umSAYFAjuMEgq9rzb6vK5W/2ydEjMtMzRajcPo8GeBEgvLANDFUexvNeJJpJDAlKd+mXBBQ/NJy/EPCpz",
        categories = "development",
        tags = "code,version control",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitCompare,
    #[strum(props(
        svg = "eJx9jDEKgDAMRa8SsosmglRoO7u4upcqKDhIkaK3t60oHUQyJOG/96VdnF0nsKdCEgj2CJsRnMIatSzvWMsHC3GDiW5+odT1TW1mn2FU2JOA1pNhYKjCUBGuTuR/wUMb1ajkIgOxr9/kAj7XNWo=",
        categories = "development",
        tags = "code,version control",
        contributors = "ericfennis"
    ))]
    GitFork,
    #[strum(props(
        svg = "eJxVyjEKgDAMBdCrhFxAq1AsJLmBq3uJgoKDFAe9vWm7WP7wP/xHeiQ9N0iMI4I+jG6yfksLdfUWapndvmr/R1e8d1gZZw+DW0IMEKAvsZVhBvIBXeofdw==",
        categories = "development",
        tags = "code,version control",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    GitMerge,
    #[strum(props(
        svg = "eJxtzUEKwjAQheGrPGYf5aV0aCGTG7h1L7FgoYoUF8ntNa1gFq6Gme+HCWle0zIhFRMOgtWkE6S8bTEcd46hyXR3/cZN9Ly8brianDiAPPRn9pXr+Yd3T3ROof9pdOoaWubHhEKTUVC8iacgc/uefR2fsjbxDUTJNxo=",
        categories = "development",
        tags = "code,version control,rejected",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestClosed,
    #[strum(props(
        svg = "eJxVjEEKgCAQRa8yuI+YIjFQb9C2fZiQIBESobdv1EW5evx5j5HGBeMtBMVGBiYqhoKYCrXsq9ayzUjzWvN/dG33AbtiCwrg65RVPrUC8enwU96dFiKWh3EoSLRmAq2hlLnRLxyaLxk=",
        categories = "development",
        tags = "code,version control,draft",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    GitPullRequestDraft,
    #[strum(props(
        svg = "eJxNzOEJhTAMBOBVjizwXlKoCm03cAipgoKIiEjd3rQqSH4cyX3ExWmL84B4euKasHkyhJjKFtzvroN7mRaWirYP/qC120f0nlo2sKPpBIK/DmvKUWWaSXDztAxIUp6cGsKa7KkhJM5XldmEC8zSKr0=",
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
        svg = "eJxtzUEKAjEMBdCrfLJPNWnjKLRzAw8xREHBhQwu9Pa2VKULCYGEvE+yX1e/neHPQjuCvwqJEdZCiea86dc5D0r2n/MXD+y+PC44FTpKgtiiUGxbsXKbfxu0pZoeMhoMEmGYPEwsIUJCqjbyXy2dy6F67oHaVl+NgTfA9Tf4",
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
        svg = "eJxtjUEKgDAMBL+y9K42UW9tf+AjpAoKIiIeWl9v2t5UAhkyWVjj19NvM06rSCv4IGRhzHSmKX9ntnWfEcgq+UYuqUiFQW7O6ZRy5hivBZNVAzF4pL5ukZeWIXQgjZes/mX1lUjyTl2pxT3VuSv5",
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
        tags = "school,university,learn,study",
        contributors = "Tummerhore,ericfennis"
    ))]
    GraduationCap,
    #[strum(props(
        svg = "eJyNj70OwjAMhF8l8o6pkzokUtI3YGVHAQmkDqhigLfH+UF0oFUXy/J9dzqHx/l5U5cIR60Vn/S4Y3Re5QFD2Gd1COk+pfGq0jsCFUWll6wWLagpgslkZX6sAA6JoLgO2C+S5VwDNRquBiK0/C9WNGnAzWDQ05ZgafIN7rDzq7BFto01qNdzO3QNlQ/7hb7kYeaZf/UBh3Vofw==",
        categories = "food-beverage",
        tags = "fruit,wine,food",
        contributors = "karsa-mistmere"
    ))]
    Grape,
    #[strum(props(
        svg = "eJxNi8sNgCAQRFuZbAMGuHjYpQOLMEJcboZs/HSv4IXLZGZeHte8GW6hQHh61m94guayqwm5mXCVZNpr5KkJkY/VFEloCXBef9K+gTiPcA7kBVn9Hj0=",
        categories = "text,layout,design,shapes,maths",
        tags = "table,rows,columns,blocks,plot,land,geometry,measure,size,width,height,distance,surface area,square meter,acre",
        contributors = "danielbayley"
    ))]
    Grid2X2,
    #[strum(props(
        svg = "eJxty10KgCAQBOCrLHuBMBES1Bt0iEhpfQtZ+rl9az4Jvg3zzbiSdobHo0aglA9ij2pBuHNkavH9schmxuCmegju3Jggelw1WJKVQK06UGYsFvQ1BGV6+QDLUiyp",
        categories = "text,layout,design",
        tags = "table,rows,columns",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Grid3X3,
    #[strum(props(
        svg = "eJx1zjEKwCAMBdCrhH+BYsGhEL1M6FDo5NTevhqrIuqU4b/kh+UKcp8kr8MBCg4GJE8cOzxvOfXcK82LnqgY23pLV0bUNakydnErVf1gVdm6EsoPNPUBhLBBeg==",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis"
    ))]
    GripHorizontal,
    #[strum(props(
        svg = "eJx9zT0KwCAMhuGryHeBkoKDEL1M6FDo5FRvr5CAgj9TCHnCy/Jm+R6XIwhO/ojQRmnbjcSXXhObGu4e+nNEFLaqB8kfisY0aHqRHFszq4oWQXo=",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "ericfennis,johnletey"
    ))]
    GripVertical,
    #[strum(props(
        svg = "eJyNkMEKwCAIhl8lfIGxwENgvYzsMNipU779opINGrOTyP/5iRKfma/D5Qg7OC61+FolAkKiraeJBtXyoHkfmqnHhYbKg8LSuj+X7jUwtCkZsrKOha8LXsD8jBupP2Jx",
        categories = "layout",
        tags = "grab,dots,handle,move,drag",
        contributors = "koole,ericfennis"
    ))]
    Grip,
    #[strum(props(
        svg = "eJxtzrEOgzAMBNBfsbKbxodQhATMXbp2rwKq2SoU0fbvG0e0ZWA5eXh3cve4JaWxd5eawrWJnqWSqmUQGAo3dCcTQ/dzEqhWxMzIE6hqc2A9khCSsCJ6ypgN2qzykQ0EOTeRyyyD7QXGurfLFBMtr96Jo5zB0bvkcx6Tlkun+a6pd42VjG+lv7C2L0XB3m+739oHCsxJbQ==",
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
        svg = "eJxljU0KgzAQha/yeHvbTAghi8Qb5BClSsdFoUioenujIIIym+F97yf+XkXRJWaxsP/ANj43qY0H+IqHbxxcU+/EY/8uGOdES0xDV7R+htB++GhJDMSSKI7YLTW2Bc7W7CFBH0bug1nMla0bWS4G",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveDownload,
    #[strum(props(
        svg = "eJxtjUsKgzAYhK8yzL5t/hBCFok36CFKlf4uCiLBx+01gigisxn45hG7T1bUiX/x8A9XBMcqvgqo4o7fYmGHcIC++WZMiZYY2zrr6gyhTfvTnBiIOVEc0W+ZtVYKpz0PCfo0cndlrmwBAZMuBg==",
        categories = "development,devices,arrows,files",
        tags = "computer,server,memory,data,ssd,disk,hard disk,save",
        contributors = "danielbayley"
    ))]
    HardDriveUpload,
    #[strum(props(
        svg = "eJxVjk0KwjAQha8yZJ8xb2ynFpKAOzceoqBQoYgLKdbTm5hKLLOYv/dmPj/d7ldaJBiIoReCkZQXrH2ai4l+l1XRP4bnSJdgzi03LbUMkBBk1kFS4b6RqhGbgZXZ6mT3yWOVD/2x7qDcKTWnjqWpFgvuekI6/86/89foK6cWLmWHlVQLuVbSjfpfA1fccNn+038A4QA8mQ==",
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
        svg = "eJxdjEsKgDAMBa8ScgGb+EEh7W1cCOK63t6Epxa6Gkhmnp3HtVOVzBNT1cyamG7n5pBAsSGcYjA/xX8yQw1ioXPDSZhdkYwoVDr1PSJZfjdKabMP10greA==",
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
        svg = "eJxtjDEKgDAQBL9yXB9NLioRkvzA1l5OIYKFBAv9vVELg9jsws4ydh22AKPDrgJFwaC35TV5mwPTNz9A0YfwHHmZIDokBN4dqjb1kfo+PTgTkAQlWRCQ0KCLOuVrOwEixCou",
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
        svg = "eJxdTUEKwkAM/EroPbGZ3XQrtAUf4CPKKigU8eCh/t6EBREJkyHJZGba7o8r7Tp36OjdaIdTjI2X6RCiZXqurxtd5u6sgxi1BoJuXLhUVjFHNk6cBN5NbHVQoPdSgmjmLMnCNOx+TEcpAyXRqqImQIiTlNF3yKRypBZgfiiZQdlHnP4CQKNY7eMZLk4UFgwZiqu0fHM/aF42Mw==",
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
        svg = "eJxtkU1qxDAMha8ispdqyf+QySbrOURJCy2U0kUX09v3yWFmNoZYcvCnp2d5/Xn9/aC3y3LVKpmy5F07VbKAv06mpHqwSrQoVjKblFQkdk6i2sSsLNv64iLb+pCCCrncXgli/RRThdohLoFzzqzcJE+qhxEPh6MUOGEV1glqAbIA4RKkURwxzWUHEg5P+iQR4UZHo8jYzqvNyAyGnALkFzjLJmyS2iJ5tF0l1EhNUjd0VW+I9u1Qxji8lzvncmYQCSv7gK4+XLyD5Lq79dpAxAgmS8oRB6F1KBHGJK3hgzHMW2pReKtWsC/anv6+Pr/f6WaXxWyhv3tW5IVuIwF1aPsHpnB3eQ==",
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
        svg = "eJxNjsEKgzAMhl/lJ/e6JkNXoe15h+26u3SCghtDxqhvvxRRJJDkIz/h85/uO+AZ6C6M9se2EwisFhvdrvWRjTwODOXhQtGfyo/op/HdI3MgbgiLzpqQJZCIohTUaAlFn8Y5TT1SDtQSZs0Q0lJAI+sx+s3spWZcm3NlXbP23cEaqZw42FsD4V3lD2n+M8A=",
        categories = "photography,multimedia,files",
        tags = "remove,delete",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImageMinus,
    #[strum(props(
        svg = "eJxtT8EKwjAM/ZWwe2OTNbJBV/Dmxav3ocIEEQ8i7u9N2rJeJPBeGl5eXuPj/rzBSlPHHXwrsZLyWjjFnYlSfM3vBa5TdyKPgSDjzMDgQV+Ocegz2IZpU9zMqUephlTulImd2rcLRa8zkqoaahzdyj7/4qjTCAYHwnGEDD5XD/KhUDNaabdQuKCIhUYv7JBZu0BODVryzZz1a3KW5uHY8bEpf2wXS9E=",
        categories = "photography,multimedia,files",
        tags = "picture,photo",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    ImageOff,
    #[strum(props(
        svg = "eJxNj8EKgzAMhl8l5F7XZLRWaHveYbvuLp2g4MaQMezbr7VOpdDkT74/JPbdfnp4OLwxAfG3bhkYZHokUnZRRy34ftCQdF+jt6c8w9txeHUQ2aFCmMkhaYRIRaYqc0YztKK5SE1hGBenWZ3NjoZhCmMH08KE6DA5wpxDQkrT2/8Rz3yEEudKGl3+bV0puDJsQF41MG1b/wALxj6d",
        categories = "photography,multimedia,files",
        tags = "add,create",
        contributors = "mittalyashu,ericfennis,karsa-mistmere"
    ))]
    ImagePlus,
    #[strum(props(
        svg = "eJwli1EKwyAQRK8y7H9S3dBgQT1BLxGMVKGFIkLi7eNGFhZm5j1bYqg4HS2EFPMnVUfaENrdlD4w4ch7TaMvTRpvH+J5G3IJ34jQuVcfbzo0CR0Zo7f/rSbsjn6soZ/TMiuzjr8xGEpu4tmwgXqvYC2ySP4CgecpgQ==",
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
        svg = "eJxlzEEKgCAQBdCrfOYCMROYC/U2LQRRoRZ5+1RKsFafGd7/JqdQgo87cvLxPCyt0NjAghWsyJnlFc50V8QSC+FiS8I1212zcP/Xwger2ahRnelvr6V+dvXAN/DcLqk=",
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
        svg = "eJxVyU0KgCAQQOGrDLOvHIloMXqDDhFTYNAiJCJv7x8Irh68j+Xycp/gDZJCkD9Vp4ZSy1N1y8/+OjgMbqSBlm+YM+bZ0+pGRY0i5iMaTA==",
        categories = "accessibility,notifications",
        tags = "help",
        contributors = "colebemis,ericfennis,danielbayley,karsa-mistmere"
    ))]
    Info,
    #[strum(props(
        svg = "eJwtjVEKwyAQRK8y7AFs1qSmHyr0AD1EaaQKpZQgVHv6uhr2Yx6zM7t2D4+M6kgT9uLo3KR2+aYtx+ZPhBjSM+bBRaLenqTn7eeeIzZHNzZgVvN6XbBgAstoZWZcMJzuHaGfHJCqt6/0DqjsyKj2szTgtZMexISqx7Z1JO3/CBUsAA==",
        categories = "brands,social,photography",
        tags = "logo,camera,social",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Instagram,
    #[strum(props(
        svg = "eJxljTEOwCAMA7+C/IGSqAxIKb/pUKliht+TBAYkJsvJ2Zb/q2/o9OBG6OzS1FFWVUsRRS6DijhqxwRPcFzsivLB2jPNoox9ZUMHYdIgpA==",
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
        svg = "eJxti00KgCAQRq8yzAVCizRQb9AhIqVxFzL0c/scF9GizePjPT5X0spQLo8aobJHuBsp5Y3Yo7IIZ45MbQbXySG4fWGC6HG2YA4jXszHK13D8BfGGqY3PFQpJP0=",
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
        svg = "eJwljEsKgDAQQ68yzL5ja8fPou0NvIA7qUIFF1Kk6O3tKCEhi5e4c7kSrB6nFsxYbNRAPTEYUeKibLIS7WGIVfXSUwdiXQmtWPGMwTVyE1zcczw2yB6pQ4iPx+Ert0dTNwL+SHgBC4wedg==",
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
        svg = "eJxNjEEKgCAURK8y/L2GikmgnqBLxC8oMAhpUbfPTxAtBoaZx4u8VS4L+E5kvPYEvhIFKTWRDDl2L5PjMZ0r5kS7NbBq0D1aBJDjd4sIzQEHN1qLUJRT7gMfJggehQ==",
        categories = "security,account",
        tags = "password,login,authentication,secure,unlock,keychain,key ring,fob",
        contributors = "ashygee,csandman,mittalyashu,ericfennis,jguddas"
    ))]
    Key,
    #[strum(props(
        svg = "eJx9y0EKgCAQheGrDHOAmpEwF9oNOkRUNO5ChPL2ZUS00dUP7+PZsM4RksMO4fBLFIeKEM47COFNeiKr3yQ6ZI2DbfNvsPsUBRaHowYjDRFnyuOPmCrWVcyUzQCr4k/VUFewB9bC9MkFBEROmA==",
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
        svg = "eJxtjUsKgDAMRK8SegFNFXVRexsRQdqCLuztTZr6oUoWAzNvJmZd3AQRR6W1gqhFD1YkJb9R1lRMWSMsZYjCdCpVcRC0K1D2sBaU9armTgFz2D6D9x9M/s/y8FmUkddy8GucvYPgF7dvlGmgq6GHFnrGMmBPbWFEGg==",
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
        svg = "eJwdzMENgCAMheFVmreAgh48AMsoERLjoWki3d6WU5O/X5u4nkKsGRH09UtaRjhAFnZQq/1uYsWWI2MD8XBZ0uJ3JT39raTR2mrCp8swv2mY2ayr8gMdjBw1",
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
        svg = "eJxNjE0KwjAQha/ymH01L5KgkOQGXsBdiYUWikhx0dzeSbRaZvHNz/cm5GnJ84AlCo0gF6VVro0pHD/3FJ79a8Q9ypVn8NR7eBgtdh4Ou6lzI+2tJmsihXl6DCiMchEU27BWHAy1a3t1q7W5f4nuZ9HJ98tmvwFJOi6S",
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
        svg = "eJxlzlsKgCAQheGtDGcDYVcCx82UpK8iZLsPrbCpZ7/fMzrYJdLB6EDO+s1Fxgza/RodYwKFxFCglIXRTeZGl+h+qfTph8JVj+tfWb1GKlLtf1JmGY1iovpy26c6ATH7PT0=",
        categories = "design,layout",
        tags = "masonry,brick",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutDashboard,
    #[strum(props(
        svg = "eJx1jl0KgDAIx68iXiBGQS9ul6mRex1C7faVrULYXkTx9/+gHBeBPa3CHmcEjmlj0fXwOCIUnfk6HAYabjyQiizqJsv+nlZV0WZieX6tsGr6dep5vKoTH9s9PQ==",
        categories = "design,layout",
        tags = "app,home,start",
        contributors = "zenoamaro,ericfennis,csandman,mittalyashu,danielbayley"
    ))]
    LayoutGrid,
    #[strum(props(
        svg = "eJx1jEEKgDAMBL8S8gGtVkRo/IGPEFtMb1IC1t9LCooeep3ZHZfCJnAR9ggpExqEM3phwhEhF8wh7iwKZtfofHblVN9pz9gn+L6OVRg84WIsWC45RX8x1YQZaqZrP+YGeJs7bg==",
        categories = "design,layout,photography,text",
        tags = "todo,tasks,items,pending,image,photo",
        contributors = "ericfennis,danielbayley"
    ))]
    LayoutList,
    #[strum(props(
        svg = "eJxlzFEKgCAQBNCrLHOBiIL6UC9T0vorC+nty0UT8m+ZnTcm+kMoWyygmCxmEPtwsbznDkr6uMMpbLHBman0nVH1xdUXvg4zf9XzBvpOpVnzxh7jxi4g",
        categories = "design,layout",
        tags = "app,home,start,grid",
        contributors = "danielbayley"
    ))]
    LayoutPanelLeft,
    #[strum(props(
        svg = "eJxdjEsKgCAURbfyuBsIKbCBupmSdCoPeu0+PynS8HLPOSb5gyn4eAW20KA7nhws1A56LFZQkrxAUoYzS+GdqVb+1TaAqVHZXtI/S5r1yQOa/Vbs2gveyy4g",
        categories = "layout",
        tags = "window,webpage,block,section,grid,template,structure",
        contributors = "danielbayley"
    ))]
    LayoutPanelTop,
    #[strum(props(
        svg = "eJxdzN0KgCAMBeBXGXuBkP5h+jIlzVsZVG+fWgnzZhvjfIei3wTYh4PF4oxwW+wRrjJjWgbhDLtwuhZ01OW8o6JUSFeYobK1Ud97VCR3TFW+lT97AOwsLiI=",
        categories = "layout",
        tags = "window,webpage,block,section",
        contributors = "mittalyashu,danielbayley"
    ))]
    LayoutTemplate,
    #[strum(props(
        svg = "eJxNy8EJwCAMheFVQhYoSS8V1GVaqULpQYTq9k3Ug6efPL7YHM4CMaQ7Fod0IDSHO8KXrhLHUPuQJSxpGm83/fP2SW+ASl00dmiES5jkJD1FqlnkJGaILnmVP2uFJpU=",
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
        svg = "eJx9j1EKgCAQRK+y7H+WKZSg3iUsKCgI6aNun64SQeHPLMs8hhntFu/WCdxlkLcIPpwGwZ30Wl0n3+p9OGYYDW6SKQFZWkkSuei/KC5ZL0Ax3hFRFbGkpTgKIqz6wfKG0F3is+Qz4QY0fD0Q",
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
        svg = "eJxFyjEKgDAMheGrPHoBTbBEoQpuLh5CUFAQcXCwtze11pLh8YfPndO1Ym7N2IBkkN7CotQjCMR0rgigcz8jC1l5yqxCndm+HQs8tYbY4NatdTimj6s2qGT1yZ/laEP6NxN9AD9aKPM=",
        categories = "text",
        tags = "unchain,chain",
        contributors = "ericfennis,csandman,karsa-mistmere"
    ))]
    Link2Off,
    #[strum(props(
        svg = "eJxNyrEKgDAMBNBfObqLplCj0BbcXPyIgkIFEQcH+/cmFEQOchx5/kp3xhrMMoJ45snBoZMQGJytib5VEv0HyekjVUgSuV1ufvTYzw2FgiFrUGztR/YgpbNXqyq+/B4fsw==",
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
        svg = "eJxNjFEKwyAQRK8y7L80LsG0oN6ghxAjVehHEUmTnr6uhVD2Y3Z33ox9hZaxOrprg2swMJj66K5mW7KaN7UEBo/vpFjJfl7gk/lLKvMhby/S7G1NseFd1pYdzYTdERMORzdCTuWRmyPNggvobSw1PhPqwOI+MvEQ6cjP9F9A+i3e",
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
        svg = "eJxti8sKgzAQRX9lyH7a3GkaXETX3fQjSluIICIoon/vJIuAj8Vw4ZwzoWv7P61SG29oRZ5FB1ZXqcA04Z6iJpQUcoryr/JDnGB1EUvmJR4+U6Rfbd6OfMTskklsZ2CjXAhPqF7uaxkkLHqPkcG4PVkYpd8A+Dg7lQ==",
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
        svg = "eJx1ykEKgCAUhOGrDG8fYZa5UG/QISKl5yIIEarbp6taFLMavt+ksGQc0We2pAinJUm4LA2EVI4gcIgr56rOtDV3Zp8zw1vaJMSIrqxv+soVHp6EhGL9DaL7Ff2SG0EyLPw=",
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
        svg = "eJx9jkEOgDAIBL/S8AAVmpge2v7Gg4nxbH8vFLRq1BOBnQkbl3mdXMEEI7iNEhDy5DWAKyTXHHthclSSb0hQDZmGmvnCtlCdcDgPVhvoy0p4FX034EcHo84qP3S4xDev9dgB3opCcw==",
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
        svg = "eJx1j0EOgCAMBL9ieABKNaAJ8hsPJsYz/F5qiwjiqZtmtt21x35unYdVKBCdVzRDnDjiWgtne4ScJZSRZEFUzcQCVDAujTSacFbRMMllJOutqg94eZGD4eNawsTRSIYH+EZLiTihZhpaLebMBsieZouU4qdGM07ZI9teNYr6F6IcYOM=",
        categories = "multimedia,layout",
        tags = "load,wait",
        contributors = "colebemis,ericfennis"
    ))]
    Loader,
    #[strum(props(
        svg = "eJx1zVEKgCAMBuCryC4QMyIC9TLigyA9+LTdvtlCpehp6P/tnyv5TIbQgwVD1sMGhmWgPBnvGdzSUHBKJbR2oLaKxw9mLSR87FT86cXp6DFZvddxzDWWZCJpHFln9bA3pHFn8r3CGw92ARt6Qhs=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,danielbayley,jguddas,ericfennis"
    ))]
    LocateFixed,
    #[strum(props(
        svg = "eJx1kN0KwjAMhV8l9L5xae1PYNuN1z6ETEFBxAsvtrc3aec2UCnkQPrlHJL2fntcYKTOOAOTCKm6qqNoMH27U6hvF5S4/rnvmR/wDM1+Hzb8QRdmzplqzgI/T68rnDtzTEgEWg4Bs4eMniEANUgqbmjAY05SyEMqj5BBu5GsTuwxs3VioOZquzGnjImAPHIckNhi9OjYkpSqHKGxGmA1wKbyMCbpEnrpFUbY1bysOW3Optuuh9ks+QbDO16W",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "fdev"
    ))]
    LocateOff,
    #[strum(props(
        svg = "eJx1zUEKgDAMBMCvhHxAXCgitP1M6KFQPPTU/l7TgBXR0x52svElH4kaAjumtgYGU79i1cTI6BdF0RvVcr/LcYp59MKYS84M7M0/fezjA0uuUhJJMy7dsgbeFFkdT9nONw8=",
        categories = "navigation,maps",
        tags = "map,gps,location,cross",
        contributors = "csandman,ericfennis,karsa-mistmere"
    ))]
    Locate,
    #[strum(props(
        svg = "eJwdi0sOgCAQQ6/SzAV0/AQXwA3cuidChJ0hROX2DqSLtumrzuEs+AzNhCw2Ed7kSzTEGyGGdMUimWWtfa29WT20o9W3KxHe0K7AfCi3YsUoYrDYszSyMfYH9q4b1Q==",
        categories = "security",
        tags = "security,password,secure,admin",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Lock,
    #[strum(props(
        svg = "eJxNzEEKhTAMBNCrDNnL/4mKm7Y38BCCQgXRgiL29o7FhWQxYXiJS8MRMXrptUUdm8Fg+HOUaad+iopbrBoJ7vccBZe2JS/zOiFt83rsXpSqAx+pgXtX6IuCKzQrmQmuJ1umeakF2UpNX+wNS/Umyw==",
        categories = "arrows,account",
        tags = "sign in,arrow,enter,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogIn,
    #[strum(props(
        svg = "eJxNjLsKhTAQRH9l2F7u3cEHQpLaxtZeUFAQDWihf+8aLGSLswNnxsX+mDB4aWtQm6IniL+dZszYfTIsT7kE93sqwcVtuZZ5HRG3eT12L1pCKxuBEvZXSX2l4JJ60kstONULVXAZlUYmmp/cGwPOJlg=",
        categories = "arrows,account",
        tags = "sign out,arrow,exit,auth",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    LogOut,
    #[strum(props(
        svg = "eJxFjF0KgCAQhK+y7Hs/YxI+qDfoEGJBQUFID3X71IVimF2GbxgbtxT3heLtGGCKj/zk2LC3nWBvz3CtNDs+FEih0e1QXBqF/HwCCAiKFPVVulpLaky+Y5Yw5NI38QLGhiFR",
        categories = "food-beverage",
        tags = "lolly,candy,sugar,food,sweet,dessert,spiral",
        contributors = ""
    ))]
    Lollipop,
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htxdwglh6R/4LX3EoUIHqSI6N+bWEhrKTmE2Zl5u+ExPTMuUc49qFkngtDyrGPH0a8aRWfjdgC+7K8BZpUhnCp0CA3tYX50O5Tbk9xB07RetXHSbU73K+YoFKR3FOvL/ynyt3ixW6wGfPOX1pr6AjUqPtE=",
        categories = "travel,transportation",
        tags = "baggage,luggage,travel,suitcase",
        contributors = "karsa-mistmere"
    ))]
    Luggage,
    #[strum(props(
        svg = "eJwdizsKgDAQRK8ybC8SDZIiyQ1s7cUEN2AhYfFz+3wYmOLNPJvjIcifo4nAMZ0sjpQhVDIT/t5vCsIdezs2wdt7F0ZwtBqoZTOXRs2gn35poy+EGRj2",
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
        svg = "eJxljk8LwjAMxb/Ko/fGJlvbDbadPbirB2+lChMUZMjQb++64j9GSELIe7+kuYX7gGOrehGwkN27IBCYFFq0bMvfGTKxRAMmphppIYMnq7pmk0Bd88ZdZ5zXFdUelnyY1SWWkkCshYyD2c2atbXnCsKhQJGky12HPOVwkzl8bfE8xssJ8dkqrhTiI/exVUUS5fX/YyKayab8YF6EmD8y",
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
        svg = "eJwli0EKgCAURK8y/H2mUlmgnqBLSEq6CEKE6vZpMTAMzHs6h60g34Yk4Uq+xLo4IYa0x2JITIT/fAwNZHXfBKtPVyK8oUNKqG5mi8LIlBNsGfAVrxGdZHwCXyvT1CbZF/IIHQM=",
        categories = "text,account,mail",
        tags = "email,message,letter,unread",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Mail,
    #[strum(props(
        svg = "eJxdjsEKgzAQRH9lyLmm7pJoBOPFi5dePfQmbaGCaKAi+vdd05aWEpjswGPflqGb77h6dWIG5R2DkcqjRKbG/PaE20LbmpHDwCLTFrYhd2G9IwaknaRZ3FlV5XHfW5VhGrahH28IUz/OD6/IHgqQewVRJN+M0J9b4u66EAuR6CTE3P6ft6RfUZSs5FWmsMlHqcLKXuVSOVZBo+UJWyo64A==",
        categories = "mail",
        tags = "emails,messages,letters,mailing list,newsletter",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    Mailbox,
    #[strum(props(
        svg = "eJxFyzsOwjAQBNCrjLb3kl0H20h2TgCHQE6EUyChyOJze+IUiaaYYubFZcoVZZofpSYSS/gmcoRlLSX8EvWEzzzWsq6OhnhqYIivey0YEz1V4Y1ngWUfsuGzZ2uE1W4V0F0dfHNNHO6mCG+R3EFY+AJtKdLvzz844idF",
        categories = "mail",
        tags = "emails,messages,letters,multiple,mailing list,newsletter,copy",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    Mails,
    #[strum(props(
        svg = "eJxtjt0KwjAMhV8l9L6xTX+WwjbYA/gQQwUFES+8mG9v0k4HMgpfe8LJR/vn/LrCeTDHhDGAYmJ0GSpcPRG8OzmQGXiqmKlgidDYSskmM/YH1Y39T+oLelkImEi0WbX5qyUn3plFWLNly9Ahr2gzXUSmHbOIAygmwo6honmLagOENerrv4LcWVnd+3DEIlpKm8ES+qTIW/9+e1xg8YMhA2+SS+8WlxalqqXxAx3wTS8=",
        categories = "maps,navigation,travel",
        tags = "location,waypoint,marker,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    MapPinOff,
    #[strum(props(
        svg = "eJwli0EKgDAQA78S9l7crSB7aPsDP+CtrIKCB6ke9Pe2SiAhJBOOfK2YI42eIWyMwSnEf3Y6dbU68Vmh4CqBDOCJUugamYJtxfYFdkcST7CnJhNKpL6d/jm9ABkZuQ==",
        categories = "maps,navigation,travel,account",
        tags = "location,waypoint,marker,drop",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MapPin,
    #[strum(props(
        svg = "eJw1jFEKwCAMQ69SeoHRycYE9TZjCKLC9qG3X6rbT5OmL3W1pH6VTLXE/NyeDe1kyZBsMKvAYcihO4xVqxEHt3zV4FLMJ7XVs2VqMqRDDAShHAor85O4yTaZnUdxrND5eLAv6YYm1w==",
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
        svg = "eJxlzEsKwCAMBNCrhFygjdaFoN6mC0FUaBd6+/ohpcVNBpKXMTmFGnw8IScf78siKZAgaA6NzmxMnFmwfqUEUn88YBGt8UCoPfeWZFEilBaCuv/YOpfj2A2/zAq2DyyBMT8=",
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
        svg = "eJxtjlEKwkAMRK8y7P9GE9vdFNqC//UQZRVWqCBFRG9vasEWlUAgmZfJ1Nf+lnFs3CGSMLiEUAiIxEUvEGytGMQ7LyRdQYXpul+UAMksK5QpkA7WDfzymG6l40CxskeurTfT87b+RGA7F5Rkzchf4MK7GVD1FekfB0XMq306j2k4IT0bx9FhbFzpkB42vd1neR1AwHr3kj0t+V4uoEZh",
        categories = "sports,gaming",
        tags = "prize,sports,winner,trophy,award,achievement",
        contributors = "karsa-mistmere"
    ))]
    Medal,
    #[strum(props(
        svg = "eJxtzMEKwjAQBNBfGXLP2tlNY4U0f+DVe0FBIYgHEfv3bgvqpZcZWN5OeUzPK85jOB5EM9YwkC9rTMIE8wi17BZXy08rwV4snXKLezFCpbMNR0oGswyT+W4Hf4u9DNHPf91u9wtmHYNqwPvb9A6Y13K6oPoB4SErJA==",
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
        svg = "eJxdjUEOgCAMBL/S9AEqJBqbAJ9pOJAQD5zg95bWg3radHe2G7g0rhm4R3QegYdpE9kwhdXyFGq5Mgwv9o7QXcRTZJ4HwnBqCz2ph50hLZtDbZGVyGD6sS9C3+uMdT/0Da8LLMM=",
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
        svg = "eJx1y7ENgDAMBMBVLC8ACRKksLMBQyAS4XQosgRsD4YGCrrX3z/VPCvsjB3CVpIKowsIkssi+uTjxnptPEZq7BBpnVQgMY4DBHGtgVUfcP5X+recvpclUg==",
        categories = "layout,account",
        tags = "bars,navigation,hamburger,options,menu bar,panel",
        contributors = "danielbayley"
    ))]
    MenuSquare,
    #[strum(props(
        svg = "eJxlzEEKgDAMRNGrhLmAdhBxkfY2LgRxXW9vTCi0dDUkPL7e13NKTRkbpDKDK+S1M9GWvkWXXxUN26E9jE0URumZA1Od/m72A18iIKQ=",
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
        svg = "eJxNzL0KhEAMBOBXGdIvZ+ZW5GDX+hpbe0FBQcTCQn164w+ypJhk+EiYm6VHG6WiQvOGIDIbdbb9i9F5+DqpQcdefVqAu5Thcz4qwzhMHVaN8hOsjKK5YLNTM0teafZUiVU+mDcuHvt97QG0/yez",
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
        svg = "eJxtj8EKwjAMhl8l7N64JMa20A18AK/eCwoKIh6kuLe37abdYQTyH/L9kC887s8rTDR03MFnCc6Rc5pzDLsCjeEV3ze4DN2JHDoPJMhytEgMdfV1KB84mVorhVVNgfrE0YL9sQy6wZGCR9mfNQrIjBrFgzOEIhu8B5+ksaDlG86rsdWy6NHiR82v2pP/e34B9UhCsw==",
        categories = "devices,communication,connectivity,multimedia",
        tags = "record,sound,mute,microphone",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    MicOff,
    #[strum(props(
        svg = "eJxNjLsKgDAMRX/l0r3YpGoRav/A1cGtoKAg4iBF/16jQyXDyePk+j0eM8ZWdcTgaGFhpPTTJZdn1DB99b9rO6jgC/kPPqc0IJM4OrhXJE0lTNKc3XXZJpzUKmKFkz9eD1ko+0Zk0cINkz8l4Q==",
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
        svg = "eJxtjE0KgCAYRK/yMRcoox8D7QYdIkrSXYhY3T41woI2s5g3b4RVs6PdLE5LVCVIK7NqJ8Ea0ClRg+wRCCjlIIooDCJpoWrTit8rhueJ56MuS9vkNC0SI+PEfQKxeoGWWO+rHxCMD7kA8iI0fQ==",
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
        svg = "eJxtTssKwkAM/JWw98Zm9g214Af06r2gUKGIBxH792bb2nqQJTPJJJlN8+ifA12OpkuEIZm2ORSlbTY9E17C1tlOvKbgmHLvyFE9Pw4RBIbkkYMPnJPburJ1i4VY6SIn0hgrFiSWjNPuFElq9cEZda9Lq6rZEH7rCq/K/rlT130f2EWaYfnfK5YSKy4elp31JXab8Xa/0oSjAQy9vyzKhqaZdLQMtR+UNETC",
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
        svg = "eJxlzFEKgCAMBuCrjF0gt3xUb9ODICrUQ96+bSQVgUy2ff9Cb2WUXDfoLddjj+iB5Lm7ssMUlolS+HF2D5bqv9zo4IgrwkkRycsvLRPC0N7Ov6wuZWgZw2RZtZKZ9gLCZjHt",
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
        svg = "eJwly0EKgDAMRNGrhLmAtG66SHIDDyG2mO6kBNTba+hqYB6fRzuc7l7dBKmAHsEKGv9kkLV+mk94A5SXCJSv3Y2qYCuUspWAuPQDoK4WrQ==",
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
        svg = "eJxNjEsKgDAQQ68SZi86VVCh7Q08hKg4LgQpxc/tbRWLZJHAI09vvReMhtYWXECFVFlFVucRWO2mweMyVBJOQ4rg3jqW0UtYBUGmZRZviJ9bPFj9WTtW4Hr/CRNpoFiaBG4NviYH",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorCheck,
    #[strum(props(
        svg = "eJxtjLsKgDAQBH9luV70zjck1ja29hKFCBYiIvr3JgiaQrbZYWCUmTezTDCnJq4J5tJUEDZNKTUqfmyj1mG3GDV1ImA50kEgSNw4cq/NQo6kzz+GY1v7lm8EJXal8sh+TAVhW73iBm7pKh4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,running,active,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorDot,
    #[strum(props(
        svg = "eJxljT0KhEAUg68S0sv6Zly0mPEGttsvKj4LQWTw5/aOglpImpCPJG78B0XjWYmB2F/O0n2OrHQXGeQLSRMLm0Q9fGrrAG37ToOnZMTqaYilb4JGlxKbpyWmM4+1o/DMnof5nL0PqwJGtLjBDpc6LZE=",
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
        svg = "eJxtjU0KgCAUhK/yeBeop4Iu1Bu0bR8pPXch0s/tS4KKaDXMfMyMnYfCEBx21ALJXqO3Tc28fYj6kBzHAmsKhR2KFmE7BWF3KBHyZTimiYtDUrVWC+9BAaQX9XNlQBCbGxzLuCyL",
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
        svg = "eJxtjLEOgzAMRH/l5D00MW0IUpK5S9fuqKCarUIRbf8eZ4IBeXh68t3Fz1AEY6KHCwhPPzAYtp5hw/fr0cFrt7uSJVCOlzqR4z5k4frVtE3v0TbudhLpNCGHxzK9CmSa31ISOUv4K5iw/BIpvvNYJJEnqDtfe7WRNyC0MGQ=",
        categories = "connectivity,devices",
        tags = "smartphone,phone,cellphone,device,mobile,desktop,monitor,responsive,screens",
        contributors = "badraxas,karsa-mistmere,jguddas,ericfennis"
    ))]
    MonitorSmartphone,
    #[strum(props(
        svg = "eJxNTcsKwyAQ/JVh701V1KSgnnPpRwQjVeihiPTx910p1LCH2dnZmXGPrWXsnq5mMlBiXSi4cz8G95fkjEuehBxSTbHhVfaWPUlByKnccuPdEt4MivDxpAmVmeq+7jhELrCr3hQURJ8Tb087OKPKevTFUuM9oXI0IXK0NIy9ae5PPzl8AWhaNsw=",
        categories = "connectivity,devices",
        tags = "devices,connect,cast",
        contributors = "mittalyashu,ericfennis"
    ))]
    MonitorSpeaker,
    #[strum(props(
        svg = "eJxNzMsJgDAQBNBWlmlAs4ofSOzAIkSDm5uE4Kd7TSDqdd7MaG/nQIdbghg0oNOgB4l1q4QUXAYtBl3E3qBTO6uqE1cg/8wY+YbL9MPfbJuC0GIwKibV7nWUmP2kI1bSvXADI+krbg==",
        categories = "connectivity,devices,multimedia,development",
        tags = "tv,screen,display,desktop,video,movie,film,stop,shutdown,virtual machine,vm",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorStop,
    #[strum(props(
        svg = "eJxtjcsKgzAURH9lmH2pNxG0kPgH3XZfVLwuBJHg4++Nggoisxk483D9Pygqz+4DSWBfFlEs3HsDhTvwVwzE/rKLDHUZMHsaYmqroNElhNZto8FTUmLxtMSwZ2JtK9wGszF9uMphRPMTrFLzLUo=",
        categories = "connectivity,devices",
        tags = "tv,screen,display,upload,connect,remote,screen share",
        contributors = "danielbayley,karsa-mistmere,jguddas"
    ))]
    MonitorUp,
    #[strum(props(
        svg = "eJxlzV0KgzAQBOCrDPNe200jtZB4Aw9Rqrg+CCLBn9sbRRSRfVjYj9lx3S8oSs9WbJJCTJI+4jB3z1Vyd/h3Z1y4r/4Bs+ebGJsyqKd5EVo1tQZPscQUT0S/rRhbA+fXQgzkM9h7X5HBiGYHLDkvLo4=",
        categories = "connectivity,devices,development",
        tags = "tv,screen,display,desktop,virtual machine,vm,close,stop,suspend,remove,delete",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    MonitorX,
    #[strum(props(
        svg = "eJw9i1EKgCAQRK+yzAXKLaoP7TIlKUQfIqS3b13Ir5nhvbHJH5mqwwRKxYFBbzxzkDaCgo9XyA5mBinc7dAOu73j46kYh00Ii7GAqkw2kqwpbrN+t0ncoX51t1y7/AEFZiWr",
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
        svg = "eJx1zLEJACAMBMBVQhYQBQshukywEKysdHuDiWBj9Tx/PHEb3CvwzOgDwpBA4HVaIadzoct0UJ6M/5m9CY6P2o0SIPg=",
        categories = "layout,development",
        tags = "ellipsis,menu,options,operator,code,spread,rest,…,...",
        contributors = "colebemis"
    ))]
    MoreHorizontal,
    #[strum(props(
        svg = "eJxtjLEJACAMBFeRLCAKFkJ0mWAhWKXS7Q3EWIjVFXf/SJ1pNEezQIjgaClZABW96oqWiU7H2uZfhXyD92wDkgcg+A==",
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
        svg = "eJwVi8sNgCAQRFuZTAMqQb2wdGARRojLzZCNn+6F0yTz3gs1H4ZP6IhXOBO1zUo8JZkKJ09oLqdaU0bGMPQghms3RRJuk8Ny+w76FX+NhBaw",
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
        svg = "eJxlzTEKgDAMheGrPHIBidJBiL2NQ6G0BR3a25vGOkiX/Al8ECk5thjSiZJDuq+DHJjh8Ia8LJ/wMlnewRv6HMvfm61rd4Q2Wll/6GlRbvQByA8mmA==",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveDiagonal2,
    #[strum(props(
        svg = "eJxlykEKgCAQRuGrDHOB+BUXgnqbFoKoUAvn9k1KQbR58OALvRUpue7UW67nERmWHMGvAJzC9pgU/hqv1divnlKMKs8kiOyYBtYOc6/ySS+/0SaY",
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
        svg = "eJxlzDEKwDAIheGrSC5QdBAHm9t0CIQk0A719tWUTAHhB/l4Onq1WtoFo5f23GdCAQEiQD8B5JT1WCjrxjl0YN7sdEY+SQkM/77eiL+Jwk/7Abz+JoY=",
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
        svg = "eJxtyjEKwCAMheGrhFyg5A3SQb1NB0FUaAe9fZOUDoVC4A+PL45eVy3toNFLu87EO4keCCAJ+nOO24ty/OHBteHwte6mJBYwTTxdWlh1h3m3N7zMJoY=",
        categories = "arrows,cursors",
        tags = "double,arrow",
        contributors = "ericfennis"
    ))]
    MoveVertical,
    #[strum(props(
        svg = "eJyFzjsKwCAMBuCrBC9QEnAIWG/TQRAV2kFv3yQgLe3g9BPy5RFazSOnckCrqVzn7jwwECCBB/Quhm2KGH6WFZFyDysrBNk0yQXkFWf9wx7Rsc92ox13Rw46SUgOKVGTLMW/LT1NxX1i2zHtDcmvTQs=",
        categories = "arrows,cursors",
        tags = "arrows",
        contributors = "colebemis,ericfennis"
    ))]
    Move,
    #[strum(props(
        svg = "eJwBRAC7/zxjaXJjbGUgY3k9IjE4IiBjeD0iOCIgcj0iNCI+PC9jaXJjbGU+PHBhdGggZD0iTTEyIDE4VjJsNyA0Ij48L3BhdGg+n54TIw==",
        categories = "multimedia,files",
        tags = "quaver,eighth note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music2,
    #[strum(props(
        svg = "eJwBQQC+/zxjaXJjbGUgY3g9IjEyIiBjeT0iMTgiIHI9IjQiPjwvY2lyY2xlPjxwYXRoIGQ9Ik0xNiAxOFYyIj48L3BhdGg+Y9YSWw==",
        categories = "multimedia,files",
        tags = "crotchet,minim,quarter note,half note,note",
        contributors = "it-is-not,danielbayley,karsa-mistmere"
    ))]
    Music3,
    #[strum(props(
        svg = "eJxty0EKgCAQQNGrDLOPGKNQcLxB2/YxBQUGIRF2+7QIN63/+3YfjwUmxt4A6aH1pCp1UoPO1jk5+4HNQCKpliRrED+DRMYOQS5G0giB8dnfWlTOmcVfdgNXWSaD",
        categories = "multimedia,files",
        tags = "semiquaver,sixteenth note,note",
        contributors = "it-is-not,karsa-mistmere"
    ))]
    Music4,
    #[strum(props(
        svg = "eJxFyzEKgDAQBdGrLL8XWYMhQjY3sLWXVVBIIUFEb68hovW88du4LzQJ+o7YDW3kpmoONgi+zil4XZPGmfQSsAMlgQHpKbDZlPqpt2Zri3qen926rh8g",
        categories = "multimedia,files",
        tags = "note,quaver,eighth note",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Music,
    #[strum(props(
        svg = "eJxVy0EKgCAUBNCrDH+v8b+GBuYNOkRQUCDRokXdPrUI2szA8Cbs47Fg6mnotGHUaCGcnLJwsIo1O2VyUAxNwTF8F7a6NfDae7BAUsX447RuM07pSYRwcm7C9dazZlpQvAEWyiGg",
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
        svg = "eJx9jM0KgzAQhF9l2bu0E6m0kPgGvfYuVbreigR/3t5N/MtF2cMMO/ON7Zqvp8kxCiZp2p94x2qHtvYS3bhkXVAu7S0ApY1Y0l8XtGS27rFxSmnvFdkr6F95odrx+0Eo+iyvQKC7nmoGgUkfhD4PaIASFIZgPs89mQFiKUMD",
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
        svg = "eJxtUUFOAzEM/Eq095jYseNYavsCuHKvChJIBXFAiP6e8S6qeqiijR17djyZ7L6O32/lZb88sRR9luWwe8jSYXdtWOH2o0en1rRse1uXkbiXQT79RMpC3JRmE5KQwjgJWcgjiEXONIRrbifqlTqNqKRzbriawJzAZd02fiUWq0LR444sjtTVqWu/14UCOVVGG4PMO9Johr215MSHsg4UJYuaSrr1bJW1whNyVZHqyPPEH2tDJjkjGUEBUC9css1XGjcHTevg6YU4tJL7oJlcZon2AED4g8FiCiMjYOGcDk/MARAMn5FQs5wOi3NIhxk8kI3Z0zlgEdGX8BzfV6zNdTzUwum8qGLNmjLwdOuFgB2JYoO+m0c/v3++lgvvF1nK738QBMTLFgFN0OEPsoF5kw==",
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
        svg = "eJxtjEEKgzAQRa8yzN6pPyalA4k36CEkLbTgQsSF3l4TswgiM/Bh/nvj43+O45fmwB1TXAPDHLnl7P3jrHtfsFS4wikn647K5wy58ktrahqWH30Cv9GKJQPRAS0de46KWjRwYvFMWsJrqRNHRlA7jcrLgi7ODvHVOP8=",
        categories = "science",
        tags = "planet,space,physics,satellites,moons",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    Orbit,
    #[strum(props(
        svg = "eJxtjUsKwCAMRK8ScoGSFNSFepsuBFGhXdTbNwb6Eboa8niT8a3mnlPZoNVUjj2gBQcrEIMFMhj9chvRq9c5IDHCSQGZJMct2Um5FL6yQPPj8sCzOv1xb0f33CNf8xAurQ==",
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
        svg = "eJxljUEKgzAQRa/yyT6pM0ajEIUeoNvuhRYUrAqVorfvJBUJlAQ+eXl/xi/d2uPRqBuVoLIvVesvgbX+/GECZfeqYzCycDRpMi4ftdM2oZKCYK8nQo7qkxQhk0JRpASGHstEsv/LX84UsIYdahSGisSYx30cpieWeZjWd6NywzUciMPlzDiCi/ohtj7qGzWKWGGXZMmNj/cvpRDlL7r6RGw=",
        categories = "development",
        tags = "delete,remove",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageMinus,
    #[strum(props(
        svg = "eJxtkMtqw0AMRX9FeD/qSPI8DE6g+3TbRXeGFhpwQhYhJH+fKzsPcIJBwnN1pSP1h+H4T7+r5ksjd0KVa4uQMimrDcKd0RSif0G4CsWNsVDLYoOyKE1h1jkmMs7dKLArZTgHFMyqN2rhViHISd64uwB3+WnW/YeDrfs73s4gznTwWgqZsyzw0HbCG9Ej+fNigBCwnW5TucDXckkPOgk3OnsLB+8r3Ljd/9FFV41YQ2dBVmSd8wX/ql7sZc9NcGgS43QytoIp0U+NcIeQaQ0bQwZrrMsdUVEqdgw5uPy5aEC4T+Zavn3Eg/QKSR5mZw==",
        categories = "development",
        tags = "box,container,unpack,open",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PackageOpen,
    #[strum(props(
        svg = "eJxtjUEKgzAQRa/yyT6pM4lGIQo9QLfdCy0oWBUqordvkkobaEngw5v3Z9zcLh1utbhQASq6QjTuFFjjvpMKpNd/EyZQdi1bBiMLT5IkZfUgrTQJ9ekRzPmDoFGuSRF+Uyh6KYGhx34jmd/jD6tyGMUWFXJFeWJMwz704x3z1I/LsxZacQUL4vA5U5Zgo36IjYv6RrUgFtj5nduRu+fMoRDlF0CYS9U=",
        categories = "development",
        tags = "new,add,create",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackagePlus,
    #[strum(props(
        svg = "eJxNjlEKgzAQRK8y5N/UXQ1RUKEH6G//xQoVrIqVorfvJlYbErLL5M3uFFO9PPEo1Y0JFN+zmsGI3YkoIm2TPrJRGqhSRUJ6PSUkyD6BETLJGQUKROdjmUipqoqLW1sVx/KX1QapZoscRpMJiLHf+m5oMY3dsLxLlWjOYUHsLsfaEqzHf2BVeHyjUjErbFwqkroelXwVww433dz0LWahtVFoVvnOfLdJZ7SPskP/uDfZK1nJupclSX4G/gLSjlJD",
        categories = "files,development",
        tags = "find,product process",
        contributors = "Reund0,ericfennis,karsa-mistmere"
    ))]
    PackageSearch,
    #[strum(props(
        svg = "eJxNjl0KgzAQhK8y5D2puyZEIQo9QF/7LrRQwT+oFL19N2ptSGDZ4ZvZCVMzv/Co1I0JlN2LhsHI4tOkyfi8017bRJUpEuz1lJCj+CRGSFI0CpSI0ceSSFbV4RLP1uF3vPfGwRr2KOEMuYQYu7VrhyemsR3md6VywyU8iOPnzHiC3/ADrMOGL1QpYoWVjyk7y1z2XQw7fDYgyczh4HrtpK3T/xJfjABGbA==",
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
        svg = "eJyFkMFOBCEQRH+lw31auqF7IJmZy579iA2arIkHs3pQv95i2EnUmJgAXVCPCs3Snq7t+ZHaxxqcLdB1Db209zVIgtqWu4FsyzdU4m92/pM9CFyZD7T8R4oeqP9EX85vF3pYw70o6QkmodKooq8ZQuIYjas6QbHnMvGc/SaxlkJx4pzmiQVeSTZ2wqKQWjFL7Wdw3XSo3T33kD0J2Ujv0vdQLxfhWr0ljibwjM1sQisx3XRf80mBueHVEV3g27LL/nzSz95r73H7Ahj7Xu4=",
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
        svg = "eJwli9EKgCAMRX9l7D1iiuCD+i+RkkJFiJD+fZsx2OXunLma9gY5lSM3j2QRaveoEHhrLmOWt8SWfzzkHtwqf8Gd5U7QabpdsUtscCXDqWayK1Zwz9YyRI8XGbCLBr3wCBYQPhMaJGI=",
        categories = "layout,arrows",
        tags = "drawer,dock,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomClose,
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQtgoWmm7gEGKL6U1KwLq9pCcFvYTwH/+HmjeB2ggtwkXoEFq/nMvOQmhmhLMk4f7GMGghhmMVhkS4mBHMxEZFs6d4Ffsh7g/8e+sGrbkseA==",
        categories = "layout",
        tags = "drawer,dock,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomInactive,
    #[strum(props(
        svg = "eJwti9EKhSAQRH9l2PfLbZOgQP2XKEmhIkJI/75diXk4MHPG3mHJqI4MIYa0xeyIR8JdHPWEJ605fk1tTVHV27/+vN3TGVC43UsvAhOqkAchN4qrlrfXnCNWR8cE7mB+BhKddfAvDdIkRA==",
        categories = "layout,arrows",
        tags = "drawer,dock,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottomOpen,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJRfDQ9jNabEE8lIDJ703iadjd2TzbwaAFV4TextW5IO0IEs00JIMG3nFyj7nmxX813+NpIBSyJts2I/0Uy4ncdat+FOQcQg==",
        categories = "layout",
        tags = "drawer,dock",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelBottom,
    #[strum(props(
        svg = "eJxFi1EKgCAQRK+y7H+ESVHgeoMOESmtH0HIUnr7tKA+BoZ5b0z0q0DMhB1CItQI7MPGQqhGhCs44bfG9Di5Ota09WfNsQiDI5wn0GexCqjTD3Y1gOob3Wgo+fgN1EQgyg==",
        categories = "layout,arrows",
        tags = "primary,drawer,hide",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftClose,
    #[strum(props(
        svg = "eJx1zEEKgCAQheGrDHOBUFskqDfoEJHSuAsZrG5fU5uC3H4/77mSZgZKeSH2qAaE3aNB2HJkeuC4oVyuMbhOBsGtExNEj6MF1VclQegbbNW/wTTcvo5OMRwr/A==",
        categories = "layout",
        tags = "sidebar,primary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelLeftInactive,
    #[strum(props(
        svg = "eJxFy8EJgDAMBdBVQu4itR4U2m7gEGKL6UGQEtRub6Kgh/BD3o8raWG4PFoESnkl9mgGhDNHpnctop1EfaJqNbhW/4LbZyaIHqcR7CFlAT39sJkexMA2Mh/fuEIggQ==",
        categories = "layout,arrows",
        tags = "primary,drawer,show,reveal",
        contributors = "mittalyashu,danielbayley,ericfennis"
    ))]
    PanelLeftOpen,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJvSi0/YwWExAPJWDzexN7ml12Nvd2KFjBFYGbXKwFaUN45VSecfxjdySHBWpe4lfzLU+DQQV3BEs+kftpVoqfm+HUD+oeG+g=",
        categories = "layout",
        tags = "primary,drawer",
        contributors = "colebemis,danielbayley,ericfennis"
    ))]
    PanelLeft,
    #[strum(props(
        svg = "eJwdi1EKgCAQRK+y7H+ESlCg3iVSUqgIWUhv3+5+DMPMvPEtHwQ9oEP4aqIS0KwIQ4vGvWUbaiXXs5Du0c/yi/6qT4ZhlO5sZuGvZdxwtpqZFSr6d6cCKeC9wgYO3MSSVfr4A92KJBs=",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightClose,
    #[strum(props(
        svg = "eJx1zEEKgCAQBdCrDHOBGC1IUG/QISKlcRcyWN2+rE1Ebf7iP/63OU4CawrCDqlH2BxqhP1Kjmlmuft8gkJvmzrwdhmFITgcqANqC1Wp3UtMUd+i/8A8vg7FNyyw",
        categories = "layout",
        tags = "sidebar,secondary,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightInactive,
    #[strum(props(
        svg = "eJwly9sKgCAMBuBXGbuPmiJ0ob5LpKRQESKkb9+WFzvxf7Ml7hVKc6gQSv/Hm0NNDmlFSDEfqY6djUbo0r2d5c/bM98RGjEwDNSYnYbkU5FYUd4+W00QHF60AJlJTxq4JJfEfzRAJIs=",
        categories = "layout,arrows",
        tags = "sidebar,secondary,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRightOpen,
    #[strum(props(
        svg = "eJwli0EKwCAMBL8S8oESS6EH9TOtVKH0EAI1vzfRSybLzkYulwBrwoDAfeJvt9SEdCLU0p4q67dyR1C/OW6+y/FtXwGl2fRg3mGkRbUcyF238gAhFhxC",
        categories = "layout",
        tags = "sidebar,secondary,drawer",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelRight,
    #[strum(props(
        svg = "eJwti9EKgCAMRX9l7D1iClGg/kukpFARIqR/32axh8s998zksBWIIe2xWKQZITeLCuFJvsSPMNDMa+dVijOj/DlzpCtAUxYX1qhH5aaIk35THGfutUTwFs8FaAI9aOCTWQb3AtoYI/A=",
        categories = "layout,arrows",
        tags = "menu bar,drawer,hide",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopClose,
    #[strum(props(
        svg = "eJx1zMEJgDAMBdBVQhaQth4sNN3AIcQW05uUgLq9TU8Kegr5j/9DzasA57KxEJoJ4SR0CLUdi3D15yhJuGsMgxZi2BdhSISzGcGzUdDoCb6B/QD3k/vX0A0gYSvE",
        categories = "layout",
        tags = "menu bar,drawer,show,reveal,dashed",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopInactive,
    #[strum(props(
        svg = "eJwlTG0KgCAMvcrY/4hpQYF6l0hJoSJESG/flgz2eJ8mh73Am3yJFmlBqBY1Qvt/ZqIQYkhHLN3OTSRnRuk5c6Y7QFWsEZcYV16gPkHCOCkZZ56tRPAWL5qBpkGDHvjEF8d97NYkNQ==",
        categories = "layout,arrows",
        tags = "menu bar,drawer,show,reveal",
        contributors = "danielbayley,ericfennis"
    ))]
    PanelTopOpen,
    #[strum(props(
        svg = "eJwly0EKgDAMBMCvhP2ApF4Umn5Giy2Ih1Kw+b2Jvexhdza2fHQquV6lC3gDtSEIoLeevcxGBSto/NnU1xQX/6V41yeTsmA3wNMFE2yv4K1JN+kD8+Mb6A==",
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
        svg = "eJxtjDsKgDAQRK8ypFezWT8I0drGQ4QoKChIkKC3N9EijbDLg3nDaLs6u81wnSApYK9AFXi/7HXx+V4f5lwwdWKvUIHKcNHGNLmRGMSGwZCg8DKrh9arn2YLanymck4zD0mgJSM=",
        categories = "transportation,maps",
        tags = "parking lot,car park,no parking",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    ParkingCircleOff,
    #[strum(props(
        svg = "eJwlijEKgDAQBL+y3AdMjBgCydU2tvbhFCJYSLDQ33tRlmVZZqLsVY4NcieyPaHqGII83+XY/Zzjma+CNdEcYP3iy5AdHIzGascpNLlJ/AK8+xXz",
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
        svg = "eJwly8EKgCAQBNBfWfYHyoxMUM9dunaXlNZbyEL297nEMJfhjav5ZHg9agTK5SL2qFaE2jxOCE9JTP/SxAQ3yCG4OzJB8rhbUOYwNEcNGsYe1btsVqig8AFIjxo7",
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
        svg = "eJw9izEOwCAIRa9CuECrsUMT9DLEwcR0cMLbF5F2eoH3PnEb3CuwZAwRYShOBJ52Fjq2L9TbU0HC1nPxUsaMN4JEe2u9Km/dfalNk7fpb1+d/yFD",
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
        svg = "eJxlyzEKwCAMRuGrhP8CRSidEi/TSuMqAfX2mknQ5U3f45JeI035VxOEB9QEszV/poIb1L2RL4eRD75B38PyA0HSG1Q=",
        categories = "multimedia",
        tags = "music,stop",
        contributors = "colebemis,ericfennis"
    ))]
    Pause,
    #[strum(props(
        svg = "eJxtjUEKAjEMRa8Ssjc2NakV2oIHcOHWXamCggsZRPT2pjOzEYcQfsh/yU/tNrT7BYaMHqG9MzKbfjIKlrSe3JJmqu/jjJmORwuU2d5NXzj8Y4/6vMI542EH7KqCgrNiU31tSKs19B63q0BRgMmJHgOpB96SRBCSANy9/Q8OahO7U8/rOeULqwU3+g==",
        categories = "animals",
        tags = "pets,vets,veterinarian,domesticated,cat,dog,bear",
        contributors = "danielbayley"
    ))]
    PawPrint,
    #[strum(props(
        svg = "eJxti0EKgCAQRa8yzAHKCRUC9QYdIlIadyED1e3TVi7cfD7v8VxJh8DrcUHglE+WehVCeX5U1yDcOQp7JI3BzS0I7tqFIXrcyABpnhQ112jnVrBsh5xUJz6qziVN",
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
        svg = "eJxty0EKwjAQBdCrfGY/kck4JkKSE3gJiYJCBCkubE/flCy66eYPw38/fe+/Fx6ZPuIhVwQOUGjPwMq6UEmnjZS0wwhRFmccnN08fFNnkHN/RCGxGdvRrkv0RbyM3EV9T7U9UedMIoT6H3fK5Dc06rICRC0r0w==",
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
        svg = "eJxty0EKgCAQheGrDLOvnEBhYPQGHSKmwKAgpEXdPsVFBK5+eB9PdEu6r6C3RxoR9KlNOQaDDNWDnPMVYfF4kAXuHLiCZfxoYuDYG2pIPpH92QsTGSGs",
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
        svg = "eJxty00KgCAQhuGrDLPvx0JJUG/QISKlcRGEDFS3T91E0OqD7+E1KawMl8URIeUZECjEjdiimBDuCmf0TPVwpiuBM8fCBN7iLiToRoEqVM6XZg2a2l78SI6E/NgDN1Ml9A==",
        categories = "account,social,money,shopping,maths,shapes",
        tags = "verified,unverified,sale,discount,offer,marketing,sticker,price tag",
        contributors = "danielbayley"
    ))]
    PercentSquare,
    #[strum(props(
        svg = "eJxdyjEKwCAMheGrSA7QEqEthcTLBAdBOjiZ2zfGodTp5z0+quXJQZHhgNCjRy1420Rvon2gRFKa1BxEGc7NXGOIo9LnYXCSH8Vrtf58+AXn/iL2",
        categories = "maths,development,money,shopping",
        tags = "percentage,modulo,modulus,remainder,%,sale,discount,offer,marketing",
        contributors = "colebemis,ericfennis"
    ))]
    Percent,
    #[strum(props(
        svg = "eJxlykEKgCAQRuGr/Mxe0rGGAvUGHSIsKCgIiajblwQRtHqL77k4pTgPiKenipA8GUI87jAFVzwa3NptI3pPSwPWsEpgIXnI8GFBDQFDFP+1NQyj9/KVC3gsIh0=",
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
        svg = "eJxtUNEKgzAM/JXQd7MmutSCCnvfPkLYQEFUmAzd1y+tOoSNwqVJ7nIhxTh0S9f2DxiHtp+epaEcGJhBQH9kTVWcdk5VROZMysoMzFwaZgOL5qKBQ1D6Sh3rqYF7aW46iwQ9v9JaB4PVRwlj9CGPzm+4NnKUNEnRulA9r7B2JJE//EANInf5DocMiYCbg1+wQqfAmGcb2vjQgWaBsK+G2VlLRNccrQePnmpSZ9kEAtIRsksCHB1UErW/HsEAjwvuJ3nH6+qhqg+nLFMS",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneForwarded,
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaaEFUqBTt6ZvsahFaskzYZJIJU41Dt3SP/gbj8OinZ23IAoOAB2bwpqkOG6WpInFmJRlYqDZsYNYkeZFypCfq2E53uNbmwroNA7+KlmVzLkEZI3mVCejCiqnh0RZZgbnT6jFB6tjM/uErVYfc6bscSiQCvu/0VAqdAKMvV8xjoAP5KWE7DcujlIjOHvMAAQO14oa8NGDBdoTsMoW9gozE2V8NFcD9gZsl7+iuGNV8AGG7Uuc=",
        categories = "arrows,connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneIncoming,
    #[strum(props(
        svg = "eJxtUFsKwkAMvErY/8ZN2mZ3oS34r4coKFQo4oeI9fQmfVlUAhOSzGRCqv5yPcNAtWMHT0uaB65d1FITiWuqnZGa6puqs5HzkazUW3vv4FS7IzOQYOJH3jIweA3KGClqQQlDmnEaRJQ8y9EH65YTTBPJ5A/fqCYK+3U5FEgE3G38zAqDAmMsZvRjYACtjLCchkWpLaJDRJ8gYaKW1FlmgYD0hBwyg62DSkbtr4cZ4PbA5SUve5c9qnkDMIlQDg==",
        categories = "connectivity,devices,communication",
        tags = "call",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneMissed,
    #[strum(props(
        svg = "eJxlUO0KwjAMfJWw/4tNWvsB28D/+hAFhQlT/CHi3t6kHVQYLddekit3HV75PcN17C5k0Ecgi5YyeZBtyrLoCBj9QsihV8gMXFpaJ+rRHYEYo9uw6uRKGKDNChPysU3eizwKoYQhbVgbEb3tLZqgVccb1p44tKL04VFQucv7J1Rd3jk1C07sAs/2P0H1tfev3iVCC1ByauBzRJMgYaJuGg76gdOw3J83WHnsmDv4Uj1XPYVqWUd1aPoBKzxJzg==",
        categories = "connectivity,devices,communication",
        tags = "call,mute",
        contributors = "colebemis,ericfennis,csandman"
    ))]
    PhoneOff,
    #[strum(props(
        svg = "eJxtUFsKg0AMvErYf1MT7T5Ahf63hxBaUBAVKkV7+iauFqElyyxJJpkwxTh0S9f2DxiHtp+epWEGDwIMZIFNVZx2SlWsxJlKQ9bAzEo2sEju5dNU6ZE61lMD99LcWBdh4FdW69JUghJG8qoQ0IUNY8OjzZIMU6fVc4TYsYn9w1eqDrnLdznkSATcHPRUCp0Ao883TNdAB5IpYT8N87OUiK4e0wABA9VihLw4YMF2hOwShaOCjKyzvxoqgMcDd0veq7tiVPUBVPtS3g==",
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
        svg = "eJxtyzsKhUAMheGtHNLLneSCsZhxB7b2omLsRAYfu9dREAtJ9/05fu7biD3Qn7COXbRAXBC2C6wfB4u3zCcJlf6XBqWfmmjoAlUKNXYpJHoFdtDlu+RgbQQCdx5nkkmtz98Bx5Un5g==",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley"
    ))]
    PiSquare,
    #[strum(props(
        svg = "eJxNykEKgCAQheGrDLOfchzBAvUGbduHBQYRLVrY7VMX0eLxw+Nzx35ukNnjiPCUmBLtUSuErOsbXF9NcNdyJ1g9TgZsVMSdBe6EBMoSS4WV/CAPoFVsUpFQ0ySz+egL434fgg==",
        categories = "development,maths,shapes",
        tags = "constant,code,coding,programming,symbol,trigonometry,geometry,formula",
        contributors = "danielbayley,jguddas"
    ))]
    Pi,
    #[strum(props(
        svg = "eJxNjLEOgzAQQ3/Fuh2ac1ErpCRzl67dEaAeW4WiAn9PbkMeLNt6jr+hGKYkbyr6z2MgiOBq2PDVXTP41zAGaKttDx9oneR4848c13ks2JapWBINgr0aBTYvXytJnoKjNnfBWhc650Q+Ae/uIGk=",
        categories = "multimedia",
        tags = "display,play,video,pop out,always on top,window,inset,multitask",
        contributors = "cd16b,ericfennis"
    ))]
    PictureInPicture2,
    #[strum(props(
        svg = "eJwVjEsKAyEQRK/S9L47to7zAXWdTQ4RnCE9CyEMYpLbR3mLgipehfezKuwRHytM7Ju/u0JCM3SKODCNXDYkLDPxOpEdKC2FNtiazQaEjefNgx2owxRu4zOF68gVfhHFsUfQ43xpjbggXL20CJ9zr9png/DtYYc5nPQHltglcQ==",
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
        svg = "eJxti80KgCAQhF9l2XvlGvYD2rlLDyEp6S1E+nn7tE5CLDvMMPPJYNcIpzfRKaQBwVm/ufj5S2GLcL8aUuA4ySYDk9x1dGAULsSB+DzWQvNaQH6WjoBVYqY+A3laAv1B7K/pyuYB734p3g==",
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
        svg = "eJxNjM0KwjAQhF9lyb1rZ02aLqQFb1689h5QiFDEg0j79qaG/rCHWWbmmzA+Xw+apDMihmZkNTQVmYvbh9NS6sNWxR7+EfjC4FB+x0+ie2duSvoF+yYKCdX5UIEByp6O+fUt62XNanIEx2IH+FTmlqHDHBwpn+3QJMSdqit79dzqBvwAQHUzug==",
        categories = "maps",
        tags = "unpin,map,unlock,unfix,unsave,remove",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    PinOff,
    #[strum(props(
        svg = "eJxNjrEKwzAMRH9FeI/rE06cgG3o1qVrh26BFlwIpUMJ6d9XCgkxAomTdI+L0+v9pAXJgA39OBmWufCmdR9Mjid9y/Ezfgs9krm2hFDg5wY2dCMTk9MSCehumLT3jR3O+xEEcTn5v3UFh0dc/tLXmnzB7GswVjAd4IrrSLCtZX/XnJow/wEz7S6N",
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
        svg = "eJwti0EKgDAMBL+y9APaHsRDzGeCSKE0pXqwvzehnnbY2SXJXcoJGUeIKUDemd1iDUzL9ExNy7i0ommuz+0WO+KGmGDoYIUf/iF/lUsZUA==",
        categories = "shapes,multimedia",
        tags = "music,start,run",
        contributors = "colebemis"
    ))]
    PlayCircle,
    #[strum(props(
        svg = "eJwli0sKgDAMRK8yZC/iB6nQ5iDuxBbThSAloN7ehi6Ggfd4vqRDISmfooEGR3gDTYRSbyQ8Oao0/hln31vA/t5VEANdKxwWzF3dZtoE/ydwF74=",
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
        svg = "eJxNycEJgDAMRuFVfrKAJqcc0m7gEBKFCh6kiOj2tvTQnh68z/zIfu7wLxALwd/WXDJTtKl5tGu9E7ZAi4IlaaW6BmCBPh1+inkZnQ==",
        categories = "maths,development,shapes,cursors,gaming",
        tags = "add,new,increase,increment,positive,calculate,crosshair,aim,target,scope,sight,reticule,maximum,upgrade,extra,operator,join,concatenate,code,coding,+",
        contributors = "colebemis,ericfennis"
    ))]
    PlusCircle,
    #[strum(props(
        svg = "eJxNyzEOgCAQRNGrTOYCBmwoWG/gIYwQl86QjeLtDTTaTDEvP9a8GzSXQ03oAlGb0BOPcCba2Lsk06FLnHqwxHMzRRKuAc7rgH79wHmE64MXNI4d5Q==",
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
        svg = "eJxtjUEKwkAMRa/ymX3i/HZiW2h7Ay/gbhgFBRdSXOjtzYybLiTwfyCPl7nct/K4oryXwC6gfLwZsHmFdT78zuv8zK8bLks4keCQPRB9iA6xRKiJ9gm9qCGpZTVfGiAOFlEehRUySS3UzlVftTv5CKZssGYmRsQ/EAdw9B8TpsZFYdyBX/DtMew=",
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
        svg = "eJwlykEKgCAQBdCrfGafORqWoHODDiEUFES0aJG3z8HV27z0lPfAlmnlxfiAYMJUIiIsGDywM7OHJUmjRknXee+onMkRvgarrlu77eqSH/HJFhU=",
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
        svg = "eJxNjtEKgzAMRX/lkveyNWhRaPu8l32ETFkKomUrbv79MsdQAnk49+QSn+dxHdM0IM9pKs9ADi0cGLb5rZaiP/2t6HNXBH2gq9PwUnWs1lnHGja8mHoHUCDWHQF4qY8XYDG89Wtr9I/hVrAGshVBhnSXEqghvPUpwiv1RTTb/K8ZPwNBMDA=",
        categories = "devices,account",
        tags = "fax,office,device",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Printer,
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9l2Htqd7dNLSQ99+LVe4lCBAUpUvTvjQqmpWUPuzPDW8bdh0fEydOhRgNFTZ3bfbzO/ZMW9qhr/8aaEDGSo3AZw/WM8PTUEsLLEyth9PTFf+nsMXOxV7D0Ug4CQZmG05apytqkq19oI5Op5oCRKAU3G9XZgm3MDd+BdDzA",
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
        svg = "eJx1j8EKgzAMhl8l5O5mUlYv1vMuewiZsnobUrr59jZVrEJLIQTy/V/Sdh7fDuw4fawz+ED4TYOzsZv/BgkhVIWwSO3au+BdG0OJFFDj2RLx3XBNLRu7y0+Rbc8hPVLf3lkYDL6YgLStVM/AUMurQufjWQJdUSZ/qykzI4bGJwmJ5NlkQAXEtuhQ5RHp0motSsofTFw+OHymSqMV0Qd3oQ==",
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
        svg = "eJx1jl0KwjAQhK+y5N0xu0k2DbSFHsBDSBQqKEjxQW9vYrWCVPYHltn5mPa6v4106MyOE2wkj+QGtlR6LkVK5OC86dttfe7bxeJJR1heUQSSKEFlYXEZYTimBi6sWFghniKiDkr6djQQR0XRuOYQ4uZPAI41NzP0g6tADojxl5dPUz4fKd87w2Jo6kzZ+fG6ytMsf9EXdvAFZREShcLf1LXgntDlUsU=",
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
        svg = "eJxdjFsKgCAURLdymQ2UQmGg7qBFRErXv5BLj92nBBJ9zTCPY/dFmILDPJAaDw1vuxp52wo1/ZocV6HLQYPyK2cKwsX1oNvBgDimjaXa8qr7L8+Q0tw33gOinSU9",
        categories = "devices",
        tags = "device,music,connect",
        contributors = "ericfennis"
    ))]
    RadioReceiver,
    #[strum(props(
        svg = "eJxtjsEKwjAQRH9lyL2rE5ImgaYXz35EiUIFBSke9O/dRLCCZckGdnZm33CfHjNO2RydJLAXHghasSC8RLSpJDMOu7o5Dt/90NQwqcWhtX2tTsdB/Gool6Vcz1iysQblmQ3r/8qmhX7Un1hFsJobi4WW2F4xyBbqwgYGk7ASTklSj9YqB+tzwg1HEg/G2f9LtwirxzsSDly9bxI+SG4=",
        categories = "devices,multimedia,social",
        tags = "signal,broadcast,connectivity,live,frequency",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    RadioTower,
    #[strum(props(
        svg = "eJxVj7EOwjAMRH/llL0Gu7SkUtKlMx+BDBJIDKhigL/HTkVKh4ul+O7pnJ7n1w2XHE4HGsAD8cTgjgSMSBH+awpj2rlzTNV/tC33JNoItVU9MfZNpG5N6H3WxxX6yYElQN/LnHMQNy3rP7BDYXQ1Hn4qXGy4a8Bae8lJ2tLZhl3A5Rp7auILkbU5Ew==",
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
        svg = "eJw1zEsOgDAIBNCrNHMBPwtX0MtoI902JLa3F6psCBnmQa2cmgZjR+qMA+mplwpjs6D1mUupt6itKzItDjJNFgfv/sw6wb45/GmwF8pcHu8=",
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
        svg = "eJwBNwDI/zxyZWN0IHJ4PSIyIiB4PSIyIiB3aWR0aD0iMjAiIGhlaWdodD0iMTIiIHk9IjYiPjwvcmVjdD631g94",
        categories = "shapes,design",
        tags = "rectangle,aspect ratio,16:9,horizontal,shape",
        contributors = "colebemis,Reund0,danielbayley,karsa-mistmere,ericfennis"
    ))]
    RectangleHorizontal,
    #[strum(props(
        svg = "eJwBNwDI/zxyZWN0IHg9IjYiIGhlaWdodD0iMjAiIHk9IjIiIHdpZHRoPSIxMiIgcng9IjIiPjwvcmVjdD64LA94",
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
        svg = "eJxNi1sKgCAQRbdymf8eKiiCuoMWIRYY+BESUbtvJIoYmPs4XJfWmsqCdHkShpBOVkmoLBTc8ODgtrhnzJ4mKWAOnTvdaGt/TEGYaGEx8gnYrvk3acheFcXffNMbfxcgPw==",
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
        svg = "eJxtjEsKhDAQRK9SZJ/MpBtbhRhwr4eQKERwISKit/cHupFaFFQ9nhubOaItVM2gRaIo737n5t3zkIWlMkeO/xVBYrhiZN8s0aIl6i8TH6LmFdkEYtKBNZn0pUM/haFDWAtlSSFsd09HndB9+x2RMjBq",
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
        svg = "eJyV0EsKwjAQBuCr/GQ/tTONtoWmN3DrXtJiuhCkBB+3d5KFigREAkkg37wyXI4xYHJmzxbW18QVVz0JhMSMwyY9j8MLSQ3xSqAnql63ohJ0voY6SobKinfg2lPORkKpaqnmuUGLJi1qCllSksM2N95qRUUJBn7TdfYRd2fE4LZMMTjTGYR5OYWYrw9n2BqsmWhU8p9tWrD9mvlqf4+nX/VH3BO4mGGv",
        categories = "text",
        tags = "search,substitute,swap,change",
        contributors = "danielbayley,jguddas,karsa-mistmere"
    ))]
    ReplaceAll,
    #[strum(props(
        svg = "eJxtzkEKwjAQBdCrfGY/NZlW20KTG7h1L2kxXQhSAurtnamgmxBIYPLyf6bHtWTMgc6+Q5cc+8Y3IwuEheJ0sOs4/ZA4SFICPdGMulWVYEgO6tgM15U/wbvEexoLW2ut896iR2uL20qKhVyO+8d7bVRkMPs/3ZZUsL0CCSEv6y2XQAPhO3gH8h3huc4l21hfmY8fjIVHuA==",
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
        svg = "eJxljlEKg0AMRK8y5ADWRGwt7O4NegihhQqiC/XDvX2TiCJIQiaEN0NCnscyDtMHeR6m5RepqVoI7jpZbHSulMJtR1NwQ5FIUhNW1VYJFI7kLFbdnpvpZDCQuyvJdtnCDkPuly/ekV5SPeyJnhto114WAocNS3//PjG9",
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
        svg = "eJxtzD0OwjAMBeCrPHnH2K2bZkg6s3CIiFakG6oifm4PER06RF4sf88vbMutYHtH6givdS75twkhL+s9l0ie8LdPJDWawrk+TOGRSsYc6epYFOovrlq9HkxlR5WWDlB5WktG9oaRdUwGg9Q5DewcWj2dVDHuLXn4Pa3KvR7iX6VsPFE=",
        categories = "development,devices,connectivity,home",
        tags = "computer,server,cloud",
        contributors = "FrancoMaxime,ericfennis,karsa-mistmere"
    ))]
    Router,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFJvXho+xkttiAeSsDk9ybpaVhmNs92MkwpmBB6G3fngnQgaMEdYWoIifGNi3vYmjf/1fyMt4FQaDWQxZoWxZjIW6/qDxoiHDw=",
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
        svg = "eJxlytEGgEAQheFXGXOfzDKbZWffoIfIFBtFVlJvnxoqujqc7486Fp0G0F2QAoIetkXQYYq1cYqfjK3if7R0a4ZesGVoNnKZbr3e12YGCuAr/9AJrtclLw==",
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
        svg = "eJxtyrEKwzAMBNBfObSbVheCFttzl67dAy24UEqGYJK/j5wheAgaTne8OE9LwTvJc4C9xokg7n4KBhZKjrcmcjydGobCHoL1ClKhVjsZ/Cvhihqoj7GXgbWXv+//g41JlIJVk5hHq56bHrPbpvIOrUI0bw==",
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
        svg = "eJyFzssJgDAMBuBVSgZQQq320HYDh5AoVPAgRUS31xLr41JPCX++hBgaA02DoM1CUygQtF9NsHAWZ0oWzrwk6kTVL8VEucvZ+wGsc5Rzvv6sVF87d4sXvYVWCrmi9qjjNKbuAEaiQ20=",
        categories = "charts",
        tags = "statistics,diagram,graph",
        contributors = "danielbayley,ericfennis"
    ))]
    ScatterChart,
    #[strum(props(
        svg = "eJx1jrEOwjAMRH/Fym6wTRo6pJ1ZWBnYqoAUpA6oQhb8PQ5IkKGRB9v3dKeL6bak+QrpOTgWB+llmxwsttwYt188xvv0yHAZ3FEEhE59Rj9jQI8B/EGUZTId6DN2ZQ61gHIuYSWkigrAe90Qr6NdA3HfthXW9HkQUez+vdB6KdU/iHY/6xv0u0kX",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School2,
    #[strum(props(
        svg = "eJxtjsEKwkAMRH9lyH1xE2KNsLtnL35EWQWFClJk0b83VtCiJZdM3syQdO1vJxwyXRQdLCgMSiWtXveSPpQNHKGQZr1AEH04+LbTuQ7Sgg0a5L9iz552/PXHyR/n2vt1KWpYN94skO4H1PNYhyPqPRMLoT4ybQljpumjNy1POBo++Q==",
        categories = "buildings,maps",
        tags = "building,education,childhood,university",
        contributors = "karsa-mistmere"
    ))]
    School,
    #[strum(props(
        svg = "eJxtjk0KhDAMha8SsrczCbV0oO0NPMRQBQUFERd6exvFH9TNy+J7+Xiu/481lB6LXGmGn4QFYgzuIyi42AyxrWDwyAhx8qjTmT1aqWwwuN3SkQaTWZVbkHhY5JHM4VmlL5qCviqt0GsAXTxnw6SVdcYviPmGFtUrPhs=",
        categories = "design,tools",
        tags = "cut here,along,snip,chop,stationery,crafts,instructions,diagram",
        contributors = "danielbayley"
    ))]
    ScissorsLineDashed,
    #[strum(props(
        svg = "eJxtjUELAiEQhf/Kw7vbzmBioHvu0rX7YoFBQSwR9e+b1WUTVhxGZ57vff45vhIuQZ0MmEcGo5dDmjWfzX+GzIlsvQC/64WWlxr8bg4c/BpLveQeXUuxoiRdmeJtivcr4icopxC/+ZqCyl+KWPkPnSGD0omlWgzTOZQ21xYlDLILZAE3WA9xW+26vbPIfQ36AYw/UcE=",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts",
        contributors = "danielbayley"
    ))]
    ScissorsSquareDashedBottom,
    #[strum(props(
        svg = "eJyFTdEKwyAM/JWQ97lZnDiw/sE+YqQyhQ2GCGv/vqYpbd8K4XLk7nK+RKpQxh47BMF/Hmpq7IaQYn6nKnxiMfgrB4KnXOgTgdrVIdC4rLJaRAz+96oJhh6fD2W0AUHdtWEXqwePNsqBAM/uOFRpe9r1bWl7ceruLCy4PZoBy9k/pQ==",
        categories = "text,design,tools,files,development",
        tags = "cut,snippet,chop,stationery,crafts,toolbar,button",
        contributors = "danielbayley"
    ))]
    ScissorsSquare,
    #[strum(props(
        svg = "eJx9jUEKgDAQA7+y7ANqW6vsoe0PfISsgoIHKR7099pVVBCEkBwyJJ7HxFMPvAasEXiTSAFLjL44y+jndhmgC9iQMhbERJnJ3YuwGtxFVIroIb5Hhn6ejFMEYsei1ffODqqsLzo=",
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
        svg = "eJw9jEEKgDAMBL8Scq+S2kMPSf4iUVBQkOJBf29qoSzLHmYZvuZ7g0XwzEAE0ZNCQuWxAmXbix0rFMGMYK8gke/zr58aVu6W6A4KaZhqu+YDh+Ubaw==",
        categories = "text,account,social",
        tags = "find,scan,magnifier,magnifying glass,found,correct,complete,tick",
        contributors = ""
    ))]
    SearchCheck,
    #[strum(props(
        svg = "eJxNjMEKgDAMQ38l9D6l1YODbf8iVVBQkOFB/95twpS0h+SFuGM8F0yedgtrBEUUXJvz4CrlDunEFH1c16jbDL09MROip4GgV3Gp9OLfjDCETd90+evMA0dTI4U=",
        categories = "text,account,social,development",
        tags = "find,scan,magnifier,magnifying glass,grep,chevrons,<>",
        contributors = ""
    ))]
    SearchCode,
    #[strum(props(
        svg = "eJw9jDEKgDAQBL+yXG/kEg9SXPIXOQUFBQkW+nuNQootlhlGj/FcMCXaOThBdNIJhLL2FWS1tdg2w+5EzISSKBLs+t4r/Thrq3iG525woa5lHpXVG5E=",
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
        svg = "eJwlijEKgDAMAL8SsqukOjgk+YtEQUFBSof2923a4bjhju2J9l4QBXcEy4JEzaVbeRlZ+T/SDafgFwgCTdu8On540QrMxBNp",
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
        svg = "eJxdy80JgDAMhuFVPrKAphT00LqBQ4iK8SZS/Nle04oVySEhPK9bxz7g9MSWcKS1z0MQTzVhvT+GIOM8SdBP4woNGhezDJM7f/rp32jpgmDw1FZgu3FnYFDGuS9hhUo+kC0qyZAVbhlerwczLA==",
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
        svg = "eJxty1EKgCAQBNCrDHuB2i3ED/U2fQiiQn3k7dst+giCgWHgTSi5bhgciYUw5OlT96KlU5hSmEyl0FsZt+8t12OP5OHBghXs4M294scqsQhktpf78Av3/SaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorHorizontal,
    #[strum(props(
        svg = "eJxtykEKgCAQheGrPOYCNVOIC/U2LQRRoRZ5+2aEFkGr/z34Qsn1wOBIG2FIJGHCrZdFK7MpLKZS6K2M6XvL9TojeXjsYNGwM/eKH8tODWQ1rsN/+AP4TCaG",
        categories = "text,arrows,layout",
        tags = "move,split",
        contributors = "ericfennis"
    ))]
    SeparatorVertical,
    #[strum(props(
        svg = "eJx1kEELwjAMhf9K6D2xabvZwbazF6/exxQqKMiQof/exHnopKOHkL73vjRtx+s03i4wdcYbGF+dYSf1/a19u1vkvn0MzwTnzhwDVcD2EAYHDqwcRofulPUgfeI6vwA3rwLgElKlfOX+00NGt7gOW4XldKvjZlwlcAtfQ53IclHhWJbuXNEe2FNAapB8wdGQkz9RdcMgogwQDnnxlAz+Z0AZw2VChEbjZT2ShwUijlI+6BIK2TLIghB1gM9e+AF+XIcs",
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
        svg = "eJxVjF0KgCAQhK8ie4BSiehBvUxJCtGDCNnt212j9GV/ZuYbk/yaRbGgQQQf95AtLCBSVW6eV9xywEuizoozI2HOMNzaFauptu+rU9NPH/H0omgL8yAVugovXJpX4Q+zlHqzZClq6xhWuvQDe1k4sg==",
        categories = "development,devices",
        tags = "cloud,storage",
        contributors = "colebemis,ericfennis"
    ))]
    Server,
    #[strum(props(
        svg = "eJxVyjsKgEAMRdGthPSi8UMQkqltXIREYQQLGSx0934GHSzfu0fWYfMwKvZlAeyzFp3k9+fkK1QDcdekYnOwZQI7FIkRgmKFYPuzLhTzj3HsL07oBCC1JFo=",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "mittalyashu,ericfennis"
    ))]
    Settings2,
    #[strum(props(
        svg = "eJxlksEKwjAMhl8l9J64pB0qbHsDr95lCgo7iIioT2/aOtdGekhp0v9L/ra7Hu5nOPZux0IiIGekEA66gSYu1N2DePM7YWRgWvtJ6zxJWyQ06im3SM2mVNDqfEGEfJEBPYWYnfQS8aIECaHcli03JAI12wKwyFjAjLZNcYyTHSBj/V6aUkItqRwBQXlgZUlsLmGCx1oxYWK/tiv89ovkzSjJr38fFREhQdGtJYfvgNva21kpUeyL4EypIfkN7RyZ7ffVx0B5u6FbxQ80dOPlNk4nuPXOOxhfvWPR+ExRi3J6+AA5JX87",
        categories = "account",
        tags = "cog,edit,gear,preferences",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Settings,
    #[strum(props(
        svg = "eJxVjUEKwjAQRa8yzL5jpmmbFpKcoF7AXUmDCbiQELDe3om6UAaGD/89vr1vNcHu8DyTBlYbGTKg5LijqZ86JmWWlZkG0D8lMPEyd6QGvfIk6kzLnzuavjEX9PbUNrwtMVR45L0mhwahHA4ZQb5GeEoeEFLM11RbLVbjvQ25hFuEIhiNCKGR5p2ObxL0A/kXBCkyPQ==",
        categories = "shapes,gaming",
        tags = "triangle,equilateral,square,circle,classification,different,collection,toy,blocks,learning",
        contributors = ""
    ))]
    Shapes,
    #[strum(props(
        svg = "eJx1jDEOgCAQBL9C7gEoKAoJ8BlCQUIsqOD33iEFmljdZm9mbUgl5MiKgw1YqA6ExtscKPB2eb7eDop6IWGijz/KwHtzwnK6IqsSa8V3XGsUT76j0gTGjSsBrGLUXBkyyRgeEUcHurZ2jVgaI01+tBuRwTt3",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,ericfennis"
    ))]
    Share2,
    #[strum(props(
        svg = "eJxNjEEKgCAQRa8yuI9yKHFh3qBDCAUKokIievtGcRGzeDPD+18lky3cJ7t24FikQUDYxtBmOf4fC5ZFMq3WHtIqRd+8Cw+k6EJ+T8YFCGqhgAQxvGloNbzKyUEGjUioOM/Oo/vD/QDuayZQ",
        categories = "account,social",
        tags = "network,connections",
        contributors = "colebemis,csandman,ericfennis"
    ))]
    Share,
    #[strum(props(
        svg = "eJxdjVsKgCAQRbcis4EYIyhQN1OSQvQhQrr7xjF7/Thc55w7Ktg5Cmf96qIGHEFkDT2IxG+gIGkkHodfomPIqK54Rm1+tyKjhokUZCfLmmhILGRhGkmfONxLNjmz+mVT7ckv9Doy/VufVSlrRqrHGnsCdm88nQ==",
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
        svg = "eJx9kE0LwjAMhv9K6L3VvDPUQruzl129SxUmKMgQ0X9vWx0b7OMSaJ+8SXh8vHbxdqH4CYqhqAtqryi+y6v2mx+u/eP0bOkcVMMgvKyRDPPngO7sSLQYVJTLtKEBiNHqxTS7EtcL8bSad0aOwBzc5sVcGWtJ0qSZFmcSwGElXarQ6Ly/nqQF6epeTK9rEPQF9X1X0A==",
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
        svg = "eJxdjc0KwjAQhF9l2XvX7tj8QZKzFx+iRCFCDlI86NvbaA+lLMxhv4+ZWB5LaXcqn8RQpiXxmuWd2HOOpz/N8WB1rmGzd9pzflW6Jb5CRkM9KhrEWlLIhBkEGn8HUjG+BnF+91UJZliBayrWDE6m88WIom/07vwF/rMrmw==",
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
        svg = "eJwdizsOgCAQBa+y2R4MfiEBahtbeyPEJbEwhPi5vUDxmnkzOvo9wWuwQ3iCS2RQSISvgph5i0A+HJTqYXVTAquvLRE4g4sYQXK1TrM8exhYHsmbCa6KWyz7A2J/Gws=",
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
        svg = "eJwti0EKwCAMBL+y5ANtpD0I6m9KESQR2oP+vjH0NDswm7q2eauga5X3ycQRYUcEB9g84E4lbX9YUqtyYXCmkzB5PYzBdTgsXk35ALQHGMw=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipBack,
    #[strum(props(
        svg = "eJwtykEKwCAMRNGrDLlAGzGLgnqbUgQxQrswt6+imxk+vNC02KMVTXP93kgCDxawg8CdYzylcGyVQsn1RneR+CJ0Xm+7bbRMPlX6AaLAGJY=",
        categories = "multimedia,arrows",
        tags = "arrow,skip,next,music",
        contributors = "colebemis,ericfennis"
    ))]
    SkipForward,
    #[strum(props(
        svg = "eJxljV0KwjAQhK+y7PvGzEpqhKTgAXqIEoUIClKkqKc3qT8RZB7255udDek4pdOB0j0ylGkqhSndIm+5D6sX7cPb1Sgcf25+bJfxmmkfefCkdtbsZ1l43Td6hhpH2IhxAqltxuPfNqArKaOSkl0E4zpZG3Xki8pMVgCDwnfNVV9/w56j4Di/",
        categories = "gaming",
        tags = "death,danger,bone",
        contributors = "ericfennis"
    ))]
    Skull,
    #[strum(props(
        svg = "eJx1kLEOgjAQhl/lcrvI0TYpSUvixuLKToRYNkMaxbe3VxSR4nAdev3/L1/N2F88uH64Om9RI4yTRcoUwmPovLMoEJ4WCwS+F1iZIycqc2u9g87imUrQmWoodyF2CgM8ORBP3HGIn1cmwhZC6CWJa/gUz4WcwkK5YpisRQKbdxvYu0vPNLHy+2DFrCb31BQDyprkPUTajVumDvRf7re+iPyvm96hhZ8CVVPepGpxtZBeFSNwLQ==",
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
        svg = "eJx1kMENgCAMRVcxXUBbCOGAbOPBxHiW7QVaQBFPP8Dro6079nObAq2gYQqY44pBGDPeogbv5gR5l9H0iEtjI6NABG8yEUiiIWbymfWdtnkKY6Eovl5amE2Z/ebP+2hCQV87ENexuVQzawba0p/oULZhR2uwojPyNdW2C3wD3odibA==",
        categories = "account",
        tags = "settings,filters,controls",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SlidersHorizontal,
    #[strum(props(
        svg = "eJyNkFEKgCAMhq8SXqA2Q3wwb9NDED3r7XNuZqhBT2Ps28e/ufO49ingplY1Bcglpg6ophZBeTcT5N0bpRkszOqy2ZCEoMobVEXHevwB26LvUVwYRYkA5iNuLN7Bkh6cZgSVC+s3Gi1yPE4Z5GGBn9JbsfhMjZth+8A33rhibA==",
        categories = "account",
        tags = "settings,controls",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Sliders,
    #[strum(props(
        svg = "eJwli0sKgCAURbdyefM+ilkDdQctIlJ6QoMQodp9aqML555jUtgz0mtJEtLT5o4+syWhCBziwbnQkVDOidBMZ4baOXNtmeEtrUL2Ws9YIEYIyersfqCqWy33ARr1HCI=",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,power,screen",
        contributors = "mittalyashu,ericfennis"
    ))]
    SmartphoneCharging,
    #[strum(props(
        svg = "eJxti1EKgzAMhq8ScoDMtDVVaL3BDlGmrL4NKWy7/VLL0AcJ+ZMP/i9sy6PAN6Ig5GV95hKRDcL20YvwXueSI3oEZYNTuNX+FF6pZJgj3tnCQNYkT87CHp0O63qyUoVaPQtCTkDIcGImL9Dybyn1w5U20sjgSK2+fi2bRd3Oh/YDt90zzQ==",
        categories = "communication,money,devices",
        tags = "contactless,payment,near-field communication,screen",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    SmartphoneNfc,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S9gFqioKHpj/wEWKL6U1KwPp7DZ52mWFiK4dRFyygu2ZTAc+g9ggCSEs91b47gX7Suk+Ko3cpXrspZcHGgXjVYWJ3TtMLfIYY6Q==",
        categories = "connectivity,devices",
        tags = "phone,cellphone,device,screen",
        contributors = "colebemis,mittalyashu,ericfennis"
    ))]
    Smartphone,
    #[strum(props(
        svg = "eJxtzlEKgzAMBuCr/OS9rn+wskL1BjuEsIGDMQYbordfY8H6IDRNSL5A0mf8Tbj3clMFOXOkR375A1109DKki6Eh7fQKtl82AYrWwm1R4ev5fmDRXmLjKViZK8FS0mp9s6YOlqFgLaruMFS9n8AOYepObmOEznXwB3xeOVU=",
        categories = "emoji,social,notifications,communication",
        tags = "emoji,face,happy,good,emotion,react,reaction,add",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    SmilePlus,
    #[strum(props(
        svg = "eJw9jFsKgCAQRbcyzH+PkYQCdQctIiwokAjrQ3efY9rH5cCdM1fZw1u3gddIPYINiSIxZhrVfXejruXZYdU4j0DDTa0EAQOnyWGVFaPccW4QSOOEEAtERmC0PbHLVnVTTZL7KpY3HiH52y8CrCuq",
        categories = "emoji,account",
        tags = "emoji,face,happy,good,emotion",
        contributors = "colebemis,csandman,mittalyashu,ericfennis"
    ))]
    Smile,
    #[strum(props(
        svg = "eJxtjU0KAjEMha/yyH5qk461A21v4NZ9qcIILmQQ0dubdsAfkJCE5MvLi9dym3FMtBewKx4eFtxStIwafRyCdkHb2b63lOOmaXOs56VeTqjPROwIS6JAqA+d+tGKc/wyEp5Z6mja8zA447WGw66sBs1Oyd1/LN5aDnDgyTC2Rv5wEeVizfTDXyrdN0s=",
        categories = "animals,food-beverage",
        tags = "animal,insect,slow,speed,delicacy,spiral",
        contributors = "danielbayley"
    ))]
    Snail,
    #[strum(props(
        svg = "eJxtjEEKgCAQRa/ycR81g4gL9S5BQUFFixZ2+0YZwoUL+ch7b8KxXysyRcMGmWVkX/lSWa6bwlisFH6XVFKHtW3de342LNGcPIHcYAcLeYUX0nALj8rQo+SgaTf2kPNVaOoPZ402RQ==",
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
        svg = "eJwtjUEOgCAMBL/S9AFqDfEEfKYSISEeCInwexfxtOnOtLUlaKUY0hWr431j6gimJ501OpaDqbSvKRNgMOztOva81VQ0B1K0AqhwxMD9pYm9zekO1PYhLZvgiEy/o8KHLiPgD8+/1lonpw==",
        categories = "multimedia,devices",
        tags = "audio,music",
        contributors = "colebemis,ericfennis"
    ))]
    Speaker,
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
        svg = "eJxNysEJgDAQBMBWlm1ALxBEyAUswCLkFBR8SPCh3ZtEH3L32d0JtiXbF9it9IRdSumJpHSMoXnXGD5V62oLKtj/1TGdK2bl6CHdIA7523ySI6osIj55Uh9K",
        categories = "design",
        tags = "",
        contributors = "ericfennis,jguddas"
    ))]
    Spline,
    #[strum(props(
        svg = "eJxNi70KwzAMhF/l0G4qKU1/wPbcpWv34hZcKKVDCMnb55whZDhOJ74v/p9DxSvJ/QK73voSDBo8GOOPc1Fu3uCqneR4aHyOm2Un9LUrlOAwxkfTonSsOfAadtr383tjsiTmgpl9ZHkSV8Hk65tso/ICLw0lRQ==",
        categories = "layout",
        tags = "split,divide",
        contributors = "Patchethium,ericfennis"
    ))]
    SplitSquareHorizontal,
    #[strum(props(
        svg = "eJxNjL0KAkEMhF9lSB9M5jx/YHdrG1t7WYUTRCxk8d7eLMhxRSAzfN+k9/Uz4ZblPOJwGaupw5WgcnKrDgPhcWyDlLTpeEmL5Ef4rg3VQvIugad9jdeUGjvKpivv+XjdMXsWp+DLLDTBzH+OftvZTpUfN4YlYQ==",
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
        svg = "eJx1zssKwjAQBdBfucw+MTd9GWj6B27dS1tMwYJI8PH3Ji6UItkMw5yZ4fbXUwyYvBwqVEEbytDv8mzov9KhKQiJrkBVUTq44g25pds8RoR5OYfopRY8limGT/fy0gieXtjk9bz4+7TSwcHC3mlGA90qXYOKYFDtqHQLo5hmqfBIXqyy/3FWpjw19kVzG3sD+LRZMg==",
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
        svg = "eJx1y0sKgCAYBOCrDP++h2boQr1Bh4iUdBGESI/bl7Rw1WYY5mN08ktG8HEN2RBThDO6HL6aLkOc8OZAuEta3ZWD1fucA5yhiXGoQxUoU4VNtSOYgGzEH/aQqPgAoR8mOA==",
        categories = "text,security,account,maths,development",
        tags = "password,secret,access,key,multiply,multiplication,glob pattern,wildcard,*",
        contributors = "danielbayley"
    ))]
    SquareAsterisk,
    #[strum(props(
        svg = "eJxNi8EKgCAQRH9l2buk1qGD679ESushCFnI/r7WIOLNYeDNhJpXgYtwRKiN0COcJQkTuhmBc9lY3t50E8OghxiORRgS4e4sOGs8dNSr+fsJnnjT+fwNsYwf8Q==",
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
        svg = "eJwli0EKwDAIBL8ifqCkvfSQ5DM2VKEnEZr8PoqnXXZ2qg4y0NnwRFgNL4RfHuOG5UbgIS9b9hmw1yOEXkmUvgHka3GTVqZ6xClx3/aLGo0=",
        categories = "shapes,development",
        tags = "git,diff,modified,.",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareDot,
    #[strum(props(
        svg = "eJxVy8EJgDAMQNFVQhawUUEPSTdwCLHF9CYloG6v6Umv//G55s3gEhwQNJddTZBmhLuV+kKPcJZk2nrkzofIx2oKSXCZgIJScPH2l/ErD2G7Hic=",
        categories = "maths,shapes",
        tags = "calculate,=",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareEqual,
    #[strum(props(
        svg = "eJwli0EKgDAMBL8S8gFpRVBo8xkNJiAeSsD29yb0sguzs6XxafDpZVIx7QitV8wIwnqLTeRkRRiRVJY4UHn0Zeip4uGLV9rcy7NHDuxqSPQDUoUaYA==",
        categories = "shapes,development,maths",
        tags = "git,diff,ignored,divide,division,shortcut,or,/",
        contributors = "danielbayley,karsa-mistmere,ericfennis"
    ))]
    SquareSlash,
    #[strum(props(
        svg = "eJyNjMEKwjAQRH9l2HtidgliIckfePUuaXF7kxK0/r1ZhR566mVgHjMvPe9NMWa6RnCojj0jOHF+6CG3WIMha+hdY/0NIPBDD6GSTmYoafNwAJ/3opc7blqm2rCsmYTwycSR8J7HppkuhPUPdJof2oz0mx3KF2gXMKk=",
        categories = "text,files,development",
        tags = "versions,clone,copy,duplicate,multiple,revisions,version control,backup,history",
        contributors = ""
    ))]
    SquareStack,
    #[strum(props(
        svg = "eJwlizEOABAQBL9y2Q8IGgU+gzitXILfc1HNTDFxtiK0RhVOsAHEbXSW7zvBg+aDAx2NHI0O+QLIog+D",
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
        svg = "eJxFjUsKwzAMRK8yeG8VyXYkg+MTpIcotNCCKV100dy+Suhno4HhzVN7nJ5XnOdwNEoZ+xFUEh0FmUyXQiYQBgtYSRVsxObNiFRqTJRT6O2weXr72RzKPhDSCfIRxokqR17cJCMyZX/lzX89bvcLVpmDSMDrm+wZsO7h6Ab1N/9dKqc=",
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
        svg = "eJwtij0KwCAMRq8SMgvVkKGD5jalCJII7aC3byxO3897uVW9YKSC6USYVJA9fFJEGPTfko9lSe7W5m0K3aq+jzMOFIFDIvDKS9yKfBYeGDQ=",
        categories = "multimedia,arrows",
        tags = "arrow,previous,music,left,reverse",
        contributors = "danielbayley"
    ))]
    StepBack,
    #[strum(props(
        svg = "eJwti0EKwCAMBL8SchaqQXoy+U0pgiRCe2h+3wiedgdm2uh6gRMjZYSvMJ4IHlODaJG0YznSpg2/TWFa1/dhLDlVoJwKQdzIw9yO/AW9F/Q=",
        categories = "multimedia,arrows",
        tags = "arrow,next,music,right,continue",
        contributors = "danielbayley,karsa-mistmere"
    ))]
    StepForward,
    #[strum(props(
        svg = "eJx9TcsKAjEM/JWQe2OTPthDW/Dmxav3UoUVPMgiRf/elBUfFwlhZshkJl3rbYZjxr2nCYTclhw5sMC6AWTnq4Ao1zHKeqgR4qoVY7ff2sTDj9/IbLiSkLwSRziWtBmtJb27J+DQ+X9yN/7z2c5Lu5ygPTKyRVgyCkK7K9hhWs/lCdImMtk=",
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
        svg = "eJwliksKwDAIBa8iXqCfRaGQeBkrVehKhDa3r0k2b+DNFDbnR8ArbisCf8k92QapLNNTceEAFbs1Kh4IGZ4IbexrV2h/s+8d/S88GKY=",
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
        svg = "eJx9yzkOwCAMRNGrWHOBLIpS2VwmQTEtshS4PVtDRTca/cfRP0ZJcILUh09NcIOy4ALF8f/hNa1rh+Ot9Y67mvoRVnWsWAGsgh7C",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchHorizontal,
    #[strum(props(
        svg = "eJxVjEsKgDAQQ68ScgE/FFfTXkaL020ZUG9vK2rpJoHwXiTH1aAx7Wqe80jkszRR0hFH2kw9F+Kqa5Ch8kEeq5GTe4Hu6Zc/7Qaoqh7C",
        categories = "layout",
        tags = "items,flex,justify,distribute",
        contributors = "ericfennis"
    ))]
    StretchVertical,
    #[strum(props(
        svg = "eJxFyjsKgDAQBNCrDNv72c0iCknqNB4ioKAgYmERb2+SJgzDMPDsE98Dm6OVJ2hYooHBWNJJPxsoeTsU422TCpao0Ao5dw5Tc9d570jiSEZCYkdK+PJlyct1sy3K/6WlHuE=",
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
        svg = "eJyNjUEKgCAQAL+y7ANMN9OL+oMeERYYdAjpUL8v9RKE0mnZnRnW+DX6bQF/WRSEEC1KBH/mzZmuYGf26QgwWxwFgQyMiwTT8YWIg6AKezLiFSYbmWZq0KBYL/u2UkbFyf0f5fvoBuA7Tc4=",
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
        svg = "eJx9jtEKQEAQRX9l2ndjZ2wrtesPfISWoihJ4u8xPGl5ubfm3KbjQj+HoYXZK6MgbF4Rn71Lly69cemmeumg8aoiBl4FXacX0DEyGiwykCA0JBFZUY7WwpPfu4qBuIsJsP4go8XM3J+TX4MCdS6ikdkB4dBOsg==",
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
        svg = "eJxljs0KwkAMhF9lyD3YzFp/YLdnL169l1VYwYIUEX17Yz1spYQchpkvk3jvHwXnJEcz2P6w7gmi8TGl8rStGq5LK11cfZkuVjKgLX9B8GnN/BJYdIbm65hvF+R3EqNgTBIE+TUpD/3s2jDYDqQGDfBdfjBsvGnyUN0PoEk2cQ==",
        categories = "communication,devices",
        tags = "photo,selfie,front,back",
        contributors = "lscheibel,csandman,ericfennis"
    ))]
    SwitchCamera,
    #[strum(props(
        svg = "eJxljN0KgCAMRl9FfIBqVkZgvk0XgqhQF/r2bc5+oIvtY9s5Myn64l3YRYounMcmYepmAQu2UWgs7nVBJ2lNfzvWVDMDWqMURXFmyhVnqInCByZIDe2oJcuaJdz/YfqU4f2ooMHwwBfxdTJd",
        categories = "gaming,tools",
        tags = "battle,challenge,game,war,weapon",
        contributors = "karsa-mistmere,jguddas,ericfennis"
    ))]
    Sword,
    #[strum(props(
        svg = "eJx1UEkOwzAI/AryA9pCvMSSm9/0EClKIrUH+/fFhhzqJgeDwDMwQ9q3pSzz+oJ9m9fP+2nQ3hxg4DCA5yexNeqXmdL94EypMTMyazCQiXM0UEjqgq1mQg/2AqKHkGou0v8F1yahTowK5jp3k89N+Cp5ZPGEErzYiJcunKyIKsfq5rGXdWhnUBCKVUo4sauncWoa/y7zBcEwY8w=",
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
        svg = "eJxNjE0KgCAQRq8yzAVCRSgYXbfpEJHSuAsZ+rl96qLcfIv3+B4dqzAEh4uyYE41oqehMk85bgL5dqgROKadxWHxUIhBeNpeKQg3XG714OkLagXTbP5eL5TtzAu7SyUj",
        categories = "text,development,files",
        tags = "property list,plist,spreadsheet,grid,dictionary,object,hash",
        contributors = "danielbayley"
    ))]
    TableProperties,
    #[strum(props(
        svg = "eJxNjFsKgCAQRbdymQ2ESlAwuoMWESmNfyFDj92nBOHXhXs4h49VBdHTYizcaSYKPLQvcEmb4spRxVP9cXtyhFLHEiTlXfQDTwNVa0LgP+gwS9/rgBl78gLGPiVs",
        categories = "text,files",
        tags = "spreadsheet,grid",
        contributors = "colebemis,zenoamaro,ericfennis,csandman,mittalyashu"
    ))]
    Table,
    #[strum(props(
        svg = "eJxNjE0Kg0AUg68Ssh8779WWWcx4gx5CqvS5KzL4c3tHFyqBEBLyxbH/ZqyJgZiHLluieGJcEpUo/iSsH36Wy1CziY/90MR/mw1d4ueFulUofJFAnZrovYBO8r4KV5I5rQ7UDrmhAiRY5eWcNotyJ7Q=",
        categories = "devices,design,development,tools",
        tags = "responsive,screens,browser,testing,mobile",
        contributors = ""
    ))]
    TabletSmartphone,
    #[strum(props(
        svg = "eJwti0EKgDAMBL8S8gBtg4iHtp/RYgPioQRsf2+ivWTC7myoeRcomc8iEckhPHxIiehXhK4JQh1oH/QumMJsuxQuvjN0r/qmOv1sRpqc19cqMt/M9AKX8Bz+",
        categories = "devices",
        tags = "device",
        contributors = "colebemis,ericfennis"
    ))]
    Tablet,
    #[strum(props(
        svg = "eJxNikEKgCAQRa8yzD5zKrOFeoMOEVNgUBDSom5fGpWb//n/PcNz4GUCPi1qBD5SBYsKnSkf6Ewm0YuTe69M24bdw2ixpwpIe5IRxvNHay2aFkgK1YAWsitifNoFRQ8nCw==",
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
        svg = "eJx9yqsNACAMANFVSBfgI1CFZRoECaoKtqdJaepQJ+4hTaY1Au0GuUCgo2VJgo5Rf0dz75sXVz9MtmNnF8XHIVU=",
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
        svg = "eJxFy0EKgCAQBdCrDLOPGA1qod6gQ0RK4yIIGShvnxohAwP/P765NmHwFs8ZiEAN7dCZsYIzP68FSfPUJYVdgEM8WCzSgvBY1Ai5/VSCQrijF/405dqUdd25F6BMIKk=",
        categories = "development,shapes",
        tags = "code,command line,prompt,shell",
        contributors = "mittalyashu,ericfennis"
    ))]
    TerminalSquare,
    #[strum(props(
        svg = "eJw1y8EJwCAQRNFWhm0gjhhCQO0mB0FUSA7afZYFT5+BN3H0umppD0Yv7XuTBPACHUgEnJLjsUmOBqdP4p1gMgm9YOnmraVVD4Z/sMIY2g==",
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
        svg = "eJx1z00KwyAQBeCrDO5D+8aYVDC5QQ8RaCGFUroopbl9R0mMgREX/rzvKYb39JnpNpirIzsxMZ3jaGRlxnCK6RiygS8QSEcMgt9V5SpHjBI1GpLnZqjnDDVAW2lIUKlY8l/o39ADS2hrjWPyfLzutPBgLoYWpOknOziZZdtHGU0hwavpN5OaYM122aSKXW2X7R9lyX5C",
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
        svg = "eJxFy0EKwCAMBMCvhHygjbTQg+YzwYMgPXgyv2+MUk8Lu7OxljeDUsKAoMGjW9BlaS2dyPEYiONPfRzoXmh9iTaW0qRmaAkfBOkTifrZ0Jz5AxGOIQw=",
        categories = "time",
        tags = "time,timer,stopwatch",
        contributors = "ahtohbi4,ericfennis"
    ))]
    Timer,
    #[strum(props(
        svg = "eJwli0EOgCAMBL/S7AdUDsYD8JlKpImnponyewpcdpOd2aiFjT65rSaEHdQSTpD+s2qRp1rCEUC+eOrkOW7jlyOL8luIHV4gbkvVobqzaO6WMhwe",
        categories = "layout,account,development",
        tags = "on,off,switch,boolean",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    ToggleLeft,
    #[strum(props(
        svg = "eJwti0EOgCAMBL/S7AcUDp6Az1QiTTw1TZTfC5bL7mFmklY20p5xgB45rWXEHdSqXM0yQgQ51Pe/sRElbbMriUX5rsTdTR40THdJjssHtdQcTQ==",
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
        svg = "eJxNi00KgCAQRq8yfPsoRVw53qBDRErjLmTo5/Zpi2j14D1eqHlVuhkOdJakwrATSHLZRBnGg66mQPVFDGMfYtgXFUqM2ZJx0pZWuvsVY8lOx+C/9ACDXx5X",
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
        svg = "eJx1y70KgDAMBOBXCdmjjVVRaJ1dXN1FxbiJFH/e3mYTRG457vjcPo8BznUK4pErhMujRbg9xi7zukiIe4awx4OxcamCxm1DEJg8dmyg6ovRUFJSkhNr2nJgYDAaiu2wCpW8Yf2FQvZXPvGsKuY=",
        categories = "gaming,development",
        tags = "lego,block,addon,plugin,integration",
        contributors = "ericfennis,jguddas,karsa-mistmere"
    ))]
    ToyBrick,
    #[strum(props(
        svg = "eJxtjkEKwjAQRa8yZJ+xXzNtCk1v4Na9RCFCBSkiensnWtpCAyEf8t+8TPc4PxNdgjkeyKV2ADWm73b5te/mzhFwcoXCE6piAU8SLddUWRA7vfASrrfkHZXK8+E2Zl6ThVXMGKymJItlLN7GOFxpDIbFUHwH02h8goFk6F/P2NQrLSVq2bYm+GRl808WwE+z/mfarwxfN9lWIA==",
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
        svg = "eJxtzUsKhDAQhOGrFL0PM906koHEG3iIYRTbhSASfNzeqBvFbOvjp9zY/AMWTzlh9ZQRtOlaDZ64IIwRhDB3ddBjKd1rD0o3/IKi9lTlYNZT9u0iLMgm+4Tegr8mYoLYQsSISVgVs4++U0fFXTbN+zt+",
        categories = "transportation",
        tags = "railway,metro,subway,underground,track,line,tourism",
        contributors = "ericfennis,karsa-mistmere"
    ))]
    TramFront,
    #[strum(props(
        svg = "eJxtjLsKAjEQRX/lkj6YuYbsCtmtbWy3lyhEELGQZf17Z1yNjcUwr3Nuvh8fFafBHbZIVXo35o2dxtweskOaJZYA8QJ6gvuu6Bg8vWhxSn+0HmlSSTkxxbPGog4IDQFn/pzr5XbGwsFJcFhk7U/bO+22i8GGfeDv8w3HBq0hscEvfGo3dg==",
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
        svg = "eJxtjEsKgDAMRK9ScgGpH6rQ9DJaTLclYHt7TRQR6WqG4b3xOa5sKsIAhmLaiRHsDOZIG9Ndc0Hor6gaRdDgO/GCV/thPw+Lcg7k2P1onRrOpI4dX/wEhQ0sOQ==",
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
        svg = "eJx9jr0KAjEQhF9l2D4xo+v9QHK1ja29rEKEK+SQQ9/e5AQVBJuB2Zn9mHg93jJOSfZbsNttzPkGwdF5LcJDa6FcqkGxmcFqDsJrEc6kDHFVIUN8o6jos44KndUCfONquyKyW3/6dplsPMMeSdgJ7J6kFUxJls4r/aYuC/u///wFPAHa+zqx",
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
        svg = "eJwly8EJgDAMRuFVwr+AbRW8NNnAIcQW05uUgLq9ht4eD7587aZUGNtKKWoMkDz5k9zrYdQfRgJpbacaIy6gce5WTP8KoJcxO3MgH7RLFsk=",
        categories = "devices,multimedia",
        tags = "flatscreen,television,stream,display,widescreen,high-definition,hd,1080p,4k,8k,smart,digital,video,home cinema,entertainment,showtime,channels,catchup",
        contributors = "ericfennis"
    ))]
    Tv2,
    #[strum(props(
        svg = "eJw1i1EKgCAQRK8y7AVKIfxRL1OSgqjYQnn7NiEGZj7mPdvDzujDkSZIG0IM6YzsSG2EOx0c5VsJz0T6HG+Xz/O21TxyKgGtpsKXSAYaSsNIJvcT/gVb3x6w",
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
        svg = "eJxti0EKwCAMBL8S8oFqsEjB+JseBFGhPTS/b6r1JoEM7M6GVrPkVE5oNZX7YnTgwemRGc9jDNu0YuiuECMZhEdpd6VlPBDE9lj9hfuV7p/QmCin+wIbSyQZ",
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
        svg = "eJwtyjEKgEAMRNGrDLmAuyGkSnIDW/sFBQURCxH39roqU83n2V6OGaNTr5BTi0KR3mVGGoTCukbC1mWbUNmJE+HKTkKo+b9ffmxTcQOYNBU5",
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
        svg = "eJxNi1sKgCAURLcy3P8eKiiCuoJahFhg0EdIRO0+L1HEfMzjMC4tJa0z0ulJSEK6qhtCqUbBdQ8Obot7xuRpVDCHzpoZbz8iBYSJFhY9q7EN57dpyFYNCkJ91xtHBx/X",
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
        svg = "eJxljEEKgDAMBL8S9gNSqOIh6We0mF5LQP29qVAo9LSHmVmu+TB6BRGkuVxqgg30CFZQ9Qmgu5ymgh2Jl6Yn/qMGnXob4uwObz37AIKhHp8=",
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
        svg = "eJxtjtEKwjAMRX8l9L2xCevawbZ/GShMmOKDD9vfmzSd6JRCQntPcto/pucM58HdKGNugBg5AmFiL4Vmj4GniCE0YDXo8UjsE4ZUHttaLWqxixAWHWddRG7sT2oZ+7crIiUgCWOxFOzXAmKB/xaoFh3035Ller/ARoNjBxsPLjpY5ZalsTYBFdnB8lb4bCAbGI+gRNRZRq2h2nUB8wFeP2Cu/yiwDe3wC6ONV3U=",
        categories = "text",
        tags = "url,unchain",
        contributors = "ericfennis,csandman"
    ))]
    Unlink,
    #[strum(props(
        svg = "eJwli80KgCAQhF9l2Xs/W4QJ6ht07S4p6S1koXz71mJgmGG+MSUeDNUiEUKK+Uz85/JYnBDEZyn1K3cOnGRe0ZmhHZ25PCcIFjcFRLvyCywwigh0rztqYEPcC9HdG3g=",
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
        svg = "eJxtjdEKwjAMRX/l0vfWJm3XDdr9wX7AN6mCwgQZPujfm1phwvaQhHBOclO5LWW+YMmKFMo7qyjjJZtVYzo0OqafVYFvGlvVjv6sx+l5xTmryZsIGoyThlCNSlZ+Z4LTQsHg45ZPg+EO0XQ9Aohn3n1CFuRFYDgTtNSe08s9SJNkffua9gFD70HD",
        categories = "devices,multimedia,home",
        tags = "universal,serial,bus,controller,connector,interface",
        contributors = "karsa-mistmere,jguddas"
    ))]
    Usb,
    #[strum(props(
        svg = "eJwlyjEKgDAMRuGr/GQXk4LSIckNPESJgoKDFAe9vZZOb3ifxlHj3FCNJkK8RvnPYySJXMd+Xa9y71iNlsRIUjIyGAIeZAY32IB/P6sUXg==",
        categories = "account",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    User2,
    #[strum(props(
        svg = "eJwtzEEKgCAQheGrPGYfNRaS4HiDDhEWJEhJucjbhxVv+T5+m+a8YRGaeACbWUOjq2tYoSNn2wqc9eH0cYUvQobgb6GRcAoNlXyns+mIJYZ9RTrCni8h1mAGj+AeSsG8vR+5BwVcIkA=",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "karsa-mistmere"
    ))]
    UserCheck2,
    #[strum(props(
        svg = "eJxNTcsKwjAQ/JVh70F3XaqFbM5e/IgSCwZCG2op9u9ttIcyh2GYly/d/MLT6MENhBcnnUJxrnDq9N4cNXQRCv5US8HHNMXcYzJSQvwYtRutRtca+ZvBlzGvOQ09ypiG+W20/TCDb+ALRND+9vZQ+AJWGyah",
        categories = "account",
        tags = "followed,subscribed,done,todo,tick,complete,task",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserCheck,
    #[strum(props(
        svg = "eJxVyjEKgDAQRNGrDNuLSZBgsZsbeIiwCgoWEizM7TWm0DDV8B8f8VwxC012hDPRw8OUddbBUOC+gMC6Jd0X6CVkHSEJDQTNz3tRzR/Llf14424wwh+9",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle2,
    #[strum(props(
        svg = "eJxdjMEKgCAQRH9l8V65ExiB+gddu4sFBh1CIurv0zoksYfHMm9G+yX6daZoBEtB/kxE4vXQ6ubNrf689qfJUtvcHmgyYugIslYKI/cOBJLpmFAhqPInHJy1vJG79gY57iVr",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserCircle,
    #[strum(props(
        svg = "eJx9z8EKwjAMBuBXCb3vd2nWroV14AP4EFIFBQUZHvTtbdeDPXQjp5CPP8kU70t8XCl+gmKnaAlKFMVv6oyap0MZz1PFXJn7FQ81ep3fN7oEdeIehlhgjpYs9WtpYp9xRn/61IyR2GLo4DtIQ7CBzmF+W1hYYocRkkwLeDCxht4BJaFLh3B7hcsJsgkGCJVDEmk92udHJcfU4Af0XGS3",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere"
    ))]
    UserCog2,
    #[strum(props(
        svg = "eJx9z80KwjAMB/BXCb03Lk3XD2h39uJDjCooKMgQ0be3oYft0EkPKeTHP0kqt6XcL7BkxQrKJysKtX5rHdWUDq09pZXZxmJTfoue8+sK56xONACNRzdbsDDI0/X3NmLFrPJhCD2QQ6sxauSOoBENEEt7Tzh0QAE9cjU9EJGADJo/oCXougj1RwRJ4F1gkaEtUknv0EEOZYnZgh8zqmUN",
        categories = "account",
        tags = "settings,edit,cog,gear",
        contributors = "karsa-mistmere,ericfennis"
    ))]
    UserCog,
    #[strum(props(
        svg = "eJwlikEKhTAQQ68Ssv98Z5Ci0OkNPIRUQUFExIW9vVMli0eSF4/xWjAZB2kh/RgQ0NT8RNEwxX8VUszrmbcZp7El8m3sHMXYV+U7U9zWfUYRowhxqzM4vasSRd/d9aqlB33OH2I=",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "karsa-mistmere"
    ))]
    UserMinus2,
    #[strum(props(
        svg = "eJxNi8EKwkAQQ38l5L7oDMOKsLNnL35EWQsWikiR0v17Heyh5BCSvJT38Hni4bxLhsqadDAYzqFkyW75mGGrspZTnGpp09LmEYvTiLY5rz/rzksg/7GWeXqN6OoUITZxqhJd9hx9Djyw+gXIQyPD",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserMinus,
    #[strum(props(
        svg = "eJw1y0EKgDAMBMCvhNxFU6RYaPoDHyFVUBCR4qH9vUmr5LCEnfX38uywMs40ArnFgoVBryMDAwbfKwg+HimeGyTGESFmxkmiMDolrQz+PK4NimEkQYUqyvo6SaopWtVna2mxbaghY9pW/h+/5CUqcg==",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "karsa-mistmere"
    ))]
    UserPlus2,
    #[strum(props(
        svg = "eJxNy8EKwkAMBNBfCXNfNCGsFjZ79uJHlLVgoYgUKd2/d2OLlByGMG/Su/886WG4cyThJUivpHT2Cxr0Fo8/6SLI6eSjnMo4l2mgUg0X0GxQUFkNnZOtzGkaXwNVNlxBVQzc0OrZteRfNu3qYJl3FDckso/5j7/wzi7T",
        categories = "account",
        tags = "new,add,create,follow,subscribe",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserPlus,
    #[strum(props(
        svg = "eJwli0EKgCAURK8y/H3Ut4gW6g06RJik0CJEyG7f/8gsBua9sc9RE05HO28wfKxYMWkGNpjI21EFb0Mu4Y4ojhZC+BwxSzdpo1LH3pYYKoTOhDefNQnfCEU8Q0gxX6n2qakjRz34HzSxJAc=",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare2,
    #[strum(props(
        svg = "eJxNi0EKwzAMBL+y6B4aqdD2YPsHfURQTGXooRiTJr+PTHIIe9rZnVCzNtQ1khAsl4+1SPwiOLkT/mVudoCtgxRuXUhBS9VvhvqN3VRfeSTU83TMKfymZpgjvZ8QXgaZBILRw5BB7HHtkEW62520A5jiKDI=",
        categories = "account,shapes",
        tags = "person,account,contact",
        contributors = "karsa-mistmere"
    ))]
    UserSquare,
    #[strum(props(
        svg = "eJw1y1EKhDAMBNCrDPO/rOmKu0LTG+whpAoKIiJ+tLfXVCUfIZk3fu32Eb3yLzWk7Ro0qGxe4lAx+LeB4OO0xXnApqyJmJS/c2Vla+QKg5+nZUASpXyJ5JTOEVkKzucpH9Ombns/H2OVUpVSfewBuWgqQg==",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "karsa-mistmere"
    ))]
    UserX2,
    #[strum(props(
        svg = "eJxNi8EKg0AMRH9lyH1pNw3awmbPvfQjZCtUkFKkyPr3JoogOQyTeS/9uv8Hb6VXbMBxDtwJBFe/IEGezblDZqacLi7lVIapjD0mJSGUqvSwWJRaR/Yxp3H49qhRKbaEhS1vhGrJbN3+d6edOrG+ObM5G3OoB7sCv8Yuow==",
        categories = "account",
        tags = "delete,remove,unfollow,unsubscribe,unavailable",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    UserX,
    #[strum(props(
        svg = "eJxNizEKgDAQBL+yXB804UACuattfEQ4BQULCRL09xJtZKtlZtKRzxWz0OQjgq8uZAajb3PseIz/D66BNHUt0mRbsX2B3UIDoQgxwS4h/zof1QeMihjF",
        categories = "account",
        tags = "person,account,contact",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    User,
    #[strum(props(
        svg = "eJxVjEEKgCAURK8y/H2kIqLwvzfoEGJBQYuQFnX7tECIWQzDPB4f6VwxC03aQofk4KBaBm2gKPLYgMh5K3lfUIQsIV9CvtYtFBrynZG7y5i/yw0OtkZBv9t38QNEPx/i",
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
        svg = "eJxNy8sJgDAQBNBWhr0H2SSKgcQOLEJQUBAR9KDdm5/GwzCw+8buwzljdNS3kHwILZSPQQ51tgqis5/jBurQUNAwIqewddkmXOyIa8IlHRnCzalkvHoa0I+WX1yETpOXPmZ9KVg=",
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
        svg = "eJxti7EOwjAMRH/llN0hNkmUIc0fsLKjgBQkQAyoKn+PA1TtUN3w5PO7/Dy9Gs6DuQsSBEICmWlK3vV/yYul2lrBpnZISCO76mBDsD6Ae1qsesKB6dsRj7Q15gh2du+PsTrSRbeJexp57ZfJ7fq4YOLBiMH7D1Eopx9V7VL5AHalOD0=",
        categories = "devices,connectivity,account",
        tags = "smartphone,notification,rumble,haptic feedback,notifications,screen",
        contributors = "lscheibel,ericfennis"
    ))]
    VibrateOff,
    #[strum(props(
        svg = "eJxVy0EKgCAUhOGrDLOP6FHgQr1LpKSLIORBdft0EdVqFv83dp81IThuAgOBdAJ5lt72rXv7qsq+BD9W4qI4HQ2RYl6TOg4jcTlORKlhII4cNDVSX837G2bRIcw=",
        categories = "devices,connectivity,account,notifications",
        tags = "smartphone,notification,rumble,haptic feedback,screen",
        contributors = "ericfennis"
    ))]
    Vibrate,
    #[strum(props(
        svg = "eJxNjLEKwzAQQ39FeHfqU83hwfGcoV07dAu0kIIpHUpI/76XEHDQ8CSQlD/jd8Kjd1cJnSp0kDgSRDCJkTO7c6wCuZBIc3Iln9ZRyW2qEG0rb26Ix+x5S8dXz4lVzIZ7u6uv9xMLe0c6/HYuYrS4waprqfwBCjIqnw==",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film",
        contributors = "colebemis,csandman,ericfennis,karsa-mistmere"
    ))]
    VideoOff,
    #[strum(props(
        svg = "eJwlizEKgDAQBL+yXC/iEUKKJM+wsBMTvBSChAP19yZaLAszjD9XFaRABzPcYGHQNruFoh+7i77mTVHvQEyoz3dXSSqBJkP4ecOWILnsoo1zr3sXX+x0Gi0=",
        categories = "devices,communication,connectivity,photography",
        tags = "camera,movie,film,recording,motion picture,camcorder,reel",
        contributors = "colebemis,karsa-mistmere,ericfennis"
    ))]
    Video,
    #[strum(props(
        svg = "eJxlzkEKhDAMBdCrfHKBmZZh6KLtDTyExGIKLqQU1NubqqDgKpCfF74viSuWPFQJZL8ESXmUGsj8CVugH2HVgFCOEf2ngejnvgqGQJ2FE3UatFX0nAtPCazWKC4HZsWu3ZzpgzsYK+7Fb9Z6XM9uvwOLszO6",
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
        svg = "eJxNzLENwDAIBMBVLBaIsKIoBbAMcmHJSuEKtg8OLlJ98fdP2qeOVibDCUWN4YpwBqwgdGQrtNUuPoc35OrHRn9acYwyXrxmWk1suM5DLyUvH50hAA==",
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
        svg = "eJxdi0EKgDAMBL+y5AOSQIVC29+IFEpb0IP9vUnVi4fskmQm9FbG3ip6y/U8IjHDYYWHzGFbNPTM3tJRCssrpVBy3TBENUe4rFdtjiRCGNrecKN+7POb6FRkKh97A3ntJiw=",
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
        svg = "eJxVjcEKwkAMRH9l2Htjk221hd2evfgRZVvcggcpC+rfm+xBLIEMeTNDwr6mgnd03uFTd163ey7R8eCwqyEOr20puZIpnKwwhedcMpbobh7jLBC0OgxpJHP3DyBWsvihxJx9osEydCYPIabxwbaTEqPoiKv2dEFrXqMm9Q31ir0e1lK5Cv9+fAFtcDGw",
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
        svg = "eJxtjc0KwjAQhF9l2HtiduhPCknPHvTqvUQhgoIUKfr2RoW2h7Isw8x3+EK6jul2QXpH6QTpFcULxiiUPuz+sA+P4ZlxjnLvoC0aq6b8QBCunILWq3V6IKH1qZ6JMzTcV+sOTuqWoSSzNuvB/NRf5SI+elCz3wBalO1UzeQDsg8ytQ==",
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
        svg = "eJxtjrEKwjAQhl/l5/bE5JqGDEnAzcW1g1uxxRQ6iATUtzdBqS2UW767+47//L3PCUOgMzOcbNqOVc9gqFJaFDqZdS+4q9ZxGaGRbGFlOztR8O9CS+OgZleNlf/LuVD0hxoe/fKChXZJ8/7GbDaP8ZrxDqQV4TkNORVkQhqnW8pffgWy9aCq8QOOXTfT",
        categories = "buildings,maps",
        tags = "storage,logistics,building",
        contributors = "karsa-mistmere"
    ))]
    Warehouse,
    #[strum(props(
        svg = "eJxNjk0OgyAQha/ywp6pjO3AQr2LoSaaUDXWRb19B42tYULC+/mYKg5LTB3iVhvHBkttxCB+9ldT3Q67qeYpbWkYO8zTMK7vbMMVyLdOqZPTZ0rz7drjWZuXE1Lbk4il4OydikfLYBT5WLaOxPWWScJVRpaTJR+QGztbiReupxCg8FJIM/8ujm7P5Pmq7h+lc4Uf8AtuNz54",
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
        svg = "eJxNy0EKgDAMRNGrhNmLNgq6aHsDDyFRqOBCioje3pZQ6TJ5f6zsUY6N5HUwHSg6TCB50sXwtlX2tmQKOetRRlV2Lleg1WEeiTmo5V8lhhPdzfDTB5V/JPo=",
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
        svg = "eJx1kstOwzAQRX9llL0Hz/iRWGr7BemWRXdRQApSQCwQon/PnQTCo44sj1/X1+NjH16Ht4kejs2zkiqJd+Kb0+HOpk+HbVEydU5YWrJwKzgHjlhTToESicxiPQtD4ERWPYqgRi6xh6bMztZdTeRMdKnk0SEN6P9v8d++hSTNm+3IKUGeuLPkNXGxttTyF89FKLHmkQWbNHOI2Aub7LgNvQQKu7di7XB+1oqxetJJ33WIFFe1Q29yep9/pii62nXPIgvWdolhY1ZJYSFGfgW7x3XVXOrvK7lGdjP+SxaaWGgJq7VhdeBbu0THbUT67MsIrg58wdWBL7jizNyr2JfZ+Q72GbLiRX55z08vj3TVY6Pa0IegRfM1vC5DSE10+gTEAaX0",
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
        svg = "eJxtjMEKgzAQRH9l2HtsJtamhSTnXvoRZVtooYciEvTvNYp68bI7zONN0G+rvze0j+IFbZRaoEMUOknhtNAU/s/ug1eUBy1u+VJQqVJY9cWYZ7juHPpn+Hzd/Q04B/pMtagaUzWgIXivtWRrOL3pMBtu7ghctTKk",
        categories = "text",
        tags = "text,selection,letters,characters,font,typography",
        contributors = "danielbayley"
    ))]
    WholeWord,
    #[strum(props(
        svg = "eJxtkNEKwjAMRX8l9N2YpE1WYdsf+BFFBQURH3zY/t50Vpkw+nChnMsNp7/fHheYeQgSYBIPz7nltHyP/b5CY/8sryuch3DMqMCGWhQUyB9DB1TBiqxAgYxZCjv+BRNytxM03cCZ0Az0lJB4h9G8zQkPwIwxQcTOtkqGWSsiWtgXqA0JigCj5Y2O8/EPVpTkV60HFjHVADczLH5W+LiiJol+dt7gW05Y",
        categories = "connectivity,devices",
        tags = "disabled",
        contributors = "colebemis,ericfennis"
    ))]
    WifiOff,
    #[strum(props(
        svg = "eJxtykEKhDAMBdCrfHKATpKZjl3U3sBDFBQURFy40NubiooLSeDzPy/OeenR1tR4yDcLw76cQH5gSvFTRIq3C87k3/ns4U9ZvUJFcEGzGL+g8lOOw9RhlZpECZulMmHV0h2LTXpM5otMO9XOKSI=",
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
        svg = "eJxtT8sKwkAM/JWh98ZNso8s1IJ49iNEhRVUPIjo37vbongoIZNMMgnJcN8/Co7rbmcQKdaNw6qVxuHXSGBX9JrgSs+kXhc0LODwTEvTpC5hwg0LqWJGN1nbvQ8IX0qaGb7hzihqvYmy8dYohZrGaDCyJDVXjwwp8cAU4CGI1a3uIO9STy6QudwT+wAmzn+PXc63E9687qTDS2pocabvmVZpE40f0WxCoQ==",
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
        svg = "eJxVjUEKgCAQRa8yzAVKC2qh3qBDSErjLmQou32OYBCzmM/n8b7JcWd4LE4IFNNBbHFFKK3I9WmEOwUmqZ0ZBHfm9EwQLG4LKHXNXoOGsV1NNAsoiDPdrv76z9gnSkO6/wUWnijq",
        categories = "development",
        tags = "action,continuous integration,ci,automation,devops,network,node,connection",
        contributors = "danielbayley,jguddas"
    ))]
    Workflow,
    #[strum(props(
        svg = "eJxNjtEKgzAMRX/lkvexpnXFh7Z/sI8QNqggKsyH9e9NooIkcEI4JDdN4/xF85kiobHhLwgCWXqmkp7qlLQOW8Un0zuAfeXXEBDgwFIOsT46VVUSdZmaHV6Xcd5+mThCuwP3Onhn7mmVZO7tLbsjDPdk2YRXjB0Y9i6A",
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
        svg = "eJxNi0EKgCAQRa/ymX1ERZKg3iVSGhdByEB6+5jauPmL9/5zJR2C0jzNhCdHYU/TRqieFkL7llM+WX5eqj6DG7UL7t6FET1d0wo7GBhVCjtlYdGbF0dZH9U=",
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
        svg = "eJyFy8EKgzAQBNBfGfYDtt01iRGif9ODICq0B/P3nUZ6KD142RmWeWXflrrM6wP7Nq+v5yjmGgxJ+wjr4LC7xh5BB5ep3L7zqfzD/BmSDwY3OljUlFguYEZGBwu07ZKzEKZf2NDho7gLqjEFxxn1/HLepm9MBTqC",
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
        svg = "eJxdjEEKwCAMBL8S8oAWxYoH9TPiQZAePJnfN5pAS08bsjMbSxulVxgJA0KhhMZwzp05nlLn2NtdgSy//eEvhPk5TULLEkmytGBVVrkWt+rUUzj8WFJ27k5Qp+q7+wBAFC11",
        categories = "accessibility,layout,design,text,photography",
        tags = "magnifying glass,plus",
        contributors = "colebemis,ericfennis"
    ))]
    ZoomIn,
    #[strum(props(
        svg = "eJxNi0EOgCAMBL/S9AGaEiUcKJ9pOJAQD5zK762CxFO7uzNRSpOaQTojEUJjDAiib0pxH3OKtVwZlBidQX1edYb5zZ9WrdekB/4pYRj0GcfEabE3W+kidQ==",
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
