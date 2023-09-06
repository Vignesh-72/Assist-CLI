<h1>Assist-CLI</h1>
<h2>Overview</h2>
<p>Assist-CLI is a command-line tool that enables seamless communication with API's through the command-line interface. It simplifies interactions and enhances efficiency.</p>

<h2>How to Use</h2>
<p>Syntax: <code>AssistX [OPTIONS] [INPUT]</code></p>
<h3>Options:</h3>
<ul>
    <li><code>-h, --help</code>: Display help</li>
    <li><code>-?, --connect</code>: Sends Request To API</li>
    <li><code>--hfconnect</code> : Sends Request To A Huggingface Model</li>
    <li><code>--setup</code>: Setup the environment</li>
    <li><code>--uninstall</code>: Uninstall Assist-CLI</li>
    <li><code>--version</code>: Check Version</li>
    <li><code>--about</code>: About Assist-CLI</li>
</ul>
<h2>Example: Connecting Assistx With HuggingFace GPT Model</h2>
<p>Add This To Env File : <code>API="https://api-inference.huggingface.co/models/microsoft/DialoGPT-medium"</code></p>
<p>Add Huggingface Access Token In The Key Field : <code>Key="xxxxxxxxxx"</code></p>
<p>Example: <code>AssistX --connect "How Are You"</code></p>
<p>Output: <code>Model: I'm Find Thank You </code></p>

<p><strong>Note:</strong> Note That The CLI Tool Requires A Key To Connect With Huggingface Models You Can Provide It in The Key Filed In The env File.</p>

</body>
</html>
