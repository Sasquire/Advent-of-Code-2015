words words

words words

TODO words

<div class="tab-wrapper">
	{% capture tab_content %}{% highlight rust %}{%
		include_relative graph.rs
	%}{% endhighlight %}{% endcapture %}
	{% include components/tab_item.html 
		index = "1" sub_index = "a"
		name = "graph.rs"
		content = tab_content
	%}
</div>