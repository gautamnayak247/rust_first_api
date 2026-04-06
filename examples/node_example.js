// Node example for CommonJS
// The nodejs target generates CommonJS modules with automatic WASM initialization

const { create_employee, get_employee, update_employee } = require('../pkg/employee_api.js');

function main() {
  const created = create_employee({ id: 2, name: 'Ana', role: 'Contract' });
  console.log('created:', created);

  const got = get_employee();
  console.log('got:', got);

  const upd = update_employee({ id: 2, name: 'Ana', role: 'Contractor' });
  console.log('update result:', upd);
}

main();
