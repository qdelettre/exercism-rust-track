no-more-bottles = No more bottles of beer on the wall, no more bottles of beer
go-to-store = Go to the store and buy some more, { $max } bottles of beer on the wall
take_down = 
{ $count ->
    [1] Take it down and pass it around, no more bottles
    *[other] Take one down and pass it around, {$nextCount} { 
        $nextCount -> 
            [1] bottle
            *[other] bottles
     }
} of beer on the wall
bottles_on_the_wall = 
{ $count -> 
    [1] 1 bottle of beer on the wall, 1 bottle of beer
    *[other] { $count } bottles of beer on the wall, { $count } bottles of beer
}