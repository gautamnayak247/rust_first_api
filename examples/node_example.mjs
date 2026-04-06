// Node example that imports the WASM from `pkg`
// Bundler target auto-initializes WASM

import { create_employee, get_employee, update_employee } from '../pkg/employee_api.js';

function main() {
  const created = create_employee({ id: 2, name: 'Ana', role: 'Contract' });
  console.log('created:', created);

  const got = get_employee();
  console.log('got:', got);

  const upd = update_employee({ id: 2, name: 'Ana', role: 'Contractor' });
  console.log('update result:', upd);
}

main();
