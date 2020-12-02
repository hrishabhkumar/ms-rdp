export const startRdpWithGui: (server: object) => number
// sleep [duration] ms, return Promise which resolved 2 * duration
export const startRdpWithoutGui: (server: object) => Promise<number>
