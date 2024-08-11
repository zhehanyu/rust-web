<template>
    <div>
      <h1>Rust + Vite Demo</h1>
      <button @click="sayHello">Say Hello</button>
      <button @click="executeFunction">Execute Function</button>
      <div>
        <input v-model="query" placeholder="Enter query" />
        <button @click="performQuery">Query</button>
      </div>
      <pre>{{ result }}</pre>

      <h2>Accounts</h2>
      <ul>
        <li v-for="account in accounts" :key="account.name">
          {{ account.name }} (Port 1: {{ account.port1 }}, Port 2: {{ account.port2 }})
        </li>
      </ul>
    </div>
  </template>
  
  <script>
  import axios from 'axios';
  
  export default {
    data() {
      return {
        result: '',
        query: '',
        accounts: [],
      }
    },
    methods: {
      async sayHello() {
        const response = await axios.get('http://localhost:8080/api/hello');
        this.result = response.data;
      },
      async executeFunction() {
        const response = await axios.post('http://localhost:8080/api/execute', { data: 'Some data' });
        this.result = JSON.stringify(response.data, null, 2);
      },
      async performQuery() {
        const response = await axios.get(`http://localhost:8080/api/query?query=${this.query}`);
        this.result = JSON.stringify(response.data, null, 2);
      },
      async fetchAccounts() {
        try {
          const response = await axios.get('http://localhost:8080/api/accounts');
          this.accounts = response.data;
        } catch (error) {
          console.error('Error fetching accounts:', error);
        }
      }
    },
    mounted() {
      this.fetchAccounts();
    }
  }
  </script>