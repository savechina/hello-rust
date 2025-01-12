<ul>
        {% for item in items %}
            <li>{{ item }}</li>
        {% endfor %}
 </ul>

{% if user %}
    <p>User details:</p>
    <p>ID: {{ user.id }}</p>
    <p>Name: {{ user.name }}</p>
{% endif %}