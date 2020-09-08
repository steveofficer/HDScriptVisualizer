// https://blog.logrocket.com/integrating-web-workers-in-a-react-app-with-comlink/

import { expose } from 'comlink';
import { analyze } from '../analyzer';

const exports = {
    analyze
};
export type Analyzer = typeof exports;

expose(exports);