using chrono and chrono_tz for date-time things

`cargo add chrono`
`cargo add chrono_tz`


use `.format()` method on Tz to get various formattings, %c is the most descriptive, can use %Y, etc for explicit year, time, etc.

use `chrono_tz` for getting timezone things, `.with_timezone(&Tz)` for adding specific timezone