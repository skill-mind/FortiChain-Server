const { Sequelize } = require('sequelize');
const dotenv = require('dotenv');

dotenv.config();

const isTest = process.env.NODE_ENV === 'test';

let sequelize;

if (isTest) {
  sequelize = new Sequelize({
    dialect: 'sqlite',
    storage: ':memory:',
    logging: false
  });
} else {
  sequelize = new Sequelize(
    process.env.DB_NAME || 'fortichain_db',
    process.env.DB_USER || 'fortichain_user',
    process.env.DB_PASSWORD || 'fortichain_password',
    {
      host: process.env.DB_HOST || 'localhost',
      dialect: 'postgres',
      port: process.env.DB_PORT || 5432,
      pool: {
        max: 5,
        min: 0,
        acquire: 30000,
        idle: 10000
      },
      logging: false
    }
  );
}

module.exports = sequelize;
