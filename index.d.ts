declare module 'rw-intervals' {
  export interface Request {
    id: String,
    start_date: Number,
    end_date: Number,
    weight: Number,
  }

  export interface Schedule {
    name: String,
    overlaps: String[],
    requests: Request[],
    reservations: String[],
  }

  /**
   * Recalcula las reservas en un horario dado un nuevo request.
   * @param schedule - JSON string representando el horario existente.
   * @param newRequest - JSON string representando la nueva solicitud.
   * @returns JSON string con el horario actualizado.
   */
  export function recalculateReservations(schedule: String, newRequest: String) : Schedule

    /**
   * Recalcula las reservas en un horario dado un nuevo request.
   * @param sch - JSON string representando el horario existente.
   * @param new_request - JSON string representando la nueva solicitud.
   * @returns JSON string con el horario actualizado.
   */
  export function buildSchedule(requests: String) : Schedule
}