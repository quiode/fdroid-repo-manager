import { backend } from './../constants';
import * as axios_lib from 'axios';
export const axios = axios_lib.default;
export const default_config = {
  baseURL: backend,
  headers: { 'RM-PASSWORD': '' }
} satisfies axios_lib.AxiosRequestConfig;