// Demo npm project for testing confrisk-npm
// This project intentionally uses older versions with vulnerabilities

const express = require('express');
const _ = require('lodash');

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  const data = { message: 'Hello from confrisk-npm demo!' };
  const cloned = _.cloneDeep(data);
  res.json(cloned);
});

app.listen(port, () => {
  console.log(`Demo app listening at http://localhost:${port}`);
});
