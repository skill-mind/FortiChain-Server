module.exports = {
  up: async (queryInterface, Sequelize) => {
    // Drop the existing column
    await queryInterface.removeColumn('Reports', 'proofOfConcept');

    // Add the column back as JSON
    await queryInterface.addColumn('Reports', 'proofOfConcept', {
      type: Sequelize.JSON,
      allowNull: true,
    });
  },

  down: async (queryInterface, Sequelize) => {
    // Revert the column to its original type (array of strings)
    await queryInterface.removeColumn('Reports', 'proofOfConcept');
    await queryInterface.addColumn('Reports', 'proofOfConcept', {
      type: Sequelize.ARRAY(Sequelize.STRING),
      allowNull: true,
    });
  },
};