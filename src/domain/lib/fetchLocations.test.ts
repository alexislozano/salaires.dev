import { assert, assertEquals } from "assert";
import { InMemoryLocationRepository } from "@infra";
import { fetchLocations } from "./fetchLocations.ts";
import { Result } from "@utils";
import { Location } from "@domain";

Deno.test("it should return an error when an error occurs", async () => {
    const locationRepo = InMemoryLocationRepository.withError();

    const result = await fetchLocations(locationRepo);

    assert(Result.isErr(result));
});

Deno.test("it should return all locations otherwise", async () => {
    const location = Location.generate();
    const locationRepo = InMemoryLocationRepository.new();
    locationRepo.insert(location);

    const result = await fetchLocations(locationRepo);

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [location]);
});
