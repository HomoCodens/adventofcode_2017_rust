/*var b = 99;
var c = 99;
var part2 = true;*/

var b = 17;
var c = 68;
var part2 = false;

if(part2) {
    b = 100*b + 100000;
    c = b + 17000;
}

var count = 0;
do {
    var found = false;
    for(var i = 2; i < b; i++) {
        for(var j = i; j < b; j++) {
            if(i*j == b) {
                console.log(i*j);
                found = true;
                count += 1;
                break;
            }
            if(i*j > b) {
                break;
            }
        }
        if(found) {
            break;
        }
    }
    b += 17;
} while(b <= c)

console.log(count)