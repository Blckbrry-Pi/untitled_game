const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Enter your JS level:\n', (answer) => {
  // TODO: Log the answer in a database
  console.log(js_lvl_to_rust_lvl(answer));

  rl.close();
});

function js_lvl_to_rust_lvl(js_level) {
  let parsed_json = JSON.parse(js_level)

  new_json = {}

  new_json.attractors = parsed_json.attractors.map(attractor => {
    return {
      pos: {x: attractor.x, y: attractor.y},
      field_size: attractor.fieldSize,
      phys_size: attractor.physSize,
      rot_offset: 0,
      spin_mult: attractor.spinDirection
    }
  })
  new_json.zippers = parsed_json.zippers.map(zipper => {
    return {
      line: {
        start_point: {x: zipper.line.startPoint.x, y: zipper.line.startPoint.y},
        end_point: {x: zipper.line.endPoint.x, y: zipper.line.endPoint.y},
      },
      width: zipper.width,
      strength: zipper.strength,
      leading_dist: zipper.leadingDist,
    }
  })

  return JSON.stringify(new_json)
}