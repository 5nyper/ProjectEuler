let file = require('fs')

let points = file.readFileSync('points.txt', 'utf-8')

points = points.split('\n')

result = 0

points.forEach(function(set) {
  isOrigin(set.split(','))
  console.log(result)
})

function isOrigin(triangle) {
  let origin = 0
  let triangle1 = triangle.slice(), triangle2 = triangle.slice(), triangle3 = triangle.slice()
  triangle1[0] = triangle1[1] = 0
  triangle2[2] = triangle2[3] = 0
  triangle3[4] = triangle3[5] = 0
  o_area = Math.abs((triangle[0] - triangle[4])*(triangle[3] -triangle[1]) -(triangle[0]-triangle[2])*(triangle[5]-triangle[1]));
  o_area1 = Math.abs((triangle1[0] - triangle1[4])*(triangle1[3] -triangle1[1]) -(triangle1[0]-triangle1[2])*(triangle1[5]-triangle1[1]));
  o_area2 = Math.abs((triangle2[0] - triangle2[4])*(triangle2[3] -triangle2[1]) -(triangle2[0]-triangle2[2])*(triangle2[5]-triangle2[1]));
  o_area3 = Math.abs((triangle3[0] - triangle3[4])*(triangle3[3] -triangle3[1]) -(triangle3[0]-triangle3[2])*(triangle3[5]-triangle3[1]));
  if (o_area == o_area1+o_area2+o_area3)
    result++
}
