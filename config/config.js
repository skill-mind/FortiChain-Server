const dotenv = require('dotenv');
dotenv.config();

module.exports = {
  development: {
    username: process.env.DB_USER || 'fortichain_user',
    password: process.env.DB_PASSWORD || 'fortichain_password',
    database: process.env.DB_NAME || 'fortichain_db',
    host: process.env.DB_HOST || 'localhost',
    dialect: 'postgres',
    port: process.env.DB_PORT || 5432,
    logging: false
  },
  test: {
    dialect: 'sqlite',
    storage: ':memory:',
    logging: false
  },
  production: {
    username: process.env.DB_USER || 'fortichain_user',
    password: process.env.DB_PASSWORD || 'fortichain_password',
    database: process.env.DB_NAME || 'fortichain_db',
    host: process.env.DB_HOST || 'localhost',
    dialect: 'postgres',
    port: process.env.DB_PORT || 5432,
    logging: false
  },
  jwtSecret: process.env.JWT_SECRET || 'your-secret-key'
};