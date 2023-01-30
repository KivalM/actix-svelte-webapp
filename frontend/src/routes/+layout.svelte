<script>
	import '../app.css';
	import Navbar from '../components/navbar.svelte';
	import ToastHandler from '../components/toasts/toastHandler.svelte';
</script>

<div>
	<Navbar />
	<slot />

	<!-- other el -->
	<ToastHandler />
</div>
