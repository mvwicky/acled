<html>
	<head>
		<title>ACLED</title>
		<script defer src="/dart/main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
		<link rel="stylesheet" type="text/css" href="/static/styles/main.css">
	</head>
	<body>
        <h1>ACELD.rs</h1>
		<br />
		<!-- <img class="map" src="/static/images/africa_map_md.png"> -->
      
        <h3> Country List </h3>
        {{#countries}}
           <b><a href="country/{{ link }}">{{ name }}</a></b><br />
        {{/countries}}
	</body>
</html>
