import Axios from 'axios';

const axiosInstance = Axios.create({
    baseURL: "http://localhost:8080"
});

export default axiosInstance;