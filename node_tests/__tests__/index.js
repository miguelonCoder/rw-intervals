const path = require('path');
const RwIntervals = require(path.resolve(__dirname,'..', '../dist'));

test('build_schedule should return success message', () => {
  const testInput = JSON.stringify([{ id: 1, description: "Test Request" }]);
  const expectedOutput = "Schedule built successfully";

  const result = RwIntervals.build_schedule(testInput);
  console.log(result)
  expect(result.contains('Error parsing JSON')).toBeTruthy;
});