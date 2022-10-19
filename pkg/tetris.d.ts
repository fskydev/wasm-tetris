/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Direction {
  Left,
  Right,
}
/**
*/
export enum Cell {
  Active,
  Inactive,
}
/**
*/
export class Tetris {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Tetris}
*/
  static new(width: number, height: number): Tetris;
/**
*/
  tick(): void;
/**
* @param {number} direction
*/
  shift(direction: number): void;
/**
*/
  rotate(): void;
/**
* @returns {number}
*/
  draw(): number;
}
