import axios from 'axios';

const API_URL = 'https://localhost:3000/api';

const api = axios.create({
    baseURL: API_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});


const donate = async (amount, user) => {
    try {
      const response = await api.post('/donations/donate', { amount, user });
      return response.data;
    } catch (error) {
      throw error.response.data;
    }
  };

  
    export { donate };
  export default api;