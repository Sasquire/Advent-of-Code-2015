words words

words words

TODO words

<div class="tab-wrapper">
	{% capture tab_content %}{% highlight rust %}{%
		include_relative space.rs
	%}{% endhighlight %}{% endcapture %}
	{% include components/tab_item.html 
		index = "1" sub_index = "a"
		name = "space.rs"
		content = tab_content
	%}

	{% capture tab_content %}{% highlight rust %}{%
		include_relative point.rs
	%}{% endhighlight %}{% endcapture %}
	{% include components/tab_item.html 
		index = "1" sub_index = "b"
		name = "point.rs"
		content = tab_content
	%}
</div>