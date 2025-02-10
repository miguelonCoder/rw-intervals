import { createRequire } from "module";
const require = createRequire(import.meta.url);
const nativeModule = require("./dist/index.node");

export const { recalculateReservations, buildSchedule } = nativeModule;