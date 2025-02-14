declare module 'rw-intervals' {
  export interface Request {
    id: string,
    start_date: number,
    end_date: number,
    weight: number,
  }

  export interface Schedule {
    name: string,
    overlaps: string[],
    requests: Request[],
    reservations: string[],
  }

  /**
   * Recalcula las reservas en un horario dado un nuevo request.
   * @param schedule - JSON string representando el horario existente.
   * @param newRequest - JSON string representando la nueva solicitud.
   * @returns JSON string con el horario actualizado.
   */
  export function recalculateReservations(schedule: string, newRequest: string) : Schedule

    /**
   * Recalcula las reservas en un horario dado un nuevo request.
   * @param sch - JSON string representando el horario existente.
   * @param new_request - JSON string representando la nueva solicitud.
   * @returns JSON string con el horario actualizado.
   */
  export function buildSchedule(requests: string) : Schedule
}