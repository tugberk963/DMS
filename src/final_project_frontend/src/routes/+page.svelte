<script>
  import "../index.scss";
  import {backend} from "$lib/canisters";

  let showLogin = true;

  function onSignUp(event){
      const username = event.target.signup_username.value;
      const password = event.target.signup_password.value;

      backend.sign_up(username, password).then((res) => {
        console.log(res);
      }).catch((error) => {
        console.log(error);
      });
  }

  function onLogin(event){
      const username = event.target.login_username.value;
      const password = event.target.login_password.value;

      backend.login(username, password).then((res) => {
        console.log(res);
      }).catch((error) => {
        console.log(error);
      });
  }

  function onLogout()
  {
    backend.logout().then((res) => {
        console.log(res);
      }).catch((err) => {
        console.log(err);
      });
  }

  function showLoginContainer(){
    showLogin = true;
  }

  function showSignUpContainer(){
    showLogin = false;
  }

</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <br />
  <br />
  <div class="formContainer">
    <div class="toggleButtons">
      <button on:click={showLoginContainer}>Login</button>
      <button on:click={showSignUpContainer}>Sign up</button>
    </div>
    {#if showLogin}
    <form action="#" on:submit|preventDefault={onLogin}>
      <h3>Login Now !</h3>
      <div class="loginContainer">
            <input id="login_username"  placeholder="Enter your username" alt="Name" type="text" />
            <input id="login_password"  placeholder="Enter your password" alt="Password" type="password" />
            <button type="submit">Login</button>
      </div>
    </form>
    {:else}
    <form action="#" on:submit|preventDefault={onSignUp}>
      <h3>Sign Up !</h3>
      <div class="signupContainer">
            <input id="signup_username"  placeholder="Enter your username" alt="Name" type="text" />
            <input id="signup_password"  placeholder="Enter your password" alt="Password" type="password" />
            <button type="submit">Sign up</button>
      </div>
    </form>
    {/if}
  </div>

  <button on:click|preventDefault={onLogout}>Click for logout..</button>
  <!--<section id="result">{response}</section> -->
</main>


<style>
  .formContainer,
  .loginContainer,
  .signupContainer{
    border: 1px solid black;
    display: flex;
    flex-direction: column;
  }
</style>