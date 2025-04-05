using chrono and chrono_tz for date-time things

`cargo add chrono`
`cargo add chrono_tz`


use `.format()` method on Tz to get various formattings, %c is the most descriptive, can use %Y, etc for explicit year, time, etc.

use `chrono_tz` for getting timezone things, `.with_timezone(&Tz)` for adding specific timezone

about web-logging

`middleware!` expects a static string type, which should live long enough (atleast as long as sever is live); so used a closure inside middleware! block to make it work

can't move out of a value while it is still borrowed, just rust rules
simply put: ownership cant change while a borrow exists