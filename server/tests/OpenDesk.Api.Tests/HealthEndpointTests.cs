using System.Net;
using Microsoft.AspNetCore.Mvc.Testing;
using Xunit;

namespace OpenDesk.Api.Tests;

public sealed class HealthEndpointTests(WebApplicationFactory<Program> factory)
    : IClassFixture<WebApplicationFactory<Program>>
{
    [Fact]
    public async Task Live_endpoint_starts_without_connecting_to_dependencies()
    {
        using var client = factory.CreateClient();

        var response = await client.GetAsync("/health");

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
    }

    [Fact]
    public async Task Ready_endpoint_checks_postgres_and_redis()
    {
        using var client = factory.CreateClient();

        var response = await client.GetAsync("/health/ready");

        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        var body = await response.Content.ReadAsStringAsync();
        Assert.Contains("\"postgres\":\"ready\"", body, StringComparison.Ordinal);
        Assert.Contains("\"redis\":\"ready\"", body, StringComparison.Ordinal);
    }
}
