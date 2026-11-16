var builder = WebApplication.CreateBuilder(args);
builder.Services.AddOpenApi();
builder.Services.AddSingleton(_ =>
    Npgsql.NpgsqlDataSource.Create(
        builder.Configuration.GetConnectionString("DefaultConnection")
        ?? throw new InvalidOperationException("ConnectionStrings:DefaultConnection is required.")));
builder.Services.AddSingleton<StackExchange.Redis.IConnectionMultiplexer>(_ =>
    StackExchange.Redis.ConnectionMultiplexer.Connect(
        builder.Configuration["Redis:ConnectionString"]
        ?? throw new InvalidOperationException("Redis:ConnectionString is required.")));

var app = builder.Build();

if (app.Environment.IsDevelopment()) app.MapOpenApi();
app.MapGet("/health", () => Results.Ok(new { status = "healthy" }));
app.MapGet("/health/ready", async (
    Npgsql.NpgsqlDataSource dataSource,
    StackExchange.Redis.IConnectionMultiplexer redis) =>
{
    await using var connection = await dataSource.OpenConnectionAsync();
    await using var command = connection.CreateCommand();
    command.CommandText = "SELECT 1";
    await command.ExecuteScalarAsync();
    await redis.GetDatabase().PingAsync();

    return Results.Ok(new { status = "ready", postgres = "ready", redis = "ready" });
});
app.Run();

public partial class Program { }
